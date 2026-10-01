<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<script setup lang="ts">
import { onShow } from '@dcloudio/uni-app'
import { ref } from 'vue'
import { authApi } from '@/services'
import { useSession } from '@/store/session'
import { ensureAuthenticated, LOGIN_PAGE } from '@/utils/auth-guard'

const session = useSession()
const groups = ref<string[]>([])

function refreshGroups() {
  const g = session.user?.groups
  if (Array.isArray(g)) {
    groups.value = g.map((x) => (typeof x === 'string' ? x : (x?.name ?? x?.id ?? ''))).filter(Boolean)
  }
}

onShow(async () => {
  if (!(await ensureAuthenticated())) return
  refreshGroups()
})

async function logout() {
  await session.logout()
  uni.reLaunch({ url: LOGIN_PAGE })
}

function goCollect() {
  uni.navigateTo({ url: '/pages/collect/collect' })
}

// ── 修改密码（PUT /api/person/password，对齐桌面 Personal.vue） ──
const pwdForm = ref({ oldPassword: '', newPassword: '', confirmPassword: '' })
const pwdSaving = ref(false)

async function changePassword() {
  if (!pwdForm.value.oldPassword || !pwdForm.value.newPassword) {
    uni.showToast({ title: '请填写完整密码', icon: 'none' })
    return
  }
  if (pwdForm.value.newPassword !== pwdForm.value.confirmPassword) {
    uni.showToast({ title: '两次密码不一致', icon: 'none' })
    return
  }
  pwdSaving.value = true
  try {
    await authApi.changePassword({
      oldPassword: pwdForm.value.oldPassword,
      newPassword: pwdForm.value.newPassword,
    })
    uni.showToast({ title: '密码修改成功', icon: 'success' })
    pwdForm.value = { oldPassword: '', newPassword: '', confirmPassword: '' }
  } catch (e: unknown) {
    uni.showToast({ title: e instanceof Error ? e.message : '修改失败', icon: 'none' })
  } finally {
    pwdSaving.value = false
  }
}
</script>

<template>
  <view class="page">
    <view class="card">
      <view class="avatar">{{ (session.user?.name || 'U').slice(0, 1).toUpperCase() }}</view>
      <view class="info">
        <view class="name">{{ session.user?.name || session.user?.unique || '未登录' }}</view>
        <view class="sub">{{ session.user?.unique || '' }}</view>
      </view>
    </view>

    <view class="rows">
      <view class="row">
        <text class="k">邮箱</text>
        <text class="v">{{ session.user?.email || '—' }}</text>
      </view>
      <view class="row">
        <text class="k">手机号</text>
        <text class="v">{{ session.user?.mobile || '—' }}</text>
      </view>
      <view class="row">
        <text class="k">所属部门</text>
        <text class="v">{{ groups.length ? groups.join('、') : '—' }}</text>
      </view>
    </view>

    <view class="card entry-card" @tap="goCollect">
      <text class="entry-k">我的收藏</text>
      <text class="entry-arrow">›</text>
    </view>

    <view class="card pwd-card">
      <view class="pwd-title">修改密码</view>
      <input
        v-model="pwdForm.oldPassword"
        class="pwd-input"
        password
        type="text"
        placeholder="当前密码"
      />
      <input
        v-model="pwdForm.newPassword"
        class="pwd-input"
        password
        type="text"
        placeholder="新密码（6-64 位，含字母和数字）"
      />
      <input
        v-model="pwdForm.confirmPassword"
        class="pwd-input"
        password
        type="text"
        placeholder="确认新密码"
      />
      <button class="pwd-btn" :disabled="pwdSaving" @tap="changePassword">
        {{ pwdSaving ? '提交中…' : '保存新密码' }}
      </button>
    </view>

    <button class="logout" @tap="logout">退出登录</button>
  </view>
</template>

<style scoped>
.page {
  min-height: 100vh;
  background: #f5f7fa;
  padding: 32rpx;
  box-sizing: border-box;
}
.card {
  display: flex;
  align-items: center;
  gap: 24rpx;
  background: #fff;
  border-radius: 20rpx;
  padding: 40rpx;
  margin-bottom: 24rpx;
}
.avatar {
  width: 96rpx;
  height: 96rpx;
  border-radius: 50%;
  background: #2d8cf0;
  color: #fff;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 44rpx;
  font-weight: 700;
}
.info {
  flex: 1;
}
.name {
  font-size: 36rpx;
  font-weight: 700;
  color: #263238;
}
.sub {
  margin-top: 6rpx;
  font-size: 24rpx;
  color: #90979f;
}
.rows {
  background: #fff;
  border-radius: 20rpx;
  margin-bottom: 40rpx;
}
.row {
  display: flex;
  padding: 28rpx 32rpx;
  border-bottom: 1px solid #f0f2f5;
}
.row:last-child {
  border-bottom: none;
}
.k {
  width: 160rpx;
  color: #90979f;
  font-size: 28rpx;
}
.v {
  flex: 1;
  color: #263238;
  font-size: 28rpx;
}
.entry-card {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 24rpx 28rpx;
}
.entry-k {
  font-size: 28rpx;
  color: #263238;
  font-weight: 600;
}
.entry-arrow {
  color: #90979f;
  font-size: 32rpx;
}
.logout {
  background: #fff;
  color: #f56c6c;
  border: 1px solid #f56c6c;
  border-radius: 12rpx;
  font-size: 30rpx;
}
.pwd-card {
  display: block;
  padding: 32rpx 40rpx;
}
.pwd-title {
  font-size: 30rpx;
  font-weight: 600;
  color: #263238;
  margin-bottom: 20rpx;
}
.pwd-input {
  background: #f5f7fa;
  border-radius: 10rpx;
  padding: 18rpx 24rpx;
  font-size: 28rpx;
  margin-bottom: 18rpx;
  color: #263238;
}
.pwd-btn {
  background: #2d8cf0;
  color: #fff;
  border-radius: 10rpx;
  font-size: 28rpx;
  margin-top: 8rpx;
}
.pwd-btn[disabled] {
  opacity: 0.6;
}
</style>
