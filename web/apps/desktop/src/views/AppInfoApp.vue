<template>
  <div class="work-view">
    <div class="view-header glass-card">
      <h1>AppInfoApp</h1>
      <p class="subtitle">接入 /jaxrs/📱 — 模块开发中</p>
    </div>
    <div class="content-panel glass-card">
      <div class="empty-state">
        <div class="empty-icon">📱</div>
        <p>AppInfoApp 模块开发中，敬请期待...</p>
        <p class="hint">相关 API: /jaxrs/📱</p>
      </div>
    </div>
  </div>
</template>
<script setup lang="ts">
import { ref } from 'vue';
const loading = ref(false);
const items = ref([]);
</script>
<style scoped>
.work-view { display: flex; flex-direction: column; gap: 16px; height: 100%; }
.view-header { padding: 20px 24px; }
.view-header h1 { font-family: 'Orbitron', sans-serif; font-size: 20px; color: var(--color-primary); margin: 0 0 4px; text-shadow: 0 0 15px var(--color-primary-glow); }
.subtitle { font-size: 12px; color: var(--text-muted); margin: 0; font-family: 'JetBrains Mono', monospace; }
.content-panel { flex: 1; overflow-y: auto; padding: 16px; }
.empty-state { display: flex; flex-direction: column; align-items: center; justify-content: center; padding: 60px; color: var(--text-muted); gap: 12px; }
.empty-icon { font-size: 64px; opacity: 0.4; }
.hint { font-size: 12px; color: var(--text-muted); font-family: 'JetBrains Mono', monospace; }
</style>
