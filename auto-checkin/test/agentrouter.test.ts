// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import assert from "node:assert/strict";
import { test } from "node:test";
import { agentrouterAdapter } from "../src/sites/agentrouter.js";
import { FakePage, makeAdapterCtx, newApiSelf } from "./fake-page.js";
import type { AdapterOutcome } from "../src/types.js";

function selfResult(quota: number | null, username = "tester") {
  return { user: { quota, username }, wafBlocked: false };
}

async function runCheckin(page: FakePage) {
  const ctx = makeAdapterCtx(page);
  const outcome: AdapterOutcome = await agentrouterAdapter.checkin(ctx);
  return { outcome, ctx };
}

/** 与 runCheckin 相同，但把浏览器切成有头模式（WAF 等待路径只有有头才会走）。 */
async function runCheckinHeadful(page: FakePage) {
  const ctx = makeAdapterCtx(page);
  ctx.config.browser.headless = false;
  const outcome: AdapterOutcome = await agentrouterAdapter.checkin(ctx);
  return { outcome, ctx };
}

test("退出重登成功且余额增加 → success", async () => {
  const page = new FakePage();
  page
    .script(selfResult(100_000)) // 第一次 oauthLogin 复用会话读余额
    .script({ status: 200, json: { success: true } }) // logout 的 POST
    .script(undefined) // logout 的 localStorage 清理
    .script(selfResult(600_000)); // 重登后的 fetchSelf

  const { outcome } = await runCheckin(page);
  assert.equal(outcome.status, "success");
  assert.equal(outcome.reward, 1);
  assert.equal(outcome.balance, 1.2);
  assert.match(outcome.message, /重登领取 \+1\$/);
});

test("重登后余额未增 → already", async () => {
  const page = new FakePage();
  page
    .script(selfResult(100_000))
    .script({ status: 200, json: { success: true } })
    .script(undefined)
    .script(selfResult(100_000));

  const { outcome } = await runCheckin(page);
  assert.equal(outcome.status, "already");
  assert.match(outcome.message, /余额未增/);
});

test("重登后余额下降 → failed", async () => {
  const page = new FakePage();
  page
    .script(selfResult(600_000))
    .script({ status: 200, json: { success: true } })
    .script(undefined)
    .script(selfResult(100_000));

  const { outcome } = await runCheckin(page);
  assert.equal(outcome.status, "failed");
  assert.match(outcome.message, /余额异常下降 1\$/);
});

test("领取前余额就读不到（疑似 WAF）→ 跳过退出重登直接失败", async () => {
  const page = new FakePage();
  // 第一次 oauthLogin 复用会话但 quota=null；readKnownQuota 重试 3 次都 null
  page
    .script(selfResult(null))
    .script(selfResult(null))
    .script(selfResult(null))
    .script(selfResult(null));

  const { outcome, ctx } = await runCheckin(page);
  assert.equal(outcome.status, "failed");
  assert.match(outcome.message, /已跳过退出重登/);
  assert.equal(ctx.screenshots.at(-1), "agentrouter-balance-unknown");
  // 无头模式没有窗口可拖滑块：必须直接返回，不能打开验证页白等 4 分钟。
  assert.ok(
    !page.gotoCalls.some((u) => u.includes("/api/user/self")),
    "无头模式不得打开 WAF 验证页",
  );
});

test("重登未确认且之后余额仍读不到 → failed（不把未知伪装成 already）", async () => {
  const page = new FakePage();
  page
    .script(selfResult(100_000)) // 首次余额可读
    .script({ status: 200, json: { success: true } }) // logout
    .script(undefined) // logout 清理
    // forceLogin 的 oauthLogin：登录按钮不存在 → {user:null, needManual:false}
    .script(selfResult(null)) // 重登后的 fetchSelf
    .script(selfResult(null)) // readKnownQuota 重试 1
    .script(selfResult(null)) // 重试 2
    .script(selfResult(null)); // 重试 3

  const { outcome } = await runCheckin(page);
  assert.equal(outcome.status, "failed");
  assert.match(outcome.message, /WAF 人机验证/);
});

test("GitHub 会话失效（弹窗停在登录页）→ 提示手动登录", async () => {
  const page = new FakePage({
    locators: { "role:button:GitHub": { count: 1 } },
    popups: [new FakePage({ url: "https://github.com/login" })],
  });
  page.script(selfResult(100_000)); // 首次复用会话可读余额

  const { outcome } = await runCheckin(page);
  assert.equal(outcome.status, "failed");
  assert.match(outcome.message, /npm run login -- agentrouter/);
});

test("evaluate 抛异常 → 收敛为 failed", async () => {
  const page = new FakePage();
  page.script(() => {
    throw new Error("net down");
  });

  const { outcome } = await runCheckin(page);
  assert.equal(outcome.status, "failed");
  assert.match(outcome.message, /异常：net down/);
});

test("有头模式：领取前被 WAF 拦，等待中配额恢复 → 继续重登并 success", async () => {
  const page = new FakePage();
  page
    .script(selfResult(null)) // 首次 oauthLogin 复用会话但 quota=null
    .script(selfResult(null)) // readKnownQuota 重试 1
    .script(selfResult(null)) // 重试 2
    .script(selfResult(null)) // 重试 3
    .script(selfResult(500_000)) // WAF 等待第 1 次轮询恢复
    .script({ status: 200, json: { success: true } }) // logout
    .script(undefined) // logout 清理
    .script(selfResult(600_000)); // 重登后的 fetchSelf

  const { outcome, ctx } = await runCheckinHeadful(page);
  assert.equal(outcome.status, "success");
  assert.match(outcome.message, /重登领取/);
  assert.ok(page.gotoCalls.some((u) => u.includes("/api/user/self")), "有头模式必须先打开验证页");
  assert.ok(ctx.logs.some((l) => l.includes("WAF 已放行")), "恢复必须留痕");
});

test("有头模式：重登后等待中配额恢复 → success", async () => {
  const page = new FakePage();
  page
    .script(selfResult(100_000)) // 首次余额可读
    .script({ status: 200, json: { success: true } }) // logout
    .script(undefined) // logout 清理
    .script(selfResult(null)) // 重登后的 fetchSelf：仍被 WAF 拦
    .script(selfResult(null)) // readKnownQuota 重试 1
    .script(selfResult(null)) // 重试 2
    .script(selfResult(null)) // 重试 3
    .script(selfResult(200_000)); // WAF 等待中恢复

  const { outcome } = await runCheckinHeadful(page);
  assert.equal(outcome.status, "success");
  assert.match(outcome.message, /重登领取/);
});

test("有头模式：配额始终不恢复 → 4 分钟超时后 failed（不伪装成 already）", async () => {
  const page = new FakePage({
    // 队列耗尽后所有 fetchSelf 都命中 WAF 挑战页
    fetchRoutes: { "api/user/self": { status: 200, body: '<html><meta name="aliyun_waf_aa"></html>' } },
  });
  page
    .script(selfResult(100_000)) // 首次余额可读
    .script({ status: 200, json: { success: true } }) // logout
    .script(undefined) // logout 清理
    .script(selfResult(null)) // 重登后的 fetchSelf：被拦
    .script(selfResult(null)) // readKnownQuota 重试 1
    .script(selfResult(null)) // 重试 2
    .script(selfResult(null)); // 重试 3

  // 验证页渲染失败也不能中断等待（.catch 必须吞掉 goto 错误）；
  // 只让 WAF 验证页的 goto 失败，登录页的 goto 仍要正常走。
  const realGoto = page.goto.bind(page);
  page.goto = async (url: string, ...rest: unknown[]) => {
    if (url.includes("/api/user/self")) throw new Error("goto timeout");
    return realGoto(url, ...(rest as []));
  };

  // 测试内假时钟：每次读 Date.now 快进 5 秒，4 分钟超时不会真等 4 分钟。
  const realNow = Date.now;
  let fakeNow = realNow();
  Date.now = () => (fakeNow += 5_000);
  try {
    const { outcome, ctx } = await runCheckinHeadful(page);
    assert.equal(outcome.status, "failed");
    assert.match(outcome.message, /WAF 人机验证/);
    assert.ok(fakeNow - realNow() >= 4 * 60 * 1000 - 5_000, "必须真的走满等待窗口才超时");
    // 至少轮询到 8 次，才会触发「已等 N 分钟」的提示分支。
    assert.ok(ctx.logs.some((l) => l.includes("请继续拖动滑块")), "长时间等待必须提示用户");
  } finally {
    Date.now = realNow;
  }
});
