use state::{Action, OrderState, TransitionError};

// Every new action adds branches to this central transition function.
pub fn run(actions: &[Action]) -> Result<OrderState, TransitionError> {
    let mut state = OrderState::Draft;
    for action in actions {
        state = match (state, action) {
            (OrderState::Draft, Action::Place) => OrderState::Placed,
            (OrderState::Placed, Action::Ship) => OrderState::Shipped,
            _ => return Err(TransitionError::InvalidTransition),
        };
    }
    Ok(state)
}
#[allow(dead_code)]
pub fn main() {}
