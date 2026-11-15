/// The interface the ordering code expects.
pub trait PaymentProcessor {
    fn charge(&self, amount: u64) -> Result<(), PaymentError>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaymentError {
    InvalidAmount,
    Declined,
    Unavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GatewayError {
    Rejected,
    Offline,
}

/// A legacy API with a signed amount and different error vocabulary.
pub struct LegacyGateway {
    decline: bool,
    offline: bool,
}
impl LegacyGateway {
    pub fn new(decline: bool) -> Self {
        Self {
            decline,
            offline: false,
        }
    }
    pub fn offline() -> Self {
        Self {
            decline: false,
            offline: true,
        }
    }
    pub fn debit_cents(&self, cents: i64) -> Result<(), GatewayError> {
        if self.offline {
            Err(GatewayError::Offline)
        } else if self.decline || cents <= 0 {
            Err(GatewayError::Rejected)
        } else {
            Ok(())
        }
    }
}

pub struct LegacyAdapter<G> {
    gateway: G,
}
impl<G> LegacyAdapter<G> {
    pub fn new(gateway: G) -> Self {
        Self { gateway }
    }
}
impl PaymentProcessor for LegacyAdapter<LegacyGateway> {
    fn charge(&self, amount: u64) -> Result<(), PaymentError> {
        if amount == 0 {
            return Err(PaymentError::InvalidAmount);
        }
        let cents = i64::try_from(amount).map_err(|_| PaymentError::InvalidAmount)?;
        self.gateway
            .debit_cents(cents)
            .map_err(|error| match error {
                GatewayError::Rejected => PaymentError::Declined,
                GatewayError::Offline => PaymentError::Unavailable,
            })
    }
}
