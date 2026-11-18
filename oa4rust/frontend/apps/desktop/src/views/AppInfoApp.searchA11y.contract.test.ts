// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const SOURCE = resolve(import.meta.dirname, 'AppInfoApp.vue')

describe('AppInfoApp 搜索框可访问名（WCAG 4.1.2 Name/Role/Value）', () => {
  const source = readFileSync(SOURCE, 'utf8')

  it('占位提示不能充当搜索框的可访问名：必须显式提供 aria-label 或关联 label', () => {
    const line = source
      .split('\n')
      .find((line) => line.includes('v-model="keyword"') && line.includes('placeholder="搜索应用..."'))
    expect(line, '应用管理页搜索框未找到').toBeDefined()
    expect(line, '搜索框只有 placeholder，屏幕阅读器读出“编辑框 空白”；必须加 aria-label="搜索应用"').toMatch(
      /aria-label="搜索应用"|aria-labelledby=/,
    )
  })
})
