import { appendFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";
import { ROOT_DIR } from "./config.js";

const LOG_DIR = join(ROOT_DIR, "logs");
mkdirSync(LOG_DIR, { recursive: true });

function ts(): string {
  return new Date().toISOString();
}

function write(level: string, scope: string, msg: string): void {
  const line = `${ts()} [${level}] [${scope}] ${msg}`;
  // 控制台
  if (level === "ERROR") console.error(line);
  else console.log(line);
  // 落盘（按天分文件）
  const day = new Date().toISOString().slice(0, 10);
  try {
    appendFileSync(join(LOG_DIR, `${day}.log`), line + "\n");
  } catch {
    // 落盘失败不影响运行
  }
}

export function createLogger(scope: string) {
  return {
    info: (msg: string) => write("INFO", scope, msg),
    warn: (msg: string) => write("WARN", scope, msg),
    error: (msg: string) => write("ERROR", scope, msg),
  };
}

export const log = createLogger("app");
