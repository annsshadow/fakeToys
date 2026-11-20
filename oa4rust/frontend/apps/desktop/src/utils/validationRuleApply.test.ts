// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { describe, expect, test } from 'vitest'
import { applyBuilderRules, type BuilderValidationRule } from './validationRuleApply'

function rule(rule: string, message = ''): BuilderValidationRule {
  return { fieldKey: 'amount', rule, message }
}

describe('R227 验证规则构建器规则应用', () => {
  test('required: 规则把字段标记为必填，保留原有校验', () => {
    expect(applyBuilderRules({ min: 0 }, [rule('required:')])).toEqual({ min: 0, required: true })
  })

  test('裸 required（addValidationRule 默认值，validateAll 也认它）同样标记为必填', () => {
    expect(applyBuilderRules(undefined, [rule('required')])).toEqual({ required: true })
  })

  test('min:/max: 规则解析为数字并写入上下界', () => {
    expect(applyBuilderRules(undefined, [rule('min:1'), rule('max:100')])).toEqual({ min: 1, max: 100 })
  })

  test('pattern: 规则保留正则原文（含冒号）并使用自定义提示', () => {
    const out = applyBuilderRules(undefined, [rule('pattern:^a:b$', '必须包含冒号')])
    expect(out.pattern).toBe('^a:b$')
    expect(out.patternMsg).toBe('必须包含冒号')
  })

  test('非数字参数不会污染 min/max 为 NaN', () => {
    const out = applyBuilderRules({ min: 5 }, [rule('min:abc')])
    expect(out.min).toBe(5)
    expect(Number.isNaN(out.min)).toBe(false)
    expect(Number.isNaN(out.max)).toBe(false)
  })

  test('不修改传入的 base 对象（避免就地污染历史快照）', () => {
    const base = { min: 1 }
    const out = applyBuilderRules(base, [rule('required:')])
    expect(base).toEqual({ min: 1 })
    expect(out).not.toBe(base)
  })
})
