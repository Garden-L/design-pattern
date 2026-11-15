use mediator::ChatRoom;
pub fn conversation(messages: &[&str]) -> Vec<String> {
    let mut room = ChatRoom::default();
    room.register("alice");
    room.register("bob");
    for message in messages {
        room.send("alice", message).expect("registered sender");
    }
    room.inbox("bob").expect("registered recipient").to_vec()
}
#[allow(dead_code)]
pub fn main() {}
