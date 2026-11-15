# 03 — Behavioral patterns

행동의 교체, 전달, 상태 전환, 순회와 협력을 공부합니다.

## 패턴 목록

| 패턴 | 강의 코드 | 연습 |
| --- | --- | --- |
| Strategy | [직접 구현 / 적용](01-strategy/) | [exercise](../../exercise/03-behavioral/01-strategy/) |
| Observer | [직접 구현 / 적용](02-observer/) | [exercise](../../exercise/03-behavioral/02-observer/) |
| Command | [직접 구현 / 적용](03-command/) | [exercise](../../exercise/03-behavioral/03-command/) |
| State | [직접 구현 / 적용](04-state/) | [exercise](../../exercise/03-behavioral/04-state/) |
| Chain of Responsibility | [직접 구현 / 적용](05-chain-of-responsibility/) | [exercise](../../exercise/03-behavioral/05-chain-of-responsibility/) |
| Iterator | [직접 구현 / 적용](06-iterator/) | [exercise](../../exercise/03-behavioral/06-iterator/) |
| Visitor | [직접 구현 / 적용](07-visitor/) | [exercise](../../exercise/03-behavioral/07-visitor/) |
| Template Method | [직접 구현 / 적용](08-template-method/) | [exercise](../../exercise/03-behavioral/08-template-method/) |
| Mediator | [직접 구현 / 적용](09-mediator/) | [exercise](../../exercise/03-behavioral/09-mediator/) |
| Memento | [직접 구현 / 적용](10-memento/) | [exercise](../../exercise/03-behavioral/10-memento/) |
| Interpreter | [직접 구현 / 적용](11-interpreter/) | [exercise](../../exercise/03-behavioral/11-interpreter/) |

## 행동 패턴 프로젝트

[단일 활성 이벤트 중재기](projects/01-event-arbiter/README.md)는 Strategy,
Chain, State, Observer를 하나의 도메인에 통합합니다.

## 실행

```sh
cargo +stable test -p behavioral
```

