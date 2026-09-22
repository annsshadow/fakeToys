// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync, writeFileSync, mkdirSync, existsSync } from "node:fs";
import { join } from "node:path";
import { ROOT_DIR } from "./config.js";
import type { CheckinResult } from "./types.js";

const DATA_DIR = join(ROOT_DIR, "data");
mkdirSync(DATA_DIR, { recursive: true });
const DB_PATH = join(DATA_DIR, "records.json");

interface Db {
  records: CheckinResult[];
}

function read(): Db {
  if (!existsSync(DB_PATH)) return { records: [] };
  try {
    return JSON.parse(readFileSync(DB_PATH, "utf-8")) as Db;
  } catch {
    return { records: [] };
  }
}

function persist(db: Db): void {
  writeFileSync(DB_PATH, JSON.stringify(db, null, 2));
}

export function addRecord(result: CheckinResult): void {
  const db = read();
  db.records.push(result);
  // 仅保留最近 2000 条，避免文件无限膨胀
  if (db.records.length > 2000) db.records = db.records.slice(-2000);
  persist(db);
}

export function getAllRecords(): CheckinResult[] {
  return read().records;
}

function localDay(iso: string): string {
  // 用本地时区判断"今天"
  const d = new Date(iso);
  const y = d.getFullYear();
  const m = String(d.getMonth() + 1).padStart(2, "0");
  const day = String(d.getDate()).padStart(2, "0");
  return `${y}-${m}-${day}`;
}

function todayStr(): string {
  return localDay(new Date().toISOString());
}

/** ISO 时间戳所在周的周一（本地时区），返回 YYYY-MM-DD */
function weekStart(d: Date): string {
  const t = new Date(d);
  const day = (t.getDay() + 6) % 7; // 周一=0
  t.setDate(t.getDate() - day);
  return localDay(t.toISOString());
}

/** ISO 时间戳所在月，返回 YYYY-MM */
function monthKey(iso: string): string {
  return localDay(iso).slice(0, 7);
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
  /** 成功率（success+already / 有效运行，0~100） */
  successRate: number;
  /** 平均耗时（毫秒） */
  avgDurationMs: number;
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
  const byDay = new Map<string, Record<string, number>>();
  for (const r of records) {
    if (typeof r.reward !== "number" || r.reward === 0) continue;
    const day = localDay(r.timestamp);
    if (!byDay.has(day)) byDay.set(day, {});
    const bucket = byDay.get(day)!;
    bucket[r.siteId] = (bucket[r.siteId] ?? 0) + r.reward;
  }

  const points: TrendPoint[] = [];
  const cursor = new Date();
  cursor.setDate(cursor.getDate() - (days - 1));
  for (let i = 0; i < days; i++) {
    const day = localDay(cursor.toISOString());
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
  if (!days.has(localDay(cursor.toISOString()))) {
    cursor.setDate(cursor.getDate() - 1);
  }
  while (days.has(localDay(cursor.toISOString()))) {
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
    if (localDay(prev.toISOString()) === sorted[i]) {
      cur++;
      best = Math.max(best, cur);
    } else {
      cur = 1;
    }
  }
  return best;
}

export function computeSiteStats(siteId: string, siteName: string): SiteStats {
  const records = read().records.filter((r) => r.siteId === siteId);
  const today = todayStr();
  const thisWeek = weekStart(new Date());
  const thisMonth = monthKey(new Date().toISOString());
  const successDays = new Set<string>();

  let totalCheckins = 0;
  let totalReward = 0;
  let weekReward = 0;
  let monthReward = 0;
  let latestBalance: number | null = null;
  let currency: string | null = null;
  let totalRuns = 0;
  let failedRuns = 0;
  let effectiveRuns = 0; // 非跳过的运行
  let successRuns = 0;
  let durationSum = 0;
  let durationCount = 0;

  for (const r of records) {
    totalRuns++;
    if (r.status !== "skipped") {
      effectiveRuns++;
      if (r.status === "success" || r.status === "already") successRuns++;
      if (r.status === "failed") failedRuns++;
    }
    if (typeof r.durationMs === "number" && r.durationMs > 0) {
      durationSum += r.durationMs;
      durationCount++;
    }
    const isSuccess = r.status === "success" || r.status === "already";
    if (isSuccess) {
      successDays.add(localDay(r.timestamp));
      if (r.status === "success") totalCheckins++;
    }
    if (typeof r.reward === "number") {
      totalReward += r.reward;
      if (weekStart(new Date(r.timestamp)) === thisWeek) weekReward += r.reward;
      if (monthKey(r.timestamp) === thisMonth) monthReward += r.reward;
    }
    if (typeof r.balance === "number") latestBalance = r.balance;
    if (r.currency) currency = r.currency;
  }

  const todayRecords = records.filter((r) => localDay(r.timestamp) === today);
  const lastToday = todayRecords[todayRecords.length - 1] ?? null;
  // 今日奖励取当天最后一条"有奖励值"的记录，避免被之后的跳过/失败记录覆盖成 —
  const todayRewardRec = [...todayRecords].reverse().find((r) => typeof r.reward === "number") ?? null;
  const last = records[records.length - 1] ?? null;
  const lastFailed = [...records].reverse().find((r) => r.screenshot) ?? null;
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
    successRate: effectiveRuns > 0 ? Math.round((successRuns / effectiveRuns) * 100) : 0,
    avgDurationMs: durationCount > 0 ? Math.round(durationSum / durationCount) : 0,
    lastDurationMs: last?.durationMs ?? null,
    lastScreenshot: lastFailed?.screenshot ?? null,
  };
}
