---
title: feat: DingTalk batch watering tool
type: feat
status: active
date: 2026-07-28
origin: docs/brainstorms/2026-07-28-dingtalk-batch-watering-tool-requirements.md
---

# feat: DingTalk Batch Watering Tool

## Summary

Build a standalone Go CLI/service that manages 100 DingTalk work accounts to trigger Ant Forest watering via group messages on a schedule, with manual trigger support and automatic gap-filling based on message delivery confirmation.

---

## Problem Frame

The user needs to maximize Ant Forest energy collection by having 100 DingTalk work accounts water their main account daily. The existing manual workflow—sending "1" and "2" messages in a DingTalk group to trigger two batches of ~50 accounts—is error-prone and misses time windows. This plan implements an automated tool that schedules the triggers, confirms delivery, and fills gaps.

---

## Requirements

From origin document `docs/brainstorms/2026-07-28-dingtalk-batch-watering-tool-requirements.md`:

- R1. Scheduled daily tasks send "1" and "2" group messages to trigger two fixed-batch watering cycles.
- R2. Task execution times are configurable, supporting multiple time windows.
- R3. Scheduled tasks support pause/resume without losing configuration.
- R4. Manual trigger entry point to send specified batch ("1" or "2") immediately, outside scheduled windows.
- R5. After each watering trigger, periodically detect watering status of each work account via the main account, identifying unfinished or unsuccessful batches.
- R6. Automatically resend the corresponding batch message to fill gaps, with configurable retry上限.
- R7. After each watering cycle completes (trigger + gap-filling), log the result (success/failure batches, retry count).
- R8. Maintain a list of 100 work accounts, each associated with its DingTalk sending capability configuration.
- R9. Group the 100 accounts into batches (~50 each), mapping each batch to its trigger message ("1" for batch 1, "2" for batch 2).
- R10. Log each scheduled task execution including trigger time, batches sent, retry count, and final result.
- R11. Log errors when a batch send fails, without interrupting subsequent batch execution.
- R12. View current configuration list and grouping status of the 100 work accounts.

**Origin actors:** A1. Main account owner (1 person), A2. 100 DingTalk work accounts
**Origin flows:** F1. Scheduled trigger and gap-fill cycle, F2. Manual trigger flow
**Origin acceptance examples:** AE1 (scheduled trigger with gap-filling), AE2 (manual trigger with failure handling), AE3 (pause/resume with logging)

---

## Scope Boundaries

- Single-user tool, not multi-tenant or SaaS
- Watering triggered only via DingTalk group messages; no direct Alipay API integration
- 100 work accounts are a fixed list; no dynamic group membership management
- No energy collection automation; only watering trigger and status closure
- No bulk registration or automatic login state acquisition for DingTalk accounts

---

## Context & Research

### Relevant Code and Patterns

- `csv2sql/` — existing Go project in the repo with standard Go module layout (`src/`, logging via `go.uber.org/zap`, environment-based config). Pattern to follow for project structure.
- `train/enhanceTXT.py` — Python logging pattern with dual file/console output, relevant for execution log design.
- No existing DingTalk integration code in this repo beyond o2oa (a full OA platform, not reused here).

### Institutional Learnings

No `docs/solutions/` directory exists. No prior automation or integration patterns to reference.

### External References

- DingTalk Open API: group message sending via `https://api.dingtalk.com/` (robot or group message interfaces)
- `robfig/cron` — standard Go cron scheduler
- `spf13/viper` — Go configuration management (env vars + config files)
- `go.uber.org/zap` — structured logging

---

## Key Technical Decisions

- **Language: Go** — repo already contains a Go project (`csv2sql/`) with established patterns; Go produces a single binary, easy to deploy and run as a service or CLI.
- **Sending method: DingTalk Open API** (see origin Key Decisions) — avoids maintaining 100 browser instances; requires one DingTalk app with group bot permissions.
- **Status detection: message delivery confirmation** (see origin Key Decisions) — tool considers a batch successful when the DingTalk API confirms the group message was sent to all target members. No UI scraping or energy-state inference.
- **Configuration: YAML file + environment overrides** — human-editable account list and batch grouping, with secrets (DingTalk app credentials) injected via env vars.
- **Deployment: CLI with optional background daemon** — runs as a scheduled process via systemd/cron, or as a foreground service; no complex orchestration required.

---

## Open Questions

### Resolved During Planning

- **Account sending method:** DingTalk Open API chosen over local multi-login. Rationale: lower operational burden, aligns with user's enterprise DingTalk setup.
- **Status detection:** Message delivery confirmation chosen over UI scraping or energy inference. Rationale: simplest reliable signal; if gap-filling proves insufficient later, detection can be upgraded without changing the messaging layer.

### Deferred to Implementation

- **Exact DingTalk API endpoint and permission model:** Which API (group robot vs. workflow message) depends on the user's DingTalk enterprise setup; implementer will select the appropriate endpoint during setup.
- **Token refresh strategy:** If the DingTalk app uses tenant access tokens with expiry, the refresh cadence and storage location will be determined during implementation.
- **Batch failure granularity:** Whether "partial send failure" (some of 50 accounts failed) triggers per-account retry or full-batch retry will be decided during implementation based on API error shapes.

---

## Output Structure

    dingtalk-watering/
    ├── go.mod
    ├── main.go
    ├── config.yaml.example
    ├── accounts/
    │   ├── accounts.go
    │   ├── group.go
    │   └── loader.go
    ├── dingtalk/
    │   ├── client.go
    │   └── sender.go
    ├── scheduler/
    │   ├── cron.go
    │   ├── task.go
    │   └── manager.go
    ├── waterloop/
    │   ├── detector.go
    │   ├── gapfill.go
    │   └── recorder.go
    ├── cli/
    │   ├── root.go
    │   ├── trigger.go
    │   ├── status.go
    │   └── config.go
    └── logs/
        └── execution.log

---

## Implementation Units

### U1. Project Scaffold and Configuration Management

**Goal:** Initialize Go module, establish configuration loading (YAML + env overrides), and set up structured logging.

**Requirements:** R8, R9, R12

**Dependencies:** None

**Files:**
- Create: `dingtalk-watering/go.mod`
- Create: `dingtalk-watering/main.go`
- Create: `dingtalk-watering/config.yaml.example`
- Create: `dingtalk-watering/internal/config/config.go`
- Modify: `dingtalk-watering/go.sum` (after `go mod tidy`)

**Approach:**
- Use `spf13/viper` for config: YAML file for account list and batch mapping, environment variables for DingTalk app credentials.
- Use `go.uber.org/zap` for structured logging to both file and console.
- Define config structs: `AppConfig` (top-level), `AccountConfig` (per-account: name, DingTalk user ID), `BatchConfig` (batch-to-message mapping).

**Technical design:**
```
Config structure:
- app:
    dingtalk_app_key: env DINGTALK_APP_KEY
    dingtalk_app_secret: env DINGTALK_APP_SECRET
    group_conversation_id: "xxx"
- accounts:
    - id: "worker001"
      name: "Account 1"
    - id: "worker002"
      name: "Account 2"
    ... (100 entries)
- batches:
    - message: "1"
      account_ids: ["worker001", ..., "worker050"]
    - message: "2"
      account_ids: ["worker051", ..., "worker100"]
- scheduler:
    cron_batch1: "0 9 * * *"
    cron_batch2: "0 18 * * *"
    max_retries: 3
    retry_interval: "5m"
```

**Patterns to follow:** `csv2sql/` Go module layout and logging patterns.

**Test scenarios:**
- Happy path: Load valid `config.yaml` with 100 accounts and 2 batches; all fields populate correctly.
- Edge case: Load config with missing optional fields (e.g., `max_retries`); defaults apply.
- Error path: Load config with invalid YAML; return descriptive error.
- Error path: Load config where batch references non-existent account ID; validation fails before startup.

**Verification:**
- `go build` succeeds, binary runs with `--help` and `--status` flags.
- `cp config.yaml.example config.yaml` and edit; tool loads without error.

---

### U2. DingTalk Message Sending Integration

**Goal:** Implement DingTalk Open API client to send group messages and track delivery status per account.

**Requirements:** R1, R4, R8, R11

**Dependencies:** U1

**Files:**
- Create: `dingtalk-watering/internal/dingtalk/client.go`
- Create: `dingtalk-watering/internal/dingtalk/sender.go`
- Test: `dingtalk-watering/internal/dingtalk/client_test.go`

**Approach:**
- Obtain tenant access token via app credentials.
- Send group messages using the appropriate API endpoint (robot message or group message, selected based on user's DingTalk setup).
- For each send call, record per-account delivery status (success/failure with error detail).
- On partial failure, continue to next batch without stopping (R11).

**Technical design:**
```
Client:
  - NewClient(appKey, appSecret) (*Client, error)
  - GetAccessToken(ctx) (string, error)
  - SendGroupMessage(ctx, conversationID, message, recipientUserIDs) (*SendResult, error)

SendResult:
  - FailedUserIDs: []string
  - ErrorMessages: map[string]string  // userID -> error
```

**Patterns to follow:** Standard Go HTTP client patterns from `csv2sql/src/http.go`.

**Test scenarios:**
- Happy path: Mock DingTalk API returns success for all recipients; `SendGroupMessage` returns empty `FailedUserIDs`.
- Edge case: API returns partial failure (some user IDs invalid); `FailedUserIDs` contains exactly those IDs.
- Error path: API returns 401 (invalid credentials); error is propagated and logged.
- Error path: Network timeout; error is recorded and batch continues.
- Covers AE2: Given 5 send failures in batch 2, logs error per failed ID but does not interrupt batch 2 flow.

**Verification:**
- Unit tests pass for client with mocked HTTP responses.
- Manual smoke test: tool sends a test message to a small test group.

---

### U3. Account Management and Batch Grouping

**Goal:** Load and validate the 100 work accounts, enforce batch groupings, and expose configuration status.

**Requirements:** R8, R9, R12

**Dependencies:** U1

**Files:**
- Create: `dingtalk-watering/internal/accounts/accounts.go`
- Create: `dingtalk-watering/internal/accounts/group.go`
- Create: `dingtalk-watering/internal/accounts/loader.go`
- Test: `dingtalk-watering/internal/accounts/accounts_test.go`

**Approach:**
- `AccountStore` loads accounts from config and provides lookup by ID.
- `BatchStore` loads batch definitions and validates that each batch's `account_ids` references exist in the account store.
- Expose a `Status()` method returning counts and batch summary for R12.

**Technical design:**
```
AccountStore:
  - Load(config *Config) error
  - Get(id string) (*Account, bool)
  - List() []Account
  - Count() int

BatchStore:
  - Load(config *Config, accounts *AccountStore) error
  - Get(message string) (*Batch, bool)
  - List() []Batch
  - Validate() error  // checks for duplicate accounts across batches
```

**Patterns to follow:** Standard Go repository pattern with interface definitions.

**Test scenarios:**
- Happy path: Load valid config; account count is 100, batch count is 2, each batch has ~50 accounts.
- Edge case: Account appears in multiple batches; validation warns or errors based on policy.
- Error path: Batch references unknown account ID; `Load` returns error with account ID detail.
- Edge case: Empty account list; `Count()` returns 0, `Status()` reports empty.

**Verification:**
- Unit tests pass for account loading, batch validation, and status reporting.

---

### U4. Scheduled Task Engine with Pause/Resume

**Goal:** Implement cron-based scheduling for multiple time windows, with pause/resume capability.

**Requirements:** R1, R2, R3

**Dependencies:** U2, U3

**Files:**
- Create: `dingtalk-watering/internal/scheduler/cron.go`
- Create: `dingtalk-watering/internal/scheduler/task.go`
- Create: `dingtalk-watering/internal/scheduler/manager.go`
- Test: `dingtalk-watering/internal/scheduler/cron_test.go`

**Approach:**
- Use `robfig/cron` v3 for cron parsing and triggering.
- `Scheduler` holds registered `Task` entries, each with a cron spec and an action function.
- `TaskManager` wraps `Scheduler` with pause/resume: paused tasks are skipped at trigger time without being removed.
- On trigger, execute batch send via `Sender`, then enqueue gap-fill detection.

**Technical design:**
```
Scheduler:
  - New() *Scheduler
  - Add(cronSpec string, job Job) error
  - Start() / Stop()

Job:
  - Run(ctx context.Context) error

TaskManager:
  - New(scheduler *Scheduler) *TaskManager
  - RegisterBatch(message string, cronSpec string, sendFunc SendFunc) error
  - Pause(message string) / Resume(message string)
  - IsPaused(message string) bool
```

**Patterns to follow:** Standard Go cron wrapper pattern; pause/resume via flag in task metadata.

**Test scenarios:**
- Happy path: Register task with cron "*/5 * * * *"; trigger fires at next 5-minute boundary.
- Edge case: Pause task; trigger time arrives but job is not executed. Resume; trigger resumes.
- Edge case: Multiple tasks with different cron specs; each fires independently.
- Error path: Invalid cron spec; `Add` returns error immediately.

**Verification:**
- Unit tests with accelerated time or mock clock validate scheduling behavior.
- Integration test: run scheduler with 1-second cron, verify job executes.

---

### U5. Gap-Fill Detection and Closure Loop

**Goal:** After each watering trigger, detect incomplete batches and automatically resend until closure or retry limit reached.

**Requirements:** R5, R6, R7, R10

**Dependencies:** U2, U4

**Files:**
- Create: `dingtalk-watering/internal/waterloop/detector.go`
- Create: `dingtalk-watering/internal/waterloop/gapfill.go`
- Create: `dingtalk-watering/internal/waterloop/recorder.go`
- Test: `dingtalk-watering/internal/waterloop/gapfill_test.go`

**Approach:**
- `Detector` examines the `SendResult` from the initial trigger; accounts in `FailedUserIDs` are marked as needing retry.
- `GapFiller` retries the batch (or targeted subset) up to `max_retries` with `retry_interval` delay between attempts.
- `Recorder` persists a `CycleRecord` (timestamp, batches, initial send results, retry attempts, final status) to a local log file.
- A cycle is "closed" when either all accounts have succeeded or retries are exhausted.

**Technical design:**
```
CycleRecord:
  - ID: string (UUID)
  - TriggeredAt: time.Time
  - TriggeredBy: "scheduled" | "manual"
  - Batches: []BatchResult
  - Retries: int
  - ClosedAt: time.Time
  - Status: "success" | "partial" | "failed"

BatchResult:
  - Message: string ("1" | "2")
  - SendResult: *dingtalk.SendResult
  - RetryCount: int
```

**Patterns to follow:** Standard Go retry pattern with backoff; append-only log file.

**Test scenarios:**
- Happy path: Initial send succeeds for all accounts; no retries; cycle records `success`.
- Happy path: 8 accounts fail initially, retry succeeds for all; cycle records `success` with retry count 1.
- Edge case: Retry fails for same accounts after 3 attempts; cycle records `partial` with failed user IDs.
- Error path: Retry encounters API error; error is logged, retry count increments, loop continues until上限.
- Covers AE1: Given 9:00 trigger for batch "1", 8 failures, one retry succeeds, cycle closed with success.

**Verification:**
- Unit tests for detector and gap-filler with mocked sender.
- Cycle log file is created and parseable after a full run.

---

### U6. CLI Entrypoint and Execution Logging

**Goal:** Provide command-line interface for manual trigger, status viewing, and configuration inspection.

**Requirements:** R4, R10, R12

**Dependencies:** U1, U5

**Files:**
- Create: `dingtalk-watering/cmd/root.go`
- Create: `dingtalk-watering/cmd/trigger.go`
- Create: `dingtalk-watering/cmd/status.go`
- Create: `dingtalk-watering/cmd/config.go`
- Create: `dingtalk-watering/internal/cli/runner.go`

**Approach:**
- Use `spf13/cobra` for CLI structure.
- Commands:
  - `watering trigger --batch 1|2` — manually trigger a batch (R4).
  - `watering status` — show current account/batch config and last cycle results (R12, R10).
  - `watering config check` — validate config file and report issues.
- Execution logging (R10) is handled by `waterloop.Recorder` and also surfaced in `watering status`.

**Patterns to follow:** Cobra CLI pattern; status command formats output as tables.

**Test scenarios:**
- Happy path: `watering trigger --batch 1` initiates manual send for batch 1.
- Edge case: `watering trigger --batch 1` when batch 1 is already in progress; returns error or queues.
- Happy path: `watering status` prints account count, batch summary, and last cycle result.
- Error path: `watering trigger --batch 3` (invalid); CLI returns usage error.

**Verification:**
- CLI builds and runs; all commands show help text.
- Manual trigger sends a real message when configured with valid credentials.

---

## System-Wide Impact

- **Interaction graph:** Standalone binary; no callbacks into existing repo systems. Only external dependency is DingTalk Open API.
- **Error propagation:** Send failures are logged per-batch; retry logic contains failures within the gap-fill loop. No cross-service propagation.
- **State lifecycle risks:** Execution logs append to a local file; log rotation not implemented in v1 (acceptable for single-user, low-volume use). Token state (if DingTalk access token is cached) is in-memory; process restart re-acquires.
- **API surface parity:** None — this is a new standalone tool.
- **Integration coverage:** End-to-end flow (CLI trigger → DingTalk send → gap-fill → log) requires integration test against a real DingTalk group to fully validate; unit tests cover logic with mocks.

---

## Risks & Dependencies

| Risk | Mitigation |
|------|------------|
| DingTalk API permission setup requires enterprise admin | User must create app and grant group bot permissions; plan documents setup steps in operational notes. |
| Access token expiry or rate limiting | Implementer will add token refresh and backoff during implementation; retry interval config provides initial buffer. |
| Partial batch failure (some accounts unreachable) | Gap-fill retries target only failed accounts; after max retries, status is logged for user review. |
| 100-account config maintenance burden | CLI `config check` validates on load; future enhancement could add interactive add/remove commands. |

---

## Documentation / Operational Notes

- Include setup guide: how to create DingTalk app, get app key/secret, configure group bot, obtain `group_conversation_id`.
- Include config example with all 100 accounts in batch structure.
- Note: first run requires manual `cp config.yaml.example config.yaml` and editing.

---

## Sources & References

- **Origin document:** [docs/brainstorms/2026-07-28-dingtalk-batch-watering-tool-requirements.md](../brainstorms/2026-07-28-dingtalk-batch-watering-tool-requirements.md)
- Related code: `csv2sql/src/http.go` (Go HTTP client pattern), `csv2sql/go.mod` (Go module setup)
- External docs: DingTalk Open Platform API reference, `robfig/cron` v3 docs, `spf13/viper` docs
