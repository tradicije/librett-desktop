use crate::categories::active_category;
use crate::{ApplicationError, CategoryRulesRepository, PlayerRepository, TournamentRepository};
use librett_domain::CompetitionFormat;
use uuid::Uuid;

pub trait DrawRepository {
    fn find_draw(
        &self,
        category_id: Uuid,
    ) -> Result<Option<librett_domain::CategoryDraw>, ApplicationError>;
    fn save_draw(
        &mut self,
        draw: librett_domain::CategoryDraw,
        expected_revision: u32,
        confirm_restart: bool,
    ) -> Result<librett_domain::CategoryDraw, ApplicationError>;
}

fn draw_entries(
    repository: &impl PlayerRepository,
    category_id: Uuid,
) -> Result<Vec<librett_domain::Entry>, ApplicationError> {
    let mut entries = repository.list_entries(category_id)?;
    for entry in &mut entries {
        for member in &mut entry.members {
            member.checked_in = false;
        }
    }
    Ok(entries)
}

pub fn get_category_draw(
    repository: &(impl TournamentRepository + DrawRepository),
    tournament_id: Uuid,
    category_id: Uuid,
) -> Result<Option<librett_domain::CategoryDraw>, ApplicationError> {
    active_category(repository, tournament_id, category_id)?;
    repository.find_draw(category_id)
}

pub fn preview_category_draw(
    repository: &(impl TournamentRepository + PlayerRepository + CategoryRulesRepository),
    tournament_id: Uuid,
    category_id: Uuid,
    mode: librett_domain::DrawMode,
    settings: librett_domain::DrawSettings,
    seeds: Vec<Uuid>,
) -> Result<librett_domain::CategoryDraw, ApplicationError> {
    let category = active_category(repository, tournament_id, category_id)?;
    let configured = repository.find_category_rules(category_id)?;
    if category.format == CompetitionFormat::GroupsKnockout
        && (settings.group_count != configured.rules.group_count
            || settings.qualifiers_per_group != configured.rules.qualifiers_per_group)
    {
        return Err(ApplicationError::InvalidRules);
    }
    let mut draw = librett_domain::create_draw(
        &category,
        &draw_entries(repository, category_id)?,
        mode,
        settings,
        seeds,
        Uuid::new_v4(),
    )?;
    if !configured.rules.allow_same_club {
        librett_domain::separate_group_clubs(&mut draw)?;
    }
    Ok(draw)
}

pub fn save_category_draw(
    repository: &mut (impl TournamentRepository
              + PlayerRepository
              + DrawRepository
              + CategoryRulesRepository),
    tournament_id: Uuid,
    mut draw: librett_domain::CategoryDraw,
    expected_revision: u32,
    confirm_restart: bool,
) -> Result<librett_domain::CategoryDraw, ApplicationError> {
    let category = active_category(repository, tournament_id, draw.category_id)?;
    let configured = repository.find_category_rules(draw.category_id)?;
    if category.format == CompetitionFormat::GroupsKnockout
        && (draw.settings.group_count != configured.rules.group_count
            || draw.settings.qualifiers_per_group != configured.rules.qualifiers_per_group)
    {
        return Err(ApplicationError::InvalidRules);
    }
    let entries = draw_entries(repository, draw.category_id)?;
    librett_domain::validate_draw(&draw, &category, &entries)?;
    // Names and clubs come from persisted entry snapshots, never from client text.
    draw.participants = entries
        .into_iter()
        .filter(|e| e.status == librett_domain::EntryStatus::Registered)
        .collect();
    repository.save_draw(draw, expected_revision, confirm_restart)
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CompetitionState {
    #[serde(default)]
    pub match_version: u64,
    #[serde(default)]
    pub filler_revision: u32,
    #[serde(default)]
    pub rules_revision: u32,
    #[serde(default)]
    pub fillers: std::collections::HashMap<usize, librett_domain::FillerChoice>,
    #[serde(default)]
    pub candidates: Vec<librett_domain::LuckyLoserCandidate>,
    pub draw_id: Option<Uuid>,
    pub stale: bool,
    pub groups: Vec<librett_domain::GroupStanding>,
    pub slots: Vec<librett_domain::QualificationSlot>,
    pub matches: Vec<librett_domain::ScheduledMatch>,
    pub order_revisions: Vec<u32>,
    pub result_versions: Vec<u64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GroupOrderRequest {
    pub request_id: Uuid,
    pub tournament_id: Uuid,
    pub category_id: Uuid,
    pub draw_id: Uuid,
    pub group: usize,
    pub expected_revision: u32,
    pub expected_result_version: u64,
    pub order: Option<Vec<Uuid>>,
    pub invalidate_downstream: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SaveFillersRequest {
    pub request_id: Uuid,
    pub tournament_id: Uuid,
    pub category_id: Uuid,
    pub draw_id: Uuid,
    pub expected_revision: u32,
    pub expected_match_version: u64,
    pub rules_revision: u32,
    pub order_revisions: Vec<u32>,
    pub result_versions: Vec<u64>,
    pub fillers: std::collections::HashMap<usize, librett_domain::FillerChoice>,
    pub invalidate_downstream: bool,
}
