# 04 — Async server

Serve a finite stream of requests over bounded Tokio channels, dispatch work
to the existing thread pool, collect ordered responses, and stop cleanly.
Prerequisites: [chapter 04](../../04-rust-native/README.md) and
[chapter 05](../../05-concurrency-async/README.md).

From the workspace root:

```sh
cargo +stable run --quiet -p async-server
cargo +stable test -p async-server
```

The demo prints:

```text
7:9
2:16
```

| Requirement | Pattern / original chapter API | Project boundary |
| --- | --- | --- |
| Validate configuration | [Builder, chapter 01](../../01-creational/src/builder.rs) | Local ServerBuilder rejects zero workers/capacity without starting resources |
| Reject invalid lifecycle operations | [State, chapter 03](../../03-behavioral/src/state.rs), [connection state, chapter 04](../../04-rust-native/src/enum_state_machine.rs) | Reuse ConnectionState connect/send checks; RAII resets on exit |
| Process owned work and join workers | [Work queue, chapter 05](../../05-concurrency-async/src/work_queue.rs) | WorkQueue lives entirely inside spawn_blocking |
| Cancel pending async operations | [Cancellation, chapter 05](../../05-concurrency-async/src/async_pipeline.rs) | Biased select at both receive and forwarding boundaries |
| Retain failure categories and sources | [Error design, chapter 04](../../04-rust-native/src/error_design.rs) | ServerError keeps state, queue, cancellation, and JoinError cases |

[ServerBuilder](src/lib.rs) defaults to one worker and bridge capacity one.
`build` needs no runtime and starts no threads. `start` checks the transition
to Connected; starting twice fails. `serve(&mut self, Receiver<Job>, cancel)`
requires Connected and a Tokio runtime. A Job contains an arbitrary ID and a
u32 value; the response keeps the ID and squares the value into u64. Duplicate
IDs are allowed. Receive order, not ID or completion order, determines output.

The caller owns the bounded ingress sender and cancellation sender. Start the
producer and service concurrently, as in [main.rs](src/main.rs). Drop all
ingress senders for normal EOF, while retaining the cancellation sender.
Sending cancellation **or dropping its sender** requests cancellation. Ready
cancellation wins over buffered requests/EOF. It drops ingress immediately,
releases blocked producers, closes the bridge, drains/joins already accepted
queue jobs, and returns Cancelled without partial responses. Queue/worker
failures take precedence if cleanup itself fails. Cancellation does not undo
work already accepted; it may wait for that work to finish.

After EOF, error, or cancellation, the server is Disconnected and can be
started again. Dropping a polled `serve` future also closes input and resets
state, but its blocking task finishes cleanup in the background; only driving
`serve` to completion guarantees workers have joined before control returns.
Use the cancellation signal and await the result when cleanup must complete.

[Integration tests](tests/server.rs) cover zero configuration, invalid states,
restart, ordered work beyond channel capacity, maximum arithmetic, empty EOF,
pending cancellation, cancellation priority, cancellation-sender drop, blocked
producer release, and future drop. They use explicit Pending polls and
five-second timeouts, without sleeps.

This is an in-process request server, not an HTTP/TCP implementation. Tokio
is reused for the executor, async channels, selection, and the blocking bridge;
the queue and workers use the standard library. Input buffers are bounded,
but completed results accumulate until shutdown, so the workload must be
finite. Thread creation can panic; unexpected queue panics become Worker
errors at the blocking-task boundary where possible. No throughput claim is
made. Exercise: design streaming responses and specify their behavior under
cancellation before adding a socket adapter. See the
[project lesson](../../../docs/course/06-projects.md#async-server).
