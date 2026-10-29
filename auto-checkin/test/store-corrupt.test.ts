// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

// 记录文件损坏时的行为：**隔离**而不是静默当空账。
//
// 原实现 `readFromDisk` 的 catch 直接 `return { records: [] }`，而下一次
// `addRecord` 会以「空账 + 1 条」整份重写 `records.json` —— 一次坏 JSON
// 等于全部签到历史被永久覆盖，且没有任何信号。
//
// 单独成文件：`node --test` 每个测试文件一个进程，`store.ts` 的内存缓存与
// 模块级 DATA_DIR 不会污染 store.test.ts 里那些依赖记录条数的用例。
import assert from "node:assert/strict";
import { test } from "node:test";
import { mkdtempSync, readFileSync, writeFileSync, existsSync, readdirSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

const DATA_DIR = mkdtempSync(join(tmpdir(), "auto-checkin-store-corrupt-"));
process.env.AUTO_CHECKIN_DATA_DIR = DATA_DIR;

const { addRecord, getAllRecords } = await import("../src/store.js");

const dbFile = join(DATA_DIR, "records.json");

test("损坏的记录文件被隔离为 records.corrupt-*.json，而不是被下一次写入覆盖", () => {
  const broken = '{"records": [{"siteId": "a", "status": "suc';
  writeFileSync(dbFile, broken, "utf-8");

  // 首次读取触发解析失败 → 隔离 + 空账
  assert.deepEqual(getAllRecords(), [], "损坏文件应按空账读取，而不是抛穿调用方");

  const quarantined = readdirSync(DATA_DIR).filter((n) => n.startsWith("records.corrupt-"));
  assert.equal(quarantined.length, 1, `应恰好留下一个隔离文件，实得 ${JSON.stringify(quarantined)}`);
  assert.equal(
    readFileSync(join(DATA_DIR, quarantined[0]), "utf-8"),
    broken,
    "隔离文件必须逐字节保留原内容，否则「抢救」这条路就断了",
  );
  assert.equal(existsSync(dbFile), false, "原文件应已被改名移走");

  // 之后仍能正常写入：新账从 0 开始，不覆盖任何东西
  addRecord({
    siteId: "site-a",
    siteName: "Site A",
    timestamp: new Date().toISOString(),
    status: "success",
    reward: 1,
    balance: 1,
    currency: "$",
    message: "ok",
    screenshot: null,
    durationMs: 1,
  });
  assert.equal(getAllRecords().length, 1);
  assert.equal(readdirSync(DATA_DIR).filter((n) => n.startsWith("records.corrupt-")).length, 1,
    "正常写入不应再产生新的隔离文件");
});
