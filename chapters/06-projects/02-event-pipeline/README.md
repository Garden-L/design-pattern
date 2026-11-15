# 02 — Event pipeline

Route `add <signed-i32>` requests, execute checked commands, notify observers,
and count accepted events through a channel actor. Prerequisites:
[chapter 03](../../03-behavioral/README.md) and
[chapter 05](../../05-concurrency-async/README.md).

From the workspace root:

```sh
cargo +stable run --quiet -p event-pipeline
cargo +stable test -p event-pipeline
```

The demo prints:

```text
balance=5
balance=3
processed=2, balance=3
```

| Requirement | Pattern / original chapter API | Project boundary |
| --- | --- | --- |
| Validate and route input | [Chain, chapter 03](../../03-behavioral/src/chain.rs) | RejectEmpty passes nonblank requests to RouteAdd |
| Change balance safely | [Command, chapter 03](../../03-behavioral/src/command.rs) | Add executes checked arithmetic on Context |
| Deliver change notifications | [Observer, chapter 03](../../03-behavioral/src/observer.rs) | EventBus delivers balance events in subscription order |
| Own processed count on one thread | [Actor, chapter 05](../../05-concurrency-async/src/actor.rs) | CounterActor acknowledges each accepted command |

[EventPipeline](src/lib.rs) keeps its balance, chain, and observer bus on the
calling thread. Only metrics cross the actor mailbox. EventBus uses Rc and
RefCell and is deliberately not made Send. `metrics()` returns a read-only
view, preventing callers from changing the count.

Blank/malformed additions return `PipelineError::Chain(Rejected)`, unknown
commands return `Chain(Unhandled)`, and balance overflow returns
`Command(Overflow)`. These failures neither notify nor change balance/count.
Signed and zero amounts are valid; each accepted command increments the count
once. Observer callbacks run synchronously after acceptance; a callback panic
propagates after the balance/count update, with no transactional rollback.

`shutdown(self)` joins the actor and returns balance/count. Empty input gives
zero/zero. Drop also joins through the actor's existing RAII implementation.
Retained metrics views report `ActorError::Closed` after either path.
[Tests](tests/pipeline.rs) cover delivery order, rejected/overflowing commands,
100 acknowledged requests, empty shutdown, Drop, and retained handles, with
five-second deadlines and no sleeps.

The coordinator blocks waiting for actor replies and should not run on an
async executor. This illustrates an actor for metrics rather than making
single-threaded observers thread safe. A local counter is simpler if no
thread-owned state is required. Exercise: add a second request handler and
verify it does not run after rejection. See the
[project lesson](../../../docs/course/06-projects.md#event-pipeline).
