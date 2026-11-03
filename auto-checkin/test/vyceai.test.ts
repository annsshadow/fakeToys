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
  // 第 41 轮：登录态探测（ensureLogin 的 loggedInState）与余额读取合并为
  // 一次 dashboard 请求 —— 领取前余额 = 首次探测值 1.75，复查第一次
  // （2.0）即超过它，轮询收敛到 2.0（旧实现多拉一次余额，收敛到 2.25）。
  assert.equal(outcome.balance, 2.0);
  assert.match(outcome.message, /领取成功 \+0\.5 · 连续4天/);
});

test("领取后落账延迟：前几次复查 canClaim 仍 true，轮询到 false 才判成功", async () => {
  let rewardPolls = 0;
  let balance = 1.5;
  const page = new FakePage({
    fetchRoutes: {
      [DASHBOARD]: (): FetchStubResult => {
        balance = Math.min(3, balance + 0.25);
        return { status: 200, body: JSON.stringify({ user: { id: 7, totalBalance: Number(balance.toFixed(4)) } }) };
      },
      "user/daily-reward/claim": { status: 200, body: JSON.stringify({ amount: 0.5, newBalance: 3 }) },
      // 前 3 次复查仍显示可领取（服务端落账延迟），第 4 次才变 false
      "user/daily-reward": (): FetchStubResult => {
        rewardPolls++;
        return rewardPolls <= 3
          ? { status: 200, body: JSON.stringify({ canClaim: true, streak: 3, rewardAmount: 0.4 }) }
          : { status: 200, body: JSON.stringify({ canClaim: false, streak: 4 }) };
      },
    },
  });

  const { outcome, ctx } = await runCheckin(page);
  assert.equal(outcome.status, "success");
  assert.equal(outcome.reward, 0.5);
  assert.match(outcome.message, /领取成功 \+0\.5 · 连续4天/);
  // 轮询日志应记录延迟落账，而非直接判失败
  assert.ok(ctx.logs.some((l) => l.includes("第 1 次复查") && l.includes("仍为 true")));
});

test("领取后 30s 内 canClaim 始终 true → 判未生效（保留最终状态）", async () => {
  const page = new FakePage({
    fetchRoutes: {
      [DASHBOARD]: { status: 200, body: JSON.stringify({ user: { id: 7, totalBalance: 1.5 } }) },
      "user/daily-reward/claim": { status: 200, body: JSON.stringify({ success: true, amount: 0.5 }) },
      // 复查永远 true：模拟服务端一直没落账
      "user/daily-reward": { status: 200, body: JSON.stringify({ canClaim: true, streak: 3, lastClaim: "2026-10-28" }) },
    },
  });

  const { outcome, ctx } = await runCheckin(page);
  assert.equal(outcome.status, "failed");
  assert.match(outcome.message, /领取未生效/);
  assert.match(outcome.message, /30s 内未变 false/);
  assert.equal(ctx.screenshots.at(-1), "vyceai-claim-ineffective");
});

test("领取响应缺 amount 时用状态接口的 rewardAmount 兜底", async () => {
  let polled = false;
  const page = new FakePage({
    fetchRoutes: {
      [DASHBOARD]: { status: 200, body: JSON.stringify({ user: { id: 7, totalBalance: 1.5 } }) },
      // 领取响应只有 success，没有 amount
      "user/daily-reward/claim": { status: 200, body: JSON.stringify({ success: true }) },
      // 第一次读状态：可领取（含 rewardAmount）；复查：已领取
      "user/daily-reward": (): FetchStubResult => {
        if (polled) return { status: 200, body: JSON.stringify({ canClaim: false, streak: 4 }) };
        polled = true;
        return { status: 200, body: JSON.stringify({ canClaim: true, streak: 3, rewardAmount: 0.4 }) };
      },
    },
  });

  const { outcome } = await runCheckin(page);
  assert.equal(outcome.status, "success");
  assert.equal(outcome.reward, 0.4);
  assert.match(outcome.message, /领取成功 \+0\.4 · 连续4天/);
});

test("领取响应和状态接口都缺奖励数值 → reward 为 null 但成功", async () => {
  let polled = false;
  const page = new FakePage({
    fetchRoutes: {
      [DASHBOARD]: { status: 200, body: JSON.stringify({ user: { id: 7, totalBalance: 1.5 } }) },
      // 领取响应无 amount，状态接口也无 rewardAmount
      "user/daily-reward/claim": { status: 200, body: JSON.stringify({ success: true }) },
      "user/daily-reward": (): FetchStubResult => {
        if (polled) return { status: 200, body: JSON.stringify({ canClaim: false, streak: 4 }) };
        polled = true;
        return { status: 200, body: JSON.stringify({ canClaim: true, streak: 3 }) };
      },
    },
  });

  const { outcome } = await runCheckin(page);
  assert.equal(outcome.status, "success");
  assert.equal(outcome.reward, null);
  assert.equal(outcome.message, "领取成功 +? · 连续4天");
});

test("余额复查始终不涨 → readBalanceAfterClaim 轮询到超时仍返回末次余额", async () => {
  let polled = false;
  const page = new FakePage({
    fetchRoutes: {
      // 余额恒定不变：模拟服务端余额延迟落账但一直不涨
      [DASHBOARD]: { status: 200, body: JSON.stringify({ user: { id: 7, totalBalance: 1.5 } }) },
      "user/daily-reward/claim": { status: 200, body: JSON.stringify({ amount: 0.5, newBalance: 2 }) },
      // 第一次读状态：可领取；复查：已领取（余额却不涨，触发轮询超时）
      "user/daily-reward": (): FetchStubResult => {
        if (polled) return { status: 200, body: JSON.stringify({ canClaim: false, streak: 4 }) };
        polled = true;
        return { status: 200, body: JSON.stringify({ canClaim: true, streak: 3, rewardAmount: 0.4 }) };
      },
    },
  });

  const { outcome } = await runCheckin(page);
  assert.equal(outcome.status, "success");
  // 余额不涨，轮询超时返回末次余额（1.5），而非领取响应里的 newBalance
  assert.equal(outcome.balance, 1.5);
  assert.match(outcome.message, /领取成功 \+0\.5 · 连续4天/);
});

test("领取接口网络异常 → 透传接口报错", async () => {
  const page = new FakePage({
    fetchRoutes: {
      [DASHBOARD]: { status: 200, body: JSON.stringify({ user: { id: 7, totalBalance: 1.5 } }) },
      // 领取时网络异常：模拟 fetch 抛错（服务端不可达/连接被重置）
      "user/daily-reward/claim": (): FetchStubResult => {
        throw new TypeError("Failed to fetch");
      },
      "user/daily-reward": { status: 200, body: JSON.stringify({ canClaim: true, streak: 3, rewardAmount: 0.4 }) },
    },
  });

  const { outcome } = await runCheckin(page);
  assert.equal(outcome.status, "failed");
  assert.match(outcome.message, /领取未生效/);
  assert.match(outcome.message, /接口报错：TypeError: Failed to fetch/);
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
