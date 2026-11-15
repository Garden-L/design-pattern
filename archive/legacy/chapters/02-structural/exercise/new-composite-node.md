# Exercise: add a composite node

Add `Bracketed` with one child `Box<dyn Node>`. Its `render` wraps the child's
rendered text in `[` and `]`. Nest it inside a `Group` with `Text` nodes.

Acceptance criteria: a group containing `Text("A")`, a `Bracketed` group
containing `Text("B")` and `Text("C")`, then `Text("D")` renders `"A[BC]D"`.
An empty bracketed group renders `"[]"`. Discuss the ownership choice and
why an enum is simpler when all node variants are known in advance.
