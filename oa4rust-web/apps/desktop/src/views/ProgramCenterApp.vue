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
          <button class="btn-primary" @click="loadAgents">刷新</button>
          <button class="btn-create" @click="showCreateAgent=true">+ 新建Agent</button>
        </div>
        <div v-if="loadingAgent" class="loading-row"><div class="sk" v-for="i in 4" :key="i"></div></div>
        <div v-else-if="agents.length===0" class="empty"><div class="ei">🤖</div><p>暂无Agent</p></div>
        <div v-else class="item-table">
          <div class="table-header"><span class="col-name">名称</span><span class="col-flag">Flag</span><span class="col-status">状态</span><span class="col-actions">操作</span></div>
          <div v-for="a in agents" :key="a.id" class="table-row glass-card">
            <span class="col-name">{{ a.name || a.label || a.agentName || '未命名' }}</span>
            <span class="col-flag font-mono">{{ a.flag || a.id }}</span>
            <span class="col-status" :class="a.enabled!==false?'enabled':'disabled'">{{ a.enabled!==false?'启用':'禁用' }}</span>
            <span class="col-actions">
              <button class="btn-sm" @click="toggleAgent(a)">
                {{ a.enabled!==false ? '禁用' : '启用' }}
              </button>
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
import { ref } from 'vue'
import { api } from '@oa4rust/sdk'

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
    if (a.enabled !== false) await api.post(`/jaxrs/program_center/agent/${a.flag || a.id}/disable`, null)
    else await api.post(`/jaxrs/program_center/agent/${a.flag || a.id}/enable`, null)
    loadAgents()
  } catch (e: any) { alert((e?.message ?? '操作失败')) }
}

async function onCreateAgent() {
  try {
    await api.post('/jaxrs/program_center/agent/create', agentForm.value)
    showCreateAgent.value = false
    agentForm.value = { name: '', flag: '' }
    loadAgents()
  } catch (e: any) { alert('创建失败: ' + (e?.message ?? '')) }
}

// Watch tab changes to load data
import { watch } from 'vue'
watch(tab, (t) => switchTab(t), { immediate: true })

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
