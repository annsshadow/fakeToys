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
