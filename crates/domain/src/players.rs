use crate::{validated_name, Discipline, DomainError};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Player {
    pub id: Uuid,
    pub name: String,
    pub club: String,
}

impl Player {
    pub fn new(name: &str, club: &str) -> Result<Self, DomainError> {
        Ok(Self {
            id: Uuid::new_v4(),
            name: validated_name(name)?,
            club: if club.trim().is_empty() {
                String::new()
            } else {
                validated_name(club)?
            },
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Entry {
    pub id: Uuid,
    pub category_id: Uuid,
    pub members: Vec<Player>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn doubles_require_two_distinct_players_and_singles_require_one() {
        let first = Player::new("Aleksa", "Bubušinac").unwrap();
        let second = Player::new("Marko", "").unwrap();
        let category = Uuid::new_v4();
        assert_eq!(
            Entry::new(category, Discipline::Doubles, vec![first.clone()]),
            Err(DomainError::InvalidMembers)
        );
        assert_eq!(
            Entry::new(
                category,
                Discipline::Doubles,
                vec![first.clone(), first.clone()]
            ),
            Err(DomainError::InvalidMembers)
        );
        assert_eq!(
            Entry::new(
                category,
                Discipline::Singles,
                vec![first.clone(), second.clone()]
            ),
            Err(DomainError::InvalidMembers)
        );
        assert!(Entry::new(category, Discipline::Doubles, vec![first, second]).is_ok());
    }
}

impl Entry {
    pub fn new(
        category_id: Uuid,
        discipline: Discipline,
        members: Vec<Player>,
    ) -> Result<Self, DomainError> {
        let expected = match discipline {
            Discipline::Singles => 1,
            Discipline::Doubles => 2,
        };
        if members.len() != expected || (members.len() == 2 && members[0].id == members[1].id) {
            return Err(DomainError::InvalidMembers);
        }
        Ok(Self {
            id: Uuid::new_v4(),
            category_id,
            members,
        })
    }
}
