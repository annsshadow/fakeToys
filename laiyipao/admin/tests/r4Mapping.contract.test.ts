// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R4 剩余页面设计规范映射守卫（AI_VISUAL_SYSTEM.md §1C）。
 * 钉住十页 H5 + 四视图 admin 的组件映射、轻量 AI 规则与令牌兼容锚点。
 * 只校验规范文本与令牌真源；页面实现契约由各端 owner 补齐，不在本测试范围。
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
const start = doc.indexOf("## 1C.");
const end = doc.indexOf("## 2.");
const spec = start >= 0 && end > start ? doc.slice(start, end) : "";

const H5 = ["announcements", "bag", "battles", "enemies", "login", "me", "rank", "signin", "skins", "verify"];
const ADMIN = ["ContentView", "EconomyView", "DefensesView", "EquipmentView"];
const COMPONENTS = ["SectionKicker", "KpiStrip", "EmptyState", "DataGrid", "FilterBar", "ProgressTree"];

describe("R4 剩余页面设计规范映射", () => {
  it("规范存在且五个子条款齐全", () => {
    expect(start, "AI_VISUAL_SYSTEM.md 缺少 §1C").toBeGreaterThanOrEqual(0);
    for (const n of [1, 2, 3, 4, 5]) expect(spec, "缺少 §1C." + n).toContain("### 1C." + n);
  });

  it("十页 H5 与四视图 admin 全部有映射", () => {
    for (const p of H5) expect(spec, "H5 映射缺少 " + p).toContain(p + ".vue");
    for (const v of ADMIN) expect(spec, "admin 映射缺少 " + v).toContain(v + ".vue");
  });

  it("沿用 §1B 六组件与 AI 四判据", () => {
    for (const c of COMPONENTS) expect(spec, "缺少组件 " + c).toContain(c);
    for (const s of ["可见汉字「AI」", "真实数据结论", "依据", "CTA"])
      expect(spec, "缺少 AI 判据 " + s).toContain(s);
  });

  it("登录与验真有轻量、不打断流程的 AI 规则", () => {
    for (const p of ["login", "verify"]) {
      const i = spec.indexOf(p + ".vue");
      expect(i, "缺少 " + p).toBeGreaterThanOrEqual(0);
      const next = spec.indexOf("### 1C.4", i);
      const chunk = spec.slice(i, next < 0 ? spec.length : next);
      expect(chunk).toContain("轻量");
      expect(chunk).toContain("不打断");
    }
  });

  it("禁止低透明度底纹冒充 AI，并钉住 test-first", () => {
    expect(spec).toContain(".022");
    expect(spec).toContain(".06");
    expect(spec).toMatch(/禁止|不算/);
    expect(spec).toContain("test-first");
    expect(spec).toContain("不弱化既有断言");
  });

  it("令牌锚点保持 v1/v3 兼容且未引入 v4", () => {
    for (const css of [adminTokens, miniTokens]) {
      expect(css).toMatch(/--lyp-token-version:\s*lyp-tokens-v1/);
      expect(css).toMatch(/--lyp-r3-version:\s*lyp-tokens-v3-r3/);
      expect(css).not.toMatch(/lyp-tokens-v4/);
    }
  });
});
