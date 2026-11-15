// TODO: implement a recording test double and assert the observable calls.
#[derive(Default)]
pub struct RecordingSender {
    pub messages: Vec<String>,
}
