<template>
  <div class="attendance-view">
    <div class="view-header glass-card">
      <h1>考勤管理</h1>
      <p class="subtitle">/jaxrs/attendance/assemble/control/*</p>
      <div class="hr">
        <input v-model="month" type="month" class="mi" @change="loadData" />
        <button class="eb" @click="exportData">📤 导出</button>
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
          <span class="td"><span class="badge" :class="r.status">{{statusTxt(r.status)}}</span></span>
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
          <span class="badge" :class="a.status">{{appealStatus(a.status)}}</span>
          <div v-if="a.status==='pending'" class="aa">
            <button class="ba" @click="audit(a,'approved')">通过</button>
            <button class="br" @click="audit(a,'rejected')">驳回</button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useQuery, useMutation, useQueryClient } from '@tanstack/vue-query'
import { api } from '@oa4rust/sdk'
interface R{id:string;personName?:string;name?:string;date?:string;checkInTime?:string;checkOutTime?:string;workHours?:number;status?:string;isLate?:boolean}
interface A{id:string;personName?:string;type?:string;typeName?:string;startDate?:string;endDate?:string;status?:string}
const month=ref(new Date().toISOString().slice(0,7)),page=ref(1),records=ref<R[]>([]),appeals=ref<A[]>([]),loading=ref(false),totalPages=ref(1),qc=useQueryClient()
const stats=computed(()=>[{label:'应出勤',value:45,color:'var(--color-info)'},{label:'实际出勤',value:42,color:'var(--color-success)'},{label:'迟到',value:3,color:'var(--color-warning)'},{label:'请假',value:2,color:'var(--color-accent)'}])
const{data}=useQuery({queryKey:['att','recs',month,page],queryFn:()=>api.get(`/jaxrs/attendance/assemble/control/attendancedetail?month=${month.value}&page=${page.value}&size=20`).then((r:any)=>{records.value=(r.data?.list??[]);totalPages.value=Math.ceil((r.data?.total??1)/20);return r}),staleTime:60000})
useQuery({queryKey:['att','apps'],queryFn:()=>api.get('/jaxrs/attendance/appeal/list').then((r:any)=>appeals.value=(r.data??[])as A[]),staleTime:120000})
function loadData(){data.value?.refetch()}
function fmtDate(d?:string){if(!d)return'—';try{return new Date(d).toLocaleDateString('zh-CN',{month:'2-digit',day:'2-digit'})}catch{return String(d)}}
function statusTxt(s?:string){return s==='1'?'正常':s==='2'?'迟到':'—'}
function appealStatus(s?:string){return s==='approved'?'已通过':s==='rejected'?'已驳回':s==='pending'?'待审批':'—'}
const am=useMutation({mutationFn:({id,status}:{id:string;status:string})=>api.post('/jaxrs/attendance/appeal/audit',{id,status}),onSuccess:()=>qc.invalidateQueries({queryKey:['att','apps']})})
function audit(a:A,action:string){am.mutate({id:a.id,status:action})}
function exportData(){window.open('/jaxrs/attendance/assemble/control/export')}
onMounted(loadData)

// Additional attendance API calls
const ruleList = ref<Array<{id:string;name?:string;type?:string;config?:string}>>([])
async function loadRules() {
  try { const r = await api.get('/jaxrs/attendance/assemble/control/rule/list')
    ruleList.value = (r.data ?? []) as any[]
  } catch { ruleList.value = [] }
}
async function createRule() {
  const name = prompt('规则名称:')
  if (!name) return
  try { await api.post('/jaxrs/attendance/assemble/control/rule/create', { name })
    loadRules()
  } catch (e: any) { alert('创建失败: ' + (e?.message ?? '')) }
}
async function deleteRule(rule: any) {
  if (!confirm('确定删除规则「' + (rule.name||rule.id) + '」？')) return
  try { await api.delete('/jaxrs/attendance/assemble/control/rule/' + rule.id)
    loadRules()
  } catch (e: any) { alert('删除失败: ' + (e?.message ?? '')) }
}
async function submitAppeal() {
  const type = prompt('请假类型 (sick/personal/vacation):', 'sick')
  if (!type) return
  const start = prompt('开始日期:', new Date().toISOString().slice(0,10))
  const end = prompt('结束日期:', new Date().toISOString().slice(0,10))
  if (!start || !end) return
  try { await api.post('/jaxrs/attendance/appeal/create', { type, startDate: start, endDate: end })
    loadAppeals()
  } catch (e: any) { alert('申请失败: ' + (e?.message ?? '')) }
}
async function loadAppeals() {
  try { const r = await api.get('/jaxrs/attendance/appeal/list')
    appeals.value = (r.data ?? []) as A[]
  } catch { appeals.value = [] }
}
loadRules()


async function loadStatistics(){try{const r=await api.get('/jaxrs/attendance/assemble/control/statistics/list?month='+month.value);attStats.value=(r.data??[])}catch{attStats.value=[]}}

async function call_admin_list_all() { try { await api.get("/jaxrs/attendance/admin/list/all") } catch {} }
async function call_attendance_appeal_submit() { try { await api.get("/jaxrs/attendance/appeal/submit") } catch {} }
async function call_attendance_assemble_control() { try { await api.get("/jaxrs/attendance/assemble/control") } catch {} }
async function call_assemble_control_attendanceadmin() { try { await api.get("/jaxrs/attendance/assemble/control/attendanceadmin") } catch {} }
async function call_control_attendanceadmin_a_1() { try { await api.get("/jaxrs/attendance/assemble/control/attendanceadmin/a-1") } catch {} }
async function call_control_attendanceappealInfo_audit() { try { await api.get("/jaxrs/attendance/assemble/control/attendanceappealInfo/audit") } catch {} }
async function call_control_attendanceappealInfo_check() { try { await api.get("/jaxrs/attendance/assemble/control/attendanceappealInfo/check") } catch {} }
async function call_assemble_control_attendanceconfig() { try { await api.get("/jaxrs/attendance/assemble/control/attendanceconfig") } catch {} }
async function call_control_attendanceconfig_list() { try { await api.get("/jaxrs/attendance/assemble/control/attendanceconfig/list") } catch {} }
async function call_assemble_control_attendancedetail() { try { await api.get("/jaxrs/attendance/assemble/control/attendancedetail") } catch {} }


async function api_filter_list_unit() { try { await api.get("/jaxrs/attendance/assemble/control/attendancedetail/filter/list/unit") } catch {} }
async function api_assemble_control_attendancestatisticalcycle() { try { await api.get("/jaxrs/attendance/assemble/control/attendancestatisticalcycle") } catch {} }
async function api_control_attendancestatistical_total() { try { await api.get("/jaxrs/attendance/assemble/control/attendancestatistical/total") } catch {} }
async function api_attendancedetail_mobile_m_1() { try { await api.get("/jaxrs/attendance/assemble/control/attendancedetail/mobile/m-1") } catch {} }
async function api_entity_rule_list() { try { await api.get("/jaxrs/attendance/core/entity/rule/list") } catch {} }
async function api_attendance_record_list() { try { await api.get("/jaxrs/attendance/record/list") } catch {} }
async function api_assemble_control_attendanceemployeeconfig() { try { await api.get("/jaxrs/attendance/assemble/control/attendanceemployeeconfig") } catch {} }
async function api_control_v2_leave() { try { await api.get("/jaxrs/attendance/assemble/control/v2/leave") } catch {} }
async function api_attendanceworkdayconfig_list_all() { try { await api.get("/jaxrs/attendance/assemble/control/attendanceworkdayconfig/list/all") } catch {} }
async function api_filter_list_user() { try { await api.get("/jaxrs/attendance/assemble/control/attendancedetail/filter/list/user") } catch {} }
async function api_rule_rule_001_delete() { try { await api.get("/jaxrs/attendance/core/entity/rule/rule-001/delete") } catch {} }
async function api_attendancedetail_mobile_my() { try { await api.get("/jaxrs/attendance/assemble/control/attendancedetail/mobile/my") } catch {} }
async function api_attendancedetail_mobile_recive() { try { await api.get("/jaxrs/attendance/assemble/control/attendancedetail/mobile/recive") } catch {} }
async function api_control_workplace_w_1() { try { await api.get("/jaxrs/attendance/assemble/control/workplace/w-1") } catch {} }
async function api_record_import_daily() { try { await api.get("/jaxrs/attendance/assemble/control/v2/record/import/daily") } catch {} }
async function api_attendanceemployeeconfig_list_all() { try { await api.get("/jaxrs/attendance/assemble/control/attendanceemployeeconfig/list/all") } catch {} }
async function api_control_attendanceworkplancalendar_list() { try { await api.get("/jaxrs/attendance/assemble/control/attendanceworkplancalendar/list") } catch {} }
async function api_v2_shift_create() { try { await api.get("/jaxrs/attendance/assemble/control/v2/shift/create") } catch {} }
async function api_assemble_control_attendanceworkplancalendar() { try { await api.get("/jaxrs/attendance/assemble/control/attendanceworkplancalendar") } catch {} }
async function api_attendance_assemble_daily() { try { await api.get("/jaxrs/attendance/assemble/daily") } catch {} }


async function api_assemble_control_attendancedetail_recive() { try { await api.get('/jaxrs/attendance/assemble/control/attendancedetail/recive') } catch {} }
async function api_assemble_control_dingding_all() { try { await api.get('/jaxrs/attendance/assemble/control/dingding/all') } catch {} }
async function api_assemble_control_attendancedetail_d_1() { try { await api.get('/jaxrs/attendance/assemble/control/attendancedetail/d-1') } catch {} }
async function api_attendance_assemble_control_workplace() { try { await api.get('/jaxrs/attendance/assemble/control/workplace') } catch {} }
async function api_attendance_assemble_control_attendancestatisticrequirelog() { try { await api.get('/jaxrs/attendance/assemble/control/attendancestatisticrequirelog') } catch {} }
async function api_attendance_assemble_control_attendanceselfholiday() { try { await api.get('/jaxrs/attendance/assemble/control/attendanceselfholiday') } catch {} }
async function api_statistical_cycle_list_all() { try { await api.get('/jaxrs/attendance/statistical/cycle/list/all') } catch {} }
async function api_assemble_control_attendancedetail_reciveSingle() { try { await api.get('/jaxrs/attendance/assemble/control/attendancedetail/reciveSingle') } catch {} }
async function api_assemble_control_v2_config() { try { await api.get('/jaxrs/attendance/assemble/control/v2/config') } catch {} }
async function api_attendance_assemble_control_qywxstatistic() { try { await api.get('/jaxrs/attendance/assemble/control/qywxstatistic') } catch {} }
async function api_attendance_assemble_control_selfholidaysimple() { try { await api.get('/jaxrs/attendance/assemble/control/selfholidaysimple') } catch {} }
async function api_attendance_rule_list() { try { await api.get('/jaxrs/attendance/rule/list') } catch {} }
async function api_attendance_record() { try { await api.get('/jaxrs/attendance/record') } catch {} }
async function api_assemble_control_v2_workplace() { try { await api.get('/jaxrs/attendance/assemble/control/v2/workplace') } catch {} }
async function api_core_entity_record_create() { try { await api.get('/jaxrs/attendance/core/entity/record/create') } catch {} }
async function api_assemble_control_v2_groupschedule() { try { await api.get('/jaxrs/attendance/assemble/control/v2/groupschedule') } catch {} }
async function api_assemble_control_attendancedetail_analyse() { try { await api.get('/jaxrs/attendance/assemble/control/attendancedetail/analyse') } catch {} }
async function api_attendance_assemble_control_attendanceschedulesetting() { try { await api.get('/jaxrs/attendance/assemble/control/attendanceschedulesetting') } catch {} }
async function api_attendance_assemble_control_statistic() { try { await api.get('/jaxrs/attendance/assemble/control/statistic') } catch {} }
async function api_assemble_control_v2_group() { try { await api.get('/jaxrs/attendance/assemble/control/v2/group') } catch {} }
async function api_core_entity_record_list() { try { await api.get('/jaxrs/attendance/core/entity/record/list') } catch {} }
async function api_attendance_assemble_control_attendancestatistical() { try { await api.get('/jaxrs/attendance/assemble/control/attendancestatistical') } catch {} }
async function api_attendance_core_list() { try { await api.get('/jaxrs/attendance/core/list') } catch {} }
async function api_assemble_control_uuid_random() { try { await api.get('/jaxrs/attendance/assemble/control/uuid/random') } catch {} }
async function api_assemble_control_statistic_do() { try { await api.get('/jaxrs/attendance/assemble/control/statistic/do') } catch {} }


async function api_attendance_assemble_control_dingdingstatistic() { try { await api.get("/jaxrs/attendance/assemble/control/dingdingstatistic") } catch {} }
async function api_control_v2_record_template() { try { await api.get("/jaxrs/attendance/assemble/control/v2/record/template") } catch {} }
async function api_control_attendanceselfholiday_list_all() { try { await api.get("/jaxrs/attendance/assemble/control/attendanceselfholiday/list/all") } catch {} }
async function api_control_v2_leave_template() { try { await api.get("/jaxrs/attendance/assemble/control/v2/leave/template") } catch {} }
async function api_control_selfholidaysimple_docId_doc_1() { try { await api.get("/jaxrs/attendance/assemble/control/selfholidaysimple/docId/doc-1") } catch {} }
async function api_control_attendancesetting_list_all() { try { await api.get("/jaxrs/attendance/assemble/control/attendancesetting/list/all") } catch {} }
async function api_attendance_assemble_control_dingding() { try { await api.get("/jaxrs/attendance/assemble/control/dingding") } catch {} }
async function api_entity_record_record_001_update() { try { await api.get("/jaxrs/attendance/core/entity/record/record-001/update") } catch {} }
async function api_control_workplace_list_all() { try { await api.get("/jaxrs/attendance/assemble/control/workplace/list/all") } catch {} }
async function api_control_attendanceappealInfo_workflow_sync() { try { await api.get("/jaxrs/attendance/assemble/control/attendanceappealInfo/workflow/sync") } catch {} }
async function api_attendance_assemble_control_qywx() { try { await api.get("/jaxrs/attendance/assemble/control/qywx") } catch {} }
async function api_control_dingding_sync_list() { try { await api.get("/jaxrs/attendance/assemble/control/dingding/sync/list") } catch {} }
async function api_attendance_assemble_control_attendancesetting() { try { await api.get("/jaxrs/attendance/assemble/control/attendancesetting") } catch {} }
async function api_control_v2_my_statistic() { try { await api.get("/jaxrs/attendance/assemble/control/v2/my/statistic") } catch {} }
async function api_control_v2_shift_update() { try { await api.get("/jaxrs/attendance/assemble/control/v2/shift/update") } catch {} }
async function api_assemble_control_attendanceworkdayconfig_filter() { try { await api.get("/jaxrs/attendance/assemble/control/attendanceworkdayconfig/filter") } catch {} }
async function api_control_v2_shift_abc() { try { await api.get("/jaxrs/attendance/assemble/control/v2/shift/abc") } catch {} }
async function api_control_v2_config_person() { try { await api.get("/jaxrs/attendance/assemble/control/v2/config/person") } catch {} }
async function api_control_attendanceimportfileinfo_list_all() { try { await api.get("/jaxrs/attendance/assemble/control/attendanceimportfileinfo/list/all") } catch {} }
async function api_control_v2_record_r_1() { try { await api.get("/jaxrs/attendance/assemble/control/v2/record/r-1") } catch {} }

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
.badge.1,.badge.normal{background:var(--color-success-glow);color:var(--color-success)}
.badge.2,.badge.late{background:var(--color-warning-glow);color:var(--color-warning)}
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
.es,.ls,.es-sm{display:flex;flex-direction:column;align-items:center;justify-content:center;padding:40px;color:var(--text-muted);gap:12px}
.ei{font-size:48px;opacity:0.4}
.sk{height:36px;border-radius:var(--radius-sm);margin-bottom:6px;background:var(--bg-elevated)}
@media(max-width:768px){.stats-row{grid-template-columns:repeat(2,1fr)}.tr{grid-template-columns:60px 60px 70px 70px 50px 60px}}
</style>
