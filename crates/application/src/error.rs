use librett_domain::DomainError;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ApplicationError {
    InvalidRegistry,
    RegistryConflict,
    RegistryFetch,
    AttendanceRequired,
    RegistrationInactive,
    CompetitionStarted,
    InvalidBackup,
    ScheduleConflict,
    PlayerBusy,
    TableBusy,
    CompetitionClosed,
    CompletionConflict,
    CompetitionIncomplete,
    BirthYearRequired,
    InvalidResult,
    MatchConflict,
    ResultImpact,
    PlayerConflict,
    CashConflict,
    InvalidRules,
    InvalidDraw,
    DrawConflict,
    InvalidCash,
    PlayerInUse,
    InvalidProfile,
    InvalidMembers,
    AlreadyRegistered,
    NameRequired,
    NameTooLong,
    DuplicateCategory,
    NotFound,
    Storage,
}

impl From<DomainError> for ApplicationError {
    fn from(error: DomainError) -> Self {
        match error {
            DomainError::InvalidResult => Self::InvalidResult,
            DomainError::InvalidRules => Self::InvalidRules,
            DomainError::InvalidDraw => Self::InvalidDraw,
            DomainError::InvalidCash => Self::InvalidCash,
            DomainError::InvalidProfile => Self::InvalidProfile,
            DomainError::InvalidMembers => Self::InvalidMembers,
            DomainError::NameRequired => Self::NameRequired,
            DomainError::NameTooLong => Self::NameTooLong,
            DomainError::DuplicateCategory => Self::DuplicateCategory,
        }
    }
}
