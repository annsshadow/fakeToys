// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import type { CheckinResult } from "./types.js";

/** CLI/一次性入口应把基础设施或站点失败反映为非零退出码。 */
export function shouldFailRun(results: CheckinResult[]): boolean {
  return (
    results.length === 0 ||
    results.some(
      (result) =>
        result.status === "failed" ||
        (result.status === "skipped" && result.message !== "配置中已禁用"),
    )
  );
}
