use crate::{CategoryDraw, CategoryRules, CompetitionFormat, DomainError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MatchOutcome {
    Played,
    Retired,
    Walkover,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SetScore {
    pub first: u16,
    pub second: u16,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MatchResult {
    pub first: Uuid,
    pub second: Uuid,
    pub winner: Uuid,
    pub outcome: MatchOutcome,
    pub sets: Vec<SetScore>,
    pub rules: CategoryRules,
}
impl MatchResult {
    pub fn validate(&self) -> Result<(), DomainError> {
        self.rules.validate(CompetitionFormat::Knockout)?;
        if self.first == self.second
            || ![self.first, self.second].contains(&self.winner)
            || self.sets.len() > self.rules.best_of as usize
        {
            return Err(DomainError::InvalidResult);
        }
        if self.outcome == MatchOutcome::Walkover {
            return if self.sets.is_empty() {
                Ok(())
            } else {
                Err(DomainError::InvalidResult)
            };
        }
        let needed = self.rules.best_of / 2 + 1;
        let mut wins = [0u8; 2];
        for (index, set) in self.sets.iter().enumerate() {
            if wins.contains(&needed) || set.first > 999 || set.second > 999 {
                return Err(DomainError::InvalidResult);
            }
            let high = set.first.max(set.second);
            let low = set.first.min(set.second);
            let target = u16::from(self.rules.points_to_win);
            let margin = u16::from(self.rules.win_by);
            // A completed set ends at the first score satisfying both requirements.
            let complete = high >= target && high - low >= margin;
            if complete
                && !(high == target && low <= target.saturating_sub(margin)
                    || high > target && high - low == margin)
            {
                return Err(DomainError::InvalidResult);
            }
            if !complete {
                if self.outcome != MatchOutcome::Retired || index + 1 != self.sets.len() {
                    return Err(DomainError::InvalidResult);
                }
            } else {
                wins[usize::from(set.second > set.first)] += 1;
            }
        }
        match self.outcome {
            MatchOutcome::Played => {
                let side = usize::from(self.winner == self.second);
                if wins[side] != needed {
                    return Err(DomainError::InvalidResult);
                }
            }
            MatchOutcome::Retired => {
                if wins.contains(&needed) {
                    return Err(DomainError::InvalidResult);
                }
            }
            MatchOutcome::Walkover => unreachable!(),
        }
        Ok(())
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredMatchResult {
    pub key: String,
    pub revision: u32,
    pub result: Option<MatchResult>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledMatch {
    pub key: String,
    pub round: usize,
    pub position: usize,
    pub first: Option<Uuid>,
    pub second: Option<Uuid>,
    pub bye: bool,
    pub result: Option<MatchResult>,
    pub revision: u32,
}
fn scheduled(
    key: String,
    round: usize,
    position: usize,
    first: Option<Uuid>,
    second: Option<Uuid>,
    bye: bool,
    results: &HashMap<String, StoredMatchResult>,
) -> ScheduledMatch {
    let stored = results.get(&key);
    let result = stored
        .and_then(|s| s.result.clone())
        .filter(|r| Some(r.first) == first && Some(r.second) == second);
    ScheduledMatch {
        key,
        round,
        position,
        first,
        second,
        bye,
        result,
        revision: stored.map_or(0, |s| s.revision),
    }
}
pub fn knockout_matches(
    draw: &CategoryDraw,
    results: &HashMap<String, StoredMatchResult>,
) -> Vec<ScheduledMatch> {
    let Some(slots) = draw.sections.first() else {
        return vec![];
    };
    if draw.format != CompetitionFormat::Knockout
        || slots.len() < 2
        || !slots.len().is_power_of_two()
    {
        return vec![];
    }
    knockout_from_slots(slots.iter().map(|id| (*id, true)).collect(), results)
}
pub fn knockout_from_slots(
    mut current: Vec<(Option<Uuid>, bool)>,
    results: &HashMap<String, StoredMatchResult>,
) -> Vec<ScheduledMatch> {
    if current.len() < 2 || !current.len().is_power_of_two() {
        return vec![];
    }
    let mut matches = Vec::new();
    let mut round = 0;
    while current.len() > 1 {
        let mut next = Vec::new();
        for (position, pair) in current.chunks_exact(2).enumerate() {
            let bye = pair[0].1 && pair[1].1 && (pair[0].0.is_none() != pair[1].0.is_none());
            let item = scheduled(
                format!("ko:{round}:{position}"),
                round,
                position,
                pair[0].0,
                pair[1].0,
                bye,
                results,
            );
            let winner = item.result.as_ref().map(|r| r.winner).or_else(|| {
                if bye {
                    item.first.or(item.second)
                } else {
                    None
                }
            });
            let settled = winner.is_some() || pair.iter().all(|p| p.1 && p.0.is_none());
            next.push((winner, settled));
            matches.push(item);
        }
        current = next;
        round += 1;
    }
    matches
}
// Circle method for only the requested round; no quadratic schedule allocation.
pub fn group_round_matches(
    draw: &CategoryDraw,
    group: usize,
    round: usize,
    results: &HashMap<String, StoredMatchResult>,
) -> Vec<ScheduledMatch> {
    let Some(section) = draw.sections.get(group) else {
        return vec![];
    };
    let ids: Vec<_> = section.iter().flatten().copied().collect();
    if ids.len() < 2 {
        return vec![];
    }
    let size = ids.len() + ids.len() % 2;
    if round >= size - 1 {
        return vec![];
    }
    let at = |index: usize| {
        let original = if index == 0 {
            0
        } else {
            1 + (index - 1 + size - 1 - round) % (size - 1)
        };
        ids.get(original).copied()
    };
    (0..size / 2)
        .filter_map(|position| {
            let (a, b) = (at(position)?, at(size - 1 - position)?);
            let (a, b) = if round % 2 == 1 { (b, a) } else { (a, b) };
            Some(scheduled(
                format!("group:{group}:{round}:{position}"),
                round,
                position,
                Some(a),
                Some(b),
                false,
                results,
            ))
        })
        .collect()
}
