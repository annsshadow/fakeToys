// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/** 第 61 轮：签到结果 webhook 通知。

 * 跑完一轮（定时触发/面板触发/启动即跑）后把结果摘要 POST 到 webhook。
 * best-effort：通知失败只留 warn 日志，绝不影响签到主流程。
 * URL 配置在 `config.json` 的 `notify.webhookUrl`（可含 secret，不进仓库）
 * 或环境变量 `AUTO_CHECKIN_WEBHOOK_URL`（env 优先）。
 */

import type { AppConfig, CheckinResult } from "./types.js";
import { createLogger } from "./logger.js";

const log = createLogger("notify");

export function resolveWebhookUrl(config: AppConfig): string {
  const fromEnv = process.env.AUTO_CHECKIN_WEBHOOK_URL;
  if (fromEnv && fromEnv.trim()) return fromEnv.trim();
  const fromConfig = config.notify?.webhookUrl;
  return fromConfig && fromConfig.trim() ? fromConfig.trim() : "";
}

/** 把一轮签到结果 POST 到 webhook。无 URL / 无结果时静默跳过。 */
export async function notifyResults(config: AppConfig, results: CheckinResult[]): Promise<void> {
  const url = resolveWebhookUrl(config);
  if (!url || results.length === 0) return;

  const payload = {
    type: "auto-checkin.results",
    timestamp: new Date().toISOString(),
    summary: {
      total: results.length,
      ok: results.filter((r) => r.status === "success" || r.status === "already").length,
      skipped: results.filter((r) => r.status === "skipped").length,
      failed: results.filter((r) => r.status === "failed").length,
    },
    results: results.map((r) => ({
      site: r.siteName,
      status: r.status,
      reward: r.reward,
      message: r.message,
      durationMs: r.durationMs,
    })),
  };

  try {
    const resp = await fetch(url, {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify(payload),
      signal: AbortSignal.timeout(5000),
    });
    if (resp.ok) {
      log.info(`webhook 通知成功（${results.length} 条结果）`);
    } else {
      log.warn(`webhook 响应 ${resp.status}`);
    }
  } catch (err) {
    log.warn(`webhook 通知失败（不影响主流程）：${(err as Error).message}`);
  }
}