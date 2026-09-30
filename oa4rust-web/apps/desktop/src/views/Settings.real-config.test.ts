// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const source = readFileSync(resolve(import.meta.dirname, 'Settings.vue'), 'utf8')

describe('Settings 系统参数真实读写', () => {
  it('预览横幅与假保存提示已随后端实装移除', () => {
    expect(source).not.toContain('backend-off')
    expect(source).not.toContain('暂未启用（后端 501）')
    expect(source).not.toContain('功能预览态')
  })

  it('装载走真读端点，保存走真 UPSERT 端点', () => {
    expect(source).toContain("api.get('/api/config/system')")
    expect(source).toContain("api.post('/api/config', { configs: buildConfigs() })")
    expect(source).toContain('onMounted(loadCfg)')
  })

  it('装载按表单类型还原（数字/布尔），保存统一字符串落库', () => {
    expect(source).toContain("typeof cur === 'number'")
    expect(source).toContain("raw === 'true'")
    expect(source).toContain('value: String(value)')
  })
})
