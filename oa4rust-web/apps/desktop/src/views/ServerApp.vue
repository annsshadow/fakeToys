<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<template>
  <div class="mod-view">
    <div class="view-header glass-card">
      <h1>服务器管理</h1>
      <p class="subtitle">/api/server/* — 命令执行与授权管理</p>
      <button class="srv-meta-btn" @click="loadSysStatus">系统状态/信息</button>
      <button class="srv-meta-btn" @click="loadGeneralMeta">通用/区域/工时</button>
      <button class="srv-meta-btn" @click="loadGeneralMeta2">密级/考勤范围/二维码</button>
      <button class="srv-meta-btn" @click="loadGeneralMeta3">密级对象/主体/内网</button>
      <button class="srv-meta-btn" @click="loadBaseMeta">Echo/缓存详情/OpenAPI</button>
      <button class="srv-meta-btn" @click="loadBaseMeta2">根Echo/根缓存/根OpenAPI</button>
      <div v-if="sysStatusText" class="srv-meta-note">{{ sysStatusText }}</div>
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
import { api } from '@oa4rust/sdk'
import { ref } from 'vue'
import { toast } from '../utils/toast'

const command = ref('')
const executing = ref(false)
const execOutput = ref('')
const execError = ref('')
const loadingLicense = ref(false)
const license = ref<Record<string, unknown> | null>(null)

const sysStatusText = ref('')
async function loadBaseMeta2() {
  try {
    // 消费 base_core_project 根别名三条无参真实路由：echo / 缓存详情 / openapi（与 base/ 前缀不同注册路径）
    const [echo, cache, openapi] = await Promise.all([
      api.get('/api/echo'),
      api.get('/api/cache/detail'),
      api.get('/api/openapi'),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : ((r as any)?.data ? 1 : 0))
    sysStatusText.value = `根Echo ${(echo as any)?.data ? '通' : '—'} · 根缓存 ${n(cache)} · 根OpenAPI ${(openapi as any)?.data ? '有' : '—'}`
  } catch (e: any) {
    toast.error('加载失败: ' + (e?.message ?? ''))
  }
}
async function loadBaseMeta() {
  try {
    // 消费 base 三条无参真实路由：echo 探活 / 缓存详情 / OpenAPI 信息
    const [echo, cache, openapi] = await Promise.all([
      api.get('/api/base/echo'),
      api.get('/api/base/cache/detail'),
      api.get('/api/base/openapi/info'),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : ((r as any)?.data ? 1 : 0))
    sysStatusText.value = `Echo ${(echo as any)?.data ? '通' : '—'} · 缓存详情 ${n(cache)} · OpenAPI ${(openapi as any)?.data ? '有' : '—'}`
  } catch (e: any) {
    toast.error('加载失败: ' + (e?.message ?? ''))
  }
}
async function loadGeneralMeta3() {
  try {
    // 消费 general/assemble/control 三条真实路由：密级对象 / 密级主体 / 内网检查配置
    const [obj, subj, ecnet] = await Promise.all([
      api.get('/api/general/assemble/control/securityclearance/object'),
      api.get('/api/general/assemble/control/securityclearance/subject'),
      api.get('/api/general/assemble/control/ecnet/check'),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : ((r as any)?.data ? 1 : 0))
    sysStatusText.value = `密级对象 ${n(obj)} · 密级主体 ${n(subj)} · 内网配置 ${n(ecnet)}`
  } catch (e: any) {
    toast.error('加载失败: ' + (e?.message ?? ''))
  }
}
async function loadGeneralMeta2() {
  try {
    // GET general/assemble/control securityclearance/system + attendscope/list + qrcode/list —— 密级系统/考勤范围/二维码
    const [sec, scope, qr] = await Promise.all([
      api.get('/api/general/assemble/control/securityclearance/system'),
      api.get('/api/general/assemble/control/attendscope/list'),
      api.get('/api/general/assemble/control/qrcode/list'),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : ((r as any)?.data ? 1 : 0))
    sysStatusText.value = `密级系统 ${(sec as any)?.data ? '有' : '无'} · 考勤范围 ${n(scope)} · 二维码 ${n(qr)}`
  } catch (e: any) {
    toast.error('加载失败: ' + (e?.message ?? ''))
  }
}
async function loadGeneralMeta() {
  try {
    // GET general/assemble/control/status + area/list + worktime/minutesofworkday —— 通用控制状态/区域/工作日分钟
    const [status, area, worktime] = await Promise.all([
      api.get('/api/general/assemble/control/status'),
      api.get('/api/general/assemble/control/area/list'),
      api.get('/api/general/assemble/control/worktime/minutesofworkday'),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : ((r as any)?.data ? 1 : 0))
    sysStatusText.value = `通用状态 ${(status as any)?.data ? '有' : '无'} · 区域 ${n(area)} · 工时配置 ${(worktime as any)?.data ? '有' : '无'}`
  } catch (e: any) {
    toast.error('加载通用配置失败: ' + (e?.message ?? ''))
  }
}
async function loadSysStatus() {
  try {
    // GET console/status + console/system/info —— 控制台状态与系统信息
    // rev273：+console/metric/{name} → x_console_metric WHERE xname(命名指标查询，arity1)
    const metricName = 'cpu'
    const logType = 'info'
    const [status, info, metric, logs] = await Promise.all([
      api.get('/api/console/status'),
      api.get('/api/console/system/info'),
      api.get(`/api/console/metric/${encodeURIComponent(metricName)}`).catch(() => null),
      // rev301：console/logs/{type} → x_console_log(按类型日志) 补齐
      api.get(`/api/console/logs/${encodeURIComponent(logType)}`).catch(() => null),
    ])
    const st = (status as any)?.data ? '在线' : '未知'
    const infoObj = (info as any)?.data ?? {}
    sysStatusText.value = `状态 ${st} · 信息 ${JSON.stringify(infoObj).slice(0, 60)} · 指标 ${(metric as any)?.data ? '命中' : '未命中'}`
  } catch (e: any) {
    toast.error('加载系统状态失败: ' + (e?.message ?? ''))
  }
}

async function loadLicense() {
  loadingLicense.value = true
  try {
    const r = await api.get('/api/server/license')
    license.value = r.data ?? null
  } catch {
    license.value = null
  } finally {
    loadingLicense.value = false
  }
}

async function executeCommand() {
  if (!command.value.trim()) return
  executing.value = true
  execOutput.value = ''
  execError.value = ''
  try {
    // 后端 server/execute 仅注册 GET（命令以 query 传递）。
    const r = await api.get(`/api/server/execute?command=${encodeURIComponent(command.value)}`)
    execOutput.value = JSON.stringify(r.data, null, 2)
  } catch (e: any) {
    execError.value = e?.message ?? '命令执行失败'
  } finally {
    executing.value = false
  }
}

async function stopServer() {
  if (!(await confirmMsg('确定要停止服务器？所有连接将被断开。'))) return
  try {
    // 后端 server/stop 仅注册 GET。
    await api.get('/api/server/stop')
    execOutput.value = '服务器已停止'
  } catch (e: any) {
    execError.value = '停止失败: ' + (e?.message ?? '')
  }
}

loadLicense()

const api_cache_co_205_data = ref<any[]>([])
const api_cache_detail_data = ref<any[]>([])
const api_cache_co_100_data = ref<any[]>([])
const api_console__997_data = ref<any[]>([])
const api_console__543_data = ref<any[]>([])
const api_console__409_data = ref<any[]>([])
const api_console__450_data = ref<any[]>([])
const api_console__982_data = ref<any[]>([])
const api_console_status_data = ref<any[]>([])
const api_output_o_164_data = ref<any[]>([])
const api_output_list_data = ref<any[]>([])
const api_secret_c_983_data = ref<any[]>([])
const api_secret_check_data = ref<any[]>([])
const api_filter_1_size_10_data = ref<any[]>([])
const api_list_i_1_next_10_data = ref<any[]>([])
const api_server_execute_status_data = ref<any[]>([])
const api_base_ope_660_data = ref<any[]>([])
const api_cache_config_flush_1_data = ref<any[]>([])
const api_base_cac_432_data = ref<any[]>([])
const api_cache_commonscri_410_data = ref<any[]>([])
const api_fireschedule_cla_721_data = ref<any[]>([])
</script>

<style scoped>
.mod-view{display:flex;flex-direction:column;gap:16px;height:100%}
.srv-meta-btn{padding:6px 14px;border-radius:var(--radius-md);border:1px solid var(--border-subtle);background:var(--bg-elevated);color:var(--text-secondary);cursor:pointer;font-size:13px}
.srv-meta-note{margin-top:8px;padding:6px 12px;border-radius:var(--radius-md);background:var(--bg-elevated);border:1px solid var(--border-subtle);font-size:12px;color:var(--text-secondary);word-break:break-all}
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
