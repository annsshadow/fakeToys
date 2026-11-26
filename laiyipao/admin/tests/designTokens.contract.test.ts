// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * 双端设计令牌契约守卫（AI_VISUAL_SYSTEM.md §8.1 / §8.3）。
 *
 * 这块测试是「令牌唯一真源」的执法者：两端 tokens.css 的版本、五元素色值、
 * 文本四阶对比度、on-soft 派生 ink、玻璃文本、动效令牌必须与规范一致；
 * 同时扫描真实 CSS 规则块，防止 soft 叠 panel-3 这类已实测不达标的组合回潮。
 * 判据全部按 WCAG sRGB 相对亮度实算，不是「文件里出现了颜色字样」。
 */
import { describe, expect, it } from "vitest";
import { readFileSync, existsSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";

const adminRoot = join(dirname(fileURLToPath(import.meta.url)), "..");
const miniRoot = join(adminRoot, "..", "miniapp");
const files = {
  adminTokens: join(adminRoot, "src", "styles", "tokens.css"),
  miniTokens: join(miniRoot, "src", "styles", "tokens.css"),
  adminCss: join(adminRoot, "src", "styles", "main.css"),
  miniCss: join(miniRoot, "src", "styles", "common.css"),
};
for (const [name, path] of Object.entries(files)) {
  if (!existsSync(path)) throw new Error("令牌契约缺少文件 " + name + "：" + path);
}
function read(path: string): string { return readFileSync(path, "utf8"); }
function vars(css: string): Record<string, string> {
  const out: Record<string, string> = {};
  for (const m of css.matchAll(/(--[a-zA-Z0-9-]+)\s*:\s*([^;]+);/g)) out[m[1]] = m[2].trim();
  return out;
}
const A = vars(read(files.adminTokens));
const M = vars(read(files.miniTokens));
const adminCss = read(files.adminCss);
const miniCss = read(files.miniCss);

function hex(name: string, map: Record<string, string>): string {
  const value = map[name];
  if (!value) throw new Error("缺少令牌 " + name);
  if (!/^#[0-9a-fA-F]{6}$/.test(value)) throw new Error("令牌 " + name + " 不是 6 位 hex：" + value);
  return value.toLowerCase();
}
function rgb(value: string): [number, number, number] {
  const h = value.replace("#", "");
  return [0, 2, 4].map((i) => parseInt(h.slice(i, i + 2), 16)) as [number, number, number];
}
function lum(value: string): number {
  const c = rgb(value).map((v) => { const x = v / 255; return x <= 0.03928 ? x / 12.92 : Math.pow((x + 0.055) / 1.055, 2.4); });
  return 0.2126 * c[0] + 0.7152 * c[1] + 0.0722 * c[2];
}
function contrast(fg: string, bg: string): number {
  const [hi, lo] = [lum(fg), lum(bg)].sort((a, b) => b - a);
  return (hi + 0.05) / (lo + 0.05);
}
function blend(fg: string, bg: string, alpha: number): string {
  const f = rgb(fg), b = rgb(bg);
  return "#" + f.map((v, i) => Math.round(v * alpha + b[i] * (1 - alpha)).toString(16).padStart(2, "0")).join("");
}

const ELEMENTS = ["fire", "ice", "lightning", "corrosion", "kinetic"] as const;
const ELEM_HEX: Record<string, string> = {
  fire: "#ff6b35", ice: "#58a6ff", lightning: "#ffd33d", corrosion: "#7ee787", kinetic: "#c9a7ff",
};
const ends = [
  { name: "miniapp", map: M, base: (k: string) => "--" + k, elem: (k: string) => "--" + k },
  { name: "admin", map: A, base: (k: string) => "--lyp-" + k, elem: (k: string) => "--el-" + k },
];
function cssBlocks(css: string): string[] {
  return css.split("}").map((b) => b.slice(b.lastIndexOf("{") + 1)).filter((b) => b.includes("background"));
}

describe("双端设计令牌契约（AI_VISUAL_SYSTEM §8.1）", () => {
  it("两端令牌版本一致，防止单边漂移", () => {
    expect(M["--lyp-token-version"]).toBe("lyp-tokens-v1");
    expect(A["--lyp-token-version"]).toBe("lyp-tokens-v1");
  });

  it("五元素主色两端同值且等于规范 §2.2", () => {
    for (const key of ELEMENTS) {
      expect(M["--" + key], "miniapp " + key).toBe(ELEM_HEX[key]);
      expect(A["--el-" + key], "admin " + key).toBe(ELEM_HEX[key]);
    }
  });

  it("文本四阶对四种表面均 ≥4.5:1，且亮度递减", () => {
    for (const end of ends) {
      for (const key of ["text-2", "muted", "dim"]) {
        for (const surface of ["bg", "panel", "panel-2", "panel-3"]) {
          expect(contrast(hex(end.base(key), end.map), hex(end.base(surface), end.map)), end.name + " " + key + " on " + surface).toBeGreaterThanOrEqual(4.5);
        }
      }
      const levels = ["text", "text-2", "muted", "dim"].map((k) => lum(hex(end.base(k), end.map)));
      for (let i = 1; i < levels.length; i++) expect(levels[i - 1], end.name + " 文本层级 " + i).toBeGreaterThan(levels[i]);
    }
  });

  it("元素 on-soft ink 令牌齐全，叠自身 18% soft + panel-3 仍 ≥4.5:1", () => {
    for (const key of ELEMENTS) {
      expect(M["--" + key + "-ink"], "miniapp " + key + "-ink").toBeTruthy();
      expect(A["--el-" + key + "-ink"], "admin " + key + "-ink").toBeTruthy();
    }
    for (const end of ends) {
      const p3 = hex(end.base("panel-3"), end.map);
      for (const key of ELEMENTS) {
        const softBg = blend(hex(end.elem(key), end.map), p3, 0.18);
        expect(contrast(hex(end.elem(key + "-ink"), end.map), softBg), end.name + " " + key + "-ink on soft+panel-3").toBeGreaterThanOrEqual(4.5);
      }
    }
  });

  it("玻璃浮层令牌以 panel 为底、文本对最坏玻璃底 ≥4.5:1", () => {
    for (const end of ends) {
      const glass = end.map[end.base("glass-bg")] ?? "";
      expect(glass, end.name + " glass-bg").toMatch(/rgba\(/);
      expect(glass.toLowerCase(), end.name + " 玻璃底不得用 panel-3").not.toMatch(/34,\s*44,\s*61|33,\s*43,\s*55/);
      const p3 = hex(end.base("panel-3"), end.map);
      const worst = blend("#ffffff", p3, 0.08);
      for (const key of ["glass-text-muted", "glass-text-dim"]) {
        expect(contrast(hex(end.base(key), end.map), worst), end.name + " " + key).toBeGreaterThanOrEqual(4.5);
      }
    }
    expect(M["--glass-text-muted"]).toBe(A["--lyp-glass-text-muted"]);
    expect(M["--glass-text-dim"]).toBe(A["--lyp-glass-text-dim"]);
  });

  it("动效令牌双端同名同值（rpx 与 px 仅几何不同）", () => {
    for (const key of ["motion-fast", "motion-base", "motion-slow", "motion-ease"]) {
      expect(M["--" + key], key).toBeTruthy();
      expect(A["--lyp-" + key], key).toBe(M["--" + key]);
    }
  });

  it("两端全局键盘焦点环覆盖全部交互元素、用令牌且带偏移", () => {
    const rule = /:where\(a,\s*button,\s*input,\s*textarea,\s*select,\s*\[tabindex\]\):focus-visible\s*\{[^}]*\}/;
    const mini = miniCss.match(rule)?.[0] ?? "";
    const admin = adminCss.match(rule)?.[0] ?? "";
    expect(mini, "miniapp 焦点环").toMatch(/outline:\s*3rpx\s+solid\s+var\(--[a-z0-9-]+\)/);
    expect(mini, "miniapp 焦点偏移").toMatch(/outline-offset:\s*2rpx/);
    expect(admin, "admin 焦点环").toMatch(/outline:\s*2px\s+solid\s+var\(--lyp-[a-z0-9-]+\)/);
  });

  it("禁止模式扫描：18% 元素 soft 不得与 panel-3 在同一背景规则块共现", () => {
    for (const [name, css] of [["miniapp", miniCss], ["admin", adminCss], ["miniapp tokens", read(files.miniTokens)], ["admin tokens", read(files.adminTokens)]] as const) {
      for (const block of cssBlocks(css)) {
        const backgrounds = [...block.matchAll(/background(?:-color)?\s*:\s*([^;]+);/g)].map((m) => m[1]);
        const hasSoft = backgrounds.some((v) => /-soft\b|0\.18\)/.test(v));
        const hasP3 = backgrounds.some((v) => /panel-3|#222c3d|#212b37/i.test(v));
        expect(!(hasSoft && hasP3), name + " 出现 soft+panel-3 禁止组合：" + block.trim().slice(0, 120)).toBe(true);
      }
    }
  });

  it("miniapp .dim 工具类必须走 --dim 令牌，不得回退到 --muted", () => {
    expect(miniCss).toMatch(/\.dim\s*\{\s*color:\s*var\(--dim\)/);
    expect(miniCss).not.toMatch(/\.dim\s*\{\s*color:\s*var\(--muted\)/);
  });
});