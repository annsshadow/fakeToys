// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R529 多选组分组语义契约测试（WCAG 2.1 AA / 1.3.1 Info and Relationships）。
 * R528 甄别判定 FormDesigner 预览态 .pv-cb-group 多选组容器仅 class，
 * 无 role=group / aria-labelledby，字段标题裸 label 未程序化关联；
 * CDP partial+full AX 双通道坐实容器 role=none / ignored=true / name 空。
 * 修复仅改模板标记：标题 label 用 v-for 索引 i 生成唯一 :id，
 * 容器加 role="group" 与 :aria-labelledby 指向该 id。业务零改。
 * 核心断言真父 RED fail、修复后 GREEN pass；守卫断言 RED/GREEN 均 pass。
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

function fieldLabel(source: string): string {
  const needle = "['row_start','row_end'].includes(field.type)"
  const at = source.indexOf(needle)
  expect(at, '未找到字段标题 label 的 row_start/row_end 分支').toBeGreaterThan(-1)
  const start = source.lastIndexOf('<label', at)
  const close = '</label>'
  const end = source.indexOf(close, at)
  expect(start, '字段标题 label 无起始标签').toBeGreaterThan(-1)
  expect(end, '字段标题 label 无结束标签').toBeGreaterThan(-1)
  return source.slice(start, end + close.length)
}

function cbGroupOpen(source: string): string {
  const needle = 'class="pv-cb-group"'
  const at = source.indexOf(needle)
  expect(at, '未找到 class="pv-cb-group" 容器').toBeGreaterThan(-1)
  const start = source.lastIndexOf('<div', at)
  const end = source.indexOf('>', at)
  expect(start, 'pv-cb-group 容器无 <div> 起始').toBeGreaterThan(-1)
  expect(end, 'pv-cb-group 容器开标签无 >').toBeGreaterThan(-1)
  return source.slice(start, end + 1)
}

describe('R529 预览多选组分组语义契约（WCAG 2.1 AA / 1.3.1）', () => {
  const fd = vue('apps/desktop/src/views/FormDesigner.vue')

  it('字段标题 label 用 v-for 索引 i 绑定唯一 id', () => {
    expect(fieldLabel(fd)).toMatch(/:id="'pv-field-label-' \+ i"/)
  })

  it('.pv-cb-group 容器声明 role="group"', () => {
    expect(cbGroupOpen(fd)).toContain('role="group"')
  })

  it('.pv-cb-group 容器 aria-labelledby 指向字段标题 id', () => {
    expect(cbGroupOpen(fd)).toMatch(/:aria-labelledby="'pv-field-label-' \+ i"/)
  })

  it('多选组容器 v-else-if 分支与 class 零改', () => {
    expect(cbGroupOpen(fd)).toContain('checkbox_group')
    expect(cbGroupOpen(fd)).toContain('class="pv-cb-group"')
  })

  it('option 列表 parseOptions 调用零改', () => {
    expect(fd).toContain('v-for="opt in parseOptions(field.optionsStr)" :key="opt.value" class="pv-cb-label"')
  })

  it('三复选框自身 value/disabled/class 零改', () => {
    expect(fd).toContain(':value="opt.value" :disabled="field.disabled" class="pv-cb"')
  })

  it('字段标题 row_start/row_end 分支与必填星号零改', () => {
    expect(fieldLabel(fd)).toContain("row_end'].includes(field.type)")
    expect(fieldLabel(fd)).toContain('class="req">*</span>')
  })

  it('预览开关 @click=togglePreview 零改', () => {
    expect(fd).toContain('@click="togglePreview"')
  })

  it('预览表单 @submit.prevent 与提交按钮零改', () => {
    expect(fd).toContain('<form @submit.prevent>')
    expect(fd).toContain('class="btn btn-primary pv-submit">提交</button>')
  })

  it('预览字段 v-for 索引 i 与 pv-field class 零改', () => {
    expect(fd).toContain('v-for="(field, i) in currentForm.fields" :key="i" class="pv-field"')
  })

  it('parseOptions 业务函数仍存在', () => {
    expect(fd).toMatch(/function parseOptions|const parseOptions/)
  })
})
