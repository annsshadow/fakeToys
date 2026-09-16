/**
 * 移动端「发起流程」表单解析（简版渲染 + 复杂控件）。
 *
 * 桌面 XformRuntime 依赖 DOM（DOMParser）与完整控件集，不适配 uni-app 跨端。
 * 这里做"容错解析 + 移动端可渲染"：从 `GET /jaxrs/form/{flag}` 返回的表单定义
 * 中提取字段，字段 key/label/必填/选项/默认值均取自后端 moduleList，不做虚构。
 * 与桌面 contracts/xform.ts 的 moduleList 契约保持一致（type 归一化到小写匹配）。
 *
 * 复杂控件（P3）：
 *  - 附件（Attachment / Oofiles / Image / Signature）→ type='attachment'，值存 FILE_FILE 附件 id（逗号串）。
 *  - 富文本（Html / RichText）→ type='richtext'，移动端降级为多行文本编辑（不含 DOM 富文本引擎）。
 *  - 布局容器（Div / Form / Actionbar / Datatable$*）→ 不作为可提交字段，而是按 pid 层级
 *    对可提交字段分组（groupLabel），视图按分组渲染分区卡片。
 */

export type MobileFieldType =
  | 'text'
  | 'number'
  | 'textarea'
  | 'date'
  | 'select'
  | 'checkbox'
  | 'attachment'
  | 'richtext'

export interface MobileFormOption {
  value: string
  label: string
}

export interface MobileFormField {
  id: string
  key: string
  label: string
  type: MobileFieldType
  required: boolean
  readonly: boolean
  options: MobileFormOption[]
  placeholder: string
  defaultValue: string
  /** 所属布局容器 id（无容器时为 ''）。 */
  groupId: string
  /** 所属布局容器标题（无容器时为 ''）。 */
  groupLabel: string
}

export interface MobileFormGroup {
  id: string
  label: string
}

export interface MobileForm {
  fields: MobileFormField[]
  groups: MobileFormGroup[]
}

function object(value: unknown): Record<string, unknown> {
  return value !== null && typeof value === 'object' && !Array.isArray(value) ? (value as Record<string, unknown>) : {}
}

function parseJson(value: string): unknown {
  try {
    return JSON.parse(value)
  } catch {
    return null
  }
}

/** 从多种可能形态中定位 moduleList（Record<id, module>）。 */
function findModuleList(raw: unknown): Record<string, Record<string, unknown>> {
  let top: unknown = raw
  if (typeof raw === 'string') {
    const parsed = parseJson(raw)
    if (parsed === null) return {}
    top = parsed
  }
  const envelope = object(top)
  const direct = object(envelope.moduleList)
  if (Object.keys(direct).length) return normalizeModules(direct)
  const definition = envelope.definition
  if (definition !== undefined) {
    const defObj = object(typeof definition === 'string' ? parseJson(definition) : definition)
    const defModules = object(defObj.moduleList)
    if (Object.keys(defModules).length) return normalizeModules(defModules)
  }
  const pcData = object(envelope.pcData)
  const pcModules = object(object(pcData.json).moduleList)
  if (Object.keys(pcModules).length) return normalizeModules(pcModules)
  const mobileData = object(envelope.mobileData)
  const mobileModules = object(object(mobileData.json).moduleList)
  if (Object.keys(mobileModules).length) return normalizeModules(mobileModules)
  return {}
}

function normalizeModules(modules: Record<string, unknown>): Record<string, Record<string, unknown>> {
  return Object.fromEntries(
    Object.entries(modules).map(([id, value]) => {
      const module = object(value)
      return [id, { ...module, id: String(module.id || id) }]
    }),
  )
}

/** 布局容器类型（不作为可提交字段，仅用于分组）。 */
const CONTAINER_TYPES = new Set(['form', 'actionbar', 'div', 'oocontainer', 'datatable$title', 'datatable$data'])

function typeOf(module: Record<string, unknown>): string {
  return String(module.type || module.mwftype || 'Textfield').toLowerCase()
}

function isContainer(module: Record<string, unknown>): boolean {
  const t = typeOf(module)
  if (CONTAINER_TYPES.has(t)) return true
  // 无 key/value 且带子节点的纯布局模块
  const hasChildren = Array.isArray(module.children) && (module.children as unknown[]).length > 0
  const noKey = !module.name && !module.key
  return hasChildren && noKey
}

/** module type（小写）→ 移动端字段类型；容器/不可渲染类型返回 null。 */
function uiTypeOf(module: Record<string, unknown>): MobileFieldType | null {
  const raw = typeOf(module)
  switch (raw) {
    case 'textfield':
    case 'text':
    case 'ooinput':
    case 'org':
    case 'ooorg':
      return 'text'
    case 'number':
    case 'currency':
      return 'number'
    case 'textarea':
    case 'ootextarea':
      return 'textarea'
    case 'calendar':
    case 'oodatetime':
      return 'date'
    case 'select':
    case 'ooselect':
      return 'select'
    case 'radio':
    case 'ooradiogroup':
    case 'checkbox':
    case 'oocheckgroup':
      return 'checkbox'
    case 'attachment':
    case 'oofiles':
    case 'file':
    case 'upload':
    case 'image':
    case 'signature':
      return 'attachment'
    case 'html':
    case 'richtext':
    case 'oohtml':
      return 'richtext'
    default:
      // Form / Actionbar / Div / Label / Datatable$* 等：容器或静态文本，跳过
      return null
  }
}

function parseOptions(module: Record<string, unknown>): MobileFormOption[] {
  const raw = module.options
  if (Array.isArray(raw)) {
    return raw
      .map((o) => {
        const item = object(o)
        const value = String(item.value ?? o ?? '')
        const label = String(item.label ?? value)
        return { value, label }
      })
      .filter((o) => o.value !== '')
  }
  const optionsStr = String(module.optionsStr || '')
  return optionsStr
    .split('\n')
    .filter(Boolean)
    .map((line) => {
      const [value, label = value] = line.split('|')
      return { value: value.trim(), label: label.trim() }
    })
    .filter((o) => o.value !== '')
}

/** 由 pid（父 id 串）与 children 建立 字段 → 最近布局容器 的映射。 */
function resolveGroup(
  module: Record<string, unknown>,
  modules: Record<string, Record<string, unknown>>,
): MobileFormGroup {
  const pids: string[] = []
  const rawPid = module.pid
  if (typeof rawPid === 'string' && rawPid) pids.push(rawPid)
  else if (Array.isArray(rawPid)) for (const p of rawPid) pids.push(String(p))
  // 从最外层父到最近父逐个找容器
  for (const pid of pids) {
    const parent = modules[pid]
    if (parent && isContainer(parent)) {
      const label = String(parent.label || parent.name || pid)
      return { id: pid, label }
    }
  }
  return { id: '', label: '' }
}

export function parseMobileForm(raw: unknown): MobileForm {
  const modules = findModuleList(raw)
  const fields: MobileFormField[] = []
  const seenGroups = new Map<string, MobileFormGroup>()

  for (const module of Object.values(modules)) {
    if (module.hidden === true) continue
    if (isContainer(module)) continue
    const uiType = uiTypeOf(module)
    if (!uiType) continue
    const id = String(module.id)
    const key = String(module.name || module.key || id)
    if (!key) continue
    const group = resolveGroup(module, modules)
    if (!seenGroups.has(group.id)) seenGroups.set(group.id, group)
    fields.push({
      id,
      key,
      label: String(module.label || module.name || key),
      type: uiType,
      required: Boolean(module.required),
      readonly: Boolean(module.readonly || module.disabled),
      options: parseOptions(module),
      placeholder: String(module.placeholder || ''),
      defaultValue: String(object(module.defaultValue).value ?? module.defaultValue ?? ''),
      groupId: group.id,
      groupLabel: group.label,
    })
  }

  const groups = [...seenGroups.values()]
  // 过滤掉"字段全落在默认组以外"的噪声：默认组 id='' 恒保留（承载无容器字段）
  const defaultHasField = fields.some((f) => f.groupId === '')
  const finalGroups: MobileFormGroup[] = []
  for (const g of groups) {
    if (g.id === '' && !defaultHasField) continue
    finalGroups.push(g)
  }
  return { fields, groups: finalGroups }
}

/** 向后兼容：返回扁平字段（含 group 信息）。 */
export function parseSimpleForm(raw: unknown): MobileFormField[] {
  return parseMobileForm(raw).fields
}

/**
 * 计算分组渲染顺序：布局容器组按 groups 声明顺序排列，无容器的默认组（groupId=''）恒在最前。
 * 扁平表单（所有字段 groupId=''）时 groups 已含 ''，需去重，避免默认组重复导致字段渲染两次。
 */
export function computeGroupOrder(fields: MobileFormField[], groups: MobileFormGroup[]): string[] {
  const order: string[] = []
  for (const g of groups) order.push(g.label || g.id)
  if (fields.some((f) => f.groupId === '') && !order.includes('')) order.unshift('')
  return order
}

export function initialValues(fields: MobileFormField[]): Record<string, string> {
  return Object.fromEntries(
    fields.map((f) => [f.key, f.defaultValue || (f.type === 'checkbox' || f.type === 'attachment' ? '' : '')]),
  )
}

export function validateSimpleForm(fields: MobileFormField[], values: Record<string, string>): Record<string, string> {
  const errors: Record<string, string> = {}
  for (const field of fields) {
    if (!field.required) continue
    const v = values[field.key]
    const empty = v === undefined || v === '' || (Array.isArray(v) && v.length === 0)
    if (empty) errors[field.key] = `${field.label || field.key} 为必填项`
  }
  return errors
}
