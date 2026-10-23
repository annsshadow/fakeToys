// Copyright (C) 2026 annssshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import assert from "node:assert/strict";
import { test } from "node:test";
import { mkdtempSync, readFileSync, writeFileSync, existsSync, readdirSync, mkdirSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

// store.ts 在 import 时根据 AUTO_CHECKIN_DATA_DIR 确定数据文件位置，
// 必须先设置环境变量再动态导入，避免测试碰到真实 data/records.json。
const DATA_DIR = mkdtempSync(join(tmpdir(), "auto-checkin-store-"));
process.env.AUTO_CHECKIN_DATA_DIR = DATA_DIR;

const { addRecord, getAllRecords, computeSiteStats, getStoreVersion } = await import("../src/store.js");
import type { CheckinResult } from "../src/types.js";

function result(overrides: Partial<CheckinResult> = {}): CheckinResult {
  return {
    siteId: "site-a",
    siteName: "Site A",
    timestamp: new Date().toISOString(),
    status: "success",
    reward: 1,
    balance: 10,
    currency: "$",
    message: "ok",
    screenshot: null,
    durationMs: 100,
    ...overrides,
  };
}

const dbFile = join(DATA_DIR, "records.json");

test("写入后立即可以从缓存读到，无需再读盘", () => {
  addRecord(result({ message: "first" }));
  addRecord(result({ message: "second" }));
  const records = getAllRecords();
  assert.equal(records.length, 2);
  assert.equal(records[1].message, "second");
});

test("缓存命中时不会重新读盘：外部改动在下次写入前不可见", () => {
  // 直接改文件内容（模拟进程外改动）；若实现没缓存，这里会读到 99 条
  const db = JSON.parse(readFileSync(dbFile, "utf-8")) as { records: CheckinResult[] };
  for (let i = 0; i < 99; i++) db.records.push(result({ siteId: "site-b" }));
  writeFileSync(dbFile, JSON.stringify(db, null, 2));

  // 缓存仍然有效：看不到外部改动
  assert.equal(getAllRecords().length, 2);

  // 发生一次写入后缓存失效，重新读盘，外部改动 + 新记录都可见
  addRecord(result({ message: "third" }));
  const records = getAllRecords();
  assert.equal(records.length, 102);
  assert.equal(records[records.length - 1].message, "third");
  assert.equal(getStoreVersion() >= 3, true);
});

test("原子写入：落盘后不存在临时文件，JSON 完整可解析", () => {
  const files = readdirSync(DATA_DIR);
  assert.equal(files.includes("records.json.tmp"), false, `残留临时文件: ${files.join(", ")}`);
  const parsed = JSON.parse(readFileSync(dbFile, "utf-8")) as { records: CheckinResult[] };
  assert.ok(Array.isArray(parsed.records));
  assert.ok(parsed.records.length > 0);
});

test("并发连续写入不会丢记录（每次写入都是完整快照的原子替换）", () => {
  const before = getAllRecords().length;
  const n = 50;
  for (let i = 0; i < n; i++) addRecord(result({ message: `burst-${i}` }));
  const records = getAllRecords();
  assert.equal(records.length, before + n);
  const onDisk = JSON.parse(readFileSync(dbFile, "utf-8")) as { records: CheckinResult[] };
  assert.equal(onDisk.records.length, before + n);
  const messages = new Set(onDisk.records.map((r) => r.message));
  for (let i = 0; i < n; i++) assert.equal(messages.has(`burst-${i}`), true, `丢失记录 burst-${i}`);
  assert.equal(existsSync(join(DATA_DIR, "records.json.tmp")), false);
});

test("统计基于同一份缓存数据计算，结果与记录一致", () => {
  const stats = computeSiteStats("site-a", "Site A");
  const total = getAllRecords().filter((r) => r.siteId === "site-a").length;
  assert.equal(stats.totalRuns, total);
});

test("写入确实经过临时文件再 rename（占用临时路径时写入必失败）", () => {
  const tmpPath = join(DATA_DIR, "records.json.tmp");
  mkdirSync(tmpPath); // 把临时文件路径占成一个目录
  try {
    // 直接 writeFileSync(DB_PATH) 的实现不会碰临时路径、会成功；
    // tmp+rename 的实现在这一步就会 EISDIR 失败。
    assert.throws(() => addRecord(result({ message: "atomic-proof" })));
  } finally {
    rmSync(tmpPath, { recursive: true, force: true });
  }
});
