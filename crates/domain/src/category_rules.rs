use crate::{CompetitionFormat, DomainError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RankingCriterion {
    HeadToHead,
    SetRatio,
    PointRatio,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CategoryRules {
    pub group_count: usize,
    pub qualifiers_per_group: usize,
    pub best_of: u8,
    pub points_to_win: u8,
    pub win_by: u8,
    pub ranking: Vec<RankingCriterion>,
}
impl Default for CategoryRules {
    fn default() -> Self {
        Self {
            group_count: 2,
            qualifiers_per_group: 2,
            best_of: 5,
            points_to_win: 11,
            win_by: 2,
            ranking: vec![
                RankingCriterion::HeadToHead,
                RankingCriterion::SetRatio,
                RankingCriterion::PointRatio,
            ],
        }
    }
}
impl CategoryRules {
    pub fn validate(&self, format: CompetitionFormat) -> Result<(), DomainError> {
        if self.best_of == 0
            || self.best_of > 9
            || self.best_of % 2 == 0
            || !(1..=99).contains(&self.points_to_win)
            || !(1..=10).contains(&self.win_by)
            || self.ranking.len() != 3
            || [
                RankingCriterion::HeadToHead,
                RankingCriterion::SetRatio,
                RankingCriterion::PointRatio,
            ]
            .iter()
            .any(|criterion| self.ranking.iter().filter(|c| *c == criterion).count() != 1)
            || (format == CompetitionFormat::GroupsKnockout
                && (self.group_count == 0
                    || self.group_count > 2048
                    || self.qualifiers_per_group == 0
                    || self.qualifiers_per_group > 4096
                    || !(2..=4096)
                        .contains(&self.group_count.saturating_mul(self.qualifiers_per_group))))
        {
            return Err(DomainError::InvalidRules);
        }
        Ok(())
    }
}
