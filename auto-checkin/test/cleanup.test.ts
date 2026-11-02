// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import assert from "node:assert/strict";
import { test } from "node:test";
import { mkdirSync, writeFileSync, readdirSync, utimesSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { cleanupOldFiles, LOG_RETENTION_DAYS, SHOT_KEEP } from "../src/cleanup.js";

// 第 59 轮：日志/截图保留窗口清理。

function makeDirs(): { logDir: string; shotDir: string; base: string } {
  const base = join(tmpdir(), `cleanup-${process.pid}-${Math.random().toString(36).slice(4)}`);
  const logDir = join(base, "logs");
  const shotDir = join(base, "screenshots");
  mkdirSync(logDir, { recursive: true });
  mkdirSync(shotDir, { recursive: true });
  return { logDir, shotDir, base };
}

test("旧日志（超保留窗口）被删，新日志保留", () => {
  const { logDir, shotDir } = makeDirs();
  const oldLog = join(logDir, "old.log");
  const newLog = join(logDir, "new.log");
  writeFileSync(oldLog, "old");
  writeFileSync(newLog, "new");
  // 把 old.log 的 mtime 拨到保留窗口之外
  const oldTime = new Date(Date.now() - (LOG_RETENTION_DAYS + 1) * 86_400_000);
  utimesSync(oldLog, oldTime, oldTime);

  cleanupOldFiles(logDir, shotDir);

  const remaining = readdirSync(logDir);
  assert.ok(remaining.includes("new.log"), "新日志必须保留");
  assert.ok(!remaining.includes("old.log"), "过期日志应被删除");
});

test("截图超保留上限时删最旧、保留最新", () => {
  const { logDir, shotDir } = makeDirs();
  const total = SHOT_KEEP + 10; // 210 张，应删最旧 10 张
  const baseNow = Date.now();
  for (let i = 0; i < total; i++) {
    const p = join(shotDir, `shot-${String(i).padStart(3, "0")}.png`);
    writeFileSync(p, `s${i}`);
    // 越大的 i 越新（10ms 递增 mtime）
    const t = new Date(baseNow - 1_000_000 + i * 10);
    utimesSync(p, t, t);
  }

  cleanupOldFiles(logDir, shotDir);

  const remaining = readdirSync(shotDir).filter((f) => f.endsWith(".png"));
  assert.equal(remaining.length, SHOT_KEEP, "截图数应收敛到保留上限");
  // 最旧 10 张（shot-000..009）被删，最新 200 张保留
  assert.ok(!remaining.includes("shot-000.png"), "最旧截图应被删");
  assert.ok(remaining.includes("shot-199.png"), "较新截图应保留");
});

test("目录不存在时不抛异常（fail-safe）", () => {
  // 传不存在的目录：清理应静默通过，不崩。
  assert.doesNotThrow(() =>
    cleanupOldFiles(join(tmpdir(), "no-such-log-dir"), join(tmpdir(), "no-such-shot-dir")),
  );
});