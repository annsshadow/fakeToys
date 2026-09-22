// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync, writeFileSync, unlinkSync, existsSync, mkdirSync } from "node:fs";
import { join } from "node:path";
import { ROOT_DIR } from "./config.js";

/**
 * 单实例锁：常驻模式下防止同时跑多个进程。
 *
 * 背景：定时签到用的是进程内 node-cron，runAll 的互斥锁也只在进程内生效。
 * 若手动起了多个 `npm start`，每个进程各有一份 cron 与锁，会在同一时刻
 * 重复触发（实测出现过一次正常、一次因环境不同全跳过的双触发）。
 * 锁文件记录存活进程的 PID，第二个进程启动即拒绝。
 */
const LOCK_FILE = join(ROOT_DIR, "sessions", ".daemon.lock");

/** 进程是否存活（signal 0 只探测存在性，不真正发信号） */
function isAlive(pid: number): boolean {
  try {
    process.kill(pid, 0);
    return true;
  } catch (err) {
    // ESRCH=进程不存在；EPERM=存在但无权限（仍算存活）
    return (err as NodeJS.ErrnoException).code === "EPERM";
  }
}

/**
 * 尝试获取单实例锁。成功返回 true 并写入自身 PID；
 * 若已有存活进程持锁，返回持锁 PID（调用方据此拒绝启动）。
 */
export function acquireLock(): { ok: true } | { ok: false; holderPid: number } {
  mkdirSync(join(ROOT_DIR, "sessions"), { recursive: true });

  if (existsSync(LOCK_FILE)) {
    const raw = readFileSync(LOCK_FILE, "utf-8").trim();
    const pid = Number(raw);
    if (Number.isInteger(pid) && pid > 0 && pid !== process.pid && isAlive(pid)) {
      return { ok: false, holderPid: pid };
    }
    // 锁文件残留（持锁进程已退出）→ 覆盖
  }

  writeFileSync(LOCK_FILE, String(process.pid), "utf-8");
  return { ok: true };
}

/** 释放锁（仅当锁文件仍属于本进程时删除，避免误删接管者的锁） */
export function releaseLock(): void {
  try {
    if (!existsSync(LOCK_FILE)) return;
    const pid = Number(readFileSync(LOCK_FILE, "utf-8").trim());
    if (pid === process.pid) unlinkSync(LOCK_FILE);
  } catch {
    // ignore
  }
}
