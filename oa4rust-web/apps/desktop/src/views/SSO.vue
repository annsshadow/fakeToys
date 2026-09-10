<template>
  <div class="sso-view">
    <div class="sso-card glass-card">
      <div class="sso-icon">🔐</div>
      <h2>单点登录</h2>
      <p class="status" :class="statusClass">{{ statusText }}</p>
      <div class="sso-detail" v-if="detail">{{ detail }}</div>
      <router-link to="/login" class="back-btn" v-if="status === 'error'">返回登录</router-link>
      <router-link to="/app/dashboard" class="back-btn" v-if="status === 'success'">进入工作台</router-link>
    </div>
  </div>
</template>
<script setup lang="ts">
import { useSession } from '@oa4rust/sdk'
import { onMounted, ref } from 'vue'
import { useRoute, useRouter } from 'vue-router'

const route = useRoute()
const router = useRouter()
const session = useSession()
const status = ref<'loading' | 'success' | 'error'>('loading')
const detail = ref('')

onMounted(async () => {
  // SSO redirect delivers the provider-issued client + token; pass them straight to
  // the backend (never persist or log them). Route has no :platform param, so both
  // arrive via query.
  const q = route.query as Record<string, string>
  const client = q.client ?? ''
  const ssoToken = q.token ?? ''
  try {
    if (!client || !ssoToken) throw new Error('missing client/token')
    const response = await fetch('/jaxrs/authentication/sso', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      credentials: 'include',
      body: JSON.stringify({ client, token: ssoToken }),
    })
    if (!response.ok) throw new Error(`认证失败 (${response.status})`)
    await session.init(true)
    if (!session.isAuthenticated) throw new Error('SSO 登录未建立会话')
    status.value = 'success'
    detail.value = `${client} SSO 授权成功`
    setTimeout(() => router.replace('/app/dashboard'), 1000)
  } catch (e: unknown) {
    status.value = 'error'
    detail.value = e instanceof Error ? e.message : '认证失败'
  }
})
</script>
<style scoped>
.sso-view{display:flex;align-items:center;justify-content:center;min-height:100vh;background:var(--bg-base)}
.sso-card{text-align:center;padding:48px 40px;max-width:400px;width:90%;border:1px solid var(--border-subtle);border-radius:var(--radius-lg);background:var(--bg-surface)}
.sso-icon{font-size:56px;margin-bottom:16px}
h2{font-family:'Orbitron',sans-serif;color:var(--color-primary);margin:0 0 12px;font-size:22px}
.status{font-size:14px;color:var(--text-muted);margin:0 0 8px}
.status.loading{color:var(--color-info)}.status.success{color:var(--color-success)}.status.error{color:var(--color-error)}
.sso-detail{font-size:12px;color:var(--text-muted);font-family:'JetBrains Mono',monospace;margin-bottom:20px;word-break:break-all}
.back-btn{display:inline-block;padding:10px 24px;margin:4px;background:var(--color-primary-soft);border:1px solid var(--color-primary);border-radius:var(--radius-md);color:var(--color-primary);text-decoration:none;font-size:13px;transition:all var(--transition-fast)}
.back-btn:hover{background:var(--color-primary);color:var(--text-inverse)}
</style>
