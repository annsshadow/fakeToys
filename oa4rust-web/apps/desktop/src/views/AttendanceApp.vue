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
import { toast } from '../utils/toast'
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
  } catch (e: any) { toast.error('创建失败: ' + (e?.message ?? '')) }
}
async function deleteRule(rule: any) {
  if (!confirmMsg('确定删除规则「' + (rule.name||rule.id) + '」？')) return
  try { await api.delete('/jaxrs/attendance/assemble/control/rule/' + rule.id)
    loadRules()
  } catch (e: any) { toast.error('删除失败: : ' + (e?.message ?? '')) }
}
async function submitAppeal() {
  const type = prompt('请假类型 (sick/personal/vacation):', 'sick')
  if (!type) return
  const start = prompt('开始日期:', new Date().toISOString().slice(0,10))
  const end = prompt('结束日期:', new Date().toISOString().slice(0,10))
  if (!start || !end) return
  try { await api.post('/jaxrs/attendance/appeal/create', { type, startDate: start, endDate: end })
    loadAppeals()
  } catch (e: any) { toast.error('申请失败: : ' + (e?.message ?? '')) }
}
async function loadAppeals() {
  try { const r = await api.get('/jaxrs/attendance/appeal/list')
    appeals.value = (r.data ?? []) as A[]
  } catch { appeals.value = [] }
}
loadRules()

async function loadStatistics(){try{const r=await api.get('/jaxrs/attendance/assemble/control/statistics/list?month='+month.value);attStats.value=(r.data??[])}catch{attStats.value=[]}}

const assemble_control_attendancedetail_recive_ref = ref<any[]>([]);
const assemble_control_attendancedetail_recive_q = useQuery({
  queryKey: ['assemble_control_attendancedetail_recive'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/attendance/assemble/control/attendancedetail/recive"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const assemble_control_dingding_all_ref = ref<any[]>([]);
const assemble_control_dingding_all_q = useQuery({
  queryKey: ['assemble_control_dingding_all'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/attendance/assemble/control/dingding/all"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const assemble_control_attendancedetail_d_1_ref = ref<any[]>([]);
const assemble_control_attendancedetail_d_1_q = useQuery({
  queryKey: ['assemble_control_attendancedetail_d_1'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/attendance/assemble/control/attendancedetail/d-1"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const attendance_assemble_control_workplace_ref = ref<any[]>([]);
const attendance_assemble_control_workplace_q = useQuery({
  queryKey: ['attendance_assemble_control_workplace'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/attendance/assemble/control/workplace"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const attendance_assemble_control_attendancestatisticrequirelog_ref = ref<any[]>([]);
const attendance_assemble_control_attendancestatisticrequirelog_q = useQuery({
  queryKey: ['attendance_assemble_control_attendancestatisticrequirelog'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/attendance/assemble/control/attendancestatisticrequirelog"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const attendance_assemble_control_attendanceselfholiday_ref = ref<any[]>([]);
const attendance_assemble_control_attendanceselfholiday_q = useQuery({
  queryKey: ['attendance_assemble_control_attendanceselfholiday'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/attendance/assemble/control/attendanceselfholiday"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const statistical_cycle_list_all_ref = ref<any[]>([]);
const statistical_cycle_list_all_q = useQuery({
  queryKey: ['statistical_cycle_list_all'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/attendance/statistical/cycle/list/all"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const assemble_control_attendancedetail_reciveSingle_ref = ref<any[]>([]);
const assemble_control_attendancedetail_reciveSingle_q = useQuery({
  queryKey: ['assemble_control_attendancedetail_reciveSingle'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/attendance/assemble/control/attendancedetail/reciveSingle"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const assemble_control_v2_config_ref = ref<any[]>([]);
const assemble_control_v2_config_q = useQuery({
  queryKey: ['assemble_control_v2_config'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/attendance/assemble/control/v2/config"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const attendance_assemble_control_qywxstatistic_ref = ref<any[]>([]);
const attendance_assemble_control_qywxstatistic_q = useQuery({
  queryKey: ['attendance_assemble_control_qywxstatistic'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/attendance/assemble/control/qywxstatistic"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const attendance_assemble_control_selfholidaysimple_ref = ref<any[]>([]);
const attendance_assemble_control_selfholidaysimple_q = useQuery({
  queryKey: ['attendance_assemble_control_selfholidaysimple'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/attendance/assemble/control/selfholidaysimple"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const attendance_rule_list_ref = ref<any[]>([]);
const attendance_rule_list_q = useQuery({
  queryKey: ['attendance_rule_list'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/attendance/rule/list"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const attendance_record_ref = ref<any[]>([]);
const attendance_record_q = useQuery({
  queryKey: ['attendance_record'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/attendance/record"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const assemble_control_v2_workplace_ref = ref<any[]>([]);
const assemble_control_v2_workplace_q = useQuery({
  queryKey: ['assemble_control_v2_workplace'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/attendance/assemble/control/v2/workplace"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const core_entity_record_create_ref = ref<any[]>([]);
const core_entity_record_create_q = useQuery({
  queryKey: ['core_entity_record_create'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/attendance/core/entity/record/create"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const assemble_control_v2_groupschedule_ref = ref<any[]>([]);
const assemble_control_v2_groupschedule_q = useQuery({
  queryKey: ['assemble_control_v2_groupschedule'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/attendance/assemble/control/v2/groupschedule"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const assemble_control_attendancedetail_analyse_ref = ref<any[]>([]);
const assemble_control_attendancedetail_analyse_q = useQuery({
  queryKey: ['assemble_control_attendancedetail_analyse'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/attendance/assemble/control/attendancedetail/analyse"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const attendance_assemble_control_attendanceschedulesetting_ref = ref<any[]>([]);
const attendance_assemble_control_attendanceschedulesetting_q = useQuery({
  queryKey: ['attendance_assemble_control_attendanceschedulesetting'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/attendance/assemble/control/attendanceschedulesetting"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const attendance_assemble_control_statistic_ref = ref<any[]>([]);
const attendance_assemble_control_statistic_q = useQuery({
  queryKey: ['attendance_assemble_control_statistic'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/attendance/assemble/control/statistic"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const assemble_control_v2_group_ref = ref<any[]>([]);
const assemble_control_v2_group_q = useQuery({
  queryKey: ['assemble_control_v2_group'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/attendance/assemble/control/v2/group"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const core_entity_record_list_ref = ref<any[]>([]);
const core_entity_record_list_q = useQuery({
  queryKey: ['core_entity_record_list'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/attendance/core/entity/record/list"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const attendance_assemble_control_attendancestatistical_ref = ref<any[]>([]);
const attendance_assemble_control_attendancestatistical_q = useQuery({
  queryKey: ['attendance_assemble_control_attendancestatistical'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/attendance/assemble/control/attendancestatistical"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const attendance_core_list_ref = ref<any[]>([]);
const attendance_core_list_q = useQuery({
  queryKey: ['attendance_core_list'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/attendance/core/list"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const assemble_control_uuid_random_ref = ref<any[]>([]);
const assemble_control_uuid_random_q = useQuery({
  queryKey: ['assemble_control_uuid_random'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/attendance/assemble/control/uuid/random"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const assemble_control_statistic_do_ref = ref<any[]>([]);
const assemble_control_statistic_do_q = useQuery({
  queryKey: ['assemble_control_statistic_do'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/attendance/assemble/control/statistic/do"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});

// Confirmation dialog (replaces window.confirm)
function confirmMsg(msg: string): Promise<boolean> {
  return new Promise(resolve => {
    const overlay = document.createElement('div')
    overlay.style.cssText = 'position:fixed;inset:0;background:rgba(0,0,0,.6);z-index:10000;display:flex;align-items:center;justify-content:center'
    const box = document.createElement('div')
    box.style.cssText = 'background:var(--bg-surface);border:1px solid var(--border-color);border-radius:var(--radius-lg);padding:24px;max-width:360px;width:90%;display:flex;flex-direction:column;gap:16px'
    box.innerHTML = '<p style="margin:0;color:var(--text-primary);font-size:14px">' + msg + '</p>' +
      '<div style="display:flex;gap:8px;justify-content:flex-end">' +
      '<button class="tc-cancel" style="padding:6px 16px;border-radius:var(--radius-md);border:1px solid var(--border-color);background:transparent;color:var(--text-primary);cursor:pointer">取消</button>' +
      '<button class="tc-ok" style="padding:6px 16px;border-radius:var(--radius-md);border:none;background:var(--color-primary);color:#000;cursor:pointer;font-weight:600">确认</button>' +
      '</div>'
    overlay.appendChild(box)
    document.body.appendChild(overlay)
    const ok = () => { overlay.remove(); resolve(true) }
    const cancel = () => { overlay.remove(); resolve(false) }
    box.querySelector('.tc-ok').addEventListener('click', ok)
    box.querySelector('.tc-cancel').addEventListener('click', cancel)
    overlay.addEventListener('click', e => { if (e.target === overlay) cancel() })
  })
}

const jaxrs_attendance_admin_list_all_ref = ref<any[]>([]);
const jaxrs_attendance_admin_list_all_q = useQuery({
  queryKey: ['jaxrs_attendance_admin_list_all'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/attendance/admin/list/all"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const jaxrs_attendance_appeal_submit_ref = ref<any[]>([]);
const jaxrs_attendance_appeal_submit_q = useQuery({
  queryKey: ['jaxrs_attendance_appeal_submit'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/attendance/appeal/submit"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const jaxrs_attendance_assemble_control_ref = ref<any[]>([]);
const jaxrs_attendance_assemble_control_q = useQuery({
  queryKey: ['jaxrs_attendance_assemble_control'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/attendance/assemble/control"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const jaxrs_attendance_assemble_control_attendanceadmin_ref = ref<any[]>([]);
const jaxrs_attendance_assemble_control_attendanceadmin_q = useQuery({
  queryKey: ['jaxrs_attendance_assemble_control_attendanceadmin'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/attendance/assemble/control/attendanceadmin"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const jaxrs_attendance_assemble_control_attendanceadmin_a_1_ref = ref<any[]>([]);
const jaxrs_attendance_assemble_control_attendanceadmin_a_1_q = useQuery({
  queryKey: ['jaxrs_attendance_assemble_control_attendanceadmin_a_1'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/attendance/assemble/control/attendanceadmin/a-1"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const jaxrs_attendance_assemble_control_attendanceadmin_list_all_ref = ref<any[]>([]);
const jaxrs_attendance_assemble_control_attendanceadmin_list_all_q = useQuery({
  queryKey: ['jaxrs_attendance_assemble_control_attendanceadmin_list_all'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/attendance/assemble/control/attendanceadmin/list/all"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const jaxrs_attendance_assemble_control_attendanceappealInfo_audit_ref = ref<any[]>([]);
const jaxrs_attendance_assemble_control_attendanceappealInfo_audit_q = useQuery({
  queryKey: ['jaxrs_attendance_assemble_control_attendanceappealInfo_audit'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/attendance/assemble/control/attendanceappealInfo/audit"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const jaxrs_attendance_assemble_control_attendanceappealInfo_check_ref = ref<any[]>([]);
const jaxrs_attendance_assemble_control_attendanceappealInfo_check_q = useQuery({
  queryKey: ['jaxrs_attendance_assemble_control_attendanceappealInfo_check'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/attendance/assemble/control/attendanceappealInfo/check"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const jaxrs_attendance_assemble_control_attendanceappealInfo_workflow_sync_ref = ref<any[]>([]);
const jaxrs_attendance_assemble_control_attendanceappealInfo_workflow_sync_q = useQuery({
  queryKey: ['jaxrs_attendance_assemble_control_attendanceappealInfo_workflow_sync'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/attendance/assemble/control/attendanceappealInfo/workflow/sync"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const jaxrs_attendance_assemble_control_attendanceconfig_ref = ref<any[]>([]);
const jaxrs_attendance_assemble_control_attendanceconfig_q = useQuery({
  queryKey: ['jaxrs_attendance_assemble_control_attendanceconfig'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/attendance/assemble/control/attendanceconfig"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const jaxrs_attendance_assemble_control_attendanceconfig_list_ref = ref<any[]>([]);
const jaxrs_attendance_assemble_control_attendanceconfig_list_q = useQuery({
  queryKey: ['jaxrs_attendance_assemble_control_attendanceconfig_list'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/attendance/assemble/control/attendanceconfig/list"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const jaxrs_attendance_assemble_control_attendancedetail_ref = ref<any[]>([]);
const jaxrs_attendance_assemble_control_attendancedetail_q = useQuery({
  queryKey: ['jaxrs_attendance_assemble_control_attendancedetail'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/attendance/assemble/control/attendancedetail"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const jaxrs_attendance_assemble_control_attendancedetail_analyse_redo_ref = ref<any[]>([]);
const jaxrs_attendance_assemble_control_attendancedetail_analyse_redo_q = useQuery({
  queryKey: ['jaxrs_attendance_assemble_control_attendancedetail_analyse_redo'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/attendance/assemble/control/attendancedetail/analyse/redo"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const jaxrs_attendance_assemble_control_attendancedetail_filter_list_ref = ref<any[]>([]);
const jaxrs_attendance_assemble_control_attendancedetail_filter_list_q = useQuery({
  queryKey: ['jaxrs_attendance_assemble_control_attendancedetail_filter_list'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/attendance/assemble/control/attendancedetail/filter/list"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const jaxrs_attendance_assemble_control_attendancedetail_filter_list_topUnit_ref = ref<any[]>([]);
const jaxrs_attendance_assemble_control_attendancedetail_filter_list_topUnit_q = useQuery({
  queryKey: ['jaxrs_attendance_assemble_control_attendancedetail_filter_list_topUnit'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/attendance/assemble/control/attendancedetail/filter/list/topUnit"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const jaxrs_attendance_assemble_control_attendancedetail_filter_list_unit_ref = ref<any[]>([]);
const jaxrs_attendance_assemble_control_attendancedetail_filter_list_unit_q = useQuery({
  queryKey: ['jaxrs_attendance_assemble_control_attendancedetail_filter_list_unit'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/attendance/assemble/control/attendancedetail/filter/list/unit"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const jaxrs_attendance_assemble_control_attendancedetail_filter_list_user_ref = ref<any[]>([]);
const jaxrs_attendance_assemble_control_attendancedetail_filter_list_user_q = useQuery({
  queryKey: ['jaxrs_attendance_assemble_control_attendancedetail_filter_list_user'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/attendance/assemble/control/attendancedetail/filter/list/user"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const jaxrs_attendance_assemble_control_attendancedetail_list_persons_nonesign_ref = ref<any[]>([]);
const jaxrs_attendance_assemble_control_attendancedetail_list_persons_nonesign_q = useQuery({
  queryKey: ['jaxrs_attendance_assemble_control_attendancedetail_list_persons_nonesign'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/attendance/assemble/control/attendancedetail/list/persons/nonesign"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const jaxrs_attendance_assemble_control_attendancedetail_mobile_m_1_ref = ref<any[]>([]);
const jaxrs_attendance_assemble_control_attendancedetail_mobile_m_1_q = useQuery({
  queryKey: ['jaxrs_attendance_assemble_control_attendancedetail_mobile_m_1'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/attendance/assemble/control/attendancedetail/mobile/m-1"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const jaxrs_attendance_assemble_control_attendancedetail_mobile_mobilepreview_ref = ref<any[]>([]);
const jaxrs_attendance_assemble_control_attendancedetail_mobile_mobilepreview_q = useQuery({
  queryKey: ['jaxrs_attendance_assemble_control_attendancedetail_mobile_mobilepreview'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/attendance/assemble/control/attendancedetail/mobile/mobilepreview"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const jaxrs_attendance_assemble_control_attendancedetail_mobile_my_ref = ref<any[]>([]);
const jaxrs_attendance_assemble_control_attendancedetail_mobile_my_q = useQuery({
  queryKey: ['jaxrs_attendance_assemble_control_attendancedetail_mobile_my'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/attendance/assemble/control/attendancedetail/mobile/my"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const jaxrs_attendance_assemble_control_attendancedetail_mobile_recive_ref = ref<any[]>([]);
const jaxrs_attendance_assemble_control_attendancedetail_mobile_recive_q = useQuery({
  queryKey: ['jaxrs_attendance_assemble_control_attendancedetail_mobile_recive'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/attendance/assemble/control/attendancedetail/mobile/recive"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const jaxrs_attendance_assemble_control_attendanceemployeeconfig_ref = ref<any[]>([]);
const jaxrs_attendance_assemble_control_attendanceemployeeconfig_q = useQuery({
  queryKey: ['jaxrs_attendance_assemble_control_attendanceemployeeconfig'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/attendance/assemble/control/attendanceemployeeconfig"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const jaxrs_attendance_assemble_control_attendanceemployeeconfig_list_all_ref = ref<any[]>([]);
const jaxrs_attendance_assemble_control_attendanceemployeeconfig_list_all_q = useQuery({
  queryKey: ['jaxrs_attendance_assemble_control_attendanceemployeeconfig_list_all'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/attendance/assemble/control/attendanceemployeeconfig/list/all"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const jaxrs_attendance_assemble_control_attendanceimportfileinfo_list_all_ref = ref<any[]>([]);
const jaxrs_attendance_assemble_control_attendanceimportfileinfo_list_all_q = useQuery({
  queryKey: ['jaxrs_attendance_assemble_control_attendanceimportfileinfo_list_all'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/attendance/assemble/control/attendanceimportfileinfo/list/all"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const api_jaxrs_at_71_data = ref<any[]>([]);
const { data: api_jaxrs_at_71_q } = useQuery({queryKey: ['api_jaxrs_at_71', '/jaxrs/attendance/assemble/control/attendanceschedulesetting/list/all'], queryFn: async () => { try { const r = await api.get("/jaxrs/attendance/assemble/control/attendanceschedulesetting/list/all"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_at_71_q, (v) => { api_jaxrs_at_71_data.value = v ?? []; });
const api_jaxrs_at_298_data = ref<any[]>([]);
const { data: api_jaxrs_at_298_q } = useQuery({queryKey: ['api_jaxrs_at_298', '/jaxrs/attendance/assemble/control/attendanceselfholiday/list/all'], queryFn: async () => { try { const r = await api.get("/jaxrs/attendance/assemble/control/attendanceselfholiday/list/all"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_at_298_q, (v) => { api_jaxrs_at_298_data.value = v ?? []; });
const api_jaxrs_at_857_data = ref<any[]>([]);
const { data: api_jaxrs_at_857_q } = useQuery({queryKey: ['api_jaxrs_at_857', '/jaxrs/attendance/assemble/control/attendancesetting'], queryFn: async () => { try { const r = await api.get("/jaxrs/attendance/assemble/control/attendancesetting"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_at_857_q, (v) => { api_jaxrs_at_857_data.value = v ?? []; });
const api_jaxrs_at_962_data = ref<any[]>([]);
const { data: api_jaxrs_at_962_q } = useQuery({queryKey: ['api_jaxrs_at_962', '/jaxrs/attendance/assemble/control/attendancesetting/enable/type'], queryFn: async () => { try { const r = await api.get("/jaxrs/attendance/assemble/control/attendancesetting/enable/type"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_at_962_q, (v) => { api_jaxrs_at_962_data.value = v ?? []; });
const api_jaxrs_at_991_data = ref<any[]>([]);
const { data: api_jaxrs_at_991_q } = useQuery({queryKey: ['api_jaxrs_at_991', '/jaxrs/attendance/assemble/control/attendancesetting/list/all'], queryFn: async () => { try { const r = await api.get("/jaxrs/attendance/assemble/control/attendancesetting/list/all"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_at_991_q, (v) => { api_jaxrs_at_991_data.value = v ?? []; });
const api_jaxrs_at_906_data = ref<any[]>([]);
const { data: api_jaxrs_at_906_q } = useQuery({queryKey: ['api_jaxrs_at_906', '/jaxrs/attendance/assemble/control/attendancestatistical/total'], queryFn: async () => { try { const r = await api.get("/jaxrs/attendance/assemble/control/attendancestatistical/total"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_at_906_q, (v) => { api_jaxrs_at_906_data.value = v ?? []; });
const api_jaxrs_at_844_data = ref<any[]>([]);
const { data: api_jaxrs_at_844_q } = useQuery({queryKey: ['api_jaxrs_at_844', '/jaxrs/attendance/assemble/control/attendancestatisticalcycle'], queryFn: async () => { try { const r = await api.get("/jaxrs/attendance/assemble/control/attendancestatisticalcycle"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_at_844_q, (v) => { api_jaxrs_at_844_data.value = v ?? []; });
const api_jaxrs_at_47_data = ref<any[]>([]);
const { data: api_jaxrs_at_47_q } = useQuery({queryKey: ['api_jaxrs_at_47', '/jaxrs/attendance/assemble/control/attendancestatisticalcycle/list/all'], queryFn: async () => { try { const r = await api.get("/jaxrs/attendance/assemble/control/attendancestatisticalcycle/list/all"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_at_47_q, (v) => { api_jaxrs_at_47_data.value = v ?? []; });
const api_jaxrs_at_106_data = ref<any[]>([]);
const { data: api_jaxrs_at_106_q } = useQuery({queryKey: ['api_jaxrs_at_106', '/jaxrs/attendance/assemble/control/attendancestatisticrequirelog/list/all'], queryFn: async () => { try { const r = await api.get("/jaxrs/attendance/assemble/control/attendancestatisticrequirelog/list/all"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_at_106_q, (v) => { api_jaxrs_at_106_data.value = v ?? []; });
const api_jaxrs_at_986_data = ref<any[]>([]);
const { data: api_jaxrs_at_986_q } = useQuery({queryKey: ['api_jaxrs_at_986', '/jaxrs/attendance/assemble/control/attendanceworkdayconfig'], queryFn: async () => { try { const r = await api.get("/jaxrs/attendance/assemble/control/attendanceworkdayconfig"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_at_986_q, (v) => { api_jaxrs_at_986_data.value = v ?? []; });
const api_jaxrs_at_832_data = ref<any[]>([]);
const { data: api_jaxrs_at_832_q } = useQuery({queryKey: ['api_jaxrs_at_832', '/jaxrs/attendance/assemble/control/attendanceworkdayconfig/filter'], queryFn: async () => { try { const r = await api.get("/jaxrs/attendance/assemble/control/attendanceworkdayconfig/filter"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_at_832_q, (v) => { api_jaxrs_at_832_data.value = v ?? []; });
const api_jaxrs_at_129_data = ref<any[]>([]);
const { data: api_jaxrs_at_129_q } = useQuery({queryKey: ['api_jaxrs_at_129', '/jaxrs/attendance/assemble/control/attendanceworkdayconfig/list/all'], queryFn: async () => { try { const r = await api.get("/jaxrs/attendance/assemble/control/attendanceworkdayconfig/list/all"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_at_129_q, (v) => { api_jaxrs_at_129_data.value = v ?? []; });
const api_jaxrs_at_773_data = ref<any[]>([]);
const { data: api_jaxrs_at_773_q } = useQuery({queryKey: ['api_jaxrs_at_773', '/jaxrs/attendance/assemble/control/attendanceworkplancalendar'], queryFn: async () => { try { const r = await api.get("/jaxrs/attendance/assemble/control/attendanceworkplancalendar"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_at_773_q, (v) => { api_jaxrs_at_773_data.value = v ?? []; });
const api_jaxrs_at_282_data = ref<any[]>([]);
const { data: api_jaxrs_at_282_q } = useQuery({queryKey: ['api_jaxrs_at_282', '/jaxrs/attendance/assemble/control/attendanceworkplancalendar/list'], queryFn: async () => { try { const r = await api.get("/jaxrs/attendance/assemble/control/attendanceworkplancalendar/list"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_at_282_q, (v) => { api_jaxrs_at_282_data.value = v ?? []; });
const api_jaxrs_at_500_data = ref<any[]>([]);
const { data: api_jaxrs_at_500_q } = useQuery({queryKey: ['api_jaxrs_at_500', '/jaxrs/attendance/assemble/control/dingding'], queryFn: async () => { try { const r = await api.get("/jaxrs/attendance/assemble/control/dingding"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_at_500_q, (v) => { api_jaxrs_at_500_data.value = v ?? []; });
const api_jaxrs_at_997_data = ref<any[]>([]);
const { data: api_jaxrs_at_997_q } = useQuery({queryKey: ['api_jaxrs_at_997', '/jaxrs/attendance/assemble/control/dingding/sync/list'], queryFn: async () => { try { const r = await api.get("/jaxrs/attendance/assemble/control/dingding/sync/list"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_at_997_q, (v) => { api_jaxrs_at_997_data.value = v ?? []; });
const api_jaxrs_at_516_data = ref<any[]>([]);
const { data: api_jaxrs_at_516_q } = useQuery({queryKey: ['api_jaxrs_at_516', '/jaxrs/attendance/assemble/control/dingdingstatistic'], queryFn: async () => { try { const r = await api.get("/jaxrs/attendance/assemble/control/dingdingstatistic"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_at_516_q, (v) => { api_jaxrs_at_516_data.value = v ?? []; });
const api_jaxrs_at_395_data = ref<any[]>([]);
const { data: api_jaxrs_at_395_q } = useQuery({queryKey: ['api_jaxrs_at_395', '/jaxrs/attendance/assemble/control/qywx'], queryFn: async () => { try { const r = await api.get("/jaxrs/attendance/assemble/control/qywx"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_at_395_q, (v) => { api_jaxrs_at_395_data.value = v ?? []; });
const api_jaxrs_at_760_data = ref<any[]>([]);
const { data: api_jaxrs_at_760_q } = useQuery({queryKey: ['api_jaxrs_at_760', '/jaxrs/attendance/assemble/control/qywx/all'], queryFn: async () => { try { const r = await api.get("/jaxrs/attendance/assemble/control/qywx/all"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_at_760_q, (v) => { api_jaxrs_at_760_data.value = v ?? []; });
const api_jaxrs_at_858_data = ref<any[]>([]);
const { data: api_jaxrs_at_858_q } = useQuery({queryKey: ['api_jaxrs_at_858', '/jaxrs/attendance/assemble/control/qywx/sync/list'], queryFn: async () => { try { const r = await api.get("/jaxrs/attendance/assemble/control/qywx/sync/list"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_at_858_q, (v) => { api_jaxrs_at_858_data.value = v ?? []; });
const api_jaxrs_at_684_data = ref<any[]>([]);
const { data: api_jaxrs_at_684_q } = useQuery({queryKey: ['api_jaxrs_at_684', '/jaxrs/attendance/assemble/control/rule/r-1/toggle'], queryFn: async () => { try { const r = await api.get("/jaxrs/attendance/assemble/control/rule/r-1/toggle"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_at_684_q, (v) => { api_jaxrs_at_684_data.value = v ?? []; });
const api_jaxrs_at_297_data = ref<any[]>([]);
const { data: api_jaxrs_at_297_q } = useQuery({queryKey: ['api_jaxrs_at_297', '/jaxrs/attendance/assemble/control/selfholidaysimple/docId/doc-1'], queryFn: async () => { try { const r = await api.get("/jaxrs/attendance/assemble/control/selfholidaysimple/docId/doc-1"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_at_297_q, (v) => { api_jaxrs_at_297_data.value = v ?? []; });
const api_jaxrs_at_175_data = ref<any[]>([]);
const { data: api_jaxrs_at_175_q } = useQuery({queryKey: ['api_jaxrs_at_175', '/jaxrs/attendance/assemble/control/statisticshow/filter/unitDay/list/x/next/5'], queryFn: async () => { try { const r = await api.get("/jaxrs/attendance/assemble/control/statisticshow/filter/unitDay/list/x/next/5"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_at_175_q, (v) => { api_jaxrs_at_175_data.value = v ?? []; });
const api_jaxrs_at_519_data = ref<any[]>([]);
const { data: api_jaxrs_at_519_q } = useQuery({queryKey: ['api_jaxrs_at_519', '/jaxrs/attendance/assemble/control/v2/config/person'], queryFn: async () => { try { const r = await api.get("/jaxrs/attendance/assemble/control/v2/config/person"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_at_519_q, (v) => { api_jaxrs_at_519_data.value = v ?? []; });
const api_jaxrs_at_460_data = ref<any[]>([]);
const { data: api_jaxrs_at_460_q } = useQuery({queryKey: ['api_jaxrs_at_460', '/jaxrs/attendance/assemble/control/v2/detail/list/1/size/50'], queryFn: async () => { try { const r = await api.get("/jaxrs/attendance/assemble/control/v2/detail/list/1/size/50"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_at_460_q, (v) => { api_jaxrs_at_460_data.value = v ?? []; });
const api_jaxrs_at_299_data = ref<any[]>([]);
const { data: api_jaxrs_at_299_q } = useQuery({queryKey: ['api_jaxrs_at_299', '/jaxrs/attendance/assemble/control/v2/detail/statistic/export/filter'], queryFn: async () => { try { const r = await api.get("/jaxrs/attendance/assemble/control/v2/detail/statistic/export/filter"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_at_299_q, (v) => { api_jaxrs_at_299_data.value = v ?? []; });
const api_jaxrs_at_311_data = ref<any[]>([]);
const { data: api_jaxrs_at_311_q } = useQuery({queryKey: ['api_jaxrs_at_311', '/jaxrs/attendance/assemble/control/v2/detail/statistic/filter'], queryFn: async () => { try { const r = await api.get("/jaxrs/attendance/assemble/control/v2/detail/statistic/filter"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_at_311_q, (v) => { api_jaxrs_at_311_data.value = v ?? []; });
const api_jaxrs_at_385_data = ref<any[]>([]);
const { data: api_jaxrs_at_385_q } = useQuery({queryKey: ['api_jaxrs_at_385', '/jaxrs/attendance/assemble/control/v2/group/list/1/size/20'], queryFn: async () => { try { const r = await api.get("/jaxrs/attendance/assemble/control/v2/group/list/1/size/20"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_at_385_q, (v) => { api_jaxrs_at_385_data.value = v ?? []; });
const api_jaxrs_at_227_data = ref<any[]>([]);
const { data: api_jaxrs_at_227_q } = useQuery({queryKey: ['api_jaxrs_at_227', '/jaxrs/attendance/assemble/control/v2/group/person/u001/date/2026-08-01'], queryFn: async () => { try { const r = await api.get("/jaxrs/attendance/assemble/control/v2/group/person/u001/date/2026-08-01"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_at_227_q, (v) => { api_jaxrs_at_227_data.value = v ?? []; });
const api_jaxrs_at_595_data = ref<any[]>([]);
const { data: api_jaxrs_at_595_q } = useQuery({queryKey: ['api_jaxrs_at_595', '/jaxrs/attendance/assemble/control/v2/group/some-id'], queryFn: async () => { try { const r = await api.get("/jaxrs/attendance/assemble/control/v2/group/some-id"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_at_595_q, (v) => { api_jaxrs_at_595_data.value = v ?? []; });
const api_jaxrs_at_96_data = ref<any[]>([]);
const { data: api_jaxrs_at_96_q } = useQuery({queryKey: ['api_jaxrs_at_96', '/jaxrs/attendance/assemble/control/v2/group/some-id/delete'], queryFn: async () => { try { const r = await api.get("/jaxrs/attendance/assemble/control/v2/group/some-id/delete"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_at_96_q, (v) => { api_jaxrs_at_96_data.value = v ?? []; });
const api_jaxrs_at_436_data = ref<any[]>([]);
const { data: api_jaxrs_at_436_q } = useQuery({queryKey: ['api_jaxrs_at_436', '/jaxrs/attendance/assemble/control/v2/group/some-id/refresh/participate'], queryFn: async () => { try { const r = await api.get("/jaxrs/attendance/assemble/control/v2/group/some-id/refresh/participate"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_at_436_q, (v) => { api_jaxrs_at_436_data.value = v ?? []; });
const api_jaxrs_at_114_data = ref<any[]>([]);
const { data: api_jaxrs_at_114_q } = useQuery({queryKey: ['api_jaxrs_at_114', '/jaxrs/attendance/assemble/control/v2/groupschedule/list/filter'], queryFn: async () => { try { const r = await api.get("/jaxrs/attendance/assemble/control/v2/groupschedule/list/filter"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_at_114_q, (v) => { api_jaxrs_at_114_data.value = v ?? []; });
const api_jaxrs_at_767_data = ref<any[]>([]);
const { data: api_jaxrs_at_767_q } = useQuery({queryKey: ['api_jaxrs_at_767', '/jaxrs/attendance/assemble/control/v2/leave'], queryFn: async () => { try { const r = await api.get("/jaxrs/attendance/assemble/control/v2/leave"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_at_767_q, (v) => { api_jaxrs_at_767_data.value = v ?? []; });
const api_jaxrs_at_324_data = ref<any[]>([]);
const { data: api_jaxrs_at_324_q } = useQuery({queryKey: ['api_jaxrs_at_324', '/jaxrs/attendance/assemble/control/v2/leave/delete/l-1'], queryFn: async () => { try { const r = await api.get("/jaxrs/attendance/assemble/control/v2/leave/delete/l-1"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_at_324_q, (v) => { api_jaxrs_at_324_data.value = v ?? []; });
const api_jaxrs_at_615_data = ref<any[]>([]);
const { data: api_jaxrs_at_615_q } = useQuery({queryKey: ['api_jaxrs_at_615', '/jaxrs/attendance/assemble/control/v2/leave/import'], queryFn: async () => { try { const r = await api.get("/jaxrs/attendance/assemble/control/v2/leave/import"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_at_615_q, (v) => { api_jaxrs_at_615_data.value = v ?? []; });
const api_jaxrs_at_731_data = ref<any[]>([]);
const { data: api_jaxrs_at_731_q } = useQuery({queryKey: ['api_jaxrs_at_731', '/jaxrs/attendance/assemble/control/v2/leave/import/result/flag/flag-x'], queryFn: async () => { try { const r = await api.get("/jaxrs/attendance/assemble/control/v2/leave/import/result/flag/flag-x"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_at_731_q, (v) => { api_jaxrs_at_731_data.value = v ?? []; });
const api_jaxrs_at_121_data = ref<any[]>([]);
const { data: api_jaxrs_at_121_q } = useQuery({queryKey: ['api_jaxrs_at_121', '/jaxrs/attendance/assemble/control/v2/leave/list/1/size/20'], queryFn: async () => { try { const r = await api.get("/jaxrs/attendance/assemble/control/v2/leave/list/1/size/20"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_at_121_q, (v) => { api_jaxrs_at_121_data.value = v ?? []; });
const api_jaxrs_at_14_data = ref<any[]>([]);
const { data: api_jaxrs_at_14_q } = useQuery({queryKey: ['api_jaxrs_at_14', '/jaxrs/attendance/assemble/control/v2/leave/template'], queryFn: async () => { try { const r = await api.get("/jaxrs/attendance/assemble/control/v2/leave/template"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_at_14_q, (v) => { api_jaxrs_at_14_data.value = v ?? []; });
const api_jaxrs_at_242_data = ref<any[]>([]);
const { data: api_jaxrs_at_242_q } = useQuery({queryKey: ['api_jaxrs_at_242', '/jaxrs/attendance/assemble/control/v2/mobile/check'], queryFn: async () => { try { const r = await api.get("/jaxrs/attendance/assemble/control/v2/mobile/check"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_at_242_q, (v) => { api_jaxrs_at_242_data.value = v ?? []; });
const api_jaxrs_at_891_data = ref<any[]>([]);
const { data: api_jaxrs_at_891_q } = useQuery({queryKey: ['api_jaxrs_at_891', '/jaxrs/attendance/assemble/control/v2/mobile/check/%20from/out'], queryFn: async () => { try { const r = await api.get("/jaxrs/attendance/assemble/control/v2/mobile/check/%20from/out"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_at_891_q, (v) => { api_jaxrs_at_891_data.value = v ?? []; });
const api_jaxrs_at_381_data = ref<any[]>([]);
const { data: api_jaxrs_at_381_q } = useQuery({queryKey: ['api_jaxrs_at_381', '/jaxrs/attendance/assemble/control/v2/mobile/check/pre'], queryFn: async () => { try { const r = await api.get("/jaxrs/attendance/assemble/control/v2/mobile/check/pre"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_at_381_q, (v) => { api_jaxrs_at_381_data.value = v ?? []; });
const api_jaxrs_at_81_data = ref<any[]>([]);
const { data: api_jaxrs_at_81_q } = useQuery({queryKey: ['api_jaxrs_at_81', '/jaxrs/attendance/assemble/control/v2/my/controls'], queryFn: async () => { try { const r = await api.get("/jaxrs/attendance/assemble/control/v2/my/controls"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_at_81_q, (v) => { api_jaxrs_at_81_data.value = v ?? []; });
const api_jaxrs_at_493_data = ref<any[]>([]);
const { data: api_jaxrs_at_493_q } = useQuery({queryKey: ['api_jaxrs_at_493', '/jaxrs/attendance/assemble/control/v2/my/detail/list'], queryFn: async () => { try { const r = await api.get("/jaxrs/attendance/assemble/control/v2/my/detail/list"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_at_493_q, (v) => { api_jaxrs_at_493_data.value = v ?? []; });
const api_jaxrs_at_372_data = ref<any[]>([]);
const { data: api_jaxrs_at_372_q } = useQuery({queryKey: ['api_jaxrs_at_372', '/jaxrs/attendance/assemble/control/v2/my/rest/date/check'], queryFn: async () => { try { const r = await api.get("/jaxrs/attendance/assemble/control/v2/my/rest/date/check"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_at_372_q, (v) => { api_jaxrs_at_372_data.value = v ?? []; });
const api_jaxrs_at_746_data = ref<any[]>([]);
const { data: api_jaxrs_at_746_q } = useQuery({queryKey: ['api_jaxrs_at_746', '/jaxrs/attendance/assemble/control/v2/my/statistic'], queryFn: async () => { try { const r = await api.get("/jaxrs/attendance/assemble/control/v2/my/statistic"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_at_746_q, (v) => { api_jaxrs_at_746_data.value = v ?? []; });
const api_jaxrs_at_346_data = ref<any[]>([]);
const { data: api_jaxrs_at_346_q } = useQuery({queryKey: ['api_jaxrs_at_346', '/jaxrs/attendance/assemble/control/v2/my/version'], queryFn: async () => { try { const r = await api.get("/jaxrs/attendance/assemble/control/v2/my/version"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_at_346_q, (v) => { api_jaxrs_at_346_data.value = v ?? []; });
const api_jaxrs_at_752_data = ref<any[]>([]);
const { data: api_jaxrs_at_752_q } = useQuery({queryKey: ['api_jaxrs_at_752', '/jaxrs/attendance/assemble/control/v2/record/import'], queryFn: async () => { try { const r = await api.get("/jaxrs/attendance/assemble/control/v2/record/import"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_at_752_q, (v) => { api_jaxrs_at_752_data.value = v ?? []; });
const api_jaxrs_at_271_data = ref<any[]>([]);
const { data: api_jaxrs_at_271_q } = useQuery({queryKey: ['api_jaxrs_at_271', '/jaxrs/attendance/assemble/control/v2/record/import/daily'], queryFn: async () => { try { const r = await api.get("/jaxrs/attendance/assemble/control/v2/record/import/daily"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_at_271_q, (v) => { api_jaxrs_at_271_data.value = v ?? []; });
const api_jaxrs_attendance_785_data = ref<any[]>([]);
const { data: api_jaxrs_attendance_785_q } = useQuery({queryKey: ['api_jaxrs_attendance_785', '/jaxrs/attendance/assemble/control/v2/record/list/1/size/20'], queryFn: async () => { try { const r = await api.get("/jaxrs/attendance/assemble/control/v2/record/list/1/size/20"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_attendance_785_q, (v) => { api_jaxrs_attendance_785_data.value = v ?? []; });
const api_jaxrs_at_256_data = ref<any[]>([]);
const { data: api_jaxrs_at_256_q } = useQuery({queryKey: ['api_jaxrs_at_256', '/jaxrs/attendance/assemble/control/v2/record/r-1'], queryFn: async () => { try { const r = await api.get("/jaxrs/attendance/assemble/control/v2/record/r-1"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_at_256_q, (v) => { api_jaxrs_at_256_data.value = v ?? []; });
const api_jaxrs_attendance_474_data = ref<any[]>([]);
const { data: api_jaxrs_attendance_474_q } = useQuery({queryKey: ['api_jaxrs_attendance_474', '/jaxrs/attendance/assemble/control/v2/record/template'], queryFn: async () => { try { const r = await api.get("/jaxrs/attendance/assemble/control/v2/record/template"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_attendance_474_q, (v) => { api_jaxrs_attendance_474_data.value = v ?? []; });
const api_jaxrs_attendance_481_data = ref<any[]>([]);
const { data: api_jaxrs_attendance_481_q } = useQuery({queryKey: ['api_jaxrs_attendance_481', '/jaxrs/attendance/assemble/control/v2/shift/abc'], queryFn: async () => { try { const r = await api.get("/jaxrs/attendance/assemble/control/v2/shift/abc"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_attendance_481_q, (v) => { api_jaxrs_attendance_481_data.value = v ?? []; });
const api_jaxrs_attendance_160_data = ref<any[]>([]);
const { data: api_jaxrs_attendance_160_q } = useQuery({queryKey: ['api_jaxrs_attendance_160', '/jaxrs/attendance/assemble/control/v2/shift/create'], queryFn: async () => { try { const r = await api.get("/jaxrs/attendance/assemble/control/v2/shift/create"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_attendance_160_q, (v) => { api_jaxrs_attendance_160_data.value = v ?? []; });
const api_jaxrs_attendance_738_data = ref<any[]>([]);
const { data: api_jaxrs_attendance_738_q } = useQuery({queryKey: ['api_jaxrs_attendance_738', '/jaxrs/attendance/assemble/control/v2/shift/delete/abc'], queryFn: async () => { try { const r = await api.get("/jaxrs/attendance/assemble/control/v2/shift/delete/abc"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_attendance_738_q, (v) => { api_jaxrs_attendance_738_data.value = v ?? []; });
const api_jaxrs_attendance_374_data = ref<any[]>([]);
const { data: api_jaxrs_attendance_374_q } = useQuery({queryKey: ['api_jaxrs_attendance_374', '/jaxrs/attendance/assemble/control/v2/shift/list/1/size/20'], queryFn: async () => { try { const r = await api.get("/jaxrs/attendance/assemble/control/v2/shift/list/1/size/20"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_attendance_374_q, (v) => { api_jaxrs_attendance_374_data.value = v ?? []; });
const api_jaxrs_attendance_557_data = ref<any[]>([]);
const { data: api_jaxrs_attendance_557_q } = useQuery({queryKey: ['api_jaxrs_attendance_557', '/jaxrs/attendance/assemble/control/v2/shift/update'], queryFn: async () => { try { const r = await api.get("/jaxrs/attendance/assemble/control/v2/shift/update"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_attendance_557_q, (v) => { api_jaxrs_attendance_557_data.value = v ?? []; });
const api_jaxrs_attendance_703_data = ref<any[]>([]);
const { data: api_jaxrs_attendance_703_q } = useQuery({queryKey: ['api_jaxrs_attendance_703', '/jaxrs/attendance/assemble/control/v2/workplace/list/all'], queryFn: async () => { try { const r = await api.get("/jaxrs/attendance/assemble/control/v2/workplace/list/all"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_attendance_703_q, (v) => { api_jaxrs_attendance_703_data.value = v ?? []; });
const api_jaxrs_attendance_968_data = ref<any[]>([]);
const { data: api_jaxrs_attendance_968_q } = useQuery({queryKey: ['api_jaxrs_attendance_968', '/jaxrs/attendance/assemble/control/v2/workplace/list/ids'], queryFn: async () => { try { const r = await api.get("/jaxrs/attendance/assemble/control/v2/workplace/list/ids"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_attendance_968_q, (v) => { api_jaxrs_attendance_968_data.value = v ?? []; });
const api_jaxrs_attendance_232_data = ref<any[]>([]);
const { data: api_jaxrs_attendance_232_q } = useQuery({queryKey: ['api_jaxrs_attendance_232', '/jaxrs/attendance/assemble/control/workplace/list/all'], queryFn: async () => { try { const r = await api.get("/jaxrs/attendance/assemble/control/workplace/list/all"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_attendance_232_q, (v) => { api_jaxrs_attendance_232_data.value = v ?? []; });
const api_jaxrs_attendance_425_data = ref<any[]>([]);
const { data: api_jaxrs_attendance_425_q } = useQuery({queryKey: ['api_jaxrs_attendance_425', '/jaxrs/attendance/assemble/control/workplace/w-1'], queryFn: async () => { try { const r = await api.get("/jaxrs/attendance/assemble/control/workplace/w-1"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_attendance_425_q, (v) => { api_jaxrs_attendance_425_data.value = v ?? []; });
const api_jaxrs_attendance_1_data = ref<any[]>([]);
const { data: api_jaxrs_attendance_1_q } = useQuery({queryKey: ['api_jaxrs_attendance_1', '/jaxrs/attendance/assemble/daily'], queryFn: async () => { try { const r = await api.get("/jaxrs/attendance/assemble/daily"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_attendance_1_q, (v) => { api_jaxrs_attendance_1_data.value = v ?? []; });
const api_jaxrs_attendance_547_data = ref<any[]>([]);
const { data: api_jaxrs_attendance_547_q } = useQuery({queryKey: ['api_jaxrs_attendance_547', '/jaxrs/attendance/core/entity/record/record-001/delete'], queryFn: async () => { try { const r = await api.get("/jaxrs/attendance/core/entity/record/record-001/delete"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_attendance_547_q, (v) => { api_jaxrs_attendance_547_data.value = v ?? []; });
const api_jaxrs_attendance_860_data = ref<any[]>([]);
const { data: api_jaxrs_attendance_860_q } = useQuery({queryKey: ['api_jaxrs_attendance_860', '/jaxrs/attendance/core/entity/record/record-001/update'], queryFn: async () => { try { const r = await api.get("/jaxrs/attendance/core/entity/record/record-001/update"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_attendance_860_q, (v) => { api_jaxrs_attendance_860_data.value = v ?? []; });
const api_jaxrs_attendance_877_data = ref<any[]>([]);
const { data: api_jaxrs_attendance_877_q } = useQuery({queryKey: ['api_jaxrs_attendance_877', '/jaxrs/attendance/core/entity/rule/create'], queryFn: async () => { try { const r = await api.get("/jaxrs/attendance/core/entity/rule/create"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_attendance_877_q, (v) => { api_jaxrs_attendance_877_data.value = v ?? []; });
const api_jaxrs_attendance_279_data = ref<any[]>([]);
const { data: api_jaxrs_attendance_279_q } = useQuery({queryKey: ['api_jaxrs_attendance_279', '/jaxrs/attendance/core/entity/rule/list'], queryFn: async () => { try { const r = await api.get("/jaxrs/attendance/core/entity/rule/list"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_attendance_279_q, (v) => { api_jaxrs_attendance_279_data.value = v ?? []; });
const api_jaxrs_attendance_662_data = ref<any[]>([]);
const { data: api_jaxrs_attendance_662_q } = useQuery({queryKey: ['api_jaxrs_attendance_662', '/jaxrs/attendance/core/entity/rule/rule-001/delete'], queryFn: async () => { try { const r = await api.get("/jaxrs/attendance/core/entity/rule/rule-001/delete"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_attendance_662_q, (v) => { api_jaxrs_attendance_662_data.value = v ?? []; });
const api_jaxrs_attendance_402_data = ref<any[]>([]);
const { data: api_jaxrs_attendance_402_q } = useQuery({queryKey: ['api_jaxrs_attendance_402', '/jaxrs/attendance/core/entity/rule/rule-001/update'], queryFn: async () => { try { const r = await api.get("/jaxrs/attendance/core/entity/rule/rule-001/update"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_attendance_402_q, (v) => { api_jaxrs_attendance_402_data.value = v ?? []; });
const api_jaxrs_attendance_744_data = ref<any[]>([]);
const { data: api_jaxrs_attendance_744_q } = useQuery({queryKey: ['api_jaxrs_attendance_744', '/jaxrs/attendance/employee/config/list/all'], queryFn: async () => { try { const r = await api.get("/jaxrs/attendance/employee/config/list/all"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_attendance_744_q, (v) => { api_jaxrs_attendance_744_data.value = v ?? []; });
const api_jaxrs_attendance_135_data = ref<any[]>([]);
const { data: api_jaxrs_attendance_135_q } = useQuery({queryKey: ['api_jaxrs_attendance_135', '/jaxrs/attendance/record/list'], queryFn: async () => { try { const r = await api.get("/jaxrs/attendance/record/list"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_attendance_135_q, (v) => { api_jaxrs_attendance_135_data.value = v ?? []; });
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
