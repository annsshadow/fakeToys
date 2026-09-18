import { describe, expect, it } from 'vitest'
import {
  definitionToDesignerFields,
  formSavePayload,
  initialFormValues,
  parseFormDefinition,
  serializeFormDefinition,
  validateFormValues,
} from './xform'

describe('Xform moduleList contract', () => {
  it('round-trips desktop/mobile layouts, DOM tree, actions, events and validation', () => {
    const definition = serializeFormDefinition({
      name: '报销表单',
      application: 'app-1',
      fields: [
        { id: 'subject', type: 'text', label: '标题', key: 'subject', required: true },
        { id: 'amount', type: 'number', label: '金额', key: 'amount', validation: { required: true } },
      ],
      desktop: { mode: 'desktop', root: ['subject', 'amount'], domTree: [], columns: 2, gutter: 20 },
      mobile: { mode: 'mobile', root: ['amount', 'subject'], domTree: [], columns: 1, gutter: 12 },
      actions: { submit: { code: 'submit()' } },
      events: { afterLoad: { code: 'loaded()' } },
      validation: { mode: 'strict' },
    })
    const payload = formSavePayload(definition, { name: '报销表单', appId: 'app-1' })
    const parsed = parseFormDefinition({ id: 'form-1', appId: 'app-1', definition: payload.definition })

    expect(parsed.moduleList.subject.type).toBe('Textfield')
    expect(parsed.layouts.desktop).toMatchObject({ root: ['subject', 'amount'], columns: 2, gutter: 20 })
    expect(parsed.layouts.mobile.root).toEqual(['amount', 'subject'])
    expect(parsed.actions.submit.code).toBe('submit()')
    expect(parsed.events.afterLoad.code).toBe('loaded()')
    expect(parsed.validation).toEqual({ mode: 'strict' })
    expect(definitionToDesignerFields(parsed).map((field) => field.key)).toEqual(['subject', 'amount'])
  })

  it('initializes values and rejects empty required fields before submission', () => {
    const definition = serializeFormDefinition({
      name: '申请',
      application: 'app-1',
      fields: [{ id: 'subject', type: 'text', label: '标题', key: 'subject', required: true }],
    })
    const values = initialFormValues(definition, {})
    expect(values).toEqual({ subject: '' })
    expect(validateFormValues(definition, values)).toEqual({ subject: '标题为必填项' })
    expect(validateFormValues(definition, { subject: '真实数据' })).toEqual({})
  })

  it('requires the real application id for create payloads', () => {
    const definition = serializeFormDefinition({ name: '申请', application: '', fields: [] })
    expect(() => formSavePayload(definition, { name: '申请', appId: '' })).toThrow('appId required')
  })

  it('validateFormValues enforces per-field pattern rules and flags broken regexes', () => {
    const definition = serializeFormDefinition({
      name: '校验',
      application: 'app-1',
      fields: [
        {
          id: 'age',
          type: 'number',
          label: '年龄',
          key: 'age',
          required: true,
          validation: { pattern: '^[0-9]+$', patternMsg: '必须是数字' },
        },
        {
          id: 'code',
          type: 'text',
          label: '编码',
          key: 'code',
          validation: { pattern: '([invalid' },
        },
      ],
    })
    const errors = validateFormValues(definition, { age: '12a', code: 'x' })
    expect(errors.age).toBe('必须是数字')
    expect(errors.code).toBe('校验表达式无效')
  })

  it('serializes optionsStr into the backend options array and back', () => {
    const definition = serializeFormDefinition({
      name: '选项',
      application: 'app-1',
      fields: [{ id: 'level', type: 'select', label: '级别', key: 'level', optionsStr: 'l1|一级\nl2|二级\n' }],
    })
    const moduleOptions = (
      definition.moduleList.level as unknown as { options: Array<{ value: string; label: string }> }
    ).options
    expect(moduleOptions).toEqual([
      { value: 'l1', label: '一级' },
      { value: 'l2', label: '二级' },
    ])
    // 反向：module.options 数组 → 设计器表格的 optionsStr
    expect(definitionToDesignerFields(definition).find((field) => field.key === 'level')?.optionsStr).toBe(
      'l1|一级\nl2|二级',
    )
  })

  it('survives malformed JSON definition envelopes with an empty module list', () => {
    const parsed = parseFormDefinition('{definitely not json')
    expect(parsed.moduleList).toEqual({})
    expect(parsed.$version).toBe('5.2')
    expect(parsed.name).toBe('')
  })

  it('domTree follows pid-inferred children and terminates on cyclic parent references', () => {
    const parsed = parseFormDefinition({
      moduleList: {
        a: { id: 'a', type: 'Div', pid: 'b', children: ['b'] },
        b: { id: 'b', type: 'Textfield', pid: 'a' },
      },
    })
    const tree = parsed.layouts.desktop.domTree
    expect(JSON.parse(JSON.stringify(tree))).toEqual([
      { id: 'a', children: [{ id: 'b', children: [{ id: 'a' }] }] },
      { id: 'b', children: [{ id: 'a', children: [{ id: 'b' }] }] },
    ])
  })
})
