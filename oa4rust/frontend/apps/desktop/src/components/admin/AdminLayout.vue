<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<template>
  <div class="admin-layout">
    <nav class="admin-sidebar glass-card" aria-label="管理区导航">
      <div class="admin-sidebar-header">
        <span class="admin-sidebar-icon">⚙</span>
        <span class="admin-sidebar-title">系统管理</span>
      </div>
      <div v-for="group in adminNavGroups" :key="group.label" class="admin-nav-group">
        <div class="admin-nav-group-label">
          <span class="admin-nav-group-icon">{{ group.icon }}</span>
          {{ group.label }}
        </div>
        <router-link
          v-for="item in group.items"
          :key="item.path"
          :to="item.path"
          class="admin-nav-item"
          :class="{ active: isActive(item.path) }"
          :aria-current="isActive(item.path) ? 'page' : undefined"
        >
          <span class="admin-nav-icon">{{ item.icon }}</span>
          <span class="admin-nav-label">{{ item.label }}</span>
        </router-link>
      </div>
    </nav>

    <div class="admin-main">
      <header class="admin-header">
        <div class="admin-breadcrumb">
          <router-link to="/app/admin/control-panel" class="admin-breadcrumb-link">系统管理</router-link>
          <span class="admin-breadcrumb-sep">/</span>
          <span class="admin-breadcrumb-group">{{ currentGroupLabel }}</span>
          <span class="admin-breadcrumb-sep">/</span>
          <span class="admin-breadcrumb-current">{{ pageTitle }}</span>
        </div>
        <h1 v-if="!hideTitle" class="admin-title">{{ pageTitle }}</h1>
      </header>
      <div class="admin-content">
        <router-view />
      </div>
    </div>
  </div>
</template>

<script lang="ts">
/**
 * R298 页面标题隐藏开关：自带大标题的管理页 inject 该 key 并将 Ref 置 true，
 * AdminLayout 即隐藏自身 h1；默认 false，既有页面行为不变。
 */
export { ADMIN_LAYOUT_HIDE_TITLE } from './adminInjection'
</script>

<script setup lang="ts">
import { isNavActive } from '@oa4rust/ui/components/navActive'
import { computed, inject, provide, ref } from 'vue'
import { ADMIN_LAYOUT_HIDE_TITLE } from './adminInjection'
import { useRoute } from 'vue-router'
import { adminNavGroups } from './adminNav'

const route = useRoute()

function isActive(path: string): boolean {
  return isNavActive(route.path, path)
}

const pageTitle = computed<string>(() => (route.meta.title as string | undefined) ?? '系统管理')

const currentGroupLabel = computed<string>(() => {
  const group = adminNavGroups.find((g) => g.items.some((item) => isNavActive(route.path, item.path)))
  return group?.label ?? '概览'
})

// 允许更上层统一控制，也向子路由提供开关；默认不隐藏标题。
const inheritedHideTitle = inject(ADMIN_LAYOUT_HIDE_TITLE, undefined)
const hideTitle = inheritedHideTitle ?? ref(false)
provide(ADMIN_LAYOUT_HIDE_TITLE, hideTitle)
</script>

<style scoped>
.admin-layout {
  display: flex;
  gap: var(--space-4);
  align-items: flex-start;
  min-height: 100%;
}

.admin-sidebar {
  position: sticky;
  top: 0;
  width: 232px;
  flex-shrink: 0;
  max-height: calc(100vh - var(--topbar-height) - var(--space-8));
  overflow-y: auto;
  padding: var(--space-3);
  border-radius: var(--radius-lg);
}

.admin-sidebar-header {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  padding: var(--space-2) var(--space-3);
  font-weight: 600;
  color: var(--text-primary);
}

.admin-nav-group {
  margin-top: var(--space-3);
}

.admin-nav-group-label {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  padding: var(--space-1) var(--space-3);
  font-size: 12px;
  color: var(--text-muted);
  text-transform: none;
}

.admin-nav-item {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  padding: var(--space-2) var(--space-3);
  border-radius: var(--radius-md);
  color: var(--text-secondary);
  text-decoration: none;
  transition: background var(--transition-fast), color var(--transition-fast);
}

.admin-nav-item:hover {
  background: var(--color-primary-soft);
  color: var(--text-primary);
}

.admin-nav-item.active {
  background: var(--color-primary-soft);
  color: var(--color-primary);
  box-shadow: inset 2px 0 0 var(--color-primary);
}

.admin-nav-icon {
  flex-shrink: 0;
}

.admin-nav-label {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.admin-main {
  flex: 1;
  min-width: 0;
}

.admin-header {
  margin-bottom: var(--space-4);
}

.admin-breadcrumb {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  font-size: 12px;
  color: var(--text-muted);
}

.admin-breadcrumb-link {
  color: var(--text-muted);
  text-decoration: none;
}

.admin-breadcrumb-link:hover {
  color: var(--color-primary);
}

.admin-breadcrumb-current {
  color: var(--text-secondary);
}

.admin-title {
  margin: var(--space-2) 0 0;
  font-size: 20px;
  font-weight: 600;
  color: var(--text-primary);
}

@media (max-width: 768px) {
  .admin-layout {
    flex-direction: column;
  }

  .admin-sidebar {
    position: static;
    width: 100%;
    max-height: none;
  }
}
</style>
