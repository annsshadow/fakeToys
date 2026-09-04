<template>
  <div class="oauth-callback">
    <div class="callback-card glass-card">
      <div class="spinner-large"></div>
      <p class="callback-text">正在处理 {{ platform }} 登录...</p>
      <p v-if="error" class="error-text">{{ error }}</p>
      <router-link v-if="error" to="/login" class="back-link">返回登录</router-link>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { useRouter, useRoute } from 'vue-router';
import { useSession } from '@oa4rust/sdk';
import { api } from '@oa4rust/sdk';

const router = useRouter();
const route = useRoute();
const session = useSession();

const platform = ref(route.params.platform as string || '第三方');
const error = ref('');

onMounted(async () => {
  const code = route.query.code as string;
  if (!code) {
    error.value = '缺少授权码';
    return;
  }

  try {
    // 调用后端 OAuth 登录端点
    const resp = await api.post<{ data: { token: string; person: import('@oa4rust/sdk').O2User } }>(
      `/jaxrs/authentication/oauth/login/${platform.value}/code/${code}`,
      null,
    );

    const { token, person } = resp.data;
    session.login(person.name, '', undefined, undefined);
    // 写入 session（简化处理，实际应复用 login 方法）
    localStorage.setItem('oa4rust_session', JSON.stringify({ token, user: person }));
    session.state.value = {
      token,
      user: person,
      loading: false,
      systemUninitialized: false,
    };

    router.replace('/app/dashboard');
  } catch (e: unknown) {
    error.value = e instanceof Error ? e.message : 'OAuth 登录失败';
  }
});
</script>

<style scoped>
.oauth-callback {
  height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--bg-base);
}

.callback-card {
  padding: 48px 60px;
  text-align: center;
}

.spinner-large {
  width: 48px;
  height: 48px;
  border: 3px solid var(--border-subtle);
  border-top-color: var(--color-primary);
  border-radius: 50%;
  animation: spin 0.8s linear infinite;
  margin: 0 auto 20px;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

.callback-text {
  color: var(--text-secondary);
  font-size: 15px;
}

.error-text {
  color: var(--color-error);
  margin-top: 12px;
  font-size: 14px;
}

.back-link {
  display: inline-block;
  margin-top: 16px;
  color: var(--color-primary);
  font-size: 13px;
  text-decoration: none;
}

.back-link:hover {
  text-decoration: underline;
}
</style>
