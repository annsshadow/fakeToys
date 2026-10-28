// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import assert from "node:assert/strict";
import { test } from "node:test";
import { mkdtempSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

// login.ts 被导入时不得自动执行 main（直接运行守卫靠 import.meta.url 判断），
// 但其 import 链会加载 store——先设隔离目录
process.env.AUTO_CHECKIN_DATA_DIR = mkdtempSync(join(tmpdir(), "auto-checkin-login-"));

const { clearWafGate, shortUrl } = await import("../src/login.js");
import { FakePage, newApiSelf } from "./fake-page.js";

// clearWafGate 的 log 参数是 createLogger 的返回对象（info/warn/error 三通道）
function makeLogs() {
  const lines: string[] = [];
  const logger = {
    info: (m: string) => lines.push(`[info] ${m}`),
    warn: (m: string) => lines.push(`[warn] ${m}`),
    error: (m: string) => lines.push(`[error] ${m}`),
  };
  return { logger, lines };
}

test("shortUrl：取 host + 截断 pathname，非法输入回退原文", () => {
  assert.equal(shortUrl("https://agentrouter.org/console/overview?x=1"), "agentrouter.org/console/overview");
  assert.equal(shortUrl("not a url"), "not a url");
});

test("clearWafGate：接口读到数字余额 → 放行", async () => {
  const page = new FakePage({
    fetchRoutes: { "api/user/self": newApiSelf(123_456, "waf-tester") },
  });
  const { logger, lines } = makeLogs();
  const ok = await clearWafGate(page as never, "https://site.test", logger, 15_000);
  assert.equal(ok, true);
  assert.ok(lines.some((l) => l.includes("余额接口已放行")), "应记录放行与 quota");
});

test("clearWafGate：接口持续被 WAF 拦（非 JSON 挑战页）→ 超时返回 false", async () => {
  const { logger, lines } = makeLogs();
  // localStorage 里残留用户缓存：只判断「self 非 null」会把兜底误报成放行；
  // 这里的契约是「quota 必须是数字」——挑战页拿不到 quota，即便兜底读到用户名。
  const page = new FakePage({
    localStorage: { user: JSON.stringify({ username: "waf", quota: "残留" }) },
    fetchRoutes: {
      "api/user/self": { status: 200, body: '<html><meta name="aliyun_waf_aa"></html>' },
      "api/user/auth/refresh": { status: 200, body: '<html><meta name="aliyun_waf_bb"></html>' },
    },
  });
  const ok = await clearWafGate(page as never, "https://site.test", logger, 100);
  assert.equal(ok, false, "被拦截时必须判定未放行");
  assert.ok(lines.some((l) => l.includes("等待超时")));
});
