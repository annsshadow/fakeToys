// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import type { AppConfig, CheckinResult, SiteAdapter } from "./types.js";
import { getCredentials } from "./config.js";
import { openSiteContext, closeContext, saveScreenshot } from "./browser.js";
import { addRecord } from "./store.js";
import { createLogger } from "./logger.js";
import { adapters, getAdapter } from "./sites/index.js";
import { acquireRunLock, releaseRunLock } from "./lock.js";

let running = false;

export function isRunning(): boolean {
  return running;
}

/**
 * 同时运行的站点上限。站点之间相互独立，但每个站点都会开一个 Chromium
 * 持久化上下文，全部并行会争抢 CPU/内存，因此限制并发数而不是完全串行。
 */
const MAX_CONCURRENT_SITES = 2;

/**
 * 有界并发地映射一组任务：
 * - 保持结果顺序与输入一致；
 * - 单个任务抛错不会中断其他任务（先让全部任务跑完再抛出第一个错误）。
 */
export async function mapWithConcurrency<T, R>(
  items: T[],
  limit: number,
  fn: (item: T, index: number) => Promise<R>,
): Promise<R[]> {
  const results = new Array<R>(items.length);
  let cursor = 0;
  const workers = Array.from({ length: Math.max(1, Math.min(limit, items.length)) }, async () => {
    for (;;) {
      const i = cursor++;
      if (i >= items.length) return;
      results[i] = await fn(items[i], i);
    }
  });
  const settled = await Promise.allSettled(workers);
  const firstError = settled.find((s) => s.status === "rejected");
  if (firstError && firstError.status === "rejected") throw firstError.reason;
  return results;
}

function failedResult(adapter: SiteAdapter, message: string): CheckinResult {
  return {
    siteId: adapter.id,
    siteName: adapter.name,
    timestamp: new Date().toISOString(),
    status: "failed",
    reward: null,
    balance: null,
    currency: null,
    message,
    screenshot: null,
    durationMs: 0,
  };
}

/** 运行单个适配器，落库并返回结果。任何异常都收敛为 failed 结果，绝不让单站拖垮整体。 */
async function runOneSafe(adapter: SiteAdapter, config: AppConfig): Promise<CheckinResult> {
  try {
    return await runOne(adapter, config);
  } catch (err) {
    const result = failedResult(adapter, `运行异常：${(err as Error).message}`);
    addRecord(result);
    return result;
  }
}

/**
 * 运行单个适配器，落库并返回结果。
 *
 * @param existingContext 复用已有浏览器上下文。手动登录流程（login）已经持有
 *   同一个持久化 profile，此时再开一个会撞 Chromium 的 profile 锁；且 WAF 放行
 *   窗口只有十几秒，重开上下文会白白过期。
 */
async function runOne(
  adapter: SiteAdapter,
  config: AppConfig,
  existingContext?: Awaited<ReturnType<typeof openSiteContext>>,
): Promise<CheckinResult> {
  const siteLog = createLogger(adapter.id);
  const start = Date.now();

  const base: Omit<CheckinResult, "status" | "reward" | "balance" | "currency" | "message" | "screenshot" | "durationMs"> = {
    siteId: adapter.id,
    siteName: adapter.name,
    timestamp: new Date().toISOString(),
  };

  // 站点被禁用
  if (config.sites[adapter.id]?.enabled === false) {
    siteLog.info("已在配置中禁用，跳过");
    const result: CheckinResult = { ...base, status: "skipped", reward: null, balance: null, currency: null, message: "配置中已禁用", screenshot: null, durationMs: 0 };
    addRecord(result);
    return result;
  }

  // 凭据检查：password 类站点需 .env 账号密码；oauth 类站点靠持久化会话（login 命令建立），无需凭据
  const credentials = getCredentials(adapter.id) ?? { email: "", password: "" };
  if (adapter.authType === "password" && (!credentials.email || !credentials.password)) {
    siteLog.warn("未配置账号密码，跳过（在 .env 中填写后生效）");
    const result: CheckinResult = { ...base, status: "skipped", reward: null, balance: null, currency: null, message: "未配置账号密码", screenshot: null, durationMs: 0 };
    addRecord(result);
    return result;
  }

  siteLog.info("开始签到…");
  let context: Awaited<ReturnType<typeof openSiteContext>> | undefined = existingContext;
  const ownsContext = !existingContext;
  try {
    // 浏览器启动也必须纳入单站故障边界：某一站启动失败不能阻断后续站点。
    if (!context) context = await openSiteContext(adapter.id, config);
    const out = await adapter.checkin({
      credentials,
      config,
      browserContext: context,
      log: (m) => siteLog.info(m),
      saveScreenshot,
    });
    const result: CheckinResult = {
      ...base,
      status: out.status,
      reward: out.reward,
      balance: out.balance,
      currency: out.currency,
      message: out.message,
      screenshot: out.screenshot ?? null,
      durationMs: Date.now() - start,
    };
    siteLog.info(`完成：${result.status} — ${result.message}`);
    addRecord(result);
    return result;
  } catch (err) {
    const result = failedResult(adapter, `运行异常：${(err as Error).message}`);
    siteLog.error(result.message);
    addRecord(result);
    return result;
  } finally {
    if (ownsContext && context) await closeContext(context);
  }
}

/**
 * 在已打开的浏览器上下文里跑一次签到，供手动登录流程在过完人机验证后立刻复用。
 * 调用方负责关闭上下文。
 */
export async function runSiteInContext(
  siteId: string,
  config: AppConfig,
  context: Awaited<ReturnType<typeof openSiteContext>>,
): Promise<CheckinResult | null> {
  const adapter = getAdapter(siteId);
  if (!adapter) return null;
  return runOne(adapter, config, context);
}

/** 运行全部站点（有界并发，单站故障互不影响；结果顺序与 adapters 一致） */
export async function runAll(config: AppConfig): Promise<CheckinResult[]> {
  if (running) {
    createLogger("runner").warn("已有任务在运行，跳过本次触发");
    return [];
  }
  const lock = acquireRunLock();
  if (!lock.ok) {
    createLogger("runner").warn(`已有其他签到/登录进程在运行（PID=${lock.holderPid}），跳过本次触发`);
    return [];
  }

  running = true;
  try {
    return await mapWithConcurrency(adapters, MAX_CONCURRENT_SITES, (adapter) =>
      runOneSafe(adapter, config),
    );
  } finally {
    running = false;
    releaseRunLock();
  }
}

/** 运行单个指定站点 */
export async function runSite(siteId: string, config: AppConfig): Promise<CheckinResult | null> {
  const adapter = getAdapter(siteId);
  if (!adapter) return null;
  if (running) {
    createLogger("runner").warn("已有任务在运行，跳过本次触发");
    return null;
  }
  const lock = acquireRunLock();
  if (!lock.ok) {
    createLogger("runner").warn(`已有其他签到/登录进程在运行（PID=${lock.holderPid}），跳过本次运行`);
    return null;
  }

  running = true;
  try {
    return await runOne(adapter, config);
  } finally {
    running = false;
    releaseRunLock();
  }
}
