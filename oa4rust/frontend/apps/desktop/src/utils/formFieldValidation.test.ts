// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { describe, expect, it } from 'vitest'
import { validateFieldValue } from './formFieldValidation'

describe('validateFieldValue：表单字段校验边界（R168）', () => {
  it('必填字段：0 / false / 空数组都是已填写的有效值，只有 null/undefined/空串/空集合算空', () => {
    const f = (over: Record<string, unknown> = {}) => ({ label: '数量', required: true, ...over })
    expect(validateFieldValue(f(), 0)).toBeNull()
    expect(validateFieldValue(f(), '0')).toBeNull()
    expect(validateFieldValue(f(), false)).toBeNull()
    expect(validateFieldValue(f(), [1, 2])).toBeNull()
    for (const blank of [undefined, null, '', '   ']) {
      expect(validateFieldValue(f(), blank)).toBe('数量 不能为空')
    }
    expect(validateFieldValue(f(), [])).toBe('数量 不能为空')
  })

  it('数值边界：0 也参与 min/max 比较，非数字与 NaN 不绕过范围校验', () => {
    expect(validateFieldValue({ label: '分数', min: 1 }, 0)).toBe('分数 不能小于1')
    expect(validateFieldValue({ label: '分数', max: 0 }, 1)).toBe('分数 不能大于0')
    expect(validateFieldValue({ label: '分数', min: 1 }, 'abc')).toBe('分数 不是有效数字')
    expect(validateFieldValue({ label: '分数', max: 100 }, Number.NaN)).toBe('分数 不是有效数字')
    expect(validateFieldValue({ label: '分数', min: 1 }, 5)).toBeNull()
  })

  it('缺失值不应被 min/max/pattern/length 规则误报', () => {
    for (const blank of [undefined, null, '']) {
      expect(validateFieldValue({ label: '可选', min: 1, max: 9, minLength: 2, pattern: '^\\d+$' }, blank)).toBeNull()
    }
  })

  it('格式错误有提示，非法正则不抛异常而是给出稳定错误', () => {
    expect(validateFieldValue({ label: '编码', pattern: '^\\d+$' }, 'abc')).toBe('编码 格式错误')
    expect(validateFieldValue({ label: '编码', pattern: '(' }, 'abc')).toBe('编码 校验表达式无效')
  })

  it('长度边界', () => {
    expect(validateFieldValue({ label: '名称', minLength: 2 }, 'a')).toBe('名称 至少2字符')
    expect(validateFieldValue({ label: '名称', maxLength: 2 }, 'abcd')).toBe('名称 最多2字符')
  })
})
