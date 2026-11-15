use structural::newtype::{OrderId, UserId};

pub fn describe(user_id: UserId, order_id: OrderId) -> String {
    format!("user {} ordered {}", user_id.get(), order_id.get())
}
