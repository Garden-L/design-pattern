# Exercise: fix an invalid connection transition

Start with this deliberately failing test in `tests/rust_native.rs`:

```rust
use rust_native::enum_state_machine::ConnectionState;
assert!(ConnectionState::Disconnected.send().is_ok()); // intentionally wrong
```

Run `cargo +stable test -p rust-native` and observe the failure. Fix the
*caller sequence* by connecting before sending; do not make `send` on a
disconnected state succeed. Add a second assertion that a second connect
returns `ConnectionError::AlreadyConnected`. The before and after runners
must keep the same expected results for the corresponding action sequences.
