# 04 — Rust-native design

## Goals

Use enums for closed runtime states, typestate for required construction
steps, `Drop` for scope cleanup, `RefCell` for single-threaded shared access,
standard error sources, and extension traits for local APIs on foreign types.
Prerequisites: ownership, `Result`, generics, `Option`, traits, and borrowing.
See the [chapter index](../../chapters/04-rust-native/README.md) for source links.

## Problem

Six independent small contracts need to keep their behavior while changing
where invariants live. A connection starts disconnected; only a connected
connection can send or disconnect, and a second connect fails. A request
requires method and URL. A tracker must end with zero active scopes. An audit
log preserves entries in order. A count parser distinguishes empty input from
malformed numbers and retains the parse error. A slug helper joins whitespace
separated words with hyphens and lowercases them. The paired callers share
the same test fixtures and results.

## Before

The [direct callers](../../chapters/04-rust-native/) use a boolean and
local transition match, `Option` validation at construction, manual resource
accounting, a mutable vector, local parse-error mapping, and repeated slug
formatting. These are reasonable for a single local caller. They become
fragile when rules spread across call sites.

## Failure

If a second caller adds a connection action, it can forget one branch or use
different errors. If a request is constructed in several stages, every caller
must remember the final validation. An early return or panic can skip manual
cleanup. A helper that accepts only `&mut self` cannot be called through a
shared reference. Converting an error to a string discards its source.
Duplicated slug rules can diverge on whitespace or casing.

## Refactor

1. `ConnectionState` is a closed enum. `connect`, `send`, `disconnect`, and
   `apply` return `ConnectionError` for invalid runtime operations. Runtime
   state is appropriate because the action list is data supplied at runtime.
2. `RequestBuilder<MissingMethod, MissingUrl>` carries two marker types in
   `PhantomData`. Calling `method` or `url` consumes the builder and changes
   the corresponding marker to `Ready`. Only `RequestBuilder<Ready>` (whose
   second type argument defaults to `Ready`) implements `build`. Dynamic
   `Option` inputs still get checked at the boundary before typed building.
3. `Tracker::enter` returns a guard borrowing the tracker. `Drop` decrements
   the count on normal return, early return, or unwinding. The guard cannot
   outlive its tracker.
4. `AuditLog` uses `RefCell<Vec<String>>` so `record(&self, ...)` works through
   a shared reference. Each borrow is short. A conflicting runtime borrow
   panics; ordinary `&mut self` is preferable when shared access is unneeded.
5. `AppError` separates missing input, parse failures, and I/O failures.
   `From` supports `?`; `Error::source` exposes wrapped errors. Callers can
   inspect the cause instead of parsing display text.
6. `SlugExt` adds `slug()` to `str` within this crate. Callers import the
   trait explicitly. Its rule is intentionally narrow: whitespace splitting
   and Unicode lowercase, not URL-safe transliteration or punctuation removal.

## After

The reusable [APIs](../../chapters/04-rust-native/src/) hold invariants and
the pattern callers use them. No external
crate, unsafe code, heap allocation for a state object, or async runtime is
needed. The `RequestBuilder` doctests demonstrate three operations rejected
by the compiler: building with either missing field and setting the method
twice. Runtime tests demonstrate connection errors and error-source retention.

## Comparison

| Choice | Benefit | Cost or boundary | Prefer simpler form when |
| --- | --- | --- | --- |
| Enum transitions | One checked transition table | Invalid runtime actions remain possible | One small caller owns the only match |
| Typestate | Missing fields cannot reach `build` | More generic types; dynamic input still checked | Runtime configuration has many optional fields |
| RAII | Cleanup follows scope, including unwind | Guard lifetime constrains ownership | No resource or cleanup obligation exists |
| Interior mutability | Mutate behind `&self` | Borrow conflict moves to runtime; not thread safe | `&mut self` is available |
| Error enum | Distinct cases and retained causes | Explicit variants and conversions | One local error needs no public contract |
| Extension trait | Reuse a focused method on `str` | Trait import and possible name collision | One local function suffices |

Type safety pays off when an invariant is stable and many callers would
otherwise recheck it. Encoding every transient state as a type can make code
harder to read, especially when states are chosen from runtime data. Use enums
and `Result` for runtime workflows; reserve typestate for construction steps
whose valid sequence is fixed at compile time.

## Tests

Run `cargo +stable test -p rust-native` for paired fixtures, API assertions,
and compile-fail doctests. Run `cargo +stable fmt --all -- --check` and
`cargo +stable clippy --workspace --all-targets -- -D warnings` for the
workspace. Tests cover empty action sequences, invalid connection actions,
both builder field orders, missing inputs, scope cleanup on unwind, empty and
ordered audit logs, parse and I/O sources, and whitespace slug cases.

## Exercises

1. [Invalid transition](../../exercise/04-rust-native/01-invalid-transition/README.md):
   start from a failing send assertion, then fix the action sequence.
2. [Required fields](../../exercise/04-rust-native/03-typestate-fields/README.md):
   observe a compile failure and make both construction orders compile.
3. [Scope cleanup](../../exercise/04-rust-native/02-raii-cleanup/README.md):
   add an early return and compare normal return with unwinding.

## Instructor notes

Start with the shared fixtures and ask which invalid actions can only be
known at runtime. Have learners predict the compiler error for incomplete
builders and the active count after a panic. Contrast `RefCell`'s runtime
borrow checking with ordinary exclusive borrowing. For the error lesson,
inspect `source()` rather than matching an error string. Review whether each
refactor would be worthwhile for one fixed call site.

## Summary

Rust's enums, type parameters, ownership, `Drop`, standard error trait, and
extension traits cover many needs without a class hierarchy. Use the smallest
mechanism that makes the actual invariant clear. Which requirement is known
only at runtime? Which refactor would you remove if this code had one caller?
# 러스트 고유 설계 확장

함수 조합과 선언적 매크로를 별도 강의로 추가했다.

- 함수 조합: `Result::and_then`과 closure로 변환 파이프라인을 구성한다.
- 선언적 매크로: 반복 선언을 `macro_rules!` 입력과 확장 결과로 비교한다.

실행 코드는 `src/function_composition.rs`, `src/macros.rs`, `tests/composition_macros.rs`에 있다.

