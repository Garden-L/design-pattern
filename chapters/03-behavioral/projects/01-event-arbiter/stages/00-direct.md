# 00 — Before: 한 함수에 모든 정책 넣기

이 단계에서는 패턴을 적용하지 않는다. 필터, 점수 계산, 현재 상태 변경,
외부 통보가 한 함수에 섞여 있을 때의 변경 비용을 관찰한다.

```rust
fn select_event(events: &[Event], now: u64) -> Option<&Event> {
    let mut selected = None;
    for event in events {
        if event.cooldown || event.distance > 20 {
            continue;
        }
        if selected
            .map(|current: &Event| event.priority > current.priority)
            .unwrap_or(true)
        {
            selected = Some(event);
        }
    }
    let _ = now;
    selected
}
```

질문: 시간대 필터와 거리 중심 점수 계산을 추가할 때 어디를 수정해야 하는가?
