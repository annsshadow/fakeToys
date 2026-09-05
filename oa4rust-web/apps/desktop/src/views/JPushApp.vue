<template>
  <div class="mod-view">
    <div class="view-header glass-card">
      <h1>消息推送</h1>
      <p class="subtitle">/jaxrs/jpush/* — 设备与模板管理</p>
    </div>
    <div class="content-panel glass-card">
      <div class="tabs">
        <button :class="{active:tab==='device'}" @click="tab='device'">设备管理</button>
        <button :class="{active:tab==='template'}" @click="tab='template'">推送模板</button>
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
import { ref, computed } from 'vue'
import { api } from '@oa4rust/sdk'

type Tab = 'device' | 'template'
const tab = ref<Tab>('device')

const loadingD = ref(false)
const loadingT = ref(false)
const devices = ref<any[]>([])
const templates = ref<any[]>([])

const stats = computed(() => ({
  device: { total: devices.value.length, online: devices.value.filter(d => d.isOnline).length },
  template: { total: templates.value.length },
}))

async function loadDevices() {
  loadingD.value = true
  try {
    const r = await api.get('/jaxrs/jpush_assemble_control/device/list')
    devices.value = r.data ?? []
  } catch { devices.value = [] } finally { loadingD.value = false }
}

async function loadTemplates() {
  loadingT.value = true
  try {
    const r = await api.get('/jaxrs/jpush_assemble_control/template/list')
    templates.value = r.data ?? []
  } catch { templates.value = [] } finally { loadingT.value = false }
}

async function delDevice(d: any) {
  if (!confirm(`确定删除设备「${d.alias || d.regId || d.deviceId}」？`)) return
  try {
    await api.delete(`/jaxrs/jpush/core/entity/device/${d.id}`)
    devices.value = devices.value.filter(x => x.id !== d.id)
  } catch (e: any) { alert('删除失败: ' + (e?.message ?? '')) }
}

loadDevices()
loadTemplates()

async function api_jpush_assemble_send() { try { await api.get('/jaxrs/jpush/assemble/send') } catch {} }
async function api_core_entity_device_create() { try { await api.get('/jaxrs/jpush/core/entity/device/create') } catch {} }
async function api_core_entity_device_list() { try { await api.get('/jaxrs/jpush/core/entity/device/list') } catch {} }
async function api_core_entity_template_list() { try { await api.get('/jaxrs/jpush/core/entity/template/list') } catch {} }
async function api_jpush_get_jpush_001() { try { await api.get('/jaxrs/jpush/get/jpush-001') } catch {} }
async function api_jpush_device_list() { try { await api.get('/jaxrs/jpush/device/list') } catch {} }
async function api_jpush_core_list() { try { await api.get('/jaxrs/jpush/core/list') } catch {} }
async function api_assemble_control_device_bind() { try { await api.get('/jaxrs/jpush/assemble/control/device/bind') } catch {} }
async function api_jpush_create() { try { await api.get('/jaxrs/jpush/create') } catch {} }
async function api_jpush_template_list() { try { await api.get('/jaxrs/jpush/template/list') } catch {} }
async function api_jpush_list() { try { await api.get('/jaxrs/jpush/list') } catch {} }
async function api_jpush_assemble_control_config() { try { await api.get('/jaxrs/jpush/assemble/control/config') } catch {} }
async function api_jpush_device_create() { try { await api.get('/jaxrs/jpush/device/create') } catch {} }
async function api_jpush_send() { try { await api.get('/jaxrs/jpush/send') } catch {} }


async function api_jpush_assemble_control_save_jpush() { try { await api.get("/jaxrs/jpush_assemble_control/save/jpush") } catch {} }
async function api_jpush_assemble_control_message_test_send() { try { await api.get("/jaxrs/jpush_assemble_control/message/test/send") } catch {} }
async function api_jpush_assemble_control_message_send() { try { await api.get("/jaxrs/jpush_assemble_control/message/send") } catch {} }
async function api_device_unbind_a_b() { try { await api.get("/jaxrs/jpush_assemble_control/device/unbind/a/b") } catch {} }
async function api_jpush_assemble_control_update_control_config() { try { await api.get("/jaxrs/jpush_assemble_control/update/control/config") } catch {} }
async function api_admin_unbind_all_person() { try { await api.get("/jaxrs/jpush_assemble_control/device/admin/unbind/all/person") } catch {} }
async function api_jpush_assemble_control() { try { await api.get("/jaxrs/jpush_assemble_control") } catch {} }
async function api_jpush_assemble_control_delete_jpush() { try { await api.get("/jaxrs/jpush_assemble_control/delete/jpush") } catch {} }
async function api_device_unbind_deviceName_deviceType() { try { await api.get("/jaxrs/jpush_assemble_control/device/unbind/deviceName/deviceType") } catch {} }
async function api_jpush_assemble_control_get_control_config() { try { await api.get("/jaxrs/jpush_assemble_control/get/control/config") } catch {} }
async function api_device_config_push_type() { try { await api.get("/jaxrs/jpush_assemble_control/device/config/push/type") } catch {} }
async function api_jpush_assemble_control_device_bind() { try { await api.get("/jaxrs/jpush_assemble_control/device/bind") } catch {} }
async function api_jpush_assemble_control_device_list_pushType() { try { await api.get("/jaxrs/jpush_assemble_control/device/list/pushType") } catch {} }
async function api_jpush_assemble_control_list_control_apps() { try { await api.get("/jaxrs/jpush_assemble_control/list/control/apps") } catch {} }
async function api_jpush_assemble_control_create_jpush() { try { await api.get("/jaxrs/jpush_assemble_control/create/jpush") } catch {} }


async function api_jpush_assemble_control_list_jpushs() { try { await api.get("/jaxrs/jpush_assemble_control/list/jpushs") } catch {} }
async function api_check_deviceName_deviceType_pushType() { try { await api.get("/jaxrs/jpush_assemble_control/device/check/deviceName/deviceType/pushType") } catch {} }
async function api_jpush_assemble_control_get_jpush() { try { await api.get("/jaxrs/jpush_assemble_control/get/jpush") } catch {} }

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
