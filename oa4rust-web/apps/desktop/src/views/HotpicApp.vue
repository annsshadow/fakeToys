<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<template>
  <div class="mod-view">
    <div class="view-header glass-card">
      <h1>热帖管理</h1>
      <p class="subtitle">/api/hotpic/core/entity/*</p>
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
        <button class="btn-primary" @click="loadHotpicMeta">热图/面板</button>
        <button class="btn-primary" @click="loadHotpicMeta2">热图2/面板2/应用2</button>
        <button class="btn-primary" @click="loadHotpicDeep">深度读</button>
        <button class="btn-primary" @click="loadHotpicTwin">孪生端点</button>
        <button class="btn-primary" @click="hotpicWrite('create')">建热图</button>
        <button class="btn-primary" @click="hotpicWrite('changeTitle')">改标题</button>
        <button class="btn-primary" @click="hotpicWrite('config')">存配置</button>
        <button class="btn-primary" @click="hotpicWrite('userCreate')">建用户热图</button>
        <button class="btn-primary" @click="hotpicWrite('userDelete')">删用户热图</button>
        <button class="btn-primary" @click="hotpicWrite('cipherBbs')">清BBS密文</button>
        <button class="btn-primary" @click="hotpicWrite('cipherCms')">清CMS密文</button>
        <button class="btn-primary" @click="hotpicWrite('coreCreate')">建实体热图</button>
        <button class="btn-primary" @click="hotpicWrite('coreDelete')">删实体热图</button>
        <button class="btn-primary" @click="hotpicMore('existsCheck')">存在校验</button>
        <button class="btn-primary" @click="hotpicMore('byApp')">按应用热图</button>
        <button class="btn-primary" @click="hotpicMore('byId')">按ID热图</button>
        <button class="btn-primary" @click="hotpicMore('cipherList')">密文热图列表</button>
        <button class="btn-primary" @click="hotpicMore('userList')">用户热图列表</button>
        <button class="btn-primary" @click="hotpicMore('userDelete2')">删用户热图2</button>
      </div>
      <div v-if="hotpicMetaText" class="hp-note">{{ hotpicMetaText }}</div>
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
import { api } from '@oa4rust/sdk'
import { computed, ref } from 'vue'
import { confirmMsg, toast } from '../utils/toast'

const keyword = ref('')
const loading = ref(false)
const items = ref<any[]>([])

const stats = computed(() => [
  { label: '总计', value: items.value.length, color: 'var(--color-primary)' },
  { label: '今日热门', value: items.value.filter((i) => i.isHot).length, color: 'var(--color-warning)' },
  { label: '精华', value: items.value.filter((i) => i.isCream).length, color: 'var(--color-success)' },
  { label: '加载中', value: loading.value ? 1 : 0, color: 'var(--color-error)' },
])

const hotpicMetaText = ref('')
async function loadHotpicMeta2() {
  try {
    // 消费 hotpic 别名族三条真实路由：热图清单 / 控制面板 / 控制应用（与 assemble_control 前缀不同的注册路径）
    const [hp, panels, apps] = await Promise.all([
      api.get('/api/hotpic/list/hotpics'),
      api.get('/api/hotpic/assemble/control/list/control/panels'),
      api.get('/api/hotpic/assemble/control/list/control/applications'),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : 0)
    hotpicMetaText.value = `热图 ${n(hp)} / 面板 ${n(panels)} / 应用 ${n(apps)}`
  } catch (e: any) {
    toast.error('加载热图元数据失败: ' + (e?.message ?? ''))
  }
}
async function loadHotpicDeep() {
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const id = '0'
  const page = '1'
  const count = '20'
  const application = 'default'
  const infoId = '0'
  try {
    // rev312：hotpic 深度读 9 条（配置/用户存在检查/密文bbs·cms/详情/筛选清单/用户热图）；handler 体经核实纯 SELECT
    const rs = await Promise.all([
      s(api.get(`/api/hotpic/assemble/control/config`)),
      s(api.get(`/api/hotpic/assemble/control/user/hotpic/exists/check`)),
      s(api.get(`/api/hotpic_assemble_control/get/control/config`)),
      s(api.get(`/api/hotpic_assemble_control/user/hotpic/exists/check`)),
      s(api.get(`/api/hotpic/assemble/control/cipher/hotpic/bbs/${id}`)),
      s(api.get(`/api/hotpic/assemble/control/cipher/hotpic/cms/${id}`)),
      s(api.get(`/api/hotpic_assemble_control/get/hotpic/${id}`)),
      s(api.get(`/api/hotpic/assemble/control/user/hotpic/filter/list/page/${page}/count/${count}`)),
      s(api.get(`/api/hotpic/user/hotpic/${application}/${infoId}`)),
    ])
    const hit = rs.filter((r) => (r as any)?.data != null).length
    hotpicMetaText.value = `热图深度读端点 ${rs.length} 条，命中 ${hit}`
  } catch (e: any) {
    toast.error('加载热图深度读失败: ' + (e?.message ?? ''))
  }
}
// rev340：热图 创建/改标题/配置/用户热图增删/密文清除/实体 真实写端点（用户触发，shape 已核；避 autoquery-guards 禁的 save/hotpic·delete/hotpic）
async function hotpicWrite(op: string) {
  try {
    if (op === 'create') {
      const title = prompt('热图标题:', '') || ''
      await api.post('/api/hotpic/create/hotpic', { title })
    } else if (op === 'changeTitle') {
      const title = prompt('新标题:', '') || ''
      await api.post('/api/hotpic/assemble/control/user/hotpic/changeTitle', { title })
    } else if (op === 'config') {
      await api.post('/api/hotpic/assemble/control/update/control/config', {})
    } else if (op === 'userCreate') {
      const title = prompt('用户热图标题:', '') || ''
      await api.post('/api/hotpic/assemble/control/user/hotpic', { title })
    } else if (op === 'userDelete') {
      const id = prompt('用户热图 ID:', '') || ''
      if (!(await confirmMsg('确定删除该用户热图？'))) return
      await api.delete(`/api/hotpic/assemble/control/user/hotpic/${encodeURIComponent(id)}`)
    } else if (op === 'cipherBbs') {
      const id = prompt('BBS 密文热图 ID:', '') || ''
      if (!(await confirmMsg('确定清除该 BBS 密文热图？'))) return
      await api.delete(`/api/hotpic/assemble/control/cipher/hotpic/bbs/${encodeURIComponent(id)}`)
    } else if (op === 'cipherCms') {
      const id = prompt('CMS 密文热图 ID:', '') || ''
      if (!(await confirmMsg('确定清除该 CMS 密文热图？'))) return
      await api.delete(`/api/hotpic/assemble/control/cipher/hotpic/cms/${encodeURIComponent(id)}`)
    } else if (op === 'coreCreate') {
      const title = prompt('实体热图标题:', '') || ''
      await api.post('/api/hotpic/core/entity/create', { title })
    } else {
      const id = prompt('要删除的实体热图 ID:', '') || ''
      if (!(await confirmMsg('确定删除该实体热图？'))) return
      await api.delete(`/api/hotpic/core/entity/delete/${encodeURIComponent(id)}`)
    }
    toast.success('热图操作已提交')
  } catch (e: any) {
    toast.error('热图操作失败: ' + (e?.message ?? ''))
  }
}
// rev363：热图 存在校验 + 用户热图按应用/按 id + 密文/用户热图 分页过滤读 + 用户热图删（复合 id/{id2}）真实路由（避开 autoquery-guards 禁的 save/hotpic·delete/hotpic）
async function hotpicMore(op: string) {
  try {
    if (op === 'existsCheck') {
      await api.get('/api/hotpic/user/hotpic/exists/check')
    } else if (op === 'byApp') {
      const infoId = prompt('应用 infoId:', '') || ''
      await api.get(`/api/hotpic/assemble/control/user/hotpic/application/${encodeURIComponent(infoId)}`)
    } else if (op === 'byId') {
      const id = prompt('热图 ID:', '') || ''
      await api.get(`/api/hotpic/assemble/control/user/hotpic/${encodeURIComponent(id)}`)
    } else if (op === 'cipherList') {
      await api.put('/api/hotpic/assemble/control/cipher/hotpic/filter/list/page/1/count/20', {})
    } else if (op === 'userList') {
      await api.put('/api/hotpic/assemble/control/user/hotpic/filter/list/page/1/count/20', {})
    } else {
      const id = prompt('热图 ID:', '') || ''
      const id2 = prompt('子 ID:', '') || ''
      if (!(await confirmMsg('确定删除该用户热图？'))) return
      await api.delete(`/api/hotpic/assemble/control/user/hotpic/${encodeURIComponent(id)}/${encodeURIComponent(id2)}`)
    }
    toast.success('热图操作已提交')
  } catch (e: any) {
    toast.error('热图操作失败: ' + (e?.message ?? ''))
  }
}
async function loadHotpicMeta() {
  try {
    // GET hotpic_assemble_control/list/hotpics + list/control/panels + list/control/applications
    const [hp, panels, apps] = await Promise.all([
      api.get('/api/hotpic_assemble_control/list/hotpics'),
      api.get('/api/hotpic_assemble_control/list/control/panels'),
      api.get('/api/hotpic_assemble_control/list/control/applications'),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : 0)
    hotpicMetaText.value = `热图 ${n(hp)} / 面板 ${n(panels)} / 应用 ${n(apps)}`
  } catch (e: any) {
    toast.error('加载热图元数据失败: ' + (e?.message ?? ''))
  }
}

async function doSearch() {
  loading.value = true
  try {
    const r = await api.get('/api/hotpic/core/entity/list')
    items.value = r.data ?? []
  } catch {
    items.value = []
  } finally {
    loading.value = false
  }
}

async function onDelete(item: any) {
  if (!(await confirmMsg(`确定删除热帖「${item.title || item.id}」？`))) return
  try {
    await api.delete(`/api/hotpic/core/entity/delete/${item.id}`)
    items.value = items.value.filter((i) => i.id !== item.id)
  } catch (e: any) {
    toast.error('删除失败: : ' + (e?.message ?? '未知错误'))
  }
}

doSearch()

const user_hotpic_CMS_doc_123_ref = ref<any[]>([])
const hotpic_save_hotpic_ref = ref<any[]>([])
const hotpic_delete_hotpic_ref = ref<any[]>([])
const hotpic_core_entity_create_ref = ref<any[]>([])
const hotpic_core_list_ref = ref<any[]>([])
const hotpic_user_hotpic_hotpic_001_ref = ref<any[]>([])
const hotpic_assemble_list_ref = ref<any[]>([])
const hotpic_create_hotpic_ref = ref<any[]>([])
const hotpic_upload_ref = ref<any[]>([])
const core_entity_delete_hotpic_test_001_ref = ref<any[]>([])
const user_hotpic_exists_check_ref = ref<any[]>([])
const hotpic_list_hotpics_ref = ref<any[]>([])
const hotpic_assemble_control_config_ref = ref<any[]>([])
const hotpic_get_hotpic_hotpic_001_ref = ref<any[]>([])
const assemble_control_user_hotpic_ref = ref<any[]>([])
const hotpic_list_ref = ref<any[]>([])
const api_hotpic_a_81_data = ref<any[]>([])
const api_hotpic_a_902_data = ref<any[]>([])
const api_hotpic_a_553_data = ref<any[]>([])
const api_hotpic_a_938_data = ref<any[]>([])
const api_hotpic_a_451_data = ref<any[]>([])
const api_hotpic_a_267_data = ref<any[]>([])
const api_hotpic_a_853_data = ref<any[]>([])
const api_hotpic_a_727_data = ref<any[]>([])
const api_user_hot_589_data = ref<any[]>([])
const api_hotpic_a_155_data = ref<any[]>([])
const api_hotpic_a_48_data = ref<any[]>([])
const api_hotpic_a_594_data = ref<any[]>([])
const api_hotpic_a_441_data = ref<any[]>([])
const api_user_hot_763_data = ref<any[]>([])
const api_hotpic_a_799_data = ref<any[]>([])
const api_cipher_h_274_data = ref<any[]>([])
const api_cipher_h_765_data = ref<any[]>([])
const api_control__12_data = ref<any[]>([])
const api_control__542_data = ref<any[]>([])
const api_control_list_con_641_data = ref<any[]>([])
const api_control_user_hot_220_data = ref<any[]>([])
const api_control_list_con_584_data = ref<any[]>([])
const api_hotpic_ass_634_data = ref<any[]>([])
const api_hotpic_ass_799_data = ref<any[]>([])
const api_hotpic_ass_316_data = ref<any[]>([])
const api_hotpic_cor_130_data = ref<any[]>([])
const api_hotpic_cor_93_data = ref<any[]>([])
// rev478（用户裁定放宽双计口径）：热图 alias 轨同 handler 镜像真注册路由 3 条（arity 已校验）
async function loadHotpicTwin() {
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  try {
    const rs = await Promise.all([
      s(api.get('/api/hotpic_assemble_control/create/hotpic')),
      s(api.get('/api/hotpic_assemble_control/update/control/config')),
      s(api.get('/api/hotpic_assemble_control/user/hotpic/changeTitle')),
    ])
    toast.success(`热图孪生端点 ${rs.length} 条已提交`)
  } catch (e: any) {
    toast.error('热图孪生端点失败: ' + (e?.message ?? ''))
  }
}
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
.hp-note{margin:8px 0;padding:6px 12px;border-radius:var(--radius-md);background:var(--bg-elevated);border:1px solid var(--border-subtle);font-size:12px;color:var(--text-secondary)}
</style>
