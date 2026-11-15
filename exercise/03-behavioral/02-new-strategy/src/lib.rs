// TODO: implement a third strategy and compare it with the existing policies.
pub trait Strategy<Input> {
    type Output;
    fn apply(&self, input: Input) -> Self::Output;
}
