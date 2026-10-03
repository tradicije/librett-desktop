use crate::{validated_name, Discipline, DomainError};
use base64::{engine::general_purpose::STANDARD, Engine};
use chrono::Datelike;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Player {
    pub id: Uuid,
    pub name: String,
    pub club: String,
    #[serde(flatten)]
    pub profile: PlayerProfile,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct PlayerProfile {
    pub birth_year: Option<u16>,
    pub city: String,
    pub country: String,
    pub email: String,
    pub phone: String,
    pub notes: String,
    pub photo: Option<String>,
}

impl PlayerProfile {
    pub fn validated(mut self) -> Result<Self, DomainError> {
        if self
            .birth_year
            .is_some_and(|year| !(1900..=chrono::Utc::now().year() as u16).contains(&year))
        {
            return Err(DomainError::InvalidProfile);
        }
        for field in [
            &mut self.city,
            &mut self.country,
            &mut self.email,
            &mut self.phone,
            &mut self.notes,
        ] {
            *field = field.trim().to_owned();
            if field.chars().count() > 2000 {
                return Err(DomainError::InvalidProfile);
            }
        }
        if let Some(photo) = &self.photo {
            validate_jpeg(photo, false)?;
        }
        Ok(self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntryMember {
    pub id: Uuid,
    pub name: String,
    pub club: String,
    #[serde(default)]
    pub checked_in: bool,
}

impl Player {
    pub fn new(name: &str, club: &str) -> Result<Self, DomainError> {
        Ok(Self {
            id: Uuid::new_v4(),
            profile: PlayerProfile::default(),
            name: validated_name(name)?,
            club: if club.trim().is_empty() {
                String::new()
            } else {
                validated_name(club)?
            },
        })
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntryStatus {
    #[default]
    Registered,
    Withdrawn,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Entry {
    pub id: Uuid,
    pub category_id: Uuid,
    #[serde(default)]
    pub status: EntryStatus,
    pub members: Vec<EntryMember>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn profiles_reject_future_years_external_photos_and_overlong_details() {
        let profile = PlayerProfile {
            birth_year: Some(chrono::Utc::now().year() as u16 + 1),
            ..Default::default()
        };
        assert_eq!(profile.validated(), Err(DomainError::InvalidProfile));
        let profile = PlayerProfile {
            photo: Some("https://example.test/photo.jpg".into()),
            ..Default::default()
        };
        assert_eq!(profile.validated(), Err(DomainError::InvalidProfile));
        let profile = PlayerProfile {
            photo: Some("data:image/jpeg;base64,aGVsbG8=".into()),
            ..Default::default()
        };
        assert_eq!(profile.validated(), Err(DomainError::InvalidProfile));
        let profile = PlayerProfile {
            notes: "ž".repeat(2001),
            ..Default::default()
        };
        assert_eq!(profile.validated(), Err(DomainError::InvalidProfile));
    }

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
            status: EntryStatus::Registered,
            members: members
                .into_iter()
                .map(|player| EntryMember {
                    id: player.id,
                    name: player.name,
                    club: player.club,
                    checked_in: false,
                })
                .collect(),
        })
    }
}

pub fn validate_jpeg(photo: &str, cover: bool) -> Result<(), DomainError> {
    if photo.len() > if cover { 1_000_000 } else { 350_000 } {
        return Err(DomainError::InvalidProfile);
    }
    let encoded = photo
        .strip_prefix("data:image/jpeg;base64,")
        .ok_or(DomainError::InvalidProfile)?;
    let bytes = STANDARD
        .decode(encoded)
        .map_err(|_| DomainError::InvalidProfile)?;
    if !bytes.starts_with(&[0xff, 0xd8, 0xff]) || !bytes.ends_with(&[0xff, 0xd9]) {
        return Err(DomainError::InvalidProfile);
    }
    let mut decoder = jpeg_decoder::Decoder::new(std::io::Cursor::new(bytes));
    decoder.set_max_decoding_buffer_size(if cover { 1024 * 768 * 4 } else { 512 * 512 * 4 });
    decoder
        .read_info()
        .map_err(|_| DomainError::InvalidProfile)?;
    let info = decoder.info().ok_or(DomainError::InvalidProfile)?;
    let (width, height) = (usize::from(info.width), usize::from(info.height));
    if width == 0
        || height == 0
        || if cover {
            width > 1024 || height > 768 || (width * 9 != height * 16 && width * 3 != height * 4)
        } else {
            width > 512 || height > 512
        }
    {
        return Err(DomainError::InvalidProfile);
    }
    decoder.decode().map_err(|_| DomainError::InvalidProfile)?;
    Ok(())
}
