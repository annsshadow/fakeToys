<template>
  <div class="mod-view">
    <div class="view-header glass-card">
      <h1>分类管理</h1>
      <p class="subtitle">/jaxrs/categoryinfo/*</p>
    </div>
    <div class="content-panel glass-card">
      <div class="stats-row">
        <div v-for="s in stats" :key="s.label" class="stat-card glass-card">
          <div class="stat-num" :style="{color:s.color}">{{s.value}}</div>
          <div class="stat-label">{{s.label}}</div>
        </div>
      </div>
      <div class="list-panel">
        <div v-if="loading" class="loading-row"><div class="sk" v-for="i in 6" :key="i"></div></div>
        <div v-else-if="items.length===0" class="empty"><div class="ei">📂</div><p>暂无分类数据</p></div>
        <div v-else class="item-grid">
          <div v-for="item in items" :key="item.id" class="item-card glass-card">
            <div class="ic">📁</div>
            <div class="ib">
              <div class="it">{{ item.name || item.title || item.categoryName || '未命名' }}</div>
              <div class="im">{{ item.desc || item.description || item.alias || '' }}</div>
              <div class="meta">flag: {{ item.flag || item.id }}</div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { api } from '@oa4rust/sdk'

const loading = ref(false)
const items = ref<any[]>([])

const stats = computed(() => [
  { label: '总计', value: items.value.length, color: 'var(--color-primary)' },
  { label: '有效', value: items.value.length, color: 'var(--color-success)' },
  { label: '禁用', value: 0, color: 'var(--color-error)' },
  { label: '加载中', value: loading.value ? 1 : 0, color: 'var(--color-warning)' },
])

async function load() {
  loading.value = true
  try {
    const r = await api.get('/jaxrs/categoryinfo/list')
    items.value = r.data ?? []
  } catch { items.value = [] } finally { loading.value = false }
}

load()

const categoryinfo_ref = ref<any[]>([]);
const categoryinfo_q = useQuery({
  queryKey: ['categoryinfo'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/categoryinfo"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const categoryinfo_alias_alpha_ref = ref<any[]>([]);
const categoryinfo_alias_alpha_q = useQuery({
  queryKey: ['categoryinfo_alias_alpha'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/categoryinfo/alias/alpha"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const categoryinfo_list_objects_ref = ref<any[]>([]);
const categoryinfo_list_objects_q = useQuery({
  queryKey: ['categoryinfo_list_objects'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/categoryinfo/list/objects"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const categoryinfo_list_all_ref = ref<any[]>([]);
const categoryinfo_list_all_q = useQuery({
  queryKey: ['categoryinfo_list_all'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/categoryinfo/list/all"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const categoryinfo_extContent_ref = ref<any[]>([]);
const categoryinfo_extContent_q = useQuery({
  queryKey: ['categoryinfo_extContent'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/categoryinfo/extContent"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const categoryinfo_c_1_permission_ref = ref<any[]>([]);
const categoryinfo_c_1_permission_q = useQuery({
  queryKey: ['categoryinfo_c_1_permission'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/categoryinfo/c-1/permission"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const categoryinfo_flag_ref = ref<any[]>([]);
const categoryinfo_flag_q = useQuery({
  queryKey: ['categoryinfo_flag'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/categoryinfo/flag"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const categoryinfo_c_1_control_ref = ref<any[]>([]);
const categoryinfo_c_1_control_q = useQuery({
  queryKey: ['categoryinfo_c_1_control'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/categoryinfo/c-1/control"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const categoryinfo_c_1_execute_projection_ref = ref<any[]>([]);
const categoryinfo_c_1_execute_projection_q = useQuery({
  queryKey: ['categoryinfo_c_1_execute_projection'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/categoryinfo/c-1/execute/projection"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const categoryinfo_c_1_ref = ref<any[]>([]);
const categoryinfo_c_1_q = useQuery({
  queryKey: ['categoryinfo_c_1'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/categoryinfo/c-1"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const categoryinfo_bind_c_1_view_ref = ref<any[]>([]);
const categoryinfo_bind_c_1_view_q = useQuery({
  queryKey: ['categoryinfo_bind_c_1_view'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/categoryinfo/bind/c-1/view"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});


const api_list_p_1_860_data = ref<any[]>([]);
const { data: api_list_p_1_860_q } = useQuery({queryKey: ['api_list_p_1_860', '/jaxrs/categoryinfo/filter/list/p-1/size/10'], queryFn: async () => { try { const r = await api.get("/jaxrs/categoryinfo/filter/list/p-1/size/10"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_list_p_1_860_q, (v) => { api_list_p_1_860_data.value = v ?? []; });

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
.list-panel{flex:1}
.item-grid{display:grid;grid-template-columns:repeat(auto-fill,minmax(220px,1fr));gap:12px}
.item-card{display:flex;align-items:center;gap:12px;padding:14px;transition:all var(--transition-fast);border:1px solid var(--border-subtle);border-radius:var(--radius-md);background:var(--bg-elevated)}
.item-card:hover{border-color:var(--color-primary);transform:translateX(4px);box-shadow:var(--shadow-glow)}
.ic{font-size:28px}
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
