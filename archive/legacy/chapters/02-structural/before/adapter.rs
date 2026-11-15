use structural::adapter::{GatewayError, LegacyGateway, PaymentError};

pub fn charge(amount: u64, gateway: LegacyGateway) -> Result<(), PaymentError> {
    if amount == 0 {
        return Err(PaymentError::InvalidAmount);
    }
    let cents = i64::try_from(amount).map_err(|_| PaymentError::InvalidAmount)?;
    gateway.debit_cents(cents).map_err(|error| match error {
        GatewayError::Rejected => PaymentError::Declined,
        GatewayError::Offline => PaymentError::Unavailable,
    })
}
