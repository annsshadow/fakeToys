<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<script setup lang="ts">
import { onLoad } from '@dcloudio/uni-app'
import { ref } from 'vue'
import { bbsApi } from '@/services'
import { useSession } from '@/store/session'
import { ensureAuthenticated } from '@/utils/auth-guard'

const session = useSession()
const topicId = ref('')
const topic = ref<Record<string, unknown> | null>(null)
const replies = ref<Record<string, unknown>[]>([])
const loading = ref(false)
const text = ref('')
const sending = ref(false)

function titleOf(row: Record<string, unknown>): string {
  const v = row.title
  return typeof v === 'string' && v ? v : '（无标题）'
}
function creatorOf(row: Record<string, unknown>): string {
  const v = row.creator
  return typeof v === 'string' && v ? v : '匿名'
}
// 后端 TopicRow/ReplyRow 序列化为 snake_case（create_time），camelCase 仅作旧形状兜底。
function timeOf(row: Record<string, unknown>): string {
  const v = row.create_time ?? row.createTime
  return typeof v === 'string' && v ? v.slice(0, 16) : ''
}

async function load() {
  if (!topicId.value) return
  loading.value = true
  try {
    const [subjectResp, replyResp] = await Promise.all([bbsApi.subjectView(topicId.value), bbsApi.replyList(topicId.value)])
    // subject_view_id 对不存在的主题返回 type:error + data:null（HTTP 200）。
    topic.value = (subjectResp.data as Record<string, unknown> | null) ?? null
    replies.value = replyResp.data ?? []
  } catch {
    topic.value = null
    replies.value = []
  } finally {
    loading.value = false
  }
}

async function send() {
  const content = text.value.trim()
  if (!content || !topicId.value || sending.value) return
  sending.value = true
  try {
    await bbsApi.replyCreate({ topicId: topicId.value, content })
    text.value = ''
    await load()
  } catch {
    uni.showToast({ title: '回帖失败，请重试', icon: 'none' })
  } finally {
    sending.value = false
  }
}

onLoad(async (options) => {
  if (!(await ensureAuthenticated())) {
    uni.navigateBack()
    return
  }
  topicId.value = options?.id ? decodeURIComponent(options.id) : ''
  if (!topicId.value) return
  load()
})
</script>

<template>
  <view class="page">
    <view v-if="loading" class="tip">加载中…</view>
    <view v-else-if="!topic" class="tip">主题不存在或已被删除</view>
    <template v-else>
      <view class="subject">
        <view class="subject-title">{{ titleOf(topic) }}</view>
        <view class="subject-meta">{{ creatorOf(topic) }} · {{ timeOf(topic) }}</view>
        <view v-if="typeof topic.content === 'string' && topic.content" class="subject-content">{{ topic.content }}</view>
      </view>
      <view class="replies-head">回帖（{{ replies.length }}）</view>
      <view v-if="replies.length === 0" class="tip">暂无回帖，来抢沙发</view>
      <view v-for="(r, i) in replies" :key="i" class="reply">
        <view class="reply-meta">{{ creatorOf(r) }} · {{ timeOf(r) }}</view>
        <view class="reply-content">{{ r.content }}</view>
      </view>
      <view class="composer">
        <input v-model="text" class="composer-input" placeholder="写下你的回帖…" confirm-type="send" @confirm="send" />
        <view class="composer-send" :class="{ disabled: sending || !text.trim() }" @tap="send">
          {{ sending ? '发送中…' : '发送' }}
        </view>
      </view>
    </template>
  </view>
</template>

<style scoped>
.page {
  padding: 12px;
  padding-bottom: 80px;
  min-height: 100vh;
}
.tip {
  text-align: center;
  color: #8a94a6;
  padding: 40px 0;
  font-size: 14px;
}
.subject {
  padding: 14px;
  border-radius: 10px;
  background: #ffffff;
}
.subject-title {
  font-size: 17px;
  font-weight: 600;
  color: #1f2733;
}
.subject-meta {
  margin-top: 6px;
  font-size: 12px;
  color: #8a94a6;
}
.subject-content {
  margin-top: 10px;
  font-size: 14px;
  line-height: 1.6;
  color: #3a4356;
  white-space: pre-wrap;
  word-break: break-word;
}
.replies-head {
  margin: 14px 2px 8px;
  font-size: 13px;
  font-weight: 600;
  color: #5a6478;
}
.reply {
  padding: 10px 12px;
  border-radius: 10px;
  background: #ffffff;
  margin-bottom: 8px;
}
.reply-meta {
  font-size: 12px;
  color: #8a94a6;
}
.reply-content {
  margin-top: 4px;
  font-size: 14px;
  color: #3a4356;
  white-space: pre-wrap;
  word-break: break-word;
}
.composer {
  position: fixed;
  left: 0;
  right: 0;
  bottom: 0;
  display: flex;
  gap: 8px;
  padding: 10px 12px calc(10px + constant(safe-area-inset-bottom)) 12px;
  padding-bottom: calc(10px + env(safe-area-inset-bottom));
  background: #f4f6fa;
  border-top: 1px solid #e3e8f0;
}
.composer-input {
  flex: 1;
  height: 36px;
  padding: 0 12px;
  border-radius: 18px;
  background: #ffffff;
  font-size: 14px;
}
.composer-send {
  padding: 0 16px;
  border-radius: 18px;
  background: #2f6bff;
  color: #ffffff;
  font-size: 14px;
  line-height: 36px;
}
.composer-send.disabled {
  opacity: 0.5;
}
</style>
