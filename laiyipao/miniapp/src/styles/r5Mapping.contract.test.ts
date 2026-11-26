// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * H5 侧 R5 映射守卫：独立确认 §1D 的五页待升级映射、AI 四判据下限与 H5 令牌锚点。
 */
import { describe, expect, it } from "vitest";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";

const dir = dirname(fileURLToPath(import.meta.url));
const root = join(dir, "..", "..", "..", "..");
const doc = readFileSync(join(root, "docs", "laiyipao", "design", "AI_VISUAL_SYSTEM.md"), "utf8");
const css = readFileSync(join(dir, "tokens.css"), "utf8");
const start = doc.indexOf("## 1D.");
const end = doc.indexOf("## 2.", start);
const spec = start >= 0 && end > start ? doc.slice(start, end) : "";
const h5Spec = (() => {
  const s = spec.indexOf("### 1D.3");
  const e = spec.indexOf("### 1D.4", s);
  return s >= 0 && e > s ? spec.slice(s, e) : "";
})();

describe("H5 R5 剩余页面映射", () => {
  it("八页 H5 均在范围内且标注升级状态", () => {
    expect(start).toBeGreaterThanOrEqual(0);
    for (const p of ["announcements", "equipment", "index", "login", "reactions", "signin", "stage", "verify"])
      expect(spec, "缺少 " + p + ".vue").toContain(p + ".vue");
    expect(spec).toContain("已达标");
    expect(spec).toContain("待升级");
  });

  it("五页待升级 H5 映射含真实数据字段、AI 结论与 CTA", () => {
    for (const [page, fields] of Object.entries({
      announcements: ["items", "groupedByMonth"],
      login: ["role=\"note\"", "不打断"],
      signin: ["claimedCount", "offlineReward", "tasks"],
      stage: ["chapterStars", "difficultyCurve", "terrainIndex"],
      verify: ["replayInfo", "expected_hash", "result.matched"],
    })) {
      const i = h5Spec.indexOf(page + ".vue");
      expect(i, "§1D.3 缺少 " + page).toBeGreaterThanOrEqual(0);
      const line = h5Spec.slice(h5Spec.lastIndexOf("\n", i) + 1, h5Spec.indexOf("\n", i) < 0 ? h5Spec.length : h5Spec.indexOf("\n", i));
      expect(line, page + " 缺少 AI 结论").toContain("AI 结论");
      expect(line, page + " 缺少 CTA").toContain("CTA");
      for (const f of fields) expect(line, page + " 缺少 " + f).toContain(f);
    }
  });

  it("AI 四判据是硬下限，低透明度底纹不算改造", () => {
    for (const k of ["可见汉字「AI」", "真实数据结论", "依据", "CTA", "缺一不算"]) expect(spec, "缺少 " + k).toContain(k);
    expect(spec).toContain(".022");
    expect(spec).toContain(".06");
  });

  it("H5 令牌保持 v1/v3 双锚点且复用已有 AI 令牌", () => {
    expect(css).toMatch(/--lyp-token-version:\s*lyp-tokens-v1/);
    expect(css).toMatch(/--lyp-r3-version:\s*lyp-tokens-v3-r3/);
    expect(css).not.toMatch(/lyp-tokens-v4/);
    expect(css).toMatch(/--ai-ink:\s*#d7bfff/);
  });
});
