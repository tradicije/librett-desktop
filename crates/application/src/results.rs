use uuid::Uuid;

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct CompletionState {
    pub revision: u32,
    pub completed_at: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CategoryResults {
    #[serde(default)]
    pub third_place: librett_domain::ThirdPlaceRule,
    pub category_id: Uuid,
    pub category_name: String,
    pub completion: CompletionState,
    pub tournament_completion: CompletionState,
    pub version: String,
    pub draw: Option<librett_domain::CategoryDraw>,
    pub ready: bool,
    pub blockers: Vec<String>,
    pub group_completed: usize,
    pub group_total: usize,
    pub knockout_completed: usize,
    pub knockout_total: usize,
    pub placements: Vec<librett_domain::FinalPlacement>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TournamentProgress {
    pub tournament_id: Uuid,
    pub completion: CompletionState,
    pub version: String,
    pub ready: bool,
    pub categories: Vec<CategoryResults>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CompletionRequest {
    pub request_id: Uuid,
    pub tournament_id: Uuid,
    pub category_id: Option<Uuid>,
    pub expected_revision: u32,
    pub expected_version: String,
    pub complete: bool,
}
