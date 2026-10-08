<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<script setup lang="ts">
import { onLoad } from '@dcloudio/uni-app'
import { ref } from 'vue'
import { bbsApi } from '@/services'
import { useSession } from '@/store/session'
import { ensureAuthenticated } from '@/utils/auth-guard'

const session = useSession()
const forums = ref<{ id: string; name: string }[]>([])
const forumIndex = ref(-1)
const title = ref('')
const content = ref('')
const submitting = ref(false)

const forumName = () => (forumIndex.value >= 0 ? forums.value[forumIndex.value]?.name : '')
const canSubmit = () =>
  Boolean(forumIndex.value >= 0 && title.value.trim() && content.value.trim() && !submitting.value)

async function loadForums() {
  try {
    const resp = await bbsApi.forumList()
    forums.value = (resp.data ?? [])
      .map((f) => ({ id: typeof f.id === 'string' ? f.id : '', name: typeof f.name === 'string' ? f.name : '' }))
      .filter((f) => f.id)
  } catch {
    forums.value = []
  }
}

function onForumChange(e: { detail: { value: number | string } }) {
  forumIndex.value = Number(e.detail.value)
}

async function submit() {
  const creator = session.user?.unique ?? ''
  if (!canSubmit() || !creator) return
  submitting.value = true
  try {
    const resp = await bbsApi.topicCreate({
      forumId: forums.value[forumIndex.value].id,
      title: title.value.trim(),
      content: content.value.trim(),
      creator,
    })
    const newId = resp.data?.id ?? ''
    uni.showToast({ title: '发布成功', icon: 'success' })
    // 新主题详情可直接打开；拿不到 id 时退回列表刷新即可。
    setTimeout(() => {
      if (newId) {
        uni.redirectTo({ url: `/pages/bbs/topic?id=${encodeURIComponent(newId)}` })
      } else {
        uni.navigateBack()
      }
    }, 600)
  } catch {
    uni.showToast({ title: '发布失败，请重试', icon: 'none' })
  } finally {
    submitting.value = false
  }
}

onLoad(async () => {
  if (!(await ensureAuthenticated())) {
    uni.navigateBack()
    return
  }
  loadForums()
})
</script>

<template>
  <view class="page">
    <view class="form-card">
      <view class="field">
        <text class="label">版块</text>
        <picker class="picker" :range="forums.map((f) => f.name)" @change="onForumChange">
          <view class="picker-value" :class="{ placeholder: forumIndex < 0 }">
            {{ forumName() || '请选择版块' }}
          </view>
        </picker>
      </view>
      <view class="field">
        <text class="label">标题</text>
        <input v-model="title" class="input" placeholder="请输入标题" maxlength="100" />
      </view>
      <view class="field">
        <text class="label">内容</text>
        <textarea v-model="content" class="textarea" placeholder="请输入内容" />
      </view>
    </view>
    <view class="submit" :class="{ disabled: !canSubmit() }" @tap="submit">
      {{ submitting ? '发布中…' : '发布主题' }}
    </view>
  </view>
</template>

<style scoped>
.page {
  padding: 12px;
  min-height: 100vh;
}
.form-card {
  padding: 14px;
  border-radius: 10px;
  background: #ffffff;
}
.field {
  margin-bottom: 14px;
}
.label {
  display: block;
  margin-bottom: 6px;
  font-size: 13px;
  color: #5a6478;
}
.picker-value {
  height: 36px;
  line-height: 36px;
  padding: 0 12px;
  border-radius: 8px;
  background: #f4f6fa;
  font-size: 14px;
  color: #1f2733;
}
.picker-value.placeholder {
  color: #8a94a6;
}
.input {
  height: 36px;
  padding: 0 12px;
  border-radius: 8px;
  background: #f4f6fa;
  font-size: 14px;
}
.textarea {
  width: 100%;
  min-height: 120px;
  padding: 10px 12px;
  border-radius: 8px;
  background: #f4f6fa;
  font-size: 14px;
  box-sizing: border-box;
}
.submit {
  margin-top: 18px;
  height: 44px;
  line-height: 44px;
  text-align: center;
  border-radius: 22px;
  background: #2f6bff;
  color: #ffffff;
  font-size: 15px;
}
.submit.disabled {
  opacity: 0.5;
}
</style>
