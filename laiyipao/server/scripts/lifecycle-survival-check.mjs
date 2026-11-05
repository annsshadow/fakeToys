#!/usr/bin/env node
// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// lifecycle-survival-check.mjs — 进程级存活断言（R103 前的结构性守卫）。
//
// # 为什么需要它（同类缺陷的真实样本）
//
// oa4rust 的 `shared::graceful::serve_router_with_shutdown` 曾把
// `tokio::time::timeout(grace, server_future)` 包在**整个服务生命周期**上：
// 服务启动 10 秒后无条件自杀，与有没有收到关闭信号无关，持续 3 天无人发现。
// 之所以没人发现，是因为启动冒烟在进程启动第 2 秒就 exit 0 —— 永远看不到
// +10s 的事件。
//
// laiyipao 当前的写法是正确的（`app.Listen` 裸调用，宽限只作用于
// `ShutdownWithTimeout`）。但「暂时正确」不是守卫：把 `app.Listen` 包进
// 启动 ctx、或在 `ShutdownWithTimeout` 外套一层超时，代码一样能编译、
// 短冒烟一样全绿。所以本脚本**不发任何信号**，先让真实进程活过
// 最长内部超时，再发 SIGTERM。
//
// # 判据
//
//   1. 起真实 `cmd/api`（真实 PG），不发任何信号；
//   2. 在 15s / 30s / 60s / 窗口终点各检查一次：进程存活 +
//      `/healthz` 200 + `/readyz` 200；
//   3. 窗口终点必须 ≥ 2× 最长内部超时（启动 ctx 60s → 120s）；
//   4. 之后才发 SIGTERM，断言在停机宽限（DB_STATEMENT_TIMEOUT 30s + 5s =
//      35s，留 5s 余量 → 40s）内退出。
//
// # 为什么窗口是 120s 而不是 65s
//
// 65s 只覆盖「60s 启动 ctx + 35s 停机宽限」的一次边界；任何把超时
// 写进**生命周期**的缺陷在边界附近都可能因调度噪声漏检。
// `smoke-window-guard.mjs` 的统一口径是「最后一条检查 ≥ 2× 最长内部超时」，
// 这里直接按那个口径执行：2 × 60s = 120s。窗口不可被命令行改小
// （`LIFECYCLE_WINDOW_MS` 低于下限直接退出 2），否则守卫会被「调快」成装饰。
//
// # 用法
//
//   node scripts/lifecycle-survival-check.mjs [--port 18092]
//
// 前置：DATABASE_URL 指向已 migrate + seed 的库（CI 的 server job 已满足）。
// 需要 Node 18+（fetch / AbortSignal.timeout）。POSIX 上会校验优雅退出码 0；
// Windows 的 SIGTERM 是强制终止，只校验「按预期退出」，不校验退出码。
import { spawn, execSync } from 'node:child_process';
import { mkdirSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

// LIFECYCLE-COVERAGE: full window_ms=120000 reason=真实进程无信号存活断言（覆盖启动 ctx 60s 与停机宽限 35s）
const SERVER_DIR = dirname(dirname(fileURLToPath(import.meta.url)));
const args = process.argv.slice(2);
const argOf = (name, def) => {
  const i = args.indexOf('--' + name);
  return i >= 0 ? args[i + 1] : def;
};

// 下限来自 cmd/api/main.go 的启动 ctx（60s）；smoke-window-guard 会静态
// 对拍这个常量与 main.go，改源码而不改这里会让守卫红。
const MIN_WINDOW_MS = 120000;
const WINDOW_MS = Number(process.env.LIFECYCLE_WINDOW_MS || MIN_WINDOW_MS);
const SHUTDOWN_LIMIT_MS = 40000; // 35s 宽限 + 5s 余量

if (!Number.isFinite(WINDOW_MS) || WINDOW_MS < MIN_WINDOW_MS) {
  console.error(
    `::error::LIFECYCLE_WINDOW_MS=${WINDOW_MS} 低于下限 ${MIN_WINDOW_MS}ms —— ` +
    '窗口不能调小，否则这条守卫就看不到「服务在固定秒数自杀」的缺陷。',
  );
  process.exit(2);
}

const DSN = process.env.DATABASE_URL || 'postgres://postgres@127.0.0.1:5432/laiyipao?sslmode=disable';
const PORT = Number(argOf('port', 18092));

const sleep = (ms) => new Promise((r) => setTimeout(r, ms));
const elapsed = (t0) => Date.now() - t0;

async function probe(path) {
  try {
    const r = await fetch(`http://127.0.0.1:${PORT}${path}`, {
      signal: AbortSignal.timeout(3000),
    });
    return r.status;
  } catch {
    return 'ERR';
  }
}

function build() {
  const binDir = join(tmpdir(), 'laiyipao-lifecycle');
  mkdirSync(binDir, { recursive: true });
  const exe = join(binDir, process.platform === 'win32' ? 'api.exe' : 'api');
  console.log(`构建 api → ${exe}`);
  execSync(`go build -o "${exe}" ./cmd/api`, { cwd: SERVER_DIR, stdio: 'inherit' });
  return exe;
}

async function main() {
  const exe = build();
  const t0 = Date.now();
  const child = spawn(exe, [], {
    env: {
      ...process.env,
      DATABASE_URL: DSN,
      ADDR: `:${PORT}`,
      LOG_LEVEL: 'error',
      // 生命周期检查不需要迁移：数据库由前置步骤负责。关掉迁移也让
      // 本守卫只回答「进程会不会自己死」，不和迁移耗时混在一起。
      MIGRATIONS_ON_BOOT: 'false',
    },
    stdio: ['ignore', 'pipe', 'pipe'],
  });
  let exitInfo = null;
  const output = [];
  child.stdout.on('data', (d) => output.push(String(d)));
  child.stderr.on('data', (d) => output.push(String(d)));
  child.on('exit', (code, signal) => {
    exitInfo = { code, signal, atMs: elapsed(t0) };
  });

  const cleanExit = () => {
    if (!exitInfo) {
      try { child.kill('SIGKILL'); } catch { /* 已退 */ }
    }
  };

  try {
    // 等就绪（不计入生命周期判定，只是让后面的探活有意义）。
    let readyAt = null;
    for (;;) {
      if (exitInfo) {
        throw new Error(
          `进程在就绪前退出（${exitInfo.atMs}ms，code=${exitInfo.code} signal=${exitInfo.signal}）\n` +
          output.join(''),
        );
      }
      if ((await probe('/healthz')) === 200 && (await probe('/readyz')) === 200) {
        readyAt = elapsed(t0);
        break;
      }
      if (elapsed(t0) > 60000) {
        throw new Error('60s 内 /healthz 与 /readyz 未同时 200');
      }
      await sleep(500);
    }
    console.log(`[${elapsed(t0)}ms] 就绪（${readyAt}ms）`);

    const checkpoints = [...new Set([15000, 30000, 60000, WINDOW_MS])]
      .filter((at) => at <= WINDOW_MS)
      .sort((a, b) => a - b);
    if (checkpoints[checkpoints.length - 1] !== WINDOW_MS) checkpoints.push(WINDOW_MS);

    for (const at of checkpoints) {
      while (elapsed(t0) < at) await sleep(200);
      const now = elapsed(t0);
      if (exitInfo) {
        throw new Error(
          `进程在未收到任何信号时退出：${exitInfo.atMs}ms（检查点 ${at}ms，` +
          `code=${exitInfo.code} signal=${exitInfo.signal}）\n` + output.join(''),
        );
      }
      const [health, ready] = await Promise.all([probe('/healthz'), probe('/readyz')]);
      if (health !== 200 || ready !== 200) {
        throw new Error(
          `检查点 ${at}ms：进程存活但探活失败（/healthz=${health} /readyz=${ready}）`,
        );
      }
      console.log(`[${now}ms] 存活 ✓ /healthz=200 /readyz=200（检查点 ${at}ms）`);
    }

    // 到这里才发终止信号 —— 这是本守卫与短冒烟的根本区别。
    const termAt = elapsed(t0);
    console.log(`[${termAt}ms] 发送 SIGTERM，等待优雅退出（上限 ${SHUTDOWN_LIMIT_MS}ms）`);
    try { child.kill('SIGTERM'); } catch { /* 已退 */ }

    while (!exitInfo) {
      if (elapsed(t0) - termAt > SHUTDOWN_LIMIT_MS) {
        throw new Error(
          `SIGTERM 后 ${SHUTDOWN_LIMIT_MS}ms 内未退出（停机宽限被突破）\n` + output.join(''),
        );
      }
      await sleep(100);
    }
    const shutdownMs = exitInfo.atMs - termAt;
    if (process.platform !== 'win32' && exitInfo.code !== 0) {
      throw new Error(
        `SIGTERM 后退出码 = ${exitInfo.code}（期望 0，说明优雅关闭路径没走完）\n` +
        output.join(''),
      );
    }
    console.log(
      `[${exitInfo.atMs}ms] SIGTERM 后 ${shutdownMs}ms 退出（code=${exitInfo.code} ` +
      `signal=${exitInfo.signal}）`,
    );
    console.log(
      `LIFECYCLE-SURVIVAL: PASS window_ms=${WINDOW_MS} ready_ms=${readyAt} ` +
      `shutdown_ms=${shutdownMs}`,
    );
    process.exitCode = 0;
  } catch (e) {
    console.error('::error::' + e.message);
    process.exitCode = 1;
  } finally {
    cleanExit();
  }
}

main();
