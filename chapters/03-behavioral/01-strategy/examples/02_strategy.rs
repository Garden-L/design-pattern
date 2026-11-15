use strategy::{price_with, Order, PricingStrategy, Regular, TenPercentOff};

// Runtime selection stays at the boundary; the pricing caller has no rule branches.
pub fn quote(subtotal: i64, discounted: bool) -> i64 {
    let strategy: &dyn PricingStrategy = if discounted { &TenPercentOff } else { &Regular };
    price_with(strategy, &Order { subtotal })
}
#[allow(dead_code)]
pub fn main() {}
