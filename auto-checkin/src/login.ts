// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { loadConfig } from "./config.js";
import { openSiteContext, closeContext } from "./browser.js";
import { fetchSelf } from "./sites/newapi-client.js";
import { getAdapter } from "./sites/index.js";
import { runSiteInContext } from "./runner.js";
import { createLogger } from "./logger.js";
import { acquireRunLock, releaseRunLock, wasInterrupted } from "./lock.js";

/**
 * 一次性手动登录引导：为 GitHub/LinuxDO OAuth 类站点建立持久化会话。
 *
 * 用法：npm run login -- <siteId>
 * 打开有头浏览器，你在里面自己用第三方登录一次（含 2FA）；工具检测到登录成功后
 * 把会话保存到 sessions/<siteId>/，之后自动签到复用。
 * 你的第三方密码只在官方页面输入，工具不接触、不存储。
 */
async function main(): Promise<void> {
  const siteId = process.argv[2];
  const log = createLogger("login");
  if (!siteId) {
    log.error("请指定站点：npm run login -- <siteId>（vyceai/agentrouter/justwoker）");
    process.exit(1);
  }
  const adapter = getAdapter(siteId);
  if (!adapter) {
    log.error(`未知站点：${siteId}`);
    process.exit(1);
  }

  const lock = acquireRunLock();
  if (!lock.ok) {
    log.error(`已有其他签到/登录进程在运行（PID=${lock.holderPid}），本次登录退出`);
    process.exitCode = 1;
    return;
  }

  const config = loadConfig();
  config.browser.headless = false; // 手动登录必须有头

  log.info(`即将打开浏览器，请在其中登录 ${adapter.name}（${adapter.baseUrl}）`);
  const context = await openSiteContext(siteId, config);
  const page = await context.newPage();
  await page.goto(`${adapter.baseUrl}/login`, { waitUntil: "domcontentloaded" }).catch(() => {});

  log.info("👉 请在弹出的浏览器窗口里完成登录。检测到成功后会自动保存并退出。");
  log.info("（若已登录但迟迟未检测到，可在浏览器里手动打开控制台/个人页刷新一下）");

  const host = new URL(adapter.baseUrl).hostname;
  const deadline = Date.now() + 8 * 60 * 1000; // 8 分钟
  let ok = false;
  let tries = 0;

  while (Date.now() < deadline) {
    await new Promise((r) => setTimeout(r, 3000));
    tries++;
    try {
      // 仅当页面已回到站点自身域名时才判断（OAuth 期间在第三方域名不判断）
      const url = page.url() || "";
      const onSite = url.includes(host);
      if (onSite) {
        const notAuthPage = /\/login|\/sign-?in|\/register|\/oauth/i.test(url);

        // 信号1：接口（Cookie 会话类站点，如 VyceAI/AgentRouter）
        const self = await fetchSelf(page, adapter.baseUrl).catch(() => null);
        if (self) {
          ok = true;
          log.info(`✅ 登录成功${self.username ? `（${self.username}）` : ""}！会话已保存到 sessions/${siteId}/`);
          break;
        }

        // 信号2：localStorage 里有用户对象（token 类 new-api 分支，如 JustWoker）
        const hasLsUser = await page
          .evaluate(() => {
            try {
              for (const k of Object.keys(localStorage)) {
                const v = localStorage.getItem(k) || "";
                if (/"id"|"username"|"access_token"|"user"/i.test(v) && v.length > 20) return true;
              }
            } catch {}
            return false;
          })
          .catch(() => false);

        // 信号3：已停留在认证后的页面（dashboard/wallet/profile/console 等），且不在登录页
        const onAuthRoute = /\/(dashboard|wallet|profile|console|panel|token|topup|setting|overview)/i.test(url);

        if (!notAuthPage && (hasLsUser || onAuthRoute)) {
          ok = true;
          log.info(`✅ 检测到登录成功（页面 ${shortUrl(url)}）！会话已保存到 sessions/${siteId}/`);
          break;
        }
      }
      if (tries % 5 === 0) {
        log.info(`…仍在等待登录（已检测 ${tries} 次，当前页面 ${shortUrl(page.url())}）`);
      }
    } catch {
      // 页面导航中，忽略本次
    }
  }

  if (!ok) log.warn("超时未检测到登录。请重试，或确认是在本工具弹出的窗口里登录的。");
  if (ok) {
    const cleared = await clearWafGate(page, adapter.baseUrl, log);
    // WAF 放行窗口实测只有十几秒，拆成"先登录再单独跑签到"两步大概率已经过期，
    // 所以过完滑块立刻在同一个上下文里把签到做完。
    if (cleared && process.argv.includes("--checkin")) {
      log.info("人机验证已通过，立即执行签到（放行窗口很短，不另开浏览器）…");
      const result = await runSiteInContext(siteId, config, context);
      if (result) {
        log.info(`签到结果：${result.status} — ${result.message}`);
        if (result.status === "failed") ok = false;
      }
    }
  }
  await new Promise((r) => setTimeout(r, 1500));
  await closeContext(context);
  releaseRunLock();
  process.exit(ok && !wasInterrupted() ? 0 : 1);
}

/**
 * 打开余额接口地址，让人手动过掉 WAF 人机验证。
 *
 * 实测（2026-10，AgentRouter）：/api/user/self 被阿里云 WAF 单点保护，
 * 页内 fetch 会拿到 200 + 挑战页；直接访问该地址才会看到滑块。
 * 滑块属于人机验证，不做自动绕过——这里只是把地址打开，请人自己划一次。
 *
 * 判定必须用"接口是否还在返回挑战页"，不能用页面文案：
 * 早期版本匹配 innerText 里的"访问验证/滑块"，结果页面已是 JSON 时仍误判为未通过，
 * 白等 4 分钟还占着 run 锁。
 */
async function clearWafGate(
  page: import("playwright").Page,
  baseUrl: string,
  log: ReturnType<typeof createLogger>,
  timeoutMs = 4 * 60 * 1000,
): Promise<boolean> {
  const apiUrl = `${baseUrl}/api/user/self`;
  log.info(`正在打开余额接口 ${apiUrl}`);
  log.info("👉 若出现滑块验证，请手动拖到最右侧；工具会一直等到能读到余额为止。");
  await page.goto(apiUrl, { waitUntil: "domcontentloaded" }).catch(() => {});

  const deadline = Date.now() + timeoutMs;
  let tries = 0;
  while (Date.now() < deadline) {
    await new Promise((r) => setTimeout(r, 3000));
    tries++;
    // 必须真的读到余额才算放行。
    // fetchSelf 还有一层 localStorage 兜底：接口被 WAF 拦时它照样能返回用户名，
    // 只判断"非 null"会把被拦截误报成已放行，然后签到立刻又失败。
    const self = await fetchSelf(page, baseUrl).catch(() => null);
    if (self && typeof self.quota === "number") {
      log.info(`✅ 余额接口已放行（quota=${self.quota}），会话可用`);
      return true;
    }
    if (tries % 4 === 0) {
      log.info(
        `…余额接口仍被 WAF 拦截（已等 ${Math.round((tries * 3) / 60)} 分钟），请在浏览器窗口里拖动滑块`,
      );
    }
  }
  log.warn("等待超时，余额接口仍被 WAF 拦截；自动签到将无法确认余额。");
  return false;
}

function shortUrl(u: string): string {
  try {
    const x = new URL(u);
    return x.hostname + x.pathname.slice(0, 20);
  } catch {
    return u.slice(0, 30);
  }
}

main().catch((e) => {
  createLogger("login").error(String(e));
  process.exit(1);
});
