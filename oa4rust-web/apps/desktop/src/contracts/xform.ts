export type FormMode = 'desktop' | 'mobile'
export type FormValue = string | number | boolean | string[] | null

export interface XformScript {
  code: string
  html?: string
}

export interface XformModule extends Record<string, unknown> {
  id: string
  type: string
  name?: string
  label?: string
  key?: string
  pid?: string
  children?: string[]
  required?: boolean
  readonly?: boolean
  disabled?: boolean
  hidden?: boolean
  options?: Array<{ value: string; label: string }>
  events?: Record<string, XformScript>
  validation?: Record<string, unknown> | XformScript
}

export interface XformLayout {
  mode: FormMode
  root: string[]
  domTree: XformDomNode[]
  columns: number
  gutter: number
}

export interface XformDomNode {
  id: string
  children?: XformDomNode[]
}

export interface XformDefinition extends Record<string, unknown> {
  $version: string
  name: string
  application: string
  moduleList: Record<string, XformModule>
  layouts: Record<FormMode, XformLayout>
  actions: Record<string, XformScript>
  events: Record<string, XformScript>
  validation: Record<string, unknown>
}

function object(value: unknown): Record<string, unknown> {
  return value !== null && typeof value === 'object' && !Array.isArray(value) ? (value as Record<string, unknown>) : {}
}

function parseJson(value: string): unknown {
  try {
    return JSON.parse(value)
  } catch {
    return {}
  }
}

function domRootIds(html: unknown, modules: Record<string, XformModule>): string[] {
  if (typeof html !== 'string' || typeof DOMParser === 'undefined') return Object.keys(modules)
  const doc = new DOMParser().parseFromString(html, 'text/html')
  const ids = Array.from(doc.body.querySelectorAll('[id]'))
    .map((node) => node.id)
    .filter((id) => modules[id])
  return ids.filter((id) => !ids.some((candidate) => candidate !== id && modules[id]?.pid?.includes(candidate)))
}

function domTree(ids: string[], modules: Record<string, XformModule>): XformDomNode[] {
  const visit = (id: string, seen: Set<string>): XformDomNode => {
    if (seen.has(id)) return { id }
    const next = new Set(seen).add(id)
    const explicit = Array.isArray(modules[id]?.children) ? (modules[id].children as string[]) : []
    const inferred = Object.values(modules)
      .filter((module) => module.pid?.includes(id))
      .map((module) => module.id)
    const children = [...new Set([...explicit, ...inferred])].filter((child) => modules[child])
    return children.length ? { id, children: children.map((child) => visit(child, next)) } : { id }
  }
  return ids.map((id) => visit(id, new Set()))
}

export function parseFormDefinition(input: unknown): XformDefinition {
  const envelope = object(typeof input === 'string' ? parseJson(input) : input)
  const source = object(
    envelope.definition
      ? typeof envelope.definition === 'string'
        ? parseJson(envelope.definition)
        : envelope.definition
      : envelope,
  )
  const pcData = object(source.pcData)
  const mobileData = object(source.mobileData)
  const root = Object.keys(pcData).length ? object(pcData.json) : source
  const rawModules = object(root.moduleList)
  const moduleList = Object.fromEntries(
    Object.entries(rawModules).map(([id, value]) => {
      const module = object(value)
      return [
        id,
        { ...module, id: String(module.id || id), type: String(module.type || module.mwftype || 'Textfield') },
      ]
    }),
  ) as Record<string, XformModule>
  const mobileRoot = object(mobileData.json)
  const desktopIds = domRootIds(pcData.html ?? source.html, moduleList)
  const mobileIds = domRootIds(mobileData.html, moduleList)
  const layouts = object(root.layouts)
  return {
    ...root,
    $version: String(root.$version || '5.2'),
    name: String(root.name || envelope.name || ''),
    application: String(root.application || envelope.appId || ''),
    moduleList,
    layouts: {
      desktop: normalizeLayout(object(layouts.desktop), 'desktop', desktopIds, moduleList),
      mobile: normalizeLayout(object(layouts.mobile), 'mobile', mobileIds.length ? mobileIds : desktopIds, moduleList),
    },
    actions: scripts(root.actions),
    events: scripts(root.events),
    validation: object(root.validation),
    ...(Object.keys(mobileRoot).length ? { mobileData: { ...mobileData, json: mobileRoot } } : {}),
  }
}

function scripts(value: unknown): Record<string, XformScript> {
  return Object.fromEntries(
    Object.entries(object(value)).map(([key, script]) => {
      const body = typeof script === 'string' ? { code: script } : object(script)
      return [key, { code: String(body.code || ''), html: String(body.html || body.code || '') }]
    }),
  )
}

function normalizeLayout(
  value: Record<string, unknown>,
  mode: FormMode,
  fallback: string[],
  modules: Record<string, XformModule>,
): XformLayout {
  const root = Array.isArray(value.root) ? value.root.map(String).filter((id) => modules[id]) : fallback
  return {
    mode,
    root,
    domTree: Array.isArray(value.domTree) ? (value.domTree as XformDomNode[]) : domTree(root, modules),
    columns: Number(value.columns) || 1,
    gutter: Number(value.gutter) || 16,
  }
}

export interface DesignerField extends Record<string, unknown> {
  id: string
  type: string
  label: string
  key: string
  placeholder?: string
  defaultValue?: string
  required?: boolean
  readonly?: boolean
  disabled?: boolean
  hidden?: boolean
  optionsStr?: string
  validation?: Record<string, unknown>
}

const FIELD_TO_MODULE: Record<string, string> = {
  text: 'Textfield',
  textarea: 'Textarea',
  number: 'Number',
  date: 'Calendar',
  datetime: 'Calendar',
  select: 'Select',
  checkbox: 'Radio',
  checkbox_group: 'Checkbox',
  file: 'Attachment',
  upload: 'Attachment',
  email: 'Textfield',
  phone: 'Textfield',
  switch: 'Select',
  rating: 'Number',
  slider: 'Number',
  section: 'Div',
  divider: 'Div',
  spacer: 'Div',
  html: 'Html',
  signature: 'Image',
  image: 'Image',
}

const MODULE_TO_FIELD: Record<string, string> = {
  textfield: 'text',
  ooinput: 'text',
  textarea: 'textarea',
  ootextarea: 'textarea',
  number: 'number',
  currency: 'number',
  calendar: 'date',
  oodatetime: 'date',
  select: 'select',
  ooselect: 'select',
  radio: 'checkbox',
  ooradiogroup: 'checkbox',
  checkbox: 'checkbox_group',
  oocheckgroup: 'checkbox_group',
  attachment: 'file',
  oofiles: 'file',
  org: 'text',
  ooorg: 'text',
  label: 'html',
  html: 'html',
  div: 'section',
}

function parseOptions(value?: string): Array<{ value: string; label: string }> {
  return (value || '')
    .split('\n')
    .filter(Boolean)
    .map((line) => {
      const [option, label = option] = line.split('|')
      return { value: option.trim(), label: label.trim() }
    })
}

export function definitionToDesignerFields(definition: XformDefinition): DesignerField[] {
  return Object.values(definition.moduleList)
    .filter((module) => !['Form', 'Actionbar', 'Datatable$Title', 'Datatable$Data'].includes(module.type))
    .map((module) => ({
      ...module,
      id: module.id,
      type: MODULE_TO_FIELD[module.type.toLowerCase()] ?? 'text',
      label: String(module.label || module.name || module.id),
      key: String(module.key || module.name || module.id),
      placeholder: String(module.placeholder || ''),
      defaultValue: String(object(module.defaultValue).value ?? module.defaultValue ?? ''),
      required: Boolean(module.required),
      readonly: Boolean(module.readonly),
      disabled: Boolean(module.disabled),
      hidden: Boolean(module.hidden),
      optionsStr: Array.isArray(module.options)
        ? module.options.map((option) => `${option.value}|${option.label}`).join('\n')
        : '',
      validation: object(module.validation),
    }))
}

export interface SerializeFormOptions {
  name: string
  application: string
  fields: DesignerField[]
  desktop?: Partial<XformLayout>
  mobile?: Partial<XformLayout>
  actions?: Record<string, XformScript>
  events?: Record<string, XformScript>
  validation?: Record<string, unknown>
  base?: Partial<XformDefinition>
}

export function serializeFormDefinition(options: SerializeFormOptions): XformDefinition {
  const previous = options.base?.moduleList ?? {}
  const moduleList = Object.fromEntries(
    options.fields.map((field) => {
      const module: XformModule = {
        ...(previous[field.id] ?? {}),
        ...field,
        id: field.id,
        type: String(field.moduleType || FIELD_TO_MODULE[field.type] || 'Textfield'),
        name: field.key,
        label: field.label,
        placeholder: field.placeholder,
        defaultValue: { value: field.defaultValue ?? '', code: '', html: '' },
        required: Boolean(field.required || field.validation?.required),
        readonly: Boolean(field.readonly),
        disabled: Boolean(field.disabled),
        hidden: Boolean(field.hidden),
        options: parseOptions(field.optionsStr),
        validation: field.validation ?? {},
        events: scripts(field.events),
      }
      delete module.key
      delete module.optionsStr
      return [field.id, module]
    }),
  )
  const root = options.fields.map((field) => field.id)
  const desktop = normalizeLayout(object(options.desktop), 'desktop', root, moduleList)
  const mobile = normalizeLayout(object(options.mobile), 'mobile', root, moduleList)
  return {
    ...(options.base ?? {}),
    $version: String(options.base?.$version || '5.2'),
    name: options.name,
    application: options.application,
    moduleList,
    layouts: { desktop, mobile },
    actions: options.actions ?? options.base?.actions ?? {},
    events: options.events ?? options.base?.events ?? {},
    validation: options.validation ?? options.base?.validation ?? {},
    domTree: desktop.domTree,
    desktop,
    mobile,
  }
}

export function formSavePayload(
  definition: XformDefinition,
  meta: { name: string; appId: string; status?: string },
): { name: string; appId: string; definition: string; status: string } {
  if (!meta.appId.trim()) throw new Error('appId required')
  return {
    name: meta.name,
    appId: meta.appId,
    definition: JSON.stringify(definition),
    status: meta.status || 'draft',
  }
}

export function initialFormValues(definition: XformDefinition, source: unknown = {}): Record<string, FormValue> {
  const data = object(source)
  return Object.fromEntries(
    Object.values(definition.moduleList).map((module) => {
      const key = String(module.name || module.key || module.id)
      const rawDefault = object(module.defaultValue).value ?? module.defaultValue
      const fallback = module.type.toLowerCase().includes('checkbox') ? [] : (rawDefault ?? null)
      return [key, (data[key] as FormValue) ?? (fallback as FormValue)]
    }),
  )
}

export function validateFormValues(
  definition: XformDefinition,
  values: Record<string, FormValue>,
): Record<string, string> {
  const errors: Record<string, string> = {}
  for (const module of Object.values(definition.moduleList)) {
    const key = String(module.name || module.key || module.id)
    const value = values[key]
    const validation = object(module.validation)
    const required = Boolean(module.required || validation.required)
    if (required && (value === null || value === '' || (Array.isArray(value) && !value.length))) {
      errors[key] = String(validation.message || `${module.label || key}为必填项`)
    }
    const pattern = typeof validation.pattern === 'string' ? validation.pattern : ''
    if (!errors[key] && pattern && typeof value === 'string') {
      try {
        if (!new RegExp(pattern).test(value)) errors[key] = String(validation.patternMsg || '格式不正确')
      } catch {
        errors[key] = '校验表达式无效'
      }
    }
  }
  return errors
}
