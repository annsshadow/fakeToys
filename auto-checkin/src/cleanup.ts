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

/** 周期清理的默认间隔：6 小时（常驻进程里磁盘增长按天计，无需更频繁） */
export const CLEANUP_INTERVAL_MS = 6 * 60 * 60 * 1000;

export interface CleanupLoopOptions {
  logDir?: string;
  shotDir?: string;
  intervalMs?: number;
  onError?: (err: Error) => void;
}

/**
 * 第 89 轮：周期清理。第 59 轮只在**启动时**清一次——常驻进程连续运行数周后，
 * `logs/` 与 `screenshots/` 又会越过保留窗口（失败截图尤其），等于清理只对
 * 「当天重启过的部署」有效。
 *
 * 启动即跑一次（保留旧行为），之后按 `intervalMs` 重复；返回停止函数，便于测试
 * 与嵌入式场景显式收尾。定时器 `unref()`，不单独阻止进程退出。
 */
export function startCleanupLoop(opts: CleanupLoopOptions = {}): () => void {
  const { logDir, shotDir, intervalMs = CLEANUP_INTERVAL_MS, onError } = opts;
  let timer: ReturnType<typeof setInterval> | null = null;
  let stopped = false;

  const run = (): void => {
    if (stopped) return;
    try {
      cleanupOldFiles(logDir, shotDir);
    } catch (err) {
      // 清理是后台维护动作，失败只上报，绝不能让常驻进程崩掉。
      onError?.(err as Error);
    }
  };

  run();
  timer = setInterval(run, intervalMs);
  timer.unref?.();
  return () => {
    stopped = true;
    if (timer) clearInterval(timer);
    timer = null;
  };
}