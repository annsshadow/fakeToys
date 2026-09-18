import { describe, expect, it } from 'vitest'
import {
  computeGroupOrder,
  initialValues,
  type MobileFormField,
  type MobileFormGroup,
  parseMobileForm,
  parseSimpleForm,
  validateSimpleForm,
} from './form'

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

/** 构造一个「定义体藏在 definition / pcData / mobileData 里」的完整后端响应。 */
function envelope(kind: 'definition' | 'pcData' | 'mobileData', modules: Record<string, Record<string, unknown>>) {
  const moduleList: Record<string, unknown> = { m1: modules.m1 }
  if (kind === 'definition') {
    return { definition: { moduleList }, pcData: { json: '{}' } }
  }
  if (kind === 'pcData') {
    // 解析器要求 pcData.json 已是对象（object() 助手对字符串返回 {}）
    return { pcData: { json: { moduleList } } }
  }
  return { mobileData: { json: { moduleList } } }
}

describe('mobile form parser — moduleList location variants', () => {
  it('finds moduleList in a top-level JSON string definition', () => {
    // 后端有的端点直接返回 definition JSON 字符串；解析失败会整表单空白。
    const form = parseMobileForm(
      JSON.stringify({ moduleList: { t: { id: 't', type: 'text', name: 'n', label: 'N' } } }),
    )
    expect(form.fields).toHaveLength(1)
    expect(form.fields[0].key).toBe('n')
  })

  it('finds moduleList inside a nested definition object (with string JSON body)', () => {
    const form = parseMobileForm(envelope('definition', { m1: { id: 't', type: 'text', name: 'n', label: 'N' } }))
    expect(form.fields.map((f) => f.key)).toEqual(['n'])
    // definition 也接受 JSON 字符串形态
    const form2 = parseMobileForm({
      definition: JSON.stringify({ moduleList: { m1: { id: 't', type: 'text', name: 'n', label: 'N' } } }),
    })
    expect(form2.fields.map((f) => f.key)).toEqual(['n'])
  })

  it('finds moduleList inside pcData.json and mobileData.json (PC definitions first)', () => {
    const modules = { m1: { id: 't', type: 'text', name: 'n', label: 'N' } }
    expect(parseMobileForm(envelope('pcData', modules)).fields).toHaveLength(1)
    expect(parseMobileForm(envelope('mobileData', modules)).fields).toHaveLength(1)
  })

  it('falls back to {} for unrecognized envelopes instead of crashing', () => {
    expect(parseMobileForm({ bogus: 1 })).toEqual({ fields: [], groups: [] })
    expect(parseMobileForm(null)).toEqual({ fields: [], groups: [] })
  })
})

describe('mobile form fields — options, grouping, defaults', () => {
  it('parses select options from both object arrays and optionsStr lines', () => {
    const form = parseMobileForm(
      def({
        s1: { id: 's1', type: 'select', name: 'city', label: '城市', options: [{ value: 'sh', label: '上海' }, 'gz'] },
        s2: { id: 's2', type: 'select', name: 'level', label: '级别', optionsStr: 'l1|一级\nl2|二级\n\n' },
      }),
    )
    const [city, level] = form.fields
    expect(city.options).toEqual([
      { value: 'sh', label: '上海' },
      { value: 'gz', label: 'gz' }, // 无 label 的项回退为 value
    ])
    expect(level.options).toEqual([
      { value: 'l1', label: '一级' },
      { value: 'l2', label: '二级' },
    ])
  })

  it('groups fields under the nearest layout container via pid', () => {
    // pid 串按「最外层父 → 最近父」顺序逐个找容器，首个命中的容器胜出；
    // 容器自身不作为可提交字段。
    const form = parseMobileForm(
      def({
        c1: { id: 'c1', type: 'form', label: '基本信息' },
        c2: { id: 'c2', type: 'div', label: '嵌套组' },
        f1: { id: 'f1', type: 'text', name: 'a', label: 'A', pid: 'c1' },
        f2: { id: 'f2', type: 'text', name: 'b', label: 'B', pid: ['c1', 'c2'] },
        f3: { id: 'f3', type: 'text', name: 'c', label: 'C', pid: ['c2'] },
      }),
    )
    expect(form.fields.map((f) => [f.groupId, f.groupLabel])).toEqual([
      ['c1', '基本信息'],
      ['c1', '基本信息'], // pid 数组中 c1 先命中
      ['c2', '嵌套组'],
    ])
    // 容器不出现在 fields 中
    expect(form.fields.map((f) => f.key)).toEqual(['a', 'b', 'c'])
  })

  it('skips hidden modules and non-renderable types (Label/static)', () => {
    const form = parseMobileForm(
      def({
        h: { id: 'h', type: 'text', name: 'hidden', label: 'H', hidden: true },
        l: { id: 'l', type: 'Label', name: 'hint', label: '提示' },
        ok: { id: 'ok', type: 'number', name: 'n', label: 'N' },
      }),
    )
    expect(form.fields.map((f) => f.key)).toEqual(['n'])
  })

  it('honors defaultValue object form and readonly/disabled flags', () => {
    const form = parseMobileForm(
      def({
        a: { id: 'a', type: 'text', name: 'a', label: 'A', defaultValue: { value: 'preset' } },
        b: { id: 'b', type: 'text', name: 'b', label: 'B', readonly: true },
        c: { id: 'c', type: 'text', name: 'c', label: 'C', disabled: true },
      }),
    )
    const by = Object.fromEntries(form.fields.map((f) => [f.key, f]))
    expect(by.a.defaultValue).toBe('preset')
    expect(by.b.readonly).toBe(true)
    expect(by.c.readonly).toBe(true)
  })

  it('initialValues seeds every field key with its default', () => {
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
        defaultValue: 'x',
        groupId: '',
        groupLabel: '',
      },
      {
        id: 'b',
        key: 'b',
        label: 'B',
        type: 'checkbox',
        required: false,
        readonly: false,
        options: [],
        placeholder: '',
        defaultValue: '',
        groupId: '',
        groupLabel: '',
      },
    ]
    expect(initialValues(fields)).toEqual({ a: 'x', b: '' })
  })

  it('validateSimpleForm flags only missing required values', () => {
    const fields: MobileFormField[] = [
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
      {
        id: 'o',
        key: 'note',
        label: '备注',
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
    expect(validateSimpleForm(fields, {})).toEqual({ reason: '申请原因 为必填项' })
    expect(validateSimpleForm(fields, { reason: 'ok' })).toEqual({})
    // 附件/复选值以数组形式提交：空数组按未填处理
    const attachFields: MobileFormField[] = [
      {
        id: 'f',
        key: 'attach',
        label: '附件',
        type: 'attachment',
        required: true,
        readonly: false,
        options: [],
        placeholder: '',
        defaultValue: '',
        groupId: '',
        groupLabel: '',
      },
    ]
    expect(validateSimpleForm(attachFields, { attach: [] as unknown as string })).toEqual({
      attach: '附件 为必填项',
    })
    expect(validateSimpleForm(attachFields, { attach: '1,2' })).toEqual({})
  })

  it('parseSimpleForm returns the flat field list (backward-compatible API)', () => {
    const form = parseMobileForm(def({ t: { id: 't', type: 'text', name: 'n', label: 'N' } }))
    expect(parseSimpleForm(def({ t: { id: 't', type: 'text', name: 'n', label: 'N' } }))).toEqual(form.fields)
  })
})
