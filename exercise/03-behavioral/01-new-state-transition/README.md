# Exercise: cancel an order

Add `Action::Cancel` and `OrderState::Cancelled`. Draft and Placed orders can
be cancelled. Shipped and Cancelled orders reject every action, including
Cancel, with `TransitionError::InvalidTransition`.

Acceptance criteria:

- Empty input still returns Draft; Place then Ship still returns Shipped.
- Cancel returns Cancelled; Place then Cancel returns Cancelled.
- Ship from Draft, repeated Place, Ship then Cancel after placement, and every
  action after cancellation return `InvalidTransition`.
- Add a `cancel(self) -> Result<Self, TransitionError>` method. Extend `apply`
  and retain the existing `place` and `ship` behavior.
- Extend the direct match in `before/state.rs`, then run the same new sequence
  fixtures through both `before_state::run` and `after_state::run` in the
  integration suite. Do not make only one version accept cancellation.
- Test the full state/action table, including Cancelled as a terminal state.

Start with tests that fail because cancellation is unavailable. Implement the
smallest changes and run `cargo +stable test -p behavioral` and
`cargo +stable clippy -p behavioral --all-targets -- -D warnings`.

Discussion: Which invalid operations does this enum reject at runtime? Sketch
how distinct Draft, Placed, and Shipped types would move some checks to compile
time, and explain the extra work needed for states loaded from storage.
