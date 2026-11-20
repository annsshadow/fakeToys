<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<!-- R298 表单分区卡片：标题 + 描述 + 默认插槽（表单项） -->
<template>
  <section class="admin-form-section glass-card">
    <div class="admin-form-heading">
      <h3 class="admin-form-title">{{ title }}</h3>
      <p v-if="description" class="admin-form-description">{{ description }}</p>
    </div>
    <div class="admin-form-body">
      <slot />
    </div>
  </section>
</template>

<script setup lang="ts">
defineProps<{
  title: string
  description?: string
}>()
</script>

<style scoped>
.admin-form-section {
  border-radius: var(--radius-md);
  padding: var(--space-4);
  margin-bottom: var(--space-4);
}

.admin-form-title {
  margin: 0;
  font-size: 15px;
  font-weight: 600;
  color: var(--text-primary);
}

.admin-form-description {
  margin: var(--space-1) 0 0;
  font-size: 13px;
  color: var(--text-secondary);
}

.admin-form-body {
  margin-top: var(--space-3);
}
</style>
