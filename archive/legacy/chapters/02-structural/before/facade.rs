use structural::facade::{InventoryError, OrderError, PaymentError};

pub fn place_order(sku: &str, quantity: u32, decline: bool) -> Result<u64, OrderError> {
    if quantity == 0 {
        return Err(OrderError::InvalidQuantity);
    }
    if sku != "book" {
        return Err(OrderError::Inventory(InventoryError::UnknownSku));
    }
    let total = 20_u64
        .checked_mul(u64::from(quantity))
        .ok_or(OrderError::Inventory(InventoryError::AmountOverflow))?;
    if decline {
        return Err(OrderError::Payment(PaymentError::Declined));
    }
    Ok(total)
}
