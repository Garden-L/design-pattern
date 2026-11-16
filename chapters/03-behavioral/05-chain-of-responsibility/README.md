# 05. 책임 연쇄 패턴 — 단일 활성 이벤트 관리자

이 장에서는 여러 이벤트 판단기를 순서대로 연결하고, 최종적으로 **하나의 이벤트만
활성화**하는 `EventManager`를 만든다. 판단기는 계속 추가할 수 있으며, 특정 판단기가
즉시 이벤트를 선택하면 뒤의 판단기는 실행하지 않는다.

## 1. 학습 목표

- `EventHandler` 계약으로 이벤트 판단 알고리즘을 교체·추가한다.
- `Continue`와 `Activate`를 구분해 책임 연쇄의 조기 종료를 구현한다.
- `EventManager`의 상태를 `Option<Event>`로 모델링해 활성 이벤트가 항상 0개 또는
  1개임을 보장한다.
- 이벤트 교체 시 `deactivated`와 `activated`를 결과로 통보한다.
- 후보가 없을 때 기존 이벤트를 해제하고, 같은 이벤트가 다시 선택되면 중복 전환을
  만들지 않는다.
- 조건문 하나에 계속 정책을 추가하는 방식과 책임 연쇄의 차이를 설명한다.

## 2. 선수 지식

- `struct`, `enum`, `trait`, `impl`, `Option`, `Vec`
- `Box<dyn Trait>`와 동적 디스패치
- `Result`·`match`, 소유권과 `Clone`
- 이전 장: [04. State](../04-state/README.md)

## 3. 설계할 시스템

### 도메인

이벤트 매니저가 여러 후보를 받아 우선순위, 긴급성, 거리, 쿨다운 같은 정책을
순서대로 적용한다. 정책은 처리기(Handler)로 분리한다.

```text
EventRequest
    │
    ▼
EmergencyHandler ── Activate(alarm) ──┐
    │ Continue                         │ 즉시 종료
    ▼                                  │
PriorityHandler ─── Activate(task) ──┘
    │ Continue
    ▼
EventManager.active: Option<Event>  // 항상 하나 이하
```

### 핵심 상태 전이

```text
old = normal, new = alarm
        └─ deactivated(normal)
        └─ activated(alarm)
        └─ active = alarm
```

## 4. 기능 요구사항

- [ ] `Event`는 `id`, `priority`, `urgent`를 가진다.
- [ ] `EventRequest`는 한 번에 평가할 후보 목록을 가진다.
- [ ] `EventHandler`는 `Continue` 또는 `Activate(Event)`를 반환한다.
- [ ] `Continue`이면 다음 처리기로 넘어간다.
- [ ] `Activate(Event)`이면 이후 처리기는 실행하지 않는다.
- [ ] `EventManager`는 활성 이벤트를 최대 하나만 보관한다.
- [ ] 새 이벤트가 선택되면 이전 이벤트는 `deactivated`로 보고되고 새 이벤트는
  `activated`로 보고된다.
- [ ] 후보가 없으면 기존 활성 이벤트를 해제하고 `active`를 `None`으로 만든다.
- [ ] 같은 이벤트가 다시 선택되면 `changed == false`이고 중복 전환을 보고하지 않는다.
- [ ] 새 판단 알고리즘은 `EventHandler` 구현과 `push`만으로 추가할 수 있다.

## 5. 사용 사례

### 긴급 이벤트의 즉시 활성화

긴급 후보가 있으면 `EmergencyHandler`가 즉시 활성화한다. 우선순위 처리기는
실행되지 않는다. 이 규칙은 “특정 조건이면 바로 활성화하고 연쇄를 끝낸다”는
책임 연쇄의 핵심이다.

### 일반 이벤트의 우선순위 선택

긴급 후보가 없으면 첫 처리기는 `Continue`를 반환하고, `PriorityHandler`가 가장
높은 우선순위 후보 하나를 활성화한다.

### 활성 이벤트 교체

`normal`이 활성화된 상태에서 `alarm`이 선택되면 결과에는 두 사건이 모두 담긴다.
호출자는 UI 갱신, 사운드 중지·재생, 로그 기록을 이 결과로 처리할 수 있다.

### 이벤트 소멸

후보가 모두 사라지면 `deactivated`에 이전 이벤트가 담기고 `active`는 `None`이
된다. 매니저 내부에 오래된 이벤트가 남아 있지 않아야 한다.

## 6. 완료 조건

- [ ] 긴급 이벤트가 선택되면 뒤 처리기의 호출 횟수가 0이다.
- [ ] 매니저의 `active_event()`는 항상 `None` 또는 하나의 이벤트만 반환한다.
- [ ] 이벤트 교체 결과에 해제·활성화 이벤트가 모두 기록된다.
- [ ] 후보가 비면 기존 이벤트가 해제된다.
- [ ] 같은 이벤트 재평가 시 `changed`가 `false`다.
- [ ] `cargo test -p chain-of-responsibility --all-targets`가 통과한다.

## 7. 핵심 개념

| 개념 | 이 예제의 역할 |
|---|---|
| `EventHandler` | 정책 하나를 독립된 확장 지점으로 만든다 |
| `Continue` | 현재 정책이 결정하지 못했음을 다음 처리기에 전달한다 |
| `Activate` | 선택을 확정하고 연쇄를 조기 종료한다 |
| `Option<Event>` | 단일 활성 이벤트 불변식을 표현한다 |
| `EventTransition` | 해제·활성화·현재 상태를 외부에 통보한다 |

## 8. 단계별 실습

### `examples/01_direct.rs` — 패턴을 적용하지 않은 구현

조건문으로 긴급 이벤트를 먼저 찾고, 없으면 우선순위를 비교한다. 정책이 늘어날수록
한 함수가 커지고 테스트 경계가 흐려지는 것을 관찰한다.

```bash
cargo run -p chain-of-responsibility --example chain-of-responsibility-direct
```

### `examples/02_chain_of_responsibility.rs` — 판단 책임 분리

`EmergencyHandler`와 `PriorityHandler`를 연결한다. `EmergencyHandler`가 선택하면
`PriorityHandler`가 실행되지 않는지 확인한다. 이후 `CooldownHandler` 또는
`DistanceHandler`를 직접 추가한다.

```bash
cargo run -p chain-of-responsibility --example chain-of-responsibility-pattern
```

### 연습 패키지

`exercise/03-behavioral/05-chain-of-responsibility/`에서 `Event`, `EventRequest`,
`EventDecision`, `EventHandler`, `EventManager`, `EventTransition`을 직접 구현한다.

## 9. 과제

- `CooldownHandler`를 추가해 쿨다운 이벤트를 건너뛴다.
- 긴급 이벤트가 있으면 후속 처리기가 호출되지 않는 테스트를 작성한다.
- 이벤트 교체와 후보 소멸의 전환 결과를 검증한다.
- 판단기를 `Vec<Box<dyn EventHandler>>`에 추가하는 것만으로 확장되는지 확인한다.

## 10. 회고 질문

1. 모든 판단기를 실행해 점수를 합산해야 한다면 책임 연쇄 대신 어떤 패턴이 더 적합한가?
2. `Activate`와 `Continue`를 하나의 `Option<Event>`로 표현하면 무엇을 잃는가?
3. `active: Option<Event>`가 “하나만 활성”이라는 요구사항을 어떻게 드러내는가?
4. 이벤트 교체 시 해제를 먼저 통보해야 하는 이유는 무엇인가?

## 코드 연결

- 공개 API: `src/lib.rs`
- 직접 구현: `examples/01_direct.rs`
- 패턴 적용: `examples/02_chain_of_responsibility.rs`
- 단위 계약 테스트: `tests/event_manager.rs`
- 직접 구현 연습: `exercise/03-behavioral/05-chain-of-responsibility/`
- 확장 프로젝트: `chapters/03-behavioral/projects/01-event-arbiter/`
