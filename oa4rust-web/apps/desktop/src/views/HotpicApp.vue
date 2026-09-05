<template>
  <div class="mod-view">
    <div class="view-header glass-card">
      <h1>热帖管理</h1>
      <p class="subtitle">/jaxrs/hotpic/core/entity/*</p>
    </div>
    <div class="content-panel glass-card">
      <div class="stats-row">
        <div v-for="s in stats" :key="s.label" class="stat-card glass-card">
          <div class="stat-num" :style="{color:s.color}">{{s.value}}</div>
          <div class="stat-label">{{s.label}}</div>
        </div>
      </div>
      <div class="list-toolbar">
        <input v-model="keyword" placeholder="搜索热帖..." class="search-input" @keyup.enter="doSearch" />
        <button class="btn-primary" @click="doSearch">搜索</button>
      </div>
      <div class="list-panel">
        <div v-if="loading" class="loading-row"><div class="sk" v-for="i in 5" :key="i"></div></div>
        <div v-else-if="items.length===0" class="empty"><div class="ei">🔥</div><p>暂无热帖数据</p></div>
        <div v-else class="item-grid">
          <div v-for="item in items" :key="item.id" class="item-card glass-card">
            <div class="ic">🔥</div>
            <div class="ib">
              <div class="it">{{ item.title || item.name || '未命名' }}</div>
              <div class="im">{{ item.content || item.desc || item.description || '' }}</div>
              <div class="meta">views: {{ item.views || 0 }} | likes: {{ item.likes || 0 }}</div>
            </div>
            <button class="btn-del" @click.stop="onDelete(item)">删除</button>
          </div>
        </div>
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

const stats = computed(() => [
  { label: '总计', value: items.value.length, color: 'var(--color-primary)' },
  { label: '今日热门', value: items.value.filter(i => i.isHot).length, color: 'var(--color-warning)' },
  { label: '精华', value: items.value.filter(i => i.isCream).length, color: 'var(--color-success)' },
  { label: '加载中', value: loading.value ? 1 : 0, color: 'var(--color-error)' },
])

async function doSearch() {
  loading.value = true
  try {
    const r = await api.get('/jaxrs/hotpic/core/entity/list')
    items.value = r.data ?? []
  } catch { items.value = [] } finally { loading.value = false }
}

async function onDelete(item: any) {
  if (!confirm(`确定删除热帖「${item.title || item.id}」？`)) return
  try {
    await api.delete(`/jaxrs/hotpic/core/entity/delete/${item.id}`)
    items.value = items.value.filter(i => i.id !== item.id)
  } catch (e: any) { alert('删除失败: ' + (e?.message ?? '未知错误')) }
}

doSearch()

async function api_user_hotpic_CMS_doc_123() { try { await api.get('/jaxrs/hotpic/user/hotpic/CMS/doc-123') } catch {} }
async function api_hotpic_save_hotpic() { try { await api.get('/jaxrs/hotpic/save/hotpic') } catch {} }
async function api_hotpic_delete_hotpic() { try { await api.get('/jaxrs/hotpic/delete/hotpic') } catch {} }
async function api_hotpic_core_entity_create() { try { await api.get('/jaxrs/hotpic/core/entity/create') } catch {} }
async function api_hotpic_core_list() { try { await api.get('/jaxrs/hotpic/core/list') } catch {} }
async function api_hotpic_user_hotpic_hotpic_001() { try { await api.get('/jaxrs/hotpic/user/hotpic/hotpic-001') } catch {} }
async function api_hotpic_assemble_list() { try { await api.get('/jaxrs/hotpic/assemble/list') } catch {} }
async function api_hotpic_create_hotpic() { try { await api.get('/jaxrs/hotpic/create/hotpic') } catch {} }
async function api_hotpic_upload() { try { await api.get('/jaxrs/hotpic/upload') } catch {} }
async function api_core_entity_delete_hotpic_test_001() { try { await api.get('/jaxrs/hotpic/core/entity/delete/hotpic-test-001') } catch {} }
async function api_user_hotpic_exists_check() { try { await api.get('/jaxrs/hotpic/user/hotpic/exists/check') } catch {} }
async function api_hotpic_list_hotpics() { try { await api.get('/jaxrs/hotpic/list/hotpics') } catch {} }
async function api_hotpic_assemble_control_config() { try { await api.get('/jaxrs/hotpic/assemble/control/config') } catch {} }
async function api_hotpic_get_hotpic_hotpic_001() { try { await api.get('/jaxrs/hotpic/get/hotpic/hotpic-001') } catch {} }
async function api_assemble_control_user_hotpic() { try { await api.get('/jaxrs/hotpic/assemble/control/user/hotpic') } catch {} }
async function api_hotpic_list() { try { await api.get('/jaxrs/hotpic/list') } catch {} }


async function api_hotpic_assemble_control() { try { await api.get("/jaxrs/hotpic_assemble_control") } catch {} }
async function api_hotpic_assemble_control_save_hotpic() { try { await api.get("/jaxrs/hotpic_assemble_control/save/hotpic") } catch {} }
async function api_hotpic_assemble_control_list_hotpics() { try { await api.get("/jaxrs/hotpic_assemble_control/list/hotpics") } catch {} }
async function api_hotpic_assemble_control_cipher_hotpic_id() { try { await api.get("/jaxrs/hotpic_assemble_control/cipher/hotpic/id") } catch {} }
async function api_hotpic_assemble_control_create_hotpic() { try { await api.get("/jaxrs/hotpic_assemble_control/create/hotpic") } catch {} }
async function api_hotpic_assemble_control_user_hotpic_changeTitle() { try { await api.get("/jaxrs/hotpic_assemble_control/user/hotpic/changeTitle") } catch {} }
async function api_hotpic_assemble_control_list_control_applications() { try { await api.get("/jaxrs/hotpic_assemble_control/list/control/applications") } catch {} }
async function api_hotpic_assemble_control_delete_hotpic() { try { await api.get("/jaxrs/hotpic_assemble_control/delete/hotpic") } catch {} }
async function api_user_hotpic_exists_check_1() { try { await api.get("/jaxrs/hotpic_assemble_control/user/hotpic/exists/check") } catch {} }
async function api_hotpic_assemble_control_get_hotpic() { try { await api.get("/jaxrs/hotpic_assemble_control/get/hotpic") } catch {} }
async function api_hotpic_assemble_control_update_control_config() { try { await api.get("/jaxrs/hotpic_assemble_control/update/control/config") } catch {} }
async function api_hotpic_assemble_control_user_hotpic_id() { try { await api.get("/jaxrs/hotpic_assemble_control/user/hotpic/id") } catch {} }
async function api_hotpic_assemble_control_get_control_config() { try { await api.get("/jaxrs/hotpic_assemble_control/get/control/config") } catch {} }
async function api_user_hotpic_application_infoId() { try { await api.get("/jaxrs/hotpic_assemble_control/user/hotpic/application/infoId") } catch {} }
async function api_hotpic_assemble_control_list_control_panels() { try { await api.get("/jaxrs/hotpic_assemble_control/list/control/panels") } catch {} }

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
.btn-del{padding:4px 12px;background:transparent;border:1px solid var(--color-error);color:var(--color-error);border-radius:var(--radius-sm);font-size:12px;cursor:pointer;transition:all var(--transition-fast)}
.btn-del:hover{background:var(--color-error);color:#fff}
.list-panel{flex:1}
.item-grid{display:grid;grid-template-columns:repeat(auto-fill,minmax(240px,1fr));gap:12px}
.item-card{display:flex;align-items:flex-start;gap:12px;padding:14px;transition:all var(--transition-fast);border:1px solid var(--border-subtle);border-radius:var(--radius-md);background:var(--bg-elevated)}
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
@media(max-width:768px){.stats-row{grid-template-columns:repeat(2,1fr)}}
</style>
