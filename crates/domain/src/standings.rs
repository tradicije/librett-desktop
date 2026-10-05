use crate::{
    CategoryDraw, CategoryRules, MatchOutcome, MatchResult, RankingCriterion, StoredMatchResult,
};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StandingRow {
    pub entry_id: Uuid,
    pub played: usize,
    pub wins: usize,
    pub losses: usize,
    pub sets_for: u64,
    pub sets_against: u64,
    pub points_for: u64,
    pub points_against: u64,
    pub tied: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupStanding {
    pub group: usize,
    pub rows: Vec<StandingRow>,
    pub completed: usize,
    pub total: usize,
    pub complete: bool,
    pub resolved: bool,
    pub manual: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualificationSlot {
    pub entry_id: Option<Uuid>,
    pub group: Option<usize>,
    pub place: Option<usize>,
    pub bye: bool,
    #[serde(default)]
    pub lucky_loser: bool,
}
fn apply(rows: &mut HashMap<Uuid, StandingRow>, result: &MatchResult) {
    let mut sets = [0u64; 2];
    let mut points = [0u64; 2];
    for score in &result.sets {
        points[0] += u64::from(score.first);
        points[1] += u64::from(score.second);
        if score.first.max(score.second) >= u16::from(result.rules.points_to_win)
            && score.first.abs_diff(score.second) >= u16::from(result.rules.win_by)
        {
            sets[usize::from(score.second > score.first)] += 1;
        }
    }
    // Non-played endings award the remaining sets at the configured target to the winner.
    if result.outcome != MatchOutcome::Played {
        let winner = usize::from(result.winner == result.second);
        let remaining = u64::from(result.rules.best_of / 2 + 1).saturating_sub(sets[winner]);
        sets[winner] += remaining;
        points[winner] += remaining * u64::from(result.rules.points_to_win);
    }
    for (side, id) in [result.first, result.second].into_iter().enumerate() {
        if let Some(row) = rows.get_mut(&id) {
            row.played += 1;
            if id == result.winner {
                row.wins += 1;
            } else {
                row.losses += 1;
            }
            row.sets_for += sets[side];
            row.sets_against += sets[side ^ 1];
            row.points_for += points[side];
            row.points_against += points[side ^ 1];
        }
    }
}
fn ratio(a: (u64, u64), b: (u64, u64)) -> Ordering {
    let infinity = |(n, d): (u64, u64)| d == 0 && n > 0;
    if infinity(a) || infinity(b) {
        return infinity(a).cmp(&infinity(b));
    }
    (u128::from(a.0) * u128::from(b.1.max(1))).cmp(&(u128::from(b.0) * u128::from(a.1.max(1))))
}
fn rank_tied(
    ids: &[Uuid],
    all: &HashMap<Uuid, StandingRow>,
    results: &HashMap<Uuid, Vec<&MatchResult>>,
    ranking: &[RankingCriterion],
) -> Vec<Vec<Uuid>> {
    if ids.len() < 2 || ranking.is_empty() {
        return vec![ids.to_vec()];
    }
    let set: HashSet<_> = ids.iter().copied().collect();
    let mut mini: HashMap<_, _> = ids
        .iter()
        .map(|id| {
            (
                *id,
                StandingRow {
                    entry_id: *id,
                    ..Default::default()
                },
            )
        })
        .collect();
    for id in ids {
        for result in results
            .get(id)
            .into_iter()
            .flatten()
            .filter(|r| r.first == *id && set.contains(&r.second))
        {
            apply(&mut mini, result);
        }
    }
    let head = ranking
        .iter()
        .position(|c| *c == RankingCriterion::HeadToHead);
    let compare = |a: &Uuid, b: &Uuid| {
        for (index, criterion) in ranking.iter().enumerate() {
            let source = if head.is_some_and(|h| h <= index) {
                &mini
            } else {
                all
            };
            let (a, b) = (&source[a], &source[b]);
            let order = match criterion {
                RankingCriterion::HeadToHead => a.wins.cmp(&b.wins),
                RankingCriterion::SetRatio => {
                    ratio((a.sets_for, a.sets_against), (b.sets_for, b.sets_against))
                }
                RankingCriterion::PointRatio => ratio(
                    (a.points_for, a.points_against),
                    (b.points_for, b.points_against),
                ),
            };
            if order != Ordering::Equal {
                return order;
            }
        }
        Ordering::Equal
    };
    let mut sorted = ids.to_vec();
    sorted.sort_by(|a, b| compare(b, a));
    let mut buckets: Vec<Vec<Uuid>> = vec![];
    for id in sorted {
        if let Some(bucket) = buckets
            .last_mut()
            .filter(|bucket| compare(&bucket[0], &id) == Ordering::Equal)
        {
            bucket.push(id);
        } else {
            buckets.push(vec![id]);
        }
    }
    buckets
}
pub fn group_standings(
    draw: &CategoryDraw,
    rules: &CategoryRules,
    stored: &HashMap<String, StoredMatchResult>,
    orders: &HashMap<usize, Vec<Uuid>>,
) -> Vec<GroupStanding> {
    // Results are already tied to this draw and structural match keys by the storage layer.
    let mut by_group: HashMap<usize, Vec<MatchResult>> = HashMap::new();
    for item in stored.values() {
        let parts: Vec<_> = item.key.split(':').collect();
        if parts.len() != 4 || parts[0] != "group" {
            continue;
        }
        if let (Ok(group), Some(result)) = (parts[1].parse::<usize>(), &item.result) {
            by_group.entry(group).or_default().push(result.clone());
        }
    }
    draw.sections
        .iter()
        .enumerate()
        .map(|(group, section)| {
            let ids: Vec<_> = section.iter().flatten().copied().collect();
            let mut rows: HashMap<_, _> = ids
                .iter()
                .map(|id| {
                    (
                        *id,
                        StandingRow {
                            entry_id: *id,
                            ..Default::default()
                        },
                    )
                })
                .collect();
            let results: Vec<_> = by_group
                .remove(&group)
                .unwrap_or_default()
                .into_iter()
                .filter(|r| rows.contains_key(&r.first) && rows.contains_key(&r.second))
                .collect();
            for result in &results {
                apply(&mut rows, result);
            }
            let total = ids.len() * ids.len().saturating_sub(1) / 2;
            let complete = total > 0 && results.len() == total && ids.len() == section.len();
            let mut sorted = ids.clone();
            sorted.sort_by_key(|id| std::cmp::Reverse(rows[id].wins));
            let mut buckets: Vec<Vec<Uuid>> = vec![];
            for id in sorted {
                if let Some(bucket) = buckets
                    .last_mut()
                    .filter(|b| rows[&b[0]].wins == rows[&id].wins)
                {
                    bucket.push(id);
                } else {
                    buckets.push(vec![id]);
                }
            }
            let mut adjacency: HashMap<Uuid, Vec<&MatchResult>> = HashMap::new();
            for result in &results {
                adjacency.entry(result.first).or_default().push(result);
                adjacency.entry(result.second).or_default().push(result);
            }
            let mut ordered = Vec::new();
            let mut resolved = true;
            for bucket in buckets {
                for tied in rank_tied(&bucket, &rows, &adjacency, &rules.ranking) {
                    if tied.len() > 1 {
                        resolved = false;
                        for id in &tied {
                            rows.get_mut(id).unwrap().tied = true;
                        }
                    }
                    ordered.extend(tied);
                }
            }
            let manual = complete
                && orders.get(&group).is_some_and(|order| {
                    order.len() == ids.len()
                        && order.iter().copied().collect::<HashSet<_>>()
                            == ids.iter().copied().collect()
                });
            if manual {
                ordered = orders[&group].clone();
                resolved = true;
            }
            GroupStanding {
                group,
                completed: results.len(),
                total,
                complete,
                resolved,
                manual,
                rows: ordered
                    .into_iter()
                    .map(|id| rows.remove(&id).unwrap())
                    .collect(),
            }
        })
        .collect()
}
pub fn qualification_slots(
    draw: &CategoryDraw,
    groups: &[GroupStanding],
) -> Vec<QualificationSlot> {
    let count = draw.settings.group_count * draw.settings.qualifiers_per_group;
    if !(2..=4096).contains(&count) {
        return vec![];
    }
    let mut qualified = Vec::new();
    for place in 0..draw.settings.qualifiers_per_group {
        for group in 0..draw.settings.group_count {
            let entry_id = groups
                .get(group)
                .filter(|g| g.complete && g.resolved)
                .and_then(|g| g.rows.get(place))
                .map(|r| r.entry_id);
            qualified.push(QualificationSlot {
                entry_id,
                group: Some(group),
                place: Some(place + 1),
                bye: false,
                lucky_loser: false,
            });
        }
    }
    let mut ranks = vec![1usize, 2];
    while ranks.len() < count.next_power_of_two() {
        let total = ranks.len() * 2 + 1;
        ranks = ranks.into_iter().flat_map(|r| [r, total - r]).collect();
    }
    let mut slots: Vec<_> = ranks
        .iter()
        .map(|rank| {
            qualified
                .get(rank - 1)
                .cloned()
                .unwrap_or(QualificationSlot {
                    entry_id: None,
                    group: None,
                    place: None,
                    bye: true,
                    lucky_loser: false,
                })
        })
        .collect();
    for i in (0..slots.len()).step_by(2) {
        if slots[i].group.is_none() || slots[i].group != slots[i + 1].group {
            continue;
        }
        let moving = if slots[i].place.unwrap_or(0) > slots[i + 1].place.unwrap_or(0) {
            i
        } else {
            i + 1
        };
        if let Some(candidate) = slots.iter().enumerate().position(|(index, slot)| {
            index != i
                && index != i + 1
                && slot.group.is_some()
                && slot.group != slots[moving ^ 1].group
                && slot.place.unwrap_or(0) > 1
                && slots[index ^ 1].group != slots[moving].group
        }) {
            slots.swap(moving, candidate);
        }
    }
    slots
}

/// Only places below the direct qualification cutoff can be lucky losers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LuckyLoserCandidate {
    pub group: usize,
    pub place: usize,
    pub standing: StandingRow,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "entry_id", rename_all = "snake_case")]
pub enum FillerChoice {
    Bye,
    Entry(Uuid),
}

fn candidate_cmp(a: &LuckyLoserCandidate, b: &LuckyLoserCandidate) -> Ordering {
    a.place
        .cmp(&b.place)
        .then_with(|| {
            ratio(
                (b.standing.wins as u64, b.standing.played as u64),
                (a.standing.wins as u64, a.standing.played as u64),
            )
        })
        .then_with(|| {
            ratio(
                (b.standing.sets_for, b.standing.sets_against),
                (a.standing.sets_for, a.standing.sets_against),
            )
        })
        .then_with(|| {
            ratio(
                (b.standing.points_for, b.standing.points_against),
                (a.standing.points_for, a.standing.points_against),
            )
        })
}
pub fn lucky_loser_candidates(
    draw: &CategoryDraw,
    groups: &[GroupStanding],
) -> Vec<LuckyLoserCandidate> {
    if groups.len() != draw.settings.group_count
        || groups.iter().any(|g| !g.complete || !g.resolved)
    {
        return vec![];
    }
    let mut candidates: Vec<_> = groups
        .iter()
        .flat_map(|g| {
            g.rows
                .iter()
                .enumerate()
                .skip(draw.settings.qualifiers_per_group)
                .map(move |(index, row)| LuckyLoserCandidate {
                    group: g.group,
                    place: index + 1,
                    standing: row.clone(),
                })
        })
        .collect();
    candidates.sort_by(candidate_cmp);
    candidates
}
pub fn filled_qualification_slots(
    draw: &CategoryDraw,
    groups: &[GroupStanding],
    mode: crate::KnockoutFilling,
    choices: &HashMap<usize, FillerChoice>,
) -> Vec<QualificationSlot> {
    let mut slots = qualification_slots(draw, groups);
    if mode == crate::KnockoutFilling::Bye {
        return slots;
    }
    let ready = groups.len() == draw.settings.group_count
        && groups.iter().all(|g| g.complete && g.resolved);
    let candidates = lucky_loser_candidates(draw, groups);
    let mut used = HashSet::new();
    let mut automatic = Vec::new();
    for (index, slot) in slots.iter_mut().enumerate() {
        if !slot.bye {
            continue;
        }
        slot.lucky_loser = true;
        match choices.get(&index) {
            Some(FillerChoice::Bye) => {}
            Some(FillerChoice::Entry(id)) => {
                slot.bye = false;
                if let Some(c) = candidates
                    .iter()
                    .find(|c| c.standing.entry_id == *id && !used.contains(id))
                {
                    slot.entry_id = Some(*id);
                    slot.group = Some(c.group);
                    slot.place = Some(c.place);
                    used.insert(*id);
                }
                // A formerly eligible manual choice must be reviewed, never silently replaced.
            }
            None => {
                slot.bye = false;
                automatic.push(index);
            }
        }
    }
    if !ready {
        return slots;
    }
    let available: Vec<_> = candidates
        .iter()
        .filter(|c| !used.contains(&c.standing.entry_id))
        .collect();
    let cutoff = automatic.len().min(available.len());
    let boundary = if cutoff > 0
        && cutoff < available.len()
        && candidate_cmp(available[cutoff - 1], available[cutoff]) == Ordering::Equal
    {
        let mut start = cutoff - 1;
        while start > 0 && candidate_cmp(available[start - 1], available[cutoff]) == Ordering::Equal
        {
            start -= 1;
        }
        start
    } else {
        cutoff
    };
    for (rank, index) in automatic.into_iter().enumerate() {
        let slot = &mut slots[index];
        if rank < boundary {
            let c = available[rank];
            slot.entry_id = Some(c.standing.entry_id);
            slot.group = Some(c.group);
            slot.place = Some(c.place);
        } else if rank >= available.len() {
            slot.bye = true;
        }
    }
    slots
}
