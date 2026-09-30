// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

// mobile vitest 环境未装配 .vue 插件（main.ts 引 App.vue 无法运行时导入），
// 故按本仓视图测试惯例走源码断言。
const source = readFileSync(resolve(import.meta.dirname, 'main.ts'), 'utf8')

describe('mobile 全局错误兜底', () => {
  it('createApp 挂上与 desktop 同口径的 errorHandler，未捕获错误不静默吞', () => {
    expect(source).toContain('app.config.errorHandler')
    expect(source).toContain("console.error('[unhandled]', info, err)")
  })
})
