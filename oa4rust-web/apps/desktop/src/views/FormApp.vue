<template>
  <div class="mod-view">
    <!-- Header -->
    <div class="view-header glass-card">
      <div>
        <h1>表单管理</h1>
        <p class="subtitle">/jaxrs/form/* — 表单定义、版本管理、预览发布</p>
      </div>
      <div class="header-actions">
        <button class="btn-outline" @click="showSearch=true">🔍 搜索</button>
        <button class="btn-outline" @click="showImportExport=true">📤 导入导出</button>
        <button class="btn-primary" @click="showCreate=true">+ 新建表单</button>
      </div>
    </div>

    <!-- Main content -->
    <div class="content-panel glass-card">
      <!-- Tabs -->
      <div class="tabs">
        <button :class="{active:tab==='list'}" @click="tab='list'">表单列表</button>
        <button :class="{active:tab==='v2'}" @click="tab='v2'">表单V2</button>
        <button :class="{active:tab==='versions'}" @click="tab='versions'">版本历史</button>
        <button :class="{active:tab==='templates'}" @click="tab='templates'">模板库</button>
      </div>

      <!-- List tab -->
      <div v-if="tab==='list'" class="tab-content">
        <div v-if="loading" class="loading-row"><div class="sk" v-for="i in 6" :key="i"></div></div>
        <div v-else-if="items.length===0" class="empty"><div class="ei">📝</div><p>暂无表单</p></div>
        <div v-else class="item-grid">
          <div v-for="f in items" :key="f.id" class="item-card glass-card" @click="viewDetail(f)">
            <div class="ic">📝</div>
            <div class="ib">
              <div class="it">{{ f.name || f.title || '未命名表单' }}</div>
              <div class="im">flag: {{ f.flag || f.formFlag }}</div>
              <div class="iv">v{{ f.version || '1.0' }}</div>
            </div>
            <div class="ia">
              <button class="btn-sm" @click.stop="editForm(f)">✏ 编辑</button>
              <button class="btn-sm" @click.stop="previewForm(f)">👁 预览</button>
              <button class="btn-sm btn-del" @click.stop="deleteForm(f)">🗑</button>
            </div>
          </div>
        </div>
        <div class="item-footer">
          <span class="item-count">{{ items.length }} 个表单</span>
          <button class="btn-sm" @click="loadList()">🔄 刷新</button>
        </div>
      </div>

      <!-- V2 tab -->
      <div v-if="tab==='v2'" class="tab-content">
        <div v-if="loadingV2" class="loading-row"><div class="sk" v-for="i in 6" :key="i"></div></div>
        <div v-else-if="itemsV2.length===0" class="empty"><div class="ei">📋</div><p>暂无V2表单</p></div>
        <div v-else class="item-grid">
          <div v-for="f in itemsV2" :key="f.id" class="item-card glass-card" @click="viewDetailV2(f)">
            <div class="ic">📋</div>
            <div class="ib">
              <div class="it">{{ f.name || f.title || '未命名' }}</div>
              <div class="im">version: {{ f.version || '2.0' }}</div>
              <div class="im">fields: {{ f.fieldCount || 0 }}</div>
            </div>
            <div class="ia">
              <button class="btn-sm" @click.stop="editFormV2(f)">✏</button>
              <button class="btn-sm" @click.stop="previewFormV2(f)">👁</button>
            </div>
          </div>
        </div>
      </div>

      <!-- Versions tab -->
      <div v-if="tab==='versions'" class="tab-content">
        <div v-if="!selectedVersionForm" class="empty"><p>请先选择一个表单查看版本历史</p></div>
        <div v-else class="version-list">
          <div v-for="(v,vi) in versionList" :key="vi" class="version-item" :class="{active:vi===0}">
            <div class="vi-header">
              <span class="vi-version">v{{ v.version || '1.0' }}</span>
              <span class="vi-time">{{ fmtTime(v.updateTime) }}</span>
              <span class="vi-size">{{ v.size || 0 }} 字符</span>
            </div>
            <pre class="vi-diff">{{ v.snapshot?.substring(0,100) }}{{ (v.snapshot?.length||0)>100?'...':'' }}</pre>
            <div class="vi-actions">
              <button class="btn-sm" @click="restoreVersion(vi)">↩ 恢复此版本</button>
              <button class="btn-sm" @click="compareVersion(vi)">🔀 对比</button>
            </div>
          </div>
        </div>
      </div>

      <!-- Templates tab -->
      <div v-if="tab==='templates'" class="tab-content">
        <div class="template-grid">
          <div v-for="(t,ti) in formTemplates" :key="ti" class="template-card glass-card" @click="useTemplate(t)">
            <div class="tc-icon">{{ t.icon }}</div>
            <div class="tc-name">{{ t.name }}</div>
            <div class="tc-desc">{{ t.desc }}</div>
            <div class="tc-fields">{{ t.fields.length }} 个字段</div>
          </div>
        </div>
      </div>
    </div>

    <!-- Create/Edit Modal -->
    <div v-if="showCreate" class="modal-overlay" @click.self="showCreate=false">
      <div class="modal glass-card">
        <h3>{{ editingForm ? '编辑表单' : '新建表单' }}</h3>
        <div class="fg"><label>名称</label><input v-model="mform.name" class="fi" placeholder="表单名称" /></div>
        <div class="fg"><label>标识</label><input v-model="mform.flag" class="fi" placeholder="唯一标识" /></div>
        <div class="fg"><label>分类</label>
          <select v-model="mform.category" class="fi">
            <option value="">选择分类</option>
            <option value="person">人员</option><option value="org">组织</option><option value="biz">业务</option><option value="other">其他</option>
          </select>
        </div>
        <div class="fg"><label>JSON Schema</label><textarea v-model="mform.schema" class="fta code-area" rows="8" placeholder='{"fields":[...]}'></textarea></div>
        <div class="mf">
          <button class="bc" @click="showCreate=false">取消</button>
          <button class="bs" :disabled="!mform.name" @click="saveForm">{{ editingForm ? '更新' : '创建' }}</button>
        </div>
      </div>
    </div>

    <!-- Search Modal -->
    <div v-if="showSearch" class="modal-overlay" @click.self="showSearch=false">
      <div class="modal glass-card" style="width:480px">
        <h3>🔍 搜索表单</h3>
        <input v-model="searchQuery" class="fi" placeholder="搜索名称、标识..." style="width:100%" />
        <div class="search-results">
          <div v-for="f in searchedForms" :key="f.id" class="search-result-item" @click="viewDetail(f);showSearch=false">
            <span class="sri-icon">📝</span>
            <span class="sri-name">{{ f.name||f.title }}</span>
            <span class="sri-flag">{{ f.flag||f.formFlag }}</span>
          </div>
          <div v-if="searchedForms.length===0" class="empty">暂无结果</div>
        </div>
      </div>
    </div>

    <!-- Import/Export Modal -->
    <div v-if="showImportExport" class="modal-overlay" @click.self="showImportExport=false">
      <div class="modal glass-card" style="width:480px">
        <h3>📤 导入/导出</h3>
        <div class="ie-body">
          <div class="ie-option"><label>导出格式:</label>
            <select v-model="exportFmt" class="ie-select">
              <option value="json">JSON</option><option value="csv">CSV</option>
            </select>
          </div>
          <button class="bs" @click="doExport()">📥 导出全部</button>
          <div class="ie-divider">或导入</div>
          <textarea v-model="importData" class="ie-textarea" placeholder="粘贴JSON数据..."></textarea>
          <button class="bs" @click="doImport()">📤 导入</button>
          <div v-if="importMsg" :class="['ie-msg',importMsg.ok?'ok':'err']">{{importMsg.txt}}</div>
        </div>
        <div class="mf"><button class="bc" @click="showImportExport=false">关闭</button></div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { toast } from '../utils/toast'
import { api } from '@oa4rust/sdk'

type Tab = 'list' | 'v2' | 'versions' | 'templates'
type FormItem = { id: string; name?: string; title?: string; flag?: string; formFlag?: string; version?: string; category?: string; schema?: string; fieldCount?: number; updateTime?: string; snapshot?: string; size?: number }

const tab = ref<Tab>('list')
const loading = ref(false), loadingV2 = ref(false)
const items = ref<FormItem[]>([])
const itemsV2 = ref<FormItem[]>([])
const detailItem = ref<FormItem | null>(null)
const showCreate = ref(false), editingForm = ref<FormItem|null>(null)
const mform = ref({ name: '', flag: '', category: '', schema: '{}' })
const showSearch = ref(false), searchQuery = ref('')
const showImportExport = ref(false), exportFmt = ref<'json'|'csv'>('json')
const importData = ref(''), importMsg = ref<{ok:boolean;txt:string}|null>(null)
const selectedVersionForm = ref<FormItem|null>(null)
const versionList = ref<Array<{version:string;updateTime?:string;snapshot?:string;size?:number}>>([])
const formTemplates = ref<Array<{name:string;icon:string;desc:string;fields:Array<{name:string;type:string;label:string}>}>>([
  {name:'人员信息',icon:'👤',desc:'包含姓名、工号、部门等基础信息',fields:[{name:'name',type:'string',label:'姓名'},{name:'empId',type:'string',label:'工号'},{name:'dept',type:'string',label:'部门'}]},
  {name:'考勤记录',icon:'📅',desc:'包含打卡时间、出勤状态等',fields:[{name:'date',type:'date',label:'日期'},{name:'checkIn',type:'time',label:'签到'},{name:'checkOut',type:'time',label:'签退'}]},
  {name:'报销申请',icon:'💰',desc:'包含金额、事由、附件等',fields:[{name:'amount',type:'number',label:'金额'},{name:'reason',type:'text',label:'事由'},{name:'attachment',type:'file',label:'附件'}]},
  {name:'会议预约',icon:'👥',desc:'包含会议时间、地点、参与人等',fields:[{name:'title',type:'string',label:'会议主题'},{name:'time',type:'datetime',label:'时间'},{name:'room',type:'string',label:'会议室'}]},
])

const searchedForms = computed(() => {
  if (!searchQuery.value.trim()) return []
  const q = searchQuery.value.toLowerCase()
  return [...items.value, ...itemsV2.value].filter(f => (f.name||'').toLowerCase().includes(q) || (f.flag||'').toLowerCase().includes(q))
})

async function loadList() {
  loading.value = true
  try { const r = await api.get('/jaxrs/form/list'); items.value = r.data ?? [] }
  catch { items.value = [] } finally { loading.value = false }
}

async function loadV2() {
  loadingV2.value = true
  try { const r = await api.get('/jaxrs/form/v2/list'); itemsV2.value = r.data ?? [] }
  catch { itemsV2.value = [] } finally { loadingV2.value = false }
}

function viewDetail(f: FormItem) { api.get('/jaxrs/form/' + f.id).then(r => { detailItem.value = r.data ?? f }).catch(() => { detailItem.value = f }) }
function viewDetailV2(f: FormItem) { detailItem.value = f }
function editForm(f: FormItem) { editingForm.value = f; mform.value = { name: f.name||'', flag: f.flag||'', category: f.category||'', schema: f.schema||'{}' }; showCreate.value = true }
function editFormV2(f: FormItem) { editForm(f as FormItem) }
function previewForm(f: FormItem) { toast.info('预览表单: ' + (f.name||f.id)) }
function previewFormV2(f: FormItem) { previewForm(f as FormItem) }

async function saveForm() {
  if (!mform.value.name.trim()) { toast.info('请输入表单名称'); return }
  try {
    if (editingForm.value?.id) { await api.put('/jaxrs/form/update/' + editingForm.value.id, mform.value) }
    else { await api.post('/jaxrs/form/create', mform.value) }
    showCreate.value = false; loadList()
  } catch (e: any) { toast.error('保存失败: : ' + (e?.message ?? '')) }
}

async function deleteForm(f: FormItem) {
  if (!confirmMsg('确定删除表单「' + (f.name||f.id) + '」？')) return
  try { await api.delete('/jaxrs/form/delete/' + f.id); items.value = items.value.filter(x => x.id !== f.id) }
  catch (e: any) { toast.error('删除失败: : ' + (e?.message ?? '')) }
}

function useTemplate(t: any) {
  mform.value = { name: t.name, flag: t.name.toLowerCase() + '_form', category: 'biz', schema: JSON.stringify({ fields: t.fields }, null, 2) }
  editingForm.value = null; showCreate.value = true
}

function restoreVersion(vi: number) { toast.info('恢复版本 ' + (versionList.value[vi]?.version || '?')) }
function compareVersion(vi: number) { toast.info('对比版本 ' + (vi + 1)) }

function doExport() {
  const data = items.value.map(f => ({ name: f.name, flag: f.flag, version: f.version, schema: f.schema }))
  if (exportFmt.value === 'json') {
    downloadBlob(new Blob([JSON.stringify(data, null, 2)], { type: 'application/json' }), 'forms_' + new Date().toISOString().slice(0, 10) + '.json')
  } else {
    const csv = 'name,flag,version\n' + data.map(d => '"' + d.name + '","' + d.flag + '","' + d.version + '"').join('\n')
    downloadBlob(new Blob([csv], { type: 'text/csv' }), 'forms_' + new Date().toISOString().slice(0, 10) + '.csv')
  }
  showImportExport.value = false
}
function downloadBlob(blob: Blob, filename: string) {
  const a = document.createElement('a'); a.href = URL.createObjectURL(blob); a.download = filename; a.click()
}
async function doImport() {
  if (!importData.value.trim()) return
  try {
    const data = JSON.parse(importData.value)
    if (!Array.isArray(data)) { importMsg.value = { ok: false, txt: '格式错误' }; return }
    for (const f of data) { try { await api.post('/jaxrs/form/create', f) } catch {} }
    importMsg.value = { ok: true, txt: '成功导入 ' + data.length + ' 个表单' }
    loadList()
    showImportExport.value = false
  } catch (e: any) { importMsg.value = { ok: false, txt: '导入失败: ' + e.message } }
}

function fmtTime(t?: string) { if (!t) return ''; try { return new Date(t).toLocaleString('zh-CN', { month: '2-digit', day: '2-digit', hour: '2-digit', minute: '2-digit' }) } catch { return String(t) } }
loadList()

// Confirmation dialog (replaces window.confirm)
function confirmMsg(msg: string): Promise<boolean> {
  return new Promise(resolve => {
    const overlay = document.createElement('div')
    overlay.style.cssText = 'position:fixed;inset:0;background:rgba(0,0,0,.6);z-index:10000;display:flex;align-items:center;justify-content:center'
    const box = document.createElement('div')
    box.style.cssText = 'background:var(--bg-surface);border:1px solid var(--border-color);border-radius:var(--radius-lg);padding:24px;max-width:360px;width:90%;display:flex;flex-direction:column;gap:16px'
    box.innerHTML = '<p style="margin:0;color:var(--text-primary);font-size:14px">' + msg + '</p>' +
      '<div style="display:flex;gap:8px;justify-content:flex-end">' +
      '<button class="tc-cancel" style="padding:6px 16px;border-radius:var(--radius-md);border:1px solid var(--border-color);background:transparent;color:var(--text-primary);cursor:pointer">取消</button>' +
      '<button class="tc-ok" style="padding:6px 16px;border-radius:var(--radius-md);border:none;background:var(--color-primary);color:#000;cursor:pointer;font-weight:600">确认</button>' +
      '</div>'
    overlay.appendChild(box)
    document.body.appendChild(overlay)
    const ok = () => { overlay.remove(); resolve(true) }
    const cancel = () => { overlay.remove(); resolve(false) }
    box.querySelector('.tc-ok').addEventListener('click', ok)
    box.querySelector('.tc-cancel').addEventListener('click', cancel)
    overlay.addEventListener('click', e => { if (e.target === overlay) cancel() })
  })
}


const form_f_1_appinfo_app_1_ref = ref<any[]>([]);
const form_f_1_appinfo_app_1_q = useQuery({
  queryKey: ['form_f_1_appinfo_app_1'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/form/f-1/appinfo/app-1"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const form_v2_f_1_mobile_ref = ref<any[]>([]);
const form_v2_f_1_mobile_q = useQuery({
  queryKey: ['form_v2_f_1_mobile'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/form/v2/f-1/mobile"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const form_list_all_ref = ref<any[]>([]);
const form_list_all_q = useQuery({
  queryKey: ['form_list_all'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/form/list/all"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const form_f_1_mockdeletetoget_ref = ref<any[]>([]);
const form_f_1_mockdeletetoget_q = useQuery({
  queryKey: ['form_f_1_mockdeletetoget'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/form/f-1/mockdeletetoget"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const form_f_1_ref = ref<any[]>([]);
const form_f_1_q = useQuery({
  queryKey: ['form_f_1'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/form/f-1"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const formversion_list_form_f_1_ref = ref<any[]>([]);
const formversion_list_form_f_1_q = useQuery({
  queryKey: ['formversion_list_form_f_1'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/formversion/list/form/f-1"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const formversion_fv_1_ref = ref<any[]>([]);
const formversion_fv_1_q = useQuery({
  queryKey: ['formversion_fv_1'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/formversion/fv-1"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});

</script>

<style scoped>
.mod-view{display:flex;flex-direction:column;gap:16px;height:100%}
.view-header{display:flex;align-items:center;justify-content:space-between;padding:16px 24px;flex-wrap:wrap;gap:8px}
.view-header h1{font-family:'Orbitron',sans-serif;font-size:20px;color:var(--color-primary);margin:0 0 4px;text-shadow:0 0 15px var(--color-primary-glow)}
.subtitle{font-size:12px;color:var(--text-muted);margin:0;font-family:'JetBrains Mono',monospace}
.header-actions{display:flex;gap:8px;flex-wrap:wrap}
.btn-outline{padding:8px 16px;background:transparent;border:1px solid var(--border-subtle);color:var(--text-secondary);border-radius:var(--radius-md);cursor:pointer;font-size:13px}
.btn-outline:hover{border-color:var(--color-primary);color:var(--color-primary)}
.btn-primary{padding:8px 16px;background:var(--color-primary);color:#000;border:none;border-radius:var(--radius-md);cursor:pointer;font-size:13px;font-weight:600}
.btn-sm{padding:4px 10px;border-radius:var(--radius-sm);border:1px solid var(--border-subtle);background:transparent;color:var(--text-secondary);cursor:pointer;font-size:11px}
.btn-sm:hover{border-color:var(--color-primary);color:var(--color-primary)}
.btn-del{border-color:var(--color-error);color:var(--color-error)}
.content-panel{flex:1;overflow-y:auto;padding:16px;display:flex;flex-direction:column;gap:16px}
.tabs{display:flex;gap:8px;flex-wrap:wrap}
.tabs button{padding:8px 20px;background:var(--bg-elevated);border:1px solid var(--border-subtle);border-radius:var(--radius-md);color:var(--text-secondary);font-size:13px;cursor:pointer;transition:all var(--transition-fast)}
.tabs button.active{background:var(--color-primary);color:#000;border-color:var(--color-primary);font-weight:600}
.tab-content{flex:1;display:flex;flex-direction:column;gap:12px}
.item-grid{display:grid;grid-template-columns:repeat(auto-fill,minmax(220px,1fr));gap:12px}
.item-card{display:flex;align-items:center;gap:12px;padding:14px;cursor:pointer;transition:all var(--transition-fast);border:1px solid var(--border-subtle);border-radius:var(--radius-md);background:var(--bg-elevated)}
.item-card:hover{border-color:var(--color-primary);transform:translateX(4px);box-shadow:var(--shadow-glow)}
.ic{font-size:28px;flex-shrink:0}
.ib{flex:1;min-width:0}
.it{font-size:14px;font-weight:600;color:var(--text-primary)}
.im{font-size:11px;color:var(--text-muted);margin-top:2px}
.ia{display:flex;gap:4px;flex-shrink:0}
.item-footer{display:flex;align-items:center;justify-content:space-between;padding:8px 0;border-top:1px solid var(--border-subtle)}
.item-count{font-size:12px;color:var(--text-muted)}
.empty,.loading-row{display:flex;flex-direction:column;align-items:center;justify-content:center;padding:40px;color:var(--text-muted);gap:12px;flex:1}
.ei{font-size:48px;opacity:0.4}
.sk{height:40px;border-radius:var(--radius-md);background:var(--bg-elevated);animation:pulse 1.2s ease-in-out infinite}
@keyframes pulse{0%,100%{opacity:.4}50%{opacity:.8}}
.modal-overlay{position:fixed;inset:0;background:rgba(0,0,0,.7);display:flex;align-items:center;justify-content:center;z-index:100}
.modal{background:var(--bg-surface);border:1px solid var(--border-subtle);border-radius:var(--radius-lg);padding:24px;width:500px;max-width:90vw;max-height:85vh;display:flex;flex-direction:column;gap:14px;overflow-y:auto}
.modal h3{font-family:'Orbitron',sans-serif;color:var(--color-primary);margin:0;font-size:15px}
.fg{display:flex;flex-direction:column;gap:6px}
.fg label{font-size:13px;color:var(--text-muted)}
.fi,.fta{background:var(--bg-elevated);border:1px solid var(--border-subtle);border-radius:var(--radius-md);color:var(--text-primary);padding:10px 12px;font-size:14px}
.fi:focus,.fta:focus{outline:none;border-color:var(--color-primary)}
.fta{resize:vertical;min-height:100px;font-family:'JetBrains Mono',monospace}
.mf{display:flex;justify-content:flex-end;gap:8px;margin-top:8px}
.bc{padding:8px 20px;background:transparent;border:1px solid var(--border-subtle);color:var(--text-secondary);border-radius:var(--radius-md);cursor:pointer}
.bs{padding:8px 20px;background:var(--color-primary);color:#000;border:none;border-radius:var(--radius-md);font-size:13px;cursor:pointer;font-weight:600}
.bs:disabled{opacity:0.5;cursor:not-allowed}
.code-area{font-size:12px}
/* Search */
.search-results{display:flex;flex-direction:column;gap:4px;max-height:300px;overflow-y:auto}
.search-result-item{display:flex;align-items:center;gap:8px;padding:8px 12px;border-radius:var(--radius-sm);cursor:pointer;font-size:13px}
.search-result-item:hover{background:var(--color-primary-soft)}
.sri-icon{font-size:16px}.sri-name{flex:1;color:var(--text-primary)}.sri-flag{font-size:11px;color:var(--text-muted);font-family:monospace}
/* Version list */
.version-list{display:flex;flex-direction:column;gap:8px}
.version-item{padding:12px;border:1px solid var(--border-subtle);border-radius:var(--radius-md);background:var(--bg-elevated)}
.version-item.active{border-color:var(--color-primary);background:rgba(59,130,246,0.05)}
.vi-header{display:flex;align-items:center;gap:8px;font-size:12px;margin-bottom:6px}
.vi-version{font-weight:600;color:var(--color-primary);font-family:monospace}
.vi-time,.vi-size{color:var(--text-muted)}
.vi-diff{margin:0;padding:6px 10px;background:rgba(0,0,0,0.3);color:#7fdbca;font-size:11px;font-family:monospace;border-radius:var(--radius-sm);max-height:60px;overflow-y:auto;white-space:pre-wrap}
.vi-actions{display:flex;gap:6px;margin-top:6px}
/* Templates */
.template-grid{display:grid;grid-template-columns:repeat(auto-fill,minmax(180px,1fr));gap:12px}
.template-card{padding:16px;text-align:center;cursor:pointer;border:1px solid var(--border-subtle);border-radius:var(--radius-md);background:var(--bg-elevated);transition:all var(--transition-fast)}
.template-card:hover{border-color:var(--color-primary);transform:translateY(-2px);box-shadow:var(--shadow-glow)}
.tc-icon{font-size:32px;margin-bottom:8px}.tc-name{font-size:13px;font-weight:600;color:var(--text-primary)}.tc-desc{font-size:11px;color:var(--text-muted);margin-top:4px}.tc-fields{font-size:10px;color:var(--color-primary);margin-top:4px}
/* Import/Export */
.ie-body{display:flex;flex-direction:column;gap:10px}
.ie-option{display:flex;align-items:center;gap:8px;font-size:12px}
.ie-select{padding:5px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-subtle);background:var(--bg-elevated);color:var(--text-primary);font-size:11px;outline:none}
.ie-divider{font-size:12px;color:var(--text-muted);text-align:center;padding:4px 0}
.ie-textarea{width:100%;height:100px;background:rgba(0,0,0,0.3);border:1px solid var(--border-subtle);color:#7fdbca;font-family:monospace;font-size:11px;padding:10px;resize:vertical;outline:none;border-radius:var(--radius-sm)}
.ie-msg{padding:8px;border-radius:var(--radius-sm);font-size:12px}
.ie-msg.ok{background:rgba(16,185,129,0.1);color:#10b981}
.ie-msg.err{background:rgba(239,68,68,0.1);color:#ef4444}
@media(max-width:768px){.item-grid{grid-template-columns:1fr}.view-header{flex-direction:column;align-items:flex-start}}
</style>
