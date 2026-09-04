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
      const resp = await api.get('/jaxrs/authentication/who');
      const who = (resp as any)?.data;
      if (who) {
        state.value.user = who;
        if (!state.value.token) state.value.token = stored?.token ?? '';
        storeSession(state.value.token!, who);
      } else { await logout(); }
    } catch { await logout(); }
    finally { state.value.loading = false; }
  }

  async function login(username: string, password: string, captchaId?: string, captchaAnswer?: string): Promise<O2User> {
    const resp = await api.post('/jaxrs/authentication/login', { username, password, captchaId, captchaAnswer });
    const body = (resp as any)?.data;
    const token = body?.token;
    const person = body?.person;
    if (token && person) { state.value.token = token; state.value.user = person; storeSession(token, person); }
    return person;
  }

  async function logout(): Promise<void> {
    try { await api.post('/jaxrs/authentication/logout', null, { requireAuth: false }); }
    finally { state.value.token = null; state.value.user = null; clearStored(); }
  }

  async function refresh(): Promise<void> {
    const resp = await api.post('/jaxrs/authentication/refresh', null, { requireAuth: false });
    const token = (resp as any)?.data?.token;
    if (token) {
      state.value.token = token;
      const stored = loadStored();
      if (stored) storeSession(token, stored.user!);
    }
  }

  async function switchUser(targetUnique: string): Promise<O2User> {
    const resp = await api.post('/jaxrs/authentication/switchuser', { targetUnique });
    const user = (resp as any)?.data;
    if (user) { state.value.user = user; storeSession(state.value.token!, user); }
    return user;
  }

  return {
    state: readonly(state),
    init, login, logout, refresh, switchUser,
    get isAuthenticated() { return !!state.value.token && !!state.value.user; },
  };
});

export function useSession() { return useSessionStore(); }
