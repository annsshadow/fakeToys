// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import assert from "node:assert/strict";
import { test } from "node:test";
import { rmSync, writeFileSync, mkdirSync, existsSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

// lock.ts 的锁文件在 <repo>/sessions/（真实目录，gitignored）。测试必须自清理，
// 避免给并行的 server/runner 用例留陈旧锁（已知 flake 来源）。
const REPO = join(dirname(fileURLToPath(import.meta.url)), "..");
const SESSIONS = join(REPO, "sessions");
const RUN_LOCK = join(SESSIONS, ".run.lock");
const DAEMON_LOCK = join(SESSIONS, ".daemon.lock");

const { acquireRunLock, acquireDaemonLock, releaseRunLock, releaseAllLocks, wasInterrupted } =
  await import("../src/lock.js");
const proper = (await import("proper-lockfile")).default;

function cleanupLockFiles(): void {
  for (const f of [RUN_LOCK, DAEMON_LOCK]) {
    rmSync(f, { recursive: true, force: true });
    rmSync(f + ".owner", { force: true });
  }
}

test("同进程重复获取运行锁：可重入返回 ok（heldLocks 命中臂）", () => {
  cleanupLockFiles();
  const first = acquireRunLock();
  assert.equal(first.ok, true);
  const second = acquireRunLock();
  assert.equal(second.ok, true, "同进程重入必须直接 ok");
  releaseRunLock();
  releaseRunLock();
  cleanupLockFiles();
});

test("外部锁占用 → ELOCKED 降级为 ok:false 且读出持锁 pid（owner 文件合法时）", () => {
  cleanupLockFiles();
  // 模拟另一进程持锁：直接用 proper-lockfile 占住锁文件并写合法 owner
  mkdirSync(SESSIONS, { recursive: true });
  const release = proper.lockSync(RUN_LOCK, { lockfilePath: RUN_LOCK, realpath: false });
  writeFileSync(RUN_LOCK + ".owner", JSON.stringify({ pid: 12345, token: "t" }), "utf-8");

  const result = acquireRunLock();
  assert.equal(result.ok, false, "外部持锁时必须拿不到");
  assert.equal(!result.ok && result.holderPid, 12345, "应从 owner 文件读出持锁 pid");
  release();
  cleanupLockFiles();
});

test("外部锁的 owner 文件损坏 → holderPid 回退 0（readOwner catch 臂）", () => {
  cleanupLockFiles();
  const release = proper.lockSync(RUN_LOCK, { lockfilePath: RUN_LOCK, realpath: false });
  writeFileSync(RUN_LOCK + ".owner", "not-json{{{", "utf-8");

  const result = acquireRunLock();
  assert.equal(result.ok, false);
  assert.equal(!result.ok && result.holderPid, 0, "owner 损坏时 pid 回退 0");
  release();
  cleanupLockFiles();
});

test("releaseAllLocks：同时释放 run 与 daemon 两把锁", () => {
  cleanupLockFiles();
  assert.equal(acquireRunLock().ok, true);
  assert.equal(acquireDaemonLock().ok, true);
  releaseAllLocks();
  assert.equal(existsSync(RUN_LOCK), false, "run 锁必须已释放");
  assert.equal(existsSync(DAEMON_LOCK), false, "daemon 锁必须已释放");
  // 释放后可重新获取（证明真的释放了，而不是留着占位）
  assert.equal(acquireRunLock().ok, true);
  cleanupLockFiles();
});

test("wasInterrupted 基线：未收到终止信号时为 false", () => {
  assert.equal(wasInterrupted(), false);
});
