# Exercise 3: Inject a test double

Start from the existing `RecordingSender` in the integration test; do not copy
the type. Extend it with an explicit empty-message policy and a reset or
inspection method. Inject it into `Notifier`, call `notify` twice, and assert
the two messages arrive in order. Then add a second sender type that records
rejected empty messages. The current baseline API returns `()`, so the required
observable behavior is the recording/rejection policy; changing the API to a
typed `Result` is an optional extension. Explain which behavior belongs to
`Notifier` and which belongs to the sender. No network service is needed.
