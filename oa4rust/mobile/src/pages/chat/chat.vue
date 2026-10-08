<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<script setup lang="ts">
import { onLoad } from '@dcloudio/uni-app'
import { computed, ref } from 'vue'
import type { MessageRow } from '@/services'
import { messageApi, messageConversationId } from '@/services'
import { useSession } from '@/store/session'
import { ensureAuthenticated } from '@/utils/auth-guard'

const session = useSession()
const conversationId = ref('')
const title = ref('会话')
const msgs = ref<MessageRow[]>([])
const loading = ref(false)
const text = ref('')
const sending = ref(false)
const myUnique = computed(() => session.user?.unique ?? '')

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
  try {
    const resp = await messageApi.msgHistory(conversationId.value, 1, 200)
    // 后端已按会话过滤；此处保留一次防御性过滤（逆序返回，还原时间正序展示）。
    msgs.value = (resp.data ?? [])
      .filter((m) => messageConversationId(m as Record<string, unknown>) === conversationId.value)
      .reverse()
  } catch {
    msgs.value = []
  } finally {
    loading.value = false
    // 进入会话即标记已读（失败不阻塞展示）。
    try {
      await messageApi.markRead(conversationId.value)
    } catch {
      /* noop */
    }
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
      <view v-else-if="msgs.length === 0" class="tip">暂无消息</view>
      <view v-for="m in msgs" :key="m.id" class="row" :class="{ me: m.sender === myUnique }">
        <view class="bubble">{{ m.content }}</view>
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
