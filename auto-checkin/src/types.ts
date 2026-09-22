// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

// 全局共享类型定义

export interface SiteCredentials {
  email: string;
  password: string;
}

export interface SiteConfig {
  enabled: boolean;
}

export interface AppConfig {
  schedule: {
    cron: string;
    timezone: string;
    runOnStartup: boolean;
  };
  browser: {
    headless: boolean;
    timeoutMs: number;
  };
  sites: Record<string, SiteConfig>;
  panel: {
    host: string;
    port: number;
    token: string;
  };
}

/** 单次签到的结果 */
export interface CheckinResult {
  siteId: string;
  siteName: string;
  /** ISO 时间戳 */
  timestamp: string;
  /** 成功领取 / 今日已领 / 失败 / 跳过 */
  status: "success" | "already" | "failed" | "skipped";
  /** 本次领取到的奖励量（quota 差值），领取前后余额已知时才有值 */
  reward: number | null;
  /** 领取后的账户余额 */
  balance: number | null;
  /** 展示用货币/单位，如 "$" 或 "credits" */
  currency: string | null;
  /** 人类可读的说明或错误信息 */
  message: string;
  /** 失败时的截图相对路径（若有） */
  screenshot: string | null;
  /** 本次耗时（毫秒） */
  durationMs: number;
}

/** 站点适配器接口：每个公益站实现一个 */
export interface SiteAdapter {
  id: string;
  name: string;
  /** 站点根地址，用于拼接接口与页面 */
  baseUrl: string;
  /**
   * 认证方式：
   *   - "password"：需在 .env 配置账号密码（如 VyceAI）
   *   - "oauth"：GitHub 等第三方登录，需先 `npm run login -- <id>` 建立会话（如 AgentRouter/JustWoker）
   */
  authType: "password" | "oauth";
  /**
   * 执行一次签到/领奖。适配器内部负责登录、触发领取、读取余额。
   * 通过 ctx 提供浏览器、凭据、配置、日志等能力。
   */
  checkin(ctx: AdapterContext): Promise<AdapterOutcome>;
}

/** 适配器返回的原始结果（runner 会补齐 siteId/时间等字段） */
export interface AdapterOutcome {
  status: CheckinResult["status"];
  reward: number | null;
  balance: number | null;
  currency: string | null;
  message: string;
  screenshot?: string | null;
}

export interface AdapterContext {
  credentials: SiteCredentials;
  config: AppConfig;
  /** 供适配器创建页面的 Playwright 浏览器上下文（每站独立持久化会话） */
  browserContext: import("playwright").BrowserContext;
  log: (msg: string) => void;
  /** 失败时保存截图，返回相对路径 */
  saveScreenshot: (page: import("playwright").Page, tag: string) => Promise<string | null>;
}
