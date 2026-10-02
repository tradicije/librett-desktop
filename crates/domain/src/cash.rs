use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub const MAX_CASH_MINOR: i64 = 1_000_000_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CashKind {
    Charge,
    Discount,
    Payment,
    Refund,
}

impl CashKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Charge => "charge",
            Self::Discount => "discount",
            Self::Payment => "payment",
            Self::Refund => "refund",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CashRecord {
    pub id: Uuid,
    pub entry_id: Uuid,
    pub kind: CashKind,
    pub amount_minor: i64,
    pub note: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CashBalance {
    pub charges: i64,
    pub discounts: i64,
    pub payments: i64,
    pub refunds: i64,
}
impl CashBalance {
    pub fn apply(&mut self, kind: CashKind, amount: i64) -> Result<(), crate::DomainError> {
        let mut next = self.clone();
        if !(1..=MAX_CASH_MINOR).contains(&amount) {
            return Err(crate::DomainError::InvalidCash);
        }
        let total = match kind {
            CashKind::Charge => &mut next.charges,
            CashKind::Discount => &mut next.discounts,
            CashKind::Payment => &mut next.payments,
            CashKind::Refund => &mut next.refunds,
        };
        *total = total
            .checked_add(amount)
            .filter(|v| *v <= MAX_CASH_MINOR)
            .ok_or(crate::DomainError::InvalidCash)?;
        if next.discounts > next.charges || next.refunds > next.payments {
            return Err(crate::DomainError::InvalidCash);
        }
        *self = next;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exact_partial_payments_discounts_refunds_and_limits() {
        let mut b = CashBalance::default();
        b.apply(CashKind::Charge, 100_000).unwrap();
        b.apply(CashKind::Discount, 10_000).unwrap();
        b.apply(CashKind::Payment, 40_001).unwrap();
        assert_eq!(b.charges - b.discounts - b.payments + b.refunds, 49_999);
        let before = b.clone();
        for (kind, amount) in [
            (CashKind::Refund, 40_002),
            (CashKind::Discount, 90_001),
            (CashKind::Payment, 0),
            (CashKind::Charge, i64::MAX),
        ] {
            assert!(b.apply(kind, amount).is_err());
            assert_eq!(b, before);
        }
        b.apply(CashKind::Payment, 60_000).unwrap();
        b.apply(CashKind::Refund, 10_001).unwrap();
        assert_eq!(b.charges - b.discounts - b.payments + b.refunds, 0);
    }
}
