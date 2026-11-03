// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { existsSync, mkdirSync } from "node:fs";
import { join } from "node:path";
import { ROOT_DIR } from "./config.js";
import type { AppConfig } from "./types.js";
// 只做类型导入：Playwright 在模块加载时就会缓存 PLAYWRIGHT_BROWSERS_PATH，
// 必须先在下方清理无效路径，再通过动态 import 触发它真正加载。
import type { BrowserContext, Page } from "playwright";

const SESSION_DIR = join(ROOT_DIR, "sessions");
const SHOT_DIR = join(ROOT_DIR, "screenshots");
mkdirSync(SESSION_DIR, { recursive: true });
mkdirSync(SHOT_DIR, { recursive: true });

// 某些启动环境（IDE/终端/外部工具注入）可能带一个指向不存在目录的值
// （如 Z:\nonexistent\browsers），会让 Playwright 永远找不到浏览器可执行文件，
// 表现为"每天签到全部失败但浏览器其实已装好"。这里在 Playwright 加载前清掉它。
const invalidBrowserPath = process.env.PLAYWRIGHT_BROWSERS_PATH && !existsSync(process.env.PLAYWRIGHT_BROWSERS_PATH);
if (invalidBrowserPath) {
  console.warn(
    `[browser] 检测到无效的 PLAYWRIGHT_BROWSERS_PATH（${process.env.PLAYWRIGHT_BROWSERS_PATH}），已忽略并回退默认浏览器目录`,
  );
  delete process.env.PLAYWRIGHT_BROWSERS_PATH;
}

const { chromium } = await import("playwright");

/**
 * 清理无效的 PLAYWRIGHT_BROWSERS_PATH。
 *
 * 某些启动环境（IDE/终端/外部工具注入）可能带一个指向不存在目录的值
 * （如 Z:\nonexistent\browsers），会让 launchPersistentContext 永远找不到
 * 浏览器可执行文件，表现为"每天签到全部失败但浏览器其实已装好"。
 *
 * 若该环境变量存在且目录不存在，删除它，让 Playwright 回退到默认安装路径。
 * 返回是否发生了回退（供日志/测试确认）。
 */
export function sanitizeBrowserEnv(): boolean {
  const p = process.env.PLAYWRIGHT_BROWSERS_PATH;
  if (!p) return false;
  if (existsSync(p)) return false;
  delete process.env.PLAYWRIGHT_BROWSERS_PATH;
  return true;
}

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
  // 继承到无效浏览器路径时回退默认安装路径，避免"浏览器明明装好却天天失败"。
  if (sanitizeBrowserEnv()) {
    console.warn(
      `[browser] 检测到无效的 PLAYWRIGHT_BROWSERS_PATH，已忽略并回退默认浏览器目录`,
    );
  }
  const context = await chromium.launchPersistentContext(userDataDir, {
    headless: config.browser.headless,
    timeout: config.browser.timeoutMs,
    args: ["--no-sandbox", "--disable-blink-features=AutomationControlled"],
    viewport: { width: 1280, height: 900 },
  });
  try {
    context.setDefaultTimeout(config.browser.timeoutMs);
    context.setDefaultNavigationTimeout(config.browser.timeoutMs);
    // tsx/esbuild 会给注入页面的 evaluate 代码加 __name 辅助函数（浏览器端不存在）。
    // 预注入一个恒等实现，避免 "ReferenceError: __name is not defined"。
    await context.addInitScript(() => {
      // @ts-expect-error 浏览器端全局兜底
      globalThis.__name = globalThis.__name || ((fn: unknown) => fn);
    });
    return context;
  } catch (err) {
    // 浏览器已启动但初始化失败时，不能把 context 丢失给 runner。
    await closeContext(context);
    throw err;
  }
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
