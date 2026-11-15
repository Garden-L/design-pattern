//! Closed workflow represented by an enum and checked transitions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderState {
    Draft,
    Placed,
    Shipped,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Place,
    Ship,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransitionError {
    InvalidTransition,
}

impl OrderState {
    pub fn place(self) -> Result<Self, TransitionError> {
        match self {
            Self::Draft => Ok(Self::Placed),
            _ => Err(TransitionError::InvalidTransition),
        }
    }
    pub fn ship(self) -> Result<Self, TransitionError> {
        match self {
            Self::Placed => Ok(Self::Shipped),
            _ => Err(TransitionError::InvalidTransition),
        }
    }
    pub fn apply(self, action: Action) -> Result<Self, TransitionError> {
        match action {
            Action::Place => self.place(),
            Action::Ship => self.ship(),
        }
    }
}
