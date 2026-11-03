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
      let beforeQuota = first.user.quota ?? await readKnownQuota(page, ctx.log);
      if (beforeQuota === null) {
        // 领取前余额被 WAF 拦：有头模式等用户拖完滑块再读，而不是直接跳过。
        const waited = await waitForQuotaAfterWaf(page, ctx);
        if (waited !== null) beforeQuota = waited;
      }
      if (beforeQuota === null) {
        // 领取前余额都读不到，后面算不出差值，注定无法判定结果。
        // 此时退出重登只会再打一轮 OAuth，对被风控的站点是雪上加霜，直接失败。
        const shot = await ctx.saveScreenshot(page, "agentrouter-balance-unknown");
        return fail(
          "AgentRouter 接口读不到余额（疑似阿里云 WAF 人机验证），已跳过退出重登，需人工过一次：npm run login -- agentrouter",
          shot,
        );
      }
      ctx.log(`当前余额 quota=${beforeQuota}，准备退出重登`);

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
      const fresh = await fetchSelf(page, BASE_URL, ctx.log);
      let afterQuota = fresh?.quota ?? second.user?.quota;
      if (afterQuota == null) {
        // fresh.quota 可能是 null（WAF 拦截但 localStorage 有用户对象），
        // second.user 可能是 undefined（重登未确认）——两种都要等 WAF 放行。
        // 用 == null 而非 === null，否则 undefined 漏判、滑块等待不触发。
        afterQuota = await waitForQuotaAfterWaf(page, ctx);
      }
      afterQuota ??= await readKnownQuota(page, ctx.log);
      if (beforeQuota === null || afterQuota === null) {
        const shot = await ctx.saveScreenshot(page, "agentrouter-balance-unknown");
        return fail(
          "AgentRouter 触发了阿里云 WAF 人机验证（滑块），接口读不到余额，需人工过一次：npm run login -- agentrouter",
          shot,
        );
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

async function readKnownQuota(
  page: import("playwright").Page,
  log: (m: string) => void,
): Promise<number | null> {
  for (let attempt = 0; attempt < 3; attempt++) {
    const user = await fetchSelf(page, BASE_URL, log);
    if (user?.quota !== null && user?.quota !== undefined) return user.quota;
    await page.waitForTimeout(1000);
  }
  return null;
}

/**
 * 重登后余额被阿里云 WAF 拦：有头模式下浏览器窗口还开着，等用户手动拖完滑块。
 * 无头模式（计划任务）没有窗口，等也白等——直接返回 null 让上层走失败分支。
 * 返回读到的 quota，超时返回 null。
 *
 * 关键：WAF 挑战页（滑动验证）必须作为**文档渲染**（page.goto）滑块才可见可拖；
 * fetch() 拿到的挑战页 HTML 永远不渲染，滑块不可见，用户无从拖动。
 */
async function waitForQuotaAfterWaf(
  page: import("playwright").Page,
  ctx: AdapterContext,
): Promise<number | null> {
  if (ctx.config.browser.headless) return null;
  // 先把挑战页渲染成文档（滑块才可见）
  ctx.log("检测到阿里云 WAF 人机验证（滑块），正在打开验证页…");
  await page.goto(`${BASE_URL}/api/user/self`, { waitUntil: "domcontentloaded", timeout: 30000 }).catch(() => {});
  await page.waitForTimeout(2000);
  ctx.log("👉 请在浏览器窗口里按住滑块，拖动到最右侧（放行后自动继续签到）");

  const deadline = Date.now() + 4 * 60 * 1000;
  let tries = 0;
  while (Date.now() < deadline) {
    await page.waitForTimeout(3000);
    tries++;
    const user = await fetchSelf(page, BASE_URL, ctx.log);
    if (user?.quota != null && user?.quota != undefined) {
      ctx.log(`✅ WAF 已放行（quota=${user.quota}），继续签到`);
      return user.quota;
    }
    if (tries % 8 === 0) {
      ctx.log(`…余额接口仍被 WAF 拦截（已等 ${Math.round((tries * 3) / 60)} 分钟），请继续拖动滑块`);
    }
  }
  return null;
}

function fail(message: string, screenshot: string | null): AdapterOutcome {
  return { status: "failed", reward: null, balance: null, currency: null, message, screenshot };
}
