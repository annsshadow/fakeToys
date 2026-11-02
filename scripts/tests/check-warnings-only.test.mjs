// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/** 第 62 轮：check-warnings.mjs 的 `--only` 参数行为回归。
 *  验证的是**参数校验层**（不发任何构建命令）：未知项点名、缺参、
 *  合法值放行、`--list` 列出全部检查项。
 */

import { spawnSync } from "node:child_process";
import assert from "node:assert/strict";
import { test } from "node:test";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const ROOT = join(dirname(fileURLToPath(import.meta.url)), "..", "..");

function run(...args) {
  const r = spawnSync(process.execPath, [join(ROOT, "scripts", "check-warnings.mjs"), ...args], {
    cwd: ROOT,
    encoding: "utf8",
    timeout: 60_000,
  });
  if (r.error) throw r.error;
  return r;
}

test("未知 --only 项：exit 2 且点名未知值", () => {
  const r = run("--only", "notexist");
  assert.equal(r.status, 2);
  assert.match(r.stderr, /未知检查项：notexist/);
  assert.match(r.stderr, /可用：/);
});

test("混合 --only（合法+未知）：未知项被点名，不静默跳过", () => {
  const r = run("--only", "agetnor,rust");
  assert.equal(r.status, 2);
  assert.match(r.stderr, /未知检查项：agetnor/);
});

test("--only 缺检查项（末尾/空格）：人话报错而非崩", () => {
  const r = run("--only");
  assert.equal(r.status, 2);
  assert.match(r.stderr, /缺少检查项/);
});

test("--list 列出全部检查项（非空）", () => {
  const r = run("--list");
  assert.equal(r.status, 0);
  assert.match(r.stdout, /rust-lints/);
  assert.match(r.stdout, /augmentor/);
});
