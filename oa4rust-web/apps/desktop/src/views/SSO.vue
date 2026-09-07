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
import { ref, computed, onMounted } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { api } from '@oa4rust/sdk';

const route = useRoute();
const router = useRouter();
const status = ref<'loading' | 'success' | 'error'>('loading');
const detail = ref('');

const statusText = computed(() => {
  if (status.value === 'loading') return '正在处理 SSO 登录...';
  if (status.value === 'success') return '登录成功，正在跳转...';
  return '登录失败，请重试';
});
const statusClass = computed(() => status.value);

onMounted(async () => {
  const platform = route.params.platform as string;
  const token = route.query.token as string;
  const code = route.query.code as string;
  try {
    if (token) {
      await api.post('/jaxrs/authentication/sso', { token });
      status.value = 'success';
      detail.value = `SSO 平台: ${platform}`;
      setTimeout(() => router.replace('/app/dashboard'), 1000);
    } else if (code) {
      await api.post('/jaxrs/authentication/sso', { platform, code });
      status.value = 'success';
      detail.value = `${platform} OAuth 授权成功`;
      setTimeout(() => router.replace('/app/dashboard'), 1000);
    } else {
      throw new Error('缺少 token 或 code 参数');
    }
  } catch (e: any) {
    status.value = 'error';
    detail.value = e?.message ?? '认证失败';
  }
});
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
