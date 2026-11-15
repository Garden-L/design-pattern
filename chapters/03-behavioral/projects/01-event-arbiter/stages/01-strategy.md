# 01 — Strategy: 점수 계산 정책 분리

이벤트를 통과시킨 뒤 점수를 계산하는 책임을 `EventScorer`로 분리한다.

```rust
pub trait EventScorer {
    fn score(&self, event: &Event, context: &Context) -> i32;
}

pub struct PriorityScorer;

impl EventScorer for PriorityScorer {
    fn score(&self, event: &Event, _: &Context) -> i32 {
        event.priority
    }
}
```

실습: `DistanceScorer`를 추가하고 동일한 후보 집합에서 선택 결과가 어떻게
달라지는지 비교한다.
