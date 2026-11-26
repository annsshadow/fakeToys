// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/** R10 设计规范守卫（AI_VISUAL_SYSTEM.md §1I）。只守规范文本与令牌锚点。 */
import { describe, expect, it } from "vitest";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";
const dir = dirname(fileURLToPath(import.meta.url));
const root = join(dir, "..", "..", "..");
const doc = readFileSync(join(root, "docs", "laiyipao", "design", "AI_VISUAL_SYSTEM.md"), "utf8");
const quick = readFileSync(join(root, "docs", "laiyipao", "design", "TOKENS_QUICK_REF.md"), "utf8");
const start = doc.indexOf("## 1I.");
const end = doc.indexOf("## 2.", start);
const spec = start >= 0 && end > start ? doc.slice(start, end) : "";
const css = [
  readFileSync(join(dir, "..", "src", "styles", "tokens.css"), "utf8"),
  readFileSync(join(root, "laiyipao", "miniapp", "src", "styles", "tokens.css"), "utf8"),
];
describe("R10 养成因果链与收益可解释规范", () => {
  it("§1I 五子条款与速查 §7G 齐全", () => {
    expect(start, "AI_VISUAL_SYSTEM.md 缺少 §1I").toBeGreaterThanOrEqual(0);
    for (const n of [1, 2, 3, 4, 5]) expect(spec, "缺少 §1I." + n).toContain("### 1I." + n);
    expect(quick).toContain("## 7G.");
  });
  it("依据锚定 R10 真浏览器审计与真实数字", () => {
    for (const k of ["round10-gameplay", "非物理真机", "10614", "12088", "r10-05-upgraded-battle-result.png", "r10-07-gems-bag-cta-visible.png"]) {
      expect(spec, "R10 依据缺少 " + k).toContain(k);
    }
  });
  it("推荐 CTA 必须参数可达且客户端不得伪造", () => {
    for (const k of ["candidate", "replace", "params", "cta", "客户端不得伪造", "禁止下发无参数 CTA", "诚实空态"]) {
      expect(spec, "CTA 可达缺少 " + k).toContain(k);
    }
  });
  it("养成前后因果视图必须来自两次真实结算", () => {
    for (const k of ["养成前", "养成后", "同一关", "可信差值", "两次真实结算", "禁止编造", "估算", "hash"]) {
      expect(spec, "养成因果缺少 " + k).toContain(k);
    }
  });
  it("宝石收集-镶嵌-收益三段打通且不虚构收益", () => {
    for (const k of ["去背包镶嵌", "/pages/bag/bag", "收益", "不虚构", "属性差分", "最短路径"]) {
      expect(spec, "宝石三段缺少 " + k).toContain(k);
    }
  });
  it("AI 四判据为硬下限且低透明度底纹不算", () => {
    for (const k of ["可见汉字「AI」", "真实数据结论", "依据", "CTA", "缺一不算", "不宣称 GPT"]) {
      expect(spec, "AI 判据缺少 " + k).toContain(k);
    }
  });
  it("保留 v1/v3-r3/r6 锚点且 R10 不新增视觉令牌", () => {
    for (const s of css) {
      expect(s).toMatch(/--lyp-token-version:\s*lyp-tokens-v1/);
      expect(s).toMatch(/--lyp-r3-version:\s*lyp-tokens-v3-r3/);
      expect(s).toMatch(/--lyp-r6-version:\s*lyp-tokens-r6/);
      expect(s).not.toMatch(/lyp-tokens-v4/);
      expect(s).not.toMatch(/--(?:lyp-)?r10-[a-z-]+:/);
    }
    expect(spec).toContain("不新增视觉令牌");
  });
});
