<template>
  <div class="mod-view">
    <div class="view-header glass-card">
      <h1>表单管理</h1>
      <p class="subtitle">/jaxrs/form/*</p>
    </div>
    <div class="content-panel glass-card">
      <div class="tabs">
        <button :class="{active:tab==='list'}" @click="tab='list'">表单列表</button>
        <button :class="{active:tab==='v2'}" @click="tab='v2'">表单V2</button>
      </div>
      <div v-if="tab==='list'" class="tab-content">
        <div v-if="loading" class="loading-row"><div class="sk" v-for="i in 5" :key="i"></div></div>
        <div v-else-if="items.length===0" class="empty"><div class="ei">📝</div><p>暂无表单</p></div>
        <div v-else class="item-grid">
          <div v-for="f in items" :key="f.id" class="item-card glass-card" @click="viewDetail(f)">
            <div class="ic">📝</div>
            <div class="ib">
              <div class="it">{{ f.name || f.title || '未命名表单' }}</div>
              <div class="im">flag: {{ f.flag || f.formFlag }}</div>
            </div>
          </div>
        </div>
      </div>
      <div v-if="tab==='v2'" class="tab-content">
        <div v-if="loadingV2" class="loading-row"><div class="sk" v-for="i in 5" :key="i"></div></div>
        <div v-else-if="itemsV2.length===0" class="empty"><div class="ei">📋</div><p>暂无V2表单</p></div>
        <div v-else class="item-grid">
          <div v-for="f in itemsV2" :key="f.id" class="item-card glass-card" @click="viewDetailV2(f)">
            <div class="ic">📋</div>
            <div class="ib">
              <div class="it">{{ f.name || f.title || '未命名' }}</div>
              <div class="im">version: {{ f.version || '1.0' }}</div>
            </div>
          </div>
        </div>
      </div>
    </div>
    <!-- Detail modal -->
    <div v-if="detailItem" class="modal-overlay" @click.self="detailItem=null">
      <div class="modal glass-card">
        <h3>{{ detailItem.name || detailItem.title }}</h3>
        <pre class="detail-pre">{{ JSON.stringify(detailItem, null, 2) }}</pre>
        <button class="btn-close-modal" @click="detailItem=null">关闭</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { api } from '@oa4rust/sdk'

type Tab = 'list' | 'v2'
type FormItem = { id: string; name?: string; title?: string; flag?: string; formFlag?: string; version?: string }

const tab = ref<Tab>('list')
const loading = ref(false)
const loadingV2 = ref(false)
const items = ref<FormItem[]>([])
const itemsV2 = ref<FormItem[]>([])
const detailItem = ref<FormItem | null>(null)

async function loadList() {
  loading.value = true
  try {
    const r = await api.get('/jaxrs/form/list')
    items.value = r.data ?? []
  } catch { items.value = [] } finally { loading.value = false }
}

async function loadV2() {
  loadingV2.value = true
  try {
    const r = await api.get('/jaxrs/form/v2/list')
    itemsV2.value = r.data ?? []
  } catch { itemsV2.value = [] } finally { loadingV2.value = false }
}

function switchTab(t: Tab) { tab.value = t; if (t === 'list') loadList(); else loadV2() }
function viewDetail(f: FormItem) { api.get(`/jaxrs/form/${f.id}`).then(r => { detailItem.value = r.data ?? f }).catch(() => { detailItem.value = f }) }
function viewDetailV2(f: FormItem) { detailItem.value = f }

loadList()
</script>

<style scoped>
.mod-view{display:flex;flex-direction:column;gap:16px;height:100%}
.view-header{padding:16px 24px}
.view-header h1{font-family:'Orbitron',sans-serif;font-size:20px;color:var(--color-primary);margin:0 0 4px;text-shadow:0 0 15px var(--color-primary-glow)}
.subtitle{font-size:12px;color:var(--text-muted);margin:0;font-family:'JetBrains Mono',monospace}
.content-panel{flex:1;overflow-y:auto;padding:16px;display:flex;flex-direction:column;gap:16px}
.tabs{display:flex;gap:8px}
.tabs button{padding:8px 20px;background:var(--bg-elevated);border:1px solid var(--border-subtle);border-radius:var(--radius-md);color:var(--text-secondary);font-size:13px;cursor:pointer;transition:all var(--transition-fast)}
.tabs button.active{background:var(--color-primary);color:#000;border-color:var(--color-primary);font-weight:600}
.tab-content{flex:1}
.item-grid{display:grid;grid-template-columns:repeat(auto-fill,minmax(200px,1fr));gap:12px}
.item-card{display:flex;align-items:center;gap:12px;padding:14px;cursor:pointer;transition:all var(--transition-fast);border:1px solid var(--border-subtle);border-radius:var(--radius-md);background:var(--bg-elevated)}
.item-card:hover{border-color:var(--color-primary);transform:translateX(4px);box-shadow:var(--shadow-glow)}
.ic{font-size:28px}
.ib{flex:1;min-width:0}
.it{font-size:14px;font-weight:600;color:var(--text-primary)}
.im{font-size:12px;color:var(--text-muted);margin-top:2px}
.empty,.loading-row{display:flex;flex-direction:column;align-items:center;justify-content:center;padding:40px;color:var(--text-muted);gap:12px;flex:1}
.ei{font-size:48px;opacity:0.4}
.sk{height:40px;border-radius:var(--radius-md);background:var(--bg-elevated);animation:pulse 1.2s ease-in-out infinite}
@keyframes pulse{0%,100%{opacity:.4}50%{opacity:.8}}
.modal-overlay{position:fixed;inset:0;background:rgba(0,0,0,.7);display:flex;align-items:center;justify-content:center;z-index:100}
.modal{background:var(--bg-surface);border:1px solid var(--border-subtle);border-radius:var(--radius-lg);padding:24px;width:500px;max-width:90vw;max-height:80vh;display:flex;flex-direction:column;gap:12px;overflow:hidden}
.modal h3{font-family:'Orbitron',sans-serif;color:var(--color-primary);margin:0;font-size:15px}
.detail-pre{flex:1;overflow:auto;background:var(--bg-base);border:1px solid var(--border-subtle);border-radius:var(--radius-md);padding:12px;font-size:12px;color:var(--text-secondary);font-family:'JetBrains Mono',monospace;white-space:pre-wrap}
.btn-close-modal{padding:8px 20px;background:transparent;border:1px solid var(--color-primary);color:var(--color-primary);border-radius:var(--radius-md);cursor:pointer}
</style>
