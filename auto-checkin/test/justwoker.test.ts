// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import assert from "node:assert/strict";
import { test } from "node:test";
import { classifyUnavailableBalance, justwokerAdapter } from "../src/sites/justwoker.js";
import { FakePage, makeAdapterCtx, newApiSelf } from "./fake-page.js";
import type { AdapterOutcome } from "../src/types.js";

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

/** fetchSelf 的 evaluate 返回形状：{user, wafBlocked} */
function selfResult(quota: number | null, username = "tester") {
  return { user: { quota, username }, wafBlocked: false };
}

async function runCheckin(page: FakePage): Promise<{ outcome: AdapterOutcome; ctx: ReturnType<typeof makeAdapterCtx> }> {
  const ctx = makeAdapterCtx(page);
  const outcome = await justwokerAdapter.checkin(ctx);
  return { outcome, ctx };
}

test("访问领取成功：余额差值换算成美元奖励", async () => {
  const page = new FakePage();
  // oauthLogin 复用会话（fetchSelf 命中）→ apiCheckIn 成功 → 领取后再读余额
  page
    .script(selfResult(100_000))
    .script({ status: 200, json: { success: true, message: "领取成功" } })
    .script(selfResult(600_000));

  const { outcome } = await runCheckin(page);
  assert.equal(outcome.status, "success");
  assert.equal(outcome.reward, 1); // (600000-100000)/500000
  assert.equal(outcome.balance, 1.2);
  assert.equal(outcome.currency, "$");
  assert.match(outcome.message, /访问领取 \+1\$/);
});

test("余额未增时判为 already", async () => {
  const page = new FakePage();
  page
    .script(selfResult(100_000))
    .script({ status: 200, json: { success: true, message: "ok" } })
    .script(selfResult(100_000));

  const { outcome } = await runCheckin(page);
  assert.equal(outcome.status, "already");
  assert.equal(outcome.reward, 0);
  assert.equal(outcome.balance, 0.2);
});

test("领取后余额异常下降 → failed 且不留悬案", async () => {
  const page = new FakePage();
  page
    .script(selfResult(600_000))
    .script({ status: 200, json: { success: true, message: "ok" } })
    .script(selfResult(100_000));

  const { outcome, ctx } = await runCheckin(page);
  assert.equal(outcome.status, "failed");
  assert.match(outcome.message, /余额异常下降 1\$/);
  assert.equal(ctx.screenshots.at(-1), "justwoker-balance-decreased");
});

test("签到接口硬失败（非 already 语义）→ failed", async () => {
  const page = new FakePage();
  page
    .script(selfResult(100_000))
    .script({ status: 500, json: { success: false, message: "出错了" } });

  const { outcome, ctx } = await runCheckin(page);
  assert.equal(outcome.status, "failed");
  assert.match(outcome.message, /签到接口失败：出错了/);
  assert.equal(ctx.screenshots.at(-1), "justwoker-api-failed");
});

test("接口报已领取语义时不算硬失败，继续读余额判 already", async () => {
  const page = new FakePage();
  page
    .script(selfResult(100_000))
    .script({ status: 200, json: { success: false, message: "今日已领取" } })
    .script(selfResult(100_000));

  const { outcome } = await runCheckin(page);
  assert.equal(outcome.status, "already");
});

test("两边余额都读不到：接口确认成功 → success（余额未知）", async () => {
  const page = new FakePage();
  page
    .script(selfResult(null))
    .script({ status: 200, json: { success: true, message: "ok" } })
    .script({ user: null, wafBlocked: false });

  const { outcome } = await runCheckin(page);
  assert.equal(outcome.status, "success");
  assert.match(outcome.message, /余额接口不可用/);
});

test("两边余额都读不到：接口报已领取 → already", async () => {
  const page = new FakePage();
  page
    .script(selfResult(null))
    .script({ status: 200, json: { success: false, message: "已领取" } })
    .script({ user: null, wafBlocked: false });

  const { outcome } = await runCheckin(page);
  assert.equal(outcome.status, "already");
});

test("两边余额都读不到且 profile 落点不对 → 状态未知 failed", async () => {
  const page = new FakePage({
    url: "https://api.justwoker.icu/panel",
    // profile 落点被重定向走（SPA 兜底/风控），profileConfirmed = false
    gotoUrls: { "/profile": "https://api.justwoker.icu/parked" },
  });
  page
    .script(selfResult(null))
    .script({ status: 404, json: null })
    .script({ user: null, wafBlocked: false });

  const { outcome, ctx } = await runCheckin(page);
  assert.equal(outcome.status, "failed");
  assert.match(outcome.message, /状态无法确认/);
  assert.equal(ctx.screenshots.at(-1), "justwoker-state-unknown");
});

test("fetchSelf 未命中且受保护页全部被踢回登录页 → 视为未登录失败", async () => {
  const page = new FakePage({ url: "https://api.justwoker.icu/login" });
  page.script({ user: null, wafBlocked: false }); // oauthLogin 的复用探测

  const { outcome, ctx } = await runCheckin(page);
  assert.equal(outcome.status, "failed");
  assert.match(outcome.message, /^登录失败$/);
  assert.equal(ctx.screenshots.at(-1), "justwoker-login-fail");
});

test("OAuth 按钮存在但 GitHub 会话停在登录页 → 提示手动登录", async () => {
  const page = new FakePage({
    url: "https://api.justwoker.icu/login",
    locators: { "role:button:GitHub": { count: 1 } },
    popups: [new FakePage({ url: "https://github.com/login" })],
  });
  page.script({ user: null, wafBlocked: false }); // 复用探测失败

  const { outcome } = await runCheckin(page);
  assert.equal(outcome.status, "failed");
  assert.match(outcome.message, /npm run login -- justwoker/);
});

test("evaluate 抛异常 → 收敛为 failed（带截图）", async () => {
  const page = new FakePage();
  page.script(() => {
    throw new Error("boom");
  });

  const { outcome, ctx } = await runCheckin(page);
  assert.equal(outcome.status, "failed");
  assert.match(outcome.message, /异常：boom/);
  assert.equal(ctx.screenshots.at(-1), "justwoker-error");
});
