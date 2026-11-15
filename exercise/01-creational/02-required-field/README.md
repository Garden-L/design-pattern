# Exercise 2: Require an author

Add a required `author` field to `ReportBuilder` and `Report`. A successful
report must render `title`, `author`, and `body` on separate lines. A missing
or whitespace-only author must return `BuildError::MissingAuthor`. Extend
the before implementation and the shared before/after tests so both versions
follow the same new contract. Keep the `build()` result type.
