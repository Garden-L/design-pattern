# 02 — Structural patterns

## Goals

Learners can adapt an external interface, wrap a service with behavior,
compose renderable nodes, provide a single order operation, cache reads, and
make IDs distinct types. Prerequisites: traits, generics, `Result`, `Box`,
`Rc<RefCell<_>>`, and checked numeric conversion.

## Problem

An order tool integrates a legacy payment gateway, formats service output,
renders nested content, coordinates inventory and payment, reads documents,
and passes user and order identifiers. Each concern starts with a direct
implementation. A new gateway, another wrapper, nested content, or reused
document introduces a reason to change its structure.

## Before

The numbered pattern directories in [`chapters/02-structural/`](../../chapters/02-structural/) use direct
calls: payment conversion at the call site, inline logging, manual string
joining, explicit order steps, repeated store reads, and raw `u64` IDs.
These versions are reasonable while there is one caller and one fixed shape.

## Failure

A second payment provider makes every caller repeat conversion and error
mapping. A second cross-cutting behavior interleaves with service logic.
Nested nodes require special-case rendering branches. Order callers must
remember validation and step order. Repeated document reads do unnecessary
work. Two `u64` IDs can be swapped silently by the compiler.

## Refactor

1. `PaymentProcessor` defines `charge(u64)`. `LegacyAdapter` checks zero and
   signed-range overflow, calls `debit_cents(i64)`, and maps gateway errors
   into `PaymentError`. No error category is silently turned into success.
2. `Logger<T: Service>` owns the wrapped service and shares an event sink.
   It records entry, delegates once, records success or error, and returns
   the original result. This example uses single-threaded `Rc<RefCell<_>>`;
   a multi-threaded service would need a different sink.
3. `Node::render` gives `Text` and `Group` one operation. `Group` owns boxed
   child nodes, allowing nested heterogeneous trees. An empty group renders
   an empty string.
4. `OrderFacade<I, P>` owns an `Inventory` and `Payment`. It validates the
   quantity, reserves inventory, charges, and returns a receipt. The
   `OrderError` variants preserve the stage and typed cause. This teaching
   example does not implement rollback if payment fails after reservation.
5. `CachedProxy<T>` keeps successful document results in a `RefCell<HashMap>`.
   A cache hit skips the store; failures remain visible and are retried.
   This is a single-threaded, no-expiry cache, suitable for the exercise.
6. `UserId` and `OrderId` have private `u64` fields. `new`, `from_raw`,
   `get`, and `into_raw` make boundary conversion explicit. The compiler
   rejects using an `OrderId` where a `UserId` is required.

## After

Each pattern is an independent Rust crate. Its public API is in `src/lib.rs`;
the direct implementation and pattern example are in
`examples/01_direct.rs` and `examples/02_<pattern>.rs`.
The important signatures are `PaymentProcessor::charge(&self, u64) ->
Result<(), PaymentError>`, `Service::execute(&self, &str) -> Result<String,
ServiceError>`, `Node::render(&self) -> String`, and
`OrderFacade::place_order(&self, OrderRequest) -> Result<OrderReceipt,
OrderError>`. The proxy also implements its wrapped `DocumentStore` trait.

## Comparison

The before and after examples produce the same observable results and typed
errors. Both Adapter examples accept a `LegacyGateway`, so the same online,
declining, or offline gateway can exercise each path; offline maps to
`PaymentError::Unavailable`. The after versions move repeated conversion,
logging, traversal, coordination, or caching to one place. The newtypes add no
hidden conversion.
The cost is extra types and, for the composite, boxed trait objects. The
examples make no unmeasured performance claim.

For a closed set of node kinds, an `enum` and `match` is often easier to
follow than `Box<dyn Node>`. For one service with one logging call, a plain
function is often clearer than `Logger<T>`. A plain function can also wrap
one fixed external API. Use the traits when callers truly need several
implementations or independent composition.

## Tests

Run the relevant pattern crate, such as `cargo test -p adapter`. The
integration suite checks adapter error
translation and amount bounds, decorator event order and failure return,
nested and empty composites, facade success and stage errors, proxy cache
hits and retried failures, and ID conversion. A compile-fail doc test proves
the two ID types cannot be interchanged. Run
`cargo clippy -p structural --all-targets -- -D warnings` for lint checks.

## Exercises

1. [Adapt another API](../../exercise/02-structural/03-second-external-api/README.md):
   keep the `PaymentProcessor` contract and map errors from a voucher API.
2. [Add a decorator](../../exercise/02-structural/02-second-decorator/README.md):
   compose a prefix wrapper with logging in both orders.
3. [Add a node](../../exercise/02-structural/01-new-composite-node/README.md):
   render a bracketed nested group and its empty case.

## Instructor notes

Begin by asking students to trace `charge(0)` and an amount larger than
`i64::MAX`. Then have them predict the decorator event sequence on error.
Draw the ownership tree of the composite and the facade's reserve-then-charge
sequence. Ask what must change if a failed charge needs reservation rollback.
For the proxy, ask why only successful reads are cached and what a stale
value would mean. Close with the `compile_fail` newtype example. The exercise
prompts specify concrete expected output and failure behavior; review each
against the existing integration tests.

## Summary

Structural patterns organize relationships among existing parts: adapting,
wrapping, nesting, coordinating, and mediating access. Newtypes use Rust's
type system to prevent identifier confusion. Review: Which boundary maps
errors? Which wrapper owns its inner service? Who owns composite children?
When would an enum or plain function be enough?
# 구조 패턴 확장

추가 강의: Bridge와 Flyweight.

- Bridge: 메시지 정책과 렌더러의 두 변화 축을 조합으로 분리한다.
- Flyweight: 공유 가능한 불변 스타일과 요청별 위치 상태를 분리한다.

실행 코드는 `src/bridge.rs`, `src/flyweight.rs` 및 `tests/missing_patterns.rs`에 있다.

