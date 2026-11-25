// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R521 展开态属性契约测试（WCAG 2.1 AA / 4.1.2 Name, Role, Value）。
 *
 * R519 只读甄别判定两处「新建」 disclosure 按钮在折叠/展开两态均缺失
 * aria-expanded，AXTree 无 expanded 属性，辅助技术用户无法感知面板当前
 * 展开状态，违反 WCAG 4.1.2 State Properties。
 *
 * 修复仅给两按钮各加一行 :aria-expanded 绑定（对齐正面锚点
 * AttendanceApp.vue / CalendarApp.vue），业务逻辑（@click 切换、v-if 显隐、
 * 按钮文案、data-testid）必须逐字保持不变。
 *
 * 本测试读 .vue 模板源码：核心断言验证 aria-expanded 绑定存在（真父 RED fail、
 * 修复后 GREEN pass）；守卫断言锁定 @click / v-if / 文案 / testid 零改
 * （RED 与 GREEN 均 pass），确保修复不越界改动业务。
 */
import { readFileSync } from 'node:fs'
import path from 'node:path'
import { fileURLToPath } from 'node:url'
import { describe, expect, it } from 'vitest'

const here = path.dirname(fileURLToPath(import.meta.url))
const frontendRoot = path.resolve(here, '../../../..')

function vue(rel: string): string {
  return readFileSync(path.join(frontendRoot, rel), 'utf8')
}

/** 取含指定 data-testid 的完整 <button>...</button> 片段。 */
function buttonOuter(source: string, testid: string): string {
  const needle = 'data-testid="' + testid + '"'
  const at = source.indexOf(needle)
  expect(at, '未找到 data-testid="' + testid + '"').toBeGreaterThan(-1)
  const start = source.lastIndexOf('<button', at)
  const close = '</button>'
  const end = source.indexOf(close, at)
  expect(start, 'data-testid="' + testid + '" 不在 <button> 标签内').toBeGreaterThan(-1)
  expect(end, 'data-testid="' + testid + '" 的 <button> 无结束标签').toBeGreaterThan(-1)
  return source.slice(start, end + close.length)
}

describe('R521 展开态 aria-expanded 契约（WCAG 2.1 AA / 4.1.2）', () => {
  const unit = vue('apps/desktop/src/views/OrgDirectoryApp.vue')
  const group = vue('apps/desktop/src/views/OrgViewer.vue')

  // ---- 核心断言：修复前 RED fail，修复后 GREEN pass ----
  it('新建部门按钮绑定 :aria-expanded="showUnitCreate"', () => {
    expect(buttonOuter(unit, 'org-create-unit-btn')).toMatch(/:aria-expanded="showUnitCreate"/)
  })

  it('新建组织组按钮绑定 :aria-expanded="showGroupCreate"', () => {
    expect(buttonOuter(group, 'org-create-group-btn')).toMatch(/:aria-expanded="showGroupCreate"/)
  })

  // ---- 守卫断言：RED 与 GREEN 均须 pass，锁定业务零改 ----
  it('新建部门按钮 @click 切换逻辑零改', () => {
    expect(buttonOuter(unit, 'org-create-unit-btn')).toContain('@click="showUnitCreate = !showUnitCreate"')
  })

  it('新建组织组按钮 @click 切换逻辑零改', () => {
    expect(buttonOuter(group, 'org-create-group-btn')).toContain('@click="showGroupCreate = !showGroupCreate"')
  })

  it('新建部门面板 v-if 显隐与提交处理器零改', () => {
    expect(unit).toContain('<form v-if="showUnitCreate" @submit.prevent="createUnitFromForm">')
  })

  it('新建组织组面板 v-if 显隐与提交处理器零改', () => {
    expect(group).toContain('<form v-if="showGroupCreate" @submit.prevent="createGroupFromForm">')
  })

  it('新建部门按钮文案零改', () => {
    expect(buttonOuter(unit, 'org-create-unit-btn')).toContain("{{ showUnitCreate ? '收起' : '新建部门' }}")
  })

  it('新建组织组按钮文案零改', () => {
    expect(buttonOuter(group, 'org-create-group-btn')).toContain("{{ showGroupCreate ? '收起' : '新建组织组' }}")
  })
})
