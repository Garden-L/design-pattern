# 러스트로 배우는 디자인 패턴

입문부터 고급까지 디자인 패턴을 러스트의 소유권, 트레잇, enum, 동시성 모델로 학습하는 책자형 강의 코드입니다.

## 시작하기

```bash
cargo test --workspace --all-targets
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
```

## 구성

- `chapters/00-foundations`: 러스트 설계 기초
- `chapters/01-creational`: 생성 패턴
- `chapters/02-structural`: 구조 패턴
- `chapters/03-behavioral`: 행동 패턴
- `chapters/04-rust-native`: 러스트 고유 설계
- `chapters/05-concurrency-async`: 동시성·비동기
- `chapters/06-projects`: 통합 프로젝트
- `docs/course`: 강의 지도, 패턴 비교표, 강사용 자료

각 패턴 챕터는 패턴별 디렉터리 안에 직접 구현 단계와 설계 적용 단계,
동일 계약 테스트, 독립 실습 프로젝트를 포함합니다.

## 검증 상태

2026-11-14 기준 전체 workspace 테스트, 포맷 검사, Clippy가 통과했습니다. 비동기 서버는 실행 중 취소와 drain/join 순서를 결정적 채널 동기화로 검증합니다.

