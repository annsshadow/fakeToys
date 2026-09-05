<template>
  <div class="mod-view">
    <div class="view-header glass-card">
      <h1>服务器管理</h1>
      <p class="subtitle">/jaxrs/server/* — 命令执行与授权管理</p>
    </div>
    <div class="content-panel glass-card">
      <div class="grid-2col">
        <!-- License -->
        <div class="panel glass-card">
          <h3>授权信息</h3>
          <div v-if="loadingLicense" class="loading-small">加载中...</div>
          <div v-else class="license-info">
            <div v-if="license" v-for="(v,k) in license" :key="k" class="info-row">
              <span class="info-key">{{ k }}</span>
              <span class="info-val">{{ String(v) }}</span>
            </div>
            <div v-else class="empty-license">暂无授权信息</div>
          </div>
        </div>
        <!-- Command -->
        <div class="panel glass-card">
          <h3>执行命令</h3>
          <div class="cmd-input-row">
            <input v-model="command" placeholder="输入服务器命令..." class="cmd-input" @keydown.enter="executeCommand" />
            <button class="btn-execute" :disabled="executing" @click="executeCommand">执行</button>
          </div>
          <div v-if="execOutput" class="cmd-output">
            <pre>{{ execOutput }}</pre>
          </div>
          <div v-if="execError" class="cmd-error">{{ execError }}</div>
        </div>
      </div>
      <!-- Stop server -->
      <div class="danger-zone glass-card">
        <h3 style="color:var(--color-error)">⚠ 危险操作</h3>
        <button class="btn-stop" @click="stopServer">停止服务器</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { api } from '@oa4rust/sdk'

const command = ref('')
const executing = ref(false)
const execOutput = ref('')
const execError = ref('')
const loadingLicense = ref(false)
const license = ref<Record<string, unknown> | null>(null)

async function loadLicense() {
  loadingLicense.value = true
  try {
    const r = await api.get('/jaxrs/server/license')
    license.value = r.data ?? null
  } catch { license.value = null } finally { loadingLicense.value = false }
}

async function executeCommand() {
  if (!command.value.trim()) return
  executing.value = true
  execOutput.value = ''
  execError.value = ''
  try {
    const r = await api.post('/jaxrs/server/execute', { command: command.value })
    execOutput.value = JSON.stringify(r.data, null, 2)
  } catch (e: any) {
    execError.value = e?.message ?? '命令执行失败'
  } finally { executing.value = false }
}

async function stopServer() {
  if (!confirmMsg('确定要停止服务器？所有连接将被断开。')) return
  try {
    await api.post('/jaxrs/server/stop', null)
    execOutput.value = '服务器已停止'
  } catch (e: any) {
    execError.value = '停止失败: ' + (e?.message ?? '')
  }
}

loadLicense()

async function api_cache() { try { await api.get("/jaxrs/cache") } catch {} }
const api_cache_co_205_data = ref<any[]>([]);
const { data: api_cache_co_205_q } = useQuery({queryKey: ['api_cache_co_205', '/jaxrs/cache/commonscript/flush'], queryFn: async () => { try { const r = await api.get("/jaxrs/cache/commonscript/flush"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_cache_co_205_q, (v) => { api_cache_co_205_data.value = v ?? []; });
async function api_cache_detail() { try { await api.get("/jaxrs/cache/detail") } catch {} }
const api_cache_co_100_data = ref<any[]>([]);
const { data: api_cache_co_100_q } = useQuery({queryKey: ['api_cache_co_100', '/jaxrs/cache/config/flush'], queryFn: async () => { try { const r = await api.get("/jaxrs/cache/config/flush"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_cache_co_100_q, (v) => { api_cache_co_100_data.value = v ?? []; });


const api_console__997_data = ref<any[]>([]);
const { data: api_console__997_q } = useQuery({queryKey: ['api_console__997', '/jaxrs/console/cache/clear/all'], queryFn: async () => { try { const r = await api.get("/jaxrs/console/cache/clear/all"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_console__997_q, (v) => { api_console__997_data.value = v ?? []; });
const api_console__543_data = ref<any[]>([]);
const { data: api_console__543_q } = useQuery({queryKey: ['api_console__543', '/jaxrs/console/logs/error'], queryFn: async () => { try { const r = await api.get("/jaxrs/console/logs/error"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_console__543_q, (v) => { api_console__543_data.value = v ?? []; });
async function api_console_metric_cpu_usage() { try { await api.get("/jaxrs/console/metric/cpu_usage") } catch {} }
async function api_console_system_info() { try { await api.get("/jaxrs/console/system/info") } catch {} }
const api_console__450_data = ref<any[]>([]);
const { data: api_console__450_q } = useQuery({queryKey: ['api_console__450', '/jaxrs/console/command/execute'], queryFn: async () => { try { const r = await api.get("/jaxrs/console/command/execute"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_console__450_q, (v) => { api_console__450_data.value = v ?? []; });
const api_console__982_data = ref<any[]>([]);
const { data: api_console__982_q } = useQuery({queryKey: ['api_console__982', '/jaxrs/console/send/message'], queryFn: async () => { try { const r = await api.get("/jaxrs/console/send/message"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_console__982_q, (v) => { api_console__982_data.value = v ?? []; });
const api_console_status_data = ref<any[]>([]);
const { data: api_console_status_q } = useQuery({queryKey: ['api_console_status', '/jaxrs/console/status'], queryFn: async () => { try { const r = await api.get("/jaxrs/console/status"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_console_status_q, (v) => { api_console_status_data.value = v ?? []; });


async function api_output_o_1_select_mockputtopost() { try { await api.get("/jaxrs/output/o-1/select/mockputtopost") } catch {} }
const api_output_o_164_data = ref<any[]>([]);
const { data: api_output_o_164_q } = useQuery({queryKey: ['api_output_o_164', '/jaxrs/output/o-1/select'], queryFn: async () => { try { const r = await api.get("/jaxrs/output/o-1/select"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_output_o_164_q, (v) => { api_output_o_164_data.value = v ?? []; });
const api_output_list_data = ref<any[]>([]);
const { data: api_output_list_q } = useQuery({queryKey: ['api_output_list', '/jaxrs/output/list'], queryFn: async () => { try { const r = await api.get("/jaxrs/output/list"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_output_list_q, (v) => { api_output_list_data.value = v ?? []; });


const api_secret_c_983_data = ref<any[]>([]);
const { data: api_secret_c_983_q } = useQuery({queryKey: ['api_secret_c_983', '/jaxrs/secret/captcha/verify'], queryFn: async () => { try { const r = await api.get("/jaxrs/secret/captcha/verify"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_secret_c_983_q, (v) => { api_secret_c_983_data.value = v ?? []; });
async function api_secret_cancel() { try { await api.get("/jaxrs/secret/cancel") } catch {} }
const api_secret_check_data = ref<any[]>([]);
const { data: api_secret_check_q } = useQuery({queryKey: ['api_secret_check', '/jaxrs/secret/check'], queryFn: async () => { try { const r = await api.get("/jaxrs/secret/check"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_secret_check_q, (v) => { api_secret_check_data.value = v ?? []; });
async function api_secret_set_cancel() { try { await api.get("/jaxrs/secret/set/cancel") } catch {} }
async function api_secret_set() { try { await api.get("/jaxrs/secret/set") } catch {} }


const api_filter_1_size_10_data = ref<any[]>([]);
const { data: api_filter_1_size_10_q } = useQuery({queryKey: ['api_filter_1_size_10', '/jaxrs/log/list/filter/1/size/10'], queryFn: async () => { try { const r = await api.get("/jaxrs/log/list/filter/1/size/10"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_filter_1_size_10_q, (v) => { api_filter_1_size_10_data.value = v ?? []; });
const api_list_i_1_next_10_data = ref<any[]>([]);
const { data: api_list_i_1_next_10_q } = useQuery({queryKey: ['api_list_i_1_next_10', '/jaxrs/log/filter/list/i-1/next/10'], queryFn: async () => { try { const r = await api.get("/jaxrs/log/filter/list/i-1/next/10"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_list_i_1_next_10_q, (v) => { api_list_i_1_next_10_data.value = v ?? []; });


const api_server_execute_status_data = ref<any[]>([]);
const { data: api_server_execute_status_q } = useQuery({queryKey: ['api_server_execute_status', '/jaxrs/server/execute/status'], queryFn: async () => { try { const r = await api.get("/jaxrs/server/execute/status"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_server_execute_status_q, (v) => { api_server_execute_status_data.value = v ?? []; });


async function api_base_openapi_info() { try { await api.get("/jaxrs/base/openapi/info") } catch {} }
const api_cache_config_flush_1_data = ref<any[]>([]);
const { data: api_cache_config_flush_1_q } = useQuery({queryKey: ['api_cache_config_flush_1', '/jaxrs/base/cache/config/flush'], queryFn: async () => { try { const r = await api.get("/jaxrs/base/cache/config/flush"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_cache_config_flush_1_q, (v) => { api_cache_config_flush_1_data.value = v ?? []; });
async function api_base_cache_detail() { try { await api.get("/jaxrs/base/cache/detail") } catch {} }
async function api_base_echo() { try { await api.get("/jaxrs/base/echo") } catch {} }
async function api_base_cache() { try { await api.get("/jaxrs/base/cache") } catch {} }
const api_cache_commonscri_410_data = ref<any[]>([]);
const { data: api_cache_commonscri_410_q } = useQuery({queryKey: ['api_cache_commonscri_410', '/jaxrs/base/cache/commonscript/flush'], queryFn: async () => { try { const r = await api.get("/jaxrs/base/cache/commonscript/flush"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_cache_commonscri_410_q, (v) => { api_cache_commonscri_410_data.value = v ?? []; });
const api_fireschedule_cla_721_data = ref<any[]>([]);
const { data: api_fireschedule_cla_721_q } = useQuery({queryKey: ['api_fireschedule_cla_721', '/jaxrs/base/fireschedule/classname/com.x.processplatform.service.processing.ScheduleApplication'], queryFn: async () => { try { const r = await api.get("/jaxrs/base/fireschedule/classname/com.x.processplatform.service.processing.ScheduleApplication"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_fireschedule_cla_721_q, (v) => { api_fireschedule_cla_721_data.value = v ?? []; });
async function api_base_echo_get() { try { await api.get("/jaxrs/base/echo/get") } catch {} }

</script>

<style scoped>
.mod-view{display:flex;flex-direction:column;gap:16px;height:100%}
.view-header{padding:16px 24px}
.view-header h1{font-family:'Orbitron',sans-serif;font-size:20px;color:var(--color-primary);margin:0 0 4px;text-shadow:0 0 15px var(--color-primary-glow)}
.subtitle{font-size:12px;color:var(--text-muted);margin:0;font-family:'JetBrains Mono',monospace}
.content-panel{flex:1;overflow-y:auto;padding:16px;display:flex;flex-direction:column;gap:16px}
.grid-2col{display:grid;grid-template-columns:1fr 1fr;gap:16px}
.panel{padding:16px;display:flex;flex-direction:column;gap:12px}
.panel h3{font-size:15px;color:var(--color-primary);margin:0;font-family:'Orbitron',sans-serif}
.license-info{display:flex;flex-direction:column;gap:8px}
.info-row{display:flex;justify-content:space-between;padding:8px 12px;background:var(--bg-elevated);border-radius:var(--radius-sm)}
.info-key{font-size:13px;color:var(--text-muted);font-weight:600}
.info-val{font-size:13px;color:var(--text-primary);font-family:'JetBrains Mono',monospace}
.empty-license{color:var(--text-muted);font-size:13px;text-align:center;padding:20px}
.cmd-input-row{display:flex;gap:8px}
.cmd-input{flex:1;background:var(--bg-elevated);border:1px solid var(--border-subtle);border-radius:var(--radius-md);color:var(--text-primary);padding:8px 12px;font-size:13px;font-family:'JetBrains Mono',monospace}
.cmd-input:focus{outline:none;border-color:var(--color-primary)}
.btn-execute{padding:8px 20px;background:var(--color-primary);color:#000;border:none;border-radius:var(--radius-md);font-size:13px;cursor:pointer;font-weight:600}
.btn-execute:disabled{opacity:0.5;cursor:not-allowed}
.cmd-output{background:var(--bg-base);border:1px solid var(--border-subtle);border-radius:var(--radius-md);padding:12px;font-size:12px;color:var(--color-success);font-family:'JetBrains Mono',monospace;max-height:200px;overflow:auto;white-space:pre-wrap}
.cmd-error{color:var(--color-error);font-size:13px;padding:8px;background:rgba(239,68,68,.1);border-radius:var(--radius-sm)}
.danger-zone{padding:16px;display:flex;align-items:center;gap:16px}
.danger-zone h3{margin:0;font-size:15px}
.btn-stop{padding:10px 24px;background:transparent;border:2px solid var(--color-error);color:var(--color-error);border-radius:var(--radius-md);font-size:14px;cursor:pointer;font-weight:600;transition:all var(--transition-fast)}
.btn-stop:hover{background:var(--color-error);color:#fff}
.loading-small{color:var(--text-muted);font-size:13px}
@media(max-width:768px){.grid-2col{grid-template-columns:1fr}}
</style>
