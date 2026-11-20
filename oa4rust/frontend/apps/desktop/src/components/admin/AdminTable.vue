<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<!-- R298 标准列表表格：表头/数据行/骨架加载/空态；错误态复用 ListLoadError 并转发 retry -->
<template>
  <div class="admin-table glass-card ai-page-b5">
    <ListLoadError v-if="error" :message="error" mode="empty" @retry="emit('retry')" />
    <table v-else class="admin-table-grid">
      <thead>
        <tr>
          <th v-for="col in columns" :key="col.key" class="admin-table-th" :style="col.width ? { width: col.width } : undefined">
            {{ col.label }}
          </th>
        </tr>
      </thead>
      <tbody v-if="loading">
        <tr v-for="n in skeletonRows" :key="'skeleton-' + n">
          <td v-for="col in columns" :key="col.key" class="admin-table-td">
            <div class="skeleton" />
          </td>
        </tr>
      </tbody>
      <tbody v-else-if="rows.length === 0">
        <tr>
          <td :colspan="columns.length" class="admin-table-empty">{{ emptyText }}</td>
        </tr>
      </tbody>
      <tbody v-else>
        <tr v-for="(row, index) in rows" :key="rowKey(row, index)" class="admin-table-row">
          <td v-for="col in columns" :key="col.key" class="admin-table-td">
            <slot :name="'cell-' + col.key" :row="row" :value="row[col.key]">
              {{ row[col.key] ?? '—' }}
            </slot>
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script lang="ts">
export interface AdminTableColumn {
  key: string
  label: string
  width?: string
}
</script>

<script setup lang="ts">
import ListLoadError from '../../views/ListLoadError.vue'

const props = withDefaults(
  defineProps<{
    columns: AdminTableColumn[]
    rows: Record<string, unknown>[]
    loading?: boolean
    error?: string
    emptyText?: string
    skeletonRows?: number
    rowKeyField?: string
  }>(),
  {
    loading: false,
    error: '',
    emptyText: '暂无数据',
    skeletonRows: 5,
    rowKeyField: 'id',
  },
)

const emit = defineEmits<{
  retry: []
}>()

function rowKey(row: Record<string, unknown>, index: number): string | number {
  const key = row[props.rowKeyField]
  return typeof key === 'string' || typeof key === 'number' ? key : index
}
</script>

<style scoped>
.admin-table.ai-page-b5 {
  background: var(--ai-glass-bg);
  border: 1px solid var(--ai-glass-border);
  border-radius: var(--ai-radius-lg, 12px);
  padding: var(--space-3);
  overflow-x: auto;
}

.admin-table-grid {
  width: 100%;
  border-collapse: collapse;
}

.admin-table-th {
  text-align: left;
  padding: var(--space-2) var(--space-3);
  font-size: 12px;
  font-weight: 600;
  color: var(--text-muted);
  border-bottom: 1px solid var(--border-subtle);
  white-space: nowrap;
}

.admin-table-td {
  padding: var(--space-2) var(--space-3);
  color: var(--text-secondary);
  border-bottom: 1px solid var(--border-subtle);
}

.admin-table-row:hover .admin-table-td {
  background: var(--color-primary-soft);
}

.admin-table-td .skeleton {
  background: var(--ai-skeleton-base);
  height: 16px;
  border-radius: var(--radius-sm);
}

.admin-table-empty {
  padding: var(--space-8);
  text-align: center;
  color: var(--text-muted);
}
</style>
