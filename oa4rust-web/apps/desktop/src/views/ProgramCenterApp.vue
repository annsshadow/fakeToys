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

const media_add_forever_ref = ref<any[]>([]);
const media_add_forever_q = useQuery({
  queryKey: ['media_add_forever'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/mpweixin/media/add/forever"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const login_avatar_erase_ref = ref<any[]>([]);
const login_avatar_erase_q = useQuery({
  queryKey: ['login_avatar_erase'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/appstyle/image/login/avatar/erase"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const file_download_pk_1_ref = ref<any[]>([]);
const file_download_pk_1_q = useQuery({
  queryKey: ['file_download_pk_1'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/apppackanony/pack/info/file/download/pk-1"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_config_proxy_ref = ref<any[]>([]);
const program_center_config_proxy_q = useQuery({
  queryKey: ['program_center_config_proxy'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/config/proxy"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const register_callback_enable_ref = ref<any[]>([]);
const register_callback_enable_q = useQuery({
  queryKey: ['register_callback_enable'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/dingding/sync/organization/register/callback/enable"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const with_category_category_ref = ref<any[]>([]);
const with_category_category_q = useQuery({
  queryKey: ['with_category_category'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/invoke/list/with/category/category"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const dingding_get_callback_ref = ref<any[]>([]);
const dingding_get_callback_q = useQuery({
  queryKey: ['dingding_get_callback'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/dingding/get/callback"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_config_person_ref = ref<any[]>([]);
const program_center_config_person_q = useQuery({
  queryKey: ['program_center_config_person'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/config/person"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_designer_search_ref = ref<any[]>([]);
const program_center_designer_search_q = useQuery({
  queryKey: ['program_center_designer_search'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/designer/search"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const output_appInfoFlag_select_ref = ref<any[]>([]);
const output_appInfoFlag_select_q = useQuery({
  queryKey: ['output_appInfoFlag_select'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/output/appInfoFlag/select"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_collect_person_ref = ref<any[]>([]);
const program_center_collect_person_q = useQuery({
  queryKey: ['program_center_collect_person'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/collect/person"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const mobile_answer_answer_ref = ref<any[]>([]);
const mobile_answer_answer_q = useQuery({
  queryKey: ['mobile_answer_answer'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/code/validate/mobile/mobile/answer/answer"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_prompterrorlog_id_ref = ref<any[]>([]);
const program_center_prompterrorlog_id_q = useQuery({
  queryKey: ['program_center_prompterrorlog_id'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/prompterrorlog/id"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_appstyle_ref = ref<any[]>([]);
const program_center_appstyle_q = useQuery({
  queryKey: ['program_center_appstyle'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/appstyle"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const id_prev_count_ref = ref<any[]>([]);
const id_prev_count_q = useQuery({
  queryKey: ['id_prev_count'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/prompterrorlog/list/id/prev/count"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const output_flag_file_ref = ref<any[]>([]);
const output_flag_file_q = useQuery({
  queryKey: ['output_flag_file'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/module/output/flag/file"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const mass_0_10_ref = ref<any[]>([]);
const mass_0_10_q = useQuery({
  queryKey: ['mass_0_10'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/bar/create/mass/0/10"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const module_id_compare_ref = ref<any[]>([]);
const module_id_compare_q = useQuery({
  queryKey: ['module_id_compare'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/module/id/compare"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const create_mobile_mobile_ref = ref<any[]>([]);
const create_mobile_mobile_q = useQuery({
  queryKey: ['create_mobile_mobile'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/code/create/mobile/mobile"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const m_1_install_log_ref = ref<any[]>([]);
const m_1_install_log_q = useQuery({
  queryKey: ['m_1_install_log'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/market/m-1/install/log"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});


const program_center_invoke_flag_execute_ref = ref<any[]>([]);
const program_center_invoke_flag_execute_q = useQuery({
  queryKey: ['program_center_invoke_flag_execute'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/invoke/flag/execute"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_jest_center_list_ref = ref<any[]>([]);
const program_center_jest_center_list_q = useQuery({
  queryKey: ['program_center_jest_center_list'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/jest/center/list"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const module_remove_structure_id_ref = ref<any[]>([]);
const module_remove_structure_id_q = useQuery({
  queryKey: ['module_remove_structure_id'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/module/remove/structure/id"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_collect_validate_ref = ref<any[]>([]);
const program_center_collect_validate_q = useQuery({
  queryKey: ['program_center_collect_validate'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/collect/validate"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_invoke_flag_file_ref = ref<any[]>([]);
const program_center_invoke_flag_file_q = useQuery({
  queryKey: ['program_center_invoke_flag_file'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/invoke/flag/file"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_invoke_ref = ref<any[]>([]);
const program_center_invoke_q = useQuery({
  queryKey: ['program_center_invoke'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/invoke"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_schedule_list_schedulelocal_ref = ref<any[]>([]);
const program_center_schedule_list_schedulelocal_q = useQuery({
  queryKey: ['program_center_schedule_list_schedulelocal'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/schedule/list/schedulelocal"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_collect_sync_area_ref = ref<any[]>([]);
const program_center_collect_sync_area_q = useQuery({
  queryKey: ['program_center_collect_sync_area'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/collect/sync/area"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_module_list_category_ref = ref<any[]>([]);
const program_center_module_list_category_q = useQuery({
  queryKey: ['program_center_module_list_category'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/module/list/category"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const collect_name_n_exist_ref = ref<any[]>([]);
const collect_name_n_exist_q = useQuery({
  queryKey: ['collect_name_n_exist'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/collect/name/n/exist"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_invoke_some_flag_ref = ref<any[]>([]);
const program_center_invoke_some_flag_q = useQuery({
  queryKey: ['program_center_invoke_some_flag'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/invoke/some-flag"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_andfx_pull_sync_ref = ref<any[]>([]);
const program_center_andfx_pull_sync_q = useQuery({
  queryKey: ['program_center_andfx_pull_sync'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/andfx/pull/sync"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_validation_timeout_30000_ref = ref<any[]>([]);
const program_center_validation_timeout_30000_q = useQuery({
  queryKey: ['program_center_validation_timeout_30000'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/validation/timeout/30000"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_output_list_ref = ref<any[]>([]);
const program_center_output_list_q = useQuery({
  queryKey: ['program_center_output_list'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/output/list"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_deploy_server_o2_ref = ref<any[]>([]);
const program_center_deploy_server_o2_q = useQuery({
  queryKey: ['program_center_deploy_server_o2'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/deploy/server/o2"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_collect_controllebbs_ref = ref<any[]>([]);
const program_center_collect_controllebbs_q = useQuery({
  queryKey: ['program_center_collect_controllebbs'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/collect/controllebbs"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_datastructure_fileds_all_ref = ref<any[]>([]);
const program_center_datastructure_fileds_all_q = useQuery({
  queryKey: ['program_center_datastructure_fileds_all'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/datastructure/fileds/all"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_module_m_1_compare_ref = ref<any[]>([]);
const program_center_module_m_1_compare_q = useQuery({
  queryKey: ['program_center_module_m_1_compare'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/module/m-1/compare"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_unexpectederrorlog_ref = ref<any[]>([]);
const program_center_unexpectederrorlog_q = useQuery({
  queryKey: ['program_center_unexpectederrorlog'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/unexpectederrorlog"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_applications_ref = ref<any[]>([]);
const program_center_applications_q = useQuery({
  queryKey: ['program_center_applications'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/applications"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_deploy_id_ref = ref<any[]>([]);
const program_center_deploy_id_q = useQuery({
  queryKey: ['program_center_deploy_id'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/deploy/id"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const market_m_1_installed_version_ref = ref<any[]>([]);
const market_m_1_installed_version_q = useQuery({
  queryKey: ['market_m_1_installed_version'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/market/m-1/installed/version"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_market_id_download_ref = ref<any[]>([]);
const program_center_market_id_download_q = useQuery({
  queryKey: ['program_center_market_id_download'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/market/id/download"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_market_m_1_uninstall_ref = ref<any[]>([]);
const program_center_market_m_1_uninstall_q = useQuery({
  queryKey: ['program_center_market_m_1_uninstall'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/market/m-1/uninstall"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const dict_dictFlag_path_data_ref = ref<any[]>([]);
const dict_dictFlag_path_data_q = useQuery({
  queryKey: ['dict_dictFlag_path_data'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/dict/dictFlag/path/data"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});


const program_center_center_version_ref = ref<any[]>([]);
const program_center_center_version_q = useQuery({
  queryKey: ['program_center_center_version'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/center/version"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_agent_flag_disable_ref = ref<any[]>([]);
const program_center_agent_flag_disable_q = useQuery({
  queryKey: ['program_center_agent_flag_disable'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/agent/flag/disable"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const config_list_dump_data_ref = ref<any[]>([]);
const config_list_dump_data_q = useQuery({
  queryKey: ['config_list_dump_data'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/config/list/dump/data"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const market_m_1_cover_pic_ref = ref<any[]>([]);
const market_m_1_cover_pic_q = useQuery({
  queryKey: ['market_m_1_cover_pic'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/market/m-1/cover/pic"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const list_schedulelog_application_app_1_ref = ref<any[]>([]);
const list_schedulelog_application_app_1_q = useQuery({
  queryKey: ['list_schedulelog_application_app_1'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/schedule/list/schedulelog/application/app-1"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_datastructure_modules_all_ref = ref<any[]>([]);
const program_center_datastructure_modules_all_q = useQuery({
  queryKey: ['program_center_datastructure_modules_all'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/datastructure/modules/all"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_prompterrorlog_count_loggername_ref = ref<any[]>([]);
const program_center_prompterrorlog_count_loggername_q = useQuery({
  queryKey: ['program_center_prompterrorlog_count_loggername'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/prompterrorlog/count/loggername"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_agent_flag_enable_ref = ref<any[]>([]);
const program_center_agent_flag_enable_q = useQuery({
  queryKey: ['program_center_agent_flag_enable'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/agent/flag/enable"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const apppack_pack_info_logo_ref = ref<any[]>([]);
const apppack_pack_info_logo_q = useQuery({
  queryKey: ['apppack_pack_info_logo'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/apppack/pack/info/logo"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_qiyeweixin_ref = ref<any[]>([]);
const program_center_qiyeweixin_q = useQuery({
  queryKey: ['program_center_qiyeweixin'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/qiyeweixin"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const market_flag_installed_version_ref = ref<any[]>([]);
const market_flag_installed_version_q = useQuery({
  queryKey: ['market_flag_installed_version'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/market/flag/installed/version"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_prompterrorlog_p_1_ref = ref<any[]>([]);
const program_center_prompterrorlog_p_1_q = useQuery({
  queryKey: ['program_center_prompterrorlog_p_1'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/prompterrorlog/p-1"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_validation_timeout_timeout_ref = ref<any[]>([]);
const program_center_validation_timeout_timeout_q = useQuery({
  queryKey: ['program_center_validation_timeout_timeout'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/validation/timeout/timeout"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const list_p_1_next_10_ref = ref<any[]>([]);
const list_p_1_next_10_q = useQuery({
  queryKey: ['list_p_1_next_10'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/prompterrorlog/list/p-1/next/10"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const list_id_next_count_ref = ref<any[]>([]);
const list_id_next_count_q = useQuery({
  queryKey: ['list_id_next_count'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/prompterrorlog/list/id/next/count"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});


const program_center_unexpectederrorlog_id_ref = ref<any[]>([]);
const program_center_unexpectederrorlog_id_q = useQuery({
  queryKey: ['program_center_unexpectederrorlog_id'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/unexpectederrorlog/id"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_mpweixin_check_ref = ref<any[]>([]);
const program_center_mpweixin_check_q = useQuery({
  queryKey: ['program_center_mpweixin_check'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/mpweixin/check"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const zhengwudingding_sync_organization_callback_ref = ref<any[]>([]);
const zhengwudingding_sync_organization_callback_q = useQuery({
  queryKey: ['zhengwudingding_sync_organization_callback'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/zhengwudingding/sync/organization/callback"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_jest_list_ref = ref<any[]>([]);
const program_center_jest_list_q = useQuery({
  queryKey: ['program_center_jest_list'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/jest/list"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_zhengwudingding_pull_sync_ref = ref<any[]>([]);
const program_center_zhengwudingding_pull_sync_q = useQuery({
  queryKey: ['program_center_zhengwudingding_pull_sync'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/zhengwudingding/pull/sync"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_structure_ref = ref<any[]>([]);
const program_center_structure_q = useQuery({
  queryKey: ['program_center_structure'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/structure"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_input_compare_ref = ref<any[]>([]);
const program_center_input_compare_q = useQuery({
  queryKey: ['program_center_input_compare'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/input/compare"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_dict_my_flag_data_ref = ref<any[]>([]);
const program_center_dict_my_flag_data_q = useQuery({
  queryKey: ['program_center_dict_my_flag_data'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/dict/my-flag/data"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_config_collect_ref = ref<any[]>([]);
const program_center_config_collect_q = useQuery({
  queryKey: ['program_center_config_collect'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/config/collect"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_validation_meta_ref = ref<any[]>([]);
const program_center_validation_meta_q = useQuery({
  queryKey: ['program_center_validation_meta'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/validation/meta"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const jest_clear_cache_es_ref = ref<any[]>([]);
const jest_clear_cache_es_q = useQuery({
  queryKey: ['jest_clear_cache_es'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/jest/clear/cache/es"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_collect_resetpassword_ref = ref<any[]>([]);
const program_center_collect_resetpassword_q = useQuery({
  queryKey: ['program_center_collect_resetpassword'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/collect/resetpassword"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_mpweixin_menu_add_ref = ref<any[]>([]);
const program_center_mpweixin_menu_add_q = useQuery({
  queryKey: ['program_center_mpweixin_menu_add'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/mpweixin/menu/add"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const image_menu_logo_focus_ref = ref<any[]>([]);
const image_menu_logo_focus_q = useQuery({
  queryKey: ['image_menu_logo_focus'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/appstyle/image/menu/logo/focus"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const open_run_time_config_ref = ref<any[]>([]);
const open_run_time_config_q = useQuery({
  queryKey: ['open_run_time_config'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/config/open/run/time/config"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const bar_select2_count_10_ref = ref<any[]>([]);
const bar_select2_count_10_q = useQuery({
  queryKey: ['bar_select2_count_10'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/bar/select2/count/10"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const create_mass_5_20_ref = ref<any[]>([]);
const create_mass_5_20_q = useQuery({
  queryKey: ['create_mass_5_20'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/foo/create/mass/5/20"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const bar_select2_count_count_ref = ref<any[]>([]);
const bar_select2_count_count_q = useQuery({
  queryKey: ['bar_select2_count_count'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/bar/select2/count/count"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const collect_code_mobile_mobile_ref = ref<any[]>([]);
const collect_code_mobile_mobile_q = useQuery({
  queryKey: ['collect_code_mobile_mobile'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/collect/code/mobile/mobile"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_agent_a_1_ref = ref<any[]>([]);
const program_center_agent_a_1_q = useQuery({
  queryKey: ['program_center_agent_a_1'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/agent/a-1"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});


const dictFlag_path_data_mockputtopost_ref = ref<any[]>([]);
const dictFlag_path_data_mockputtopost_q = useQuery({
  queryKey: ['dictFlag_path_data_mockputtopost'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/dict/dictFlag/path/data/mockputtopost"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_warnlog_ref = ref<any[]>([]);
const program_center_warnlog_q = useQuery({
  queryKey: ['program_center_warnlog'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/warnlog"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_test_test2_ref = ref<any[]>([]);
const program_center_test_test2_q = useQuery({
  queryKey: ['program_center_test_test2'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/test/test2"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const flag_install_or_update_ref = ref<any[]>([]);
const flag_install_or_update_q = useQuery({
  queryKey: ['flag_install_or_update'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/market/flag/install/or/update"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_agent_flag_ref = ref<any[]>([]);
const program_center_agent_flag_q = useQuery({
  queryKey: ['program_center_agent_flag'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/agent/flag"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_dict_ref = ref<any[]>([]);
const program_center_dict_q = useQuery({
  queryKey: ['program_center_dict'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/dict"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_input_create_ref = ref<any[]>([]);
const program_center_input_create_q = useQuery({
  queryKey: ['program_center_input_create'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/input/create"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_script_name_name_ref = ref<any[]>([]);
const program_center_script_name_name_q = useQuery({
  queryKey: ['program_center_script_name_name'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/script/name/name"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_market_flag_uninstall_ref = ref<any[]>([]);
const program_center_market_flag_uninstall_q = useQuery({
  queryKey: ['program_center_market_flag_uninstall'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/market/flag/uninstall"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_module_list_ref = ref<any[]>([]);
const program_center_module_list_q = useQuery({
  queryKey: ['program_center_module_list'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/module/list"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const appstyle_image_application_top_ref = ref<any[]>([]);
const appstyle_image_application_top_q = useQuery({
  queryKey: ['appstyle_image_application_top'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/appstyle/image/application/top"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_datastructure_ref = ref<any[]>([]);
const program_center_datastructure_q = useQuery({
  queryKey: ['program_center_datastructure'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/datastructure"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_qiyeweixin_pull_sync_ref = ref<any[]>([]);
const program_center_qiyeweixin_pull_sync_q = useQuery({
  queryKey: ['program_center_qiyeweixin_pull_sync'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/qiyeweixin/pull/sync"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_schedule_list_schedule_ref = ref<any[]>([]);
const program_center_schedule_list_schedule_q = useQuery({
  queryKey: ['program_center_schedule_list_schedule'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/schedule/list/schedule"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const module_output_m_1_file_ref = ref<any[]>([]);
const module_output_m_1_file_q = useQuery({
  queryKey: ['module_output_m_1_file'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/module/output/m-1/file"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_applications_list_ref = ref<any[]>([]);
const program_center_applications_list_q = useQuery({
  queryKey: ['program_center_applications_list'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/applications/list"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_appstyle_current_style_ref = ref<any[]>([]);
const program_center_appstyle_current_style_q = useQuery({
  queryKey: ['program_center_appstyle_current_style'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/appstyle/current/style"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_collect_disconnect_ref = ref<any[]>([]);
const program_center_collect_disconnect_q = useQuery({
  queryKey: ['program_center_collect_disconnect'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/collect/disconnect"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_core_list_ref = ref<any[]>([]);
const program_center_core_list_q = useQuery({
  queryKey: ['program_center_core_list'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/core/list"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_config_list_entity_ref = ref<any[]>([]);
const program_center_config_list_entity_q = useQuery({
  queryKey: ['program_center_config_list_entity'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/config/list/entity"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_application_some_id_ref = ref<any[]>([]);
const program_center_application_some_id_q = useQuery({
  queryKey: ['program_center_application_some_id'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/application/some-id"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_jest_version_ref = ref<any[]>([]);
const program_center_jest_version_q = useQuery({
  queryKey: ['program_center_jest_version'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/jest/version"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_welink_pull_sync_ref = ref<any[]>([]);
const program_center_welink_pull_sync_q = useQuery({
  queryKey: ['program_center_welink_pull_sync'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/welink/pull/sync"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const list_id_prev_count_ref = ref<any[]>([]);
const list_id_prev_count_q = useQuery({
  queryKey: ['list_id_prev_count'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/unexpectederrorlog/list/id/prev/count"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_deploy_list_ref = ref<any[]>([]);
const program_center_deploy_list_q = useQuery({
  queryKey: ['program_center_deploy_list'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/deploy/list"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_deploy_server_resource_ref = ref<any[]>([]);
const program_center_deploy_server_resource_q = useQuery({
  queryKey: ['program_center_deploy_server_resource'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/deploy/server/resource"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const output_f_1_select_file_ref = ref<any[]>([]);
const output_f_1_select_file_q = useQuery({
  queryKey: ['output_f_1_select_file'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/output/f-1/select/file"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const id_validate_answer_answer_ref = ref<any[]>([]);
const id_validate_answer_answer_q = useQuery({
  queryKey: ['id_validate_answer_answer'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/captcha/id/validate/answer/answer"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_module_output_ref = ref<any[]>([]);
const program_center_module_output_q = useQuery({
  queryKey: ['program_center_module_output'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/module/output"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_module_output_structure_ref = ref<any[]>([]);
const program_center_module_output_structure_q = useQuery({
  queryKey: ['program_center_module_output_structure'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/module/output/structure"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});


const program_center_config_change_password_ref = ref<any[]>([]);
const program_center_config_change_password_q = useQuery({
  queryKey: ['program_center_config_change_password'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/config/change/password"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_unknown_ref = ref<any[]>([]);
const program_center_unknown_q = useQuery({
  queryKey: ['program_center_unknown'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/unknown"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_collect_connect_ref = ref<any[]>([]);
const program_center_collect_connect_q = useQuery({
  queryKey: ['program_center_collect_connect'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/collect/connect"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_collect_validate_password_ref = ref<any[]>([]);
const program_center_collect_validate_password_q = useQuery({
  queryKey: ['program_center_collect_validate_password'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/collect/validate/password"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_dict_d_id_ref = ref<any[]>([]);
const program_center_dict_d_id_q = useQuery({
  queryKey: ['program_center_dict_d_id'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/dict/d-id"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_input_prepare_create_ref = ref<any[]>([]);
const program_center_input_prepare_create_q = useQuery({
  queryKey: ['program_center_input_prepare_create'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/input/prepare/create"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_agent_a_flag_ref = ref<any[]>([]);
const program_center_agent_a_flag_q = useQuery({
  queryKey: ['program_center_agent_a_flag'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/agent/a-flag"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_agent_a_1_disable_ref = ref<any[]>([]);
const program_center_agent_a_1_disable_q = useQuery({
  queryKey: ['program_center_agent_a_1_disable'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/agent/a-1/disable"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_center_regist_applications_ref = ref<any[]>([]);
const program_center_center_regist_applications_q = useQuery({
  queryKey: ['program_center_center_regist_applications'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/center/regist/applications"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_dict_dictFlag_data_ref = ref<any[]>([]);
const program_center_dict_dictFlag_data_q = useQuery({
  queryKey: ['program_center_dict_dictFlag_data'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/dict/dictFlag/data"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_command_list_node_ref = ref<any[]>([]);
const program_center_command_list_node_q = useQuery({
  queryKey: ['program_center_command_list_node'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/command/list/node"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const c_1_validate_answer_1234_ref = ref<any[]>([]);
const c_1_validate_answer_1234_q = useQuery({
  queryKey: ['c_1_validate_answer_1234'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/captcha/c-1/validate/answer/1234"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const list_with_category_cms_ref = ref<any[]>([]);
const list_with_category_cms_q = useQuery({
  queryKey: ['list_with_category_cms'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/invoke/list/with/category/cms"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_invoke_i_1_execute_ref = ref<any[]>([]);
const program_center_invoke_i_1_execute_q = useQuery({
  queryKey: ['program_center_invoke_i_1_execute'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/invoke/i-1/execute"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_script_sc_flag_ref = ref<any[]>([]);
const program_center_script_sc_flag_q = useQuery({
  queryKey: ['program_center_script_sc_flag'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/script/sc-flag"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_script_flag_ref = ref<any[]>([]);
const program_center_script_flag_q = useQuery({
  queryKey: ['program_center_script_flag'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/script/flag"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_collect_login_ref = ref<any[]>([]);
const program_center_collect_login_q = useQuery({
  queryKey: ['program_center_collect_login'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/collect/login"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_agent_a_1_execute_ref = ref<any[]>([]);
const program_center_agent_a_1_execute_q = useQuery({
  queryKey: ['program_center_agent_a_1_execute'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/agent/a-1/execute"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_market_list_category_ref = ref<any[]>([]);
const program_center_market_list_category_q = useQuery({
  queryKey: ['program_center_market_list_category'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/market/list/category"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_invoke_list_category_ref = ref<any[]>([]);
const program_center_invoke_list_category_q = useQuery({
  queryKey: ['program_center_invoke_list_category'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/invoke/list/category"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});


const program_center_config_license_ref = ref<any[]>([]);
const program_center_config_license_q = useQuery({
  queryKey: ['program_center_config_license'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/config/license"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_prompterrorlog_ref = ref<any[]>([]);
const program_center_prompterrorlog_q = useQuery({
  queryKey: ['program_center_prompterrorlog'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/prompterrorlog"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_collect_add_ref = ref<any[]>([]);
const program_center_collect_add_q = useQuery({
  queryKey: ['program_center_collect_add'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/collect/add"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const dingding_sync_organization_callback_ref = ref<any[]>([]);
const dingding_sync_organization_callback_q = useQuery({
  queryKey: ['dingding_sync_organization_callback'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/dingding/sync/organization/callback"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_appstyle_current_update_ref = ref<any[]>([]);
const program_center_appstyle_current_update_q = useQuery({
  queryKey: ['program_center_appstyle_current_update'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/appstyle/current/update"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const image_launch_logo_erase_ref = ref<any[]>([]);
const image_launch_logo_erase_q = useQuery({
  queryKey: ['image_launch_logo_erase'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/appstyle/image/launch/logo/erase"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const list_id_next_count_1_ref = ref<any[]>([]);
const list_id_next_count_1_q = useQuery({
  queryKey: ['list_id_next_count_1'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/unexpectederrorlog/list/id/next/count"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const webserver_assemble_source_source_ref = ref<any[]>([]);
const webserver_assemble_source_source_q = useQuery({
  queryKey: ['webserver_assemble_source_source'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/distribute/webserver/assemble/source/source"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const script_name_name_imported_ref = ref<any[]>([]);
const script_name_name_imported_q = useQuery({
  queryKey: ['script_name_name_imported'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/script/name/name/imported"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_apppack_pack_info_ref = ref<any[]>([]);
const program_center_apppack_pack_info_q = useQuery({
  queryKey: ['program_center_apppack_pack_info'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/apppack/pack/info"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_input_cover_ref = ref<any[]>([]);
const program_center_input_cover_q = useQuery({
  queryKey: ['program_center_input_cover'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/input/cover"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const dingding_get_callback_aes_ref = ref<any[]>([]);
const dingding_get_callback_aes_q = useQuery({
  queryKey: ['dingding_get_callback_aes'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/dingding/get/callback/aes"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_config_token_ref = ref<any[]>([]);
const program_center_config_token_q = useQuery({
  queryKey: ['program_center_config_token'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/config/token"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_market_m_1_download_ref = ref<any[]>([]);
const program_center_market_m_1_download_q = useQuery({
  queryKey: ['program_center_market_m_1_download'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/market/m-1/download"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_script_ref = ref<any[]>([]);
const program_center_script_q = useQuery({
  queryKey: ['program_center_script'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/script"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});


const dictFlag_path_data_mockdeletetoget_ref = ref<any[]>([]);
const dictFlag_path_data_mockdeletetoget_q = useQuery({
  queryKey: ['dictFlag_path_data_mockdeletetoget'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/dict/dictFlag/path/data/mockdeletetoget"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const mpweixin_menu_delete_wm_1_ref = ref<any[]>([]);
const mpweixin_menu_delete_wm_1_q = useQuery({
  queryKey: ['mpweixin_menu_delete_wm_1'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/mpweixin/menu/delete/wm-1"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const cloud_unit_is_vip_ref = ref<any[]>([]);
const cloud_unit_is_vip_q = useQuery({
  queryKey: ['cloud_unit_is_vip'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/market/cloud/unit/is/vip"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_config_open_ref = ref<any[]>([]);
const program_center_config_open_q = useQuery({
  queryKey: ['program_center_config_open'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/config/open"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const dingding_request_pull_sync_ref = ref<any[]>([]);
const dingding_request_pull_sync_q = useQuery({
  queryKey: ['dingding_request_pull_sync'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/dingding/request/pull/sync"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_code_list_ref = ref<any[]>([]);
const program_center_code_list_q = useQuery({
  queryKey: ['program_center_code_list'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/code/list"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_schedule_schedule_fire_ref = ref<any[]>([]);
const program_center_schedule_schedule_fire_q = useQuery({
  queryKey: ['program_center_schedule_schedule_fire'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/schedule/schedule/fire"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_storagemappings_ref = ref<any[]>([]);
const program_center_storagemappings_q = useQuery({
  queryKey: ['program_center_storagemappings'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/storagemappings"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_zhengwudingding_regist_callback_ref = ref<any[]>([]);
const program_center_zhengwudingding_regist_callback_q = useQuery({
  queryKey: ['program_center_zhengwudingding_regist_callback'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/zhengwudingding/regist/callback"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_config_centerserver_ref = ref<any[]>([]);
const program_center_config_centerserver_q = useQuery({
  queryKey: ['program_center_config_centerserver'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/config/centerserver"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_validation_scripting_benchmark_ref = ref<any[]>([]);
const program_center_validation_scripting_benchmark_q = useQuery({
  queryKey: ['program_center_validation_scripting_benchmark'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/validation/scripting/benchmark"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_jest_clear_cache_ref = ref<any[]>([]);
const program_center_jest_clear_cache_q = useQuery({
  queryKey: ['program_center_jest_clear_cache'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/jest/clear/cache"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const collect_mobile_check_connect_ref = ref<any[]>([]);
const collect_mobile_check_connect_q = useQuery({
  queryKey: ['collect_mobile_check_connect'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/collect/mobile/check/connect"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_application_create_ref = ref<any[]>([]);
const program_center_application_create_q = useQuery({
  queryKey: ['program_center_application_create'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/application/create"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_collect_updateUnit_ref = ref<any[]>([]);
const program_center_collect_updateUnit_q = useQuery({
  queryKey: ['program_center_collect_updateUnit'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/collect/updateUnit"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});


const script_ref = ref<any[]>([]);
const script_q = useQuery({
  queryKey: ['script'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/script"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const s_1_app_app_1_imported_ref = ref<any[]>([]);
const s_1_app_app_1_imported_q = useQuery({
  queryKey: ['s_1_app_app_1_imported'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/script/s-1/app/app-1/imported"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const script_s_1_ref = ref<any[]>([]);
const script_s_1_q = useQuery({
  queryKey: ['script_s_1'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/script/s-1"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const script_list_manager_ref = ref<any[]>([]);
const script_list_manager_q = useQuery({
  queryKey: ['script_list_manager'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/script/list/manager"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const scriptversion_sv_1_ref = ref<any[]>([]);
const scriptversion_sv_1_q = useQuery({
  queryKey: ['scriptversion_sv_1'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/scriptversion/sv-1"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const list_i_1_next_10_ref = ref<any[]>([]);
const list_i_1_next_10_q = useQuery({
  queryKey: ['list_i_1_next_10'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/script/list/i-1/next/10"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const script_s_1_appInfo_app_1_ref = ref<any[]>([]);
const script_s_1_appInfo_app_1_q = useQuery({
  queryKey: ['script_s_1_appInfo_app_1'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/script/s-1/appInfo/app-1"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const script_s_1_app_app_1_ref = ref<any[]>([]);
const script_s_1_app_app_1_q = useQuery({
  queryKey: ['script_s_1_app_app_1'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/script/s-1/app/app-1"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const scriptversion_list_script_s_1_ref = ref<any[]>([]);
const scriptversion_list_script_s_1_q = useQuery({
  queryKey: ['scriptversion_list_script_s_1'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/scriptversion/list/script/s-1"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});


const program_center_schedule_report_ref = ref<any[]>([]);
const program_center_schedule_report_q = useQuery({
  queryKey: ['program_center_schedule_report'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/schedule/report"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const image_menu_logo_blur_ref = ref<any[]>([]);
const image_menu_logo_blur_q = useQuery({
  queryKey: ['image_menu_logo_blur'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/appstyle/image/menu/logo/blur"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_collect_remove_ref = ref<any[]>([]);
const program_center_collect_remove_q = useQuery({
  queryKey: ['program_center_collect_remove'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/collect/remove"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_warnlog_w_1_ref = ref<any[]>([]);
const program_center_warnlog_w_1_q = useQuery({
  queryKey: ['program_center_warnlog_w_1'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/warnlog/w-1"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const image_setup_about_logo_ref = ref<any[]>([]);
const image_setup_about_logo_q = useQuery({
  queryKey: ['image_setup_about_logo'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/appstyle/image/setup/about/logo"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_script_id_ref = ref<any[]>([]);
const program_center_script_id_q = useQuery({
  queryKey: ['program_center_script_id'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/script/id"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const qiyeweixin_request_pull_sync_ref = ref<any[]>([]);
const qiyeweixin_request_pull_sync_q = useQuery({
  queryKey: ['qiyeweixin_request_pull_sync'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/qiyeweixin/request/pull/sync"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_authentication_ref = ref<any[]>([]);
const program_center_authentication_q = useQuery({
  queryKey: ['program_center_authentication'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/authentication"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const module_remove_structure_m_1_ref = ref<any[]>([]);
const module_remove_structure_m_1_q = useQuery({
  queryKey: ['module_remove_structure_m_1'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/module/remove/structure/m-1"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_config_get_ref = ref<any[]>([]);
const program_center_config_get_q = useQuery({
  queryKey: ['program_center_config_get'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/config/get"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const market_flag_cover_pic_ref = ref<any[]>([]);
const market_flag_cover_pic_q = useQuery({
  queryKey: ['market_flag_cover_pic'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/market/flag/cover/pic"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_invoke_flag_ref = ref<any[]>([]);
const program_center_invoke_flag_q = useQuery({
  queryKey: ['program_center_invoke_flag'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/invoke/flag"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const output_flag_select_file_ref = ref<any[]>([]);
const output_flag_select_file_q = useQuery({
  queryKey: ['output_flag_select_file'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/output/flag/select/file"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const mpweixin_message_template_send_ref = ref<any[]>([]);
const mpweixin_message_template_send_q = useQuery({
  queryKey: ['mpweixin_message_template_send'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/mpweixin/message/template/send"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const create_mass_from_count_ref = ref<any[]>([]);
const create_mass_from_count_q = useQuery({
  queryKey: ['create_mass_from_count'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/bar/create/mass/from/count"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_invoke_token_ref = ref<any[]>([]);
const program_center_invoke_token_q = useQuery({
  queryKey: ['program_center_invoke_token'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/invoke/token"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const create_mass_from_count_1_ref = ref<any[]>([]);
const create_mass_from_count_1_q = useQuery({
  queryKey: ['create_mass_from_count_1'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/foo/create/mass/from/count"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const distribute_assemble_source_o2_ref = ref<any[]>([]);
const distribute_assemble_source_o2_q = useQuery({
  queryKey: ['distribute_assemble_source_o2'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/distribute/assemble/source/o2"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_config_portal_ref = ref<any[]>([]);
const program_center_config_portal_q = useQuery({
  queryKey: ['program_center_config_portal'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/config/portal"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const list_schedulelog_application_application_ref = ref<any[]>([]);
const list_schedulelog_application_application_q = useQuery({
  queryKey: ['list_schedulelog_application_application'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/schedule/list/schedulelog/application/application"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_market_flag_ref = ref<any[]>([]);
const program_center_market_flag_q = useQuery({
  queryKey: ['program_center_market_flag'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/market/flag"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const module_output_list_structure_ref = ref<any[]>([]);
const module_output_list_structure_q = useQuery({
  queryKey: ['module_output_list_structure'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/module/output/list/structure"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const pack_info_file_publish_ref = ref<any[]>([]);
const pack_info_file_publish_q = useQuery({
  queryKey: ['pack_info_file_publish'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/apppack/pack/info/file/publish"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_market_m_1_ref = ref<any[]>([]);
const program_center_market_m_1_q = useQuery({
  queryKey: ['program_center_market_m_1'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/market/m-1"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const jest_clear_cache_source_ref = ref<any[]>([]);
const jest_clear_cache_source_q = useQuery({
  queryKey: ['jest_clear_cache_source'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/jest/clear/cache/source"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_output_f_1_select_ref = ref<any[]>([]);
const program_center_output_f_1_select_q = useQuery({
  queryKey: ['program_center_output_f_1_select'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/output/f-1/select"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_config_list_ref = ref<any[]>([]);
const program_center_config_list_q = useQuery({
  queryKey: ['program_center_config_list'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/config/list"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const dict_my_flag_my_path_data_ref = ref<any[]>([]);
const dict_my_flag_my_path_data_q = useQuery({
  queryKey: ['dict_my_flag_my_path_data'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/dict/my-flag/my-path/data"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const my_flag_my_path_data_mockdeletetoget_ref = ref<any[]>([]);
const my_flag_my_path_data_mockdeletetoget_q = useQuery({
  queryKey: ['my_flag_my_path_data_mockdeletetoget'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/dict/my-flag/my-path/data/mockdeletetoget"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_market_installed_version_ref = ref<any[]>([]);
const program_center_market_installed_version_q = useQuery({
  queryKey: ['program_center_market_installed_version'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/market/installed/version"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});


const program_center_collect_urlMapping_ref = ref<any[]>([]);
const program_center_collect_urlMapping_q = useQuery({
  queryKey: ['program_center_collect_urlMapping'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/collect/urlMapping"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_collect_ref = ref<any[]>([]);
const program_center_collect_q = useQuery({
  queryKey: ['program_center_collect'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/collect"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_datastructure_tables_all_ref = ref<any[]>([]);
const program_center_datastructure_tables_all_q = useQuery({
  queryKey: ['program_center_datastructure_tables_all'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/datastructure/tables/all"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const appstyle_image_process_default_ref = ref<any[]>([]);
const appstyle_image_process_default_q = useQuery({
  queryKey: ['appstyle_image_process_default'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/appstyle/image/process/default"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_unexpectederrorlog_u_1_ref = ref<any[]>([]);
const program_center_unexpectederrorlog_u_1_q = useQuery({
  queryKey: ['program_center_unexpectederrorlog_u_1'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/unexpectederrorlog/u-1"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_prompterrorlog_count_exceptionclass_ref = ref<any[]>([]);
const program_center_prompterrorlog_count_exceptionclass_q = useQuery({
  queryKey: ['program_center_prompterrorlog_count_exceptionclass'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/prompterrorlog/count/exceptionclass"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_config_list_application_ref = ref<any[]>([]);
const program_center_config_list_application_q = useQuery({
  queryKey: ['program_center_config_list_application'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/config/list/application"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_market_install_offline_ref = ref<any[]>([]);
const program_center_market_install_offline_q = useQuery({
  queryKey: ['program_center_market_install_offline'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/market/install/offline"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_structure_list_ref = ref<any[]>([]);
const program_center_structure_list_q = useQuery({
  queryKey: ['program_center_structure_list'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/structure/list"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_agent_flag_execute_ref = ref<any[]>([]);
const program_center_agent_flag_execute_q = useQuery({
  queryKey: ['program_center_agent_flag_execute'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/agent/flag/execute"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_cachedispatch_ref = ref<any[]>([]);
const program_center_cachedispatch_q = useQuery({
  queryKey: ['program_center_cachedispatch'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/cachedispatch"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const pack_info_file_last_ref = ref<any[]>([]);
const pack_info_file_last_q = useQuery({
  queryKey: ['pack_info_file_last'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/apppack/pack/info/file/last"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const pack_info_android_repack_ref = ref<any[]>([]);
const pack_info_android_repack_q = useQuery({
  queryKey: ['pack_info_android_repack'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/apppack/pack/info/android/repack"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_dict_id_ref = ref<any[]>([]);
const program_center_dict_id_q = useQuery({
  queryKey: ['program_center_dict_id'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/dict/id"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const my_flag_my_path_data_mockputtopost_ref = ref<any[]>([]);
const my_flag_my_path_data_mockputtopost_q = useQuery({
  queryKey: ['my_flag_my_path_data_mockputtopost'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/dict/my-flag/my-path/data/mockputtopost"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_input_prepare_cover_ref = ref<any[]>([]);
const program_center_input_prepare_cover_q = useQuery({
  queryKey: ['program_center_input_prepare_cover'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/input/prepare/cover"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_list_ref = ref<any[]>([]);
const program_list_q = useQuery({
  queryKey: ['program_list'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program/list"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_collect_validate_codeanswer_ref = ref<any[]>([]);
const program_center_collect_validate_codeanswer_q = useQuery({
  queryKey: ['program_center_collect_validate_codeanswer'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/collect/validate/codeanswer"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_ref = ref<any[]>([]);
const program_center_q = useQuery({
  queryKey: ['program_center'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const qiyeweixin_send_getprivateinfo_message_ref = ref<any[]>([]);
const qiyeweixin_send_getprivateinfo_message_q = useQuery({
  queryKey: ['qiyeweixin_send_getprivateinfo_message'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/qiyeweixin/send/getprivateinfo/message"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_center_applications_list_ref = ref<any[]>([]);
const program_center_center_applications_list_q = useQuery({
  queryKey: ['program_center_center_applications_list'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/center/applications/list"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_collect_validate_direct_ref = ref<any[]>([]);
const program_center_collect_validate_direct_q = useQuery({
  queryKey: ['program_center_collect_validate_direct'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/collect/validate/direct"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const webserver_assemble_source_o2_ref = ref<any[]>([]);
const webserver_assemble_source_o2_q = useQuery({
  queryKey: ['webserver_assemble_source_o2'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/distribute/webserver/assemble/source/o2"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_module_write_flag_ref = ref<any[]>([]);
const program_center_module_write_flag_q = useQuery({
  queryKey: ['program_center_module_write_flag'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/module/write/flag"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_invoke_i_1_file_ref = ref<any[]>([]);
const program_center_invoke_i_1_file_q = useQuery({
  queryKey: ['program_center_invoke_i_1_file'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/invoke/i-1/file"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_appstyle_current_style_ref = ref<any[]>([]);
const program_appstyle_current_style_q = useQuery({
  queryKey: ['program_appstyle_current_style'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program/appstyle/current/style"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const image_process_default_erase_ref = ref<any[]>([]);
const image_process_default_erase_q = useQuery({
  queryKey: ['image_process_default_erase'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/appstyle/image/process/default/erase"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_datastructure_modules_all_ref = ref<any[]>([]);
const program_datastructure_modules_all_q = useQuery({
  queryKey: ['program_datastructure_modules_all'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program/datastructure/modules/all"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const pack_info_android_start_ref = ref<any[]>([]);
const pack_info_android_start_q = useQuery({
  queryKey: ['pack_info_android_start'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/apppack/pack/info/android/start"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const invoke_i_1_execute_get_ref = ref<any[]>([]);
const invoke_i_1_execute_get_q = useQuery({
  queryKey: ['invoke_i_1_execute_get'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/invoke/i-1/execute/get"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_deploy_d_1_ref = ref<any[]>([]);
const program_center_deploy_d_1_q = useQuery({
  queryKey: ['program_center_deploy_d_1'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/deploy/d-1"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const mpweixin_menu_list_weixin_ref = ref<any[]>([]);
const mpweixin_menu_list_weixin_q = useQuery({
  queryKey: ['mpweixin_menu_list_weixin'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/mpweixin/menu/list/weixin"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const image_application_top_erase_ref = ref<any[]>([]);
const image_application_top_erase_q = useQuery({
  queryKey: ['image_application_top_erase'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/appstyle/image/application/top/erase"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const code_create_mobile_13800000000_ref = ref<any[]>([]);
const code_create_mobile_13800000000_q = useQuery({
  queryKey: ['code_create_mobile_13800000000'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/code/create/mobile/13800000000"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_adminlogin_ref = ref<any[]>([]);
const program_center_adminlogin_q = useQuery({
  queryKey: ['program_center_adminlogin'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/adminlogin"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_apppack_server_connect_ref = ref<any[]>([]);
const program_center_apppack_server_connect_q = useQuery({
  queryKey: ['program_center_apppack_server_connect'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/apppack/server/connect"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_tokenthreshold_update_ref = ref<any[]>([]);
const program_center_tokenthreshold_update_q = useQuery({
  queryKey: ['program_center_tokenthreshold_update'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/tokenthreshold/update"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const get_disable_export_enable_ref = ref<any[]>([]);
const get_disable_export_enable_q = useQuery({
  queryKey: ['get_disable_export_enable'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/config-open/get/disable/export/enable"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_agent_a_1_enable_ref = ref<any[]>([]);
const program_center_agent_a_1_enable_q = useQuery({
  queryKey: ['program_center_agent_a_1_enable'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/agent/a-1/enable"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_application_ref = ref<any[]>([]);
const program_center_application_q = useQuery({
  queryKey: ['program_center_application'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/application"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});


const collect_name_name_exist_ref = ref<any[]>([]);
const collect_name_name_exist_q = useQuery({
  queryKey: ['collect_name_name_exist'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/collect/name/name/exist"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const market_list_top_three_ref = ref<any[]>([]);
const market_list_top_three_q = useQuery({
  queryKey: ['market_list_top_three'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/market/list/top/three"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const list_w_1_prev_5_ref = ref<any[]>([]);
const list_w_1_prev_5_q = useQuery({
  queryKey: ['list_w_1_prev_5'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/warnlog/list/w-1/prev/5"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const qiyeweixin_get_callback_aes_ref = ref<any[]>([]);
const qiyeweixin_get_callback_aes_q = useQuery({
  queryKey: ['qiyeweixin_get_callback_aes'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/qiyeweixin/get/callback/aes"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_config_save_ref = ref<any[]>([]);
const program_center_config_save_q = useQuery({
  queryKey: ['program_center_config_save'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/config/save"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_center_applications_ref = ref<any[]>([]);
const program_center_center_applications_q = useQuery({
  queryKey: ['program_center_center_applications'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/center/applications"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_test_test1_ref = ref<any[]>([]);
const program_center_test_test1_q = useQuery({
  queryKey: ['program_center_test_test1'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/test/test1"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_script_name_demo_ref = ref<any[]>([]);
const program_center_script_name_demo_q = useQuery({
  queryKey: ['program_center_script_name_demo'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/script/name/demo"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const script_name_demo_imported_ref = ref<any[]>([]);
const script_name_demo_imported_q = useQuery({
  queryKey: ['script_name_demo_imported'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/script/name/demo/imported"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_script_sc_id_ref = ref<any[]>([]);
const program_center_script_sc_id_q = useQuery({
  queryKey: ['program_center_script_sc_id'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/script/sc-id"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const list_u_1_prev_5_ref = ref<any[]>([]);
const list_u_1_prev_5_q = useQuery({
  queryKey: ['list_u_1_prev_5'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/unexpectederrorlog/list/u-1/prev/5"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const appstyle_image_login_avatar_ref = ref<any[]>([]);
const appstyle_image_login_avatar_q = useQuery({
  queryKey: ['appstyle_image_login_avatar'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/appstyle/image/login/avatar"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_dingding_pull_sync_ref = ref<any[]>([]);
const program_center_dingding_pull_sync_q = useQuery({
  queryKey: ['program_center_dingding_pull_sync'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/dingding/pull/sync"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_agent_flag_file_ref = ref<any[]>([]);
const program_center_agent_flag_file_q = useQuery({
  queryKey: ['program_center_agent_flag_file'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/agent/flag/file"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_command_execute_ref = ref<any[]>([]);
const program_center_command_execute_q = useQuery({
  queryKey: ['program_center_command_execute'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/command/execute"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const m_1_install_or_update_ref = ref<any[]>([]);
const m_1_install_or_update_q = useQuery({
  queryKey: ['m_1_install_or_update'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/market/m-1/install/or/update"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const pack_info_file_last_1_ref = ref<any[]>([]);
const pack_info_file_last_1_q = useQuery({
  queryKey: ['pack_info_file_last_1'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/apppackanony/pack/info/file/last"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_agent_ref = ref<any[]>([]);
const program_center_agent_q = useQuery({
  queryKey: ['program_center_agent'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/agent"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const welink_request_pull_sync_ref = ref<any[]>([]);
const welink_request_pull_sync_q = useQuery({
  queryKey: ['welink_request_pull_sync'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/welink/request/pull/sync"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_config_ternary_management_ref = ref<any[]>([]);
const program_center_config_ternary_management_q = useQuery({
  queryKey: ['program_center_config_ternary_management'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/config/ternary/management"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const mpweixin_menu_update_id_ref = ref<any[]>([]);
const mpweixin_menu_update_id_q = useQuery({
  queryKey: ['mpweixin_menu_update_id'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/mpweixin/menu/update/id"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_session_list_all_ref = ref<any[]>([]);
const program_center_session_list_all_q = useQuery({
  queryKey: ['program_center_session_list_all'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/session/list/all"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_module_write_m_1_ref = ref<any[]>([]);
const program_center_module_write_m_1_q = useQuery({
  queryKey: ['program_center_module_write_m_1'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/module/write/m-1"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_module_compare_upload_ref = ref<any[]>([]);
const program_center_module_compare_upload_q = useQuery({
  queryKey: ['program_center_module_compare_upload'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/module/compare/upload"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const menu_create_to_weixin_ref = ref<any[]>([]);
const menu_create_to_weixin_q = useQuery({
  queryKey: ['menu_create_to_weixin'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/mpweixin/menu/create/to/weixin"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const distribute_assemble_source_source_ref = ref<any[]>([]);
const distribute_assemble_source_source_q = useQuery({
  queryKey: ['distribute_assemble_source_source'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/distribute/assemble/source/source"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const invoke_flag_execute_get_ref = ref<any[]>([]);
const invoke_flag_execute_get_q = useQuery({
  queryKey: ['invoke_flag_execute_get'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/invoke/flag/execute/get"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_mpweixin_menu_subscribe_ref = ref<any[]>([]);
const program_center_mpweixin_menu_subscribe_q = useQuery({
  queryKey: ['program_center_mpweixin_menu_subscribe'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/mpweixin/menu/subscribe"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const program_center_captcha_list_ref = ref<any[]>([]);
const program_center_captcha_list_q = useQuery({
  queryKey: ['program_center_captcha_list'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program_center/captcha/list"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
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

const jaxrs_program_ref = ref<any[]>([]);
const jaxrs_program_q = useQuery({
  queryKey: ['jaxrs_program'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/program"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
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
