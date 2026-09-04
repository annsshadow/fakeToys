const fs = require('fs');
let content = fs.readFileSync('D:/WORKSPACE/fakeToys/oa4rust-web/apps/desktop/src/views/FormDesigner.vue', 'utf8');

// === 1. Enhance FormField interface ===
const oldInterface = `interface FormField {
  id: string; type: string; label: string; key: string
  placeholder?: string; defaultValue?: string; required?: boolean; disabled?: boolean
  rows?: number; min?: number; max?: number; optionsStr?: string
}`;
const newInterface = `interface FormField {
  id: string; type: string; label: string; key: string
  placeholder?: string; defaultValue?: string; required?: boolean; disabled?: boolean
  rows?: number; min?: number; max?: number; optionsStr?: string
  description?: string; cssClass?: string; helpText?: string
  maxLength?: number; minLength?: number; pattern?: string; step?: number
  readonly?: boolean; hidden?: boolean; readonlyExpr?: string
  visibleIf?: string; visibleIfField?: string; visibleIfOp?: string; visibleIfValue?: string
  defaultValueFn?: string; transformFn?: string
  // Grouping
  parentId?: string; children?: FormField[]
  // Validation
  validation?: FieldValidation
  // Styling
  width?: string; colSpan?: number; rowSpan?: number
  // Custom
  customAttrs?: Record<string,string>
}
interface FieldValidation {
  required?: boolean; min?: number; max?: number; minLength?: number; maxLength?: number
  pattern?: string; patternMsg?: string; customMsg?: string
  asyncValidator?: string; validateFn?: string
}
interface FormDef {
  id?: string; name: string; flag: string; desc?: string
  layout?: 'single'|'two_col'|'three_col'
  submitAction?: string; submitUrl?: string
  fields: FormField[]; updatedAt?: string; version?: string
  settings?: { showReset?: boolean; showSubmit?: boolean; layoutClass?: string }
}`;
content = content.replace(oldInterface, newInterface);

// === 2. Enhance layoutTypes and add more field types ===
const oldLayoutTypes = `const layoutTypes = [
  { type: 'section',  label: '分组', icon: '📁' },
  { type: 'row_start', label: '开始行', icon: '↔️' },
  { type: 'row_end',   label: '结束行', icon: '↩️' },
]`;
const newLayoutTypes = `const layoutTypes = [
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
  { type: 'rate',        label: '星级',     icon: '🌟' },
  { type: 'switch',      label: '开关',     icon: '🔘' },
  { type: 'rate',        label: '评分',     icon: '🌟' },
  { type: 'upload',      label: '上传',     icon: '📤' },
  { type: 'map',         label: '地图',     icon: '🗺️' },
  { type: 'code',        label: '代码',     icon: '</>' },
]
const fieldTypesExpanded = [...fieldTypes, ...extraFieldTypes];`;
content = content.replace(oldLayoutTypes, newLayoutTypes);

// === 3. Add more state variables ===
const oldState = "const mode = ref<'edit'|'preview'>('edit')";
const newState = `const mode = ref<'edit'|'preview'|'schema'>('edit')
const selectedField = ref<FormField|null>(null)
const draggedType = ref<string|null>(null)
const dragFieldIdx = ref<number|null>(null)
const dragOverIdx = ref<number|null>(null)
const showFieldTemplates = ref(false)
const showValidationPanel = ref(false)
const showAdvancedProps = ref(false)
const formSettings = ref({ showReset: true, showSubmit: true, layoutClass: '' })
const previewData = ref<Record<string,any>>({})
const previewErrors = ref<Record<string,string>>({})
const showSchema = ref(false)
const formHistory = ref<{fields: FormField[]; timestamp: number; label: string}[]>([])
const historyIdx = ref(-1)
const canUndo = computed(() => historyIdx.value > 0)
const canRedo = computed(() => historyIdx.value < formHistory.value.length - 1)
const columnCount = ref<1|2|3>(1)
const dragDropTarget = ref<number|null>(null)`;
// Remove duplicate selectedField declaration
content = content.replace("const selectedField = ref<FormField|null>(null)\nconst draggedType = ref<string|null>(null)", "");
content = content.replace(oldState, newState);

// === 4. Enhance makeField to support new fields ===
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
    text:      { label: '文本字段', key: 'text_field', placeholder: '请输入' },
    textarea:  { label: '多行文本', key: 'textarea_field', placeholder: '请输入内容', rows: 4 },
    number:    { label: '数字', key: 'number_field', placeholder: '请输入数字' },
    date:      { label: '日期', key: 'date_field' },
    select:    { label: '下拉选择', key: 'select_field', optionsStr: 'option1|选项1\\noption2|选项2' },
    checkbox:  { label: '单选框', key: 'checkbox_field' },
    checkbox_group: { label: '多选框', key: 'checkbox_group_field', optionsStr: 'a|A\\nb|B\\nc|C' },
    file:      { label: '文件上传', key: 'file_field' },
    email:     { label: '邮箱', key: 'email_field', placeholder: 'example@domain.com' },
    phone:     { label: '手机号', key: 'phone_field', placeholder: '请输入手机号' },
    section:   { label: '分组标题', key: '' },
    section_end: { label: '分组结束', key: '' },
    row_start: { label: '开始行', key: '' },
    row_end:   { label: '结束行', key: '' },
    columns:   { label: '多列布局', key: '' },
    spacer:    { label: '间距', key: '' },
    divider:   { label: '分割线', key: '' },
    html:      { label: 'HTML内容', key: '' },
    rating:    { label: '评分', key: 'rating_field', min: 1, max: 5 },
    slider:    { label: '滑块', key: 'slider_field', min: 0, max: 100, defaultValue: '50' },
    color_picker: { label: '颜色选择', key: 'color_field' },
    signature: { label: '签字', key: 'signature_field' },
    image:     { label: '图片上传', key: 'image_field' },
    rich_text: { label: '富文本', key: 'rich_text_field' },
    json_editor: { label: 'JSON编辑器', key: 'json_field' },
    cascader:  { label: '级联选择', key: 'cascader_field' },
    tree_select: { label: '树形选择', key: 'tree_select_field' },
    transfer:  { label: '穿梭框', key: 'transfer_field' },
    time_range:{ label: '时间范围', key: 'time_range_field' },
    date_range:{ label: '日期范围', key: 'date_range_field' },
    rate:      { label: '星级', key: 'rate_field', min: 1, max: 5 },
    switch:    { label: '开关', key: 'switch_field' },
    upload:    { label: '文件上传', key: 'upload_field' },
    map:       { label: '地图', key: 'map_field' },
    code:      { label: '代码编辑', key: 'code_field' },
  }
  const base: FormField = {
    id: genId(), type,
    label: defaults[type]?.label || '未命名',
    key: defaults[type]?.key || 'field_' + type + '_' + Date.now(),
    required: false, disabled: false,
    ...(defaults[type] || {}),
  }
  // Add default validation for certain types
  if (type === 'email') base.pattern = '^[^\\s@]+@[^\\s@]+\\.[^\\s@]+$'
  if (type === 'phone') base.pattern = '^1[3-9]\\d{9}$'
  if (type === 'number') { base.min = 0; base.max = 999999; base.step = 1 }
  return base
}`;
content = content.replace(oldMakeField, newMakeField);

// === 5. Add advanced functions ===
const lifecycleMarker = 'onMounted(loadForms)';
const advancedCode = `
// ── Form History ──────────────────────────────────────────────────────
function pushFormHistory() {
  if (!currentForm.value) return
  formHistory.value = formHistory.value.slice(0, historyIdx.value + 1)
  formHistory.value.push({ fields: JSON.parse(JSON.stringify(currentForm.value.fields)), timestamp: Date.now(), label: '自动保存 ' + new Date().toLocaleTimeString('zh-CN') })
  historyIdx.value = formHistory.value.length - 1
}
function formUndo() {
  if (!canUndo.value || !currentForm.value) return
  historyIdx.value--
  currentForm.value.fields = JSON.parse(JSON.stringify(formHistory.value[historyIdx.value].fields))
}
function formRedo() {
  if (!canRedo.value || !currentForm.value) return
  historyIdx.value++
  currentForm.value.fields = JSON.parse(JSON.stringify(formHistory.value[historyIdx.value].fields))
}

// ── Conditional Logic ─────────────────────────────────────────────────
interface FieldCondition { operator: string; value: string; fieldKey?: string }
function addCondition() {
  if (!selectedField.value) return
  if (!(selectedField.value as any).conditions) (selectedField.value as any).conditions = []
  ;(selectedField.value as any).conditions.push({ operator: 'equals', value: '', fieldKey: '' })
}
function removeCondition(i: number) {
  if (!selectedField.value) return
  const conds = (selectedField.value as any).conditions
  if (Array.isArray(conds)) conds.splice(i, 1)
}

// ── Validation Rules ──────────────────────────────────────────────────
function addValidationRule() {
  if (!selectedField.value) return
  if (!selectedField.value.validation) selectedField.value.validation = {}
  selectedField.value.validation.required = true
}
function setValidationPattern(regex: string, msg?: string) {
  if (!selectedField.value) return
  if (!selectedField.value.validation) selectedField.value.validation = {}
  selectedField.value.validation.pattern = regex
  if (msg) selectedField.value.validation.patternMsg = msg
}

// ── Field Templates ───────────────────────────────────────────────────
const fieldTemplates = [
  { name: '姓名', icon: '👤', fields: [{type:'text',label:'姓名',key:'name',required:true},{type:'text',label:'邮箱',key:'email',required:true},{type:'phone',label:'手机',key:'phone'}] },
  { name: '联系方式', icon: '📞', fields: [{type:'text',label:'姓名',key:'contact_name'},{type:'phone',label:'手机',key:'phone',required:true},{type:'email',label:'邮箱',key:'email'}] },
  { name: '地址信息', icon: '📍', fields: [{type:'text',label:'省份',key:'province'},{type:'text',label:'城市',key:'city'},{type:'textarea',label:'详细地址',key:'address',rows:3},{type:'text',label:'邮编',key:'zip'}] },
  { name: '反馈表单', icon: '💬', fields: [{type:'select',label:'反馈类型',key:'type',optionsStr:'问题|问题反馈\\n建议|建议\\n其他|其他'},{type:'textarea',label:'详细内容',key:'content',rows:5,required:true},{type:'email',label:'联系方式',key:'contact'},{type:'checkbox',label:'接受回复通知',key:'notify'}] },
  { name: '订单信息', icon: '📦', fields: [{type:'text',label:'订单号',key:'order_id'},{type:'date',label:'下单日期',key:'order_date'},{type:'number',label:'金额',key:'amount',min:0},{type:'select',label:'状态',key:'status',optionsStr:'待支付|待支付\\n已支付|已支付\\n已完成|已完成\\n已取消|已取消'}] },
  { name: '员工信息', icon: '👥', fields: [{type:'text',label:'姓名',key:'employee_name',required:true},{type:'text',label:'工号',key:'employee_id'},{type:'select',label:'部门',key:'department',optionsStr:'技术部|技术部\\n销售部|销售部\\n市场部|市场部\\n人事部|人事部'},{type:'date',label:'入职日期',key:'hire_date'},{type:'number',label:'薪资',key:'salary',min:0}] },
  { name: '预约表单', icon: '📅', fields: [{type:'text',label:'姓名',key:'name',required:true},{type:'phone',label:'手机',key:'phone',required:true},{type:'date',label:'预约日期',key:'appointment_date',required:true},{type:'time',label:'预约时间',key:'appointment_time',required:true},{type:'textarea',label:'备注',key:'notes',rows:2}] },
  { name: '调查问卷', icon: '📊', fields: [{type:'text',label:'您的姓名',key:'name'},{type:'email',label:'电子邮箱',key:'email'},{type:'radio_group',label:'年龄段',key:'age_group',optionsStr:'18-25|18-25岁\\n26-35|26-35岁\\n36-50|36-50岁\\n50+|50岁以上'},{type:'checkbox_group',label:'兴趣领域',key:'interests',optionsStr:'科技|科技\\n教育|教育\\n娱乐|娱乐\\n体育|体育'},{type:'rating',label:'整体评分',key:'rating',min:1,max:5},{type:'textarea',label:'其他意见',key:'comments',rows:3}] },
]
function applyTemplate(tpl: typeof fieldTemplates[0]) {
  if (!currentForm.value) return
  currentForm.value.fields = tpl.fields.map(f => makeField(f as any))
  selectedField.value = null
  pushFormHistory()
}

// ── Column Layout ─────────────────────────────────────────────────────
function setColumnCount(n: 1|2|3) { columnCount.value = n }
function getFieldColumns(fieldType: string): number {
  if (fieldType === 'section' || fieldType === 'divider' || fieldType === 'spacer' || fieldType === 'html') return 0
  if (columnCount.value === 1) return 12
  if (columnCount.value === 2) return 6
  return 4
}

// ── Preview & Submit ──────────────────────────────────────────────────
function validatePreview(): boolean {
  if (!currentForm.value) return false
  previewErrors.value = {}
  for (const field of currentForm.value.fields) {
    if (field.type === 'section' || field.type === 'section_end' || field.type === 'divider' || field.type === 'spacer') continue
    if (field.required && !previewData.value[field.key]) {
      previewErrors.value[field.key] = field.label + ' 不能为空'
    }
    if (field.pattern && previewData.value[field.key] && !new RegExp(field.pattern).test(previewData.value[field.key])) {
      previewErrors.value[field.key] = field.label + ' 格式不正确'
    }
    if (field.minLength && previewData.value[field.key] && previewData.value[field.key].length < field.minLength) {
      previewErrors.value[field.key] = field.label + ' 至少' + field.minLength + '个字符'
    }
    if (field.maxLength && previewData.value[field.key] && previewData.value[field.key].length > field.maxLength) {
      previewErrors.value[field.key] = field.label + ' 最多' + field.maxLength + '个字符'
    }
  }
  return Object.keys(previewErrors.value).length === 0
}
async function submitPreview() {
  if (!validatePreview()) return
  try {
    const payload = { formFlag: currentForm.value.flag, data: previewData.value }
    if (currentForm.value.submitUrl) {
      await api.post(currentForm.value.submitUrl, payload)
    } else {
      await api.post('/jaxrs/form/submit', payload)
    }
    alert('提交成功！')
    previewData.value = {}
    previewErrors.value = {}
  } catch(e: any) { alert('提交失败: ' + (e?.message ?? '')) }
}

// ── Export/Import ─────────────────────────────────────────────────────
function exportFormJson(): string {
  if (!currentForm.value) return '{}'
  return JSON.stringify(currentForm.value, null, 2)
}
function importFormJson(text: string) {
  try {
    const data = JSON.parse(text)
    currentForm.value = {
      id: data.id, name: data.name, flag: data.flag, desc: data.desc,
      layout: data.layout || 'single',
      fields: (data.fields || []).map((f: any) => ({...f, id: f.id || genId()})),
      updatedAt: data.updatedAt, version: data.version, settings: data.settings,
    }
    selectedField.value = null
    pushFormHistory()
  } catch { alert('JSON格式错误') }
}
function downloadFormJson() {
  const blob = new Blob([exportFormJson()], { type: 'application/json' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url; a.download = (currentForm.value?.flag || 'form') + '.json'
  a.click(); URL.revokeObjectURL(url)
}

// ── Form Schema Editor ────────────────────────────────────────────────
const schemaJson = computed(() => {
  if (!currentForm.value) return ''
  return JSON.stringify({
    name: currentForm.value.name, flag: currentForm.value.flag,
    layout: currentForm.value.layout, fields: currentForm.value.fields.map(f => ({
      type: f.type, label: f.label, key: f.key, required: f.required,
      placeholder: f.placeholder, defaultValue: f.defaultValue,
      validation: f.validation, conditions: (f as any).conditions,
    }))
  }, null, 2)
})

// ── Bulk Operations ───────────────────────────────────────────────────
function batchSetRequired(flag: boolean) {
  if (!currentForm.value) return
  const ids = selectedField.value ? [selectedField.value.id] : Array.from(multiSelectedFields.value)
  for (const id of ids) {
    const f = currentForm.value.fields.find(f => f.id === id)
    if (f) f.required = flag
  }
}
function batchSetLabel(prefix: string) {
  if (!currentForm.value) return
  const ids = selectedField.value ? [selectedField.value.id] : Array.from(multiSelectedFields.value)
  for (const id of ids) {
    const f = currentForm.value.fields.find(f => f.id === id)
    if (f) f.label = prefix + (f.label || '')
  }
}

// ── Field Drag Reorder ────────────────────────────────────────────────
const multiSelectedFields = ref<Set<string>>(new Set())

function onFieldDragStart(e: DragEvent, idx: number) {
  dragFieldIdx.value = idx
  e.dataTransfer?.setData('text/plain', String(idx))
  if (e.dataTransfer) e.dataTransfer.effectAllowed = 'move'
}

function onFieldDragOver(e: DragEvent, idx: number) {
  e.preventDefault()
  dragOverIdx.value = idx
}

function onFieldDrop(e: DragEvent, idx: number) {
  e.preventDefault()
  if (dragFieldIdx.value === null || dragFieldIdx.value === idx || !currentForm.value) return
  const fields = currentForm.value.fields
  const from = dragFieldIdx.value
  const to = idx
  const [removed] = fields.splice(from, 1)
  fields.splice(to, 0, removed)
  dragFieldIdx.value = null
  dragOverIdx.value = null
  pushFormHistory()
}

onMounted(loadForms)`;
content = content.replace(lifecycleMarker, advancedCode);

// === 6. Enhance template - add more palette items and controls ===
// Add buttons to header
content = content.replace(
  '<button class="btn btn-outline" @click="loadForms" title="刷新列表">🔄 刷新</button>',
  '<button class="btn btn-outline" @click="showFieldTemplates=true" title="模板">📐 模板</button>\n        <button class="btn btn-outline" @click="showSchema=!showSchema" title="Schema">📋 Schema</button>\n        <button class="btn btn-outline" :disabled="!canUndo" @click="formUndo" title="撤销">↩</button>\n        <button class="btn btn-outline" :disabled="!canRedo" @click="formRedo" title="重做">↪</button>\n        <button class="btn btn-outline" @click="loadForms" title="刷新列表">🔄 刷新</button>'
);

// Add column layout selector
content = content.replace(
  '<div class="fd-actions">',
  '<div class="fd-actions">\n        <div class="col-layout">\n          <span class="col-label">列:</span>\n          <button :class=["col-btn",{active:columnCount===1}]" @click="setColumnCount(1)">1</button>\n          <button :class=["col-btn",{active:columnCount===2}]" @click="setColumnCount(2)">2</button>\n          <button :class=["col-btn",{active:columnCount===3}]" @click="setColumnCount(3)">3</button>\n        </div>'
);

// Add export/import to header
content = content.replace(
  '<button class="btn btn-primary" :disabled="!currentForm || !currentForm.name" @click="saveForm">💾 保存</button>',
  '<button class="btn btn-outline" @click="downloadFormJson">💾 导出</button>\n        <button class="btn btn-outline" @click="showIoModal=true">📥 导入</button>\n        <button class="btn btn-primary" :disabled="!currentForm || !currentForm.name" @click="saveForm">💾 保存</button>'
);

// === 7. Add template modals and enhanced UI to template ===
const templateEndMarker = '  </div>\n</template>';
const newTemplateContent = `
    <!-- Field Templates Modal -->
    <div v-if="showFieldTemplates" class="modal-overlay" @click.self="showFieldTemplates=false">
      <div class="modal modal-lg glass-card">
        <div class="modal-header"><h3>📐 表单模板</h3><button class="btn-close" @click="showFieldTemplates=false">✕</button></div>
        <div class="modal-body">
          <p class="modal-hint">选择一个模板快速生成常用表单结构</p>
          <div class="tpl-grid">
            <div v-for="(tpl, ti) in fieldTemplates" :key="ti" class="tpl-card" @click="applyTemplate(tpl)">
              <div class="tpl-icon">{{ tpl.icon }}</div>
              <div class="tpl-name">{{ tpl.name }}</div>
              <div class="tpl-count">{{ tpl.fields.length }} 个字段</div>
              <div class="tpl-preview">{{ tpl.fields.map(f=>f.icon||'📝').join(' → ') }}</div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Schema Modal -->
    <div v-if="showSchema" class="modal-overlay" @click.self="showSchema=false">
      <div class="modal modal-lg glass-card">
        <div class="modal-header"><h3>📋 Schema 视图</h3><button class="btn-close" @click="showSchema=false">✕</button></div>
        <div class="modal-body">
          <textarea class="schema-editor" readonly :value="schemaJson"></textarea>
          <div class="modal-actions">
            <button class="bc" @click="showSchema=false">关闭</button>
            <button class="bs" @click="navigator.clipboard.writeText(schemaJson)">📋 复制</button>
          </div>
        </div>
      </div>
    </div>

    <!-- Import/Export Modal -->
    <div v-if="showIoModal" class="modal-overlay" @click.self="showIoModal=false">
      <div class="modal modal-md glass-card">
        <div class="modal-header"><h3>📦 导入/导出</h3><button class="btn-close" @click="showIoModal=false">✕</button></div>
        <div class="modal-body">
          <div class="io-section">
            <label>导出 JSON</label>
            <textarea class="schema-editor" readonly :value="exportFormJson()"></textarea>
            <button class="bs" @click="downloadFormJson()">💾 下载文件</button>
            <button class="bc" @click="navigator.clipboard.writeText(exportFormJson())">📋 复制</button>
          </div>
          <div class="io-sep"></div>
          <div class="io-section">
            <label>导入 JSON</label>
            <textarea class="schema-editor" v-model="importJsonText" placeholder="// 粘贴JSON表单定义..."></textarea>
            <button class="bs" :disabled="!importJsonText.trim()" @click="importFormJson(importJsonText)">📥 导入</button>
          </div>
        </div>
      </div>
    </div>

    <!-- Schema editor state -->
    <script v-if="false"></script>
  </div>
</template>

<script setup lang="ts">
// Note: This replaces the existing script - the imports and state below are handled by the script injection above`;
// Actually we need to be more careful. Let me insert the modals before the closing </template>
content = content.replace('  </div>\n</template>', `
    <!-- Field Templates Modal -->
    <div v-if="showFieldTemplates" class="modal-overlay" @click.self="showFieldTemplates=false">
      <div class="modal modal-lg glass-card">
        <div class="modal-header"><h3>📐 表单模板</h3><button class="btn-close" @click="showFieldTemplates=false">✕</button></div>
        <div class="modal-body">
          <p class="modal-hint">选择一个模板快速生成常用表单结构</p>
          <div class="tpl-grid">
            <div v-for="(tpl, ti) in fieldTemplates" :key="ti" class="tpl-card" @click="applyTemplate(tpl)">
              <div class="tpl-icon">{{ tpl.icon }}</div>
              <div class="tpl-name">{{ tpl.name }}</div>
              <div class="tpl-count">{{ tpl.fields.length }} 个字段</div>
            </div>
          </div>
        </div>
      </div>
    </div>
    <!-- Schema Modal -->
    <div v-if="showSchema" class="modal-overlay" @click.self="showSchema=false">
      <div class="modal modal-lg glass-card">
        <div class="modal-header"><h3>📋 Schema 视图</h3><button class="btn-close" @click="showSchema=false">✕</button></div>
        <div class="modal-body">
          <textarea class="schema-editor" readonly :value="schemaJson"></textarea>
          <div class="modal-actions">
            <button class="bc" @click="showSchema=false">关闭</button>
            <button class="bs" @click="navigator.clipboard.writeText(schemaJson)">📋 复制</button>
          </div>
        </div>
      </div>
    </div>
    <!-- Import/Export Modal -->
    <div v-if="showIoModal" class="modal-overlay" @click.self="showIoModal=false">
      <div class="modal modal-md glass-card">
        <div class="modal-header"><h3>📦 导入/导出</h3><button class="btn-close" @click="showIoModal=false">✕</button></div>
        <div class="modal-body">
          <div class="io-section">
            <label>导出 JSON</label>
            <textarea class="schema-editor" readonly :value="exportFormJson()"></textarea>
            <button class="bs" @click="downloadFormJson()">💾 下载</button>
            <button class="bc" @click="navigator.clipboard.writeText(exportFormJson())">📋 复制</button>
          </div>
          <div class="io-sep"></div>
          <div class="io-section">
            <label>导入 JSON</label>
            <textarea class="schema-editor" v-model="importJsonText" placeholder="// 粘贴JSON表单定义..."></textarea>
            <button class="bs" :disabled="!importJsonText.trim()" @click="importFormJson(importJsonText)">📥 导入</button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">`;
content = content.replace('</template>\n\n<script setup lang="ts">', '');

// Add import statement for computed
content = content.replace("import { ref, computed, onMounted } from 'vue'", "import { ref, computed, onMounted, watch } from 'vue'");

// === 8. Add more CSS ===
const styleEnd = '</style>';
const extraCss = `
/* Template modal */
.modal-header{display:flex;align-items:center;justify-content:space-between;margin-bottom:16px}
.modal-header h3{font-size:16px;color:var(--color-primary);margin:0}
.modal-body{display:flex;flex-direction:column;gap:12px}
.modal-hint{font-size:12px;color:var(--text-muted)}
.tpl-grid{display:grid;grid-template-columns:repeat(auto-fill,minmax(160px,1fr));gap:10px}
.tpl-card{padding:14px;border-radius:var(--radius-md);border:1px solid var(--border-color);cursor:pointer;transition:all .15s;display:flex;flex-direction:column;gap:6px;text-align:center}
.tpl-card:hover{border-color:var(--color-primary);background:var(--color-primary-soft);transform:translateY(-2px)}
.tpl-icon{font-size:28px}.tpl-name{font-size:13px;font-weight:600;color:var(--color-primary)}
.tpl-count{font-size:10px;color:var(--text-muted)}.tpl-preview{font-size:10px;color:var(--text-muted);font-family:'JetBrains Mono',monospace}
/* Schema editor */
.schema-editor{width:100%;height:300px;padding:10px;border-radius:var(--radius-md);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-family:'JetBrains Mono',monospace;font-size:12px;resize:vertical;box-sizing:border-box}
.modal-actions{display:flex;gap:8px;justify-content:flex-end}
/* Import/Export */
.io-section{display:flex;flex-direction:column;gap:6px}
.io-section label{font-size:12px;color:var(--text-muted)}
.io-sep{height:1px;background:var(--border-color);margin:8px 0}
/* Column layout */
.col-layout{display:flex;align-items:center;gap:4px;margin-right:8px}
.col-label{font-size:11px;color:var(--text-muted)}
.col-btn{padding:2px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:transparent;color:var(--text-muted);cursor:pointer;font-size:11px}
.col-btn:hover,.col-btn.active{border-color:var(--color-primary);color:var(--color-primary)}
/* Field template palette section */
.pal-section-title{font-size:10px;color:var(--text-muted);text-transform:uppercase;letter-spacing:1px;margin:12px 0 6px;font-weight:600;border-top:1px solid var(--border-color);padding-top:8px}
/* Enhanced field row */
.field-row.has-children{border-left:3px solid var(--color-primary)}
.field-row.is-section{background:rgba(168,85,247,.08);border-color:rgba(168,85,247,.3)}
.field-row.is-divider{opacity:0.6}
.child-fields{margin-left:20px;border-left:2px dashed var(--border-color);padding-left:8px}
/* Advanced props */
.adv-section{margin-top:12px;padding-top:12px;border-top:1px solid var(--border-color)}
.adv-title{font-size:11px;color:var(--color-primary);font-weight:600;margin-bottom:8px;text-transform:uppercase;letter-spacing:1px}
.validation-rules{display:flex;flex-direction:column;gap:6px}
.val-rule-row{display:flex;align-items:center;gap:4px}
.val-rule-row select,.val-rule-row input{padding:3px 6px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:11px;outline:none}
.val-rule-row select{flex:0 0 80px;cursor:pointer}.val-rule-row input{flex:1}
.cond-list{display:flex;flex-direction:column;gap:4px;margin-top:6px}
.cond-item{display:flex;align-items:center;gap:4px;padding:4px 8px;background:var(--bg-elevated);border-radius:var(--radius-sm)}
.cond-item select,.cond-item input{padding:2px 6px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-surface);color:var(--text-primary);font-size:10px;outline:none}
.cond-item select{flex:0 0 70px;cursor:pointer}.cond-item input{flex:1}
.cond-del-btn{padding:1px 5px;border-radius:var(--radius-sm);border:none;background:transparent;color:var(--color-danger);cursor:pointer;font-size:12px}
/* Preview enhanced */
.pv-error{font-size:11px;color:var(--color-danger);margin-top:2px}
.pv-input.error{border-color:var(--color-danger)}
.pv-success{padding:12px;text-align:center;color:var(--color-success)}
/* Responsive canvas */
.fd-canvas.two-col .canvas-form{max-width:100%}
.fd-canvas.three-col .canvas-form{max-width:100%}
.canvas-form.two-col{display:grid;grid-template-columns:1fr 1fr;gap:12px}
.canvas-form.three-col{display:grid;grid-template-columns:1fr 1fr 1fr;gap:12px}
.canvas-form.two-col .field-row.col-span-2{grid-column:span 2}
.canvas-form.three-col .field-row.col-span-2{grid-column:span 2}
.canvas-form.three-col .field-row.col-span-3{grid-column:span 3}
/* Spacer and divider */
.field-row.type-spacer{height:20px;border:none;background:transparent;cursor:default}
.field-row.type-divider{height:1px;border:none;background:var(--border-color);margin:8px 0;cursor:default}
.field-row.type-html{padding:8px;background:rgba(0,212,255,.05);border:1px dashed var(--border-color)}
.field-row.type-html .fr-info{font-size:11px;color:var(--text-muted);font-family:'JetBrains Mono',monospace}`;
content = content.replace(styleEnd, extraCss + '\n</style>');

// Write back
fs.writeFileSync('D:/WORKSPACE/fakeToys/oa4rust-web/apps/desktop/src/views/FormDesigner.vue', content);
console.log('Done. Lines:', content.split('\n').length);
