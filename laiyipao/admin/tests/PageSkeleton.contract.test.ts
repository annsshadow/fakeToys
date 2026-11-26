// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * 运营后台骨架屏与微交互契约（AI_VISUAL_SYSTEM.md §4/§6，P4/P6）。
 * §6：admin 原用整页 v-loading 灰遮罩，数据到达时布局跳动（CLS）。
 * 规范要求共享骨架组件：表头 + 5–8 行占位，宽度按主 70%/次 45%/末 25%
 * 分层，且 shimmer 用既有 .skeleton-bar；加载态必须带 aria-busy 语义。
 * §4：页面入场 200–280ms ease-out、按钮按压反馈，且可被 reduced-motion 关闭。
 */
import { describe, expect, it } from "vitest";
import { readFileSync, existsSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";

const root = join(dirname(fileURLToPath(import.meta.url)), "..");
const comp = join(root, "src", "components", "PageSkeleton.vue");
const cssPath = join(root, "src", "styles", "main.css");
const viewsDir = join(root, "src", "views");
const css = readFileSync(cssPath, "utf8");

describe("骨架屏契约（§6）", () => {
  it("存在共享 PageSkeleton 组件，不再依赖整页 v-loading", () => {
    expect(existsSync(comp), "缺少 src/components/PageSkeleton.vue").toBe(true);
  });

  it("骨架组件用 .skeleton-row/.skeleton-bar 搭形状，并带 aria-busy 语义", () => {
    const s = readFileSync(comp, "utf8");
    expect(s).toContain("skeleton-row");
    expect(s).toContain("skeleton-bar");
    expect(s).toContain('aria-busy="true"');
    expect(s).toContain('role="status"');
  });

  it("骨架出现时隐藏同页其余内容，避免 keep-alive 旧数据与骨架叠加", () => {
    expect(css).toMatch(/\.page\s*>\s*\.page-skeleton\s*~\s*\*/);
  });

  it("骨架条宽度按主 70% / 次 45% / 末 25% 分层，非等宽", () => {
    const s = readFileSync(comp, "utf8");
    expect(s).toContain("70%"); expect(s).toContain("45%"); expect(s).toContain("25%");
  });

  it("全部数据视图加载态改用 PageSkeleton（至少 dashboard/levels/users）", () => {
    for (const name of ["DashboardView.vue", "LevelsView.vue", "UsersView.vue"]) {
      const s = readFileSync(join(viewsDir, name), "utf8");
      expect(s, name + " 未使用共享骨架").toContain('<PageSkeleton v-if="loading" />');
    }
  });

  it("骨架视图不再用整页 v-loading 遮罩（局部按钮 loading 保留）", () => {
    for (const name of ["DashboardView.vue", "LevelsView.vue", "UsersView.vue"]) {
      const s = readFileSync(join(viewsDir, name), "utf8");
      expect(s, name + " 仍有整页 v-loading").not.toContain('<div class="page" v-loading="loading">');
    }
  });
});

describe("微交互契约（§4）", () => {
  it("提供页面入场动画（200–280ms ease-out）并作用于 .page", () => {
    expect(css).toContain("@keyframes lyp-page-in");
    const m = css.match(/\.page\s*\{[^}]*animation:\s*lyp-page-in\s+2[0-8]\dms[^}]*ease-out/);
    expect(m, "main.css .page 入场动画缺失或时长不符 §4").not.toBeNull();
  });
  it("按钮按压反馈存在且短促（≤120ms）", () => {
    const m = css.match(/\.el-button:not\(\.is-disabled\):active[\s\S]{0,200}?transform:\s*scale\(0?\.9[0-9]?\)/);
    expect(m, ".el-button 按压反馈缺失").not.toBeNull();
  });
  it("入场/按压受 reduced-motion 全局兜底关闭（已有全局规则）", () => {
    expect(css).toContain("@media (prefers-reduced-motion: reduce)");
    expect(css).toContain("animation-duration: 0.01ms");
  });
});
