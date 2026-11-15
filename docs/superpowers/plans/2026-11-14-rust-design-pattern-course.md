# Rust Design Pattern Course Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a self-study and instructor-led Rust design-pattern course with chapter-scoped executable code, before/after comparisons, exercises, and a Notion book hierarchy.

**Architecture:** The repository contains a Cargo workspace under `chapters/`, with one independently testable crate per teaching chapter. The Notion page `디자인 패턴` is the learner-facing book; category pages contain pattern subpages, while project pages link back to the relevant pattern pages and repository chapters.

**Tech Stack:** Stable Rust, Cargo workspace, Rust standard library first, `cargo test`, `cargo clippy`, optional Tokio only in the async chapter, Notion pages and code blocks.

**Spec:** `docs/superpowers/specs/2026-11-14-rust-design-pattern-course-design.md`

## Global Constraints

- 노션의 기존 루트 페이지 `디자인 패턴`을 유지하고 하위 페이지를 추가한다.
- 코드의 기준 언어는 안정 버전 Rust와 Cargo이다.
- 외부 라이브러리는 패턴 설명에 꼭 필요할 때만 사용하고, 핵심 개념은 표준 라이브러리로 먼저 보여준다.
- 적용 전/후 코드는 동일한 요구사항과 테스트로 비교한다.
- 성능 수치는 측정한 경우에만 기재하고, 측정하지 않은 주장은 정성적으로 표시한다.
- 한 번에 전체 완성본을 만들지 않고 기초 챕터부터 검증하면서 확장한다.

## Review Focus

- Empty input and zero-item collections must produce documented results rather than panic; test in the foundation and iterator chapters.
- Trait-object and generic implementations must preserve the same observable behavior; test in Strategy, Adapter, and Factory chapters.
- Shared mutable state must not introduce data races or deadlocks; test in the concurrency project.
- State transitions must reject invalid transitions or return typed errors; test in State and typestate chapters.
- Async cancellation and channel closure must terminate cleanly; test in the async chapter.

## File Map

- Create `Cargo.toml`: workspace members and shared lint/test settings.
- Create `chapters/00-foundations/`: Rust design foundations and testing conventions.
- Create `chapters/01-creational/`: Factory, Builder, Prototype, DI, and Singleton alternatives.
- Create `chapters/02-structural/`: Adapter, Decorator, Composite, Facade, Proxy, Newtype, and related examples.
- Create `chapters/03-behavioral/`: Strategy, Observer, Command, State, Chain, Iterator, Visitor, and related examples.
- Create `chapters/04-rust-native/`: enum state machines, typestate, RAII, interior mutability, error design, and macros.
- Create `chapters/05-concurrency-async/`: Send/Sync, channels, actor, work queue, Future, Stream, and cancellation.
- Create `chapters/06-projects/`: progressively integrated CLI, event pipeline, plugin, and async server projects.
- Create `docs/course/`: course map, chapter templates, instructor guide, glossary, and pattern comparison matrix.
- Modify the Notion page at the supplied URL by adding the root navigation and child pages after the first repository slice is verified.

### Task 1: Workspace foundation and chapter template

**Files:**
- Create: `Cargo.toml`
- Create: `chapters/00-foundations/Cargo.toml`
- Create: `chapters/00-foundations/src/lib.rs`
- Create: `chapters/00-foundations/tests/foundation.rs`
- Create: `chapters/00-foundations/README.md`
- Create: `docs/course/chapter-template.md`
- Test: `chapters/00-foundations/tests/foundation.rs`

**Interfaces:**
- Produces `pub fn summarize_items(items: &[&str]) -> String` and the chapter template used by all later crates.

- [ ] **Step 1: Write the failing foundation tests** for empty input, one item, and multiple items; assert exact output strings.
- [ ] **Step 2: Run `cargo test -p foundations`** and confirm failure because the function is absent.
- [ ] **Step 3: Implement `summarize_items`** with standard-library iteration and no panics for empty slices.
- [ ] **Step 4: Run `cargo test --workspace` and `cargo clippy --workspace --all-targets -- -D warnings`**; expect PASS.
- [ ] **Step 5: Write `README.md` and `chapter-template.md`** with the fixed order: goals, problem, before, failure, refactor, after, comparison, tests, exercises, instructor notes, summary.

### Task 2: Generating pattern chapters

**Files:**
- Create: `chapters/01-creational/src/factory_method.rs`
- Create: `chapters/01-creational/src/builder.rs`
- Create: `chapters/01-creational/src/prototype.rs`
- Create: `chapters/01-creational/src/dependency_injection.rs`
- Create: `chapters/01-creational/src/singleton_alternatives.rs`
- Create: matching `before/`, `after/`, `tests/`, `exercise/`, and `README.md` files under `chapters/01-creational/`
- Create: `docs/course/01-creational.md`

**Interfaces:**
- `factory_method::NotificationFactory::create(&self, message: &str) -> Box<dyn Notification>`.
- `builder::ReportBuilder::new() -> ReportBuilder`, chainable setters, and `build() -> Result<Report, BuildError>`.
- `prototype::Prototype::clone_box(&self) -> Box<dyn Prototype>`.
- `dependency_injection::Notifier<S: NotificationSender>` with `notify(&self, message: &str)`.

- [ ] **Step 1: Add before/after tests** proving the same notification/report behavior and builder validation behavior.
- [ ] **Step 2: Run targeted tests** and record the expected initial failures.
- [ ] **Step 3: Implement each after example** with the exact interfaces above; keep the before examples intentionally simple and duplicated.
- [ ] **Step 4: Add exercises** that require one new product type, one new required builder field, and one test double.
- [ ] **Step 5: Run `cargo test -p creational` and clippy**; update the chapter guide with tradeoffs, including why global mutable Singleton state is avoided.

### Task 3: Structural pattern chapters

**Files:**
- Create: `chapters/02-structural/src/adapter.rs`
- Create: `chapters/02-structural/src/decorator.rs`
- Create: `chapters/02-structural/src/composite.rs`
- Create: `chapters/02-structural/src/facade.rs`
- Create: `chapters/02-structural/src/proxy.rs`
- Create: `chapters/02-structural/src/newtype.rs`
- Create: matching chapter READMEs, before/after examples, tests, and exercises
- Create: `docs/course/02-structural.md`

**Interfaces:**
- `adapter::PaymentProcessor::charge(&self, amount: u64) -> Result<(), PaymentError>`.
- `decorator::Logger<T: Service>` implementing `Service`.
- `composite::Node::render(&self) -> String`.
- `facade::OrderFacade::place_order(&self, request: OrderRequest) -> Result<OrderReceipt, OrderError>`.
- `newtype::UserId(u64)` with explicit conversion functions and no implicit mixing with `OrderId`.

- [ ] **Step 1: Write behavior tests** for each interface, including adapter error translation, nested composite rendering, decorator ordering, and newtype type safety.
- [ ] **Step 2: Implement before examples** with direct calls and duplicated cross-cutting logic.
- [ ] **Step 3: Implement after examples** using the exact interfaces and preserve errors rather than hiding them.
- [ ] **Step 4: Add exercises** that introduce a second external API, a second decorator, and a new composite node.
- [ ] **Step 5: Run structural tests and clippy** and document when an enum or plain function is simpler than a trait hierarchy.

### Task 4: Behavioral pattern chapters

**Files:**
- Create: `chapters/03-behavioral/src/strategy.rs`
- Create: `chapters/03-behavioral/src/observer.rs`
- Create: `chapters/03-behavioral/src/command.rs`
- Create: `chapters/03-behavioral/src/state.rs`
- Create: `chapters/03-behavioral/src/chain.rs`
- Create: `chapters/03-behavioral/src/iterator.rs`
- Create: `chapters/03-behavioral/src/visitor.rs`
- Create: matching chapter READMEs, before/after examples, tests, and exercises
- Create: `docs/course/03-behavioral.md`

**Interfaces:**
- `strategy::PricingStrategy::price(&self, order: &Order) -> Money`.
- `observer::EventBus::subscribe(...)` and `publish(&self, event: Event)`.
- `command::Command::execute(&mut self, context: &mut Context) -> Result<(), CommandError>`.
- `state::OrderState` transitions returning typed states or `Result`.
- `chain::Handler::handle(&self, request: &Request) -> Result<Response, ChainError>`.

- [ ] **Step 1: Write tests** for strategy substitution, observer delivery, command undo, valid/invalid state transitions, chain short-circuiting, iterator exhaustion, and visitor coverage.
- [ ] **Step 2: Implement the minimal before versions** and verify the change request causes duplication or branching pressure.
- [ ] **Step 3: Implement after versions** with trait objects only where runtime substitution is needed; prefer enums and generics when they improve clarity.
- [ ] **Step 4: Add exercises** for a new strategy, an observer unsubscribe path, and a new state transition.
- [ ] **Step 5: Run behavioral tests and clippy** and write the pattern comparison matrix entries.

### Task 5: Rust-native design chapters

**Files:**
- Create: `chapters/04-rust-native/src/enum_state_machine.rs`
- Create: `chapters/04-rust-native/src/typestate.rs`
- Create: `chapters/04-rust-native/src/raii.rs`
- Create: `chapters/04-rust-native/src/interior_mutability.rs`
- Create: `chapters/04-rust-native/src/error_design.rs`
- Create: `chapters/04-rust-native/src/extension_trait.rs`
- Create: matching tests, exercises, and `docs/course/04-rust-native.md`

**Interfaces:**
- `enum_state_machine::ConnectionState` must make invalid operations return `ConnectionError`.
- `typestate::RequestBuilder<MissingMethod, MissingUrl>` transitions to `RequestBuilder<Ready>` only after required fields are present.
- `error_design::AppError` must preserve source errors with `source()`.

- [ ] **Step 1: Write compile-time and runtime tests** for typestate construction, enum transitions, error source preservation, and extension-trait behavior.
- [ ] **Step 2: Implement the minimal before versions** using runtime checks and duplicated helper logic.
- [ ] **Step 3: Implement Rust-native after versions** using enums, phantom types, RAII, and focused error enums.
- [ ] **Step 4: Add exercises** that intentionally attempt an invalid transition and require the learner to fix it.
- [ ] **Step 5: Run tests, doctests, and clippy**; document the boundary between useful type safety and over-engineering.

### Task 6: Concurrency and async chapters

**Files:**
- Create: `chapters/05-concurrency-async/Cargo.toml`
- Create: `chapters/05-concurrency-async/src/channel_pipeline.rs`
- Create: `chapters/05-concurrency-async/src/actor.rs`
- Create: `chapters/05-concurrency-async/src/work_queue.rs`
- Create: `chapters/05-concurrency-async/src/async_pipeline.rs`
- Create: matching tests, exercises, and `docs/course/05-concurrency-async.md`

**Interfaces:**
- `channel_pipeline::run_pipeline(input: Vec<Job>, workers: usize) -> Result<Vec<JobResult>, PipelineError>`.
- `work_queue::WorkQueue::submit(&self, job: Job) -> Result<(), QueueError>` and `shutdown(self) -> Vec<JobResult>`.
- `async_pipeline::run_async_pipeline(...) -> impl Future<Output = Result<...>>`.

- [ ] **Step 1: Write tests** for ordering guarantees, worker zero rejection, channel closure, clean shutdown, and cancellation.
- [ ] **Step 2: Implement thread/channel versions** with `Arc`, `Mutex` only where required and explicit shutdown signaling.
- [ ] **Step 3: Add the async crate and implement the smallest async pipeline** using Tokio only where async behavior cannot be shown with the standard library.
- [ ] **Step 4: Run deterministic tests** with bounded jobs and timeouts; ensure no test relies on sleeps for correctness.
- [ ] **Step 5: Document Send/Sync, backpressure, cancellation, and deadlock risks** and run the full workspace test suite.

### Task 7: Integrated projects

**Files:**
- Create: `chapters/06-projects/01-notification-cli/`
- Create: `chapters/06-projects/02-event-pipeline/`
- Create: `chapters/06-projects/03-plugin-system/`
- Create: `chapters/06-projects/04-async-server/`
- Create: `docs/course/06-projects.md`

- [ ] **Step 1: Implement the notification CLI** using Factory, Strategy, Builder, and Facade; add command-level tests.
- [ ] **Step 2: Implement the event pipeline** using Observer, Command, Chain, and channel-based Actor; add shutdown tests.
- [ ] **Step 3: Implement the plugin system** using Adapter, Proxy, Type Erasure, and error design; add a fake plugin test.
- [ ] **Step 4: Implement the async server project** using State, Builder, work queue, cancellation, and typed errors; add integration tests.
- [ ] **Step 5: Add project maps** linking each project requirement to the pattern and chapter that introduced it.

### Task 8: Course documentation and Notion book

**Files:**
- Create: `docs/course/README.md`
- Create: `docs/course/pattern-comparison-matrix.md`
- Create: `docs/course/glossary.md`
- Create: `docs/course/instructor-guide.md`
- Modify: Notion page `https://app.notion.com/p/fb5ab68cd9a64447a44e4535f5833c39`

- [ ] **Step 1: Write the course map** with prerequisites, difficulty, expected outcomes, and project checkpoints.
- [ ] **Step 2: Write the pattern matrix and glossary** from the verified chapter interfaces and tradeoffs.
- [ ] **Step 3: Create Notion category pages** `00` through `06` and the appendix under `디자인 패턴`.
- [ ] **Step 4: Create pattern subpages** with the fixed chapter template; include code blocks only after the corresponding repository chapter passes tests.
- [ ] **Step 5: Add repository links and navigation links** from each Notion pattern page to its code directory and from project pages back to patterns.
- [ ] **Step 6: Fetch the root and representative subpages** to verify titles, hierarchy, links, and saved content.

### Task 9: Whole-course verification

**Files:**
- Modify: `README.md`
- Modify: `docs/course/README.md`
- Test: all workspace crates and project integration tests

- [ ] **Step 1: Run `cargo fmt --all -- --check`.**
- [ ] **Step 2: Run `cargo test --workspace --all-targets`.**
- [ ] **Step 3: Run `cargo clippy --workspace --all-targets -- -D warnings`.**
- [ ] **Step 4: Run the project examples and confirm every README command works from a clean checkout.**
- [ ] **Step 5: Verify Notion root navigation and five representative pattern pages** against the chapter template.
- [ ] **Step 6: Record verification results and known limitations** in `docs/course/README.md`.

