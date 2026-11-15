# 06 — Integrated projects

## Goals

Compose the verified chapter APIs into four independently runnable projects.
Practice choosing boundaries, preserving errors, testing executable behavior,
and assigning ownership of shutdown. Read chapters 01–05 first; the CLI and
plugin registry are intermediate exercises, and the event/async lifecycle
projects require the concurrency chapter. Suggested teaching time: one session
per project, with extra time for async ownership. These are estimates.

| Project | Run / test package | Learning checkpoint |
| --- | --- | --- |
| [Notification CLI](../../chapters/06-projects/01-notification-cli/README.md) | notification-cli | Compose construction, policy, and orchestration |
| [Event pipeline](../../chapters/06-projects/02-event-pipeline/README.md) | event-pipeline | Separate routing, command effects, observers, and actor state |
| [Plugin system](../../chapters/06-projects/03-plugin-system/README.md) | plugin-system | Adapt, proxy, erase types, and preserve errors |
| [Async server](../../chapters/06-projects/04-async-server/README.md) | async-server | Configure, transition, backpressure, cancel, and join |

## Problem

### Notification CLI

An executable must select email/SMS and regular/discount pricing, require a
nonblank title and message, and return a consistent receipt. Missing/invalid
options must fail with exit 2 and no delivery. The default price is 100 teaching
units; discount is 90. Delivery is local string rendering, making command tests
deterministic and independent of an external provider.

### Event pipeline

Accept `add <signed-i32>`, reject malformed/overflowing commands without effects,
publish each accepted balance in observer registration order, and count
accepted commands in a channel-owned actor. Unknown routes are Unhandled.
Empty shutdown returns balance/count zero; retaining a metrics view must not
prevent shutdown. The coordinator stays single-threaded; the metric actor is
the separate state owner.

### Plugin system

Use one registry for heterogeneous plugins. Adapt a chapter DocumentStore
with its caching proxy and adapt a legacy I/O reader without discarding error
causes. Reject duplicate/blank names and blank keys. Runtime selection means
trait objects in this project, not loading arbitrary binary libraries.

### Async server

Configure a worker count and bridge capacity, reject zero values, and require
start before serving. Receive finite Jobs on bounded Tokio channels and return
their squared values in receive order, retaining arbitrary/duplicate IDs.
EOF drains and joins; cancellation closes input, drains accepted queue work,
joins, and returns a typed cancellation error without partial output. The
transport is an in-process channel, keeping protocol concerns out of lifecycle
teaching. No existing chapter API is changed.


## Before

A direct implementation could select notification formatting with branches,
validate fields inline, update balances and call each observer directly, switch
on plugin names with provider-specific errors, and process server jobs in one
loop. These are reasonable for fixed, small requirements. The earlier chapters
contain executable before/after comparisons; these projects are integration
callers of their public after APIs rather than duplicate chapter implementations.

## Failure

Adding a channel or pricing policy couples a direct CLI's choices. Adding an
observer couples notification recipients to balance mutation. Adding a provider
spreads error translation and caching through plugin callers. Adding async
waiting around a blocking queue can freeze a current-thread runtime. Closing
only one sender clone, joining before releasing input, or ignoring a pending
send during cancellation can prevent termination.

## Refactor

Use the following maps to trace each requirement to the chapter that introduced
its mechanism and the integrated implementation. These source links identify
actual APIs; domain-specific facades/builders/adapters remain local.

| CLI requirement | Pattern / introducing chapter | Integration |
| --- | --- | --- |
| Select product | [Factory, 01](../../chapters/01-creational/src/factory_method.rs) | EmailFactory / SmsFactory create notifications |
| Require report fields | [Builder, 01](../../chapters/01-creational/src/builder.rs) | ReportBuilder validates before delivery |
| Exchange price rule | [Strategy, 03](../../chapters/03-behavioral/src/strategy.rs) | PricingStrategy borrowed at runtime |
| One orchestration call | [Facade, 02](../../chapters/02-structural/src/facade.rs) | NotificationFacade follows the pattern in its own domain |

| Event requirement | Pattern / introducing chapter | Integration |
| --- | --- | --- |
| Validate then route | [Chain, 03](../../chapters/03-behavioral/src/chain.rs) | RejectEmpty → RouteAdd; only Pass continues |
| Apply checked changes | [Command, 03](../../chapters/03-behavioral/src/command.rs) | Add executes on Context |
| Deliver accepted changes | [Observer, 03](../../chapters/03-behavioral/src/observer.rs) | EventBus publishes balance events |
| Own metrics across a mailbox | [Actor, 05](../../chapters/05-concurrency-async/src/actor.rs) | CounterActor replies acknowledge count updates |

| Plugin requirement | Pattern / introducing chapter | Integration |
| --- | --- | --- |
| Translate interfaces | [Adapter, 02](../../chapters/02-structural/src/adapter.rs) | DocumentPlugin and LegacyAdapter implement Plugin |
| Cache successes, retry errors | [Proxy, 02](../../chapters/02-structural/src/proxy.rs) | Actual CachedProxy wraps the chapter store |
| Hold different concrete plugins | [Boxed Factory products, 01](../../chapters/01-creational/src/factory_method.rs), [Composite nodes, 02](../../chapters/02-structural/src/composite.rs) | Box<dyn Plugin> makes the earlier type erasure technique explicit |
| Retain underlying causes | [Error design, 04](../../chapters/04-rust-native/src/error_design.rs) | PluginError::App wraps AppError; source reaches io::Error |

| Server requirement | Pattern / introducing chapter | Integration |
| --- | --- | --- |
| Validate construction | [Builder, 01](../../chapters/01-creational/src/builder.rs) | ServerBuilder stores validated worker/capacity settings |
| Check lifecycle | [State, 03](../../chapters/03-behavioral/src/state.rs), [enum connection, 04](../../chapters/04-rust-native/src/enum_state_machine.rs) | ConnectionState guards start/serve; local RAII resets on exit |
| Own and join parallel work | [Work queue, 05](../../chapters/05-concurrency-async/src/work_queue.rs) | A blocking task owns WorkQueue and its shutdown |
| Stop async waiting | [Cancellation, 05](../../chapters/05-concurrency-async/src/async_pipeline.rs) | Biased select handles input and bridge-send waits |
| Distinguish failure causes | [Error design, 04](../../chapters/04-rust-native/src/error_design.rs) | ServerError retains queue, cancellation, and task sources |
| Reset on early exit/drop | [RAII, 04](../../chapters/04-rust-native/src/raii.rs) | ResetState guard closes the lifecycle scope |


## After

The four project READMEs link source, tests, runnable commands, and expected
output. No new third-party dependency is introduced: Tokio was already used
in chapter 05. Synchronous components use the standard library; the server
uses Tokio for real async waiting and spawn_blocking to isolate queue operations.

The event coordinator applies a command, acknowledges its count update, then
publishes. Observer panics propagate after acceptance; delivery is not a
transaction and subscribers are not automatically retried. Metrics views expose
no mutation. Shutdown and Drop join even while those views remain alive.

The plugin registry erases only the public Plugin interface. The store proxy
caches successful values without expiry and retries errors. StoreError is a
typed variant because its upstream enum does not implement Error. The legacy
adapter preserves PluginError → AppError → io::Error without stringifying it.
The registry/cache remain single-threaded, as in the original chapter.

The server's caller must retain the cancellation sender during normal EOF.
Sending cancellation or dropping that sender requests shutdown; ready
cancellation has priority over queued input/EOF. Input closes before cleanup
is awaited, freeing blocked producers. Closing the bridge lets accepted queue
work drain, then shutdown joins workers. Queue/JoinError failures take priority
over cancellation if cleanup fails. Returning from serve resets Disconnected,
allowing start again. Dropping a polled serve future closes input and resets
state, but its blocking task finishes cleanup in the background. Cancel and
await serve when joined cleanup is required before proceeding.

## Comparison

| Project | Why compose patterns | Cost / simpler option |
| --- | --- | --- |
| CLI | Independent product, policy, validation, and orchestration | More boundaries; one function/enum for a fixed workflow |
| Events | Isolated routing, state effects, recipient delivery, and actor ownership | Synchronous callback/reply cost; local counter for one-thread state |
| Plugins | Provider-independent dispatch, caching, and error inspection | Boxes and single-threaded cache; enum for a closed set |
| Server | Explicit async/backpressure/shutdown ownership around a reusable queue | Blocking bridge and retained vectors; serial map for tiny finite work |

These examples make no measured performance claims. Bounded input does not
bound total memory: the queue/server retain completed responses until shutdown,
and the plugin cache retains successes. Thread creation may panic on resource
exhaustion. Inherited actor/queue shutdown APIs can panic on internal worker
failure; server task joins translate caught task panics to a typed Worker error.
Do not call the event pipeline's blocking actor operations on an executor.

## Tests

Run all commands from the workspace root using stable Rust:

```sh
cargo +stable test -p notification-cli -p event-pipeline -p plugin-system -p async-server
cargo +stable test --workspace
cargo +stable test --workspace --all-targets
cargo +stable fmt --all -- --check
cargo +stable clippy --workspace --all-targets -- -D warnings
```

Command tests spawn the actual CLI. Event tests verify subscription order,
failure atomicity, actor replies, empty shutdown, retained handles, and Drop.
Plugin tests use the real registry/adapters/proxy with small fake providers and
inspect underlying error sources. Server tests cover invalid states/config,
ordered and empty work, restart, cancellation priority, sender drop, blocked
producer release, and future-drop lifecycle. Thread deadlines and async
timeouts detect hangs; explicit Pending polls establish waiting. No sleeps
establish correctness. Async timeouts cannot preempt a blocking executor bug;
an external CI deadline is an additional safeguard.

## Exercises

1. CLI: inject a third factory through the facade, test output, then add its CLI
   selection. Keep existing error and pricing behavior.
2. Events: introduce a new handler and show rejected requests never reach it.
   Explain which operations are acknowledged before observer delivery.
3. Plugins: implement another Plugin without editing the registry. Add a
   failure whose source remains inspectable after trait-object dispatch.
4. Server: design bounded streaming responses before adding a network adapter.
   Specify accepted-work behavior, partial results, and shutdown ownership.

## Instructor notes

Ask learners to trace ownership from CLI flags to owned product, from EventBus
callback to event borrow, from a registry box to a concrete cache, and from a
Tokio sender through the bridge to the worker pool. Have them predict which
sender keeps each receive alive. Discuss why type erasure does not provide a
dynamic-library ABI, why a callback panic does not undo a command, why cancel
does not roll back accepted jobs, and why dropping an async future cannot
perform an awaited join. Use the README outputs as checkpoints before exercises.

## Summary

Review: which choices are open runtime substitutions and which are closed enum
states? Where are errors translated and causes retained? Who closes input,
who joins threads, and what evidence establishes completion? Remove any pattern
whose boundary does not serve a concrete change requirement.
