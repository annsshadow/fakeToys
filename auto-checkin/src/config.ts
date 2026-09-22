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
};

/** 深合并：用户 config.json 覆盖默认值 */
function mergeConfig(base: AppConfig, override: Record<string, unknown>): AppConfig {
  const out: AppConfig = structuredClone(base);
  const o = override as Partial<AppConfig>;
  if (o.schedule) Object.assign(out.schedule, o.schedule);
  if (o.browser) Object.assign(out.browser, o.browser);
  if (o.sites) {
    for (const [id, cfg] of Object.entries(o.sites)) {
      out.sites[id] = { ...(out.sites[id] ?? { enabled: true }), ...cfg };
    }
  }
  return out;
}

export function loadConfig(): AppConfig {
  let config = DEFAULT_CONFIG;
  const configPath = join(ROOT_DIR, "config.json");
  if (existsSync(configPath)) {
    try {
      const raw = JSON.parse(readFileSync(configPath, "utf-8")) as Record<string, unknown>;
      config = mergeConfig(DEFAULT_CONFIG, raw);
    } catch (err) {
      console.error(`[config] 解析 config.json 失败，使用默认配置：${(err as Error).message}`);
    }
  }

  // 面板配置由环境变量覆盖（避免把 host/port/token 写进入库文件）
  config.panel = {
    host: process.env.PANEL_HOST || DEFAULT_CONFIG.panel.host,
    port: Number(process.env.PANEL_PORT) || DEFAULT_CONFIG.panel.port,
    token: process.env.PANEL_TOKEN || "",
  };
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
