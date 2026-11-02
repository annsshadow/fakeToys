// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/** 第 63 轮：license header 门禁。

 * 扫描 git 跟踪的源码文件（ts/tsx/js/mjs/cjs/go/rs/py），缺
 * `SPDX-License-Identifier` 即点名失败。排除第三方/生成物目录。
 * 独立可跑（node scripts/check-license-headers.mjs），也作为
 * check-warnings.mjs 的 `license` 检查项。
 */

import { execSync } from "node:child_process";
import { readFileSync } from "node:fs";

const EXCLUDE = /node_modules|\/dist\/|target\/|vendor\/|\.generated\.|\.gen\.|components\.d\.ts$/;

function main() {
  const out = execSync('git ls-files | grep -E "\\.(ts|tsx|js|mjs|cjs|go|rs|py)$"', {
    encoding: "utf8",
  });
  const files = out
    .split("\n")
    .filter(Boolean)
    .filter((f) => !EXCLUDE.test(f));

  const missing = [];
  for (const f of files) {
    let text;
    try {
      text = readFileSync(f, "utf8");
    } catch {
      continue; // 二进制同名等读不出来的跳过
    }
    if (!text.slice(0, 600).includes("SPDX-License-Identifier")) missing.push(f);
  }

  console.log(`license-headers: ${files.length} 个源码文件，缺失 ${missing.length}`);
  if (missing.length > 0) {
    for (const f of missing) console.log(`    缺 header：${f}`);
    process.exit(1);
  }
}

main();
