// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R7 玩法体验规范守卫（AI_VISUAL_SYSTEM.md §1F）。
 * 只守规范文本与双端令牌锚点；玩法 UI 与后端实现由对应任务负责。
 */
import { describe, expect, it } from "vitest";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";

const dir = dirname(fileURLToPath(import.meta.url));
const root = join(dir, "..", "..", "..");
const doc = readFileSync(join(root, "docs", "laiyipao", "design", "AI_VISUAL_SYSTEM.md"), "utf8");
const quick = readFileSync(join(root, "docs", "laiyipao", "design", "TOKENS_QUICK_REF.md"), "utf8");
const start = doc.indexOf("## 1F.");
const end = doc.indexOf("## 2.", start);
const spec = start >= 0 && end > start ? doc.slice(start, end) : "";
const css = [
  readFileSync(join(dir, "..", "src", "styles", "tokens.css"), "utf8"),
  readFileSync(join(root, "laiyipao", "miniapp", "src", "styles", "tokens.css"), "utf8"),
];

describe("R7 玩法体验规范", () => {
  it("§1F 五子条款与速查 §7D 齐全", () => {
    expect(start, "AI_VISUAL_SYSTEM.md 缺少 §1F").toBeGreaterThanOrEqual(0);
    for (const n of [1, 2, 3, 4, 5]) expect(spec, "缺少 §1F." + n).toContain("### 1F." + n);
    expect(quick).toContain("## 7D.");
  });

  it("三项缺口全部引用真浏览器证据与真实数字", () => {
    for (const k of ["round7-gameplay", "非物理真机", "r7-09-battle-result.png", "r7-10-battle-report.png", "r7-03-stage-select.png", "r7-11-skills.png", "r7-11-equipment.png"]) {
      expect(spec, "缺少证据 " + k).toContain(k);
    }
    for (const k of ["19726", "10909", "11941", "74s"]) expect(spec, "缺少实测数字 " + k).toContain(k);
  });

  it("战后复盘归因钉住权威字段与诚实空态", () => {
    for (const k of ["为什么没拿星", "clamp_note", "clamped", "settleResult.score", "settleResult.stars", "leaked", "reactions", "star_targets", "replay_hash", "得分超时长额度", "时长额度", "诚实空态", "不得重算", "30%"]) {
      expect(spec, "归因缺少 " + k).toContain(k);
    }
  });

  it("实时差额 HUD 钉住服务端门槛与不遮挡约束", () => {
    for (const k of ["战斗实时星级差额", "star_targets", "不得硬编码 11941", "max_score", "回合结束前", "非阻塞", "不遮挡塔位", "prefers-reduced-motion", "score_full_at_sec"]) {
      expect(spec, "HUD 缺少 " + k).toContain(k);
    }
  });

  it("推荐一键落地写清对比、确认与真实接口", () => {
    for (const k of ["/me/recommendations", "evidence[]", "expected_gain", "当前 vs 推荐", "确认替换", "取消", "禁止无确认", "本地规则引擎", "重炮弹", "母核碎片"]) {
      expect(spec, "落地流程缺少 " + k).toContain(k);
    }
  });

  it("AI 四判据与 R1-R6 不回退项保持硬下限", () => {
    for (const k of ["可见汉字「AI」", "真实数据结论", "依据", "CTA", "不宣称 GPT", "缺一不算", "R1-R6", "25% AoE", "顶栏脱敏"]) {
      expect(spec, "门禁缺少 " + k).toContain(k);
    }
  });

  it("双端令牌 v1/v3/r6 保留且 R7 不新增视觉令牌", () => {
    for (const s of css) {
      expect(s).toMatch(/--lyp-token-version:\s*lyp-tokens-v1/);
      expect(s).toMatch(/--lyp-r3-version:\s*lyp-tokens-v3-r3/);
      expect(s).toMatch(/--lyp-r6-version:\s*lyp-tokens-r6/);
      expect(s).not.toMatch(/lyp-tokens-v4/);
      expect(s).not.toMatch(/--(?:lyp-)?r7-[a-z-]+:/);
    }
    expect(spec).toContain("不新增视觉令牌");
  });
});
