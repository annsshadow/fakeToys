<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<!-- R298 标准列表表格：表头/数据行/骨架加载/空态；错误态复用 ListLoadError 并转发 retry -->
<template>
  <div class="admin-table glass-card ai-page-b5" :class="densityClass">
    <div class="admin-table-density-bar">
      <span>表格密度</span>
      <button type="button" data-testid="admin-table-density" :class="['density-btn', { active: density === 'standard' }]" @click="density = 'standard'">标准</button>
      <button type="button" class="density-btn" :class="{ active: density === 'compact' }" @click="density = 'compact'">紧凑</button>
    </div>
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
import { computed, ref } from 'vue'
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

const density = ref<'standard' | 'compact'>('standard')
const densityClass = computed(() => (density.value === 'compact' ? 'admin-table--compact' : 'admin-table--standard'))

function rowKey(row: Record<string, unknown>, index: number): string | number {
  const key = row[props.rowKeyField]
  return typeof key === 'string' || typeof key === 'number' ? key : index
}
</script>

<style scoped>
.admin-table-density-bar {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  gap: var(--space-2);
  padding: 0 0 var(--space-2);
  font-size: 12px;
  color: var(--text-muted);
}
.density-btn {
  padding: 2px 10px;
  border-radius: var(--radius-sm);
  border: 1px solid var(--border-subtle);
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
  font-size: 12px;
}
.density-btn.active {
  border-color: var(--color-primary);
  color: var(--color-primary);
  background: var(--color-primary-soft);
}
.admin-table--compact .admin-table-th,
.admin-table--compact .admin-table-td {
  padding: calc(var(--space-1, 4px)) var(--space-2);
  font-size: 12px;
}
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
