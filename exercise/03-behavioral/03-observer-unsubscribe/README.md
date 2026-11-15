# Exercise: deliver a subscription at most once

`EventBus::unsubscribe` already exists and its basic removal path is tested.
Add a separate helper, rather than reimplementing that method:

```rust,ignore
fn subscribe_once(
    bus: &std::rc::Rc<EventBus>,
    callback: impl Fn(&Event) + 'static,
) -> SubscriptionId;
```

The helper registers a callback that removes itself before calling user code.
Use a `Weak<EventBus>` inside the closure so that the bus does not own a strong
reference to itself. The bus snapshots callbacks at the start of each publish;
removal alone cannot stop an already captured callback from being invoked.

Acceptance criteria:

- Publish `paid` then `shipped`: the one-shot callback records only `paid`,
  while a normal subscriber records both events in subscription order.
- Removing the returned ID before publishing prevents any one-shot delivery.
  Removing it after its delivery returns `false`.
- Publishing from inside the one-shot callback does not deliver it again.
- Register a normal callback before the one-shot callback. Have it publish a
  nested event once. The one-shot callback records only that nested event;
  its stale entry in the outer snapshot must not invoke user code a second time.
- After dropping the bus and external handles, an independently retained
  `Weak<EventBus>` cannot upgrade. No strong reference cycle remains.
- Existing snapshot tests and shared before/after output tests still pass.

Hint: a shared `Cell<Option<SubscriptionId>>` can both hold the ID assigned
after registration and guard delivery. Take the ID before removing it and
invoking user code; return immediately when it was already taken. This also
handles callbacks still present in an older snapshot. Keep mutable borrows of
the output sink short before recursively publishing.

Write and run the new tests first. Run `cargo +stable test -p behavioral` and
`cargo +stable clippy -p behavioral --all-targets -- -D warnings` afterward.
