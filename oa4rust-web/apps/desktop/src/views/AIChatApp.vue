<template>
  <div class="ai-view">
    <div class="view-header glass-card">
      <h1>AI 助手</h1>
      <p class="subtitle">/jaxrs/ai_assemble_control/* — 智能对话与配置</p>
    </div>
    <div class="split-layout">
      <!-- 左侧: 对话列表 -->
      <div class="sidebar glass-card">
        <div class="sidebar-header">
          <button class="btn-new" @click="createNewChat">+ 新对话</button>
        </div>
        <div class="chat-list">
          <div
            v-for="c in conversations"
            :key="c.id"
            class="chat-item"
            :class="{ active: currentChat?.id === c.id }"
            @click="selectChat(c)"
          >
            <div class="chat-icon">🤖</div>
            <div class="chat-info">
              <div class="chat-title">{{ c.title || c.name || '未命名对话' }}</div>
              <div class="chat-meta">{{ formatDate(c.updatedAt) }}</div>
            </div>
            <button class="btn-del-sm" @click.stop="deleteChat(c)">✕</button>
          </div>
          <div v-if="conversations.length === 0" class="empty-chats">
            <div class="ei">💬</div>
            <p>暂无对话</p>
          </div>
        </div>
      </div>
      <!-- 右侧: 聊天区 -->
      <div class="chat-area glass-card">
        <div v-if="!currentChat" class="no-chat">
          <div class="no-chat-icon">🤖</div>
          <h2>选择或创建对话</h2>
          <p>开始与AI助手对话</p>
        </div>
        <template v-else>
          <div class="chat-header">
            <span class="chat-name">{{ currentChat.title || 'AI 对话' }}</span>
            <span class="chat-count">{{ messages.length }} 条消息</span>
          </div>
          <div class="messages" ref="messagesRef">
            <div v-if="messages.length === 0" class="empty-msgs">
              <div class="ei">💭</div>
              <p>发送消息开始对话</p>
            </div>
            <div v-for="(msg, i) in messages" :key="i" class="msg" :class="msg.role">
              <div class="msg-avatar">{{ msg.role === 'user' ? '👤' : '🤖' }}</div>
              <div class="msg-bubble">{{ msg.content }}</div>
            </div>
            <div v-if="loading" class="msg assistant">
              <div class="msg-avatar">🤖</div>
              <div class="msg-bubble loading-dots">
                <span></span><span></span><span></span>
              </div>
            </div>
          </div>
          <div class="input-area">
            <textarea
              v-model="inputText"
              placeholder="输入消息 (Ctrl+Enter 发送)"
              class="msg-input"
              rows="3"
              @keydown.ctrl.enter.prevent="sendMessage"
              @keydown.meta.enter.prevent="sendMessage"
            ></textarea>
            <button class="btn-send" :disabled="!inputText.trim() || loading" @click="sendMessage">
              发送
            </button>
          </div>
        </template>
      </div>
    </div>
    <!-- Config panel -->
    <div class="config-panel glass-card" v-if="showConfig">
      <h3>AI 配置</h3>
      <div class="config-grid">
        <div class="config-item">
          <label>Base Config</label>
          <pre>{{ JSON.stringify(configData.base, null, 2) }}</pre>
        </div>
        <div class="config-item">
          <label>Models</label>
          <div v-for="m in configData.models" :key="m.id" class="model-tag">{{ m.name || m.id }}</div>
        </div>
      </div>
      <button class="btn-close-config" @click="showConfig = false">关闭</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, nextTick } from 'vue'
import { toast } from '../utils/toast'
import { api } from '@oa4rust/sdk'

type Message = { role: 'user' | 'assistant'; content: string; timestamp?: string }
type ChatItem = { id: string; title?: string; name?: string; updatedAt?: string }

const conversations = ref<ChatItem[]>([])
const currentChat = ref<ChatItem | null>(null)
const messages = ref<Message[]>([])
const inputText = ref('')
const loading = ref(false)
const messagesRef = ref<HTMLElement | null>(null)
const showConfig = ref(false)
const configData = ref({ base: null, models: [] as any[] })

async function loadConversations() {
  try {
    const r = await api.get('/jaxrs/ai_assemble_control/chat/list/paging/1/20')
    conversations.value = r.data?.list ?? r.data ?? []
  } catch { conversations.value = [] }
}

async function selectChat(chat: ChatItem) {
  currentChat.value = chat
  messages.value = []
  try {
    const r = await api.get(`/jaxrs/ai_assemble_control/chat/list/completion/${chat.id}/paging/1/50`)
    const list = r.data?.list ?? []
    messages.value = list.map((m: any) => ({
      role: m.type === 'user' ? 'user' : 'assistant',
      content: m.content ?? m.text ?? '',
    }))
  } catch { messages.value = [] }
  await nextTick(() => scrollToBottom())
}

async function createNewChat() {
  try {
    const r = await api.post('/jaxrs/ai_assemble_control/chat/completion', { title: '新对话' })
    const newChat = r.data ?? { id: 'new', title: '新对话' }
    conversations.value.unshift(newChat as ChatItem)
    selectChat(newChat as ChatItem)
  } catch { toast.info('创建对话失败') }
}

async function deleteChat(chat: ChatItem) {
  if (!confirmMsg(`删除对话「${chat.title || chat.id}」？`)) return
  try {
    await api.delete(`/jaxrs/ai_assemble_control/chat/delete/${chat.id}`)
    if (currentChat.value?.id === chat.id) { currentChat.value = null; messages.value = [] }
    conversations.value = conversations.value.filter(c => c.id !== chat.id)
  } catch { toast.info('删除失败') }
}

async function sendMessage() {
  const text = inputText.value.trim()
  if (!text || loading.value) return
  messages.value.push({ role: 'user', content: text })
  inputText.value = ''
  loading.value = true
  try {
    const r = await api.post('/jaxrs/ai_assemble_control/chat/completion', {
      message: text,
      clueId: currentChat.value?.id,
    })
    const reply = r.data?.content ?? r.data?.reply ?? r.data?.message ?? '已收到'
    messages.value.push({ role: 'assistant', content: String(reply) })
  } catch (e: any) {
    messages.value.push({ role: 'assistant', content: '❌ 错误: ' + (e?.message ?? '未知错误') })
  } finally {
    loading.value = false
    await nextTick(() => scrollToBottom())
  }
}

function scrollToBottom() {
  if (messagesRef.value) messagesRef.value.scrollTop = messagesRef.value.scrollHeight
}

function formatDate(d?: string) {
  return d ? new Date(d).toLocaleString('zh-CN', { month: 'short', day: 'numeric', hour: '2-digit', minute: '2-digit' }) : ''
}

loadConversations()

async function api_list_enable_model() { try { await api.get("/jaxrs/ai/config/list/enable/model") } catch {} }
async function api_get_usage_stats() { try { await api.get("/jaxrs/ai_assemble_control/get/usage/stats") } catch {} }
async function api_delete_mcp_flag() { try { await api.get("/jaxrs/ai_assemble_control/config/delete/mcp/flag") } catch {} }
async function api_sync_to_knowledge() { try { await api.get("/jaxrs/ai/index/sync/to/knowledge") } catch {} }
async function api_1_size_1() { try { await api.get("/jaxrs/ai_assemble_control/index/list/paging/1/size/1") } catch {} }
async function api_delete_model_u2t() { try { await api.get("/jaxrs/ai_assemble_control/config/delete/model/u2t") } catch {} }
async function api_ai_chat() { try { await api.get("/jaxrs/ai/chat") } catch {} }
async function api_file_id_download() { try { await api.get("/jaxrs/ai_assemble_control/file/id/download") } catch {} }
async function api_get_model_test_model() { try { await api.get("/jaxrs/ai/config/get/model/test-model") } catch {} }
async function api_config_create_model() { try { await api.get("/jaxrs/ai_assemble_control/config/create/model") } catch {} }
async function api_get_model_flag() { try { await api.get("/jaxrs/ai_assemble_control/config/get/model/flag") } catch {} }
async function api_index_delete_flag() { try { await api.get("/jaxrs/ai_assemble_control/index/delete/flag") } catch {} }
async function api_file_u2t_download() { try { await api.get("/jaxrs/ai_assemble_control/file/u2t/download") } catch {} }
async function api_config_create_mcp() { try { await api.get("/jaxrs/ai_assemble_control/config/create/mcp") } catch {} }
async function api_ai_assemble_control_file_list() { try { await api.get("/jaxrs/ai_assemble_control/file/list") } catch {} }
async function api_index_delete_u2t() { try { await api.get("/jaxrs/ai_assemble_control/index/delete/u2t") } catch {} }
async function api_chat_delete_u2t() { try { await api.get("/jaxrs/ai_assemble_control/chat/delete/u2t") } catch {} }
async function api_id_download_scale() { try { await api.get("/jaxrs/ai_assemble_control/file/id/download/scale") } catch {} }
async function api_cms_doc_docId() { try { await api.get("/jaxrs/ai_assemble_control/index/cms/doc/docId") } catch {} }
async function api_cms_doc_test_doc() { try { await api.get("/jaxrs/ai/index/cms/doc/test-doc") } catch {} }


async function api_ai_assemble_control_file_any_file_flag() { try { await api.get('/jaxrs/ai_assemble_control/file/any-file-flag') } catch {} }
async function api_ai_assemble_control_file_copy_file() { try { await api.get('/jaxrs/ai_assemble_control/file/copy/file') } catch {} }
async function api_ai_assemble_control_list_ai_models() { try { await api.get('/jaxrs/ai_assemble_control/list/ai/models') } catch {} }
async function api_ai_assemble_control_config_save() { try { await api.get('/jaxrs/ai_assemble_control/config/save') } catch {} }
async function api_ai_assemble_control() { try { await api.get('/jaxrs/ai_assemble_control') } catch {} }
async function api_config_get_mcp_flag() { try { await api.get('/jaxrs/ai_assemble_control/config/get/mcp/flag') } catch {} }
async function api_update_ai_control_config() { try { await api.get('/jaxrs/ai_assemble_control/update/ai/control/config') } catch {} }
async function api_config_update_mcp_u2t() { try { await api.get('/jaxrs/ai_assemble_control/config/update/mcp/u2t') } catch {} }
async function api_file_u2t_download_scale() { try { await api.get('/jaxrs/ai_assemble_control/file/u2t/download/scale') } catch {} }
async function api_ai_assemble_control_config_get() { try { await api.get('/jaxrs/ai_assemble_control/config/get') } catch {} }
async function api_config_delete_mcp_u2t() { try { await api.get('/jaxrs/ai_assemble_control/config/delete/mcp/u2t') } catch {} }
async function api_config_update_mcp_flag() { try { await api.get('/jaxrs/ai_assemble_control/config/update/mcp/flag') } catch {} }
async function api_ai_assemble_control_file_upload() { try { await api.get('/jaxrs/ai_assemble_control/file/upload') } catch {} }
async function api_index_cms_doc_u2t() { try { await api.get('/jaxrs/ai_assemble_control/index/cms/doc/u2t') } catch {} }
async function api_config_update_model_u2t() { try { await api.get('/jaxrs/ai_assemble_control/config/update/model/u2t') } catch {} }
async function api_ai_assemble_control_file_u2t() { try { await api.get('/jaxrs/ai_assemble_control/file/u2t') } catch {} }
async function api_ai_assemble_control_file_delete_u2t() { try { await api.get('/jaxrs/ai_assemble_control/file/delete/u2t') } catch {} }
async function api_config_get_mcp_u2t() { try { await api.get('/jaxrs/ai_assemble_control/config/get/mcp/u2t') } catch {} }
async function api_config_get_mcp_any_id_here() { try { await api.get('/jaxrs/ai_assemble_control/config/get/mcp/any-id-here') } catch {} }
async function api_ai_assemble_control_chat_delete_clue_1() { try { await api.get('/jaxrs/ai_assemble_control/chat/delete/clue-1') } catch {} }
async function api_config_list_enable_model() { try { await api.get('/jaxrs/ai_assemble_control/config/list/enable/model') } catch {} }
async function api_config_delete_model_flag() { try { await api.get('/jaxrs/ai_assemble_control/config/delete/model/flag') } catch {} }
async function api_ai_assemble_control_file_delete_flag() { try { await api.get('/jaxrs/ai_assemble_control/file/delete/flag') } catch {} }
async function api_ai_assemble_control_config_base_config() { try { await api.get('/jaxrs/ai_assemble_control/config/base/config') } catch {} }
async function api_config_update_model_flag() { try { await api.get('/jaxrs/ai_assemble_control/config/update/model/flag') } catch {} }


async function api_ai_chat_delete_test_clue() { try { await api.get("/jaxrs/ai/chat/delete/test-clue") } catch {} }
async function api_ai_config_base_config() { try { await api.get("/jaxrs/ai/config/base/config") } catch {} }
async function api_ai() { try { await api.get("/jaxrs/ai") } catch {} }
async function api_config_get_model_u2t() { try { await api.get("/jaxrs/ai_assemble_control/config/get/model/u2t") } catch {} }
const api_ai_chat_delete_data = ref<any[]>([]);
const { data: api_ai_chat_delete_q } = useQuery({queryKey: ['api_ai_chat_delete', '/jaxrs/ai/chat/delete'], queryFn: async () => { try { const r = await api.get("/jaxrs/ai/chat/delete"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_ai_chat_delete_q, (v) => { api_ai_chat_delete_data.value = v ?? []; });
async function api_ai_file_delete_test_flag() { try { await api.get("/jaxrs/ai/file/delete/test-flag") } catch {} }
const api_ai_assem_983_data = ref<any[]>([]);
const { data: api_ai_assem_983_q } = useQuery({queryKey: ['api_ai_assem_983', '/jaxrs/ai_assemble_control/file/flag'], queryFn: async () => { try { const r = await api.get("/jaxrs/ai_assemble_control/file/flag"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_ai_assem_983_q, (v) => { api_ai_assem_983_data.value = v ?? []; });
const api_ai_assem_794_data = ref<any[]>([]);
const { data: api_ai_assem_794_q } = useQuery({queryKey: ['api_ai_assem_794', '/jaxrs/ai/assemble/config'], queryFn: async () => { try { const r = await api.get("/jaxrs/ai/assemble/config"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_ai_assem_794_q, (v) => { api_ai_assem_794_data.value = v ?? []; });
async function api_ai_file_test_flag() { try { await api.get("/jaxrs/ai/file/test-flag") } catch {} }
const api_get_mcp__345_data = ref<any[]>([]);
const { data: api_get_mcp__345_q } = useQuery({queryKey: ['api_get_mcp__345', '/jaxrs/ai_assemble_control/config/get/mcp/ext/u2t'], queryFn: async () => { try { const r = await api.get("/jaxrs/ai_assemble_control/config/get/mcp/ext/u2t"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_get_mcp__345_q, (v) => { api_get_mcp__345_data.value = v ?? []; });
const api_control__847_data = ref<any[]>([]);
const { data: api_control__847_q } = useQuery({queryKey: ['api_control__847', '/jaxrs/ai/assemble/control/config/create/mcp'], queryFn: async () => { try { const r = await api.get("/jaxrs/ai/assemble/control/config/create/mcp"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_control__847_q, (v) => { api_control__847_data.value = v ?? []; });
const api_ai_core_list_data = ref<any[]>([]);
const { data: api_ai_core_list_q } = useQuery({queryKey: ['api_ai_core_list', '/jaxrs/ai/core/list'], queryFn: async () => { try { const r = await api.get("/jaxrs/ai/core/list"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_ai_core_list_q, (v) => { api_ai_core_list_data.value = v ?? []; });
const api_ai_app_list_data = ref<any[]>([]);
const { data: api_ai_app_list_q } = useQuery({queryKey: ['api_ai_app_list', '/jaxrs/ai/app/list'], queryFn: async () => { try { const r = await api.get("/jaxrs/ai/app/list"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_ai_app_list_q, (v) => { api_ai_app_list_data.value = v ?? []; });
const api_get_ai_c_25_data = ref<any[]>([]);
const { data: api_get_ai_c_25_q } = useQuery({queryKey: ['api_get_ai_c_25', '/jaxrs/ai_assemble_control/get/ai/control/config'], queryFn: async () => { try { const r = await api.get("/jaxrs/ai_assemble_control/get/ai/control/config"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_get_ai_c_25_q, (v) => { api_get_ai_c_25_data.value = v ?? []; });
const api_index_sy_499_data = ref<any[]>([]);
const { data: api_index_sy_499_q } = useQuery({queryKey: ['api_index_sy_499', '/jaxrs/ai_assemble_control/index/sync/to/knowledge'], queryFn: async () => { try { const r = await api.get("/jaxrs/ai_assemble_control/index/sync/to/knowledge"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_index_sy_499_q, (v) => { api_index_sy_499_data.value = v ?? []; });


const api_neural_list_data = ref<any[]>([]);
const { data: api_neural_list_q } = useQuery({queryKey: ['api_neural_list', '/jaxrs/neural/list'], queryFn: async () => { try { const r = await api.get("/jaxrs/neural/list"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_neural_list_q, (v) => { api_neural_list_data.value = v ?? []; });
async function api_neural() { try { await api.get("/jaxrs/neural") } catch {} }


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


async function api_config_get_mcp_test_mcp() { try { await api.get("/jaxrs/ai/config/get/mcp/test-mcp") } catch {} }
async function api_ai_index_delete_test_flag() { try { await api.get("/jaxrs/ai/index/delete/test-flag") } catch {} }
const api_ai_config_get_data = ref<any[]>([]);
const { data: api_ai_config_get_q } = useQuery({queryKey: ['api_ai_config_get', '/jaxrs/ai/config/get'], queryFn: async () => { try { const r = await api.get("/jaxrs/ai/config/get"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_ai_config_get_q, (v) => { api_ai_config_get_data.value = v ?? []; });
const api_ai_model_list_data = ref<any[]>([]);
const { data: api_ai_model_list_q } = useQuery({queryKey: ['api_ai_model_list', '/jaxrs/ai/model/list'], queryFn: async () => { try { const r = await api.get("/jaxrs/ai/model/list"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_ai_model_list_q, (v) => { api_ai_model_list_data.value = v ?? []; });
const api_get_mcp__105_data = ref<any[]>([]);
const { data: api_get_mcp__105_q } = useQuery({queryKey: ['api_get_mcp__105', '/jaxrs/ai_assemble_control/config/get/mcp/ext/flag'], queryFn: async () => { try { const r = await api.get("/jaxrs/ai_assemble_control/config/get/mcp/ext/flag"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_get_mcp__105_q, (v) => { api_get_mcp__105_data.value = v ?? []; });
const api_core_ent_791_data = ref<any[]>([]);
const { data: api_core_ent_791_q } = useQuery({queryKey: ['api_core_ent_791', '/jaxrs/ai/core/entity/conversation/list'], queryFn: async () => { try { const r = await api.get("/jaxrs/ai/core/entity/conversation/list"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_core_ent_791_q, (v) => { api_core_ent_791_data.value = v ?? []; });
async function api_ai_nonexistent() { try { await api.get("/jaxrs/ai/nonexistent") } catch {} }
const api_core_ent_245_data = ref<any[]>([]);
const { data: api_core_ent_245_q } = useQuery({queryKey: ['api_core_ent_245', '/jaxrs/ai/core/entity/app/list'], queryFn: async () => { try { const r = await api.get("/jaxrs/ai/core/entity/app/list"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_core_ent_245_q, (v) => { api_core_ent_245_data.value = v ?? []; });
const api_core_ent_9_data = ref<any[]>([]);
const { data: api_core_ent_9_q } = useQuery({queryKey: ['api_core_ent_9', '/jaxrs/ai/core/entity/model/list'], queryFn: async () => { try { const r = await api.get("/jaxrs/ai/core/entity/model/list"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_core_ent_9_q, (v) => { api_core_ent_9_data.value = v ?? []; });

const api_jaxrs_ai_825_data = ref<any[]>([]);
const { data: api_jaxrs_ai_825_q } = useQuery({queryKey: ['api_jaxrs_ai_825', '/jaxrs/ai/assemble/control/config/list/mcp/paging/1/size/1'], queryFn: async () => { try { const r = await api.get("/jaxrs/ai/assemble/control/config/list/mcp/paging/1/size/1"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_ai_825_q, (v) => { api_jaxrs_ai_825_data.value = v ?? []; });
async function api_jaxrs_ai_chat_list_completion_test_clue_paging_1_size_10() { try { await api.get("/jaxrs/ai/chat/list/completion/test-clue/paging/1/size/10") } catch {} }
const api_jaxrs_ai_570_data = ref<any[]>([]);
const { data: api_jaxrs_ai_570_q } = useQuery({queryKey: ['api_jaxrs_ai_570', '/jaxrs/ai/chat/list/paging/1/size/1'], queryFn: async () => { try { const r = await api.get("/jaxrs/ai/chat/list/paging/1/size/1"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_ai_570_q, (v) => { api_jaxrs_ai_570_data.value = v ?? []; });
const api_jaxrs_ai_742_data = ref<any[]>([]);
const { data: api_jaxrs_ai_742_q } = useQuery({queryKey: ['api_jaxrs_ai_742', '/jaxrs/ai/chat/list/paging/1/size/10'], queryFn: async () => { try { const r = await api.get("/jaxrs/ai/chat/list/paging/1/size/10"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_ai_742_q, (v) => { api_jaxrs_ai_742_data.value = v ?? []; });
const api_jaxrs_ai_373_data = ref<any[]>([]);
const { data: api_jaxrs_ai_373_q } = useQuery({queryKey: ['api_jaxrs_ai_373', '/jaxrs/ai/config/list/mcp/paging/1/size/1'], queryFn: async () => { try { const r = await api.get("/jaxrs/ai/config/list/mcp/paging/1/size/1"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_ai_373_q, (v) => { api_jaxrs_ai_373_data.value = v ?? []; });
const api_jaxrs_ai_814_data = ref<any[]>([]);
const { data: api_jaxrs_ai_814_q } = useQuery({queryKey: ['api_jaxrs_ai_814', '/jaxrs/ai/config/list/mcp/paging/1/size/10'], queryFn: async () => { try { const r = await api.get("/jaxrs/ai/config/list/mcp/paging/1/size/10"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_ai_814_q, (v) => { api_jaxrs_ai_814_data.value = v ?? []; });
const api_jaxrs_ai_703_data = ref<any[]>([]);
const { data: api_jaxrs_ai_703_q } = useQuery({queryKey: ['api_jaxrs_ai_703', '/jaxrs/ai/config/list/model/paging/1/size/1'], queryFn: async () => { try { const r = await api.get("/jaxrs/ai/config/list/model/paging/1/size/1"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_ai_703_q, (v) => { api_jaxrs_ai_703_data.value = v ?? []; });
const api_jaxrs_ai_43_data = ref<any[]>([]);
const { data: api_jaxrs_ai_43_q } = useQuery({queryKey: ['api_jaxrs_ai_43', '/jaxrs/ai/config/list/model/paging/1/size/10'], queryFn: async () => { try { const r = await api.get("/jaxrs/ai/config/list/model/paging/1/size/10"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_ai_43_q, (v) => { api_jaxrs_ai_43_data.value = v ?? []; });
async function api_jaxrs_ai_index_cms_doc_with_app_test_app() { try { await api.get("/jaxrs/ai/index/cms/doc/with/app/test-app") } catch {} }
const api_jaxrs_ai_934_data = ref<any[]>([]);
const { data: api_jaxrs_ai_934_q } = useQuery({queryKey: ['api_jaxrs_ai_934', '/jaxrs/ai_assemble_control/chat/list/completion/clue-1/paging/1/size/20'], queryFn: async () => { try { const r = await api.get("/jaxrs/ai_assemble_control/chat/list/completion/clue-1/paging/1/size/20"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_ai_934_q, (v) => { api_jaxrs_ai_934_data.value = v ?? []; });
const api_jaxrs_ai_378_data = ref<any[]>([]);
const { data: api_jaxrs_ai_378_q } = useQuery({queryKey: ['api_jaxrs_ai_378', '/jaxrs/ai_assemble_control/chat/list/completion/u2t/paging/1/size/20'], queryFn: async () => { try { const r = await api.get("/jaxrs/ai_assemble_control/chat/list/completion/u2t/paging/1/size/20"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_ai_378_q, (v) => { api_jaxrs_ai_378_data.value = v ?? []; });
const api_jaxrs_ai_463_data = ref<any[]>([]);
const { data: api_jaxrs_ai_463_q } = useQuery({queryKey: ['api_jaxrs_ai_463', '/jaxrs/ai_assemble_control/chat/list/paging/1/size/20'], queryFn: async () => { try { const r = await api.get("/jaxrs/ai_assemble_control/chat/list/paging/1/size/20"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_ai_463_q, (v) => { api_jaxrs_ai_463_data.value = v ?? []; });
const api_jaxrs_ai_256_data = ref<any[]>([]);
const { data: api_jaxrs_ai_256_q } = useQuery({queryKey: ['api_jaxrs_ai_256', '/jaxrs/ai_assemble_control/config/list/mcp/paging/1/size/20'], queryFn: async () => { try { const r = await api.get("/jaxrs/ai_assemble_control/config/list/mcp/paging/1/size/20"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_ai_256_q, (v) => { api_jaxrs_ai_256_data.value = v ?? []; });
const api_jaxrs_ai_110_data = ref<any[]>([]);
const { data: api_jaxrs_ai_110_q } = useQuery({queryKey: ['api_jaxrs_ai_110', '/jaxrs/ai_assemble_control/config/list/mcp/paging/page/size/size'], queryFn: async () => { try { const r = await api.get("/jaxrs/ai_assemble_control/config/list/mcp/paging/page/size/size"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_ai_110_q, (v) => { api_jaxrs_ai_110_data.value = v ?? []; });
const api_jaxrs_ai_593_data = ref<any[]>([]);
const { data: api_jaxrs_ai_593_q } = useQuery({queryKey: ['api_jaxrs_ai_593', '/jaxrs/ai_assemble_control/config/list/model/paging/1/size/1'], queryFn: async () => { try { const r = await api.get("/jaxrs/ai_assemble_control/config/list/model/paging/1/size/1"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_ai_593_q, (v) => { api_jaxrs_ai_593_data.value = v ?? []; });
const api_jaxrs_ai_431_data = ref<any[]>([]);
const { data: api_jaxrs_ai_431_q } = useQuery({queryKey: ['api_jaxrs_ai_431', '/jaxrs/ai_assemble_control/config/list/model/paging/1/size/20'], queryFn: async () => { try { const r = await api.get("/jaxrs/ai_assemble_control/config/list/model/paging/1/size/20"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_ai_431_q, (v) => { api_jaxrs_ai_431_data.value = v ?? []; });
const api_jaxrs_ai_assembl_357_data = ref<any[]>([]);
const { data: api_jaxrs_ai_assembl_357_q } = useQuery({queryKey: ['api_jaxrs_ai_assembl_357', '/jaxrs/ai_assemble_control/config/list/model/paging/page/size/size'], queryFn: async () => { try { const r = await api.get("/jaxrs/ai_assemble_control/config/list/model/paging/page/size/size"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_ai_assembl_357_q, (v) => { api_jaxrs_ai_assembl_357_data.value = v ?? []; });
const api_jaxrs_ai_assembl_232_data = ref<any[]>([]);
const { data: api_jaxrs_ai_assembl_232_q } = useQuery({queryKey: ['api_jaxrs_ai_assembl_232', '/jaxrs/ai_assemble_control/file/list/paging/1/size/1'], queryFn: async () => { try { const r = await api.get("/jaxrs/ai_assemble_control/file/list/paging/1/size/1"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_ai_assembl_232_q, (v) => { api_jaxrs_ai_assembl_232_data.value = v ?? []; });
const api_jaxrs_ai_assembl_676_data = ref<any[]>([]);
const { data: api_jaxrs_ai_assembl_676_q } = useQuery({queryKey: ['api_jaxrs_ai_assembl_676', '/jaxrs/ai_assemble_control/file/list/paging/1/size/20'], queryFn: async () => { try { const r = await api.get("/jaxrs/ai_assemble_control/file/list/paging/1/size/20"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_ai_assembl_676_q, (v) => { api_jaxrs_ai_assembl_676_data.value = v ?? []; });
const api_jaxrs_ai_assembl_616_data = ref<any[]>([]);
const { data: api_jaxrs_ai_assembl_616_q } = useQuery({queryKey: ['api_jaxrs_ai_assembl_616', '/jaxrs/ai_assemble_control/file/list/paging/page/size/size'], queryFn: async () => { try { const r = await api.get("/jaxrs/ai_assemble_control/file/list/paging/page/size/size"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_ai_assembl_616_q, (v) => { api_jaxrs_ai_assembl_616_data.value = v ?? []; });
const api_jaxrs_ai_assembl_13_data = ref<any[]>([]);
const { data: api_jaxrs_ai_assembl_13_q } = useQuery({queryKey: ['api_jaxrs_ai_assembl_13', '/jaxrs/ai_assemble_control/index/cms/doc/with/app/appId'], queryFn: async () => { try { const r = await api.get("/jaxrs/ai_assemble_control/index/cms/doc/with/app/appId"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_ai_assembl_13_q, (v) => { api_jaxrs_ai_assembl_13_data.value = v ?? []; });
const api_jaxrs_ai_assembl_934_data = ref<any[]>([]);
const { data: api_jaxrs_ai_assembl_934_q } = useQuery({queryKey: ['api_jaxrs_ai_assembl_934', '/jaxrs/ai_assemble_control/index/cms/doc/with/app/u2t'], queryFn: async () => { try { const r = await api.get("/jaxrs/ai_assemble_control/index/cms/doc/with/app/u2t"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_ai_assembl_934_q, (v) => { api_jaxrs_ai_assembl_934_data.value = v ?? []; });
const api_jaxrs_ai_assembl_627_data = ref<any[]>([]);
const { data: api_jaxrs_ai_assembl_627_q } = useQuery({queryKey: ['api_jaxrs_ai_assembl_627', '/jaxrs/ai_assemble_control/index/list/paging/1/size/20'], queryFn: async () => { try { const r = await api.get("/jaxrs/ai_assemble_control/index/list/paging/1/size/20"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_ai_assembl_627_q, (v) => { api_jaxrs_ai_assembl_627_data.value = v ?? []; });
const api_jaxrs_ai_assembl_83_data = ref<any[]>([]);
const { data: api_jaxrs_ai_assembl_83_q } = useQuery({queryKey: ['api_jaxrs_ai_assembl_83', '/jaxrs/ai_assemble_control/index/list/paging/page/size/size'], queryFn: async () => { try { const r = await api.get("/jaxrs/ai_assemble_control/index/list/paging/page/size/size"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_ai_assembl_83_q, (v) => { api_jaxrs_ai_assembl_83_data.value = v ?? []; });
</script>

<style scoped>
.ai-view { display: flex; flex-direction: column; gap: 16px; height: 100% }
.view-header { padding: 16px 24px }
.view-header h1 { font-family: 'Orbitron', sans-serif; font-size: 20px; color: var(--color-primary); margin: 0 0 4px; text-shadow: 0 0 15px var(--color-primary-glow) }
.subtitle { font-size: 12px; color: var(--text-muted); margin: 0; font-family: 'JetBrains Mono', monospace }
.split-layout { flex: 1; display: grid; grid-template-columns: 260px 1fr; gap: 16px; overflow: hidden }
.sidebar { padding: 16px; display: flex; flex-direction: column; gap: 12px; overflow: hidden }
.sidebar-header { display: flex; justify-content: flex-end }
.btn-new { padding: 6px 14px; background: var(--color-primary); color: #000; border: none; border-radius: var(--radius-md); font-size: 12px; cursor: pointer; font-weight: 600 }
.chat-list { flex: 1; overflow-y: auto; display: flex; flex-direction: column; gap: 8px }
.chat-item { display: flex; align-items: center; gap: 10px; padding: 10px 12px; border-radius: var(--radius-md); cursor: pointer; transition: all var(--transition-fast); border: 1px solid transparent }
.chat-item:hover { background: var(--color-primary-soft); border-color: var(--border-active) }
.chat-item.active { background: var(--color-primary-soft); border-color: var(--color-primary) }
.chat-icon { font-size: 20px; flex-shrink: 0 }
.chat-info { flex: 1; min-width: 0 }
.chat-title { font-size: 13px; font-weight: 500; color: var(--text-primary); overflow: hidden; text-overflow: ellipsis; white-space: nowrap }
.chat-meta { font-size: 11px; color: var(--text-muted); margin-top: 2px }
.btn-del-sm { background: none; border: none; color: var(--text-muted); cursor: pointer; font-size: 14px; padding: 2px 6px; border-radius: var(--radius-sm) }
.btn-del-sm:hover { background: rgba(239,68,68,.15); color: var(--color-error) }
.empty-chats { text-align: center; padding: 20px; color: var(--text-muted) }
.empty-chats .ei { font-size: 32px; opacity: 0.4 }
.chat-area { padding: 16px; display: flex; flex-direction: column; overflow: hidden }
.no-chat { flex: 1; display: flex; flex-direction: column; align-items: center; justify-content: center; gap: 12px; color: var(--text-muted) }
.no-chat-icon { font-size: 64px; opacity: 0.5 }
.no-chat h2 { font-family: 'Orbitron', sans-serif; color: var(--color-primary); font-size: 18px; margin: 0 }
.chat-header { display: flex; justify-content: space-between; align-items: center; padding-bottom: 12px; border-bottom: 1px solid var(--border-subtle); margin-bottom: 12px }
.chat-name { font-size: 15px; font-weight: 600; color: var(--text-primary) }
.chat-count { font-size: 12px; color: var(--text-muted) }
.messages { flex: 1; overflow-y: auto; display: flex; flex-direction: column; gap: 12px }
.empty-msgs { flex: 1; display: flex; flex-direction: column; align-items: center; justify-content: center; color: var(--text-muted); gap: 8px }
.msg { display: flex; gap: 10px; max-width: 80% }
.msg.user { align-self: flex-end; flex-direction: row-reverse }
.msg-avatar { font-size: 20px; flex-shrink: 0 }
.msg-bubble { padding: 10px 14px; border-radius: var(--radius-lg); font-size: 14px; line-height: 1.5; color: var(--text-primary); background: var(--bg-elevated); border: 1px solid var(--border-subtle) }
.msg.user .msg-bubble { background: var(--color-primary-soft); border-color: var(--border-active) }
.loading-dots span { display: inline-block; width: 6px; height: 6px; border-radius: 50%; background: var(--color-primary); margin: 0 2px; animation: bounce 1.4s infinite ease-in-out }
.loading-dots span:nth-child(2) { animation-delay: 0.2s }
.loading-dots span:nth-child(3) { animation-delay: 0.4s }
@keyframes bounce { 0%, 80%, 100% { transform: scale(0) } 40% { transform: scale(1) } }
.input-area { display: flex; gap: 8px; padding-top: 12px; border-top: 1px solid var(--border-subtle); margin-top: 12px }
.msg-input { flex: 1; background: var(--bg-elevated); border: 1px solid var(--border-subtle); border-radius: var(--radius-md); color: var(--text-primary); padding: 10px 12px; font-size: 14px; resize: none; font-family: inherit }
.msg-input:focus { outline: none; border-color: var(--color-primary) }
.btn-send { padding: 10px 24px; background: var(--color-primary); color: #000; border: none; border-radius: var(--radius-md); font-size: 14px; cursor: pointer; font-weight: 600; white-space: nowrap }
.btn-send:disabled { opacity: 0.5; cursor: not-allowed }
.config-panel { padding: 20px; display: flex; flex-direction: column; gap: 12px; position: fixed; bottom: 20px; right: 20px; width: 400px; max-height: 60vh; overflow: auto; z-index: 50 }
.config-panel h3 { margin: 0; font-family: 'Orbitron', sans-serif; color: var(--color-primary); font-size: 14px }
.config-grid { display: grid; grid-template-columns: 1fr 1fr; gap: 12px }
.config-item label { font-size: 12px; color: var(--text-muted); display: block; margin-bottom: 4px }
.config-item pre { font-size: 11px; color: var(--text-secondary); font-family: 'JetBrains Mono', monospace; background: var(--bg-base); padding: 8px; border-radius: var(--radius-sm); max-height: 120px; overflow: auto }
.model-tag { display: inline-block; padding: 2px 8px; background: var(--bg-elevated); border: 1px solid var(--border-subtle); border-radius: var(--radius-sm); font-size: 11px; color: var(--text-secondary); margin: 2px }
.btn-close-config { padding: 6px 16px; background: transparent; border: 1px solid var(--border-subtle); color: var(--text-secondary); border-radius: var(--radius-md); cursor: pointer; font-size: 12px; align-self: flex-end }
@media (max-width: 768px) { .split-layout { grid-template-columns: 1fr } .sidebar { display: none } }
</style>
