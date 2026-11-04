// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import assert from "node:assert/strict";
import { test } from "node:test";
import type { Page } from "playwright";
import {
  apiCheckIn,
  domLogin,
  fetchSelf,
  gotoWithNavigationRetry,
  isSessionAlive,
  isWafChallengeBody,
  logout,
  oauthLogin,
  quotaToDisplay,
} from "../src/sites/newapi-client.js";
import { FakePage, newApiSelf } from "./fake-page.js";

test("retries a navigation interrupted by the SPA", async () => {
  let gotoCalls = 0;
  let waitCalls = 0;
  const page = {
    goto: async () => {
      gotoCalls++;
      if (gotoCalls === 1) throw new Error("Navigation is interrupted by another navigation");
    },
    waitForLoadState: async () => {},
    waitForTimeout: async () => {
      waitCalls++;
    },
  } as unknown as Page;

  await gotoWithNavigationRetry(page, "https://example.test/login");

  assert.equal(gotoCalls, 2);
  assert.equal(waitCalls, 1);
});

test("allows two transient SPA navigation races before succeeding", async () => {
  let gotoCalls = 0;
  const page = {
    goto: async () => {
      gotoCalls++;
      if (gotoCalls < 3) throw new Error("Navigation is interrupted by another navigation");
    },
    waitForLoadState: async () => {},
    waitForTimeout: async () => {},
  } as unknown as Page;

  await gotoWithNavigationRetry(page, "https://example.test/login");

  assert.equal(gotoCalls, 3);
});

test("does not hide an unrelated navigation error", async () => {
  let gotoCalls = 0;
  const page = {
    goto: async () => {
      gotoCalls++;
      throw new Error("connection reset");
    },
    waitForLoadState: async () => {},
    waitForTimeout: async () => {},
  } as unknown as Page;

  await assert.rejects(
    () => gotoWithNavigationRetry(page, "https://example.test/login"),
    /connection reset/,
  );
  assert.equal(gotoCalls, 1);
});

// ─────────────────────────────────────────────────────────────────────────
// 沙箱执行层：page.evaluate 的浏览器端回调在 Node 里真实执行
// （fetch/localStorage/document 由 FakePage 注入），把此前 0% 的回调体
// 一并纳入覆盖。
// ─────────────────────────────────────────────────────────────────────────

const logs: string[] = [];
const log = (m: string) => logs.push(m);
function resetLogs() {
  logs.length = 0;
}

test("fetchSelf：带 localStorage token 时以 Bearer 头调 /api/user/self", async () => {
  const page = new FakePage({
    localStorage: { user: JSON.stringify({ access_token: "tok-1", id: 5 }) },
    fetchRoutes: { "api/user/self": newApiSelf(250_000, "alice") },
  });
  const user = await fetchSelf(page as unknown as Page, "https://site.test");
  assert.deepEqual(user, { quota: 250_000, username: "alice" });
  const self = page.fetchCalls.find((c) => c.url.includes("/api/user/self"));
  const headers = self?.init?.headers as Record<string, string>;
  assert.equal(headers["Authorization"], "Bearer tok-1");
  assert.equal(headers["New-Api-User"], "5");
});

test("fetchSelf：识别阿里云 WAF 挑战页并回报日志", async () => {
  resetLogs();
  const page = new FakePage({
    fetchRoutes: {
      "api/user/self": { status: 200, body: '<html><meta name="aliyun_waf_aa"></html>' },
      "api/user/auth/refresh": { status: 200, body: '<html><meta name="aliyun_waf_bb"></html>' },
    },
  });
  const user = await fetchSelf(page as unknown as Page, "https://site.test", log);
  assert.equal(user, null);
  assert.ok(logs.some((l) => l.includes("WAF 挑战页")), "WAF 拦截必须显式留痕");
});

test("fetchSelf：主接口不可用时走 refresh 换 token 重试", async () => {
  const page = new FakePage({
    fetchRoutes: {},
  });
  const routes = (page as unknown as { opts: { fetchRoutes: Record<string, unknown> } }).opts.fetchRoutes;
  routes["api/user/auth/refresh"] = { status: 200, body: JSON.stringify({ data: { access_token: "at-2" } }) };
  routes["api/user/self"] = ((_url: string, init?: RequestInit): { status: number; body: string } => {
    const h = (init?.headers ?? {}) as Record<string, string>;
    return h["Authorization"] === "Bearer at-2"
      ? newApiSelf(300_000, "bob")
      : { status: 404, body: "not found" };
  }) as never;

  const user = await fetchSelf(page as unknown as Page, "https://site.test");
  assert.deepEqual(user, { quota: 300_000, username: "bob" });
  const selfCalls = page.fetchCalls.filter((c) => c.url.includes("/api/user/self"));
  assert.equal(selfCalls.length, 2, "主接口失败后应重试一次");
  // 重试必须带 refresh 拿到的 token；不带就只是把同一个失败请求又发一遍。
  const headers = selfCalls[1]?.init?.headers as Record<string, string>;
  assert.equal(headers["Authorization"], "Bearer at-2");
});

test("fetchSelf：refresh 非 200 时不解析错误体、不重试", async () => {
  const page = new FakePage({
    fetchRoutes: {
      "api/user/self": { status: 404, body: "not found" },
      // 错误响应体故意伪装成合法 token 响应：只看 body 不看状态码的实现会误重试。
      "api/user/auth/refresh": {
        status: 404,
        body: JSON.stringify({ data: { access_token: "leaked-404-token" } }),
      },
    },
  });
  const user = await fetchSelf(page as unknown as Page, "https://site.test");
  assert.equal(user, null);
  const selfCalls = page.fetchCalls.filter((c) => c.url.includes("/api/user/self"));
  assert.equal(selfCalls.length, 1, "refresh 返回 404 时不得用错误体重试");
  assert.ok(
    !selfCalls.some((c) => (c.init?.headers as Record<string, string> | undefined)?.Authorization?.includes("leaked-404-token")),
    "错误响应体里的 token 不得被采用",
  );
});

test("fetchSelf：refresh 200 但不是 JSON 时不重试", async () => {
  const page = new FakePage({
    fetchRoutes: {
      "api/user/self": { status: 500, body: "oops" },
      "api/user/auth/refresh": { status: 200, body: "<html>maintenance</html>" },
    },
  });
  assert.equal(await fetchSelf(page as unknown as Page, "https://site.test"), null);
  const selfCalls = page.fetchCalls.filter((c) => c.url.includes("/api/user/self"));
  assert.equal(selfCalls.length, 1, "refresh 未返回 JSON 时不得重试");
});

test("fetchSelf：refresh 重试仍被 WAF 拦 → 兜底只给用户名，quota 必须为 null", async () => {
  resetLogs();
  const page = new FakePage({
    localStorage: { user: JSON.stringify({ username: "dave", quota: 123 }) },
    fetchRoutes: {
      "api/user/self": { status: 200, body: '<html><meta name="aliyun_waf_aa"></html>' },
      "api/user/auth/refresh": { status: 200, body: JSON.stringify({ data: { access_token: "at-3" } }) },
    },
  });
  const user = await fetchSelf(page as unknown as Page, "https://site.test", log);
  // SPA 残留的 quota 不可信，拿来算差值会误报奖励
  assert.deepEqual(user, { quota: null, username: "dave" });
  assert.ok(logs.some((l) => l.includes("WAF 挑战页")), "重试被拦也必须留痕");
});

test("fetchSelf：接口全挂时从 localStorage 兜底读用户名（quota 不可信）", async () => {
  const page = new FakePage({
    localStorage: { user: JSON.stringify({ username: "carol", quota: 999 }) },
    fetchRoutes: { "api/user/self": { status: 500, body: "oops" } },
  });
  const user = await fetchSelf(page as unknown as Page, "https://site.test");
  // 兜底分支刻意不给 quota：SPA 残留值可能为 0，拿来算差值会误报奖励
  assert.deepEqual(user, { quota: null, username: "carol" });
});

test("fetchSelf：display_name 兜底与彻底无用户", async () => {
  const page = new FakePage({
    fetchRoutes: {
      "api/user/self": {
        status: 200,
        body: JSON.stringify({ success: true, data: { quota: "not-a-number", display_name: "戴维" } }),
      },
    },
  });
  assert.deepEqual(await fetchSelf(page as unknown as Page, "https://site.test"), { quota: null, username: "戴维" });

  const empty = new FakePage();
  assert.equal(await fetchSelf(empty as unknown as Page, "https://site.test"), null);
});

test("apiCheckIn：404 返回 null（站点无签到接口），其余按 success/message", async () => {
  const missing = new FakePage({
    fetchRoutes: { "api/user/check_in": { status: 404, body: "" } },
  });
  assert.equal(await apiCheckIn(missing as unknown as Page), null);

  const ok = new FakePage({
    fetchRoutes: { "api/user/check_in": { status: 200, body: JSON.stringify({ success: true, message: "已领取" }) } },
  });
  assert.deepEqual(await apiCheckIn(ok as unknown as Page), { ok: true, message: "已领取" });

  const err = new FakePage({
    fetchRoutes: { "api/user/check_in": { status: 500, body: "oops" } },
  });
  assert.deepEqual(await apiCheckIn(err as unknown as Page), { ok: false, message: "HTTP 500" });
});

test("quotaToDisplay：500000 quota = $1，null 走 quota 单位", () => {
  assert.deepEqual(quotaToDisplay(500_000), { value: 1, currency: "$" });
  assert.deepEqual(quotaToDisplay(123_456), { value: 0.2469, currency: "$" });
  assert.deepEqual(quotaToDisplay(null), { value: null, currency: "quota" });
});

test("isWafChallengeBody 只认阿里云挑战页特征", () => {
  assert.equal(isWafChallengeBody('<meta name="aliyun_waf_aa">'), true);
  assert.equal(isWafChallengeBody("plain json"), false);
});

test("logout：清本站 Cookie 但保留 WAF/CDN 基础设施 cookie 与第三方 cookie", async () => {
  const cookies = [
    { name: "session", domain: "site.test" },
    { name: "acw_tc", domain: "site.test" },
    { name: "cf_clearance", domain: "site.test" },
    { name: "third_party", domain: "other.example" },
  ];
  const page = new FakePage({
    fetchRoutes: { "api/user/logout": { status: 200, body: JSON.stringify({ success: true }) } },
    cookies,
  } as never);
  await logout(page as unknown as Page, "https://site.test");
  const left = cookies.map((c) => c.name).sort();
  // session 清掉；acw_tc / cf_clearance 是 WAF 放行凭证必须保留；第三方不动
  assert.deepEqual(left, ["acw_tc", "cf_clearance", "third_party"]);
});

test("isSessionAlive：任一受保护页未被踢回登录页即视为存活", async () => {
  const alive = new FakePage({ url: "https://site.test/console" });
  assert.equal(await isSessionAlive(alive as unknown as Page, "https://site.test"), true);

  const dead = new FakePage({ url: "https://site.test/login" });
  assert.equal(await isSessionAlive(dead as unknown as Page, "https://site.test"), false);
});

test("domLogin：已登录直接复用会话，不再走表单", async () => {
  resetLogs();
  const page = new FakePage({
    fetchRoutes: { "api/user/self": newApiSelf(1, "reuse") },
  });
  const user = await domLogin(page as unknown as Page, "https://site.test", { email: "e", password: "p" }, log);
  assert.deepEqual(user, { quota: 1, username: "reuse" });
  assert.ok(logs.at(-1)?.includes("已是登录态"));
  assert.equal(page.gotoCalls.filter((u) => u.includes("/login")).length, 0);
});

test("domLogin：填表提交成功后返回用户", async () => {
  resetLogs();
  let submitted = false;
  const page = new FakePage({
    locators: {
      'css:input[type="password"]': { count: 1 },
      'css:input[name="username"]': { count: 1 },
      "role:button:登录": { count: 1 },
    },
    fetchRoutes: {},
  });
  const routes = (page as unknown as { opts: { fetchRoutes: Record<string, unknown> } }).opts.fetchRoutes;
  routes["api/user/self"] = (): { status: number; body: string } =>
    submitted ? newApiSelf(2, "form-user") : { status: 500, body: "nope" };
  // waitForTimeout 出现 4 次：初始探测 1 次 + 登录页等待 + 弹窗清理 + 提交后等待；
  // 只有第 4 次（提交后）才把登录态打开，否则首轮 fetchSelf 就复用会话、表单路径测不到
  let waits = 0;
  page.waitForTimeout = async () => {
    if (++waits >= 4) submitted = true;
  };

  const user = await domLogin(page as unknown as Page, "https://site.test", { email: "e", password: "p" }, log);
  assert.deepEqual(user, { quota: 2, username: "form-user" });
  assert.ok(logs.some((l) => l.includes("登录成功")));
  assert.equal(page.fills.length, 2, "密码与用户名各填一次");
});

test("domLogin：找不到表单输入框 → 返回 null 并留痕", async () => {
  resetLogs();
  const page = new FakePage();
  const user = await domLogin(page as unknown as Page, "https://site.test", { email: "e", password: "p" }, log);
  assert.equal(user, null);
  assert.ok(logs.some((l) => l.includes("未找到登录表单输入框")));
});

test("domLogin：先点「展开账密表单」，并等待 Turnstile token 就绪", async () => {
  resetLogs();
  let submitted = false;
  const page = new FakePage({
    locators: {
      'css:input[type="password"]': { count: 1 },
      'css:input[name="username"]': { count: 1 },
      "role:button:登录": { count: 1 },
      "role:button:使用 邮箱或用户名 登录": { count: 1 },
    },
    fetchRoutes: {},
  });
  const routes = (page as unknown as { opts: { fetchRoutes: Record<string, unknown> } }).opts.fetchRoutes;
  routes["api/user/self"] = (): { status: number; body: string } =>
    submitted ? newApiSelf(3, "expand-user") : { status: 500, body: "nope" };
  // 等待点：探测 1 + 登录页 2 + 关弹窗 3 + 展开表单 4 + 提交后 5
  let waits = 0;
  page.waitForTimeout = async () => {
    if (++waits >= 5) submitted = true;
  };

  const user = await domLogin(page as unknown as Page, "https://site.test", { email: "e", password: "p" }, log, {
    expandButtonText: "使用 邮箱或用户名 登录",
    waitTurnstile: true,
  });
  assert.deepEqual(user, { quota: 3, username: "expand-user" });
  // 不点展开按钮就没有账密表单可填——这条断言钉住「必须先展开」的顺序。
  assert.ok(
    page.clicks.some((c) => c.includes("使用 邮箱或用户名 登录")),
    "必须点击展开账密表单按钮",
  );
  assert.ok(logs.some((l) => l.includes("Turnstile token 就绪")), "token 就绪必须留痕");
});

test("domLogin：Turnstile 超时也不阻断登录（仍尝试提交）", async () => {
  resetLogs();
  let submitted = false;
  const page = new FakePage({
    // 挑战框始终没有算出 token
    turnstileToken: "",
    locators: {
      'css:input[type="password"]': { count: 1 },
      'css:input[name="username"]': { count: 1 },
      "role:button:登录": { count: 1 },
    },
    fetchRoutes: {},
  });
  const routes = (page as unknown as { opts: { fetchRoutes: Record<string, unknown> } }).opts.fetchRoutes;
  routes["api/user/self"] = (): { status: number; body: string } =>
    submitted ? newApiSelf(4, "turnstile-timeout") : { status: 500, body: "nope" };
  let waits = 0;
  page.waitForTimeout = async () => {
    if (++waits >= 4) submitted = true;
  };
  // 测试内假时钟：每次读 Date.now 快进 5 秒，15 秒轮询不会真等。
  const realNow = Date.now;
  let fakeNow = realNow();
  Date.now = () => (fakeNow += 5_000);
  try {
    const user = await domLogin(page as unknown as Page, "https://site.test", { email: "e", password: "p" }, log, {
      waitTurnstile: true,
    });
    // 超时只降级为日志，不能把整次登录判死。
    assert.deepEqual(user, { quota: 4, username: "turnstile-timeout" });
    assert.ok(logs.some((l) => l.includes("Turnstile token 超时（仍尝试提交）")));
  } finally {
    Date.now = realNow;
  }
});

test("domLogin：用户名只靠 placeholder 能找到时走 placeholder 路径", async () => {
  resetLogs();
  let submitted = false;
  const page = new FakePage({
    locators: {
      'css:input[type="password"]': { count: 1 },
      "ph:you@example": { count: 1 },
      "role:button:登录": { count: 1 },
    },
    fetchRoutes: {},
  });
  const routes = (page as unknown as { opts: { fetchRoutes: Record<string, unknown> } }).opts.fetchRoutes;
  routes["api/user/self"] = (): { status: number; body: string } =>
    submitted ? newApiSelf(5, "ph-user") : { status: 500, body: "nope" };
  let waits = 0;
  page.waitForTimeout = async () => {
    if (++waits >= 4) submitted = true;
  };

  const user = await domLogin(page as unknown as Page, "https://site.test", { email: "ph@test", password: "p" }, log);
  assert.deepEqual(user, { quota: 5, username: "ph-user" });
  // 填到的定位器若换成别的（比如兜底文本框），说明 placeholder 路径没生效。
  assert.ok(page.fills.some((f) => f.desc.startsWith("ph:") && f.value === "ph@test"));
});

test("domLogin：没有任何具名/placeholder 输入框时兜底填第一个可见文本框", async () => {
  resetLogs();
  let submitted = false;
  const textInput =
    'css:input:not([type="password"]):not([type="hidden"]):not([type="checkbox"]):not([type="submit"])';
  const page = new FakePage({
    locators: {
      'css:input[type="password"]': { count: 1 },
      [textInput]: { count: 1 },
      "role:button:登录": { count: 1 },
    },
    fetchRoutes: {},
  });
  const routes = (page as unknown as { opts: { fetchRoutes: Record<string, unknown> } }).opts.fetchRoutes;
  routes["api/user/self"] = (): { status: number; body: string } =>
    submitted ? newApiSelf(6, "fallback-user") : { status: 500, body: "nope" };
  let waits = 0;
  page.waitForTimeout = async () => {
    if (++waits >= 4) submitted = true;
  };

  const user = await domLogin(page as unknown as Page, "https://site.test", { email: "fb@test", password: "p" }, log);
  assert.deepEqual(user, { quota: 6, username: "fallback-user" });
  assert.ok(page.fills.some((f) => f.desc === textInput && f.value === "fb@test"));
});

test("oauthLogin：无登录按钮 → 返回无用户且不需手动", async () => {
  resetLogs();
  const page = new FakePage({ url: "https://site.test/login" });
  page.script({ user: null, wafBlocked: false }); // 复用探测失败
  const r = await oauthLogin(page as unknown as Page, "https://site.test", "linuxdo", log);
  assert.deepEqual(r, { user: null, needManual: false });
  assert.ok(logs.some((l) => l.includes("未找到 linuxdo 登录按钮")));
});

test("oauthLogin：GitHub 会话停在授权登录页 → needManual", async () => {
  const page = new FakePage({
    url: "https://site.test/login",
    locators: { "role:button:GitHub": { count: 1 } },
    popups: [new FakePage({ url: "https://github.com/login" })],
  });
  page.script({ user: null, wafBlocked: false });
  const r = await oauthLogin(page as unknown as Page, "https://site.test", "github", log);
  assert.deepEqual(r, { user: null, needManual: true });
});

test("oauthLogin：授权弹窗完成跳回 → 不谎报成功也不需手动", async () => {
  const popup = new FakePage({
    url: "https://github.com/authorize",
    locators: { "role:button:Authorize": { count: 1 } },
  });
  const page = new FakePage({
    url: "https://site.test/login",
    locators: { "role:button:GitHub": { count: 1 } },
    popups: [popup],
  });
  page.script({ user: null, wafBlocked: false }); // 复用探测失败
  // 弹窗 waitForURL 等跳回站点：OAuth 授权页 URL 不满足谓词 → 立即超时（真实语义）
  // 之后回原页面读 self——沙箱 fetch 无路由 → null，不得谎报成功
  const r = await oauthLogin(page as unknown as Page, "https://site.test", "github", log);
  assert.equal(r.needManual, false);
  assert.equal(r.user, null);
  assert.ok(logs.some((l) => l.includes("点击 github 授权")), "授权按钮存在时应点击");
});
