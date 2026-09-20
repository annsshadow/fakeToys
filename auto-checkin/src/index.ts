import { loadConfig } from "./config.js";
import { startServer } from "./server.js";
import { startScheduler } from "./scheduler.js";
import { runAll, runSite } from "./runner.js";
import { createLogger } from "./logger.js";

const log = createLogger("main");

async function main(): Promise<void> {
  const config = loadConfig();
  const args = process.argv.slice(2);

  // 一次性模式：跑完即退出，供系统定时任务调用（不启动面板/定时器）
  if (args.includes("--once")) {
    log.info("一次性模式：运行全部站点后退出");
    await runAll(config);
    process.exit(0);
  }

  // 单站模式：--site <id>
  const siteIdx = args.indexOf("--site");
  if (siteIdx !== -1 && args[siteIdx + 1]) {
    const siteId = args[siteIdx + 1];
    log.info(`单站模式：${siteId}`);
    const r = await runSite(siteId, config);
    if (!r) log.error(`未知站点：${siteId}`);
    process.exit(0);
  }

  // 常驻模式：启动面板 + 定时器
  log.info("常驻模式启动");
  startServer(config);
  startScheduler(config);
}

main().catch((err) => {
  log.error(`启动失败：${(err as Error).stack || err}`);
  process.exit(1);
});
