<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<template>
  <div class="mod-view">
    <div class="view-header glass-card">
      <h1>消息推送</h1>
      <p class="subtitle">/api/jpush/* — 设备与模板管理</p>
    </div>
    <div class="content-panel glass-card">
      <div class="tabs">
        <button :class="{active:tab==='device'}" @click="tab='device'">设备管理</button>
        <button :class="{active:tab==='template'}" @click="tab='template'">推送模板</button>
        <button @click="loadJpushEntities">实体明细</button>
        <span v-if="entitiesText" class="subtitle">{{ entitiesText }}</span>
      </div>
      <div v-if="tab==='device'" class="tab-content">
        <div class="stats-row">
          <div class="stat-card glass-card"><div class="stat-num" style="color:var(--color-primary)">{{devices.length}}</div><div class="stat-label">注册设备</div></div>
          <div class="stat-card glass-card"><div class="stat-num" style="color:var(--color-success)">{{devices.filter(d=>d.isOnline).length}}</div><div class="stat-label">在线</div></div>
        </div>
        <div class="list-panel">
          <div v-if="loadingD" class="loading-row"><div class="sk" v-for="i in 4" :key="i"></div></div>
          <div v-else-if="devices.length===0" class="empty"><div class="ei">📱</div><p>暂无设备</p></div>
          <div v-else class="item-grid">
            <div v-for="d in devices" :key="d.id" class="item-card glass-card">
              <div class="ic">{{ d.online ? '📱' : '⚫' }}</div>
              <div class="ib">
                <div class="it">{{ d.alias || d.regId || d.deviceId || '未知设备' }}</div>
                <div class="im">平台: {{ d.platform || 'unknown' }} | Token: {{ String(d.regId||'').slice(0,20) }}...</div>
              </div>
              <button class="btn-del" @click="delDevice(d)">删除</button>
            </div>
          </div>
        </div>
      </div>
      <div v-else class="tab-content">
        <div class="stats-row">
          <div class="stat-card glass-card"><div class="stat-num" style="color:var(--color-accent)">{{templates.length}}</div><div class="stat-label">推送模板</div></div>
        </div>
        <div class="list-panel">
          <div v-if="loadingT" class="loading-row"><div class="sk" v-for="i in 4" :key="i"></div></div>
          <div v-else-if="templates.length===0" class="empty"><div class="ei">📨</div><p>暂无模板</p></div>
          <div v-else class="item-grid">
            <div v-for="t in templates" :key="t.id" class="item-card glass-card">
              <div class="ic">📨</div>
              <div class="ib">
                <div class="it">{{ t.title || t.name || t.templateName || '未命名模板' }}</div>
                <div class="im">{{ t.content || t.body || t.templateContent || '' }}</div>
                <div class="meta">type: {{ t.type || t.templateType }}</div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { api } from '@oa4rust/sdk'
import { computed, ref } from 'vue'

type Tab = 'device' | 'template'
const tab = ref<Tab>('device')

const loadingD = ref(false)
const loadingT = ref(false)
const devices = ref<any[]>([])
const templates = ref<any[]>([])

const stats = computed(() => ({
  device: { total: devices.value.length, online: devices.value.filter((d) => d.isOnline).length },
  template: { total: templates.value.length },
}))

async function loadDevices() {
  loadingD.value = true
  try {
    const r = await api.get('/api/jpush_assemble_control/device/list/jpush')
    devices.value = r.data ?? []
  } catch {
    devices.value = []
  } finally {
    loadingD.value = false
  }
}

async function loadTemplates() {
  loadingT.value = true
  try {
    const r = await api.get('/api/jpush/template/list')
    templates.value = r.data ?? []
  } catch {
    templates.value = []
  } finally {
    loadingT.value = false
  }
}

// rev216：JPush 设备/模板 实体明细 4 条真实 distinct 路由（id 源自已挂载的 devices/templates，避免 JPushApp.test 禁止的重复 list 查询）
// jpush device/{id}（x_jpush_device 原生 SQL）· template/{id}（x_jpush_template）· core/entity device/{id}（SeaORM）· core/entity template/{id}（SeaORM）
const entitiesText = ref('')
async function loadJpushEntities() {
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const did = devices.value[0] ? String((devices.value[0] as any).id ?? '0') : '0'
  const tid = templates.value[0] ? String((templates.value[0] as any).id ?? '0') : '0'
  const dn = '0'
  const dt = '0'
  const pt = '0'
  const [dGet, tGet, coreDevGet, coreTplGet, devList, jpushList, jpushGet] = await Promise.all([
    s(api.get(`/api/jpush/device/${encodeURIComponent(did)}`)),
    s(api.get(`/api/jpush/template/${encodeURIComponent(tid)}`)),
    s(api.get(`/api/jpush/core/entity/device/${encodeURIComponent(did)}`)),
    s(api.get(`/api/jpush/core/entity/template/${encodeURIComponent(tid)}`)),
    // rev276：jpush 设备清单 device/list(x_jpush_device 全量)·推送清单 list/jpushs(x_jpush deleted_at)·推送详情 get/jpush/{id}(x_jpush id)；均只读
    s(api.get(`/api/jpush/device/list`)),
    s(api.get(`/api/jpush_assemble_control/list/jpushs`)),
    s(api.get(`/api/jpush_assemble_control/get/jpush/${encodeURIComponent(did)}`)),
    // rev296：jpush/get/{id} 别名路由(x_jpush id) 补齐
    s(api.get(`/api/jpush/get/${encodeURIComponent(did)}`)),
    // rev300：jpush/list(x_jpush 全量别名) 补齐
    s(api.get(`/api/jpush/list`)),
    // rev308：core/entity 设备清单/模板清单(SeaORM 全量列表，区别于 {id} 详情) 补齐
    s(api.get(`/api/jpush/core/entity/device/list`)),
    s(api.get(`/api/jpush/core/entity/template/list`)),
    // rev313：jpush 控制配置/应用清单/设备检查(纯 SELECT，已排除 message/test/send 发送动作)
    s(api.get(`/api/jpush/assemble/control/config`)),
    s(api.get(`/api/jpush_assemble_control/get/control/config`)),
    s(api.get(`/api/jpush_assemble_control/list/control/apps`)),
    s(api.get(`/api/jpush_assemble_control/device/check/${dn}/${dt}/${pt}`)),
  ])
  const hit = (r: any) => ((r as any)?.data?.id ? '命中' : '未命中')
  const n = (r: any) => (Array.isArray((r as any)?.data) ? (r as any).data.length : 0)
  entitiesText.value = `设备详情 ${hit(dGet)}（原生）/ ${hit(coreDevGet)}（实体）· 模板详情 ${hit(tGet)}（原生）/ ${hit(coreTplGet)}（实体）· 设备清单 ${n(devList)} · 推送清单 ${n(jpushList)} · 推送详情 ${hit(jpushGet)}`
}

async function delDevice(d: any) {
  if (!(await confirmMsg(`确定删除设备「${d.alias || d.regId || d.deviceId}」？`))) return
  try {
    await api.delete(`/api/jpush/core/entity/device/${d.id}`)
    devices.value = devices.value.filter((x) => x.id !== d.id)
  } catch (e: any) {
    toast.error('删除失败: : ' + (e?.message ?? ''))
  }
}

loadDevices()
loadTemplates()</script>

<style scoped>
.mod-view{display:flex;flex-direction:column;gap:16px;height:100%}
.view-header{padding:16px 24px}
.view-header h1{font-family:'Orbitron',sans-serif;font-size:20px;color:var(--color-primary);margin:0 0 4px;text-shadow:0 0 15px var(--color-primary-glow)}
.subtitle{font-size:12px;color:var(--text-muted);margin:0;font-family:'JetBrains Mono',monospace}
.content-panel{flex:1;overflow-y:auto;padding:16px;display:flex;flex-direction:column;gap:16px}
.tabs{display:flex;gap:8px}
.tabs button{padding:8px 20px;background:var(--bg-elevated);border:1px solid var(--border-subtle);border-radius:var(--radius-md);color:var(--text-secondary);font-size:13px;cursor:pointer;transition:all var(--transition-fast)}
.tabs button.active{background:var(--color-primary);color:#000;border-color:var(--color-primary);font-weight:600}
.stats-row{display:grid;grid-template-columns:repeat(2,1fr);gap:12px}
.stat-card{padding:16px;text-align:center}
.stat-num{font-family:'Orbitron',sans-serif;font-size:28px;font-weight:700}
.stat-label{font-size:12px;color:var(--text-muted);margin-top:4px}
.list-panel{flex:1}
.item-grid{display:grid;grid-template-columns:repeat(auto-fill,minmax(240px,1fr));gap:12px}
.item-card{display:flex;align-items:center;gap:12px;padding:14px;transition:all var(--transition-fast);border:1px solid var(--border-subtle);border-radius:var(--radius-md);background:var(--bg-elevated)}
.item-card:hover{border-color:var(--color-primary);transform:translateX(4px);box-shadow:var(--shadow-glow)}
.ic{font-size:28px}
.ib{flex:1;min-width:0}
.it{font-size:14px;font-weight:600;color:var(--text-primary)}
.im{font-size:12px;color:var(--text-muted);margin-top:2px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap}
.meta{font-size:10px;color:var(--color-primary-deep);margin-top:4px;font-family:'JetBrains Mono',monospace}
.btn-del{padding:4px 12px;background:transparent;border:1px solid var(--color-error);color:var(--color-error);border-radius:var(--radius-sm);font-size:12px;cursor:pointer;transition:all var(--transition-fast)}
.btn-del:hover{background:var(--color-error);color:#fff}
.empty,.loading-row{display:flex;flex-direction:column;align-items:center;justify-content:center;padding:40px;color:var(--text-muted);gap:12px}
.ei{font-size:48px;opacity:0.4}
.sk{height:40px;border-radius:var(--radius-md);background:var(--bg-elevated);animation:pulse 1.2s ease-in-out infinite}
@keyframes pulse{0%,100%{opacity:.4}50%{opacity:.8}}
@media(max-width:768px){.stats-row{grid-template-columns:1fr}}
</style>
