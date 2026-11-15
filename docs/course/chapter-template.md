# Chapter: <number and title>

Use this structure for each chapter. Keep the sections in this order; use a
short explanation where a section does not need a long treatment.

## Goals

- What the learner will be able to explain or implement.
- Prerequisites and vocabulary needed for this chapter.

## Problem

Describe the concrete requirement that motivates the example. State the
constraints and the behavior a caller needs.

## 직접 구현

Show the smallest reasonable implementation before the design is introduced.
Place it in `chapters/<chapter>/<pattern>/examples/01_direct.rs`.

## Failure

Introduce a realistic change in requirements. Explain where the initial
implementation becomes awkward, fragile, or difficult to extend, with
evidence from the example.

## 설계 적용

Walk through the design changes in small steps. Explain ownership, borrowing,
traits, enums, or other Rust choices that matter to the refactoring.

## 패턴 적용 코드

Show the completed implementation in `examples/02_<pattern>.rs`, identify its
public API from `src/lib.rs`, and explain the important tradeoffs.

## Comparison

Compare direct implementation and pattern-applied behavior, structure,
complexity, and extensibility.
Discuss performance only when it has been measured, and note simpler Rust
alternatives where they fit.

## Tests

List the behaviors covered, including relevant edge cases and failure paths.
Provide the command learners can run, for example
`cargo test -p <pattern-crate>`, to execute the pattern crate's tests.

## Exercises

Give progressively challenging tasks. Put the runnable starter package at
`exercise/<chapter>/<exercise>/`, with `src/lib.rs` and `tests/exercise.rs`.
State expected behavior and provide enough acceptance criteria for learners to
check their work.

## Instructor notes

Include a suggested discussion flow, questions to ask, likely misconceptions,
and hints or solution guidance for the exercises.

## Summary

Recap the problem, the central design idea, when it is useful, and when a
simpler approach is preferable. End with a few review prompts if appropriate.
