import type { Page } from "playwright";
import type { SiteCredentials } from "../types.js";

/**
 * new-api / one-api 系面板的通用客户端。
 *
 * 这三个公益站都是 new-api/one-api 的分支，共享接口约定：
 *   - GET  /api/user/self           读取当前用户信息（含 quota 余额）
 *   - POST /api/user/logout         退出登录
 *   - POST /api/user/check_in       部分分支的签到接口
 *
 * ⚠️ 登录必须走「真实登录页表单」而非纯接口：
 *   实测三个站的登录页都需要真实浏览器环境——JustWoker 有 Cloudflare
 *   Turnstile（需等页面 JS 算出 token），纯 page.request.post 会因
 *   "Turnstile token 为空" 被拒。因此统一用 domLogin() 填表登录。
 *
 * 所有 API 读取都用 page 内 fetch（同源、自动带 HttpOnly Cookie）。
 */

export interface NewApiUser {
  quota: number | null;
  username: string | null;
}

export interface LoginOptions {
  /** 登录页路径，默认 /login */
  loginPath?: string;
  /** 登录前需先点击的"展开表单"按钮文案（如 agentrouter 的"使用 邮箱或用户名 登录"） */
  expandButtonText?: string;
  /** 是否需要等待 Turnstile token（JustWoker=true） */
  waitTurnstile?: boolean;
}

/** 页面内 fetch 读取 JSON 接口（同源，自动带 Cookie） */
async function pageFetch(page: Page, path: string): Promise<{ status: number; json: any }> {
  return page.evaluate(async (p) => {
    try {
      const r = await fetch(p, { credentials: "include" });
      let json: any = null;
      try {
        json = await r.clone().json();
      } catch {
        json = null;
      }
      return { status: r.status, json };
    } catch (e) {
      return { status: 0, json: null };
    }
  }, path);
}

async function pagePost(page: Page, path: string): Promise<{ status: number; json: any }> {
  return page.evaluate(async (p) => {
    try {
      const r = await fetch(p, { method: "POST", credentials: "include" });
      let json: any = null;
      try {
        json = await r.clone().json();
      } catch {
        json = null;
      }
      return { status: r.status, json };
    } catch {
      return { status: 0, json: null };
    }
  }, path);
}

/**
 * 读取当前登录用户信息；未登录返回 null。
 * 兼容两类 new-api 分支：Cookie 会话，以及 localStorage token + 请求头鉴权（如 JustWoker）。
 */
export async function fetchSelf(page: Page, baseUrl: string): Promise<NewApiUser | null> {
  // 注意：page.evaluate 回调内禁止具名嵌套函数（tsx/esbuild 会注入浏览器端不存在的 __name）
  const result = await page.evaluate(async () => {
    let token = "";
    let userId = "";
    try {
      for (const k of Object.keys(localStorage)) {
        const raw = localStorage.getItem(k) || "";
        if (!token) {
          const m = raw.match(/"access_token"\s*:\s*"([^"]+)"/);
          if (m) token = m[1];
        }
        if (!userId) {
          const mi = raw.match(/"id"\s*:\s*(\d+)/);
          if (mi) userId = mi[1];
        }
      }
    } catch {}
    const headers: Record<string, string> = {};
    if (token) headers["Authorization"] = "Bearer " + token;
    if (userId) headers["New-Api-User"] = userId;

    // 优先接口（Cookie/token 会话）
    try {
      const r = await fetch("/api/user/self", { credentials: "include", headers });
      const t = await r.text();
      if (t.trim().startsWith("{")) {
        const j = JSON.parse(t);
        if (j && j.success === true && j.data) {
          return {
            quota: typeof j.data.quota === "number" ? j.data.quota : null,
            username: typeof j.data.username === "string" ? j.data.username : (typeof j.data.display_name === "string" ? j.data.display_name : null),
          };
        }
      }
    } catch {}

    // token 刷新型分支（如 JustWoker）：先用 refresh cookie 换 access_token，再带头重试
    try {
      const rr = await fetch("/api/user/auth/refresh", { method: "POST", credentials: "include" });
      const rt = await rr.text();
      if (rt.trim().startsWith("{")) {
        const rj = JSON.parse(rt);
        const at = rj?.data?.access_token;
        if (at) {
          const r2 = await fetch("/api/user/self", {
            credentials: "include",
            headers: { Authorization: "Bearer " + at },
          });
          const t2 = await r2.text();
          if (t2.trim().startsWith("{")) {
            const j2 = JSON.parse(t2);
            if (j2 && j2.success === true && j2.data) {
              return {
                quota: typeof j2.data.quota === "number" ? j2.data.quota : null,
                username: typeof j2.data.username === "string" ? j2.data.username : (typeof j2.data.display_name === "string" ? j2.data.display_name : null),
              };
            }
          }
        }
      }
    } catch {}

    // 兜底：直接从 localStorage 的 user 对象读（绕开 WAF/token 接口，适配 AgentRouter 等）
    try {
      for (const k of Object.keys(localStorage)) {
        const raw = localStorage.getItem(k) || "";
        if (raw.trim().startsWith("{") && raw.indexOf('"username"') >= 0 && raw.indexOf('"quota"') >= 0) {
          const obj = JSON.parse(raw);
          const u = obj && typeof obj.username === "string" ? obj : obj.user || obj.data || obj;
          if (u && typeof u.username === "string") {
            return { quota: typeof u.quota === "number" ? u.quota : null, username: u.username };
          }
        }
      }
    } catch {}
    return null;
  });
  if (!result || !result.username) return null;
  return { quota: result.quota, username: result.username };
}

/** 退出登录 */
export async function logout(page: Page): Promise<void> {
  await pagePost(page, "/api/user/logout").catch(() => {});
}

/** 尝试调用签到接口。返回 null 表示接口不存在（404） */
export async function apiCheckIn(
  page: Page,
): Promise<{ ok: boolean; message: string } | null> {
  const { status, json } = await pagePost(page, "/api/user/check_in");
  if (status === 404) return null;
  return { ok: json?.success === true, message: json?.message ?? `HTTP ${status}` };
}

/** 关闭 new-api 面板常见的公告弹窗 */
async function dismissModals(page: Page): Promise<void> {
  for (const name of ["今日不再显示", "关闭", "Close", "我知道了", "确定"]) {
    await page
      .getByRole("button", { name, exact: false })
      .first()
      .click({ timeout: 1500 })
      .catch(() => {});
  }
  await page.waitForTimeout(400);
}

/**
 * 真实浏览器 DOM 登录。返回登录后的用户信息（失败为 null）。
 * 已在持久化上下文中登录过则直接复用会话。
 */
export async function domLogin(
  page: Page,
  baseUrl: string,
  creds: SiteCredentials,
  log: (m: string) => void,
  opts: LoginOptions = {},
): Promise<NewApiUser | null> {
  // 先探测是否已是登录态（复用持久化 Cookie）
  await page.goto(baseUrl, { waitUntil: "domcontentloaded" }).catch(() => {});
  await page.waitForTimeout(1500);
  let self = await fetchSelf(page, baseUrl);
  if (self) {
    log("已是登录态（复用会话）");
    return self;
  }

  // 打开登录页
  const loginUrl = `${baseUrl}${opts.loginPath ?? "/login"}`;
  await page.goto(loginUrl, { waitUntil: "domcontentloaded" });
  await page.waitForTimeout(3000);
  await dismissModals(page);

  // 部分站点需先点"展开账密表单"
  if (opts.expandButtonText) {
    await page
      .getByRole("button", { name: opts.expandButtonText, exact: false })
      .first()
      .click({ timeout: 3000 })
      .catch(() => {});
    await page.waitForTimeout(1500);
  }

  // 填密码：type=password 最可靠，跨站通用
  const filledPass = await fillFirst(page, ['input[type="password"]'], creds.password);
  // 填用户名：先试常见 name/type，再试 placeholder，最后兜底"第一个非密码可见输入框"
  let filledUser = await fillFirst(
    page,
    ['input[name="username"]', 'input[type="email"]', 'input[name="email"]'],
    creds.email,
  );
  if (!filledUser) filledUser = await fillByPlaceholder(page, creds.email);
  if (!filledUser) filledUser = await fillFirstTextInput(page, creds.email);
  if (!filledUser || !filledPass) {
    log(`未找到登录表单输入框（user=${filledUser} pass=${filledPass}）`);
    return null;
  }

  // 等待 Turnstile token（若该站有）
  if (opts.waitTurnstile) {
    log("等待 Cloudflare Turnstile token…");
    const ok = await waitTurnstileToken(page, 15000);
    log(ok ? "Turnstile token 就绪" : "Turnstile token 超时（仍尝试提交）");
  }

  // 点击登录按钮
  await clickLogin(page);
  await page.waitForTimeout(4500);

  self = await fetchSelf(page, baseUrl);
  if (self) log(`登录成功${self.username ? `（${self.username}）` : ""}`);
  else log("登录后仍未获取到用户信息（凭据或验证码可能未通过）");
  return self;
}

async function fillFirst(page: Page, selectors: string[], value: string): Promise<boolean> {
  for (const sel of selectors) {
    const loc = page.locator(sel).first();
    if ((await loc.count()) > 0) {
      const ok = await loc.fill(value, { timeout: 3000 }).then(() => true).catch(() => false);
      if (ok) return true;
    }
  }
  return false;
}

/** 按常见 placeholder 文案填用户名/邮箱输入框 */
async function fillByPlaceholder(page: Page, value: string): Promise<boolean> {
  const phs = [/you@example\.com/i, /email/i, /用户名|电子邮件|邮箱/, /username/i];
  for (const ph of phs) {
    const loc = page.getByPlaceholder(ph).first();
    if ((await loc.count()) > 0) {
      const ok = await loc.fill(value, { timeout: 3000 }).then(() => true).catch(() => false);
      if (ok) return true;
    }
  }
  return false;
}

/** 兜底：填第一个可见的、非密码、非隐藏的文本输入框 */
async function fillFirstTextInput(page: Page, value: string): Promise<boolean> {
  const loc = page
    .locator('input:not([type="password"]):not([type="hidden"]):not([type="checkbox"]):not([type="submit"])')
    .first();
  if ((await loc.count()) === 0) return false;
  return loc.fill(value, { timeout: 3000 }).then(() => true).catch(() => false);
}

/** 轮询等待隐藏字段 cf-turnstile-response 出现非空 token */
async function waitTurnstileToken(page: Page, timeoutMs: number): Promise<boolean> {
  const deadline = Date.now() + timeoutMs;
  while (Date.now() < deadline) {
    const len = await page.evaluate(() => {
      const t = document.querySelector('input[name="cf-turnstile-response"]') as HTMLInputElement | null;
      return t ? t.value.length : -1;
    });
    if (len > 20) return true;
    await page.waitForTimeout(800);
  }
  return false;
}

async function clickLogin(page: Page): Promise<void> {
  // 优先精确"登录"/"Login"按钮
  for (const name of ["登录", "Login", "登 录", "Sign in"]) {
    const btn = page.getByRole("button", { name, exact: true }).last();
    if ((await btn.count()) > 0) {
      const ok = await btn.click({ timeout: 3000 }).then(() => true).catch(() => false);
      if (ok) return;
    }
  }
  // 兜底：提交型按钮
  await page.locator('button[type="submit"]').last().click({ timeout: 3000 }).catch(() => {});
}

/** 访问受保护页，若未被重定向回登录页则视为会话有效（适配 token 类分支） */
export async function isSessionAlive(page: Page, baseUrl: string): Promise<boolean> {
  for (const path of ["/panel/personal", "/console", "/dashboard", "/profile", "/panel"]) {
    try {
      await page.goto(`${baseUrl}${path}`, { waitUntil: "domcontentloaded" });
      await page.waitForTimeout(2000);
      const url = page.url();
      if (!/\/login|\/sign-?in|\/register|\/oauth/i.test(url) && url.includes(new URL(baseUrl).hostname)) {
        return true;
      }
    } catch {
      // 该路径不存在，试下一个
    }
  }
  return false;
}

/** new-api 的 quota 通常 500000 = $1 */
export function quotaToDisplay(quota: number | null): { value: number | null; currency: string } {
  if (quota === null) return { value: null, currency: "quota" };
  return { value: Number((quota / 500000).toFixed(4)), currency: "$" };
}

/**
 * GitHub OAuth 登录（复用持久化会话里的 GitHub 登录态）。
 *
 * 前提：该站的持久化会话中已登录过 GitHub（通过 `npm run login -- <站点>`
 * 一次性手动完成）。之后本函数点「使用 GitHub 继续」→ GitHub 自动授权跳回
 * → 完成站点登录，全程无需再输 GitHub 密码。
 *
 * 返回登录后的用户信息；若 GitHub 会话失效（需重新手动登录）返回 { needManual: true }。
 */
export type OAuthProvider = "github" | "linuxdo";

const PROVIDER_META: Record<OAuthProvider, { label: RegExp; authHost: RegExp; needLogin: RegExp; authorize: RegExp }> = {
  github: {
    label: /GitHub/i,
    authHost: /github\.com/i,
    needLogin: /github\.com\/(login|session)/i,
    authorize: /Authorize/i,
  },
  linuxdo: {
    label: /LinuxDO|Linux ?Do/i,
    authHost: /linux\.do|connect\.linux\.do/i,
    needLogin: /linux\.do\/(login|session|sign)/i,
    authorize: /授权|同意|Authorize|Approve|允许/i,
  },
};

export async function oauthLogin(
  page: Page,
  baseUrl: string,
  provider: OAuthProvider,
  log: (m: string) => void,
  loginPath = "/login",
): Promise<{ user: NewApiUser | null; needManual: boolean }> {
  const meta = PROVIDER_META[provider];
  // 已登录则直接复用
  await page.goto(baseUrl, { waitUntil: "domcontentloaded" }).catch(() => {});
  await page.waitForTimeout(1500);
  let self = await fetchSelf(page, baseUrl);
  if (self) {
    log("已是登录态（复用会话）");
    return { user: self, needManual: false };
  }
  // 兜底：token 类分支（如 JustWoker）fetchSelf 读不到，用「访问受保护页是否被踢回登录页」判断
  const reused = await isSessionAlive(page, baseUrl);
  if (reused) {
    log("已是登录态（会话复用，接口读取受限）");
    return { user: { quota: null, username: null }, needManual: false };
  }

  // 打开登录页，点第三方登录按钮
  await page.goto(`${baseUrl}${loginPath}`, { waitUntil: "domcontentloaded" });
  await page.waitForTimeout(2500);
  await dismissModals(page);

  const btnByRole = page.getByRole("button", { name: meta.label }).first();
  const linkByRole = page.getByRole("link", { name: meta.label }).first();
  const btn = (await btnByRole.count()) > 0 ? btnByRole : linkByRole;
  if ((await btn.count()) === 0) {
    log(`未找到 ${provider} 登录按钮`);
    return { user: null, needManual: false };
  }
  log(`点击「使用 ${provider} 继续」，等待授权跳转…`);
  await btn.click().catch(() => {});
  await page.waitForTimeout(5000);

  const host = new URL(baseUrl).hostname;
  // 落在第三方授权页则点授权
  if (meta.authHost.test(page.url())) {
    const auth = page.getByRole("button", { name: meta.authorize }).first();
    if ((await auth.count()) > 0) {
      log(`点击 ${provider} 授权…`);
      await auth.click().catch(() => {});
      await page.waitForTimeout(5000);
    }
  }
  // 仍停在第三方登录页 = 未登录该平台，需手动
  if (meta.needLogin.test(page.url())) {
    log(`${provider} 未登录，需先运行：npm run login -- <站点>`);
    return { user: null, needManual: true };
  }
  // 等待跳回站点
  await page.waitForURL((u) => u.hostname.includes(host), { timeout: 15000 }).catch(() => {});
  await page.goto(baseUrl, { waitUntil: "domcontentloaded" }).catch(() => {});
  await page.waitForTimeout(2000);
  self = await fetchSelf(page, baseUrl);
  if (self) log(`登录成功${self.username ? `（${self.username}）` : ""}`);
  else log("OAuth 后仍未获取到用户信息");
  return { user: self, needManual: false };
}
