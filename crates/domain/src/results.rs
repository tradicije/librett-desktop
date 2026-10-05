use crate::{CategoryDraw, ScheduledMatch};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashSet};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlacementStage {
    Winner,
    Finalist,
    Knockout,
    Groups,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinalPlacement {
    pub entry_id: Uuid,
    pub place: usize,
    pub place_end: usize,
    pub stage: PlacementStage,
    pub round: Option<usize>,
}
/// Derive placements only from a complete knockout projection. Entries eliminated
/// in the same round share a range; no unplayed bronze match is invented.
pub fn final_placements(
    draw: &CategoryDraw,
    matches: &[ScheduledMatch],
    qualifying: &HashSet<Uuid>,
) -> Vec<FinalPlacement> {
    let Some(final_match) = matches.last() else {
        return vec![];
    };
    let Some(final_result) = &final_match.result else {
        return vec![];
    };
    if qualifying.len() < 2
        || matches.iter().filter(|m| m.result.is_some()).count() != qualifying.len() - 1
    {
        return vec![];
    }
    let loser = |r: &crate::MatchResult| {
        if r.winner == r.first {
            r.second
        } else {
            r.first
        }
    };
    let mut placements = vec![
        FinalPlacement {
            entry_id: final_result.winner,
            place: 1,
            place_end: 1,
            stage: PlacementStage::Winner,
            round: Some(final_match.round),
        },
        FinalPlacement {
            entry_id: loser(final_result),
            place: 2,
            place_end: 2,
            stage: PlacementStage::Finalist,
            round: Some(final_match.round),
        },
    ];
    let mut rounds: BTreeMap<usize, Vec<Uuid>> = BTreeMap::new();
    for m in matches.iter().filter(|m| m.round < final_match.round) {
        if let Some(result) = &m.result {
            rounds.entry(m.round).or_default().push(loser(result));
        }
    }
    let mut next = 3;
    for (round, ids) in rounds.into_iter().rev() {
        let end = next + ids.len() - 1;
        for id in ids {
            placements.push(FinalPlacement {
                entry_id: id,
                place: next,
                place_end: end,
                stage: PlacementStage::Knockout,
                round: Some(round),
            });
        }
        next = end + 1;
    }
    let non_qualifiers: Vec<_> = draw
        .participants
        .iter()
        .filter(|e| !qualifying.contains(&e.id))
        .collect();
    let end = next + non_qualifiers.len().saturating_sub(1);
    for entry in non_qualifiers {
        placements.push(FinalPlacement {
            entry_id: entry.id,
            place: next,
            place_end: end,
            stage: PlacementStage::Groups,
            round: None,
        });
    }
    // A malformed projection must not publish duplicate or missing placements.
    let ids: HashSet<_> = placements.iter().map(|p| p.entry_id).collect();
    if ids.len() != placements.len() || ids != draw.participants.iter().map(|e| e.id).collect() {
        return vec![];
    }
    placements
}
