# OA4RUST Phase 4 — Parity-Regression Scaffold (RECORD mode)

**Date:** 2026-08-15
**Scope:** Add a compile-checked, `#[ignore]`d parity-regression harness that records
oa4rust READ endpoint responses into a corpus for later o2server comparison.
No production code, `integration_runner.rs`, or `integration_tests/scenarios/mod.rs` modified.
**CI impact:** None — the new test is `#[ignore]`d and a separate `[[test]]` target
(`parity_runner`), so the existing green gate is untouched.

---

## 1. Why

For **takeover verification** we eventually need to compare oa4rust endpoint responses
against the original Java **o2server**. o2server is *not* available in this environment,
so this scaffold implements **RECORD mode only**: it captures oa4rust behaviour into a
corpus of JSON fixtures, and provides a real (but currently stubbed) `diff` so that the
moment an o2server corpus exists, a genuine field-by-field comparison can run with no
further scaffolding work.

This is intentionally safe:

- It does **not** edit `tests/integration_runner.rs` or
  `tests/integration_tests/scenarios/mod.rs`.
- The new target `parity_runner` is fully independent of the existing `integration_runner`
  target; both can coexist and the green gate is unchanged.

---

## 2. What was built

### 2.1 `tests/parity_runner.rs` — separate test target

Mirrors `tests/integration_runner.rs` structure:

- A single `#[ignore]`d `#[test] fn parity_record()`:
  1. Calls `integration_tests::db::init_test_database()` → boots the disposable
     `oa4rust_test_<pid>` Postgres DB, runs migrations, seeds the admin + session
     (identical lifecycle to the integration harness).
  2. Gets the pool via `TestContext::pool()`.
  3. Boots the full app via `integration_tests::helpers::setup_test_server(pool)`
     → `(SocketAddr, JoinHandle, token)`.
  4. For each READ endpoint (see §2.3) issues a `GET` with `Authorization: Bearer <token>`
     and records `{ "path", "status", "body" }` into
     `tests/parity/corpus/oa4rust-<name>.json`.
- A second `#[ignore]`d `#[test] fn parity_compare()` that demonstrates COMPARE mode
  by loading each oa4rust corpus and diffing it against the (currently missing) o2server
  corpus via `parity::diff`.

To reuse the existing helpers verbatim **without** dragging the scenario module into this
target (which would otherwise run the pre-existing scenario tests), the file declares a
narrowed inline module:

```rust
mod integration_tests {
    pub mod db;
    pub mod helpers;
}
mod parity;
```

This compiles the *same* `tests/integration_tests/db.rs` and `tests/integration_tests/helpers.rs`
source files the integration runner uses — no edits to them — but keeps `parity_runner`
containing only parity tests.

### 2.2 `tests/parity/mod.rs` — compare module

Provides `pub fn diff(a: &serde_json::Value, b: &serde_json::Value) -> Vec<String>`:

- Performs a **real recursive, field-by-field** structural comparison (object keys,
  array lengths + elements, scalar values, and type mismatches) over `status`, `path`,
  and `body`.
- **o2server stub:** when `b` is the missing sentinel (`"o2server corpus not provided"`),
  `diff` returns exactly `vec!["o2server corpus not provided".into()]`.
- Helpers: `load_corpus(prefix, name)` and `compare_corpora(name)` (loads
  `oa4rust-<name>.json` vs `o2server-<name>.json`).
- Includes three `#[cfg(test)]` unit tests validating the stub path and the real
  diff (key/scalar mismatch detection + equal-pass). These are pure and run without
  any database.

### 2.3 Endpoints recorded (pure GET list endpoints, HTTP 200, no prior write)

| # | Name (corpus file) | Path | Basis |
|---|--------------------|------|-------|
| 1 | `cms_assemble_data_document` | `/jaxrs/cms_assemble_control/data/document` | proven 200 in `cms_document` scenario |
| 2 | `program_applications` | `/jaxrs/program/applications` | proven 200 in `program_center` scenario |
| 3 | `program_datastructure_modules_all` | `/jaxrs/program/datastructure/modules/all` | proven 200 in `program_center` scenario |
| 4 | `cms_assemble_appinfo` | `/jaxrs/cms_assemble_control/appinfo` | proven 200 in `cms_extended` scenario |

> Note: there are **no** `/jaxrs/organization/...` routes anywhere in the codebase, so
> the originally suggested `organization` list endpoint was substituted with the four
> proven GET list endpoints above. To add more, append to `ENDPOINTS` in
> `parity_runner.rs`.

### 2.4 `tests/parity/corpus/` — output directory

Created (with a `.gitkeep`). `parity_record` populates it with
`oa4rust-<name>.json` at runtime. The directory is part of the repo so the recorded
fixtures can be committed as a baseline.

---

## 3. How to run

All commands require a local PostgreSQL reachable via `DATABASE_URL`
(default `postgres://o2server:password@localhost:5432/postgres`), exactly like the
integration harness.

### 3.1 RECORD (capture the oa4rust corpus)

```bash
cargo test --test parity_runner -- --ignored --nocapture
```

This boots the app, replays the four READ endpoints, and writes:

```
tests/parity/corpus/oa4rust-cms_assemble_data_document.json
tests/parity/corpus/oa4rust-program_applications.json
tests/parity/corpus/oa4rust-program_datastructure_modules_all.json
tests/parity/corpus/oa4rust-cms_assemble_appinfo.json
```

Each file looks like:

```json
{
  "path": "/jaxrs/program/applications",
  "status": 200,
  "body": { "...": "..." }
}
```

### 3.2 Run the pure `diff` unit tests (no database needed)

```bash
cargo test --test parity_runner
```

Runs only the 3 `parity::tests::*` unit tests (3 passed; the 2 `#[ignore]`d parity tests
are skipped). This is safe to run in any environment.

---

## 4. How to run `diff` once an o2server corpus exists

1. Capture the equivalent o2server responses for the same endpoints and save them as
   `tests/parity/corpus/o2server-<name>.json` in the **same `{path, status, body}` shape**
   (e.g. via a Java-side test client, or by replaying recorded traffic).
2. Run:

   ```bash
   cargo test --test parity_runner parity_compare -- --ignored --nocapture
   ```

3. `parity::compare_corpora(name)` loads `oa4rust-<name>.json` and `o2server-<name>.json`,
   then calls `diff`. With a real o2server corpus present, `diff` performs the genuine
   field-by-field comparison and prints lines such as:

   ```
   DIFF: program_applications —
     - number mismatch at body.data.count: oa4rust=12 o2server=13
     - missing in o2server: body.data.data[3].flag
   ```

   When the o2server corpus is still absent, the output is simply:

   ```
   DIFF: program_applications —
     - o2server corpus not provided
   ```

To make `diff` fully live, remove the `is_missing_o2server` guard in `tests/parity/mod.rs`
and the comparison will already run end-to-end against the two JSON fixtures.

---

## 5. Verification

- `cargo check --test parity_runner` → **Finished** (only pre-existing warnings from the
  shared `tests/integration_tests/db.rs` helper; no errors introduced).
- `cargo test --test parity_runner` → 3 passed, 2 ignored, 0 failed.

No `cargo test --workspace`, `cargo build --workspace`, or
`cargo test --test integration_runner` was executed, per the task constraints.
