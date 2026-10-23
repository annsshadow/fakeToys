// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import express from "express";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { dirname } from "node:path";
import type { AppConfig } from "./types.js";
import { adapters } from "./sites/index.js";
import { getCredentials } from "./config.js";
import { computeSiteStats, computeTrend, getAllRecords, getStoreVersion, type StatsSummary } from "./store.js";
import { runAll, runSite, isRunning } from "./runner.js";
import { shouldFailRun } from "./run-status.js";
import { createLogger } from "./logger.js";
import { ROOT_DIR } from "./config.js";
import { existsSync } from "node:fs";

const __dirname = dirname(fileURLToPath(import.meta.url));
const log = createLogger("server");

/** 站点是否已就绪：password 类看 .env 凭据；oauth 类看是否已建立持久化会话 */
function isSiteReady(siteId: string, authType: "password" | "oauth"): boolean {
  if (authType === "password") return getCredentials(siteId) !== null;
  return existsSync(join(ROOT_DIR, "sessions", siteId));
}

let nextRunAt: string | null = null;
export function setNextRunAt(iso: string | null): void {
  nextRunAt = iso;
}

// 统计摘要缓存：记录库版本号不变就复用上次计算结果，
// 只有发生写入（签到落库）后才重新计算。
let summaryCache: { version: number; summary: StatsSummary } | null = null;

function buildSummary(): StatsSummary {
  const version = getStoreVersion();
  if (summaryCache && summaryCache.version === version) return summaryCache.summary;
  const sites = adapters.map((a) => computeSiteStats(a.id, a.name));
  const totalRuns = sites.reduce((s, x) => s + x.totalRuns, 0);
  const failedRuns = sites.reduce((s, x) => s + x.failedRuns, 0);
  const durationSites = sites.filter((x) => x.avgDurationMs > 0);
  const totals = {
    totalCheckins: sites.reduce((s, x) => s + x.totalCheckins, 0),
    totalReward: Number(sites.reduce((s, x) => s + x.totalReward, 0).toFixed(4)),
    weekReward: Number(sites.reduce((s, x) => s + x.weekReward, 0).toFixed(4)),
    monthReward: Number(sites.reduce((s, x) => s + x.monthReward, 0).toFixed(4)),
    sitesCheckedInToday: sites.filter((x) => x.checkedInToday).length,
    sitesConfigured: adapters.filter((a) => isSiteReady(a.id, a.authType)).length,
    totalRuns,
    failedRuns,
    successRate:
      totalRuns > 0 ? Math.round(((totalRuns - failedRuns) / totalRuns) * 100) : 0,
    avgDurationMs:
      durationSites.length > 0
        ? Math.round(durationSites.reduce((s, x) => s + x.avgDurationMs, 0) / durationSites.length)
        : 0,
  };
  const summary: StatsSummary = { sites, totals, generatedAt: new Date().toISOString() };
  summaryCache = { version, summary };
  return summary;
}

export function createApp(config: AppConfig): express.Express {
  const app = express();
  app.use(express.json());

  // 可选访问口令：设置了 PANEL_TOKEN 时，仅 /api/ 需带 X-Token 头。
  // 不接受 ?token= 查询参数——查询串会进入代理/访问日志，容易泄漏。
  app.use((req, res, next) => {
    if (!config.panel.token) return next();
    // 只守 API：静态资源（面板 HTML/CSS/JS 与失败截图）不带头即可访问。
    // 曾经的放行清单是 "/" 与 /assets*，但 public/ 实际只有 index.html、
    // style.css、app.js（根路径），截图在 /screenshots/ 下——设了 PANEL_TOKEN
    // 后 CSS/JS/截图全部 401，面板不可用（2026-10-24 修）。敏感数据全在
    // /api/ 之下，静态面不含敏感内容，故按前缀整体放行。
    if (!req.path.startsWith("/api/")) return next();
    const token = req.header("X-Token");
    if (token !== config.panel.token) {
      res.status(401).json({ error: "unauthorized" });
      return;
    }
    next();
  });

  // 静态前端
  app.use(express.static(join(__dirname, "public")));
  // 失败截图（只读，位于项目根 screenshots/）
  app.use("/screenshots", express.static(join(ROOT_DIR, "screenshots")));

  // 统计总览
  app.get("/api/stats", (_req, res) => {
    res.json({ ...buildSummary(), nextRunAt, running: isRunning() });
  });

  // 每日奖励趋势（用于折线图）
  app.get("/api/trend", (req, res) => {
    const days = Math.min(Math.max(Number(req.query.days) || 30, 7), 90);
    const siteIds = adapters.map((a) => a.id);
    res.json({
      days,
      sites: adapters.map((a) => ({ id: a.id, name: a.name })),
      points: computeTrend(siteIds, days),
    });
  });

  // 历史记录（最近 N 条）
  app.get("/api/records", (req, res) => {
    const limit = Math.min(Math.max(Number(req.query.limit) || 100, 1), 500);
    const all = getAllRecords();
    res.json(all.slice(-limit).reverse());
  });

  // 手动触发全部
  app.post("/api/run", async (_req, res) => {
    if (isRunning()) {
      res.status(409).json({ error: "已有任务在运行" });
      return;
    }
    log.info("面板触发：全部签到");
    runAll(config)
      .then((results) => {
        if (shouldFailRun(results)) log.error("面板触发的全部签到未完成或存在失败站点");
      })
      .catch((e) => log.error(String(e)));
    res.json({ ok: true, message: "已开始运行全部站点签到" });
  });

  // 手动触发单站
  app.post("/api/run/:siteId", async (req, res) => {
    const { siteId } = req.params;
    if (!adapters.some((a) => a.id === siteId)) {
      res.status(404).json({ error: "未知站点" });
      return;
    }
    if (isRunning()) {
      res.status(409).json({ error: "已有任务在运行" });
      return;
    }
    log.info(`面板触发：${siteId}`);
    runSite(siteId, config)
      .then((result) => {
        if (!result || shouldFailRun([result])) log.error(`面板触发的 ${siteId} 未完成或失败`);
      })
      .catch((e) => log.error(String(e)));
    res.json({ ok: true, message: `已开始运行 ${siteId}` });
  });

  return app;
}

export function startServer(config: AppConfig): void {
  const app = createApp(config);
  app.listen(config.panel.port, config.panel.host, () => {
    log.info(`Web 面板已启动： http://${config.panel.host}:${config.panel.port}`);
    if (config.panel.token) log.info("已启用访问口令（PANEL_TOKEN）");
  });
}