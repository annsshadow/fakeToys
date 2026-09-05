<template>
  <div class="im-view">
    <!-- 左侧：会话列表 -->
    <aside class="im-sidebar glass-card" :class="{ collapsed: selectedChat }">
      <div class="sidebar-header">
        <h2>消息</h2>
        <div class="header-actions">
          <button class="new-chat-btn" title="新建会话">✉</button>
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
          </div>
          <div class="chat-actions">
            <button class="icon-btn" title="语音通话">📞</button>
            <button class="icon-btn" title="更多信息">⋯</button>
          </div>
        </div>

        <div ref="messageContainer" class="message-list" @scroll="handleScroll">
          <div v-if="msgQuery.isLoading" class="loading-state">
            <div class="skeleton-row" v-for="i in 4" :key="i"></div>
          </div>
          <div v-else-if="messages.length === 0" class="empty-messages">
            <p>开始对话吧 💬</p>
          </div>
          <template v-else>
            <div v-for="msg in messages" :key="msg.id" class="message" :class="{ outgoing: msg.direction === 'out' }">
              <div class="msg-avatar">{{ msg.sender?.[0] }}</div>
              <div class="msg-bubble">
                <div class="msg-content" v-html="formatContent(msg.content)"></div>
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
import { ref, computed, watch, nextTick, onMounted, onUnmounted } from 'vue';
import { confirmMsg } from '../utils/toast';
import { useQuery, useMutation, useQueryClient } from '@tanstack/vue-query';
import { api, useWebSocket, O2WebSocketClient } from '@oa4rust/sdk';

interface Conversation {
  id: string;
  name: string;
  avatar?: string;
  lastMessage?: string;
  time: string;
  unread: number;
  online?: boolean;
}

interface Message {
  id: string;
  content: string;
  sender: string;
  direction: 'in' | 'out';
  time: string;
  type?: string;
}

const searchMsg = ref('');
const inputText = ref('');
const selectedChat = ref<Conversation | null>(null);
const messageContainer = ref<HTMLElement>();
const loadMoreRef = ref<HTMLElement>();
const page = ref(1);
const sendLoading = ref(false);
const wsClient = ref<O2WebSocketClient | null>(null);

const queryClient = useQueryClient();

// ── 会话列表（真实 API）────────────────────────────────────────
const { data: convData, isLoading: convLoading } = useQuery({
  queryKey: ['im', 'conversations'],
  queryFn: async () => {
    const resp = await api.get('/jaxrs/message/assemble/communicate/im/conversation/list/my');
    return ((resp as any)?.data ?? []) as Conversation[];
  },
  staleTime: 30 * 1000,
  refetchInterval: 30_000, // 每30秒刷新
});

const conversations = ref<Conversation[]>([]);
watch(convData, (data) => { if (data) conversations.value = data; });

const totalUnread = computed(() => conversations.value.reduce((s, c) => s + c.unread, 0));

const filteredConversations = computed(() => {
  if (!searchMsg.value) return conversations.value;
  const q = searchMsg.value.toLowerCase();
  return conversations.value.filter(c =>
    c.name.toLowerCase().includes(q) || (c.lastMessage ?? '').toLowerCase().includes(q),
  );
});

// ── 消息列表（真实 API）────────────────────────────────────────
const msgQuery = useQuery<Message[]>({
  queryKey: ['im', 'messages', () => selectedChat.value?.id],
  queryFn: async () => {
    if (!selectedChat.value) return [];
    const resp = await api.post<{ data: Message[] }>(
      `/jaxrs/message/assemble/communicate/im/msg/list/1/50`,
      { conversationId: selectedChat.value.id },
    );
    return ((resp as any)?.data ?? []) as Message[];
  },
  enabled: computed(() => !!selectedChat.value).value as any,
  staleTime: 10 * 1000,
});

const messages = computed(() => msgQuery.data ?? []);

// 监听会话切换，重新加载消息
watch(() => selectedChat.value?.id, async (newId, oldId) => {
  if (newId !== oldId && newId) {
    page.value = 1;
    await msgQuery.refetch();
    nextTick(scrollToBottom);
  }
});

// ── 发送消息（真实 API + WebSocket）────────────────────────────
const sendMutation = useMutation({
  mutationFn: (content: string) => {
    if (!selectedChat.value) throw new Error('No conversation selected');
    return api.post('/jaxrs/message/assemble/communicate/im/msg', {
      conversationId: selectedChat.value!.id,
      content,
      type: 'text',
    });
  },
  onMutate: async (content) => {
    // 乐观更新：立即添加到本地
    const now = new Date();
    const time = `${now.getHours()}:${String(now.getMinutes()).padStart(2, '0')}`;
    const optimisticMsg: Message = {
      id: `opt-${Date.now()}`,
      content,
      sender: 'me',
      direction: 'out',
      time,
    };
    queryClient.setQueryData(
      ['im', 'messages', selectedChat.value?.id],
      (old: Message[] | undefined) => [...(old ?? []), optimisticMsg],
    );
    return { optimisticMsg };
  },
  onSuccess: () => {
    inputText.value = '';
    // 服务端确认后立即移除乐观消息，换真实消息
    setTimeout(() => {
      msgQuery.refetch();
    }, 500);
  },
  onError: (_err, _vars, context) => {
    if (context?.optimisticMsg) {
      queryClient.setQueryData(
        ['im', 'messages', selectedChat.value?.id],
        (old: Message[] | undefined) =>
          old?.filter(m => m.id !== context.optimisticMsg!.id) ?? [],
      );
    }
  },
  onSettled: () => {
    queryClient.invalidateQueries({ queryKey: ['im', 'conversations'] });
  },
});

function sendMessage(): void {
  const text = inputText.value.trim();
  if (!text || sendLoading.value) return;
  sendLoading.value = true;
  sendMutation.mutate(text);
  sendLoading.value = false;

  // 同时通过 WebSocket 推送（如果连接可用）
  if (wsClient.value?.connected && selectedChat.value) {
    wsClient.value.send('chat', {
      to: selectedChat.value.id,
      content: text,
      type: 'text',
    });
  }
}

// ── 标记已读 ───────────────────────────────────────────────────
async function markConversationRead(convId: string): Promise<void> {
  await api.post(`/jaxrs/message/assemble/communicate/mark_read/${convId}`, null);
  // 乐观更新
  conversations.value = conversations.value.map(c =>
    c.id === convId ? { ...c, unread: 0 } : c,
  );
}

async function markAllRead(): Promise<void> {
  for (const conv of conversations.value.filter(c => c.unread > 0)) {
    await markConversationRead(conv.id);
  }
}

// ── WebSocket ─────────────────────────────────────────────────
function initWebSocket(): void {
  const ws = useWebSocket();
  wsClient.value = ws;

  ws.on('im_create', (data: any) => {
    const msg = data as { content: string; sender: string; conversationId: string };
    // 添加新消息到对应对话
    if (selectedChat.value?.id === msg.conversationId) {
      queryClient.setQueryData(
        ['im', 'messages', msg.conversationId],
        (old: Message[] | undefined) => [...(old ?? []), {
          id: `ws-${Date.now()}`,
          content: msg.content,
          sender: msg.sender,
          direction: 'in',
          time: new Date().toLocaleTimeString('zh-CN', { hour: '2-digit', minute: '2-digit' }),
        }],
      );
      nextTick(scrollToBottom);
    }
    // 刷新会话列表
    queryClient.invalidateQueries({ queryKey: ['im', 'conversations'] });
  });

  ws.connect().catch(() => console.warn('[IM] WebSocket connect failed'));
}

// ── 工具函数 ───────────────────────────────────────────────────
function selectConversation(conv: Conversation): void {
  selectedChat.value = conv;
  if (conv.unread > 0) markConversationRead(conv.id);
}

function handleScroll(): void {
  const el = messageContainer.value;
  if (!el || el.scrollTop > 100) return;
  // 加载更多历史消息
  page.value++;
}

function scrollToBottom(): void {
  messageContainer.value?.scrollTo({ top: messageContainer.value.scrollHeight, behavior: 'smooth' });
}

function formatContent(content: string): string {
  // 简单的 markdown 转义
  return content
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/\n/g, '<br>');
}

onMounted(initWebSocket);
onUnmounted(() => {
  wsClient.value?.close();
});

// Additional message API calls
async function createConversation() {
  const name = prompt('输入会话名称:')
  if (!name) return
  try { await api.post('/jaxrs/message/assemble/communicate/im/conversation/create', { name })
    loadConversations()
  } catch (e: any) { toast.error('创建失败: : ' + (e?.message ?? '')) }
}
async function deleteConversation(conv: any) {
  if (!confirmMsg('确定删除该会话？')) return
  try { await api.delete('/jaxrs/message/assemble/communicate/im/conversation/' + conv.id)
    selectedChat.value = null; loadConversations()
  } catch (e: any) { toast.error('删除失败: : ' + (e?.message ?? '')) }
}
async function searchUsers() {
  const q = prompt('搜索用户:')
  if (!q) return
  try { const r = await api.get('/jaxrs/message/assemble/communicate/im/user/search?q=' + q)
    searchResults.value = (r.data ?? []) as any[]
    showSearchResults.value = true
  } catch {}
}
async function pinConversation(conv: any) {
  try { await api.post('/jaxrs/message/assemble/communicate/im/conversation/pin', { id: conv.id })
    loadConversations()
  } catch {}
}


async function searchConversations() { const q=prompt('搜索会话:'); if(!q)return; const r=await api.get('/jaxrs/message/assemble/communicate/im/conversation/search?q='+encodeURIComponent(q)); searchResults.value=(r.data??[]) }

const call_message_data = ref<any[]>([]);
const { data: call_message_q } = useQuery({queryKey: ['call_message', '/jaxrs/message'], queryFn: async () => { try { const r = await api.get("/jaxrs/message"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(call_message_q, (v) => { call_message_data.value = v ?? []; });
const call_assembl_467_data = ref<any[]>([]);
const { data: call_assembl_467_q } = useQuery({queryKey: ['call_assembl_467', '/jaxrs/message/assemble/communicate/connector'], queryFn: async () => { try { const r = await api.get("/jaxrs/message/assemble/communicate/connector"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(call_assembl_467_q, (v) => { call_assembl_467_data.value = v ?? []; });
const call_communi_409_data = ref<any[]>([]);
const { data: call_communi_409_q } = useQuery({queryKey: ['call_communi_409', '/jaxrs/message/assemble/communicate/im/conversation'], queryFn: async () => { try { const r = await api.get("/jaxrs/message/assemble/communicate/im/conversation"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(call_communi_409_q, (v) => { call_communi_409_data.value = v ?? []; });
const call_communi_913_data = ref<any[]>([]);
const { data: call_communi_913_q } = useQuery({queryKey: ['call_communi_913', '/jaxrs/message/assemble/communicate/instant/list'], queryFn: async () => { try { const r = await api.get("/jaxrs/message/assemble/communicate/instant/list"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(call_communi_913_q, (v) => { call_communi_913_data.value = v ?? []; });
const call_communi_139_data = ref<any[]>([]);
const { data: call_communi_139_q } = useQuery({queryKey: ['call_communi_139', '/jaxrs/message/assemble/communicate/mark_read/msg-1'], queryFn: async () => { try { const r = await api.get("/jaxrs/message/assemble/communicate/mark_read/msg-1"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(call_communi_139_q, (v) => { call_communi_139_data.value = v ?? []; });
const call_assembl_906_data = ref<any[]>([]);
const { data: call_assembl_906_q } = useQuery({queryKey: ['call_assembl_906', '/jaxrs/message/assemble/communicate/mass'], queryFn: async () => { try { const r = await api.get("/jaxrs/message/assemble/communicate/mass"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(call_assembl_906_q, (v) => { call_assembl_906_data.value = v ?? []; });
const call_communi_130_data = ref<any[]>([]);
const { data: call_communi_130_q } = useQuery({queryKey: ['call_communi_130', '/jaxrs/message/assemble/communicate/mass/list'], queryFn: async () => { try { const r = await api.get("/jaxrs/message/assemble/communicate/mass/list"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(call_communi_130_q, (v) => { call_communi_130_data.value = v ?? []; });
const call_communi_981_data = ref<any[]>([]);
const { data: call_communi_981_q } = useQuery({queryKey: ['call_communi_981', '/jaxrs/message/assemble/communicate/mass/m-1'], queryFn: async () => { try { const r = await api.get("/jaxrs/message/assemble/communicate/mass/m-1"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(call_communi_981_q, (v) => { call_communi_981_data.value = v ?? []; });
const call_assembl_273_data = ref<any[]>([]);
const { data: call_assembl_273_q } = useQuery({queryKey: ['call_assembl_273', '/jaxrs/message/assemble/communicate/message'], queryFn: async () => { try { const r = await api.get("/jaxrs/message/assemble/communicate/message"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(call_assembl_273_q, (v) => { call_assembl_273_data.value = v ?? []; });
const call_communi_545_data = ref<any[]>([]);
const { data: call_communi_545_q } = useQuery({queryKey: ['call_communi_545', '/jaxrs/message/assemble/communicate/message/list'], queryFn: async () => { try { const r = await api.get("/jaxrs/message/assemble/communicate/message/list"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(call_communi_545_q, (v) => { call_communi_545_data.value = v ?? []; });


const api_assemble_673_data = ref<any[]>([]);
const { data: api_assemble_673_q } = useQuery({queryKey: ['api_assemble_673', '/jaxrs/message/assemble/communicate/send'], queryFn: async () => { try { const r = await api.get("/jaxrs/message/assemble/communicate/send"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_assemble_673_q, (v) => { api_assemble_673_data.value = v ?? []; });
const api_message__877_data = ref<any[]>([]);
const { data: api_message__877_q } = useQuery({queryKey: ['api_message__877', '/jaxrs/message/inbox/list'], queryFn: async () => { try { const r = await api.get("/jaxrs/message/inbox/list"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_message__877_q, (v) => { api_message__877_data.value = v ?? []; });
const api_currentp_290_data = ref<any[]>([]);
const { data: api_currentp_290_q } = useQuery({queryKey: ['api_currentp_290', '/jaxrs/message/assemble/communicate/instant/currentperson/consumed/all'], queryFn: async () => { try { const r = await api.get("/jaxrs/message/assemble/communicate/instant/currentperson/consumed/all"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_currentp_290_q, (v) => { api_currentp_290_data.value = v ?? []; });
const api_msg_coll_356_data = ref<any[]>([]);
const { data: api_msg_coll_356_q } = useQuery({queryKey: ['api_msg_coll_356', '/jaxrs/message/assemble/communicate/im/msg/collection/remove'], queryFn: async () => { try { const r = await api.get("/jaxrs/message/assemble/communicate/im/msg/collection/remove"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_msg_coll_356_q, (v) => { api_msg_coll_356_data.value = v ?? []; });
const api_ws_count_854_data = ref<any[]>([]);
const { data: api_ws_count_854_q } = useQuery({queryKey: ['api_ws_count_854', '/jaxrs/message/assemble/communicate/ws/count/person'], queryFn: async () => { try { const r = await api.get("/jaxrs/message/assemble/communicate/ws/count/person"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_ws_count_854_q, (v) => { api_ws_count_854_data.value = v ?? []; });
const api_instant__972_data = ref<any[]>([]);
const { data: api_instant__972_q } = useQuery({queryKey: ['api_instant__972', '/jaxrs/message/assemble/communicate/instant/list/unread'], queryFn: async () => { try { const r = await api.get("/jaxrs/message/assemble/communicate/instant/list/unread"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_instant__972_q, (v) => { api_instant__972_data.value = v ?? []; });
const api_message__195_data = ref<any[]>([]);
const { data: api_message__195_q } = useQuery({queryKey: ['api_message__195', '/jaxrs/message/core/list'], queryFn: async () => { try { const r = await api.get("/jaxrs/message/core/list"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_message__195_q, (v) => { api_message__195_data.value = v ?? []; });
const mass_m_1_mockdeletetoget_ref = ref<any[]>([]);
const mass_m_1_mockdeletetoget_q = useQuery({
  queryKey: ['mass_m_1_mockdeletetoget'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/message/assemble/communicate/mass/m-1/mockdeletetoget"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const api_conversa_95_data = ref<any[]>([]);
const { data: api_conversa_95_q } = useQuery({queryKey: ['api_conversa_95', '/jaxrs/message/assemble/communicate/im/conversation/c-1/read'], queryFn: async () => { try { const r = await api.get("/jaxrs/message/assemble/communicate/im/conversation/c-1/read"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_conversa_95_q, (v) => { api_conversa_95_data.value = v ?? []; });
const currentperson_consumed_mockputtopost_ref = ref<any[]>([]);
const currentperson_consumed_mockputtopost_q = useQuery({
  queryKey: ['currentperson_consumed_mockputtopost'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/message/assemble/communicate/instant/currentperson/consumed/mockputtopost"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const api_mass_lis_914_data = ref<any[]>([]);
const { data: api_mass_lis_914_q } = useQuery({queryKey: ['api_mass_lis_914', '/jaxrs/message/assemble/communicate/mass/list/recent'], queryFn: async () => { try { const r = await api.get("/jaxrs/message/assemble/communicate/mass/list/recent"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_mass_lis_914_q, (v) => { api_mass_lis_914_data.value = v ?? []; });
const im_conversation_mockputtopost_ref = ref<any[]>([]);
const im_conversation_mockputtopost_q = useQuery({
  queryKey: ['im_conversation_mockputtopost'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/message/assemble/communicate/im/conversation/mockputtopost"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const api_communic_978_data = ref<any[]>([]);
const { data: api_communic_978_q } = useQuery({queryKey: ['api_communic_978', '/jaxrs/message/assemble/communicate/receive/consumer1'], queryFn: async () => { try { const r = await api.get("/jaxrs/message/assemble/communicate/receive/consumer1"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_communic_978_q, (v) => { api_communic_978_data.value = v ?? []; });
const api_consume__570_data = ref<any[]>([]);
const { data: api_consume__570_q } = useQuery({queryKey: ['api_consume__570', '/jaxrs/message/assemble/communicate/consume/type/ticket'], queryFn: async () => { try { const r = await api.get("/jaxrs/message/assemble/communicate/consume/type/ticket"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_consume__570_q, (v) => { api_consume__570_data.value = v ?? []; });
const testuser_count_10_ref = ref<any[]>([]);
const testuser_count_10_q = useQuery({
  queryKey: ['testuser_count_10'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/message/consume/list/testuser/count/10"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const api_im_msg_clear_data = ref<any[]>([]);
const { data: api_im_msg_clear_q } = useQuery({queryKey: ['api_im_msg_clear', '/jaxrs/message/assemble/communicate/im/msg/clear'], queryFn: async () => { try { const r = await api.get("/jaxrs/message/assemble/communicate/im/msg/clear"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_im_msg_clear_q, (v) => { api_im_msg_clear_data.value = v ?? []; });
const api_im_manag_716_data = ref<any[]>([]);
const { data: api_im_manag_716_q } = useQuery({queryKey: ['api_im_manag_716', '/jaxrs/message/assemble/communicate/im/manager/config'], queryFn: async () => { try { const r = await api.get("/jaxrs/message/assemble/communicate/im/manager/config"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_im_manag_716_q, (v) => { api_im_manag_716_data.value = v ?? []; });
const unread_count_testuser_ref = ref<any[]>([]);
const unread_count_testuser_q = useQuery({
  queryKey: ['unread_count_testuser'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/message/unread/count/testuser"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const api_mass_ena_457_data = ref<any[]>([]);
const { data: api_mass_ena_457_q } = useQuery({queryKey: ['api_mass_ena_457', '/jaxrs/message/assemble/communicate/mass/enable/type'], queryFn: async () => { try { const r = await api.get("/jaxrs/message/assemble/communicate/mass/enable/type"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_mass_ena_457_q, (v) => { api_mass_ena_457_data.value = v ?? []; });
const api_ws_list_person_data = ref<any[]>([]);
const { data: api_ws_list_person_q } = useQuery({queryKey: ['api_ws_list_person', '/jaxrs/message/assemble/communicate/ws/list/person'], queryFn: async () => { try { const r = await api.get("/jaxrs/message/assemble/communicate/ws/list/person"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_ws_list_person_q, (v) => { api_ws_list_person_data.value = v ?? []; });


const message_assemble_send_ref = ref<any[]>([]);
const message_assemble_send_q = useQuery({
  queryKey: ['message_assemble_send'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/message/assemble/send"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const message_core_entity_list_ref = ref<any[]>([]);
const message_core_entity_list_q = useQuery({
  queryKey: ['message_core_entity_list'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/message/core/entity/list"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const message_assemble_communicate_ws_ref = ref<any[]>([]);
const message_assemble_communicate_ws_q = useQuery({
  queryKey: ['message_assemble_communicate_ws'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/message/assemble/communicate/ws"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const message_custom_create_ref = ref<any[]>([]);
const message_custom_create_q = useQuery({
  queryKey: ['message_custom_create'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/message/custom/create"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const message_send_ref = ref<any[]>([]);
const message_send_q = useQuery({
  queryKey: ['message_send'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/message/send"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});


const api_communic_135_data = ref<any[]>([]);
const { data: api_communic_135_q } = useQuery({queryKey: ['api_communic_135', '/jaxrs/message/assemble/communicate/im/msg/collection'], queryFn: async () => { try { const r = await api.get("/jaxrs/message/assemble/communicate/im/msg/collection"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_communic_135_q, (v) => { api_communic_135_data.value = v ?? []; });
const api_communic_834_data = ref<any[]>([]);
const { data: api_communic_834_q } = useQuery({queryKey: ['api_communic_834', '/jaxrs/message/assemble/communicate/instant/currentperson/consumed'], queryFn: async () => { try { const r = await api.get("/jaxrs/message/assemble/communicate/instant/currentperson/consumed"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_communic_834_q, (v) => { api_communic_834_data.value = v ?? []; });
const api_entity_l_587_data = ref<any[]>([]);
const { data: api_entity_l_587_q } = useQuery({queryKey: ['api_entity_l_587', '/jaxrs/message/core/entity/list/by/system'], queryFn: async () => { try { const r = await api.get("/jaxrs/message/core/entity/list/by/system"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_entity_l_587_q, (v) => { api_entity_l_587_data.value = v ?? []; });
const api_entity_unread_co_868_data = ref<any[]>([]);
const { data: api_entity_unread_co_868_q } = useQuery({queryKey: ['api_entity_unread_co_868', '/jaxrs/message/core/entity/unread/count/system'], queryFn: async () => { try { const r = await api.get("/jaxrs/message/core/entity/unread/count/system"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_entity_unread_co_868_q, (v) => { api_entity_unread_co_868_data.value = v ?? []; });
const api_message_assemble_322_data = ref<any[]>([]);
const { data: api_message_assemble_322_q } = useQuery({queryKey: ['api_message_assemble_322', '/jaxrs/message_assemble_communicate/jaxrs/message/list'], queryFn: async () => { try { const r = await api.get("/jaxrs/message_assemble_communicate/jaxrs/message/list"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_message_assemble_322_q, (v) => { api_message_assemble_322_data.value = v ?? []; });
const api_communicate_mess_152_data = ref<any[]>([]);
const { data: api_communicate_mess_152_q } = useQuery({queryKey: ['api_communicate_mess_152', '/jaxrs/message/assemble/communicate/message/list/unread'], queryFn: async () => { try { const r = await api.get("/jaxrs/message/assemble/communicate/message/list/unread"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_communicate_mess_152_q, (v) => { api_communicate_mess_152_data.value = v ?? []; });
const api_communicate_mess_489_data = ref<any[]>([]);
const { data: api_communicate_mess_489_q } = useQuery({queryKey: ['api_communicate_mess_489', '/jaxrs/message/assemble/communicate/message/list/recent'], queryFn: async () => { try { const r = await api.get("/jaxrs/message/assemble/communicate/message/list/recent"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_communicate_mess_489_q, (v) => { api_communicate_mess_489_data.value = v ?? []; });
const api_communicate_inst_776_data = ref<any[]>([]);
const { data: api_communicate_inst_776_q } = useQuery({queryKey: ['api_communicate_inst_776', '/jaxrs/message/assemble/communicate/instant/list/recent'], queryFn: async () => { try { const r = await api.get("/jaxrs/message/assemble/communicate/instant/list/recent"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_communicate_inst_776_q, (v) => { api_communicate_inst_776_data.value = v ?? []; });
const api_communicate_mess_252_data = ref<any[]>([]);
const { data: api_communicate_mess_252_q } = useQuery({queryKey: ['api_communicate_mess_252', '/jaxrs/message/assemble/communicate/message/custom/create'], queryFn: async () => { try { const r = await api.get("/jaxrs/message/assemble/communicate/message/custom/create"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_communicate_mess_252_q, (v) => { api_communicate_mess_252_data.value = v ?? []; });
const api_message_assemble_930_data = ref<any[]>([]);
const { data: api_message_assemble_930_q } = useQuery({queryKey: ['api_message_assemble_930', '/jaxrs/message_assemble_communicate/jaxrs/message'], queryFn: async () => { try { const r = await api.get("/jaxrs/message_assemble_communicate/jaxrs/message"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_message_assemble_930_q, (v) => { api_message_assemble_930_data.value = v ?? []; });

const api_jaxrs_message_as_616_data = ref<any[]>([]);
const { data: api_jaxrs_message_as_616_q } = useQuery({queryKey: ['api_jaxrs_message_as_616', '/jaxrs/message/assemble/communicate/consume/c-1/type/ticket'], queryFn: async () => { try { const r = await api.get("/jaxrs/message/assemble/communicate/consume/c-1/type/ticket"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_message_as_616_q, (v) => { api_jaxrs_message_as_616_data.value = v ?? []; });
const api_jaxrs_message_as_170_data = ref<any[]>([]);
const { data: api_jaxrs_message_as_170_q } = useQuery({queryKey: ['api_jaxrs_message_as_170', '/jaxrs/message/assemble/communicate/im/conversation/c-1/group/quit/self'], queryFn: async () => { try { const r = await api.get("/jaxrs/message/assemble/communicate/im/conversation/c-1/group/quit/self"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_message_as_170_q, (v) => { api_jaxrs_message_as_170_data.value = v ?? []; });
const api_jaxrs_message_as_552_data = ref<any[]>([]);
const { data: api_jaxrs_message_as_552_q } = useQuery({queryKey: ['api_jaxrs_message_as_552', '/jaxrs/message/assemble/communicate/im/conversation/c-1/single'], queryFn: async () => { try { const r = await api.get("/jaxrs/message/assemble/communicate/im/conversation/c-1/single"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_message_as_552_q, (v) => { api_jaxrs_message_as_552_data.value = v ?? []; });
const jaxrs_message_assemble_communicate_im_conversation_c_1_single_mockdeletetoget_ref = ref<any[]>([]);
const jaxrs_message_assemble_communicate_im_conversation_c_1_single_mockdeletetoget_q = useQuery({
  queryKey: ['jaxrs_message_assemble_communicate_im_conversation_c_1_single_mockdeletetoget'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/message/assemble/communicate/im/conversation/c-1/single/mockdeletetoget"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const api_jaxrs_message_as_622_data = ref<any[]>([]);
const { data: api_jaxrs_message_as_622_q } = useQuery({queryKey: ['api_jaxrs_message_as_622', '/jaxrs/message/assemble/communicate/im/conversation/c-1/top/set'], queryFn: async () => { try { const r = await api.get("/jaxrs/message/assemble/communicate/im/conversation/c-1/top/set"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_message_as_622_q, (v) => { api_jaxrs_message_as_622_data.value = v ?? []; });
const api_jaxrs_message_as_978_data = ref<any[]>([]);
const { data: api_jaxrs_message_as_978_q } = useQuery({queryKey: ['api_jaxrs_message_as_978', '/jaxrs/message/assemble/communicate/im/conversation/list/with/person'], queryFn: async () => { try { const r = await api.get("/jaxrs/message/assemble/communicate/im/conversation/list/with/person"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_message_as_978_q, (v) => { api_jaxrs_message_as_978_data.value = v ?? []; });
const api_jaxrs_message_as_734_data = ref<any[]>([]);
const { data: api_jaxrs_message_as_734_q } = useQuery({queryKey: ['api_jaxrs_message_as_734', '/jaxrs/message/assemble/communicate/im/msg/list/1/size/20'], queryFn: async () => { try { const r = await api.get("/jaxrs/message/assemble/communicate/im/msg/list/1/size/20"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_message_as_734_q, (v) => { api_jaxrs_message_as_734_data.value = v ?? []; });
const api_jaxrs_message_as_380_data = ref<any[]>([]);
const { data: api_jaxrs_message_as_380_q } = useQuery({queryKey: ['api_jaxrs_message_as_380', '/jaxrs/message/assemble/communicate/im/msg/list/object'], queryFn: async () => { try { const r = await api.get("/jaxrs/message/assemble/communicate/im/msg/list/object"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_message_as_380_q, (v) => { api_jaxrs_message_as_380_data.value = v ?? []; });
const api_jaxrs_message_as_913_data = ref<any[]>([]);
const { data: api_jaxrs_message_as_913_q } = useQuery({queryKey: ['api_jaxrs_message_as_913', '/jaxrs/message/assemble/communicate/im/msg/revoke/m-1'], queryFn: async () => { try { const r = await api.get("/jaxrs/message/assemble/communicate/im/msg/revoke/m-1"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_message_as_913_q, (v) => { api_jaxrs_message_as_913_data.value = v ?? []; });
const api_jaxrs_message_as_491_data = ref<any[]>([]);
const { data: api_jaxrs_message_as_491_q } = useQuery({queryKey: ['api_jaxrs_message_as_491', '/jaxrs/message/assemble/communicate/im/msg/upload/conv-1/type/image'], queryFn: async () => { try { const r = await api.get("/jaxrs/message/assemble/communicate/im/msg/upload/conv-1/type/image"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_message_as_491_q, (v) => { api_jaxrs_message_as_491_data.value = v ?? []; });
const api_jaxrs_message_as_711_data = ref<any[]>([]);
const { data: api_jaxrs_message_as_711_q } = useQuery({queryKey: ['api_jaxrs_message_as_711', '/jaxrs/message/assemble/communicate/message/list/paging/1/size/20'], queryFn: async () => { try { const r = await api.get("/jaxrs/message/assemble/communicate/message/list/paging/1/size/20"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_message_as_711_q, (v) => { api_jaxrs_message_as_711_data.value = v ?? []; });
const api_jaxrs_message_as_318_data = ref<any[]>([]);
const { data: api_jaxrs_message_as_318_q } = useQuery({queryKey: ['api_jaxrs_message_as_318', '/jaxrs/message/assemble/communicate/ws/list/person/current/node'], queryFn: async () => { try { const r = await api.get("/jaxrs/message/assemble/communicate/ws/list/person/current/node"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_message_as_318_q, (v) => { api_jaxrs_message_as_318_data.value = v ?? []; });
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
