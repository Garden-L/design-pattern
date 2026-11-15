# Exercise: fallible shutdown

The required `shutdown(self) -> Vec<JobResult>` cannot return a worker error.
Design an additional `try_shutdown` API that returns a typed error and retains
any successfully completed results if an internal worker fails. Keep the
existing API working as a compatibility wrapper.

Write tests for zero workers, an empty queue, closing twice, rejection after
close, and draining a finite batch larger than the input capacity. For failure
testing, introduce a processing strategy whose deliberate panic is observed
through a real thread join; avoid sleeps or global flags.

Acceptance: every worker is joined even when one fails, no result channel
blocks a worker while shutdown waits for it, and Drop cannot double-panic
during unwinding. Explain which accepted jobs may lack results after failure.

Instructor hint: the existing implementation joins all workers before
reporting panic. Do not introduce `?` inside the join loop if it would skip
the remaining joins. Bounded input does not bound accumulated result memory.
