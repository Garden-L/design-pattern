# 04 — Rust-native design

전통적인 패턴을 Rust의 enum, typestate, ownership, error type, macro로 다시 표현합니다.

## 패턴 목록

| 주제 | 강의 코드 | 연습 |
| --- | --- | --- |
| Enum state machine | [직접 구현 / 적용](01-enum-state-machine/) | [exercise](../../exercise/04-rust-native/01-enum-state-machine/) |
| Typestate | [직접 구현 / 적용](02-typestate/) | [exercise](../../exercise/04-rust-native/02-typestate/) |
| RAII | [직접 구현 / 적용](03-raii/) | [exercise](../../exercise/04-rust-native/03-raii/) |
| Interior mutability | [직접 구현 / 적용](04-interior-mutability/) | [exercise](../../exercise/04-rust-native/04-interior-mutability/) |
| Error design | [직접 구현 / 적용](05-error-design/) | [exercise](../../exercise/04-rust-native/05-error-design/) |
| Extension trait | [직접 구현 / 적용](06-extension-trait/) | [exercise](../../exercise/04-rust-native/06-extension-trait/) |
| Function composition | [직접 구현 / 적용](07-function-composition/) | [exercise](../../exercise/04-rust-native/07-function-composition/) |
| Macros | [직접 구현 / 적용](08-macros/) | [exercise](../../exercise/04-rust-native/08-macros/) |

## 실행

```sh
cargo +stable test -p rust-native
```

