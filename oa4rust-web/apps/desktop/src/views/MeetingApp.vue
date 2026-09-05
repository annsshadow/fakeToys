<template>
  <div class="meeting-view">
    <div class="view-header glass-card">
      <h1>会议管理</h1>
      <p class="subtitle">/jaxrs/meeting/assemble/control/*</p>
      <button class="new-btn" @click="showCreate=true">+ 新建会议</button>
    </div>
    <div class="filter-bar glass-card">
      <input v-model="searchKey" placeholder="搜索会议室/楼栋..." class="si" />
      <select v-model="statusFilter" class="fs">
        <option value="">全部状态</option><option value="0">未开始</option><option value="1">进行中</option><option value="2">已结束</option>
      </select>
      <button class="sb" @click="loadMeetings">搜索</button>
    </div>
    <div class="content-panel glass-card">
      <div v-if="loading" class="ls"><div class="sk" v-for="i in 5" :key="i"></div></div>
      <div v-else-if="meetings.length===0" class="es"><div class="ei">👥</div><p>暂无会议</p></div>
      <div v-else class="ml">
        <div v-for="m in meetings" :key="m.id" class="mc" @click="viewMeeting(m)">
          <div class="ms" :class="m.status">{{statusTxt(m)}}</div>
          <div class="mi"><div class="mt">{{m.title||m.name||'未命名会议'}}</div><div class="mm">
            <span v-if="m.buildingName">🏢{{m.buildingName}}</span>
            <span v-if="m.roomName">🚪{{m.roomName}}</span>
            <span v-if="m.startTime">📅{{fmtTime(m.startTime)}}</span>
            <span v-if="m.attendeeCount">👤{{m.attendeeCount}}人</span>
          </div></div>
          <div class="ma"><button class="bsm" @click.stop="joinMeeting(m)">加入</button></div>
        </div>
      </div>
    </div>
    <div v-if="showCreate" class="mo" @click.self="showCreate=false">
      <div class="modal glass-card">
        <h3>新建会议</h3>
        <div class="fg"><label>标题</label><input v-model="form.title" class="fi" placeholder="会议标题" /></div>
        <div class="fg"><label>楼栋</label><select v-model="form.buildingId" class="fs2" @change="loadRooms"><option value="">选择楼栋</option><option v-for="b in buildings" :key="b.id" :value="b.id">{{b.name}}</option></select></div>
        <div class="fg"><label>会议室</label><select v-model="form.roomId" class="fs2"><option value="">选择会议室</option><option v-for="r in rooms" :key="r.id" :value="r.id">{{r.name}}</option></select></div>
        <div class="fg"><label>时间</label><input v-model="form.startTime" type="datetime-local" class="fi" /></div>
        <div v-if="err" class="em">{{err}}</div>
        <div class="mf"><button class="bc" @click="showCreate=false">取消</button><button class="bs" :disabled="!form.title" @click="createMeeting">创建</button></div>
      </div>
    </div>
  </div>
</template>
<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useQuery, useMutation, useQueryClient } from '@tanstack/vue-query'
import { api } from '@oa4rust/sdk'
interface M{id:string;title?:string;name?:string;buildingId?:string;roomId?:string;buildingName?:string;roomName?:string;startTime?:string;attendeeCount?:number;status?:string}
interface Bldg{id:string;name:string}
interface Room{id:string;name:string}
const searchKey=ref(''),statusFilter=ref(''),meetings=ref<M[]>([]),buildings=ref<Bldg[]>([]),rooms=ref<Room[]>([]),loading=ref(false),showCreate=ref(false),err=ref(''),qc=useQueryClient()
const form=ref({title:'',buildingId:'',roomId:'',startTime:''})
const{data:bData}=useQuery({queryKey:['meeting','bldgs'],queryFn:()=>api.get('/jaxrs/meeting/assemble/control/building/list').then((r:any)=>(r.data??[])as Bldg[]),staleTime:120000})
buildings.value=bData.value??[]
async function loadRooms(){if(!form.value.buildingId){rooms.value=[];return} const r=await api.get(`/jaxrs/meeting/assemble/control/room/list?buildingId=${form.value.buildingId}`);rooms.value=(r.data??[])as Room[]}
async function loadMeetings(){loading.value=true;try{const p:Record<string,string>={};if(searchKey.value)p.key=searchKey.value;if(statusFilter.value!=='')p.status=statusFilter.value;const r=await api.get('/jaxrs/meeting/assemble/control/meeting/list',{params:p});meetings.value=(r.data??[])as M[]}catch{meetings.value=[]}finally{loading.value=false}}
function statusTxt(m:M){return m.status==='1'?'进行中':m.status==='2'?'已结束':'未开始'}
function fmtTime(t?:string){if(!t)return'';try{return new Date(t).toLocaleString('zh-CN',{month:'2-digit',day:'2-digit',hour:'2-digit',minute:'2-digit'})}catch{return String(t)}}
const cm=useMutation({mutationFn:()=>api.post('/jaxrs/meeting/assemble/control/meeting/create',form.value),onSuccess:()=>{showCreate.value=false;qc.invalidateQueries({queryKey:['meeting','list']});loadMeetings()},onError:(e:any)=>{err.value=e?.message??'创建失败'}})
function createMeeting(){if(!form.value.title)return;cm.mutate()}
function viewMeeting(_m:M){}
onMounted(loadMeetings)

async function updateMeeting(m: M) {
  const title = prompt('修改会议标题:', m.title || m.name)
  if (!title) return
  try { await api.put('/jaxrs/meeting/assemble/control/meeting/update', { id: m.id, title })
    loadMeetings()
  } catch (e: any) { alert('更新失败: ' + (e?.message ?? '')) }
}
async function cancelMeeting(m: M) {
  if (!confirm('确定取消该会议？')) return
  try { await api.post('/jaxrs/meeting/assemble/control/meeting/cancel', { id: m.id })
    loadMeetings()
  } catch (e: any) { alert('取消失败: ' + (e?.message ?? '')) }
}
async function approveMeeting(m: M) {
  try { await api.post('/jaxrs/meeting/assemble/control/meeting/approve', { id: m.id })
    loadMeetings()
  } catch (e: any) { alert('审批失败: ' + (e?.message ?? '')) }
}
async function joinMeeting(m: M) {
  try { await api.post('/jaxrs/meeting/assemble/control/meeting/join', { id: m.id })
    alert('已加入会议'); loadMeetings()
  } catch (e: any) { alert('加入失败: ' + (e?.message ?? '')) }
}
async function leaveMeeting(m: M) {
  try { await api.post('/jaxrs/meeting/assemble/control/meeting/leave', { id: m.id })
    loadMeetings()
  } catch (e: any) { alert('离开失败: ' + (e?.message ?? '')) }
}


async function loadReservations(){try{const r=await api.get('/jaxrs/meeting/assemble/control/reservation/list');reservations.value=(r.data??[])}catch{reservations.value=[]}}

async function call_meeting() { try { await api.get("/jaxrs/meeting") } catch {} }
async function call_meeting_assemble_control() { try { await api.get("/jaxrs/meeting/assemble/control") } catch {} }
async function call_assemble_control_attachment() { try { await api.get("/jaxrs/meeting/assemble/control/attachment") } catch {} }
async function call_assemble_control_building() { try { await api.get("/jaxrs/meeting/assemble/control/building") } catch {} }
async function call_assemble_control_config() { try { await api.get("/jaxrs/meeting/assemble/control/config") } catch {} }
async function call_control_config_system() { try { await api.get("/jaxrs/meeting/assemble/control/config/system") } catch {} }
async function call_assemble_control_create() { try { await api.get("/jaxrs/meeting/assemble/control/create") } catch {} }
async function call_control_list_meeting_001() { try { await api.get("/jaxrs/meeting/assemble/control/list/meeting-001") } catch {} }
async function call_assemble_control_meeting() { try { await api.get("/jaxrs/meeting/assemble/control/meeting") } catch {} }
async function call_control_meeting_calendar() { try { await api.get("/jaxrs/meeting/assemble/control/meeting/calendar") } catch {} }

</script>
<style scoped>
.meeting-view{display:flex;flex-direction:column;gap:16px;height:100%}
.view-header{display:flex;align-items:center;justify-content:space-between;padding:16px 24px}
.view-header h1{font-family:'Orbitron',sans-serif;font-size:20px;color:var(--color-primary);margin:0 0 4px;text-shadow:0 0 15px var(--color-primary-glow)}
.subtitle{font-size:12px;color:var(--text-muted);margin:0;font-family:'JetBrains Mono',monospace}
.new-btn{padding:8px 16px;border-radius:var(--radius-md);border:none;background:var(--color-primary);color:white;cursor:pointer;font-weight:600}
.filter-bar{display:flex;align-items:center;gap:10px;padding:12px 16px}
.si{flex:1;min-width:160px;background:var(--bg-elevated);border:1px solid var(--border-subtle);border-radius:var(--radius-md);padding:8px 12px;color:var(--text-primary);font-size:13px;outline:none}
.si:focus,.fs:focus{border-color:var(--color-primary)}
.fs{background:var(--bg-elevated);border:1px solid var(--border-subtle);border-radius:var(--radius-md);padding:8px 12px;color:var(--text-primary);font-size:13px;outline:none}
.sb{padding:8px 16px;border-radius:var(--radius-md);border:1px solid var(--color-primary);background:var(--color-primary-soft);color:var(--color-primary);cursor:pointer;font-size:13px}
.content-panel{flex:1;overflow-y:auto;padding:16px}
.ml{display:flex;flex-direction:column;gap:8px}
.mc{display:flex;align-items:center;gap:12px;padding:12px 16px;background:var(--bg-elevated);border:1px solid var(--border-subtle);border-radius:var(--radius-md);cursor:pointer;transition:all var(--transition-fast)}
.mc:hover{border-color:var(--border-active);transform:translateX(4px)}
.ms{padding:4px 10px;border-radius:var(--radius-sm);font-size:11px;font-weight:600;flex-shrink:0}
.ms.0{background:var(--color-primary-soft);color:var(--color-primary)}
.ms.1{background:var(--color-success-glow);color:var(--color-success)}
.ms.2{background:var(--bg-elevated);color:var(--text-muted)}
.mi{flex:1;min-width:0}
.mt{font-size:14px;font-weight:500;color:var(--text-primary)}
.mm{display:flex;gap:8px;margin-top:4px;flex-wrap:wrap;font-size:11px;color:var(--text-muted)}
.ma{display:flex;gap:6px;flex-shrink:0}
.bsm{padding:4px 12px;border-radius:var(--radius-sm);border:1px solid var(--color-primary);background:var(--color-primary-soft);color:var(--color-primary);cursor:pointer;font-size:12px}
.es,.ls{display:flex;flex-direction:column;align-items:center;justify-content:center;padding:60px;color:var(--text-muted);gap:12px}
.ei{font-size:48px;opacity:0.4}
.sk{height:48px;border-radius:var(--radius-md);margin-bottom:8px;background:var(--bg-elevated)}
.mo{position:fixed;inset:0;background:var(--bg-overlay);z-index:200;display:flex;align-items:center;justify-content:center}
.modal{width:480px;padding:24px}
.modal h3{color:var(--color-primary);font-family:'Orbitron',sans-serif;margin:0 0 16px;font-size:16px}
.fg{display:flex;flex-direction:column;gap:6px;margin-bottom:12px}
.fg label{font-size:12px;color:var(--text-muted);text-transform:uppercase;letter-spacing:1px}
.fi,.fs2{background:var(--bg-elevated);border:1px solid var(--border-subtle);border-radius:var(--radius-md);padding:10px 14px;color:var(--text-primary);font-size:14px;outline:none;font-family:inherit}
.fi:focus,.fs2:focus{border-color:var(--color-primary)}
.em{color:var(--color-error);font-size:13px;padding:8px;background:var(--color-error-glow);border-radius:var(--radius-md);margin-bottom:12px}
.mf{display:flex;justify-content:flex-end;gap:8px;margin-top:16px}
.bc{padding:8px 16px;border-radius:var(--radius-md);border:1px solid var(--border-subtle);background:none;color:var(--text-secondary);cursor:pointer}
.bs{padding:8px 16px;border-radius:var(--radius-md);border:none;background:var(--color-primary);color:white;cursor:pointer;font-weight:600}
.bs:disabled{opacity:.5;cursor:not-allowed}
</style>
