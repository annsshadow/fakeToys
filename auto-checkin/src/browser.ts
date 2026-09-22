// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { chromium, type BrowserContext, type Page } from "playwright";
import { mkdirSync } from "node:fs";
import { join } from "node:path";
import { ROOT_DIR } from "./config.js";
import type { AppConfig } from "./types.js";

const SESSION_DIR = join(ROOT_DIR, "sessions");
const SHOT_DIR = join(ROOT_DIR, "screenshots");
mkdirSync(SESSION_DIR, { recursive: true });
mkdirSync(SHOT_DIR, { recursive: true });

/**
 * 为某站点打开一个持久化浏览器上下文。
 * 每站独立 userDataDir，Cookie/登录态会在多次运行间保留，
 * 这样"每天访问即领"的站点不必每次都重新登录。
 */
export async function openSiteContext(
  siteId: string,
  config: AppConfig,
): Promise<BrowserContext> {
  const userDataDir = join(SESSION_DIR, siteId);
  const context = await chromium.launchPersistentContext(userDataDir, {
    headless: config.browser.headless,
    timeout: config.browser.timeoutMs,
    args: ["--no-sandbox", "--disable-blink-features=AutomationControlled"],
    viewport: { width: 1280, height: 900 },
  });
  context.setDefaultTimeout(config.browser.timeoutMs);
  context.setDefaultNavigationTimeout(config.browser.timeoutMs);
  // tsx/esbuild 会给注入页面的 evaluate 代码加 __name 辅助函数（浏览器端不存在）。
  // 预注入一个恒等实现，避免 "ReferenceError: __name is not defined"。
  await context.addInitScript(() => {
    // @ts-expect-error 浏览器端全局兜底
    globalThis.__name = globalThis.__name || ((fn: unknown) => fn);
  });
  return context;
}

export async function closeContext(context: BrowserContext): Promise<void> {
  try {
    await context.close();
  } catch {
    // ignore
  }
}

/** 保存截图到 screenshots/，返回相对路径（供面板展示，不含隐私路径） */
export async function saveScreenshot(page: Page, tag: string): Promise<string | null> {
  try {
    const name = `${tag}-${Date.now()}.png`;
    await page.screenshot({ path: join(SHOT_DIR, name), fullPage: true });
    return `screenshots/${name}`;
  } catch {
    return null;
  }
}
