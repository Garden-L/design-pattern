# Exercise: add a threshold discount

The shipped strategies are `Regular` and `TenPercentOff`. Add a new
`ThresholdDiscount` strategy that subtracts 30 money units when the subtotal
is at least 200 and otherwise returns the subtotal unchanged. Do not change
`Order`, `PricingStrategy`, or `price_with`.

Acceptance criteria:

- Subtotals 0, 199, 200, and 250 yield 0, 199, 170, and 220 respectively.
- Run the same fixtures through `price_with(&ThresholdDiscount, &order)` and
  a `&dyn PricingStrategy`. Existing strategies retain their current results.
- First write a test using the new strategy and run it, then implement it.
- Keep the existing boolean `quote` examples and their shared tests intact;
  the exercise extends the reusable API rather than changing their contract.

Extension: expose a choice of three pricing policies at an application
boundary. Compare an enum match, a generic caller, and `&dyn PricingStrategy`.
Explain why runtime selection still needs a decision somewhere, even though
pricing callers no longer repeat policy logic.

Check with `cargo +stable test -p behavioral` and
`cargo +stable clippy -p behavioral --all-targets -- -D warnings`.
