// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * 运营后台弱化文本对比度守卫（WCAG 2.1 AA 1.4.3）。
 *
 * admin 的 :root 定义了「文本四阶：主 / 常规 / 弱化 / 更弱」，其中
 * --lyp-muted 与 --lyp-dim 承载大量活动正文小字：
 *   .kpi-sub（KPI 副标题）、.hint（登录提示）、.nav-group-title（导航分组）、
 *   .dim 工具类、以及 Element Plus 的 secondary text / placeholder。
 * 这些都是用户要读的信息，不是纯装饰，必须达到 4.5:1。
 *
 * 判据按 WCAG sRGB 相对亮度实算，不是「文件里出现了颜色值」——
 * 后者会被注释或任意色值满足，等于没有守卫。
 */
import { describe, expect, it } from "vitest";
import { readFileSync, existsSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";

const root = join(dirname(fileURLToPath(import.meta.url)), "..");
const cssPath = join(root, "src", "styles", "tokens.css");
if (!existsSync(cssPath)) throw new Error("对比度守卫找不到 tokens.css（令牌真源被改名或挪走了？）");
const css = readFileSync(cssPath, "utf8");

function token(name: string): string {
  const m = css.match(new RegExp("--lyp-" + name + ":\\s*(#[0-9a-fA-F]{6})\\s*;"));
  if (!m) throw new Error("tokens.css 缺少 token --lyp-" + name);
  return m[1];
}

function luminance(hex: string): number {
  const c = hex.replace("#", "");
  const channels = [c.slice(0, 2), c.slice(2, 4), c.slice(4, 6)].map((part) => {
    const v = parseInt(part, 16) / 255;
    return v <= 0.03928 ? v / 12.92 : Math.pow((v + 0.055) / 1.055, 2.4);
  });
  return 0.2126 * channels[0] + 0.7152 * channels[1] + 0.0722 * channels[2];
}

function contrast(a: string, b: string): number {
  const l1 = luminance(a);
  const l2 = luminance(b);
  const hi = Math.max(l1, l2);
  const lo = Math.min(l1, l2);
  return (hi + 0.05) / (lo + 0.05);
}

// dim 文本实际落在的全部深色表面：body 底、面板、表格头/悬浮行、overlay。
const backgrounds: Record<string, string> = {
  bg: token("bg"),
  panel: token("panel"),
  "panel-2": token("panel-2"),
  "panel-3": token("panel-3"),
};

describe("运营后台弱化文本 WCAG AA 对比度", () => {
  it("--lyp-muted 对四种深色背景均 ≥4.5:1", () => {
    const fg = token("muted");
    for (const [name, bg] of Object.entries(backgrounds)) {
      expect(contrast(fg, bg), "muted on " + name).toBeGreaterThanOrEqual(4.5);
    }
  });

  it("--lyp-dim 承载 kpi-sub/hint/nav-group-title 正文，对四种背景均 ≥4.5:1", () => {
    const fg = token("dim");
    for (const [name, bg] of Object.entries(backgrounds)) {
      expect(contrast(fg, bg), "dim on " + name).toBeGreaterThanOrEqual(4.5);
    }
  });

  it("文本四阶层级保持：text-2 > muted > dim（亮度递减）", () => {
    const t2 = luminance(token("text-2"));
    const muted = luminance(token("muted"));
    const dim = luminance(token("dim"));
    expect(t2).toBeGreaterThan(muted);
    expect(muted).toBeGreaterThan(dim);
  });
});