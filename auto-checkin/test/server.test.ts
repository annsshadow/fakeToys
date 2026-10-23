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

const { addRecord } = await import("../src/store.js");
const { createApp } = await import("../src/server.js");
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
