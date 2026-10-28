// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import assert from "node:assert/strict";
import { test } from "node:test";
import { mkdtempSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import type { Server } from "node:http";
import type { AddressInfo } from "node:net";

// 必须在导入 store/server 前设置，确保测试用临时数据目录
const DATA_DIR = mkdtempSync(join(tmpdir(), "auto-checkin-server-"));
process.env.AUTO_CHECKIN_DATA_DIR = DATA_DIR;

// run 路由会触发 runAll/runSite → 浏览器启动；指向空目录让其确定性失败，
// 测试绝不碰真实站点（必须在 server.js 加载 runner 链之前设置）
process.env.PLAYWRIGHT_BROWSERS_PATH = "Z:" + join("nonexistent", "browsers");

const { addRecord } = await import("../src/store.js");
const { createApp } = await import("../src/server.js");
const { isRunning } = await import("../src/runner.js");

/** 等 runner 空闲：run 路由是 fire-and-forget，跨测试边界仍可能在途 */
async function waitForIdle(timeoutMs = 5000): Promise<void> {
  const deadline = Date.now() + timeoutMs;
  while (Date.now() < deadline) {
    if (!isRunning()) return;
    await new Promise((r) => setTimeout(r, 20));
  }
  throw new Error("runner 5 秒内未空闲，测试卫生失败");
}
import type { AppConfig } from "../src/types.js";

function config(token = ""): AppConfig {
  return {
    schedule: { cron: "0 9 * * *", timezone: "Asia/Shanghai", runOnStartup: false },
    browser: { headless: true, timeoutMs: 45000 },
    sites: { vyceai: { enabled: true }, agentrouter: { enabled: true }, justwoker: { enabled: true } },
    panel: { host: "127.0.0.1", port: 0, token },
  };
}

async function withServer(
  token: string,
  fn: (base: string) => Promise<void>,
): Promise<void> {
  const app = createApp(config(token));
  const server: Server = await new Promise((resolve) => {
    const s = app.listen(0, "127.0.0.1", () => resolve(s));
  });
  const { port } = server.address() as AddressInfo;
  try {
    await fn(`http://127.0.0.1:${port}`);
  } finally {
    await new Promise((resolve) => server.close(resolve));
  }
}

function record(message: string) {
  return {
    siteId: "vyceai",
    siteName: "VyceAI",
    timestamp: new Date().toISOString(),
    status: "success" as const,
    reward: 1,
    balance: 10,
    currency: "$",
    message,
    screenshot: null,
    durationMs: 100,
  };
}

test("统计接口：无写入的连续请求复用缓存（generatedAt 不变），写入后刷新", async () => {
  await withServer("", async (base) => {
    const r1 = await (await fetch(`${base}/api/stats`)).json();
    const r2 = await (await fetch(`${base}/api/stats`)).json();
    assert.equal(r1.generatedAt, r2.generatedAt, "缓存未命中：两次请求重新计算了统计");
    assert.equal(r1.totals.totalRuns, 0);

    // 故意拉开时间差：若没有缓存，generatedAt 必然不同
    await new Promise((r) => setTimeout(r, 20));
    const r3 = await (await fetch(`${base}/api/stats`)).json();
    assert.equal(r3.generatedAt, r1.generatedAt);

    // 写入一条记录后缓存失效，统计反映新数据
    addRecord(record("cache-invalidation"));
    const r4 = await (await fetch(`${base}/api/stats`)).json();
    assert.equal(r4.totals.totalRuns, 1);
    assert.notEqual(r4.generatedAt, r1.generatedAt);
  });
});

test("口令启用时：?token= 查询参数一律 401", async () => {
  await withServer("secret", async (base) => {
    const q = await fetch(`${base}/api/stats?token=secret`);
    assert.equal(q.status, 401, "查询参数中的 token 必须被拒绝");
    const none = await fetch(`${base}/api/stats`);
    assert.equal(none.status, 401);
    const wrong = await fetch(`${base}/api/stats`, { headers: { "X-Token": "nope" } });
    assert.equal(wrong.status, 401);
  });
});

test("口令启用时：X-Token 头可以正常访问", async () => {
  await withServer("secret", async (base) => {
    const ok = await fetch(`${base}/api/stats`, { headers: { "X-Token": "secret" } });
    assert.equal(ok.status, 200);
    const body = await ok.json();
    assert.ok(Array.isArray(body.sites));
    assert.ok(body.totals.totalRuns >= 1, "应看到之前写入的记录（写穿透缓存）");
  });
});

test("未设置口令时接口直接放行", async () => {
  await withServer("", async (base) => {
    const res = await fetch(`${base}/api/stats`);
    assert.equal(res.status, 200);
  });
});

test("口令启用时：面板静态资源放行、API 仍守（2026-10-24 面板 401 缺陷回归）", async () => {
  await withServer("secret", async (base) => {
    // 曾经的放行清单是 "/" 与 /assets*，与实际文件布局（根路径 style.css/app.js）
    // 不符，导致设了 PANEL_TOKEN 后 CSS/JS 全 401、面板不可用。
    for (const p of ["/", "/style.css", "/app.js"]) {
      const r = await fetch(`${base}${p}`);
      assert.equal(r.status, 200, `${p} 应放行（静态资源不属敏感面）`);
    }
    // 静态面不存在的文件：应落到 static 的 404，而非被口令中间件 401——
    // 证明放行规则没有把"任意路径都返回 200"
    const missing = await fetch(`${base}/screenshots/definitely-not-here.png`);
    assert.equal(missing.status, 404, "静态缺失文件应为 404（static 落空），而非 401");
    // API 面依旧只认 X-Token 头
    const api = await fetch(`${base}/api/stats`);
    assert.equal(api.status, 401);
  });
});

test("历史记录：limit 下限夹取（负值不静默丢数据）", async () => {
  await withServer("", async (base) => {
    addRecord(record("limit-clamp"));
    // ?limit=-5：未夹取时 slice(5) 静默丢掉前若干条；夹取后等价于 limit=1
    const r = await (await fetch(`${base}/api/records?limit=-5`)).json();
    assert.equal(r.length, 1, "?limit=-5 应被夹到 1，返回最近 1 条而非空集");
    const zero = await (await fetch(`${base}/api/records?limit=0`)).json();
    assert.ok(zero.length >= 1, "?limit=0 走默认 100，不应被夹成 0 条");
  });
});

test("趋势接口：days 夹取到 [7, 90]，返回站点清单与点位", async () => {
  await withServer("", async (base) => {
    const r = await (await fetch(`${base}/api/trend`)).json();
    assert.equal(r.days, 30, "缺省 30 天");
    assert.ok(Array.isArray(r.sites) && r.sites.length >= 3, "应列出全部注册站点");
    assert.ok("points" in r);

    assert.equal((await (await fetch(`${base}/api/trend?days=5`)).json()).days, 7, "下限夹到 7");
    assert.equal((await (await fetch(`${base}/api/trend?days=999`)).json()).days, 90, "上限夹到 90");
    assert.equal((await (await fetch(`${base}/api/trend?days=abc`)).json()).days, 30, "非数字走默认");
  });
});

test("面板触发全部签到：立即 200；进行中再触发 → 409（running 守卫）", async () => {
  await withServer("", async (base) => {
    // 两个请求**同时发出**（不先等第一个返回）：CI 上三个适配器会在数毫秒内
    // 跳过/失败完毕，若等第一个响应再发第二个，runAll 可能已结束、409 断言
    // 必 flaky。同时发出时第二个请求的处理必然落在第一个 runAll 的在途窗口内。
    const pending = [
      fetch(`${base}/api/run`, { method: "POST" }),
      fetch(`${base}/api/run`, { method: "POST" }),
    ];
    const [first, second] = await Promise.all(pending);
    assert.equal(first.status, 200);
    assert.equal(second.status, 409, "running 守卫必须拦下并发触发");
    await waitForIdle(); // 不把在途的 runAll 泄漏给下一个测试
  });
});

test("面板触发单站：未知站点 404（不占锁），已知站点 200", async () => {
  await waitForIdle(); // 防上一条用例的在途 runAll 污染 isRunning
  await withServer("", async (base) => {
    const unknown = await fetch(`${base}/api/run/no-such-site`, { method: "POST" });
    assert.equal(unknown.status, 404);

    // 单站的 409 守卫不在此断言：单站 runSite（浏览器启动失败）毫秒级完成，
    // 第二个请求必然赶不上 running 窗口，严格断言是 flaky——守卫语义已由
    // 「全部签到」路由的 409 用例覆盖（同一 isRunning 判据）。
    const known = await fetch(`${base}/api/run/vyceai`, { method: "POST" });
    assert.equal(known.status, 200);
  });
});
