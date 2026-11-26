// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * H5/miniapp 令牌与无障碍基建守卫。
 * admin/tests/designTokens.contract.test.ts 负责双端对齐；
 * 本测试确保 H5 端自身 vitest 也能独立拦住对比度/焦点环回退。
 */
import { describe, expect, it } from "vitest";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";

const dir = dirname(fileURLToPath(import.meta.url));
const css = readFileSync(join(dir, "tokens.css"), "utf8");
const common = readFileSync(join(dir, "common.css"), "utf8");
function token(name: string): string {
  const m = css.match(new RegExp(name + ":\\s*(#[0-9a-fA-F]{6})\\s*;"));
  if (!m) throw new Error("tokens.css 缺少 " + name);
  return m[1].toLowerCase();
}
function rgb(h: string) { return [1, 3, 5].map((i) => parseInt(h.slice(i, i + 2), 16) / 255); }
function lum(h: string) {
  const c = rgb(h).map((v) => (v <= 0.03928 ? v / 12.92 : Math.pow((v + 0.055) / 1.055, 2.4)));
  return 0.2126 * c[0] + 0.7152 * c[1] + 0.0722 * c[2];
}
function contrast(a: string, b: string) { const [hi, lo] = [lum(a), lum(b)].sort((x, y) => y - x); return (hi + 0.05) / (lo + 0.05); }

describe("H5 设计令牌与键盘可达性", () => {
  it("令牌版本标记为 lyp-tokens-v1", () => { expect(css).toMatch(/--lyp-token-version:\s*lyp-tokens-v1/); });
  it("文本四阶对四种表面 ≥4.5:1 且亮度递减", () => {
    const surfaces = ["--bg", "--panel", "--panel-2", "--panel-3"].map(token);
    for (const key of ["--text-2", "--muted", "--dim"])
      for (const bg of surfaces) expect(contrast(token(key), bg), key + " on " + bg).toBeGreaterThanOrEqual(4.5);
    const levels = ["--text", "--text-2", "--muted", "--dim"].map(token).map(lum);
    for (let i = 1; i < levels.length; i++) expect(levels[i - 1]).toBeGreaterThan(levels[i]);
  });
  it("键盘焦点环覆盖交互元素并带偏移", () => {
    const m = common.match(/:where\(a,\s*button,\s*input,\s*textarea,\s*select,\s*\[tabindex\]\):focus-visible\s*\{[^}]*\}/);
    expect(m?.[0] ?? "").toMatch(/outline:\s*3rpx\s+solid\s+var\(--[a-z0-9-]+\)/);
    expect(m?.[0] ?? "").toMatch(/outline-offset:\s*2rpx/);
  });
  it("系统减少动态效果时全局兜底关闭动画", () => {
    expect(common).toMatch(/@media\s*\(prefers-reduced-motion:\s*reduce\)[\s\S]*animation-duration:\s*0?\.01ms\s*!important/);
  });
});