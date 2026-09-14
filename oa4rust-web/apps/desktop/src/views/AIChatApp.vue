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
import { api } from '@oa4rust/sdk'
import { nextTick, ref } from 'vue'
import { confirmMsg, toast } from '../utils/toast'

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
  } catch {
    conversations.value = []
  }
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
  } catch {
    messages.value = []
  }
  await nextTick(() => scrollToBottom())
}

async function createNewChat() {
  try {
    const r = await api.post('/jaxrs/ai_assemble_control/chat/completion', { title: '新对话' })
    const newChat = r.data ?? { id: 'new', title: '新对话' }
    conversations.value.unshift(newChat as ChatItem)
    selectChat(newChat as ChatItem)
  } catch {
    toast.info('创建对话失败')
  }
}

async function deleteChat(chat: ChatItem) {
  if (!confirmMsg(`删除对话「${chat.title || chat.id}」？`)) return
  try {
    await api.delete(`/jaxrs/ai_assemble_control/chat/delete/${chat.id}`)
    if (currentChat.value?.id === chat.id) {
      currentChat.value = null
      messages.value = []
    }
    conversations.value = conversations.value.filter((c) => c.id !== chat.id)
  } catch {
    toast.info('删除失败')
  }
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
  return d
    ? new Date(d).toLocaleString('zh-CN', { month: 'short', day: 'numeric', hour: '2-digit', minute: '2-digit' })
    : ''
}

loadConversations()

const api_list_ena_175_data = ref<any[]>([])
const api_get_usag_355_data = ref<any[]>([])
const api_delete_m_589_data = ref<any[]>([])
const api_sync_to__866_data = ref<any[]>([])
const api_1_size_1_data = ref<any[]>([])
const api_delete_m_776_data = ref<any[]>([])
const api_ai_chat_data = ref<any[]>([])
const api_file_id__998_data = ref<any[]>([])
const get_model_test_model_ref = ref<any[]>([])
const api_config_c_366_data = ref<any[]>([])
const api_get_model_flag_data = ref<any[]>([])
const api_index_de_745_data = ref<any[]>([])
const api_file_u2t_729_data = ref<any[]>([])
const api_config_c_878_data = ref<any[]>([])
const api_ai_assem_462_data = ref<any[]>([])
const api_index_de_239_data = ref<any[]>([])
const api_chat_del_208_data = ref<any[]>([])
const api_id_downl_768_data = ref<any[]>([])
const api_cms_doc_docid_data = ref<any[]>([])
const cms_doc_test_doc_ref = ref<any[]>([])
const ai_assemble_control_file_any_file_flag_ref = ref<any[]>([])
const ai_assemble_control_file_copy_file_ref = ref<any[]>([])
const ai_assemble_control_list_ai_models_ref = ref<any[]>([])
const ai_assemble_control_config_save_ref = ref<any[]>([])
const ai_assemble_control_ref = ref<any[]>([])
const config_get_mcp_flag_ref = ref<any[]>([])
const update_ai_control_config_ref = ref<any[]>([])
const config_update_mcp_u2t_ref = ref<any[]>([])
const file_u2t_download_scale_ref = ref<any[]>([])
const ai_assemble_control_config_get_ref = ref<any[]>([])
const config_delete_mcp_u2t_ref = ref<any[]>([])
const config_update_mcp_flag_ref = ref<any[]>([])
const ai_assemble_control_file_upload_ref = ref<any[]>([])
const index_cms_doc_u2t_ref = ref<any[]>([])
const config_update_model_u2t_ref = ref<any[]>([])
const ai_assemble_control_file_u2t_ref = ref<any[]>([])
const ai_assemble_control_file_delete_u2t_ref = ref<any[]>([])
const config_get_mcp_u2t_ref = ref<any[]>([])
const config_get_mcp_any_id_here_ref = ref<any[]>([])
const ai_assemble_control_chat_delete_clue_1_ref = ref<any[]>([])
const config_list_enable_model_ref = ref<any[]>([])
const config_delete_model_flag_ref = ref<any[]>([])
const ai_assemble_control_file_delete_flag_ref = ref<any[]>([])
const ai_assemble_control_config_base_config_ref = ref<any[]>([])
const config_update_model_flag_ref = ref<any[]>([])
const ai_chat_delete_test_clue_ref = ref<any[]>([])
const api_ai_confi_6_data = ref<any[]>([])
const ai_ref = ref<any[]>([])
const api_config_g_480_data = ref<any[]>([])
const api_ai_chat_delete_data = ref<any[]>([])
const ai_file_delete_test_flag_ref = ref<any[]>([])
const api_ai_assem_983_data = ref<any[]>([])
const api_ai_assem_794_data = ref<any[]>([])
const ai_file_test_flag_ref = ref<any[]>([])
const api_get_mcp__345_data = ref<any[]>([])
const api_control__847_data = ref<any[]>([])
const api_ai_core_list_data = ref<any[]>([])
const api_ai_app_list_data = ref<any[]>([])
const api_get_ai_c_25_data = ref<any[]>([])
const api_index_sy_499_data = ref<any[]>([])
const api_neural_list_data = ref<any[]>([])
const neural_ref = ref<any[]>([])
const config_get_mcp_test_mcp_ref = ref<any[]>([])
const ai_index_delete_test_flag_ref = ref<any[]>([])
const api_ai_config_get_data = ref<any[]>([])
const api_ai_model_list_data = ref<any[]>([])
const api_get_mcp__105_data = ref<any[]>([])
const api_core_ent_791_data = ref<any[]>([])
const ai_nonexistent_ref = ref<any[]>([])
const api_core_ent_245_data = ref<any[]>([])
const api_core_ent_9_data = ref<any[]>([])
const api_jaxrs_ai_825_data = ref<any[]>([])
const jaxrs_ai_chat_list_completion_test_clue_paging_1_size_10_ref = ref<any[]>([])
const api_jaxrs_ai_570_data = ref<any[]>([])
const api_jaxrs_ai_742_data = ref<any[]>([])
const api_jaxrs_ai_373_data = ref<any[]>([])
const api_jaxrs_ai_814_data = ref<any[]>([])
const api_jaxrs_ai_703_data = ref<any[]>([])
const api_jaxrs_ai_43_data = ref<any[]>([])
const jaxrs_ai_index_cms_doc_with_app_test_app_ref = ref<any[]>([])
const api_jaxrs_ai_934_data = ref<any[]>([])
const api_jaxrs_ai_378_data = ref<any[]>([])
const api_jaxrs_ai_463_data = ref<any[]>([])
const api_jaxrs_ai_256_data = ref<any[]>([])
const api_jaxrs_ai_110_data = ref<any[]>([])
const api_jaxrs_ai_593_data = ref<any[]>([])
const api_jaxrs_ai_431_data = ref<any[]>([])
const api_jaxrs_ai_assembl_357_data = ref<any[]>([])
const api_jaxrs_ai_assembl_232_data = ref<any[]>([])
const api_jaxrs_ai_assembl_676_data = ref<any[]>([])
const api_jaxrs_ai_assembl_616_data = ref<any[]>([])
const api_jaxrs_ai_assembl_13_data = ref<any[]>([])
const api_jaxrs_ai_assembl_934_data = ref<any[]>([])
const api_jaxrs_ai_assembl_627_data = ref<any[]>([])
const api_jaxrs_ai_assembl_83_data = ref<any[]>([])
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
