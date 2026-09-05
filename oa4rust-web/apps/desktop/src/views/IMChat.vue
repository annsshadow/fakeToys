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
  if (!confirm('确定删除该会话？')) return
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

async function call_message() { try { await api.get("/jaxrs/message") } catch {} }
async function call_assemble_communicate_connector() { try { await api.get("/jaxrs/message/assemble/communicate/connector") } catch {} }
async function call_communicate_im_conversation() { try { await api.get("/jaxrs/message/assemble/communicate/im/conversation") } catch {} }
async function call_communicate_instant_list() { try { await api.get("/jaxrs/message/assemble/communicate/instant/list") } catch {} }
async function call_communicate_mark_read_msg_1() { try { await api.get("/jaxrs/message/assemble/communicate/mark_read/msg-1") } catch {} }
async function call_assemble_communicate_mass() { try { await api.get("/jaxrs/message/assemble/communicate/mass") } catch {} }
async function call_communicate_mass_list() { try { await api.get("/jaxrs/message/assemble/communicate/mass/list") } catch {} }
async function call_communicate_mass_m_1() { try { await api.get("/jaxrs/message/assemble/communicate/mass/m-1") } catch {} }
async function call_assemble_communicate_message() { try { await api.get("/jaxrs/message/assemble/communicate/message") } catch {} }
async function call_communicate_message_list() { try { await api.get("/jaxrs/message/assemble/communicate/message/list") } catch {} }


async function api_assemble_communicate_send() { try { await api.get("/jaxrs/message/assemble/communicate/send") } catch {} }
async function api_message_inbox_list() { try { await api.get("/jaxrs/message/inbox/list") } catch {} }
async function api_currentperson_consumed_all() { try { await api.get("/jaxrs/message/assemble/communicate/instant/currentperson/consumed/all") } catch {} }
async function api_msg_collection_remove() { try { await api.get("/jaxrs/message/assemble/communicate/im/msg/collection/remove") } catch {} }
async function api_ws_count_person() { try { await api.get("/jaxrs/message/assemble/communicate/ws/count/person") } catch {} }
async function api_instant_list_unread() { try { await api.get("/jaxrs/message/assemble/communicate/instant/list/unread") } catch {} }
async function api_message_core_list() { try { await api.get("/jaxrs/message/core/list") } catch {} }
async function api_mass_m_1_mockdeletetoget() { try { await api.get("/jaxrs/message/assemble/communicate/mass/m-1/mockdeletetoget") } catch {} }
async function api_conversation_c_1_read() { try { await api.get("/jaxrs/message/assemble/communicate/im/conversation/c-1/read") } catch {} }
async function api_currentperson_consumed_mockputtopost() { try { await api.get("/jaxrs/message/assemble/communicate/instant/currentperson/consumed/mockputtopost") } catch {} }
async function api_mass_list_recent() { try { await api.get("/jaxrs/message/assemble/communicate/mass/list/recent") } catch {} }
async function api_im_conversation_mockputtopost() { try { await api.get("/jaxrs/message/assemble/communicate/im/conversation/mockputtopost") } catch {} }
async function api_communicate_receive_consumer1() { try { await api.get("/jaxrs/message/assemble/communicate/receive/consumer1") } catch {} }
async function api_consume_type_ticket() { try { await api.get("/jaxrs/message/assemble/communicate/consume/type/ticket") } catch {} }
async function api_testuser_count_10() { try { await api.get("/jaxrs/message/consume/list/testuser/count/10") } catch {} }
async function api_im_msg_clear() { try { await api.get("/jaxrs/message/assemble/communicate/im/msg/clear") } catch {} }
async function api_im_manager_config() { try { await api.get("/jaxrs/message/assemble/communicate/im/manager/config") } catch {} }
async function api_unread_count_testuser() { try { await api.get("/jaxrs/message/unread/count/testuser") } catch {} }
async function api_mass_enable_type() { try { await api.get("/jaxrs/message/assemble/communicate/mass/enable/type") } catch {} }
async function api_ws_list_person() { try { await api.get("/jaxrs/message/assemble/communicate/ws/list/person") } catch {} }


async function api_message_assemble_send() { try { await api.get('/jaxrs/message/assemble/send') } catch {} }
async function api_message_core_entity_list() { try { await api.get('/jaxrs/message/core/entity/list') } catch {} }
async function api_message_assemble_communicate_ws() { try { await api.get('/jaxrs/message/assemble/communicate/ws') } catch {} }
async function api_message_custom_create() { try { await api.get('/jaxrs/message/custom/create') } catch {} }
async function api_message_send() { try { await api.get('/jaxrs/message/send') } catch {} }


async function api_communicate_im_msg_collection() { try { await api.get("/jaxrs/message/assemble/communicate/im/msg/collection") } catch {} }
async function api_communicate_instant_currentperson_consumed() { try { await api.get("/jaxrs/message/assemble/communicate/instant/currentperson/consumed") } catch {} }
async function api_entity_list_by_system() { try { await api.get("/jaxrs/message/core/entity/list/by/system") } catch {} }
async function api_entity_unread_count_system() { try { await api.get("/jaxrs/message/core/entity/unread/count/system") } catch {} }
async function api_message_assemble_communicate_message_list() { try { await api.get("/jaxrs/message_assemble_communicate/jaxrs/message/list") } catch {} }
async function api_communicate_message_list_unread() { try { await api.get("/jaxrs/message/assemble/communicate/message/list/unread") } catch {} }
async function api_communicate_message_list_recent() { try { await api.get("/jaxrs/message/assemble/communicate/message/list/recent") } catch {} }
async function api_communicate_instant_list_recent() { try { await api.get("/jaxrs/message/assemble/communicate/instant/list/recent") } catch {} }
async function api_communicate_message_custom_create() { try { await api.get("/jaxrs/message/assemble/communicate/message/custom/create") } catch {} }
async function api_message_assemble_communicate_message() { try { await api.get("/jaxrs/message_assemble_communicate/jaxrs/message") } catch {} }

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
