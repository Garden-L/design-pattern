# 강의 운영 안내

## 소스 코드 구조

각 챕터는 강의 코드와 학생 작업 코드를 분리합니다.

```text
chapters/*/  챕터별 강의 순서와 패턴별 직접 구현·설계 적용 코드
src/        챕터에서 재사용하는 실제 패턴 구현
exercise/*/ 학습자가 수정하는 starter.rs와 과제 설명
tests/      강의·연습 결과를 검증하는 통합 테스트
```

기존 `before/`, `after/`, 챕터 내부 `lessons/`, `exercises/`,
`exercise/`는 새 구조와 섞이지 않도록 `archive/legacy/`로 이동했습니다.
새 강의 자료에서는 `chapters/<chapter>/<pattern>/`와
`exercise/<chapter>/<pattern>/`의 대칭 구조만 사용합니다.

## 학습 경로

1. 러스트 설계 기초
2. 생성 패턴
3. 구조 패턴
4. 행동 패턴
5. 러스트 고유 설계 패턴
6. 동시성·비동기 패턴
7. 통합 프로젝트

## 대상·선수 지식

대상은 Rust 문법을 읽을 수 있는 입문자부터 객체지향 설계 경험이 있는 고급자까지다. 00장에서는 소유권·trait·제네릭·동적 디스패치·Result를 다시 설명하므로 다른 언어 경험자도 시작할 수 있다.

## 난이도와 결과

| 단계 | 난이도 | 학습 결과 | checkpoint |
| --- | --- | --- | --- |
| 00–01 | 입문 | Rust 경계와 생성 계약 설명 | Builder/Factory 테스트 통과 |
| 02–03 | 중급 | 구조·행동 변경을 분리 | Adapter/State/Command 실습 |
| 04–05 | 고급 | 타입·소유권·동시성으로 불변식 표현 | typestate/취소·join 검증 |
| 06 | 종합 | 여러 패턴의 선택 근거 설명 | 네 개 프로젝트 테스트 통과 |

## 챕터 운영

각 챕터는 문제 상황, 직접 구현, 변경 요구사항, 설계 적용, 동일 테스트,
트레이드오프, 실습, 강사용 질문 순서로 진행합니다.

## 확인 명령

```bash
cargo fmt --all -- --check
cargo test --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --doc
```

## 검증 결과

2026-11-15 기준 `cargo test --workspace --all-targets`는 통합 테스트를, `cargo test --workspace --doc`는 compile-fail doctest 4개를 별도로 검증한다. format check와 strict Clippy도 함께 실행한다.

상세 비교표는 `pattern-comparison-matrix.md`, 용어는 `glossary.md`, 수업 운영은 `instructor-guide.md`에서 확인한다.

## 알려진 교육용 한계

- 일부 직접 구현 예제는 교육 목적의 단순화된 구현입니다.
- Observer는 동기식이며, 일부 공유 상태 예제는 단일 스레드 범위를 사용합니다.
- 비동기 예제는 유한 작업과 명시적 종료를 전제로 합니다.
- 프로젝트는 외부 HTTP·플러그인 바이너리 로딩 대신 결정적 인프로세스 경계를 사용합니다.

