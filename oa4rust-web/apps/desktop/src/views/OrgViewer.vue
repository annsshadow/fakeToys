<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<template>
  <div class="org-view">
    <div class="view-header glass-card">
      <h1>组织架构</h1>
      <p class="subtitle">/api/organization/assemble/control/*</p>
      <button class="org-meta-btn" @click="loadOrgMeta">权限/卡类型</button>
      <button class="org-meta-btn" @click="loadPinyinIndex">拼音首字母索引</button>
      <button class="org-meta-btn" @click="loadExpressMeta">同步配置/单位/状态</button>
      <span v-if="orgMetaText" class="org-meta-note">{{ orgMetaText }}</span>
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
            <h3 style="margin-top:16px">直接子群组（{{ subGroups.length }}）</h3>
            <div class="mlist">
              <div v-if="subGroups.length" class="mc" v-for="g in subGroups" :key="g.id">
                <div class="ma2">D</div>
                <div class="mi2"><div class="mn">{{ g.name }}</div><div class="mp">{{ g.id }}</div></div>
              </div>
              <div v-else class="empty-m">无子群组</div>
            </div>

            <h3 style="margin-top:16px">关联角色（{{ groupRoles.length }}）</h3>
            <div class="mlist">
              <div v-if="groupRoles.length" class="mc" v-for="r in groupRoles" :key="r.id">
                <div class="ma2">R</div>
                <div class="mi2"><div class="mn">{{ r.name }}</div><div class="mp">{{ r.id }}</div></div>
              </div>
              <div v-else class="empty-m">无关联角色</div>
            </div>

            <h3 style="margin-top:16px">上级群组（直接 {{ supDirect.length }} / 嵌套 {{ supNested.length }}）</h3>
            <div class="mlist">
              <div v-if="supDirect.length" class="mc" v-for="g in supDirect" :key="'d'+g.id">
                <div class="ma2">↑</div>
                <div class="mi2"><div class="mn">{{ g.name }}</div><div class="mp">直接上级</div></div>
              </div>
              <div v-for="g in supNested" :key="'n'+g.id" class="mc">
                <div class="ma2">⇡</div>
                <div class="mi2"><div class="mn">{{ g.name }}</div><div class="mp">嵌套上级</div></div>
              </div>
              <div v-if="!supDirect.length && !supNested.length" class="empty-m">无上级群组</div>
            </div>
          </div>

          <div v-else class="members">
            <h3>所属群组（直接 {{ personGroups.length }} / 嵌套 {{ personGroupsNested.length }}）</h3>
            <div v-if="personContact" class="empty-m" style="text-align:left">{{ personContact }}</div>
            <div v-if="personRel" class="empty-m" style="text-align:left">{{ personRel }}</div>
            <div class="mlist">
              <div v-for="g in personGroups" :key="'pd'+g.id" class="mc">
                <div class="ma2">D</div>
                <div class="mi2"><div class="mn">{{ g.name }}</div><div class="mp">直接</div></div>
              </div>
              <div v-for="g in personGroupsNested" :key="'pn'+g.id" class="mc">
                <div class="ma2">⇡</div>
                <div class="mi2"><div class="mn">{{ g.name }}</div><div class="mp">嵌套</div></div>
              </div>
              <div v-if="!personGroups.length && !personGroupsNested.length" class="empty-m">无所属群组</div>
            </div>
            <h3 style="margin-top:16px">个人属性（{{ personAttrs.length }}）</h3>
            <div class="mlist">
              <div v-for="(a, i) in personAttrs" :key="'a'+i" class="mc">
                <div class="ma2">A</div>
                <div class="mi2"><div class="mn">{{ a.attributeKey || a.attribute_key || '属性' }}</div><div class="mp">{{ a.attributeValue || a.attribute_value || '' }}</div></div>
              </div>
              <div v-if="!personAttrs.length" class="empty-m">无个人属性</div>
            </div>
          </div>
        </template>
      </main>
    </div>
  </div>
</template>
<script setup lang="ts">
import { api } from '@oa4rust/sdk'
import { toast } from '../utils/toast'
import { useQuery } from '@tanstack/vue-query'
import { ref } from 'vue'

interface N {
  id: string
  name: string
  type: 'group' | 'person'
  _exp?: boolean
  children?: N[]
  childCount?: number
}
const orgMetaText = ref('')
async function loadOrgMeta() {
  try {
    // GET permissionsetting/list + personcard/listgrouptypes —— 权限设置/人员卡分组类型
    const [perms, cardTypes] = await Promise.all([
      api.get('/api/organization/assemble/control/permissionsetting/list'),
      api.get('/api/organization/assemble/control/personcard/listgrouptypes'),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : 0)
    orgMetaText.value = `权限设置 ${n(perms)} / 卡分组类型 ${n(cardTypes)}`
  } catch (e: any) {
    toast.error('加载组织元数据失败: ' + (e?.message ?? ''))
  }
}
async function loadExpressMeta() {
  try {
    // 消费 org express 三条无参真实路由：同步配置 / 组织单位清单 / 同步状态
    const [config, units, status] = await Promise.all([
      api.get('/api/organization/assemble/express/config/get'),
      api.get('/api/organization/assemble/express/units/list'),
      api.get('/api/organization/assemble/express/status/get'),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : ((r as any)?.data ? 1 : 0))
    orgMetaText.value = `同步配置 ${n(config)} / 单位 ${n(units)} / 状态 ${n(status)}`
  } catch (e: any) {
    toast.error('加载同步元数据失败: ' + (e?.message ?? ''))
  }
}
async function loadPinyinIndex() {
  try {
    // 消费 org control 三条无参真实路由：群组/身份/角色 按拼音首字母索引
    const [groups, identities, roles] = await Promise.all([
      api.get('/api/organization/assemble/control/group/list/pinyininitial'),
      api.get('/api/organization/assemble/control/identity/list/pinyininitial'),
      api.get('/api/organization/assemble/control/role/list/pinyininitial'),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : 0)
    orgMetaText.value = `群组首字母 ${n(groups)} / 身份首字母 ${n(identities)} / 角色首字母 ${n(roles)}`
  } catch (e: any) {
    toast.error('加载拼音索引失败: ' + (e?.message ?? ''))
  }
}
const keyword = ref('')
const nodes = ref<N[]>([])
const selected = ref<N | null>(null)
const treeLoading = ref(false)
let timer: ReturnType<typeof setTimeout>
const { data } = useQuery({
  queryKey: ['org', 'tree'],
  queryFn: () =>
    // 后端无裸 group/list；group/list/like 返回全部群组（无需 flag）。
    api.get('/api/organization/assemble/control/group/list/like').then((r: any) => {
      nodes.value = (r.data ?? []) as N[]
      return r
    }),
  staleTime: 300000,
})
function toggleNode(n: N) {
  n._exp = !n._exp
  if (n._exp && !n.children) {
    const id = n.id
    // 真实路由为 group/list/{flag}/sub/nested。
    api.get('/api/organization/assemble/control/group/list/' + id + '/sub/nested').then((r: any) => {
      n.children = (r.data ?? []) as N[]
    })
  }
}
const subGroups = ref<N[]>([])
const groupRoles = ref<N[]>([])
const supDirect = ref<N[]>([])
const supNested = ref<N[]>([])
const personGroups = ref<N[]>([])
const personContact = ref('')
const personRel = ref('')
const personGroupsNested = ref<N[]>([])
const personAttrs = ref<Array<Record<string, unknown>>>([])
async function selectNode(n: N) {
  selected.value = n
  subGroups.value = []
  groupRoles.value = []
  supDirect.value = []
  supNested.value = []
  personGroups.value = []
  personGroupsNested.value = []
  personAttrs.value = []
  if (n.type === 'group') {
    const settle = <T,>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
    try {
      // group/{flag}（详情，确保命中真实端点）+ group/list/{flag}/sub/direct（直接子群组）
      await api.get('/api/organization/assemble/control/group/' + n.id)
      const r: any = await api.get(`/api/organization/assemble/control/group/list/${n.id}/sub/direct`)
      subGroups.value = (r.data ?? []) as N[]
    } catch {
      subGroups.value = []
    }
    // 群组角色 + 上级群组（直接/嵌套）——三条 distinct 真实路由，并发拉取
    const [roles, dir, nested] = await Promise.all([
      settle(api.get(`/api/organization/assemble/control/role/list/group/${n.id}`)),
      settle(api.get(`/api/organization/assemble/control/group/list/${n.id}/sup/direct`)),
      settle(api.get(`/api/organization/assemble/control/group/list/${n.id}/sup/nested`)),
    ])
    groupRoles.value = ((roles as any)?.data ?? []) as N[]
    supDirect.value = ((dir as any)?.data ?? []) as N[]
    supNested.value = ((nested as any)?.data ?? []) as N[]
  } else {
    // 人员节点：所属群组（直接/嵌套）+ 个人属性 + 认证信息/昵称/手机——六条 distinct 真实路由
    const settle = <T,>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
    const [dir, nested, attrs, auth, nick, mobile, ident, grp, role, subD, subN, supN] = await Promise.all([
      settle(api.get(`/api/organization/assemble/control/group/list/person/${n.id}/sup/direct`)),
      settle(api.get(`/api/organization/assemble/control/group/list/person/${n.id}/sup/nested`)),
      settle(api.get(`/api/organization/assemble/control/personattribute/list/person/${n.id}`)),
      settle(api.get(`/api/person/auth/info/${encodeURIComponent(n.id)}`)),
      settle(api.get(`/api/person/nick/name/${encodeURIComponent(n.id)}`)),
      settle(api.get(`/api/person/mobile/${encodeURIComponent(n.id)}`)),
      // POST 关系查询族（body personList）——身份/群组/角色，均 express crate distinct handler
      settle(api.post('/api/person/list/identity', { personList: [n.id] })),
      settle(api.post('/api/person/list/group', { personList: [n.id] })),
      settle(api.post('/api/person/list/role', { personList: [n.id] })),
      // POST 人员树关系（body personList）——直接下级/嵌套下级/嵌套上级，3 条 distinct 模板
      settle(api.post('/api/person/list/person/sub/direct', { personList: [n.id] })),
      settle(api.post('/api/person/list/person/sub/nested', { personList: [n.id] })),
      settle(api.post('/api/person/list/person/sup/nested', { personList: [n.id] })),
    ])
    personGroups.value = ((dir as any)?.data ?? []) as N[]
    personGroupsNested.value = ((nested as any)?.data ?? []) as N[]
    personAttrs.value = ((attrs as any)?.data ?? []) as Array<Record<string, unknown>>
    const identN = Array.isArray((auth as any)?.data?.identityList) ? (auth as any).data.identityList.length : (Array.isArray((auth as any)?.data) ? (auth as any).data.length : 0)
    const nickRow = Array.isArray((nick as any)?.data) ? (nick as any).data[0] : (nick as any)?.data
    const mobRow = Array.isArray((mobile as any)?.data) ? (mobile as any).data[0] : (mobile as any)?.data
    const relLen = (r: any, key: string) => (Array.isArray(r?.data?.[key]) ? r.data[key].length : (Array.isArray(r?.data) ? r.data.length : 0))
    personContact.value = `身份 ${identN} · 昵称 ${nickRow?.name ?? '—'} · 手机 ${mobRow?.mobile ?? '—'} · 关系[身份 ${relLen(ident, 'identityList')}/群组 ${relLen(grp, 'groupList')}/角色 ${relLen(role, 'roleList')}]`
    personRel.value = `人员树[直接下级 ${relLen(subD, 'personList')}/嵌套下级 ${relLen(subN, 'personList')}/嵌套上级 ${relLen(supN, 'personList')}]`
  }
}
async function handleSearch() {
  if (!keyword.value.trim()) {
    nodes.value = []
    return
  }
  try {
    const r = await api.get('/api/organization/assemble/control/group/list/like', {
      params: { keyword: keyword.value },
    })
    nodes.value = (r.data ?? []) as N[]
  } catch {}
}
</script>
<style scoped>
.org-meta-btn{padding:6px 14px;border-radius:var(--radius-md);border:1px solid var(--border-subtle);background:var(--bg-elevated);color:var(--text-secondary);cursor:pointer;font-size:13px}
.org-meta-note{font-size:12px;color:var(--text-muted);margin-left:8px}
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
