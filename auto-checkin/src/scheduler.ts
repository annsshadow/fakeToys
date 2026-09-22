import cron from "node-cron";
import type { AppConfig } from "./types.js";
import { runAll } from "./runner.js";
import { createLogger } from "./logger.js";
import { setNextRunAt } from "./server.js";

const log = createLogger("scheduler");

/** 粗略估算下一次 cron 触发时间（仅用于面板展示） */
function estimateNext(expr: string): string | null {
  // node-cron 不直接暴露下次时间，这里给个近似：找未来 24h 内首个匹配的整分钟
  const parts = expr.trim().split(/\s+/);
  if (parts.length < 5) return null;
  const [min, hour] = parts;
  const now = new Date();
  for (let i = 1; i <= 60 * 24; i++) {
    const t = new Date(now.getTime() + i * 60000);
    const mOk = min === "*" || Number(min) === t.getMinutes();
    const hOk = hour === "*" || Number(hour) === t.getHours();
    if (mOk && hOk) return t.toISOString();
  }
  return null;
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
        .then(() => setNextRunAt(estimateNext(expr)))
        .catch((e) => log.error(String(e)));
    },
    { timezone },
  );

  setNextRunAt(estimateNext(expr));
  log.info(`定时任务已启动：cron="${expr}" 时区=${timezone}`);

  if (runOnStartup) {
    log.info("启动即运行一次…");
    runAll(config).catch((e) => log.error(String(e)));
  }
}
