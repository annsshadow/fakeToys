// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import assert from "node:assert/strict";
import { test } from "node:test";
import type { Page } from "playwright";
import { logout } from "../src/sites/newapi-client.js";

/**
 * AgentRouter 的奖励在"重新登录"时发放，所以必须先退干净。
 * 但 /api/user/self 被阿里云 WAF 保护，放行凭证 acw_tc 必须活过这次退出，
 * 否则领取后余额读不到，会误判成签到失败。
 */
test("keeps WAF clearance cookies while clearing the site session", async () => {
  const cleared: Array<Record<string, unknown>> = [];
  const cookies = [
    { name: "session", domain: "agentrouter.org", path: "/" },
    { name: "acw_tc", domain: "agentrouter.org", path: "/" },
    { name: "_c_WBKFRo", domain: "agentrouter.org", path: "/" },
  ];
  const ctx = {
    cookies: async () => cookies,
    clearCookies: async (filter: Record<string, unknown>) => {
      cleared.push(filter);
    },
  };
  const page = {
    context: () => ctx,
    evaluate: async () => {},
  } as unknown as Page;

  await logout(page, "https://agentrouter.org");

  assert.deepEqual(
    cleared.map((c) => c.name),
    ["session"],
    "只应清站点会话 cookie",
  );
});

test("keeps the Cloudflare clearance cookie too", async () => {
  const cleared: Array<Record<string, unknown>> = [];
  const ctx = {
    cookies: async () => [
      { name: "session", domain: ".vyceai.com", path: "/" },
      { name: "cf_clearance", domain: ".vyceai.com", path: "/" },
      { name: "_vfp_s", domain: "vyceai.com", path: "/" },
    ],
    clearCookies: async (filter: Record<string, unknown>) => {
      cleared.push(filter);
    },
  };
  const page = {
    context: () => ctx,
    evaluate: async () => {},
  } as unknown as Page;

  await logout(page, "https://vyceai.com");

  const names = cleared.map((c) => c.name);
  assert.ok(names.includes("session"));
  assert.ok(!names.includes("cf_clearance"));
});

test("never touches third-party cookies such as the GitHub session", async () => {
  const cleared: Array<Record<string, unknown>> = [];
  const ctx = {
    cookies: async () => [
      { name: "session", domain: "agentrouter.org", path: "/" },
      { name: "user_session", domain: "github.com", path: "/" },
    ],
    clearCookies: async (filter: Record<string, unknown>) => {
      cleared.push(filter);
    },
  };
  const page = {
    context: () => ctx,
    evaluate: async () => {},
  } as unknown as Page;

  await logout(page, "https://agentrouter.org");

  assert.deepEqual(
    cleared.map((c) => c.name),
    ["session"],
  );
});