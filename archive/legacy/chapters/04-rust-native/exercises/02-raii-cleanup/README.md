# Exercise: cleanup on early return

Add a second early-return branch to the before and after `run` examples.
Check that both return `(1, 0)`. Then make the branch unwind under
`catch_unwind`: the `Tracker` in the after version must return to zero
through `Drop`. Explain why a manual decrement in the before version does
not execute during unwinding.
