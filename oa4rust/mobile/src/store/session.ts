// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/**
 * 移动端会话存储（pinia）。
 * 与桌面端 @oa4rust/sdk 的 session 语义对齐：init 时拉取 /who 恢复会话，
 * 401 由传输层的 refresh + 认证失败回调驱动本地登出。
 */

import type { O2User } from '@oa4rust/sdk'
import { defineStore } from 'pinia'
import { computed, ref } from 'vue'
import { authApi } from '@/services'
import { setAuthenticationFailureHandler } from '@/services/http'

function isAuthenticatedUser(user: O2User | null | undefined): user is O2User {
  return Boolean(user?.unique)
}

export const useSessionStore = defineStore('mobile-session', () => {
  const user = ref<O2User | null>(null)
  const loading = ref(false)
  const initialized = ref(false)
  // 代际令牌：force init 发起新 /who 读取时递增，旧代响应晚到不得覆盖新身份（对齐桌面 R172）。
  let initTicket = 0

  setAuthenticationFailureHandler(() => {
    user.value = null
    initialized.value = true
  })

  async function restoreCurrentUser(ticket: number): Promise<void> {
    try {
      const resp = await authApi.who()
      if (ticket === initTicket) user.value = isAuthenticatedUser(resp.data) ? resp.data : null
    } catch {
      if (ticket === initTicket) user.value = null
    }
  }

  // 在途普通 init 合并：多个页面 onShow 同时触发只打一次 /who。
  // force 不复用在途请求（身份已变，旧响应可能把新身份覆盖回去）。
  let pendingInit: Promise<void> | null = null

  async function init(force = false): Promise<void> {
    if (initialized.value && !force) return
    if (!force && pendingInit) return pendingInit
    const ticket = ++initTicket
    loading.value = true
    const task = restoreCurrentUser(ticket).finally(() => {
      // 只有当前代才收尾：被 force 取代的旧请求后完成不得提前解锁或覆盖状态。
      if (ticket === initTicket) {
        loading.value = false
        initialized.value = true
      }
      // force 发起的新代不会覆盖 pendingInit；旧代完成后自行清理。
      if (pendingInit === task) pendingInit = null
    })
    if (!force) pendingInit = task
    return task
  }

  async function login(
    credential: string,
    password: string,
    captchaId?: string,
    captchaAnswer?: string,
  ): Promise<O2User> {
    await authApi.login({ credential, password, captchaId, captchaAnswer })
    await init(true)
    if (!user.value) throw new Error('Login did not create a session')
    return user.value
  }

  async function logout(): Promise<void> {
    try {
      await authApi.logout()
    } catch {
      // 本地会话无论如何都会结束；服务端调用失败不阻塞清除本地用户。
    } finally {
      user.value = null
      initialized.value = true
    }
  }

  async function refresh(): Promise<void> {
    await authApi.refresh()
    await init(true)
  }

  const isAuthenticated = computed(() => isAuthenticatedUser(user.value))

  return {
    user,
    loading,
    initialized,
    isAuthenticated,
    init,
    login,
    logout,
    refresh,
  }
})

export function useSession(): ReturnType<typeof useSessionStore> {
  return useSessionStore()
}
