use uuid::Uuid;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MatchPage {
    pub draw: Option<librett_domain::CategoryDraw>,
    pub rules: librett_domain::CategoryRules,
    pub rules_revision: u32,
    pub stale: bool,
    pub round_count: usize,
    pub total: usize,
    pub matches: Vec<librett_domain::ScheduledMatch>,
}

fn is_false(value: &bool) -> bool {
    !value
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SaveMatchRequest {
    #[serde(default, skip_serializing_if = "is_false")]
    pub allow_unconfirmed_start: bool,
    pub request_id: Uuid,
    pub tournament_id: Uuid,
    pub category_id: Uuid,
    pub draw_id: Uuid,
    pub rules_revision: u32,
    pub key: String,
    pub expected_revision: u32,
    pub result: librett_domain::MatchResult,
    pub invalidate_downstream: bool,
}
