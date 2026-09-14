<script setup lang="ts">
import { onShow } from '@dcloudio/uni-app'
import { ref } from 'vue'
import { useSession } from '@/store/session'
import { ensureAuthenticated, navigateHome } from '@/utils/auth-guard'

const session = useSession()
const credential = ref('')
const password = ref('')
const submitting = ref(false)
const error = ref('')

async function onShowGuard() {
  // 已登录则直接进工作台，避免重复显示登录页。
  if (await ensureAuthenticated()) navigateHome()
}

async function submit() {
  if (!credential.value || !password.value) {
    error.value = '请输入账号和密码'
    return
  }
  submitting.value = true
  error.value = ''
  try {
    await session.login(credential.value, password.value)
    navigateHome()
  } catch (e) {
    error.value = e instanceof Error ? e.message : '登录失败'
  } finally {
    submitting.value = false
  }
}

onShow(onShowGuard)
</script>

<template>
  <view class="login">
    <view class="brand">OA4Rust Mobile</view>
    <view class="form">
      <view class="field">
        <text class="label">账号</text>
        <input v-model="credential" type="text" placeholder="工号 / 用户名" />
      </view>
      <view class="field">
        <text class="label">密码</text>
        <input v-model="password" type="password" placeholder="密码" />
      </view>
      <text v-if="error" class="error">{{ error }}</text>
      <button class="submit" :disabled="submitting" @tap="submit">
        {{ submitting ? '登录中…' : '登 录' }}
      </button>
    </view>
  </view>
</template>

<style scoped>
.login {
  min-height: 100vh;
  padding: 80rpx 48rpx;
  box-sizing: border-box;
}
.brand {
  font-size: 44rpx;
  font-weight: 700;
  color: #2d8cf0;
  margin-bottom: 64rpx;
}
.form {
  display: flex;
  flex-direction: column;
  gap: 32rpx;
}
.field {
  display: flex;
  flex-direction: column;
  gap: 12rpx;
}
.label {
  font-size: 26rpx;
  color: #666;
}
.field input {
  border: 1px solid #e3e6eb;
  border-radius: 12rpx;
  padding: 20rpx 24rpx;
  font-size: 30rpx;
  background: #fff;
}
.error {
  color: #f56c6c;
  font-size: 26rpx;
}
.submit {
  margin-top: 16rpx;
  background: #2d8cf0;
  color: #fff;
  border-radius: 12rpx;
  font-size: 32rpx;
}
.submit[disabled] {
  opacity: 0.6;
}
</style>
