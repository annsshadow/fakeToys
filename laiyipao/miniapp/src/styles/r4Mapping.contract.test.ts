// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * H5 侧 R4 映射守卫：独立确认十页映射与登录/验真轻量 AI 规则在规范中落地，
 * 并确认 H5 令牌仍保持 v1/v3 双锚点。admin/tests/r4Mapping.contract.test.ts 负责双端对齐。
 */
import { describe, expect, it } from "vitest";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";

const dir = dirname(fileURLToPath(import.meta.url));
const root = join(dir, "..", "..", "..", "..");
const doc = readFileSync(join(root, "docs", "laiyipao", "design", "AI_VISUAL_SYSTEM.md"), "utf8");
const css = readFileSync(join(dir, "tokens.css"), "utf8");
const start = doc.indexOf("## 1C.");
const end = doc.indexOf("## 2.");
const spec = start >= 0 && end > start ? doc.slice(start, end) : "";

describe("H5 R4 剩余页面映射", () => {
  it("十页映射齐全", () => {
    expect(start).toBeGreaterThanOrEqual(0);
    for (const p of ["announcements", "bag", "battles", "enemies", "login", "me", "rank", "signin", "skins", "verify"])
      expect(spec, "缺少 " + p + ".vue").toContain(p + ".vue");
  });

  it("登录页轻量 AI 不新增输入、不弹窗、不阻塞提交", () => {
    const i = spec.indexOf("login.vue");
    expect(i).toBeGreaterThanOrEqual(0);
    const j = spec.indexOf("### 1C.4", i);
    const chunk = spec.slice(i, j < 0 ? spec.length : j);
    for (const s of ["轻量", "不打断", "不新增", "不弹窗", "role=note"]) expect(chunk).toContain(s);
  });

  it("验真页轻量 AI 只在取到复现信息后出现，不伪造思考", () => {
    const i = spec.indexOf("verify.vue");
    expect(i).toBeGreaterThanOrEqual(0);
    const chunk = spec.slice(i);
    for (const s of ["轻量", "不打断", "replayInfo", "不伪造"]) expect(chunk).toContain(s);
  });

  it("H5 令牌保持 v1/v3 双锚点", () => {
    expect(css).toMatch(/--lyp-token-version:\s*lyp-tokens-v1/);
    expect(css).toMatch(/--lyp-r3-version:\s*lyp-tokens-v3-r3/);
    expect(css).not.toMatch(/lyp-tokens-v4/);
  });
});
