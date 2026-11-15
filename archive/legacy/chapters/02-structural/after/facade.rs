use structural::facade::{CardGateway, Catalog, OrderError, OrderFacade, OrderRequest};

pub fn place_order(sku: &str, quantity: u32, decline: bool) -> Result<u64, OrderError> {
    OrderFacade::new(Catalog::new(20), CardGateway::new(decline))
        .place_order(OrderRequest::new(sku, quantity))
        .map(|receipt| receipt.total_cents)
}
