// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const tpl = readFileSync(resolve(import.meta.dirname, '../components/admin/EndpointFold.vue'), 'utf8')

describe('R301 EndpointFold 端点折叠容器契约', () => {
  it('默认折叠：open 初值取自 defaultOpen，默认 false', () => {
    expect(tpl).toContain('const open = ref(props.defaultOpen)')
    expect(tpl).toContain('defaultOpen?: boolean')
    expect(tpl).toContain('defaultOpen: false')
  })

  it('提供 aria-expanded 与可测试开关/面板', () => {
    expect(tpl).toContain(':aria-expanded="open"')
    expect(tpl).toContain("testId + '-toggle'")
    expect(tpl).toContain("testId + '-panel'")
    expect(tpl).toContain('v-show="open"')
  })

  it('支持读/写/配置分组的具名插槽', () => {
    expect(tpl).toContain('groups?: EndpointFoldGroup[]')
    expect(tpl).toContain("'group-' + group.key")
    expect(tpl).toContain('v-for="group in groups"')
  })

  it('不销毁按钮：面板用 v-if 仅控制显隐，插槽内容原样透传', () => {
    expect(tpl).toContain('<slot :name=')
    expect(tpl).toContain('<slot />')
  })
})
