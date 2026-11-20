<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<!-- R301 端点折叠容器：把密集的 API 调试按钮默认收纳，按需展开；支持读/写/配置分组。 -->
<template>
  <section class="endpoint-fold" :data-testid="testId">
    <button
      type="button"
      class="endpoint-fold-trigger"
      :aria-expanded="open"
      :data-testid="testId + '-toggle'"
      @click="open = !open"
    >
      <span class="endpoint-fold-icon" aria-hidden="true">⚙</span>
      <span class="endpoint-fold-label">{{ label }}</span>
      <span class="endpoint-fold-badge">{{ totalCount }}</span>
      <span class="endpoint-fold-hint">{{ open ? '点击收起' : '展开查看全部端点' }}</span>
      <span class="endpoint-fold-chevron" aria-hidden="true">{{ open ? '▾' : '▸' }}</span>
    </button>
    <div v-show="open" class="endpoint-fold-panel" :data-testid="testId + '-panel'">
      <template v-if="groups && groups.length">
        <div v-for="group in groups" :key="group.key" class="endpoint-fold-group">
          <h4 class="endpoint-fold-group-title">{{ group.label }}</h4>
          <div class="endpoint-fold-buttons">
            <slot :name="'group-' + group.key" />
          </div>
        </div>
      </template>
      <div v-else class="endpoint-fold-buttons">
        <slot />
      </div>
    </div>
  </section>
</template>

<script setup lang="ts">
import { ref } from 'vue'

interface EndpointFoldGroup {
  key: string
  label: string
}

const props = withDefaults(
  defineProps<{
    label?: string
    groups?: EndpointFoldGroup[]
    totalCount?: number
    testId?: string
    defaultOpen?: boolean
  }>(),
  {
    label: '端点调试',
    groups: () => [],
    totalCount: 0,
    testId: 'endpoint-fold',
    defaultOpen: false,
  },
)

// 默认折叠；仅在显式 defaultOpen 时展开，确保页面首屏清爽。
const open = ref(props.defaultOpen)
</script>

<style scoped>
.endpoint-fold {
  margin: 10px 0;
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-md);
  background: var(--bg-elevated);
  overflow: hidden;
}

.endpoint-fold-trigger {
  display: flex;
  align-items: center;
  gap: 10px;
  width: 100%;
  padding: 10px 14px;
  border: 0;
  background: transparent;
  color: var(--text-primary);
  cursor: pointer;
  text-align: left;
}

.endpoint-fold-trigger:hover {
  background: var(--color-primary-soft, rgba(127, 127, 255, 0.08));
}

.endpoint-fold-icon {
  color: var(--color-primary);
  font-size: 15px;
}

.endpoint-fold-label {
  font-weight: 600;
  font-size: 13px;
}

.endpoint-fold-badge {
  min-width: 20px;
  padding: 1px 7px;
  border-radius: 999px;
  background: var(--color-primary-soft, rgba(127, 127, 255, 0.12));
  color: var(--color-primary);
  font-size: 11px;
  text-align: center;
}

.endpoint-fold-hint {
  color: var(--text-muted);
  font-size: 11px;
}

.endpoint-fold-chevron {
  margin-left: auto;
  color: var(--text-secondary);
}

.endpoint-fold-panel {
  padding: 12px 14px 16px;
  border-top: 1px solid var(--border-subtle);
}

.endpoint-fold-group + .endpoint-fold-group {
  margin-top: 14px;
}

.endpoint-fold-group-title {
  margin: 0 0 8px;
  color: var(--color-secondary, var(--text-secondary));
  font-size: 12px;
  font-weight: 600;
  letter-spacing: 0.05em;
}

.endpoint-fold-buttons {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}
</style>
