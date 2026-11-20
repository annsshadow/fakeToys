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

describe('validateSimpleForm 必填附件与上传态一致（R257）', () => {
  const attach = [{ ...requiredField('attach', '证明材料'), type: 'attachment' as const }]

  it('已上传的必填附件不再被误判为未填', () => {
    // start.vue 把上传结果放在 attachments[field.key]，不写回 values；校验必须认这个来源，
    // 否则用户上传附件后仍被"为必填项"拦住，永远无法发起带必填附件的流程。
    expect(validateSimpleForm(attach, {}, { attach: [{ id: 'f1', name: 'proof.pdf' }] })).toEqual({})
  })

  it('附件数组为空仍判未填', () => {
    expect(validateSimpleForm(attach, {}, { attach: [] })).toEqual({
      attach: '证明材料 为必填项',
    })
  })
})
