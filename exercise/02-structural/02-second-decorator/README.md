# Exercise: add a second decorator

Create `Prefix<T: Service>` that prepends a configured prefix to successful
output and passes `ServiceError` through unchanged. Compose it with
`Logger<T>` in both orders.

Acceptance criteria: `Prefix::new(Uppercase, "result:").execute("hi")`
returns `"result:HI"`; empty input remains `EmptyInput`. Record events
with a service double and show which value each logger sees when it sits
inside or outside `Prefix`. Explain when a single plain function would be
clearer than a decorator chain.
