// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { describe, expect, it } from 'vitest'
import { type MobileFormField, parseMobileForm, validateSimpleForm } from './form'

function numberField(key: string, label: string, required = false): MobileFormField {
  return {
    id: key,
    key,
    label,
    type: 'number',
    required,
    readonly: false,
    options: [],
    placeholder: '',
    defaultValue: '',
    groupId: '',
    groupLabel: '',
  }
}

describe('validateSimpleForm 数字字段校验（R287）', () => {
  it('必填数字字段填非数字被拦截，不能静默提交给后端', () => {
    const fields = [numberField('days', '请假天数', true)]
    expect(validateSimpleForm(fields, { days: 'abc' })).toEqual({ days: '请假天数 不是有效数字' })
    expect(validateSimpleForm(fields, { days: '1.2.3' })).toEqual({ days: '请假天数 不是有效数字' })
  })

  it('可选数字字段填非数字同样拦截，避免脏值进入 payload', () => {
    const fields = [numberField('score', '评分')]
    expect(validateSimpleForm(fields, { score: 'abc' })).toEqual({ score: '评分 不是有效数字' })
  })

  it('合法数字值通过校验：0、负数、小数与带空格数字', () => {
    const fields = [numberField('score', '评分', true)]
    for (const v of ['0', '-3', '3.14', ' 12 ', '1e3']) {
      expect(validateSimpleForm(fields, { score: v })).toEqual({})
    }
  })

  it('可选数字字段留空不报错，必填数字为空仍优先报必填', () => {
    expect(validateSimpleForm([numberField('score', '评分')], { score: '' })).toEqual({})
    expect(validateSimpleForm([numberField('score', '评分')], {})).toEqual({})
    expect(validateSimpleForm([numberField('score', '评分', true)], { score: '  ' })).toEqual({
      score: '评分 为必填项',
    })
  })

  it('可达链：parseMobileForm 解析 number/currency 模块后，非数字值在提交前被拦截', () => {
    const form = parseMobileForm({
      moduleList: {
        n: { id: 'n', type: 'number', name: 'days', label: '请假天数', required: true },
        c: { id: 'c', type: 'currency', name: 'amount', label: '报销金额', required: false },
      },
    })
    expect(form.fields.map((f) => f.type)).toEqual(['number', 'number'])
    expect(validateSimpleForm(form.fields, { days: '3', amount: '12.5' })).toEqual({})
    expect(validateSimpleForm(form.fields, { days: '3', amount: '很多' })).toEqual({
      amount: '报销金额 不是有效数字',
    })
  })
})
