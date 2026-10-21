<template>
  <view class="page-root">
    <view class="pad" style="padding-top: 120rpx">
      <view class="card">
        <view class="row" style="gap: 20rpx">
          <view class="avatar">炮</view>
          <view class="flex-1">
            <text class="nick">来一炮</text>
            <text class="dim" style="display: block">测试环境 · 账号登录</text>
          </view>
        </view>
        <text class="muted" style="display: block; margin-top: 16rpx">
          本环境要求用账号登录，<b>全体测试者共用同一个账号</b> ——
          这样金币、关卡进度、专精、战报都聚在同一个号上，
          不会散成「每个浏览器一个游客号」。
        </text>
      </view>

      <view class="card">
        <text class="card-title">账号</text>
        <input
          v-model="username"
          class="inp"
          type="text"
          placeholder="用户名"
          placeholder-class="ph"
          :disabled="submitting"
        />
        <input
          v-model="password"
          class="inp"
          type="text"
          password
          placeholder="口令"
          placeholder-class="ph"
          :disabled="submitting"
          @confirm="submit"
        />
        <view
          class="btn btn-primary"
          :class="{ 'btn-disabled': submitting || !username || !password }"
          @click="submit"
        >
          {{ submitting ? '登录中…' : '登录' }}
        </view>
        <text v-if="errMsg" class="muted" style="display: block; margin-top: 12rpx; color: var(--bad)">
          {{ errMsg }}
        </text>
        <text class="dim" style="display: block; margin-top: 12rpx">
          账号与口令与运营后台一致（服务端 <span class="mono">PLAYER_LOGIN_REQUIRED</span> 开启时，
          启动会引导一个与超管同名的测试账号）。
        </text>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
/**
 * 账号登录页。
 *
 * # 为什么需要它
 *
 * 游客通道给**每个浏览器/设备**一个独立账号，凭证存在 localStorage。
 * 对「随手玩两下」是合适的，对**测试环境**不合适：
 *
 *   - 测试数据散落在无数个游客号里，后台看到的是一堆孤立的号；
 *   - 换一个浏览器（或者清一次缓存）就是一个新号，
 *     「我刚打的关卡呢」变成一个无法解释的 bug；
 *   - 多人同时测时互相看不见对方的进度。
 *
 * 所以服务端加了 `POST /api/v1/auth/login`（用户名 + 口令），
 * 测试环境用一个**共享账号**，数据就聚在一处了。
 *
 * # 口令为什么不预填
 *
 * 预填口令等于把口令**打进前端产物** —— 那份产物是公开可下载的，
 * 于是运营超管的口令就公开了（而它同时是玩家测试账号的口令）。
 *
 * 所以这里只预填**用户名**（不是秘密，且省一次输入），口令由使用者输入。
 */
import { ref } from 'vue'
import { onLoad } from '@dcloudio/uni-app'

import { useGameStore } from '@/store/game'
import { loadLastUsername } from '@/api/client'

const store = useGameStore()

const username = ref('')
const password = ref('')
const errMsg = ref('')
const submitting = ref(false)

onLoad(() => {
  // 只回填用户名：口令不存、也不预填（见上方注释）。
  username.value = loadLastUsername() || 'admin'
})

async function submit(): Promise<void> {
  if (submitting.value) return
  if (!username.value || !password.value) {
    errMsg.value = '请填写用户名与口令'
    return
  }
  submitting.value = true
  errMsg.value = ''
  try {
    const ok = await store.loginWithPassword(username.value.trim(), password.value)
    if (!ok) {
      errMsg.value = store.loginError || '登录失败'
      return
    }
    password.value = ''
    // 登录页是闸门：登录成功后重置到首页栈，
    // 免得用户按返回键又回到这张登录页。
    uni.reLaunch({ url: '/pages/index/index' })
  } finally {
    submitting.value = false
  }
}
</script>

<style scoped>
.avatar {
  width: 96rpx;
  height: 96rpx;
  border-radius: 20rpx;
  background: linear-gradient(135deg, #ff6b35, #ff3d71);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 44rpx;
  font-weight: 700;
  color: #fff;
}

.nick {
  font-size: 32rpx;
  font-weight: 700;
  color: var(--text);
}

.inp {
  display: block;
  height: 80rpx;
  padding: 0 20rpx;
  margin-bottom: 16rpx;
  background: var(--panel-2);
  border: 1rpx solid var(--border);
  border-radius: 12rpx;
  font-size: 26rpx;
  color: var(--text);
}

.ph {
  color: var(--dim);
}
</style>
