<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<!-- R298 标准管理页页头：标题 + 副标题 + 右侧主操作插槽（新建/导出等） -->
<template>
  <div class="admin-page-header">
    <div class="admin-page-heading">
      <h2 class="admin-page-title">{{ title }}</h2>
      <p v-if="subtitle" class="admin-page-subtitle">{{ subtitle }}</p>
    </div>
    <div class="admin-page-actions">
      <slot name="actions" />
    </div>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  title: string
  subtitle?: string
}>()
</script>

<style scoped>
.admin-page-header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: var(--space-4);
  margin-bottom: var(--space-4);
}

.admin-page-title {
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
