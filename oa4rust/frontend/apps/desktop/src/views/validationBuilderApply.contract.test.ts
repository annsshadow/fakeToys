// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/**
 * R227 契约：FormDesigner "验证规则构建器" 的 "应用到选中" 按钮必须绑定到一个真实存在
 * 的处理函数。模板里只有 @click 引用，若脚本没有定义，点击只会抛
 * "Property ... was accessed during render but is not defined" 并且规则永远无法应用到字段。
 */
import fs from 'node:fs'
import path from 'node:path'
import { describe, expect, test } from 'vitest'

const source = fs.readFileSync(path.resolve(__dirname, 'FormDesigner.vue'), 'utf-8')

describe('R227 验证规则构建器 "应用到选中" 接线契约', () => {
  test('模板按钮引用的 applyValidationRules 必须在脚本中有函数定义', () => {
    expect(source).toContain('@click="applyValidationRules"')
    expect(source).toMatch(/function\s+applyValidationRules\s*\(/)
  })
})
