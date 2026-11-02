// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync, existsSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";
import dotenv from "dotenv";
import type { AppConfig, SiteCredentials } from "./types.js";

const __dirname = dirname(fileURLToPath(import.meta.url));
export const ROOT_DIR = join(__dirname, "..");

// 加载 .env（不存在也不报错，交由各站点凭据缺失时跳过）
dotenv.config({ path: join(ROOT_DIR, ".env") });

const DEFAULT_CONFIG: AppConfig = {
  schedule: { cron: "0 9 * * *", timezone: "Asia/Shanghai", runOnStartup: true },
  browser: { headless: true, timeoutMs: 45000 },
  sites: {
    vyceai: { enabled: true },
    agentrouter: { enabled: true },
    justwoker: { enabled: true },
  },
  panel: { host: "127.0.0.1", port: 8787, token: "" },
  // 第 60 轮：默认关闭重试 —— 网络/站点故障立即落一条 failed 记录并留痕，
  // 需要「失败自动再试」的部署在 config.json 里打开 retry。
  retry: { enabled: false, attempts: 1, intervalMs: 30000 },
};

/** 深合并：用户 config.json 覆盖默认值 */
function mergeConfig(base: AppConfig, override: Record<string, unknown>): AppConfig {
  const out: AppConfig = structuredClone(base);
  const o = override as Partial<AppConfig>;
  if (o.schedule) Object.assign(out.schedule, o.schedule);
  if (o.browser) Object.assign(out.browser, o.browser);
  // 第 39 轮：panel 段此前被整段丢弃 —— 用户 config.json 写
  // `{"panel":{"port":9999}}` 永远不生效（仍起 8787）。
  if (o.panel) Object.assign(out.panel, o.panel);
  // 第 60 轮：retry 段（缺省关闭）。
  if (o.retry) Object.assign(out.retry ?? {}, o.retry);
  if (o.sites) {
    for (const [id, cfg] of Object.entries(o.sites)) {
      out.sites[id] = { ...(out.sites[id] ?? { enabled: true }), ...cfg };
    }
  }
  return out;
}

// 第 39 轮：configPath 可注入（测试传临时目录），默认仍读仓库根目录 config.json。
export function loadConfig(configPath: string = join(ROOT_DIR, "config.json")): AppConfig {
  let config = DEFAULT_CONFIG;
  if (existsSync(configPath)) {
    try {
      const raw = JSON.parse(readFileSync(configPath, "utf-8")) as Record<string, unknown>;
      config = mergeConfig(DEFAULT_CONFIG, raw);
    } catch (err) {
      console.error(`[config] 解析 config.json 失败，使用默认配置：${(err as Error).message}`);
    }
  }

  // 面板配置由环境变量**逐项**覆盖（避免把 host/port/token 写进入库文件）；
  // config.json 里已写的值保留，环境变量只覆盖对应单项（第 39 轮）。
  config.panel.host = process.env.PANEL_HOST || config.panel.host;
  config.panel.port = Number(process.env.PANEL_PORT) || config.panel.port;
  config.panel.token = process.env.PANEL_TOKEN || config.panel.token;
  return config;
}

/** 从环境变量读取某站点的凭据；未配置则返回 null（该站点将被跳过） */
export function getCredentials(siteId: string): SiteCredentials | null {
  const prefix = siteId.toUpperCase();
  const email = process.env[`${prefix}_EMAIL`];
  const password = process.env[`${prefix}_PASSWORD`];
  if (!email || !password) return null;
  return { email, password };
}
