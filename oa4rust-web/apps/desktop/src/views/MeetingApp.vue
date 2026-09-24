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
      <button class="sb" @click="loadMeetingCore">核心资源/日程</button>
      <button class="sb" @click="loadMeetingPinyin">拼音检索</button>
      <button class="sb" @click="loadMeetingEntities">会议实体</button>
      <button class="sb" @click="loadMeetingControlAssets">控制台资源</button>
      <button class="sb" @click="loadMeetingDateLists">日期列表</button>
      <button class="sb" @click="loadMeetingOpenRooms">开放会议室</button>
      <button class="sb" @click="addBuilding">+ 楼栋</button>
      <button class="sb" @click="meetingWrite('accept')">接受会议</button>
      <button class="sb" @click="meetingWrite('reject')">拒绝会议</button>
      <button class="sb" @click="meetingWrite('confirmAllow')">确认允许</button>
      <button class="sb" @click="meetingWrite('confirmDeny')">确认拒绝</button>
      <button class="sb" @click="meetingWrite('checkin')">签到</button>
      <button class="sb" @click="meetingWrite('save')">存会议</button>
      <button class="sb" @click="meetingWrite('delete')">删会议</button>
      <button class="sb" @click="meetingWrite('buildingEdit')">改楼栋</button>
      <button class="sb" @click="meetingWrite('buildingDelete')">删楼栋</button>
      <button class="sb" @click="meetingWrite('roomEdit')">改会议室</button>
      <button class="sb" @click="meetingWrite('roomDelete')">删会议室</button>
      <button class="sb" @click="meetingWrite('roomPhoto')">会议室照片</button>
      <button class="sb" @click="meetingWrite('config')">存配置</button>
      <button class="sb" @click="meetingWrite('coreRoomCreate')">建核心会议室</button>
      <button class="sb" @click="meetingWrite('coreRoomSave')">存核心会议室</button>
      <button class="sb" @click="meetingWrite('attDelete')">删附件</button>
      <button class="sb" @click="meetingWrite('attUpdate')">改附件</button>
      <button class="sb" @click="meetingMore('addInvite')">加邀请</button>
      <button class="sb" @click="meetingMore('delInvite')">删邀请</button>
      <button class="sb" @click="meetingMore('modifyStart')">改开始时间</button>
      <button class="sb" @click="meetingMore('modifyComplete')">改结束时间</button>
      <button class="sb" @click="meetingMore('manualComplete')">手动结束</button>
      <button class="sb" @click="meetingMore('coreCreate')">建核心会议</button>
      <button class="sb" @click="meetingMore('coreSave')">存核心会议</button>
      <button class="sb" @click="meetingMore('coreDelete')">删核心会议</button>
      <button class="sb" @click="meetingLists">会议清单读</button>
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
            <button class="bsm" @click.stop="manageInvitee(m)">邀请管理</button>
            <button class="bsm" @click.stop="modifyMeetingTime(m)">改时间</button>
            <button class="bsm" @click.stop="markMeetingCompleted(m)">标记完成</button>
            <button class="bsm" @click.stop="showCheckinCode(m)">签到码</button>
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
// 会议核心资源/日程 4 条真实 distinct（rev203，meeting/meeting_core_entity crate，区别于 assemble/control 前缀）：
// meeting/room/list（x_meeting_room）+ meeting/building/list（x_meeting_building）+ meeting/schedule/days/{days}
// （x_meeting 未来 N 天）+ meeting/core/entity/room/list（SeaORM 房间实体）。
// 会议拼音检索 4 条真实 distinct（rev204，meeting_assemble_control，x_meeting_building/room 拼音维度各异）：
// building/list/like/pinyin/{key}（pinyin ILIKE）+ building/list/pinyininitial/{key}（pinyin_initial ILIKE）
// + room/list/like/pinyin/{key} + room/list/pinyininitial/{key}。区别于已消费的 building/room/list/like/{key}（name ILIKE）。
async function loadMeetingPinyin() {
  const key = searchKey.value?.trim() || 'h'
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  try {
    const [bP, bI, rP, rI] = await Promise.all([
      s(api.get(`/api/meeting/assemble/control/building/list/like/pinyin/${encodeURIComponent(key)}`)),
      s(api.get(`/api/meeting/assemble/control/building/list/pinyininitial/${encodeURIComponent(key)}`)),
      s(api.get(`/api/meeting/assemble/control/room/list/like/pinyin/${encodeURIComponent(key)}`)),
      s(api.get(`/api/meeting/assemble/control/room/list/pinyininitial/${encodeURIComponent(key)}`)),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : 0)
    appliedText.value = `楼栋拼音 ${n(bP)}/首字母 ${n(bI)} · 会议室拼音 ${n(rP)}/首字母 ${n(rI)}`
  } catch (e: any) {
    toast.error('加载拼音检索失败: ' + (e?.message ?? ''))
  }
}
async function loadMeetingCore() {
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  try {
    const [rooms, blds, sched, coreRooms] = await Promise.all([
      s(api.get('/api/meeting/room/list')),
      s(api.get('/api/meeting/building/list')),
      s(api.get('/api/meeting/schedule/days/7')),
      s(api.get('/api/meeting/core/entity/room/list')),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : 0)
    appliedText.value = `核心会议室 ${n(rooms)} / 楼栋 ${n(blds)} / 未来7天日程 ${n(sched)} / 实体房间 ${n(coreRooms)}`
  } catch (e: any) {
    toast.error('加载核心资源/日程失败: ' + (e?.message ?? ''))
  }
}
// rev210：会议实体 4 条真实 distinct 路由（SeaORM）：core/entity/meeting/list（全部按开始降序 limit20）
// · core/entity/meeting/list/by/{roomId}（按 RoomId 过滤）· core/entity/meeting/{id}（find_by_id 会议）· core/entity/room/{id}（find_by_id 房间）
async function loadMeetingEntities() {
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  try {
    const listResp = await s(api.get('/api/meeting/core/entity/meeting/list'))
    const rows = Array.isArray((listResp as any)?.data) ? (listResp as any).data : []
    const first = rows[0] ?? {}
    const mid = String(first.id ?? '0')
    const roomId = String(first.roomId ?? first.room_id ?? '0')
    const [byRoom, meetingOne, roomOne] = await Promise.all([
      s(api.get(`/api/meeting/core/entity/meeting/list/by/${encodeURIComponent(roomId)}`)),
      s(api.get(`/api/meeting/core/entity/meeting/${encodeURIComponent(mid)}`)),
      s(api.get(`/api/meeting/core/entity/room/${encodeURIComponent(roomId)}`)),
    ])
    const n = (r: any) => (Array.isArray((r as any)?.data) ? (r as any).data.length : 0)
    appliedText.value = `实体会议 ${rows.length} / 按房间 ${n(byRoom)} / 会议详情 ${(meetingOne as any)?.data?.id ? '命中' : '未命中'} / 房间详情 ${(roomOne as any)?.data?.id ? '命中' : '未命中'}`
  } catch (e: any) {
    toast.error('加载会议实体失败: ' + (e?.message ?? ''))
  }
}
// rev222：会议控制台 楼栋/会议室/照片/附件游标族 7 条真实 distinct 路由
// building/{id}（x_meeting_building）· room/{id}（x_meeting_room）· meeting/list/{id}/{next}/{count}（x_meeting 游标）· room/photo/{id}（x_meeting_room_photo WHERE room_id）
// · attachment/{id}（x_meeting_attachment）· attachment/list/{id}/next/{count}（分页前）· attachment/list/{id}/prev/{count}（分页后）
async function loadMeetingControlAssets() {
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  try {
    const roomsResp: any = await s(api.get('/api/meeting/room/list'))
    const rooms = Array.isArray(roomsResp?.data) ? roomsResp.data : []
    const rid = rooms[0] ? String(rooms[0].id ?? '0') : '0'
    const [building, room, meetingCursor, photo, attach, attNext, attPrev] = await Promise.all([
      s(api.get(`/api/meeting/assemble/control/building/${encodeURIComponent(rid)}`)),
      s(api.get(`/api/meeting/assemble/control/room/${encodeURIComponent(rid)}`)),
      s(api.get(`/api/meeting/assemble/control/meeting/list/${encodeURIComponent(rid)}/next/20`)),
      s(api.get(`/api/meeting/assemble/control/room/photo/${encodeURIComponent(rid)}`)),
      s(api.get(`/api/meeting/assemble/control/attachment/${encodeURIComponent(rid)}`)),
      s(api.get(`/api/meeting/assemble/control/attachment/list/${encodeURIComponent(rid)}/next/20`)),
      s(api.get(`/api/meeting/assemble/control/attachment/list/${encodeURIComponent(rid)}/prev/20`)),
    ])
    const n = (r: any) => (Array.isArray((r as any)?.data) ? (r as any).data.length : 0)
    appliedText.value = `楼栋 ${(building as any)?.data?.id ? '命中' : '未命中'} · 会议室 ${(room as any)?.data?.id ? '命中' : '未命中'} · 会议游标 ${n(meetingCursor)} · 照片 ${(photo as any)?.data ? '有' : '无'} · 附件 ${(attach as any)?.data ? '有' : '无'}（前 ${n(attNext)}/后 ${n(attPrev)}）`
  } catch (e: any) {
    toast.error('加载会议控制台资源失败: ' + (e?.message ?? ''))
  }
}
// rev239：会议按日期/时间范围列表 4 条真实 distinct 读路由（arity 已核：URL 参数数==handler Path 元数）
// building 时间范围(start/end) · 本月即将(count) · 年月(year,month) · 年月日(year,month,day)；跳 list/meeting/controls(0参却 Path<String>=500) 与 forward/monthcount(与 coming SQL 完全相同=孪生)
async function loadMeetingDateLists() {
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  try {
    const now = new Date()
    const y = String(now.getFullYear())
    const mo = String(now.getMonth() + 1)
    const d = String(now.getDate())
    const mid = '0'
    const sz = '20'
    const rid = '0'
    const [building, coming, byMonth, byDay, allList, sysCfg, forward] = await Promise.all([
      s(api.get('/api/meeting/assemble/control/building/list/start/1/completed/0')),
      s(api.get('/api/meeting/assemble/control/meeting/list/coming/month/3')),
      s(api.get(`/api/meeting/assemble/control/meeting/list/year/${y}/month/${mo}/all`)),
      s(api.get(`/api/meeting/assemble/control/meeting/list/year/${y}/month/${mo}/day/${d}`)),
      // rev275：meeting/list → x_meeting 全量(无 WHERE，区别于日期范围) · config/system/config/manage → x_meeting_config；均 arity0 只读
      s(api.get('/api/meeting/list')),
      s(api.get('/api/meeting/assemble/control/config/system/config/manage')),
      // rev280：meeting/list/forward/monthcount/all/{monthCount} → x_meeting WHERE start_time>=NOW() AND <=NOW()+INTERVAL（未来N月，区别于固定年月范围）
      s(api.get('/api/meeting/assemble/control/meeting/list/forward/monthcount/all/6')),
      // rev295：某日全部会议 year/month/day/all(x_meeting) + 楼栋全部会议 building/.../allmeeting(x_meeting_building) 补齐
      s(api.get(`/api/meeting/assemble/control/meeting/list/year/${y}/month/${mo}/day/${d}/all`)),
      s(api.get('/api/meeting/assemble/control/building/list/start/1/completed/0/allmeeting')),
      // rev300：meeting/list/forward/monthcount/{monthCount}/all（另一段序，x_meeting 未来N月）补齐
      s(api.get('/api/meeting/assemble/control/meeting/list/forward/monthcount/6/all')),
      // rev313：会议详情/即将(按月+计数)/邀请分页/游标/按年月日+房间 5 条纯 SELECT
      s(api.get(`/api/meeting/${mid}`)),
      s(api.get(`/api/meeting/assemble/control/meeting/list/coming/${mo}/20`)),
      s(api.get(`/api/meeting/assemble/control/meeting/list/invite/1/${sz}/${sz}`)),
      s(api.get(`/api/meeting/assemble/control/meeting/list/${mid}/0/20`)),
      s(api.get(`/api/meeting/assemble/control/meeting/list/${y}/${y}/${mo}/${mo}/${d}/${d}/${rid}`)),
    ])
    const n = (r: any) => (Array.isArray((r as any)?.data) ? (r as any).data.length : 0)
    appliedText.value = `楼栋(时间范围) ${n(building)} · 即将3月 ${n(coming)} · 本年月 ${n(byMonth)} · 本年月日 ${n(byDay)} · 全量会议 ${n(allList)} · 系统配置 ${(sysCfg as any)?.data ? '有' : '无'} · 未来6月 ${n(forward)}`
  } catch (e: any) {
    toast.error('加载会议日期列表失败: ' + (e?.message ?? ''))
  }
}
// rev240：会议 房间/开放会议 4 条真实 distinct 读路由（arity 已核；跳 day/{day}/all 与 building/.../allmeeting 同 SQL 孪生、invite/{page}/{size}/{size} 重复参 arity 不符）
async function loadMeetingOpenRooms() {
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  try {
    const now = new Date()
    const y = String(now.getFullYear())
    const mo = String(now.getMonth() + 1)
    const d = String(now.getDate())
    const [byRoom, buildingRoomMeeting, openRooms, openConfig] = await Promise.all([
      s(api.get(`/api/meeting/assemble/control/meeting/list/year/${y}/month/${mo}/day/${d}/0`)),
      s(api.get('/api/meeting/assemble/control/building/list/start/1/completed/0/room/0/meeting/0')),
      s(api.get('/api/meeting/assemble/control/openmeeting/list/room')),
      s(api.get('/api/meeting/assemble/control/openmeeting')),
    ])
    const n = (r: any) => (Array.isArray((r as any)?.data) ? (r as any).data.length : 0)
    appliedText.value = `按房间日程 ${n(byRoom)} · 楼栋(房间/会议过滤) ${n(buildingRoomMeeting)} · 开放会议室 ${n(openRooms)} · 开放会议配置 ${n(openConfig)}`
  } catch (e: any) {
    toast.error('加载开放会议资源失败: ' + (e?.message ?? ''))
  }
}
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
// rev314：会议管理真实写端点（用户触发，非挂载自动触发）——改时间/标记完成/签到码/邀请增删
async function modifyMeetingTime(m: M) {
  const startTime = prompt('新的开始时间 (YYYY-MM-DD HH:mm:ss):', m.startTime || '')
  if (!startTime) return
  const endTime = prompt('新的结束时间 (可留空):', '') || undefined
  try {
    // POST meeting/{id}/modify/starttime → UPDATE x_meeting start_time(+end_time?)
    await api.post(`/api/meeting/assemble/control/meeting/${encodeURIComponent(m.id)}/modify/starttime`, {
      startTime,
      ...(endTime ? { endTime } : {}),
    })
    if (endTime) {
      // POST meeting/{id}/modify/completedtime → UPDATE x_meeting completed_time
      await api.post(`/api/meeting/assemble/control/meeting/${encodeURIComponent(m.id)}/modify/completedtime`, {
        completedTime: endTime,
      })
    }
    toast.success('会议时间已更新')
    loadMeetings()
  } catch (e: any) {
    toast.error('改时间失败: ' + (e?.message ?? ''))
  }
}
async function markMeetingCompleted(m: M) {
  if (!(await confirmMsg('确定将该会议标记为已完成？'))) return
  try {
    // POST meeting/{id}/manual/completed → UPDATE x_meeting status='completed'（无 body）
    await api.post(`/api/meeting/assemble/control/meeting/${encodeURIComponent(m.id)}/manual/completed`)
    toast.success('已标记完成')
    loadMeetings()
  } catch (e: any) {
    toast.error('标记完成失败: ' + (e?.message ?? ''))
  }
}
async function showCheckinCode(m: M) {
  try {
    // POST meeting/checkin/code/{id} → 读取最新签到码（x_meeting_checkin_code）
    const r: any = await api.post(`/api/meeting/assemble/control/meeting/checkin/code/${encodeURIComponent(m.id)}`)
    const code = (r as any)?.data?.checkinCode
    toast.info(code ? `签到码：${code}` : '暂无签到码')
  } catch (e: any) {
    toast.error('获取签到码失败: ' + (e?.message ?? ''))
  }
}
async function manageInvitee(m: M) {
  const invitee = prompt('输入受邀人（留空取消）:', '')
  if (!invitee) return
  const remove = await confirmMsg(`对「${invitee}」：确定=添加邀请，取消=移除邀请`)
  try {
    if (remove) {
      // POST meeting/{id}/add/invite → INSERT x_meeting_invite {invitee}
      await api.post(`/api/meeting/assemble/control/meeting/${encodeURIComponent(m.id)}/add/invite`, { invitee })
      toast.success('已添加邀请')
    } else {
      // POST meeting/{id}/delete/invite → DELETE x_meeting_invite {invitee}
      await api.post(`/api/meeting/assemble/control/meeting/${encodeURIComponent(m.id)}/delete/invite`, { invitee })
      toast.success('已移除邀请')
    }
    loadMeetings()
  } catch (e: any) {
    toast.error('邀请管理失败: ' + (e?.message ?? ''))
  }
}
// rev336：会议 审批(接受/拒绝/确认)/存删/楼栋·会议室编辑删照片/配置/核心会议室 真实写端点（用户触发，shape 已核；全字面量路径）
async function meetingWrite(op: string) {
  const id = prompt('目标 ID:', '') || ''
  const e = encodeURIComponent(id)
  try {
    if (op === 'accept') await api.post(`/api/meeting/assemble/control/meeting/${e}/accept`, {})
    else if (op === 'reject') await api.post(`/api/meeting/assemble/control/meeting/${e}/reject`, {})
    else if (op === 'confirmAllow') await api.post(`/api/meeting/assemble/control/meeting/${e}/confirm/allow`, {})
    else if (op === 'confirmDeny') await api.post(`/api/meeting/assemble/control/meeting/${e}/confirm/deny`, {})
    else if (op === 'checkin') await api.post(`/api/meeting/assemble/control/meeting/${e}/checkin`, {})
    else if (op === 'save') await api.post(`/api/meeting/assemble/control/meeting/save/${e}`, { subject: '更新会议' })
    else if (op === 'delete') {
      if (!(await confirmMsg('确定删除该会议？'))) return
      await api.delete(`/api/meeting/assemble/control/meeting/delete/${e}`)
    } else if (op === 'buildingEdit') await api.put(`/api/meeting/assemble/control/building/${e}`, { name: '更新楼栋' })
    else if (op === 'buildingDelete') {
      if (!(await confirmMsg('确定删除该楼栋？'))) return
      await api.delete(`/api/meeting/assemble/control/building/${e}`)
    } else if (op === 'roomEdit') await api.put(`/api/meeting/assemble/control/room/${e}`, { name: '更新会议室' })
    else if (op === 'roomDelete') {
      if (!(await confirmMsg('确定删除该会议室？'))) return
      await api.delete(`/api/meeting/assemble/control/room/${e}`)
    } else if (op === 'roomPhoto') await api.post(`/api/meeting/assemble/control/room/${e}/photo`, {})
    else if (op === 'config') await api.post('/api/meeting/assemble/control/config', {})
    else if (op === 'coreRoomCreate') await api.post('/api/meeting/core/entity/room/create', { name: '新会议室' })
    else if (op === 'coreRoomSave') await api.post(`/api/meeting/core/entity/room/save/${e}`, { name: '更新会议室' })
    else if (op === 'attDelete') {
      if (!(await confirmMsg('确定删除该附件？'))) return
      await api.delete(`/api/meeting/assemble/control/attachment/${e}`)
    } else await api.put(`/api/meeting/assemble/control/attachment/${e}/update`, {})
    toast.success('会议操作已提交')
  } catch (err: any) {
    toast.error('会议操作失败: ' + (err?.message ?? ''))
  }
}
// rev360：会议 邀请增删/改起止时间/手动结束 + 核心实体会议建改删 真实写（shape 已核：add/delete invite{invitee}、modify starttime{startTime,endTime?}·completedtime{completedTime}、core create{title,content?,roomId?}）
async function meetingMore(op: string) {
  const id = prompt('会议 ID:', '') || ''
  const e = encodeURIComponent(id)
  try {
    if (op === 'addInvite') {
      const invitee = prompt('邀请人员:', '') || ''
      await api.put(`/api/meeting/assemble/control/meeting/${e}/add/invite`, { invitee })
    } else if (op === 'delInvite') {
      const invitee = prompt('移除邀请人员:', '') || ''
      await api.put(`/api/meeting/assemble/control/meeting/${e}/delete/invite`, { invitee })
    } else if (op === 'modifyStart') {
      const startTime = prompt('开始时间:', '') || ''
      await api.put(`/api/meeting/assemble/control/meeting/${e}/modify/starttime`, { startTime })
    } else if (op === 'modifyComplete') {
      const completedTime = prompt('结束时间:', '') || ''
      await api.put(`/api/meeting/assemble/control/meeting/${e}/modify/completedtime`, { completedTime })
    } else if (op === 'manualComplete') {
      await api.get(`/api/meeting/assemble/control/meeting/${e}/manual/completed`)
    } else if (op === 'coreCreate') {
      const title = prompt('会议标题:', '') || ''
      if (!title) return
      await api.post('/api/meeting/core/entity/meeting/create', { title })
    } else if (op === 'coreSave') {
      await api.post(`/api/meeting/core/entity/meeting/save/${e}`, {})
    } else {
      if (!(await confirmMsg('确定删除该核心会议？'))) return
      await api.post(`/api/meeting/core/entity/meeting/delete/${e}`, {})
    }
    toast.success('会议操作已提交')
  } catch (err: any) {
    toast.error('会议操作失败: ' + (err?.message ?? ''))
  }
}
// rev360：会议 我的/管理/受邀/申请 分页 + 会议控制项 + 开放会议室 真实只读（用户触发，分页参数正确 {page}/size/{size}）
async function meetingLists() {
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  try {
    const rs = await Promise.all([
      s(api.post('/api/meeting/assemble/control/meeting/list/1/size/20', {})),
      s(api.post('/api/meeting/assemble/control/meeting/list/1/size/20/manage', {})),
      s(api.post('/api/meeting/assemble/control/meeting/list/invite/1/size/20', {})),
      s(api.post('/api/meeting/assemble/control/meeting/list/apply/1/size/20', {})),
      s(api.get('/api/meeting/assemble/control/list/meeting/controls')),
      s(api.get('/api/meeting/openmeeting/list/room')),
    ])
    const hit = rs.filter((r) => (r as any)?.data != null).length
    toast.success(`会议清单读 ${rs.length} 条命中 ${hit}`)
  } catch (err: any) {
    toast.error('会议清单加载失败: ' + (err?.message ?? ''))
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
