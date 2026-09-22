# AGENTS.md
These rules apply to every task unless explicitly overridden. Bias toward caution over speed on non-trivial work; use judgment on trivial tasks.

## Engineering Rules
1. **Think before coding** — State assumptions. If ambiguous, present interpretations and ask rather than guess. Stop when confused and name what's unclear.
2. **Simplicity first** — Minimum code that solves the problem. No speculative features, no abstractions for single-use code.
3. **Surgical changes** — Touch only what you must. Don't refactor or reformat working code. Match existing style.
4. **Read before you write** — Before adding code, read its exports, callers, and shared utilities. If unsure why code is structured a way, ask.
5. **Goal-driven execution** — Define success criteria, then loop until verified instead of following fixed steps.
6. **Use the model only for judgment** — Classification, drafting, summarization, extraction. If code can answer deterministically, code answers.
7. **Surface conflicts, don't average them** — If two patterns contradict, pick one (more recent / more tested), explain why, and flag the other for cleanup.
8. **Tests verify intent** — Tests encode *why* behavior matters, not just *what* it does. A test that can't fail when business logic changes is wrong.
9. **Checkpoint after each significant step** — Summarize what's done, verified, and left. Don't continue from a state you can't describe back.
10. **Match codebase conventions** — Conformance over taste. If a convention is genuinely harmful, surface it; don't fork silently.
11. **Fail loud** — "Completed" is wrong if anything was skipped silently. "Tests pass" is wrong if any were skipped. Surface uncertainty, don't hide it.

## Custom Instructions
- **Language**: Always respond in Chinese (Simplified), including summaries and explanations for all plugin commands (e.g. `/claude-hud`, `/ce:plan`).
- **Errors**: Explain technical errors (e.g. PostgreSQL 23505) in Chinese first, then the technical details.

## Compact Instructions
When compressing, preserve in priority order:
1. Architecture decisions (never summarize away)
2. Modified files and their key changes
3. Current verification status (pass/fail)
4. Open TODOs and rollback notes
5. Tool outputs (drop detail, keep pass/fail)

## Knowledge Store
`docs/solutions/` — documented solutions to past problems (bugs, best practices, architecture patterns, workflow improvements), organized by category with YAML frontmatter (`module`, `tags`, `problem_type`). Relevant when implementing or debugging in the oa4rust or oa subproject areas.
