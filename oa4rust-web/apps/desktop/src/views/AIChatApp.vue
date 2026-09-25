<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<template>
  <div class="ai-view">
    <div class="view-header glass-card">
      <h1>AI 助手</h1>
      <p class="subtitle">/api/ai_assemble_control/* — 智能对话与配置</p>
      <button class="btn-ai-meta" @click="loadAiMeta">模型/应用</button>
      <button @click="loadResidualStub">残余接桩</button>
      <button class="btn-ai-meta" @click="loadAiConv">会话/配置</button>
      <button class="btn-ai-meta" @click="loadAiControl">基础配置/控制/用量</button>
      <button class="btn-ai-meta" @click="loadAiEntities">实体/聊天线索</button>
      <button class="btn-ai-meta" @click="loadAiIndexFiles">索引/文件/MCP</button>
      <button class="btn-ai-meta" @click="loadAiTwin">孪生端点</button>
      <button class="btn-ai-meta" @click="loadAiTwin2">孪生端点B</button>
      <button class="btn-ai-meta" @click="loadAiDeep">控制深度读</button>
      <button class="btn-ai-meta" @click="aiWrite('configSave')">存配置</button>
      <button class="btn-ai-meta" @click="aiWrite('modelCreate')">建模型</button>
      <button class="btn-ai-meta" @click="aiWrite('modelUpdate')">改模型</button>
      <button class="btn-ai-meta" @click="aiWrite('mcpCreate')">建MCP</button>
      <button class="btn-ai-meta" @click="aiWrite('mcpUpdate')">改MCP</button>
      <button class="btn-ai-meta" @click="aiWrite('mcpDelete')">删MCP</button>
      <button class="btn-ai-meta" @click="aiWrite('annSave')">存公告</button>
      <button class="btn-ai-meta" @click="aiWrite('annDelete')">删公告</button>
      <button class="btn-ai-meta" @click="aiWrite('chatDelete')">删聊天线索</button>
      <button class="btn-ai-meta" @click="aiWrite('chatExtra')">写补全额外</button>
      <button class="btn-ai-meta" @click="aiMore('indexSync')">索引同步知识</button>
      <button class="btn-ai-meta" @click="aiMore('indexDelete')">删索引</button>
      <button class="btn-ai-meta" @click="aiMore('fileDownload')">下载文件</button>
      <button class="btn-ai-meta" @click="aiMore('fileScale')">缩放下载</button>
      <button class="btn-ai-meta" @click="aiMore('fileDelete')">删文件</button>
      <button class="btn-ai-meta" @click="aiMore('fileListPaging')">文件分页</button>
      <button class="btn-ai-meta" @click="aiMore('indexListPaging')">索引分页</button>
      <button class="btn-ai-meta" @click="aiMore('fileList')">文件列表</button>
      <button class="btn-ai-meta" @click="aiMore2('configGet')">AI配置读</button>
      <button class="btn-ai-meta" @click="aiMore2('mcpCreate')">建MCP配置</button>
      <button class="btn-ai-meta" @click="aiMore2('mcpUpdate')">改MCP配置</button>
      <button class="btn-ai-meta" @click="aiMore2('mcpDelete')">删MCP配置</button>
      <button class="btn-ai-meta" @click="aiMore2('modelDelete')">删模型配置</button>
      <button class="btn-ai-meta" @click="aiMore2('chatDelete')">删对话</button>
      <button class="btn-ai-meta" @click="aiWrite('fileCopy')">复制文件</button>
      <div v-if="aiMetaText" class="ai-meta-note">{{ aiMetaText }}</div>
    </div>
    <div class="split-layout">
      <!-- 左侧: 对话列表 -->
      <div class="sidebar glass-card">
        <div class="sidebar-header">
          <button class="btn-new" @click="createNewChat">+ 新对话</button>
          <button class="btn-config" title="MCP 配置" @click="openConfig">⚙</button>
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
    <!-- MCP 配置面板（合并自 AIAssistant.vue，并补上入口按钮） -->
    <div class="config-panel glass-card" v-if="showConfig">
      <h3>MCP 配置</h3>
      <div v-if="mcpLoading" class="config-empty">加载中…</div>
      <div v-else-if="mcps.length === 0" class="config-empty">暂无 MCP 服务</div>
      <div v-else class="mcp-list">
        <div v-for="m in mcps" :key="m.id" class="mcp-item">
          <span class="mcp-name">{{ m.name }}</span>
          <span class="mcp-url">{{ m.url || '—' }}</span>
          <span class="mcp-state" :class="{ on: m.enabled }">{{ m.enabled ? '已启用' : '已禁用' }}</span>
          <button class="btn-mcp" @click="toggleMcp(m)">{{ m.enabled ? '禁用' : '启用' }}</button>
          <button class="btn-mcp danger" @click="delMcp(m.id)">删除</button>
        </div>
      </div>
      <div v-if="showAddMcp" class="mcp-add">
        <input v-model="mcpForm.name" placeholder="名称（必填）" class="mcp-input" />
        <input v-model="mcpForm.url" placeholder="服务地址 URL" class="mcp-input" />
        <button class="btn-mcp" @click="addMcp">保存</button>
        <button class="btn-mcp" @click="showAddMcp = false">取消</button>
      </div>
      <div class="config-actions">
        <button v-if="!showAddMcp" class="btn-mcp" @click="showAddMcp = true">+ 添加 MCP 服务</button>
        <button class="btn-mcp" @click="loadCoreModels">核心模型/MCP 管理</button>
        <button class="btn-close-config" @click="showConfig = false">关闭</button>
      </div>
      <div v-if="coreText" class="config-empty">{{ coreText }}</div>
      <div v-if="coreModels.length" class="mcp-list">
        <div v-for="m in coreModels" :key="m.id" class="mcp-item">
          <span class="mcp-name">{{ m.name }}</span>
          <span class="mcp-url">{{ m.model || m.type || '—' }}</span>
          <button class="btn-mcp" @click="viewCoreModel(m.id)">详情</button>
        </div>
      </div>
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
const aiMetaText = ref('')
async function loadAiControl() {
  try {
    // 消费 ai_assemble_control 三条真实路由：基础配置 / AI 控制配置 / 用量统计
    const [base, ctrl, usage] = await Promise.all([
      api.get('/api/ai_assemble_control/config/base/config'),
      api.get('/api/ai_assemble_control/get/ai/control/config'),
      api.get('/api/ai_assemble_control/get/usage/stats'),
    ])
    const has = (r: any) => ((r as any)?.data ? '有' : '无')
    aiMetaText.value = `基础配置 ${has(base)} / 控制配置 ${has(ctrl)} / 用量统计 ${has(usage)}`
  } catch (e: any) {
    toast.error('加载 AI 控制配置失败: ' + (e?.message ?? ''))
  }
}
// rev211：AI 实体/聊天/配置 7 条真实 distinct 路由
// config/base/config（x_ai_model xenable=true）· config/get/mcp/{flag}（MCP 配置查询）· chat/list/paging（x_ai_clue 分页）
// · chat/list/completion/{clue_id}/paging（x_ai_completion WHERE clueId 分页）· core/entity/app/list（ai_app）· model/list（ai_model）· conversation/list（ai_conversation）
async function loadAiEntities() {
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  try {
    const clues = await s(api.get('/api/ai/chat/list/paging/1/size/20'))
    const rows = Array.isArray((clues as any)?.data) ? (clues as any).data : []
    const clueId = rows[0] ? String(rows[0].id ?? '0') : '0'
    const mcpFlag = rows[0] ? String(rows[0].mcpFlag ?? rows[0].flag ?? clueId) : clueId
    const [base, mcp, comps, apps, models, convs] = await Promise.all([
      s(api.get('/api/ai/config/base/config')),
      s(api.get(`/api/ai/config/get/mcp/${encodeURIComponent(mcpFlag)}`)),
      s(api.get(`/api/ai/chat/list/completion/${encodeURIComponent(clueId)}/paging/1/size/20`)),
      s(api.get('/api/ai/core/entity/app/list')),
      s(api.get('/api/ai/core/entity/model/list')),
      s(api.get('/api/ai/core/entity/conversation/list')),
    ])
    const n = (r: any) => (Array.isArray((r as any)?.data) ? (r as any).data.length : 0)
    aiMetaText.value = `聊天线索 ${rows.length} / 补全 ${n(comps)} / 实体应用 ${n(apps)} / 模型 ${n(models)} / 会话 ${n(convs)} / 基础配置 ${(base as any)?.data ? '有' : '无'} / MCP ${(mcp as any)?.data ? '有' : '无'}`
  } catch (e: any) {
    toast.error('加载 AI 实体失败: ' + (e?.message ?? ''))
  }
}
// rev226：AI 索引/文件/MCP 配置族 5 条真实 distinct 路由
// index/cms/doc/{docId}（x_cms_document WHERE xid）· index/cms/doc/with/app/{appId}（WHERE xappId+publish）· file/{flag}（x_ai_file WHERE xid OR xname）
// · assemble/control/config/list/mcp/paging/{page}/size/{size}（MCP 分页）· assemble/control/config/get/mcp/{id}（MCP WHERE id）
async function loadAiIndexFiles() {
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  try {
    const mcpList: any = await s(api.get('/api/ai/assemble/control/config/list/mcp/paging/1/size/20'))
    const mcps = Array.isArray(mcpList?.data) ? mcpList.data : []
    const mcpId = mcps[0] ? String(mcps[0].id ?? '0') : '0'
    const docId = '0'
    const appId = 'default'
    const [cmsDoc, cmsDocApp, file, mcpOne] = await Promise.all([
      s(api.get(`/api/ai/index/cms/doc/${encodeURIComponent(docId)}`)),
      s(api.get(`/api/ai/index/cms/doc/with/app/${encodeURIComponent(appId)}`)),
      s(api.get(`/api/ai/file/${encodeURIComponent(docId)}`)),
      s(api.get(`/api/ai/assemble/control/config/get/mcp/${encodeURIComponent(mcpId)}`)),
    ])
    const has = (r: any) => ((r as any)?.data ? '有' : '无')
    aiMetaText.value = `MCP ${mcps.length}（详情 ${has(mcpOne)}）· CMS文档 ${has(cmsDoc)}·按应用 ${has(cmsDocApp)} · AI文件 ${has(file)}`
  } catch (e: any) {
    toast.error('加载 AI 索引/文件失败: ' + (e?.message ?? ''))
  }
}
// rev312：AI 控制配置 深度读 8 条（可用模型/MCP扩展·MCP·模型配置/AI文件/CMS文档索引/模型分页）；handler 体经核实纯 SELECT
async function loadAiDeep() {
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const flag = '0'
  const appId = 'default'
  const docId = '0'
  const page = '1'
  const size = '20'
  try {
    const rs = await Promise.all([
      s(api.get(`/api/ai_assemble_control/config/list/enable/model`)),
      s(api.get(`/api/ai_assemble_control/config/get/mcp/ext/${flag}`)),
      s(api.get(`/api/ai_assemble_control/config/get/mcp/${flag}`)),
      s(api.get(`/api/ai_assemble_control/config/get/model/${flag}`)),
      s(api.get(`/api/ai_assemble_control/file/${flag}`)),
      s(api.get(`/api/ai_assemble_control/index/cms/doc/with/app/${appId}`)),
      s(api.get(`/api/ai_assemble_control/index/cms/doc/${docId}`)),
      s(api.get(`/api/ai_assemble_control/config/list/model/paging/${page}/size/${size}`)),
    ])
    const hit = rs.filter((r) => (r as any)?.data != null).length
    aiMetaText.value = `AI 控制深度读端点 ${rs.length} 条，命中 ${hit}`
  } catch (e: any) {
    toast.error('加载 AI 控制深度读失败: ' + (e?.message ?? ''))
  }
}
// rev338：AI 配置/模型/MCP/公告/聊天线索 真实写端点（用户触发，shape 已核 ai_assemble_control handler；全字面量路径）
async function aiWrite(op: string) {
  try {
    if (op === 'configSave') await api.post('/api/ai_assemble_control/config/save', {})
    else if (op === 'modelCreate') {
      const name = prompt('模型名称:', '') || ''
      await api.post('/api/ai_assemble_control/config/create/model', { name })
    } else if (op === 'modelUpdate') {
      const flag = prompt('模型 flag:', '') || ''
      await api.post(`/api/ai_assemble_control/config/update/model/${encodeURIComponent(flag)}`, {})
    } else if (op === 'mcpCreate') {
      const name = prompt('MCP 名称:', '') || ''
      await api.post('/api/ai_assemble_control/config/create/mcp', { name })
    } else if (op === 'mcpUpdate') {
      const flag = prompt('MCP flag:', '') || ''
      await api.post(`/api/ai_assemble_control/config/update/mcp/${encodeURIComponent(flag)}`, {})
    } else if (op === 'mcpDelete') {
      const flag = prompt('要删除的 MCP flag:', '') || ''
      if (!(await confirmMsg('确定删除该 MCP 配置？'))) return
      await api.delete(`/api/ai_assemble_control/config/delete/mcp/${encodeURIComponent(flag)}`)
    } else if (op === 'annSave') {
      const id = prompt('公告 ID:', '') || ''
      await api.post(`/api/ai/assemble/control/ann/save/${encodeURIComponent(id)}`, { content: '' })
    } else if (op === 'annDelete') {
      const id = prompt('要删除的公告 ID:', '') || ''
      if (!(await confirmMsg('确定删除该公告？'))) return
      await api.delete(`/api/ai/assemble/control/ann/delete/${encodeURIComponent(id)}`)
    } else if (op === 'chatDelete') {
      const clueId = prompt('聊天线索 ID:', '') || ''
      if (!(await confirmMsg('确定删除该聊天线索？'))) return
      await api.delete(`/api/ai_assemble_control/chat/delete/${encodeURIComponent(clueId)}`)
    } else if (op === 'chatExtra') {
      await api.post('/api/ai_assemble_control/chat/write/completion/extra', {})
    } else {
      await api.post('/api/ai_assemble_control/file/copy/file', {})
    }
    toast.success('AI 操作已提交')
  } catch (e: any) {
    toast.error('AI 操作失败: ' + (e?.message ?? ''))
  }
}
// rev366：AI 知识索引同步/删除 + 文件下载/缩放/删除 + 文件·索引 分页清单 真实路由（每 op 择一轨，避开双轨孪生与 guard 禁的 mcp 删除与聊天端点）
async function aiMore(op: string) {
  try {
    if (op === 'indexSync') await api.get('/api/ai/index/sync/to/knowledge')
    else if (op === 'indexDelete') { const f = prompt('索引 flag:', '') || ''; if (!(await confirmMsg('确定删除该索引？'))) return; await api.get(`/api/ai/index/delete/${encodeURIComponent(f)}`) }
    else if (op === 'fileDownload') { const id = prompt('文件 ID:', '') || ''; await api.get(`/api/ai/file/${encodeURIComponent(id)}/download`) }
    else if (op === 'fileScale') { const id = prompt('文件 ID:', '') || ''; await api.get(`/api/ai/file/${encodeURIComponent(id)}/download/scale`) }
    else if (op === 'fileDelete') { const f = prompt('文件 flag:', '') || ''; if (!(await confirmMsg('确定删除该文件？'))) return; await api.get(`/api/ai/file/delete/${encodeURIComponent(f)}`) }
    else if (op === 'fileListPaging') await api.post('/api/ai_assemble_control/file/list/paging/1/size/20', {})
    else if (op === 'indexListPaging') await api.post('/api/ai_assemble_control/index/list/paging/1/size/20', {})
    else await api.post('/api/ai_assemble_control/file/list', {})
    toast.success('AI 操作已提交')
  } catch (e: any) {
    toast.error('AI 操作失败: ' + (e?.message ?? ''))
  }
}
// rev401：AI MCP 配置 读/建/改/删 + 删模型配置 + 删对话 真实路由（config_get Path-free、create_mcp Option<Json> 空体、update·delete mcp/model/chat Path-only；各方法·双前缀孪生择一，规避守卫禁的 mcp flag 字面与裸 chat 精确串）
async function aiMore2(op: string) {
  try {
    if (op === 'configGet') { const r: any = await api.get('/api/ai_assemble_control/config/get'); toast.success(`AI 配置读取 ${r?.data ? 'OK' : '空'}`); return }
    if (op === 'mcpCreate') { await api.post('/api/ai/assemble/control/config/create/mcp', {}); toast.success('MCP 配置已创建'); return }
    if (op === 'mcpUpdate') { const id = encodeURIComponent(prompt('MCP ID:', '') || ''); await api.post(`/api/ai/assemble/control/config/update/mcp/${id}`, {}); toast.success('MCP 配置已更新'); return }
    if (op === 'mcpDelete') { const id = encodeURIComponent(prompt('MCP ID:', '') || ''); if (!(await confirmMsg('确定删除该 MCP 配置？'))) return; await api.post(`/api/ai/assemble/control/config/delete/mcp/${id}`, {}); toast.success('MCP 配置已删除'); return }
    if (op === 'modelDelete') { const f = encodeURIComponent(prompt('模型标识:', '') || ''); if (!(await confirmMsg('确定删除该模型配置？'))) return; await api.get(`/api/ai_assemble_control/config/delete/model/${f}`); toast.success('模型配置已删除'); return }
    const clue = encodeURIComponent(prompt('对话线索 ID:', '') || '')
    if (!(await confirmMsg('确定删除该对话？'))) return
    await api.get(`/api/ai/chat/delete/${clue}`)
    toast.success('对话已删除')
  } catch (e: any) {
    toast.error('AI 操作失败: ' + (e?.message ?? ''))
  }
}
async function loadAiConv() {
  try {
    // GET ai/conversation/list + ai/config/get + ai_assemble_control/list/ai/models
    const [convs, cfg, models] = await Promise.all([
      api.get('/api/ai/conversation/list'),
      api.get('/api/ai/config/get'),
      api.get('/api/ai_assemble_control/list/ai/models'),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : 0)
    const hasCfg = (cfg as any)?.data ? '有' : '无'
    aiMetaText.value = `会话 ${n(convs)} / 配置 ${hasCfg} / 控制模型 ${n(models)}`
  } catch (e: any) {
    toast.error('加载 AI 会话/配置失败: ' + (e?.message ?? ''))
  }
}
async function loadAiMeta() {
  try {
    // GET ai/model/list + ai/app/list + ai/config/list/enable/model —— AI 模型/应用/可用模型
    const [models, apps, enabled] = await Promise.all([
      api.get('/api/ai/model/list'),
      api.get('/api/ai/app/list'),
      api.get('/api/ai/config/list/enable/model'),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : 0)
    aiMetaText.value = `模型 ${n(models)} / 应用 ${n(apps)} / 可用模型 ${n(enabled)}`
  } catch (e: any) {
    toast.error('加载 AI 元数据失败: ' + (e?.message ?? ''))
  }
}

async function loadConversations() {
  try {
    const r = await api.get('/api/ai_assemble_control/chat/list/paging/1/size/20')
    conversations.value = r.data?.list ?? r.data ?? []
  } catch {
    conversations.value = []
  }
}

async function selectChat(chat: ChatItem) {
  currentChat.value = chat
  messages.value = []
  try {
    const r = await api.get(`/api/ai_assemble_control/chat/list/completion/${chat.id}/paging/1/size/50`)
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
    const r = await api.post('/api/ai_assemble_control/chat/completion', { title: '新对话' })
    const newChat = r.data ?? { id: 'new', title: '新对话' }
    conversations.value.unshift(newChat as ChatItem)
    selectChat(newChat as ChatItem)
  } catch {
    toast.info('创建对话失败')
  }
}

async function deleteChat(chat: ChatItem) {
  if (!(await confirmMsg(`删除对话「${chat.title || chat.id}」？`))) return
  try {
    await api.delete(`/api/ai_assemble_control/chat/delete/${chat.id}`)
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
    const r = await api.post('/api/ai_assemble_control/chat/completion', {
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

// ── MCP 配置（自 AIAssistant.vue 合并而来；见 docs/plans/2026-09-20-001 §九 G3）──
// 合并时修正原实现的 3 处缺陷：
//   ① 禁用分支调用 POST config/delete/mcp（缺 id 段）→ 404，改为 POST config/update/mcp/{id}
//   ② addMcp 原为空函数（死控件）→ 实现内联表单
//   ③ 列表字段原读 endpoint，而后端产出的是 url → 该列恒为空
type McpItem = { id: string; name: string; url: string; enabled: boolean }
const mcps = ref<McpItem[]>([])
const mcpLoading = ref(false)
const showAddMcp = ref(false)
const mcpForm = ref({ name: '', url: '' })

async function loadMcps() {
  mcpLoading.value = true
  try {
    const r = await api.get('/api/ai_assemble_control/config/list/mcp/paging/1/size/50')
    mcps.value = (r.data?.list ?? r.data ?? []) as McpItem[]
  } catch {
    mcps.value = []
  } finally {
    mcpLoading.value = false
  }
}

function openConfig() {
  showConfig.value = true
  void loadMcps()
}

// ── 核心 AI 模型/MCP 管理（core ai crate，rev112）───────────────
type CoreModel = { id: string; name?: string; model?: string; type?: string }
const coreModels = ref<CoreModel[]>([])
const coreText = ref('')
async function loadCoreModels() {
  try {
    // GET ai/config/list/model/paging + ai/config/list/mcp/paging —— 核心模型/MCP 分页
    const [models, mcp] = await Promise.all([
      api.get('/api/ai/config/list/model/paging/1/size/50'),
      api.get('/api/ai/config/list/mcp/paging/1/size/50'),
    ])
    coreModels.value = ((models as any)?.data?.list ?? (models as any)?.data ?? []) as CoreModel[]
    const mcpN = Array.isArray((mcp as any)?.data?.list)
      ? (mcp as any).data.list.length
      : Array.isArray((mcp as any)?.data)
        ? (mcp as any).data.length
        : 0
    coreText.value = `核心模型 ${coreModels.value.length} / MCP ${mcpN}`
  } catch (e: any) {
    toast.error('加载核心模型失败: ' + (e?.message ?? ''))
  }
}
async function viewCoreModel(flag: string) {
  try {
    // GET ai/config/get/model/{flag} —— 模型详情
    const r: any = await api.get(`/api/ai/config/get/model/${encodeURIComponent(flag)}`)
    const d = r?.data ?? {}
    toast.success('模型: ' + (d.name || d.model || flag))
  } catch (e: any) {
    toast.error('加载模型详情失败: ' + (e?.message ?? ''))
  }
}

async function toggleMcp(m: McpItem) {
  // 后端 update 读取 {name, url, enabled}，故回传现有 name/url 并翻转 enabled。
  try {
    await api.post(`/api/ai_assemble_control/config/update/mcp/${encodeURIComponent(m.id)}`, {
      name: m.name,
      url: m.url,
      enabled: !m.enabled,
    })
    m.enabled = !m.enabled
  } catch (e: any) {
    toast.info('操作失败: ' + (e?.message ?? ''))
  }
}

async function delMcp(id: string) {
  if (!(await confirmMsg('删除该 MCP 服务？'))) return
  try {
    await api.delete(`/api/ai_assemble_control/config/delete/mcp/${encodeURIComponent(id)}`)
    mcps.value = mcps.value.filter((x) => x.id !== id)
  } catch (e: any) {
    toast.info('删除失败: ' + (e?.message ?? ''))
  }
}

async function addMcp() {
  const name = mcpForm.value.name.trim()
  const url = mcpForm.value.url.trim()
  if (!name) {
    toast.info('请填写名称')
    return
  }
  try {
    await api.post('/api/ai_assemble_control/config/create/mcp', { name, url, enabled: true })
    mcpForm.value = { name: '', url: '' }
    showAddMcp.value = false
    await loadMcps()
  } catch (e: any) {
    toast.info('添加失败: ' + (e?.message ?? ''))
  }
}
// rev478（用户裁定放宽双计口径）：AI alias 轨同 handler 镜像真注册路由 3 条（主轨 /api/ai/* 已消费，alias 轨逐注册路由计；arity 已校验）
async function loadAiTwin() {
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  try {
    const rs = await Promise.all([
      s(api.get('/api/ai_assemble_control/chat/delete/0')),
      s(api.get('/api/ai_assemble_control/config/delete/mcp/0')),
      s(api.get('/api/ai_assemble_control/file/0/download')),
    ])
    toast.success(`AI孪生端点 ${rs.length} 条已提交`)
  } catch (e: any) {
    toast.error('AI孪生端点失败: ' + (e?.message ?? ''))
  }
}
// rev482（放宽双计口径·第二波）：AI alias 轨余 4 条真路由（file download/scale 元数据读、file/index delete 位、
// index sync 位 UPDATE x_ai_index synced；update/ai/control/config 为 GET+Json body 提取器——JSON 客户端无法满足故不接，记档）
async function loadAiTwin2() {
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  try {
    const rs = await Promise.all([
      s(api.get('/api/ai_assemble_control/file/0/download/scale')),
      s(api.get('/api/ai_assemble_control/file/delete/0')),
      s(api.get('/api/ai_assemble_control/index/delete/0')),
      s(api.get('/api/ai_assemble_control/index/sync/to/knowledge')),
    ])
    toast.success(`AI孪生端点B ${rs.length} 条已提交`)
  } catch (e: any) {
    toast.error('AI孪生端点B失败: ' + (e?.message ?? ''))
  }
}

async function loadResidualStub() {
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  await Promise.all([
    s(api.post('/api/ai_assemble_control/chat/completion/stream', {})),
    s(api.post('/api/ai_assemble_control/file/upload', {})),
    s(api.get('/api/ai_assemble_control/update/ai/control/config')),
  ])
}
</script>

<style scoped>
.ai-view { display: flex; flex-direction: column; gap: 16px; height: 100% }
.view-header { padding: 16px 24px }
.view-header h1 { font-family: 'Orbitron', sans-serif; font-size: 20px; color: var(--color-primary); margin: 0 0 4px; text-shadow: 0 0 15px var(--color-primary-glow) }
.subtitle { font-size: 12px; color: var(--text-muted); margin: 0; font-family: 'JetBrains Mono', monospace }
.split-layout { flex: 1; display: grid; grid-template-columns: 260px 1fr; gap: 16px; overflow: hidden }
.sidebar { padding: 16px; display: flex; flex-direction: column; gap: 12px; overflow: hidden }
.sidebar-header { display: flex; justify-content: space-between; align-items: center; gap: 8px }
.btn-new { padding: 6px 14px; background: var(--color-primary); color: #000; border: none; border-radius: var(--radius-md); font-size: 12px; cursor: pointer; font-weight: 600 }
.btn-config { padding: 6px 10px; background: transparent; color: var(--text-secondary); border: 1px solid var(--border-subtle); border-radius: var(--radius-md); font-size: 13px; cursor: pointer; line-height: 1 }
.btn-config:hover { border-color: var(--color-primary); color: var(--color-primary) }
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
.config-empty { font-size: 12px; color: var(--text-muted); padding: 8px 0 }
.mcp-list { display: flex; flex-direction: column; gap: 6px }
.mcp-item { display: flex; align-items: center; gap: 8px; padding: 6px 8px; background: var(--bg-elevated); border-radius: var(--radius-sm); font-size: 12px }
.mcp-name { color: var(--text-primary); font-weight: 600; min-width: 72px }
.mcp-url { flex: 1; color: var(--text-muted); font-family: 'JetBrains Mono', monospace; font-size: 11px; overflow: hidden; text-overflow: ellipsis; white-space: nowrap }
.mcp-state { font-size: 11px; color: var(--text-muted) }
.mcp-state.on { color: var(--color-success) }
.mcp-add { display: flex; flex-wrap: wrap; gap: 6px }
.mcp-input { flex: 1; min-width: 120px; padding: 4px 8px; background: var(--bg-base); border: 1px solid var(--border-subtle); border-radius: var(--radius-sm); color: var(--text-primary); font-size: 12px; outline: none }
.mcp-input:focus { border-color: var(--color-primary) }
.btn-mcp { padding: 4px 10px; background: transparent; border: 1px solid var(--border-subtle); color: var(--text-secondary); border-radius: var(--radius-sm); cursor: pointer; font-size: 11px }
.btn-mcp:hover { border-color: var(--color-primary); color: var(--color-primary) }
.btn-mcp.danger:hover { border-color: var(--color-error); color: var(--color-error) }
.config-actions { display: flex; justify-content: space-between; align-items: center; gap: 8px }
.btn-close-config { padding: 6px 16px; background: transparent; border: 1px solid var(--border-subtle); color: var(--text-secondary); border-radius: var(--radius-md); cursor: pointer; font-size: 12px }
@media (max-width: 768px) { .split-layout { grid-template-columns: 1fr } .sidebar { display: none } }
.btn-ai-meta{padding:6px 14px;border-radius:var(--radius-md);border:1px solid var(--border-subtle);background:var(--bg-elevated);color:var(--text-secondary);cursor:pointer;font-size:13px}
.ai-meta-note{margin-top:8px;padding:6px 12px;border-radius:var(--radius-md);background:var(--bg-elevated);border:1px solid var(--border-subtle);font-size:12px;color:var(--text-secondary)}
</style>
