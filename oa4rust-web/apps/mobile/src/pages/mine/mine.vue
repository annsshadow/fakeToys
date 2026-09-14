<script setup lang="ts">
import { onShow } from '@dcloudio/uni-app'
import { ref } from 'vue'
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
.logout {
  background: #fff;
  color: #f56c6c;
  border: 1px solid #f56c6c;
  border-radius: 12rpx;
  font-size: 30rpx;
}
</style>
