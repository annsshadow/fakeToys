// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/** 第 59 轮：日志与截图的保留窗口清理。

 * `logs/` 与 `screenshots/` 只写不清 —— 长期运行会无限增长。
 * 启动时清理一次：日志保留最近 30 天，截图保留最近 200 张（删最旧）。
 * 目录可注入（测试用临时目录）。
 */

import { readdirSync, statSync, unlinkSync } from "node:fs";
import { join } from "node:path";
import { ROOT_DIR } from "./config.js";

export const LOG_RETENTION_DAYS = 30;
export const SHOT_KEEP = 200;

export function cleanupOldFiles(
  logDir: string = join(ROOT_DIR, "logs"),
  shotDir: string = join(ROOT_DIR, "screenshots"),
): void {
  // 日志：删除修改时间早于保留窗口的 .log
  try {
    const cutoff = Date.now() - LOG_RETENTION_DAYS * 86_400_000;
    for (const f of readdirSync(logDir)) {
      if (!f.endsWith(".log")) continue;
      const p = join(logDir, f);
      try {
        if (statSync(p).mtimeMs < cutoff) unlinkSync(p);
      } catch {
        /* 文件可能已被并发删除，忽略 */
      }
    }
  } catch {
    /* logs 目录不存在，无需清理 */
  }

  // 截图：超过保留上限时删除最旧
  try {
    const shots = readdirSync(shotDir)
      .filter((f) => f.endsWith(".png"))
      .map((f) => ({ f, m: statSync(join(shotDir, f)).mtimeMs }))
      .sort((a, b) => a.m - b.m);
    for (let i = 0; i < shots.length - SHOT_KEEP; i++) {
      try {
        unlinkSync(join(shotDir, shots[i].f));
      } catch {
        /* 并发删除忽略 */
      }
    }
  } catch {
    /* screenshots 目录不存在，无需清理 */
  }
}