<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<!-- R298 标准管理页页头：标题 + 副标题 + 右侧主操作插槽（新建/导出等） -->
<template>
  <div class="admin-page-header ai-page-b5">
    <div v-if="!hidePageTitle" class="admin-page-heading">
      <h2 class="admin-page-title ai-gradient-text">{{ title }}</h2>
      <p v-if="subtitle" class="admin-page-subtitle">{{ subtitle }}</p>
    </div>
    <div class="admin-page-actions">
      <slot name="actions" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { inject } from 'vue'
import { ADMIN_HIDE_PAGE_TITLE } from './adminInjection'

defineProps<{
  title: string
  subtitle?: string
}>()

// R300：AdminLayout 提供该开关；子页 inject 后置 true 隐藏标题/副标题，保留 actions 插槽。
const hidePageTitle = inject(ADMIN_HIDE_PAGE_TITLE, undefined)
</script>

<style scoped>
.admin-page-header.ai-page-b5 {
  background: var(--ai-gradient-surface);
  border: 1px solid var(--ai-glass-border);
  border-radius: var(--ai-radius-lg, 12px);
  padding: var(--ai-grid-2, 8px) var(--ai-grid-3, 12px);
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: var(--space-4);
  margin-bottom: var(--space-4);
}

.admin-page-title.ai-gradient-text {
  background: var(--ai-gradient-text);
  -webkit-background-clip: text;
  background-clip: text;
  color: transparent;
  margin: 0;
  font-size: 18px;
  font-weight: 600;
  color: var(--text-primary);
}

.admin-page-subtitle {
  margin: var(--space-1) 0 0;
  font-size: 13px;
  color: var(--text-secondary);
}

.admin-page-actions {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  flex-shrink: 0;
}
</style>
