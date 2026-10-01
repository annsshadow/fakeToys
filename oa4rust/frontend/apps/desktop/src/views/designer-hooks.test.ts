// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const read = (name: string) => readFileSync(resolve(import.meta.dirname, name), 'utf8')

describe('设计器死钩子清理与实时校验', () => {
  it('DesignCenter 删除空的 filterDesigners，检索仍由 filteredDesigners computed 响应式驱动', () => {
    const s = read('DesignCenterApp.vue')
    expect(s).not.toContain('function filterDesigners')
    expect(s).not.toContain('@input="filterDesigners"')
    expect(s).toContain('const filteredDesigners = computed(')
    expect(s).toContain('v-model="searchQuery"')
  })

  it('ConfigDesigner 空 onConfigChange 换成 configValid 实时 JSON 合法性徽标', () => {
    const s = read('ConfigDesignerApp.vue')
    expect(s).not.toContain('function onConfigChange')
    expect(s).not.toContain('@input="onConfigChange"')
    expect(s).toContain('const configValid = computed(')
    expect(s).toContain('configValid === true')
    expect(s).toContain('configValid === false')
  })
})
