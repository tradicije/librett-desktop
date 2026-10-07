use crate::{Category, CompetitionFormat, DomainError, Entry, EntryStatus};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DrawMode {
    Automatic,
    Manual,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DrawSettings {
    pub group_count: usize,
    pub qualifiers_per_group: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CategoryDraw {
    pub id: Uuid,
    pub revision: u32,
    pub category_id: Uuid,
    pub format: CompetitionFormat,
    pub mode: DrawMode,
    pub algorithm_version: u32,
    pub random_seed: Uuid,
    pub settings: DrawSettings,
    pub participants: Vec<Entry>,
    pub seeds: Vec<Uuid>,
    // Groups: one section per group. Knockout: one section of first-round
    // bracket slots; adjacent slots are opponents. An empty slot is a bye
    // only when its opponent is present. Manual drafts may be incomplete.
    pub sections: Vec<Vec<Option<Uuid>>>,
}

fn capacities(
    format: CompetitionFormat,
    count: usize,
    settings: &DrawSettings,
) -> Result<Vec<usize>, DomainError> {
    if !(2..=4096).contains(&count) {
        return Err(DomainError::InvalidDraw);
    }
    match format {
        CompetitionFormat::Knockout => Ok(vec![count.next_power_of_two()]),
        CompetitionFormat::GroupsKnockout => {
            let groups = settings.group_count;
            if groups == 0
                || groups > count / 2
                || settings.qualifiers_per_group == 0
                || settings.qualifiers_per_group > count / groups
                || groups * settings.qualifiers_per_group < 2
            {
                return Err(DomainError::InvalidDraw);
            }
            Ok((0..groups)
                .map(|i| count / groups + usize::from(i < count % groups))
                .collect())
        }
    }
}

pub fn validate_draw(
    draw: &CategoryDraw,
    category: &Category,
    entries: &[Entry],
) -> Result<(), DomainError> {
    if category.archived
        || draw.category_id != category.id
        || draw.format != category.format
        || !matches!(draw.algorithm_version, 1 | 2)
    {
        return Err(DomainError::InvalidDraw);
    }
    let active: HashSet<_> = entries
        .iter()
        .filter(|e| e.status == EntryStatus::Registered)
        .map(|e| e.id)
        .collect();
    let participants: HashSet<_> = draw.participants.iter().map(|e| e.id).collect();
    if participants != active
        || participants.len() != draw.participants.len()
        || draw
            .participants
            .iter()
            .any(|e| e.category_id != category.id || e.status != EntryStatus::Registered)
    {
        return Err(DomainError::InvalidDraw);
    }
    let sizes = capacities(draw.format, active.len(), &draw.settings)?;
    if draw.sections.len() != sizes.len()
        || draw
            .sections
            .iter()
            .zip(sizes)
            .any(|(section, size)| section.len() != size)
    {
        return Err(DomainError::InvalidDraw);
    }
    let mut used = HashSet::new();
    for id in draw.sections.iter().flatten().flatten() {
        if !active.contains(id) || !used.insert(*id) {
            return Err(DomainError::InvalidDraw);
        }
    }
    let seeds: HashSet<_> = draw.seeds.iter().copied().collect();
    if seeds.len() != draw.seeds.len() || !seeds.is_subset(&active) {
        return Err(DomainError::InvalidDraw);
    }
    if (draw.mode == DrawMode::Automatic && used != active)
        || (used == active
            && draw.format == CompetitionFormat::Knockout
            && draw.sections[0]
                .chunks_exact(2)
                .any(|pair| pair.iter().all(Option::is_none)))
    {
        return Err(DomainError::InvalidDraw);
    }
    Ok(())
}

// SplitMix64 with Fisher-Yates. Version 2 adds snake seeding and club separation.
// Version 1 saved layouts remain valid; new proposals are marked version 2.
struct Random(u64);
impl Random {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e3779b97f4a7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
    }
    fn shuffle<T>(&mut self, values: &mut [T]) {
        for i in (1..values.len()).rev() {
            let bound = (i + 1) as u64;
            let limit = u64::MAX - u64::MAX % bound;
            let sample = loop {
                let value = self.next();
                if value < limit {
                    break value;
                }
            };
            values.swap(i, (sample % bound) as usize);
        }
    }
}

pub fn create_draw(
    category: &Category,
    entries: &[Entry],
    mode: DrawMode,
    settings: DrawSettings,
    seeds: Vec<Uuid>,
    random_seed: Uuid,
) -> Result<CategoryDraw, DomainError> {
    let participants: Vec<_> = entries
        .iter()
        .filter(|e| e.status == EntryStatus::Registered)
        .cloned()
        .collect();
    let sizes = capacities(category.format, participants.len(), &settings)?;
    let mut draw = CategoryDraw {
        id: Uuid::new_v4(),
        revision: 0,
        category_id: category.id,
        format: category.format,
        mode,
        algorithm_version: 2,
        random_seed,
        settings,
        participants,
        seeds,
        sections: sizes.iter().map(|&size| vec![None; size]).collect(),
    };
    // Validate the seed list before indexing into generated slots.
    let mut validation = draw.clone();
    validation.mode = DrawMode::Manual;
    validate_draw(&validation, category, entries)?;
    if mode == DrawMode::Manual {
        return Ok(draw);
    }
    let seed = random_seed.as_u128();
    let mut random = Random(seed as u64 ^ (seed >> 64) as u64);
    let mut others: Vec<_> = draw
        .participants
        .iter()
        .map(|e| e.id)
        .filter(|id| !draw.seeds.contains(id))
        .collect();
    random.shuffle(&mut others);
    match category.format {
        CompetitionFormat::GroupsKnockout => {
            for (index, &id) in draw.seeds.iter().enumerate() {
                let offset = index % draw.sections.len();
                let group = if (index / draw.sections.len()) % 2 == 0 {
                    offset
                } else {
                    draw.sections.len() - 1 - offset
                };
                let slot = draw.sections[group]
                    .iter()
                    .position(Option::is_none)
                    .ok_or(DomainError::InvalidDraw)?;
                draw.sections[group][slot] = Some(id);
            }
            for id in others {
                let group = draw
                    .sections
                    .iter()
                    .enumerate()
                    .filter(|(_, slots)| slots.iter().any(Option::is_none))
                    .min_by_key(|(_, slots)| slots.iter().flatten().count())
                    .map(|(group, _)| group)
                    .ok_or(DomainError::InvalidDraw)?;
                let slot = draw.sections[group]
                    .iter()
                    .position(Option::is_none)
                    .ok_or(DomainError::InvalidDraw)?;
                draw.sections[group][slot] = Some(id);
            }
        }
        CompetitionFormat::Knockout => {
            let size = sizes[0];
            let mut ranks = vec![1, 2];
            while ranks.len() < size {
                let next = ranks.len() * 2 + 1;
                ranks = ranks
                    .into_iter()
                    .flat_map(|rank| [rank, next - rank])
                    .collect();
            }
            let mut blocked = vec![false; size];
            for (rank, &id) in draw.seeds.iter().enumerate() {
                let position = ranks
                    .iter()
                    .position(|&value| value == rank + 1)
                    .ok_or(DomainError::InvalidDraw)?;
                draw.sections[0][position] = Some(id);
            }
            let byes = size - draw.participants.len();
            for rank in 1..=byes.min(draw.seeds.len()) {
                let position = ranks
                    .iter()
                    .position(|&value| value == rank)
                    .ok_or(DomainError::InvalidDraw)?;
                blocked[position ^ 1] = true;
            }
            let extra = byes.saturating_sub(draw.seeds.len());
            let mut free_pairs: Vec<_> = (0..size)
                .step_by(2)
                .filter(|&p| {
                    draw.sections[0][p].is_none()
                        && draw.sections[0][p + 1].is_none()
                        && !blocked[p]
                        && !blocked[p + 1]
                })
                .collect();
            random.shuffle(&mut free_pairs);
            for (&position, &id) in free_pairs.iter().zip(others.iter()).take(extra) {
                draw.sections[0][position] = Some(id);
                blocked[position + 1] = true;
            }
            let mut remaining = others.into_iter().skip(extra);
            for (position, slot) in draw.sections[0].iter_mut().enumerate() {
                if slot.is_none() && !blocked[position] {
                    *slot = remaining.next();
                }
            }
        }
    }
    validate_draw(&draw, category, entries)?;
    Ok(draw)
}

/// Club identity for draw constraints, without guessing typographical errors.
pub fn club_key(name: &str) -> String {
    let normalized: String = name
        .to_lowercase()
        .chars()
        .map(|c| match c {
            'š' | 'ś' => 's',
            'č' | 'ć' => 'c',
            'ž' => 'z',
            'đ' => 'd',
            c if c.is_alphanumeric() => c,
            _ => ' ',
        })
        .collect();
    let words: Vec<_> = normalized.split_whitespace().collect();
    let words = if words.starts_with(&["stoni", "teniski", "klub"]) {
        &words[3..]
    } else if words.first() == Some(&"stk") {
        &words[1..]
    } else {
        &words[..]
    };
    words.join("")
}
fn clubs(entry: &Entry) -> HashSet<String> {
    entry
        .members
        .iter()
        .map(|m| club_key(&m.club))
        .filter(|club| !club.is_empty())
        .collect()
}
pub fn has_group_club_conflicts(draw: &CategoryDraw) -> bool {
    let identities: std::collections::HashMap<_, _> = draw
        .participants
        .iter()
        .map(|entry| (entry.id, clubs(entry)))
        .collect();
    draw.sections.iter().any(|group| {
        let mut used = HashSet::new();
        group.iter().flatten().any(|id| {
            identities
                .get(id)
                .is_some_and(|clubs| clubs.iter().any(|club| !used.insert(club.clone())))
        })
    })
}
/// Preserve seeded positions; distribute the remaining entries with bounded retries.
pub fn separate_group_clubs(draw: &mut CategoryDraw) -> Result<(), DomainError> {
    if draw.mode != DrawMode::Automatic || draw.format != CompetitionFormat::GroupsKnockout {
        return Ok(());
    }
    let identities: std::collections::HashMap<_, _> = draw
        .participants
        .iter()
        .map(|entry| (entry.id, clubs(entry)))
        .collect();
    let mut fixed = draw.sections.clone();
    let seeded: HashSet<_> = draw.seeds.iter().copied().collect();
    for slot in fixed.iter_mut().flatten() {
        if slot.is_some_and(|id| !seeded.contains(&id)) {
            *slot = None;
        }
    }
    let mut random =
        Random(draw.random_seed.as_u128() as u64 ^ (draw.random_seed.as_u128() >> 64) as u64);
    let mut remaining: Vec<_> = draw
        .participants
        .iter()
        .filter(|entry| !seeded.contains(&entry.id))
        .map(|entry| entry.id)
        .collect();
    let mut frequencies = std::collections::HashMap::<String, usize>::new();
    for entry in &draw.participants {
        for club in &identities[&entry.id] {
            *frequencies.entry(club.clone()).or_default() += 1;
        }
    }
    if frequencies
        .values()
        .any(|count| *count > draw.sections.len())
    {
        return Err(DomainError::InvalidDraw);
    }
    for _ in 0..64 {
        let mut sections = fixed.clone();
        let mut used: Vec<HashSet<String>> = sections
            .iter()
            .map(|group| {
                group
                    .iter()
                    .flatten()
                    .flat_map(|id| identities[id].iter().cloned())
                    .collect()
            })
            .collect();
        if sections.iter().enumerate().any(|(index, group)| {
            group
                .iter()
                .flatten()
                .map(|id| identities[id].len())
                .sum::<usize>()
                > used[index].len()
        }) {
            return Err(DomainError::InvalidDraw);
        }
        random.shuffle(&mut remaining);
        remaining.sort_by_key(|id| {
            std::cmp::Reverse(
                identities[id]
                    .iter()
                    .map(|club| frequencies[club])
                    .max()
                    .unwrap_or(0),
            )
        });
        let mut success = true;
        for id in &remaining {
            let mut candidates: Vec<_> = sections
                .iter()
                .enumerate()
                .filter(|(group, slots)| {
                    slots.iter().any(Option::is_none) && identities[id].is_disjoint(&used[*group])
                })
                .map(|(index, _)| index)
                .collect();
            random.shuffle(&mut candidates);
            candidates.sort_by_key(|group| {
                std::cmp::Reverse(
                    sections[*group]
                        .iter()
                        .filter(|slot| slot.is_none())
                        .count(),
                )
            });
            let Some(&group) = candidates.first() else {
                success = false;
                break;
            };
            let slot = sections[group].iter().position(Option::is_none).unwrap();
            sections[group][slot] = Some(*id);
            used[group].extend(identities[id].iter().cloned());
        }
        if success {
            draw.sections = sections;
            return Ok(());
        }
    }
    Err(DomainError::InvalidDraw)
}
