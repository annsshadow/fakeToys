// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { describe, expect, it } from 'vitest'
import { type MobileFormField, validateSimpleForm } from './form'

function requiredField(key: string, label: string): MobileFormField {
  return {
    id: key,
    key,
    label,
    type: 'text',
    required: true,
    readonly: false,
    options: [],
    placeholder: '',
    defaultValue: '',
    groupId: '',
    groupLabel: '',
  }
}

function optionalField(key: string, label: string): MobileFormField {
  return { ...requiredField(key, label), required: false }
}

describe('validateSimpleForm 必填空白判定（R246）', () => {
  it('纯空格/制表符/换行的必填值按未填处理，不再绕过校验', () => {
    const fields = [requiredField('reason', '申请原因')]
    for (const blank of ['   ', '\t', '\n \t', ' \r\n ']) {
      expect(validateSimpleForm(fields, { reason: blank })).toEqual({
        reason: '申请原因 为必填项',
      })
    }
  })

  it('含非空白字符的值仍判为已填，trim 不误伤正常输入', () => {
    const fields = [requiredField('reason', '申请原因')]
    expect(validateSimpleForm(fields, { reason: '  已填写  ' })).toEqual({})
    expect(validateSimpleForm(fields, { reason: '0' })).toEqual({})
  })

  it('空串与空数组的既有判定保留（回归保护）', () => {
    const fields = [requiredField('reason', '申请原因')]
    expect(validateSimpleForm(fields, { reason: '' })).toEqual({ reason: '申请原因 为必填项' })
    expect(validateSimpleForm(fields, {})).toEqual({ reason: '申请原因 为必填项' })
    const attach = [{ ...requiredField('attach', '附件'), type: 'attachment' as const }]
    expect(validateSimpleForm(attach, { attach: [] as unknown as string })).toEqual({
      attach: '附件 为必填项',
    })
  })

  it('非必填字段空白不报错；一组字段中只报真正空白的必填项', () => {
    const fields = [requiredField('reason', '申请原因'), optionalField('note', '备注')]
    expect(validateSimpleForm(fields, { note: '   ' })).toEqual({ reason: '申请原因 为必填项' })
    expect(validateSimpleForm(fields, { reason: 'ok', note: '   ' })).toEqual({})
  })
})
