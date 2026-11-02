// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import assert from "node:assert/strict";
import { test } from "node:test";
import { mkdtempSync } from "node:fs";
import { createServer } from "node:http";
import { tmpdir } from "node:os";
import { join } from "node:path";
import type { Server } from "node:http";
import type { AddressInfo } from "node:net";

// 必须在导入 store/server 前设置，确保测试用临时数据目录
const DATA_DIR = mkdtempSync(join(tmpdir(), "auto-checkin-server-"));
process.env.AUTO_CHECKIN_DATA_DIR = DATA_DIR;
// 锁目录与数据目录一起隔离，避免与并行测试文件抢同一把 .run.lock
process.env.AUTO_CHECKIN_LOCK_DIR = DATA_DIR;

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
  webhookUrl = "",
): Promise<void> {
  const cfg = config(token);
  if (webhookUrl) cfg.notify = { webhookUrl };
  const app = createApp(cfg);
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

// 第 40 轮：/api/trend 在记录库版本不变时复用缓存（无写入不重算，
// 写入后必须看到新趋势）。ESM 命名导出无法被 node:test mock 计数，
// 所以用行为判据：无写入时连续请求返回相同快照、addRecord 后趋势变化。
test("trend 缓存：无写入时快照稳定，写入后更新", async () => {
  await withServer("", async (base) => {
    const first = await (await fetch(`${base}/api/trend?days=30`)).json()
    const second = await (await fetch(`${base}/api/trend?days=30`)).json()
    assert.deepEqual(second, first, "无写入时趋势快照必须稳定（缓存复用）")

    // 写入一条记录 → 版本号变化 → 趋势必须更新（不能返回旧缓存）
    const { addRecord } = await import("../src/store.js")
    addRecord({
      siteId: "vyceai",
      siteName: "vyceai",
      timestamp: new Date().toISOString(),
      status: "success",
      reward: 1,
      balance: 1,
      currency: "quota",
      durationMs: 10,
      message: "ok",
      screenshot: null,
    })
    const third = await (await fetch(`${base}/api/trend?days=30`)).json()
    assert.notDeepEqual(third.points, first.points, "写入后趋势应重算（缓存失效）")
  })
})

// 第 74 轮：单站手动触发（/api/run/:siteId）也必须推 webhook。
// 第 61 轮只接入了定时/全部签到/启动即跑，单站入口漏推——webhook 看不到单站结果。
test("单站手动触发同样推 webhook", async () => {
  await waitForIdle();
  // env 优先级高于 config；清空以免本机环境覆盖测试配置
  const savedEnv = process.env.AUTO_CHECKIN_WEBHOOK_URL;
  process.env.AUTO_CHECKIN_WEBHOOK_URL = "";

  const got: Array<{ type: string; results: Array<{ site: string }> }> = [];
  const receiver = createServer((req, res) => {
    let body = "";
    req.on("data", (chunk) => { body += chunk; });
    req.on("end", () => {
      got.push(JSON.parse(body) as typeof got[number]);
      res.statusCode = 204;
      res.end();
    });
  });
  await new Promise<void>((r) => receiver.listen(0, "127.0.0.1", () => r()));
  const { port } = receiver.address() as AddressInfo;

  try {
    await withServer("", async (base) => {
      const res = await fetch(`${base}/api/run/vyceai`, { method: "POST" });
      assert.equal(res.status, 200);
      await waitForIdle();
      // webhook 是 fire-and-forget，等投递完成（最多 3 秒）
      const deadline = Date.now() + 3000;
      while (got.length === 0 && Date.now() < deadline) {
        await new Promise((r) => setTimeout(r, 20));
      }
      assert.ok(got.length >= 1, "单站触发必须推送 webhook");
      assert.equal(got[got.length - 1].type, "auto-checkin.results");
      assert.equal(got[got.length - 1].results.length, 1, "单站 webhook 只含该站一条结果");
      assert.equal(got[got.length - 1].results[0].site, "VyceAI");
    }, `http://127.0.0.1:${port}/hook`);
  } finally {
    await new Promise((r) => receiver.close(r));
    if (savedEnv === undefined) delete process.env.AUTO_CHECKIN_WEBHOOK_URL;
    else process.env.AUTO_CHECKIN_WEBHOOK_URL = savedEnv;
  }
});

// 第 71 轮：records/CSV 支持 status 过滤；未知状态必须 400
// （静默回空集会被误读成「今天没有成功记录」——那是句假话）。
test("records 按状态过滤 + CSV 状态过滤 + 未知状态 fail-loud", async () => {
  const baseTime = Date.now()
  addRecord({ siteId: "vyceai", siteName: "VyceAI", timestamp: new Date(baseTime).toISOString(), status: "success", reward: 1, balance: 2, currency: "quota", durationMs: 10, message: "l71-success", screenshot: null })
  addRecord({ siteId: "vyceai", siteName: "VyceAI", timestamp: new Date(baseTime + 1).toISOString(), status: "failed", reward: null, balance: null, currency: null, durationMs: 20, message: "l71-vyceai-failed", screenshot: null })
  addRecord({ siteId: "agentrouter", siteName: "AgentRouter", timestamp: new Date(baseTime + 2).toISOString(), status: "failed", reward: null, balance: null, currency: null, durationMs: 30, message: "l71-agent-failed", screenshot: null })

  await withServer("", async (base) => {
    type Row = { siteId: string; status: string; message: string }

    // 单条件：只回失败记录
    const onlyFailed = (await (await fetch(`${base}/api/records?status=failed`)).json()) as Row[]
    assert.ok(onlyFailed.length >= 2)
    assert.ok(onlyFailed.every((r) => r.status === "failed"))
    assert.ok(onlyFailed.some((r) => r.message === "l71-vyceai-failed"))
    assert.ok(!onlyFailed.some((r) => r.message === "l71-success"))

    // 站点 + 状态：两个条件必须同时成立（不是「或」）
    const combo = (await (await fetch(`${base}/api/records?siteId=vyceai&status=failed`)).json()) as Row[]
    assert.ok(combo.length >= 1)
    assert.ok(combo.every((r) => r.siteId === "vyceai" && r.status === "failed"))
    assert.ok(!combo.some((r) => r.message === "l71-agent-failed"))

    // 未知状态：接口必须 400，不能静默回空集
    const bad = await fetch(`${base}/api/records?status=succes`)
    assert.equal(bad.status, 400)
    assert.match(((await bad.json()) as { error: string }).error, /未知状态/)
    assert.equal((await fetch(`${base}/api/records.csv?status=nope`)).status, 400)

    // CSV 同样按状态过滤
    const csv = await (await fetch(`${base}/api/records.csv?status=failed`)).text()
    assert.ok(csv.includes("l71-vyceai-failed"))
    assert.ok(!csv.includes("l71-success"))
  })
})

test("records 按站点过滤 + CSV 导出", async () => {
  const { addRecord } = await import("../src/store.js")
  const baseTime = Date.now()
  addRecord({ siteId: "vyceai", siteName: "VyceAI", timestamp: new Date(baseTime).toISOString(), status: "success", reward: 1, balance: 2, currency: "quota", durationMs: 10, message: "a", screenshot: null })
  addRecord({ siteId: "agentrouter", siteName: "AgentRouter", timestamp: new Date(baseTime + 1).toISOString(), status: "failed", reward: null, balance: null, currency: null, durationMs: 20, message: "b", screenshot: null })

  await withServer("", async (base) => {
    // 站点过滤
    const filtered = await (await fetch(`${base}/api/records?siteId=vyceai`)).json()
    assert.ok(filtered.length >= 1)
    assert.ok(filtered.every((r: { siteId: string }) => r.siteId === "vyceai"))

    // CSV 导出：BOM + 表头 + 数据行（Response.text() 会剥离 BOM，用字节验证）
    const csvResp = await fetch(`${base}/api/records.csv`)
    const buf = new Uint8Array(await csvResp.arrayBuffer())
    assert.deepEqual([...buf.slice(0, 3)], [0xef, 0xbb, 0xbf], "CSV 应以 UTF-8 BOM 开头")
    const csv = new TextDecoder().decode(buf)
    assert.ok(csv.includes("timestamp,site_id"), "CSV 应含表头")
    assert.ok(csv.includes("vyceai,VyceAI"))
    assert.ok(csv.includes("agentrouter,AgentRouter"))

    // CSV 按站点过滤
    const csvFiltered = await (await fetch(`${base}/api/records.csv?siteId=vyceai`)).text()
    assert.ok(csvFiltered.includes("vyceai"))
    assert.ok(!csvFiltered.includes("agentrouter,AgentRouter"))
  })
})

// 第 79 轮：总成功率与单站同口径——跳过（未配置/已禁用）不进分母。
// 旧实现总量用 totalRuns 当分母，混入 skipped 后与站点卡片的 successRate
// （排除 skipped）自相矛盾；面板脚注也把 skipped 当成了成功。
test("总成功率排除 skipped，与单站口径一致", async () => {
  const baseTime = Date.now()
  addRecord({ siteId: "justwoker", siteName: "JustWoker", timestamp: new Date(baseTime).toISOString(), status: "skipped", reward: null, balance: null, currency: null, durationMs: 0, message: "l79-skipped", screenshot: null })
  addRecord({ siteId: "justwoker", siteName: "JustWoker", timestamp: new Date(baseTime + 1).toISOString(), status: "failed", reward: null, balance: null, currency: null, durationMs: 20, message: "l79-failed", screenshot: null })

  await withServer("", async (base) => {
    const body = await (await fetch(`${base}/api/stats`)).json() as {
      sites: Array<{ totalRuns: number; failedRuns: number; skippedRuns: number; successRate: number }>
      totals: { totalRuns: number; failedRuns: number; successRate: number }
    }
    const skipped = body.sites.reduce((a, s) => a + s.skippedRuns, 0)
    assert.ok(skipped >= 1, "应统计到 l79-skipped")
    const effective = body.sites.reduce((a, s) => a + s.totalRuns - s.skippedRuns, 0)
    const failed = body.sites.reduce((a, s) => a + s.failedRuns, 0)
    assert.equal(
      body.totals.successRate,
      effective > 0 ? Math.round(((effective - failed) / effective) * 100) : 0,
      "总成功率必须按「有效运行」算，与单站同口径",
    )
    // 若实现退回旧口径（totalRuns 含 skipped），两值必然不同——保证用例不会空转
    const oldRate = body.totals.totalRuns > 0
      ? Math.round(((body.totals.totalRuns - failed) / body.totals.totalRuns) * 100)
      : 0
    assert.notEqual(body.totals.successRate, oldRate, "存在 skipped 时新旧口径应不同")
  })
})

// 第 80 轮：CSV 公式注入防护。message/currency 等字段由远端站点控制，
// 导出后若以 = + - @ 开头会被 Excel 当公式执行。
test("CSV 导出对公式前缀做文本化处理", async () => {
  addRecord({
    siteId: "vyceai", siteName: "VyceAI", timestamp: new Date().toISOString(),
    status: "success", reward: 1, balance: 2,
    currency: "+1", durationMs: 10,
    message: '=HYPERLINK("http://evil")', screenshot: null,
  })
  await withServer("", async (base) => {
    const csv = await (await fetch(`${base}/api/records.csv`)).text()
    assert.ok(csv.includes("'=HYPERLINK"), "公式前缀必须被单引号文本化")
    assert.ok(!csv.includes("\n=HYPERLINK"), "不得原样输出以 = 开头的单元格")
    assert.ok(csv.includes("'+1"), "以 + 开头的 currency 同样需要文本化")
    // 正常文本不受影响
    assert.ok(csv.includes("l71-success"))
  })
})

// 第 87 轮：总平均耗时必须按耗时样本数加权。旧实现把各站 avgDurationMs
// 算术平均，1 次运行的站与 500 次运行的站权重相同——站点间样本量差异越大，
// 总量读数越失真。
test("总平均耗时按样本数加权，而非站点平均再平均", async () => {
  const baseTime = Date.now()
  // 制造悬殊样本量：vyceai 5 条 5000ms，justwoker 1 条 1ms
  for (let i = 0; i < 5; i++) {
    addRecord({ siteId: "vyceai", siteName: "VyceAI", timestamp: new Date(baseTime + i).toISOString(), status: "success", reward: 1, balance: 1, currency: "quota", durationMs: 5000, message: `l87-heavy-${i}`, screenshot: null })
  }
  addRecord({ siteId: "justwoker", siteName: "JustWoker", timestamp: new Date(baseTime + 10).toISOString(), status: "success", reward: 1, balance: 1, currency: "quota", durationMs: 1, message: "l87-light", screenshot: null })

  await withServer("", async (base) => {
    const body = await (await fetch(`${base}/api/stats`)).json() as {
      sites: Array<{ siteId: string; avgDurationMs: number; durationMsSum: number; durationMsCount: number }>
      totals: { avgDurationMs: number }
    }
    const sum = body.sites.reduce((a, s) => a + s.durationMsSum, 0)
    const count = body.sites.reduce((a, s) => a + s.durationMsCount, 0)
    assert.equal(body.totals.avgDurationMs, Math.round(sum / count), "总量必须是全部样本的加权平均")

    // 旧口径（各站平均值的算术平均）必须不同，保证用例不会空转
    const positive = body.sites.filter((s) => s.avgDurationMs > 0)
    const old = Math.round(positive.reduce((a, s) => a + s.avgDurationMs, 0) / positive.length)
    assert.notEqual(body.totals.avgDurationMs, old)
  })
})

// 第 92 轮：`sitesConfigured` 依赖凭据 / oauth 会话，不能只按记录库版本缓存。
// 旧实现无签到写入时永远返回首次快照——刚配好凭据的站点在面板上一直显示「未配置」。
test("凭据变化必须让统计缓存失效（sitesConfigured 不冻结）", async () => {
  await waitForIdle()
  const savedEmail = process.env.VYCEAI_EMAIL
  const savedPassword = process.env.VYCEAI_PASSWORD
  delete process.env.VYCEAI_EMAIL
  delete process.env.VYCEAI_PASSWORD
  try {
    await withServer("", async (base) => {
      const r1 = await (await fetch(`${base}/api/stats`)).json() as {
        totals: { sitesConfigured: number }
      }
      const before = r1.totals.sitesConfigured
      // 配置一份假凭据（无需真实站点，getCredentials 只看环境变量）
      process.env.VYCEAI_EMAIL = "placeholder@example.invalid"
      process.env.VYCEAI_PASSWORD = "placeholder"
      const r2 = await (await fetch(`${base}/api/stats`)).json() as {
        totals: { sitesConfigured: number }
      }
      // 旧实现按记录版本缓存：无写入时这里必回旧快照，before+1 当场失败
      assert.equal(r2.totals.sitesConfigured, before + 1, "新配置的站点必须立即计入已配置数")
    })
  } finally {
    if (savedEmail === undefined) delete process.env.VYCEAI_EMAIL
    else process.env.VYCEAI_EMAIL = savedEmail
    if (savedPassword === undefined) delete process.env.VYCEAI_PASSWORD
    else process.env.VYCEAI_PASSWORD = savedPassword
  }
})

// 第 93 轮：/api/records 游标分页。旧接口只能取最近 limit 条，更早的历史
// 除导出 CSV 无法翻页取得；`before` + `X-Next-Cursor` 让客户端能一路翻到最早。
test("records 游标分页：不重不漏，非法游标 400", async () => {
  const baseTime = Date.now()
  const expected: string[] = []
  for (let i = 0; i < 12; i++) {
    const message = `l93-page-${i}`
    expected.push(message)
    addRecord({
      siteId: "cursor-l93", siteName: "Cursor", timestamp: new Date(baseTime + i).toISOString(),
      status: "success", reward: 1, balance: 1, currency: "quota", durationMs: 10,
      message, screenshot: null,
    })
  }

  await withServer("", async (base) => {
    const got: string[] = []
    let cursor = ""
    for (let page = 0; page < 10; page++) {
      const url = `${base}/api/records?siteId=cursor-l93&limit=5`
        + (cursor ? `&before=${encodeURIComponent(cursor)}` : "")
      const res = await fetch(url)
      assert.equal(res.status, 200)
      const rows = await res.json() as Array<{ message: string }>
      if (rows.length === 0) break
      got.push(...rows.map((r) => r.message))
      const next = res.headers.get("x-next-cursor")
      assert.ok(next, "非空页必须带 X-Next-Cursor")
      cursor = next
    }
    assert.deepEqual(got, [...expected].reverse(), "逐页翻完必须恰好覆盖全部记录，且新→旧")
    assert.equal(new Set(got).size, expected.length, "翻页不得出现重复记录")

    // 非法游标 fail-loud：静默回第一页会让客户端无限循环
    const bad = await fetch(`${base}/api/records?siteId=cursor-l93&before=not-a-date`)
    assert.equal(bad.status, 400)
    assert.match(((await bad.json()) as { error: string }).error, /非法游标/)
  })
})

test("同毫秒多条记录翻页不重不漏（游标 skip 段）", async () => {
  const shared = new Date(Date.now() - 1000).toISOString()
  addRecord({
    siteId: "cursor-same", siteName: "Cursor", timestamp: new Date(Date.now() - 60_000).toISOString(),
    status: "success", reward: 1, balance: 1, currency: "quota", durationMs: 10,
    message: "same-oldest", screenshot: null,
  })
  for (let i = 0; i < 4; i++) {
    addRecord({
      siteId: "cursor-same", siteName: "Cursor", timestamp: shared,
      status: "success", reward: 1, balance: 1, currency: "quota", durationMs: 10,
      message: `same-${i}`, screenshot: null,
    })
  }

  await withServer("", async (base) => {
    const got: string[] = []
    let cursor = ""
    for (let page = 0; page < 10; page++) {
      const url = `${base}/api/records?siteId=cursor-same&limit=2`
        + (cursor ? `&before=${encodeURIComponent(cursor)}` : "")
      const res = await fetch(url)
      const rows = await res.json() as Array<{ message: string }>
      if (rows.length === 0) break
      got.push(...rows.map((r) => r.message))
      cursor = res.headers.get("x-next-cursor") ?? ""
    }
    assert.deepEqual(got, ["same-3", "same-2", "same-1", "same-0", "same-oldest"])
  })
})
