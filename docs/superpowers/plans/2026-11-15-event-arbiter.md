# Event Arbiter Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a standalone `event-arbiter` workspace package that filters, scores, reconciles, and observes one active event.

**Architecture:** `EventArbiter` owns ordered filter strategies, one composable scorer, the single `Option<&'static str>` active state, and listeners. Selection is deterministic by score, priority, distance, then id. Results are returned as `SelectionResult`; listeners receive only changed results.

**Tech Stack:** Rust 2021 workspace, standard library only, integration tests with Cargo.

**Spec:** `docs/superpowers/specs/2026-11-15-event-arbiter-design.md`

## Global Constraints

- Do not modify `chapters/06-projects/02-event-pipeline`.
- Add a new independent package at `chapters/03-behavioral/projects/01-event-arbiter`.
- The root workspace member list must include `event-arbiter`.
- `active` is exactly one `Option<&'static str>`.
- Same active event means no change and no listener notification.

## Review Focus

- Empty candidates return `active=None` and do not panic; test in `selects_none_when_no_candidate_passes`.
- All candidates rejected return `active=None` with a useful reason; test in `filters_candidates_before_scoring`.
- Ties are independent of input order; test in `tie_breaking_is_deterministic`.
- Repeated reconciliation does not notify listeners; test in `same_active_event_is_notified_once`.
- Transition and deactivation semantics are exact; test in `switching_active_event_reports_both_sides` and `clearing_active_event_reports_deactivation`.

### Task 1: Package scaffold and public contracts

**Files:**
- Modify: `Cargo.toml`
- Create: `chapters/03-behavioral/projects/01-event-arbiter/Cargo.toml`
- Create: `chapters/03-behavioral/projects/01-event-arbiter/src/lib.rs`
- Create: `chapters/03-behavioral/projects/01-event-arbiter/src/main.rs`
- Create: `chapters/03-behavioral/projects/01-event-arbiter/tests/arbiter.rs`

**Interfaces:**
- `Event`, `Context`, `EventFilter`, `EventScorer`, `SelectionResult`, `EventListener`, and `EventArbiter` are public from `event_arbiter`.
- `EventArbiter::new(filters, scorer)`, `reconcile(candidates, context)`, `active()`, and `add_listener(listener)` are the core API.

- [ ] Write integration tests for filtering, scoring, result fields, listeners, and tie-break rules.
- [ ] Run `cargo test -p event-arbiter` and verify it fails because the contracts are not implemented.
- [ ] Implement the minimal public types and behavior in `src/lib.rs`.
- [ ] Add a small runnable demonstration in `src/main.rs`.
- [ ] Run `cargo fmt` and `cargo test -p event-arbiter` and verify all tests pass.

### Task 2: Teaching README

**Files:**
- Create: `chapters/03-behavioral/projects/01-event-arbiter/README.md`

**Interfaces:**
- Documents the public API from Task 1 and the exact test command.

- [ ] Write README sections for before/after, why/when/when-not, Mermaid structure, pattern responsibilities, run/test instructions, and exercises.
- [ ] Verify all required sections and all referenced paths/commands exist.

### Task 3: Final verification

**Files:**
- No additional files.

- [ ] Run `cargo fmt --all -- --check`.
- [ ] Run `cargo test -p event-arbiter`.
- [ ] Report changed files and command results.
