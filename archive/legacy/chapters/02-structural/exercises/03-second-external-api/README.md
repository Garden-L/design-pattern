# Exercise: adapt a second payment API

Add a `VoucherGateway` whose API accepts a decimal amount as a string and
returns its own error enum. Implement `PaymentProcessor` for a new adapter.
Keep `charge(&self, amount: u64) -> Result<(), PaymentError>` unchanged.

Acceptance criteria: `charge(250)` reaches the gateway as `"250"`;
zero and any unsupported amount return `InvalidAmount`; a voucher rejection
becomes `Declined`; gateway unavailability becomes `Unavailable`. Test both
adapters through a `&dyn PaymentProcessor` so callers observe the same
contract. Do not discard the original error category.
