use behavioral::state::{Action, OrderState, TransitionError};

pub fn run(actions: &[Action]) -> Result<OrderState, TransitionError> {
    let mut state = OrderState::Draft;
    for action in actions {
        state = state.apply(*action)?;
    }
    Ok(state)
}
