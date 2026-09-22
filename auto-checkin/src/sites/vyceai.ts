// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import type { Page } from "playwright";
import type { SiteAdapter, AdapterContext, AdapterOutcome } from "../types.js";

const BASE_URL = "https://vyceai.com";

/**
 * 站点 1: VyceAI —— 点击签到即可领取。
 *
 * ⚠️ VyceAI 不是 new-api，而是自定义 API，且登录有反爬（CSRF + 工作量证明 PoW）：
 *   - POST /user/login               登录（返回 user.totalBalance，单位美元）
 *   - GET  /user/dashboard           读取余额
 *   - GET  /user/daily-reward        领取状态 {canClaim, streak, lastClaim, rewardAmount}
 *   - POST /user/daily-reward        领取每日奖励（canClaim=true 时）
 *
 * 因 PoW 无法用裸接口复现，必须用真实浏览器让页面 SPA 自己完成登录（含 PoW），
 * 登录后再调 /user/* 端点。余额直接是美元，不做 quota 换算。
 */
export const vyceaiAdapter: SiteAdapter = {
  id: "vyceai",
  name: "VyceAI",
  baseUrl: BASE_URL,
  authType: "password",

  async checkin(ctx: AdapterContext): Promise<AdapterOutcome> {
    const page = await ctx.browserContext.newPage();
    try {
      const logged = await ensureLogin(page, ctx);
      if (!logged) {
        const shot = await ctx.saveScreenshot(page, "vyceai-login-fail");
        return fail("登录失败（可能凭据错误或 PoW 未通过）", shot);
      }

      const reward = await getJson(page, "/user/daily-reward");
      const balance = await readBalance(page);
      if (!reward) {
        return { status: "failed", reward: null, balance, currency: "$", message: "无法读取每日奖励状态", screenshot: null };
      }
      ctx.log(`daily-reward: canClaim=${reward.canClaim} streak=${reward.streak} lastClaim=${reward.lastClaim}`);

      if (reward.canClaim === false) {
        return { status: "already", reward: 0, balance, currency: "$", message: `今日已领 · 连续${reward.streak ?? "?"}天`, screenshot: null };
      }

      const claim = await postJson(page, "/user/daily-reward");
      const got = typeof reward.rewardAmount === "number" ? reward.rewardAmount : null;
      ctx.log(`领取返回：${JSON.stringify(claim).slice(0, 160)}`);

      // HTTP 200 不代表落账：曾出现连续多天 200 但 canClaim/streak/lastClaim 纹丝不动（实际未领取，还导致断签）。
      // 必须复查 daily-reward 状态，canClaim 变 false 才算真成功。
      const after = await getJson(page, "/user/daily-reward");
      if (after && after.canClaim === false) {
        const newBalance = await readBalanceAfterClaim(page, balance);
        return {
          status: "success",
          reward: got,
          balance: newBalance ?? balance,
          currency: "$",
          message: `领取成功 +${got ?? "?"} · 连续${after.streak ?? "?"}天`,
          screenshot: null,
        };
      }
      const shot = await ctx.saveScreenshot(page, "vyceai-claim-ineffective");
      return {
        status: "failed",
        reward: null,
        balance,
        currency: "$",
        message: `领取未生效（返回 200 但 canClaim 仍为 true，streak=${after?.streak ?? "?"} lastClaim=${after?.lastClaim ?? "?"}）`,
        screenshot: shot,
      };
    } catch (err) {
      const shot = await ctx.saveScreenshot(page, "vyceai-error");
      return fail(`异常：${(err as Error).message}`, shot);
    } finally {
      await page.close();
    }
  },
};

/** 确保登录：先探 dashboard，未登录则打开登录页由 SPA 完成登录（含 PoW），轮询等待 */
async function ensureLogin(page: Page, ctx: AdapterContext): Promise<boolean> {
  await page.goto(BASE_URL, { waitUntil: "domcontentloaded" }).catch(() => {});
  await page.waitForTimeout(1500);
  if (await isLoggedIn(page)) {
    ctx.log("已是登录态（复用会话）");
    return true;
  }
  ctx.log("打开登录页，由页面完成 CSRF+PoW 登录…");
  await page.goto(`${BASE_URL}/login`, { waitUntil: "domcontentloaded" });
  await page.waitForTimeout(3000);
  await page.getByRole("textbox", { name: "you@example.com" }).fill(ctx.credentials.email).catch(() => {});
  await page.getByRole("textbox", { name: "••••••••" }).fill(ctx.credentials.password).catch(() => {});
  await page.getByRole("button", { name: "Login", exact: true }).click().catch(() => {});
  // PoW 求解 + 登录需要几秒，轮询等待最多 ~30s
  for (let i = 0; i < 12; i++) {
    await page.waitForTimeout(2500);
    if (await isLoggedIn(page)) {
      ctx.log("登录成功");
      return true;
    }
  }
  return false;
}

async function isLoggedIn(page: Page): Promise<boolean> {
  const d = await getJson(page, "/user/dashboard");
  return !!(d && d.user && d.user.id);
}

async function readBalance(page: Page): Promise<number | null> {
  const d = await getJson(page, "/user/dashboard");
  const b = d?.user?.totalBalance;
  return typeof b === "number" ? Number(b.toFixed(4)) : null;
}

/** 领取成功后 /user/dashboard 短时间内仍返回旧余额，轮询等待其更新（每 2s 一次，最多 ~10s） */
async function readBalanceAfterClaim(page: Page, before: number | null): Promise<number | null> {
  let last = await readBalance(page);
  for (let i = 0; i < 5; i++) {
    if (typeof last !== "number") break;
    if (before === null || last > before) return last;
    await page.waitForTimeout(2000);
    last = await readBalance(page);
  }
  return last;
}

async function getJson(page: Page, path: string): Promise<any> {
  return page.evaluate(async (p) => {
    try {
      const r = await fetch(p, { credentials: "include" });
      const t = await r.text();
      return t.trim().startsWith("{") ? JSON.parse(t) : null;
    } catch {
      return null;
    }
  }, path);
}

async function postJson(page: Page, path: string): Promise<any> {
  return page.evaluate(async (p) => {
    try {
      const r = await fetch(p, { method: "POST", credentials: "include", headers: { "Content-Type": "application/json" } });
      const t = await r.text();
      // 非 JSON 响应体也保留前 120 字符，便于排查"返回 200 但未落账"类问题
      return t.trim().startsWith("{") ? JSON.parse(t) : { status: r.status, body: t.slice(0, 120) };
    } catch (e) {
      return { error: String(e) };
    }
  }, path);
}

function fail(message: string, screenshot: string | null): AdapterOutcome {
  return { status: "failed", reward: null, balance: null, currency: null, message, screenshot };
}
