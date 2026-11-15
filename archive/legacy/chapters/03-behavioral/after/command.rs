use behavioral::command::{Add, Command, CommandError, Context};

pub fn run(start: i32, amount: i32) -> Result<(i32, i32), CommandError> {
    let mut context = Context { balance: start };
    let mut command = Add::new(amount);
    command.execute(&mut context)?;
    let current = context.balance;
    command.undo(&mut context)?;
    Ok((current, context.balance))
}
