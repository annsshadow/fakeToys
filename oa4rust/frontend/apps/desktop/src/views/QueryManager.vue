<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<template>
  <div class="query-view modern-page ai-page">
    <div class="view-header glass-card ai-page ai-panel">
      <span class="hdr-a">
        <button class="nb ghost" @click="loadQueryMeta">分类/语句</button>
        <button class="nb ghost" @click="loadQueryExtra">模型/导出</button>
        <button class="nb ghost" @click="loadQueryAll">全部查询/概要/分类</button>
        <button class="nb" @click="showCreate=true">+ 新建查询</button>
      </span>
    </div>
    <div v-if="queryMetaText" class="qmeta">{{ queryMetaText }}</div>
    <div class="ql">
      <aside class="qs glass-card">
        <div class="sb2"><span class="si2">⌕</span><input v-model="sq" placeholder="搜索查询..." class="si3" /></div>
        <ListLoadError v-if="qs2.length > 0 && loadError" mode="banner" :message="loadError" @retry="reload" />
        <ListLoadError v-if="qs2.length===0 && loadError" mode="empty" :message="loadError" @retry="reload" />
        <div v-else-if="qs2.length===0" class="es2 ai-empty"><span class="ai-empty-icon">🔎</span><p>暂无查询定义</p><p class="ai-empty-hint">点击顶部「+ 新建查询」创建第一条查询定义</p></div>
        <ul class="ql2">
          <li v-for="q in qs2" :key="q.id" class="qi2" :class="{active:selected?.id===q.id}" @click="selQ(q)">
            <span class="qicon">{{q.icon||'📊'}}</span>
            <div class="qi2i"><div class="qin">{{q.name||q.queryName||'未命名'}}</div><div class="qim">{{q.category||q.entityCategory||'通用'}}</div></div>
            <span class="qit">{{fmtT(q.updateTime)}}</span>
          </li>
        </ul>
      </aside>
      <main class="qm glass-card">
        <template v-if="selected">
          <div class="qh"><h2>{{selected.name||selected.queryName}}</h2><div class="qa2">
            <button class="abtn" @click="runQ">▶ 执行</button>
            <button class="abtn" @click="delQ">🗑 删除</button>
          </div></div>
          <div class="qf"><input v-model="filterText" class="fi2" placeholder="输入筛选条件..." /><button class="rb" @click="runQ">执行</button></div>
          <div class="qr">
            <div v-if="rloading" class="ls2"><div class="sk2" v-for="i in 5" :key="i"></div></div>
            <div v-else-if="rdata.length===0" class="es2"><p>暂无结果，点击执行</p></div>
            <div v-else class="rt2">
              <div class="rth"><span v-for="h in rheaders" :key="h" class="rh2">{{h}}</span></div>
              <div v-for="(row,ri) in rdata" :key="ri" class="tr2"><span v-for="h in rheaders" :key="h" class="rc2">{{row[h]??'—'}}</span></div>
            </div>
          </div>
        </template>
        <div v-else class="em2"><div class="emi2">📊</div><h2>选择查询定义</h2><p>点击左侧查询列表执行查询</p></div>
      </main>
    </div>
    <div v-if="showCreate" class="mo" @click.self="showCreate=false">
      <div class="modal glass-card">
        <h3>新建查询</h3>
        <div class="fg3"><label>名称</label><input v-model="nform.name" class="fi3" placeholder="查询名称" /></div>
        <div class="fg3"><label>SQL</label><textarea v-model="nform.sql" class="fta3" rows="5" placeholder="SELECT ..."></textarea></div>
        <div class="mf3"><button class="bc3" @click="showCreate=false">取消</button><button class="bs3" :disabled="!nform.name || createSaving" @click="createQ">{{ createSaving ? '创建中...' : '创建' }}</button></div>
      </div>
    </div>
  </div>
</template>
<script setup lang="ts">
import { api } from '@oa4rust/sdk'
import { useMutation, useQuery, useQueryClient } from '@tanstack/vue-query'
import { computed, ref, watch } from 'vue'
import { formatServerDateTime } from '../utils/date'
import { toast } from '../utils/toast'
// biome-ignore lint/correctness/noUnusedImports: Vue templates consume component imports.
import ListLoadError from './ListLoadError.vue'
import { useListError } from './listQuery'
import { mutationErrorMessage } from './mutationError'

interface Q {
  id: string
  name?: string
  queryName?: string
  icon?: string
  category?: string
  entityCategory?: string
  updateTime?: string
  sql?: string
}
const queryMetaText = ref('')
async function loadQueryAll() {
  try {
    // 消费 query designer 三条无参真实路由：全部查询 / 查询概要 / query 分类
    const [all, summary, cats] = await Promise.all([
      api.get('/api/query/assemble/designer/query/list/all'),
      api.get('/api/query/assemble/designer/query/list/summary'),
      api.get('/api/query/assemble/designer/query/querycategory/list'),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : 0)
    queryMetaText.value = `全部查询 ${n(all)} / 概要 ${n(summary)} / 分类 ${n(cats)}`
  } catch (e: any) {
    toast.error(`加载失败: ${e?.message ?? ''}`)
  }
}
async function loadQueryExtra() {
  try {
    // GET neural/list/model + output/list + list/summary —— 神经模型/导出/查询概要
    const [models, outputs, summary] = await Promise.all([
      api.get('/api/query/assemble/designer/neural/list/model'),
      api.get('/api/query/assemble/designer/output/list'),
      api.get('/api/query/assemble/designer/list/summary'),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : 0)
    queryMetaText.value = `神经模型 ${n(models)} / 导出 ${n(outputs)} / 概要 ${n(summary)}`
  } catch (e: any) {
    toast.error(`加载失败: ${e?.message ?? ''}`)
  }
}
async function loadQueryMeta() {
  try {
    // GET querycategory/list + statement/list/manage —— 查询分类与语句管理列表
    const [cats, stmts] = await Promise.all([
      api.get('/api/query/assemble/designer/querycategory/list'),
      api.get('/api/query/assemble/designer/statement/list/manage'),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : 0)
    queryMetaText.value = `查询分类 ${n(cats)} / 语句 ${n(stmts)}`
  } catch (e: any) {
    toast.error(`加载失败: ${e?.message ?? ''}`)
  }
}
const sq = ref(''),
  qs2 = ref<Q[]>([]),
  selected = ref<Q | null>(null),
  filterText = ref(''),
  rdata = ref<any[]>([]),
  rheaders = ref<string[]>([]),
  rloading = ref(false),
  showCreate = ref(false),
  nform = ref({ name: '', sql: '' }),
  qc = useQueryClient()
const { data, isError, error } = useQuery({
  queryKey: ['query', 'defs'],
  queryFn: () => api.get('/api/query/assemble/designer/list/all').then((r: any) => (r.data ?? []) as Q[]),
})
watch(
  data,
  (rows) => {
    if (rows) qs2.value = rows
  },
  { immediate: true },
)
const { loadError, reload } = useListError({
  isError,
  error,
  queryKey: ['query', 'defs'],
  label: '查询定义加载失败',
})
const qsFiltered = computed(() =>
  sq.value ? qs2.value.filter((q) => (q.name || '').toLowerCase().includes(sq.value.toLowerCase())) : qs2.value,
)
function selQ(q: Q) {
  selected.value = q
  rdata.value = []
  rheaders.value = []
}
async function runQ() {
  if (!selected.value) return
  rloading.value = true
  try {
    const r = await api.post('/api/query/assemble/designer/execute', {
      id: selected.value!.id,
      filter: filterText.value,
    })
    const d = (r as any)?.data
    rdata.value = d?.list ?? []
    if (rdata.value.length > 0) rheaders.value = Object.keys(rdata.value[0])
  } catch (e: any) {
    rdata.value = []
    rheaders.value = []
    toast.error(`执行失败: ${e?.message ?? '未知错误'}`)
  } finally {
    rloading.value = false
  }
}
const dm = useMutation({
  onError: (e) => toast.error(mutationErrorMessage(e)),
  mutationFn: (id: string) => api.delete(`/api/query/assemble/designer/delete/${id}`),
  onSuccess: () => {
    qc.invalidateQueries({ queryKey: ['query', 'defs'] })
    if (selected.value?.id) selected.value = null
  },
})
async function delQ() {
  if (!selected.value) return
  // confirmMsg 返回 Promise<boolean>：未 await 时恒为 truthy，删除会在用户点确认前就发出
  if (await confirmMsg('确定删除？')) dm.mutate(selected.value.id)
}
const createSaving = ref(false)
const cm = useMutation({
  onSettled: () => (createSaving.value = false),
  onError: (e) => toast.error(mutationErrorMessage(e)),
  mutationFn: () => api.post('/api/query/assemble/designer/create', nform.value),
  onSuccess: () => {
    showCreate.value = false
    qc.invalidateQueries({ queryKey: ['query', 'defs'] })
  },
})
function createQ() {
  if (createSaving.value) return
  if (!nform.value.name) return
  createSaving.value = true
  cm.mutate()
}
function fmtT(t?: string) {
  return t ? formatServerDateTime(t) : ''
}
</script>
<style scoped>
.query-view{display:flex;flex-direction:column;gap:var(--ai-grid-2);height:100%;padding:var(--ai-grid-3);background:var(--ai-gradient-surface);border-radius:var(--ai-grid-2)}
.view-header{display:flex;align-items:center;justify-content:space-between;padding:var(--ai-grid-2) var(--ai-grid-3);background:var(--ai-gradient-surface);border:1px solid var(--ai-glass-border);backdrop-filter:var(--ai-glass-blur);border-radius:var(--ai-grid-2)}
.view-header h1{font-family:'Orbitron',sans-serif;font-size:20px;color:var(--color-primary);margin:0 0 4px;text-shadow:0 0 15px var(--color-primary-glow)}
.nb{padding:8px 16px;border-radius:var(--radius-md);border:none;background:var(--ai-gradient-brand);color:#fff;cursor:pointer;font-weight:600;box-shadow:var(--ai-shadow-1);transition:transform var(--ai-motion-fast) var(--ai-ease-spring)}
.nb:hover{transform:translateY(-1px)}
.ql{display:flex;flex:1;gap:16px;min-height:0}
.qs{width:260px;flex-shrink:0;display:flex;flex-direction:column;padding:var(--ai-grid-2);background:var(--ai-glass-bg);border:1px solid var(--ai-glass-border);backdrop-filter:var(--ai-glass-blur);box-shadow:var(--ai-glass-shadow);border-radius:var(--ai-grid-2)}
.sb2{display:flex;align-items:center;gap:8px;padding:8px;background:var(--bg-elevated);border-radius:var(--radius-md);margin-bottom:8px}
.si2{color:var(--text-muted)}
.si3{flex:1;background:none;border:none;outline:none;color:var(--text-primary);font-size:13px}
.ql2{list-style:none;padding:0;margin:0;flex:1;overflow-y:auto}
.qi2{display:flex;align-items:center;gap:8px;padding:10px;border-radius:var(--radius-md);cursor:pointer;transition:all var(--transition-fast);margin-bottom:4px}
.qi2:hover,.qi2.active{background:var(--color-primary-soft)}
.qicon{font-size:18px}
.qi2i{flex:1;min-width:0}
.qin{font-size:13px;color:var(--text-primary);font-weight:500;overflow:hidden;text-overflow:ellipsis;white-space:nowrap}
.qim{font-size:11px;color:var(--text-muted)}
.qit{font-size:10px;color:var(--text-muted)}
.qm{flex:1;display:flex;flex-direction:column;overflow:hidden;padding:16px}
.qh{display:flex;align-items:center;justify-content:space-between;margin-bottom:16px;padding-bottom:12px;border-bottom:1px solid var(--border-subtle)}
.qh h2{font-size:16px;color:var(--color-primary);margin:0;font-family:'Orbitron',sans-serif}
.qa2{display:flex;gap:6px}
.abtn{padding:4px 10px;border-radius:var(--radius-sm);border:1px solid var(--border-subtle);background:var(--bg-elevated);color:var(--text-secondary);cursor:pointer;font-size:12px;transition:all var(--transition-fast)}
.abtn:hover{border-color:var(--color-primary);color:var(--color-primary)}
.qf{display:flex;gap:8px;margin-bottom:12px;align-items:center}
.fi2{flex:1;background:var(--bg-elevated);border:1px solid var(--border-subtle);border-radius:var(--radius-md);padding:8px 12px;color:var(--text-primary);font-size:13px;outline:none;font-family:'JetBrains Mono',monospace}
.fi2:focus{border-color:var(--color-primary)}
.rb{padding:8px 16px;border-radius:var(--radius-md);border:none;background:var(--color-primary);color:white;cursor:pointer;font-weight:600;white-space:nowrap}
.qr{flex:1;overflow:auto}
.rt2{display:flex;flex-direction:column}
.rth,.tr2{display:grid;gap:2px;padding:6px 10px;font-size:13px}
.rth{background:var(--bg-elevated);color:var(--text-muted);font-weight:600;position:sticky;top:0;z-index:1}
.tr2:hover{background:var(--color-primary-soft)}
.rh2,.rc2{padding:2px 4px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap}
.em2,.es2,.ls2{display:flex;flex-direction:column;align-items:center;justify-content:center;padding:40px;color:var(--text-muted);gap:12px}
.emi2{font-size:48px;opacity:0.4}
.sk2{height:32px;border-radius:var(--radius-sm);margin-bottom:6px;background:linear-gradient(90deg,var(--ai-skeleton-base) 25%,var(--ai-skeleton-shine) 50%,var(--ai-skeleton-base) 75%);background-size:200% 100%;animation:ai-shimmer 1.6s var(--ai-ease-spring) infinite}
@keyframes ai-shimmer{0%{background-position:200% 0}100%{background-position:-200% 0}}
.mo{position:fixed;inset:0;background:var(--bg-overlay);z-index:200;display:flex;align-items:center;justify-content:center}
.modal{width:560px;padding:24px}
.modal h3{color:var(--color-primary);font-family:'Orbitron',sans-serif;margin:0 0 16px;font-size:16px}
.fg3{display:flex;flex-direction:column;gap:6px;margin-bottom:12px}
.fg3 label{font-size:12px;color:var(--text-muted);text-transform:uppercase;letter-spacing:1px}
.fi3{background:var(--bg-elevated);border:1px solid var(--border-subtle);border-radius:var(--radius-md);padding:10px 14px;color:var(--text-primary);font-size:14px;outline:none}
.fta3{background:var(--bg-elevated);border:1px solid var(--border-subtle);border-radius:var(--radius-md);padding:10px 14px;color:var(--color-primary);font-size:13px;outline:none;font-family:'JetBrains Mono',monospace;resize:vertical}
.fta3:focus{border-color:var(--color-primary)}
.mf3{display:flex;justify-content:flex-end;gap:8px;margin-top:16px}
.bc3{padding:8px 16px;border-radius:var(--radius-md);border:1px solid var(--border-subtle);background:none;color:var(--text-secondary);cursor:pointer}
.bs3{padding:8px 16px;border-radius:var(--radius-md);border:none;background:var(--color-primary);color:white;cursor:pointer;font-weight:600}
.bs3:disabled{opacity:.5;cursor:not-allowed}
@media(max-width:768px){.ql{flex-direction:column}.qs{width:100%;max-height:200px}}
.hdr-a{display:flex;gap:8px;align-items:center}
.nb.ghost{background:transparent;border:1px solid var(--border-subtle);color:var(--text-secondary)}
.qmeta{margin:8px 0;padding:8px 12px;border-radius:var(--radius-md);background:var(--bg-elevated);border:1px solid var(--border-subtle);font-size:12px;color:var(--text-secondary)}
.ai-empty{display:flex;flex-direction:column;align-items:center;justify-content:center;gap:var(--ai-grid-1);padding:var(--ai-grid-6);text-align:center;color:var(--text-secondary);background:var(--ai-glass-bg);border:1px dashed var(--ai-glass-border);border-radius:var(--ai-grid-2)}
.ai-empty-icon{font-size:36px;line-height:1}
.ai-empty-hint{font-size:12px;opacity:.75}
</style>
