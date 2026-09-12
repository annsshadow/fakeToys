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
})
