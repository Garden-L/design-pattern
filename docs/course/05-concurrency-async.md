# 05 — Concurrency and async

## Goals

Move owned messages between threads, distinguish Send from Sync, keep mutex
guards short, define ordering and shutdown, and cancel a pending Future using
signals rather than timing assumptions. Prerequisites: ownership, Result,
enums, iterators, RAII, and basic async/await syntax. Allow roughly two hours
plus exercises. The [chapter index](../../chapters/05-concurrency-async/README.md)
links every before/after example and API.

## Problem

A finite sequence of jobs must square each `u32` into a `u64`, retaining IDs
and input order. Widening before multiplication handles even `u32::MAX` without
overflow. IDs are arbitrary labels: duplicate and descending IDs are valid.
A separate checked counter must accept updates from multiple callers without
lost updates. Empty work terminates normally. Zero thread workers is an error
even with no jobs. Every started thread needs an owner responsible for joining.

## Before

The [channel baseline](../../chapters/05-concurrency-async/01-channel-pipeline/examples/01_direct.rs)
maps a vector serially. The [queue baseline](../../chapters/05-concurrency-async/03-work-queue/examples/01_direct.rs)
builds a result vector in one loop. Both preserve the same finite output
contract as their pattern callers; their workers argument is validated but does
not introduce concurrency. A [local counter](../../chapters/05-concurrency-async/02-actor/examples/01_direct.rs)
works through exclusive mutation by one caller. These are good simple choices
when no concurrent producer or shared state owner is needed.

The [async baseline](../../chapters/05-concurrency-async/04-async-pipeline/examples/01_direct.rs)
uses `std::future::poll_fn` and explicitly polls a Tokio receiver and a oneshot
cancellation future. It preserves wakeups, accumulated results, EOF, and
cancellation priority manually. Tokio is already necessary here for the
asynchronous channel; a standard blocking receiver would block the executor.

## Failure

A serial batch cannot accept ongoing submissions through a reusable pool.
Thread completion order does not promise input order. Sharing a local mutable
counter requires ownership coordination; wrapping every operation in unrelated
locks can obscure the invariant. Handwritten Future polling adds several
branches that must all register the right waker. Busy polling or sleeping
would introduce unnecessary work and unreliable coordination.

Shutdown bugs often come from keeping an unused sender alive, holding a mutex
while computing, joining a producer before draining its bounded output, or
waiting for a reply from a component that is itself waiting on the caller.

## Refactor

1. **Channel pipeline:** `run_pipeline(Vec<Job>, workers)` uses a bounded input
   `sync_channel` with capacity equal to workers. One producer tags jobs by
   input position. Workers share the standard single-consumer receiver through
   `Arc<Mutex<_>>`; only `recv` holds the guard. The collector runs while the
   producer and workers run, then sorts by position. The sole job sender drops
   at producer EOF, and every result sender drops before collection ends.
   All threads are joined. Unexpected thread panic becomes `WorkerPanicked`;
   unexpected input disconnection becomes `ChannelClosed`.
2. **Actor:** one thread owns a `u64` counter and receives Add, Value, or Shutdown
   messages. Its bounded mailbox has capacity 16. Cloneable handles carry no
   counter reference and use unbounded one-reply channels, so an abandoned
   reply cannot block the actor. Checked addition returns Overflow without
   mutation. A reply acknowledges an applied operation. Explicit Shutdown
   works even while other sender handles remain alive.
3. **Work queue:** `WorkQueue::new(workers)` starts a reusable pool with bounded
   input capacity equal to workers. `submit(&self, Job)` blocks for capacity
   and assigns a unique atomic ticket before sending. Sequential submissions
   retain their order; overlapping submissions use ticket order, with no
   promise about which producer gets the earlier ticket. Relaxed ordering is
   enough for ticket allocation; the channel communicates the actual job.
   `close(&mut self)` drops the only sender and is idempotent. `shutdown(self)`
   closes and joins every worker, returning all accepted results in ticket
   order. Workers return local result vectors through joins, so shutdown cannot
   deadlock against a full output channel.
4. **Async pipeline:** `run_async_pipeline(Receiver<Job>, oneshot::Receiver<()>)`
   is an `async fn`, hence returns a Future yielding
   `Result<Vec<JobResult>, AsyncPipelineError>`. Tokio `select!` waits for input
   or cancellation. `biased` puts ready cancellation first, including when EOF
   or buffered input is also ready. The stage spawns no tasks. Async expresses
   waiting for messages here; squaring these tiny values needs no CPU pool.

## After

The [APIs](../../chapters/05-concurrency-async/src/) own lifecycle rules; the
[pattern callers](../../chapters/05-concurrency-async/) demonstrate use.
For example, a synchronous caller can submit then explicitly drain:

```rust
use concurrency_async::{work_queue::WorkQueue, Job, JobResult};
let queue = WorkQueue::new(2).unwrap();
queue.submit(Job { id: 7, value: 3 }).unwrap();
assert_eq!(queue.shutdown(), vec![JobResult { id: 7, value: 9 }]);
```

`Job`, results, queue, and actor handles can cross thread boundaries: Send
allows ownership transfer. Sync means a shared reference can cross thread
boundaries. Arc provides shared ownership, not automatic thread safety for
its contents. A Mutex provides exclusive access; the standard receiver needs
that protection when shared. The actor counter itself requires neither Arc
nor Mutex because exactly one thread owns it. `Rc<RefCell<_>>` is unsuitable
for this cross-thread sharing.

For async use, keep the cancellation sender alive until normal completion or
explicit cancellation, and run a bounded producer concurrently with the stage:

```rust
use concurrency_async::{async_pipeline::run_async_pipeline, Job};
use tokio::sync::{mpsc, oneshot};
async fn example() {
let (sender, input) = mpsc::channel(1);
let (_cancel, cancelled) = oneshot::channel();
let producer = async move {
    sender.send(Job { id: 7, value: 3 }).await.unwrap();
}; // sender drops when producer completes, signalling EOF
let (_, results) = tokio::join!(producer, run_async_pipeline(input, cancelled));
assert_eq!(results.unwrap()[0].value, 9);
}
```

Normal input EOF drains buffered jobs and returns Ok. Sending `()` or dropping
the cancellation sender requests cancellation, returns Cancelled, discards
partial results, and drops the input receiver. Retaining any input sender
prevents normal EOF; use cancellation to stop an idle pipeline. Dropping a
polled pipeline Future also drops its receiver; there are no detached tasks
to abort or join. Cancellation is cooperative at await/select boundaries and
does not undo external side effects if future exercises add them.

Tokio is restricted to runtime, sync, and macro features; tests add time for
deadlines. No stream crate is needed: repeated `recv().await` models a stream
of messages. The standard Future trait supplies the polling contract; Tokio
supplies the executor and async channels. See the primary documentation for
[select and cancellation safety](https://docs.rs/tokio/latest/tokio/macro.select.html)
and [bounded send behavior](https://docs.rs/tokio/latest/tokio/sync/mpsc/struct.Sender.html).

## Comparison

| Design | Ordering | Closure / cancellation | Cost and simpler alternative |
| --- | --- | --- | --- |
| Channel pipeline | Input position, regardless of worker completion | Producer EOF drains and joins | Thread setup and sorting; serial map for tiny finite work |
| Actor | Mailbox arrival; same-handle acknowledged calls are sequential | Shutdown processes earlier messages; later requests return Closed | Request/reply overhead; local mutable counter or short Mutex operation for simple state |
| Work queue | Successful submission tickets | close rejects new work, drains accepted work; shutdown joins | Retains results until shutdown; serial loop for one fixed batch |
| Async pipeline | Receive order | EOF drains; cancellation discards partial output | Requires executor; blocking channel for a dedicated OS thread |

Backpressure bounds **waiting input**, not total memory: output vectors grow
with completed jobs. The channel pipeline also has an unbounded finite output
channel. These examples expect bounded workloads, not an endless service.
For continuous work, add a concurrent streaming consumer and explicit output
backpressure. These tiny arithmetic jobs do not establish a speed advantage;
no performance measurement is claimed.

Avoid calling blocking queue/actor operations on an executor thread. Never
hold a synchronous mutex guard across `.await`. Do not join a worker while
holding a resource it needs. With bounded channels, concurrently drain the
consumer instead of filling an entire channel before starting it. Do not
replace explicit shutdown with dropping arbitrary handle clones and hoping
all senders disappeared.

Queue Drop closes and joins but discards results; actor Drop sends Shutdown
and joins. Both may block. Thread creation may panic on resource exhaustion.
The required queue shutdown Vec and actor shutdown scalar cannot encode
unexpected internal worker failure: explicit shutdown panics after joining;
Drop ignores join errors to avoid a second panic during unwinding. No user
callbacks run in the current workers. A general task executor should add a
fallible shutdown API and specify partial results.

## Tests

Run `cargo +stable test -p concurrency-async` for 20 tests: shared before/after
fixtures plus direct lifecycle tests. They cover duplicate/unordered IDs,
maximum input arithmetic, empty input, zero workers, batches exceeding input
capacity, concurrent producers, overflow without state mutation, live handles
after shutdown, normal EOF, pending-receive cancellation, ready cancellation
priority, cancellation-sender drop, and pending-Future drop.

Thread tests use `recv_timeout` with a five-second deadline around the whole
operation. Async tests use a current-thread Tokio runtime and five-second
timeouts. A helper explicitly polls the real Future to Pending before sending
cancellation or dropping it. Message replies, EOF, and joins establish
correctness; timeouts only catch hangs. No test sleeps, assumes a task started
after elapsed time, or relies on thread completion order. An async timeout
cannot preempt a blocking operation on the same executor, so keep async stages
nonblocking; an external CI deadline can additionally catch such regressions.

Finish with `cargo +stable test --workspace`, formatting, and workspace Clippy
using the commands in the chapter README.

## Exercises

1. [Ordering](../../exercise/05-concurrency-async/03-channel-ordering/README.md):
   test deliberately scrambled completion records independently of scheduling.
2. [Actor messages](../../exercise/05-concurrency-async/01-actor-messages/README.md):
   add an acknowledged Reset message and retain overflow behavior.
3. [Queue lifecycle](../../exercise/05-concurrency-async/04-queue-lifecycle/README.md):
   design fallible shutdown without abandoning remaining joins.
4. [Async cancellation](../../exercise/05-concurrency-async/02-async-cancellation/README.md):
   stream results with bounded output and cancel a pending send.

## Instructor notes

Have learners predict which sender clone keeps each channel alive. Ask them
to trace EOF from producer to workers to collector, then explain why a mutex
guard must end before computation. Compare a successful send with a successful
actor reply. For cancellation, poll to Pending and ask what resource is dropped
when select returns. Ask why a bounded input cannot guarantee bounded total
memory. Discuss which examples should remain serial for such small jobs.

## Summary

Ownership and explicit lifecycle rules determine how concurrent components
finish. Channels communicate jobs and EOF, the actor serializes state changes,
the queue owns worker joins, and the async stage owns its cancellation scope.
Review: which ordering is promised, who closes the channel, who joins the
worker, and what happens to partial output when cancellation wins?
