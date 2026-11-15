# 03 — Behavioral patterns

## Goals

Learn to exchange pricing rules, deliver events to subscribers, represent
undoable operations, check order transitions, compose request handlers,
traverse borrowed collections, and add operations over fixed element kinds.
Prerequisites: traits, generics, enums and `match`, `Result`, closures, borrowing,
`Rc`, `RefCell`, and basic iterator adapters. Read the
[chapter index](../../chapters/03-behavioral/README.md) for all source links.

The seven examples are independent lessons within an order-tool domain. They
use stable Rust and the standard library. Allow roughly two sessions of 75
minutes plus exercises; this is a teaching estimate, not a measured duration.

## 행동 패턴 프로젝트 목록

패턴별 예제를 끝낸 뒤에는 다음 프로젝트에서 여러 행동 패턴을 하나의
도메인 문제에 조합한다.

| 프로젝트 | 조합하는 패턴 | 핵심 질문 |
| --- | --- | --- |
| [단일 활성 이벤트 중재기](../../chapters/03-behavioral/projects/01-event-arbiter/README.md) | Strategy, Pipeline/Chain, State, Observer | 여러 후보 중 하나를 선택하고 활성화·비활성화를 어떻게 일관되게 통보할까? |

이 목록은 실전 프로젝트 장의 최종 과제가 아니라 행동 패턴을 이해하기
위한 별도 프로젝트 묶음이다. 이후 다른 패턴 프로젝트를 같은 목록에
추가할 수 있다.

## Problem

Both versions of each example must satisfy exactly the same baseline:

| Pattern | Inputs and required results |
| --- | --- |
| Strategy | `quote(subtotal, discounted)` returns the subtotal, or `subtotal - subtotal / 10`. Money is signed `i64`; integer division truncates toward zero. Thus 105 discounts to 95, -105 to -95, and zero stays zero. |
| Observer | For each input event string, emit `email:<event>` then `audit:<event>`, retaining event order. No events produce no deliveries. |
| Command | `run(start, amount)` returns the balance after checked addition and the restored starting balance after checked subtraction. Overflow is `CommandError::Overflow`; negative and zero amounts are allowed. |
| State | Begin in Draft. Place moves Draft to Placed, Ship moves Placed to Shipped. Every other transition returns `InvalidTransition`; empty input leaves Draft. |
| Chain | `private` is rejected, `public` yields `public data`, and every other key (including empty) is unhandled. |
| Iterator | Return every input integer once, in order; empty input yields an empty vector. |
| Visitor | Concatenate text and decimal `i32` values in encounter order, without separators. Empty elements produce an empty string. |

These are deliberately small contracts. A pattern refactors their structure;
it does not earn a different output or silently discard errors.

## Before

Read the seven direct examples in the numbered pattern directories:

- [Strategy](../../chapters/03-behavioral/01-strategy/examples/01_direct.rs) selects arithmetic
  with a boolean branch at the quote call site.
- [Observer](../../chapters/03-behavioral/02-observer/examples/01_direct.rs) hardcodes email
  and audit delivery inside the publishing loop.
- [Command](../../chapters/03-behavioral/03-command/examples/01_direct.rs) adds and reverses
  inline. The caller must keep both operations consistent.
- [State](../../chapters/03-behavioral/04-state/examples/01_direct.rs) matches state/action
  pairs in a caller-owned transition loop.
- [Chain](../../chapters/03-behavioral/05-chain-of-responsibility/examples/01_direct.rs) puts authorization and
  lookup branches in one function.
- [Iterator](../../chapters/03-behavioral/06-iterator/examples/01_direct.rs) keeps an index,
  bounds check, and increment in the traversal caller.
- [Visitor](../../chapters/03-behavioral/07-visitor/examples/01_direct.rs) matches each element
  to render it directly.

Each is a reasonable starting point for one fixed workflow. The test harness
imports these files as modules, so `cargo +stable test -p behavioral` compiles
and executes them along with the after examples.

## Failure

Use these concrete change requests to locate pressure before refactoring:

| Pattern | Change request | Where the before version must change |
| --- | --- | --- |
| Strategy | Add threshold pricing and use it in a second quote caller. | A boolean cannot express three policies. Each caller that copies the rule branches must update its arithmetic and selection together. |
| Observer | Add a metrics recipient and later remove a recipient. | The publisher needs another delivery statement and recipient-specific branching. Publishing knows every recipient. |
| Command | Queue an operation, undo it later, and reject double execution. | Separate arithmetic statements carry no record of which operation ran or how to reverse it. Callers must invent bookkeeping. |
| State | Add cancellation and reuse transitions in another caller. | The central match grows; copying that match risks different transition rules. |
| Chain | Insert a handler before lookup or reorder independently configured checks. | One branching function must be edited; its ordering is embedded in its control flow. |
| Iterator | Traverse strings, filter items, or stop early in another consumer. | The cursor bookkeeping spreads into every traversal and must be adapted to each consumer. |
| Visitor | Count text and number elements in addition to rendering them. | Each new operation repeats the element-dispatch match. |

These requests explain the design choice; the shipped before and after callers
still share the baseline contract. The tests also demonstrate caller-defined
pricing and a counting visitor without changes to the reusable types. The
exercises introduce further behavior explicitly rather than altering one side
of a comparison in secret.

## Refactor

1. Strategy: extract `PricingStrategy::price(&self, order: &Order) -> Money`.
   `Regular` and `TenPercentOff` own their rules. `price_with<S: PricingStrategy
   + ?Sized>` borrows a policy and works with both concrete types and `dyn`
   references. The after caller chooses `&dyn PricingStrategy` at runtime
   from its boolean input. Selection remains at the boundary; arithmetic moves
   into the policies. No heap allocation is needed for those borrowed policies.
2. Observer: register `Fn(&Event) + 'static` callbacks and return subscription
   IDs. The bus owns callbacks through `Rc`; `Cell` and `RefCell` permit
   registration and removal through `&self`. `publish(&self, event: Event)`
   clones callback handles into a snapshot and releases the subscriber borrow
   before invoking user code. Callbacks borrow the same event in registration
   order. A subscription change affects the next publication's snapshot,
   including a nested publication, but not the already captured delivery list.
3. Command: move checked addition and its inverse into `Add`. The
   `Command::execute(&mut self, context: &mut Context) -> Result<(), CommandError>`
   interface and `undo` let the operation retain whether it has executed.
   Calculate the next balance before updating either balance or replay state;
   a failed operation changes neither. Undo before execution and repeated undo
   are `NotExecuted`; a second execution is `AlreadyExecuted`. A successful
   undo allows execution again. The concrete example needs no boxed command.
4. State: put `place`, `ship`, and action dispatch on `OrderState`. A closed enum
   expresses Draft, Placed, and Shipped clearly. Each method returns `Result`,
   so invalid transitions stay visible. The after loop propagates errors and
   no longer duplicates the transition table. This is runtime state checking;
   typestate would use distinct types to prohibit some calls at compile time.
5. Chain: each `Handler::handle(&self, request: &Request) -> Result<Response,
   ChainError>` can answer, pass, or reject. Only `Err(Pass)` advances to the
   next handler. Any success or other error terminates immediately; an empty
   chain or all-pass chain returns `Unhandled`. `Chain` owns heterogeneous
   handlers as `Box<dyn Handler>` so its ordered list can vary at runtime.
   Authorization precedes lookup in the example.
6. Iterator: `Cursor<'a, T>` borrows a slice, owns its position, and implements
   `Iterator<Item = &'a T>`. `next` uses `get`, returning `None` at the end and
   every subsequent call. Its `FusedIterator` implementation promises this
   permanent exhaustion. The after caller composes `copied()` and `collect()`.
   Elements are borrowed, so the cursor cannot outlive the underlying slice.
7. Visitor: keep the closed `Element::{Text, Number}` set and move dispatch
   into `Element::accept`. A generic mutable `Visitor` receives a borrowed
   string or copied number. `Summary` accumulates output and `finish` consumes
   it. A second visitor counts kinds in tests, adding an operation without
   editing elements. Adding an element kind still requires updating `accept`
   and every visitor; the compiler exposes missing trait methods.

## After

Public implementations live in [src](../../chapters/03-behavioral/src/), with
the exact interface names from the Task 4 brief. The seven
[pattern callers](../../chapters/03-behavioral/) preserve the direct function
signatures. They respectively select a pricing policy, register recipients,
execute and undo `Add`, apply checked state transitions, assemble authorization
and lookup handlers, collect through `Cursor`, and visit elements with `Summary`.

No external crate, unsafe code, inheritance hierarchy, or asynchronous runtime
is needed. Dynamic dispatch is used for runtime policy selection and runtime
heterogeneous callback/handler collections. State uses an enum, traversal and
visitor dispatch use generics, and Command uses a concrete operation object.

## Comparison

This chapter's matrix supplies the seven behavioral entries for later course
aggregation. The global appendix belongs to the separate documentation task.

| Pattern | Before → after | Benefit under change | Cost / limit | Simpler Rust alternative and when to prefer it |
| --- | --- | --- | --- | --- |
| Strategy | Inline pricing branch → borrowed policy trait | Independent rules share one pricing caller | Extra types; runtime policy choice still branches somewhere | A function or enum match for a small fixed policy set |
| Observer | Fixed delivery statements → registered callbacks | Recipients can be added or removed independently of publishing | Callback ownership, runtime borrows, and a snapshot allocation per publish | Direct calls for two permanent recipients |
| Command | Addition and reversal by convention → operation with replay state | Execute/undo behavior travels with the operation | Must associate operation with the correct context; history ordering is a separate responsibility | A function for one immediate irreversible operation |
| State | Caller-owned pair match → enum methods | One authoritative transition policy | Runtime errors; the match still grows as workflow grows | A local match for one tiny caller; typestate when compile-time restrictions justify more types |
| Chain | One ordered branch function → ordered handlers | Reorder and compose independent steps | Boxed handlers and an explicit pass protocol | A function or generic tuple for a fixed pipeline |
| Iterator | Manual index loop → standard Iterator implementation | Consumers compose traversal with adapters | Custom cursor code to maintain | `slice.iter()` for ordinary slice traversal; the custom cursor here teaches the protocol |
| Visitor | Operation repeats element match → accept dispatch plus visitors | Add operations without modifying element kinds | Every visitor must change for a new element kind | Direct enum matches for few operations or frequently changing element kinds |

These are structural and qualitative comparisons; no benchmarks or speedup
claims are made. Avoid applying all seven together merely to demonstrate names.

Teaching boundaries:

- Money is signed integer units with no currency or validation policy. Division
  truncation is intentional; this is not a production financial model.
- Event delivery is synchronous and single-threaded. A panicking callback
  unwinds publication and prevents later callbacks from running. There is no
  error isolation, async scheduling, or backpressure. Avoid capturing a strong
  bus reference in its own callbacks; use `Weak` to avoid an ownership cycle.
  IDs are monotonic `u64`; exhausting their space panics before registering.
- `Add` tracks execution on that command object, not the identity of its
  context. Undo applies the inverse delta to the supplied context; it is not
  a snapshot restore, transaction, or multi-command history manager. Use the
  same receiver and a suitable undo order in real applications.
- `ChainError::Pass` is an internal continuation signal represented in the
  error enum for teaching simplicity. At the chain boundary, exhausting every
  handler becomes `Unhandled`, not `Pass`.

## Tests

Run from the workspace root:

```sh
cargo +stable test -p behavioral
cargo +stable test --workspace
cargo +stable test --workspace --all-targets
cargo +stable fmt --all -- --check
cargo +stable clippy --workspace --all-targets -- -D warnings
```

The [integration suite](../../chapters/03-behavioral/tests/behavioral.rs) runs
identical assertions through both versions of each baseline. It checks pricing
rounding, zero and signed values, empty events, exact recipient order, checked
addition bounds, undo results, every invalid order transition, chain outcomes,
empty collection handling, and text/number rendering.

Additional API tests exercise generic and dynamic policy substitution, a
caller-defined policy, removal and mutation during observer delivery, command
replay and failure atomicity, response and rejection short-circuiting, empty
and all-pass chains, repeated iterator exhaustion and adapters over non-Copy
elements, and a second visitor operation. Tests use bounded synchronous inputs
and require neither networking nor timing assumptions.

## Exercises

1. [Threshold pricing](../../exercise/03-behavioral/02-new-strategy/README.md):
   implement a third strategy and check the same fixtures with generic and
   dynamic callers. Keep the baseline quote comparisons unchanged.
2. [One-shot observer](../../exercise/03-behavioral/03-observer-unsubscribe/README.md):
   build on existing unsubscribe support, handle nested publication and stale
   snapshots, and avoid strong reference cycles.
3. [Cancellation](../../exercise/03-behavioral/01-new-state-transition/README.md):
   add a checked transition and extend both before and after sequence tests
   with identical acceptance criteria.

## Instructor notes

Session one: spend 10 minutes tracing the shared baseline fixtures, 20 minutes
on Strategy and runtime selection, 25 on Observer ownership and snapshots, and
20 on Command execute/undo and failure atomicity. Ask learners to predict the
observer log when the first callback removes the second. The second still
receives the current event because it is already in the snapshot.

Session two: use 20 minutes to enumerate the three-by-two state/action table,
20 to trace pass/answer/reject through a chain, 15 to repeatedly call `next`
after exhaustion, and 20 to add the counting visitor. These allocations are
suggestions. End by asking which example they would simplify in production.

Common misconceptions: moving a match does not remove a closed-world decision;
`RefCell` is not a thread synchronization primitive; a command's inverse is not
an arbitrary historical snapshot; implementing `Iterator` does not require
allocating each yielded item; Visitor makes new operations easier but new
element kinds harder. Ask why the authorization handler must precede lookup
and why failed command arithmetic must preserve replay state as well as balance.

Exercise guidance: threshold pricing needs one new trait implementation, with
the boundary at 200 tested explicitly. For one-shot observers, use a weak bus
reference and take a shared optional subscription ID before calling user code;
that guards against both recursion and invocation from older snapshots. For
cancellation, enumerate all state/action pairs first, then update the enum
methods and the independent before match. Check terminal states as carefully
as the happy path. Do not solve the exercises by removing existing tests.

## Summary

Behavioral patterns place policy, delivery, operations, transitions, traversal,
or dispatch behind a clear boundary. Rust permits each boundary to use the
smallest suitable tool: a function, enum, generic trait, or runtime trait object.
Review: Where does runtime choice occur? Who owns callbacks and commands?
When does a subscription change take effect? Which failures preserve state?
What becomes harder when Visitor gains a new element kind?
# 행동 패턴 확장

추가 강의: Template Method, Mediator, Memento, Interpreter.

- Template Method: 고정 순서와 변하는 hook을 분리한다.
- Mediator: 컴포넌트 간 직접 참조 대신 메시지 조정자를 둔다.
- Memento: 캡슐화를 유지한 상태 snapshot과 복원을 구현한다.
- Interpreter: 작은 DSL을 enum AST와 typed evaluator로 해석한다.

실행 코드는 각 `src/*.rs`와 `tests/missing_patterns.rs`에 있다.

