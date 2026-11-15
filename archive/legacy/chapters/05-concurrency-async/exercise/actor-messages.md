# Exercise: reset with acknowledgement

Add a `Reset` message and `CounterHandle::reset(value) -> Result<u64, ActorError>`.
The actor must mutate its own counter and return the new value in a reply.
Do not expose a mutex around the counter to callers.

First write a test: start at 10, add 2, reset to 3, add 4, then shut down and
expect 7. A reset through a surviving handle after shutdown must return Closed.
Check that an overflowing add still leaves the previous value intact.

Acceptance: tests use replies and joins with the existing timeout helper,
never sleeps. Explain why requests from different cloned handles have no
promised relative order unless callers coordinate them explicitly.

Instructor hint: a successful mailbox send is not an acknowledgement that
state changed. Preserve the request/reply boundary and avoid blocking the
actor on a bounded reply channel that an abandoned caller will never drain.
