use chain_of_responsibility::{Event, EventTransition};

/// 패턴을 쓰지 않은 직접 구현. 조건이 늘어날수록 이 함수가 커진다.
pub fn run(candidates: Vec<Event>, active: Option<Event>) -> EventTransition {
    let selected = candidates
        .iter()
        .find(|event| event.urgent)
        .cloned()
        .or_else(|| candidates.into_iter().max_by_key(|event| event.priority));
    let previous = active;
    if previous == selected {
        return EventTransition {
            active: selected,
            deactivated: None,
            activated: None,
            changed: false,
        };
    }
    EventTransition {
        deactivated: previous,
        activated: selected.clone(),
        active: selected,
        changed: true,
    }
}

#[allow(dead_code)]
fn main() {
    let transition = run(
        vec![
            Event {
                id: "normal",
                priority: 10,
                urgent: false,
            },
            Event {
                id: "alarm",
                priority: 100,
                urgent: true,
            },
        ],
        None,
    );
    println!("active = {:?}", transition.active.map(|event| event.id));
}
