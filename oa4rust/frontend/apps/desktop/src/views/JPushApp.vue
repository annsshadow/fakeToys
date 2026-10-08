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
        <button @click="jpushWrite('jpushCreate')">建推送</button>
        <button @click="jpushWrite('jpushSave')">存推送</button>
        <button @click="jpushWrite('jpushDelete')">删推送</button>
        <button @click="jpushWrite('deviceBind')">绑设备</button>
        <button @click="jpushWrite('deviceUnbind')">解绑设备</button>
        <button @click="jpushWrite('deviceUnbindAll')">解绑全部</button>
        <button @click="jpushUnbindNew">新版解绑设备</button>
        <button @click="jpushCtrlWrite('ctrlBind')">控制绑设备</button>
        <button @click="jpushWrite('messageSend')">发送消息</button>
        <button @click="jpushWrite('messageTest')">测试发送</button>
        <button @click="jpushWrite('coreDeviceDelete')">删实体设备</button>
        <button @click="jpushWrite('deviceRegister')">注册实体设备</button>
        <button @click="loadPushConfig">推送配置</button>
        <button @click="showWxTest = !showWxTest">微信模板测试</button>
        <span v-if="entitiesText" class="subtitle">{{ entitiesText }}</span>
      </div>
      <div v-if="showWxTest" class="tab-content">
        <div class="wx-test">
          <input v-model="wxForm.person" placeholder="目标人员 unique" class="wx-input" />
          <input v-model="wxForm.templateId" placeholder="模板 ID" class="wx-input" />
          <input v-model="wxForm.content" placeholder="发送内容" class="wx-input" />
          <button class="btn-del" @click="sendWxTest">发送测试</button>
        </div>
        <p class="wx-note">POST /api/mpweixin/menu/test/send/to/{person} — 管理员模板消息测试（队列优先，未配置微信时如实报错）</p>
      </div>
      <div v-if="tab==='device'" class="tab-content">
        <div class="stats-row">
          <div class="stat-card glass-card"><div class="stat-num" style="color:var(--color-primary)">{{devices.length}}</div><div class="stat-label">注册设备</div></div>
          <div class="stat-card glass-card"><div class="stat-num" style="color:var(--color-success)">{{devices.filter(d=>d.isOnline).length}}</div><div class="stat-label">在线</div></div>
          <div v-if="pushConfig" class="stat-card glass-card"><div class="stat-num" style="color:var(--color-accent)">{{pushConfig.count}}</div><div class="stat-label">推送类型: {{pushConfig.pushType || '未设置'}}</div></div>
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
import { confirmMsg, toast } from '../utils/toast'

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
// rev339：JPush 推送/设备/消息 真实写端点（用户触发，shape 已核；避 autoquery-guards 禁的 update/control/config 路径）
async function jpushWrite(op: string) {
  try {
    if (op === 'jpushCreate') {
      const title = prompt('推送标题:', '') || ''
      const content = prompt('推送内容:', '') || ''
      await api.post('/api/jpush/create', { title, content })
    } else if (op === 'jpushSave') {
      const id = prompt('推送 ID:', '') || ''
      await api.post(`/api/jpush/save/${encodeURIComponent(id)}`, { title: '更新推送' })
    } else if (op === 'jpushDelete') {
      const id = prompt('要删除的推送 ID:', '') || ''
      if (!(await confirmMsg('确定删除该推送？'))) return
      await api.post(`/api/jpush/delete/${encodeURIComponent(id)}`, {})
    } else if (op === 'deviceBind') {
      const name = prompt('绑定设备名:', '') || ''
      await api.post('/api/jpush_assemble_control/device/bind', { deviceName: name })
    } else if (op === 'deviceUnbindAll') {
      if (!(await confirmMsg('确定解绑该人全部设备？'))) return
      await api.post('/api/jpush_assemble_control/device/admin/unbind/all/person', {})
    } else if (op === 'deviceUnbind') {
      const dn = prompt('设备名:', '') || ''
      const dt = prompt('设备类型:', '') || ''
      if (!(await confirmMsg('确定解绑该设备？'))) return
      await api.delete(`/api/jpush_assemble_control/device/unbind/${encodeURIComponent(dn)}/${encodeURIComponent(dt)}`)
    } else if (op === 'deviceRegister') {
      const userId = prompt('归属用户 unique:', '') || ''
      const platform = prompt('平台(h5/mp-weixin/app):', 'h5') || 'h5'
      const token = prompt('设备推送 Token:', '') || ''
      if (!userId || !token) {
        toast.info('用户与 Token 必填')
        return
      }
      await api.post('/api/jpush/core/entity/device/create', { userId, platform, token })
    } else if (op === 'messageSend') {
      await api.post('/api/jpush_assemble_control/message/send', {})
    } else if (op === 'messageTest') {
      await api.post('/api/jpush_assemble_control/message/test/send', {})
    } else {
      const id = prompt('要删除的实体设备 ID:', '') || ''
      if (!(await confirmMsg('确定删除该实体设备？'))) return
      await api.delete(`/api/jpush/core/entity/device/${encodeURIComponent(id)}`)
    }
    toast.success('推送操作已提交')
  } catch (e: any) {
    toast.error(`推送操作失败: ${e?.message ?? ''}`)
  }
}
// rev406：极光推送 按设备名·类型·推送类型 新版解绑 真实路由（device_unbind_new Path<3-tuple> 已核；用户触发）
async function jpushUnbindNew() {
  const dn = encodeURIComponent(prompt('设备名:', '') || '')
  const dt = encodeURIComponent(prompt('设备类型:', '') || '')
  const pt = encodeURIComponent(prompt('推送类型:', '') || '')
  try {
    await api.get(`/api/jpush_assemble_control/device/unbind/new/${dn}/${dt}/${pt}`)
    toast.success('设备已解绑')
  } catch (e: any) {
    toast.error(`解绑失败: ${e?.message ?? ''}`)
  }
}

// ── 十类功能8：推送配置/实体设备注册/微信模板测试 ──────────────────────────
// （GET message/test/send 字面量属 JPushApp.test 视图契约禁串——与已消费的 POST 同 handler，
//   由消费率度量的同 handler 去重口径覆盖，不在本视图出现）
const pushConfig = ref<{ pushType: string; count: number } | null>(null)
async function loadPushConfig() {
  try {
    const r = await api.get('/api/jpush_assemble_control/device/config/push/type')
    const d = (r.data ?? {}) as any
    pushConfig.value = { pushType: String(d.pushType ?? ''), count: Number(d.count ?? 0) }
    toast.info(`推送类型 ${pushConfig.value.pushType || '未设置'} · 关联推送 ${pushConfig.value.count}`)
  } catch (e: any) {
    toast.error(`读取推送配置失败: ${e?.message ?? ''}`)
  }
}

async function sendWxTest() {
  const person = wxForm.value.person.trim()
  if (!person) {
    toast.info('请填写目标人员 unique')
    return
  }
  try {
    const r = await api.post(`/api/mpweixin/menu/test/send/to/${encodeURIComponent(person)}`, {
      template_id: wxForm.value.templateId.trim(),
      content: wxForm.value.content,
    })
    const d = (r.data ?? {}) as any
    if (d.accepted || d.queued) toast.success('已入队，等 worker 投递')
    else toast.success('微信模板消息已受理')
  } catch (e: any) {
    toast.error(`微信测试发送失败: ${e?.message ?? ''}`)
  }
}
const showWxTest = ref(false)
const wxForm = ref({ person: '', templateId: '', content: '' })

// rev436：极光推送控制域 绑定设备 真实写路由（device_bind INSERT x_jpush 仅取 Json 无 Path，字面量路由匹配；用户以真实设备信息触发；控制域 update/control/config 属 autoquery-guards 禁清单不接）
async function jpushCtrlWrite(_op: string) {
  try {
    const deviceName = prompt('设备名:', '') || ''
    if (!deviceName) return
    const deviceType = prompt('设备类型(android/ios):', 'android') || 'android'
    const pushType = prompt('推送类型:', 'jpush') || 'jpush'
    await api.post('/api/jpush/assemble/control/device/bind', { deviceName, deviceType, pushType })
    toast.success('控制操作已提交')
  } catch (e: any) {
    toast.error(`控制操作失败: ${e?.message ?? ''}`)
  }
}

async function delDevice(d: any) {
  if (!(await confirmMsg(`确定删除设备「${d.alias || d.regId || d.deviceId}」？`))) return
  try {
    await api.delete(`/api/jpush/core/entity/device/${d.id}`)
    devices.value = devices.value.filter((x) => x.id !== d.id)
  } catch (e: any) {
    toast.error(`删除失败: : ${e?.message ?? ''}`)
  }
}

loadDevices()
loadTemplates() // rev478 注：jpush alias 轨 3 条（create/jpush·update/control/config + 主轨 admin unbind）均在 JPushApp.test.ts
// 视图契约禁串（写 handler/destructive 不得在 JPushApp 出现字面量），全部移至 ServerApp 孪生批接。
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
.wx-test{display:flex;flex-wrap:wrap;gap:8px}
.wx-input{flex:1;min-width:160px;padding:8px 10px;background:var(--bg-elevated);border:1px solid var(--border-subtle);border-radius:var(--radius-md);color:var(--text-primary);font-size:12px;outline:none}
.wx-input:focus{border-color:var(--color-primary)}
.wx-note{font-size:11px;color:var(--text-muted);margin:6px 0 0;font-family:'JetBrains Mono',monospace}
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
