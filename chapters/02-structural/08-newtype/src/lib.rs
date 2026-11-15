/// Distinct ID types prevent accidental interchange.
///
/// ```compile_fail
/// use structural::newtype::{OrderId, UserId};
/// fn order_owner(_user: UserId) {}
/// order_owner(OrderId::new(7));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UserId(u64);
impl UserId {
    pub fn new(raw: u64) -> Self {
        Self(raw)
    }
    pub fn from_raw(raw: u64) -> Self {
        Self(raw)
    }
    pub fn get(self) -> u64 {
        self.0
    }
    pub fn into_raw(self) -> u64 {
        self.0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OrderId(u64);
impl OrderId {
    pub fn new(raw: u64) -> Self {
        Self(raw)
    }
    pub fn from_raw(raw: u64) -> Self {
        Self(raw)
    }
    pub fn get(self) -> u64 {
        self.0
    }
    pub fn into_raw(self) -> u64 {
        self.0
    }
}
