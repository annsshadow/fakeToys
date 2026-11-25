<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<script setup lang="ts">
import { onLoad } from '@dcloudio/uni-app'
import { ref } from 'vue'
import { bbsApi } from '@/services'
import { useSession } from '@/store/session'
import { ensureAuthenticated } from '@/utils/auth-guard'
import { describeLoadError } from '@/utils/loadList'

const session = useSession()
const topicId = ref('')
const topic = ref<Record<string, unknown> | null>(null)
const replies = ref<Record<string, unknown>[]>([])
const loading = ref(false)
const loadError = ref('')
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
  loadError.value = ''
  try {
    const [subjectResp, replyResp] = await Promise.all([
      bbsApi.subjectView(topicId.value),
      bbsApi.replyList(topicId.value),
    ])
    // subject_view_id 对不存在的主题返回 type:error + data:null（HTTP 200）。
    topic.value = (subjectResp.data as Record<string, unknown> | null) ?? null
    replies.value = replyResp.data ?? []
  } catch (e) {
    // 失败保留旧主题/回帖并显式报错，不能和「主题真的没有回帖」混成同一种空态。
    loadError.value = describeLoadError('主题加载失败', e)
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
  <view class="page ai-surface-gradient">
    <view v-if="loading" class="ai-skeleton-list" aria-hidden="true">
      <view v-for="i in 3" :key="i" class="ai-skeleton-line"></view>
      <view class="ai-skeleton-line short"></view>
    </view>
    <view v-else-if="!topic && loadError" class="tip tip-error">
      <view class="error-text">{{ loadError }}</view>
      <view class="retry-btn" @tap="load">重试</view>
    </view>
    <view v-else-if="!topic" class="tip">主题不存在或已被删除</view>
    <template v-else>
      <view class="subject">
        <view class="subject-title">{{ titleOf(topic) }}</view>
        <view class="subject-meta">{{ creatorOf(topic) }} · {{ timeOf(topic) }}</view>
        <view v-if="typeof topic.content === 'string' && topic.content" class="subject-content">{{ topic.content }}</view>
      </view>
      <view class="replies-head">回帖（{{ replies.length }}）</view>
      <view v-if="replies.length > 0 && loadError" class="error-banner">
        <text class="error-text">{{ loadError }}</text>
        <view class="retry-btn" @tap="load">重试</view>
      </view>
      <view v-else-if="replies.length === 0 && loadError" class="tip tip-error">
        <view class="error-text">{{ loadError }}</view>
        <view class="retry-btn" @tap="load">重试</view>
      </view>
      <view v-else-if="replies.length === 0" class="tip">暂无回帖，来抢沙发</view>
      <view v-for="(r, i) in replies" :key="i" class="reply">
        <view class="reply-meta">{{ creatorOf(r) }} · {{ timeOf(r) }}</view>
        <view class="reply-content">{{ r.content }}</view>
      </view>
      <view class="composer">
        <input v-model="text" class="composer-input" placeholder="写下你的回帖…" aria-label="回帖内容" confirm-type="send" @confirm="send" />
        <view class="composer-send ai-brand-fill" :class="{ disabled: sending || !text.trim() }" @tap="send">
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
  color: var(--color-text-muted);
  padding: 40px 0;
  font-size: 14px;
}
.tip-error {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 10px;
}
.error-text {
  color: var(--color-error);
  font-size: 14px;
}
.retry-btn {
  padding: 4px 16px;
  border-radius: 14px;
  border: 1px solid var(--color-error);
  color: var(--color-error);
  font-size: 13px;
  line-height: 24px;
}
.error-banner {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 12px;
  margin-bottom: 8px;
  border-radius: 8px;
  background: var(--color-error-bg);
}
.subject {
  padding: 14px;
  border-radius: 10px;
  background: var(--color-surface);
}
.subject-title {
  font-size: 17px;
  font-weight: 600;
  color: var(--color-text-primary);
}
.subject-meta {
  margin-top: 6px;
  font-size: 12px;
  color: var(--color-text-muted);
}
.subject-content {
  margin-top: 10px;
  font-size: 14px;
  line-height: 1.6;
  color: var(--color-text-primary);
  white-space: pre-wrap;
  word-break: break-word;
}
.replies-head {
  margin: 14px 2px 8px;
  font-size: 13px;
  font-weight: 600;
  color: var(--color-text-secondary);
}
.reply {
  padding: 10px 12px;
  border-radius: 10px;
  background: var(--color-surface);
  margin-bottom: 8px;
}
.reply-meta {
  font-size: 12px;
  color: var(--color-text-muted);
}
.reply-content {
  margin-top: 4px;
  font-size: 14px;
  color: var(--color-text-primary);
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
  background: var(--color-bg-page);
  border-top: 1px solid var(--color-border);
}
.composer-input {
  flex: 1;
  height: 36px;
  padding: 0 12px;
  border-radius: 18px;
  background: var(--color-surface);
  font-size: 14px;
}
.composer-send {
  padding: 0 16px;
  border-radius: 18px;
  background: var(--color-primary);
  color: var(--color-surface);
  font-size: 14px;
  line-height: 36px;
}
.composer-send.disabled {
  opacity: 0.5;
}
</style>
