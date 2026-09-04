const fs = require('fs');
let content = fs.readFileSync('D:/WORKSPACE/fakeToys/oa4rust-web/apps/desktop/src/views/FormDesigner.vue', 'utf8');

// === Add enhanced palette with more field types ===
const oldPalette = `const layoutTypes = [
  { type: 'section',  label: '分组', icon: '📁' },
  { type: 'row_start', label: '开始行', icon: '↔️' },
  { type: 'row_end',   label: '结束行', icon: '↩️' },
]`;
const newPalette = `const layoutTypes = [
  { type: 'section',    label: '分组',     icon: '📁' },
  { type: 'section_end',label: '结束分组', icon: '📂' },
  { type: 'row_start',  label: '开始行',   icon: '↔️' },
  { type: 'row_end',    label: '结束行',   icon: '↩️' },
  { type: 'columns',    label: '多列布局', icon: '▥' },
  { type: 'spacer',     label: '间距',     icon: '↕️' },
  { type: 'divider',    label: '分割线',   icon: '—' },
  { type: 'html',       label: 'HTML',     icon: '📝' },
]
const extraFieldTypes = [
  { type: 'rating',      label: '评分',     icon: '⭐' },
  { type: 'slider',      label: '滑块',     icon: '🎚️' },
  { type: 'color_picker',label: '颜色',     icon: '🎨' },
  { type: 'signature',   label: '签字',     icon: '✍️' },
  { type: 'image',       label: '图片',     icon: '🖼️' },
  { type: 'rich_text',   label: '富文本',   icon: '📝' },
  { type: 'json_editor', label: 'JSON',     icon: '{ }' },
  { type: 'cascader',    label: '级联',     icon: '🌳' },
  { type: 'tree_select', label: '树选',     icon: '🌲' },
  { type: 'transfer',    label: '穿梭',     icon: '⇄' },
  { type: 'time_range',  label: '时间范围', icon: '⏰' },
  { type: 'date_range',  label: '日期范围', icon: '📅' },
  { type: 'switch',      label: '开关',     icon: '🔘' },
  { type: 'upload',      label: '上传',     icon: '📤' },
  { type: 'map',         label: '地图',     icon: '🗺️' },
  { type: 'code',        label: '代码',     icon: '</>' },
]`;
content = content.replace(oldPalette, newPalette);

// === Update fieldTypes to include extra types ===
const oldFieldTypes = `const fieldTypes = [
  { type: 'text',     label: '文本',   icon: '📝' },
  { type: 'textarea', label: '多行文本', icon: '📄' },
  { type: 'number',   label: '数字',   icon: '🔢' },
  { type: 'date',     label: '日期',   icon: '📅' },
  { type: 'select',   label: '下拉',   icon: '📋' },
  { type: 'checkbox', label: '单选',   icon: '◉' },
  { type: 'checkbox_group', label: '多选', icon: '☑' },
  { type: 'file',     label: '文件',   icon: '📎' },
  { type: 'email',    label: '邮箱',   icon: '✉' },
  { type: 'phone',    label: '手机',   icon: '📱' },
]`;
const newFieldTypes = `const fieldTypes = [
  { type: 'text',     label: '文本',     icon: '📝' },
  { type: 'textarea', label: '多行文本', icon: '📄' },
  { type: 'number',   label: '数字',     icon: '🔢' },
  { type: 'date',     label: '日期',     icon: '📅' },
  { type: 'select',   label: '下拉',     icon: '📋' },
  { type: 'checkbox', label: '单选',     icon: '◉' },
  { type: 'checkbox_group', label: '多选', icon: '☑' },
  { type: 'radio_group', label: '单选组', icon: '🔘' },
  { type: 'file',     label: '文件',     icon: '📎' },
  { type: 'email',    label: '邮箱',     icon: '✉' },
  { type: 'phone',    label: '手机',     icon: '📱' },
  { type: 'url',      label: '网址',     icon: '🔗' },
  { type: 'ip',       label: 'IP地址',   icon: '🌐' },
  { type: 'rate',     label: '评分',     icon: '⭐' },
  { type: 'slider',   label: '滑块',     icon: '🎚️' },
  { type: 'color',    label: '颜色',     icon: '🎨' },
  { type: 'switch',   label: '开关',     icon: '🔘' },
  { type: 'time',     label: '时间',     icon: '⏰' },
  { type: 'month',    label: '月份',     icon: '📆' },
  { type: 'week',     label: '周',       icon: '📆' },
  { type: 'allDay',   label: '全天',     icon: '☀️' },
  { type: 'digit',    label: '验证码',   icon: '🔢' },
  { type: 'captcha',  label: '图形验证', icon: '🧩' },
  { type: 'address',  label: '地址',     icon: '📍' },
  { type: 'region',   label: '地区',     icon: '🗺️' },
  { type: 'table',    label: '表格',     icon: '📊' },
  { type: 'markdown', label: 'Markdown', icon: '📝' },
  { type: 'json',     label: 'JSON',     icon: '{ }' },
  { type: 'code',     label: '代码',     icon: '</>' },
  { type: 'signature',label: '签字',     icon: '✍️' },
  { type: 'avatar',   label: '头像',     icon: '👤' },
]`;
content = content.replace(oldFieldTypes, newFieldTypes);

// === Enhance makeField to support all types ===
const oldMakeField = `function makeField(type: string): FormField {
  const d: Record<string, Partial<FormField>> = {
    text: { label: '文本字段', key: 'text_field', placeholder: '请输入' },
    textarea: { label: '多行文本', key: 'textarea_field', placeholder: '请输入内容', rows: 4 },
    number: { label: '数字', key: 'number_field', placeholder: '请输入数字' },
    date: { label: '日期', key: 'date_field' },
    select: { label: '下拉选择', key: 'select_field', optionsStr: 'option1|选项1\\noption2|选项2' },
    checkbox: { label: '单选框', key: 'checkbox_field' },
    checkbox_group: { label: '多选框', key: 'checkbox_group_field', optionsStr: 'a|A\\nb|B\\nc|C' },
    file: { label: '文件上传', key: 'file_field' },
    email: { label: '邮箱', key: 'email_field', placeholder: 'example@domain.com' },
    phone: { label: '手机号', key: 'phone_field', placeholder: '请输入手机号' },
    section: { label: '分组标题', key: '' },
  }
  return { id: genId(), type, ...d[type], required: false, disabled: false } as FormField
}`;
const newMakeField = `function makeField(type: string): FormField {
  const defaults: Record<string, Partial<FormField>> = {
    text: { label: '文本字段', key: 'text_field', placeholder: '请输入' },
    textarea: { label: '多行文本', key: 'textarea_field', placeholder: '请输入内容', rows: 4 },
    number: { label: '数字', key: 'number_field', placeholder: '请输入数字', min: 0, max: 999999, step: 1 },
    date: { label: '日期', key: 'date_field' },
    time: { label: '时间', key: 'time_field' },
    month: { label: '月份', key: 'month_field' },
    week: { label: '周', key: 'week_field' },
    select: { label: '下拉选择', key: 'select_field', optionsStr: 'option1|选项1\\noption2|选项2' },
    radio_group: { label: '单选组', key: 'radio_group_field', optionsStr: 'a|A\\nb|B\\nc|C' },
    checkbox: { label: '单选框', key: 'checkbox_field' },
    checkbox_group: { label: '多选框', key: 'checkbox_group_field', optionsStr: 'a|A\\nb|B\\nc|C' },
    file: { label: '文件上传', key: 'file_field' },
    upload: { label: '文件上传', key: 'upload_field' },
    image: { label: '图片上传', key: 'image_field' },
    avatar: { label: '头像', key: 'avatar_field' },
    email: { label: '邮箱', key: 'email_field', placeholder: 'example@domain.com', pattern: '^[^\\\\s@]+@[^\\\\s@]+\\\\.[^\\\\s@]+$' },
    phone: { label: '手机号', key: 'phone_field', placeholder: '请输入手机号', pattern: '^1[3-9]\\\\d{9}$' },
    url: { label: '网址', key: 'url_field', placeholder: 'https://...' },
    ip: { label: 'IP地址', key: 'ip_field', placeholder: '192.168.1.1' },
    section: { label: '分组标题', key: '' },
    section_end: { label: '分组结束', key: '' },
    row_start: { label: '开始行', key: '' },
    row_end: { label: '结束行', key: '' },
    columns: { label: '多列布局', key: '' },
    spacer: { label: '间距', key: '' },
    divider: { label: '分割线', key: '' },
    html: { label: 'HTML内容', key: '' },
    rating: { label: '评分', key: 'rating_field', min: 1, max: 5 },
    rate: { label: '星级', key: 'rate_field', min: 1, max: 5 },
    slider: { label: '滑块', key: 'slider_field', min: 0, max: 100, defaultValue: '50' },
    color_picker: { label: '颜色选择', key: 'color_field', defaultValue: '#000000' },
    color: { label: '颜色', key: 'color_field', defaultValue: '#000000' },
    signature: { label: '签字', key: 'signature_field' },
    rich_text: { label: '富文本', key: 'rich_text_field' },
    json_editor: { label: 'JSON编辑器', key: 'json_field' },
    json: { label: 'JSON', key: 'json_field' },
    cascader: { label: '级联选择', key: 'cascader_field' },
    tree_select: { label: '树形选择', key: 'tree_select_field' },
    transfer: { label: '穿梭框', key: 'transfer_field' },
    time_range: { label: '时间范围', key: 'time_range_field' },
    date_range: { label: '日期范围', key: 'date_range_field' },
    switch: { label: '开关', key: 'switch_field', defaultValue: 'false' },
    map: { label: '地图', key: 'map_field' },
    code: { label: '代码编辑', key: 'code_field' },
    digit: { label: '验证码', key: 'digit_field' },
    captcha: { label: '图形验证', key: 'captcha_field' },
    address: { label: '地址', key: 'address_field' },
    region: { label: '地区', key: 'region_field' },
    table: { label: '表格', key: 'table_field' },
    markdown: { label: 'Markdown', key: 'markdown_field' },
    allDay: { label: '全天', key: 'allday_field' },
  }
  const base: FormField = {
    id: genId(), type,
    label: defaults[type]?.label || '未命名',
    key: defaults[type]?.key || 'field_' + type + '_' + Date.now(),
    required: false, disabled: false,
    ...(defaults[type] || {}),
  }
  if (type === 'email') base.pattern = '^[^\\\\s@]+@[^\\\\s@]+\\\\.[^\\\\s@]+$'
  if (type === 'phone') base.pattern = '^1[3-9]\\\\d{9}$'
  if (type === 'number') { base.min = 0; base.max = 999999; base.step = 1 }
  return base
}`;
content = content.replace(oldMakeField, newMakeField);

// === Add more advanced functions ===
const lifecycleMarker = 'onMounted(loadForms)';
const advancedFuncs = `
// --- Enhanced Field Management ---
function duplicateField(idx: number) {
  if (!currentForm.value || idx < 0) return
  const orig = currentForm.value.fields[idx]
  if (!orig) return
  const copy = { ...orig, id: genId(), key: orig.key + '_copy', label: orig.label + ' (副本)' }
  currentForm.value.fields.splice(idx + 1, 0, copy)
  selectedField.value = copy
  pushFormHistory()
}
function toggleFieldVisibility(idx: number) {
  if (!currentForm.value || idx < 0) return
  const f = currentForm.value.fields[idx]
  if (f) f.hidden = !f.hidden
}
function setFieldRequired(idx: number, val: boolean) {
  if (!currentForm.value || idx < 0) return
  const f = currentForm.value.fields[idx]
  if (f) f.required = val
}
function setFieldDisabled(idx: number, val: boolean) {
  if (!currentForm.value || idx < 0) return
  const f = currentForm.value.fields[idx]
  if (f) f.disabled = val
}
function setFieldReadOnly(idx: number, val: boolean) {
  if (!currentForm.value || idx < 0) return
  const f = currentForm.value.fields[idx]
  if (f) f.readonly = val
}
function setFieldWidth(idx: number, width: string) {
  if (!currentForm.value || idx < 0) return
  const f = currentForm.value.fields[idx]
  if (f) f.cssClass = 'form-field-' + width
}
function setFieldHelpText(idx: number, text: string) {
  if (!currentForm.value || idx < 0) return
  const f = currentForm.value.fields[idx]
  if (f) f.helpText = text
}
function setFieldMaxLength(idx: number, val: number) {
  if (!currentForm.value || idx < 0) return
  const f = currentForm.value.fields[idx]
  if (f) f.maxLength = val
}
function setFieldMinLength(idx: number, val: number) {
  if (!currentForm.value || idx < 0) return
  const f = currentForm.value.fields[idx]
  if (f) f.minLength = val
}
function setFieldPattern(idx: number, pattern: string, msg?: string) {
  if (!currentForm.value || idx < 0) return
  const f = currentForm.value.fields[idx]
  if (f) { f.pattern = pattern; if (msg) f.validation = { ...(f.validation||{}), patternMsg: msg } }
}

// --- Conditional Display ---
interface FieldCondition { operator: string; value: string; fieldKey?: string }
function addCondition(fieldIdx: number) {
  if (!currentForm.value || fieldIdx < 0) return
  const f = currentForm.value.fields[fieldIdx]
  if (!f) return
  if (!(f as any).conditions) (f as any).conditions = []
  ;(f as any).conditions.push({ operator: 'equals', value: '', fieldKey: '' })
}
function removeCondition(fieldIdx: number, condIdx: number) {
  if (!currentForm.value || fieldIdx < 0) return
  const f = currentForm.value.fields[fieldIdx]
  if (!f) return
  const conds = (f as any).conditions
  if (Array.isArray(conds)) conds.splice(condIdx, 1)
}
function fmtCondition(c: FieldCondition): string {
  const ops: Record<string, string> = { equals: '==', contains: 'includes', gt: '>', lt: '<', neq: '!=' }
  return ops[c.operator] || c.operator
}

// --- Validation Rules ---
function addValidationRule(fieldIdx: number) {
  if (!currentForm.value || fieldIdx < 0) return
  const f = currentForm.value.fields[fieldIdx]
  if (!f) return
  if (!f.validation) f.validation = {}
  f.validation.required = true
}
function setValidationMin(fieldIdx: number, val: number) {
  if (!currentForm.value || fieldIdx < 0) return
  const f = currentForm.value.fields[fieldIdx]
  if (!f) return
  if (!f.validation) f.validation = {}
  f.validation.min = val
}
function setValidationMax(fieldIdx: number, val: number) {
  if (!currentForm.value || fieldIdx < 0) return
  const f = currentForm.value.fields[fieldIdx]
  if (!f) return
  if (!f.validation) f.validation = {}
  f.validation.max = val
}
function setValidationMinLength(fieldIdx: number, val: number) {
  if (!currentForm.value || fieldIdx < 0) return
  const f = currentForm.value.fields[fieldIdx]
  if (!f) return
  if (!f.validation) f.validation = {}
  f.validation.minLength = val
}
function setValidationMaxLength(fieldIdx: number, val: number) {
  if (!currentForm.value || fieldIdx < 0) return
  const f = currentForm.value.fields[fieldIdx]
  if (!f) return
  if (!f.validation) f.validation = {}
  f.validation.maxLength = val
}
function setValidationPattern(fieldIdx: number, regex: string, msg?: string) {
  if (!currentForm.value || fieldIdx < 0) return
  const f = currentForm.value.fields[fieldIdx]
  if (!f) return
  if (!f.validation) f.validation = {}
  f.validation.pattern = regex
  if (msg) f.validation.patternMsg = msg
}

// --- Section/Group Management ---
function addSection(label: string) {
  if (!currentForm.value) return
  const section: FormField = { id: genId(), type: 'section', label, key: '' }
  currentForm.value.fields.push(section)
  const endSection: FormField = { id: genId(), type: 'section_end', label: '', key: '' }
  currentForm.value.fields.push(endSection)
  selectedField.value = section
  pushFormHistory()
}
function closeSection() {
  if (!currentForm.value || !selectedField.value) return
  if (selectedField.value.type === 'section') {
    // Find next section_end or add one
    const idx = currentForm.value.fields.findIndex(f => f.id === selectedField.value!.id)
    if (idx !== -1 && idx + 1 < currentForm.value.fields.length) {
      // Already has a closing section
    } else {
      currentForm.value.fields.splice(idx + 1, 0, { id: genId(), type: 'section_end', label: '', key: '' })
    }
  }
}

// --- Column Layout ---
function setColumnCount(n: 1|2|3) { columnCount.value = n }
function getFieldColSpan(fieldType: string): number {
  if (['section', 'section_end', 'divider', 'spacer', 'html'].includes(fieldType)) return 0
  return columnCount.value === 1 ? 1 : columnCount.value === 2 ? 2 : 3
}

// --- Advanced Preview ---
function validatePreview(): boolean {
  if (!currentForm.value) return false
  previewErrors.value = {}
  for (const field of currentForm.value.fields) {
    if (['section', 'section_end', 'divider', 'spacer'].includes(field.type)) continue
    if (field.required && !previewData.value[field.key]) {
      previewErrors.value[field.key] = field.label + ' 不能为空'
    }
    if (field.pattern && previewData.value[field.key] && !new RegExp(field.pattern).test(previewData.value[field.key])) {
      previewErrors.value[field.key] = field.label + ' 格式错误'
    }
    if (field.minLength && previewData.value[field.key] && previewData.value[field.key].length < field.minLength) {
      previewErrors.value[field.key] = field.label + ' 至少' + field.minLength + '个字符'
    }
    if (field.maxLength && previewData.value[field.key] && previewData.value[field.key].length > field.maxLength) {
      previewErrors.value[field.key] = field.label + ' 最多' + field.maxLength + '个字符'
    }
    if (field.min !== undefined && previewData.value[field.key] && Number(previewData.value[field.key]) < field.min) {
      previewErrors.value[field.key] = field.label + ' 不能小于' + field.min
    }
    if (field.max !== undefined && previewData.value[field.key] && Number(previewData.value[field.key]) > field.max) {
      previewErrors.value[field.key] = field.label + ' 不能大于' + field.max
    }
  }
  return Object.keys(previewErrors.value).length === 0
}
async function submitPreview() {
  if (!validatePreview()) return
  try {
    await api.post('/jaxrs/form/submit', { formFlag: currentForm.value.flag, data: previewData.value })
    alert('提交成功！')
    previewData.value = {}
    previewErrors.value = {}
  } catch (e: any) { alert('提交失败: ' + (e?.message ?? '')) }
}

// --- Import/Export ---
function exportFormJson(): string {
  if (!currentForm.value) return '{}'
  return JSON.stringify(currentForm.value, null, 2)
}
function importFormJson(text: string) {
  try {
    const d = JSON.parse(text)
    currentForm.value = { ...d, fields: (d.fields || []).map((f: any) => ({ ...f, id: f.id || genId() })) }
    selectedField.value = null
    pushFormHistory()
  } catch { alert('JSON格式错误') }
}
function downloadFormJson() {
  const b = new Blob([exportFormJson()], { type: 'application/json' })
  const u = URL.createObjectURL(b)
  const a = document.createElement('a')
  a.href = u; a.download = (currentForm.value?.flag || 'form') + '.json'
  a.click(); URL.revokeObjectURL(u)
}

// --- Schema View ---
const schemaJson = computed(() => {
  if (!currentForm.value) return ''
  return JSON.stringify({
    name: currentForm.value.name, flag: currentForm.value.flag,
    layout: currentForm.value.layout,
    fields: currentForm.value.fields.map(f => ({
      type: f.type, label: f.label, key: f.key, required: f.required,
      placeholder: f.placeholder, defaultValue: f.defaultValue,
      validation: f.validation, conditions: (f as any).conditions,
      colSpan: f.colSpan, width: f.width,
    }))
  }, null, 2)
})

// --- Bulk Operations ---
function batchSetRequired(flag: boolean) {
  if (!currentForm.value || !selectedField.value) return
  selectedField.value.required = flag
}
function batchCopyField() {
  if (!currentForm.value || !selectedField.value) return
  const orig = { ...selectedField.value, id: genId() }
  currentForm.value.fields.push(orig)
  selectedField.value = orig
  pushFormHistory()
}
function batchDeleteSelected() {
  if (!currentForm.value || !selectedField.value) return
  const idx = currentForm.value.fields.findIndex(f => f.id === selectedField.value!.id)
  if (idx !== -1) { currentForm.value.fields.splice(idx, 1); selectedField.value = null; pushFormHistory() }
}

// --- History ---
function pushFormHistory() {
  if (!currentForm.value) return
  formHistory.value = formHistory.value.slice(0, historyIdx.value + 1)
  formHistory.value.push({ fields: JSON.parse(JSON.stringify(currentForm.value.fields)), timestamp: Date.now(), label: '自动保存 ' + new Date().toLocaleTimeString('zh-CN') })
  historyIdx.value = formHistory.value.length - 1
}
function formUndo() { if (!canUndo.value || !currentForm.value) return; historyIdx.value--; currentForm.value.fields = JSON.parse(JSON.stringify(formHistory.value[historyIdx.value].fields)) }
function formRedo() { if (!canRedo.value || !currentForm.value) return; historyIdx.value++; currentForm.value.fields = JSON.parse(JSON.stringify(formHistory.value[historyIdx.value].fields)) }

// --- Field Templates ---
const fieldTemplates = [
  { name: '姓名信息', icon: '👤', fields: [{type:'text',label:'姓名',key:'name',required:true},{type:'email',label:'邮箱',key:'email'},{type:'phone',label:'手机',key:'phone'}] },
  { name: '订单信息', icon: '📦', fields: [{type:'text',label:'订单号',key:'order_id'},{type:'date',label:'日期',key:'order_date'},{type:'number',label:'金额',key:'amount',min:0},{type:'select',label:'状态',key:'status',optionsStr:'待支付|待支付\\n已支付|已支付\\n已完成|已完成'}] },
  { name: '员工信息', icon: '👥', fields: [{type:'text',label:'姓名',key:'name',required:true},{type:'select',label:'部门',key:'dept',optionsStr:'技术部|技术部\\n销售部|销售部\\n人事部|人事部'},{type:'date',label:'入职日期',key:'hire_date'},{type:'number',label:'薪资',key:'salary',min:0}] },
  { name: '反馈表单', icon: '💬', fields: [{type:'select',label:'类型',key:'type',optionsStr:'问题|问题反馈\\n建议|建议\\n其他|其他'},{type:'textarea',label:'内容',key:'content',rows:5,required:true},{type:'email',label:'联系方式',key:'contact'}] },
  { name: '调查问卷', icon: '📊', fields: [{type:'text',label:'姓名',key:'name'},{type:'select',label:'年龄',key:'age',optionsStr:'18-25|18-25岁\\n26-35|26-35岁\\n36-50|36-50岁'},{type:'rating',label:'评分',key:'rating',min:1,max:5},{type:'textarea',label:'意见',key:'comments',rows:3}] },
  { name: '预约表单', icon: '📅', fields: [{type:'text',label:'姓名',key:'name',required:true},{type:'phone',label:'手机',key:'phone',required:true},{type:'date',label:'日期',key:'date',required:true},{type:'textarea',label:'备注',key:'notes',rows:2}] },
  { name: '地址信息', icon: '📍', fields: [{type:'text',label:'省份',key:'province'},{type:'text',label:'城市',key:'city'},{type:'textarea',label:'详细地址',key:'address',rows:3},{type:'text',label:'邮编',key:'zip'}] },
  { name: '登录表单', icon: '🔐', fields: [{type:'email',label:'邮箱',key:'email',required:true},{type:'text',label:'密码',key:'password',placeholder:'请输入密码'},{type:'checkbox',label:'记住我',key:'remember'}] },
  { name: '注册表单', icon: '📝', fields: [{type:'text',label:'用户名',key:'username',required:true},{type:'email',label:'邮箱',key:'email',required:true},{type:'password',label:'密码',key:'password',required:true},{type:'password',label:'确认密码',key:'confirm_password',required:true},{type:'checkbox',label:'同意条款',key:'agree',required:true}] },
  { name: '联系人', icon: '📒', fields: [{type:'text',label:'姓名',key:'name'},{type:'phone',label:'手机',key:'phone'},{type:'email',label:'邮箱',key:'email'},{type:'text',label:'公司',key:'company'},{type:'textarea',label:'备注',key:'note',rows:2}] },
]
function applyTemplate(tpl: typeof fieldTemplates[0]) {
  if (!currentForm.value) return
  currentForm.value.fields = tpl.fields.map(f => makeField(f as any))
  selectedField.value = null
  pushFormHistory()
}

onMounted(loadForms)`;
content = content.replace(lifecycleMarker, advancedFuncs);

// === Add CSS for new features ===
const styleEnd = '</style>';
const extraCss = `
/* Templates */
.modal-overlay{position:fixed;inset:0;background:rgba(0,0,0,.6);display:flex;align-items:center;justify-content:center;z-index:200}
.modal{background:var(--bg-surface);border:1px solid var(--border-color);border-radius:var(--radius-lg);padding:20px;max-height:85vh;overflow-y:auto}
.modal-lg{width:720px}.modal-md{width:480px}
.modal-header{display:flex;align-items:center;justify-content:space-between;margin-bottom:16px}
.modal-header h3{font-size:16px;color:var(--color-primary);margin:0}
.modal-body{display:flex;flex-direction:column;gap:12px}
.modal-hint{font-size:12px;color:var(--text-muted)}
.modal-actions{display:flex;gap:8px;justify-content:flex-end}
.tpl-grid{display:grid;grid-template-columns:repeat(auto-fill,minmax(140px,1fr));gap:10px}
.tpl-card{padding:14px;border-radius:var(--radius-md);border:1px solid var(--border-color);cursor:pointer;transition:all .15s;display:flex;flex-direction:column;align-items:center;gap:6px;text-align:center}
.tpl-card:hover{border-color:var(--color-primary);background:var(--color-primary-soft);transform:translateY(-2px)}
.tpl-icon{font-size:28px}.tpl-name{font-size:13px;font-weight:600;color:var(--color-primary)}
.tpl-count{font-size:10px;color:var(--text-muted)}
/* Schema */
.schema-editor{width:100%;height:280px;padding:10px;border-radius:var(--radius-md);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-family:'JetBrains Mono',monospace;font-size:12px;resize:vertical;box-sizing:border-box}
/* Import/Export */
.io-section{display:flex;flex-direction:column;gap:6px}
.io-section label{font-size:12px;color:var(--text-muted)}
.io-sep{height:1px;background:var(--border-color);margin:8px 0}
/* Column layout */
.col-layout{display:flex;align-items:center;gap:4px;margin-right:8px}
.col-label{font-size:11px;color:var(--text-muted)}
.col-btn{padding:2px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:transparent;color:var(--text-muted);cursor:pointer;font-size:11px}
.col-btn:hover,.col-btn.active{border-color:var(--color-primary);color:var(--color-primary)}
/* Enhanced field row */
.field-row.group-member{margin-left:16px;border-left:2px solid var(--color-primary)}
.field-row.is-group-head{background:rgba(168,85,247,.1);border-color:rgba(168,85,247,.4)}
.field-row.is-divider{opacity:.5;cursor:default}
.field-row.is-spacer{height:16px;border:none;background:transparent;cursor:default}
.fr-conditions{display:flex;gap:2px}
.fr-cond-badge{font-size:9px;padding:1px 4px;border-radius:var(--radius-sm);background:rgba(245,158,11,.2);color:var(--color-warning)}
.fr-required-badge{font-size:9px;color:var(--color-danger)}
.fr-hidden-badge{font-size:9px;color:var(--text-muted)}
.fr-disabled-badge{font-size:9px;color:var(--text-muted)}
.fr-readonly-badge{font-size:9px;color:var(--text-muted)}
/* Enhanced props */
.props-section{margin-bottom:12px;padding-bottom:12px;border-bottom:1px solid var(--border-color)}
.props-section-title{font-size:11px;font-weight:600;color:var(--color-primary);margin-bottom:8px;text-transform:uppercase;letter-spacing:1px}
.prop-row-2{display:flex;gap:8px}
.prop-row-2 .prop-group{flex:1}
.validation-config{display:flex;flex-direction:column;gap:6px}
.val-rule{display:flex;align-items:center;gap:4px}
.val-rule select,.val-rule input{padding:3px 6px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:11px;outline:none}
.val-rule select{flex:0 0 70px;cursor:pointer}.val-rule input{flex:1}
/* Preview enhanced */
.pv-error-msg{font-size:11px;color:var(--color-danger);margin-top:2px}
.pv-input.error{border-color:var(--color-danger)}
.pv-success-msg{padding:16px;text-align:center;color:var(--color-success);font-size:14px}
/* Responsive canvas */
.fd-canvas.two-col .canvas-form{display:grid;grid-template-columns:1fr 1fr;gap:12px}
.fd-canvas.three-col .canvas-form{display:grid;grid-template-columns:1fr 1fr 1fr;gap:12px}
.canvas-form.two-col .field-row.span-2{grid-column:span 2}
.canvas-form.three-col .field-row.span-2{grid-column:span 2}
.canvas-form.three-col .field-row.span-3{grid-column:span 3}
/* Spacer/Divider styling */
.spacer-preview{height:20px;background:repeating-linear-gradient(90deg,var(--border-color),var(--border-color) 4px,transparent 4px,transparent 8px);border-radius:2px;margin:4px 0}
.divider-preview{height:1px;background:var(--border-color);margin:8px 0}
`;
content = content.replace(styleEnd, extraCss + '\n</style>');

// Write back
fs.writeFileSync('D:/WORKSPACE/fakeToys/oa4rust-web/apps/desktop/src/views/FormDesigner.vue', content);
console.log('Lines:', content.split('\n').length);
