//! Exchange pricing rules without changing the order or caller.
/// Signed integer units; the example imposes no currency or validation policy.
pub type Money = i64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Order {
    pub subtotal: Money,
}

pub trait PricingStrategy {
    fn price(&self, order: &Order) -> Money;
}

pub struct Regular;
impl PricingStrategy for Regular {
    fn price(&self, order: &Order) -> Money {
        order.subtotal
    }
}

/// Subtracts one tenth, with integer division truncating toward zero.
pub struct TenPercentOff;
impl PricingStrategy for TenPercentOff {
    fn price(&self, order: &Order) -> Money {
        order.subtotal - order.subtotal / 10
    }
}

pub fn price_with<S: PricingStrategy + ?Sized>(strategy: &S, order: &Order) -> Money {
    strategy.price(order)
}
