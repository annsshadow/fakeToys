// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R93 规范：admin RecomputesView 表格页骨架屏一致性（RED）。
 * 权威口径 = PageSkeleton.vue + main.css .page > .page-skeleton ~ * + AI_VISUAL_SYSTEM.md §6:848。
 * 契约真实读源码：RecomputesView.vue 导入/插入骨架、移除表格 v-loading、保留竞态守卫与四态语义。
 */
import { describe, expect, it } from "vitest";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";

const root = join(dirname(fileURLToPath(import.meta.url)), "..");
const viewPath = join(root, "src", "views", "RecomputesView.vue");
const cssPath = join(root, "src", "styles", "tokens.css");
const view = readFileSync(viewPath, "utf8");
const tokens = readFileSync(cssPath, "utf8");
const lines = view.split(/\r?\n/);
const line = (needle: string) => lines.findIndex((l) => l.includes(needle));

describe("R93 RecomputesView 骨架屏一致性契约", () => {
  it("导入共享 PageSkeleton 组件", () => {
    expect(view).toMatch(/import\s+PageSkeleton\s+from\s+["']@\/components\/PageSkeleton\.vue["']/);
  });

  it("根节点 .page 之后插入 PageSkeleton v-if=loading", () => {
    const rootIdx = line('<div class="page">');
    expect(rootIdx, "缺少根节点 <div class=\"page\">").toBeGreaterThanOrEqual(0);
    const after = lines.slice(rootIdx + 1, rootIdx + 6).join("\n");
    expect(after).toContain('<PageSkeleton v-if="loading" />');
  });

  it("表格移除 v-loading=loading，但 stat-row 的 v-loading=statsLoading 保留", () => {
    expect(view).not.toContain('v-loading="loading"');
    expect(view).toContain('<div v-loading="statsLoading" class="stat-row">');
    const tableLine = lines.find((l) => l.includes('class="data-grid"') && l.includes(':data="rows"'));
    expect(tableLine, "缺少 data-grid 表格锚点").toBeDefined();
    expect(tableLine).not.toContain("v-loading");
  });

  it("两套竞态守卫与 Promise.all 刷新语义不回退", () => {
    expect(view).toContain("let loadSeq = 0");
    expect(view).toContain("let statsLoadSeq = 0");
    expect(view).toContain("if (seq !== loadSeq) return");
    expect(view).toContain("if (seq !== statsLoadSeq) return");
    expect(view).toContain("return Promise.all([load(), loadStats(), loadMode()])");
  });

  it("KPI / result-count / modeState / emptyText / AiInsightCard 不变量保留", () => {
    expect(view).toContain('class="kpi-strip"');
    expect(view).toContain("summary?.totals.total ?? '—'");
    expect(view).toContain("summary?.totals.mismatched ?? '—'");
    expect(view).toContain('class="result-count" aria-live="polite"');
    expect(view).toContain("async function switchMode(next: string)");
    expect(view).toContain("const emptyText = '没有匹配的重算记录'");
    expect(view).toContain('<el-pagination');
    expect(view).toContain("cappedTotal(total, LEDGER_PAGE_SIZE)");
    expect(view).toContain('<AiInsightCard :insight="aiInsight" :loading="statsLoading" />');
  });

  it("令牌三锚点保留 + 业务源码零 --r93- + 写域边界", () => {
    expect(tokens).toContain("--lyp-token-version: lyp-tokens-v1");
    expect(tokens).toContain("--lyp-r3-version: lyp-tokens-v3-r3");
    expect(tokens).toContain("--lyp-r6-version: lyp-tokens-r6");
    expect(view).not.toContain("--r93-");
    // 写域边界：spec/contract 为新增规范轮文件，业务改动只允许 RecomputesView.vue
    expect(viewPath.endsWith(join("src", "views", "RecomputesView.vue"))).toBe(true);
  });
});
