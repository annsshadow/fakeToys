// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync, writeFileSync, renameSync, mkdirSync, existsSync } from "node:fs";
import { resolve, join } from "node:path";
import { ROOT_DIR } from "./config.js";
import { createLogger } from "./logger.js";
import type { CheckinResult } from "./types.js";

// 数据目录默认在项目内；测试等场景可用 AUTO_CHECKIN_DATA_DIR 指到别处
const DATA_DIR = process.env.AUTO_CHECKIN_DATA_DIR
  ? resolve(process.env.AUTO_CHECKIN_DATA_DIR)
  : join(ROOT_DIR, "data");
mkdirSync(DATA_DIR, { recursive: true });
const DB_PATH = join(DATA_DIR, "records.json");

interface Db {
  records: CheckinResult[];
}

// 内存缓存：读多写少，避免每个 /api/stats 请求都全量读盘。
// 写操作（addRecord）会同步刷新缓存并递增版本号，缓存即失效。
let cache: Db | null = null;
let version = 0;

/** 记录库版本号：每次写入 +1，供上层（如面板统计）判断缓存是否过期 */
export function getStoreVersion(): number {
  return version;
}

function readFromDisk(): Db {
  if (!existsSync(DB_PATH)) return { records: [] };
  try {
    const db = JSON.parse(readFileSync(DB_PATH, "utf-8")) as Db;
    if (!Array.isArray(db.records)) return { records: [] };
    return db;
  } catch {
    // 文件损坏（半截 JSON / 非 UTF-8 / 被别的东西覆盖）时**先隔离再当空账**。
    //
    // 原来这里直接返回空账：下一次 `addRecord` 会以「空账 + 1 条」整份重写，
    // 把损坏文件里本可人工抢救的历史记录永久覆盖掉 —— 一次坏 JSON 等于
    // 全部签到历史归零，而且没有任何信号。改名成 `records.corrupt-<时间戳>.json`
    // 后，「抢救」与「继续跑」两件事互不冲突。
    quarantineCorruptFile();
    return { records: [] };
  }
}

function quarantineCorruptFile(): void {
  const stamp = new Date().toISOString().replace(/[:.]/g, "-");
  const dest = join(DATA_DIR, `records.corrupt-${stamp}.json`);
  try {
    renameSync(DB_PATH, dest);
    createLogger("store").warn(`记录文件损坏，已隔离到 ${dest}，本次按空记录继续`);
  } catch (err) {
    // 隔离失败也不能让读取路径抛：调用方（面板统计）应拿到空账而不是 500。
    createLogger("store").error(`记录文件损坏且隔离失败：${(err as Error).message}`);
  }
}

function read(): Db {
  if (cache) return cache;
  const db = readFromDisk();
  cache = db;
  return db;
}

function persist(db: Db): void {
  // 原子写入：先写临时文件再 rename，避免半截 JSON 或崩溃时丢失全部记录
  const tmp = `${DB_PATH}.tmp`;
  writeFileSync(tmp, JSON.stringify(db, null, 2));
  renameSync(tmp, DB_PATH);
  cache = db;
  version++;
}

export function addRecord(result: CheckinResult): void {
  // 写入前以磁盘最新内容为基准：记录文件可能被进程外改动（补跑/修复脚本），
  // 直接沿用内存旧快照会把它们覆盖掉。
  const db = readFromDisk();
  db.records.push(result);
  // 仅保留最近 2000 条，避免文件无限膨胀
  if (db.records.length > 2000) db.records = db.records.slice(-2000);
  persist(db);
}

export function getAllRecords(): CheckinResult[] {
  return read().records;
}

// 第 78 轮：Date 只解析一次。旧实现对同一条记录分别用
// `localDay(iso)`、`weekStart(new Date(iso))`、`monthKey(iso)` 各解析一次，
// 统计每条记录要新建 3 个 Date、3 个日期串；改为拿到 Date 后在各处复用。
function localDayOf(d: Date): string {
  // 用本地时区判断"今天"
  const y = d.getFullYear();
  const m = String(d.getMonth() + 1).padStart(2, "0");
  const day = String(d.getDate()).padStart(2, "0");
  return `${y}-${m}-${day}`;
}

function todayStr(): string {
  return localDayOf(new Date());
}

/** 给定日期所在周的周一（本地时区），返回 YYYY-MM-DD */
function weekStartOf(d: Date): string {
  const t = new Date(d);
  const day = (t.getDay() + 6) % 7; // 周一=0
  t.setDate(t.getDate() - day);
  return localDayOf(t);
}

/** 给定日期所在月，返回 YYYY-MM */
function monthKeyOf(d: Date): string {
  return localDayOf(d).slice(0, 7);
}

export interface SiteStats {
  siteId: string;
  siteName: string;
  /** 今日是否已成功领取（success 或 already） */
  checkedInToday: boolean;
  todayStatus: CheckinResult["status"] | null;
  todayReward: number | null;
  /** 最近一次已知余额 */
  latestBalance: number | null;
  currency: string | null;
  /** 累计成功领取次数 */
  totalCheckins: number;
  /** 累计领取奖励总量（仅统计有 reward 值的记录） */
  totalReward: number;
  /** 连续签到天数 */
  streak: number;
  /** 历史最长连续签到天数 */
  bestStreak: number;
  lastRun: string | null;
  lastMessage: string | null;
  // ---- 新增统计维度 ----
  /** 本周领取奖励（周一为一周起点） */
  weekReward: number;
  /** 本月领取奖励 */
  monthReward: number;
  /** 运行总次数（含失败/跳过） */
  totalRuns: number;
  /** 失败次数 */
  failedRuns: number;
  /** 跳过次数（未配置凭据/已禁用等原因，不参与成功率分母） */
  skippedRuns: number;
  /** 成功率（success+already / 有效运行，0~100） */
  successRate: number;
  /** 平均耗时（毫秒） */
  avgDurationMs: number;
  /** 有效耗时样本总和（供总量加权平均用，面板不展示） */
  durationMsSum: number;
  /** 有效耗时样本数（durationMs > 0 的记录数） */
  durationMsCount: number;
  /** 最近一次耗时（毫秒） */
  lastDurationMs: number | null;
  /** 最近一次失败截图相对路径 */
  lastScreenshot: string | null;
}

export interface StatsSummary {
  sites: SiteStats[];
  totals: {
    totalCheckins: number;
    totalReward: number;
    weekReward: number;
    monthReward: number;
    sitesCheckedInToday: number;
    sitesConfigured: number;
    totalRuns: number;
    failedRuns: number;
    successRate: number;
    avgDurationMs: number;
  };
  generatedAt: string;
}

/** 每日趋势的一个点：某天各站奖励与总额 */
export interface TrendPoint {
  date: string; // YYYY-MM-DD
  total: number;
  perSite: Record<string, number>;
}

/**
 * 最近 N 天的每日奖励趋势（用于折线图）。
 * 缺失的日期补 0，保证 X 轴连续。
 */
export function computeTrend(siteIds: string[], days = 30): TrendPoint[] {
  const records = read().records;
  // 第 91 轮：趋势只覆盖最近 days 天，窗口外的历史记录不必解析日期、不必建桶。
  // 记录库最多 2000 条，但面板每 15 秒轮询一次，省下的仍是逐条 Date + 字符串分配。
  const cursor = new Date();
  cursor.setDate(cursor.getDate() - (days - 1));
  const cutoffDay = localDayOf(cursor);
  const byDay = new Map<string, Record<string, number>>();
  for (const r of records) {
    if (typeof r.reward !== "number" || r.reward === 0) continue;
    const day = localDayOf(new Date(r.timestamp));
    // 本地日期是 YYYY-MM-DD 定长格式，字符串比较即日期先后
    if (day < cutoffDay) continue;
    if (!byDay.has(day)) byDay.set(day, {});
    const bucket = byDay.get(day)!;
    bucket[r.siteId] = (bucket[r.siteId] ?? 0) + r.reward;
  }

  const points: TrendPoint[] = [];
  for (let i = 0; i < days; i++) {
    const day = localDayOf(cursor);
    const bucket = byDay.get(day) ?? {};
    const perSite: Record<string, number> = {};
    let total = 0;
    for (const id of siteIds) {
      const v = Number((bucket[id] ?? 0).toFixed(4));
      perSite[id] = v;
      total += v;
    }
    points.push({ date: day, total: Number(total.toFixed(4)), perSite });
    cursor.setDate(cursor.getDate() + 1);
  }
  return points;
}

/** 从当前往前推的连续签到天数 */
function computeStreak(days: Set<string>): number {
  if (days.size === 0) return 0;
  let streak = 0;
  const cursor = new Date();
  // 若今天还没签，从昨天开始算也不断
  if (!days.has(localDayOf(cursor))) {
    cursor.setDate(cursor.getDate() - 1);
  }
  while (days.has(localDayOf(cursor))) {
    streak++;
    cursor.setDate(cursor.getDate() - 1);
  }
  return streak;
}

/** 历史最长连续签到天数 */
function computeBestStreak(days: Set<string>): number {
  if (days.size === 0) return 0;
  const sorted = [...days].sort();
  let best = 1;
  let cur = 1;
  for (let i = 1; i < sorted.length; i++) {
    const prev = new Date(sorted[i - 1]);
    prev.setDate(prev.getDate() + 1);
    if (localDayOf(prev) === sorted[i]) {
      cur++;
      best = Math.max(best, cur);
    } else {
      cur = 1;
    }
  }
  return best;
}

/** 单站统计的纯计算部分：records 已按站点过滤 */
function summarizeSite(siteId: string, siteName: string, records: CheckinResult[]): SiteStats {
  const today = todayStr();
  const thisWeek = weekStartOf(new Date());
  const thisMonth = monthKeyOf(new Date());
  const successDays = new Set<string>();
  const todayRecords: CheckinResult[] = [];

  let totalCheckins = 0;
  let totalReward = 0;
  let weekReward = 0;
  let monthReward = 0;
  let latestBalance: number | null = null;
  let currency: string | null = null;
  let totalRuns = 0;
  let failedRuns = 0;
  let skippedRuns = 0;
  let effectiveRuns = 0; // 非跳过的运行
  let successRuns = 0;
  let durationSum = 0;
  let durationCount = 0;

  for (const r of records) {
    const when = new Date(r.timestamp);
    const day = localDayOf(when);
    totalRuns++;
    if (r.status !== "skipped") {
      effectiveRuns++;
      if (r.status === "success" || r.status === "already") successRuns++;
      if (r.status === "failed") failedRuns++;
    } else {
      skippedRuns++;
    }
    if (typeof r.durationMs === "number" && r.durationMs > 0) {
      durationSum += r.durationMs;
      durationCount++;
    }
    const isSuccess = r.status === "success" || r.status === "already";
    if (isSuccess) {
      successDays.add(day);
      if (r.status === "success") totalCheckins++;
    }
    if (typeof r.reward === "number") {
      totalReward += r.reward;
      if (weekStartOf(when) === thisWeek) weekReward += r.reward;
      if (monthKeyOf(when) === thisMonth) monthReward += r.reward;
    }
    if (typeof r.balance === "number") latestBalance = r.balance;
    if (r.currency) currency = r.currency;
    if (day === today) todayRecords.push(r);
  }

  const lastToday = todayRecords[todayRecords.length - 1] ?? null;
  // 今日奖励取当天最后一条"有奖励值"的记录，避免被之后的跳过/失败记录覆盖成 —
  //
  // 第 98 轮：旧写法 `[...todayRecords].reverse()` / `[...records].reverse()` 各复制
  // 一份全量数组再反向遍历；这里直接从尾部找，找到即停，零额外分配。
  let todayRewardRec: CheckinResult | null = null;
  for (let i = todayRecords.length - 1; i >= 0; i--) {
    if (typeof todayRecords[i].reward === "number") {
      todayRewardRec = todayRecords[i];
      break;
    }
  }
  const last = records[records.length - 1] ?? null;
  let lastFailed: CheckinResult | null = null;
  for (let i = records.length - 1; i >= 0; i--) {
    if (records[i].screenshot) {
      lastFailed = records[i];
      break;
    }
  }
  const checkedInToday = todayRecords.some(
    (r) => r.status === "success" || r.status === "already",
  );

  return {
    siteId,
    siteName,
    checkedInToday,
    todayStatus: lastToday?.status ?? null,
    todayReward: todayRewardRec?.reward ?? null,
    latestBalance,
    currency,
    totalCheckins,
    totalReward: Number(totalReward.toFixed(4)),
    streak: computeStreak(successDays),
    bestStreak: computeBestStreak(successDays),
    lastRun: last?.timestamp ?? null,
    lastMessage: last?.message ?? null,
    weekReward: Number(weekReward.toFixed(4)),
    monthReward: Number(monthReward.toFixed(4)),
    totalRuns,
    failedRuns,
    skippedRuns,
    successRate: effectiveRuns > 0 ? Math.round((successRuns / effectiveRuns) * 100) : 0,
    avgDurationMs: durationCount > 0 ? Math.round(durationSum / durationCount) : 0,
    durationMsSum: durationSum,
    durationMsCount: durationCount,
    lastDurationMs: last?.durationMs ?? null,
    lastScreenshot: lastFailed?.screenshot ?? null,
  };
}

export function computeSiteStats(siteId: string, siteName: string): SiteStats {
  const records = read().records.filter((r) => r.siteId === siteId);
  return summarizeSite(siteId, siteName, records);
}

/**
 * 第 78 轮：一次遍历把记录分到各站，避免「每站都全量扫一遍」。
 *
 * 旧实现里 `/api/stats` 对 3 个站点各调一次 `computeSiteStats`，每站都
 * `records.filter(...size)` 全量过滤一遍（3×N 次比较），再各自遍历统计。
 * 现在按站点分桶一次扫完，单条记录只在本站桶里被统计一次。
 */
export function computeSiteStatsBatch(
  sites: Array<{ siteId: string; siteName: string }>,
): SiteStats[] {
  const grouped = new Map<string, CheckinResult[]>();
  for (const s of sites) grouped.set(s.siteId, []);
  for (const r of read().records) {
    const bucket = grouped.get(r.siteId);
    if (bucket) bucket.push(r);
  }
  return sites.map((s) =>
    summarizeSite(s.siteId, s.siteName, grouped.get(s.siteId) ?? []),
  );
}
