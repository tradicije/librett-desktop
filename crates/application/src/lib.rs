//! Platform-independent application operations and repository contracts.
//!
//! Features live in separate modules; root re-exports preserve the public API
//! used by storage adapters and desktop commands.

mod backups;
mod cash;
mod categories;
mod draw;
mod error;
mod matches;
mod players;
mod registrations;
pub mod registry;
mod reports;
mod results;
mod scheduling;
mod tournaments;

pub use backups::BackupInfo;
pub use cash::{record_cash, CashRepository, ExpectedCashAmount, PlayerCashRepository};
pub use categories::{
    add_category, add_category_with_fee, create_category_with_rules, get_category_rules,
    save_category_rules, update_category_with_rules, update_category_with_rules_confirmed,
    CategoryConfiguration, CategoryEditorRepository, CategoryEditorState, CategoryRepository,
    CategoryRulesRepository,
};
pub use draw::{
    get_category_draw, preview_category_draw, save_category_draw, CompetitionState, DrawRepository,
    GroupOrderRequest, SaveFillersRequest,
};
pub use error::ApplicationError;
pub use matches::{MatchPage, SaveMatchRequest};
pub use players::{
    create_player, delete_player, save_player_checked, save_player_profile,
    GuardedPlayerRepository, PlayerRepository,
};
pub use registrations::{
    register_entries, register_entry, set_entry_status, set_player_attendance,
    RegistrationRepository,
};
pub use reports::CategoryReportData;
pub use results::{CategoryResults, CompletionRequest, CompletionState, TournamentProgress};
pub use scheduling::{ReadyMatch, ScheduleAction, ScheduleRequest, ScheduleState, TableAssignment};
pub use tournaments::{create_tournament, create_tournament_with_cover, TournamentRepository};
