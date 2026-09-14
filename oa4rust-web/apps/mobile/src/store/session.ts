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

  setAuthenticationFailureHandler(() => {
    user.value = null
    initialized.value = true
  })

  async function restoreCurrentUser(): Promise<void> {
    try {
      const resp = await authApi.who()
      user.value = isAuthenticatedUser(resp.data) ? resp.data : null
    } catch {
      user.value = null
    }
  }

  async function init(force = false): Promise<void> {
    if (initialized.value && !force) return
    loading.value = true
    try {
      await restoreCurrentUser()
    } finally {
      loading.value = false
      initialized.value = true
    }
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
