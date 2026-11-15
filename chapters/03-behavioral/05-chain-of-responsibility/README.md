# 05. 책임 연쇄 패턴 — 순차 요청 처리기

이 장에서는 클라이언트 요청을 여러 처리기가 순서대로 검사하고, 처리할 수
있는 객체가 결과를 반환하도록 설계한다. 예제의 도메인은 문서 조회 요청이다.
인증, 조회, 이후 추가될 감사 로그·속도 제한·캐시 정책을 각각 독립된
처리기로 둘 수 있어야 한다.

## 1. 학습 목표

이 장을 끝낸 학습자는 다음을 할 수 있어야 한다.

- 여러 처리기가 하나의 요청 계약을 공유하도록 `trait Handler`를 정의한다.
- 현재 처리기가 요청을 처리하지 못했을 때 다음 처리기로 넘기는 흐름을 구현한다.
- 첫 번째 성공 또는 거부 결과에서 연쇄를 중단하고, 끝까지 처리되지 않은 요청을
  별도의 오류로 구분한다.
- 새 처리기를 추가할 때 호출자와 기존 처리기의 코드를 수정하지 않는다.
- 책임 연쇄가 단순한 `match` 또는 `if`보다 유리한 조건과 불필요한 조건을 설명한다.

## 2. 선수 지식

- Rust의 `struct`, `enum`, `trait`, `impl`
- `Result<T, E>`와 `match`
- `Vec`, `Box<dyn Trait>`, 소유권과 동적 디스패치
- 이전 장: [03. Command](../03-command/README.md)

## 3. 설계할 시스템

### 목표

클라이언트가 문서 키만 전달하면 요청 처리기가 정해진 순서로 요청을 검사한다.
각 처리기는 다음 중 하나를 선택한다.

1. 요청을 처리하고 응답한다.
2. 자신이 처리할 요청이 아니므로 다음 처리기로 넘긴다.
3. 처리할 수 있지만 정책상 거부한다.

### 범위

- 포함: 인증 검사, 문서 조회, 처리기 순서, 성공·거부·미처리 결과
- 포함: 새 처리기를 기존 연쇄에 추가하는 확장 지점
- 제외: 네트워크, 데이터베이스, 비동기 실행, 실제 사용자 인증

## 4. 기능 요구사항

- [ ] `Request`는 조회할 문서 키를 가진다.
- [ ] `Authorization` 처리기는 `private` 요청을 거부한다.
- [ ] `Lookup` 처리기는 `public` 요청에 `public data`를 반환한다.
- [ ] 처리기가 `Pass`를 반환하면 다음 처리기가 실행된다.
- [ ] 성공 응답이나 명시적 거부가 나오면 뒤의 처리기는 실행되지 않는다.
- [ ] 모든 처리기가 `Pass`를 반환하면 `Unhandled`를 반환한다.
- [ ] 처리기 순서를 바꾸어도 `Chain` 호출자의 코드는 바뀌지 않는다.
- [ ] 새 처리기를 추가할 때 `Handler` 구현과 `push` 호출만 추가하면 된다.

## 5. 사용 사례

### 사용 사례 A: 공개 문서 조회

- 시작 조건: 클라이언트가 `public` 문서를 요청한다.
- 입력: `Request("public")`
- 처리 흐름: `Authorization`은 `Pass` → `Lookup`은 응답 반환
- 기대 결과: `Ok("public data")`
- 실패 결과: 없음

### 사용 사례 B: 비공개 문서 요청

- 시작 조건: 클라이언트가 `private` 문서를 요청한다.
- 입력: `Request("private")`
- 처리 흐름: `Authorization`이 즉시 거부
- 기대 결과: `Err(Rejected)`
- 확인할 점: `Lookup`은 실행되지 않는다.

### 사용 사례 C: 알 수 없는 문서 요청

- 시작 조건: 등록되지 않은 문서 키가 들어온다.
- 입력: `Request("missing")`
- 처리 흐름: 모든 처리기가 `Pass`
- 기대 결과: `Err(Unhandled)`

### 사용 사례 D: 감사 로그 처리기 추가

- 변경 요청: 모든 요청을 기록하는 `AuditLog` 처리기를 추가한다.
- 기대 결과: 기존 `Authorization`, `Lookup`, `run`의 핵심 코드를 수정하지 않고
  연쇄에 처리기를 추가할 수 있다.

## 6. 완료 조건

- [ ] `cargo test -p chain-of-responsibility`가 통과한다.
- [ ] 공개 문서·비공개 문서·알 수 없는 문서의 결과가 각각 구분된다.
- [ ] `Pass` 이후 다음 처리기로 이동하고, 터미널 결과 이후 순회가 중단된다.
- [ ] 새 처리기 하나를 추가해도 `Handler` 계약만 구현하면 된다.
- [ ] 학습자가 직접 구현한 연습 코드의 테스트를 활성화해 통과시킨다.

## 7. 개념 설명

| 개념 | 해결하는 문제 | 처음 필요한 코드 | 주의할 점 |
|---|---|---|---|
| `Handler` trait | 처리기마다 다른 구현을 같은 방식으로 호출 | `src/lib.rs` | trait 자체가 책임 순서를 결정하지는 않는다 |
| `Pass` | 현재 처리기가 모르는 요청을 전달 | `Chain::handle` | `Pass`와 거부를 섞으면 흐름을 잃는다 |
| `Box<dyn Handler>` | 런타임에 처리기 목록 구성 | `Chain.handlers` | 동적 디스패치와 힙 할당 비용이 생긴다 |
| 첫 터미널 결과 | 여러 처리 결과 중 하나를 확정 | `Chain::handle` | 모든 처리기가 실행되어야 한다면 다른 설계가 필요하다 |

## 8. 단계별 실습

### `examples/01_direct.rs` — 단일 분기 함수로 요구사항 확인

- 학습 목적: 요구사항을 가장 작은 코드로 먼저 검증한다.
- 수정할 파일: `examples/01_direct.rs`
- 관찰할 문제: 인증과 조회 정책이 한 함수의 조건문에 결합된다.
- 실행 명령: `cargo run -p chain-of-responsibility --example chain-of-responsibility-direct`
- 체크포인트: `public`, `private`, `missing`의 결과를 설명할 수 있다.

### `examples/02_chain_of_responsibility.rs` — 책임을 처리기로 분리

- 이전 단계와 달라진 점: 조건문을 `Authorization`과 `Lookup`으로 분리한다.
- 수정할 파일: `src/lib.rs`, `examples/02_chain_of_responsibility.rs`
- 학습자 작업: `AuditLog` 또는 `RateLimit` 처리기를 하나 추가한다.
- 실행 명령: `cargo run -p chain-of-responsibility --example chain-of-responsibility-pattern`
- 기대 결과: 기존 `Chain` 사용 코드는 그대로 두고 처리기만 추가된다.
- 체크포인트: `Pass`, `Rejected`, `Unhandled`의 차이를 테스트로 고정한다.

### `exercise/03-behavioral/05-chain-of-responsibility/` — 직접 구현

- 학습 목적: 강의 구현을 보지 않고 동일한 계약을 설계한다.
- 수정할 파일: `src/lib.rs`, `tests/exercise.rs`
- 학습자 작업: `Request`, `Response`, `ChainError`, `Handler`, `Chain`을 구현한다.
- 실행 명령: `cargo test -p exercise-03_behavioral-05_chain_of_responsibility`
- 통과 기준: 공개·비공개·미등록 요청과 처리기 순서 테스트가 통과한다.

## 9. 과제

- 시작 파일: `exercise/03-behavioral/05-chain-of-responsibility/src/lib.rs`
- 구현할 요구사항: `AuditLog` 처리기를 추가해 모든 요청의 키를 기록한다.
- 변경하면 안 되는 파일: `Cargo.toml`의 패키지 이름과 테스트 실행 명령
- 테스트 명령: `cargo test -p exercise-03_behavioral-05_chain_of_responsibility`
- 제출 결과: 처리기 추가 전후의 결과가 같고, 로그 순서 테스트가 통과한다.
- 선택 확장: `RateLimit`, `FeatureFlag`, `Cache` 처리기를 추가하고 순서를 바꾸어
  결과 차이를 관찰한다.

## 10. 체크포인트

- [ ] `public`은 성공, `private`는 거부, `missing`은 미처리로 구분된다.
- [ ] 처리기 하나가 터미널 결과를 반환하면 다음 처리기가 실행되지 않는다.
- [ ] 처리기 추가가 `Chain::handle`의 수정으로 이어지지 않는다.
- [ ] 패턴을 제거하고 조건문으로 되돌렸을 때 변경 지점이 어디로 늘어나는지 설명한다.

## 11. 회고 질문

1. 처리기 순서가 바뀌면 결과가 달라지는가? 달라진다면 순서를 어디에서 관리해야 하는가?
2. `Pass`와 `Rejected`를 같은 오류 타입으로 표현할 때 생기는 문제는 무엇인가?
3. 처리기가 하나뿐이라면 이 구조는 과한가?
4. 모든 처리기가 실행되어야 하는 감사·관찰 요구사항에는 책임 연쇄가 적합한가?

## 12. 선택형 교육 모듈

- [x] 코드 스멜: 조건문과 정책 추가로 커지는 분기 함수
- [x] SOLID: 새 처리기 추가 시 기존 연쇄 코드를 수정하지 않는 확장
- [x] 리팩터링: 기존 세 가지 결과 계약을 보존하는 단계별 분리
- [x] 테스트 전략: 순서, 조기 종료, 빈 연쇄, 미처리 요청 검증

## 실행

````bash
cargo test -p chain-of-responsibility
cargo run -p chain-of-responsibility --example chain-of-responsibility-direct
cargo run -p chain-of-responsibility --example chain-of-responsibility-pattern
````

## 코드 연결

- 공개 API: `src/lib.rs`
- 직접 구현: `examples/01_direct.rs`
- 패턴 적용: `examples/02_chain_of_responsibility.rs`
- 연습 패키지: `exercise/03-behavioral/05-chain-of-responsibility/`
- 이벤트 프로젝트 연결: `chapters/03-behavioral/projects/01-event-arbiter/stages/02-pipeline.md`

