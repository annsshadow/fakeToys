// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const src = readFileSync(resolve(import.meta.dirname, './OrgViewer.vue'), 'utf8')
const template = src.slice(src.indexOf('<template>'), src.lastIndexOf('</template>'))

const groupKeys = ['read', 'write', 'config'] as const
const buttonRe = /<button class="org-meta-btn"/g
const countButtons = (text: string) => (text.match(buttonRe) ?? []).length

describe('OrgViewer 端点折叠契约', () => {
  it('模板使用 EndpointFold，分组为读/写/配置三组', () => {
    expect(src).toContain("import { EndpointFold } from '../components/admin/'")
    expect(template).toContain('<EndpointFold')
    expect(template).toContain(':groups="foldGroups"')
    for (const key of groupKeys) {
      expect(template).toContain(`<template #group-${key}>`)
    }
    const groupsStart = src.indexOf('const foldGroups = [')
    const groupsEnd = src.indexOf(']', groupsStart)
    expect(groupsStart).toBeGreaterThan(-1)
    expect(groupsEnd).toBeGreaterThan(groupsStart)
    const groupsBlock = src.slice(groupsStart, groupsEnd)
    for (const key of groupKeys) {
      expect(groupsBlock).toContain(`key: '${key}'`)
    }
    expect(groupsBlock.match(/\{ key:/g) ?? []).toHaveLength(3)
  })

  it('98 个调试按钮全部保留，逐字落在三个分组插槽内，总数与 endpointCount 一致', () => {
    expect(template.match(buttonRe) ?? []).toHaveLength(98)
    expect(src).toContain('const endpointCount = 98')
    expect(template).toContain(':total-count="endpointCount"')

    const slots = groupKeys.map((key) => {
      const start = template.indexOf(`<template #group-${key}>`)
      const end = template.indexOf('</template>', start)
      expect(start).toBeGreaterThan(-1)
      expect(end).toBeGreaterThan(start)
      return template.slice(start, end)
    })
    const perGroup = slots.map((slot) => countButtons(slot))
    expect(perGroup.every((n) => n > 0)).toBe(true)
    expect(perGroup.reduce((a, b) => a + b, 0)).toBe(98)
  })

  it('保留 h1 主标题，删除暴露 /api 端点的 subtitle', () => {
    expect(template).toContain('<h1>组织架构</h1>')
    expect(template).not.toContain('class="subtitle"')
    expect(template).not.toContain('/api/')
  })
})
