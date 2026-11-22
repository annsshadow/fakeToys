// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/** R9 战斗反馈与选卡因果规范守卫（AI_VISUAL_SYSTEM.md §1H）。只守规范文本与令牌锚点。 */
import { describe, expect, it } from "vitest";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";
const dir = dirname(fileURLToPath(import.meta.url));
const root = join(dir, "..", "..", "..");
const doc = readFileSync(join(root, "docs", "laiyipao", "design", "AI_VISUAL_SYSTEM.md"), "utf8");
const quick = readFileSync(join(root, "docs", "laiyipao", "design", "TOKENS_QUICK_REF.md"), "utf8");
const start = doc.indexOf("## 1H.");
const end = doc.indexOf("## 2.", start);
const spec = start >= 0 && end > start ? doc.slice(start, end) : "";
const css = [
  readFileSync(join(dir, "..", "src", "styles", "tokens.css"), "utf8"),
  readFileSync(join(root, "laiyipao", "miniapp", "src", "styles", "tokens.css"), "utf8"),
];
describe("R9 战斗反馈与选卡因果规范", () => {
  it("§1H 五子条款与速查 §7F 齐全", () => {
    expect(start, "AI_VISUAL_SYSTEM.md 缺少 §1H").toBeGreaterThanOrEqual(0);
    for (const n of [1, 2, 3, 4, 5]) expect(spec, "缺少 §1H." + n).toContain("### 1H." + n);
    expect(quick).toContain("## 7F.");
  });
  it("依据锚定 R9 真浏览器审计与真实数字", () => {
    for (const k of ["round9-gameplay", "非物理真机", "C/C/B", "19684→8992", "11521", "r9-06-reaction-feed.png"]) {
      expect(spec, "R9 依据缺少 " + k).toContain(k);
    }
  });
  it("反应因果链可追溯且钉住真实事件字段", () => {
    for (const k of ["reaction", "name", "damage", "触发元素", "抗性削减", "为什么这一击痛", "recentReactions", "REACTIONS", "不遮挡"]) {
      expect(spec, "反应因果缺少 " + k).toContain(k);
    }
  });
  it("选卡取舍反馈可归因，禁止数值碾压", () => {
    for (const k of ["card_picks", "取卡", "弃卡", "跳过卡牌", "伤害占比", "结算", "不做数值碾压", "诚实空态"]) {
      expect(spec, "选卡反馈缺少 " + k).toContain(k);
    }
  });
  it("钳分与星级预警统一实时换算口径", () => {
    for (const k of ["上限→实得→差额", "钳分中·距满分 Xs", "免钳分", "liveStarGap", "score_full_at_sec", "star_targets", "禁止各说各话", "保分窗口"]) {
      expect(spec, "实时换算缺少 " + k).toContain(k);
    }
  });
  it("AI 四判据为硬下限且低透明度底纹不算", () => {
    for (const k of ["可见汉字「AI」", "真实数据结论", "依据", "CTA", "缺一不算", "不宣称 GPT"]) {
      expect(spec, "AI 判据缺少 " + k).toContain(k);
    }
  });
  it("保留 v1/v3-r3/r6/v1E-v1F 锚点且 R9 不新增视觉令牌", () => {
    for (const s of css) {
      expect(s).toMatch(/--lyp-token-version:\s*lyp-tokens-v1/);
      expect(s).toMatch(/--lyp-r3-version:\s*lyp-tokens-v3-r3/);
      expect(s).toMatch(/--lyp-r6-version:\s*lyp-tokens-r6/);
      expect(s).not.toMatch(/lyp-tokens-v4/);
      expect(s).not.toMatch(/--(?:lyp-)?r9-[a-z-]+:/);
    }
    expect(spec).toContain("不新增视觉令牌");
  });
});
