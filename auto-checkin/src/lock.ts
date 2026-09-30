// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import lockfile from "proper-lockfile";
import { mkdirSync, readFileSync, unlinkSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { ROOT_DIR } from "./config.js";

/**
 * 两类锁解决两个不同问题：
 * - daemon 锁：只允许一个常驻面板；
 * - run 租约：只允许一个签到/单站/登录操作访问浏览器 profile。
 *
 * daemon 可以长期运行，但在真正执行签到时会与计划任务/手动登录互斥，
 * 不会因为常驻进程存在而阻止空闲状态下的补跑和 OAuth 恢复。
 */
const LOCK_FILES = {
  daemon: join(ROOT_DIR, "sessions", ".daemon.lock"),
  run: join(ROOT_DIR, "sessions", ".run.lock"),
} as const;
const STALE_LOCK_MS = 5 * 60_000;
const UPDATE_LOCK_MS = 10_000;
const SHUTDOWN_GRACE_MS = 30_000;

type LockName = keyof typeof LOCK_FILES;
type LockResult = { ok: true } | { ok: false; holderPid: number };
type HeldLock = { release: () => void; ownerPath: string };

const heldLocks = new Map<LockName, HeldLock>();
let handlersInstalled = false;
let shutdownTimer: NodeJS.Timeout | undefined;
let interrupted = false;

function ownerPath(lockFile: string): string {
  return `${lockFile}.owner`;
}

function readOwner(lockFile: string): { pid: number; token: string } | null {
  try {
    const raw = JSON.parse(readFileSync(ownerPath(lockFile), "utf-8")) as {
      pid?: unknown;
      token?: unknown;
    };
    if (typeof raw.pid !== "number" || typeof raw.token !== "string") return null;
    return { pid: raw.pid, token: raw.token };
  } catch {
    return null;
  }
}

function acquireNamedLock(name: LockName): LockResult {
  const lockFile = LOCK_FILES[name];
  const ownerFile = ownerPath(lockFile);
  mkdirSync(join(ROOT_DIR, "sessions"), { recursive: true });

  if (heldLocks.has(name)) return { ok: true };

  try {
    // proper-lockfile 使用原子 mkdir、mtime 心跳和 stale 接管，避免 wx/rename 的竞态。
    const release = lockfile.lockSync(lockFile, {
      lockfilePath: lockFile,
      realpath: false,
      stale: STALE_LOCK_MS,
      update: UPDATE_LOCK_MS,
      onCompromised: (err) => {
        process.stderr.write(`[lock] ${name} 锁已失效：${(err as Error).message}\n`);
      },
    });
    const owner = { pid: process.pid, token: `${process.pid}-${Date.now()}` };
    try {
      writeFileSync(ownerFile, JSON.stringify(owner), "utf-8");
    } catch (err) {
      try {
        release();
      } catch {
        // 保留原始 owner 写入错误。
      }
      throw err;
    }
    heldLocks.set(name, { release, ownerPath: ownerFile });
    return { ok: true };
  } catch (err) {
    if ((err as NodeJS.ErrnoException).code !== "ELOCKED") throw err;
    return { ok: false, holderPid: readOwner(lockFile)?.pid ?? 0 };
  }
}

function releaseNamedLock(name: LockName): void {
  const held = heldLocks.get(name);
  if (!held) return;
  heldLocks.delete(name);
  try {
    unlinkSync(held.ownerPath);
  } catch {
    // owner 文件可能已被清理；不影响释放真正的锁目录。
  }
  try {
    held.release();
  } catch (err) {
    process.stderr.write(`[lock] 释放 ${name} 锁失败：${(err as Error).message}\n`);
  }
}

function releaseAllLocks(): void {
  releaseNamedLock("daemon");
  releaseNamedLock("run");
}

/** 获取 daemon 单实例锁。常驻入口使用；一次性/单站/登录不获取此锁。 */
export function acquireDaemonLock(): LockResult {
  const lock = acquireNamedLock("daemon");
  if (lock.ok) installShutdownHandlers();
  return lock;
}

/** 获取一次运行租约。每次 runAll/runSite/login 使用，执行结束后释放。 */
export function acquireRunLock(): LockResult {
  const lock = acquireNamedLock("run");
  if (lock.ok) installShutdownHandlers();
  return lock;
}

/** 释放一次运行租约；正常完成或启动失败时由调用方执行。 */
export function releaseRunLock(): void {
  releaseNamedLock("run");
}

/** 进程是否收到过终止信号；用于防止正常收尾把失败退出码覆盖为 0。 */
export function wasInterrupted(): boolean {
  return interrupted;
}

function installShutdownHandlers(): void {
  if (handlersInstalled) return;
  handlersInstalled = true;
  process.on("exit", releaseAllLocks);
  for (const sig of ["SIGINT", "SIGTERM", "SIGHUP"] as const) {
    process.on(sig, () => {
      if (shutdownTimer) return;
      interrupted = true;
      process.exitCode = 1;
      process.stderr.write(`[lock] received ${sig}; waiting up to ${SHUTDOWN_GRACE_MS}ms before exit\n`);
      shutdownTimer = setTimeout(() => process.exit(1), SHUTDOWN_GRACE_MS);
      shutdownTimer.unref();
    });
  }
}
