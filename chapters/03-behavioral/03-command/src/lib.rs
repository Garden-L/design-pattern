//! An operation object retains the information needed to undo its own effect.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Context {
    pub balance: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandError {
    AlreadyExecuted,
    NotExecuted,
    Overflow,
}

pub trait Command {
    fn execute(&mut self, context: &mut Context) -> Result<(), CommandError>;
    fn undo(&mut self, context: &mut Context) -> Result<(), CommandError>;
}

/// An inverse-delta command, intended for execute/undo on the same context.
/// This is not a balance snapshot or a multi-command history manager.
/// Failed arithmetic leaves both context and execution state unchanged.
pub struct Add {
    amount: i32,
    executed: bool,
}
impl Add {
    pub fn new(amount: i32) -> Self {
        Self {
            amount,
            executed: false,
        }
    }
}
impl Command for Add {
    fn execute(&mut self, context: &mut Context) -> Result<(), CommandError> {
        if self.executed {
            return Err(CommandError::AlreadyExecuted);
        }
        let next = context
            .balance
            .checked_add(self.amount)
            .ok_or(CommandError::Overflow)?;
        context.balance = next;
        self.executed = true;
        Ok(())
    }
    fn undo(&mut self, context: &mut Context) -> Result<(), CommandError> {
        if !self.executed {
            return Err(CommandError::NotExecuted);
        }
        let next = context
            .balance
            .checked_sub(self.amount)
            .ok_or(CommandError::Overflow)?;
        context.balance = next;
        self.executed = false;
        Ok(())
    }
}
