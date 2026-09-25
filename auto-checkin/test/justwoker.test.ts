// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import assert from "node:assert/strict";
import { test } from "node:test";
import { classifyUnavailableBalance } from "../src/sites/justwoker.js";

test("API success is authoritative when balances are unavailable", () => {
  assert.equal(classifyUnavailableBalance(true, false, false), "success");
});

test("explicit already response is authoritative when balances are unavailable", () => {
  assert.equal(classifyUnavailableBalance(false, true, false), "already");
});

test("a confirmed profile trigger is sufficient when the API is absent", () => {
  assert.equal(classifyUnavailableBalance(false, false, true), "already");
});

test("unknown state is a failure", () => {
  assert.equal(classifyUnavailableBalance(false, false, false), "failed");
});
