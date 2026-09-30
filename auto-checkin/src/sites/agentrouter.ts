// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import type { SiteAdapter, AdapterContext, AdapterOutcome } from "../types.js";
import { oauthLogin, fetchSelf, logout, quotaToDisplay } from "./newapi-client.js";

const BASE_URL = "https://agentrouter.org";

/**
 * 站点 2: AgentRouter —— 需退出重新登录才能领取（LinuxDO OAuth 登录）。
 *
 * 实测：new-api 面板，仅支持 GitHub/LinuxDO OAuth（邮箱密码被拒）。用户用 LinuxDO 授权。
 * 奖励在"登录"动作时发放。流程：读余额 → 退出 → LinuxDO 重新授权登录 → 再读余额。
 * 需先 `npm run login -- agentrouter` 建立 LinuxDO 会话。
 */
export const agentrouterAdapter: SiteAdapter = {
  id: "agentrouter",
  name: "AgentRouter",
  baseUrl: BASE_URL,
  authType: "oauth",

  async checkin(ctx: AdapterContext): Promise<AdapterOutcome> {
    const page = await ctx.browserContext.newPage();
    try {
      // 1. 确保登录并读当前余额（AgentRouter 用 GitHub 授权）
      const first = await oauthLogin(page, BASE_URL, "github", ctx.log);
      if (first.needManual) {
        return fail("GitHub 会话失效，请运行：npm run login -- agentrouter", null);
      }
      if (!first.user) {
        const shot = await ctx.saveScreenshot(page, "agentrouter-login-fail");
        return fail("登录失败", shot);
      }
      const beforeQuota = first.user.quota ?? await readKnownQuota(page);
      if (beforeQuota === null) {
        ctx.log("领取前余额暂不可读，仍尝试退出并重登以恢复会话");
      } else {
        ctx.log(`当前余额 quota=${beforeQuota}，准备退出重登`);
      }

      // 2. 退出
      await logout(page, BASE_URL);
      await page.waitForTimeout(1500);
      ctx.log("已退出登录");

      // 3. GitHub 重新授权登录（触发发奖）——强制真实 OAuth 往返，绕开复用会话误判
      const second = await oauthLogin(page, BASE_URL, "github", ctx.log, "/login", true);
      if (second.needManual) {
        const shot = await ctx.saveScreenshot(page, "agentrouter-oauth-expired");
        return fail("GitHub 会话失效，请运行：npm run login -- agentrouter", shot);
      }
      if (!second.user) {
        ctx.log("自动重登未确认，继续读取当前余额");
      } else {
        ctx.log("重新登录成功");
      }

      // 4. 算差值；无法确认前后余额时必须失败，不能把未知状态伪装成 already。
      const fresh = await fetchSelf(page, BASE_URL);
      const afterQuota = fresh?.quota ?? second.user?.quota ?? await readKnownQuota(page);
      if (beforeQuota === null || afterQuota === null) {
        const shot = await ctx.saveScreenshot(page, "agentrouter-balance-unknown");
        return fail("重登后无法确认领取前后余额，未判定签到结果", shot);
      }
      const disp = quotaToDisplay(afterQuota);
      const delta = afterQuota - beforeQuota;
      if (delta > 0) {
        const r = quotaToDisplay(delta);
        return { status: "success", reward: r.value, balance: disp.value, currency: "$", message: `重登领取 +${r.value}$`, screenshot: null };
      }
      if (delta < 0) {
        const shot = await ctx.saveScreenshot(page, "agentrouter-balance-decreased");
        const lost = quotaToDisplay(-delta).value;
        return fail(`重登后余额异常下降 ${lost}$，未判定签到结果`, shot);
      }
      return { status: "already", reward: 0, balance: disp.value, currency: "$", message: "重登完成，余额未增（今日可能已领）", screenshot: null };
    } catch (err) {
      const shot = await ctx.saveScreenshot(page, "agentrouter-error");
      return fail(`异常：${(err as Error).message}`, shot);
    } finally {
      await page.close();
    }
  },
};

async function readKnownQuota(page: import("playwright").Page): Promise<number | null> {
  for (let attempt = 0; attempt < 3; attempt++) {
    const user = await fetchSelf(page, BASE_URL);
    if (user?.quota !== null && user?.quota !== undefined) return user.quota;
    await page.waitForTimeout(1000);
  }
  return null;
}

function fail(message: string, screenshot: string | null): AdapterOutcome {
  return { status: "failed", reward: null, balance: null, currency: null, message, screenshot };
}
