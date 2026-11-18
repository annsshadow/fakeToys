// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment node
/**
 * R238 列定义表格行内控件可访问名契约（WCAG 4.1.2 Name/Role/Value、3.3.2 Labels or Instructions）。
 *
 * 缺陷证据：QueryTableDesignerApp.vue 的列定义表格每行是可变长 v-for 行：
 *   <tr v-for="(column,index) in form.columns">
 *     <td><input v-model="column.name" placeholder="lower_snake_case" /></td>
 *     <td><select v-model="column.type">…</select></td>
 *     <td><input v-model="column.nullable" type="checkbox" /></td>
 *   </tr>
 * 这些控件位于 <td> 而非 <th>，既不在 <label> 内，也没有 aria-label / aria-labelledby；
 * 可空复选框连 placeholder 都没有。读屏器只会念「编辑框」「 combobox 」「复选框 未选中」，
 * 用户无法得知每个控件对应哪一列、控制哪个属性。
 *
 * 现有增强器均不覆盖此场景：labelA11y 只处理「孤儿 label 紧随的单个控件」，
 * searchFieldA11y 的选择器只含搜索类名（不含裸输入框/select/checkbox），
 * tableA11y 只补 <th> 的 scope，formErrorA11y 只处理错误文本关联。
 * 因此用源码级契约钉住这三个行内控件的可访问名，防止回退。
 */
import { readFileSync } from 'node:fs'
import path from 'node:path'
import { fileURLToPath } from 'node:url'
import { describe, expect, it } from 'vitest'

const viewsDir = path.join(path.dirname(fileURLToPath(import.meta.url)), '..', 'views')
const source = readFileSync(path.join(viewsDir, 'QueryTableDesignerApp.vue'), 'utf8')

/** 提取列定义 v-for 行（<tr v-for=...>...</tr>）全文。 */
function columnRow(): string {
  const start = source.indexOf('<tr v-for="(column,index) in form.columns"')
  expect(start, '未找到列定义 v-for 行').toBeGreaterThan(-1)
  const end = source.indexOf('</tr>', start)
  expect(end, '列定义行未闭合').toBeGreaterThan(start)
  return source.slice(start, end + '</tr>'.length)
}

/** 取包含给定绑定表达式的 <input>/<select> 起始标签全文。 */
function controlTag(row: string, tag: 'input' | 'select', vmodel: string): string {
  const hook = `v-model="${vmodel}"`
  const at = row.indexOf(hook)
  expect(at, `未找到 ${hook}`).toBeGreaterThan(-1)
  const start = row.lastIndexOf(`<${tag}`, at)
  const end = row.indexOf('>', at)
  expect(start, `${hook} 不在 <${tag}> 标签内`).toBeGreaterThan(-1)
  return row.slice(start, end + 1)
}

/** 控件是否具备某种形式的可访问名。 */
function hasAccessibleName(tag: string): boolean {
  return /\baria-label="[^"]+"/.test(tag) || /\baria-labelledby="[^"]+"/.test(tag) || /\b:aria-label="[^"]+"/.test(tag)
}

const row = columnRow()

describe('列定义表格行内控件可访问名（R238 / WCAG 4.1.2、3.3.2）', () => {
  it('列名输入框具有可访问名（placeholder 不是可访问名）', () => {
    const tag = controlTag(row, 'input', 'column.name')
    expect(hasAccessibleName(tag), `列名输入框缺少可访问名: ${tag}`).toBe(true)
  })

  it('列类型下拉框具有可访问名', () => {
    const tag = controlTag(row, 'select', 'column.type')
    expect(hasAccessibleName(tag), `列类型下拉框缺少可访问名: ${tag}`).toBe(true)
  })

  it('可空复选框具有可访问名', () => {
    const tag = controlTag(row, 'input', 'column.nullable')
    expect(hasAccessibleName(tag), `可空复选框缺少可访问名: ${tag}`).toBe(true)
  })
})
