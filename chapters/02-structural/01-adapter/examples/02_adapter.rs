use adapter::{LegacyAdapter, LegacyGateway, PaymentError, PaymentProcessor};

pub fn charge(amount: u64, gateway: LegacyGateway) -> Result<(), PaymentError> {
    LegacyAdapter::new(gateway).charge(amount)
}
#[allow(dead_code)]
pub fn main() {}
