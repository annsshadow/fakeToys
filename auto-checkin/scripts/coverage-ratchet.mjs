// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

// auto-checkin 覆盖率棘轮（2026-10-29 B7 收口轮落地）。
//
// 跑一遍 node:test 的 --experimental-test-coverage，把 5 个核心文件的
// 行覆盖与 coverage-baseline.json 里的地板对撞：
//   - 低于地板 → 退出 1（覆盖回退，或有人删了测试）；
//   - 高于地板 → 提示回来抬地板（修复没留守卫等于没修）；
//   - 目标文件从报表里消失 → 退出 1（文件被改名/删除时棘轮不许静默放行）。
//
// 用法：npm run test:ratchet

import { spawnSync } from "node:child_process";
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const HERE = dirname(fileURLToPath(import.meta.url));
const baselinePath = join(HERE, "..", "coverage-baseline.json");

const baseline = JSON.parse(readFileSync(baselinePath, "utf-8"));

const run = spawnSync("npm", ["run", "test:coverage"], {
  shell: true,
  encoding: "utf-8",
  stdio: ["ignore", "pipe", "pipe"],
});
const output = `${run.stdout ?? ""}${run.stderr ?? ""}`;

// 测试本身必须全绿：覆盖率高但用例红没有意义
const passMatch = output.match(/^. pass (\d+)$/m);
const failMatch = output.match(/^. fail (\d+)$/m);
if (!passMatch || !failMatch) {
  console.error("coverage-ratchet: 无法从输出解析测试结果（报表格式变了？）");
  console.error(output.slice(-2000));
  process.exit(1);
}
if (Number(failMatch[1]) > 0) {
  console.error(`coverage-ratchet: 有 ${failMatch[1]} 条测试失败，先修测试再谈覆盖率`);
  process.exit(1);
}

// 解析逐文件行覆盖：`ℹ  justwoker.ts   |  100.00 | ...`（node>=24）
// 或 `#  justwoker.ts   |  100.00 | ...`（node 22 的 spec/TAP 注释前缀）
const table = new Map();
for (const line of output.split(/\r?\n/)) {
  const m = line.match(/^\s*[ℹ#]\s+(\S+\.ts)\s*\|\s*([\d.]+)/);
  if (m) table.set(m[1], Number(m[2]));
}

let fail = 0;
for (const [file, floor] of Object.entries(baseline)) {
  if (typeof floor !== "number") continue; // comment 等元数据键
  const actual = table.get(file);
  if (actual === undefined) {
    console.error(`::error::coverage-ratchet: 目标文件 ${file} 不在覆盖率报表里（被改名/删除/排除？），棘轮拒绝放行`);
    fail = 1;
    continue;
  }
  if (actual < floor) {
    console.error(`::error::coverage-ratchet: ${file} 行覆盖 ${actual}% 低于地板 ${floor}%（覆盖回退或测试被删）`);
    fail = 1;
  } else if (actual > floor) {
    console.log(
      `coverage-ratchet: ${file} 行覆盖升到 ${actual}%（地板 ${floor}%）——请同步抬高 coverage-baseline.json，否则这次修复没留下守卫`,
    );
  } else {
    console.log(`coverage-ratchet: ${file} ${actual}%（持平地板 ${floor}%）`);
  }
}

process.exit(fail);
