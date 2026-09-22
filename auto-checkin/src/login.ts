// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { loadConfig } from "./config.js";
import { openSiteContext, closeContext } from "./browser.js";
import { fetchSelf } from "./sites/newapi-client.js";
import { getAdapter } from "./sites/index.js";
import { createLogger } from "./logger.js";

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
  await new Promise((r) => setTimeout(r, 1500));
  await closeContext(context);
  process.exit(ok ? 0 : 1);
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
