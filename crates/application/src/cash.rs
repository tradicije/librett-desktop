use crate::ApplicationError;
use uuid::Uuid;

pub trait CashRepository {
    fn list_cash(
        &self,
        tournament_id: Uuid,
    ) -> Result<Vec<librett_domain::CashRecord>, ApplicationError>;
    fn record_cash(
        &mut self,
        tournament_id: Uuid,
        record: librett_domain::CashRecord,
    ) -> Result<(), ApplicationError>;
}

pub trait PlayerCashRepository {
    fn settle_player_cash_checked(
        &mut self,
        request_id: Uuid,
        tournament_id: Uuid,
        player_id: Uuid,
        entry_ids: Vec<Uuid>,
        paid: bool,
        expected: Vec<ExpectedCashAmount>,
    ) -> Result<(), ApplicationError>;
    fn cash_ledger(
        &self,
        tournament_id: Uuid,
    ) -> Result<librett_domain::CashLedger, ApplicationError>;
    fn settle_player_cash(
        &mut self,
        request_id: Uuid,
        tournament_id: Uuid,
        player_id: Uuid,
        entry_ids: Vec<Uuid>,
        paid: bool,
    ) -> Result<(), ApplicationError>;
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ExpectedCashAmount {
    pub entry_id: Uuid,
    pub amount_minor: i64,
}

pub fn record_cash(
    repository: &mut impl CashRepository,
    tournament_id: Uuid,
    mut record: librett_domain::CashRecord,
) -> Result<(), ApplicationError> {
    record.note = record.note.trim().to_owned();
    if !(1..=librett_domain::MAX_CASH_MINOR).contains(&record.amount_minor)
        || record.note.chars().count() > 500
    {
        return Err(ApplicationError::InvalidCash);
    }
    repository.record_cash(tournament_id, record)
}
