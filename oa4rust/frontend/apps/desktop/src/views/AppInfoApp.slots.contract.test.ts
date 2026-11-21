// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment node
/**
 * R333-A 契约：AppInfoApp.vue 必须把工具栏与行操作入口放进 AdminToolbar/AdminTable 具名插槽。
 * R332 审计坐实：默认插槽内容被 Vue 静默丢弃（pageerror=0），新建/管理视图/权限等入口在真 DOM 全部缺失。
 * 静态源码模板断言，遵循仓库 *.contract.test.ts 惯例，不引入 @vue/test-utils。
 */
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { describe, expect, it } from "vitest";

const src = readFileSync(resolve(import.meta.dirname, "AppInfoApp.vue"), "utf8");
const tpl = src.slice(src.indexOf("<template>"), src.lastIndexOf("</template>"));

function namedSlot(template: string, name: string) {
  const marker = "<template #" + name;
  const start = template.indexOf(marker);
  if (start < 0) return "";
  const gt = template.indexOf(">", start);
  if (gt < 0) return "";
  const bodyStart = gt + 1;
  const end = template.indexOf("</template>", bodyStart);
  return template.slice(bodyStart, end < 0 ? template.length : end);
}

function defaultSlotOf(template: string, tag: string) {
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

describe("R333-A AppInfoApp.vue 具名插槽业务入口契约", () => {
  it("AdminToolbar 用组件自带 v-model + @search/@refresh，默认插槽不再裸放自造搜索 input", () => {
    expect(tpl).toContain('v-model="keyword"');
    expect(tpl).toContain('@search="doSearch"');
    expect(tpl).toContain('@refresh="doSearch"');
    const toolbar = defaultSlotOf(tpl, "AdminToolbar");
    expect(toolbar, "默认插槽不应再裸放搜索 input").not.toContain('class="search-input"');
  });

  it("工具栏 6 个按钮全在具名 #actions/#filters，一个不少", () => {
    const named = namedSlot(tpl, "actions") + namedSlot(tpl, "filters");
    const labels = ["搜索", "+ 新建应用", "管理/视图/类型", "发布/全部视图/含文档", "受控栏目/带流程/视图数据", "类型管理/含文档类型/输出"];
    for (const label of labels) expect(named, "缺按钮: " + label).toContain(label);
    for (const fn of ["doSearch", "createApp", "loadAppViews", "loadAppPublish", "loadAppExtra", "loadAppExtra2"]) {
      expect(named, "缺事件: " + fn).toContain(fn);
    }
  });

  it("AdminToolbar/AdminTable 默认插槽内不再裸放按钮或卡片列表结构（fail-loud）", () => {
    expect(defaultSlotOf(tpl, "AdminToolbar"), "AdminToolbar 默认插槽仍有按钮").not.toContain("<button");
    const table = defaultSlotOf(tpl, "AdminTable");
    expect(table, "AdminTable 默认插槽仍有裸内容").not.toContain('class="list-panel"');
    expect(table).not.toContain('class="item-grid"');
    expect(table).not.toContain('class="item-card"');
  });

  it("AdminTable 只提供 cell-actions 具名插槽，其余列回退 row[col.key]", () => {
    expect(tpl, "缺 #cell-actions").toContain("<template #cell-actions");
    expect(tpl).not.toContain("<template #cell-name");
    expect(tpl).not.toContain("<template #cell-appFlag");
    expect(tpl).not.toContain("<template #cell-appType");
  });

  it("cell-actions 保留 4 btn-act2 + 1 btn-del 及全部 @click.stop 事件", () => {
    const actions = namedSlot(tpl, "cell-actions");
    expect(actions.split('class="btn-act2"').length - 1, "btn-act2 应为 4").toBe(4);
    expect(actions.split('class="btn-del"').length - 1, "btn-del 应为 1").toBe(1);
    for (const e of ["showPerms", "writePerms", "createFile", "loadIcon", "deleteApp"]) {
      expect(actions, "缺 @click.stop " + e).toContain('@click.stop="' + e + "(");
    }
  });

  it("列表状态与列定义保持：:loading/:error/empty-text 保留，ListLoadError 仍 import", () => {
    expect(tpl).toContain(':loading="loading"');
    expect(tpl).toContain(':error="loadError"');
    expect(tpl).toContain('empty-text=""');
    expect(tpl).toContain(':columns="appInfoColumns"');
    expect(src).toContain("import ListLoadError");
  });
});
