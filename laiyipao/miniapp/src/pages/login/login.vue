<template>
  <view class="page-root">
    <view class="pad safe-bottom" :style="{ paddingTop: statusBarPad }">
      <!-- 品牌头部（v2）：登录页是产品的第一眼 -->
      <view class="login-hero">
        <view class="avatar">炮</view>
        <text class="nick">来一炮</text>
        <text class="dim">测试环境 · 账号登录</text>
      </view>

      <view class="card">
        <text class="muted" style="display: block; margin-bottom: 20rpx">
          本环境要求用账号登录，<b>全体测试者共用同一个账号</b> ——
          这样金币、关卡进度、专精、战报都聚在同一个号上，
          不会散成「每个浏览器一个游客号」。
        </text>
        <text class="card-title" style="text-align: left">账号</text>
        <input
          v-model="username"
          class="inp"
          type="text"
          placeholder="用户名"
          placeholder-class="ph"
          :disabled="submitting"
        />
        <view class="row" style="align-items: center; gap: 12rpx">
          <input
            v-model="password"
            class="inp flex-1"
            type="text"
            :password="!showPassword"
            placeholder="口令"
            placeholder-class="ph"
            :disabled="submitting"
            @confirm="submit"
          />
          <!-- 显示密码切换（2026-11-01 第 83 轮；R29 换成眼睛图标 + 更大点击区） -->
          <view
            class="pw-toggle"
            :class="{ on: showPassword }"
            style="flex-shrink: 0"
            @click="showPassword = !showPassword"
          >
            <Icon :name="showPassword ? 'eye-off' : 'eye'" :size="36" />
            <text class="pw-toggle-label">{{ showPassword ? '隐藏' : '显示' }}</text>
          </view>
        </view>
        <!-- R81 空字段实时提示：按钮变暗的同时说明为什么 -->
        <text v-if="!username || !password" class="form-hint">请填写用户名与口令后可登录</text>
        <view
          class="btn btn-primary"
          :class="{ 'btn-dim': submitting || !username || !password }"
          @click="submit"
        >
          <view v-if="submitting" class="spinner" />
          {{ submitting ? '登录中…' : '登录' }}
        </view>
        <text v-if="errMsg" :key="errMsg" class="muted shake" style="display: block; margin-top: 12rpx; color: var(--bad)">
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
import { useStatusBarPad } from '@/utils/statusBar'
import Icon from '@/components/Icon.vue'

const store = useGameStore()

/** 本页是 `navigationStyle: custom`：顶部避让只走这一种机制（2026-11-16）。 */
const { statusBarPad } = useStatusBarPad()

const username = ref('')
const password = ref('')
const errMsg = ref('')
const submitting = ref(false)
/** 显示密码（2026-11-01 新一轮第 83 轮）：避免长口令输错。 */
const showPassword = ref(false)

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
/* R81 表单必填提示 */
.form-hint {
  display: block;
  font-size: 22rpx;
  color: var(--warn);
  margin: 4rpx 0 12rpx;
}

/* R29 口令显隐：眼睛图标 + 足够大的点击盒 + 聚焦反馈 */
.pw-toggle {
  display: flex;
  flex-direction: row;
  align-items: center;
  gap: 4rpx;
  padding: 12rpx 8rpx;
  border-radius: 10rpx;
  color: var(--muted);
}

.pw-toggle.on {
  color: var(--fire);
}

.pw-toggle-label {
  font-size: 22rpx;
}

.login-hero {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 12rpx;
  padding: 48rpx 0 36rpx;
}

.login-hero .nick {
  font-size: 44rpx;
  font-weight: 800;
  letter-spacing: 4rpx;
}

.avatar {
  width: 128rpx;
  height: 128rpx;
  border-radius: 28rpx;
  background: var(--accent-grad);
  box-shadow: 0 10rpx 30rpx rgba(255, 107, 53, 0.35);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 60rpx;
  font-weight: 800;
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


/*
 * 登录按钮保持可点击（仅视觉变灰）。
 *
 * 全局 `.btn-disabled` 带 `pointer-events: none` —— 口令未填时按钮挂上它，
 * 点击被 CSS 直接吞掉，`submit()` 里的空值校验（"请填写用户名与口令"）
 * 永远弹不出来，看起来像按钮坏了。这里改用纯视觉的 `btn-dim`：
 * 半透明提示"还没法提交"，但点击可达 submit()，由它给出具体提示。
 * 提交中防重入由 submit() 开头的 `if (submitting.value) return` 兜底。
 */
.btn-dim {
  opacity: 0.4;
}
</style>
