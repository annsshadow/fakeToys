// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import assert from "node:assert/strict";
import { test } from "node:test";
import { existsSync, mkdirSync, utimesSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { startCleanupLoop, LOG_RETENTION_DAYS } from "../src/cleanup.js";

// 第 89 轮：第 59 轮的清理只在启动时跑一次，常驻进程连续数周后目录又会长回来。
// startCleanupLoop 必须「启动即清 + 周期重复 + stop 后停」。

function makeDirs(): { logDir: string; shotDir: string } {
  const base = join(tmpdir(), `cleanup-loop-${process.pid}-${Math.random().toString(36).slice(4)}`);
  const logDir = join(base, "logs");
  const shotDir = join(base, "screenshots");
  mkdirSync(logDir, { recursive: true });
  mkdirSync(shotDir, { recursive: true });
  return { logDir, shotDir };
}

function writeOldLog(dir: string, name: string): string {
  const p = join(dir, name);
  writeFileSync(p, "old");
  const t = new Date(Date.now() - (LOG_RETENTION_DAYS + 1) * 86_400_000);
  utimesSync(p, t, t);
  return p;
}

async function waitGone(p: string, timeoutMs = 2000): Promise<void> {
  const deadline = Date.now() + timeoutMs;
  while (Date.now() < deadline) {
    if (!existsSync(p)) return;
    await new Promise((r) => setTimeout(r, 10));
  }
  throw new Error(`等待清理超时：${p}`);
}

test("周期清理：启动即清一次，之后按间隔重复，stop 后停止", async () => {
  const { logDir, shotDir } = makeDirs();
  const first = writeOldLog(logDir, "first.log");

  const stop = startCleanupLoop({ logDir, shotDir, intervalMs: 30 });
  try {
    // 启动时的那一次（保留第 59 轮行为）
    await waitGone(first);

    // 下一轮定时清理：运行中新增的过期日志也必须被清掉
    const second = writeOldLog(logDir, "second.log");
    await waitGone(second);
  } finally {
    stop();
  }

  // stop 之后不再清理：过期文件保持原样
  const third = writeOldLog(logDir, "third.log");
  await new Promise((r) => setTimeout(r, 150));
  assert.ok(existsSync(third), "stop() 之后不得再执行清理");
});

test("目录不存在时周期清理不抛异常（fail-safe 且循环存活）", async () => {
  const base = join(tmpdir(), `cleanup-loop-missing-${process.pid}-${Math.random().toString(36).slice(4)}`);
  const stop = startCleanupLoop({
    logDir: join(base, "no-logs"),
    shotDir: join(base, "no-shots"),
    intervalMs: 20,
  });
  await new Promise((r) => setTimeout(r, 80));
  stop();
});
