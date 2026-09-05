<template>
  <div class="editor-view">
    <!-- Header -->
    <div class="view-header glass-card">
      <div>
        <h1>配置设计器</h1>
        <p class="subtitle">/jaxrs/config/* — 系统配置与参数管理</p>
      </div>
      <div class="header-actions">
        <button class="btn-outline" @click="showFormat=true">📐 格式化</button>
        <button class="btn-outline" @click="showHistory=true">📜 历史</button>
        <button class="btn-outline" @click="showImportExport=true">📤 导入导出</button>
        <button class="btn-secondary" @click="preview">👁 预览</button>
        <button class="btn-primary" @click="save">💾 保存</button>
      </div>
    </div>

    <!-- Toolbar -->
    <div class="toolbar glass-card">
      <div class="toolbar-left">
        <select v-model="editorLang" class="tb-select">
          <option value="json">JSON</option>
          <option value="yaml">YAML</option>
          <option value="properties">Properties</option>
        </select>
        <select v-model="editorTheme" class="tb-select">
          <option value="dark">暗色主题</option>
          <option value="light">亮色主题</option>
        </select>
      </div>
      <div class="toolbar-right">
        <span class="tb-info">{{ configLines }} 行 · {{ config.length }} 字符</span>
        <button class="btn-sm" @click="formatConfig">📐 格式化</button>
        <button class="btn-sm" @click="validateConfig">✅ 验证</button>
        <button class="btn-sm btn-danger" @click="clearConfig">🗑 清空</button>
      </div>
    </div>

    <!-- Editor layout -->
    <div class="editor-layout glass-card">
      <!-- Sidebar -->
      <div class="editor-sidebar">
        <div class="sb-search">
          <input v-model="searchKey" placeholder="搜索配置..." class="sb-input" />
        </div>
        <div class="sb-tabs">
          <button :class="{active:sbTab==='all'}" @click="sbTab='all'">全部</button>
          <button :class="{active:sbTab==='system'}" @click="sbTab='system'">系统</button>
          <button :class="{active:sbTab==='business'}" @click="sbTab='business'">业务</button>
        </div>
        <div class="sb-list">
          <div v-if="loading" class="loading-sm">加载中...</div>
          <template v-else>
            <div v-for="item in filteredItems" :key="item.id" class="sb-item"
              :class="{active:selected?.id===item.id}"
              @click="selectItem(item)">
              <div class="si-icon">{{ itemIcon(item) }}</div>
              <div class="si-info">
                <div class="si-name">{{ item.name||item.flag||item.id }}</div>
                <div class="si-meta">{{ item.category||'通用' }}</div>
              </div>
              <div class="si-actions">
                <button class="si-btn" @click.stop="editItem(item)" title="编辑">✏</button>
                <button class="si-btn si-del" @click.stop="deleteItem(item)" title="删除">🗑</button>
              </div>
            </div>
            <div v-if="filteredItems.length===0" class="empty">暂无配置</div>
          </template>
        </div>
        <button class="btn-sm sb-add" @click="createNew">+ 新建配置</button>
      </div>

      <!-- Main editor -->
      <div class="editor-main">
        <div v-if="!selected" class="empty-main">
          <div class="emi">⚙</div>
          <p>选择或创建配置</p>
        </div>
        <div v-else class="editor-content">
          <div class="ec-header">
            <span class="ec-title">{{ selected.name||selected.flag||'未命名' }}</span>
            <span class="ec-meta">{{ selected.category||'通用' }} · {{ fmtTime(selected.updateTime) }}</span>
          </div>
          <div class="ec-breadcrumb" v-if="selected.config">
            <span class="bc-label">配置路径:</span>
            <code class="bc-path">{{ selected.flag || selected.id }}</code>
          </div>
          <textarea v-model="config" class="code-editor" :placeholder="'在此输入JSON配置...'" spellcheck="false" @input="onConfigChange"></textarea>
          <div class="ec-footer">
            <div class="ec-status">{{ statusText }}</div>
            <div class="ec-actions">
              <button class="btn-sm" @click="copyConfig">📋 复制</button>
              <button class="btn-sm" @click="downloadConfig">📥 下载</button>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Format Modal -->
    <div v-if="showFormat" class="modal-overlay" @click.self="showFormat=false">
      <div class="modal glass-card">
        <h3>📐 JSON 格式化</h3>
        <pre class="fmt-out">{{ formattedOutput }}</pre>
        <div class="modal-actions">
          <button class="btn-cancel" @click="showFormat=false">关闭</button>
          <button class="btn-save" @click="applyFormat()">✓ 应用到编辑器</button>
        </div>
      </div>
    </div>

    <!-- History Modal -->
    <div v-if="showHistory" class="modal-overlay" @click.self="showHistory=false">
      <div class="modal glass-card" style="width:520px">
        <h3>📜 配置历史</h3>
        <div class="history-list">
          <div v-for="(h,hi) in configHistory" :key="hi" class="hist-item">
            <div class="hist-meta">
              <span class="hist-time">{{h.time}}</span>
              <span class="hist-size">{{h.size}} 字符</span>
              <span :class="['hist-tag',h.isAuto?'auto':'manual']">{{h.isAuto?'自动':'手动'}}</span>
            </div>
            <pre class="hist-preview">{{h.snapshot.substring(0,80)}}</pre>
            <div class="hist-actions">
              <button class="btn-sm" @click="restoreHistory(hi)">↩ 恢复</button>
              <button class="btn-sm btn-del" @click="configHistory.splice(hi,1)">🗑</button>
            </div>
          </div>
          <div v-if="configHistory.length===0" class="hist-empty">暂无历史记录</div>
        </div>
        <div class="hist-footer">
          <button class="btn-sm" @click="configHistory=[]">清除</button>
          <button class="btn-cancel" @click="showHistory=false">关闭</button>
        </div>
      </div>
    </div>

    <!-- Import/Export Modal -->
    <div v-if="showImportExport" class="modal-overlay" @click.self="showImportExport=false">
      <div class="modal glass-card" style="width:480px">
        <h3>📤 导入/导出</h3>
        <div class="ie-body">
          <div class="ie-section">
            <div class="ie-title">导出配置</div>
            <button class="btn-sm" @click="exportConfigs()">📥 导出全部JSON</button>
            <button class="btn-sm" @click="exportSelected()">📥 导出当前</button>
          </div>
          <div class="ie-section">
            <div class="ie-title">导入配置</div>
            <textarea v-model="importData" class="ie-textarea" placeholder="粘贴JSON配置..."></textarea>
            <button class="btn-sm" @click="importConfigs()">📤 导入</button>
            <div v-if="importMsg" :class="['ie-msg',importMsg.ok?'ok':'err']">{{importMsg.txt}}</div>
          </div>
        </div>
        <div class="ie-footer"><button class="btn-cancel" @click="showImportExport=false">关闭</button></div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { toast } from '../utils/toast'
import { useQuery, useMutation, useQueryClient } from '@tanstack/vue-query'
import { api } from '@oa4rust/sdk'

interface ConfigItem { id:string; name?:string; flag?:string; category?:string; config?:string; updateTime?:string; createTime?:string }

const loading = ref(false), searchKey = ref(''), sbTab = ref('all')
const selected = ref<ConfigItem|null>(null), config = ref('')
const editorLang = ref('json'), editorTheme = ref('dark')
const configLines = computed(() => config.value.split('\n').length)
const statusText = computed(() => selected.value ? `已选择: ${selected.value.name||selected.value.flag}` : '未选择配置')
const formattedOutput = computed(() => { try { return JSON.stringify(JSON.parse(config.value), null, 2) } catch { return config.value } })
const showFormat = ref(false), showHistory = ref(false), showImportExport = ref(false)
const configHistory = ref<Array<{time:string;size:number;snapshot:string;isAuto:boolean}>>([])
const importData = ref(''), importMsg = ref<{ok:boolean;txt:string}|null>(null)

const qc = useQueryClient()
const { data } = useQuery({ queryKey: ['config','list'], queryFn: async () => { loading.value=true; try { const r:any = await api.get('/jaxrs/config/list'); return r?.data ?? [] } finally { loading.value=false } } })
const items = ref<ConfigItem[]>(data.value ?? [])

const filteredItems = computed(() => {
  let list = items.value
  if (searchKey.value) { const q = searchKey.value.toLowerCase(); list = list.filter(i => (i.name||i.flag||'').toLowerCase().includes(q)) }
  if (sbTab.value !== 'all') list = list.filter(i => (i.category||'').toLowerCase() === sbTab.value)
  return list
})

function itemIcon(item: ConfigItem) {
  const cat = (item.category||'').toLowerCase()
  if (cat === 'system') return '⚙'
  if (cat === 'business') return '📋'
  return '🔧'
}

function selectItem(item: ConfigItem) { selected.value = item; config.value = item.config ? '\n' + item.config : '{}' }
function createNew() {
  const n: ConfigItem = { id: 'c'+Date.now(), name: '未命名', flag: '', config: '{}', category: 'business' }
  items.value = [n, ...items.value]; selectItem(n)
}
function editItem(item: ConfigItem) { selectItem(item) }
async function deleteItem(item: ConfigItem) {
  if (!confirmMsg(`删除配置「${item.name||item.flag}」？`)) return
  try { await api.delete('/jaxrs/config/delete/'+item.id) } catch {}
  items.value = items.value.filter(i => i.id !== item.id)
  if (selected.value?.id === item.id) selected.value = null
}

async function save() {
  if (!selected.value) return
  try {
    await api.put('/jaxrs/config/update/'+selected.value.id, { ...selected.value, config: config.value })
    qc.invalidateQueries({ queryKey: ['config','list'] })
    addHistory(true)
  } catch (e: any) { toast.error('保存失败: : ' + (e?.message??'')) }
}
async function preview() { toast.info('配置预览: ' + config.value) }
function clearConfig() { if(confirmMsg('清空配置？')) config.value = '{}' }
function formatConfig() { try { config.value = JSON.stringify(JSON.parse(config.value), null, 2) } catch { toast.info('JSON格式错误') } }
function validateConfig() { try { JSON.parse(config.value); toast.info('JSON格式有效') } catch (e: any) { toast.error('JSON格式错误: ' + e.message) } }
function applyFormat() { config.value = formattedOutput.value; showFormat.value = false }
function copyConfig() { navigator.clipboard.writeText(config.value); toast.info('已复制') }
function downloadConfig() {
  const blob = new Blob([config.value], { type: 'application/json' })
  const a = document.createElement('a'); a.href = URL.createObjectURL(blob)
  a.download = (selected.value?.flag || 'config') + '.json'; a.click()
}
function exportConfigs() {
  const data = items.value.map(i => ({ name: i.name, flag: i.flag, category: i.category, config: i.config }))
  const blob = new Blob([JSON.stringify(data, null, 2)], { type: 'application/json' })
  const a = document.createElement('a'); a.href = URL.createObjectURL(blob); a.download = 'configs_'+new Date().toISOString().slice(0,10)+'.json'; a.click()
}
function exportSelected() {
  if (!selected.value) return
  const blob = new Blob([JSON.stringify({ name: selected.value.name, flag: selected.value.flag, config: config.value }, null, 2)], { type: 'application/json' })
  const a = document.createElement('a'); a.href = URL.createObjectURL(blob); a.download = (selected.value.flag||'config')+'.json'; a.click()
}
function importConfigs() {
  try {
    const data = JSON.parse(importData.value)
    if (Array.isArray(data)) {
      for (const item of data) { try { api.post('/jaxrs/config/create', item) } catch {} }
      importMsg.value = { ok: true, txt: '成功导入 '+data.length+' 项' }
    } else { importMsg.value = { ok: false, txt: '格式错误: 期望数组' } }
    qc.invalidateQueries({ queryKey: ['config','list'] })
  } catch(e: any) { importMsg.value = { ok: false, txt: '导入失败: '+e.message } }
}
function addHistory(isAuto: boolean) {
  configHistory.value.unshift({ time: new Date().toLocaleTimeString('zh-CN'), size: config.value.length, snapshot: config.value.substring(0, 100), isAuto })
}
function restoreHistory(idx: number) {
  const h = configHistory.value[idx]
  if (h) { try { config.value = JSON.stringify(JSON.parse(h.snapshot), null, 2) } catch { config.value = h.snapshot } }
}
function onConfigChange() { /* auto-save debounce could go here */ }
function fmtTime(t?: string) { if (!t) return ''; try { return new Date(t).toLocaleString('zh-CN',{month:'2-digit',day:'2-digit',hour:'2-digit',minute:'2-digit'}) } catch { return String(t) } }
onMounted(() => { qc.invalidateQueries({ queryKey: ['config','list'] }) })

const api_input_pr_78_data = ref<any[]>([]);
const { data: api_input_pr_78_q } = useQuery({queryKey: ['api_input_pr_78', '/jaxrs/input/prepare/create'], queryFn: async () => { try { const r = await api.get("/jaxrs/input/prepare/create"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_input_pr_78_q, (v) => { api_input_pr_78_data.value = v ?? []; });
const input_compare_mockputtopost_ref = ref<any[]>([]);
const input_compare_mockputtopost_q = useQuery({
  queryKey: ['input_compare_mockputtopost'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/input/compare/mockputtopost"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const input_prepare_cover_ref = ref<any[]>([]);
const input_prepare_cover_q = useQuery({
  queryKey: ['input_prepare_cover'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/input/prepare/cover"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const input_create_mockputtopost_ref = ref<any[]>([]);
const input_create_mockputtopost_q = useQuery({
  queryKey: ['input_create_mockputtopost'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/input/create/mockputtopost"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const api_input_compare_data = ref<any[]>([]);
const { data: api_input_compare_q } = useQuery({queryKey: ['api_input_compare', '/jaxrs/input/compare'], queryFn: async () => { try { const r = await api.get("/jaxrs/input/compare"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_input_compare_q, (v) => { api_input_compare_data.value = v ?? []; });
const input_cover_ref = ref<any[]>([]);
const input_cover_q = useQuery({
  queryKey: ['input_cover'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/input/cover"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const input_prepare_cover_mockputtopost_ref = ref<any[]>([]);
const input_prepare_cover_mockputtopost_q = useQuery({
  queryKey: ['input_prepare_cover_mockputtopost'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/input/prepare/cover/mockputtopost"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const input_prepare_create_mockputtopost_ref = ref<any[]>([]);
const input_prepare_create_mockputtopost_q = useQuery({
  queryKey: ['input_prepare_create_mockputtopost'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/input/prepare/create/mockputtopost"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const api_input_create_data = ref<any[]>([]);
const { data: api_input_create_q } = useQuery({queryKey: ['api_input_create', '/jaxrs/input/create'], queryFn: async () => { try { const r = await api.get("/jaxrs/input/create"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_input_create_q, (v) => { api_input_create_data.value = v ?? []; });
const input_cover_mockputtopost_ref = ref<any[]>([]);
const input_cover_mockputtopost_q = useQuery({
  queryKey: ['input_cover_mockputtopost'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/input/cover/mockputtopost"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});


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


const api_config_data = ref<any[]>([]);
const { data: api_config_q } = useQuery({queryKey: ['api_config', '/jaxrs/config'], queryFn: async () => { try { const r = await api.get("/jaxrs/config"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_config_q, (v) => { api_config_data.value = v ?? []; });
const api_config_s_497_data = ref<any[]>([]);
const { data: api_config_s_497_q } = useQuery({queryKey: ['api_config_s_497', '/jaxrs/config/system/config'], queryFn: async () => { try { const r = await api.get("/jaxrs/config/system/config"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_config_s_497_q, (v) => { api_config_s_497_data.value = v ?? []; });
const api_config_i_771_data = ref<any[]>([]);
const { data: api_config_i_771_q } = useQuery({queryKey: ['api_config_i_771', '/jaxrs/config/is/file/manager'], queryFn: async () => { try { const r = await api.get("/jaxrs/config/is/file/manager"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_config_i_771_q, (v) => { api_config_i_771_data.value = v ?? []; });

</script>

<style scoped>
.editor-view{display:flex;flex-direction:column;gap:0;height:100%}
.view-header{display:flex;align-items:center;justify-content:space-between;padding:12px 20px;flex-shrink:0}
.view-header h1{font-family:'Orbitron',sans-serif;font-size:18px;color:var(--color-primary);margin:0 0 2px;text-shadow:0 0 15px var(--color-primary-glow)}
.subtitle{font-size:11px;color:var(--text-muted);margin:0;font-family:'JetBrains Mono',monospace}
.header-actions{display:flex;gap:6px;flex-wrap:wrap}
.btn-outline{padding:5px 12px;border-radius:var(--radius-md);border:1px solid var(--border-color);background:transparent;color:var(--text-primary);cursor:pointer;font-size:12px}
.btn-outline:hover{border-color:var(--color-primary);color:var(--color-primary)}
.btn-secondary{padding:5px 12px;border-radius:var(--radius-md);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);cursor:pointer;font-size:12px}
.btn-primary{padding:5px 12px;border-radius:var(--radius-md);border:none;background:var(--color-primary);color:#000;cursor:pointer;font-size:12px;font-weight:600}
.btn-sm{padding:4px 10px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);cursor:pointer;font-size:11px}
.btn-sm:hover{border-color:var(--color-primary);color:var(--color-primary)}
.btn-danger{border-color:var(--color-danger);color:var(--color-danger)}
.btn-del{border-color:var(--color-danger);color:var(--color-danger)}
.toolbar{display:flex;align-items:center;justify-content:space-between;padding:8px 16px;border-bottom:1px solid var(--border-color);flex-shrink:0;flex-wrap:wrap;gap:8px}
.toolbar-left,.toolbar-right{display:flex;align-items:center;gap:8px}
.tb-select{padding:4px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:11px;outline:none}
.tb-info{font-size:11px;color:var(--text-muted)}
.editor-layout{display:flex;flex:1;min-height:0;overflow:hidden}
.editor-sidebar{width:260px;flex-shrink:0;display:flex;flex-direction:column;border-right:1px solid var(--border-color);overflow:hidden}
.sb-search{padding:8px}
.sb-input{width:100%;padding:5px 10px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:12px;outline:none;box-sizing:border-box}
.sb-tabs{display:flex;gap:4px;padding:4px 8px;border-bottom:1px solid var(--border-color)}
.sb-tabs button{flex:1;padding:4px;font-size:11px;border-radius:var(--radius-sm);border:1px solid transparent;background:transparent;color:var(--text-muted);cursor:pointer}
.sb-tabs button.active{border-color:var(--color-primary);color:var(--color-primary);background:var(--color-primary-soft)}
.sb-list{flex:1;overflow-y:auto;padding:4px}
.sb-item{display:flex;align-items:center;gap:8px;padding:8px;border-radius:var(--radius-sm);cursor:pointer;margin-bottom:2px}
.sb-item:hover{background:var(--bg-hover)}
.sb-item.active{background:var(--color-primary-soft);border-left:3px solid var(--color-primary)}
.si-icon{font-size:16px;flex-shrink:0}
.si-info{flex:1;min-width:0}
.si-name{font-size:13px;color:var(--text-primary);overflow:hidden;text-overflow:ellipsis;white-space:nowrap}
.si-meta{font-size:10px;color:var(--text-muted);font-family:monospace}
.si-actions{display:flex;gap:2px;flex-shrink:0}
.si-btn{padding:2px 5px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:transparent;color:var(--text-muted);cursor:pointer;font-size:10px}
.si-btn:hover{border-color:var(--color-primary);color:var(--color-primary)}
.si-del:hover{border-color:var(--color-danger);color:var(--color-danger)}
.sb-add{margin:8px;padding:6px;border-radius:var(--radius-sm);border:1px dashed var(--border-color);background:transparent;color:var(--text-muted);cursor:pointer;font-size:12px;width:calc(100% - 16px)}
.sb-add:hover{border-color:var(--color-primary);color:var(--color-primary)}
.editor-main{flex:1;display:flex;flex-direction:column;min-width:0;overflow:hidden}
.empty-main{flex:1;display:flex;flex-direction:column;align-items:center;justify-content:center;color:var(--text-muted);gap:12px}
.emi{font-size:32px;opacity:0.4}
.editor-content{flex:1;display:flex;flex-direction:column;gap:8px;padding:12px;overflow:hidden}
.ec-header{display:flex;align-items:center;justify-content:space-between;padding-bottom:8px;border-bottom:1px solid var(--border-color)}
.ec-title{font-size:14px;font-weight:600;color:var(--text-primary)}
.ec-meta{font-size:11px;color:var(--text-muted);font-family:monospace}
.ec-breadcrumb{display:flex;align-items:center;gap:6px;padding:6px 10px;background:rgba(245,158,11,0.05);border:1px solid rgba(245,158,11,0.2);border-radius:var(--radius-sm);font-size:11px}
.bc-label{color:var(--text-muted)}
.bc-path{color:#f59e0b;font-family:monospace}
.code-editor{flex:1;min-height:200px;padding:12px;border-radius:var(--radius-md);border:1px solid var(--border-color);background:var(--bg-terminal);color:#7fdbca;font-family:'Fira Code','JetBrains Mono',monospace;font-size:13px;outline:none;resize:none;line-height:1.6;tab-size:2}
.code-editor:focus{border-color:var(--color-primary)}
.ec-footer{display:flex;align-items:center;justify-content:space-between;padding:6px 0;border-top:1px solid var(--border-color);margin-top:4px}
.ec-status{font-size:11px;color:var(--text-muted)}
.ec-actions{display:flex;gap:6px}
.empty{padding:12px;color:var(--text-muted);text-align:center;font-size:12px}
.loading-sm{padding:12px;color:var(--text-muted);font-size:12px}
/* Modals */
.modal-overlay{position:fixed;inset:0;background:rgba(0,0,0,0.6);display:flex;align-items:center;justify-content:center;z-index:200}
.modal{padding:20px;width:560px;max-width:90vw;max-height:85vh;display:flex;flex-direction:column;gap:12px;overflow:hidden}
.modal h3{font-size:15px;color:var(--color-primary);margin:0;font-family:'Orbitron',sans-serif}
.modal-actions{display:flex;justify-content:flex-end;gap:8px;margin-top:auto;padding-top:8px;border-top:1px solid var(--border-color)}
.btn-cancel{padding:6px 14px;border-radius:var(--radius-md);border:1px solid var(--border-color);background:transparent;color:var(--text-primary);cursor:pointer;font-size:12px}
.btn-save{padding:6px 14px;border-radius:var(--radius-md);border:none;background:var(--color-primary);color:#000;cursor:pointer;font-size:12px;font-weight:600}
.fmt-out{margin:0;padding:10px;background:rgba(0,0,0,0.3);color:#10b981;font-size:12px;font-family:monospace;border-radius:var(--radius-sm);white-space:pre-wrap;word-break:break-all;max-height:300px;overflow-y:auto}
.history-list{flex:1;overflow-y:auto;display:flex;flex-direction:column;gap:6px}
.hist-item{background:rgba(255,255,255,0.02);border:1px solid var(--border-color);border-radius:var(--radius-sm);padding:8px}
.hist-meta{display:flex;align-items:center;gap:8px;font-size:10px;margin-bottom:4px}
.hist-time{color:var(--text-muted);font-family:monospace}
.hist-size{color:var(--text-muted)}
.hist-tag{padding:1px 6px;border-radius:10px;font-size:9px}
.hist-tag.auto{background:rgba(59,130,246,0.15);color:var(--color-primary)}
.hist-tag.manual{background:rgba(245,158,11,0.15);color:#f59e0b}
.hist-preview{margin:0;padding:4px 8px;background:rgba(0,0,0,0.3);color:#7fdbca;font-size:10px;font-family:monospace;border-radius:4px;max-height:40px;overflow-y:auto;white-space:pre-wrap}
.hist-actions{display:flex;gap:4px;margin-top:4px}
.hist-empty{color:var(--text-muted);font-size:12px;text-align:center;padding:24px}
.hist-footer{display:flex;gap:6px;padding-top:8px;border-top:1px solid var(--border-color)}
.ie-body{padding:12px;display:flex;flex-direction:column;gap:12px}
.ie-section{display:flex;flex-direction:column;gap:6px}
.ie-title{font-size:12px;font-weight:600;color:var(--color-primary)}
.ie-textarea{width:100%;height:100px;background:rgba(0,0,0,0.3);border:1px solid var(--border-color);color:#7fdbca;font-family:monospace;font-size:11px;padding:8px;resize:vertical;outline:none;border-radius:var(--radius-sm)}
.ie-msg{padding:6px 10px;border-radius:var(--radius-sm);font-size:11px}
.ie-msg.ok{background:rgba(16,185,129,0.1);color:#10b981}
.ie-msg.err{background:rgba(239,68,68,0.1);color:#ef4444}
.ie-footer{display:flex;justify-content:flex-end;padding-top:8px;border-top:1px solid var(--border-color)}
/* Scrollbar */
.sb-list::-webkit-scrollbar,.history-list::-webkit-scrollbar{width:4px}
.sb-list::-webkit-scrollbar-thumb,.history-list::-webkit-scrollbar-thumb{background:var(--border-color);border-radius:2px}
</style>
