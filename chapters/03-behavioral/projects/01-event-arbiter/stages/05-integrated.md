# 05 — Integrated: EventArbiter

최종 구현은 다음 순서를 한 번의 `reconcile` 호출로 제공한다.

```text
후보 → 필터 Pipeline → Scorer Strategy → 결정적 선택
     → 이전 active와 비교 → SelectionResult 반환 및 Listener 통보
```

통합 테스트는 `A → B`, `A → None`, 동일 이벤트 유지, 동률 처리, 사용자
정의 필터와 scorer를 검증한다.

실행:

```sh
cargo +stable test -p event-arbiter
```
