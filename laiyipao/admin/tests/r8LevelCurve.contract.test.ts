// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/** R8 设计规范守卫（AI_VISUAL_SYSTEM.md §1G）。只守规范文本与令牌锚点。 */
import { describe, expect, it } from "vitest";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";
const dir = dirname(fileURLToPath(import.meta.url));
const root = join(dir, "..", "..", "..");
const doc = readFileSync(join(root, "docs", "laiyipao", "design", "AI_VISUAL_SYSTEM.md"), "utf8");
const quick = readFileSync(join(root, "docs", "laiyipao", "design", "TOKENS_QUICK_REF.md"), "utf8");
const start = doc.indexOf("## 1G.");
const end = doc.indexOf("## 2.", start);
const spec = start >= 0 && end > start ? doc.slice(start, end) : "";
const css = [
  readFileSync(join(dir, "..", "src", "styles", "tokens.css"), "utf8"),
  readFileSync(join(root, "laiyipao", "miniapp", "src", "styles", "tokens.css"), "utf8"),
];
describe("R8 星级口径·机制拐点·时间预算规范", () => {
  it("§1G 五子条款与速查 §7E 齐全", () => {
    expect(start, "AI_VISUAL_SYSTEM.md 缺少 §1G").toBeGreaterThanOrEqual(0);
    for (const n of [1, 2, 3, 4, 5]) expect(spec, "缺少 §1G." + n).toContain("### 1G." + n);
    expect(quick).toContain("## 7E.");
  });
  it("星级口径钉住非单调实测与按本关满分解释", () => {
    for (const k of ["降 47 次", "降 44 次", "约 0.60", "非单调", "高关更容易", "按本关满分百分比给星", "max_score", "star_targets", "不得硬编码 11941", "11941"]) {
      expect(spec, "星级口径缺少 " + k).toContain(k);
    }
  });
  it("机制拐点标注钉住 boss 关并替换 r5AiStage 假洞察", () => {
    for (const k of ["56/60/91/95/96/100", "r5AiStage", "+33‰", "本章最大跳变", "新敌种首现", "地形变化", "boss", "飞行", "潜地", "远程"]) {
      expect(spec, "机制拐点缺少 " + k).toContain(k);
    }
  });
  it("时间预算 HUD 来自配置且临近截止即时提示", () => {
    for (const k of ["剩余保分窗口", "score_full_at_sec", "不得硬编码", "不遮挡", "临近截止", "即时提示", "钳分"]) {
      expect(spec, "时间预算缺少 " + k).toContain(k);
    }
  });
  it("AI 四判据为硬下限且低透明度底纹不算", () => {
    for (const k of ["可见汉字「AI」", "真实数据结论", "依据", "CTA", "低透明度底纹不算", "缺一不算", "不宣称 GPT"]) {
      expect(spec, "AI 判据缺少 " + k).toContain(k);
    }
  });
  it("双端令牌锚点保持 v1/v3-r3/r6 且 R8 不新增视觉令牌", () => {
    for (const s of css) {
      expect(s).toMatch(/--lyp-token-version:\s*lyp-tokens-v1/);
      expect(s).toMatch(/--lyp-r3-version:\s*lyp-tokens-v3-r3/);
      expect(s).toMatch(/--lyp-r6-version:\s*lyp-tokens-r6/);
      expect(s).not.toMatch(/lyp-tokens-v4/);
      expect(s).not.toMatch(/--(?:lyp-)?r8-[a-z-]+:/);
    }
    expect(spec).toContain("不新增视觉令牌");
  });
});
