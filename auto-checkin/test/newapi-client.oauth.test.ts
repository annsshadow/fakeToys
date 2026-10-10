// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import assert from "node:assert/strict";
import { test } from "node:test";
import type { Page } from "playwright";
import { adoptOAuthPopup, isWafChallengeBody } from "../src/sites/newapi-client.js";

test("recognizes an Aliyun WAF challenge page served with HTTP 200", () => {
  // AgentRouter 的 /api/user/self 会这样返回：状态码 200，但不是 JSON。
  const challenge = '<!doctype html><meta name="aliyun_waf_aa" content="ff926c7f"><title></title>';
  assert.equal(isWafChallengeBody(challenge), true);
});

test("does not mistake a normal API payload for a WAF challenge", () => {
  assert.equal(isWafChallengeBody('{"success":true,"data":{"quota":1}}'), false);
  assert.equal(isWafChallengeBody(""), false);
});

test("adopts the OAuth popup so a stalled provider login is visible", async () => {
  // 真实行为：点「使用 GitHub 继续」后 window.open 弹窗，原页面 URL 不动。
  // 若不接管弹窗，就会把"卡在 GitHub 登录页"误判成流程走完。
  const popup = {
    url: () => "https://github.com/login?client_id=x",
    isClosed: () => false,
    waitForLoadState: async () => {},
  } as unknown as Page;
  const page = {
    url: () => "https://agentrouter.org/login",
    isClosed: () => false,
    waitForTimeout: async () => {},
    context: () => ({ pages: () => [page, popup] }),
  } as unknown as Page;

  const logs: string[] = [];
  const target = await adoptOAuthPopup(
    page,
    new Set([page]),
    "https://agentrouter.org/login",
    5000,
    (m) => logs.push(m),
  );

  assert.equal(target, popup);
  assert.ok(logs.some((m) => m.includes("弹窗")));
});

test("keeps following the original page for same-tab OAuth", async () => {
  const page = {
    url: () => "https://agentrouter.org/login",
    isClosed: () => false,
    waitForTimeout: async () => {},
    context: () => ({ pages: () => [page] }),
  } as unknown as Page;

  const target = await adoptOAuthPopup(
    page,
    new Set([page]),
    "https://agentrouter.org/login",
    300,
    () => {},
  );

  assert.equal(target, page);
});

test("stops waiting once the original page moves on to the provider", async () => {
  // 同页跳转的站点：主页面已跳走就不必再等弹窗。
  // 关键是 GitHub 授权页路径里也含 "login"，不能用正则判断。
  const page = {
    url: () => "https://github.com/login/oauth/authorize?client_id=x",
    isClosed: () => false,
    waitForTimeout: async () => {},
    context: () => ({ pages: () => [page] }),
  } as unknown as Page;

  const started = Date.now();
  const target = await adoptOAuthPopup(
    page,
    new Set([page]),
    "https://agentrouter.org/login",
    10_000,
    () => {},
  );

  assert.equal(target, page);
  assert.ok(Date.now() - started < 2000, "应立刻返回，而不是等满超时");
});