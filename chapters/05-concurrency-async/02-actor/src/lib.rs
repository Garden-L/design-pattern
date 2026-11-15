//! A single thread owns the counter; callers own only mailbox senders.
use std::{
    error::Error,
    fmt,
    sync::mpsc,
    thread::{self, JoinHandle},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActorError {
    Closed,
    Overflow,
}

impl fmt::Display for ActorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Closed => "counter actor is closed",
            Self::Overflow => "counter would overflow",
        })
    }
}
impl Error for ActorError {}

enum Message {
    Add(u64, mpsc::Sender<Result<u64, ActorError>>),
    Value(mpsc::Sender<u64>),
    Shutdown,
}

pub struct CounterActor {
    sender: mpsc::SyncSender<Message>,
    worker: Option<JoinHandle<u64>>,
}
#[derive(Clone)]
pub struct CounterHandle {
    sender: mpsc::SyncSender<Message>,
}

impl CounterActor {
    pub fn start(initial: u64) -> Self {
        let (sender, receiver) = mpsc::sync_channel(16);
        let worker = thread::spawn(move || {
            let mut value = initial;
            for message in receiver {
                match message {
                    Message::Add(amount, reply) => {
                        let result = value.checked_add(amount).ok_or(ActorError::Overflow);
                        if let Ok(next) = result {
                            value = next;
                        }
                        let _ = reply.send(result);
                    }
                    Message::Value(reply) => {
                        let _ = reply.send(value);
                    }
                    Message::Shutdown => break,
                }
            }
            value
        });
        Self {
            sender,
            worker: Some(worker),
        }
    }
    pub fn handle(&self) -> CounterHandle {
        CounterHandle {
            sender: self.sender.clone(),
        }
    }
    /// Processes messages ahead of Shutdown, joins, then invalidates all handles.
    /// Racing requests behind Shutdown return Closed (no reply), not success.
    pub fn shutdown(mut self) -> u64 {
        self.finish().expect("counter actor worker panicked")
    }
    fn finish(&mut self) -> thread::Result<u64> {
        let worker = self.worker.take().expect("actor joined once");
        let _ = self.sender.send(Message::Shutdown);
        worker.join()
    }
}

impl Drop for CounterActor {
    fn drop(&mut self) {
        if self.worker.is_some() {
            let _ = self.finish();
        }
    }
}

impl CounterHandle {
    pub fn add(&self, amount: u64) -> Result<u64, ActorError> {
        let (reply, response) = mpsc::channel();
        self.sender
            .send(Message::Add(amount, reply))
            .map_err(|_| ActorError::Closed)?;
        response.recv().map_err(|_| ActorError::Closed)?
    }
    pub fn value(&self) -> Result<u64, ActorError> {
        let (reply, response) = mpsc::channel();
        self.sender
            .send(Message::Value(reply))
            .map_err(|_| ActorError::Closed)?;
        response.recv().map_err(|_| ActorError::Closed)
    }
}
