# Exercise: completion order is not input order

Start from `before/channel_pipeline.rs` and the shared fixtures in
`tests/paired.rs`. Explain why sorting results by job ID would fail when IDs
repeat or arrive as `[8, 1, 8]`.

Extract the after version's ordered collection into a private helper. Write a
unit test feeding completion records with sequence numbers `[2, 0, 1]` and
assert input-order results. Then temporarily remove the sort and observe the
test fail. Keep sequence numbers separate from user IDs.

Acceptance: empty records return an empty vector; the deliberately scrambled
fixture restores input order; all existing tests still pass. Use no sleeps.

Instructor hint: an output assertion alone need not force a particular thread
schedule. A scrambled collector fixture tests reordering directly. Discuss
the memory and latency costs of waiting for a slow earliest job.
