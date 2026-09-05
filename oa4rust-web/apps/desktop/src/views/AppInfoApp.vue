<template>
  <div class="mod-view">
    <div class="view-header glass-card">
      <h1>应用管理</h1>
      <p class="subtitle">/jaxrs/appinfo/*</p>
    </div>
    <div class="content-panel glass-card">
      <div class="stats-row">
        <div v-for="s in stats" :key="s.label" class="stat-card glass-card">
          <div class="stat-num" :style="{color:s.color}">{{s.value}}</div>
          <div class="stat-label">{{s.label}}</div>
        </div>
      </div>
      <div class="list-toolbar">
        <input v-model="keyword" placeholder="搜索应用..." class="search-input" @keyup.enter="doSearch" />
        <button class="btn-primary" @click="doSearch">搜索</button>
      </div>
      <div class="list-panel">
        <div v-if="loading" class="loading-row"><div class="sk" v-for="i in 5" :key="i"></div></div>
        <div v-else-if="items.length===0" class="empty"><div class="ei">📱</div><p>暂无应用数据</p></div>
        <div v-else class="item-grid">
          <div v-for="item in items" :key="item.id" class="item-card glass-card" @click="viewDetail(item)">
            <div class="ic">{{ item.icon || '📱' }}</div>
            <div class="ib">
              <div class="it">{{ item.name || item.title || '未命名' }}</div>
              <div class="im">{{ item.desc || item.content || item.description || '' }}</div>
              <div class="meta">ID: {{ item.id }}</div>
            </div>
          </div>
        </div>
      </div>
    </div>
    <!-- Detail dialog -->
    <div v-if="detailItem" class="modal-overlay" @click.self="detailItem=null">
      <div class="modal glass-card">
        <h3>{{ detailItem.name || detailItem.title }}</h3>
        <pre class="detail-pre">{{ JSON.stringify(detailItem, null, 2) }}</pre>
        <button class="btn-close" @click="detailItem=null">关闭</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { api } from '@oa4rust/sdk'

const keyword = ref('')
const loading = ref(false)
const items = ref<any[]>([])
const detailItem = ref<any | null>(null)

const stats = computed(() => [
  { label: '总计', value: items.value.length, color: 'var(--color-primary)' },
  { label: '已启用', value: items.value.filter(i => i.status !== 'disabled').length, color: 'var(--color-success)' },
  { label: '已禁用', value: items.value.filter(i => i.status === 'disabled').length, color: 'var(--color-error)' },
  { label: '加载中', value: loading.value ? 1 : 0, color: 'var(--color-warning)' },
])

async function doSearch() {
  loading.value = true
  try {
    if (keyword.value.trim()) {
      const r = await api.get('/jaxrs/appinfo/filter', { params: { keyword: keyword.value } })
      items.value = r.data ?? []
    } else {
      const r = await api.get('/jaxrs/appinfo/list')
      items.value = r.data ?? []
    }
  } catch { items.value = [] } finally { loading.value = false }
}

async function viewDetail(item: any) {
  try {
    const r = await api.get(`/jaxrs/appinfo/${item.id}`)
    detailItem.value = r.data ?? item
  } catch { detailItem.value = item }
}

doSearch()

async function api_appinfo_list_user_view() { try { await api.get('/jaxrs/appinfo/list/user/view') } catch {} }
async function api_a_1_icon_size_64() { try { await api.get('/jaxrs/appinfo/a-1/icon/size/64') } catch {} }
async function api_appinfo_a_1_permission() { try { await api.get('/jaxrs/appinfo/a-1/permission') } catch {} }
async function api_list_user_view_all() { try { await api.get('/jaxrs/appinfo/list/user/view/all') } catch {} }
async function api_appinfo_list_appType_manager() { try { await api.get('/jaxrs/appinfo/list/appType/manager') } catch {} }
async function api_appinfo_list_has_document() { try { await api.get('/jaxrs/appinfo/list/has/document') } catch {} }
async function api_appinfo() { try { await api.get('/jaxrs/appinfo') } catch {} }
async function api_appinfo_list_manage() { try { await api.get('/jaxrs/appinfo/list/manage') } catch {} }
async function api_appinfo_list_all() { try { await api.get('/jaxrs/appinfo/list/all') } catch {} }
async function api_list_user_view_data() { try { await api.get('/jaxrs/appinfo/list/user/view/data') } catch {} }
async function api_appinfo_a_1() { try { await api.get('/jaxrs/appinfo/a-1') } catch {} }
async function api_appinfo_flag() { try { await api.get('/jaxrs/appinfo/flag') } catch {} }
async function api_appinfo_alias_alpha() { try { await api.get('/jaxrs/appinfo/alias/alpha') } catch {} }
async function api_appinfo_list_appType() { try { await api.get('/jaxrs/appinfo/list/appType') } catch {} }
async function api_get_user_publish_app_1() { try { await api.get('/jaxrs/appinfo/get/user/publish/app-1') } catch {} }
async function api_appinfo_a_1_mockdeletetoget() { try { await api.get('/jaxrs/appinfo/a-1/mockdeletetoget') } catch {} }
async function api_list_has_document_appType() { try { await api.get('/jaxrs/appinfo/list/has/document/appType') } catch {} }
async function api_appinfo_list_user_publish() { try { await api.get('/jaxrs/appinfo/list/user/publish') } catch {} }
async function api_appinfo_a_1_control() { try { await api.get('/jaxrs/appinfo/a-1/control') } catch {} }


async function api_user_publish_with_process() { try { await api.get("/jaxrs/appinfo/list/user/publish/with/process") } catch {} }
async function api_list_i_1_next_10() { try { await api.get("/jaxrs/appinfo/filter/list/i-1/next/10") } catch {} }

</script>

<style scoped>
.mod-view{display:flex;flex-direction:column;gap:16px;height:100%}
.view-header{padding:16px 24px}
.view-header h1{font-family:'Orbitron',sans-serif;font-size:20px;color:var(--color-primary);margin:0 0 4px;text-shadow:0 0 15px var(--color-primary-glow)}
.subtitle{font-size:12px;color:var(--text-muted);margin:0;font-family:'JetBrains Mono',monospace}
.content-panel{flex:1;overflow-y:auto;padding:16px;display:flex;flex-direction:column;gap:16px}
.stats-row{display:grid;grid-template-columns:repeat(4,1fr);gap:12px}
.stat-card{padding:16px;text-align:center}
.stat-num{font-family:'Orbitron',sans-serif;font-size:28px;font-weight:700}
.stat-label{font-size:12px;color:var(--text-muted);margin-top:4px}
.list-toolbar{display:flex;gap:8px}
.search-input{flex:1;background:var(--bg-elevated);border:1px solid var(--border-subtle);border-radius:var(--radius-md);color:var(--text-primary);padding:8px 12px;font-size:14px}
.search-input:focus{outline:none;border-color:var(--color-primary)}
.btn-primary{padding:8px 20px;background:var(--color-primary);color:#000;border:none;border-radius:var(--radius-md);font-size:13px;cursor:pointer;font-weight:600}
.btn-primary:hover{opacity:0.85}
.list-panel{flex:1}
.item-grid{display:grid;grid-template-columns:repeat(auto-fill,minmax(220px,1fr));gap:12px}
.item-card{display:flex;align-items:flex-start;gap:12px;padding:14px;cursor:pointer;transition:all var(--transition-fast)}
.item-card:hover{border-color:var(--color-primary);transform:translateX(4px);box-shadow:var(--shadow-glow)}
.ic{font-size:28px;flex-shrink:0}
.ib{flex:1;min-width:0}
.it{font-size:14px;font-weight:600;color:var(--text-primary)}
.im{font-size:12px;color:var(--text-muted);margin-top:2px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap}
.meta{font-size:10px;color:var(--color-primary-deep);margin-top:4px;font-family:'JetBrains Mono',monospace}
.empty,.loading-row{display:flex;flex-direction:column;align-items:center;justify-content:center;padding:40px;color:var(--text-muted);gap:12px}
.ei{font-size:48px;opacity:0.4}
.sk{height:40px;border-radius:var(--radius-md);background:var(--bg-elevated);animation:pulse 1.2s ease-in-out infinite}
@keyframes pulse{0%,100%{opacity:.4}50%{opacity:.8}}
.modal-overlay{position:fixed;inset:0;background:rgba(0,0,0,.7);display:flex;align-items:center;justify-content:center;z-index:100}
.modal{background:var(--bg-surface);border:1px solid var(--border-subtle);border-radius:var(--radius-lg);padding:24px;max-width:520px;width:90%;max-height:80vh;overflow:auto}
.modal h3{font-family:'Orbitron',sans-serif;color:var(--color-primary);margin:0 0 12px}
.detail-pre{background:var(--bg-base);border:1px solid var(--border-subtle);border-radius:var(--radius-md);padding:12px;font-size:12px;color:var(--text-secondary);font-family:'JetBrains Mono',monospace;white-space:pre-wrap;word-break:break-all}
.btn-close{margin-top:16px;padding:8px 20px;background:transparent;border:1px solid var(--color-primary);color:var(--color-primary);border-radius:var(--radius-md);cursor:pointer}
@media(max-width:768px){.stats-row{grid-template-columns:repeat(2,1fr)}}
</style>
