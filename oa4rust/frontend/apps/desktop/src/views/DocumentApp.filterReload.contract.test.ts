// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment node
/** R600 契约：DocumentApp.vue 页签切换必须触发重新查询，统计取过滤后 items.length。 */
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { describe, expect, it } from "vitest";

const src = readFileSync(resolve(import.meta.dirname, "DocumentApp.vue"), "utf8");

describe("R600 DocumentApp.vue 筛选联动契约", () => {
  it("两枚页签点击时都调用 doSearch", () => {
    expect(src).toContain("@click=\"tab='published'; doSearch()\"");
    expect(src).toContain("@click=\"tab='draft'; doSearch()\"");
  });

  it("import watch 且 watch(tab) 触发 doSearch 兜底", () => {
    expect(src).toMatch(/import \{[^}]*\bwatch\b[^}]*\} from 'vue'/);
    expect(src).toMatch(/watch\(tab,\s*\(\)\s*=>\s*\{\s*void doSearch\(\)\s*\}\)/);
  });

  it("doSearch 仍发 type/keyword 且统计取 items.length", () => {
    expect(src).toContain("params.keyword = keyword.value");
    expect(src).toContain("params.type = 'draft'");
    expect(src).toContain("api.post('/api/document/filter/list/1/size/50/manager', params");
    expect(src).toContain("value: items.value.length");
  });
});
