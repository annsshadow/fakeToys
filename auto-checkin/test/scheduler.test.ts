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
