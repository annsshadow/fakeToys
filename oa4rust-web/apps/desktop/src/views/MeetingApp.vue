<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<template>
  <div class="meeting-view">
    <div class="view-header glass-card">
      <h1>会议管理</h1>
      <p class="subtitle">/api/meeting/assemble/control/*</p>
      <button class="new-btn" @click="showCreate=true">+ 新建会议</button>
    </div>
    <div class="filter-bar glass-card">
      <input v-model="searchKey" placeholder="搜索会议室/楼栋..." class="si" />
      <select v-model="statusFilter" class="fs">
        <option value="">全部状态</option><option value="0">未开始</option><option value="1">进行中</option><option value="2">已结束</option>
      </select>
      <button class="sb" @click="loadMeetings">搜索</button>
      <button class="sb" @click="loadMyApplied">我的申请</button>
      <button class="sb" @click="loadMyInvited">我的邀请</button>
      <button class="sb" @click="loadMeetingMore">待接受/本月/配置</button>
      <button class="sb" @click="loadMeetingSearch">检索/前瞻</button>
      <button class="sb" @click="addBuilding">+ 楼栋</button>
    </div>
    <div v-if="appliedText" class="applied-note">{{ appliedText }}</div>
    <div v-if="buildings.length" class="bld-bar glass-card">
      <span class="bld-title">楼栋：</span>
      <span v-for="b in buildings" :key="b.id" class="bld-chip">{{ b.name }}<button class="bld-del" @click="removeBuilding(b)">×</button></span>
    </div>
    <div class="bld-bar glass-card">
      <span class="bld-title">会议室：</span>
      <select v-model="roomBuildingId" class="fs" @change="loadRoomList"><option value="">选楼栋看会议室</option><option v-for="b in buildings" :key="b.id" :value="b.id">{{b.name}}</option></select>
      <button class="sb" @click="addRoom">+ 会议室</button>
      <span v-for="rm in roomList" :key="rm.id" class="bld-chip">{{ rm.name }}<button class="bld-del" @click="removeRoom(rm)">×</button></span>
    </div>
    <div class="content-panel glass-card">
      <div v-if="loading" class="ls"><div class="sk" v-for="i in 5" :key="i"></div></div>
      <div v-else-if="meetings.length===0" class="es"><div class="ei">👥</div><p>暂无会议</p></div>
      <div v-else class="ml">
        <div v-for="m in meetings" :key="m.id" class="mc" @click="viewMeeting(m)">
          <div class="ms" :class="statusCls(m)">{{statusTxt(m)}}</div>
          <div class="mi"><div class="mt">{{m.title||m.name||'未命名会议'}}</div><div class="mm">
            <span v-if="m.buildingName">🏢{{m.buildingName}}</span>
            <span v-if="m.roomName">🚪{{m.roomName}}</span>
            <span v-if="m.startTime">📅{{fmtTime(m.startTime)}}</span>
            <span v-if="m.attendeeCount">👤{{m.attendeeCount}}人</span>
          </div></div>
          <div class="ma">
            <button class="bsm" @click.stop="joinMeeting(m)">加入</button>
            <button class="bsm" @click.stop="showParticipants(m)">参会人</button>
            <button class="bsm" @click.stop="inviteParticipant(m)">邀请</button>
          </div>
        </div>
      </div>
    </div>
    <div v-if="showCreate" class="mo" @click.self="showCreate=false">
      <div class="modal glass-card">
        <h3>新建会议</h3>
        <div class="fg"><label>标题</label><input v-model="form.title" class="fi" placeholder="会议标题" /></div>
        <div class="fg"><label>楼栋</label><select v-model="form.buildingId" class="fs2" @change="loadRooms"><option value="">选择楼栋</option><option v-for="b in buildings" :key="b.id" :value="b.id">{{b.name}}</option></select></div>
        <div class="fg"><label>会议室</label><select v-model="form.roomId" class="fs2"><option value="">选择会议室</option><option v-for="r in rooms" :key="r.id" :value="r.id">{{r.name}}</option></select></div>
        <div class="fg"><label>开始</label><input v-model="form.startTime" type="datetime-local" class="fi" /></div>
        <div class="fg"><label>结束</label><input v-model="form.endTime" type="datetime-local" class="fi" /></div>
        <div class="fg"><label>说明</label><input v-model="form.content" class="fi" placeholder="会议说明（可选）" /></div>
        <div v-if="err" class="em">{{err}}</div>
        <div class="mf"><button class="bc" @click="showCreate=false">取消</button><button class="bs" :disabled="!form.title" @click="createMeeting">创建</button></div>
      </div>
    </div>
    <div v-if="detail.open" class="mo" @click.self="detail.open=false">
      <div class="modal glass-card">
        <h3>会议详情</h3>
        <div v-if="detail.loading" class="es"><p>加载中…</p></div>
        <template v-else>
          <div class="fg"><label>标题</label><div class="dv">{{detail.title||'—'}}</div></div>
          <div class="fg"><label>时间</label><div class="dv">{{detail.startTime||'—'}} ~ {{detail.endTime||'—'}}</div></div>
          <div class="fg"><label>创建者</label><div class="dv">{{detail.creator||'—'}}</div></div>
          <div class="fg"><label>说明</label><div class="dv">{{detail.content||'—'}}</div></div>
          <div class="fg"><label>控制项</label><div class="dv">{{detail.controlCount}} 项</div></div>
          <div v-if="detail.extra" class="fg"><label>关联</label><div class="dv">{{detail.extra}}</div></div>
        </template>
        <div class="mf"><button class="bc" @click="detail.open=false">关闭</button></div>
      </div>
    </div>
  </div>
</template>
<script setup lang="ts">
import { api, useSession } from '@oa4rust/sdk'
import { useMutation, useQuery, useQueryClient } from '@tanstack/vue-query'
import { onMounted, ref } from 'vue'
import { confirmMsg, toast } from '../utils/toast'

interface M {
  id: string
  title?: string
  name?: string
  buildingId?: string
  roomId?: string
  buildingName?: string
  roomName?: string
  startTime?: string
  attendeeCount?: number
  status?: string
}
interface Bldg {
  id: string
  name: string
}
interface Room {
  id: string
  name: string
}
const searchKey = ref(''),
  statusFilter = ref(''),
  meetings = ref<M[]>([]),
  buildings = ref<Bldg[]>([]),
  rooms = ref<Room[]>([]),
  loading = ref(false),
  showCreate = ref(false),
  err = ref(''),
  qc = useQueryClient()
const session = useSession()
const form = ref({ title: '', content: '', buildingId: '', roomId: '', startTime: '', endTime: '' })
const { data: bData } = useQuery({
  queryKey: ['meeting', 'bldgs'],
  queryFn: () => api.get('/api/meeting/assemble/control/building/list').then((r: any) => (r.data ?? []) as Bldg[]),
  staleTime: 120000,
})
buildings.value = bData.value ?? []
async function loadRooms() {
  if (!form.value.buildingId) {
    rooms.value = []
    return
  }
  const r = await api.get(`/api/meeting/assemble/control/room/list?buildingId=${form.value.buildingId}`)
  rooms.value = (r.data ?? []) as Room[]
}
async function loadMeetings() {
  loading.value = true
  try {
    const p: Record<string, string> = {}
    if (searchKey.value) p.key = searchKey.value
    if (statusFilter.value !== '') p.status = statusFilter.value
    // 后端无裸 meeting/list；meeting/list/apply/{page}/size/{size} 为真实列表端点。
    const r = await api.get('/api/meeting/assemble/control/meeting/list/apply/1/size/50')
    meetings.value = (r.data ?? []) as M[]
  } catch {
    meetings.value = []
  } finally {
    loading.value = false
  }
}
function statusTxt(m: M) {
  return m.status === '1' ? '进行中' : m.status === '2' ? '已结束' : '未开始'
}
function statusCls(m: M) {
  return m.status === '1' ? 'active' : m.status === '2' ? 'ended' : 'pending'
}
function fmtTime(t?: string) {
  if (!t) return ''
  try {
    return new Date(t).toLocaleString('zh-CN', { month: '2-digit', day: '2-digit', hour: '2-digit', minute: '2-digit' })
  } catch {
    return String(t)
  }
}
const cm = useMutation({
  // 后端 create_meeting 契约：title/content/startTime/endTime/creator/roomId（buildingId 仅用于筛选会议室，不下发）
  mutationFn: () =>
    api.post('/api/meeting/assemble/control/meeting/create', {
      title: form.value.title,
      content: form.value.content,
      startTime: form.value.startTime,
      endTime: form.value.endTime,
      roomId: form.value.roomId,
      creator: session.user?.unique ?? '',
    }),
  onSuccess: () => {
    showCreate.value = false
    qc.invalidateQueries({ queryKey: ['meeting', 'list'] })
    loadMeetings()
  },
  onError: (e: any) => {
    err.value = e?.message ?? '创建失败'
  },
})
function createMeeting() {
  if (!form.value.title) return
  cm.mutate()
}
// 会议详情：GET meeting/{id}（x_meeting 主体）+ list/{meetingId}（控制项 x_meeting_assemble_control）
const detail = ref({ open: false, loading: false, title: '', startTime: '', endTime: '', creator: '', content: '', controlCount: 0, extra: '' })
async function viewMeeting(m: M) {
  detail.value = { open: true, loading: true, title: m.title || m.name || '', startTime: '', endTime: '', creator: '', content: '', controlCount: 0, extra: '' }
  const settle = <T,>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const [main, ctrl, atts, next, prev] = await Promise.all([
    settle(api.get(`/api/meeting/assemble/control/meeting/${encodeURIComponent(m.id)}`)),
    settle(api.get(`/api/meeting/assemble/control/list/${encodeURIComponent(m.id)}`)),
    settle(api.get(`/api/meeting/assemble/control/attachment/list/meeting/${encodeURIComponent(m.id)}`)),
    settle(api.get(`/api/meeting/assemble/control/meeting/list/${encodeURIComponent(m.id)}/next/5`)),
    settle(api.get(`/api/meeting/assemble/control/meeting/list/${encodeURIComponent(m.id)}/prev/5`)),
  ])
  const d: any = (main as any)?.data ?? {}
  detail.value.title = String(d.title ?? m.title ?? m.name ?? '')
  detail.value.startTime = String(d.start_time ?? d.startTime ?? '')
  detail.value.endTime = String(d.end_time ?? d.endTime ?? '')
  detail.value.creator = String(d.creator ?? '')
  detail.value.content = String(d.content ?? '')
  const cd: any = (ctrl as any)?.data
  detail.value.controlCount = Array.isArray(cd) ? cd.length : Array.isArray(cd?.data) ? cd.data.length : 0
  const len = (r: any) => (Array.isArray(r?.data) ? r.data.length : 0)
  detail.value.extra = `附件 ${len(atts)} · 后续会议 ${len(next)} · 前序会议 ${len(prev)}`
  detail.value.loading = false
}
async function showParticipants(m: M) {
  try {
    // GET /api/meeting/{meetingId}/participant/list —— 参会人列表
    const r: any = await api.get(`/api/meeting/${encodeURIComponent(m.id)}/participant/list`)
    const n = Array.isArray(r.data) ? r.data.length : 0
    toast.success('参会人数：' + n)
  } catch (e: any) {
    toast.error('查询失败: ' + (e?.message ?? ''))
  }
}
async function inviteParticipant(m: M) {
  const invitee = prompt('邀请参会人（用户标识 invitee）:')
  if (!invitee) return
  try {
    // POST /api/meeting/{meetingId}/participant/add —— 读 invitee
    await api.post(`/api/meeting/${encodeURIComponent(m.id)}/participant/add`, { invitee })
    toast.success('已邀请')
  } catch (e: any) {
    toast.error('邀请失败: ' + (e?.message ?? ''))
  }
}
const appliedText = ref('')
async function loadMyInvited() {
  try {
    // GET meeting/list/invited/processing + completed + rejected —— 我受邀会议（进行/已办/已拒）
    const [proc, done, rej] = await Promise.all([
      api.get('/api/meeting/assemble/control/meeting/list/invited/processing'),
      api.get('/api/meeting/assemble/control/meeting/list/invited/completed'),
      api.get('/api/meeting/assemble/control/meeting/list/invited/rejected'),
    ])
    const cnt = (r: any) => (Array.isArray(r?.data) ? r.data.length : 0)
    appliedText.value = `受邀进行 ${cnt(proc)} / 已办 ${cnt(done)} / 已拒 ${cnt(rej)}`
  } catch (e: any) {
    toast.error('加载我的邀请失败: ' + (e?.message ?? ''))
  }
}
async function loadMyApplied() {
  try {
    // GET meeting/list/applied/wait + processing + completed —— 我申请的会议（待审/进行/已办）
    const [wait, proc, done] = await Promise.all([
      api.get('/api/meeting/assemble/control/meeting/list/applied/wait'),
      api.get('/api/meeting/assemble/control/meeting/list/applied/processing'),
      api.get('/api/meeting/assemble/control/meeting/list/applied/completed'),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : 0)
    appliedText.value = `待审 ${n(wait)} / 进行 ${n(proc)} / 已办 ${n(done)}`
  } catch (e: any) {
    toast.error('加载我的申请失败: ' + (e?.message ?? ''))
  }
}
// 检索/前瞻族 3 条真实 distinct 路由：楼栋名模糊 building/list/like/{key}（x_meeting_building ILIKE）
// + 会议室名模糊 room/list/like/{key}（x_meeting_room ILIKE）+ 未来 N 月会议 meeting/list/forward/monthcount/{monthCount}（x_meeting）
async function loadMeetingSearch() {
  const key = searchKey.value?.trim() || '会'
  try {
    const [blds, rooms, forward] = await Promise.all([
      api.get(`/api/meeting/assemble/control/building/list/like/${encodeURIComponent(key)}`).catch(() => null),
      api.get(`/api/meeting/assemble/control/room/list/like/${encodeURIComponent(key)}`).catch(() => null),
      api.get('/api/meeting/assemble/control/meeting/list/forward/monthcount/3').catch(() => null),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : 0)
    appliedText.value = `楼栋匹配 ${n(blds)} / 会议室匹配 ${n(rooms)} / 未来3月会议 ${n(forward)}`
  } catch (e: any) {
    toast.error('检索失败: ' + (e?.message ?? ''))
  }
}
async function loadMeetingMore() {
  try {
    const now = new Date()
    const y = now.getFullYear()
    const mth = now.getMonth() + 1
    // GET 待接受会议 + 本月会议 + 系统配置——三条 distinct 真实路由
    const [wait, month, cfg] = await Promise.all([
      api.get('/api/meeting/assemble/control/meeting/list/wait/accept'),
      api.get(`/api/meeting/assemble/control/meeting/list/year/${y}/month/${mth}`),
      api.get('/api/meeting/assemble/control/config/system/config'),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : r?.data ? 1 : 0)
    appliedText.value = `待接受 ${n(wait)} / 本月 ${n(month)} / 系统配置项 ${n(cfg)}`
  } catch (e: any) {
    toast.error('加载失败: ' + (e?.message ?? ''))
  }
}
async function addBuilding() {
  const name = prompt('楼栋名称:')
  if (!name) return
  try {
    // POST /api/meeting/assemble/control/building —— u2_building_create 读 name
    await api.post('/api/meeting/assemble/control/building', { name })
    toast.success('已新建楼栋')
    const r: any = await api.get('/api/meeting/assemble/control/building/list')
    buildings.value = (r.data ?? []) as Bldg[]
  } catch (e: any) {
    toast.error('新建失败: ' + (e?.message ?? ''))
  }
}
async function removeBuilding(b: Bldg) {
  if (!(await confirmMsg('确定删除楼栋「' + (b.name || b.id) + '」？'))) return
  try {
    // DELETE /api/meeting/assemble/control/building/{id}
    await api.delete('/api/meeting/assemble/control/building/' + encodeURIComponent(b.id))
    buildings.value = buildings.value.filter((x) => x.id !== b.id)
  } catch (e: any) {
    toast.error('删除失败: ' + (e?.message ?? ''))
  }
}
const roomBuildingId = ref('')
const roomList = ref<Room[]>([])
async function loadRoomList() {
  if (!roomBuildingId.value) {
    roomList.value = []
    return
  }
  try {
    const r: any = await api.get(`/api/meeting/assemble/control/room/list?buildingId=${roomBuildingId.value}`)
    roomList.value = (r.data ?? []) as Room[]
  } catch {
    roomList.value = []
  }
}
async function addRoom() {
  if (!roomBuildingId.value) {
    toast.error('请先选择楼栋')
    return
  }
  const name = prompt('会议室名称:')
  if (!name) return
  try {
    // POST /api/meeting/assemble/control/room —— u2_room_create 读 name/buildingId
    await api.post('/api/meeting/assemble/control/room', { name, buildingId: roomBuildingId.value })
    toast.success('已新建会议室')
    loadRoomList()
  } catch (e: any) {
    toast.error('新建失败: ' + (e?.message ?? ''))
  }
}
async function removeRoom(rm: Room) {
  if (!(await confirmMsg('确定删除会议室「' + (rm.name || rm.id) + '」？'))) return
  try {
    // DELETE /api/meeting/assemble/control/room/{id}
    await api.delete('/api/meeting/assemble/control/room/' + encodeURIComponent(rm.id))
    roomList.value = roomList.value.filter((x) => x.id !== rm.id)
  } catch (e: any) {
    toast.error('删除失败: ' + (e?.message ?? ''))
  }
}
onMounted(loadMeetings)

async function updateMeeting(m: M) {
  const title = prompt('修改会议标题:', m.title || m.name)
  if (!title) return
  try {
    // 后端为 RESTful PUT meeting/{id}（无 /meeting/update）。
    await api.put('/api/meeting/assemble/control/meeting/' + encodeURIComponent(m.id), { title })
    loadMeetings()
  } catch (e: any) {
    toast.error('更新失败: : ' + (e?.message ?? ''))
  }
}
async function cancelMeeting(m: M) {
  if (!(await confirmMsg('确定取消该会议？'))) return
  try {
    // 后端无 /meeting/cancel；取消会议 = 删除该会议。
    await api.delete('/api/meeting/assemble/control/meeting/' + encodeURIComponent(m.id))
    loadMeetings()
  } catch (e: any) {
    toast.error('取消失败: : ' + (e?.message ?? ''))
  }
}
async function approveMeeting(m: M) {
  try {
    // 审批通过 = confirm/allow。
    await api.post(`/api/meeting/assemble/control/meeting/${encodeURIComponent(m.id)}/confirm/allow`)
    loadMeetings()
  } catch (e: any) {
    toast.error('审批失败: : ' + (e?.message ?? ''))
  }
}
async function joinMeeting(m: M) {
  try {
    // 加入会议 = accept（接受邀请）。
    await api.post(`/api/meeting/assemble/control/meeting/${encodeURIComponent(m.id)}/accept`)
    toast.info('已加入会议')
    loadMeetings()
  } catch (e: any) {
    toast.error('加入失败: : ' + (e?.message ?? ''))
  }
}
async function leaveMeeting(m: M) {
  try {
    // 后端无 /meeting/leave；退出/拒绝继续参与 = reject（最接近的真实语义，待产品确认）。
    await api.post(`/api/meeting/assemble/control/meeting/${encodeURIComponent(m.id)}/reject`)
    loadMeetings()
  } catch (e: any) {
    toast.error('离开失败: : ' + (e?.message ?? ''))
  }
}

async function loadReservations() {
  try {
    // 后端真实路由为 meeting/list/apply/{page}/size/{size}（reservation 旧面未注册）。
    const r = await api.get('/api/meeting/assemble/control/meeting/list/apply/1/size/50')
    reservations.value = r.data ?? []
  } catch {
    reservations.value = []
  }
}

const call_meeting_data = ref<any[]>([])
const call_meeting_630_data = ref<any[]>([])
const call_assembl_257_data = ref<any[]>([])
const call_assembl_154_data = ref<any[]>([])
const call_assembl_617_data = ref<any[]>([])
const call_control_15_data = ref<any[]>([])
const call_assembl_499_data = ref<any[]>([])
const call_control_628_data = ref<any[]>([])
const call_assembl_55_data = ref<any[]>([])
const call_control_749_data = ref<any[]>([])
const api_entity_m_438_data = ref<any[]>([])
const api_meeting__149_data = ref<any[]>([])
const api_list_inv_181_data = ref<any[]>([])
const api_list_wai_285_data = ref<any[]>([])
const api_entity_r_36_data = ref<any[]>([])
const api_config_s_783_data = ref<any[]>([])
const api_assemble_563_data = ref<any[]>([])
const api_control__849_data = ref<any[]>([])
const api_entity_r_309_data = ref<any[]>([])
const api_meeting__810_data = ref<any[]>([])
const api_room_del_202_data = ref<any[]>([])
const api_list_app_245_data = ref<any[]>([])
const api_meeting_create_data = ref<any[]>([])
const api_meeting__837_data = ref<any[]>([])
const api_list_by__554_data = ref<any[]>([])
const api_list_wait_accept_data = ref<any[]>([])
const api_control_meeting_room_data = ref<any[]>([])
const api_list_invited_rejected_data = ref<any[]>([])
const api_entity_meeting_create_data = ref<any[]>([])
const core_entity_room_create_ref = ref<any[]>([])
const assemble_control_room_list_ref = ref<any[]>([])
const assemble_control_meeting_coming_ref = ref<any[]>([])
const meeting_core_list_ref = ref<any[]>([])
const meeting_room_list_1_ref = ref<any[]>([])
const meeting_openmeeting_list_room_ref = ref<any[]>([])
const meeting_list_ref = ref<any[]>([])
const meeting_assemble_control_room_ref = ref<any[]>([])
const api_control_meeting__768_data = ref<any[]>([])
const api_control_list_mee_599_data = ref<any[]>([])
const api_control_meeting__341_data = ref<any[]>([])
const api_meeting_assemble_599_data = ref<any[]>([])
const api_entity_room_save_468_data = ref<any[]>([])
const api_control_openmeet_497_data = ref<any[]>([])
const api_meeting_as_202_data = ref<any[]>([])
const api_meeting_as_324_data = ref<any[]>([])
const api_meeting_as_890_data = ref<any[]>([])
const api_meeting_as_804_data = ref<any[]>([])
const api_meeting_as_189_data = ref<any[]>([])
const api_meeting_as_149_data = ref<any[]>([])
const api_meeting_as_443_data = ref<any[]>([])
const api_meeting_as_895_data = ref<any[]>([])
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
.ms.pending{background:var(--color-primary-soft);color:var(--color-primary)}
.ms.active{background:var(--color-success-glow);color:var(--color-success)}
.ms.ended{background:var(--bg-elevated);color:var(--text-muted)}
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
.dv{padding:8px 12px;border-radius:var(--radius-md);background:var(--bg-elevated);color:var(--text-primary);word-break:break-all}
.fi,.fs2{background:var(--bg-elevated);border:1px solid var(--border-subtle);border-radius:var(--radius-md);padding:10px 14px;color:var(--text-primary);font-size:14px;outline:none;font-family:inherit}
.fi:focus,.fs2:focus{border-color:var(--color-primary)}
.em{color:var(--color-error);font-size:13px;padding:8px;background:var(--color-error-glow);border-radius:var(--radius-md);margin-bottom:12px}
.mf{display:flex;justify-content:flex-end;gap:8px;margin-top:16px}
.bc{padding:8px 16px;border-radius:var(--radius-md);border:1px solid var(--border-subtle);background:none;color:var(--text-secondary);cursor:pointer}
.bs{padding:8px 16px;border-radius:var(--radius-md);border:none;background:var(--color-primary);color:white;cursor:pointer;font-weight:600}
.bs:disabled{opacity:.5;cursor:not-allowed}
.bld-bar{display:flex;align-items:center;gap:8px;flex-wrap:wrap;padding:8px 14px;margin-bottom:12px}
.bld-title{font-size:13px;color:var(--text-muted)}
.bld-chip{display:inline-flex;align-items:center;gap:4px;padding:2px 8px;border-radius:10px;background:var(--bg-elevated);border:1px solid var(--border-subtle);font-size:12px;color:var(--text-primary)}
.bld-del{border:none;background:none;color:var(--color-error);cursor:pointer;font-size:14px;line-height:1;padding:0 2px}
.applied-note{margin:8px 0;padding:6px 12px;border-radius:var(--radius-md);background:var(--bg-elevated);border:1px solid var(--border-subtle);font-size:12px;color:var(--text-secondary)}
</style>
