<template>
  <div class="mod-view">
    <div class="view-header glass-card">
      <h1>程序中心</h1>
      <p class="subtitle">/api/program_center/* — 319条路由</p>
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
              <button class="btn-sm" @click="editAgent(a)">编辑</button>
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
              <button class="btn-sm" style="margin-top:4px" @click="openScriptEditor(s)">编辑代码</button>
              <button class="btn-sm" style="margin-top:4px" @click="loadVersions(s)">版本</button>
              <button class="btn-sm" style="margin-top:4px" @click="runScript(s)">执行</button>
              <button class="btn-sm" style="color:var(--color-error);margin-top:4px" @click="deleteScript(s)">删除</button>
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
              <button class="btn-sm" style="margin-top:4px" :disabled="!d.flag" @click="openDictData(d)">数据</button>
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
    <!-- Agent 属性编辑（POST agent/save/{id}） -->
    <div v-if="showEditAgent" class="modal-overlay" @click.self="showEditAgent=false">
      <div class="modal glass-card">
        <h3>编辑Agent</h3>
        <div class="form-group"><label>名称</label><input v-model="agentEdit.name" class="form-input"/></div>
        <div class="form-group"><label>Flag</label><input v-model="agentEdit.flag" class="form-input"/></div>
        <div class="form-group"><label>描述</label><input v-model="agentEdit.description" class="form-input"/></div>
        <div class="modal-actions">
          <button class="btn-cancel" @click="showEditAgent=false">取消</button>
          <button class="btn-primary" :disabled="agentSaving" @click="saveAgentEdit">{{ agentSaving?'保存中…':'保存' }}</button>
        </div>
      </div>
    </div>
    <!-- 新建字典（POST /api/program_center/dict，dictFlag 为后端字段名） -->
    <div v-if="showCreateDict" class="modal-overlay" @click.self="showCreateDict=false">
      <div class="modal glass-card">
        <h3>新建字典</h3>
        <div class="form-group"><label>名称</label><input v-model="dictForm.name" class="form-input" placeholder="字典名称"/></div>
        <div class="form-group"><label>Flag</label><input v-model="dictForm.dictFlag" class="form-input" placeholder="唯一标识（dictFlag）"/></div>
        <div class="modal-actions">
          <button class="btn-cancel" @click="showCreateDict=false">取消</button>
          <button class="btn-primary" :disabled="!dictForm.name?.trim()||!dictForm.dictFlag?.trim()" @click="onCreateDict">创建</button>
        </div>
      </div>
    </div>
    <!-- 字典数据编辑器（GET dict/{flag}/data + POST dict/{flag}/data/data） -->
    <div v-if="showDictData" class="modal-overlay" @click.self="closeDictData">
      <div class="modal glass-card">
        <h3>字典数据 · {{ dictDataFlag }}</h3>
        <div v-if="dictDataLoading" class="hint">加载中…</div>
        <div v-else>
          <div class="form-group"><label>数据（JSON）</label><textarea v-model="dictDataText" rows="10" class="form-input mono" style="width:100%;resize:vertical"/></div>
          <div v-if="dictDataError" class="error" style="color:var(--color-error);font-size:12px;margin-top:6px">{{ dictDataError }}</div>
        </div>
        <div class="modal-actions">
          <button class="btn-cancel" @click="closeDictData">关闭</button>
          <button class="btn-primary" :disabled="dictDataLoading||!dictDataText.trim()" @click="saveDictData">保存数据</button>
        </div>
      </div>
    </div>
    <!-- 脚本代码编辑器（GET script/{flag} + POST script/{flag}） -->
    <div v-if="showScriptEdit" class="modal-overlay" @click.self="closeScriptEdit">
      <div class="modal glass-card" style="width:640px">
        <h3>脚本代码 · {{ scriptEdit.name || scriptEdit.flag }}</h3>
        <div v-if="scriptEditLoading" class="hint">加载中…</div>
        <div v-else class="form-group">
          <textarea v-model="scriptEdit.content" rows="16" class="form-input mono" style="width:100%;resize:vertical;font-size:12px"></textarea>
        </div>
        <div class="modal-actions">
          <button class="btn-cancel" @click="closeScriptEdit">关闭</button>
          <button class="btn-primary" :disabled="scriptEditLoading||!scriptEdit.flag" @click="saveScript">保存脚本</button>
        </div>
      </div>
    </div>
    <!-- 脚本版本历史（GET /api/scriptversion/list/script/{scriptId}） -->
    <div v-if="showVersions" class="modal-overlay" @click.self="closeVersions">
      <div class="modal glass-card">
        <h3>版本历史 · {{ versionsScript.name || versionsScript.flag }}</h3>
        <div v-if="versions.length===0" class="hint">暂无版本记录</div>
        <table v-else class="ver-table">
          <thead><tr><th>ID</th><th>创建时间</th></tr></thead>
          <tbody>
            <tr v-for="v in versions" :key="v.id">
              <td class="mono">{{ v.id }}</td>
              <td>{{ v.createTime || v.create_time || '—' }}</td>
            </tr>
          </tbody>
        </table>
        <div class="modal-actions"><button class="btn-cancel" @click="closeVersions">关闭</button></div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { api } from '@oa4rust/sdk'
import { useMutation } from '@tanstack/vue-query'
import { computed, ref, watch } from 'vue'
import { toast } from '../utils/toast'

type Tab = 'agent' | 'application' | 'script' | 'dict' | 'market'
type Agent = { id?: string; name?: string; label?: string; agentName?: string; flag?: string; enabled?: boolean }
type App = { id?: string; name?: string; appName?: string; desc?: string; description?: string; flag?: string }
type Script = { id?: string; name?: string; scriptName?: string; flag?: string }
type Dict = { id?: string; name?: string; dictName?: string; flag?: string; keyName?: string }
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

// Agent 属性编辑（POST agent/save/{id}）
const showEditAgent = ref(false)
const agentEdit = ref({ id: '', name: '', flag: '', description: '' })
const agentSaving = ref(false)

// 字典创建（POST /api/program_center/dict；后端字段名为 dictFlag）
const dictForm = ref({ name: '', dictFlag: '' })

// 字典数据编辑器（GET dict/{flag}/data + POST dict/{flag}/data/data）
const showDictData = ref(false)
const dictDataFlag = ref('')
const dictDataText = ref('')
const dictDataLoading = ref(false)
const dictDataSaving = ref(false)
const dictDataError = ref('')

// 脚本代码编辑器（GET script/{flag} + POST script/{flag}）
const showScriptEdit = ref(false)
const scriptEdit = ref({ flag: '', name: '', content: '' })
const scriptEditLoading = ref(false)
const scriptEditSaving = ref(false)

// 脚本版本历史（GET /api/scriptversion/list/script/{scriptId}）
const showVersions = ref(false)
const versionsScript = ref<Script>({})
const versions = ref<Array<{ id: string; createTime?: string; create_time?: string }>>([])
const filteredAgents = computed(() =>
  agentSearch.value
    ? agents.value.filter((a) => (a.name || a.flag || '').toLowerCase().includes(agentSearch.value.toLowerCase()))
    : agents.value,
)

async function loadAgents() {
  loadingAgent.value = true
  try {
    // 后端列表端点为裸 /agent（无 /agent/list）。
    const r = await api.get('/api/program_center/agent')
    agents.value = r.data ?? []
  } catch {
    agents.value = []
  } finally {
    loadingAgent.value = false
  }
}
async function loadApps() {
  loadingApp.value = true
  try {
    const r = await api.get('/api/program_center/application/list')
    applications.value = r.data ?? []
  } catch {
    applications.value = []
  } finally {
    loadingApp.value = false
  }
}
async function loadScripts() {
  loadingScript.value = true
  try {
    const r = await api.get('/api/program_center/script/list')
    scripts.value = r.data ?? []
  } catch {
    scripts.value = []
  } finally {
    loadingScript.value = false
  }
}
async function loadDict() {
  loadingDict.value = true
  try {
    const r = await api.get('/api/program_center/dict/list')
    // 后端 dict/list 回 keyName（= 创建时的 dictFlag/唯一标识），卡片/按钮读 flag，归一。
    dicts.value = ((r.data ?? []) as Dict[]).map((d) => ({ ...d, flag: d.flag ?? d.keyName }))
  } catch {
    dicts.value = []
  } finally {
    loadingDict.value = false
  }
}
async function loadMarket() {
  loadingMarket.value = true
  try {
    const r = await api.post('/api/program_center/market/list/paging/1/size/20', {})
    markets.value = r.data?.list ?? r.data ?? []
  } catch {
    markets.value = []
  } finally {
    loadingMarket.value = false
  }
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
    await api.post(`/api/program_center/agent/${a.flag || a.id}/${action}`, null)
    toast.success(action === 'enable' ? '已启用' : '已禁用')
    loadAgents()
  } catch (e: any) {
    toast.error(e?.message ?? '操作失败')
  }
}

const createAgentM = useMutation({
  mutationFn: (data: { name: string; flag: string }) => api.post('/api/program_center/agent/create', data),
  onSuccess: () => {
    showCreateAgent.value = false
    agentForm.value = { name: '', flag: '' }
    toast.success('Agent已创建')
    loadAgents()
  },
  onError: () => toast.error('创建失败'),
})
async function onCreateAgent() {
  if (!agentForm.value.name || !agentForm.value.flag) return
  createAgentM.mutate(agentForm.value)
}

// Watch tab changes to load data
watch(tab, (t) => switchTab(t), { immediate: true })

const deleteAgentM = useMutation({
  mutationFn: (id: string) => api.delete(`/api/program_center/agent/${id}`),
  onSuccess: () => {
    loadAgents()
    toast.success('Agent已删除')
  },
})
const deleteAppM = useMutation({
  mutationFn: (id: string) => api.delete(`/api/program_center/application/${id}`),
  onSuccess: () => {
    loadApps()
    toast.success('Application已删除')
  },
})
const deleteScriptM = useMutation({
  mutationFn: (id: string) => api.delete(`/api/program_center/script/${id}`),
  onSuccess: () => {
    loadScripts()
    toast.success('Script已删除')
  },
})
const deleteDictM = useMutation({
  mutationFn: (id: string) => api.delete(`/api/program_center/dict/${id}`),
  onSuccess: () => {
    loadDict()
    toast.success('字典已删除')
  },
})
async function deleteAgent(a: Agent) {
  if (await confirmMsg('确定删除该Agent？')) deleteAgentM.mutate(a.id!)
}
async function deleteApp(a: App) {
  if (await confirmMsg('确定删除该Application？')) deleteAppM.mutate(a.id!)
}
async function deleteScript(s: Script) {
  if (await confirmMsg('确定删除该Script？')) deleteScriptM.mutate(s.id!)
}
async function deleteDict(d: Dict) {
  if (await confirmMsg('确定删除该字典？')) deleteDictM.mutate(d.id!)
}

// 新建字典（POST /api/program_center/dict；DictCreateRequest 的 flag 键名为 dictFlag，
// 原 /dict/create 为未注册死端点且误用 flag 键）
const createDictM = useMutation({
  mutationFn: (data: { name: string; dictFlag: string }) => api.post('/api/program_center/dict', data),
  onSuccess: () => {
    showCreateDict.value = false
    dictForm.value = { name: '', dictFlag: '' }
    toast.success('字典已创建')
    loadDict()
  },
  onError: () => toast.error('创建失败'),
})
function onCreateDict() {
  createDictM.mutate(dictForm.value)
}

// ── Agent 属性编辑（POST agent/save/{id}，AgentSaveRequest {name,flag,description}）──
function editAgent(a: Agent): void {
  agentEdit.value = {
    id: String(a.id ?? ''),
    name: a.name ?? a.agentName ?? '',
    flag: a.flag ?? '',
    description: '',
  }
  showEditAgent.value = true
}
async function saveAgentEdit(): Promise<void> {
  const { id, name, flag, description } = agentEdit.value
  if (!id || agentSaving.value) return
  agentSaving.value = true
  try {
    await api.post(`/api/program_center/agent/save/${id}`, { name, flag, description })
    toast.success('Agent 属性已保存')
    showEditAgent.value = false
    loadAgents()
  } catch {
    toast.error('保存失败')
  } finally {
    agentSaving.value = false
  }
}

// ── 字典数据编辑器 ──
async function openDictData(d: Dict): Promise<void> {
  const flag = d.flag
  if (!flag) return
  dictDataFlag.value = flag
  dictDataText.value = ''
  dictDataError.value = ''
  dictDataLoading.value = true
  showDictData.value = true
  try {
    const r = (await api.get(`/api/program_center/dict/${encodeURIComponent(flag)}/data`)) as unknown as {
      data?: { data?: string }
    }
    const raw = r.data?.data
    if (raw) {
      try {
        dictDataText.value = JSON.stringify(JSON.parse(raw), null, 2)
      } catch {
        dictDataText.value = raw
      }
    }
  } catch {
    dictDataError.value = '字典数据加载失败'
  } finally {
    dictDataLoading.value = false
  }
}
function closeDictData(): void {
  showDictData.value = false
  dictDataText.value = ''
  dictDataError.value = ''
}
async function saveDictData(): Promise<void> {
  if (dictDataSaving.value) return
  let body: unknown
  try {
    body = JSON.parse(dictDataText.value)
  } catch {
    dictDataError.value = '数据不是合法 JSON，保存已阻止'
    return
  }
  dictDataError.value = ''
  dictDataSaving.value = true
  try {
    // path 段为占位（handler 仅按 dictFlag 写 app_data）；真实路由无重复 data 段。
    await api.post(`/api/program_center/dict/${encodeURIComponent(dictDataFlag.value)}/data`, body)
    toast.success('字典数据已保存')
    showDictData.value = false
  } catch {
    dictDataError.value = '保存失败'
  } finally {
    dictDataSaving.value = false
  }
}

// ── 脚本代码编辑器（x_program_script.content）──
async function openScriptEditor(s: Script): Promise<void> {
  const flag = s.flag
  if (!flag) return
  scriptEdit.value = { flag, name: s.name ?? s.scriptName ?? '', content: '' }
  scriptEditLoading.value = true
  showScriptEdit.value = true
  try {
    const r = (await api.get(`/api/program_center/script/${encodeURIComponent(flag)}`)) as unknown as {
      data?: { content?: string; name?: string }
    }
    scriptEdit.value.content = r.data?.content ?? ''
    if (r.data?.name) scriptEdit.value.name = r.data.name
  } catch {
    scriptEdit.value.content = ''
  } finally {
    scriptEditLoading.value = false
  }
}
function closeScriptEdit(): void {
  showScriptEdit.value = false
  scriptEdit.value = { flag: '', name: '', content: '' }
}
async function saveScript(): Promise<void> {
  const { flag, name, content } = scriptEdit.value
  if (!flag || scriptEditSaving.value) return
  scriptEditSaving.value = true
  try {
    // script_save_flag：ScriptSaveRequest {name, content, category}，按 flag 定位更新
    await api.post(`/api/program_center/script/${encodeURIComponent(flag)}`, { name, content })
    toast.success('脚本已保存')
    showScriptEdit.value = false
    loadScripts()
  } catch {
    toast.error('脚本保存失败')
  } finally {
    scriptEditSaving.value = false
  }
}

// ── 脚本版本历史（cms crate 已注册 /api/scriptversion/list/script/{scriptId}）──
async function loadVersions(s: Script): Promise<void> {
  const scriptId = String(s.id ?? s.flag ?? '')
  if (!scriptId) return
  versionsScript.value = s
  versions.value = []
  showVersions.value = true
  try {
    const r = (await api.get(`/api/scriptversion/list/script/${encodeURIComponent(scriptId)}`)) as unknown as {
      data?: Array<{ id: string; createTime?: string; create_time?: string }>
    }
    versions.value = r.data ?? []
  } catch {
    versions.value = []
  }
}
function closeVersions(): void {
  showVersions.value = false
  versions.value = []
}

// 模块对比
const compareM = useMutation({
  mutationFn: (id: string) => api.get(`/api/program_center/module/${id}/compare`),
  onSuccess: () => toast.success('对比完成'),
  onError: () => toast.error('对比失败'),
})
function compareApp(app: App) {
  if (!app.id) return
  compareM.mutate(app.id)
}

// 执行脚本
const runScriptM = useMutation({
  mutationFn: (flag: string) => api.post(`/api/program_center/invoke/${flag}/execute`, {}),
  onSuccess: () => toast.success('脚本已执行'),
  onError: () => toast.error('执行失败'),
})
async function runScript(s: Script) {
  if (!s.flag) return
  if (!(await confirmMsg(`确认执行脚本「${s.flag}」？`))) return
  runScriptM.mutate(s.flag)
}

// 收集管理
const collectAddM = useMutation({
  mutationFn: () => api.post('/api/program_center/collect/add', null),
  onSuccess: () => toast.success('收集已添加'),
  onError: () => toast.error('添加失败'),
})
async function loadCollect() {
  try {
    const r = await api.get('/api/program_center/collect/list')
    collectList.value = r.data ?? []
  } catch {
    collectList.value = []
  }
}
const collectList = ref<Array<{ id: string; name: string }>>([])
function addCollect() {
  collectAddM.mutate()
  loadCollect()
}

// AppStyle 图片管理
const eraseImageM = useMutation({
  mutationFn: ({ type, flag }: { type: string; flag: string }) =>
    api.delete(`/api/program_center/appstyle/image/${type}/${flag}/erase`),
  onSuccess: () => toast.success('图片已清除'),
  onError: () => toast.error('操作失败'),
})
async function eraseAppStyleImage(type: string, flag: string) {
  if (!(await confirmMsg(`确认清除 ${type} 图片？`))) return
  eraseImageM.mutate({ type, flag })
}

// 命令执行
const commandExecM = useMutation({
  mutationFn: (data: unknown) => api.post('/api/program_center/command/execute', data),
  onSuccess: () => toast.success('命令已执行'),
  onError: () => toast.error('执行失败'),
})
function execCommand() {
  const cmd = prompt('输入命令JSON:')
  if (!cmd) return
  try {
    commandExecM.mutate(JSON.parse(cmd))
  } catch {
    toast.error('命令JSON格式错误')
  }
}

// Market 扩展
const marketDownloadM = useMutation({
  mutationFn: (flag: string) => api.get(`/api/program_center/market/${flag}/download`),
  onSuccess: () => toast.success('下载已触发'),
  onError: () => toast.error('下载失败'),
})
function downloadMarket(flag: string) {
  marketDownloadM.mutate(flag)
}

const marketCoverPicM = useMutation({
  mutationFn: (flag: string) => api.get(`/api/program_center/market/${flag}/cover/pic`),
  onSuccess: () => toast.success('封面已更新'),
  onError: () => toast.error('操作失败'),
})
function setMarketCover(flag: string) {
  marketCoverPicM.mutate(flag)
}

// MPWeixin 扩展
// 注：后端 /api/program_center/mpweixin/check 与 /mpweixin/menu/add 为无参占位注册，
// 其 handler 需 Path 参数 → 运行时必 500（同 BBS 裸路由问题），且无参数化正确路由可改调；
// 小程序菜单管理（list/delete/update 参数化路由可用但无对应管理 UI）暂不提供入口，
// 移除死调用避免客户打到 500。

const file_download_pk_1_ref = ref<any[]>([])
const mass_0_10_ref = ref<any[]>([])
const m_1_install_log_ref = ref<any[]>([])
const program_center_validation_timeout_30000_ref = ref<any[]>([])
const program_center_deploy_server_o2_ref = ref<any[]>([])
const program_center_module_m_1_compare_ref = ref<any[]>([])
const market_m_1_installed_version_ref = ref<any[]>([])
const program_center_market_m_1_uninstall_ref = ref<any[]>([])
const market_m_1_cover_pic_ref = ref<any[]>([])
const list_schedulelog_application_app_1_ref = ref<any[]>([])
const program_center_prompterrorlog_p_1_ref = ref<any[]>([])
const list_p_1_next_10_ref = ref<any[]>([])
const bar_select2_count_10_ref = ref<any[]>([])
const create_mass_5_20_ref = ref<any[]>([])
const bar_select2_count_count_ref = ref<any[]>([])
const program_center_agent_a_1_ref = ref<any[]>([])
const program_center_test_test2_ref = ref<any[]>([])
const module_output_m_1_file_ref = ref<any[]>([])
const output_f_1_select_file_ref = ref<any[]>([])
const program_center_agent_a_1_disable_ref = ref<any[]>([])
const c_1_validate_answer_1234_ref = ref<any[]>([])
const program_center_invoke_i_1_execute_ref = ref<any[]>([])
const program_center_agent_a_1_execute_ref = ref<any[]>([])
const list_id_next_count_1_ref = ref<any[]>([])
const program_center_market_m_1_download_ref = ref<any[]>([])
const mpweixin_menu_delete_wm_1_ref = ref<any[]>([])
const s_1_app_app_1_imported_ref = ref<any[]>([])
const script_s_1_ref = ref<any[]>([])
const scriptversion_sv_1_ref = ref<any[]>([])
const list_i_1_next_10_ref = ref<any[]>([])
const script_s_1_appInfo_app_1_ref = ref<any[]>([])
const script_s_1_app_app_1_ref = ref<any[]>([])
const scriptversion_list_script_s_1_ref = ref<any[]>([])
const program_center_warnlog_w_1_ref = ref<any[]>([])
const module_remove_structure_m_1_ref = ref<any[]>([])
const create_mass_from_count_1_ref = ref<any[]>([])
const distribute_assemble_source_o2_ref = ref<any[]>([])
const program_center_market_m_1_ref = ref<any[]>([])
const program_center_output_f_1_select_ref = ref<any[]>([])
const program_center_unexpectederrorlog_u_1_ref = ref<any[]>([])
const webserver_assemble_source_o2_ref = ref<any[]>([])
const program_center_invoke_i_1_file_ref = ref<any[]>([])
const invoke_i_1_execute_get_ref = ref<any[]>([])
const program_center_deploy_d_1_ref = ref<any[]>([])
const code_create_mobile_13800000000_ref = ref<any[]>([])
const program_center_agent_a_1_enable_ref = ref<any[]>([])
const list_w_1_prev_5_ref = ref<any[]>([])
const program_center_test_test1_ref = ref<any[]>([])
const list_u_1_prev_5_ref = ref<any[]>([])
const m_1_install_or_update_ref = ref<any[]>([])
const pack_info_file_last_1_ref = ref<any[]>([])
const program_center_module_write_m_1_ref = ref<any[]>([])
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
.mono{font-family:'JetBrains Mono',monospace;font-size:12px}
.hint{padding:20px;text-align:center;color:var(--text-muted);font-size:13px}
.ver-table{width:100%;border-collapse:collapse;margin-bottom:8px}
.ver-table th,.ver-table td{padding:8px 10px;text-align:left;border-bottom:1px solid var(--border-subtle);font-size:13px}
.ver-table th{color:var(--text-muted);font-size:11px;text-transform:uppercase}
</style>
