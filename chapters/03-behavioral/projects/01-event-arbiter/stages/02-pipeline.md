# 02 — Pipeline: 후보 필터 연결

후보의 적합성 판단을 `EventFilter`로 나누고, 모든 필터를 통과한 후보만
점수 계산 단계로 보낸다.

```rust
pub trait EventFilter {
    fn accepts(&self, event: &Event, context: &Context) -> bool;
}

let filters: Vec<Box<dyn EventFilter>> = vec![
    Box::new(RejectCooldown),
    Box::new(MaxDistance::new(20)),
];
```

실습: `TimeWindowFilter`를 추가하고 필터 순서를 바꾸어도 각 후보의
적합성 결과가 예측 가능한지 테스트한다.
