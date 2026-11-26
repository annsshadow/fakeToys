// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * H5「AI 风格视觉层」契约守卫。
 * 基线证据：docs/laiyipao/audits/baseline-round1/screenshots/h5-01/h5-03/h5-14
 * （非物理真机，桌面 Chrome 模拟 H5 视口）—— 19 页卡片为扁平深色块、层次弱，
 * t4 落地的 --glass-* 令牌全端零消费。本测试锁定改造后的最小视觉契约。
 * 规范：docs/laiyipao/design/AI_VISUAL_SYSTEM.md §2.1 / §2.3 / §4。
 */
import { describe, expect, it } from "vitest";
import { readFileSync, readdirSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";

const dir = dirname(fileURLToPath(import.meta.url));
const common = readFileSync(join(dir, "common.css"), "utf8");
const pagesDir = join(dir, "..", "pages");
const pageFiles = readdirSync(pagesDir, { withFileTypes: true })
  .filter((e) => e.isDirectory())
  .map((e) => join(pagesDir, e.name, e.name + ".vue"))
  .filter((p) => {
    try { readFileSync(p); return true; } catch { return false; }
  });

function block(src: string, selector: string): string {
  return src.match(new RegExp(selector.replace(/[.*+?^${}()|[\]\\]/g, "\\$&") + "\\s*\\{([^}]*)\\}"))?.[1] ?? "";
}

describe("H5 AI 风格视觉层", () => {
  it("V1 19 页统一挂 page-root，scoped 不覆盖全局卡片样式", () => {
    expect(pageFiles.length).toBe(19);
    for (const file of pageFiles) {
      const src = readFileSync(file, "utf8");
      expect(src, file).toMatch(/class="page-root/);
      expect(src, file).not.toMatch(/^\.card\s*\{|^\.card-title\s*\{/m);
    }
  });

  it("V2 common.css 声明视觉层版本与具名段落", () => {
    expect(common).toMatch(/--lyp-visual-version:\s*lyp-visual-v1/);
    expect(common).toMatch(/LYP-VISUAL-V1/);
  });

  it("V3 页面氛围层为静态低透明网格+双色辉光，不承载动画", () => {
    expect(common).toMatch(/\.page-root\s*\{[^}]*position:\s*relative/);
    const ambient = block(common, ".page-root::before");
    expect(ambient).toMatch(/pointer-events:\s*none/);
    expect(ambient).toMatch(/z-index:\s*-1/);
    expect(ambient).toMatch(/linear-gradient/);
    expect(ambient).toMatch(/radial-gradient/);
    expect(ambient).not.toMatch(/animation/);
    for (const m of ambient.matchAll(/rgba\([^)]*,\s*([\d.]+)\s*\)/g))
      expect(Number(m[1]), "氛围光透明度不得超过 0.10").toBeLessThanOrEqual(0.10);
  });

  it("V4 卡片升级为令牌渐变表面+内高光，标题保留橙色竖条并叠加辉光", () => {
    const card = block(common, ".page-root .card");
    expect(card).toMatch(/linear-gradient/);
    expect(card).toMatch(/inset 0 1rpx 0/);
    expect(card).toMatch(/var\(--panel\)/);
    const title = block(common, ".page-root .card-title");
    expect(title).toMatch(/linear-gradient/);
    expect(common).toMatch(/border-left:\s*5rpx solid var\(--fire\)/);
  });

  it("V5 真浮层消费 --glass-* 令牌（正文卡片禁止玻璃化）", () => {
    const battle = readFileSync(join(pagesDir, "battle", "battle.vue"), "utf8");
    expect(block(battle, ".legend-overlay")).toMatch(/var\(--glass-bg\)/);
    expect(block(battle, ".legend-overlay")).toMatch(/var\(--glass-blur\)/);
    expect(battle).toMatch(/var\(--glass-border\)/);
    expect(block(common, ".page-root .card")).not.toMatch(/backdrop-filter|--glass-bg/);
  });

  it("V7 章节页签横向可滚动，基线截图中被裁掉的章节仍可触达", () => {
    const stage = readFileSync(join(pagesDir, "stage", "stage.vue"), "utf8");
    expect(block(stage, ".chapter-row")).toMatch(/overflow-x:\s*auto/);
    expect(block(stage, ".chapter-row")).toMatch(/-webkit-overflow-scrolling:\s*touch/);
    expect(block(stage, ".chapter-chip")).toMatch(/flex:\s*0 0 auto/);
  });

  it("V6 清除陈旧 fallback 与低对比硬编码弱色", () => {
    for (const file of pageFiles) {
      const src = readFileSync(file, "utf8");
      expect(src, file).not.toMatch(/var\(--muted,\s*#9aa0a6\)/);
      expect(src, file).not.toMatch(/var\(--ok,\s*#3fb950\)/);
    }
    const battle = readFileSync(join(pagesDir, "battle", "battle.vue"), "utf8");
    expect(battle).not.toMatch(/style="[^"]*#8b949e/);
  });

  it("V8 globalStyle 四处背景色与 tokens.css --bg 同值，消除导航栏接缝", () => {
    const tokens = readFileSync(join(dir, "tokens.css"), "utf8");
    const bg = tokens.match(/--bg:\s*(#[0-9a-fA-F]{6})\s*;/)?.[1];
    expect(bg, "tokens.css 必须声明 --bg").toBeTruthy();
    const pages = JSON.parse(readFileSync(join(dir, "..", "pages.json"), "utf8"));
    const globalStyle = pages.globalStyle;
    for (const key of [
      "navigationBarBackgroundColor",
      "backgroundColor",
      "backgroundColorTop",
      "backgroundColorBottom",
    ]) {
      expect(globalStyle[key], `globalStyle.${key} 必须与 tokens --bg 同值`).toBe(bg);
    }
  });
});
