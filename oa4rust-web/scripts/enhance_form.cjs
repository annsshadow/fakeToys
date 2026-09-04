const fs = require('fs');
let content = fs.readFileSync('D:/WORKSPACE/fakeToys/oa4rust-web/apps/desktop/src/views/FormDesigner.vue', 'utf8');

// 1. Enhance interfaces
content = content.replace(
  'interface FormField {\n  id: string; type: string; label: string; key: string\n  placeholder?: string; defaultValue?: string; required?: boolean; disabled?: boolean\n  rows?: number; min?: number; max?: number; optionsStr?: string\n}',
  `interface FormField {
  id: string; type: string; label: string; key: string
  placeholder?: string; defaultValue?: string; required?: boolean; disabled?: boolean
  rows?: number; min?: number; max?: number; optionsStr?: string
  description?: string; cssClass?: string; helpText?: string
  maxLength?: number; minLength?: number; pattern?: string; step?: number
  readonly?: boolean; hidden?: boolean; colSpan?: number
  conditions?: FieldCondition[]
  validation?: FieldValidation
}
interface FieldCondition { operator: string; value: string; fieldKey?: string }
interface FieldValidation { required?: boolean; min?: number; max?: number; minLength?: number; maxLength?: number; pattern?: string; patternMsg?: string; customMsg?: string }
interface FormDef {
  id?: string; name: string; flag: string; desc?: string
  layout?: 'single'|'two_col'|'three_col'
  fields: FormField[]; updatedAt?: string; version?: string
  settings?: { showReset?: boolean; showSubmit?: boolean; layoutClass?: string }
}`
);

// 2. Add more field types
content = content.replace(
  "  { type: 'row_end',   label: '结束行', icon: '↩️' },\n]",
  `  { type: 'row_end',    label: '结束行', icon: '↩️' },
  { type: 'columns',    label: '多列布局', icon: '▥' },
  { type: 'spacer',     label: '间距', icon: '↕️' },
  { type: 'divider',    label: '分割线', icon: '—' },
  { type: 'html',       label: 'HTML', icon: '📝' },
]
const extraFieldTypes = [
  { type: 'rating', label: '评分', icon: '⭐' },
  { type: 'slider', label: '滑块', icon: '🎚️' },
  { type: 'color', label: '颜色', icon: '🎨' },
  { type: 'signature', label: '签字', icon: '✍️' },
  { type: 'image', label: '图片', icon: '🖼️' },
  { type: 'rich_text', label: '富文本', icon: '📝' },
  { type: 'json', label: 'JSON', icon: '{ }' },
  { type: 'cascader', label: '级联', icon: '🌳' },
  { type: 'switch', label: '开关', icon: '🔘' },
  { type: 'upload', label: '上传', icon: '📤' },
  { type: 'map', label: '地图', icon: '🗺️' },
  { type: 'code', label: '代码', icon: '</>' },
]`
);

// 3. Add new state variables
content = content.replace(
  "const mode = ref<'edit'|'preview'>('edit')",
  `const mode = ref<'edit'|'preview'|'schema'>('edit')
const listFilter = ref('')
const showFieldTemplates = ref(false)
const showSchema = ref(false)
const showIoModal = ref(false)
const importJsonText = ref('')
const columnCount = ref<1|2|3>(1)
const formHistory = ref<{fields: FormField[]; timestamp: number; label: string}[]>([])
const historyIdx = ref(-1)
const canUndo = computed(() => historyIdx.value > 0)
const canRedo = computed(() => historyIdx.value < formHistory.value.length - 1)
const previewData = ref<Record<string,any>>({})
const previewErrors = ref<Record<string,string>>({})`
);

// 4. Enhance makeField
content = content.replace(
  "    section: { label: '分组标题', key: '' },\n  }",
  `    section: { label: '分组标题', key: '' },
    section_end: { label: '分组结束', key: '' },
    row_start: { label: '开始行', key: '' },
    row_end: { label: '结束行', key: '' },
    columns: { label: '多列布局', key: '' },
    spacer: { label: '间距', key: '' },
    divider: { label: '分割线', key: '' },
    html: { label: 'HTML内容', key: '' },
    rating: { label: '评分', key: 'rating_field', min: 1, max: 5 },
    slider: { label: '滑块', key: 'slider_field', min: 0, max: 100, defaultValue: '50' },
    color: { label: '颜色选择', key: 'color_field', defaultValue: '#000000' },
    signature: { label: '签字', key: 'signature_field' },
    image: { label: '图片上传', key: 'image_field' },
    rich_text: { label: '富文本', key: 'rich_text_field' },
    json: { label: 'JSON编辑器', key: 'json_field' },
    cascader: { label: '级联选择', key: 'cascader_field' },
    switch: { label: '开关', key: 'switch_field', defaultValue: 'false' },
    upload: { label: '文件上传', key: 'upload_field' },
    map: { label: '地图', key: 'map_field' },
    code: { label: '代码编辑', key: 'code_field' },
  }`
);

// 5. Add advanced functions before onMounted
content = content.replace(
  'onMounted(loadForms)',
  `
// --- Form History ---
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
  { name: '登录表单', icon: '🔐', fields: [{type:'email',label:'邮箱',key:'email',required:true},{type:'text',label:'密码',key:'password'},{type:'checkbox',label:'记住我',key:'remember'}] },
  { name: '注册表单', icon: '📝', fields: [{type:'text',label:'用户名',key:'username',required:true},{type:'email',label:'邮箱',key:'email',required:true},{type:'text',label:'密码',key:'password',required:true},{type:'checkbox',label:'同意条款',key:'agree',required:true}] },
  { name: '联系人', icon: '📒', fields: [{type:'text',label:'姓名',key:'name'},{type:'phone',label:'手机',key:'phone'},{type:'email',label:'邮箱',key:'email'},{type:'text',label:'公司',key:'company'},{type:'textarea',label:'备注',key:'note',rows:2}] },
]
function applyTemplate(tpl: typeof fieldTemplates[0]) {
  if (!currentForm.value) return
  currentForm.value.fields = tpl.fields.map(f => makeField(f as any))
  selectedField.value = null
  pushFormHistory()
}

// --- Validation & Preview ---
function validatePreview(): boolean {
  if (!currentForm.value) return false
  previewErrors.value = {}
  for (const field of currentForm.value.fields) {
    if (['section','section_end','divider','spacer'].includes(field.type)) continue
    if (field.required && !previewData.value[field.key]) previewErrors.value[field.key] = field.label + ' 不能为空'
    if (field.pattern && previewData.value[field.key] && !new RegExp(field.pattern).test(previewData.value[field.key])) previewErrors.value[field.key] = field.label + ' 格式错误'
    if (field.minLength && previewData.value[field.key] && previewData.value[field.key].length < field.minLength) previewErrors.value[field.key] = field.label + ' 至少' + field.minLength + '字符'
    if (field.maxLength && previewData.value[field.key] && previewData.value[field.key].length > field.maxLength) previewErrors.value[field.key] = field.label + ' 最多' + field.maxLength + '字符'
    if (field.min !== undefined && previewData.value[field.key] && Number(previewData.value[field.key]) < field.min) previewErrors.value[field.key] = field.label + ' 不能小于' + field.min
    if (field.max !== undefined && previewData.value[field.key] && Number(previewData.value[field.key]) > field.max) previewErrors.value[field.key] = field.label + ' 不能大于' + field.max
  }
  return Object.keys(previewErrors.value).length === 0
}
async function submitPreview() {
  if (!validatePreview()) return
  try { await api.post('/jaxrs/form/submit', { formFlag: currentForm.value.flag, data: previewData.value }); alert('提交成功'); previewData.value = {}; previewErrors.value = {} }
  catch(e: any) { alert('提交失败: ' + (e?.message ?? '')) }
}

// --- Import/Export ---
function exportFormJson(): string { return JSON.stringify(currentForm.value, null, 2) }
function importFormJson(text: string) {
  try { const d = JSON.parse(text); currentForm.value = {...d, fields: (d.fields||[]).map((f:any)=>({...f,id:f.id||genId()}))}; selectedField.value=null; pushFormHistory() }
  catch { alert('JSON格式错误') }
}
function downloadFormJson() {
  const b = new Blob([exportFormJson()], {type:'application/json'}); const u = URL.createObjectURL(b)
  const a = document.createElement('a'); a.href=u; a.download=(currentForm.value?.flag||'form')+'.json'; a.click(); URL.revokeObjectURL(u)
}

// --- Schema ---
const schemaJson = computed(() => {
  if (!currentForm.value) return ''
  return JSON.stringify({name:currentForm.value.name,flag:currentForm.value.flag,layout:currentForm.value.layout,fields:currentForm.value.fields.map(f=>({type:f.type,label:f.label,key:f.key,required:f.required,validation:f.validation}))},null,2)
})

// --- Column Layout ---
function setColumnCount(n:1|2|3) { columnCount.value = n }
function getFieldWidth(type:string): string {
  if (['section','section_end','divider','spacer','html'].includes(type)) return '100%'
  return columnCount.value===1?'100%':columnCount.value===2?'50%':'33.33%'
}

// --- Bulk Operations ---
function batchCopyField() {
  if (!currentForm.value||!selectedField.value) return
  const orig={...selectedField.value,id:genId()}
  currentForm.value.fields.push(orig); selectedField.value=orig; pushFormHistory()
}
function batchDeleteField() {
  if (!currentForm.value||!selectedField.value) return
  const idx=currentForm.value.fields.findIndex(f=>f.id===selectedField.value!.id)
  if(idx!==-1){currentForm.value.fields.splice(idx,1);selectedField.value=null;pushFormHistory()}
}

// --- Conditional Logic ---
function addCondition() {
  if (!selectedField.value) return
  if (!(selectedField.value as any).conditions) (selectedField.value as any).conditions = []
  ;(selectedField.value as any).conditions.push({operator:'equals',value:'',fieldKey:''})
}
function removeCondition(i:number) {
  if (!selectedField.value) return
  const conds = (selectedField.value as any).conditions
  if (Array.isArray(conds)) conds.splice(i, 1)
}

onMounted(loadForms)`
);

// 6. Add CSS
const styleEnd = '</style>';
const extraCss = `
/* Templates modal */
.modal-overlay{position:fixed;inset:0;background:rgba(0,0,0,.6);display:flex;align-items:center;justify-content:center;z-index:200}
.modal{background:var(--bg-surface);border:1px solid var(--border-color);border-radius:var(--radius-lg);padding:20px;max-height:85vh;overflow-y:auto}
.modal-lg{width:720px}.modal-md{width:480px}
.modal-header{display:flex;align-items:center;justify-content:space-between;margin-bottom:16px}
.modal-header h3{font-size:16px;color:var(--color-primary);margin:0}
.modal-body{display:flex;flex-direction:column;gap:12px}
.modal-hint{font-size:12px;color:var(--text-muted)}
.modal-actions{display:flex;gap:8px;justify-content:flex-end}
.tpl-grid{display:grid;grid-template-columns:repeat(auto-fill,minmax(130px,1fr));gap:10px}
.tpl-card{padding:14px;border-radius:var(--radius-md);border:1px solid var(--border-color);cursor:pointer;transition:all .15s;display:flex;flex-direction:column;align-items:center;gap:6px;text-align:center}
.tpl-card:hover{border-color:var(--color-primary);background:var(--color-primary-soft);transform:translateY(-2px)}
.tpl-icon{font-size:28px}.tpl-name{font-size:13px;font-weight:600;color:var(--color-primary)}
.tpl-count{font-size:10px;color:var(--text-muted)}
.schema-editor{width:100%;height:280px;padding:10px;border-radius:var(--radius-md);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-family:'JetBrains Mono',monospace;font-size:12px;resize:vertical;box-sizing:border-box}
.io-section{display:flex;flex-direction:column;gap:6px}
.io-section label{font-size:12px;color:var(--text-muted)}
.io-sep{height:1px;background:var(--border-color);margin:8px 0}
.col-layout{display:flex;align-items:center;gap:4px}
.col-label{font-size:11px;color:var(--text-muted)}
.col-btn{padding:2px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:transparent;color:var(--text-muted);cursor:pointer;font-size:11px}
.col-btn:hover,.col-btn.active{border-color:var(--color-primary);color:var(--color-primary)}
.field-row.is-section{background:rgba(168,85,247,.08);border-color:rgba(168,85,247,.3)}
.field-row.is-divider{opacity:.6}
.field-row.is-spacer{height:12px;border:none;background:transparent;cursor:default}
.fr-cond-badge{font-size:9px;padding:1px 4px;border-radius:var(--radius-sm);background:rgba(245,158,11,.2);color:var(--color-warning)}
.fr-required-badge{font-size:9px;color:var(--color-danger)}
.pv-error-msg{font-size:11px;color:var(--color-danger);margin-top:2px}
.pv-input.error{border-color:var(--color-danger)}
.pv-success-msg{padding:16px;text-align:center;color:var(--color-success);font-size:14px}
.fd-canvas.two-col .canvas-form{display:grid;grid-template-columns:1fr 1fr;gap:12px}
.fd-canvas.three-col .canvas-form{display:grid;grid-template-columns:1fr 1fr 1fr;gap:12px}
.canvas-form.two-col .field-row.span-2{grid-column:span 2}
.canvas-form.three-col .field-row.span-2{grid-column:span 2}
.canvas-form.three-col .field-row.span-3{grid-column:span 3}
.spacer-preview{height:16px;background:repeating-linear-gradient(90deg,var(--border-color),var(--border-color) 4px,transparent 4px,transparent 8px);border-radius:2px;margin:4px 0}
.divider-preview{height:1px;background:var(--border-color);margin:8px 0}
`;
content = content.replace(styleEnd, extraCss + '\n</style>');

fs.writeFileSync('D:/WORKSPACE/fakeToys/oa4rust-web/apps/desktop/src/views/FormDesigner.vue', content);
console.log('Lines:', content.split('\n').length);
