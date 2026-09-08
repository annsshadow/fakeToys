import { ref, readonly } from 'vue';
import { defineStore } from 'pinia';
import type { O2User, SessionState } from './types.js';
import { api } from './api.js';

const STORAGE_KEY = 'oa4rust_session';

function loadStored(): { token: string; user: O2User } | null {
  try {
    const raw = localStorage.getItem(STORAGE_KEY);
    if (!raw) return null;
    return JSON.parse(raw) as { token: string; user: O2User };
  } catch { return null; }
}

function storeSession(token: string, user: O2User): void {
  localStorage.setItem(STORAGE_KEY, JSON.stringify({ token, user }));
}

function clearStored(): void {
  localStorage.removeItem(STORAGE_KEY);
}

export const useSessionStore = defineStore('session', () => {
  const state = ref<SessionState>({ token: null, user: null, loading: true, systemUninitialized: false });

  async function init(): Promise<void> {
    state.value.loading = true;
    const stored = loadStored();
    if (stored?.token) { state.value.token = stored.token; state.value.user = stored.user; }
    try {
      const resp = await api.get<O2User>('/jaxrs/authentication/who');
      const who = resp.data;
      if (who) {
        state.value.user = who;
        if (!state.value.token) state.value.token = stored?.token ?? '';
        storeSession(state.value.token!, who);
      } else { await logout(); }
    } catch { await logout(); }
    finally { state.value.loading = false; }
  }

  async function login(username: string, password: string, captchaId?: string, captchaAnswer?: string): Promise<O2User> {
    const resp = await api.post<{ token: string; person: O2User }>(
      '/jaxrs/authentication/login',
      { username, password, captchaId, captchaAnswer },
      { requireAuth: false },
    );
    const { token, person } = resp.data;
    if (token && person) { state.value.token = token; state.value.user = person; storeSession(token, person); }
    return person;
  }

  async function logout(): Promise<void> {
    try { await api.post('/jaxrs/authentication/logout', null, { requireAuth: false }); }
    finally { state.value.token = null; state.value.user = null; clearStored(); }
  }

  async function refresh(): Promise<void> {
    const resp = await api.post<{ token: string }>('/jaxrs/authentication/refresh', null, { requireAuth: false });
    const token = resp.data.token;
    if (token) {
      state.value.token = token;
      const stored = loadStored();
      if (stored) storeSession(token, stored.user!);
    }
  }

  async function switchUser(targetUnique: string): Promise<O2User> {
    const resp = await api.post<O2User>('/jaxrs/authentication/switchuser', { targetUnique });
    const user = resp.data;
    if (user) { state.value.user = user; storeSession(state.value.token!, user); }
    return user;
  }

  /** OAuth / SSO 回调专用：跳过登录请求，直接将服务端发放的 token+user 写入 store。 */
  function setSession(token: string, user: O2User): void {
    state.value.token = token;
    state.value.user = user;
    storeSession(token, user);
  }

  return {
    state: readonly(state),
    init, login, logout, refresh, switchUser, setSession,
    get isAuthenticated() { return !!state.value.token && !!state.value.user; },
  };
});

export function useSession() { return useSessionStore(); }
