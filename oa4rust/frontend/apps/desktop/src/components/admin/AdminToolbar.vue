<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<!-- R298 标准列表工具条：搜索框(v-model) + 筛选插槽 + 刷新/新建 -->
<template>
  <div class="admin-toolbar glass-card ai-page-b5">
    <input
      :value="modelValue"
      class="admin-toolbar-search"
      type="search"
      :placeholder="searchPlaceholder"
      :aria-label="searchPlaceholder"
      @input="emit('update:modelValue', ($event.target as HTMLInputElement).value)"
      @keydown.enter="emit('search')"
    />
    <div class="admin-toolbar-filters">
      <slot name="filters" />
    </div>
    <div class="admin-toolbar-actions">
      <slot name="actions" />
      <button type="button" class="admin-toolbar-btn" @click="emit('refresh')">刷新</button>
      <button v-if="createLabel" type="button" class="admin-toolbar-btn primary" @click="emit('create')">
        {{ createLabel }}
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
withDefaults(
  defineProps<{
    modelValue?: string
    searchPlaceholder?: string
    createLabel?: string
  }>(),
  {
    modelValue: '',
    searchPlaceholder: '搜索…',
    createLabel: '',
  },
)

const emit = defineEmits<{
  'update:modelValue': [value: string]
  search: []
  refresh: []
  create: []
}>()
</script>

<style scoped>
.admin-toolbar.ai-page-b5 {
  background: var(--ai-glass-bg);
  border: 1px solid var(--ai-glass-border);
  border-radius: var(--ai-radius-lg, 12px);
  display: flex;
  align-items: center;
  gap: var(--space-3);
  flex-wrap: wrap;
  padding: var(--space-3);
  border-radius: var(--radius-md);
  margin-bottom: var(--space-4);
}

.admin-toolbar-search {
  flex: 1;
  min-width: 200px;
  padding: var(--space-2) var(--space-3);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-sm);
  background: var(--bg-surface);
  color: var(--text-primary);
}

.admin-toolbar-search:focus {
  outline: none;
  border-color: var(--border-active);
}

.admin-toolbar-filters {
  display: flex;
  align-items: center;
  gap: var(--space-2);
}

.admin-toolbar-actions {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  margin-left: auto;
}

.admin-toolbar-btn {
  padding: var(--space-2) var(--space-3);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-sm);
  background: var(--bg-surface);
  color: var(--text-secondary);
  cursor: pointer;
  transition: background var(--transition-fast), color var(--transition-fast);
}

.admin-toolbar-btn:hover {
  background: var(--color-primary-soft);
  color: var(--text-primary);
}

.admin-toolbar-btn.primary {
  background: var(--ai-gradient-brand);
  border-color: var(--color-primary);
  color: var(--bg-base);
}
</style>
