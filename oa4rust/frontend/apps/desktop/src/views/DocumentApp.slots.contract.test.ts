// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment node
/**
 * R330 契约：DocumentApp.vue 必须把业务入口放进 AdminToolbar/AdminTable 的具名插槽。
 * R329 审计坐实：默认插槽内容被 Vue 静默丢弃，导致新建/发布/删除等入口在真 DOM 全部缺失。
 * 断言为源码模板静态结构（不引入 @vue/test-utils，遵循仓库 *.contract.test.ts 惯例）。
 */
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { describe, expect, it } from "vitest";

const src = readFileSync(resolve(import.meta.dirname, "DocumentApp.vue"), "utf8");
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

describe("R330 DocumentApp.vue 具名插槽业务入口契约", () => {
  it("AdminToolbar 用组件自带 v-model + @search/@refresh，不再裸放自造搜索 input", () => {
    expect(tpl).toContain('v-model="keyword"');
    expect(tpl).toContain('@search="doSearch"');
    expect(tpl).toContain('@refresh="doSearch"');
    const toolbar = defaultSlotOf(tpl, "AdminToolbar");
    expect(toolbar, "默认插槽不应再裸放搜索 input").not.toContain('class="search-input"');
  });

  it("工具栏按钮 + doc-meta-note 全在具名 #actions/#filters，一个不少", () => {
    const named = namedSlot(tpl, "actions") + namedSlot(tpl, "filters");
    const labels = ["搜索", "+ 新建文档", "字段/批量状态", "管理视图", "孪生端点", "孪生端点B", "数据·文档深度C", "密文文档列表", "doc-meta-note"];
    for (const label of labels) expect(named, "缺按钮/节点: " + label).toContain(label);
    for (const fn of ["doSearch", "showCreate=true", "loadDocMeta", "loadManagerList", "loadDocTwin", "loadDocTwin2", "loadDocTwin3", "loadCipherList"]) {
      expect(named, "缺事件: " + fn).toContain(fn);
    }
  });

  it("AdminToolbar/AdminTable 默认插槽内不再裸放按钮或列表结构（fail-loud）", () => {
    expect(defaultSlotOf(tpl, "AdminToolbar"), "AdminToolbar 默认插槽仍有按钮").not.toContain("<button");
    const table = defaultSlotOf(tpl, "AdminTable");
    expect(table, "AdminTable 默认插槽仍有裸内容").not.toContain('class="item-table"');
    expect(table).not.toContain('class="list-panel"');
  });

  it("AdminTable 提供 cell-title/cell-status/cell-createdAt/cell-actions 四个具名插槽", () => {
    for (const name of ["cell-title", "cell-status", "cell-createdAt", "cell-actions"]) {
      expect(tpl, "缺 #" + name).toContain("<template #" + name);
    }
    expect(namedSlot(tpl, "cell-title")).toContain("未命名");
    expect(namedSlot(tpl, "cell-status")).toContain("statusLabel(item)");
    expect(namedSlot(tpl, "cell-status")).toContain("docStatusCls(item.status)");
  });

  it("cell-actions 保留 14 btn-act + 1 btn-del 及全部 15 个 @click 与 disabled", () => {
    const actions = namedSlot(tpl, "cell-actions");
    expect(actions.split('class="btn-act"').length - 1, "btn-act 应为 14").toBe(14);
    expect(actions.split('class="btn-del"').length - 1, "btn-del 应为 1").toBe(1);
    const events = ["onCommend", "onUncommend", "onTop", "onUnTop", "onPublish", "onPublishCancel", "onCipherPublish", "onViewCount", "onPersons", "onNotify", "onViewRecord", "onDocLog", "onCommendList", "onDetail", "onDelete"];
    for (const e of events) expect(actions, "缺 @click " + e).toContain('@click="' + e + "(");
    expect(actions).toContain(':disabled="deletingId === item.id"');
  });

  it("500 兜底/loading 行为保持：AdminTable 仍绑定 :loading/:error 且 ListLoadError 保留", () => {
    expect(tpl).toContain(':loading="loading"');
    expect(tpl).toContain(':error="loadError"');
    expect(src).toContain("import ListLoadError");
  });
});
