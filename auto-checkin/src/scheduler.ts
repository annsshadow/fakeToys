// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import cron from "node-cron";
import type { AppConfig } from "./types.js";
import { runAll } from "./runner.js";
import { notifyResults } from "./notify.js";
import { createLogger } from "./logger.js";
import { shouldFailRun } from "./run-status.js";
import { setNextRunAt } from "./server.js";

const log = createLogger("scheduler");

/** 把 cron 的一个字段编译成谓词：命中返回 true，表达式非法返回 null。
 *
 * 第 77 轮：旧实现只认 `*` 与单个数字，步进（星斜杠 + 数字）、列表
 * （`1,15`）、区间（`9-18`）一律回退成「暂不能估算具体时刻」——而这些正是
 * 面板上最常见的写法。现在按 cron 语义解析：列表逐项、区间取闭区间、
 * 步进从区间下界（`*` 时取该字段最小合法值，如日字段的 1）起步长 n。
 *
 * 周字段允许 7 代表周日（本机 `getDay()` 是 0），由调用方额外测 `+7`。
 */
function compileField(
  field: string,
  min: number,
  max: number,
): ((value: number) => boolean) | null {
  if (field === "*") return () => true;

  const predicates: Array<(value: number) => boolean> = [];
  for (const chunk of field.split(",")) {
    const slash = chunk.split("/");
    if (slash.length > 2) return null;
    const [range, stepRaw] = slash;
    let step = 1;
    if (stepRaw !== undefined) {
      step = Number(stepRaw);
      if (!Number.isInteger(step) || step <= 0) return null;
    }

    let lo: number;
    let hi: number;
    if (range === "*") {
      lo = min;
      hi = max;
    } else if (range.includes("-")) {
      const [a, b] = range.split("-");
      lo = Number(a);
      hi = Number(b);
      if (!Number.isInteger(lo) || !Number.isInteger(hi)) return null;
      if (lo < min || hi > max || lo > hi) return null;
    } else {
      lo = Number(range);
      hi = lo;
      if (!Number.isInteger(lo) || lo < min || lo > max) return null;
    }

    predicates.push(
      (value) => value >= lo && value <= hi && (value - lo) % step === 0,
    );
  }
  return (value) => predicates.some((p) => p(value));
}

/** 粗略估算下一次 cron 触发时间（仅用于面板展示）
 *
 * 返回 `{ at, note }`：能估算时 at 是 ISO 时间；表达式无法解析或一年内无
 * 匹配时刻时 at 为 null、note 是回退说明。曾经的实现直接返回 null，面板
 * 于是显示「空闲待命」——对一条**已排定**的 cron 来说那是假话（2026-10-24 修）。
 */
export function estimateNext(expr: string): { at: string | null; note: string | null } {
  // 第 58 轮：完整 5 字段匹配（分 时 日 月 周），扫描范围扩到未来 366 天。
  // 第 77 轮：字段解析支持步进/列表/区间（见 compileField）。
  // 日与周是 OR 语义（cron 规定两者都非 * 时任一命中即可）。
  const parts = expr.trim().split(/\s+/);
  if (parts.length < 5) return { at: null, note: null };
  const [min, hour, dom, month, dow] = parts;
  const mPred = compileField(min, 0, 59);
  const hPred = compileField(hour, 0, 23);
  const domPred = compileField(dom, 1, 31);
  const monthPred = compileField(month, 1, 12);
  const dowPred = compileField(dow, 0, 7);
  if (!mPred || !hPred || !domPred || !monthPred || !dowPred) {
    return { at: null, note: `cron "${expr}"（表达式无法解析，任务已排定）` };
  }
  const now = new Date();
  for (let i = 1; i <= 366 * 24 * 60; i++) {
    const t = new Date(now.getTime() + i * 60000);
    const dowIndex = t.getDay();
    const dowOk = dowPred(dowIndex) || dowPred(dowIndex + 7);
    let dayOk: boolean;
    if (dom === "*" && dow === "*") {
      dayOk = true;
    } else if (dom !== "*" && dow === "*") {
      dayOk = domPred(t.getDate());
    } else if (dom === "*" && dow !== "*") {
      dayOk = dowOk;
    } else {
      // 两者都非 *：cron 语义是 OR
      dayOk = domPred(t.getDate()) || dowOk;
    }
    if (mPred(t.getMinutes()) && hPred(t.getHours()) && dayOk && monthPred(t.getMonth() + 1)) {
      return { at: t.toISOString(), note: null };
    }
  }
  return { at: null, note: `cron "${expr}"（未来一年内没有匹配时刻，任务已排定）` };
}

export function startScheduler(config: AppConfig): void {
  const { cron: expr, timezone, runOnStartup } = config.schedule;

  if (!cron.validate(expr)) {
    log.error(`无效的 cron 表达式：${expr}，定时任务未启动`);
    return;
  }

  cron.schedule(
    expr,
    () => {
      log.info("定时触发：开始全部站点签到");
      runAll(config)
        .then((results) => {
          if (shouldFailRun(results)) log.error("定时触发未完成或存在失败站点");
          const { at, note } = estimateNext(expr);
          setNextRunAt(at, note);
          // 第 61 轮：跑完一轮把结果推给 webhook（best-effort，失败不影响主流程）。
          void notifyResults(config, results);
        })
        .catch((e) => log.error(String(e)));
    },
    { timezone },
  );

  const first = estimateNext(expr);
  setNextRunAt(first.at, first.note);
  log.info(`定时任务已启动：cron="${expr}" 时区=${timezone}`);

  if (runOnStartup) {
    log.info("启动即运行一次…");
    runAll(config)
      .then((results) => {
        if (shouldFailRun(results)) log.error("启动即运行未完成或存在失败站点");
        void notifyResults(config, results);
      })
      .catch((e) => log.error(String(e)));
  }
}
