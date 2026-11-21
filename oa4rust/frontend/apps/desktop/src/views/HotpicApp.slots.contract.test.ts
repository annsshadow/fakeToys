// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment node
/**
 * R333-B 契约：HotpicApp.vue 必须把业务入口放进 AdminToolbar/AdminTable 的具名插槽。
 * R332 审计坐实：默认插槽内容被 Vue 静默丢弃，21 个热图入口 + 行删除在真 DOM 全部缺失。
 * 断言为源码模板静态结构（不引入 @vue/test-utils，遵循仓库 *.contract.test.ts 惯例）。
 */
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { describe, expect, it } from "vitest";

const src = readFileSync(resolve(import.meta.dirname, "HotpicApp.vue"), "utf8");
const tpl = src.slice(src.indexOf("<template>"), src.lastIndexOf("</template>"));

function namedSlot(template: string, name: string): string {
  const marker = "<template #" + name;
  const start = template.indexOf(marker);
  if (start < 0) return "";
  const gt = template.indexOf(">", start);
  if (gt < 0) return "";
  const bodyStart = gt + 1;
  const end = template.indexOf("</template>", bodyStart);
  return template.slice(bodyStart, end < 0 ? template.length : end);
}

function defaultSlotOf(template: string, tag: string): string {
  const open = template.indexOf("<" + tag);
  if (open < 0) return "";
  const close = template.indexOf("</" + tag + ">", open);
  let block = template.slice(open, close < 0 ? template.length : close);
  let cursor = 0;
  while (true) {
    const s = block.indexOf("<template #", cursor);
    if (s < 0) break;
    const e = block.indexOf("</template>", s);
    if (e < 0) break;
    block = block.slice(0, s) + block.slice(e + "</template>".length);
    cursor = s;
  }
  return block;
}

describe("R333-B HotpicApp.vue 具名插槽业务入口契约", () => {
  it("AdminToolbar 用组件自带 v-model + @search/@refresh，提供 #filters/#actions 具名插槽", () => {
    expect(tpl).toContain('v-model="keyword"');
    expect(tpl).toContain('@search="doSearch"');
    expect(tpl).toContain('@refresh="doSearch"');
    expect(tpl).toContain("<template #filters>");
    expect(tpl).toContain("<template #actions>");
  });

  it("AdminToolbar 默认插槽内不再裸放 input 或按钮（fail-loud）", () => {
    const d = defaultSlotOf(tpl, "AdminToolbar");
    expect(d, "AdminToolbar 默认插槽仍有搜索 input").not.toContain('class="search-input"');
    expect(d, "AdminToolbar 默认插槽仍有按钮").not.toContain("<button");
  });

  it("21 个工具栏入口（1 自造 input + 20 按钮）全在具名插槽，文案与事件齐全", () => {
    const named = namedSlot(tpl, "filters") + namedSlot(tpl, "actions");
    expect(named).toContain('class="search-input"');
    expect(named).toContain('@keyup.enter="doSearch"');
    const labels = ["搜索", "热图/面板", "热图2/面板2/应用2", "深度读", "孪生端点", "建热图", "改标题", "存配置", "建用户热图", "删用户热图", "清BBS密文", "清CMS密文", "建实体热图", "删实体热图", "存在校验", "按应用热图", "按ID热图", "密文热图列表", "用户热图列表", "删用户热图2"];
    for (const label of labels) expect(named, "缺入口: " + label).toContain(label);
    expect(named.split('class="btn-primary"').length - 1, "btn-primary 应为 20").toBe(20);
    const events = ["doSearch", "loadHotpicMeta", "loadHotpicMeta2", "loadHotpicDeep", "loadHotpicTwin", "hotpicWrite('create')", "hotpicWrite('changeTitle')", "hotpicWrite('config')", "hotpicWrite('userCreate')", "hotpicWrite('userDelete')", "hotpicWrite('cipherBbs')", "hotpicWrite('cipherCms')", "hotpicWrite('coreCreate')", "hotpicWrite('coreDelete')", "hotpicMore('existsCheck')", "hotpicMore('byApp')", "hotpicMore('byId')", "hotpicMore('cipherList')", "hotpicMore('userList')", "hotpicMore('userDelete2')"];
    for (const e of events) expect(named, "缺事件: " + e).toContain(e);
  });

  it("AdminTable 加 cell-actions 放行行删除，其他列不新增 cell-* 插槽保持回退展示", () => {
    expect(tpl).toContain('<template #cell-actions="{ row: item }">');
    expect(tpl, "标题列不应新增 cell 插槽").not.toContain("<template #cell-title");
    expect(tpl, "应用列不应新增 cell 插槽").not.toContain("<template #cell-appId");
    expect(tpl, "状态列不应新增 cell 插槽").not.toContain("<template #cell-status");
    const actions = namedSlot(tpl, "cell-actions");
    expect(actions.split('class="btn-del"').length - 1, "btn-del 应为 1").toBe(1);
    expect(actions).toContain('@click.stop="onDelete(item)"');
    expect(actions).toContain("删除");
  });

  it("ListLoadError/loading/空态分支与所有 load* 方法保持不动，AdminTable 仍绑定 :loading/:error", () => {
    expect(src).toContain("import ListLoadError");
    expect(tpl).toContain('mode="banner"');
    expect(tpl).toContain('mode="empty"');
    expect(tpl).toContain('class="loading-row"');
    expect(tpl).toContain("暂无热帖数据");
    expect(tpl).toContain(':loading="loading"');
    expect(tpl).toContain(':error="loadError"');
    for (const fn of ["loadHotpicMeta", "loadHotpicMeta2", "loadHotpicDeep", "loadHotpicTwin", "hotpicWrite", "hotpicMore", "doSearch", "onDelete"]) {
      expect(src, "缺方法: " + fn).toContain(fn);
    }
  });
});
