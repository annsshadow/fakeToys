// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

// runOne/runAll 分支测试。import 顺序敏感：
//  1. 必须先设 AUTO_CHECKIN_DATA_DIR（store.ts 在 import 时决定数据文件位置）；
//  2. 必须先设 PLAYWRIGHT_BROWSERS_PATH 指向空目录——playwright 的注册表在
//     模块加载时解析，环境变量必须先于 runner→browser→playwright 的加载链；
//     这样 chromium 启动必然失败，runner 的异常收敛分支才能被确定性覆盖，
//     且测试绝不碰真实站点。
// 因此本文件对 runner 只用动态导入，不写任何静态 import。
import assert from "node:assert/strict";
import { test } from "node:test";
import { mkdtempSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import type { AppConfig } from "../src/types.js";

process.env.AUTO_CHECKIN_DATA_DIR = mkdtempSync(join(tmpdir(), "auto-checkin-runner-"));
// 锁目录同样隔离：与其它测试文件并行时不能抢仓库里的 .run.lock
process.env.AUTO_CHECKIN_LOCK_DIR = mkdtempSync(join(tmpdir(), "auto-checkin-runner-lock-"));
process.env.PLAYWRIGHT_BROWSERS_PATH = "Z:" + join("nonexistent", "browsers");
delete process.env.VYCEAI_EMAIL;
delete process.env.VYCEAI_PASSWORD;

const { isRunning, runAll, runSite, runSiteInContext } = await import("../src/runner.js");

function cfg(sites: Record<string, { enabled: boolean }>): AppConfig {
  return {
    schedule: { cron: "0 9 * * *", timezone: "Asia/Shanghai", runOnStartup: false },
    browser: { headless: true, timeoutMs: 1000 },
    sites,
    panel: { host: "127.0.0.1", port: 8787, token: "" },
  };
}

test("未注册的站点：runSite / runSiteInContext 返回 null", async () => {
  assert.equal(await runSite("nonexistent", cfg({})), null);
  assert.equal(await runSiteInContext("nonexistent", cfg({}), {} as never), null);
  assert.equal(isRunning(), false);
});

test("配置中禁用的站点被跳过且落库，结果顺序与 adapters 一致", async () => {
  const results = await runAll(
    cfg({ vyceai: { enabled: false }, agentrouter: { enabled: false }, justwoker: { enabled: false } }),
  );
  assert.equal(results.length, 3, "adapters 注册了 3 个站点");
  assert.ok(results.every((r) => r.status === "skipped"), `全部应 skipped：${JSON.stringify(results.map((r) => r.status))}`);
  assert.ok(results.every((r) => r.message === "配置中已禁用"));
  assert.equal(isRunning(), false, "跑完必须释放运行标志");
});

test("password 站点未配置凭据 → skipped（不启动浏览器）", async () => {
  // dotenv 在上面的动态导入时才注入本地 .env 的凭据，必须在此刻删除才是「未配置」
  delete process.env.VYCEAI_EMAIL;
  delete process.env.VYCEAI_PASSWORD;
  const result = await runSite("vyceai", cfg({ vyceai: { enabled: true } }));
  assert.equal(result?.status, "skipped");
  assert.equal(result?.message, "未配置账号密码");
});

test("oauth 站点浏览器启动失败 → 收敛为 failed 并落库", async () => {
  const result = await runSite("justwoker", cfg({ justwoker: { enabled: true } }));
  assert.equal(result?.status, "failed");
  assert.match(result?.message ?? "", /运行异常/);
  assert.equal(isRunning(), false);
});

// 第 60 轮：失败自动重试策略（默认关闭，旧行为不变）。
test("retry: 失败站点按策略重试，每轮重试都落库", async () => {
  const { getAllRecords } = await import("../src/store.js");
  const before = getAllRecords().filter((r) => r.siteId === "justwoker").length;

  const c = cfg({ justwoker: { enabled: true } });
  c.retry = { enabled: true, attempts: 2, intervalMs: 20 };
  const result = await runSite("justwoker", c);
  assert.equal(result?.status, "failed", "浏览器不可用场景重试后仍应 failed");

  const after = getAllRecords().filter((r) => r.siteId === "justwoker").length;
  assert.equal(after - before, 3, "首次 + 2 次重试 = 3 条落库记录");
});

test("retry: 未配置 retry 段时行为不变（只跑一次，落一条）", async () => {
  const { getAllRecords } = await import("../src/store.js");
  const before = getAllRecords().filter((r) => r.siteId === "justwoker").length;

  const result = await runSite("justwoker", cfg({ justwoker: { enabled: true } }));
  assert.equal(result?.status, "failed");

  const after = getAllRecords().filter((r) => r.siteId === "justwoker").length;
  assert.equal(after - before, 1, "默认不重试，只落一条失败记录");
});

test("retry: 非 failed 结果（skipped）不触发重试", async () => {
  const { getAllRecords } = await import("../src/store.js");
  const before = getAllRecords().filter((r) => r.siteId === "vyceai").length;

  const c = cfg({ vyceai: { enabled: true }, justwoker: { enabled: false } });
  c.retry = { enabled: true, attempts: 3, intervalMs: 10 };
  const result = await runSite("vyceai", c);
  assert.equal(result?.status, "skipped", "凭据未配置应 skipped");

  const after = getAllRecords().filter((r) => r.siteId === "vyceai").length;
  assert.equal(after - before, 1, "skipped 是确定态，不应触发重试");
});
