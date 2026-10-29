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

test("步进/列表表达式不能估算时给回退说明，而不是假装空闲", () => {
  // 曾经的实现对 `*/30` 返回 null，面板显示「空闲待命」——对已排定的
  // cron 是假话（2026-10-24 修：改为返回人话说明）。
  for (const expr of ["*/30 * * * *", "0 */2 * * *", "1,15 * * * *", "0 9-18 * * *"]) {
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
