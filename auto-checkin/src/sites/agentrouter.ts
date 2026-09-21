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
      const beforeQuota = first.user.quota;
      ctx.log(`当前余额 quota=${beforeQuota}，准备退出重登`);

      // 2. 退出
      await logout(page, BASE_URL);
      await page.waitForTimeout(1500);
      ctx.log("已退出登录");

      // 3. GitHub 重新授权登录（触发发奖）——强制真实 OAuth 往返，绕开复用会话误判
      const second = await oauthLogin(page, BASE_URL, "github", ctx.log, "/login", true);
      if (!second.user) {
        // 退出后 GitHub 会话通常仍在，理论上能自动重登；失败则回退到已有登录态
        ctx.log("自动重登未确认，回退读取当前余额");
      } else {
        ctx.log("重新登录成功");
      }

      // 4. 算差值
      const fresh = await fetchSelf(page, BASE_URL);
      const afterQuota = fresh?.quota ?? second.user?.quota ?? beforeQuota;
      const disp = quotaToDisplay(afterQuota);
      if (beforeQuota !== null && afterQuota !== null) {
        const delta = afterQuota - beforeQuota;
        if (delta > 0) {
          const r = quotaToDisplay(delta);
          return { status: "success", reward: r.value, balance: disp.value, currency: "$", message: `重登领取 +${r.value}$`, screenshot: null };
        }
        return { status: "already", reward: 0, balance: disp.value, currency: "$", message: "重登完成，余额未增（今日可能已领）", screenshot: null };
      }
      return { status: "success", reward: null, balance: disp.value, currency: "$", message: "已重登", screenshot: null };
    } catch (err) {
      const shot = await ctx.saveScreenshot(page, "agentrouter-error");
      return fail(`异常：${(err as Error).message}`, shot);
    } finally {
      await page.close();
    }
  },
};

function fail(message: string, screenshot: string | null): AdapterOutcome {
  return { status: "failed", reward: null, balance: null, currency: null, message, screenshot };
}
