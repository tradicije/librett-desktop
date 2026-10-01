use serde::{Deserialize, Serialize};
use uuid::Uuid;

mod players;
pub use players::{Entry, EntryMember, Player, PlayerProfile};

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
    pub id: Uuid,
    pub name: String,
    pub discipline: Discipline,
    pub format: CompetitionFormat,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tournament {
    pub id: Uuid,
    pub name: String,
    pub categories: Vec<Category>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DomainError {
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
            .any(|c| c.name.to_lowercase() == name.to_lowercase())
        {
            return Err(DomainError::DuplicateCategory);
        }
        self.categories.push(Category {
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
