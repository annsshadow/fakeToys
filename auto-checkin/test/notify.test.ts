// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import assert from "node:assert/strict";
import { test } from "node:test";
import { createServer, type IncomingMessage, type ServerResponse } from "node:http";
import type { AppConfig, CheckinResult } from "../src/types.js";
import { notifyResults, resolveWebhookUrl } from "../src/notify.js";

// 第 61 轮：签到结果 webhook 通知。
// 用一个本地 http 服务（随机端口）当 webhook 接收端，断言 payload 形状；
// 另验 env 优先级、空 URL 跳过、网络失败不抛（best-effort 语义）。

function cfg(over: Partial<AppConfig> = {}): AppConfig {
  return {
    schedule: { cron: "0 9 * * *", timezone: "Asia/Shanghai", runOnStartup: false },
    browser: { headless: true, timeoutMs: 45000 },
    sites: { vyceai: { enabled: true } },
    panel: { host: "127.0.0.1", port: 0, token: "" },
    ...over,
  };
}

function result(status: CheckinResult["status"], siteId = "vyceai"): CheckinResult {
  return {
    siteId,
    siteName: "VyceAI",
    timestamp: new Date().toISOString(),
    status,
    reward: status === "success" ? 1 : null,
    balance: 5,
    currency: "$",
    durationMs: 100,
    message: status === "success" ? "领取成功" : "出错了",
    screenshot: null,
  };
}

test("有 webhookUrl 时 POST 结果摘要，payload 含 summary 与逐站结果", async () => {
  let received: any = null;
  const server = createServer((req: IncomingMessage, res: ServerResponse) => {
    let body = "";
    req.on("data", (c) => (body += c));
    req.on("end", () => {
      received = { method: req.method, url: req.url, headers: req.headers, body: JSON.parse(body) };
      res.writeHead(200);
      res.end("ok");
    });
  });
  await new Promise<void>((r) => server.listen(0, "127.0.0.1", () => r()));
  const port = (server.address() as any).port;

  const c = cfg({ notify: { webhookUrl: `http://127.0.0.1:${port}/hook` } });
  await notifyResults(c, [result("success"), result("failed", "agentrouter"), result("skipped", "justwoker")]);

  server.close();
  assert.ok(received, "webhook 应收到 POST");
  assert.equal(received.method, "POST");
  assert.equal(received.url, "/hook");
  assert.equal(received.headers["content-type"], "application/json");
  assert.equal(received.body.type, "auto-checkin.results");
  assert.deepEqual(
    received.body.summary,
    { total: 3, ok: 1, skipped: 1, failed: 1 },
    "summary 分档计数应准确",
  );
  assert.equal(received.body.results.length, 3);
  assert.equal(received.body.results[0].site, "VyceAI");
});

test("resolveWebhookUrl：env 优先于 config", () => {
  const saved = process.env.AUTO_CHECKIN_WEBHOOK_URL;
  process.env.AUTO_CHECKIN_WEBHOOK_URL = "http://env.example/hook";
  try {
    const c = cfg({ notify: { webhookUrl: "http://config.example/hook" } });
    assert.equal(resolveWebhookUrl(c), "http://env.example/hook");
    delete process.env.AUTO_CHECKIN_WEBHOOK_URL;
    assert.equal(resolveWebhookUrl(c), "http://config.example/hook");
  } finally {
    if (saved === undefined) delete process.env.AUTO_CHECKIN_WEBHOOK_URL;
    else process.env.AUTO_CHECKIN_WEBHOOK_URL = saved;
  }
});

test("无 URL / 空结果：静默跳过（不发请求、不抛）", async () => {
  await assert.doesNotReject(async () => {
    await notifyResults(cfg(), [result("success")]); // 无 notify 段
  });
  const c = cfg({ notify: { webhookUrl: "http://127.0.0.1:1/never" } });
  await assert.doesNotReject(async () => {
    await notifyResults(c, []); // 无结果不发
  });
});

test("webhook 网络失败：best-effort 不抛异常（不影响主流程）", async () => {
  const c = cfg({ notify: { webhookUrl: "http://127.0.0.1:9/unreachable" } });
  await assert.doesNotReject(async () => {
    await notifyResults(c, [result("failed")]);
  });
});