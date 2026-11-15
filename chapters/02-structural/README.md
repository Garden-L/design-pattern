# 02 — Structural patterns

객체와 타입의 연결 구조를 Rust의 trait, generic, newtype으로 표현합니다.

## 패턴 목록

| 패턴 | 강의 코드 | 연습 |
| --- | --- | --- |
| Adapter | [직접 구현 / 적용](01-adapter/) | [exercise](../../exercise/02-structural/01-adapter/) |
| Bridge | [직접 구현 / 적용](02-bridge/) | [exercise](../../exercise/02-structural/02-bridge/) |
| Composite | [직접 구현 / 적용](03-composite/) | [exercise](../../exercise/02-structural/03-composite/) |
| Decorator | [직접 구현 / 적용](04-decorator/) | [exercise](../../exercise/02-structural/04-decorator/) |
| Facade | [직접 구현 / 적용](05-facade/) | [exercise](../../exercise/02-structural/05-facade/) |
| Flyweight | [직접 구현 / 적용](06-flyweight/) | [exercise](../../exercise/02-structural/06-flyweight/) |
| Proxy | [직접 구현 / 적용](07-proxy/) | [exercise](../../exercise/02-structural/07-proxy/) |
| Newtype | [직접 구현 / 적용](08-newtype/) | [exercise](../../exercise/02-structural/08-newtype/) |

각 패턴 디렉터리는 `Cargo.toml`, `src/lib.rs`, `examples/01_direct.rs`,
`examples/02_*.rs`를 포함합니다.
실제 공개 라이브러리 API는 챕터 루트의 `src/`에 유지합니다.

## 실행

```sh
cargo +stable test -p structural
```

