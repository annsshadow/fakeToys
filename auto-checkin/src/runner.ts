// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import type { AppConfig, CheckinResult, SiteAdapter } from "./types.js";
import { getCredentials } from "./config.js";
import { openSiteContext, closeContext, saveScreenshot } from "./browser.js";
import { addRecord } from "./store.js";
import { createLogger } from "./logger.js";
import { adapters, getAdapter } from "./sites/index.js";

let running = false;

export function isRunning(): boolean {
  return running;
}

/** 运行单个适配器，落库并返回结果 */
async function runOne(adapter: SiteAdapter, config: AppConfig): Promise<CheckinResult> {
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
  const context = await openSiteContext(adapter.id, config);
  try {
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
    const result: CheckinResult = {
      ...base,
      status: "failed",
      reward: null,
      balance: null,
      currency: null,
      message: `运行异常：${(err as Error).message}`,
      screenshot: null,
      durationMs: Date.now() - start,
    };
    siteLog.error(result.message);
    addRecord(result);
    return result;
  } finally {
    await closeContext(context);
  }
}

/** 运行全部站点（串行，避免同时开多个浏览器占用资源） */
export async function runAll(config: AppConfig): Promise<CheckinResult[]> {
  if (running) {
    createLogger("runner").warn("已有任务在运行，跳过本次触发");
    return [];
  }
  running = true;
  const results: CheckinResult[] = [];
  try {
    for (const adapter of adapters) {
      results.push(await runOne(adapter, config));
    }
  } finally {
    running = false;
  }
  return results;
}

/** 运行单个指定站点 */
export async function runSite(siteId: string, config: AppConfig): Promise<CheckinResult | null> {
  const adapter = getAdapter(siteId);
  if (!adapter) return null;
  if (running) {
    createLogger("runner").warn("已有任务在运行，跳过本次触发");
    return null;
  }
  running = true;
  try {
    return await runOne(adapter, config);
  } finally {
    running = false;
  }
}
