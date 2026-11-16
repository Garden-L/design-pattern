# 05. 책임 연쇄 패턴 연습 — 단일 활성 이벤트 관리자

강의 구현을 보지 않고 이벤트 판단기를 연쇄로 연결하는 `EventManager`를 직접
구현한다. 목표는 이벤트 후보가 여러 개여도 활성 상태는 하나만 유지하는 것이다.

## 요구사항

- [ ] `Event`는 `id`, `priority`, `urgent`를 가진다.
- [ ] `EventRequest`는 후보 이벤트 목록을 가진다.
- [ ] `EventDecision::Continue`는 다음 처리기로 전달된다.
- [ ] `EventDecision::Activate(event)`는 즉시 연쇄를 종료한다.
- [ ] `EventManager`는 `Option<Event>`로 활성 이벤트를 하나만 보관한다.
- [ ] 새 이벤트가 활성화되면 이전 이벤트와 새 이벤트를 전환 결과로 보고한다.
- [ ] 후보가 없으면 기존 이벤트를 해제한다.
- [ ] 같은 이벤트를 다시 선택하면 중복 전환을 만들지 않는다.

## 구현 순서

1. `Event`, `EventRequest`, `EventDecision`을 정의한다.
2. `EventHandler` trait와 처리기 목록을 정의한다.
3. `EventManager::process`에서 `Continue`와 `Activate`를 분기한다.
4. `active_event`와 `EventTransition`을 구현한다.
5. 긴급 처리기와 우선순위 처리기를 작성한다.
6. 조기 종료·교체·해제 테스트를 활성화하고 통과시킨다.

## 실행

````bash
cargo test -p exercise-03_behavioral-05_chain_of_responsibility
````

정답 구현은 `chapters/03-behavioral/05-chain-of-responsibility/`에서 비교한다.
