// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import express from "express";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { dirname } from "node:path";
import type { AppConfig, CheckinResult } from "./types.js";
import { adapters } from "./sites/index.js";
import { getCredentials } from "./config.js";
import { computeSiteStatsBatch, computeTrend, getAllRecords, getStoreVersion, type StatsSummary } from "./store.js";
import { runAll, runSite, isRunning } from "./runner.js";
import { notifyResults } from "./notify.js";
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
// 不可估算时的回退说明（步进/列表 cron 暂不能算出具体时刻）。
// 没有它，面板会把「已排定但算不出时间」显示成「空闲待命」——那是句假话。
let nextRunNote: string | null = null;
export function setNextRunAt(iso: string | null, note: string | null = null): void {
  nextRunAt = iso;
  nextRunNote = iso ? null : note;
}

// 统计摘要缓存：记录库版本号不变就复用上次计算结果，
// 只有发生写入（签到落库）后才重新计算。
// 第 92 轮：`sitesConfigured` 还依赖凭据 / oauth 会话目录，仅按记录版本缓存会把
// 「刚配好凭据」或「刚建好会话」冻结成旧值（无签到写入时永远不刷新），改为把
// 就绪站点数一并纳入缓存键。
let summaryCache: { version: number; sitesConfigured: number; summary: StatsSummary } | null = null;

function countReadySites(): number {
  return adapters.filter((a) => isSiteReady(a.id, a.authType)).length;
}

/** 记录状态全集：面板筛选片与 ?status= 只认这四个值 */
const RECORD_STATUSES: readonly CheckinResult["status"][] = ["success", "already", "failed", "skipped"];

/**
 * 记录过滤（第 71 轮：站点 + 状态）
 *
 * 未知状态必须 fail-loud：面板手输 `?status=succes` 若静默回空集，
 * 会被误读成「今天没有成功记录」——这是句假话。
 */
function filterRecords(
  all: CheckinResult[],
  siteIdRaw: unknown,
  statusRaw: unknown,
): { ok: true; records: CheckinResult[] } | { ok: false; error: string } {
  const siteId = String(siteIdRaw ?? "").trim();
  const status = String(statusRaw ?? "").trim();
  if (status && !RECORD_STATUSES.includes(status as CheckinResult["status"])) {
    return { ok: false, error: `未知状态：${status}（可选：${RECORD_STATUSES.join(" / ")}）` };
  }
  return {
    ok: true,
    records: all.filter(
      (r) => (!siteId || r.siteId === siteId) && (!status || r.status === status),
    ),
  };
}

/**
 * 第 93 轮：`/api/records` 游标分页。
 *
 * 旧接口只能取「最近 limit 条」，更早的历史记录除了导出 CSV 无法翻页取得。
 * `before` 形如 `<ISO 时间>#<已返回的同毫秒条数>`（`#` 段可省略 = 0）：
 * 同一条毫秒内的多条记录按「从新到旧」的次序用 skip 区分，翻页不重不漏。
 */
function parseBeforeCursor(
  raw: unknown,
): { ok: true; at: number; skip: number } | { ok: false; error: string } {
  const text = String(raw ?? "").trim();
  const hash = text.lastIndexOf("#");
  const isoPart = hash >= 0 ? text.slice(0, hash) : text;
  const skip = hash >= 0 ? Number(text.slice(hash + 1)) : 0;
  const at = Date.parse(isoPart);
  if (!Number.isFinite(at) || !Number.isInteger(skip) || skip < 0) {
    return {
      ok: false,
      error: `非法游标：${text}（应为 <ISO 时间>#<已返回的同毫秒条数>）`,
    };
  }
  return { ok: true, at, skip };
}

function buildSummary(): StatsSummary {
  const version = getStoreVersion();
  const sitesConfigured = countReadySites();
  if (
    summaryCache
    && summaryCache.version === version
    && summaryCache.sitesConfigured === sitesConfigured
  ) {
    return summaryCache.summary;
  }
  // 第 78 轮：一次遍历分桶，取代「每站全量 filter 一遍记录库」。
  const sites = computeSiteStatsBatch(
    adapters.map((a) => ({ siteId: a.id, siteName: a.name })),
  );
  const totalRuns = sites.reduce((s, x) => s + x.totalRuns, 0);
  const failedRuns = sites.reduce((s, x) => s + x.failedRuns, 0);
  const skippedRuns = sites.reduce((s, x) => s + x.skippedRuns, 0);
  // 第 79 轮：总成功率与单站同口径——跳过（未配置/已禁用）不进分母。
  // 旧实现用 totalRuns 当分母，混入 skipped 的站整站被拉低，与站点卡片上的
  // successRate（排除 skipped）自相矛盾。
  const effectiveRuns = totalRuns - skippedRuns;
  // 第 87 轮：总平均耗时按**有效耗时样本数**加权。旧实现把各站 avgDurationMs
  // 直接算术平均——跑过 1 次的站与跑过 500 次的站权重相同，站点越多偏差越大。
  const durationSum = sites.reduce((s, x) => s + x.durationMsSum, 0);
  const durationCount = sites.reduce((s, x) => s + x.durationMsCount, 0);
  const totals = {
    totalCheckins: sites.reduce((s, x) => s + x.totalCheckins, 0),
    totalReward: Number(sites.reduce((s, x) => s + x.totalReward, 0).toFixed(4)),
    weekReward: Number(sites.reduce((s, x) => s + x.weekReward, 0).toFixed(4)),
    monthReward: Number(sites.reduce((s, x) => s + x.monthReward, 0).toFixed(4)),
    sitesCheckedInToday: sites.filter((x) => x.checkedInToday).length,
    sitesConfigured,
    totalRuns,
    failedRuns,
    successRate:
      effectiveRuns > 0 ? Math.round(((effectiveRuns - failedRuns) / effectiveRuns) * 100) : 0,
    avgDurationMs: durationCount > 0 ? Math.round(durationSum / durationCount) : 0,
  };
  const summary: StatsSummary = { sites, totals, generatedAt: new Date().toISOString() };
  summaryCache = { version, sitesConfigured, summary };
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
    res.json({ ...buildSummary(), nextRunAt, nextRunNote, running: isRunning() });
  });

  // 每日奖励趋势（用于折线图）
  // 第 40 轮：面板 15 秒轮询三接口，/api/trend 此前每次请求都全量重算。
  // 记录库版本不变（无签到落库）就复用上次结果，与 /api/stats 的
  // buildSummary 同款 getStoreVersion 判据；days 参数变了也重算。
  let trendCache: { version: number; days: number; data: unknown } | null = null;
  app.get("/api/trend", (req, res) => {
    const days = Math.min(Math.max(Number(req.query.days) || 30, 7), 90);
    const version = getStoreVersion();
    if (trendCache && trendCache.version === version && trendCache.days === days) {
      res.json(trendCache.data);
      return;
    }
    const siteIds = adapters.map((a) => a.id);
    const data = {
      days,
      sites: adapters.map((a) => ({ id: a.id, name: a.name })),
      points: computeTrend(siteIds, days),
    };
    trendCache = { version, days, data };
    res.json(data);
  });

  // 历史记录（最近 N 条；第 42 轮起支持按站点过滤，第 71 轮起支持按状态过滤，
  // 第 93 轮起支持 `before` 游标向前翻页）
  app.get("/api/records", (req, res) => {
    const limit = Math.min(Math.max(Number(req.query.limit) || 100, 1), 500);
    const picked = filterRecords(getAllRecords(), req.query.siteId, req.query.status);
    if (!picked.ok) {
      res.status(400).json({ error: picked.error });
      return;
    }
    const beforeRaw = String(req.query.before ?? "").trim();
    let cursor: { at: number; skip: number } | null = null;
    if (beforeRaw) {
      const parsed = parseBeforeCursor(beforeRaw);
      if (!parsed.ok) {
        res.status(400).json({ error: parsed.error });
        return;
      }
      cursor = parsed;
    }

    // 记录按写入顺序（旧→新）存储；单趟从新到旧扫描，直接收集本页。
    //
    // 第 96 轮：旧实现 `[...records].reverse()` → `filter` → `slice` 各复制一份
    // 全量数组；记录库上限 2000 条时每次翻页都要复制三份。这里只分配 `page`
    // （≤ limit 条），游标语义保持不变：时间新于游标的跳过，同毫秒按 `skip` 条数跳过。
    const page: CheckinResult[] = [];
    let skippedSame = 0;
    for (let i = picked.records.length - 1; i >= 0 && page.length < limit; i--) {
      const r = picked.records[i];
      if (cursor) {
        const at = Date.parse(r.timestamp);
        if (!Number.isFinite(at) || at > cursor.at) continue;
        if (at === cursor.at && skippedSame < cursor.skip) {
          skippedSame++;
          continue;
        }
      }
      page.push(r);
    }
    if (page.length > 0) {
      const last = page[page.length - 1];
      const lastAt = Date.parse(last.timestamp);
      if (Number.isFinite(lastAt)) {
        // 下一页游标：本页最后一条的时间 + 截至它已返回的同毫秒条数
        const returnedSame = page.filter((r) => Date.parse(r.timestamp) === lastAt).length;
        const skip = (cursor && cursor.at === lastAt ? cursor.skip : 0) + returnedSame;
        res.setHeader("X-Next-Cursor", `${new Date(lastAt).toISOString()}#${skip}`);
      }
    }
    res.json(page);
  });

  // 历史记录 CSV 导出（第 42 轮）：全量快照，可带 siteId 过滤；
  // 第 71 轮：同样支持 status 过滤（运营导出失败明细 / 成功流水）。
  // 面板轮询三接口不带 CSV；运营/排障要全量明细时用浏览器直接下载。
  app.get("/api/records.csv", (req, res) => {
    const picked = filterRecords(getAllRecords(), req.query.siteId, req.query.status);
    if (!picked.ok) {
      res.status(400).json({ error: picked.error });
      return;
    }
    const filtered = picked.records;
    // 第 80 轮：CSV 公式注入防护。站点响应里的 message 等字段完全由远端站点
    // 控制，以 `=`/`+`/`-`/`@` 开头时 Excel/Sheets 打开导出文件会当公式执行
    // （`=HYPERLINK(...)` 等）。给这类单元格前置单引号，令其按文本显示。
    const esc = (v: unknown): string => {
      let s = String(v ?? "");
      if (/^[=+\-@\t\r]/.test(s)) s = `'${s}`;
      return /[",\n]/.test(s) ? `"${s.replace(/"/g, '""')}"` : s;
    };
    const header = "timestamp,site_id,site_name,status,reward,balance,currency,duration_ms,message,screenshot";
    const rows = filtered.map((r) =>
      [
        r.timestamp, r.siteId, r.siteName, r.status,
        r.reward ?? "", r.balance ?? "", r.currency ?? "", r.durationMs,
        r.message, r.screenshot ?? "",
      ].map(esc).join(","),
    );
    res.type("text/csv; charset=utf-8");
    res.setHeader("Content-Disposition", `attachment; filename="records.csv"`);
    // \uFEFF = UTF-8 BOM：Excel 打开中文不乱码。
    res.send("\uFEFF" + [header, ...rows].join("\n"));
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
        // 第 61 轮：面板触发同样推 webhook。
        void notifyResults(config, results);
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
        if (!result) {
          log.error(`面板触发的 ${siteId} 未返回结果（站点已注册但运行器返回 null）`);
          return;
        }
        if (shouldFailRun([result])) log.error(`面板触发的 ${siteId} 未完成或失败`);
        // 第 74 轮：第 61 轮的口径是「三个触发点（定时/面板/启动即跑）全接入」
        // —— 单站手动触发也是面板触发，此前漏推，webhook 只能看到整轮结果。
        void notifyResults(config, [result]);
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