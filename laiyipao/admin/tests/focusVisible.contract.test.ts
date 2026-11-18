// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * 运营后台键盘焦点可见守卫（WCAG 2.4.7 Focus Visible，A 级）。
 *
 * admin 用 Element Plus，且 WikiView.vue:1184 等处有 outline:none 移除浏览器
 * 原生焦点轮廓；但项目层从未提供统一的 :focus-visible 兜底，键盘用户 Tab
 * 时看不清焦点落在哪个按钮/链接/输入框上。桌面端 R184 已在全局主题层建立
 * 该标准，本测试把同一条标准钉到运营后台，保证两个前端口径一致。
 *
 * 判据读全局样式真实声明，不是「文件里出现 focus 字样」——后者会被
 * #main-content:focus:not(:focus-visible) 这类抑制规则满足，等于没守卫。
 */
import { describe, expect, it } from "vitest";
import { readFileSync, existsSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";

const root = join(dirname(fileURLToPath(import.meta.url)), "..");
const cssPath = join(root, "src", "styles", "main.css");
if (!existsSync(cssPath)) throw new Error("焦点守卫找不到 main.css");
const css = readFileSync(cssPath, "utf8");

function focusRule(): string {
  const m = css.match(/:where\([^}]*:focus-visible\s*\{[^}]*\}/);
  if (!m) throw new Error("main.css 缺少全局 :focus-visible 焦点环规则");
  return m[0];
}

describe("运营后台键盘焦点可见标准（WCAG 2.4.7）", () => {
  it("全局样式提供 :focus-visible 焦点环，兜住 outline:none", () => {
    expect(() => focusRule()).not.toThrow();
  });

  it("焦点环覆盖所有可聚焦交互元素", () => {
    const rule = focusRule();
    expect(rule).toMatch(/a,\s*button,\s*input,\s*textarea,\s*select,\s*\[tabindex\]/);
  });

  it("焦点环用主题 token 而非写死色值，且两像素加偏移", () => {
    const rule = focusRule();
    expect(rule).toMatch(/outline:\s*2px\s+solid\s+var\(--lyp-[a-z0-9-]+\)/);
    expect(rule).toMatch(/outline-offset:\s*2px/);
  });

  it("焦点环用 !important 覆盖视图 scoped 样式的 outline:none", () => {
    expect(focusRule()).toMatch(/outline:\s*2px[^;]*!important/);
  });
});
