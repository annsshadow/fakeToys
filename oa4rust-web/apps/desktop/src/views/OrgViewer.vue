<template>
  <div class="org-view">
    <div class="view-header glass-card">
      <h1>组织架构</h1>
      <p class="subtitle">/jaxrs/organization/assemble/control/*</p>
    </div>
    <div class="org-layout">
      <aside class="org-tree glass-card">
        <div class="search-bar">
          <span class="si">⌕</span>
          <input v-model="keyword" @input="handleSearch" placeholder="Search..." class="si2" />
        </div>
        <div v-if="treeLoading" class="ls"><div class="sk" v-for="i in 6" :key="i"></div></div>
        <div v-else-if="nodes.length===0" class="es"><p>Loading...</p></div>
        <ul v-else class="tree-list">
          <li v-for="n in nodes" :key="n.id" class="tree-node">
            <div class="nr" :class="{active:selected?.id===n.id}" @click="selectNode(n)">
              <span class="nt" @click.stop="toggleNode(n)">{{n._exp?'v':'>'}}</span>
              <span class="ni">{{n.type==='group'?'[D]':'[P]'}}</span>
              <span class="nn">{{n.name}}</span>
              <span v-if="(n as any).childCount" class="nc">{{(n as any).childCount}}</span>
            </div>
            <ul v-show="n._exp" class="tree-children">
              <li v-for="c in n.children" :key="c.id" class="tree-node">
                <div class="nr" :class="{active:selected?.id===c.id}" @click="selectNode(c)">
                  <span class="nt"></span>
                  <span class="ni">{{c.type==='group'?'[D]':'[P]'}}</span>
                  <span class="nn">{{c.name}}</span>
                </div>
              </li>
            </ul>
          </li>
        </ul>
      </aside>
      <main class="org-detail glass-card">
        <div v-if="!selected" class="es-lg"><div class="ei">O</div><p>Select a node to view details</p></div>
        <template v-else>
          <div class="dh"><span class="di">{{selected.type==='group'?'O':'P'}}</span><div><h2>{{selected.name}}</h2><p class="ds">{{selected.type==='group'?'Dept':'Person'}}</p></div></div>
          <div class="dg"><div class="di2"><span class="dl">ID</span><span class="dv mono">{{selected.id}}</span></div><div class="di2"><span class="dl">Name</span><span class="dv">{{selected.name}}</span></div></div>
          <div v-if="selected.type==='group'" class="members">
            <h3>Members</h3>
            <div class="mlist">
              <div v-if="(selected as any).members?.length" class="mc" v-for="m in (selected as any).members" :key="m.id">
                <div class="ma2">{{m.name?.charAt(0)}}</div>
                <div class="mi2"><div class="mn">{{m.name}}</div><div class="mp">{{m.position||m.role||'Emp'}}</div></div>
              </div>
              <div v-else class="empty-m">No members</div>
            </div>
          </div>
        </template>
      </main>
    </div>
  </div>
</template>
<script setup lang="ts">
import { ref } from 'vue'
import { useQuery } from '@tanstack/vue-query'
import { api } from '@oa4rust/sdk'
interface N { id: string; name: string; type: 'group' | 'person'; _exp?: boolean; children?: N[]; childCount?: number }
const keyword = ref('')
const nodes = ref<N[]>([])
const selected = ref<N | null>(null)
const treeLoading = ref(false)
let timer: ReturnType<typeof setTimeout>
const { data } = useQuery({ queryKey: ['org', 'tree'], queryFn: () => api.get('/jaxrs/organization/assemble/control/group/list').then((r: any) => { nodes.value = (r.data ?? []) as N[]; return r }), staleTime: 300000 })
function toggleNode(n: N) { n._exp = !n._exp; if (n._exp && !n.children) { const id = n.id; api.get('/jaxrs/organization/assemble/control/group/' + id + '/sub/nested').then((r: any) => { n.children = (r.data ?? []) as N[] }) } }
function selectNode(n: N) { selected.value = n }
async function handleSearch() { if (!keyword.value.trim()) { nodes.value = []; return } try { const r = await api.get('/jaxrs/organization/assemble/control/group/list/like', { params: { keyword: keyword.value } }); nodes.value = (r.data ?? []) as N[] } catch {} }
</script>
<style scoped>
.org-view{display:flex;flex-direction:column;gap:16px;height:100%}
.view-header{padding:16px 24px}
.view-header h1{font-family:'Orbitron',sans-serif;font-size:20px;color:var(--color-primary);margin:0 0 4px;text-shadow:0 0 15px var(--color-primary-glow)}
.subtitle{font-size:12px;color:var(--text-muted);margin:0;font-family:'JetBrains Mono',monospace}
.org-layout{display:flex;flex:1;gap:16px;min-height:0}
.org-tree{width:280px;flex-shrink:0;display:flex;flex-direction:column;padding:12px}
.search-bar{display:flex;align-items:center;gap:8px;padding:8px;background:var(--bg-elevated);border-radius:var(--radius-md);margin-bottom:8px}
.si{color:var(--text-muted)}
.si2{flex:1;background:none;border:none;outline:none;color:var(--text-primary);font-size:13px}
.tree-list{list-style:none;padding:0;margin:0;flex:1;overflow-y:auto}
.tree-node{}
.nr{display:flex;align-items:center;gap:6px;padding:8px 10px;border-radius:var(--radius-sm);cursor:pointer;color:var(--text-secondary);font-size:13px;transition:all var(--transition-fast)}
.nr:hover,.nr.active{background:var(--color-primary-soft);color:var(--color-primary)}
.nt{width:16px;font-size:10px;color:var(--text-muted);flex-shrink:0}
.ni{font-size:14px;flex-shrink:0}
.nn{flex:1;overflow:hidden;text-overflow:ellipsis;white-space:nowrap}
.nc{font-size:10px;background:var(--bg-elevated);color:var(--text-muted);padding:1px 6px;border-radius:8px;flex-shrink:0}
.tree-children{list-style:none;padding:0 0 0 16px;margin:0}
.org-detail{flex:1;overflow-y:auto;padding:24px}
.dh{display:flex;align-items:center;gap:16px;margin-bottom:20px;padding-bottom:16px;border-bottom:1px solid var(--border-subtle)}
.di{font-size:32px}
.dh h2{font-size:20px;color:var(--text-primary);margin:0 0 4px}
.ds{font-size:12px;color:var(--text-muted);margin:0}
.dg{display:grid;grid-template-columns:1fr 1fr;gap:12px;margin-bottom:20px}
.di2{padding:12px 16px;background:var(--bg-elevated);border-radius:var(--radius-md)}
.dl{display:block;font-size:11px;color:var(--text-muted);text-transform:uppercase;letter-spacing:1px;margin-bottom:4px}
.dv{font-size:14px;color:var(--text-primary)}
.dv.mono{font-family:'JetBrains Mono',monospace;font-size:12px;color:var(--color-primary)}
.members h3{font-size:14px;color:var(--color-primary);margin:0 0 12px}
.mlist{display:grid;grid-template-columns:repeat(auto-fill,minmax(180px,1fr));gap:8px}
.mc{display:flex;align-items:center;gap:10px;padding:10px 12px;background:var(--bg-elevated);border-radius:var(--radius-md)}
.ma2{width:32px;height:32px;border-radius:50%;background:var(--color-primary-soft);color:var(--color-primary);display:flex;align-items:center;justify-content:center;font-weight:600;font-size:14px}
.mi2{flex:1}
.mn{font-size:13px;color:var(--text-primary);font-weight:500}
.mp{font-size:11px;color:var(--text-muted)}
.es,.ls,.es-lg{display:flex;flex-direction:column;align-items:center;justify-content:center;padding:40px;color:var(--text-muted);gap:12px}
.ei{font-size:48px;opacity:0.4}
.sk{height:32px;border-radius:var(--radius-sm);margin-bottom:6px;background:var(--bg-elevated)}
.empty-m{color:var(--text-muted);font-size:13px;padding:20px;text-align:center}
@media(max-width:768px){.org-layout{flex-direction:column}.org-tree{width:100%;max-height:200px}}
</style>
