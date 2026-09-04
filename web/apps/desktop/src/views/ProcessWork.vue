<template>
  <div class="placeholder-view">
    <div class="placeholder-icon">{{ icon }}</div>
    <h2>{{ title }}</h2>
    <p class="hint">模块开发中，敬请期待...</p>
    <router-link to="/app/dashboard" class="back-btn">返回工作台</router-link>
  </div>
</template>

<script setup lang="ts">
defineProps<{ icon: string; title: string }>();
</script>

<style scoped>
.placeholder-view {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  min-height: 400px;
  gap: 16px;
}
.placeholder-icon { font-size: 64px; }
h2 {
  font-family: 'Orbitron', sans-serif;
  color: var(--color-primary);
  font-size: 24px;
}
.hint { color: var(--text-muted); font-size: 14px; }
.back-btn {
  margin-top: 16px;
  padding: 10px 24px;
  background: var(--color-primary-soft);
  border: 1px solid var(--color-primary);
  border-radius: var(--radius-md);
  color: var(--color-primary);
  text-decoration: none;
  font-size: 13px;
  transition: all var(--transition-fast);
}
.back-btn:hover { background: var(--color-primary); color: var(--text-inverse); }
</style>
