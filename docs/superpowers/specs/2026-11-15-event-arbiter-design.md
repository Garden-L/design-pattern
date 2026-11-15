# Event Arbiter 실전 프로젝트 설계

## 의도

행동 패턴 프로젝트의 예제로, 여러 후보 이벤트에 필터와 점수 계산 알고리즘을 순차 적용하고 최종적으로 단 하나의 이벤트만 활성화하는 `event-arbiter` 패키지를 추가한다. 기존 `06-projects/02-event-pipeline`의 코드와 학습 흐름은 변경하지 않는다.

## 목표

- `EventFilter`와 여러 필터 구현으로 후보 이벤트를 Pipeline/Chain 방식으로 검증한다.
- `EventScorer`와 조합 가능한 scorer로 선택 알고리즘을 Strategy로 교체한다.
- `EventArbiter`가 이전 active와 새 선택 결과를 reconcile한다.
- `SelectionResult`로 활성화·비활성화·변경 여부·선택 이유를 반환한다.
- `EventListener`와 `RecordingListener`로 상태 변경을 Observer 방식으로 통보한다.
- deterministic tie-break를 테스트로 고정한다.
- README에서 before/after, 패턴 역할, Mermaid 구조, 사용 시점, 실행법, 연습문제를 설명한다.

## 비목표

- 동시에 여러 이벤트를 활성화하는 멀티 액티브 모델
- 비동기 listener, 스레드 안전성, 외부 저장소 연동
- 기존 `event-pipeline` 패키지의 리팩터링

## 핵심 제약

- 활성 이벤트는 `Option<&'static str>` 하나만 보유한다.
- 이벤트 필터는 후보를 통과/거부하고, scorer는 점수만 계산한다.
- 최종 선택과 active 상태 변경은 `EventArbiter` 한 곳에서 수행한다.
- 같은 이벤트가 유지되면 `changed=false`이고 listener를 호출하지 않는다.
- A→B 전환은 `deactivated=A`, `active=B`이다.
- A→None은 `deactivated=A`이다.

## 선택한 접근

필터는 `Vec<Box<dyn EventFilter>>`로 순서 있게 적용하고, 점수 계산은 하나의 `Box<dyn EventScorer>`로 추상화한다. 여러 scorer가 필요할 때는 `CombinedScorer`가 내부 scorer의 점수를 합산한다. 이 구조는 새 필터·새 점수 알고리즘을 `EventArbiter` 수정 없이 추가할 수 있으며, 단일 active 상태의 전환 책임도 coordinator에 모은다.

선택 결과는 반환값과 Observer 통보를 함께 제공한다. 단, 상태가 실제로 바뀐 경우에만 listener를 호출해 중복 통보를 막는다.

## deterministic 선택 규칙

1. 총점이 높은 후보
2. 총점이 같으면 `priority`가 높은 후보
3. 그래도 같으면 `distance`가 짧은 후보
4. 그래도 같으면 `id`가 사전순으로 앞선 후보

