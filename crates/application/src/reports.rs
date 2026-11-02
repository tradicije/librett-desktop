use crate::{CategoryResults, CompetitionState};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CategoryReportData {
    pub results: CategoryResults,
    pub competition: CompetitionState,
    pub rules: librett_domain::CategoryRules,
}
