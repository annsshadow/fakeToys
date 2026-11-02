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

/** 粗略估算下一次 cron 触发时间（仅用于面板展示）
 *
 * 返回 `{ at, note }`：能估算时 at 是 ISO 时间；不能估算时（步进、列表、
 * 区间、名称——`Number()` 全是 NaN）at 为 null、note 是回退说明。曾经的实现
 * 直接返回 null，面板于是显示「空闲待命」——对一条**已排定**的 cron 来说
 * 那是假话（2026-10-24 修）。
 *
 * 注：注释里不出现 cron 星斜杠字面量——块注释会被它提前终结。
 */
/** 某字段是否匹配给定值：`*` 恒真，纯数字按相等；步进/列表/区间返回 null（无法判断）。 */
function fieldMatch(field: string, value: number, allow7As0 = false): boolean | null {
  if (field === "*") return true;
  const n = Number(field);
  if (Number.isNaN(n)) return null;
  if (allow7As0 && n === 7 && value === 0) return true;
  return n === value;
}

export function estimateNext(expr: string): { at: string | null; note: string | null } {
  // 第 58 轮：完整 5 字段匹配（分 时 日 月 周），扫描范围扩到未来 366 天。
  // 旧实现只匹配分/时——`0 9 1 * *`（每月 1 号 9 点）被估成明天 9 点，
  // 且只扫未来 24h，月/年任务永远「不能估算」。
  // 日与周是 OR 语义（cron 规定两者都非 * 时任一命中即可）。
  const parts = expr.trim().split(/\s+/);
  if (parts.length < 5) return { at: null, note: null };
  const [min, hour, dom, month, dow] = parts;
  const now = new Date();
  for (let i = 1; i <= 366 * 24 * 60; i++) {
    const t = new Date(now.getTime() + i * 60000);
    const mOk = fieldMatch(min, t.getMinutes());
    const hOk = fieldMatch(hour, t.getHours());
    const dowOk = fieldMatch(dow, t.getDay(), true);
    let dayOk: boolean | null;
    if (dom === "*" && dow === "*") {
      dayOk = true;
    } else if (dom !== "*" && dow === "*") {
      dayOk = fieldMatch(dom, t.getDate());
    } else if (dom === "*" && dow !== "*") {
      dayOk = dowOk;
    } else {
      // 两者都非 *：cron 语义是 OR
      const dOk = fieldMatch(dom, t.getDate());
      dayOk = (dOk === true || dowOk === true);
    }
    const monOk = fieldMatch(month, t.getMonth() + 1);
    // 任一字段是步进/列表/区间（null）→ 无法用数字匹配，回退说明
    if (mOk === null || hOk === null || dayOk === null || monOk === null || dowOk === null) {
      return { at: null, note: `cron "${expr}"（步进/列表/区间表达式暂不能估算具体时刻，任务已排定）` };
    }
    if (mOk && hOk && dayOk && monOk) return { at: t.toISOString(), note: null };
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
