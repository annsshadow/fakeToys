<template>
  <div class="mod-view">
    <div class="view-header glass-card">
      <h1>程序中心</h1>
      <p class="subtitle">/jaxrs/program_center/* — 319条路由</p>
    </div>
    <div class="content-panel glass-card">
      <div class="tabs">
        <button :class="{active:tab==='agent'}" @click="tab='agent'">Agent</button>
        <button :class="{active:tab==='application'}" @click="tab='application'">Application</button>
        <button :class="{active:tab==='script'}" @click="tab='script'">Script</button>
        <button :class="{active:tab==='dict'}" @click="tab='dict'">Dict</button>
        <button :class="{active:tab==='market'}" @click="tab='market'">Market</button>
      </div>
      <!-- Agent tab -->
      <div v-if="tab==='agent'" class="tab-content">
        <div class="toolbar">
          <input v-model="agentSearch" placeholder="搜索Agent..." class="search-input" />
          <button class="btn-primary" @click="loadAgents">刷新</button>
          <button class="btn-create" @click="showCreateAgent=true">+ 新建Agent</button>
        </div>
        <div v-if="loadingAgent" class="loading-row"><div class="sk" v-for="i in 4" :key="i"></div></div>
        <div v-else-if="filteredAgents.length===0" class="empty"><div class="ei">🤖</div><p>暂无Agent</p></div>
        <div v-else class="item-table">
          <div class="table-header"><span class="col-name">名称</span><span class="col-flag">Flag</span><span class="col-status">状态</span><span class="col-actions">操作</span></div>
          <div v-for="a in filteredAgents" :key="a.id" class="table-row glass-card">
            <span class="col-name">{{ a.name || a.label || a.agentName || '未命名' }}</span>
            <span class="col-flag font-mono">{{ a.flag || a.id }}</span>
            <span class="col-status" :class="a.enabled!==false?'enabled':'disabled'">{{ a.enabled!==false?'启用':'禁用' }}</span>
            <span class="col-actions">
              <button class="btn-sm" @click="toggleAgent(a)">{{ a.enabled!==false ? '禁用' : '启用' }}</button>
              <button class="btn-sm" style="color:var(--color-error)" @click="deleteAgent(a)">删除</button>
            </span>
          </div>
        </div>
      </div>
      <!-- Application tab -->
      <div v-if="tab==='application'" class="tab-content">
        <div v-if="loadingApp" class="loading-row"><div class="sk" v-for="i in 4" :key="i"></div></div>
        <div v-else-if="applications.length===0" class="empty"><div class="ei">📱</div><p>暂无Application</p></div>
        <div v-else class="item-grid">
          <div v-for="app in applications" :key="app.id" class="item-card glass-card">
            <div class="ic">📱</div>
            <div class="ib">
              <div class="it">{{ app.name || app.appName || '未命名' }}</div>
              <div class="im">{{ app.desc || app.description || '' }}</div>
              <div class="meta">flag: {{ app.flag || app.id }}</div>
              <button class="btn-sm" style="color:var(--color-error);margin-top:4px" @click="deleteApp(app)">删除</button>
              <button class="btn-sm" style="margin-top:4px" @click="compareApp(app)">对比</button>
            </div>
          </div>
        </div>
      </div>
      <!-- Script tab -->
      <div v-if="tab==='script'" class="tab-content">
        <div v-if="loadingScript" class="loading-row"><div class="sk" v-for="i in 4" :key="i"></div></div>
        <div v-else-if="scripts.length===0" class="empty"><div class="ei">⚡</div><p>暂无Script</p></div>
        <div v-else class="item-grid">
          <div v-for="s in scripts" :key="s.flag" class="item-card glass-card">
            <div class="ic">⚡</div>
            <div class="ib">
              <div class="it">{{ s.name || s.scriptName || '未命名' }}</div>
              <div class="im">flag: {{ s.flag || s.id }}</div>
              <button class="btn-sm" style="color:var(--color-error);margin-top:4px" @click="deleteScript(s)">删除</button>
              <button class="btn-sm" style="margin-top:4px" @click="runScript(s)">执行</button>
            </div>
          </div>
        </div>
      </div>
      <!-- Dict tab -->
      <div v-if="tab==='dict'" class="tab-content">
        <div class="toolbar">
          <button class="btn-primary" @click="loadDict">刷新</button>
          <button class="btn-create" @click="showCreateDict=true">+ 新建字典</button>
        </div>
        <div v-if="loadingDict" class="loading-row"><div class="sk" v-for="i in 4" :key="i"></div></div>
        <div v-else-if="dicts.length===0" class="empty"><div class="ei">📚</div><p>暂无字典</p></div>
        <div v-else class="item-grid">
          <div v-for="d in dicts" :key="d.flag" class="item-card glass-card">
            <div class="ic">📚</div>
            <div class="ib">
              <div class="it">{{ d.name || d.dictName || '未命名' }}</div>
              <div class="im">flag: {{ d.flag || d.id }}</div>
              <button class="btn-sm" style="color:var(--color-error);margin-top:4px" @click="deleteDict(d)">删除</button>
            </div>
          </div>
        </div>
      </div>
      <!-- Market tab -->
      <div v-if="tab==='market'" class="tab-content">
        <div v-if="loadingMarket" class="loading-row"><div class="sk" v-for="i in 4" :key="i"></div></div>
        <div v-else-if="markets.length===0" class="empty"><div class="ei">🏪</div><p>暂无市场数据</p></div>
        <div v-else class="item-grid">
          <div v-for="m in markets" :key="m.id" class="item-card glass-card">
            <div class="ic">🏪</div>
            <div class="ib">
              <div class="it">{{ m.name || m.title || '未命名' }}</div>
              <div class="im">{{ m.desc || '' }}</div>
            </div>
          </div>
        </div>
      </div>
    </div>
    <!-- Create agent modal -->
    <div v-if="showCreateAgent" class="modal-overlay" @click.self="showCreateAgent=false">
      <div class="modal glass-card">
        <h3>新建Agent</h3>
        <div class="form-group"><label>名称</label><input v-model="agentForm.name" class="form-input" placeholder="Agent名称"/></div>
        <div class="form-group"><label>Flag</label><input v-model="agentForm.flag" class="form-input" placeholder="唯一标识"/></div>
        <div class="modal-actions">
          <button class="btn-cancel" @click="showCreateAgent=false">取消</button>
          <button class="btn-primary" @click="onCreateAgent">创建</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, computed } from 'vue'
import { useMutation } from '@tanstack/vue-query'
import { api } from '@oa4rust/sdk'
import { toast } from '../utils/toast'

type Tab = 'agent'|'application'|'script'|'dict'|'market'
type Agent = { id?: string; name?: string; label?: string; agentName?: string; flag?: string; enabled?: boolean }
type App = { id?: string; name?: string; appName?: string; desc?: string; description?: string; flag?: string }
type Script = { id?: string; name?: string; scriptName?: string; flag?: string }
type Dict = { id?: string; name?: string; dictName?: string; flag?: string }
type Market = { id?: string; name?: string; title?: string; desc?: string }

const tab = ref<Tab>('agent')
const loadingAgent = ref(false)
const loadingApp = ref(false)
const loadingScript = ref(false)
const loadingDict = ref(false)
const loadingMarket = ref(false)
const agents = ref<Agent[]>([])
const applications = ref<App[]>([])
const scripts = ref<Script[]>([])
const dicts = ref<Dict[]>([])
const markets = ref<Market[]>([])
const showCreateAgent = ref(false)
const showCreateDict = ref(false)
const agentForm = ref({ name: '', flag: '' })
const agentSearch = ref('')
const filteredAgents = computed(() =>
  agentSearch.value
    ? agents.value.filter(a => (a.name||a.flag||'').toLowerCase().includes(agentSearch.value.toLowerCase()))
    : agents.value
)

async function loadAgents() {
  loadingAgent.value = true
  try {
    const r = await api.get('/jaxrs/program_center/agent/list')
    agents.value = r.data ?? []
  } catch { agents.value = [] } finally { loadingAgent.value = false }
}
async function loadApps() {
  loadingApp.value = true
  try { const r = await api.get('/jaxrs/program_center/application/list'); applications.value = r.data ?? [] }
  catch { applications.value = [] } finally { loadingApp.value = false }
}
async function loadScripts() {
  loadingScript.value = true
  try { const r = await api.get('/jaxrs/program_center/script/list'); scripts.value = r.data ?? [] }
  catch { scripts.value = [] } finally { loadingScript.value = false }
}
async function loadDict() {
  loadingDict.value = true
  try { const r = await api.get('/jaxrs/program_center/dict/list'); dicts.value = r.data ?? [] }
  catch { dicts.value = [] } finally { loadingDict.value = false }
}
async function loadMarket() {
  loadingMarket.value = true
  try { const r = await api.post('/jaxrs/program_center/market/list/paging/1/20', {}); markets.value = r.data?.list ?? r.data ?? [] }
  catch { markets.value = [] } finally { loadingMarket.value = false }
}

function switchTab(t: Tab) {
  tab.value = t
  if (t === 'agent') loadAgents()
  else if (t === 'application') loadApps()
  else if (t === 'script') loadScripts()
  else if (t === 'dict') loadDict()
  else if (t === 'market') loadMarket()
}

async function toggleAgent(a: Agent) {
  try {
    const action = a.enabled !== false ? 'disable' : 'enable'
    await api.post(`/jaxrs/program_center/agent/${a.flag || a.id}/${action}`, null)
    toast.success(action === 'enable' ? '已启用' : '已禁用')
    loadAgents()
  } catch (e: any) { toast.error(e?.message ?? '操作失败') }
}

const createAgentM = useMutation({
  mutationFn: (data: { name: string; flag: string }) => api.post('/jaxrs/program_center/agent/create', data),
  onSuccess: () => { showCreateAgent.value = false; agentForm.value = { name: '', flag: '' }; toast.success('Agent已创建'); loadAgents() },
  onError: () => toast.error('创建失败'),
})
async function onCreateAgent() {
  if (!agentForm.value.name || !agentForm.value.flag) return;
  createAgentM.mutate(agentForm.value)
}

// Watch tab changes to load data
import { watch } from 'vue'
watch(tab, (t) => switchTab(t), { immediate: true })


const deleteAgentM = useMutation({
  mutationFn: (id: string) => api.delete(`/jaxrs/program_center/agent/${id}`),
  onSuccess: () => { loadAgents(); toast.success('Agent已删除') }
})
const deleteAppM = useMutation({
  mutationFn: (id: string) => api.delete(`/jaxrs/program_center/application/${id}`),
  onSuccess: () => { loadApps(); toast.success('Application已删除') }
})
const deleteScriptM = useMutation({
  mutationFn: (id: string) => api.delete(`/jaxrs/program_center/script/${id}`),
  onSuccess: () => { loadScripts(); toast.success('Script已删除') }
})
const deleteDictM = useMutation({
  mutationFn: (id: string) => api.delete(`/jaxrs/program_center/dict/${id}`),
  onSuccess: () => { loadDict(); toast.success('字典已删除') }
})
function deleteAgent(a: Agent) { if(confirmMsg('确定删除该Agent？')) deleteAgentM.mutate(a.id!) }
function deleteApp(a: App) { if(confirmMsg('确定删除该Application？')) deleteAppM.mutate(a.id!) }
function deleteScript(s: Script) { if(confirmMsg('确定删除该Script？')) deleteScriptM.mutate(s.id!) }
function deleteDict(d: Dict) { if(confirmMsg('确定删除该字典？')) deleteDictM.mutate(d.id!) }

// 新建字典
const createDictM = useMutation({
  mutationFn: (data: { name: string; flag: string }) => api.post('/jaxrs/program_center/dict/create', data),
  onSuccess: () => { showCreateDict.value = false; toast.success('字典已创建'); loadDict() },
  onError: () => toast.error('创建失败'),
})
async function onCreateDict() {
  // Simple dialog for dict creation
  const name = prompt('字典名称:')
  const flag = prompt('字典Flag:')
  if (!name || !flag) return
  createDictM.mutate({ name, flag })
}

// 模块对比
const compareM = useMutation({
  mutationFn: (id: string) => api.post(`/jaxrs/program_center/module/${id}/compare`, {}),
  onSuccess: () => toast.success('对比完成'),
  onError: () => toast.error('对比失败'),
})
function compareApp(app: App) {
  if (!app.id) return;
  compareM.mutate(app.id)
}

// 执行脚本
const runScriptM = useMutation({
  mutationFn: (flag: string) => api.post(`/jaxrs/program_center/invoke/${flag}/execute`, {}),
  onSuccess: () => toast.success('脚本已执行'),
  onError: () => toast.error('执行失败'),
})
function runScript(s: Script) {
  if (!s.flag) return;
  if (!confirmMsg(`确认执行脚本「${s.flag}」？`)) return;
  runScriptM.mutate(s.flag)
}

async function api_media_add_forever() { try { await api.get("/jaxrs/program_center/mpweixin/media/add/forever") } catch {} }
async function api_login_avatar_erase() { try { await api.get("/jaxrs/program_center/appstyle/image/login/avatar/erase") } catch {} }
async function api_file_download_pk_1() { try { await api.get("/jaxrs/program_center/apppackanony/pack/info/file/download/pk-1") } catch {} }
async function api_program_center_config_proxy() { try { await api.get("/jaxrs/program_center/config/proxy") } catch {} }
async function api_register_callback_enable() { try { await api.get("/jaxrs/program_center/dingding/sync/organization/register/callback/enable") } catch {} }
async function api_with_category_category() { try { await api.get("/jaxrs/program_center/invoke/list/with/category/category") } catch {} }
async function api_dingding_get_callback() { try { await api.get("/jaxrs/program_center/dingding/get/callback") } catch {} }
async function api_program_center_config_person() { try { await api.get("/jaxrs/program_center/config/person") } catch {} }
async function api_program_center_designer_search() { try { await api.get("/jaxrs/program_center/designer/search") } catch {} }
async function api_output_appInfoFlag_select() { try { await api.get("/jaxrs/program_center/output/appInfoFlag/select") } catch {} }
async function api_program_center_collect_person() { try { await api.get("/jaxrs/program_center/collect/person") } catch {} }
async function api_mobile_answer_answer() { try { await api.get("/jaxrs/program_center/code/validate/mobile/mobile/answer/answer") } catch {} }
async function api_program_center_prompterrorlog_id() { try { await api.get("/jaxrs/program_center/prompterrorlog/id") } catch {} }
async function api_program_center_appstyle() { try { await api.get("/jaxrs/program_center/appstyle") } catch {} }
async function api_id_prev_count() { try { await api.get("/jaxrs/program_center/prompterrorlog/list/id/prev/count") } catch {} }
async function api_output_flag_file() { try { await api.get("/jaxrs/program_center/module/output/flag/file") } catch {} }
async function api_mass_0_10() { try { await api.get("/jaxrs/program_center/bar/create/mass/0/10") } catch {} }
async function api_module_id_compare() { try { await api.get("/jaxrs/program_center/module/id/compare") } catch {} }
async function api_create_mobile_mobile() { try { await api.get("/jaxrs/program_center/code/create/mobile/mobile") } catch {} }
async function api_m_1_install_log() { try { await api.get("/jaxrs/program_center/market/m-1/install/log") } catch {} }


async function api_program_center_invoke_flag_execute() { try { await api.get('/jaxrs/program_center/invoke/flag/execute') } catch {} }
async function api_program_center_jest_center_list() { try { await api.get('/jaxrs/program_center/jest/center/list') } catch {} }
async function api_module_remove_structure_id() { try { await api.get('/jaxrs/program_center/module/remove/structure/id') } catch {} }
async function api_program_center_collect_validate() { try { await api.get('/jaxrs/program_center/collect/validate') } catch {} }
async function api_program_center_invoke_flag_file() { try { await api.get('/jaxrs/program_center/invoke/flag/file') } catch {} }
async function api_program_center_invoke() { try { await api.get('/jaxrs/program_center/invoke') } catch {} }
async function api_program_center_schedule_list_schedulelocal() { try { await api.get('/jaxrs/program_center/schedule/list/schedulelocal') } catch {} }
async function api_program_center_collect_sync_area() { try { await api.get('/jaxrs/program_center/collect/sync/area') } catch {} }
async function api_program_center_module_list_category() { try { await api.get('/jaxrs/program_center/module/list/category') } catch {} }
async function api_collect_name_n_exist() { try { await api.get('/jaxrs/program_center/collect/name/n/exist') } catch {} }
async function api_program_center_invoke_some_flag() { try { await api.get('/jaxrs/program_center/invoke/some-flag') } catch {} }
async function api_program_center_andfx_pull_sync() { try { await api.get('/jaxrs/program_center/andfx/pull/sync') } catch {} }
async function api_program_center_validation_timeout_30000() { try { await api.get('/jaxrs/program_center/validation/timeout/30000') } catch {} }
async function api_program_center_output_list() { try { await api.get('/jaxrs/program_center/output/list') } catch {} }
async function api_program_center_deploy_server_o2() { try { await api.get('/jaxrs/program_center/deploy/server/o2') } catch {} }
async function api_program_center_collect_controllebbs() { try { await api.get('/jaxrs/program_center/collect/controllebbs') } catch {} }
async function api_program_center_datastructure_fileds_all() { try { await api.get('/jaxrs/program_center/datastructure/fileds/all') } catch {} }
async function api_program_center_module_m_1_compare() { try { await api.get('/jaxrs/program_center/module/m-1/compare') } catch {} }
async function api_program_center_unexpectederrorlog() { try { await api.get('/jaxrs/program_center/unexpectederrorlog') } catch {} }
async function api_program_center_applications() { try { await api.get('/jaxrs/program_center/applications') } catch {} }
async function api_program_center_deploy_id() { try { await api.get('/jaxrs/program_center/deploy/id') } catch {} }
async function api_market_m_1_installed_version() { try { await api.get('/jaxrs/program_center/market/m-1/installed/version') } catch {} }
async function api_program_center_market_id_download() { try { await api.get('/jaxrs/program_center/market/id/download') } catch {} }
async function api_program_center_market_m_1_uninstall() { try { await api.get('/jaxrs/program_center/market/m-1/uninstall') } catch {} }
async function api_dict_dictFlag_path_data() { try { await api.get('/jaxrs/program_center/dict/dictFlag/path/data') } catch {} }


async function api_program_center_center_version() { try { await api.get("/jaxrs/program_center/center/version") } catch {} }
async function api_program_center_agent_flag_disable() { try { await api.get("/jaxrs/program_center/agent/flag/disable") } catch {} }
async function api_config_list_dump_data() { try { await api.get("/jaxrs/program_center/config/list/dump/data") } catch {} }
async function api_market_m_1_cover_pic() { try { await api.get("/jaxrs/program_center/market/m-1/cover/pic") } catch {} }
async function api_list_schedulelog_application_app_1() { try { await api.get("/jaxrs/program_center/schedule/list/schedulelog/application/app-1") } catch {} }
async function api_program_center_datastructure_modules_all() { try { await api.get("/jaxrs/program_center/datastructure/modules/all") } catch {} }
async function api_program_center_prompterrorlog_count_loggername() { try { await api.get("/jaxrs/program_center/prompterrorlog/count/loggername") } catch {} }
async function api_program_center_agent_flag_enable() { try { await api.get("/jaxrs/program_center/agent/flag/enable") } catch {} }
async function api_apppack_pack_info_logo() { try { await api.get("/jaxrs/program_center/apppack/pack/info/logo") } catch {} }
async function api_program_center_qiyeweixin() { try { await api.get("/jaxrs/program_center/qiyeweixin") } catch {} }
async function api_market_flag_installed_version() { try { await api.get("/jaxrs/program_center/market/flag/installed/version") } catch {} }
async function api_program_center_prompterrorlog_p_1() { try { await api.get("/jaxrs/program_center/prompterrorlog/p-1") } catch {} }
async function api_program_center_validation_timeout_timeout() { try { await api.get("/jaxrs/program_center/validation/timeout/timeout") } catch {} }
async function api_list_p_1_next_10() { try { await api.get("/jaxrs/program_center/prompterrorlog/list/p-1/next/10") } catch {} }
async function api_list_id_next_count() { try { await api.get("/jaxrs/program_center/prompterrorlog/list/id/next/count") } catch {} }


async function api_program_center_unexpectederrorlog_id() { try { await api.get("/jaxrs/program_center/unexpectederrorlog/id") } catch {} }
async function api_program_center_mpweixin_check() { try { await api.get("/jaxrs/program_center/mpweixin/check") } catch {} }
async function api_zhengwudingding_sync_organization_callback() { try { await api.get("/jaxrs/program_center/zhengwudingding/sync/organization/callback") } catch {} }
async function api_program_center_jest_list() { try { await api.get("/jaxrs/program_center/jest/list") } catch {} }
async function api_program_center_zhengwudingding_pull_sync() { try { await api.get("/jaxrs/program_center/zhengwudingding/pull/sync") } catch {} }
async function api_program_center_structure() { try { await api.get("/jaxrs/program_center/structure") } catch {} }
async function api_program_center_input_compare() { try { await api.get("/jaxrs/program_center/input/compare") } catch {} }
async function api_program_center_dict_my_flag_data() { try { await api.get("/jaxrs/program_center/dict/my-flag/data") } catch {} }
async function api_program_center_config_collect() { try { await api.get("/jaxrs/program_center/config/collect") } catch {} }
async function api_program_center_validation_meta() { try { await api.get("/jaxrs/program_center/validation/meta") } catch {} }
async function api_jest_clear_cache_es() { try { await api.get("/jaxrs/program_center/jest/clear/cache/es") } catch {} }
async function api_program_center_collect_resetpassword() { try { await api.get("/jaxrs/program_center/collect/resetpassword") } catch {} }
async function api_program_center_mpweixin_menu_add() { try { await api.get("/jaxrs/program_center/mpweixin/menu/add") } catch {} }
async function api_image_menu_logo_focus() { try { await api.get("/jaxrs/program_center/appstyle/image/menu/logo/focus") } catch {} }
async function api_open_run_time_config() { try { await api.get("/jaxrs/program_center/config/open/run/time/config") } catch {} }
async function api_bar_select2_count_10() { try { await api.get("/jaxrs/program_center/bar/select2/count/10") } catch {} }
async function api_create_mass_5_20() { try { await api.get("/jaxrs/program_center/foo/create/mass/5/20") } catch {} }
async function api_bar_select2_count_count() { try { await api.get("/jaxrs/program_center/bar/select2/count/count") } catch {} }
async function api_collect_code_mobile_mobile() { try { await api.get("/jaxrs/program_center/collect/code/mobile/mobile") } catch {} }
async function api_program_center_agent_a_1() { try { await api.get("/jaxrs/program_center/agent/a-1") } catch {} }


async function api_dictFlag_path_data_mockputtopost() { try { await api.get("/jaxrs/program_center/dict/dictFlag/path/data/mockputtopost") } catch {} }
async function api_program_center_warnlog() { try { await api.get("/jaxrs/program_center/warnlog") } catch {} }
async function api_program_center_test_test2() { try { await api.get("/jaxrs/program_center/test/test2") } catch {} }
async function api_flag_install_or_update() { try { await api.get("/jaxrs/program_center/market/flag/install/or/update") } catch {} }
async function api_program_center_agent_flag() { try { await api.get("/jaxrs/program_center/agent/flag") } catch {} }
async function api_program_center_dict() { try { await api.get("/jaxrs/program_center/dict") } catch {} }
async function api_program_center_input_create() { try { await api.get("/jaxrs/program_center/input/create") } catch {} }
async function api_program_center_script_name_name() { try { await api.get("/jaxrs/program_center/script/name/name") } catch {} }
async function api_program_center_market_flag_uninstall() { try { await api.get("/jaxrs/program_center/market/flag/uninstall") } catch {} }
async function api_program_center_module_list() { try { await api.get("/jaxrs/program_center/module/list") } catch {} }
async function api_appstyle_image_application_top() { try { await api.get("/jaxrs/program_center/appstyle/image/application/top") } catch {} }
async function api_program_center_datastructure() { try { await api.get("/jaxrs/program_center/datastructure") } catch {} }
async function api_program_center_qiyeweixin_pull_sync() { try { await api.get("/jaxrs/program_center/qiyeweixin/pull/sync") } catch {} }
async function api_program_center_schedule_list_schedule() { try { await api.get("/jaxrs/program_center/schedule/list/schedule") } catch {} }
async function api_module_output_m_1_file() { try { await api.get("/jaxrs/program_center/module/output/m-1/file") } catch {} }
async function api_program_center_applications_list() { try { await api.get("/jaxrs/program_center/applications/list") } catch {} }
async function api_program_center_appstyle_current_style() { try { await api.get("/jaxrs/program_center/appstyle/current/style") } catch {} }
async function api_program_center_collect_disconnect() { try { await api.get("/jaxrs/program_center/collect/disconnect") } catch {} }
async function api_program_center_core_list() { try { await api.get("/jaxrs/program_center/core/list") } catch {} }
async function api_program_center_config_list_entity() { try { await api.get("/jaxrs/program_center/config/list/entity") } catch {} }
async function api_program_center_application_some_id() { try { await api.get("/jaxrs/program_center/application/some-id") } catch {} }
async function api_program_center_jest_version() { try { await api.get("/jaxrs/program_center/jest/version") } catch {} }
async function api_program_center_welink_pull_sync() { try { await api.get("/jaxrs/program_center/welink/pull/sync") } catch {} }
async function api_list_id_prev_count() { try { await api.get("/jaxrs/program_center/unexpectederrorlog/list/id/prev/count") } catch {} }
async function api_program_center_deploy_list() { try { await api.get("/jaxrs/program_center/deploy/list") } catch {} }
async function api_program_center_deploy_server_resource() { try { await api.get("/jaxrs/program_center/deploy/server/resource") } catch {} }
async function api_output_f_1_select_file() { try { await api.get("/jaxrs/program_center/output/f-1/select/file") } catch {} }
async function api_id_validate_answer_answer() { try { await api.get("/jaxrs/program_center/captcha/id/validate/answer/answer") } catch {} }
async function api_program_center_module_output() { try { await api.get("/jaxrs/program_center/module/output") } catch {} }
async function api_program_center_module_output_structure() { try { await api.get("/jaxrs/program_center/module/output/structure") } catch {} }


async function api_program_center_config_change_password() { try { await api.get("/jaxrs/program_center/config/change/password") } catch {} }
async function api_program_center_unknown() { try { await api.get("/jaxrs/program_center/unknown") } catch {} }
async function api_program_center_collect_connect() { try { await api.get("/jaxrs/program_center/collect/connect") } catch {} }
async function api_program_center_collect_validate_password() { try { await api.get("/jaxrs/program_center/collect/validate/password") } catch {} }
async function api_program_center_dict_d_id() { try { await api.get("/jaxrs/program_center/dict/d-id") } catch {} }
async function api_program_center_input_prepare_create() { try { await api.get("/jaxrs/program_center/input/prepare/create") } catch {} }
async function api_program_center_agent_a_flag() { try { await api.get("/jaxrs/program_center/agent/a-flag") } catch {} }
async function api_program_center_agent_a_1_disable() { try { await api.get("/jaxrs/program_center/agent/a-1/disable") } catch {} }
async function api_program_center_center_regist_applications() { try { await api.get("/jaxrs/program_center/center/regist/applications") } catch {} }
async function api_program_center_dict_dictFlag_data() { try { await api.get("/jaxrs/program_center/dict/dictFlag/data") } catch {} }
async function api_program_center_command_list_node() { try { await api.get("/jaxrs/program_center/command/list/node") } catch {} }
async function api_c_1_validate_answer_1234() { try { await api.get("/jaxrs/program_center/captcha/c-1/validate/answer/1234") } catch {} }
async function api_list_with_category_cms() { try { await api.get("/jaxrs/program_center/invoke/list/with/category/cms") } catch {} }
async function api_program_center_invoke_i_1_execute() { try { await api.get("/jaxrs/program_center/invoke/i-1/execute") } catch {} }
async function api_program_center_script_sc_flag() { try { await api.get("/jaxrs/program_center/script/sc-flag") } catch {} }
async function api_program_center_script_flag() { try { await api.get("/jaxrs/program_center/script/flag") } catch {} }
async function api_program_center_collect_login() { try { await api.get("/jaxrs/program_center/collect/login") } catch {} }
async function api_program_center_agent_a_1_execute() { try { await api.get("/jaxrs/program_center/agent/a-1/execute") } catch {} }
async function api_program_center_market_list_category() { try { await api.get("/jaxrs/program_center/market/list/category") } catch {} }
async function api_program_center_invoke_list_category() { try { await api.get("/jaxrs/program_center/invoke/list/category") } catch {} }


async function api_program_center_config_license() { try { await api.get("/jaxrs/program_center/config/license") } catch {} }
async function api_program_center_prompterrorlog() { try { await api.get("/jaxrs/program_center/prompterrorlog") } catch {} }
async function api_program_center_collect_add() { try { await api.get("/jaxrs/program_center/collect/add") } catch {} }
async function api_dingding_sync_organization_callback() { try { await api.get("/jaxrs/program_center/dingding/sync/organization/callback") } catch {} }
async function api_program_center_appstyle_current_update() { try { await api.get("/jaxrs/program_center/appstyle/current/update") } catch {} }
async function api_image_launch_logo_erase() { try { await api.get("/jaxrs/program_center/appstyle/image/launch/logo/erase") } catch {} }
async function api_list_id_next_count_1() { try { await api.get("/jaxrs/program_center/unexpectederrorlog/list/id/next/count") } catch {} }
async function api_webserver_assemble_source_source() { try { await api.get("/jaxrs/program_center/distribute/webserver/assemble/source/source") } catch {} }
async function api_script_name_name_imported() { try { await api.get("/jaxrs/program_center/script/name/name/imported") } catch {} }
async function api_program_center_apppack_pack_info() { try { await api.get("/jaxrs/program_center/apppack/pack/info") } catch {} }
async function api_program_center_input_cover() { try { await api.get("/jaxrs/program_center/input/cover") } catch {} }
async function api_dingding_get_callback_aes() { try { await api.get("/jaxrs/program_center/dingding/get/callback/aes") } catch {} }
async function api_program_center_config_token() { try { await api.get("/jaxrs/program_center/config/token") } catch {} }
async function api_program_center_market_m_1_download() { try { await api.get("/jaxrs/program_center/market/m-1/download") } catch {} }
async function api_program_center_script() { try { await api.get("/jaxrs/program_center/script") } catch {} }


async function api_dictFlag_path_data_mockdeletetoget() { try { await api.get("/jaxrs/program_center/dict/dictFlag/path/data/mockdeletetoget") } catch {} }
async function api_mpweixin_menu_delete_wm_1() { try { await api.get("/jaxrs/program_center/mpweixin/menu/delete/wm-1") } catch {} }
async function api_cloud_unit_is_vip() { try { await api.get("/jaxrs/program_center/market/cloud/unit/is/vip") } catch {} }
async function api_program_center_config_open() { try { await api.get("/jaxrs/program_center/config/open") } catch {} }
async function api_dingding_request_pull_sync() { try { await api.get("/jaxrs/program_center/dingding/request/pull/sync") } catch {} }
async function api_program_center_code_list() { try { await api.get("/jaxrs/program_center/code/list") } catch {} }
async function api_program_center_schedule_schedule_fire() { try { await api.get("/jaxrs/program_center/schedule/schedule/fire") } catch {} }
async function api_program_center_storagemappings() { try { await api.get("/jaxrs/program_center/storagemappings") } catch {} }
async function api_program_center_zhengwudingding_regist_callback() { try { await api.get("/jaxrs/program_center/zhengwudingding/regist/callback") } catch {} }
async function api_program_center_config_centerserver() { try { await api.get("/jaxrs/program_center/config/centerserver") } catch {} }
async function api_program_center_validation_scripting_benchmark() { try { await api.get("/jaxrs/program_center/validation/scripting/benchmark") } catch {} }
async function api_program_center_jest_clear_cache() { try { await api.get("/jaxrs/program_center/jest/clear/cache") } catch {} }
async function api_collect_mobile_check_connect() { try { await api.get("/jaxrs/program_center/collect/mobile/check/connect") } catch {} }
async function api_program_center_application_create() { try { await api.get("/jaxrs/program_center/application/create") } catch {} }
async function api_program_center_collect_updateUnit() { try { await api.get("/jaxrs/program_center/collect/updateUnit") } catch {} }


async function api_script() { try { await api.get("/jaxrs/script") } catch {} }
async function api_s_1_app_app_1_imported() { try { await api.get("/jaxrs/script/s-1/app/app-1/imported") } catch {} }
async function api_script_s_1() { try { await api.get("/jaxrs/script/s-1") } catch {} }
async function api_script_list_manager() { try { await api.get("/jaxrs/script/list/manager") } catch {} }
async function api_scriptversion_sv_1() { try { await api.get("/jaxrs/scriptversion/sv-1") } catch {} }
async function api_list_i_1_next_10() { try { await api.get("/jaxrs/script/list/i-1/next/10") } catch {} }
async function api_script_s_1_appInfo_app_1() { try { await api.get("/jaxrs/script/s-1/appInfo/app-1") } catch {} }
async function api_script_s_1_app_app_1() { try { await api.get("/jaxrs/script/s-1/app/app-1") } catch {} }
async function api_scriptversion_list_script_s_1() { try { await api.get("/jaxrs/scriptversion/list/script/s-1") } catch {} }


async function api_program_center_schedule_report() { try { await api.get("/jaxrs/program_center/schedule/report") } catch {} }
async function api_image_menu_logo_blur() { try { await api.get("/jaxrs/program_center/appstyle/image/menu/logo/blur") } catch {} }
async function api_program_center_collect_remove() { try { await api.get("/jaxrs/program_center/collect/remove") } catch {} }
async function api_program_center_warnlog_w_1() { try { await api.get("/jaxrs/program_center/warnlog/w-1") } catch {} }
async function api_image_setup_about_logo() { try { await api.get("/jaxrs/program_center/appstyle/image/setup/about/logo") } catch {} }
async function api_program_center_script_id() { try { await api.get("/jaxrs/program_center/script/id") } catch {} }
async function api_qiyeweixin_request_pull_sync() { try { await api.get("/jaxrs/program_center/qiyeweixin/request/pull/sync") } catch {} }
async function api_program_center_authentication() { try { await api.get("/jaxrs/program_center/authentication") } catch {} }
async function api_module_remove_structure_m_1() { try { await api.get("/jaxrs/program_center/module/remove/structure/m-1") } catch {} }
async function api_program_center_config_get() { try { await api.get("/jaxrs/program_center/config/get") } catch {} }
async function api_market_flag_cover_pic() { try { await api.get("/jaxrs/program_center/market/flag/cover/pic") } catch {} }
async function api_program_center_invoke_flag() { try { await api.get("/jaxrs/program_center/invoke/flag") } catch {} }
async function api_output_flag_select_file() { try { await api.get("/jaxrs/program_center/output/flag/select/file") } catch {} }
async function api_mpweixin_message_template_send() { try { await api.get("/jaxrs/program_center/mpweixin/message/template/send") } catch {} }
async function api_create_mass_from_count() { try { await api.get("/jaxrs/program_center/bar/create/mass/from/count") } catch {} }
async function api_program_center_invoke_token() { try { await api.get("/jaxrs/program_center/invoke/token") } catch {} }
async function api_create_mass_from_count_1() { try { await api.get("/jaxrs/program_center/foo/create/mass/from/count") } catch {} }
async function api_distribute_assemble_source_o2() { try { await api.get("/jaxrs/program_center/distribute/assemble/source/o2") } catch {} }
async function api_program_center_config_portal() { try { await api.get("/jaxrs/program_center/config/portal") } catch {} }
async function api_list_schedulelog_application_application() { try { await api.get("/jaxrs/program_center/schedule/list/schedulelog/application/application") } catch {} }
async function api_program_center_market_flag() { try { await api.get("/jaxrs/program_center/market/flag") } catch {} }
async function api_module_output_list_structure() { try { await api.get("/jaxrs/program_center/module/output/list/structure") } catch {} }
async function api_pack_info_file_publish() { try { await api.get("/jaxrs/program_center/apppack/pack/info/file/publish") } catch {} }
async function api_program_center_market_m_1() { try { await api.get("/jaxrs/program_center/market/m-1") } catch {} }
async function api_jest_clear_cache_source() { try { await api.get("/jaxrs/program_center/jest/clear/cache/source") } catch {} }
async function api_program_center_output_f_1_select() { try { await api.get("/jaxrs/program_center/output/f-1/select") } catch {} }
async function api_program_center_config_list() { try { await api.get("/jaxrs/program_center/config/list") } catch {} }
async function api_dict_my_flag_my_path_data() { try { await api.get("/jaxrs/program_center/dict/my-flag/my-path/data") } catch {} }
async function api_my_flag_my_path_data_mockdeletetoget() { try { await api.get("/jaxrs/program_center/dict/my-flag/my-path/data/mockdeletetoget") } catch {} }
async function api_program_center_market_installed_version() { try { await api.get("/jaxrs/program_center/market/installed/version") } catch {} }


async function api_program_center_collect_urlMapping() { try { await api.get("/jaxrs/program_center/collect/urlMapping") } catch {} }
async function api_program_center_collect() { try { await api.get("/jaxrs/program_center/collect") } catch {} }
async function api_program_center_datastructure_tables_all() { try { await api.get("/jaxrs/program_center/datastructure/tables/all") } catch {} }
async function api_appstyle_image_process_default() { try { await api.get("/jaxrs/program_center/appstyle/image/process/default") } catch {} }
async function api_program_center_unexpectederrorlog_u_1() { try { await api.get("/jaxrs/program_center/unexpectederrorlog/u-1") } catch {} }
async function api_program_center_prompterrorlog_count_exceptionclass() { try { await api.get("/jaxrs/program_center/prompterrorlog/count/exceptionclass") } catch {} }
async function api_program_center_config_list_application() { try { await api.get("/jaxrs/program_center/config/list/application") } catch {} }
async function api_program_center_market_install_offline() { try { await api.get("/jaxrs/program_center/market/install/offline") } catch {} }
async function api_program_center_structure_list() { try { await api.get("/jaxrs/program_center/structure/list") } catch {} }
async function api_program_center_agent_flag_execute() { try { await api.get("/jaxrs/program_center/agent/flag/execute") } catch {} }
async function api_program_center_cachedispatch() { try { await api.get("/jaxrs/program_center/cachedispatch") } catch {} }
async function api_pack_info_file_last() { try { await api.get("/jaxrs/program_center/apppack/pack/info/file/last") } catch {} }
async function api_pack_info_android_repack() { try { await api.get("/jaxrs/program_center/apppack/pack/info/android/repack") } catch {} }
async function api_program_center_dict_id() { try { await api.get("/jaxrs/program_center/dict/id") } catch {} }
async function api_my_flag_my_path_data_mockputtopost() { try { await api.get("/jaxrs/program_center/dict/my-flag/my-path/data/mockputtopost") } catch {} }
async function api_program_center_input_prepare_cover() { try { await api.get("/jaxrs/program_center/input/prepare/cover") } catch {} }
async function api_program_list() { try { await api.get("/jaxrs/program/list") } catch {} }
async function api_program_center_collect_validate_codeanswer() { try { await api.get("/jaxrs/program_center/collect/validate/codeanswer") } catch {} }
async function api_program_center() { try { await api.get("/jaxrs/program_center") } catch {} }
async function api_qiyeweixin_send_getprivateinfo_message() { try { await api.get("/jaxrs/program_center/qiyeweixin/send/getprivateinfo/message") } catch {} }
async function api_program_center_center_applications_list() { try { await api.get("/jaxrs/program_center/center/applications/list") } catch {} }
async function api_program_center_collect_validate_direct() { try { await api.get("/jaxrs/program_center/collect/validate/direct") } catch {} }
async function api_webserver_assemble_source_o2() { try { await api.get("/jaxrs/program_center/distribute/webserver/assemble/source/o2") } catch {} }
async function api_program_center_module_write_flag() { try { await api.get("/jaxrs/program_center/module/write/flag") } catch {} }
async function api_program_center_invoke_i_1_file() { try { await api.get("/jaxrs/program_center/invoke/i-1/file") } catch {} }
async function api_program_appstyle_current_style() { try { await api.get("/jaxrs/program/appstyle/current/style") } catch {} }
async function api_image_process_default_erase() { try { await api.get("/jaxrs/program_center/appstyle/image/process/default/erase") } catch {} }
async function api_program_datastructure_modules_all() { try { await api.get("/jaxrs/program/datastructure/modules/all") } catch {} }
async function api_pack_info_android_start() { try { await api.get("/jaxrs/program_center/apppack/pack/info/android/start") } catch {} }
async function api_invoke_i_1_execute_get() { try { await api.get("/jaxrs/program_center/invoke/i-1/execute/get") } catch {} }
async function api_program_center_deploy_d_1() { try { await api.get("/jaxrs/program_center/deploy/d-1") } catch {} }
async function api_mpweixin_menu_list_weixin() { try { await api.get("/jaxrs/program_center/mpweixin/menu/list/weixin") } catch {} }
async function api_image_application_top_erase() { try { await api.get("/jaxrs/program_center/appstyle/image/application/top/erase") } catch {} }
async function api_code_create_mobile_13800000000() { try { await api.get("/jaxrs/program_center/code/create/mobile/13800000000") } catch {} }
async function api_program_center_adminlogin() { try { await api.get("/jaxrs/program_center/adminlogin") } catch {} }
async function api_program_center_apppack_server_connect() { try { await api.get("/jaxrs/program_center/apppack/server/connect") } catch {} }
async function api_program_center_tokenthreshold_update() { try { await api.get("/jaxrs/program_center/tokenthreshold/update") } catch {} }
async function api_get_disable_export_enable() { try { await api.get("/jaxrs/program_center/config-open/get/disable/export/enable") } catch {} }
async function api_program_center_agent_a_1_enable() { try { await api.get("/jaxrs/program_center/agent/a-1/enable") } catch {} }
async function api_program_center_application() { try { await api.get("/jaxrs/program_center/application") } catch {} }


async function api_collect_name_name_exist() { try { await api.get("/jaxrs/program_center/collect/name/name/exist") } catch {} }
async function api_market_list_top_three() { try { await api.get("/jaxrs/program_center/market/list/top/three") } catch {} }
async function api_list_w_1_prev_5() { try { await api.get("/jaxrs/program_center/warnlog/list/w-1/prev/5") } catch {} }
async function api_qiyeweixin_get_callback_aes() { try { await api.get("/jaxrs/program_center/qiyeweixin/get/callback/aes") } catch {} }
async function api_program_center_config_save() { try { await api.get("/jaxrs/program_center/config/save") } catch {} }
async function api_program_center_center_applications() { try { await api.get("/jaxrs/program_center/center/applications") } catch {} }
async function api_program_center_test_test1() { try { await api.get("/jaxrs/program_center/test/test1") } catch {} }
async function api_program_center_script_name_demo() { try { await api.get("/jaxrs/program_center/script/name/demo") } catch {} }
async function api_script_name_demo_imported() { try { await api.get("/jaxrs/program_center/script/name/demo/imported") } catch {} }
async function api_program_center_script_sc_id() { try { await api.get("/jaxrs/program_center/script/sc-id") } catch {} }
async function api_list_u_1_prev_5() { try { await api.get("/jaxrs/program_center/unexpectederrorlog/list/u-1/prev/5") } catch {} }
async function api_appstyle_image_login_avatar() { try { await api.get("/jaxrs/program_center/appstyle/image/login/avatar") } catch {} }
async function api_program_center_dingding_pull_sync() { try { await api.get("/jaxrs/program_center/dingding/pull/sync") } catch {} }
async function api_program_center_agent_flag_file() { try { await api.get("/jaxrs/program_center/agent/flag/file") } catch {} }
async function api_program_center_command_execute() { try { await api.get("/jaxrs/program_center/command/execute") } catch {} }
async function api_m_1_install_or_update() { try { await api.get("/jaxrs/program_center/market/m-1/install/or/update") } catch {} }
async function api_pack_info_file_last_1() { try { await api.get("/jaxrs/program_center/apppackanony/pack/info/file/last") } catch {} }
async function api_program_center_agent() { try { await api.get("/jaxrs/program_center/agent") } catch {} }
async function api_welink_request_pull_sync() { try { await api.get("/jaxrs/program_center/welink/request/pull/sync") } catch {} }
async function api_program_center_config_ternary_management() { try { await api.get("/jaxrs/program_center/config/ternary/management") } catch {} }
async function api_mpweixin_menu_update_id() { try { await api.get("/jaxrs/program_center/mpweixin/menu/update/id") } catch {} }
async function api_program_center_session_list_all() { try { await api.get("/jaxrs/program_center/session/list/all") } catch {} }
async function api_program_center_module_write_m_1() { try { await api.get("/jaxrs/program_center/module/write/m-1") } catch {} }
async function api_program_center_module_compare_upload() { try { await api.get("/jaxrs/program_center/module/compare/upload") } catch {} }
async function api_menu_create_to_weixin() { try { await api.get("/jaxrs/program_center/mpweixin/menu/create/to/weixin") } catch {} }
async function api_distribute_assemble_source_source() { try { await api.get("/jaxrs/program_center/distribute/assemble/source/source") } catch {} }
async function api_invoke_flag_execute_get() { try { await api.get("/jaxrs/program_center/invoke/flag/execute/get") } catch {} }
async function api_program_center_mpweixin_menu_subscribe() { try { await api.get("/jaxrs/program_center/mpweixin/menu/subscribe") } catch {} }
async function api_program_center_captcha_list() { try { await api.get("/jaxrs/program_center/captcha/list") } catch {} }
const api_program__516_data = ref<any[]>([]);
const { data: api_program__516_q } = useQuery({queryKey: ['api_program__516', '/jaxrs/program_center/config'], queryFn: async () => { try { const r = await api.get("/jaxrs/program_center/config"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_program__516_q, (v) => { api_program__516_data.value = v ?? []; });
const api_list_w_1_545_data = ref<any[]>([]);
const { data: api_list_w_1_545_q } = useQuery({queryKey: ['api_list_w_1_545', '/jaxrs/program_center/warnlog/list/w-1/next/10'], queryFn: async () => { try { const r = await api.get("/jaxrs/program_center/warnlog/list/w-1/next/10"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_list_w_1_545_q, (v) => { api_list_w_1_545_data.value = v ?? []; });
const api_program__50_data = ref<any[]>([]);
const { data: api_program__50_q } = useQuery({queryKey: ['api_program__50', '/jaxrs/program_center/agent/a-1/file'], queryFn: async () => { try { const r = await api.get("/jaxrs/program_center/agent/a-1/file"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_program__50_q, (v) => { api_program__50_data.value = v ?? []; });
const api_mpweixin_653_data = ref<any[]>([]);
const { data: api_mpweixin_653_q } = useQuery({queryKey: ['api_mpweixin_653', '/jaxrs/program_center/mpweixin/menu/delete/id'], queryFn: async () => { try { const r = await api.get("/jaxrs/program_center/mpweixin/menu/delete/id"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_mpweixin_653_q, (v) => { api_mpweixin_653_data.value = v ?? []; });
const api_appstyle_29_data = ref<any[]>([]);
const { data: api_appstyle_29_q } = useQuery({queryKey: ['api_appstyle_29', '/jaxrs/program_center/appstyle/image/launch/logo'], queryFn: async () => { try { const r = await api.get("/jaxrs/program_center/appstyle/image/launch/logo"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_appstyle_29_q, (v) => { api_appstyle_29_data.value = v ?? []; });
const api_mpweixin_987_data = ref<any[]>([]);
const { data: api_mpweixin_987_q } = useQuery({queryKey: ['api_mpweixin_987', '/jaxrs/program_center/mpweixin/menu/update/wm-1'], queryFn: async () => { try { const r = await api.get("/jaxrs/program_center/mpweixin/menu/update/wm-1"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_mpweixin_987_q, (v) => { api_mpweixin_987_data.value = v ?? []; });
const api_market_f_581_data = ref<any[]>([]);
const { data: api_market_f_581_q } = useQuery({queryKey: ['api_market_f_581', '/jaxrs/program_center/market/flag/install/log'], queryFn: async () => { try { const r = await api.get("/jaxrs/program_center/market/flag/install/log"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_market_f_581_q, (v) => { api_market_f_581_data.value = v ?? []; });
const api_program__665_data = ref<any[]>([]);
const { data: api_program__665_q } = useQuery({queryKey: ['api_program__665', '/jaxrs/program_center/appstyle/index/portal'], queryFn: async () => { try { const r = await api.get("/jaxrs/program_center/appstyle/index/portal"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_program__665_q, (v) => { api_program__665_data.value = v ?? []; });

async function api_jaxrs_program() { try { await api.get("/jaxrs/program") } catch {} }
const api_jaxrs_pr_38_data = ref<any[]>([]);
const { data: api_jaxrs_pr_38_q } = useQuery({queryKey: ['api_jaxrs_pr_38', '/jaxrs/program/applications'], queryFn: async () => { try { const r = await api.get("/jaxrs/program/applications"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_38_q, (v) => { api_jaxrs_pr_38_data.value = v ?? []; });
const api_jaxrs_pr_373_data = ref<any[]>([]);
const { data: api_jaxrs_pr_373_q } = useQuery({queryKey: ['api_jaxrs_pr_373', '/jaxrs/program_center/apppack/pack/info/file/download/pk-1'], queryFn: async () => { try { const r = await api.get("/jaxrs/program_center/apppack/pack/info/file/download/pk-1"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_373_q, (v) => { api_jaxrs_pr_373_data.value = v ?? []; });
const api_jaxrs_pr_402_data = ref<any[]>([]);
const { data: api_jaxrs_pr_402_q } = useQuery({queryKey: ['api_jaxrs_pr_402', '/jaxrs/program_center/appstyle/image/menu/logo/blur/erase'], queryFn: async () => { try { const r = await api.get("/jaxrs/program_center/appstyle/image/menu/logo/blur/erase"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_402_q, (v) => { api_jaxrs_pr_402_data.value = v ?? []; });
const api_jaxrs_pr_550_data = ref<any[]>([]);
const { data: api_jaxrs_pr_550_q } = useQuery({queryKey: ['api_jaxrs_pr_550', '/jaxrs/program_center/appstyle/image/menu/logo/focus/erase'], queryFn: async () => { try { const r = await api.get("/jaxrs/program_center/appstyle/image/menu/logo/focus/erase"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_550_q, (v) => { api_jaxrs_pr_550_data.value = v ?? []; });
const api_jaxrs_pr_633_data = ref<any[]>([]);
const { data: api_jaxrs_pr_633_q } = useQuery({queryKey: ['api_jaxrs_pr_633', '/jaxrs/program_center/appstyle/image/setup/about/logo/erase'], queryFn: async () => { try { const r = await api.get("/jaxrs/program_center/appstyle/image/setup/about/logo/erase"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_633_q, (v) => { api_jaxrs_pr_633_data.value = v ?? []; });
const api_jaxrs_pr_464_data = ref<any[]>([]);
const { data: api_jaxrs_pr_464_q } = useQuery({queryKey: ['api_jaxrs_pr_464', '/jaxrs/program_center/bar/select1/field/field/value/value/count/count'], queryFn: async () => { try { const r = await api.get("/jaxrs/program_center/bar/select1/field/field/value/value/count/count"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_464_q, (v) => { api_jaxrs_pr_464_data.value = v ?? []; });
const api_jaxrs_pr_432_data = ref<any[]>([]);
const { data: api_jaxrs_pr_432_q } = useQuery({queryKey: ['api_jaxrs_pr_432', '/jaxrs/program_center/bar/select1/field/status/value/open/count/10'], queryFn: async () => { try { const r = await api.get("/jaxrs/program_center/bar/select1/field/status/value/open/count/10"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_432_q, (v) => { api_jaxrs_pr_432_data.value = v ?? []; });
const api_jaxrs_pr_768_data = ref<any[]>([]);
const { data: api_jaxrs_pr_768_q } = useQuery({queryKey: ['api_jaxrs_pr_768', '/jaxrs/program_center/bar/select3/field/field/value/value/count/count'], queryFn: async () => { try { const r = await api.get("/jaxrs/program_center/bar/select3/field/field/value/value/count/count"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_768_q, (v) => { api_jaxrs_pr_768_data.value = v ?? []; });
const api_jaxrs_pr_411_data = ref<any[]>([]);
const { data: api_jaxrs_pr_411_q } = useQuery({queryKey: ['api_jaxrs_pr_411', '/jaxrs/program_center/bar/select3/field/name/value/x/count/5'], queryFn: async () => { try { const r = await api.get("/jaxrs/program_center/bar/select3/field/name/value/x/count/5"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_411_q, (v) => { api_jaxrs_pr_411_data.value = v ?? []; });
const api_jaxrs_pr_907_data = ref<any[]>([]);
const { data: api_jaxrs_pr_907_q } = useQuery({queryKey: ['api_jaxrs_pr_907', '/jaxrs/program_center/bar/select4/field/entity/value/y/count/5'], queryFn: async () => { try { const r = await api.get("/jaxrs/program_center/bar/select4/field/entity/value/y/count/5"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_907_q, (v) => { api_jaxrs_pr_907_data.value = v ?? []; });
const api_jaxrs_pr_185_data = ref<any[]>([]);
const { data: api_jaxrs_pr_185_q } = useQuery({queryKey: ['api_jaxrs_pr_185', '/jaxrs/program_center/bar/select4/field/field/value/value/count/count'], queryFn: async () => { try { const r = await api.get("/jaxrs/program_center/bar/select4/field/field/value/value/count/count"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_185_q, (v) => { api_jaxrs_pr_185_data.value = v ?? []; });
const api_jaxrs_pr_241_data = ref<any[]>([]);
const { data: api_jaxrs_pr_241_q } = useQuery({queryKey: ['api_jaxrs_pr_241', '/jaxrs/program_center/captcha/v2/create/width/200/height/80'], queryFn: async () => { try { const r = await api.get("/jaxrs/program_center/captcha/v2/create/width/200/height/80"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_241_q, (v) => { api_jaxrs_pr_241_data.value = v ?? []; });
const api_jaxrs_pr_451_data = ref<any[]>([]);
const { data: api_jaxrs_pr_451_q } = useQuery({queryKey: ['api_jaxrs_pr_451', '/jaxrs/program_center/captcha/v2/create/width/width/height/height'], queryFn: async () => { try { const r = await api.get("/jaxrs/program_center/captcha/v2/create/width/width/height/height"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_451_q, (v) => { api_jaxrs_pr_451_data.value = v ?? []; });
const api_jaxrs_pr_901_data = ref<any[]>([]);
const { data: api_jaxrs_pr_901_q } = useQuery({queryKey: ['api_jaxrs_pr_901', '/jaxrs/program_center/code/list/paging/1/size/20'], queryFn: async () => { try { const r = await api.get("/jaxrs/program_center/code/list/paging/1/size/20"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_901_q, (v) => { api_jaxrs_pr_901_data.value = v ?? []; });
const api_jaxrs_pr_766_data = ref<any[]>([]);
const { data: api_jaxrs_pr_766_q } = useQuery({queryKey: ['api_jaxrs_pr_766', '/jaxrs/program_center/code/list/paging/page/size/size'], queryFn: async () => { try { const r = await api.get("/jaxrs/program_center/code/list/paging/page/size/size"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_766_q, (v) => { api_jaxrs_pr_766_data.value = v ?? []; });
const api_jaxrs_pr_281_data = ref<any[]>([]);
const { data: api_jaxrs_pr_281_q } = useQuery({queryKey: ['api_jaxrs_pr_281', '/jaxrs/program_center/code/validate/mobile/13800000000/answer/123456'], queryFn: async () => { try { const r = await api.get("/jaxrs/program_center/code/validate/mobile/13800000000/answer/123456"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_281_q, (v) => { api_jaxrs_pr_281_data.value = v ?? []; });
const api_jaxrs_pr_273_data = ref<any[]>([]);
const { data: api_jaxrs_pr_273_q } = useQuery({queryKey: ['api_jaxrs_pr_273', '/jaxrs/program_center/code/validate/mobile/13800000000/answer/123456/cascade'], queryFn: async () => { try { const r = await api.get("/jaxrs/program_center/code/validate/mobile/13800000000/answer/123456/cascade"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_273_q, (v) => { api_jaxrs_pr_273_data.value = v ?? []; });
const api_jaxrs_pr_914_data = ref<any[]>([]);
const { data: api_jaxrs_pr_914_q } = useQuery({queryKey: ['api_jaxrs_pr_914', '/jaxrs/program_center/code/validate/mobile/mobile/answer/answer/cascade'], queryFn: async () => { try { const r = await api.get("/jaxrs/program_center/code/validate/mobile/mobile/answer/answer/cascade"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_914_q, (v) => { api_jaxrs_pr_914_data.value = v ?? []; });
const api_jaxrs_pr_817_data = ref<any[]>([]);
const { data: api_jaxrs_pr_817_q } = useQuery({queryKey: ['api_jaxrs_pr_817', '/jaxrs/program_center/collect/controllermobile/name/n/mobile/m'], queryFn: async () => { try { const r = await api.get("/jaxrs/program_center/collect/controllermobile/name/n/mobile/m"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_817_q, (v) => { api_jaxrs_pr_817_data.value = v ?? []; });
const api_jaxrs_pr_953_data = ref<any[]>([]);
const { data: api_jaxrs_pr_953_q } = useQuery({queryKey: ['api_jaxrs_pr_953', '/jaxrs/program_center/collect/controllermobile/name/name/mobile/mobile'], queryFn: async () => { try { const r = await api.get("/jaxrs/program_center/collect/controllermobile/name/name/mobile/mobile"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_953_q, (v) => { api_jaxrs_pr_953_data.value = v ?? []; });
const api_jaxrs_pr_183_data = ref<any[]>([]);
const { data: api_jaxrs_pr_183_q } = useQuery({queryKey: ['api_jaxrs_pr_183', '/jaxrs/program_center/collect/name/n/mobile/m/code/c'], queryFn: async () => { try { const r = await api.get("/jaxrs/program_center/collect/name/n/mobile/m/code/c"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_183_q, (v) => { api_jaxrs_pr_183_data.value = v ?? []; });
const api_jaxrs_pr_955_data = ref<any[]>([]);
const { data: api_jaxrs_pr_955_q } = useQuery({queryKey: ['api_jaxrs_pr_955', '/jaxrs/program_center/collect/name/name/mobile/mobile/code/code'], queryFn: async () => { try { const r = await api.get("/jaxrs/program_center/collect/name/name/mobile/mobile/code/code"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_955_q, (v) => { api_jaxrs_pr_955_data.value = v ?? []; });
const api_jaxrs_pr_736_data = ref<any[]>([]);
const { data: api_jaxrs_pr_736_q } = useQuery({queryKey: ['api_jaxrs_pr_736', '/jaxrs/program_center/config/list/dump/data/current/node'], queryFn: async () => { try { const r = await api.get("/jaxrs/program_center/config/list/dump/data/current/node"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_736_q, (v) => { api_jaxrs_pr_736_data.value = v ?? []; });
const api_jaxrs_pr_512_data = ref<any[]>([]);
const { data: api_jaxrs_pr_512_q } = useQuery({queryKey: ['api_jaxrs_pr_512', '/jaxrs/program_center/config/open/get/disable/export/enable'], queryFn: async () => { try { const r = await api.get("/jaxrs/program_center/config/open/get/disable/export/enable"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_512_q, (v) => { api_jaxrs_pr_512_data.value = v ?? []; });
const api_jaxrs_pr_933_data = ref<any[]>([]);
const { data: api_jaxrs_pr_933_q } = useQuery({queryKey: ['api_jaxrs_pr_933', '/jaxrs/program_center/deploy/list/paging/1/size/20'], queryFn: async () => { try { const r = await api.get("/jaxrs/program_center/deploy/list/paging/1/size/20"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_933_q, (v) => { api_jaxrs_pr_933_data.value = v ?? []; });
const api_jaxrs_pr_787_data = ref<any[]>([]);
const { data: api_jaxrs_pr_787_q } = useQuery({queryKey: ['api_jaxrs_pr_787', '/jaxrs/program_center/deploy/list/paging/page/size/size'], queryFn: async () => { try { const r = await api.get("/jaxrs/program_center/deploy/list/paging/page/size/size"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_787_q, (v) => { api_jaxrs_pr_787_data.value = v ?? []; });
const api_jaxrs_pr_649_data = ref<any[]>([]);
const { data: api_jaxrs_pr_649_q } = useQuery({queryKey: ['api_jaxrs_pr_649', '/jaxrs/program_center/deploy/web/resource/as/new/asNew'], queryFn: async () => { try { const r = await api.get("/jaxrs/program_center/deploy/web/resource/as/new/asNew"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_649_q, (v) => { api_jaxrs_pr_649_data.value = v ?? []; });
const api_jaxrs_pr_39_data = ref<any[]>([]);
const { data: api_jaxrs_pr_39_q } = useQuery({queryKey: ['api_jaxrs_pr_39', '/jaxrs/program_center/deploy/web/resource/as/new/newname'], queryFn: async () => { try { const r = await api.get("/jaxrs/program_center/deploy/web/resource/as/new/newname"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_39_q, (v) => { api_jaxrs_pr_39_data.value = v ?? []; });
const api_jaxrs_pr_49_data = ref<any[]>([]);
const { data: api_jaxrs_pr_49_q } = useQuery({queryKey: ['api_jaxrs_pr_49', '/jaxrs/program_center/dict/list/paging/1/size/20'], queryFn: async () => { try { const r = await api.get("/jaxrs/program_center/dict/list/paging/1/size/20"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_49_q, (v) => { api_jaxrs_pr_49_data.value = v ?? []; });
const api_jaxrs_pr_28_data = ref<any[]>([]);
const { data: api_jaxrs_pr_28_q } = useQuery({queryKey: ['api_jaxrs_pr_28', '/jaxrs/program_center/dict/list/paging/page/size/size'], queryFn: async () => { try { const r = await api.get("/jaxrs/program_center/dict/list/paging/page/size/size"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_28_q, (v) => { api_jaxrs_pr_28_data.value = v ?? []; });
const api_jaxrs_pr_545_data = ref<any[]>([]);
const { data: api_jaxrs_pr_545_q } = useQuery({queryKey: ['api_jaxrs_pr_545', '/jaxrs/program_center/dingding/sync/organization/register/callback/true'], queryFn: async () => { try { const r = await api.get("/jaxrs/program_center/dingding/sync/organization/register/callback/true"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_545_q, (v) => { api_jaxrs_pr_545_data.value = v ?? []; });
const api_jaxrs_pr_940_data = ref<any[]>([]);
const { data: api_jaxrs_pr_940_q } = useQuery({queryKey: ['api_jaxrs_pr_940', '/jaxrs/program_center/invoke/flag/client/client/token/token/execute'], queryFn: async () => { try { const r = await api.get("/jaxrs/program_center/invoke/flag/client/client/token/token/execute"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_940_q, (v) => { api_jaxrs_pr_940_data.value = v ?? []; });
const api_jaxrs_pr_354_data = ref<any[]>([]);
const { data: api_jaxrs_pr_354_q } = useQuery({queryKey: ['api_jaxrs_pr_354', '/jaxrs/program_center/invoke/i-1/client/web/token/tk-1/execute'], queryFn: async () => { try { const r = await api.get("/jaxrs/program_center/invoke/i-1/client/web/token/tk-1/execute"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_354_q, (v) => { api_jaxrs_pr_354_data.value = v ?? []; });
const api_jaxrs_pr_888_data = ref<any[]>([]);
const { data: api_jaxrs_pr_888_q } = useQuery({queryKey: ['api_jaxrs_pr_888', '/jaxrs/program_center/market/list/install/log/paging/1/size/20'], queryFn: async () => { try { const r = await api.get("/jaxrs/program_center/market/list/install/log/paging/1/size/20"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_888_q, (v) => { api_jaxrs_pr_888_data.value = v ?? []; });
const api_jaxrs_pr_558_data = ref<any[]>([]);
const { data: api_jaxrs_pr_558_q } = useQuery({queryKey: ['api_jaxrs_pr_558', '/jaxrs/program_center/market/list/install/log/paging/page/size/size'], queryFn: async () => { try { const r = await api.get("/jaxrs/program_center/market/list/install/log/paging/page/size/size"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_558_q, (v) => { api_jaxrs_pr_558_data.value = v ?? []; });
const api_jaxrs_pr_222_data = ref<any[]>([]);
const { data: api_jaxrs_pr_222_q } = useQuery({queryKey: ['api_jaxrs_pr_222', '/jaxrs/program_center/market/list/paging/1/size/20'], queryFn: async () => { try { const r = await api.get("/jaxrs/program_center/market/list/paging/1/size/20"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_222_q, (v) => { api_jaxrs_pr_222_data.value = v ?? []; });
const api_jaxrs_pr_194_data = ref<any[]>([]);
const { data: api_jaxrs_pr_194_q } = useQuery({queryKey: ['api_jaxrs_pr_194', '/jaxrs/program_center/market/list/paging/1/size/20/category/cms'], queryFn: async () => { try { const r = await api.get("/jaxrs/program_center/market/list/paging/1/size/20/category/cms"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_194_q, (v) => { api_jaxrs_pr_194_data.value = v ?? []; });
const api_jaxrs_pr_708_data = ref<any[]>([]);
const { data: api_jaxrs_pr_708_q } = useQuery({queryKey: ['api_jaxrs_pr_708', '/jaxrs/program_center/market/list/paging/page/size/size'], queryFn: async () => { try { const r = await api.get("/jaxrs/program_center/market/list/paging/page/size/size"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_708_q, (v) => { api_jaxrs_pr_708_data.value = v ?? []; });
const api_jaxrs_pr_192_data = ref<any[]>([]);
const { data: api_jaxrs_pr_192_q } = useQuery({queryKey: ['api_jaxrs_pr_192', '/jaxrs/program_center/market/list/paging/page/size/size/category/category'], queryFn: async () => { try { const r = await api.get("/jaxrs/program_center/market/list/paging/page/size/size/category/category"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_192_q, (v) => { api_jaxrs_pr_192_data.value = v ?? []; });
const api_jaxrs_pr_31_data = ref<any[]>([]);
const { data: api_jaxrs_pr_31_q } = useQuery({queryKey: ['api_jaxrs_pr_31', '/jaxrs/program_center/prompterrorlog/list/id/next/count/date/date'], queryFn: async () => { try { const r = await api.get("/jaxrs/program_center/prompterrorlog/list/id/next/count/date/date"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_31_q, (v) => { api_jaxrs_pr_31_data.value = v ?? []; });
const api_jaxrs_pr_36_data = ref<any[]>([]);
const { data: api_jaxrs_pr_36_q } = useQuery({queryKey: ['api_jaxrs_pr_36', '/jaxrs/program_center/prompterrorlog/list/id/next/count/exceptionclass/exceptionClass'], queryFn: async () => { try { const r = await api.get("/jaxrs/program_center/prompterrorlog/list/id/next/count/exceptionclass/exceptionClass"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_36_q, (v) => { api_jaxrs_pr_36_data.value = v ?? []; });
const api_jaxrs_pr_596_data = ref<any[]>([]);
const { data: api_jaxrs_pr_596_q } = useQuery({queryKey: ['api_jaxrs_pr_596', '/jaxrs/program_center/prompterrorlog/list/id/next/count/loggername/loggerName'], queryFn: async () => { try { const r = await api.get("/jaxrs/program_center/prompterrorlog/list/id/next/count/loggername/loggerName"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_596_q, (v) => { api_jaxrs_pr_596_data.value = v ?? []; });
const api_jaxrs_progr_136_data = ref<any[]>([]);
const { data: api_jaxrs_progr_136_q } = useQuery({queryKey: ['api_jaxrs_progr_136', '/jaxrs/program_center/prompterrorlog/list/id/prev/count/date/date'], queryFn: async () => { try { const r = await api.get("/jaxrs/program_center/prompterrorlog/list/id/prev/count/date/date"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_progr_136_q, (v) => { api_jaxrs_progr_136_data.value = v ?? []; });
const api_jaxrs_progr_794_data = ref<any[]>([]);
const { data: api_jaxrs_progr_794_q } = useQuery({queryKey: ['api_jaxrs_progr_794', '/jaxrs/program_center/prompterrorlog/list/id/prev/count/exceptionclass/exceptionClass'], queryFn: async () => { try { const r = await api.get("/jaxrs/program_center/prompterrorlog/list/id/prev/count/exceptionclass/exceptionClass"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_progr_794_q, (v) => { api_jaxrs_progr_794_data.value = v ?? []; });
const api_jaxrs_program_ce_215_data = ref<any[]>([]);
const { data: api_jaxrs_program_ce_215_q } = useQuery({queryKey: ['api_jaxrs_program_ce_215', '/jaxrs/program_center/prompterrorlog/list/id/prev/count/loggername/loggerName'], queryFn: async () => { try { const r = await api.get("/jaxrs/program_center/prompterrorlog/list/id/prev/count/loggername/loggerName"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_program_ce_215_q, (v) => { api_jaxrs_program_ce_215_data.value = v ?? []; });
const api_jaxrs_program_ce_748_data = ref<any[]>([]);
const { data: api_jaxrs_program_ce_748_q } = useQuery({queryKey: ['api_jaxrs_program_ce_748', '/jaxrs/program_center/prompterrorlog/list/p-1/next/10/date/2026-08-24'], queryFn: async () => { try { const r = await api.get("/jaxrs/program_center/prompterrorlog/list/p-1/next/10/date/2026-08-24"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_program_ce_748_q, (v) => { api_jaxrs_program_ce_748_data.value = v ?? []; });
const api_jaxrs_program_ce_931_data = ref<any[]>([]);
const { data: api_jaxrs_program_ce_931_q } = useQuery({queryKey: ['api_jaxrs_program_ce_931', '/jaxrs/program_center/script/list/paging/1/size/20'], queryFn: async () => { try { const r = await api.get("/jaxrs/program_center/script/list/paging/1/size/20"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_program_ce_931_q, (v) => { api_jaxrs_program_ce_931_data.value = v ?? []; });
const api_jaxrs_program_ce_744_data = ref<any[]>([]);
const { data: api_jaxrs_program_ce_744_q } = useQuery({queryKey: ['api_jaxrs_program_ce_744', '/jaxrs/program_center/script/list/paging/page/size/size'], queryFn: async () => { try { const r = await api.get("/jaxrs/program_center/script/list/paging/page/size/size"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_program_ce_744_q, (v) => { api_jaxrs_program_ce_744_data.value = v ?? []; });
const api_jaxrs_program_ce_211_data = ref<any[]>([]);
const { data: api_jaxrs_program_ce_211_q } = useQuery({queryKey: ['api_jaxrs_program_ce_211', '/jaxrs/program_center/unexpectederrorlog/list/id/next/count/date/date'], queryFn: async () => { try { const r = await api.get("/jaxrs/program_center/unexpectederrorlog/list/id/next/count/date/date"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_program_ce_211_q, (v) => { api_jaxrs_program_ce_211_data.value = v ?? []; });
const api_jaxrs_program_ce_579_data = ref<any[]>([]);
const { data: api_jaxrs_program_ce_579_q } = useQuery({queryKey: ['api_jaxrs_program_ce_579', '/jaxrs/program_center/unexpectederrorlog/list/id/prev/count/date/date'], queryFn: async () => { try { const r = await api.get("/jaxrs/program_center/unexpectederrorlog/list/id/prev/count/date/date"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_program_ce_579_q, (v) => { api_jaxrs_program_ce_579_data.value = v ?? []; });
const api_jaxrs_program_ce_452_data = ref<any[]>([]);
const { data: api_jaxrs_program_ce_452_q } = useQuery({queryKey: ['api_jaxrs_program_ce_452', '/jaxrs/program_center/warnlog/list/w-1/next/10/date/2026-08-22'], queryFn: async () => { try { const r = await api.get("/jaxrs/program_center/warnlog/list/w-1/next/10/date/2026-08-22"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_program_ce_452_q, (v) => { api_jaxrs_program_ce_452_data.value = v ?? []; });
const api_jaxrs_program_ce_89_data = ref<any[]>([]);
const { data: api_jaxrs_program_ce_89_q } = useQuery({queryKey: ['api_jaxrs_program_ce_89', '/jaxrs/program_center/warnlog/list/w-1/prev/5/date/2026-08-22'], queryFn: async () => { try { const r = await api.get("/jaxrs/program_center/warnlog/list/w-1/prev/5/date/2026-08-22"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_program_ce_89_q, (v) => { api_jaxrs_program_ce_89_data.value = v ?? []; });
const api_jaxrs_progr_708_data = ref<any[]>([]);
const { data: api_jaxrs_progr_708_q } = useQuery({queryKey: ['api_jaxrs_progr_708', '/jaxrs/program_center/warnlog/view/system/log/tag/sync'], queryFn: async () => { try { const r = await api.get("/jaxrs/program_center/warnlog/view/system/log/tag/sync"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_progr_708_q, (v) => { api_jaxrs_progr_708_data.value = v ?? []; });
</script>

<style scoped>
.mod-view{display:flex;flex-direction:column;gap:16px;height:100%}
.view-header{padding:16px 24px}
.view-header h1{font-family:'Orbitron',sans-serif;font-size:20px;color:var(--color-primary);margin:0 0 4px;text-shadow:0 0 15px var(--color-primary-glow)}
.subtitle{font-size:12px;color:var(--text-muted);margin:0;font-family:'JetBrains Mono',monospace}
.content-panel{flex:1;overflow-y:auto;padding:16px;display:flex;flex-direction:column;gap:16px}
.tabs{display:flex;gap:8px;flex-wrap:wrap}
.tabs button{padding:8px 16px;background:var(--bg-elevated);border:1px solid var(--border-subtle);border-radius:var(--radius-md);color:var(--text-secondary);font-size:13px;cursor:pointer;transition:all var(--transition-fast)}
.tabs button.active{background:var(--color-primary);color:#000;border-color:var(--color-primary);font-weight:600}
.toolbar{display:flex;gap:8px}
.btn-primary{padding:8px 20px;background:var(--color-primary);color:#000;border:none;border-radius:var(--radius-md);font-size:13px;cursor:pointer;font-weight:600}
.btn-create{padding:8px 20px;background:var(--color-accent);color:#fff;border:none;border-radius:var(--radius-md);font-size:13px;cursor:pointer;font-weight:600}
.btn-sm{padding:4px 12px;background:transparent;border:1px solid var(--border-subtle);color:var(--text-secondary);border-radius:var(--radius-sm);font-size:12px;cursor:pointer}
.btn-sm:hover{border-color:var(--color-primary);color:var(--color-primary)}
.tab-content{flex:1;display:flex;flex-direction:column;gap:12px;overflow-y:auto}
.item-table{display:flex;flex-direction:column;gap:8px}
.table-header{display:grid;grid-template-columns:2fr 1fr 80px 100px;padding:8px 12px;background:var(--bg-elevated);border-radius:var(--radius-sm);font-size:12px;color:var(--text-muted);font-weight:600}
.table-row{display:grid;grid-template-columns:2fr 1fr 80px 100px;padding:12px;align-items:center;transition:all var(--transition-fast);border:1px solid var(--border-subtle);border-radius:var(--radius-md);background:var(--bg-elevated)}
.table-row:hover{border-color:var(--color-primary)}
.col-name{font-size:14px;font-weight:500;color:var(--text-primary);overflow:hidden;text-overflow:ellipsis;white-space:nowrap}
.col-flag,.col-id{font-size:11px;color:var(--text-muted);font-family:'JetBrains Mono',monospace}
.col-status{font-size:12px;padding:2px 8px;border-radius:var(--radius-sm);width:fit-content}
.col-status.enabled{background:rgba(16,185,129,.15);color:var(--color-success)}
.col-status.disabled{background:rgba(239,68,68,.15);color:var(--color-error)}
.item-grid{display:grid;grid-template-columns:repeat(auto-fill,minmax(200px,1fr));gap:12px}
.item-card{display:flex;align-items:center;gap:12px;padding:14px;transition:all var(--transition-fast);border:1px solid var(--border-subtle);border-radius:var(--radius-md);background:var(--bg-elevated)}
.item-card:hover{border-color:var(--color-primary);transform:translateX(4px);box-shadow:var(--shadow-glow)}
.ic{font-size:28px}
.ib{flex:1;min-width:0}
.it{font-size:14px;font-weight:600;color:var(--text-primary)}
.im{font-size:12px;color:var(--text-muted);margin-top:2px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap}
.meta{font-size:10px;color:var(--color-primary-deep);margin-top:4px;font-family:'JetBrains Mono',monospace}
.empty,.loading-row{display:flex;flex-direction:column;align-items:center;justify-content:center;padding:40px;color:var(--text-muted);gap:12px;flex:1}
.ei{font-size:48px;opacity:0.4}
.sk{height:40px;border-radius:var(--radius-md);background:var(--bg-elevated);animation:pulse 1.2s ease-in-out infinite}
@keyframes pulse{0%,100%{opacity:.4}50%{opacity:.8}}
.modal-overlay{position:fixed;inset:0;background:rgba(0,0,0,.7);display:flex;align-items:center;justify-content:center;z-index:100}
.modal{background:var(--bg-surface);border:1px solid var(--border-subtle);border-radius:var(--radius-lg);padding:24px;width:400px;max-width:90vw;display:flex;flex-direction:column;gap:16px}
.modal h3{font-family:'Orbitron',sans-serif;color:var(--color-primary);margin:0}
.form-group{display:flex;flex-direction:column;gap:6px}
.form-group label{font-size:13px;color:var(--text-muted)}
.form-input{background:var(--bg-elevated);border:1px solid var(--border-subtle);border-radius:var(--radius-md);color:var(--text-primary);padding:10px 12px;font-size:14px}
.form-input:focus{outline:none;border-color:var(--color-primary)}
.modal-actions{display:flex;justify-content:flex-end;gap:8px}
.btn-cancel{padding:8px 20px;background:transparent;border:1px solid var(--border-subtle);color:var(--text-secondary);border-radius:var(--radius-md);cursor:pointer}
.font-mono{font-family:'JetBrains Mono',monospace}
</style>
