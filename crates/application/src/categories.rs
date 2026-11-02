use crate::{ApplicationError, TournamentRepository};
use librett_domain::{CompetitionFormat, Discipline, Tournament};
use uuid::Uuid;

pub trait CategoryRepository {
    fn delete_category(
        &mut self,
        tournament_id: Uuid,
        category_id: Uuid,
    ) -> Result<(), ApplicationError>;
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CategoryConfiguration {
    pub category_id: Uuid,
    pub revision: u32,
    pub rules: librett_domain::CategoryRules,
}

pub trait CategoryRulesRepository {
    fn save_configured_category_confirmed(
        &mut self,
        tournament_id: Uuid,
        category: &librett_domain::Category,
        rules: &librett_domain::CategoryRules,
        expected_revision: u32,
        _invalidate_downstream: bool,
    ) -> Result<CategoryConfiguration, ApplicationError> {
        self.save_configured_category(tournament_id, category, rules, expected_revision)
    }
    fn save_configured_category(
        &mut self,
        tournament_id: Uuid,
        category: &librett_domain::Category,
        rules: &librett_domain::CategoryRules,
        expected_revision: u32,
    ) -> Result<CategoryConfiguration, ApplicationError>;
    fn find_category_rules(
        &self,
        category_id: Uuid,
    ) -> Result<CategoryConfiguration, ApplicationError>;
    fn insert_configured_category(
        &mut self,
        tournament_id: Uuid,
        category: &librett_domain::Category,
        rules: &librett_domain::CategoryRules,
    ) -> Result<(), ApplicationError>;
    fn save_category_rules(
        &mut self,
        tournament_id: Uuid,
        category_id: Uuid,
        rules: &librett_domain::CategoryRules,
        expected_revision: u32,
    ) -> Result<CategoryConfiguration, ApplicationError>;
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CategoryEditorState {
    pub category: librett_domain::Category,
    pub configuration: CategoryConfiguration,
    pub used: bool,
}

pub trait CategoryEditorRepository {
    fn category_editor_state(
        &self,
        tournament_id: Uuid,
        category_id: Uuid,
    ) -> Result<CategoryEditorState, ApplicationError>;
}

pub(crate) fn active_category(
    repository: &impl TournamentRepository,
    tournament_id: Uuid,
    category_id: Uuid,
) -> Result<librett_domain::Category, ApplicationError> {
    repository
        .find(tournament_id)?
        .categories
        .into_iter()
        .find(|c| c.id == category_id && !c.archived)
        .ok_or(ApplicationError::NotFound)
}

pub fn add_category(
    repository: &mut impl TournamentRepository,
    tournament_id: Uuid,
    name: &str,
    discipline: Discipline,
    format: CompetitionFormat,
) -> Result<Tournament, ApplicationError> {
    let mut tournament = repository.find(tournament_id)?;
    tournament.add_category(name, discipline, format)?;
    let category = tournament
        .categories
        .last()
        .ok_or(ApplicationError::Storage)?;
    repository.insert_category(tournament_id, category)?;
    Ok(tournament)
}

pub fn add_category_with_fee(
    repository: &mut impl TournamentRepository,
    tournament_id: Uuid,
    name: &str,
    discipline: Discipline,
    format: CompetitionFormat,
    fee_minor: i64,
) -> Result<Tournament, ApplicationError> {
    if !(0..=librett_domain::MAX_CASH_MINOR).contains(&fee_minor) {
        return Err(ApplicationError::InvalidCash);
    }
    let mut tournament = repository.find(tournament_id)?;
    tournament.add_category(name, discipline, format)?;
    let category = tournament
        .categories
        .last_mut()
        .ok_or(ApplicationError::Storage)?;
    category.fee_minor = fee_minor;
    repository.insert_category(tournament_id, category)?;
    Ok(tournament)
}

pub fn create_category_with_rules(
    repository: &mut (impl TournamentRepository + CategoryRulesRepository),
    tournament_id: Uuid,
    category_id: Uuid,
    name: &str,
    discipline: Discipline,
    format: CompetitionFormat,
    fee_minor: i64,
    rules: librett_domain::CategoryRules,
) -> Result<Tournament, ApplicationError> {
    rules.validate(format)?;
    if !(0..=librett_domain::MAX_CASH_MINOR).contains(&fee_minor) {
        return Err(ApplicationError::InvalidCash);
    }
    let mut tournament = repository.find(tournament_id)?;
    if let Some(existing) = tournament.categories.iter().find(|c| c.id == category_id) {
        if existing.name != name.trim()
            || existing.discipline != discipline
            || existing.format != format
            || existing.fee_minor != fee_minor
            || existing.archived
            || repository.find_category_rules(category_id)?.rules != rules
        {
            return Err(ApplicationError::DrawConflict);
        }
        return Ok(tournament);
    }
    tournament.add_category(name, discipline, format)?;
    let category = tournament
        .categories
        .last_mut()
        .ok_or(ApplicationError::Storage)?;
    category.id = category_id;
    category.fee_minor = fee_minor;
    repository.insert_configured_category(tournament_id, category, &rules)?;
    Ok(tournament)
}

pub fn get_category_rules(
    repository: &(impl TournamentRepository + CategoryRulesRepository),
    tournament_id: Uuid,
    category_id: Uuid,
) -> Result<CategoryConfiguration, ApplicationError> {
    active_category(repository, tournament_id, category_id)?;
    repository.find_category_rules(category_id)
}

pub fn save_category_rules(
    repository: &mut (impl TournamentRepository + CategoryRulesRepository),
    tournament_id: Uuid,
    category_id: Uuid,
    rules: librett_domain::CategoryRules,
    expected_revision: u32,
) -> Result<CategoryConfiguration, ApplicationError> {
    let category = active_category(repository, tournament_id, category_id)?;
    rules.validate(category.format)?;
    repository.save_category_rules(tournament_id, category_id, &rules, expected_revision)
}

pub fn update_category_with_rules(
    repository: &mut (impl TournamentRepository + CategoryRulesRepository),
    tournament_id: Uuid,
    category_id: Uuid,
    name: &str,
    discipline: Discipline,
    format: CompetitionFormat,
    fee_minor: i64,
    rules: librett_domain::CategoryRules,
    expected_revision: u32,
) -> Result<Tournament, ApplicationError> {
    update_category_with_rules_confirmed(
        repository,
        tournament_id,
        category_id,
        name,
        discipline,
        format,
        fee_minor,
        rules,
        expected_revision,
        false,
    )
}

pub fn update_category_with_rules_confirmed(
    repository: &mut (impl TournamentRepository + CategoryRulesRepository),
    tournament_id: Uuid,
    category_id: Uuid,
    name: &str,
    discipline: Discipline,
    format: CompetitionFormat,
    fee_minor: i64,
    rules: librett_domain::CategoryRules,
    expected_revision: u32,
    invalidate_downstream: bool,
) -> Result<Tournament, ApplicationError> {
    rules.validate(format)?;
    if !(0..=librett_domain::MAX_CASH_MINOR).contains(&fee_minor) {
        return Err(ApplicationError::InvalidCash);
    }
    let tournament = repository.find(tournament_id)?;
    let existing = tournament
        .categories
        .iter()
        .find(|c| c.id == category_id && !c.archived)
        .ok_or(ApplicationError::NotFound)?;
    let mut validated = Tournament::new("category validation")?;
    validated.add_category(name, discipline, format)?;
    let mut category = validated.categories.remove(0);
    category.id = existing.id;
    category.fee_minor = fee_minor;
    if tournament.categories.iter().any(|other| {
        other.id != category_id
            && other.discipline == discipline
            && other.name.to_lowercase() == category.name.to_lowercase()
    }) {
        return Err(ApplicationError::DuplicateCategory);
    }
    repository.save_configured_category_confirmed(
        tournament_id,
        &category,
        &rules,
        expected_revision,
        invalidate_downstream,
    )?;
    repository.find(tournament_id)
}
