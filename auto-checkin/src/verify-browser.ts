// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { chromium } from "playwright";
import { acquireRunLock, releaseRunLock } from "./lock.js";

async function main(): Promise<void> {
  const lock = acquireRunLock();
  if (!lock.ok) {
    console.error(`[browser] 验证跳过：已有运行租约（PID=${lock.holderPid}）`);
    process.exit(75);
  }
  try {
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
