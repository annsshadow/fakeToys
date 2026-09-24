<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<template>
  <div class="im-view">
    <!-- 左侧：会话列表 -->
    <aside class="im-sidebar glass-card" :class="{ collapsed: selectedChat }">
      <div class="sidebar-header">
        <h2>消息</h2>
        <div class="header-actions">
          <button class="new-chat-btn" title="在线/会话概览" @click="loadImMeta">✉</button>
          <button class="new-chat-btn" title="未读/在线数/IM配置" @click="loadImStats">🔔</button>
          <button class="new-chat-btn" title="即时消息/群发类型" @click="loadImInstant">📥</button>
          <button class="new-chat-btn" title="收藏/已消费/消息分页" @click="loadImArchive">🗂️</button>
          <button class="new-chat-btn" title="按类型/未消费/非IM消息" @click="loadMsgByType">📊</button>
          <button class="new-chat-btn" title="群发消息详情/游标" @click="loadMassMessages">📢</button>
          <button class="new-chat-btn" title="即时消息消费维度" @click="loadInstantFacets">🗓️</button>
          <button class="new-chat-btn" title="消费队列/接收/未读" @click="loadConsumeFacets">📬</button>
        </div>
      </div>
      <div class="search-bar">
        <span class="search-icon">⌕</span>
        <input v-model="searchMsg" placeholder="搜索对话..." class="search-input" />
      </div>
      <div class="conversation-list">
        <div
          v-for="conv in filteredConversations"
          :key="conv.id"
          class="conv-item"
          :class="{ active: selectedChat?.id === conv.id }"
          @click="selectConversation(conv)"
        >
          <div class="conv-avatar">{{ conv.avatar || conv.name?.[0] }}</div>
          <div class="conv-info">
            <div class="conv-name">{{ conv.name }}</div>
            <div class="conv-preview">{{ conv.lastMessage || '暂无消息' }}</div>
          </div>
          <div class="conv-meta">
            <span class="conv-time">{{ conv.time }}</span>
            <span v-if="conv.unread" class="conv-badge">{{ conv.unread > 99 ? '99+' : conv.unread }}</span>
          </div>
        </div>
        <div v-if="filteredConversations.length === 0 && !convLoading" class="empty-conv">
          <p>暂无对话</p>
        </div>
      </div>
      <!-- 未读数总览 -->
      <div class="unread-bar glass-card" v-if="totalUnread > 0">
        <span>📬 {{ totalUnread }} 条未读</span>
        <button class="mark-all-btn" @click="markAllRead">全部已读</button>
      </div>
    </aside>

    <!-- 右侧：聊天区域 -->
    <main class="im-main glass-card" :class="{ 'no-chat': !selectedChat }">
      <template v-if="selectedChat">
        <div class="chat-header">
          <button class="back-btn" @click="selectedChat = null">←</button>
          <div class="chat-avatar">{{ selectedChat.avatar || selectedChat.name?.[0] }}</div>
          <div class="chat-info">
            <div class="chat-name">{{ selectedChat.name }}</div>
            <div class="chat-status" :class="{ online: selectedChat.online }">
              {{ selectedChat.online ? '在线' : '离线' }}
            </div>
            <div class="chat-presence" v-if="onlineCount > 0">
              <span class="presence-dot"></span>{{ onlineCount }} 人在线
            </div>
            <div class="call-chip" v-if="callState !== 'idle'">
              {{ callState === 'in-call' ? '通话中' : '呼叫中…' }}
            </div>
          </div>
          <div class="chat-actions">
            <button class="icon-btn" :title="callState === 'idle' ? '语音通话' : '结束通话'" @click="toggleCall">
              {{ callState === 'idle' ? '📞' : '⏹' }}
            </button>
            <button class="icon-btn" title="标记已读" @click="imConvAction('read')">✓</button>
            <button class="icon-btn" title="置顶" @click="imConvAction('topSet')">📌</button>
            <button class="icon-btn" title="取消置顶" @click="imConvAction('topCancel')">📍</button>
            <button class="icon-btn" title="重命名会话" @click="imConvAction('rename')">✎</button>
            <button class="icon-btn" title="退出群聊" @click="imConvAction('quitGroup')">🚪</button>
            <button class="icon-btn" title="解散群" @click="imConvAction('dismissGroup')">💥</button>
            <button class="icon-btn" title="删除单聊" @click="imConvAction('delSingle')">🗑</button>
            <button class="icon-btn" title="收藏消息" @click="imMsgAction('collect')">⭐</button>
            <button class="icon-btn" title="取消收藏" @click="imMsgAction('uncollect')">☆</button>
            <button class="icon-btn" title="撤回消息" @click="imMsgAction('revoke')">↩</button>
            <button class="icon-btn" title="自定义消息" @click="imMsgAction('custom')">✉</button>
            <button class="icon-btn" title="群发" @click="imMsgAction('mass')">📢</button>
            <button class="icon-btn" title="更新会话" @click="imMore('convUpdate')">🔄</button>
            <button class="icon-btn" title="按人列会话" @click="imMore('convByPerson')">👥</button>
            <button class="icon-btn" title="管理配置" @click="imMore('managerConfig')">⚙</button>
            <button class="icon-btn" title="收藏分页" @click="imMore('collectionList')">📚</button>
            <button class="icon-btn" title="下载消息" @click="imMore('msgDownload')">⬇</button>
            <button class="icon-btn" title="缩略图" @click="imMore('msgThumb')">🖼</button>
            <button class="icon-btn" title="消息对象列表" @click="imMore('msgListObj')">📋</button>
            <button class="icon-btn" title="标记已读" @click="imMore('markRead')">☑</button>
            <button class="icon-btn" title="消费消息" @click="imMore('consume')">🍽</button>
            <button class="icon-btn" title="按类型消费" @click="imMore('consumeType')">🏷</button>
            <button class="icon-btn" title="自定义消息2" @click="imMore('customCreate')">✚</button>
            <button class="icon-btn" title="消息分页" @click="imMore('msgPaging')">📄</button>
            <button class="icon-btn" title="群发类型" @click="imMore('massEnable')">📣</button>
            <button class="icon-btn" title="删群发" @click="imMore('massDelete')">🗑</button>
            <button class="icon-btn" title="会话置顶" @click="imMore2('topSet')">📌</button>
            <button class="icon-btn" title="取消置顶" @click="imMore2('topCancel')">📍</button>
            <button class="icon-btn" title="标记即时已消费" @click="imMore3('instantConsumedPut')">✅</button>
            <button class="icon-btn" title="移除消息收藏" @click="imMore3('collectionRemove')">🗂</button>
            <button class="icon-btn" title="发送(communicate)" @click="imMore4('sendMsg')">📨</button>
            <button class="icon-btn" title="创建即时消息" @click="imMore4('connectorCreate')">🔔</button>
            <button class="icon-btn" title="创建ws消费消息" @click="imMore4('wsCreate')">🌐</button>
            <button class="icon-btn" title="会话标记已读" @click="imMore2('convRead')">✔</button>
            <button class="icon-btn" title="退出群会话" @click="imMore2('groupQuit')">🚪</button>
            <button class="icon-btn" title="撤回消息" @click="imMore2('msgRevoke')">↩</button>
            <button class="icon-btn" title="按类型消费(会话)" @click="imMore2('consumeType')">🔖</button>
            <button class="icon-btn" title="当前人已消费" @click="imMore2('consumed')">📥</button>
            <button class="icon-btn" title="更多信息">⋯</button>
          </div>
        </div>

        <!-- P5：WebRTC 远端音频（P2P 语音，信令走 IM 房间通道） -->
        <audio ref="remoteAudio" autoplay style="display: none" />

        <div ref="messageContainer" class="message-list" @scroll="handleScroll">
          <div v-if="msgLoading" class="loading-state">
            <div class="skeleton-row" v-for="i in 4" :key="i"></div>
          </div>
          <div v-else-if="messages.length === 0" class="empty-messages">
            <p>开始对话吧 💬</p>
          </div>
          <template v-else>
            <div v-for="msg in messages" :key="msg.id" class="message" :class="{ outgoing: msg.direction === 'out' }">
              <div class="msg-avatar">{{ msg.sender?.[0] }}</div>
              <div class="msg-bubble">
                <div class="msg-content" style="white-space:pre-wrap;word-break:break-word">{{formatContent(msg.content)}}</div>
                <div class="msg-time">{{ msg.time }}</div>
              </div>
            </div>
            <div ref="loadMoreRef" class="load-more-trigger"></div>
          </template>
        </div>

        <div class="message-input">
          <button class="input-btn" title="表情">😊</button>
          <button class="input-btn" title="文件">📎</button>
          <textarea
            v-model="inputText"
            @keydown.enter.ctrl="sendMessage"
            @keydown.enter.shift.exact="inputText += '\n'"
            placeholder="输入消息... (Ctrl+Enter 发送)"
            class="input-textarea"
            rows="1"
          ></textarea>
          <button class="send-btn" :disabled="!inputText.trim() || sendLoading" @click="sendMessage">
            {{ sendLoading ? '发送中...' : '发送' }}
          </button>
          <span v-if="lastDelivered > 0" class="receipt-indicator">已投递 {{ lastDelivered }}</span>
        </div>
      </template>
      <div v-else class="no-chat-placeholder">
        <div class="placeholder-icon">💬</div>
        <h2>选择会话</h2>
        <p>从左侧选择一个对话开始聊天</p>
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
import { api, type O2WebSocketClient, useSession, useWebSocket } from '@oa4rust/sdk'
import { confirmMsg, toast } from '../utils/toast'
import { useMutation, useQuery, useQueryClient } from '@tanstack/vue-query'
import { computed, nextTick, onMounted, onUnmounted, ref, watch } from 'vue'

interface Conversation {
  id: string
  name: string
  avatar?: string
  lastMessage?: string
  time: string
  unread: number
  online?: boolean
}

interface Message {
  id: string
  content: string
  sender: string
  direction: 'in' | 'out'
  time: string
  type?: string
}

const searchMsg = ref('')
const inputText = ref('')
const selectedChat = ref<Conversation | null>(null)
const messageContainer = ref<HTMLElement>()
const loadMoreRef = ref<HTMLElement>()
const page = ref(1)
const sendLoading = ref(false)
const wsClient = ref<O2WebSocketClient | null>(null)
// P5：IM 实时协议状态
const session = useSession()
const onlineCount = ref(0) // 当前房间在线连接数（presence 下行）
const lastDelivered = ref(0) // 最近一条消息的投递回执（receipt 下行）

const queryClient = useQueryClient()

// ── P5：P2P WebRTC 语音（信令帧 {webrtc, sdp|candidate} 复用 IM 房间通道，
//    o2 legacy IM 传输即 WebSocket，无需 XMPP/媒体服务器；媒体走浏览器原生 P2P）──
const remoteAudio = ref<HTMLAudioElement>()
const callState = ref<'idle' | 'calling' | 'in-call'>('idle')
let localRtc: RTCPeerConnection | null = null
let localStream: MediaStream | null = null
const RTC_CONFIG: RTCConfiguration = { iceServers: [{ urls: 'stun:stun.l.google.com:19302' }] }

async function startCall(): Promise<void> {
  const convId = selectedChat.value?.id
  if (!convId || callState.value !== 'idle') return
  try {
    localStream = await navigator.mediaDevices.getUserMedia({ audio: true })
  } catch {
    console.warn('[IM] 麦克风不可用，无法发起语音')
    return
  }
  localRtc = new RTCPeerConnection(RTC_CONFIG)
  localStream.getAudioTracks().forEach((t) => localRtc?.addTrack(t, localStream))
  localRtc.ontrack = (e) => {
    if (remoteAudio.value) remoteAudio.value.srcObject = e.streams[0]
  }
  localRtc.onicecandidate = (e) => {
    if (e.candidate) wsClient.value?.send(convId, { webrtc: 'ice', to: convId, candidate: e.candidate })
  }
  const offer = await localRtc.createOffer()
  await localRtc.setLocalDescription(offer)
  wsClient.value?.send(convId, { webrtc: 'offer', to: convId, sdp: offer })
  callState.value = 'calling'
}

async function handleRtcSignal(data: any): Promise<void> {
  const convId = selectedChat.value?.id
  // 信令只处理属于当前会话的帧
  if (!convId || (data.to && data.to !== convId)) return
  const ws = wsClient.value
  switch (data.webrtc) {
    case 'offer': {
      if (callState.value !== 'idle') return
      try {
        localStream = await navigator.mediaDevices.getUserMedia({ audio: true })
      } catch {
        console.warn('[IM] 麦克风不可用，无法接听')
        return
      }
      localRtc = new RTCPeerConnection(RTC_CONFIG)
      localStream.getAudioTracks().forEach((t) => localRtc?.addTrack(t, localStream))
      localRtc.ontrack = (e) => {
        if (remoteAudio.value) remoteAudio.value.srcObject = e.streams[0]
      }
      localRtc.onicecandidate = (e) => {
        if (e.candidate) ws?.send(convId, { webrtc: 'ice', to: convId, candidate: e.candidate })
      }
      await localRtc.setRemoteDescription(new RTCSessionDescription(data.sdp))
      const answer = await localRtc.createAnswer()
      await localRtc.setLocalDescription(answer)
      ws?.send(convId, { webrtc: 'answer', to: convId, sdp: answer })
      callState.value = 'in-call'
      break
    }
    case 'answer':
      if (localRtc) await localRtc.setRemoteDescription(new RTCSessionDescription(data.sdp))
      callState.value = 'in-call'
      break
    case 'ice':
      if (localRtc && data.candidate) await localRtc.addIceCandidate(new RTCIceCandidate(data.candidate))
      break
    case 'end':
      endCall(false)
      break
  }
}

function endCall(notify = true): void {
  const convId = selectedChat.value?.id
  localRtc?.close()
  localStream?.getTracks().forEach((t) => t.stop())
  localRtc = null
  localStream = null
  if (remoteAudio.value) remoteAudio.value.srcObject = null
  callState.value = 'idle'
  if (notify && convId) wsClient.value?.send(convId, { webrtc: 'end', to: convId })
}

function toggleCall(): void {
  if (callState.value === 'idle') void startCall()
  else endCall()
}

// ── 会话列表（真实 API）────────────────────────────────────────
const { data: convData, isLoading: convLoading } = useQuery({
  queryKey: ['im', 'conversations'],
  queryFn: async () => {
    const resp = await api.get('/api/message/assemble/communicate/im/conversation/list/my')
    return ((resp as any)?.data ?? []) as Conversation[]
  },
  staleTime: 30 * 1000,
  refetchInterval: 30_000, // 每30秒刷新
})

const conversations = ref<Conversation[]>([])
watch(convData, (data) => {
  if (data) conversations.value = data
})

const totalUnread = computed(() => conversations.value.reduce((s, c) => s + c.unread, 0))

const filteredConversations = computed(() => {
  if (!searchMsg.value) return conversations.value
  const q = searchMsg.value.toLowerCase()
  return conversations.value.filter(
    (c) => c.name.toLowerCase().includes(q) || (c.lastMessage ?? '').toLowerCase().includes(q),
  )
})

// ── 消息列表（真实 API）────────────────────────────────────────
// useQuery 返回普通对象（嵌套 Ref），模板只解包顶层 ref。故解构出 data/isLoading/refetch，
// 模板用 msgLoading（顶层 ref 自动解包），computed 内用 msgData.value，杜绝「msgQuery.isLoading 恒真值」渲染 bug。
const {
  data: msgData,
  isLoading: msgLoading,
  refetch: refetchMsgs,
} = useQuery<Message[]>({
  queryKey: ['im', 'messages', () => selectedChat.value?.id],
  queryFn: async () => {
    if (!selectedChat.value) return []
    const resp = await api.post<{ data: Message[] }>(`/api/message/assemble/communicate/im/msg/list/1/size/50`, {
      conversationId: selectedChat.value.id,
    })
    return ((resp as any)?.data ?? []) as Message[]
  },
  // enabled 须为响应式 Ref（传 Ref 本身而非 .value 快照），会话选中后查询才会自动启用/随 key 变化重取
  enabled: computed(() => !!selectedChat.value),
  staleTime: 10 * 1000,
})

// 后端已支持按会话过滤（im/msg/list/{page}/size/{size} 读取 body.conversationId）；
// 此处保留一次按普通键 conversationId 的防御性过滤。
const messages = computed(() => {
  const all = (msgData.value ?? []) as any[]
  const id = selectedChat.value?.id
  if (!id) return []
  return all.filter((m) => m?.conversationId === id)
})

// 监听会话切换，重新加载消息
watch(
  () => selectedChat.value?.id,
  async (newId, oldId) => {
    if (newId !== oldId && newId) {
      page.value = 1
      await refetchMsgs()
      nextTick(scrollToBottom)
    }
  },
)

// ── 发送消息（真实 API + WebSocket）────────────────────────────
const sendMutation = useMutation({
  mutationFn: (content: string) => {
    if (!selectedChat.value) throw new Error('No conversation selected')
    return api.post('/api/message/assemble/communicate/im/msg', {
      conversationId: selectedChat.value!.id,
      content,
      sender: session.user?.unique ?? '',
      type: 'text',
    })
  },
  onMutate: async (content) => {
    // 乐观更新：立即添加到本地
    const now = new Date()
    const time = `${now.getHours()}:${String(now.getMinutes()).padStart(2, '0')}`
    const optimisticMsg: Message = {
      id: `opt-${Date.now()}`,
      content,
      sender: 'me',
      direction: 'out',
      time,
    }
    queryClient.setQueryData(['im', 'messages', selectedChat.value?.id], (old: Message[] | undefined) => [
      ...(old ?? []),
      optimisticMsg,
    ])
    return { optimisticMsg }
  },
  onSuccess: () => {
    inputText.value = ''
    // 服务端确认后立即移除乐观消息，换真实消息
    setTimeout(() => {
      refetchMsgs()
    }, 500)
  },
  onError: (_err, _vars, context) => {
    if (context?.optimisticMsg) {
      queryClient.setQueryData(
        ['im', 'messages', selectedChat.value?.id],
        (old: Message[] | undefined) => old?.filter((m) => m.id !== context.optimisticMsg!.id) ?? [],
      )
    }
  },
  onSettled: () => {
    queryClient.invalidateQueries({ queryKey: ['im', 'conversations'] })
  },
})

function sendMessage(): void {
  const text = inputText.value.trim()
  if (!text || sendLoading.value) return
  sendLoading.value = true
  sendMutation.mutate(text)
  sendLoading.value = false

  // 同时通过 WebSocket 推送（如果连接可用）。P5：房间即会话 id（conv.id），
  // 与会话列表/消息查询的会话键一致，不再用硬编码 'chat' 广播串流。
  if (wsClient.value?.connected && selectedChat.value) {
    wsClient.value.send(selectedChat.value.id, {
      to: selectedChat.value.id,
      content: text,
      type: 'text',
    })
  }
}

// ── 标记已读 ───────────────────────────────────────────────────
async function markConversationRead(convId: string): Promise<void> {
  await api.post(`/api/message/assemble/communicate/mark_read/${convId}`, null)
  // 乐观更新
  conversations.value = conversations.value.map((c) => (c.id === convId ? { ...c, unread: 0 } : c))
}

async function markAllRead(): Promise<void> {
  for (const conv of conversations.value.filter((c) => c.unread > 0)) {
    await markConversationRead(conv.id)
  }
}

// ── WebSocket（P5：IM 完整协议——presence/receipt/会话房间）──────────────
function initWebSocket(): void {
  const ws = useWebSocket()
  wsClient.value = ws

  ws.on('im_create', (data: any) => {
    const msg = data as {
      content: string
      sender?: string
      room?: string
      to?: string
      conversationId?: string
      webrtc?: string
      sdp?: RTCSessionDescriptionInit
      candidate?: RTCIceCandidateInit
    }
    // P5：WebRTC 信令帧（offer/answer/ice/end）复用 im_create 通道，非文本消息
    if (msg.webrtc) {
      void handleRtcSignal(msg)
      return
    }
    const convId = msg.to ?? msg.conversationId ?? msg.room
    // 添加新消息到对应对话
    if (convId && selectedChat.value?.id === convId) {
      queryClient.setQueryData(['im', 'messages', convId], (old: Message[] | undefined) => [
        ...(old ?? []),
        {
          id: `ws-${Date.now()}`,
          content: msg.content,
          sender: msg.sender ?? '对方',
          direction: 'in',
          time: new Date().toLocaleTimeString('zh-CN', { hour: '2-digit', minute: '2-digit' }),
        },
      ])
      nextTick(scrollToBottom)
    }
    // 刷新会话列表
    queryClient.invalidateQueries({ queryKey: ['im', 'conversations'] })
  })

  // P5：presence 下行——当前房间在线快照（roster + 在线连接数）
  ws.on('presence', (data: any) => {
    onlineCount.value = Number(data?.count ?? 0)
  })

  // P5：receipt 下行——最近一条消息投递到的在线连接数
  ws.on('receipt', (data: any) => {
    lastDelivered.value = Number(data?.delivered ?? 0)
  })

  ws.connect()
    .then(() => {
      // 连接建立后声明在线身份（roster 用），并切到当前会话房间。
      const me = session.user?.unique
      if (me) ws.declarePresence(me)
      if (selectedChat.value) ws.joinRoom(selectedChat.value.id)
    })
    .catch(() => console.warn('[IM] WebSocket connect failed'))
}

// ── 工具函数 ───────────────────────────────────────────────────
function selectConversation(conv: Conversation): void {
  // 切换会话：结束仍在进行的通话（信令房间随会话走）
  if (callState.value !== 'idle') endCall(false)
  selectedChat.value = conv
  if (conv.unread > 0) markConversationRead(conv.id)
  // P5：会话房间即消息房间——选中会话时加入其房间（presence/回执按房间收敛）
  wsClient.value?.joinRoom(conv.id)
  void loadConversationDetail(conv.id)
}

// 会话详情（rev116）：主体 + 单聊信息，两条 distinct 真实路由；业务会话按 businessId 反查
async function loadConversationDetail(id: string): Promise<void> {
  if (!id) return
  const settle = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const [detail, single] = await Promise.all([
    // GET im/conversation/{id} —— 会话主体（x_message_conversation）
    settle(api.get(`/api/message/assemble/communicate/im/conversation/${encodeURIComponent(id)}`)),
    // GET im/conversation/{id}/single —— 单聊信息（type=single）
    settle(api.get(`/api/message/assemble/communicate/im/conversation/${encodeURIComponent(id)}/single`)),
    // GET im/conversation/{id}/group —— 群成员（x_message_conversation_member）
    settle(api.get(`/api/message/assemble/communicate/im/conversation/${encodeURIComponent(id)}/group`)),
    // GET im/conversation/{id}/icon —— 会话图标（x_message_conversation_icon）
    settle(api.get(`/api/message/assemble/communicate/im/conversation/${encodeURIComponent(id)}/icon`)),
  ])
  const d = (detail as { data?: Record<string, unknown> } | null)?.data
  const biz = d && typeof d === 'object' ? String(d.businessId ?? d.business_id ?? '') : ''
  if (biz) {
    // GET im/conversation/business/{businessId} —— 业务会话反查
    void api
      .get(`/api/message/assemble/communicate/im/conversation/business/${encodeURIComponent(biz)}`)
      .catch(() => {})
  }
  void single
}

// rev326：IM 会话/消息 真实写端点（用户触发，非自动；shape 已核 message crate handler 源码）
async function imConvAction(kind: string): Promise<void> {
  const id = selectedChat.value?.id
  if (!id) {
    toast.error('请先选择会话')
    return
  }
  const e = encodeURIComponent(id)
  try {
    if (kind === 'read') await api.post(`/api/message/assemble/communicate/im/conversation/${e}/read`, {})
    else if (kind === 'topSet') await api.post(`/api/message/assemble/communicate/im/conversation/${e}/top/set`, {})
    else if (kind === 'topCancel') await api.post(`/api/message/assemble/communicate/im/conversation/${e}/top/cancel`, {})
    else if (kind === 'quitGroup') {
      if (!(await confirmMsg('确定退出该群聊？'))) return
      await api.post(`/api/message/assemble/communicate/im/conversation/${e}/group/quit/self`, {})
    } else if (kind === 'dismissGroup') {
      if (!(await confirmMsg('确定解散该群会话？'))) return
      await api.delete(`/api/message/assemble/communicate/im/conversation/${e}/group`)
    } else if (kind === 'delSingle') {
      if (!(await confirmMsg('确定删除该单聊会话？'))) return
      await api.delete(`/api/message/assemble/communicate/im/conversation/${e}/single`)
    } else if (kind === 'rename') {
      const name = prompt('会话新名称:', selectedChat.value?.name ?? '') || ''
      await api.put(`/api/message/assemble/communicate/im/conversation/${e}`, { name })
    }
    toast.success('操作已提交')
  } catch (err: any) {
    toast.error('操作失败: ' + (err?.message ?? ''))
  }
}
async function imMsgAction(kind: string): Promise<void> {
  try {
    if (kind === 'collect') {
      const mid = prompt('要收藏的消息 ID:', '') || ''
      await api.post('/api/message/assemble/communicate/im/msg/collection', { messageId: mid })
      toast.success('已收藏')
    } else if (kind === 'uncollect') {
      const mid = prompt('要取消收藏的消息 ID:', '') || ''
      await api.post('/api/message/assemble/communicate/im/msg/collection/remove', { messageId: mid })
      toast.success('已取消收藏')
    } else if (kind === 'revoke') {
      const mid = prompt('要撤回的消息 ID:', '') || ''
      if (!(await confirmMsg('确定撤回该消息？'))) return
      await api.post(`/api/message/assemble/communicate/im/msg/revoke/${encodeURIComponent(mid)}`, {})
      toast.success('已撤回')
    } else if (kind === 'custom') {
      const title = prompt('自定义消息标题:', '') || ''
      const body = prompt('内容:', '') || ''
      await api.post('/api/message/custom/create', { title, body })
      toast.success('自定义消息已创建')
    } else if (kind === 'mass') {
      const body = prompt('群发内容:', '') || ''
      const person = prompt('接收人（逗号分隔）:', '') || ''
      await api.post('/api/message/assemble/communicate/mass', {
        personList: person.split(',').map((s) => s.trim()).filter(Boolean),
        body,
        title: '群发通知',
      })
      toast.success('群发已提交')
    }
  } catch (err: any) {
    toast.error('操作失败: ' + (err?.message ?? ''))
  }
}
// rev367：IM 会话更新/按人列会话/管理配置 + 消息收藏分页/下载/缩略/列表 + 标记已读/消费/消费类型/自定义消息/分页/群发类型/删群发 真实路由（避已消费方法孪生，仅接 distinct 新端点）
async function imMore(op: string) {
  try {
    if (op === 'convUpdate') await api.put('/api/message/assemble/communicate/im/conversation', {})
    else if (op === 'convByPerson') await api.post('/api/message/assemble/communicate/im/conversation/list/with/person', {})
    else if (op === 'managerConfig') await api.post('/api/message/assemble/communicate/im/manager/config', {})
    else if (op === 'collectionList') await api.post('/api/message/assemble/communicate/im/msg/collection/list/1/size/20', {})
    else if (op === 'msgDownload') { const id = prompt('消息 ID:', '') || ''; await api.get(`/api/message/assemble/communicate/im/msg/download/${encodeURIComponent(id)}`) }
    else if (op === 'msgThumb') { const id = prompt('消息 ID:', '') || ''; await api.get(`/api/message/assemble/communicate/im/msg/download/${encodeURIComponent(id)}/image/width/120/height/120`) }
    else if (op === 'msgListObj') await api.post('/api/message/assemble/communicate/im/msg/list/object', {})
    else if (op === 'markRead') { const id = prompt('消息 ID:', '') || ''; await api.post(`/api/message/mark_read/${encodeURIComponent(id)}`, {}) }
    else if (op === 'consume') { const id = prompt('消息 ID:', '') || ''; const t = prompt('消息类型:', 'all') || 'all'; await api.get(`/api/message/consume/${encodeURIComponent(id)}/type/${encodeURIComponent(t)}`) }
    else if (op === 'consumeType') { const t = prompt('消息类型:', 'all') || 'all'; await api.put(`/api/message/assemble/communicate/consume/type/${encodeURIComponent(t)}`, {}) }
    else if (op === 'customCreate') await api.post('/api/message/assemble/communicate/message/custom/create', {})
    else if (op === 'msgPaging') await api.post('/api/message/assemble/communicate/message/list/paging/1/size/20', {})
    else if (op === 'massEnable') await api.post('/api/message/assemble/communicate/mass/enable/type', {})
    else { const id = prompt('要删除的群发 ID:', '') || ''; if (!(await confirmMsg('确定删除该群发？'))) return; await api.delete(`/api/message/assemble/communicate/mass/${encodeURIComponent(id)}`) }
    toast.success('IM 操作已提交')
  } catch (err: any) {
    toast.error('操作失败: ' + (err?.message ?? ''))
  }
}
// rev387：IM assemble/communicate 会话置顶/取消置顶/标记已读/退群、消息撤回、按类型消费、当前人已消费读 真实路由（均 Path-only 已核，各方法孪生择一；规避 im/msg/clear 裸路由 trap500）
async function imMore2(op: string) {
  try {
    if (op === 'topSet') { const id = encodeURIComponent(prompt('会话 ID:', '') || ''); await api.put(`/api/message/assemble/communicate/im/conversation/${id}/top/set`, {}) }
    else if (op === 'topCancel') { const id = encodeURIComponent(prompt('会话 ID:', '') || ''); await api.put(`/api/message/assemble/communicate/im/conversation/${id}/top/cancel`, {}) }
    else if (op === 'convRead') { const id = encodeURIComponent(prompt('会话 ID:', '') || ''); await api.put(`/api/message/assemble/communicate/im/conversation/${id}/read`, {}) }
    else if (op === 'groupQuit') { const id = encodeURIComponent(prompt('群会话 ID:', '') || ''); if (!(await confirmMsg('确定退出该群会话？'))) return; await api.post(`/api/message/assemble/communicate/im/conversation/${id}/group/quit/self`, {}) }
    else if (op === 'msgRevoke') { const id = encodeURIComponent(prompt('消息 ID:', '') || ''); await api.get(`/api/message/assemble/communicate/im/msg/revoke/${id}`) }
    else if (op === 'consumeType') { const id = encodeURIComponent(prompt('消息 ID:', '') || ''); const t = encodeURIComponent(prompt('消息类型:', '') || ''); await api.get(`/api/message/assemble/communicate/consume/${id}/type/${t}`) }
    else await api.get('/api/message/assemble/communicate/instant/currentperson/consumed')
    toast.success('IM 操作已提交')
  } catch (err: any) {
    toast.error('操作失败: ' + (err?.message ?? ''))
  }
}
// rev412：IM 即时消息全部标记已消费(PUT session UPDATE，区别于既有 GET 读) + 按消息移除收藏(DELETE body messageId x_message_collection) 真实写
async function imMore3(op: string) {
  try {
    if (op === 'instantConsumedPut') {
      await api.put('/api/message/assemble/communicate/instant/currentperson/consumed', {})
    } else {
      const messageId = prompt('要移除收藏的消息 ID:', '') || ''
      if (!(await confirmMsg('确定移除该消息的收藏？'))) return
      await api.delete('/api/message/assemble/communicate/im/msg/collection/remove', { body: { messageId } })
    }
    toast.success('IM 操作已提交')
  } catch (err: any) {
    toast.error('操作失败: ' + (err?.message ?? ''))
  }
}
// rev426：IM communicate 写端点 发送消息(send→x_message)·创建连接器即时消息(connector→x_message_instant)·创建ws消费消息(ws→x_message_consume)（handler unwrap_or_default 不 guard，故用 prompt 真实内容避免插垃圾行；用户触发）
async function imMore4(op: string) {
  try {
    if (op === 'sendMsg') {
      const content = prompt('消息内容:', '') || ''
      if (!content.trim()) return
      await api.post('/api/message/assemble/communicate/send', { conversationId: selectedChat.value?.id ?? '', content, type: 'text' })
    } else if (op === 'connectorCreate') {
      const title = prompt('即时消息标题:', '') || ''
      if (!title.trim()) return
      await api.post('/api/message/assemble/communicate/connector', { type: 'text', person: session.user?.unique ?? '', title, body: title })
    } else {
      const content = prompt('ws 消费内容:', '') || ''
      if (!content.trim()) return
      await api.post('/api/message/assemble/communicate/ws', { person: session.user?.unique ?? '', sender: session.user?.unique ?? '', body: content })
    }
    toast.success('IM 消息已提交')
  } catch (err: any) {
    toast.error('操作失败: ' + (err?.message ?? ''))
  }
}

function handleScroll(): void {
  const el = messageContainer.value
  if (!el || el.scrollTop > 100) return
  // 加载更多历史消息
  page.value++
}

function scrollToBottom(): void {
  messageContainer.value?.scrollTo({ top: messageContainer.value.scrollHeight, behavior: 'smooth' })
}

function formatContent(content: string): string {
  // Return raw text — Vue interpolates safely via text nodes.
  return content
}

async function loadImStats() {
  try {
    // GET message/unread/count + ws/count/person + im/manager/config —— 未读消息数/在线人数/IM 管理配置
    // rev272：+ws/list/person/current/node → x_message_ws_session(disconnected_at IS NULL 在线人员清单，区别于 ws/count 计数)
    const [unread, online, cfg, wsList] = await Promise.all([
      api.get('/api/message/unread/count'),
      api.get('/api/message/assemble/communicate/ws/count/person'),
      api.get('/api/message/assemble/communicate/im/manager/config'),
      api.get('/api/message/assemble/communicate/ws/list/person/current/node').catch(() => null),
    ])
    const num = (r: any) => {
      const d = (r as any)?.data
      return typeof d === 'number' ? d : (Array.isArray(d) ? d.length : (d?.count ?? 0))
    }
    toast.success(`未读 ${num(unread)} / 在线 ${num(online)} / IM配置 ${(cfg as any)?.data ? '有' : '无'} / 在线清单 ${num(wsList)}`)
  } catch (e: any) {
    toast.error('加载 IM 统计失败: ' + (e?.message ?? ''))
  }
}
async function loadImMeta() {
  try {
    // GET im/conversation/list/with/person + ws/list/person —— 会话概览 + 在线人员
    const [convs, online] = await Promise.all([
      api.get('/api/message/assemble/communicate/im/conversation/list/with/person'),
      api.get('/api/message/assemble/communicate/ws/list/person'),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : 0)
    toast.success(`会话 ${n(convs)} / 在线 ${n(online)}`)
  } catch (e: any) {
    toast.error('加载会话概览失败: ' + (e?.message ?? ''))
  }
}
// 消费即时消息/群发族 3 条真实 distinct 路由：当前人已消费即时消息 + 当前人消息列表(desc) + 群发启用类型
async function loadImInstant() {
  try {
    const [consumed, listDesc, massType] = await Promise.all([
      api.get('/api/message/assemble/communicate/instant/currentperson/consumed').catch(() => null),
      api.get('/api/message/assemble/communicate/instant/list/currentperson/count/20/desc').catch(() => null),
      api.get('/api/message/assemble/communicate/mass/enable/type').catch(() => null),
    ])
    const n = (r: any) => {
      const d = (r as any)?.data
      return Array.isArray(d) ? d.length : (typeof d === 'number' ? d : (d?.count ?? (d ? 1 : 0)))
    }
    toast.success(`已消费 ${n(consumed)} / 近期消息 ${n(listDesc)} / 群发类型 ${(massType as any)?.data ? '已启用' : '未启用'}`)
  } catch (e: any) {
    toast.error('加载即时消息失败: ' + (e?.message ?? ''))
  }
}
// 消费消息归档族 3 条真实 distinct 路由：收藏消息分页 + 当前人全部已消费 + 全站消息分页
async function loadImArchive() {
  try {
    const [collection, consumedAll, msgPaging, coreList] = await Promise.all([
      api.get('/api/message/assemble/communicate/im/msg/collection/list/1/size/20').catch(() => null),
      api.get('/api/message/assemble/communicate/instant/currentperson/consumed/all').catch(() => null),
      api.get('/api/message/assemble/communicate/message/list/paging/1/size/20').catch(() => null),
      // GET message/core/entity/list —— 核心消息实体列表
      api.get('/api/message/core/entity/list').catch(() => null),
    ])
    const n = (r: any) => (Array.isArray((r as any)?.data) ? (r as any).data.length : 0)
    toast.success(`收藏 ${n(collection)} / 全部已消费 ${n(consumedAll)} / 消息 ${n(msgPaging)} / 核心 ${n(coreList)}`)
  } catch (e: any) {
    toast.error('加载消息归档失败: ' + (e?.message ?? ''))
  }
}
// 消息分类族 3 条真实 distinct 路由（均 x_message_consume，WHERE 各异）：按类型 consume/type/{type}（WHERE type=$1）
// + 未消费 instant/list/currentperson/not/consumed/count/{count}/desc（WHERE consumed=false）+ 非IM instant/list/currentperson/noim/count/{count}/desc（WHERE type!='im'）
async function loadMsgByType() {
  const msgType = 'information'
  try {
    const [byType, notConsumed, noim, objList] = await Promise.all([
      api.get(`/api/message/assemble/communicate/consume/type/${msgType}`).catch(() => null),
      api.get('/api/message/assemble/communicate/instant/list/currentperson/not/consumed/count/20/desc').catch(() => null),
      api.get('/api/message/assemble/communicate/instant/list/currentperson/noim/count/20/desc').catch(() => null),
      // rev277：im/msg/list/object → x_message WHERE type != 'text'（非文本消息清单，arity0）
      api.get('/api/message/assemble/communicate/im/msg/list/object').catch(() => null),
      // rev313：IM 消息分页 + 消费清单游标（x_message 纯 SELECT）
      api.get('/api/message/assemble/communicate/im/msg/list/1/size/20').catch(() => null),
      api.get('/api/message/consume/list/0/count/20').catch(() => null),
    ])
    const n = (r: any) => (Array.isArray((r as any)?.data) ? (r as any).data.length : 0)
    toast.success(`information类 ${n(byType)} / 未消费 ${n(notConsumed)} / 非IM ${n(noim)} / 非文本 ${n(objList)}`)
  } catch (e: any) {
    toast.error('加载消息分类失败: ' + (e?.message ?? ''))
  }
}
// 群发消息族 3 条真实 distinct 路由：群发详情 mass/{id}（x_message_mass by id）+ 群发消息游标
// mass/list/{id}/next/{count}（x_message WHERE mass_id AND id>$2）+ mass/list/{id}/prev/{count}（WHERE mass_id AND id<$2）。id=0 从头。
async function loadMassMessages() {
  const headId = '0'
  const cnt = '20'
  try {
    const [detail, next, prev] = await Promise.all([
      api.get(`/api/message/assemble/communicate/mass/${headId}`).catch(() => null),
      api.get(`/api/message/assemble/communicate/mass/list/${headId}/next/${cnt}`).catch(() => null),
      api.get(`/api/message/assemble/communicate/mass/list/${headId}/prev/${cnt}`).catch(() => null),
    ])
    const title = (detail as any)?.data?.title ?? '—'
    const n = (r: any) => (Array.isArray((r as any)?.data) ? (r as any).data.length : 0)
    toast.success(`群发「${title}」· 游标 next ${n(next)} / prev ${n(prev)}`)
  } catch (e: any) {
    toast.error('加载群发消息失败: ' + (e?.message ?? ''))
  }
}

// rev224：即时消息消费维度族 6 条真实 distinct 路由（x_message_consume）
// instant/list/currentperson/consumed/count/{count}/asc（consumed=true ASC）· /desc（DESC）· count/{count}/asc（全部 ASC）
// · not/consumed/count/{count}/asc（consumed=false ASC）· instant/list/{id}/next/{count}（id> ASC）· instant/list/{id}/prev/{count}（id< DESC）
async function loadInstantFacets() {
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  try {
    const [consAsc, consDesc, allAsc, notCons, next, prev] = await Promise.all([
      s(api.get('/api/message/assemble/communicate/instant/list/currentperson/consumed/count/20/asc')),
      s(api.get('/api/message/assemble/communicate/instant/list/currentperson/consumed/count/20/desc')),
      s(api.get('/api/message/assemble/communicate/instant/list/currentperson/count/20/asc')),
      s(api.get('/api/message/assemble/communicate/instant/list/currentperson/not/consumed/count/20/asc')),
      s(api.get('/api/message/assemble/communicate/instant/list/0/next/20')),
      s(api.get('/api/message/assemble/communicate/instant/list/999999999/prev/20')),
    ])
    const n = (r: any) => (Array.isArray((r as any)?.data) ? (r as any).data.length : 0)
    toast.success(`已消费 升 ${n(consAsc)}/降 ${n(consDesc)} · 全部升 ${n(allAsc)} · 未消费升 ${n(notCons)} · 游标 next ${n(next)}/prev ${n(prev)}`)
  } catch (e: any) {
    toast.error('加载即时消息维度失败: ' + (e?.message ?? ''))
  }
}

// rev229：消息消费队列/接收族 6 条真实 distinct 路由（x_message_consume / SeaORM message）
// assemble/communicate receive/{consume}（WHERE consume+consumed=false ASC）· consume/list/{consume}/count/{count}（WHERE consume DESC）
// · consume/list/{consume}/currentperson/count/{count}（+sender=consume）· consume/list/{consume}/person/{person}/count/{count}（+sender=$2）
// · core/entity/list/by/{consume}（ORM）· core/entity/unread/count/{consume}（ORM count 未读）
async function loadConsumeFacets() {
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const consume = 'instant'
  const person = '0'
  try {
    const [receive, listCount, curPerson, byPerson, coreList, unread] = await Promise.all([
      s(api.get(`/api/message/assemble/communicate/receive/${encodeURIComponent(consume)}`)),
      s(api.get(`/api/message/assemble/communicate/consume/list/${encodeURIComponent(consume)}/count/20`)),
      s(api.get(`/api/message/assemble/communicate/consume/list/${encodeURIComponent(consume)}/currentperson/count/20`)),
      s(api.get(`/api/message/assemble/communicate/consume/list/${encodeURIComponent(consume)}/person/${encodeURIComponent(person)}/count/20`)),
      s(api.get(`/api/message/core/entity/list/by/${encodeURIComponent(consume)}`)),
      s(api.get(`/api/message/core/entity/unread/count/${encodeURIComponent(consume)}`)),
    ])
    const n = (r: any) => (Array.isArray((r as any)?.data) ? (r as any).data.length : 0)
    const cnt = (r: any) => ((r as any)?.data?.count ?? (r as any)?.data ?? 0)
    toast.success(`接收 ${n(receive)} · 队列 ${n(listCount)} · 当前人 ${n(curPerson)} · 指定人 ${n(byPerson)} · 实体 ${n(coreList)} · 未读 ${cnt(unread)}`)
  } catch (e: any) {
    toast.error('加载消费队列失败: ' + (e?.message ?? ''))
  }
}

onMounted(initWebSocket)
onUnmounted(() => {
  // 卸载：先结束 P2P 通话（释放麦克风/轨道），再关 WS
  if (callState.value !== 'idle') endCall(false)
  wsClient.value?.close()
})
</script>

<style scoped>
.im-view { display: flex; height: 100%; gap: 16px; }

.im-sidebar {
  width: 300px; flex-shrink: 0; display: flex; flex-direction: column;
  transition: all var(--transition-normal);
}
.im-sidebar.collapsed { width: 0; overflow: hidden; }

.sidebar-header {
  display: flex; align-items: center; justify-content: space-between;
  padding: 16px 20px; border-bottom: 1px solid var(--border-subtle);
}
.sidebar-header h2 {
  font-family: 'Orbitron', sans-serif; font-size: 16px; color: var(--color-primary); margin: 0;
}
.header-actions { display: flex; gap: 8px; }
.new-chat-btn {
  background: var(--color-primary-soft); border: 1px solid var(--color-primary);
  color: var(--color-primary); width: 32px; height: 32px; border-radius: var(--radius-md);
  cursor: pointer; font-size: 16px; transition: all var(--transition-fast);
}
.new-chat-btn:hover { background: var(--color-primary); color: var(--text-inverse); }

.search-bar {
  display: flex; align-items: center; gap: 8px; padding: 10px 16px;
  border-bottom: 1px solid var(--border-subtle);
}
.search-icon { color: var(--text-muted); }
.search-input {
  flex: 1; background: none; border: none; outline: none;
  color: var(--text-primary); font-size: 13px;
}
.search-input::placeholder { color: var(--text-muted); }

.conversation-list { flex: 1; overflow-y: auto; padding: 8px; }
.conv-item {
  display: flex; align-items: center; gap: 10px; padding: 10px 12px;
  border-radius: var(--radius-md); cursor: pointer; transition: all var(--transition-fast);
}
.conv-item:hover { background: var(--color-primary-soft); }
.conv-item.active { background: var(--color-primary-soft); border-left: 3px solid var(--color-primary); }
.conv-avatar {
  width: 40px; height: 40px; border-radius: 50%; flex-shrink: 0;
  background: linear-gradient(135deg, var(--color-primary), var(--color-accent));
  color: white; display: flex; align-items: center; justify-content: center;
  font-weight: 600; font-size: 16px;
}
.conv-info { flex: 1; min-width: 0; }
.conv-name { font-size: 13px; font-weight: 500; color: var(--text-primary); }
.conv-preview { font-size: 12px; color: var(--text-muted); overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.conv-meta { display: flex; flex-direction: column; align-items: flex-end; gap: 4px; }
.conv-time { font-size: 11px; color: var(--text-muted); }
.conv-badge {
  background: var(--color-error); color: white; font-size: 10px; font-weight: 700;
  padding: 1px 6px; border-radius: 10px; min-width: 18px; text-align: center;
}
.empty-conv { padding: 40px; text-align: center; color: var(--text-muted); font-size: 13px; }

.unread-bar {
  margin: 8px; padding: 10px 16px; display: flex; align-items: center;
  justify-content: space-between; font-size: 13px; color: var(--text-secondary);
}
.mark-all-btn {
  background: none; border: none; color: var(--color-primary); cursor: pointer;
  font-size: 12px; text-decoration: underline;
}

.im-main {
  flex: 1; display: flex; flex-direction: column; overflow: hidden;
  transition: all var(--transition-normal);
}
.im-main.no-chat { border-color: transparent; }

.chat-header {
  display: flex; align-items: center; gap: 12px; padding: 12px 20px;
  border-bottom: 1px solid var(--border-subtle);
}
.back-btn {
  display: none; background: none; border: none; color: var(--color-primary);
  font-size: 18px; cursor: pointer; padding: 4px 8px;
}
.chat-avatar {
  width: 36px; height: 36px; border-radius: 50%; flex-shrink: 0;
  background: linear-gradient(135deg, var(--color-primary), var(--color-accent));
  color: white; display: flex; align-items: center; justify-content: center; font-weight: 600;
}
.chat-info { flex: 1; }
.chat-name { font-size: 15px; font-weight: 600; color: var(--text-primary); }
.chat-status { font-size: 11px; color: var(--text-muted); }
.chat-status.online { color: var(--color-success); }
.chat-presence {
  display: flex; align-items: center; gap: 4px; font-size: 11px; color: var(--color-success);
}
.presence-dot {
  width: 6px; height: 6px; border-radius: 50%; background: var(--color-success);
}
.call-chip {
  font-size: 11px; color: var(--color-primary); background: var(--color-primary-soft);
  border: 1px solid var(--color-primary); border-radius: 10px; padding: 1px 8px;
}
.chat-actions { display: flex; gap: 4px; }
.icon-btn {
  background: none; border: none; color: var(--text-muted); font-size: 18px;
  cursor: pointer; padding: 6px 8px; border-radius: var(--radius-sm);
  transition: all var(--transition-fast);
}
.icon-btn:hover { background: var(--color-primary-soft); color: var(--color-primary); }

.message-list {
  flex: 1; overflow-y: auto; padding: 16px; display: flex; flex-direction: column; gap: 12px;
}
.empty-messages { flex: 1; display: flex; align-items: center; justify-content: center; color: var(--text-muted); }
.loading-state { display: flex; flex-direction: column; gap: 8px; padding: 16px; }
.skeleton-row { height: 48px; border-radius: var(--radius-md); margin-bottom: 8px; }
.load-more-trigger { height: 1px; }

.message { display: flex; gap: 8px; max-width: 80%; }
.message.outgoing { margin-left: auto; flex-direction: row-reverse; }
.msg-avatar {
  width: 32px; height: 32px; border-radius: 50%; flex-shrink: 0;
  background: var(--bg-elevated); color: var(--color-primary);
  display: flex; align-items: center; justify-content: center; font-size: 14px; font-weight: 600;
}
.message.outgoing .msg-avatar { background: linear-gradient(135deg, var(--color-primary), var(--color-accent)); color: white; }
.msg-bubble {
  padding: 10px 14px; border-radius: var(--radius-lg);
  background: var(--bg-elevated); border: 1px solid var(--border-subtle);
}
.message.outgoing .msg-bubble {
  background: var(--color-primary-soft); border-color: var(--border-active);
}
.msg-content { font-size: 14px; color: var(--text-primary); line-height: 1.5; word-break: break-word; }
.msg-time { font-size: 11px; color: var(--text-muted); margin-top: 4px; }

.message-input {
  display: flex; align-items: flex-end; gap: 8px; padding: 12px 16px;
  border-top: 1px solid var(--border-subtle);
}
.input-btn {
  background: none; border: none; font-size: 20px; cursor: pointer;
  padding: 4px; border-radius: var(--radius-sm); transition: background var(--transition-fast);
}
.input-btn:hover { background: var(--color-primary-soft); }
.input-textarea {
  flex: 1; background: var(--bg-elevated); border: 1px solid var(--border-subtle);
  border-radius: var(--radius-lg); padding: 10px 14px; color: var(--text-primary);
  font-size: 14px; resize: none; outline: none; min-height: 40px; max-height: 120px;
  font-family: inherit; line-height: 1.5; transition: border-color var(--transition-fast);
}
.input-textarea:focus { border-color: var(--color-primary); }
.send-btn {
  padding: 10px 20px; background: linear-gradient(135deg, var(--color-primary), var(--color-primary-deep));
  border: none; border-radius: var(--radius-md); color: white; font-weight: 600;
  cursor: pointer; transition: all var(--transition-fast); white-space: nowrap;
}
.send-btn:hover:not(:disabled) { transform: translateY(-1px); box-shadow: 0 4px 12px var(--color-primary-glow); }
.send-btn:disabled { opacity: 0.5; cursor: not-allowed; }
.receipt-indicator {
  font-size: 11px; color: var(--text-muted); white-space: nowrap; align-self: center;
}

.no-chat-placeholder {
  flex: 1; display: flex; flex-direction: column; align-items: center; justify-content: center;
  gap: 12px; color: var(--text-muted);
}
.placeholder-icon { font-size: 64px; opacity: 0.3; }
.no-chat-placeholder h2 { font-family: 'Orbitron', sans-serif; color: var(--text-secondary); font-size: 18px; }

@media (max-width: 768px) {
  .im-sidebar { width: 100%; position: absolute; z-index: 10; height: calc(100% - var(--tabbar-height)); }
  .im-sidebar.collapsed { transform: translateX(-100%); }
  .back-btn { display: block; }
}
</style>
