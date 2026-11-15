//! A closed set of connection states with checked operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    Disconnected,
    Connected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Connect,
    Send,
    Disconnect,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionError {
    NotConnected,
    AlreadyConnected,
}

impl ConnectionState {
    pub fn connect(self) -> Result<Self, ConnectionError> {
        match self {
            Self::Disconnected => Ok(Self::Connected),
            Self::Connected => Err(ConnectionError::AlreadyConnected),
        }
    }
    pub fn send(self) -> Result<(), ConnectionError> {
        match self {
            Self::Connected => Ok(()),
            Self::Disconnected => Err(ConnectionError::NotConnected),
        }
    }
    pub fn disconnect(self) -> Result<Self, ConnectionError> {
        match self {
            Self::Connected => Ok(Self::Disconnected),
            Self::Disconnected => Err(ConnectionError::NotConnected),
        }
    }
    pub fn apply(self, action: Action) -> Result<Self, ConnectionError> {
        match action {
            Action::Connect => self.connect(),
            Action::Send => {
                self.send()?;
                Ok(self)
            }
            Action::Disconnect => self.disconnect(),
        }
    }
}
