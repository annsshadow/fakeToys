// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import assert from "node:assert/strict";
import { test } from "node:test";

import { estimateNext } from "../src/scheduler.js";

// estimateNext 只被 startScheduler 间接调用（cron.schedule 需要完整 config），
// 这里直接测纯函数本身——它导出来就是为了这个。

test("字面量 cron 能估算出未来 24h 内的 ISO 时间", () => {
  const { at, note } = estimateNext("0 9 * * *");
  assert.ok(at, "每日 9:00 应能估算");
  assert.ok(!Number.isNaN(Date.parse(at!)), "at 应是合法 ISO 串");
  assert.equal(note, null);
  // 估算结果必须在未来
  assert.ok(Date.parse(at!) > Date.now() - 60_000, "估算的下次时间应在未来");
});

// 第 77 轮：步进/列表/区间不再回退说明，按 cron 语义算出具体时刻。
test("步进表达式（星斜杠 n）能估算出落在步长上的时刻", () => {
  for (const [expr, modulo] of [["*/30 * * * *", 30], ["0 */2 * * *", 2]] as const) {
    const { at, note } = estimateNext(expr);
    assert.ok(at, `${expr} 应能估算`);
    assert.equal(note, null);
    const d = new Date(at!);
    assert.ok(d.getTime() > Date.now() - 60_000, "估算应在未来");
    if (modulo === 30) assert.equal(d.getMinutes() % 30, 0);
    else assert.equal(d.getHours() % 2, 0);
  }
});

test("列表与区间表达式能估算出对应字段上的时刻", () => {
  const list = estimateNext("1,15 * * * *");
  assert.ok(list.at, "列表表达式应能估算");
  assert.equal(list.note, null);
  assert.ok([1, 15].includes(new Date(list.at!).getMinutes()));

  const range = estimateNext("0 9-18 * * *");
  assert.ok(range.at, "区间表达式应能估算");
  assert.equal(range.note, null);
  const d = new Date(range.at!);
  assert.ok(d.getHours() >= 9 && d.getHours() <= 18);
  assert.equal(d.getMinutes(), 0);
});

test("周字段 7 与 0 同义（都指周日）", () => {
  const { at, note } = estimateNext("0 9 * * 7");
  assert.ok(at);
  assert.equal(note, null);
  assert.equal(new Date(at!).getDay(), 0, "7 应映射到周日");
});

test("非法或越界字段给回退说明，而不是假装空闲", () => {
  for (const expr of ["*/0 * * * *", "60 * * * *", "0 9 * * 9"]) {
    const { at, note } = estimateNext(expr);
    assert.equal(at, null, `${expr} 不应估算出时间`);
    assert.ok(note && note.includes(expr), `${expr} 应带原始表达式回退说明，实际 ${note}`);
  }
});

test("字段不足的表达式两个通道都为 null（交给 cron.validate 报错）", () => {
  const { at, note } = estimateNext("0 9 * *");
  assert.equal(at, null);
  assert.equal(note, null);
});

// ─────────────────────────────────────────────────────────────────────────
// startScheduler 分支（2026-10-29 覆盖率轮）。import 顺序敏感：
//  AUTO_CHECKIN_DATA_DIR 先设（scheduler→runner→store import 链），
//  PLAYWRIGHT_BROWSERS_PATH 指向空目录——runOnStartup 会立即触发 runAll，
//  必须让浏览器启动确定性失败，测试绝不碰真实站点。
// ─────────────────────────────────────────────────────────────────────────
import { mkdtempSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

process.env.AUTO_CHECKIN_DATA_DIR = mkdtempSync(join(tmpdir(), "auto-checkin-sched-"));
process.env.AUTO_CHECKIN_LOCK_DIR = mkdtempSync(join(tmpdir(), "auto-checkin-sched-lock-"));
process.env.PLAYWRIGHT_BROWSERS_PATH = "Z:" + join("nonexistent", "browsers");

const cron = (await import("node-cron")).default;
const { startScheduler } = await import("../src/scheduler.js");
import type { AppConfig } from "../src/types.js";

function config(schedule: Partial<AppConfig["schedule"]>): AppConfig {
  return {
    schedule: { cron: "0 9 * * *", timezone: "Asia/Shanghai", runOnStartup: false, ...schedule },
    browser: { headless: true, timeoutMs: 1000 },
    sites: { vyceai: { enabled: false }, agentrouter: { enabled: false }, justwoker: { enabled: false } },
    panel: { host: "127.0.0.1", port: 0, token: "" },
  };
}

test("startScheduler：非法 cron 显式拒绝且不排任何任务", () => {
  startScheduler(config({ cron: "not a cron" }));
  assert.equal(cron.getTasks().size, 0, "非法表达式不得留下任何任务");
});

test("startScheduler：合法表达式注册任务", () => {
  startScheduler(config({ cron: "0 9 * * *" }));
  assert.ok(cron.getTasks().size >= 1, "应注册定时任务");
});

test("startScheduler：runOnStartup 立即触发一轮（浏览器不可用 → runner 收敛不崩）", async () => {
  startScheduler(config({ cron: "0 9 * * *", runOnStartup: true }));
  // 启动即运行是 fire-and-forget：给一点时间让 runner 的失败收敛走完
  await new Promise((r) => setTimeout(r, 200));
});

test("cleanup：停掉本文件注册的全部定时任务，不让 node-cron 定时器挂住测试进程", () => {
  for (const t of cron.getTasks().values()) t.stop();
  cron.getTasks().clear();
  assert.equal(cron.getTasks().size, 0);
});

// 第 58 轮：完整 5 字段匹配。
test("每月 1 号 9 点：估算到**下月** 1 号而不是明天（旧实现只匹配分/时）", () => {
  const { at, note } = estimateNext("0 9 1 * *");
  assert.ok(at, "月任务应能估算出具体时刻");
  assert.equal(note, null);
  const d = new Date(at!);
  // 必须落在某月 1 号 09:00，且晚于今天（未来）。
  assert.equal(d.getDate(), 1);
  assert.equal(d.getHours(), 9);
  assert.equal(d.getMinutes(), 0);
  assert.ok(d.getTime() > Date.now() - 60_000, "估算应在未来");
  // 不可能是「明天」：下月 1 号距离今天至少 1 天以上（除非今天恰是月末）
  const days = (d.getTime() - Date.now()) / 86400000;
  assert.ok(days >= 1, "每月任务不应被估成明天（除非今天就是月末）");
});

test("每周一 9 点：估算到下一个周一 09:00", () => {
  const { at, note } = estimateNext("0 9 * * 1");
  assert.ok(at, "周任务应能估算");
  assert.equal(note, null);
  const d = new Date(at!);
  assert.equal(d.getDay(), 1, "应落在周一");
  assert.equal(d.getHours(), 9);
  assert.ok(d.getTime() > Date.now() - 60_000);
});

test("cron 的日/周 OR 语义：日字段非 * 而周为 * 时按日匹配", () => {
  // 今天是几号不确定，但 `0 9 15 * *` 的日=15 应命中 15 号（可能在下月）。
  const { at, note } = estimateNext("0 9 15 * *");
  assert.ok(at, "15 号任务应能估算");
  assert.equal(note, null);
  const d = new Date(at!);
  assert.equal(d.getDate(), 15);
  assert.equal(d.getHours(), 9);
});
