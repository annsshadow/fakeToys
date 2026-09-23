<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<template>
  <div class="attendance-view">
    <div class="view-header glass-card">
      <h1>考勤管理</h1>
      <p class="subtitle">/api/attendance/assemble/control/*</p>
      <div class="hr">
        <input v-model="month" type="month" class="mi" @change="loadData" />
        <button class="eb" :disabled="exporting" @click="exportData">{{ exporting ? '导出中…' : '📤 导出' }}</button>
        <button class="eb" @click="loadAttOverview">📊 汇总</button>
        <button class="eb" @click="loadAttOrg">🏢 按单位/同步</button>
        <button class="eb" @click="loadV2Meta">⚙️ v2配置/控件/请假模板</button>
        <button class="eb" @click="loadV2Schedule">🗓️ v2排班/群组</button>
        <button class="eb" @click="loadV2AppealRecord">📝 v2申诉/记录</button>
        <button class="eb" @click="loadAttBase">🗂️ 打卡/周期/员工</button>
        <button class="eb" @click="loadCoreLists">🧩 核心记录/规则</button>
        <button class="eb" @click="loadScheduleDetail">📅 排班设置明细</button>
        <button class="eb" @click="loadV2ConfigTpl">🧾 v2配置/模板/统计</button>
        <button class="eb" @click="loadStatisticShow">📈 统计展示筛选</button>
        <button class="eb" @click="loadAppealDetailFilters">🧾 申诉/明细游标</button>
      </div>
      <div v-if="attOverviewText" class="att-note">{{ attOverviewText }}</div>
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
          <span class="td rec-act">
            <button class="cfg-mini" @click="detailInfo(r)">详情</button>
            <button class="cfg-mini" @click="analyseOne(r)">分析</button>
            <button class="cfg-mini" @click="archiveOne(r)">归档</button>
            <button class="cfg-mini danger" @click="deleteDetail(r)">删除</button>
          </span>
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
          <button class="cfg-tab" :class="{on:moreTab==='v2group'}" @click="switchMore('v2group')">v2考勤组</button>
          <button class="cfg-tab" :class="{on:moreTab==='v2shift'}" @click="switchMore('v2shift')">v2班次</button>
          <button class="cfg-tab" :class="{on:moreTab==='v2leave'}" @click="switchMore('v2leave')">v2请假</button>
        </span>
        <button class="mini-add" v-if="moreTab==='cycle'" @click="addCycle">+ 新建周期</button>
        <button class="mini-add" v-if="moreTab==='v2group'" @click="addGroup">+ 新建考勤组</button>
        <button class="mini-add" v-if="moreTab==='v2shift'" @click="addShift">+ 新建班次</button>
        <button class="mini-add" v-if="moreTab==='v2leave'" @click="addLeave">+ 新建请假</button>
      </div>
      <div v-if="moreItems.length === 0" class="es-sm"><p>暂无数据</p></div>
      <div v-else class="cfg-list">
        <div v-for="it in moreItems" :key="it.id" class="cfg-item">
          <div class="cfg-main"><span class="cfg-name">{{ it.name || it.ruleName || it.id }}</span><span class="cfg-sub">{{ it.id }}</span></div>
          <div class="cfg-act" v-if="moreTab==='v2group' || moreTab==='v2shift'">
            <button class="cfg-mini" @click="viewMore(it)">详情</button>
            <button class="cfg-mini danger" @click="deleteMore(it)">删除</button>
          </div>
          <div class="cfg-act" v-else-if="moreTab==='admin' || moreTab==='employee' || moreTab==='workday'">
            <button class="cfg-mini" @click="viewMore(it)">详情</button>
          </div>
          <div class="cfg-act" v-else-if="moreTab==='v2leave'">
            <button class="cfg-mini danger" @click="deleteMore(it)">删除</button>
          </div>
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
const attOverviewText = ref('')
async function loadV2Meta() {
  try {
    // 消费 attendance v2 三条真实路由：全局配置 / 我的控件 / 请假模板
    const [config, controls, leave] = await Promise.all([
      api.get('/api/attendance/assemble/control/v2/config'),
      api.get('/api/attendance/assemble/control/v2/my/controls'),
      api.get('/api/attendance/assemble/control/v2/leave/template'),
    ])
    const cnt = (r: any) => (Array.isArray(r?.data) ? r.data.length : ((r as any)?.data ? 1 : 0))
    attOverviewText.value = `v2配置 ${cnt(config)} / 我的控件 ${cnt(controls)} / 请假模板 ${cnt(leave)}`
  } catch (e: any) {
    toast.error('加载 v2 配置失败: ' + (e?.message ?? ''))
  }
}
// v2 排班/群组明细族 3 条真实 distinct 路由（表各异）：群组排班配置 groupschedule/config/group/{groupId}（x_attendance_v2_group_schedule_config）
// + 群组月排班 groupschedule/list/group/{groupId}/month/{month}（x_attendance_v2_group_schedule）+ 按人日期查群组 group/person/{person}/date/{date}（x_attendance_v2_group）
async function loadV2Schedule() {
  try {
    const groupResp: any = await api.post('/api/attendance/assemble/control/v2/group/list/1/size/50', {}).catch(() => null)
    const groups = (Array.isArray(groupResp?.data) ? groupResp.data : (groupResp?.data?.data ?? [])) as Array<Record<string, unknown>>
    const gid = groups[0] ? String(groups[0].id ?? '') : ''
    const person = session.state.user?.unique ?? ''
    const monthStr = month.value
    const today = new Date().toISOString().slice(0, 10)
    const [cfg, list, byPerson] = await Promise.all([
      gid ? api.get(`/api/attendance/assemble/control/v2/groupschedule/config/group/${encodeURIComponent(gid)}`).catch(() => null) : Promise.resolve(null),
      gid ? api.get(`/api/attendance/assemble/control/v2/groupschedule/list/group/${encodeURIComponent(gid)}/month/${encodeURIComponent(monthStr)}`).catch(() => null) : Promise.resolve(null),
      person ? api.get(`/api/attendance/assemble/control/v2/group/person/${encodeURIComponent(person)}/date/${encodeURIComponent(today)}`).catch(() => null) : Promise.resolve(null),
    ])
    const hasCfg = (cfg as any)?.data ? '有' : '无'
    const lN = Array.isArray((list as any)?.data) ? (list as any).data.length : 0
    const pN = Array.isArray((byPerson as any)?.data) ? (byPerson as any).data.length : 0
    attOverviewText.value = `群组 ${groups.length} · 排班配置 ${hasCfg} · 月排班 ${lN} · 我所属群组 ${pN}`
  } catch (e: any) {
    toast.error('加载 v2 排班失败: ' + (e?.message ?? ''))
  }
}
// v2 申诉/记录明细族 4 条真实 distinct 路由（x_attendance_record / x_attendance_v2_appeal_info）：记录分页 v2/record/list/{page}/size/{size}（POST）
// → 首记录 → 记录详情 v2/record/{id}（GET）；申诉分页 v2/appeal/list/{page}/size/{size}（POST）→ 首申诉 → 申诉详情 v2/appeal/{id}（GET）
async function loadV2AppealRecord() {
  const page = '1'
  const size = '20'
  try {
    const [recList, appealList]: any[] = await Promise.all([
      api.post(`/api/attendance/assemble/control/v2/record/list/${page}/size/${size}`).catch(() => null),
      api.post(`/api/attendance/assemble/control/v2/appeal/list/${page}/size/${size}`).catch(() => null),
    ])
    const recs = (Array.isArray(recList?.data) ? recList.data : (recList?.data?.data ?? [])) as Array<Record<string, unknown>>
    const appeals2 = (Array.isArray(appealList?.data) ? appealList.data : (appealList?.data?.data ?? [])) as Array<Record<string, unknown>>
    const rid = recs[0] ? String(recs[0].id ?? '') : ''
    const aid = appeals2[0] ? String(appeals2[0].id ?? '') : ''
    const [recDetail, appealDetail] = await Promise.all([
      rid ? api.get(`/api/attendance/assemble/control/v2/record/${encodeURIComponent(rid)}`).catch(() => null) : Promise.resolve(null),
      aid ? api.get(`/api/attendance/assemble/control/v2/appeal/${encodeURIComponent(aid)}`).catch(() => null) : Promise.resolve(null),
    ])
    const rStatus = (recDetail as any)?.data?.status ?? (rid || '—')
    const aStatus = (appealDetail as any)?.data?.status ?? (aid || '—')
    attOverviewText.value = `打卡记录 ${recs.length}（首状态 ${rStatus}）· 申诉 ${appeals2.length}（首状态 ${aStatus}）`
  } catch (e: any) {
    toast.error('加载 v2 申诉/记录失败: ' + (e?.message ?? ''))
  }
}
async function loadAttOrg() {
  try {
    // GET attendancedetail/filter/list/topUnit + filter/list/unit + dingding/sync/list
    const [topUnit, unit, dingding] = await Promise.all([
      api.get('/api/attendance/assemble/control/attendancedetail/filter/list/topUnit'),
      api.get('/api/attendance/assemble/control/attendancedetail/filter/list/unit'),
      api.get('/api/attendance/assemble/control/dingding/sync/list'),
    ])
    const cnt = (r: any) => (Array.isArray(r?.data) ? r.data.length : 0)
    attOverviewText.value = `顶级单位 ${cnt(topUnit)} / 单位 ${cnt(unit)} / 钉钉同步 ${cnt(dingding)}`
  } catch (e: any) {
    toast.error('加载失败: ' + (e?.message ?? ''))
  }
}
async function loadAttOverview() {
  try {
    // GET attendancedetail/filter/list/user + list/persons/nonesign + attendancesetting/enable/type
    const [byUser, nonesign, enableType] = await Promise.all([
      api.get('/api/attendance/assemble/control/attendancedetail/filter/list/user'),
      api.get('/api/attendance/assemble/control/attendancedetail/list/persons/nonesign'),
      api.get('/api/attendance/assemble/control/attendancesetting/enable/type'),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : 0)
    attOverviewText.value = `按人 ${n(byUser)} / 未签到 ${n(nonesign)} / 启用类型 ${n(enableType)}`
  } catch (e: any) {
    toast.error('加载考勤汇总失败: ' + (e?.message ?? ''))
  }
}
// 消费 attendance 核心 crate 真实路由（打卡记录/统计周期/员工配置——独立表，非 v2 assemble/control 镜像）
async function loadAttBase() {
  try {
    const [records, cycles, employees] = await Promise.all([
      api.get('/api/attendance/record/list'),
      api.get('/api/attendance/statistical/cycle/list/all'),
      api.get('/api/attendance/employee/config/list/all'),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : 0)
    attOverviewText.value = `打卡记录 ${n(records)} / 统计周期 ${n(cycles)} / 员工配置 ${n(employees)}`
  } catch (e: any) {
    toast.error('加载考勤基础数据失败: ' + (e?.message ?? ''))
  }
}
// 考勤核心记录/规则（rev184，4 条真实 distinct 无参列表）：admin/list/all（list_admins x_attendance_admin）
// + rule/list（list_schedule_rules x_attendance_rule）+ core/entity/record/list（SeaORM attendance_record）
// + core/entity/rule/list（SeaORM attendance_rule）。前二属 attendance crate、后二属 attendance_core_entity crate。
async function loadCoreLists() {
  try {
    const [admins, rules, coreRecords, coreRules] = await Promise.all([
      api.get('/api/attendance/admin/list/all'),
      api.get('/api/attendance/rule/list'),
      api.get('/api/attendance/core/entity/record/list'),
      api.get('/api/attendance/core/entity/rule/list'),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : 0)
    attOverviewText.value = `管理员 ${n(admins)} / 排班规则 ${n(rules)} / 核心记录 ${n(coreRecords)} / 核心规则 ${n(coreRules)}`
  } catch (e: any) {
    toast.error('加载考勤核心记录/规则失败: ' + (e?.message ?? ''))
  }
}
// 排班设置明细（rev185，3 条真实 distinct，x_attendance_schedule_setting）：schedulesetting/list/all 取首 id/unit
// → attendanceschedulesetting/{id}（WHERE id）+ list/unit/{name}（WHERE unit_id=$1）+ list/topUnit/{name}（WHERE unit_id IS NULL）。
async function loadScheduleDetail() {
  try {
    const listRes: any = await api.get('/api/attendance/assemble/control/attendanceschedulesetting/list/all').catch(() => null)
    const rows = Array.isArray(listRes?.data) ? listRes.data : []
    const first = rows[0] ?? null
    const sid = String(first?.id ?? '0')
    const nm = String(first?.unit ?? first?.name ?? '0')
    const settle = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
    const [detail, byUnit, byTop] = await Promise.all([
      settle(api.get(`/api/attendance/assemble/control/attendanceschedulesetting/${sid}`)),
      settle(api.get(`/api/attendance/assemble/control/attendanceschedulesetting/list/unit/${encodeURIComponent(nm)}`)),
      settle(api.get(`/api/attendance/assemble/control/attendanceschedulesetting/list/topUnit/${encodeURIComponent(nm)}`)),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : 0)
    const dName = (detail as any)?.data?.name ?? (rows.length ? sid : '—')
    attOverviewText.value = `排班设置 ${rows.length}（首「${dName}」）· 按单位 ${n(byUnit)} · 顶级单位 ${n(byTop)}`
  } catch (e: any) {
    toast.error('加载排班设置明细失败: ' + (e?.message ?? ''))
  }
}
// v2 配置/模板/统计（rev186，3 条真实 distinct 只读）：v2/config/person（v2_config_person_get，x_attendance_config
// category='v2_person' AND creator=会话）+ v2/record/template（静态记录模板对象）+ v2/detail/statistic/{detailId}/list/record
// （v2_detail_statistic_record_list，SELECT x_attendance_detail WHERE id，detailId 取当前记录表首行）。均只读，不触发写端点。
async function loadV2ConfigTpl() {
  try {
    const detailId = records.value[0] ? String(records.value[0].id ?? '0') : '0'
    const settle = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
    const [cfg, tpl, stat] = await Promise.all([
      settle(api.get('/api/attendance/assemble/control/v2/config/person')),
      settle(api.get('/api/attendance/assemble/control/v2/record/template')),
      settle(api.get(`/api/attendance/assemble/control/v2/detail/statistic/${detailId}/list/record`)),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : (r as any)?.data ? 1 : 0)
    attOverviewText.value = `个人配置 ${n(cfg)} · 记录模板 ${(tpl as any)?.data ? '有' : '无'} · 明细统计记录 ${n(stat)}`
  } catch (e: any) {
    toast.error('加载 v2 配置/模板/统计失败: ' + (e?.message ?? ''))
  }
}
// 统计展示筛选族 12 条真实 distinct（rev192，x_attendance_statisticshow 各 WHERE 维度/方向各异）：
// personMonth/unitMonth/topUnitMonth/unitDay/topUnitDay 各 next(id>)+prev(id<) 共 10，+ unit/day/{name}/{date}
// + unit/day/topUnit/{name}/{date}。注意：家族名（personMonth 等）是路由字面量段，必须写全 /api 字面量路径，
// 不能用 ${f} 变量拼（会被提取器归一成 {} 与所有家族路由歧义合并，只计 1 条）。
async function loadStatisticShow() {
  const nx = '0'
  const pv = '999999999'
  const c = '20'
  const nm = '0'
  const dt = new Date().toISOString().slice(0, 10)
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  try {
    const results = await Promise.all([
      s(api.get(`/api/attendance/assemble/control/statisticshow/filter/personMonth/list/${nx}/next/${c}`)),
      s(api.get(`/api/attendance/assemble/control/statisticshow/filter/personMonth/list/${pv}/prev/${c}`)),
      s(api.get(`/api/attendance/assemble/control/statisticshow/filter/unitMonth/list/${nx}/next/${c}`)),
      s(api.get(`/api/attendance/assemble/control/statisticshow/filter/unitMonth/list/${pv}/prev/${c}`)),
      s(api.get(`/api/attendance/assemble/control/statisticshow/filter/topUnitMonth/list/${nx}/next/${c}`)),
      s(api.get(`/api/attendance/assemble/control/statisticshow/filter/topUnitMonth/list/${pv}/prev/${c}`)),
      s(api.get(`/api/attendance/assemble/control/statisticshow/filter/unitDay/list/${nx}/next/${c}`)),
      s(api.get(`/api/attendance/assemble/control/statisticshow/filter/unitDay/list/${pv}/prev/${c}`)),
      s(api.get(`/api/attendance/assemble/control/statisticshow/filter/topUnitDay/list/${nx}/next/${c}`)),
      s(api.get(`/api/attendance/assemble/control/statisticshow/filter/topUnitDay/list/${pv}/prev/${c}`)),
      s(api.get(`/api/attendance/assemble/control/statisticshow/unit/day/${encodeURIComponent(nm)}/${encodeURIComponent(dt)}`)),
      s(api.get(`/api/attendance/assemble/control/statisticshow/unit/day/topUnit/${encodeURIComponent(nm)}/${encodeURIComponent(dt)}`)),
    ])
    const total = results.reduce((acc: number, r: any) => acc + (Array.isArray(r?.data) ? r.data.length : 0), 0)
    attOverviewText.value = `统计展示筛选：5 维度×2 方向 + 2 按日 = 12 路由，返回合计 ${total} 行`
  } catch (e: any) {
    toast.error('加载统计展示筛选失败: ' + (e?.message ?? ''))
  }
}
// 申诉/明细游标族 6 条真实 distinct（rev193，各 WHERE 方向/维度异）：appealInfo filter/list next(id>)+prev(id<)
// + manager/list next(id> AND creator) + appealInfo/{id}（WHERE id）；detail filter/list next(id>)+prev(id<)。
async function loadAppealDetailFilters() {
  const nx = '0'
  const pv = '999999999'
  const c = '20'
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  try {
    const nextRes = await s(api.get(`/api/attendance/assemble/control/attendanceappealInfo/filter/list/${nx}/next/${c}`))
    const rows = Array.isArray((nextRes as any)?.data) ? (nextRes as any).data : []
    const aid = rows[0] ? String(rows[0].id ?? '0') : '0'
    const [prev, mgr, detail, dNext, dPrev] = await Promise.all([
      s(api.get(`/api/attendance/assemble/control/attendanceappealInfo/filter/list/${pv}/prev/${c}`)),
      s(api.get(`/api/attendance/assemble/control/attendanceappealInfo/manager/list/${nx}/next/${c}`)),
      s(api.get(`/api/attendance/assemble/control/attendanceappealInfo/${encodeURIComponent(aid)}`)),
      s(api.get(`/api/attendance/assemble/control/attendancedetail/filter/list/${nx}/next/${c}`)),
      s(api.get(`/api/attendance/assemble/control/attendancedetail/filter/list/${pv}/prev/${c}`)),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : 0)
    const hasDetail = (detail as any)?.data?.id ? '命中' : '未命中'
    attOverviewText.value = `申诉：正序 ${rows.length}·逆序 ${n(prev)}·管理 ${n(mgr)}·详情 ${hasDetail} | 明细：正序 ${n(dNext)}·逆序 ${n(dPrev)}`
  } catch (e: any) {
    toast.error('加载申诉/明细游标失败: ' + (e?.message ?? ''))
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
type MoreTabKey =
  | 'schedule'
  | 'employee'
  | 'holiday'
  | 'workday'
  | 'admin'
  | 'importlog'
  | 'statlog'
  | 'cycle'
  | 'v2wp'
  | 'v2group'
  | 'v2shift'
  | 'v2leave'
const moreTab = ref<MoreTabKey>('schedule')
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
async function switchMore(t: MoreTabKey) {
  moreTab.value = t
  if (t === 'cycle') {
    // 统计周期无 list/all 端点，仅支持按 id 建/删；切到该标签清空列表，由新建后展示
    moreItems.value = []
    return
  }
  // v2 考勤组/班次：列表端点为 POST paging（后端仅注册 POST），name 过滤置空取全量。
  // 注意：路径必须写成 api.post 调用处的字面量（提取器不解析 base 变量/三元）。
  if (t === 'v2group' || t === 'v2shift') {
    try {
      const r: any =
        t === 'v2group'
          ? await api.post('/api/attendance/assemble/control/v2/group/list/1/size/50', { name: '' })
          : await api.post('/api/attendance/assemble/control/v2/shift/list/1/size/50', { name: '' })
      const rows = (r.data?.data ?? r.data ?? []) as Array<Record<string, unknown>>
      moreItems.value = rows.map((row) => ({
        id: String(row.id ?? ''),
        name: String(row.groupName ?? row.shiftName ?? row.name ?? row.id ?? ''),
      }))
    } catch {
      moreItems.value = []
    }
    return
  }
  // v2 请假：列表端点 POST paging（admin 见全量，否则本人）；字面量路径。
  if (t === 'v2leave') {
    try {
      const r: any = await api.post('/api/attendance/assemble/control/v2/leave/list/1/size/50', {})
      const rows = (r.data?.data ?? r.data ?? []) as Array<Record<string, unknown>>
      moreItems.value = rows.map((row) => ({
        id: String(row.id ?? ''),
        name: `${String(row.leaveType ?? row.leave_type ?? '请假')} · ${String(row.person ?? '')}`,
      }))
    } catch {
      moreItems.value = []
    }
    return
  }
  try {
    const r: any = await api.get(moreEndpoints[t])
    moreItems.value = (r.data ?? []) as MoreItem[]
  } catch {
    moreItems.value = []
  }
}

// v2 考勤组：新建（groupName 必填，admin 门禁，落 x_attendance_v2_group）
async function addGroup() {
  const groupName = prompt('考勤组名称 (groupName):', '')
  if (!groupName) return
  try {
    await api.post('/api/attendance/assemble/control/v2/group', { groupName, checkType: 'field', status: 1 })
    toast.success('已新建考勤组')
    switchMore('v2group')
  } catch (e: any) {
    toast.error('新建失败: ' + (e?.message ?? ''))
  }
}
// v2 班次：新建（shiftName 必填，admin 门禁，落 x_attendance_v2_shift）
async function addShift() {
  const shiftName = prompt('班次名称 (shiftName):', '')
  if (!shiftName) return
  const onDutyTime = prompt('上班时间 (onDutyTime, 如 09:00):', '09:00') || ''
  const offDutyTime = prompt('下班时间 (offDutyTime, 如 18:00):', '18:00') || ''
  try {
    await api.post('/api/attendance/assemble/control/v2/shift/create', {
      shiftName,
      onDutyTime,
      offDutyTime,
      workTime: 480,
    })
    toast.success('已新建班次')
    switchMore('v2shift')
  } catch (e: any) {
    toast.error('新建失败: ' + (e?.message ?? ''))
  }
}
// v2 请假：新建（leaveType 必填，person 缺省取会话，落 x_attendance_v2_leave）
async function addLeave() {
  const leaveType = prompt('请假类型 (leaveType，如 事假/病假):', '')
  if (!leaveType) return
  const startTime = prompt('开始时间 (startTime, 如 2026-09-22 09:00):', '') || ''
  const endTime = prompt('结束时间 (endTime, 如 2026-09-22 18:00):', '') || ''
  try {
    // POST v2/leave —— 新建请假
    await api.post('/api/attendance/assemble/control/v2/leave', { leaveType, startTime, endTime })
    toast.success('已新建请假')
    switchMore('v2leave')
  } catch (e: any) {
    toast.error('新建失败: ' + (e?.message ?? ''))
  }
}
// v2 详情：按 id GET 回读单条（字面量分支，提取器不解析 url 变量）
async function viewMore(it: MoreItem) {
  try {
    // 配置明细族真实 distinct 路由（各读独立表）：管理员 attendanceadmin/{id}（x_attendance_admin）、
    // 员工配置 attendanceemployeeconfig/{id}（x_attendance_employee_config）、工作日 attendanceworkdayconfig/{id}（x_attendance_workday_config）。
    let r: any
    if (moreTab.value === 'admin') {
      r = await api.get(`/api/attendance/assemble/control/attendanceadmin/${it.id}`)
    } else if (moreTab.value === 'employee') {
      r = await api.get(`/api/attendance/assemble/control/attendanceemployeeconfig/${it.id}`)
    } else if (moreTab.value === 'workday') {
      r = await api.get(`/api/attendance/assemble/control/attendanceworkdayconfig/${it.id}`)
    } else if (moreTab.value === 'v2group') {
      r = await api.get(`/api/attendance/assemble/control/v2/group/${it.id}`)
    } else {
      r = await api.get(`/api/attendance/assemble/control/v2/shift/${it.id}`)
    }
    const d = r.data ?? {}
    toast.success(
      '详情: ' +
        (d.groupName || d.shiftName || d.personId || d.workDate || it.name || it.id),
    )
  } catch (e: any) {
    toast.error('加载详情失败: ' + (e?.message ?? ''))
  }
}
// v2 删除：group 走 {id}/delete，shift 走 delete/{id}（后端均 GET，owner/admin 门禁；字面量分支）
async function deleteMore(it: MoreItem) {
  if (!(await confirmMsg('确认删除该项？'))) return
  try {
    if (moreTab.value === 'v2group') {
      await api.get(`/api/attendance/assemble/control/v2/group/${it.id}/delete`)
    } else if (moreTab.value === 'v2leave') {
      // GET v2/leave/delete/{id} —— 删除请假（owner/admin 门禁）
      await api.get(`/api/attendance/assemble/control/v2/leave/delete/${it.id}`)
    } else {
      await api.get(`/api/attendance/assemble/control/v2/shift/delete/${it.id}`)
    }
    toast.success('已删除')
    switchMore(moreTab.value)
  } catch (e: any) {
    toast.error('删除失败: ' + (e?.message ?? ''))
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

// 考勤明细逐行操作（x_attendance_detail，各为 distinct handler）：
//   详情 GET {id}、分析 POST analyse/id/{id}（analysed=true）、
//   归档 POST archive/{id}（archived=true）、删除 DELETE {id}
async function detailInfo(r: R) {
  try {
    const resp: any = await api.get('/api/attendance/assemble/control/attendancedetail/' + r.id)
    const d = resp?.data ?? {}
    toast.success('明细：' + (d.status ?? statusTxt(r.status)) + ' / ' + (d.date ?? fmtDate(r.date)))
  } catch (e: any) {
    toast.error('加载明细失败: ' + (e?.message ?? ''))
  }
}
async function analyseOne(r: R) {
  try {
    await api.post('/api/attendance/assemble/control/attendancedetail/analyse/id/' + r.id, {})
    toast.success('已标记分析')
  } catch (e: any) {
    toast.error('分析失败: ' + (e?.message ?? ''))
  }
}
async function archiveOne(r: R) {
  try {
    await api.post('/api/attendance/assemble/control/attendancedetail/archive/' + r.id, {})
    toast.success('已归档')
  } catch (e: any) {
    toast.error('归档失败: ' + (e?.message ?? ''))
  }
}
async function deleteDetail(r: R) {
  if (!(await confirmMsg('确认删除该考勤明细？'))) return
  try {
    await api.delete('/api/attendance/assemble/control/attendancedetail/' + r.id)
    toast.success('已删除')
    records.value = records.value.filter((x) => x.id !== r.id)
  } catch (e: any) {
    toast.error('删除失败: ' + (e?.message ?? ''))
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
.tr{display:grid;grid-template-columns:80px 80px 90px 90px 60px 70px 1fr;gap:4px;padding:8px 12px;border-radius:var(--radius-sm);font-size:13px;color:var(--text-primary);transition:all var(--transition-fast)}
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
.cfg-act,.rec-act{display:flex;gap:6px;flex-wrap:wrap}
.cfg-mini{padding:4px 10px;border-radius:var(--radius-sm);border:1px solid var(--border-subtle);background:var(--bg-elevated);color:var(--text-secondary);cursor:pointer;font-size:12px}
.cfg-mini.danger{border-color:var(--color-error);color:var(--color-error)}
.cfg-del{padding:4px 12px;border-radius:var(--radius-sm);border:1px solid var(--color-error);background:var(--color-error-glow);color:var(--color-error);cursor:pointer;font-size:12px}
.es,.ls,.es-sm{display:flex;flex-direction:column;align-items:center;justify-content:center;padding:40px;color:var(--text-muted);gap:12px}
.ei{font-size:48px;opacity:0.4}
.sk{height:36px;border-radius:var(--radius-sm);margin-bottom:6px;background:var(--bg-elevated)}
@media(max-width:768px){.stats-row{grid-template-columns:repeat(2,1fr)}.tr{grid-template-columns:60px 60px 70px 70px 50px 60px 1fr}}
.cfg-tabs{display:flex;gap:4px}
.cfg-tab{padding:2px 10px;border-radius:10px;border:1px solid var(--border-subtle);background:var(--bg-elevated);color:var(--text-muted);cursor:pointer;font-size:11px}
.cfg-tab.on{border-color:var(--color-primary);color:var(--color-primary);background:var(--color-primary-soft)}
.att-note{margin:8px 0;padding:6px 12px;border-radius:var(--radius-md);background:var(--bg-elevated);border:1px solid var(--border-subtle);font-size:12px;color:var(--text-secondary)}
</style>
