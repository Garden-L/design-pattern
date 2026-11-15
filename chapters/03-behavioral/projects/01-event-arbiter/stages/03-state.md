# 03 — State: 단일 활성 이벤트 전환

활성 이벤트가 하나이므로 상태는 `Option<EventId>`로 표현한다. 선택 계산과
상태 전환을 분리하면 유지·교체·해제의 의미를 하나의 결과 타입으로 고정할
수 있다.

```text
None    → Some(A) : A 활성화
Some(A) → Some(A) : 변경 없음
Some(A) → Some(B) : A 비활성화, B 활성화
Some(A) → None    : A 비활성화
```

실습: `SelectionResult`의 네 전환을 각각 테스트하고, 같은 이벤트 유지 시
`changed == false`인지 확인한다.
