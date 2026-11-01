// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import assert from "node:assert/strict";
import { test } from "node:test";
import { vyceaiAdapter } from "../src/sites/vyceai.js";
import { FakePage, makeAdapterCtx } from "./fake-page.js";
import type { AdapterOutcome } from "../src/types.js";
import type { FetchStubResult } from "./fake-page.js";

async function runCheckin(page: FakePage, overrides?: Parameters<typeof makeAdapterCtx>[1]) {
  const ctx = makeAdapterCtx(page, overrides);
  const outcome: AdapterOutcome = await vyceaiAdapter.checkin(ctx);
  return { outcome, ctx };
}

const DASHBOARD = "user/dashboard";

test("已登录且今日已领 → already（带连续天数）", async () => {
  const page = new FakePage({
    fetchRoutes: {
      [DASHBOARD]: { status: 200, body: JSON.stringify({ user: { id: 7, totalBalance: 1.5 } }) },
      "user/daily-reward": { status: 200, body: JSON.stringify({ canClaim: false, streak: 3, lastClaim: "2026-10-28" }) },
    },
  });

  const { outcome } = await runCheckin(page);
  assert.equal(outcome.status, "already");
  assert.equal(outcome.reward, 0);
  assert.equal(outcome.balance, 1.5);
  assert.match(outcome.message, /连续3天/);
});

test("领取成功：以复查后的 canClaim=false 为准，余额轮询到更新值", async () => {
  let balance = 1.5;
  let claimed = false;
  const page = new FakePage({
    fetchRoutes: {
      // 每查一次 dashboard 余额涨一点：领取前读到 2.0，复查轮询读到 >2.0 即收敛
      [DASHBOARD]: (): FetchStubResult => {
        balance = Math.min(3, balance + 0.25);
        return { status: 200, body: JSON.stringify({ user: { id: 7, totalBalance: Number(balance.toFixed(4)) } }) };
      },
      "user/daily-reward/claim": { status: 200, body: JSON.stringify({ amount: 0.5, newBalance: 3 }) },
      // GET 状态与 POST 领取都含 "user/daily-reward"，靠最长子串匹配区分；
      // 状态路由第一次返回可领取，复查时返回已领——对齐「领取落账」的真实时序
      "user/daily-reward": (): FetchStubResult =>
        claimed
          ? { status: 200, body: JSON.stringify({ canClaim: false, streak: 4 }) }
          : ((claimed = true), { status: 200, body: JSON.stringify({ canClaim: true, streak: 3, rewardAmount: 0.4 }) }),
    },
  });

  const { outcome } = await runCheckin(page);
  assert.equal(outcome.status, "success");
  assert.equal(outcome.reward, 0.5);
  assert.equal(outcome.balance, 2.25);
  assert.match(outcome.message, /领取成功 \+0\.5 · 连续4天/);
});

test("领取端点被 SPA 兜底成 HTML（路径又变了）→ 显式失败而非假装成功", async () => {
  const page = new FakePage({
    fetchRoutes: {
      [DASHBOARD]: { status: 200, body: JSON.stringify({ user: { id: 7, totalBalance: 1.5 } }) },
      "user/daily-reward/claim": { status: 200, body: "<!doctype html><html>index</html>" },
      "user/daily-reward": { status: 200, body: JSON.stringify({ canClaim: true, streak: 3, rewardAmount: 0.4 }) },
    },
  });

  const { outcome, ctx } = await runCheckin(page);
  assert.equal(outcome.status, "failed");
  assert.match(outcome.message, /领取端点返回 HTML/);
  assert.equal(ctx.screenshots.at(-1), "vyceai-claim-endpoint-moved");
});

test("HTTP 200 但 canClaim 未变 → 领取未生效（带接口报错透传）", async () => {
  const page = new FakePage({
    fetchRoutes: {
      [DASHBOARD]: { status: 200, body: JSON.stringify({ user: { id: 7, totalBalance: 1.5 } }) },
      "user/daily-reward/claim": { status: 200, body: JSON.stringify({ error: { message: "PoW 校验失败" } }) },
      "user/daily-reward": { status: 200, body: JSON.stringify({ canClaim: true, streak: 3, lastClaim: "2026-10-28" }) },
    },
  });

  const { outcome } = await runCheckin(page);
  assert.equal(outcome.status, "failed");
  assert.match(outcome.message, /领取未生效/);
  assert.match(outcome.message, /接口报错：PoW 校验失败/);
  assert.equal(page.fetchCalls.filter((c) => c.url.includes("/claim")).length, 1);
});

test("领取状态本身读不到（非 JSON 响应）→ failed", async () => {
  const page = new FakePage({
    fetchRoutes: {
      [DASHBOARD]: { status: 200, body: JSON.stringify({ user: { id: 7, totalBalance: 1.5 } }) },
      "user/daily-reward": { status: 502, body: "bad gateway" },
    },
  });

  const { outcome } = await runCheckin(page);
  assert.equal(outcome.status, "failed");
  assert.match(outcome.message, /无法读取每日奖励状态/);
});

test("未登录时走表单登录：找不到登录按钮也要继续轮询，最终失败", async () => {
  const page = new FakePage({
    fetchRoutes: {
      // dashboard 一直返回未登录（登录按钮缺失，PoW 也无从完成）
      [DASHBOARD]: { status: 200, body: JSON.stringify({ user: null }) },
    },
  });

  const { outcome, ctx } = await runCheckin(page);
  assert.equal(outcome.status, "failed");
  assert.match(outcome.message, /登录失败/);
  assert.equal(ctx.screenshots.at(-1), "vyceai-login-fail");
  assert.ok(ctx.logs.some((l) => l.includes("未找到登录按钮")));
});

test("未登录 → 点登录按钮 → 轮询到登录态 → 继续领取流程", async () => {
  let polls = 0;
  let loggedIn = false;
  const page = new FakePage({
    locators: { "role:button:^(Login|登录)$": { count: 1 } },
    fetchRoutes: {
      [DASHBOARD]: (): FetchStubResult => {
        polls++;
        if (polls >= 2) loggedIn = true; // ensureLogin 首探失败后，表单提交 → 轮询命中
        return loggedIn
          ? { status: 200, body: JSON.stringify({ user: { id: 7, totalBalance: 3 } }) }
          : { status: 200, body: JSON.stringify({ user: null }) };
      },
      "user/daily-reward": { status: 200, body: JSON.stringify({ canClaim: false, streak: 9 }) },
    },
  });

  const { outcome, ctx } = await runCheckin(page);
  assert.equal(outcome.status, "already");
  assert.equal(outcome.balance, 3);
  assert.ok(ctx.logs.some((l) => l.includes("登录成功")), "应记录登录成功");
});

test("evaluate 抛异常 → 收敛为 failed（带截图）", async () => {
  const page = new FakePage();
  page.script(() => {
    throw new Error("pow exploded");
  });

  const { outcome, ctx } = await runCheckin(page);
  assert.equal(outcome.status, "failed");
  assert.match(outcome.message, /异常：pow exploded/);
  assert.equal(ctx.screenshots.at(-1), "vyceai-error");
});
