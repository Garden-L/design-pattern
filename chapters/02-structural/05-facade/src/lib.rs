#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrderRequest {
    pub sku: String,
    pub quantity: u32,
}
impl OrderRequest {
    pub fn new(sku: impl Into<String>, quantity: u32) -> Self {
        Self {
            sku: sku.into(),
            quantity,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrderReceipt {
    pub total_cents: u64,
}
impl OrderReceipt {
    pub fn new(total_cents: u64) -> Self {
        Self { total_cents }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InventoryError {
    UnknownSku,
    OutOfStock,
    AmountOverflow,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaymentError {
    Declined,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderError {
    InvalidQuantity,
    Inventory(InventoryError),
    Payment(PaymentError),
}

pub trait Inventory {
    fn reserve(&self, sku: &str, quantity: u32) -> Result<u64, InventoryError>;
}
pub trait Payment {
    fn charge(&self, cents: u64) -> Result<(), PaymentError>;
}

pub struct Catalog {
    price_cents: u64,
}
impl Catalog {
    pub fn new(price_cents: u64) -> Self {
        Self { price_cents }
    }
}
impl Inventory for Catalog {
    fn reserve(&self, sku: &str, quantity: u32) -> Result<u64, InventoryError> {
        if sku != "book" {
            return Err(InventoryError::UnknownSku);
        }
        self.price_cents
            .checked_mul(u64::from(quantity))
            .ok_or(InventoryError::AmountOverflow)
    }
}
pub struct CardGateway {
    decline: bool,
}
impl CardGateway {
    pub fn new(decline: bool) -> Self {
        Self { decline }
    }
}
impl Payment for CardGateway {
    fn charge(&self, _cents: u64) -> Result<(), PaymentError> {
        if self.decline {
            Err(PaymentError::Declined)
        } else {
            Ok(())
        }
    }
}

pub struct OrderFacade<I: Inventory, P: Payment> {
    inventory: I,
    payment: P,
}
impl<I: Inventory, P: Payment> OrderFacade<I, P> {
    pub fn new(inventory: I, payment: P) -> Self {
        Self { inventory, payment }
    }
    pub fn place_order(&self, request: OrderRequest) -> Result<OrderReceipt, OrderError> {
        if request.quantity == 0 {
            return Err(OrderError::InvalidQuantity);
        }
        let total = self
            .inventory
            .reserve(&request.sku, request.quantity)
            .map_err(OrderError::Inventory)?;
        self.payment.charge(total).map_err(OrderError::Payment)?;
        Ok(OrderReceipt::new(total))
    }
}
