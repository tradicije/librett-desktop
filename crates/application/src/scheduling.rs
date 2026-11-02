use uuid::Uuid;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ReadyMatch {
    pub category_id: Uuid,
    pub category_name: String,
    pub draw_id: Uuid,
    pub key: String,
    pub phase: String,
    pub round: usize,
    pub first: Uuid,
    pub second: Uuid,
    pub first_name: String,
    pub second_name: String,
    pub players: Vec<Uuid>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TableAssignment {
    pub table: usize,
    pub status: String,
    pub scheduled: ReadyMatch,
    pub started_at: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ScheduleState {
    pub tournament_id: Uuid,
    pub table_count: usize,
    pub version: String,
    pub assignments: Vec<TableAssignment>,
    pub waiting: Vec<ReadyMatch>,
    pub truncated: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ScheduleAction {
    Configure {
        table_count: usize,
    },
    Assign {
        draw_id: Uuid,
        key: String,
        table: usize,
    },
    Start {
        table: usize,
    },
    Clear {
        table: usize,
    },
    Remove {
        draw_id: Uuid,
        key: String,
    },
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ScheduleRequest {
    pub request_id: Uuid,
    pub tournament_id: Uuid,
    pub expected_version: String,
    pub action: ScheduleAction,
}
