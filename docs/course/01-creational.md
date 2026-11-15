# 01 — Creational patterns

## Goals

After this chapter, learners can choose among a factory trait, a validating
builder, a boxed prototype, and constructor injection. Prerequisites: owned
`String`, borrowing `&str`, traits, `Box`, `Result`, and basic `Arc`.

## Problem

A course notification/report tool must format email and SMS messages, build
valid reports, duplicate report templates, and send messages through a
replaceable transport. Multiple services also need the same configuration.

## Before

The numbered pattern directories in [`chapters/01-creational/`](../../chapters/01-creational/) show
direct construction: separate email/SMS functions, a report function with
positional arguments, manual copying, a hard-coded send path, and a local
configuration value. These are reasonable for a single caller.

## Failure

A third channel duplicates another formatting branch. More optional or
required report fields make positional arguments hard to read. A caller that
needs copies through a trait object cannot call `Clone` directly on the trait
object. Hard-coded sending prevents a caller from swapping the transport or
recording its output in a test. Repeating configuration construction can
produce inconsistent values across services.

## Refactor

1. Introduce `Notification` and `NotificationFactory`; each factory owns the
   choice of product. The message becomes an owned `String` inside the product,
   so the returned box does not borrow the caller's temporary input.
2. Let `ReportBuilder` collect title and body through chainable setters.
   `build` validates both, returning `MissingTitle` or `MissingBody` for
   absent or whitespace-only values. It preserves nonblank input as supplied.
3. Implement `Prototype::clone_box` for the cloneable report template. Each
   result is an independently owned trait object.
4. Give `Notifier<S>` a sender in its constructor. Its `notify(&self, &str)`
   delegates to `NotificationSender::send`; the transport can be selected by
   the caller. A borrowed sender can be injected as well.
5. Pass `Arc<Config>` into services that share immutable configuration.
   Ownership and sharing are visible at construction sites.

## After

Each pattern is an independent Rust crate. Its public API lives in
`src/lib.rs`, the direct implementation is in `examples/01_direct.rs`,
and the pattern implementation is in `examples/02_<pattern>.rs`.
`NotificationFactory::create(&self, message: &str) -> Box<dyn Notification>`,
`ReportBuilder::new()` and `build() -> Result<Report, BuildError>`,
`Prototype::clone_box(&self) -> Box<dyn Prototype>`, and
`Notifier<S: NotificationSender>::notify(&self, message: &str)` are the
chapter's key signatures.

## Comparison

The integration tests assert equal notification text, report text and
validation outcomes for before and after. The factory, prototype, and
dependency injection variants add traits and indirection; they are useful
when callers genuinely need interchangeable products or senders. `Box<dyn
Trait>` allocates and uses dynamic dispatch. A closed set of products often
fits an `enum` and `match` better. A plain `Clone` suffices for concrete
templates. A simple constructor with named fields may suffice when no
validation or optional fields are needed. These are qualitative tradeoffs;
this chapter makes no performance claim.

Global mutable Singleton state is avoided because tests and unrelated
callers can affect one another through hidden shared state, and mutation
requires synchronization. Explicit `Arc<Config>` shows which services share
the same immutable configuration. `OnceLock` can initialize a truly
process-wide immutable value when that lifetime is required, but it is not
needed for this example.

## Tests

Run the relevant pattern crate, such as `cargo test -p factory-method`, from
the workspace root. The integration
suite covers both products, report rendering, missing and blank fields,
trait-object cloning, sender substitution, and shared configuration.

## Exercises

1. [Add a product](../../exercise/01-creational/01-new-product/README.md):
   implement push notifications and extend the shared test.
2. [Require an author](../../exercise/01-creational/02-required-field/README.md):
   add validation and matching before/after behavior.
3. [Inject a test double](../../exercise/01-creational/03-test-double/README.md):
   record two messages and check order.

## Instructor notes

Start with the email/SMS duplication, then ask when a third product justifies
a factory and when an enum stays clearer. Have learners trace where the
message changes from `&str` to `String`. For Builder, first ask them to
predict the error for a whitespace-only title. For Prototype, ask why
`Clone` is not directly object-safe. Close by replacing a real sender with
the recording double and discussing how explicit `Arc` sharing differs from
global mutation. The exercise solutions follow the new product, required
field, and recording sender criteria in their individual prompts.

## Summary

Creation patterns help when constructing a value has choices, validation,
copying, or dependency selection. Keep direct functions and concrete types
until those pressures exist. Review: Which factory calls allocate? Which
builder values are rejected? Who owns the injected sender? Why can a global
mutable configuration make tests order-dependent?
# 생성 패턴 확장

추가 강의: Abstract Factory와 Object Pool.

- Abstract Factory: 제품군의 조합 일관성을 trait factory로 보장한다.
- Object Pool: bounded capacity와 ownership 반환을 테스트한다.

실행 코드는 `src/abstract_factory.rs`, `src/object_pool.rs` 및 `tests/missing_patterns.rs`에 있다.

