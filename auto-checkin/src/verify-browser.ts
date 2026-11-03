// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { acquireRunLock, releaseRunLock } from "./lock.js";
// 不要静态 import playwright：Playwright 在模块加载时就缓存
// PLAYWRIGHT_BROWSERS_PATH，必须在加载前先清理无效路径（见 browser.ts 顶部说明）。
import { sanitizeBrowserEnv } from "./browser.js";

async function main(): Promise<void> {
  const lock = acquireRunLock();
  if (!lock.ok) {
    console.error(`[browser] 验证跳过：已有运行租约（PID=${lock.holderPid}）`);
    process.exit(75);
  }
  try {
    // 继承到无效浏览器路径时回退默认安装路径，避免误报"浏览器缺失"。
    if (sanitizeBrowserEnv()) {
      console.warn(`[browser] 检测到无效的 PLAYWRIGHT_BROWSERS_PATH，已忽略并回退默认浏览器目录`);
    }
    // 动态 import：此时无效路径已被清理，Playwright 会走默认浏览器目录。
    const { chromium } = await import("playwright");
    // launch 会走与 headless 任务相同的 browser channel，能发现残留安装标记但缺少 exe 的情况。
    const browser = await chromium.launch({ headless: true });
    await browser.close();
  } finally {
    releaseRunLock();
  }
}

main().catch((err) => {
  const message = (err as Error).message;
  const missingExecutable = /executable.*does not exist|executable doesn't exist/i.test(message);
  console.error(`[browser] 验证失败：${message}`);
  process.exit(missingExecutable ? 20 : 1);
});
