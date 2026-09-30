// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import assert from "node:assert/strict";
import { test } from "node:test";
import type { Page } from "playwright";
import { gotoWithNavigationRetry } from "../src/sites/newapi-client.js";

test("retries a navigation interrupted by the SPA", async () => {
  let gotoCalls = 0;
  let waitCalls = 0;
  const page = {
    goto: async () => {
      gotoCalls++;
      if (gotoCalls === 1) throw new Error("Navigation is interrupted by another navigation");
    },
    waitForLoadState: async () => {},
    waitForTimeout: async () => {
      waitCalls++;
    },
  } as unknown as Page;

  await gotoWithNavigationRetry(page, "https://example.test/login");

  assert.equal(gotoCalls, 2);
  assert.equal(waitCalls, 1);
});

test("allows two transient SPA navigation races before succeeding", async () => {
  let gotoCalls = 0;
  const page = {
    goto: async () => {
      gotoCalls++;
      if (gotoCalls < 3) throw new Error("Navigation is interrupted by another navigation");
    },
    waitForLoadState: async () => {},
    waitForTimeout: async () => {},
  } as unknown as Page;

  await gotoWithNavigationRetry(page, "https://example.test/login");

  assert.equal(gotoCalls, 3);
});

test("does not hide an unrelated navigation error", async () => {
  let gotoCalls = 0;
  const page = {
    goto: async () => {
      gotoCalls++;
      throw new Error("connection reset");
    },
    waitForLoadState: async () => {},
    waitForTimeout: async () => {},
  } as unknown as Page;

  await assert.rejects(
    () => gotoWithNavigationRetry(page, "https://example.test/login"),
    /connection reset/,
  );
  assert.equal(gotoCalls, 1);
});
