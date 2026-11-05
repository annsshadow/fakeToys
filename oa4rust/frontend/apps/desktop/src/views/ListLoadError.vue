<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<template>
  <div v-if="mode === 'empty'" class="list-error-empty">
    <div class="list-error-icon">⚠️</div>
    <p class="list-error-text">{{ message }}</p>
    <button class="list-error-retry" @click="emit('retry')">重试</button>
  </div>
  <div v-else class="list-error-banner">
    <span class="list-error-text">{{ message }}</span>
    <button class="list-error-retry" @click="emit('retry')">重试</button>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  /** 失败文案（来自 useListError 的 loadError）。 */
  message: string
  /** empty = 首屏失败的空态位；banner = 已有旧数据时的顶部横幅。 */
  mode: 'empty' | 'banner'
}>()

const emit = defineEmits<{ retry: [] }>()
</script>

<style scoped>
.list-error-empty {
  padding: 40px;
  text-align: center;
  color: var(--text-muted);
}
.list-error-icon {
  font-size: 32px;
  margin-bottom: 8px;
}
.list-error-text {
  color: var(--color-error, #c0392b);
  margin: 0 0 12px;
  line-height: 1.5;
  word-break: break-word;
}
.list-error-retry {
  padding: 6px 16px;
  border-radius: var(--radius-md);
  border: 1px solid var(--color-error, #c0392b);
  background: transparent;
  color: var(--color-error, #c0392b);
  cursor: pointer;
}
.list-error-banner {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  padding: 10px 14px;
  margin-bottom: 12px;
  background: var(--bg-elevated);
  border: 1px solid var(--color-error, #c0392b);
  border-radius: var(--radius-md);
}
.list-error-banner .list-error-text {
  margin: 0;
}
</style>
