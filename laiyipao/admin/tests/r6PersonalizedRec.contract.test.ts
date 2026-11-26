// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R6 真实数据驱动推荐规范守卫（AI_VISUAL_SYSTEM.md §1E）。
 * 只守规范文本与双端令牌锚点；接口/页面实现由后端与 UI 任务各自负责。
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
const start = doc.indexOf("## 1E.");
const end = doc.indexOf("## 2.", start);
const spec = start >= 0 && end > start ? doc.slice(start, end) : "";

const H5 = ["announcements", "bag", "battle", "battles", "enemies", "equipment", "gems", "index", "login", "mastery", "me", "rank", "reactions", "shop", "signin", "skills", "skins", "stage", "verify"];
const ADMIN = ["BattlesView", "ContentView", "DashboardView", "DefensesView", "EconomyView", "EquipmentView", "LevelsView", "LoginView", "RecomputesView", "SkillsView", "UsersView", "WikiView"];

describe("R6 真实数据驱动推荐规范", () => {
  it("§1E 存在且七个子条款齐全", () => {
    expect(start, "AI_VISUAL_SYSTEM.md 缺少 §1E").toBeGreaterThanOrEqual(0);
    for (const n of [1, 2, 3, 4, 5, 6, 7]) expect(spec, "缺少 §1E." + n).toContain("### 1E." + n);
  });

  it("独立审计覆盖 19 页 H5 与 12 视图 admin，并点名浅升级页", () => {
    for (const p of H5) expect(spec, "H5 审计缺少 " + p).toContain(p + ".vue");
    for (const v of ADMIN) expect(spec, "admin 审计缺少 " + v).toContain(v + ".vue");
    for (const k of ["浅升级", "已达标", "无 AI"]) expect(spec, "缺少审计状态 " + k).toContain(k);
    for (const k of ["login", "verify", "reactions", "skills", "equipment"]) expect(spec, "缺少重点页 " + k).toContain(k);
  });

  it("后端推荐接口契约写清路径、真实字段与诚实标注，且不实现", () => {
    for (const k of ["/api/v1/me/recommendations", "/api/v1/admin/recommendations/", "schema_version", "generated_at", "context", "evidence", "model", "confidence", "cta", "empty_reason", "NO_HISTORY", "不实现"]) {
      expect(spec, "接口契约缺少 " + k).toContain(k);
    }
    expect(spec).toMatch(/用户画像|战斗历史|配置版本|config_version|battle_count/);
  });

  it("玩法策略深度覆盖反应链、装备宝石搭配与关卡 BOSS 引导", () => {
    for (const k of ["元素反应链", "装备/宝石搭配", "BOSS", "难度曲线", "关卡"]) expect(spec, "缺少策略深度 " + k).toContain(k);
  });

  it("AI 四判据仍是硬下限，空态与反模式写死", () => {
    for (const k of ["可见汉字「AI」", "真实数据结论", "依据", "CTA", "不宣称 GPT", "缺一不算", "§1A.4", ".022", ".06", "Element Plus"]) {
      expect(spec, "缺少门禁 " + k).toContain(k);
    }
  });

  it("双端保留 v1/v3 并新增 r6 规范包标记，无 v4 误升", () => {
    for (const css of [adminTokens, miniTokens]) {
      expect(css).toMatch(/--lyp-token-version:\s*lyp-tokens-v1/);
      expect(css).toMatch(/--lyp-r3-version:\s*lyp-tokens-v3-r3/);
      expect(css).toMatch(/--lyp-r6-version:\s*lyp-tokens-r6/);
      expect(css).not.toMatch(/lyp-tokens-v4/);
      expect(css).toMatch(/--(?:lyp-)?ai-ink:/);
    }
    expect(spec).toContain("--lyp-r6-version");
  });

  it("写死 test-first、不弱化既有断言与非物理真机截图门禁", () => {
    for (const k of ["test-first", "不弱化既有断言", "非物理真机", "390×844", "1440×900"]) expect(spec, "缺少门禁 " + k).toContain(k);
  });
});
