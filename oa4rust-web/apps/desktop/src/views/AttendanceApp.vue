<template>
  <div class="attendance-view">
    <div class="view-header glass-card">
      <h1>考勤管理</h1>
      <p class="subtitle">/api/attendance/assemble/control/*</p>
      <div class="hr">
        <input v-model="month" type="month" class="mi" @change="loadData" />
        <button class="eb" :disabled="exporting" @click="exportData">{{ exporting ? '导出中…' : '📤 导出' }}</button>
      </div>
    </div>
    <div class="stats-row">
      <div v-for="s in stats" :key="s.label" class="stat-card glass-card">
        <div class="sn" :style="{color:s.color}">{{s.value}}</div>
        <div class="sl">{{s.label}}</div>
      </div>
    </div>
    <div class="content-panel glass-card">
      <div class="pt"><span class="th">姓名</span><span class="th">日期</span><span class="th">上班</span><span class="th">下班</span><span class="th">工时</span><span class="th">状态</span></div>
      <div v-if="loading" class="ls"><div class="sk" v-for="i in 8" :key="i"></div></div>
      <div v-else-if="records.length===0" class="es"><div class="ei">📋</div><p>暂无考勤记录</p></div>
      <template v-else>
        <div v-for="r in records" :key="r.id" class="tr" :class="{late:r.isLate}">
          <span class="td">{{r.personName||r.name||'—'}}</span>
          <span class="td">{{fmtDate(r.date)}}</span>
          <span class="td cit">{{r.checkInTime||'—'}}</span>
          <span class="td cot">{{r.checkOutTime||'—'}}</span>
          <span class="td hw">{{r.workHours??'—'}}</span>
          <span class="td"><span class="badge" :class="statusClass(r.status)">{{statusTxt(r.status)}}</span></span>
        </div>
      </template>
      <div v-if="totalPages>1" class="pagination">
        <button class="pgb" :disabled="page<=1" @click="page--">‹</button>
        <span class="pgi">第{{page}}/{{totalPages}}页</span>
        <button class="pgb" :disabled="page>=totalPages" @click="page++">›</button>
      </div>
    </div>
    <div class="content-panel glass-card">
      <div class="pt">请假申请</div>
      <div v-if="appeals.length===0" class="es-sm"><p>暂无请假申请</p></div>
      <div v-else class="al">
        <div v-for="a in appeals" :key="a.id" class="ai">
          <div class="ai-info"><span class="an">{{a.personName}}</span><span class="at">{{a.typeName||a.type}}</span><span class="ad">{{fmtDate(a.startDate)}}~{{fmtDate(a.endDate)}}</span></div>
          <span class="badge" :class="appealClass(a.status)">{{appealStatus(a.status)}}</span>
          <div v-if="a.status==='pending'" class="aa">
            <button class="ba" @click="audit(a,'approved')">通过</button>
            <button class="br" @click="audit(a,'rejected')">驳回</button>
          </div>
        </div>
      </div>
    </div>
    <div class="content-panel glass-card">
      <div class="pt cfg-pt">
        <span>打卡地点</span>
        <button class="mini-add" @click="addWorkplace">+ 新增</button>
      </div>
      <div v-if="workplaces.length === 0" class="es-sm"><p>暂无打卡地点</p></div>
      <div v-else class="cfg-list">
        <div v-for="w in workplaces" :key="w.id" class="cfg-item">
          <div class="cfg-main"><span class="cfg-name">{{ w.name || '—' }}</span><span class="cfg-sub">{{ w.address || '未填写地址' }}</span></div>
          <button class="cfg-del" @click="removeWorkplace(w)">删除</button>
        </div>
      </div>
    </div>
    <div class="content-panel glass-card">
      <div class="pt cfg-pt">
        <span>考勤设置项</span>
        <button class="mini-add" @click="addSetting">+ 新增</button>
      </div>
      <div v-if="settings.length === 0" class="es-sm"><p>暂无设置项</p></div>
      <div v-else class="cfg-list">
        <div v-for="s in settings" :key="s.id" class="cfg-item">
          <div class="cfg-main"><span class="cfg-name">{{ s.name || s.code }}</span><span class="cfg-sub">{{ s.code }} = {{ s.value || '—' }}</span></div>
          <button class="cfg-del" @click="removeSetting(s)">删除</button>
        </div>
      </div>
    </div>
    <div class="content-panel glass-card">
      <div class="pt cfg-pt">
        <span>排班与配置</span>
        <span class="cfg-tabs">
          <button class="cfg-tab" :class="{on:moreTab==='schedule'}" @click="switchMore('schedule')">排班</button>
          <button class="cfg-tab" :class="{on:moreTab==='employee'}" @click="switchMore('employee')">员工配置</button>
          <button class="cfg-tab" :class="{on:moreTab==='holiday'}" @click="switchMore('holiday')">自助假期</button>
          <button class="cfg-tab" :class="{on:moreTab==='workday'}" @click="switchMore('workday')">工作日</button>
          <button class="cfg-tab" :class="{on:moreTab==='admin'}" @click="switchMore('admin')">管理员</button>
          <button class="cfg-tab" :class="{on:moreTab==='importlog'}" @click="switchMore('importlog')">导入记录</button>
          <button class="cfg-tab" :class="{on:moreTab==='statlog'}" @click="switchMore('statlog')">统计日志</button>
          <button class="cfg-tab" :class="{on:moreTab==='cycle'}" @click="switchMore('cycle')">统计周期</button>
          <button class="cfg-tab" :class="{on:moreTab==='v2wp'}" @click="switchMore('v2wp')">v2地点</button>
        </span>
        <button class="mini-add" v-if="moreTab==='cycle'" @click="addCycle">+ 新建周期</button>
      </div>
      <div v-if="moreItems.length === 0" class="es-sm"><p>暂无数据</p></div>
      <div v-else class="cfg-list">
        <div v-for="it in moreItems" :key="it.id" class="cfg-item">
          <div class="cfg-main"><span class="cfg-name">{{ it.name || it.ruleName || it.id }}</span><span class="cfg-sub">{{ it.id }}</span></div>
        </div>
      </div>
    </div>
  </div>
</template>
<script setup lang="ts">
import { api, useSession } from '@oa4rust/sdk'
import { useMutation, useQuery, useQueryClient } from '@tanstack/vue-query'
import { computed, onMounted, ref } from 'vue'
import { confirmMsg, toast } from '../utils/toast'

const session = useSession()

interface R {
  id: string
  personName?: string
  name?: string
  date?: string
  checkInTime?: string
  checkOutTime?: string
  workHours?: number
  status?: string
  isLate?: boolean
}
interface A {
  id: string
  personName?: string
  type?: string
  typeName?: string
  startDate?: string
  endDate?: string
  status?: string
}
const month = ref(new Date().toISOString().slice(0, 7)),
  page = ref(1),
  records = ref<R[]>([]),
  appeals = ref<A[]>([]),
  loading = ref(false),
  totalPages = ref(1),
  qc = useQueryClient()
const stats = computed(() => [
  { label: '应出勤', value: 45, color: 'var(--color-info)' },
  { label: '实际出勤', value: 42, color: 'var(--color-success)' },
  { label: '迟到', value: 3, color: 'var(--color-warning)' },
  { label: '请假', value: 2, color: 'var(--color-accent)' },
])
const { data } = useQuery({
  queryKey: ['att', 'recs', month, page],
  queryFn: () =>
    // 裸 attendancedetail 仅注册 POST；列表真实端点为 attendancedetail/filter/list。
    api
      .get(`/api/attendance/assemble/control/attendancedetail/filter/list?month=${month.value}&page=${page.value}&size=20`)
      .then((r: any) => {
        records.value = r.data?.list ?? []
        totalPages.value = Math.ceil((r.data?.total ?? 1) / 20)
        return r
      }),
})
useQuery({
  queryKey: ['att', 'apps'],
  queryFn: () => api.get('/api/attendance/appeal/list').then((r: any) => (appeals.value = (r.data ?? []) as A[])),
  staleTime: 120000,
})
function loadData() {
  data.value?.refetch()
}
function fmtDate(d?: string) {
  if (!d) return '—'
  try {
    return new Date(d).toLocaleDateString('zh-CN', { month: '2-digit', day: '2-digit' })
  } catch {
    return String(d)
  }
}
function statusTxt(s?: string) {
  return s === '1' ? '正常' : s === '2' ? '迟到' : '—'
}
function statusClass(s?: string) {
  return s === '1' ? 'normal' : s === '2' ? 'late' : ''
}
function appealClass(s?: string) {
  return s === 'approved' ? 'approved' : s === 'rejected' ? 'rejected' : s === 'pending' ? 'pending' : ''
}
function appealStatus(s?: string) {
  return s === 'approved' ? '已通过' : s === 'rejected' ? '已驳回' : s === 'pending' ? '待审批' : '—'
}
const am = useMutation({
  // 后端 audit_appeal 读取 auditStatus（非 status）
  mutationFn: ({ id, status }: { id: string; status: string }) =>
    api.post('/api/attendance/appeal/audit', { id, auditStatus: status }),
  onSuccess: () => qc.invalidateQueries({ queryKey: ['att', 'apps'] }),
})
function audit(a: A, action: string) {
  am.mutate({ id: a.id, status: action })
}
const exporting = ref(false)
async function exportData() {
  if (exporting.value) return
  exporting.value = true
  try {
    // 按所选月份区间调后端 v2 统计导出端点，返回 {status,count} 聚合行，
    // 前端落成 CSV（带 BOM 保证 Excel 中文不乱码）本地下载。
    const ym = month.value || new Date().toISOString().slice(0, 7)
    const [y, m] = ym.split('-')
    const endDate = new Date(Number(y), Number(m), 0).toISOString().slice(0, 10)
    const r: any = await api.post('/api/attendance/assemble/control/v2/detail/statistic/export/filter', {
      startDate: `${ym}-01`,
      endDate,
      person: '',
    })
    const rows: Array<{ status?: string; count?: number }> = r.data?.data ?? []
    const label: Record<string, string> = { '1': '正常', '2': '迟到' }
    const csv =
      '\uFEFF状态,次数\n' + rows.map((x) => `${label[x.status ?? ''] ?? x.status ?? '未知'},${x.count ?? 0}`).join('\n')
    const blob = new Blob([csv], { type: 'text/csv;charset=utf-8' })
    const a = document.createElement('a')
    a.href = URL.createObjectURL(blob)
    a.download = `attendance-stat-${ym}.csv`
    a.click()
    URL.revokeObjectURL(a.href)
    toast.success(`已导出 ${rows.length} 条统计`)
  } catch {
    toast.error('考勤导出失败，请稍后重试')
  } finally {
    exporting.value = false
  }
}
onMounted(loadData)

// Additional attendance API calls
const ruleList = ref<Array<{ id: string; name?: string; type?: string; config?: string }>>([])
async function loadRules() {
  try {
    const r = await api.get('/api/attendance/assemble/control/rule/list')
    ruleList.value = (r.data ?? []) as any[]
  } catch {
    ruleList.value = []
  }
}
async function createRule() {
  const name = prompt('规则名称:')
  if (!name) return
  try {
    // 后端 create_control_rule 读取 ruleName/ruleType/enabled/description
    await api.post('/api/attendance/assemble/control/rule/create', {
      ruleName: name,
      ruleType: 'normal',
      enabled: true,
      description: '',
    })
    loadRules()
  } catch (e: any) {
    toast.error('创建失败: ' + (e?.message ?? ''))
  }
}
async function deleteRule(rule: any) {
  if (!(await confirmMsg('确定删除规则「' + (rule.name || rule.id) + '」？'))) return
  try {
    await api.delete('/api/attendance/assemble/control/rule/' + rule.id)
    loadRules()
  } catch (e: any) {
    toast.error('删除失败: : ' + (e?.message ?? ''))
  }
}
async function submitAppeal() {
  const type = prompt('请假类型 (sick/personal/vacation):', 'sick')
  if (!type) return
  const start = prompt('开始日期:', new Date().toISOString().slice(0, 10))
  const end = prompt('结束日期:', new Date().toISOString().slice(0, 10))
  if (!start || !end) return
  try {
    // 后端 appeal/submit 契约：personId + appealDate + reason + creator（单日期，无结束日字段）
    await api.post('/api/attendance/appeal/submit', {
      personId: session.state.user?.unique ?? '',
      creator: session.state.user?.unique ?? '',
      appealDate: start,
      reason: type,
    })
    loadAppeals()
  } catch (e: any) {
    toast.error('申请失败: : ' + (e?.message ?? ''))
  }
}
async function loadAppeals() {
  try {
    const r = await api.get('/api/attendance/appeal/list')
    appeals.value = (r.data ?? []) as A[]
  } catch {
    appeals.value = []
  }
}
loadRules()

// 考勤配置：打卡地点 + 设置项（管理端 CRUD，后端 workplace/attendancesetting 族）
interface WP {
  id: string
  name?: string
  address?: string
}
interface ST {
  id: string
  code: string
  name?: string
  value?: string
}
const workplaces = ref<WP[]>([])
const settings = ref<ST[]>([])
async function loadWorkplaces() {
  try {
    const r: any = await api.get('/api/attendance/assemble/control/workplace/list/all')
    workplaces.value = (r.data ?? []) as WP[]
  } catch {
    workplaces.value = []
  }
}
async function addWorkplace() {
  const name = prompt('打卡地点名称:')
  if (!name) return
  const address = prompt('地址（可选）:', '') ?? ''
  try {
    // 后端 workplace_create 读取 name(必填)/address(可选)，creator 取自会话
    await api.post('/api/attendance/assemble/control/workplace', { name, address })
    loadWorkplaces()
  } catch (e: any) {
    toast.error('新增失败: ' + (e?.message ?? ''))
  }
}
async function removeWorkplace(w: WP) {
  if (!(await confirmMsg('确定删除打卡地点「' + (w.name || w.id) + '」？'))) return
  try {
    await api.delete('/api/attendance/assemble/control/workplace/' + w.id)
    loadWorkplaces()
  } catch (e: any) {
    toast.error('删除失败: ' + (e?.message ?? ''))
  }
}
async function loadSettings() {
  try {
    const r: any = await api.get('/api/attendance/assemble/control/attendancesetting/list/all')
    settings.value = (r.data ?? []) as ST[]
  } catch {
    settings.value = []
  }
}
async function addSetting() {
  const code = prompt('设置项编码 (code):')
  if (!code) return
  const name = prompt('名称（可选）:', '') ?? ''
  const value = prompt('值（可选）:', '') ?? ''
  try {
    // 后端 attendancesetting_create 读取 code(必填)/name/value
    await api.post('/api/attendance/assemble/control/attendancesetting', { code, name, value })
    loadSettings()
  } catch (e: any) {
    toast.error('新增失败: ' + (e?.message ?? ''))
  }
}
async function removeSetting(s: ST) {
  if (!(await confirmMsg('确定删除设置项「' + (s.name || s.code) + '」？'))) return
  try {
    await api.delete('/api/attendance/assemble/control/attendancesetting/' + s.id)
    loadSettings()
  } catch (e: any) {
    toast.error('删除失败: ' + (e?.message ?? ''))
  }
}
type MoreItem = { id: string; name?: string; ruleName?: string }
const moreTab = ref<'schedule' | 'employee' | 'holiday' | 'workday' | 'admin' | 'importlog' | 'statlog' | 'cycle' | 'v2wp'>('schedule')
const moreItems = ref<MoreItem[]>([])
const moreEndpoints: Record<string, string> = {
  schedule: '/api/attendance/assemble/control/attendanceschedulesetting/list/all',
  employee: '/api/attendance/assemble/control/attendanceemployeeconfig/list/all',
  holiday: '/api/attendance/assemble/control/attendanceselfholiday/list/all',
  workday: '/api/attendance/assemble/control/attendanceworkdayconfig/list/all',
  admin: '/api/attendance/assemble/control/attendanceadmin/list/all',
  importlog: '/api/attendance/assemble/control/attendanceimportfileinfo/list/all',
  statlog: '/api/attendance/assemble/control/attendancestatisticrequirelog/list/all',
  v2wp: '/api/attendance/assemble/control/v2/workplace/list/all',
}
async function switchMore(t: 'schedule' | 'employee' | 'holiday' | 'workday' | 'admin' | 'importlog' | 'statlog' | 'cycle' | 'v2wp') {
  moreTab.value = t
  if (t === 'cycle') {
    // 统计周期无 list/all 端点，仅支持按 id 建/删；切到该标签清空列表，由新建后展示
    moreItems.value = []
    return
  }
  try {
    const r: any = await api.get(moreEndpoints[t])
    moreItems.value = (r.data ?? []) as MoreItem[]
  } catch {
    moreItems.value = []
  }
}
async function addCycle() {
  const cycleYear = prompt('周期年份 (cycleYear):', String(new Date().getFullYear()))
  if (!cycleYear) return
  const cycleMonth = prompt('周期月份 (cycleMonth):', String(new Date().getMonth() + 1))
  if (!cycleMonth) return
  try {
    // 后端 attendancestatisticalcycle_create 读 cycleYear/cycleMonth(必填)/topUnitName/unitName/description
    const r: any = await api.post('/api/attendance/assemble/control/attendancestatisticalcycle', {
      cycleYear,
      cycleMonth,
    })
    const id = r.data?.id
    if (id) {
      // 回读一次确认（GET {id}）
      const g: any = await api.get('/api/attendance/assemble/control/attendancestatisticalcycle/' + id)
      moreItems.value = [(g.data ?? { id, name: cycleYear + '-' + cycleMonth }) as MoreItem]
    }
    toast.success('已新建统计周期')
  } catch (e: any) {
    toast.error('新建失败: ' + (e?.message ?? ''))
  }
}

loadWorkplaces()
loadSettings()
switchMore('schedule')

async function loadStatistics() {
  try {
    const r = await api.get('/api/attendance/assemble/control/statistics/list?month=' + month.value)
    attStats.value = r.data ?? []
  } catch {
    attStats.value = []
  }
}
</script>
<style scoped>
.attendance-view{display:flex;flex-direction:column;gap:16px;height:100%}
.view-header{display:flex;align-items:center;justify-content:space-between;padding:16px 24px}
.view-header h1{font-family:'Orbitron',sans-serif;font-size:20px;color:var(--color-primary);margin:0 0 4px;text-shadow:0 0 15px var(--color-primary-glow)}
.subtitle{font-size:12px;color:var(--text-muted);margin:0;font-family:'JetBrains Mono',monospace}
.hr{display:flex;align-items:center;gap:10px}
.mi{background:var(--bg-elevated);border:1px solid var(--border-subtle);border-radius:var(--radius-md);padding:6px 12px;color:var(--text-primary);font-size:13px;outline:none}
.eb{padding:6px 14px;border-radius:var(--radius-md);border:1px solid var(--color-primary);background:var(--color-primary-soft);color:var(--color-primary);cursor:pointer;font-size:13px}
.stats-row{display:grid;grid-template-columns:repeat(4,1fr);gap:12px}
.stat-card{padding:16px;text-align:center}
.sn{font-family:'Orbitron',sans-serif;font-size:28px;font-weight:700}
.sl{font-size:12px;color:var(--text-muted);margin-top:4px}
.content-panel{flex:1;overflow-y:auto;padding:16px}
.pt{font-size:14px;color:var(--color-primary);font-weight:600;margin-bottom:12px;font-family:'Orbitron',sans-serif}
.th{font-size:11px;color:var(--text-muted);text-transform:uppercase;letter-spacing:1px}
.tr{display:grid;grid-template-columns:80px 80px 90px 90px 60px 70px;gap:4px;padding:8px 12px;border-radius:var(--radius-sm);font-size:13px;color:var(--text-primary);transition:all var(--transition-fast)}
.tr:hover{background:var(--color-primary-soft)}
.td{color:var(--text-primary)}
.td.cit.late{color:var(--color-warning)}
.badge{padding:2px 8px;border-radius:8px;font-size:11px;font-weight:600}
.badge.normal,.badge.approved{background:var(--color-success-glow);color:var(--color-success)}
.badge.late,.badge.pending{background:var(--color-warning-glow);color:var(--color-warning)}
.badge.rejected{background:var(--color-error-glow);color:var(--color-error)}
.al{display:flex;flex-direction:column;gap:8px}
.ai{display:flex;align-items:center;gap:12px;padding:10px 12px;background:var(--bg-elevated);border-radius:var(--radius-md)}
.ai-info{display:flex;gap:12px;flex:1;font-size:13px;color:var(--text-secondary)}
.an{font-weight:600;color:var(--text-primary)}
.at{background:var(--color-accent-soft);color:var(--color-accent);padding:2px 8px;border-radius:8px;font-size:11px}
.ad{font-size:12px;color:var(--text-muted)}
.aa{display:flex;gap:6px}
.ba{padding:4px 12px;border-radius:var(--radius-sm);border:1px solid var(--color-success);background:var(--color-success-soft);color:var(--color-success);cursor:pointer;font-size:12px}
.br{padding:4px 12px;border-radius:var(--radius-sm);border:1px solid var(--color-error);background:var(--color-error-glow);color:var(--color-error);cursor:pointer;font-size:12px}
.pagination{display:flex;align-items:center;justify-content:center;gap:12px;margin-top:12px;padding-top:12px;border-top:1px solid var(--border-subtle)}
.pgb{width:32px;height:32px;border-radius:var(--radius-sm);border:1px solid var(--border-subtle);background:var(--bg-elevated);color:var(--text-secondary);cursor:pointer;font-size:16px}
.pgb:disabled{opacity:.3;cursor:not-allowed}
.pgi{font-size:13px;color:var(--text-muted)}
.cfg-pt{display:flex;align-items:center;justify-content:space-between}
.mini-add{padding:4px 12px;border-radius:var(--radius-sm);border:1px solid var(--color-primary);background:var(--color-primary-soft);color:var(--color-primary);cursor:pointer;font-size:12px}
.cfg-list{display:flex;flex-direction:column;gap:8px}
.cfg-item{display:flex;align-items:center;gap:12px;padding:10px 12px;background:var(--bg-elevated);border-radius:var(--radius-md)}
.cfg-main{display:flex;flex-direction:column;gap:2px;flex:1}
.cfg-name{font-weight:600;color:var(--text-primary);font-size:13px}
.cfg-sub{font-size:12px;color:var(--text-muted)}
.cfg-del{padding:4px 12px;border-radius:var(--radius-sm);border:1px solid var(--color-error);background:var(--color-error-glow);color:var(--color-error);cursor:pointer;font-size:12px}
.es,.ls,.es-sm{display:flex;flex-direction:column;align-items:center;justify-content:center;padding:40px;color:var(--text-muted);gap:12px}
.ei{font-size:48px;opacity:0.4}
.sk{height:36px;border-radius:var(--radius-sm);margin-bottom:6px;background:var(--bg-elevated)}
@media(max-width:768px){.stats-row{grid-template-columns:repeat(2,1fr)}.tr{grid-template-columns:60px 60px 70px 70px 50px 60px}}
.cfg-tabs{display:flex;gap:4px}
.cfg-tab{padding:2px 10px;border-radius:10px;border:1px solid var(--border-subtle);background:var(--bg-elevated);color:var(--text-muted);cursor:pointer;font-size:11px}
.cfg-tab.on{border-color:var(--color-primary);color:var(--color-primary);background:var(--color-primary-soft)}
</style>
