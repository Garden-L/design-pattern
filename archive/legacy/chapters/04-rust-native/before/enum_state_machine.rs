use rust_native::enum_state_machine::{Action, ConnectionError, ConnectionState};

// The caller owns the transition table and must repeat it elsewhere.
pub fn run(actions: &[Action]) -> Result<ConnectionState, ConnectionError> {
    let mut connected = false;
    for action in actions {
        match (connected, action) {
            (false, Action::Connect) => connected = true,
            (true, Action::Disconnect) => connected = false,
            (true, Action::Send) => (),
            (true, Action::Connect) => return Err(ConnectionError::AlreadyConnected),
            (false, Action::Send | Action::Disconnect) => {
                return Err(ConnectionError::NotConnected)
            }
        }
    }
    Ok(if connected {
        ConnectionState::Connected
    } else {
        ConnectionState::Disconnected
    })
}
