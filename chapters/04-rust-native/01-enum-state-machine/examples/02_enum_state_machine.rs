use enum_state_machine::{Action, ConnectionError, ConnectionState};

pub fn run(actions: &[Action]) -> Result<ConnectionState, ConnectionError> {
    actions
        .iter()
        .try_fold(ConnectionState::Disconnected, |state, action| {
            state.apply(*action)
        })
}
#[allow(dead_code)]
pub fn main() {}
