# 05. 책임 연쇄 패턴 연습

강의 코드의 문서 조회 처리기를 보지 않고, 같은 요구사항을 직접 구현한다.
이 연습의 목표는 코드를 복사하는 것이 아니라 **요구사항 → 처리기 계약 →
연쇄 실행 규칙**을 스스로 설계하는 것이다.

## 구현할 시스템

클라이언트는 문서 키를 가진 요청을 보낸다. 처리기는 등록된 순서대로 실행되며
각 처리기는 요청을 처리하거나 다음 처리기로 넘긴다.

## 요구사항

- [ ] `Request`는 문서 키를 소유한다.
- [ ] `Response`는 반환할 문서 내용을 소유한다.
- [ ] `Handler`는 요청을 받아 성공, 다음 처리기, 거부를 구분한다.
- [ ] `private` 요청은 인증 처리기에서 `Rejected`로 종료한다.
- [ ] `public` 요청은 조회 처리기에서 `public data`를 반환한다.
- [ ] 알 수 없는 키는 모든 처리기를 통과한 뒤 `Unhandled`가 된다.
- [ ] 성공 또는 거부가 나오면 이후 처리기를 실행하지 않는다.
- [ ] 처리기 목록은 런타임에 순서대로 구성할 수 있다.

## 사용 사례와 완료 조건

| 입력 | 기대 결과 | 확인할 점 |
|---|---|---|
| `public` | `Ok("public data")` | 인증 후 조회로 이동 |
| `private` | `Err(Rejected)` | 조회 처리기는 실행되지 않음 |
| `missing` | `Err(Unhandled)` | 모든 처리기를 통과 |

완료하려면 공개·비공개·미등록 요청과 조기 종료를 검증하는 테스트를 작성하고
모두 통과시킨다.

## 작업 순서

1. `src/lib.rs`에 `Request`, `Response`, 오류 타입을 정의한다.
2. `Handler`가 반환할 처리 결과를 설계한다.
3. 처리기를 순회하는 `Chain`을 구현한다.
4. 인증과 조회 처리기를 추가한다.
5. `tests/exercise.rs`의 테스트를 활성화하거나 테스트를 추가한다.
6. `AuditLog` 처리기를 추가해 새 처리기가 기존 연쇄에 연결되는지 확인한다.

## 학습자가 수정할 파일

- 구현: `src/lib.rs`
- 검증: `tests/exercise.rs`

`Cargo.toml`의 패키지 이름은 변경하지 않는다. 강의 정답 구현은
`chapters/03-behavioral/05-chain-of-responsibility/`에서 비교한다.

## 실행

````bash
cargo test -p exercise-03_behavioral-05_chain_of_responsibility
````

## 확장 과제

- `RateLimit` 처리기를 인증 전에 배치하고 순서의 의미를 테스트한다.
- `AuditLog`가 `Pass`, `Rejected`, `Unhandled` 결과를 모두 기록하게 한다.
- `Box<dyn Handler>` 대신 enum 기반 구현을 만들고 두 설계를 비교한다.

