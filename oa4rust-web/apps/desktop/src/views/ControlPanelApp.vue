<template>
  <div class="nav-view">
    <div class="view-header glass-card">
      <div><h1>控制面板</h1><p class="subtitle">系统总览 · 快速导航</p></div>
      <button class="refresh-btn" @click="loadStats">🔄 刷新</button>
    </div>
    <div class="stats-row glass-card">
      <div class="stat-card" v-for="s in stats" :key="s.label">
        <div class="stat-num" :style="{ color: s.color }">{{ s.value }}</div>
        <div class="stat-label">{{ s.label }}</div>
      </div>
    </div>
    <div class="content-panel glass-card">
      <h3 class="panel-title">设计器中心</h3>
      <div class="nav-grid">
        <router-link v-for="item in designers" :key="item.path" :to="item.path" class="nav-card">
          <div class="nc-icon">{{ item.icon }}</div>
          <div class="nc-title">{{ item.title }}</div>
          <div class="nc-desc">{{ item.desc }}</div>
        </router-link>
      </div>
    </div>
    <div class="content-panel glass-card" v-if="apps.length > 0">
      <h3 class="panel-title">最近应用</h3>
      <div class="app-list">
        <router-link v-for="a in apps.slice(0,6)" :key="a.id" :to="'/app/program'" class="app-item">
          <span class="app-icon">📱</span>
          <span class="app-name">{{ a.name }}</span>
          <span class="app-flag">{{ a.flag }}</span>
        </router-link>
      </div>
    </div>
  </div>
</template>
<script setup lang="ts">
import { api } from '@oa4rust/sdk'
import { onMounted, ref } from 'vue'

const stats = ref([
  { label: '待办事项', value: '--', color: 'var(--color-warning)' },
  { label: '应用总数', value: '--', color: 'var(--color-info)' },
  { label: '在线用户', value: '--', color: 'var(--color-success)' },
  { label: '系统状态', value: '正常', color: 'var(--color-primary)' },
])
const apps = ref<Array<{ id: string; name: string; flag: string }>>([])
const designers = [
  { icon: '⚡', title: '流程设计器', desc: '可视化流程编排', path: '/app/process-designer' },
  { icon: '📋', title: '表单设计器', desc: 'JSON表单构建', path: '/app/form-designer' },
  { icon: '🔍', title: '查询设计器', desc: 'SQL与可视化查询', path: '/app/query-designer' },
  { icon: '🖼', title: '门户设计器', desc: '页面与组件设计', path: '/app/portal-designer' },
  { icon: '📝', title: 'CMS表单设计', desc: '内容模型表单', path: '/app/cms-form-designer' },
  { icon: '👁', title: 'CMS视图设计', desc: '内容展示视图', path: '/app/cms-view-designer' },
  { icon: '💻', title: 'CMS脚本设计', desc: '内容处理脚本', path: '/app/cms-script-designer' },
  { icon: '📚', title: 'CMS字典设计', desc: '内容字典配置', path: '/app/cms-dict-designer' },
]
async function loadStats() {
  try {
    const [p, a] = await Promise.allSettled([
      api.get('/jaxrs/processplatform/assemble/surface/work/count/currentperson'),
      api.get('/jaxrs/program_center/application/list'),
    ])
    const pc = p.status === 'fulfilled' ? ((p as any).value?.data?.count ?? '--') : '--'
    const ac = a.status === 'fulfilled' ? ((a as any).value?.data?.length ?? '--') : '--'
    stats.value = [
      { label: '待办事项', value: pc, color: 'var(--color-warning)' },
      { label: '应用总数', value: ac, color: 'var(--color-info)' },
      { label: '在线用户', value: '—', color: 'var(--color-success)' },
      { label: '系统状态', value: '正常', color: 'var(--color-primary)' },
    ]
    if (a.status === 'fulfilled') {
      const list = (a as any).value?.data ?? []
      apps.value = list.map((x: any) => ({
        id: x.id ?? x.flag,
        name: x.name ?? x.appName ?? '未命名',
        flag: x.flag ?? '',
      }))
    }
  } catch {
    /* silent */
  }
}
onMounted(loadStats)
</script>
<style scoped>
.nav-view{display:flex;flex-direction:column;gap:16px;height:100%}
.view-header{display:flex;align-items:center;justify-content:space-between;padding:16px 24px}
.view-header h1{font-family:'Orbitron',sans-serif;font-size:20px;color:var(--color-primary);margin:0 0 4px}
.subtitle{font-size:12px;color:var(--text-muted);margin:0}
.refresh-btn{padding:6px 14px;background:transparent;border:1px solid var(--border-subtle);border-radius:var(--radius-md);cursor:pointer;color:var(--text-secondary);font-size:12px}
.stats-row{display:grid;grid-template-columns:repeat(4,1fr);gap:12px;padding:16px}
.stat-card{text-align:center;padding:12px}
.stat-num{font-family:'Orbitron',sans-serif;font-size:28px;font-weight:700;margin-bottom:4px}
.stat-label{font-size:12px;color:var(--text-muted)}
.content-panel{padding:16px}
.panel-title{font-size:14px;font-weight:600;color:var(--text-primary);margin:0 0 12px}
.nav-grid{display:grid;grid-template-columns:repeat(auto-fill,minmax(160px,1fr));gap:12px}
.nav-card{display:flex;flex-direction:column;align-items:center;padding:20px;border-radius:var(--radius-md);border:1px solid var(--border-subtle);background:var(--bg-elevated);text-decoration:none;color:inherit;transition:all var(--transition-fast);cursor:pointer}
.nav-card:hover{border-color:var(--color-primary);transform:translateY(-2px)}
.nc-icon{font-size:28px;margin-bottom:6px}
.nc-title{font-size:13px;font-weight:600;color:var(--text-primary)}
.nc-desc{font-size:11px;color:var(--text-muted);margin-top:4px}
.app-list{display:flex;flex-direction:column;gap:6px}
.app-item{display:flex;align-items:center;gap:10px;padding:10px 12px;border-radius:var(--radius-md);border:1px solid var(--border-subtle);background:var(--bg-elevated);text-decoration:none;color:inherit;transition:all var(--transition-fast)}
.app-item:hover{border-color:var(--color-primary)}
.app-icon{font-size:18px}
.app-name{font-size:13px;font-weight:500;color:var(--text-primary);flex:1}
.app-flag{font-size:11px;color:var(--text-muted);font-family:'JetBrains Mono',monospace}
@media(max-width:768px){.stats-row{grid-template-columns:repeat(2,1fr)}.nav-grid{grid-template-columns:repeat(2,1fr)}}
</style>
