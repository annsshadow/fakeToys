// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import type { SiteAdapter, AdapterContext, AdapterOutcome } from "../types.js";
import { oauthLogin, fetchSelf, apiCheckIn, quotaToDisplay } from "./newapi-client.js";

const BASE_URL = "https://api.justwoker.icu";

/**
 * 站点 3: JustWoker —— 每天访问即可领取（GitHub OAuth 登录）。
 *
 * 实测：new-api 面板，登录有 Cloudflare Turnstile；仅 GitHub OAuth（邮箱密码被拒）。
 * 领奖：登录后访问 /profile 触发。需先 `npm run login -- justwoker` 建立 GitHub 会话。
 */
export const justwokerAdapter: SiteAdapter = {
  id: "justwoker",
  name: "JustWoker",
  baseUrl: BASE_URL,
  authType: "oauth",

  async checkin(ctx: AdapterContext): Promise<AdapterOutcome> {
    const page = await ctx.browserContext.newPage();
    try {
      const login = await oauthLogin(page, BASE_URL, "github", ctx.log);
      if (login.needManual) {
        return fail("GitHub 会话失效，请运行：npm run login -- justwoker", null);
      }
      if (!login.user) {
        const shot = await ctx.saveScreenshot(page, "justwoker-login-fail");
        return fail("登录失败", shot);
      }
      const beforeQuota = login.user.quota;

      // 访问 profile 触发每日发奖
      await page.goto(`${BASE_URL}/profile`, { waitUntil: "domcontentloaded" });
      await page.waitForTimeout(3000);
      const profileConfirmed = new URL(page.url()).pathname.replace(/\/$/, "") === "/profile";
      ctx.log(`已访问 /profile（页面确认=${profileConfirmed}）`);

      const api = await apiCheckIn(page);
      if (api) ctx.log(`签到接口返回：${api.message}`);
      const apiAlready = api?.ok === false && /already|已.*(领取|签到)|今日.*(领取|签到)|重复/i.test(api.message);
      if (api && !api.ok && !apiAlready) {
        const shot = await ctx.saveScreenshot(page, "justwoker-api-failed");
        return fail(`签到接口失败：${api.message}`, shot);
      }

      const after = await fetchSelf(page, BASE_URL);
      const afterQuota = after?.quota ?? null;
      if (beforeQuota === null || afterQuota === null) {
        const unknownStatus = classifyUnavailableBalance(api?.ok === true, apiAlready, profileConfirmed);
        if (unknownStatus === "success") {
          return { status: "success", reward: null, balance: null, currency: "$", message: "签到接口已确认成功，但余额接口不可用", screenshot: null };
        }
        if (unknownStatus === "already") {
          return { status: "already", reward: 0, balance: null, currency: "$", message: "已确认 profile/已领取状态，余额接口不可用（今日无新增）", screenshot: null };
        }
        const shot = await ctx.saveScreenshot(page, "justwoker-state-unknown");
        return fail("会话或领取状态无法确认，请重新登录后重试", shot);
      }
      const disp = quotaToDisplay(afterQuota);
      const delta = afterQuota - beforeQuota;
      if (delta > 0) {
        const r = quotaToDisplay(delta);
        return { status: "success", reward: r.value, balance: disp.value, currency: "$", message: `访问领取 +${r.value}$`, screenshot: null };
      }
      if (delta < 0) {
        const shot = await ctx.saveScreenshot(page, "justwoker-balance-decreased");
        const lost = quotaToDisplay(-delta).value;
        return fail(`领取后余额异常下降 ${lost}$，未判定签到结果`, shot);
      }
      return { status: "already", reward: 0, balance: disp.value, currency: "$", message: "今日已领取或无新增", screenshot: null };
    } catch (err) {
      const shot = await ctx.saveScreenshot(page, "justwoker-error");
      return fail(`异常：${(err as Error).message}`, shot);
    } finally {
      await page.close();
    }
  },
};

export function classifyUnavailableBalance(
  apiOk: boolean,
  apiAlready: boolean,
  profileConfirmed: boolean,
): "success" | "already" | "failed" {
  if (apiOk) return "success";
  if (apiAlready || profileConfirmed) return "already";
  return "failed";
}

function fail(message: string, screenshot: string | null): AdapterOutcome {
  return { status: "failed", reward: null, balance: null, currency: null, message, screenshot };
}
