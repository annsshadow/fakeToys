<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<script setup lang="ts">
import { onShow } from '@dcloudio/uni-app'
import { ref } from 'vue'
import { messageApi, orgApi } from '@/services'
import { ensureAuthenticated } from '@/utils/auth-guard'

const keyword = ref('')
const rows = ref<Record<string, unknown>[]>([])
const loading = ref(false)
const loaded = ref(false)
const busyId = ref('')

function nameOf(row: Record<string, unknown>): string {
  for (const k of ['name', 'displayName']) {
    const v = row[k]
    if (typeof v === 'string' && v) return v
  }
  return '未知成员'
}
function subOf(row: Record<string, unknown>): string {
  const parts: string[] = []
  for (const k of ['mobile', 'email', 'job']) {
    const v = row[k]
    if (typeof v === 'string' && v) parts.push(v)
  }
  return parts.join(' · ')
}

async function search() {
  loading.value = true
  try {
    const resp = await orgApi.personSearch(keyword.value.trim() || undefined)
    rows.value = resp.data ?? []
  } catch {
    rows.value = []
  } finally {
    loading.value = false
    loaded.value = true
  }
}

onShow(async () => {
  if (!(await ensureAuthenticated())) return
  if (!loaded.value) search()
})

function doSearch() {
  void search()
}

// ── 阶段 G / F1：通讯录增强（单位 / 群组维度 + 人员身份/角色）──
type Mode = 'person' | 'unit' | 'group'
const mode = ref<Mode>('person')
const units = ref<Record<string, unknown>[]>([])
const groups = ref<Record<string, unknown>[]>([])
const orgLoading = ref(false)
const detail = ref<Record<string, unknown> | null>(null)
const identities = ref<Record<string, unknown>[]>([])
const roles = ref<Record<string, unknown>[]>([])
const detailLoading = ref(false)

async function loadOrg(): Promise<void> {
  orgLoading.value = true
  try {
    if (mode.value === 'unit') {
      const r = await orgApi.unitList()
      units.value = (r.data ?? []) as Record<string, unknown>[]
    } else if (mode.value === 'group') {
      const r = await orgApi.groupList()
      groups.value = (r.data ?? []) as Record<string, unknown>[]
    }
  } catch {
    units.value = []
    groups.value = []
  } finally {
    orgLoading.value = false
  }
}

function switchMode(next: Mode): void {
  if (mode.value === next) return
  mode.value = next
  if (next !== 'person') void loadOrg()
}

/** 打开人员详情：并行取身份与角色（任一失败不影响另一）。 */
async function openPerson(row: Record<string, unknown>): Promise<void> {
  const flag = typeof row.flag === 'string' ? row.flag : ''
  detail.value = row
  identities.value = []
  roles.value = []
  if (!flag) return
  detailLoading.value = true
  try {
    const [ids, rls] = await Promise.allSettled([orgApi.personIdentities(flag), orgApi.personRoles(flag)])
    if (ids.status === 'fulfilled') identities.value = (ids.value.data ?? []) as Record<string, unknown>[]
    if (rls.status === 'fulfilled') roles.value = (rls.value.data ?? []) as Record<string, unknown>[]
  } finally {
    detailLoading.value = false
  }
}

function orgNameOf(row: Record<string, unknown>): string {
  return String(row.name ?? row.id ?? '未命名')
}

/** 发起单聊：创建一条 type=single 会话（名字取对方姓名），随即进入聊天页。 */
async function startChat(row: Record<string, unknown>): Promise<void> {
  const flag = typeof row.flag === 'string' ? row.flag : ''
  if (!flag || busyId.value) return
  const name = nameOf(row)
  busyId.value = flag
  uni.showLoading({ title: '发起会话…' })
  try {
    const resp = await messageApi.startConversation(name)
    const id = String(resp.data?.id ?? '')
    if (!id) throw new Error('后端未返回会话 ID')
    uni.hideLoading()
    uni.navigateTo({ url: `/pages/chat/chat?id=${encodeURIComponent(id)}&name=${encodeURIComponent(name)}` })
  } catch (e) {
    uni.hideLoading()
    uni.showToast({ title: e instanceof Error ? e.message : '发起会话失败', icon: 'none' })
  } finally {
    busyId.value = ''
  }
}
</script>

<template>
  <view class="page">
    <view class="search">
      <input
        v-model="keyword"
        class="input"
        type="text"
        placeholder="搜索姓名 / 手机号 / 邮箱"
        confirm-type="search"
        @confirm="doSearch"
      />
      <button class="go" size="mini" type="primary" :disabled="loading" @tap="doSearch">
        搜索
      </button>
    </view>

    <view class="modes">
      <view class="mode" :class="{ on: mode === 'person' }" @tap="switchMode('person')">人员</view>
      <view class="mode" :class="{ on: mode === 'unit' }" @tap="switchMode('unit')">单位</view>
      <view class="mode" :class="{ on: mode === 'group' }" @tap="switchMode('group')">群组</view>
    </view>

    <template v-if="mode === 'person'">
      <view v-if="loading && !loaded" class="tip">加载中…</view>
      <view v-else-if="rows.length === 0" class="tip">{{ keyword ? '未找到相关人员' : '暂无联系人' }}</view>
      <view v-else class="list">
        <view v-for="(row, i) in rows" :key="i" class="item">
          <text class="avatar">{{ nameOf(row).slice(0, 1).toUpperCase() }}</text>
          <view class="body" @tap="openPerson(row)">
            <view class="title">{{ nameOf(row) }}</view>
            <view class="meta">{{ subOf(row) || ' ' }}</view>
          </view>
          <button class="chat-btn" size="mini" type="primary" :disabled="busyId !== ''" @tap="startChat(row)">
            聊天
          </button>
        </view>
      </view>
    </template>

    <template v-else>
      <view v-if="orgLoading" class="tip">加载中…</view>
      <view v-else-if="(mode === 'unit' ? units : groups).length === 0" class="tip">
        {{ mode === 'unit' ? '暂无单位' : '暂无群组' }}
      </view>
      <view v-else class="list">
        <view v-for="(o, i) in mode === 'unit' ? units : groups" :key="i" class="item">
          <text class="avatar">{{ mode === 'unit' ? '🏢' : '👥' }}</text>
          <view class="body">
            <view class="title">{{ orgNameOf(o) }}</view>
            <view class="meta">{{ o.id ? String(o.id) : ' ' }}</view>
          </view>
        </view>
      </view>
    </template>

    <view v-if="detail" class="detail">
      <view class="detail-head">
        <text class="detail-title">{{ nameOf(detail) }}</text>
        <text class="detail-close" @tap="detail = null">关闭</text>
      </view>
      <view class="detail-sub">{{ subOf(detail) || '—' }}</view>
      <view v-if="detailLoading" class="tip">加载身份与角色…</view>
      <template v-else>
        <view class="detail-sec">
          <text class="detail-label">身份（{{ identities.length }}）</text>
          <view v-if="identities.length === 0" class="detail-empty">—</view>
          <view v-for="(x, i) in identities" :key="i" class="detail-item">{{ orgNameOf(x) }}</view>
        </view>
        <view class="detail-sec">
          <text class="detail-label">角色（{{ roles.length }}）</text>
          <view v-if="roles.length === 0" class="detail-empty">—</view>
          <view v-for="(x, i) in roles" :key="i" class="detail-item">{{ orgNameOf(x) }}</view>
        </view>
      </template>
    </view>
  </view>
</template>

<style scoped>
.page {
  min-height: 100vh;
  background: #f5f7fa;
}
.search {
  display: flex;
  gap: 16rpx;
  padding: 24rpx;
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
.tip {
  padding: 120rpx 0;
  text-align: center;
  color: #90979f;
  font-size: 28rpx;
}
.list {
  background: #fff;
}
.item {
  display: flex;
  align-items: center;
  gap: 20rpx;
  padding: 28rpx 32rpx;
  border-bottom: 1px solid #f0f2f5;
}
.avatar {
  width: 72rpx;
  height: 72rpx;
  border-radius: 50%;
  background: #2d8cf0;
  color: #fff;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 32rpx;
  font-weight: 700;
}
.body {
  flex: 1;
}
.title {
  font-size: 30rpx;
  color: #263238;
  font-weight: 600;
}
.meta {
  margin-top: 6rpx;
  font-size: 24rpx;
  color: #90979f;
}
.chat-btn {
  flex-shrink: 0;
}
/* ── 阶段 G / F1：单位·群组·人员详情 ── */
.modes {
  display: flex;
  gap: 12rpx;
  margin-bottom: 20rpx;
}
.mode {
  flex: 1;
  text-align: center;
  padding: 14rpx 0;
  font-size: 28rpx;
  color: #5b6572;
  background: #fff;
  border-radius: 12rpx;
}
.mode.on {
  background: #2d8cf0;
  color: #fff;
  font-weight: 600;
}
.detail {
  background: #fff;
  border-radius: 16rpx;
  padding: 24rpx;
  margin-top: 20rpx;
}
.detail-head {
  display: flex;
  justify-content: space-between;
  align-items: center;
}
.detail-title {
  font-size: 30rpx;
  font-weight: 700;
  color: #263238;
}
.detail-close {
  font-size: 26rpx;
  color: #2d8cf0;
}
.detail-sub {
  margin-top: 8rpx;
  font-size: 24rpx;
  color: #90979f;
}
.detail-sec {
  margin-top: 20rpx;
}
.detail-label {
  font-size: 26rpx;
  font-weight: 600;
  color: #263238;
}
.detail-item {
  font-size: 26rpx;
  color: #5b6572;
  padding: 8rpx 0;
}
.detail-empty {
  font-size: 26rpx;
  color: #c0c4cc;
  padding: 8rpx 0;
}
</style>
