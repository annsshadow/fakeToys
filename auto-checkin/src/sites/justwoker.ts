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
      ctx.log("已访问 /profile");

      const api = await apiCheckIn(page);
      if (api) ctx.log(`签到接口返回：${api.message}`);

      const after = await fetchSelf(page, BASE_URL);
      const afterQuota = after?.quota ?? beforeQuota;
      const disp = quotaToDisplay(afterQuota);
      if (beforeQuota !== null && afterQuota !== null) {
        const delta = afterQuota - beforeQuota;
        if (delta > 0) {
          const r = quotaToDisplay(delta);
          return { status: "success", reward: r.value, balance: disp.value, currency: "$", message: `访问领取 +${r.value}$`, screenshot: null };
        }
        return { status: "already", reward: 0, balance: disp.value, currency: "$", message: "今日已领取或无新增", screenshot: null };
      }
      return { status: "success", reward: null, balance: disp.value, currency: "$", message: "已访问 profile", screenshot: null };
    } catch (err) {
      const shot = await ctx.saveScreenshot(page, "justwoker-error");
      return fail(`异常：${(err as Error).message}`, shot);
    } finally {
      await page.close();
    }
  },
};

function fail(message: string, screenshot: string | null): AdapterOutcome {
  return { status: "failed", reward: null, balance: null, currency: null, message, screenshot };
}
