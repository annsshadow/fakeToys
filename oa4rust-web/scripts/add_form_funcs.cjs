const fs = require('fs');
let content = fs.readFileSync('D:/WORKSPACE/fakeToys/oa4rust-web/apps/desktop/src/views/FormDesigner.vue', 'utf8');

// Add advanced functions before onMounted
const marker = 'onMounted(loadForms)';
const idx = content.indexOf(marker);
if (idx === -1) { console.log('NOT FOUND'); process.exit(1); }

const lines = [
'',
'// --- Form History ---',
'function pushFormHistory() {',
'  if (!currentForm.value) return',
'  formHistory.value = formHistory.value.slice(0, historyIdx.value + 1)',
'  formHistory.value.push({ fields: JSON.parse(JSON.stringify(currentForm.value.fields)), timestamp: Date.now(), label: "自动保存 " + new Date().toLocaleTimeString("zh-CN") })',
'  historyIdx.value = formHistory.value.length - 1',
'}',
'function formUndo() { if (!canUndo.value || !currentForm.value) return; historyIdx.value--; currentForm.value.fields = JSON.parse(JSON.stringify(formHistory.value[historyIdx.value].fields)) }',
'function formRedo() { if (!canRedo.value || !currentForm.value) return; historyIdx.value++; currentForm.value.fields = JSON.parse(JSON.stringify(formHistory.value[historyIdx.value].fields)) }',
'',
'// --- Field Templates ---',
'const fieldTemplates = [',
'  { name: "姓名信息", icon: "👤", fields: [{type:"text",label:"姓名",key:"name",required:true},{type:"email",label:"邮箱",key:"email"},{type:"phone",label:"手机",key:"phone"}] },',
'  { name: "订单信息", icon: "📦", fields: [{type:"text",label:"订单号",key:"order_id"},{type:"date",label:"日期",key:"order_date"},{type:"number",label:"金额",key:"amount",min:0},{type:"select",label:"状态",key:"status",optionsStr:"待支付|待支付\\n已支付|已支付\\n已完成|已完成"}] },',
'  { name: "员工信息", icon: "👥", fields: [{type:"text",label:"姓名",key:"name",required:true},{type:"select",label:"部门",key:"dept",optionsStr:"技术部|技术部\\n销售部|销售部\\n人事部|人事部"},{type:"date",label:"入职日期",key:"hire_date"},{type:"number",label:"薪资",key:"salary",min:0}] },',
'  { name: "反馈表单", icon: "💬", fields: [{type:"select",label:"类型",key:"type",optionsStr:"问题|问题反馈\\n建议|建议\\n其他|其他"},{type:"textarea",label:"内容",key:"content",rows:5,required:true},{type:"email",label:"联系方式",key:"contact"}] },',
'  { name: "调查问卷", icon: "📊", fields: [{type:"text",label:"姓名",key:"name"},{type:"select",label:"年龄",key:"age",optionsStr:"18-25|18-25岁\\n26-35|26-35岁\\n36-50|36-50岁"},{type:"rating",label:"评分",key:"rating",min:1,max:5},{type:"textarea",label:"意见",key:"comments",rows:3}] },',
'  { name: "预约表单", icon: "📅", fields: [{type:"text",label:"姓名",key:"name",required:true},{type:"phone",label:"手机",key:"phone",required:true},{type:"date",label:"日期",key:"date",required:true},{type:"textarea",label:"备注",key:"notes",rows:2}] },',
'  { name: "地址信息", icon: "📍", fields: [{type:"text",label:"省份",key:"province"},{type:"text",label:"城市",key:"city"},{type:"textarea",label:"详细地址",key:"address",rows:3},{type:"text",label:"邮编",key:"zip"}] },',
'  { name: "登录表单", icon: "🔐", fields: [{type:"email",label:"邮箱",key:"email",required:true},{type:"text",label:"密码",key:"password",placeholder:"请输入密码"},{type:"checkbox",label:"记住我",key:"remember"}] },',
']',
'function applyTemplate(tpl) { if (!currentForm.value) return; currentForm.value.fields = tpl.fields.map(f => makeField(f)); selectedField.value = null; pushFormHistory() }',
'',
'// --- Validation & Preview ---',
'function validatePreview() {',
'  if (!currentForm.value) return false',
'  previewErrors.value = {}',
'  for (const field of currentForm.value.fields) {',
'    if ([\"section\",\"section_end\",\"divider\",\"spacer\"].includes(field.type)) continue',
'    if (field.required && !previewData.value[field.key]) previewErrors.value[field.key] = field.label + " 不能为空"',
'    if (field.pattern && previewData.value[field.key] && !new RegExp(field.pattern).test(previewData.value[field.key])) previewErrors.value[field.key] = field.label + " 格式错误"',
'    if (field.minLength && previewData.value[field.key] && previewData.value[field.key].length < field.minLength) previewErrors.value[field.key] = field.label + " 至少" + field.minLength + "字符"',
'    if (field.maxLength && previewData.value[field.key] && previewData.value[field.key].length > field.maxLength) previewErrors.value[field.key] = field.label + " 最多" + field.maxLength + "字符"',
'  }',
'  return Object.keys(previewErrors.value).length === 0',
'}',
'async function submitPreview() {',
'  if (!validatePreview()) return',
'  try { await api.post("/jaxrs/form/submit", { formFlag: currentForm.value.flag, data: previewData.value }); alert("提交成功"); previewData.value = {}; previewErrors.value = {} }',
'  catch(e) { alert("提交失败: " + (e?.message ?? "")) }',
'}',
'function exportFormJson() { return JSON.stringify(currentForm.value, null, 2) }',
'function importFormJson(text) { try { const d = JSON.parse(text); currentForm.value = {...d, fields: (d.fields||[]).map(f => ({...f, id: f.id || genId()}))}; selectedField.value = null; pushFormHistory() } catch { alert("JSON格式错误") } }',
'function downloadFormJson() { const b = new Blob([exportFormJson()], { type: "application/json" }); const u = URL.createObjectURL(b); const a = document.createElement("a"); a.href = u; a.download = (currentForm.value?.flag || "form") + ".json"; a.click(); URL.revokeObjectURL(u) }',
'const schemaJson = computed(() => { if (!currentForm.value) return ""; return JSON.stringify({ name: currentForm.value.name, flag: currentForm.value.flag, layout: currentForm.value.layout, fields: currentForm.value.fields.map(f => ({ type: f.type, label: f.label, key: f.key, required: f.required, validation: f.validation, conditions: f.conditions })) }, null, 2) })',
'function setColumnCount(n) { columnCount.value = n }',
'function getFieldWidth(type) { if (["section","section_end","divider","spacer","html"].includes(type)) return "100%"; return columnCount.value === 1 ? "100%" : columnCount.value === 2 ? "50%" : "33.33%" }',
'',
'// --- Bulk Operations ---',
'function batchSetRequired(flag) { if (!currentForm.value || !selectedField.value) return; selectedField.value.required = flag }',
'function batchCopyField() { if (!currentForm.value || !selectedField.value) return; const orig = { ...selectedField.value, id: genId() }; currentForm.value.fields.push(orig); selectedField.value = orig }',
'',
marker
].join('\n');

const newContent = content.substring(0, idx) + lines + content.substring(idx);
fs.writeFileSync('D:/WORKSPACE/fakeToys/oa4rust-web/apps/desktop/src/views/FormDesigner.vue', newContent);
console.log('Lines:', newContent.split('\n').length);
