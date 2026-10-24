// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import cron from "node-cron";
import type { AppConfig } from "./types.js";
import { runAll } from "./runner.js";
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
export function estimateNext(expr: string): { at: string | null; note: string | null } {
  // node-cron 不直接暴露下次时间，这里给个近似：找未来 24h 内首个匹配的整分钟
  const parts = expr.trim().split(/\s+/);
  if (parts.length < 5) return { at: null, note: null };
  const [min, hour] = parts;
  const now = new Date();
  for (let i = 1; i <= 60 * 24; i++) {
    const t = new Date(now.getTime() + i * 60000);
    const mOk = min === "*" || Number(min) === t.getMinutes();
    const hOk = hour === "*" || Number(hour) === t.getHours();
    if (mOk && hOk) return { at: t.toISOString(), note: null };
  }
  return { at: null, note: `cron "${expr}"（步进/列表/区间表达式暂不能估算具体时刻，任务已排定）` };
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
      })
      .catch((e) => log.error(String(e)));
  }
}
