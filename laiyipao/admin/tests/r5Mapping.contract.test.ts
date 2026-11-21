// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R5 剩余页面设计规范映射守卫（AI_VISUAL_SYSTEM.md §1D）。
 * 只守规范文本与双端令牌锚点；页面业务实现由 h5-ui / admin-ui 各自负责。
 */
import { describe, expect, it } from "vitest";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";

const dir = dirname(fileURLToPath(import.meta.url));
const root = join(dir, "..", "..", "..");
const doc = readFileSync(join(root, "docs", "laiyipao", "design", "AI_VISUAL_SYSTEM.md"), "utf8");
const adminTokens = readFileSync(join(dir, "..", "src", "styles", "tokens.css"), "utf8");
const miniTokens = readFileSync(join(root, "laiyipao", "miniapp", "src", "styles", "tokens.css"), "utf8");
const start = doc.indexOf("## 1D.");
const end = doc.indexOf("## 2.", start);
const spec = start >= 0 && end > start ? doc.slice(start, end) : "";

const H5 = ["announcements", "equipment", "index", "login", "reactions", "signin", "stage", "verify"];
const ADMIN = ["LevelsView", "SkillsView", "UsersView", "RecomputesView", "BattlesView"];
const COMPONENTS = ["SectionKicker", "KpiStrip", "FilterBar", "DataGrid", "EmptyState"];
const H5_TODO: Record<string, string[]> = {
  announcements: ["items", "groupedByMonth", "annFilters"],
  login: ["role=\"note\"", "不打断", "不新增"],
  signin: ["claimedCount", "cycleDays", "offlineReward", "tasks"],
  stage: ["chapterStars", "chapterTotal", "difficultyCurve", "terrainIndex"],
  verify: ["replayInfo", "expected_hash", "result.matched", "verifyHistory"],
};
const ADMIN_TODO: Record<string, string[]> = {
  SkillsView: ["skills", "recipes", "dpsPerHeat", "ELEMENT_LABEL"],
  UsersView: ["suspicious", "power>5000", "max_stage<=1", "users"],
  RecomputesView: ["summary.totals", "rate_permille", "breaker_rate_permille", "engine.status"],
  BattlesView: ["battleSummary", "winRate", "mismatched", "suspicious"],
};

function sub(n: number): string {
  const s = spec.indexOf("### 1D." + n);
  if (s < 0) return "";
  const e = spec.indexOf("### 1D.", s + 5);
  return spec.slice(s, e < 0 ? spec.length : e);
}
function row(text: string, anchor: string): string {
  const i = text.indexOf(anchor);
  if (i < 0) return "";
  const a = text.lastIndexOf("\n", i) + 1;
  const b = text.indexOf("\n", i);
  return text.slice(a, b < 0 ? text.length : b);
}

describe("R5 剩余页面设计规范映射", () => {
  it("规范存在且六个子条款齐全", () => {
    expect(start, "AI_VISUAL_SYSTEM.md 缺少 §1D").toBeGreaterThanOrEqual(0);
    for (const n of [1, 2, 3, 4, 5, 6]) expect(spec, "缺少 §1D." + n).toContain("### 1D." + n);
  });

  it("八页 H5 与五视图 admin 均在范围内且区分已达标与待升级", () => {
    for (const p of H5) expect(spec, "H5 范围缺少 " + p).toContain(p + ".vue");
    for (const v of ADMIN) expect(spec, "admin 范围缺少 " + v).toContain(v + ".vue");
    expect(spec).toContain("已达标");
    expect(spec).toContain("待升级");
  });

  it("五页 H5 待升级映射逐页写清 AI 结论来源与 CTA", () => {
    const s = sub(3);
    for (const [page, fields] of Object.entries(H5_TODO)) {
      const r = row(s, page + ".vue");
      expect(r, "§1D.3 缺少 " + page + " 映射行").not.toBe("");
      expect(r, page + " 缺少 AI 结论来源").toContain("AI 结论");
      expect(r, page + " 缺少 CTA").toContain("CTA");
      for (const f of fields) expect(r, page + " 缺少真实数据字段 " + f).toContain(f);
    }
  });

  it("四视图 admin 待升级映射逐页写清真实数据与 CTA，Levels 标记已达标", () => {
    const s = sub(4);
    for (const [page, fields] of Object.entries(ADMIN_TODO)) {
      const r = row(s, page + ".vue");
      expect(r, "§1D.4 缺少 " + page + " 映射行").not.toBe("");
      expect(r, page + " 缺少 AI 结论来源").toContain("AI 结论");
      expect(r, page + " 缺少 CTA").toContain("CTA");
      for (const f of fields) expect(r, page + " 缺少真实数据字段 " + f).toContain(f);
    }
    expect(row(s, "LevelsView.vue")).toContain("已达标");
  });

  it("AI 四判据写死为下限，六组件齐全且禁止低透明度冒充", () => {
    for (const k of ["可见汉字「AI」", "真实数据结论", "依据", "CTA", "缺一不算", "不宣称 GPT"]) expect(spec, "缺少下限 " + k).toContain(k);
    for (const c of COMPONENTS) expect(spec, "缺少组件 " + c).toContain(c);
    expect(spec).toContain(".022");
    expect(spec).toContain(".06");
    expect(spec).toMatch(/不算|禁止/);
  });

  it("双端令牌保持 v1/v3 兼容锚点，R5 不新增视觉变量且无 v4 误升", () => {
    for (const css of [adminTokens, miniTokens]) {
      expect(css).toMatch(/--lyp-token-version:\s*lyp-tokens-v1/);
      expect(css).toMatch(/--lyp-r3-version:\s*lyp-tokens-v3-r3/);
      expect(css).not.toMatch(/lyp-tokens-v4/);
      expect(css).toMatch(/--(?:lyp-)?ai-ink:/);
    }
    expect(spec).toContain("不新增视觉令牌");
    expect(spec).toContain("--ai-ink");
    expect(spec).toContain("--lyp-ai-ink");
  });

  it("写死 test-first 与不弱化既有断言，截图标注非物理真机", () => {
    expect(spec).toContain("test-first");
    expect(spec).toContain("不弱化既有断言");
    expect(spec).toContain("非物理真机");
  });
});
