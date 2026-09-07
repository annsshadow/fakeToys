<template>
  <div class="placeholder-view">
    <div class="placeholder-icon">🏢</div>
    <h2>{{ title }}</h2>
    <p class="hint">模块开发中，敬请期待...</p>
    <div class="status-bar" v-if="status">
      <span class="status-dot" :class="status.class"></span>
      <span>{{ status.text }}</span>
    </div>
    <router-link to="/app/dashboard" class="back-btn">← 返回工作台</router-link>
  </div>
</template>
<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { api } from '@oa4rust/sdk';

defineProps<{ title: string }>();

const status = ref<{ text: string; class: string } | null>(null);

onMounted(async () => {
  try {
    const r = await api.get('/jaxrs/base/version');
    status.value = { text: `后端在线 · oa4rust ${r.data?.version ?? 'ok'}`, class: 'ok' };
  } catch {
    status.value = { text: '后端连接异常', class: 'err' };
  }
});
</script>
<style scoped>
.placeholder-view{display:flex;flex-direction:column;align-items:center;justify-content:center;min-height:400px;gap:16px}
.placeholder-icon{font-size:64px}
h2{font-family:'Orbitron',sans-serif;color:var(--color-primary);font-size:24px;margin:0}
.hint{color:var(--text-muted);font-size:14px}
.status-bar{display:flex;align-items:center;gap:8px;font-size:12px;color:var(--text-muted);margin-top:8px}
.status-dot{width:8px;height:8px;border-radius:50%}
.status-dot.ok{background:var(--color-success)}
.status-dot.err{background:var(--color-error)}
.back-btn{margin-top:16px;padding:10px 24px;background:var(--color-primary-soft);border:1px solid var(--color-primary);border-radius:var(--radius-md);color:var(--color-primary);text-decoration:none;font-size:13px;transition:all var(--transition-fast)}
.back-btn:hover{background:var(--color-primary);color:var(--text-inverse)}
</style>
