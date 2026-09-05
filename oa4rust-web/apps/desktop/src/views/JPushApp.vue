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
  if (!confirmMsg(`确定删除设备「${d.alias || d.regId || d.deviceId}」？`)) return
  try {
    await api.delete(`/jaxrs/jpush/core/entity/device/${d.id}`)
    devices.value = devices.value.filter(x => x.id !== d.id)
  } catch (e: any) { toast.error('删除失败: : ' + (e?.message ?? '')) }
}

loadDevices()
loadTemplates()

const jpush_assemble_send_ref = ref<any[]>([]);
const jpush_assemble_send_q = useQuery({
  queryKey: ['jpush_assemble_send'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/jpush/assemble/send"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const core_entity_device_create_ref = ref<any[]>([]);
const core_entity_device_create_q = useQuery({
  queryKey: ['core_entity_device_create'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/jpush/core/entity/device/create"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const core_entity_device_list_ref = ref<any[]>([]);
const core_entity_device_list_q = useQuery({
  queryKey: ['core_entity_device_list'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/jpush/core/entity/device/list"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const core_entity_template_list_ref = ref<any[]>([]);
const core_entity_template_list_q = useQuery({
  queryKey: ['core_entity_template_list'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/jpush/core/entity/template/list"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const jpush_get_jpush_001_ref = ref<any[]>([]);
const jpush_get_jpush_001_q = useQuery({
  queryKey: ['jpush_get_jpush_001'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/jpush/get/jpush-001"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const jpush_device_list_ref = ref<any[]>([]);
const jpush_device_list_q = useQuery({
  queryKey: ['jpush_device_list'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/jpush/device/list"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const jpush_core_list_ref = ref<any[]>([]);
const jpush_core_list_q = useQuery({
  queryKey: ['jpush_core_list'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/jpush/core/list"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const assemble_control_device_bind_ref = ref<any[]>([]);
const assemble_control_device_bind_q = useQuery({
  queryKey: ['assemble_control_device_bind'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/jpush/assemble/control/device/bind"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const jpush_create_ref = ref<any[]>([]);
const jpush_create_q = useQuery({
  queryKey: ['jpush_create'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/jpush/create"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const jpush_template_list_ref = ref<any[]>([]);
const jpush_template_list_q = useQuery({
  queryKey: ['jpush_template_list'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/jpush/template/list"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const jpush_list_ref = ref<any[]>([]);
const jpush_list_q = useQuery({
  queryKey: ['jpush_list'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/jpush/list"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const jpush_assemble_control_config_ref = ref<any[]>([]);
const jpush_assemble_control_config_q = useQuery({
  queryKey: ['jpush_assemble_control_config'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/jpush/assemble/control/config"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const jpush_device_create_ref = ref<any[]>([]);
const jpush_device_create_q = useQuery({
  queryKey: ['jpush_device_create'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/jpush/device/create"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const jpush_send_ref = ref<any[]>([]);
const jpush_send_q = useQuery({
  queryKey: ['jpush_send'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/jpush/send"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});


const api_jpush_as_362_data = ref<any[]>([]);
const { data: api_jpush_as_362_q } = useQuery({queryKey: ['api_jpush_as_362', '/jaxrs/jpush_assemble_control/save/jpush'], queryFn: async () => { try { const r = await api.get("/jaxrs/jpush_assemble_control/save/jpush"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jpush_as_362_q, (v) => { api_jpush_as_362_data.value = v ?? []; });
const jpush_assemble_control_message_test_send_ref = ref<any[]>([]);
const jpush_assemble_control_message_test_send_q = useQuery({
  queryKey: ['jpush_assemble_control_message_test_send'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/jpush_assemble_control/message/test/send"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const api_jpush_as_598_data = ref<any[]>([]);
const { data: api_jpush_as_598_q } = useQuery({queryKey: ['api_jpush_as_598', '/jaxrs/jpush_assemble_control/message/send'], queryFn: async () => { try { const r = await api.get("/jaxrs/jpush_assemble_control/message/send"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jpush_as_598_q, (v) => { api_jpush_as_598_data.value = v ?? []; });
const api_device_u_56_data = ref<any[]>([]);
const { data: api_device_u_56_q } = useQuery({queryKey: ['api_device_u_56', '/jaxrs/jpush_assemble_control/device/unbind/a/b'], queryFn: async () => { try { const r = await api.get("/jaxrs/jpush_assemble_control/device/unbind/a/b"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_device_u_56_q, (v) => { api_device_u_56_data.value = v ?? []; });
const api_jpush_as_458_data = ref<any[]>([]);
const { data: api_jpush_as_458_q } = useQuery({queryKey: ['api_jpush_as_458', '/jaxrs/jpush_assemble_control/update/control/config'], queryFn: async () => { try { const r = await api.get("/jaxrs/jpush_assemble_control/update/control/config"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jpush_as_458_q, (v) => { api_jpush_as_458_data.value = v ?? []; });
const api_admin_un_510_data = ref<any[]>([]);
const { data: api_admin_un_510_q } = useQuery({queryKey: ['api_admin_un_510', '/jaxrs/jpush_assemble_control/device/admin/unbind/all/person'], queryFn: async () => { try { const r = await api.get("/jaxrs/jpush_assemble_control/device/admin/unbind/all/person"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_admin_un_510_q, (v) => { api_admin_un_510_data.value = v ?? []; });
const api_jpush_as_890_data = ref<any[]>([]);
const { data: api_jpush_as_890_q } = useQuery({queryKey: ['api_jpush_as_890', '/jaxrs/jpush_assemble_control'], queryFn: async () => { try { const r = await api.get("/jaxrs/jpush_assemble_control"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jpush_as_890_q, (v) => { api_jpush_as_890_data.value = v ?? []; });
const api_jpush_as_394_data = ref<any[]>([]);
const { data: api_jpush_as_394_q } = useQuery({queryKey: ['api_jpush_as_394', '/jaxrs/jpush_assemble_control/delete/jpush'], queryFn: async () => { try { const r = await api.get("/jaxrs/jpush_assemble_control/delete/jpush"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jpush_as_394_q, (v) => { api_jpush_as_394_data.value = v ?? []; });
const api_device_u_877_data = ref<any[]>([]);
const { data: api_device_u_877_q } = useQuery({queryKey: ['api_device_u_877', '/jaxrs/jpush_assemble_control/device/unbind/deviceName/deviceType'], queryFn: async () => { try { const r = await api.get("/jaxrs/jpush_assemble_control/device/unbind/deviceName/deviceType"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_device_u_877_q, (v) => { api_device_u_877_data.value = v ?? []; });
const api_jpush_as_398_data = ref<any[]>([]);
const { data: api_jpush_as_398_q } = useQuery({queryKey: ['api_jpush_as_398', '/jaxrs/jpush_assemble_control/get/control/config'], queryFn: async () => { try { const r = await api.get("/jaxrs/jpush_assemble_control/get/control/config"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jpush_as_398_q, (v) => { api_jpush_as_398_data.value = v ?? []; });
const api_device_c_516_data = ref<any[]>([]);
const { data: api_device_c_516_q } = useQuery({queryKey: ['api_device_c_516', '/jaxrs/jpush_assemble_control/device/config/push/type'], queryFn: async () => { try { const r = await api.get("/jaxrs/jpush_assemble_control/device/config/push/type"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_device_c_516_q, (v) => { api_device_c_516_data.value = v ?? []; });
const api_jpush_as_148_data = ref<any[]>([]);
const { data: api_jpush_as_148_q } = useQuery({queryKey: ['api_jpush_as_148', '/jaxrs/jpush_assemble_control/device/bind'], queryFn: async () => { try { const r = await api.get("/jaxrs/jpush_assemble_control/device/bind"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jpush_as_148_q, (v) => { api_jpush_as_148_data.value = v ?? []; });
const api_jpush_as_641_data = ref<any[]>([]);
const { data: api_jpush_as_641_q } = useQuery({queryKey: ['api_jpush_as_641', '/jaxrs/jpush_assemble_control/device/list/pushType'], queryFn: async () => { try { const r = await api.get("/jaxrs/jpush_assemble_control/device/list/pushType"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jpush_as_641_q, (v) => { api_jpush_as_641_data.value = v ?? []; });
const api_jpush_as_145_data = ref<any[]>([]);
const { data: api_jpush_as_145_q } = useQuery({queryKey: ['api_jpush_as_145', '/jaxrs/jpush_assemble_control/list/control/apps'], queryFn: async () => { try { const r = await api.get("/jaxrs/jpush_assemble_control/list/control/apps"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jpush_as_145_q, (v) => { api_jpush_as_145_data.value = v ?? []; });
const api_jpush_as_45_data = ref<any[]>([]);
const { data: api_jpush_as_45_q } = useQuery({queryKey: ['api_jpush_as_45', '/jaxrs/jpush_assemble_control/create/jpush'], queryFn: async () => { try { const r = await api.get("/jaxrs/jpush_assemble_control/create/jpush"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jpush_as_45_q, (v) => { api_jpush_as_45_data.value = v ?? []; });


const api_jpush_as_446_data = ref<any[]>([]);
const { data: api_jpush_as_446_q } = useQuery({queryKey: ['api_jpush_as_446', '/jaxrs/jpush_assemble_control/list/jpushs'], queryFn: async () => { try { const r = await api.get("/jaxrs/jpush_assemble_control/list/jpushs"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jpush_as_446_q, (v) => { api_jpush_as_446_data.value = v ?? []; });
const api_check_de_819_data = ref<any[]>([]);
const { data: api_check_de_819_q } = useQuery({queryKey: ['api_check_de_819', '/jaxrs/jpush_assemble_control/device/check/deviceName/deviceType/pushType'], queryFn: async () => { try { const r = await api.get("/jaxrs/jpush_assemble_control/device/check/deviceName/deviceType/pushType"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_check_de_819_q, (v) => { api_check_de_819_data.value = v ?? []; });
const api_jpush_as_867_data = ref<any[]>([]);
const { data: api_jpush_as_867_q } = useQuery({queryKey: ['api_jpush_as_867', '/jaxrs/jpush_assemble_control/get/jpush'], queryFn: async () => { try { const r = await api.get("/jaxrs/jpush_assemble_control/get/jpush"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jpush_as_867_q, (v) => { api_jpush_as_867_data.value = v ?? []; });


const api_control_device_l_732_data = ref<any[]>([]);
const { data: api_control_device_l_732_q } = useQuery({queryKey: ['api_control_device_l_732', '/jaxrs/jpush/assemble/control/device/list/pushType'], queryFn: async () => { try { const r = await api.get("/jaxrs/jpush/assemble/control/device/list/pushType"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_control_device_l_732_q, (v) => { api_control_device_l_732_data.value = v ?? []; });
const api_control_list_con_510_data = ref<any[]>([]);
const { data: api_control_list_con_510_q } = useQuery({queryKey: ['api_control_list_con_510', '/jaxrs/jpush/assemble/control/list/control/apps'], queryFn: async () => { try { const r = await api.get("/jaxrs/jpush/assemble/control/list/control/apps"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_control_list_con_510_q, (v) => { api_control_list_con_510_data.value = v ?? []; });
const api_control_update_c_655_data = ref<any[]>([]);
const { data: api_control_update_c_655_q } = useQuery({queryKey: ['api_control_update_c_655', '/jaxrs/jpush/assemble/control/update/control/config'], queryFn: async () => { try { const r = await api.get("/jaxrs/jpush/assemble/control/update/control/config"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_control_update_c_655_q, (v) => { api_control_update_c_655_data.value = v ?? []; });
const control_message_test_send_ref = ref<any[]>([]);
const control_message_test_send_q = useQuery({
  queryKey: ['control_message_test_send'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/jpush/assemble/control/message/test/send"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});

const api_jaxrs_jpush_asse_42_data = ref<any[]>([]);
const { data: api_jaxrs_jpush_asse_42_q } = useQuery({queryKey: ['api_jaxrs_jpush_asse_42', '/jaxrs/jpush/assemble/control/device/admin/unbind/all/person'], queryFn: async () => { try { const r = await api.get("/jaxrs/jpush/assemble/control/device/admin/unbind/all/person"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_jpush_asse_42_q, (v) => { api_jaxrs_jpush_asse_42_data.value = v ?? []; });
const api_jaxrs_jpush_asse_288_data = ref<any[]>([]);
const { data: api_jaxrs_jpush_asse_288_q } = useQuery({queryKey: ['api_jaxrs_jpush_asse_288', '/jaxrs/jpush/assemble/control/device/check/deviceName/deviceType/pushType'], queryFn: async () => { try { const r = await api.get("/jaxrs/jpush/assemble/control/device/check/deviceName/deviceType/pushType"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_jpush_asse_288_q, (v) => { api_jaxrs_jpush_asse_288_data.value = v ?? []; });
const api_jaxrs_jpush_asse_429_data = ref<any[]>([]);
const { data: api_jaxrs_jpush_asse_429_q } = useQuery({queryKey: ['api_jaxrs_jpush_asse_429', '/jaxrs/jpush/assemble/control/device/config/push/type'], queryFn: async () => { try { const r = await api.get("/jaxrs/jpush/assemble/control/device/config/push/type"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_jpush_asse_429_q, (v) => { api_jaxrs_jpush_asse_429_data.value = v ?? []; });
const api_jaxrs_jpush_asse_313_data = ref<any[]>([]);
const { data: api_jaxrs_jpush_asse_313_q } = useQuery({queryKey: ['api_jaxrs_jpush_asse_313', '/jaxrs/jpush/assemble/control/device/unbind/deviceName/deviceType'], queryFn: async () => { try { const r = await api.get("/jaxrs/jpush/assemble/control/device/unbind/deviceName/deviceType"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_jpush_asse_313_q, (v) => { api_jaxrs_jpush_asse_313_data.value = v ?? []; });
const api_jaxrs_jpush_asse_984_data = ref<any[]>([]);
const { data: api_jaxrs_jpush_asse_984_q } = useQuery({queryKey: ['api_jaxrs_jpush_asse_984', '/jaxrs/jpush/assemble/control/device/unbind/new/deviceName/deviceType/pushType'], queryFn: async () => { try { const r = await api.get("/jaxrs/jpush/assemble/control/device/unbind/new/deviceName/deviceType/pushType"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_jpush_asse_984_q, (v) => { api_jaxrs_jpush_asse_984_data.value = v ?? []; });
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
