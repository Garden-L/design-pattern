# Exercise: cancellation while output is full

Extend the async lesson with a separate streaming API that sends results to a
bounded Tokio output channel instead of collecting a Vec. Preserve the
existing batch API. Decide what happens to a transformed job if cancellation
wins while its output send is pending, and document that decision.

First fill a capacity-one output channel. Poll the pipeline until Pending,
then send cancellation and assert termination and input closure under a
timeout. A second test should close the output receiver and verify a typed
output-closed error. Run the producer, stage, and consumer concurrently.

Acceptance: select cancellation while waiting for both input and output;
dropping the pipeline leaves no detached tasks; normal EOF drains accepted
output according to the stated contract. Never use sleeps to fill a channel
or infer that a task has started.

Instructor hint: a cancelled `send` future may discard its owned message.
Cancellation does not roll back external I/O. Compare this streaming contract
with the current batch contract, which discards partial results on cancellation.
