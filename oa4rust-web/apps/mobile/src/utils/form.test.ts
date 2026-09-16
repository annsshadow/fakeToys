import { describe, expect, it } from 'vitest'
import { computeGroupOrder, type MobileFormField, type MobileFormGroup, parseMobileForm } from './form'

/** 构造一个 CMS x_cms_form.definition 形态（moduleList: Record<id, module>）。 */
function def(modules: Record<string, Record<string, unknown>>): unknown {
  return { moduleList: modules, desktopData: { html: '', columns: 1 }, mobileData: { html: '', columns: 1 } }
}

describe('mobile form parser (parseMobileForm)', () => {
  it('bare "text" module type maps to a text field (regression: was silently dropped)', () => {
    const form = parseMobileForm(
      def({
        'module-reason': { id: 'module-reason', type: 'text', name: 'reason', label: '申请原因', required: true },
      }),
    )
    // 必填文本字段必须存在，否则提交时"必填项缺失"会被漏判。
    expect(form.fields).toHaveLength(1)
    expect(form.fields[0]).toMatchObject({ key: 'reason', type: 'text', required: true })
  })

  it('attachment-family module types all map to "attachment"', () => {
    const form = parseMobileForm(
      def({
        m1: { id: 'm1', type: 'oofiles', name: 'attach', label: '证明材料' },
        m2: { id: 'm2', type: 'file', name: 'f', label: 'file' },
        m3: { id: 'm3', type: 'attachment', name: 'a', label: 'attach' },
      }),
    )
    expect(form.fields.map((f) => f.type)).toEqual(['attachment', 'attachment', 'attachment'])
  })

  it('flat form (no layout container) yields a single default group', () => {
    const form = parseMobileForm(
      def({
        d: { id: 'd', type: 'number', name: 'days', label: '请假天数' },
        r: { id: 'r', type: 'text', name: 'reason', label: '申请原因' },
      }),
    )
    // 无容器字段 groupId='' → groups 仅含默认组一项，而非重复。
    expect(form.groups).toEqual([{ id: '', label: '' }])
    expect(form.fields.every((f) => f.groupId === '')).toBe(true)
  })
})

describe('mobile form group order (computeGroupOrder)', () => {
  const flatFields: MobileFormField[] = [
    {
      id: 'd',
      key: 'days',
      label: '请假天数',
      type: 'number',
      required: false,
      readonly: false,
      options: [],
      placeholder: '',
      defaultValue: '',
      groupId: '',
      groupLabel: '',
    },
    {
      id: 'r',
      key: 'reason',
      label: '申请原因',
      type: 'text',
      required: true,
      readonly: false,
      options: [],
      placeholder: '',
      defaultValue: '',
      groupId: '',
      groupLabel: '',
    },
  ]

  it('flat form renders the default group exactly once (regression: fields were rendered twice)', () => {
    // parseMobileForm 对扁平表单返回 groups=[{id:'',label:''}]；若不去重会产出 ['',''] 导致 v-for 同键重复渲染。
    const groups: MobileFormGroup[] = [{ id: '', label: '' }]
    expect(computeGroupOrder(flatFields, groups)).toEqual([''])
  })

  it('forces the default group to the front when groups omits it but default fields exist', () => {
    // groups 不含 '' 但字段中存在 groupId='' 的默认字段 → 默认组补到最前。
    const fields: MobileFormField[] = [
      {
        id: 'a',
        key: 'a',
        label: 'A',
        type: 'text',
        required: false,
        readonly: false,
        options: [],
        placeholder: '',
        defaultValue: '',
        groupId: 'g1',
        groupLabel: '第一组',
      },
      {
        id: 'd',
        key: 'd',
        label: 'D',
        type: 'text',
        required: false,
        readonly: false,
        options: [],
        placeholder: '',
        defaultValue: '',
        groupId: '',
        groupLabel: '',
      },
    ]
    const groups: MobileFormGroup[] = [{ id: 'g1', label: '第一组' }]
    expect(computeGroupOrder(fields, groups)).toEqual(['', '第一组'])
  })

  it('does not duplicate the default group when groups already contains it', () => {
    const fields = flatFields
    const groups: MobileFormGroup[] = [
      { id: '', label: '' },
      { id: 'g1', label: '第一组' },
    ]
    expect(computeGroupOrder(fields, groups)).toEqual(['', '第一组'])
  })
})
