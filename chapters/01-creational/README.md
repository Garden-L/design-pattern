# 01 — Creational patterns

생성 시점의 선택, 검증, 복제, 의존성 주입을 공부합니다.

## 패턴 목록

| 패턴 | 강의 코드 | 연습 |
| --- | --- | --- |
| Factory Method | [직접 구현 / 적용](01-factory-method/) | [exercise](../../exercise/01-creational/01-factory-method/) |
| Abstract Factory | [직접 구현 / 적용](02-abstract-factory/) | [exercise](../../exercise/01-creational/02-abstract-factory/) |
| Builder | [직접 구현 / 적용](03-builder/) | [exercise](../../exercise/01-creational/03-builder/) |
| Prototype | [직접 구현 / 적용](04-prototype/) | [exercise](../../exercise/01-creational/04-prototype/) |
| Object Pool | [직접 구현 / 적용](05-object-pool/) | [exercise](../../exercise/01-creational/05-object-pool/) |
| Dependency Injection | [직접 구현 / 적용](06-dependency-injection/) | [exercise](../../exercise/01-creational/06-dependency-injection/) |
| Singleton alternatives | [직접 구현 / 적용](07-singleton-alternatives/) | [exercise](../../exercise/01-creational/07-singleton-alternatives/) |

각 패턴 디렉터리는 `Cargo.toml`, `src/lib.rs`, `examples/01_direct.rs`,
`examples/02_*.rs`를 포함합니다.
실제 공개 라이브러리 API는 챕터 루트의 `src/`에 유지합니다.

## 실행

```sh
cargo +stable test -p creational
```

