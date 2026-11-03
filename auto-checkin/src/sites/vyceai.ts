// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import type { Page } from "playwright";
import type { SiteAdapter, AdapterContext, AdapterOutcome } from "../types.js";

const BASE_URL = "https://vyceai.com";

/** 领取端点（2026-09-23 由 /user/daily-reward 迁移至此；读取状态仍是 /user/daily-reward） */
const CLAIM_PATH = "/user/daily-reward/claim";
const STATUS_PATH = "/user/daily-reward";

/**
 * 站点 1: VyceAI —— 点击签到即可领取。
 *
 * ⚠️ VyceAI 不是 new-api，而是自定义 API，且登录有反爬（CSRF + 工作量证明 PoW）：
 *   - POST /user/login               登录（返回 user.totalBalance，单位美元）
 *   - GET  /user/dashboard           读取余额
 *   - GET  /user/daily-reward        领取状态 {canClaim, streak, lastClaim, rewardAmount}
 *   - POST /user/daily-reward/claim  领取每日奖励（canClaim=true 时）
 *
 * ⚠️ 2026-09-23：领取端点从 POST /user/daily-reward 迁移到 POST /user/daily-reward/claim。
 * 旧路径不会 404，而是被 SPA 路由兜底成 index.html 以 HTTP 200 返回，导致"看起来成功但没落账"。
 * 改路径前请先核对前端 bundle 里的字面量（GET 状态端点路径未变）。
 *
 * ⚠️ 2026-11-03：服务端开始对 /user/* 请求校验 Authorization: Bearer <vyce_session>。
 * 仅带 session cookie 的裸 fetch 会返回 success，但领取状态在数秒后回滚（实际未落账）。
 * 必须在请求头带上 localStorage.vyce_session 里的会话 token（前端请求即如此）。
 *
 * ⚠️ 2026-11-03：服务端领取后状态更新有 ~3-30s 延迟抖动（canClaim 先 true 后 false，
 * 期间余额先 +10 再回滚再落账）。复查不能只查一次，需轮询等待 canClaim 变 false 才算成功。
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
      if (!logged.ok) {
        const shot = await ctx.saveScreenshot(page, "vyceai-login-fail");
        return fail("登录失败（可能凭据错误或 PoW 未通过）", shot);
      }

      const reward = await getJson(page, STATUS_PATH);
      // 第 41 轮：登录态路径的 dashboard 已由 ensureLogin 拉过一次，
      // 余额直接复用（旧实现 isLoggedIn + readBalance 各拉一次，重复往返）。
      // 登录流程路径拿不到余额（返回 null），仍照旧再读一次。
      const balance = logged.balance ?? (await readBalance(page));
      if (!reward) {
        return { status: "failed", reward: null, balance, currency: "$", message: "无法读取每日奖励状态", screenshot: null };
      }
      ctx.log(`daily-reward: canClaim=${reward.canClaim} streak=${reward.streak} lastClaim=${reward.lastClaim}`);
      if (reward.canClaim === false) {
        return { status: "already", reward: 0, balance, currency: "$", message: `今日已领 · 连续${reward.streak ?? "?"}天`, screenshot: null };
      }

      const claim = await postJson(page, CLAIM_PATH);
      ctx.log(`领取返回：${JSON.stringify(claim).slice(0, 160)}`);

      // 端点路径变更时，SPA 兜底会把 index.html 以 HTTP 200 返回（2026-09-23 真实踩过）。
      // 显式识别这种响应，让"路径又变了"一眼可见，而不是伪装成普通的领取失败。
      if (claim && typeof claim.body === "string" && claim.body.trimStart().toLowerCase().startsWith("<!doctype")) {
        const shot = await ctx.saveScreenshot(page, "vyceai-claim-endpoint-moved");
        return {
          status: "failed",
          reward: null,
          balance,
          currency: "$",
          message: `领取端点返回 HTML（疑似路径变更，当前 ${CLAIM_PATH}）`,
          screenshot: shot,
        };
      }

      // HTTP 200 不代表落账：曾出现连续多天 200 但 canClaim/streak/lastClaim 纹丝不动（实际未领取，还导致断签）。
      // 2026-11-03 起服务端落账有 ~3-30s 延迟抖动：领取后 canClaim 可能先保持 true，
      // 余额先 +10 再回滚，最终才稳定为 canClaim=false。必须轮询等待 canClaim 变 false 才算真成功。
      // 轮询上限 30s（每 2s 一次），超过仍未落账则按失败处理并留痕。
      let after = null;
      let ineffectiveAt: string | null = null;
      for (let i = 0; i < 15; i++) {
        await page.waitForTimeout(2000);
        after = await getJson(page, STATUS_PATH);
        if (after?.canClaim === false) break;
        ineffectiveAt = after ? `${after.streak ?? "?"}天/${after.lastClaim ?? "?"}` : "无响应";
        ctx.log(`领取后第 ${i + 1} 次复查：canClaim 仍为 true（streak=${after?.streak} lastClaim=${after?.lastClaim}）`);
      }
      if (after && after.canClaim === false) {
        const got =
          typeof claim?.amount === "number"
            ? claim.amount
            : typeof reward.rewardAmount === "number"
              ? reward.rewardAmount
              : null;
        const newBalance = await readBalanceAfterClaim(page, balance);
        return {
          status: "success",
          reward: got,
          balance: newBalance ?? (typeof claim?.newBalance === "number" ? claim.newBalance : balance),
          currency: "$",
          message: `领取成功 +${got ?? "?"} · 连续${after.streak ?? "?"}天`,
          screenshot: null,
        };
      }
      const shot = await ctx.saveScreenshot(page, "vyceai-claim-ineffective");
      // 接口若返回了业务错误（如 {"error":{"message":...}}），一并带出，便于直接定位。
      // postJson 的网络异常是字符串形式（{ error: "TypeError: ..." }），两种都要透传。
      const rawErr = claim?.error;
      const apiErr = typeof rawErr === "string" ? rawErr : rawErr?.message;
      return {
        status: "failed",
        reward: null,
        balance,
        currency: "$",
        message:
          `领取未生效（返回 200 但 canClaim 在 30s 内未变 false，最后状态 ${ineffectiveAt ?? "未知"}` +
          `${apiErr ? `；接口报错：${apiErr}` : ""}）`,
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
async function ensureLogin(page: Page, ctx: AdapterContext): Promise<{ ok: boolean; balance: number | null }> {
  await page.goto(BASE_URL, { waitUntil: "domcontentloaded" }).catch(() => {});
  await page.waitForTimeout(1500);
  const initial = await loggedInState(page);
  if (initial.ok) {
    ctx.log("已是登录态（复用会话）");
    return initial;
  }
  ctx.log("打开登录页，由页面完成 CSRF+PoW 登录…");
  await page.goto(`${BASE_URL}/login`, { waitUntil: "domcontentloaded" });
  await page.waitForTimeout(3000);
  await page.getByRole("textbox", { name: "you@example.com" }).fill(ctx.credentials.email).catch(() => {});
  await page.getByRole("textbox", { name: "••••••••" }).fill(ctx.credentials.password).catch(() => {});
  // 站点按浏览器 locale/IP 输出中英文 UI：登录按钮可访问名为 "Login" 或 "登录"。
  // 选择器不命中时必须留痕，否则会被下面的 catch 吞掉、表现为"填了表但没提交"。
  const loginBtn = page.getByRole("button", { name: /^(Login|登录)$/ });
  if (await loginBtn.count()) {
    await loginBtn.click().catch(() => {});
  } else {
    ctx.log("未找到登录按钮（名称既非 Login 也非 登录），登录页结构可能已变更");
  }
  // PoW 求解 + 登录需要几秒，轮询等待最多 ~30s
  for (let i = 0; i < 12; i++) {
    await page.waitForTimeout(2500);
    const state = await loggedInState(page);
    if (state.ok) {
      ctx.log("登录成功");
      return state;
    }
  }
  return { ok: false, balance: null };
}

// 第 41 轮：登录态探测与余额读取合并成一次 dashboard 请求。
// 返回 { ok: 是否登录, balance: 已登录时的余额（未登录为 null）}。
async function loggedInState(page: Page): Promise<{ ok: boolean; balance: number | null }> {
  const d = await getJson(page, "/user/dashboard");
  if (!d || !d.user || !d.user.id) return { ok: false, balance: null };
  return { ok: true, balance: balanceOf(d) };
}

function balanceOf(d: any): number | null {
  const b = d?.user?.totalBalance;
  return typeof b === "number" ? Number(b.toFixed(4)) : null;
}

async function readBalance(page: Page): Promise<number | null> {
  return balanceOf(await getJson(page, "/user/dashboard"));
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

/**
 * 在页面上下文中发起带会话 token 的请求。
 * 2026-11-03：服务端校验 Authorization: Bearer <vyce_session>，仅 cookie 会导致领取被回滚。
 * token 从 localStorage.vyce_session 读取，与前端请求一致。
 * nonJsonAsNull=true 时非 JSON 响应返回 null（读状态用，保持"读不到状态"可被上层识别）；
 * false 时返回 {status, body} 便于排查"返回 200 但未落账"。
 */
async function authedFetch(page: Page, path: string, init?: RequestInit, nonJsonAsNull = true): Promise<any> {
  return page.evaluate(
    async ({ p, options, asNull, timeoutMs }) => {
      const controller = new AbortController();
      const timer = setTimeout(() => controller.abort(), timeoutMs);
      try {
        const token = localStorage.getItem("vyce_session") || "";
        const headers: Record<string, string> = {
          ...((options?.headers as Record<string, string>) ?? {}),
        };
        if (token) headers["Authorization"] = `Bearer ${token}`;
        const r = await fetch(p, { ...options, headers, credentials: "include", signal: controller.signal });
        const t = await r.text();
        if (!t.trim().startsWith("{")) return asNull ? null : { status: r.status, body: t.slice(0, 120) };
        return JSON.parse(t);
      } catch (e) {
        return asNull ? null : { error: String(e) };
      } finally {
        clearTimeout(timer);
      }
    },
    { p: path, options: init ?? {}, asNull: nonJsonAsNull, timeoutMs: 15000 },
  );
}

async function getJson(page: Page, path: string): Promise<any> {
  return authedFetch(page, path);
}

async function postJson(page: Page, path: string): Promise<any> {
  return authedFetch(page, path, { method: "POST", headers: { "Content-Type": "application/json" } }, false);
}

function fail(message: string, screenshot: string | null): AdapterOutcome {
  return { status: "failed", reward: null, balance: null, currency: null, message, screenshot };
}
