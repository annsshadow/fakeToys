import { defineStore } from 'pinia'
import { computed, readonly, ref } from 'vue'
import { AuthenticationError, api } from './api.js'
import type { O2User, SessionState } from './types.js'

const LEGACY_STORAGE_KEYS = ['oa4rust_session']
let initPromise: Promise<void> | null = null
let legacyStorageCleared = false

function clearLegacyStorage(): void {
  if (legacyStorageCleared) return
  legacyStorageCleared = true
  for (const name of ['localStorage', 'sessionStorage'] as const) {
    let storage: Storage | undefined
    try {
      storage = globalThis[name]
    } catch {
      continue
    }
    for (const key of LEGACY_STORAGE_KEYS) storage?.removeItem(key)
  }
}

function isAuthenticatedUser(user: O2User | null | undefined): user is O2User {
  return Boolean(user?.unique)
}

export const useSessionStore = defineStore('session', () => {
  const state = ref<SessionState>({
    user: null,
    loading: false,
    initialized: false,
    systemUninitialized: false,
  })
  api.setAuthenticationFailureHandler(() => {
    state.value.user = null
    state.value.initialized = true
  })

  async function restoreCurrentUser(): Promise<void> {
    try {
      const resp = await api.get<O2User>('/jaxrs/authentication/who', { requireAuth: false })
      state.value.user = isAuthenticatedUser(resp.data) ? resp.data : null
    } catch {
      state.value.user = null
    }
  }

  async function init(force = false): Promise<void> {
    clearLegacyStorage()
    if (state.value.initialized && !force) return
    if (!initPromise) {
      state.value.loading = true
      initPromise = restoreCurrentUser().finally(() => {
        state.value.loading = false
        state.value.initialized = true
        initPromise = null
      })
    }
    await initPromise
  }

  async function login(
    credential: string,
    password: string,
    captchaId?: string,
    captchaAnswer?: string,
  ): Promise<O2User> {
    await api.post<never>(
      '/jaxrs/authentication/login',
      { credential, password, captchaId, captchaAnswer },
      { requireAuth: false, discardResponse: true },
    )
    await init(true)
    if (!state.value.user) throw new AuthenticationError('Login did not create a session')
    return state.value.user
  }

  async function logout(): Promise<void> {
    try {
      await api.post('/jaxrs/authentication/logout', null, {
        requireAuth: false,
        discardResponse: true,
      })
    } catch {
      // The local session is ending regardless; a failed server-side call must not
      // block clearing the local user or surface an error to the caller.
    } finally {
      state.value.user = null
      state.value.initialized = true
    }
  }

  async function refresh(): Promise<void> {
    await api.refreshSession()
    await init(true)
  }

  async function switchUser(targetUnique: string): Promise<O2User> {
    await api.post('/jaxrs/authentication/switchuser', { targetUnique }, { discardResponse: true })
    await init(true)
    if (!state.value.user) throw new AuthenticationError('User switch did not create a session')
    return state.value.user
  }

  // Pinia setup stores snapshot object-literal getters at setup time; a computed
  // ref stays live so the router guard and views see current auth state.
  const isAuthenticated = computed(() => isAuthenticatedUser(state.value.user))

  return {
    state: readonly(state),
    isAuthenticated,
    init,
    login,
    logout,
    refresh,
    switchUser,
  }
})

export function useSession() {
  return useSessionStore()
}
