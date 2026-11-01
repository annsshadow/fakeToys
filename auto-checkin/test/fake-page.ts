// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/**
 * 站点适配器的假浏览器层：不需要真实网络/Chromium 就能把 checkin 全流程跑起来。
 *
 * 两层能力：
 *  1. **脚本队列**：`page.evaluate` 按入队顺序弹出预设结果——适合直接钉住
 *     「适配器拿到某结果后走哪个分支」；
 *  2. **沙箱执行**：队列为空时把 evaluate 回调**真的在 Node 里执行**，
 *     期间临时替换 globalThis.fetch / localStorage / sessionStorage / document
 *     为测试注入的桩——浏览器端回调体（fetchSelf / getJson / postJson 等约
 *     三百行）因此可以像普通代码一样被覆盖，而不是永远 0%。
 *
 * 用完即还原全局对象（try/finally），绝不污染同进程的其它测试。
 */
import type { Page, BrowserContext } from "playwright";

export interface FetchStubResult {
  status: number;
  body: string;
}

export type FetchRouter = (url: string, init: RequestInit | undefined) => FetchStubResult;

export interface LocatorBehavior {
  /** count() 返回值，默认 0 */
  count?: number;
  /** fill() 是否成功，默认 true */
  fillOk?: boolean;
  /** click() 是否成功，默认 true */
  clickOk?: boolean;
  /** throwOnFill 为真时 fill 抛错（模拟元素不可交互） */
}

export interface FakePageOptions {
  /** 初始 URL，也是 page.url() 的返回值 */
  url?: string;
  /**
   * fetch 路由：键为「匹配子串」，值为响应。evaluate 回调里发起的
   * fetch(url) 按子串最长匹配命中；未命中返回 404 空体。
   */
  fetchRoutes?: Record<string, FetchStubResult | ((url: string, init?: RequestInit) => FetchStubResult)>;
  /** evaluate 回调里可见的 localStorage 内容 */
  localStorage?: Record<string, string>;
  /**
   * 定位器行为：键为定位描述（如 `role:button:登录`、`css:input[type="password"]`、
   * `ph:email`），值为该定位器的行为。未配置的定位器 count=0（元素不存在）。
   */
  locators?: Record<string, LocatorBehavior>;
  /** 新建弹窗页（context.pages() 额外返回的页面） */
  popups?: FakePage[];
  /** goto 时把 URL 切到这个映射（键=访问的 URL 子串，值=落点 URL），默认原地不动 */
  gotoUrls?: Record<string, string>;
}

/** 定位器描述串：由 page.locator/getByRole/getByPlaceholder 归一化而来 */
function describeLocator(kind: string, key: string): string {
  return `${kind}:${key}`;
}

export class FakePage {
  /** 脚本化 evaluate 结果队列：先进先出；值或 (arg)=>值 */
  readonly evaluateQueue: Array<unknown | ((arg: unknown) => unknown)> = [];
  readonly gotoCalls: string[] = [];
  readonly clicks: string[] = [];
  readonly fills: Array<{ desc: string; value: string }> = [];
  private currentUrl: string;
  private closed = false;
  private readonly opts: FakePageOptions;
  /** 沙箱内 fetch 命中记录，供断言「到底请求了哪个端点、带了什么头」 */
  readonly fetchCalls: Array<{ url: string; init?: RequestInit }> = [];

  constructor(opts: FakePageOptions = {}) {
    this.opts = opts;
    this.currentUrl = opts.url ?? "https://site.test/profile";
  }

  /** 入队一条 evaluate 结果（值或以 evaluate 的 arg 为参的函数） */
  script(value: unknown | ((arg: unknown) => unknown)): this {
    this.evaluateQueue.push(value);
    return this;
  }

  setUrl(url: string): void {
    this.currentUrl = url;
  }

  async goto(url: string): Promise<unknown> {
    this.gotoCalls.push(url);
    for (const [match, target] of Object.entries(this.opts.gotoUrls ?? {})) {
      if (url.includes(match)) {
        this.currentUrl = target;
        break;
      }
    }
    return { ok: true };
  }

  url(): string {
    return this.currentUrl;
  }

  async waitForTimeout(): Promise<void> {
    /* 测试里时间快进 */
  }

  async waitForLoadState(): Promise<void> {
    /* noop */
  }

  async waitForURL(predicate: (u: URL) => boolean): Promise<void> {
    // 立刻判定：不满足按超时抛错（调用方全部 .catch(() => {})）
    if (!predicate(new URL(this.currentUrl))) {
      throw new Error("TimeoutError: page.waitForURL");
    }
  }

  isClosed(): boolean {
    return this.closed;
  }

  async close(): Promise<void> {
    this.closed = true;
  }

  context(): BrowserContext {
    return new FakeContext(this.opts, this) as unknown as BrowserContext;
  }

  locator(selector: string): FakeLocator {
    return new FakeLocator(this, describeLocator("css", selector));
  }

  getByRole(role: string, options?: { name?: string | RegExp; exact?: boolean }): FakeLocator {
    const name = options?.name;
    const key = typeof name === "string" ? name : name ? name.source : "";
    return new FakeLocator(this, describeLocator("role", `${role}:${key}`));
  }

  getByPlaceholder(placeholder: string | RegExp): FakeLocator {
    const key = typeof placeholder === "string" ? placeholder : placeholder.source;
    return new FakeLocator(this, describeLocator("ph", key));
  }

  /** findLocatorBehavior：定位描述的归一化匹配（fake 配置键 → 当前定位器） */
  resolveLocator(desc: string): LocatorBehavior | undefined {
    const table = this.opts.locators ?? {};
    if (table[desc]) return table[desc];
    // 宽松匹配：配置键作为子串出现在描述里（getByRole 的 RegExp 名会被 String 化）
    for (const [k, v] of Object.entries(table)) {
      if (desc.includes(k) || k.includes(desc)) return v;
    }
    return undefined;
  }

  recordClick(desc: string): void {
    this.clicks.push(desc);
  }

  recordFill(desc: string, value: string): void {
    this.fills.push({ desc, value });
  }

  // evaluate：脚本队列优先；否则沙箱执行回调本体
  async evaluate<T = unknown>(fn: (arg: any) => T, arg?: unknown): Promise<T> {
    if (this.evaluateQueue.length > 0) {
      const next = this.evaluateQueue.shift();
      return (typeof next === "function" ? (next as (a: unknown) => unknown)(arg) : next) as T;
    }
    return withBrowserSandbox(this.opts, this.fetchCalls, () => fn(arg));
  }
}

export class FakeLocator {
  constructor(
    private readonly page: FakePage,
    private readonly desc: string,
    private readonly nameHint?: string | RegExp,
  ) {}

  first(): FakeLocator {
    return this;
  }

  last(): FakeLocator {
    return this;
  }

  async count(): Promise<number> {
    return this.page.resolveLocator(this.desc)?.count ?? 0;
  }

  async fill(value: string, _opts?: object): Promise<void> {
    const b = this.page.resolveLocator(this.desc);
    // 模拟 Playwright：元素不存在时 fill 抛错（调用方全部 .catch 兜底）
    if (!b || b.count === 0) throw new Error("locator not found (fake)");
    this.page.recordFill(this.desc, value);
    if (b.fillOk === false) throw new Error("fill failed (fake)");
  }

  async click(_opts?: object): Promise<void> {
    const b = this.page.resolveLocator(this.desc);
    // 模拟 Playwright：元素不存在时 click 等待超时后抛错——不记录为真实点击
    if (!b || b.count === 0) throw new Error("locator not found (fake)");
    this.page.recordClick(this.desc);
    if (b.clickOk === false) throw new Error("click failed (fake)");
  }
}

export class FakeContext {
  constructor(
    private readonly opts: FakePageOptions,
    private readonly owner: FakePage,
  ) {}

  pages(): Page[] {
    // 忠实模拟 window.open 时序：弹窗在主页面上发生点击之后才存在——
    // 否则 adoptOAuthPopup 建 knownPages 时就把它算作「已知页」而永远不采纳
    const popups = this.owner.clicks.length > 0 ? (this.opts.popups ?? []) : [];
    return [this.owner as unknown as Page, ...(popups as unknown as Page[])];
  }

  async cookies(): Promise<Array<{ name: string; domain: string }>> {
    return (
      (this.opts as FakePageOptions & { cookies?: Array<{ name: string; domain: string }> })
        .cookies ?? [
        { name: "session", domain: "site.test" },
        { name: "acw_tc", domain: "site.test" },
        { name: "cf_clearance", domain: "site.test" },
        { name: "third_party", domain: "other.example" },
      ]
    );
  }

  async clearCookies(opts: { name: string }): Promise<void> {
    const list = (this.opts as FakePageOptions & { cookies?: Array<{ name: string; domain: string }> })
      .cookies;
    if (list) {
      const idx = list.findIndex((c) => c.name === opts.name);
      if (idx >= 0) list.splice(idx, 1);
    }
  }

  async newPage(): Promise<Page> {
    return this.owner as unknown as Page;
  }
}

/** 临时替换浏览器全局对象，执行 fn，最后无条件还原 */
async function withBrowserSandbox<T>(
  opts: FakePageOptions,
  fetchCalls: Array<{ url: string; init?: RequestInit }>,
  fn: () => T | Promise<T>,
): Promise<T> {
  const g = globalThis as Record<string, unknown>;
  const saved = {
    fetch: g.fetch,
    localStorage: g.localStorage,
    sessionStorage: g.sessionStorage,
    document: g.document,
  };
  const storageBag: Record<string, string> = { ...(opts.localStorage ?? {}) };
  // 真实 Storage 的形状：条目是可枚举 own key（回调靠 Object.keys 遍历），
  // 方法不可枚举——把方法挂成可枚举会让遍历拿到方法名而不是数据
  const storeObj: Record<string, unknown> = { ...storageBag };
  const methods: Record<string, (...args: unknown[]) => unknown> = {
    getItem: (k: unknown): string | null => {
      const key = k as string;
      return key in storageBag ? storageBag[key] : null;
    },
    setItem: (k: unknown, v: unknown): void => {
      storageBag[k as string] = String(v);
      storeObj[k as string] = String(v);
    },
    removeItem: (k: unknown): void => {
      delete storageBag[k as string];
      delete storeObj[k as string];
    },
    clear: (): void => {
      for (const k of Object.keys(storageBag)) {
        delete storageBag[k];
        delete storeObj[k];
      }
    },
    key: (i: unknown): string | null => Object.keys(storageBag)[i as number] ?? null,
  };
  for (const [name, fn] of Object.entries(methods)) {
    Object.defineProperty(storeObj, name, { value: fn, enumerable: false });
  }
  Object.defineProperty(storeObj, "length", {
    get: () => Object.keys(storageBag).length,
    enumerable: false,
  });
  const route = (url: string, init?: RequestInit): FetchStubResult => {
    const table = opts.fetchRoutes ?? {};
    let best: { matchLen: number; res: FetchStubResult } | null = null;
    for (const [match, res] of Object.entries(table)) {
      if (url.includes(match)) {
        const r = typeof res === "function" ? res(url, init) : res;
        if (!best || match.length > best.matchLen) best = { matchLen: match.length, res: r };
      }
    }
    return best?.res ?? { status: 404, body: "" };
  };
  g.fetch = (async (url: unknown, init?: RequestInit) => {
    const u = String(url);
    fetchCalls.push({ url: u, init });
    const r = route(u, init);
    return new Response(r.body, { status: r.status });
  }) as typeof fetch;
  g.localStorage = storeObj;
  const sessionObj: Record<string, unknown> = {};
  const sessionMethods: Record<string, (...args: unknown[]) => unknown> = {
    getItem: (): string | null => null,
    setItem: (): void => {},
    removeItem: (): void => {},
    clear: (): void => {},
    key: (): string | null => null,
  };
  for (const [name, fn] of Object.entries(sessionMethods)) {
    Object.defineProperty(sessionObj, name, { value: fn, enumerable: false });
  }
  Object.defineProperty(sessionObj, "length", { get: () => 0, enumerable: false });
  g.sessionStorage = sessionObj;
  // waitTurnstileToken 会查 document.querySelector；默认给一个「token 已就绪」的输入框
  g.document = {
    querySelector: (sel: string) =>
      sel.includes("cf-turnstile-response") ? { value: "tok".repeat(10) } : null,
  };
  try {
    return await fn();
  } finally {
    Object.assign(g, saved);
  }
}

/** 适配器上下文：不需要真实浏览器，log 静默收集、截图返回固定路径 */
export function makeAdapterCtx(
  page: FakePage,
  overrides: Partial<import("../src/types.js").AdapterContext> = {},
): import("../src/types.js").AdapterContext & { logs: string[]; screenshots: string[] } {
  const logs: string[] = [];
  const screenshots: string[] = [];
  return {
    credentials: { email: "user@test", password: "pass" },
    config: {
      schedule: { cron: "0 9 * * *", timezone: "Asia/Shanghai", runOnStartup: false },
      browser: { headless: true, timeoutMs: 45000 },
      sites: {},
      panel: { host: "127.0.0.1", port: 8787, token: "" },
    },
    browserContext: page.context(),
    log: (m: string) => logs.push(m),
    saveScreenshot: async (_p: unknown, tag: string) => {
      screenshots.push(tag);
      return `${tag}.png`;
    },
    logs,
    screenshots,
    ...overrides,
  } as never;
}

/** 构造 new-api 的 /api/user/self 成功响应体 */
export function newApiSelf(quota: number | null, username = "tester"): FetchStubResult {
  return {
    status: 200,
    body: JSON.stringify({ success: true, data: { quota, username } }),
  };
}
