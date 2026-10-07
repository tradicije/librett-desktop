use serde::{Deserialize, Serialize};
use uuid::Uuid;

mod cash;
pub use cash::{
    cash_share, CashAllocation, CashBalance, CashKind, CashLedger, CashRecord, MAX_CASH_MINOR,
};
mod players;
pub use players::{Entry, EntryMember, EntryStatus, Player, PlayerProfile};
mod category_rules;
pub use category_rules::{CategoryRules, KnockoutFilling, RankingCriterion, ThirdPlaceRule};
mod standings;
pub use standings::{
    filled_qualification_slots, group_standings, lucky_loser_candidates, qualification_slots,
    FillerChoice, GroupStanding, LuckyLoserCandidate, QualificationSlot, StandingRow,
};
mod matches;
pub use matches::{
    append_bronze_match, group_round_matches, knockout_from_slots, knockout_matches, MatchOutcome,
    MatchResult, ScheduledMatch, SetScore, StoredMatchResult,
};
mod results;
pub use results::{final_placements, final_placements_with_rule, FinalPlacement, PlacementStage};
mod draw;
pub use draw::{
    club_key, create_draw, has_group_club_conflicts, separate_group_clubs, validate_draw,
    CategoryDraw, DrawMode, DrawSettings,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Discipline {
    Singles,
    Doubles,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompetitionFormat {
    Knockout,
    GroupsKnockout,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Category {
    #[serde(default)]
    pub completed: bool,
    #[serde(default)]
    pub archived: bool,
    #[serde(default)]
    pub fee_minor: i64,
    pub id: Uuid,
    pub name: String,
    pub discipline: Discipline,
    pub format: CompetitionFormat,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tournament {
    #[serde(default)]
    pub completed: bool,
    #[serde(default)]
    pub registered_count: usize,
    #[serde(default)]
    pub cover: Option<String>,
    pub id: Uuid,
    pub name: String,
    pub categories: Vec<Category>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DomainError {
    InvalidResult,
    InvalidRules,
    InvalidDraw,
    InvalidCash,
    InvalidProfile,
    InvalidMembers,
    NameRequired,
    NameTooLong,
    DuplicateCategory,
}

fn validated_name(name: &str) -> Result<String, DomainError> {
    let name = name.trim();
    if name.is_empty() {
        return Err(DomainError::NameRequired);
    }
    if name.chars().count() > 120 {
        return Err(DomainError::NameTooLong);
    }
    Ok(name.to_owned())
}

impl Tournament {
    pub fn new(name: &str) -> Result<Self, DomainError> {
        Ok(Self {
            completed: false,
            registered_count: 0,
            cover: None,
            id: Uuid::new_v4(),
            name: validated_name(name)?,
            categories: Vec::new(),
        })
    }

    pub fn add_category(
        &mut self,
        name: &str,
        discipline: Discipline,
        format: CompetitionFormat,
    ) -> Result<(), DomainError> {
        let name = validated_name(name)?;
        if self
            .categories
            .iter()
            .any(|c| c.discipline == discipline && c.name.to_lowercase() == name.to_lowercase())
        {
            return Err(DomainError::DuplicateCategory);
        }
        self.categories.push(Category {
            completed: false,
            archived: false,
            fee_minor: 0,
            id: Uuid::new_v4(),
            name,
            discipline,
            format,
        });
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_blank_and_overlong_names_but_preserves_serbian_text() {
        assert_eq!(Tournament::new(" \n "), Err(DomainError::NameRequired));
        assert_eq!(
            Tournament::new(&"ž".repeat(121)),
            Err(DomainError::NameTooLong)
        );
        assert_eq!(
            Tournament::new("  Bubušinac — prolećni kup  ")
                .unwrap()
                .name,
            "Bubušinac — prolećni kup"
        );
        assert!(Tournament::new(&"ž".repeat(120)).is_ok());
    }

    #[test]
    fn category_formats_are_independent_and_duplicate_names_are_rejected() {
        let mut tournament = Tournament::new("Kup").unwrap();
        tournament
            .add_category(
                "Apsolutna",
                Discipline::Singles,
                CompetitionFormat::GroupsKnockout,
            )
            .unwrap();
        tournament
            .add_category("Dubl", Discipline::Doubles, CompetitionFormat::Knockout)
            .unwrap();
        assert_eq!(tournament.categories.len(), 2);
        assert_eq!(
            tournament.add_category(
                " APSOLUTNA ",
                Discipline::Singles,
                CompetitionFormat::Knockout
            ),
            Err(DomainError::DuplicateCategory)
        );
        assert_eq!(tournament.categories.len(), 2);
    }
}

pub use players::validate_jpeg;
