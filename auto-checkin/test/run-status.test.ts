// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import assert from "node:assert/strict";
import { test } from "node:test";
import { shouldFailRun } from "../src/run-status.js";
import type { CheckinResult } from "../src/types.js";

function result(status: CheckinResult["status"], message = ""): CheckinResult {
  return {
    siteId: "test",
    siteName: "Test",
    timestamp: "2026-01-01T00:00:00.000Z",
    status,
    reward: null,
    balance: null,
    currency: null,
    message,
    screenshot: null,
    durationMs: 0,
  };
}

test("empty run is a failed scheduled run", () => {
  assert.equal(shouldFailRun([]), true);
});

test("one failed site makes the scheduled run fail", () => {
  assert.equal(shouldFailRun([result("success"), result("failed")]), true);
});

test("successful, already, and intentionally disabled sites do not fail the run", () => {
  assert.equal(
    shouldFailRun([result("success"), result("already"), result("skipped", "配置中已禁用")]),
    false,
  );
});

test("an enabled site skipped for missing credentials fails the run", () => {
  assert.equal(shouldFailRun([result("skipped", "未配置账号密码")]), true);
});
