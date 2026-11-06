<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<script setup lang="ts">
import { onHide, onLoad, onShow, onUnload } from '@dcloudio/uni-app'
import { computed, ref } from 'vue'
import type { MessageRow } from '@/services'
import { messageApi, messageConversationId } from '@/services'
import { useSession } from '@/store/session'
import { ensureAuthenticated } from '@/utils/auth-guard'
import { describeChatError, loadChatHistoryPage, mergeOlderMessages } from './chatHistory'
import { CHAT_POLL_INTERVAL_MS, type ChatPollHandle, mergeNewerMessages, startChatPolling } from './chatPolling'

const session = useSession()
const conversationId = ref('')
const title = ref('会话')
const msgs = ref<MessageRow[]>([])
const loading = ref(false)
const text = ref('')
const sending = ref(false)
const myUnique = computed(() => session.user?.unique ?? '')
// R108：历史分页状态（旧实现固定第 1 页 200 条，超出的更早消息静默丢失）
const historyPage = ref(1)
const historyHasMore = ref(false)
const historyError = ref('')
const loadingOlder = ref(false)

function keepConversation(rows: MessageRow[]): MessageRow[] {
  return rows.filter((m) => messageConversationId(m as Record<string, unknown>) === conversationId.value)
}

// R140：进入会话后自动刷新新消息（旧实现只在 onLoad 拉一次，之后对方发来的消息永远不出现）。
const CHAT_POLL_PAGE_SIZE = 50
let pageVisible = false
let historyReady = false
let poller: ChatPollHandle | null = null

async function markConversationRead() {
  try {
    await messageApi.markRead(conversationId.value)
  } catch (err) {
    // R139：自动已读失败不打扰用户，但必须留可诊断痕迹，避免线上无从排查。
    console.warn('[chat] markRead failed', err)
  }
}

function stopPolling() {
  if (!poller) return
  poller.stop()
  poller = null
}

function startPolling() {
  if (!pageVisible || !historyReady || !conversationId.value || poller) return
  poller = startChatPolling<MessageRow>({
    intervalMs: CHAT_POLL_INTERVAL_MS,
    loadLatest: async () => {
      const page = await loadChatHistoryPage<MessageRow>(
        (convId, pageNo, size) => messageApi.msgHistory(convId, pageNo, size),
        conversationId.value,
        1,
        CHAT_POLL_PAGE_SIZE,
      )
      return keepConversation(page.messages)
    },
    onLatest: (rows) => {
      if (rows.length === 0) return
      const before = msgs.value.length
      msgs.value = mergeNewerMessages(msgs.value, rows)
      if (msgs.value.length !== before) void markConversationRead()
    },
    // 轮询失败不弹提示（用户无操作），但要留可诊断痕迹。
    onError: (err) => console.warn('[chat] poll failed', err),
  })
}

onShow(() => {
  pageVisible = true
  startPolling()
})
onHide(() => {
  pageVisible = false
  stopPolling()
})
onUnload(() => {
  pageVisible = false
  stopPolling()
})

onLoad(async (options) => {
  if (!(await ensureAuthenticated())) {
    uni.navigateBack()
    return
  }
  conversationId.value = options?.id ? decodeURIComponent(options.id) : ''
  title.value = options?.name ? decodeURIComponent(options.name) : '会话'
  if (!conversationId.value) return
  void loadHistory()
})

async function loadHistory() {
  loading.value = true
  historyError.value = ''
  try {
    const result = await loadChatHistoryPage<MessageRow>(
      (convId, page, size) => messageApi.msgHistory(convId, page, size),
      conversationId.value,
      1,
    )
    // 后端已按会话过滤；此处保留一次防御性过滤，并按时间正序展示。
    msgs.value = mergeOlderMessages<MessageRow>([], keepConversation(result.messages))
    historyPage.value = result.page
    historyHasMore.value = result.hasMore
    historyReady = true
    startPolling()
  } catch (e) {
    msgs.value = []
    historyError.value = describeChatError('消息加载失败', e)
  } finally {
    loading.value = false
    // 进入会话即标记已读（失败不阻塞展示）。
    await markConversationRead()
  }
}

async function loadOlder() {
  if (loading.value || loadingOlder.value || !historyHasMore.value) return
  loadingOlder.value = true
  historyError.value = ''
  try {
    const result = await loadChatHistoryPage<MessageRow>(
      (convId, page, size) => messageApi.msgHistory(convId, page, size),
      conversationId.value,
      historyPage.value + 1,
    )
    msgs.value = mergeOlderMessages<MessageRow>(msgs.value, keepConversation(result.messages))
    historyPage.value = result.page
    historyHasMore.value = result.hasMore
  } catch (e) {
    historyError.value = describeChatError('历史消息加载失败', e)
  } finally {
    loadingOlder.value = false
  }
}

async function send() {
  const content = text.value.trim()
  if (!content || !conversationId.value || sending.value) return
  sending.value = true
  try {
    await messageApi.send(conversationId.value, content, myUnique.value || 'me')
    msgs.value = [
      ...msgs.value,
      {
        id: `local-${Date.now()}`,
        conversationId: conversationId.value,
        content,
        sender: myUnique.value || 'me',
        type: 'text',
      },
    ]
    text.value = ''
  } catch (e) {
    uni.showToast({ title: e instanceof Error ? e.message : '发送失败', icon: 'none' })
  } finally {
    sending.value = false
  }
}
</script>

<template>
  <view class="chat">
    <view class="head">
      {{ title }}
    </view>

    <scroll-view class="body" scroll-y>
      <view v-if="loading" class="tip">加载中…</view>
      <view v-else-if="msgs.length === 0 && historyError" class="tip tip-error">
        <view class="error-text">{{ historyError }}</view>
        <view class="retry-btn" @tap="loadHistory">重试</view>
      </view>
      <view v-else-if="msgs.length === 0" class="tip">暂无消息</view>
      <view v-else>
        <view v-if="historyError" class="tip tip-error">
          <view class="error-text">{{ historyError }}</view>
          <view class="retry-btn" @tap="loadOlder">重试</view>
        </view>
        <view v-else-if="loadingOlder" class="tip">加载更早消息…</view>
        <view v-else-if="historyHasMore" class="load-older" @tap="loadOlder">加载更早消息</view>
        <view v-else class="tip tip-end">没有更早的消息了</view>
        <view v-for="m in msgs" :key="m.id" class="row" :class="{ me: m.sender === myUnique }">
          <view class="bubble">{{ m.content }}</view>
        </view>
      </view>
    </scroll-view>

    <view class="composer">
      <input v-model="text" class="input" type="text" placeholder="发消息…" confirm-type="send" @confirm="send" />
      <button class="send" size="mini" type="primary" :disabled="sending || !text.trim()" @tap="send">
        发送
      </button>
    </view>
  </view>
</template>

<style scoped>
.chat {
  display: flex;
  flex-direction: column;
  height: 100vh;
  background: #f5f7fa;
}
.head {
  padding: 24rpx 32rpx;
  background: #fff;
  font-size: 32rpx;
  font-weight: 700;
  color: #263238;
  text-align: center;
}
.body {
  flex: 1;
  padding: 24rpx;
  box-sizing: border-box;
}
.tip {
  padding: 80rpx 0;
  text-align: center;
  color: #90979f;
  font-size: 28rpx;
}
.tip-error {
  color: #c0392b;
}
.tip-end {
  color: #b0b7bf;
}
.error-text {
  margin-bottom: 12rpx;
  line-height: 1.5;
}
.retry-btn {
  display: inline-block;
  padding: 8rpx 28rpx;
  border-radius: 24rpx;
  border: 1rpx solid #c0392b;
  color: #c0392b;
  font-size: 26rpx;
}
.load-older {
  padding: 24rpx 0;
  text-align: center;
  color: #4a90d9;
  font-size: 26rpx;
}
.row {
  display: flex;
  margin-bottom: 20rpx;
}
.row.me {
  justify-content: flex-end;
}
.bubble {
  max-width: 70%;
  padding: 16rpx 24rpx;
  border-radius: 16rpx;
  background: #fff;
  color: #263238;
  font-size: 28rpx;
  word-break: break-all;
}
.row.me .bubble {
  background: #2d8cf0;
  color: #fff;
}
.composer {
  display: flex;
  align-items: center;
  gap: 16rpx;
  padding: 16rpx 24rpx;
  background: #fff;
}
.input {
  flex: 1;
  border: 1px solid #e3e6eb;
  border-radius: 32rpx;
  padding: 14rpx 24rpx;
  font-size: 28rpx;
  background: #f5f7fa;
}
</style>
