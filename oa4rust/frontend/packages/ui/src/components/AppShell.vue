<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<template>
  <div class="app-shell" :class="{ 'collapsed': sidebarCollapsed, 'mobile': isMobile }">
    <!-- 侧边栏 -->
    <aside class="sidebar" :style="{ width: sidebarCollapsed ? 'var(--sidebar-collapsed)' : 'var(--sidebar-width)' }">
      <div class="sidebar-header">
        <div class="logo" role="button" tabindex="0" aria-label="切换侧边栏" @click="toggleSidebar" @keydown.enter="toggleSidebar" @keydown.space.prevent="toggleSidebar">
          <span class="logo-icon">◆</span>
          <span v-show="!sidebarCollapsed" class="logo-text">OA4RUST</span>
        </div>
      </div>

      <nav class="sidebar-nav" aria-label="主导航">
        <div v-for="group in navGroups" :key="group.label" class="nav-group">
          <div class="nav-section-label">{{ group.label }}</div>
          <div
            v-for="item in group.items"
            :key="item.path"
            class="nav-item"
            role="button"
            tabindex="0"
            :aria-current="isActive(item.path) ? 'page' : undefined"
            :class="{ active: isActive(item.path) }"
            @click="navigate(item)"
            @keydown.enter="navigate(item)"
            @keydown.space.prevent="navigate(item)"
          >
            <span class="nav-icon">{{ item.icon }}</span>
            <span v-show="!sidebarCollapsed" class="nav-label">{{ item.label }}</span>
          </div>
        </div>

        <!-- 系统管理折叠区：管理员专用，默认收起，避免把调试入口推给普通员工 -->
        <div class="nav-group admin-group">
          <button
            class="nav-section-label nav-section-toggle"
            type="button"
            :aria-expanded="adminExpanded"
            aria-controls="admin-nav-items"
            @click="adminExpanded = !adminExpanded"
          >
            <span class="nav-icon">⚙</span>
            <span v-show="!sidebarCollapsed" class="nav-label">系统管理</span>
            <span v-show="!sidebarCollapsed" class="nav-caret">{{ adminExpanded ? '▾' : '▸' }}</span>
          </button>
          <div v-show="adminExpanded" id="admin-nav-items" class="admin-nav-items">
            <div
              v-for="item in adminGroup.items"
              :key="item.path"
              class="nav-item"
              role="button"
              tabindex="0"
              :aria-current="isActive(item.path) ? 'page' : undefined"
              :class="{ active: isActive(item.path) }"
              @click="navigate(item)"
              @keydown.enter="navigate(item)"
              @keydown.space.prevent="navigate(item)"
            >
              <span class="nav-icon">{{ item.icon }}</span>
              <span v-show="!sidebarCollapsed" class="nav-label">{{ item.label }}</span>
            </div>
          </div>
        </div>
      </nav>

      <div class="sidebar-footer">
        <button class="theme-toggle" @click="toggleTheme" title="切换主题">
          {{ theme.theme === 'dark' ? '☀' : '☾' }}
        </button>
      </div>
    </aside>

    <!-- 主区域 -->
    <div class="main-area">
      <!-- 顶栏 -->
      <header class="topbar">
        <div class="topbar-left">
          <button class="collapse-btn" aria-label="展开或收起侧边栏" @click="toggleSidebar">
            ☰
          </button>
          <div class="breadcrumb">{{ currentTitle }}</div>
        </div>

        <div class="topbar-center">
          <div class="search-box">
            <span class="search-icon">⌕</span>
            <input
              v-model="searchQuery"
              placeholder="搜索 (⌘K)"
              data-oa-global-search
              class="search-input"
              @keydown.ctrl.k.prevent="focusSearch"
              @keydown.meta.k.prevent="focusSearch"
            />
          </div>
        </div>

        <div class="topbar-right">
          <button class="icon-btn" title="通知">
            <span class="icon">🔔</span>
            <span v-if="unreadCount > 0" class="badge">{{ unreadCount }}</span>
          </button>
          <div class="user-menu" role="button" tabindex="0" aria-haspopup="menu" :aria-expanded="showUserMenu" aria-label="用户菜单" @click="toggleUserMenu" @keydown.enter="toggleUserMenuOnKeydown" @keydown.space.prevent="toggleUserMenuOnKeydown">
            <img
              v-if="user?.icon"
              :src="user.icon"
              class="user-avatar"
              alt="avatar"
            />
            <span v-else class="user-avatar user-avatar-default">
              {{ user?.name?.charAt(0) || 'U' }}
            </span>
            <span v-show="!isMobile" class="user-name">{{ user?.name || '用户' }}</span>
            <span class="arrow">▼</span>

            <!-- 用户下拉菜单 -->
            <div v-if="showUserMenu" class="user-dropdown glass-card">
              <router-link to="/app/personal" class="dropdown-item">
                <span>👤</span> 个人中心
              </router-link>
              <router-link to="/app/settings" class="dropdown-item">
                <span>⚙</span> 系统设置
              </router-link>
              <div class="dropdown-divider"></div>
              <button class="dropdown-item" @click="handleLogout">
                <span>🚪</span> 退出登录
              </button>
            </div>
          </div>
        </div>
      </header>

      <!-- 内容区 -->
      <main id="main-content" class="content">
        <router-view v-slot="{ Component }">
          <transition name="fade-slide" mode="out-in">
            <component :is="Component" />
          </transition>
        </router-view>
      </main>
    </div>

    <!-- 移动端底部导航 -->
    <nav v-if="isMobile" class="tab-bar" aria-label="底部导航">
      <div
        v-for="item in mobileNavItems"
        :key="item.path"
        class="tab-item"
        role="button"
        tabindex="0"
        :aria-current="isActive(item.path) ? 'page' : undefined"
        :class="{ active: isActive(item.path) }"
        @click="navigate(item)"
        @keydown.enter="navigate(item)"
        @keydown.space.prevent="navigate(item)"
      >
        <span class="tab-icon">{{ item.icon }}</span>
        <span class="tab-label">{{ item.label }}</span>
      </div>
    </nav>
  </div>
</template>

<script setup lang="ts">
import { useSession, useTheme } from '@oa4rust/sdk'
import { computed, onMounted, onUnmounted, ref } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { isNavActive } from './navActive'

const router = useRouter()
const route = useRoute()
const session = useSession()
const { theme, toggleTheme } = useTheme()

const sidebarCollapsed = ref(false)
const showUserMenu = ref(false)
const searchQuery = ref('')
const unreadCount = ref(0)
const isMobile = ref(false)

const currentTitle = computed(() => {
  return (route.meta.title as string) || 'OA4Rust'
})

const user = computed(() => session.state.user ?? null)

// PC 端导航：四组员工常用业务 + 管理员折叠区（系统管理单独渲染，不塞进员工侧栏）
type NavItem = { path: string; label: string; icon: string }
type NavGroup = { label: string; icon: string; admin?: boolean; items: NavItem[] }
const navGroups: NavGroup[] = [
  {
    label: '工作台',
    icon: '🏠',
    items: [
      { path: '/app/dashboard', label: '工作台', icon: '🏠' },
      { path: '/app/directory', label: '通讯录', icon: '📒' },
      { path: '/app/calendar', label: '日历', icon: '📅' },
      { path: '/app/file', label: '文件', icon: '📁' },
      { path: '/app/search', label: '全局搜索', icon: '🔍' },
    ],
  },
  {
    label: '沟通',
    icon: '💬',
    items: [
      { path: '/app/im', label: '消息', icon: '💬' },
      { path: '/app/meeting', label: '会议', icon: '👥' },
      { path: '/app/bbs', label: '论坛', icon: '💭' },
    ],
  },
  {
    label: '协作',
    icon: '📋',
    items: [
      { path: '/app/process', label: '工作流', icon: '📋' },
      { path: '/app/attendance', label: '考勤', icon: '⏰' },
      { path: '/app/note', label: '笔记', icon: '📝' },
    ],
  },
  {
    label: '个人',
    icon: '👤',
    items: [
      { path: '/app/personal', label: '个人中心', icon: '👤' },
      { path: '/app/settings', label: '系统设置', icon: '⚙️' },
      { path: '/app/recycle', label: '回收站', icon: '🗑️' },
    ],
  },
]

const adminExpanded = ref(false)
const adminGroup: NavGroup = {
  label: '系统管理',
  icon: '⚙',
  admin: true,
  items: [
    { path: '/app/admin/document', label: '文档管理', icon: '📄' },
    { path: '/app/admin/program', label: '程序中心', icon: '⚙️' },
    { path: '/app/admin/queryview', label: '查询视图', icon: '📊' },
    { path: '/app/admin/server', label: '服务器', icon: '🖥️' },
    { path: '/app/admin/form', label: '表单管理', icon: '📝' },
    { path: '/app/admin/view', label: '视图管理', icon: '👁️' },
    { path: '/app/admin/fileinfo', label: '文件信息', icon: '📎' },
    { path: '/app/admin/control-panel', label: '控制面板', icon: '🔧' },
    { path: '/app/admin/design-center', label: '设计中心', icon: '🎨' },
    { path: '/app/admin/process-designer', label: '流程设计器', icon: '🧩' },
    { path: '/app/admin/form-designer', label: '表单设计器', icon: '📐' },
    { path: '/app/admin/query', label: '查询管理', icon: '🔎' },
    { path: '/app/admin/portal', label: '门户管理', icon: '🌐' },
    { path: '/app/admin/role', label: '角色管理', icon: '🔑' },
    { path: '/app/admin/cms-module', label: 'CMS模块', icon: '📦' },
    { path: '/app/admin/deployment', label: '部署管理', icon: '🚀' },
    { path: '/app/admin/log-viewer', label: '日志查看', icon: '📜' },
  ],
}

// 移动端导航项（精简）
const mobileNavItems: NavItem[] = [
  { path: '/app/dashboard', label: '首页', icon: '🏠' },
  { path: '/app/process', label: '待办', icon: '📋' },
  { path: '/app/im', label: '消息', icon: '💬' },
  { path: '/app/personal', label: '我的', icon: '👤' },
]

function isActive(path: string): boolean {
  return isNavActive(route.path, path)
}

function navigate(item: NavItem): void {
  if (item.path) router.push(item.path)
  showUserMenu.value = false
}

function toggleSidebar(): void {
  sidebarCollapsed.value = !sidebarCollapsed.value
}

function toggleUserMenu(): void {
  showUserMenu.value = !showUserMenu.value
}

// 仅在焦点位于 user-menu 本身时切换；下拉项（链接/按钮）的 Enter 交给其原生行为，
// 避免键盘打开子菜单时被父级 keydown 冒泡再次关闭。
function toggleUserMenuOnKeydown(e: KeyboardEvent): void {
  if (e.target === e.currentTarget) toggleUserMenu()
}

function focusSearch(): void {
  const input = document.querySelector('input[data-oa-global-search]') as HTMLInputElement
  input?.focus()
}

async function handleLogout(): Promise<void> {
  showUserMenu.value = false
  await session.logout()
  router.replace('/login')
}

function checkMobile(): void {
  isMobile.value = window.innerWidth < 768
}

onMounted(() => {
  checkMobile()
  window.addEventListener('resize', checkMobile)
  document.addEventListener('click', onDocClick)
})

onUnmounted(() => {
  window.removeEventListener('resize', checkMobile)
  document.removeEventListener('click', onDocClick)
})

// 点击外部关闭菜单
function onDocClick(e: Event): void {
  const target = e.target as HTMLElement
  if (!target.closest('.user-menu')) {
    showUserMenu.value = false
  }
}
</script>

<style scoped>
.app-shell {
  display: flex;
  height: 100vh;
  width: 100vw;
  overflow: hidden;
  background: var(--bg-base);
}

/* 侧边栏 */
.sidebar {
  position: fixed;
  top: 0;
  left: 0;
  height: 100vh;
  width: var(--sidebar-width);
  background: var(--bg-surface);
  border-right: 1px solid var(--border-subtle);
  display: flex;
  flex-direction: column;
  z-index: var(--z-dropdown);
  transition: width var(--transition-normal);
}

.sidebar-header {
  height: var(--topbar-height);
  display: flex;
  align-items: center;
  padding: 0 16px;
  border-bottom: 1px solid var(--border-subtle);
}

.logo {
  display: flex;
  align-items: center;
  gap: 10px;
  cursor: pointer;
  padding: 4px 8px;
  border-radius: var(--radius-sm);
  transition: background var(--transition-fast);
}

.logo:hover {
  background: var(--color-primary-soft);
}

.logo-icon {
  font-size: 20px;
  color: var(--color-primary);
  text-shadow: 0 0 10px var(--color-primary-glow);
}

.logo-text {
  font-family: 'Orbitron', sans-serif;
  font-size: 14px;
  font-weight: 600;
  color: var(--color-primary);
  letter-spacing: 2px;
}

.sidebar-nav {
  flex: 1;
  overflow-y: auto;
  overscroll-behavior: contain;
  padding: 8px 0;
}

.nav-section-label {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px 16px 4px;
  margin: 4px 8px;
  font-size: 11px;
  font-weight: 700;
  text-transform: uppercase;
  letter-spacing: 1px;
  color: var(--text-muted);
  border-top: 1px solid var(--border-subtle);
  margin-top: 8px;
}

.nav-group {
  display: block;
}

.nav-section-toggle {
  width: calc(100% - 16px);
  background: transparent;
  border: 0;
  border-top: 1px solid var(--border-subtle);
  cursor: pointer;
  text-align: left;
  font: inherit;
  color: var(--text-muted);
}

.nav-section-toggle:hover {
  color: var(--color-primary);
}

.nav-caret {
  margin-left: auto;
  font-size: 10px;
}

.admin-nav-items {
  padding-left: 0;
}

.nav-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 10px 16px;
  margin: 2px 8px;
  border-radius: var(--radius-md);
  cursor: pointer;
  transition: all var(--transition-fast);
  color: var(--text-secondary);
}

.nav-item:hover {
  background: var(--color-primary-soft);
  color: var(--color-primary);
}

.nav-item.active {
  background: linear-gradient(90deg, var(--color-primary-soft), transparent);
  color: var(--color-primary);
  /* 内嵌阴影替代 border-left：激活态不再改变盒宽，消除导航项跳动 */
  box-shadow: inset 3px 0 0 var(--color-primary);
}

.nav-icon {
  font-size: 18px;
  width: 24px;
  text-align: center;
  flex-shrink: 0;
}

.nav-label {
  font-size: 13px;
  font-weight: 500;
  white-space: nowrap;
  overflow: hidden;
}

.sidebar-footer {
  padding: var(--space-3);
  border-top: 1px solid var(--border-subtle);
}

.theme-toggle {
  width: 100%;
  padding: 8px;
  background: var(--bg-elevated);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-md);
  color: var(--text-secondary);
  cursor: pointer;
  font-size: 16px;
  transition: all var(--transition-fast);
}

.theme-toggle:hover {
  border-color: var(--color-primary);
  color: var(--color-primary);
}

/* 主区域 */
.main-area {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  margin-left: var(--sidebar-width);
  transition: margin-left var(--transition-normal);
}

/* 折叠态仅收窄这一处偏移，杜绝内联样式与静态 margin 的双重右移 */
.app-shell.collapsed .main-area {
  margin-left: var(--sidebar-collapsed);
}

/* 顶栏 */
.topbar {
  height: var(--topbar-height);
  display: flex;
  align-items: center;
  gap: var(--space-4);
  padding: 0 var(--space-5);
  background: var(--bg-glass);
  backdrop-filter: var(--glass-blur);
  -webkit-backdrop-filter: var(--glass-blur);
  border-bottom: 1px solid var(--border-subtle);
  position: sticky;
  top: 0;
  z-index: var(--z-sticky);
}

.topbar-left {
  display: flex;
  align-items: center;
  gap: 12px;
}

.collapse-btn {
  background: none;
  border: none;
  color: var(--text-secondary);
  font-size: 20px;
  cursor: pointer;
  padding: 4px 8px;
  border-radius: var(--radius-sm);
  transition: all var(--transition-fast);
}

.collapse-btn:hover {
  background: var(--color-primary-soft);
  color: var(--color-primary);
}

.breadcrumb {
  font-size: 13px;
  color: var(--text-muted);
}

.topbar-center {
  flex: 1;
  display: flex;
  justify-content: center;
}

.search-box {
  display: flex;
  align-items: center;
  gap: 8px;
  background: var(--bg-elevated);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-lg);
  padding: 6px 12px;
  width: 320px;
  transition: all var(--transition-fast);
}

.search-box:focus-within {
  border-color: var(--color-primary);
  box-shadow: var(--border-glow);
}

.search-icon {
  color: var(--text-muted);
  font-size: 16px;
}

.search-input {
  background: none;
  border: none;
  outline: none;
  color: var(--text-primary);
  font-size: 13px;
  width: 100%;
}

.search-input::placeholder {
  color: var(--text-muted);
}

.topbar-right {
  display: flex;
  align-items: center;
  gap: 8px;
}

.icon-btn {
  position: relative;
  background: none;
  border: none;
  color: var(--text-secondary);
  font-size: 18px;
  cursor: pointer;
  padding: 6px 8px;
  border-radius: var(--radius-sm);
  transition: all var(--transition-fast);
}

.icon-btn:hover {
  background: var(--color-primary-soft);
  color: var(--color-primary);
}

.badge {
  position: absolute;
  top: 2px;
  right: 2px;
  background: var(--color-error);
  color: white;
  font-size: 10px;
  font-weight: 700;
  padding: 1px 5px;
  border-radius: 10px;
  min-width: 16px;
  text-align: center;
  animation: badge-pulse 2s var(--ease-in-out) infinite;
}

@keyframes badge-pulse {
  0%,
  100% {
    box-shadow: 0 0 0 0 var(--color-error-glow, rgba(239, 68, 68, 0.35));
  }
  50% {
    box-shadow: 0 0 0 4px transparent;
  }
}

.user-menu {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 4px 8px;
  border-radius: var(--radius-md);
  cursor: pointer;
  transition: background var(--transition-fast);
  position: relative;
}

.user-menu:hover {
  background: var(--bg-elevated);
}

.user-avatar {
  width: 32px;
  height: 32px;
  border-radius: 50%;
  object-fit: cover;
  border: 2px solid var(--border-subtle);
}

.user-avatar-default {
  background: var(--color-primary-soft);
  color: var(--color-primary);
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: 600;
  font-size: 14px;
}

.user-name {
  font-size: 13px;
  color: var(--text-secondary);
  max-width: 100px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.arrow {
  font-size: 10px;
  color: var(--text-muted);
}

.user-dropdown {
  position: absolute;
  top: 100%;
  right: 0;
  margin-top: 8px;
  min-width: 180px;
  padding: 8px;
  display: flex;
  flex-direction: column;
  gap: 2px;
  z-index: var(--z-dropdown);
  animation: dropdown-in var(--duration-fast) var(--ease-out);
  transform-origin: top right;


  /* R294：玻璃表面，消除展开时与底层内容的透明叠影（t5 在案问题） */
  background: var(--bg-glass);
  backdrop-filter: var(--glass-blur);
  -webkit-backdrop-filter: var(--glass-blur);
  border: 1px solid var(--glass-border);
  border-radius: var(--radius-md);
  box-shadow: var(--shadow-panel);
}

@keyframes dropdown-in {
  from {
    opacity: 0;
    transform: translateY(-4px) scale(0.98);
  }
  to {
    opacity: 1;
    transform: translateY(0) scale(1);
  }
}

.dropdown-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 12px;
  border-radius: var(--radius-sm);
  color: var(--text-secondary);
  text-decoration: none;
  font-size: 13px;
  cursor: pointer;
  border: none;
  background: none;
  width: 100%;
  text-align: left;
  transition: all var(--transition-fast);
}

.dropdown-item:hover {
  background: var(--color-primary-soft);
  color: var(--color-primary);
}

.dropdown-divider {
  height: 1px;
  background: var(--border-subtle);
  margin: 4px 0;
}

/* 内容区 */
.content {
  flex: 1;
  overflow: auto;
  padding: var(--space-5);
  overscroll-behavior: contain;
  background: var(--bg-base);
}

/* 过渡动画 */
.fade-slide-enter-active,
.fade-slide-leave-active {
  transition: opacity var(--transition-normal), transform var(--transition-normal);
}

.fade-slide-enter-from {
  opacity: 0;
  transform: translateX(10px);
}

.fade-slide-leave-to {
  opacity: 0;
  transform: translateX(-10px);
}

/* 键盘焦点可见性：统一令牌化焦点环（WCAG 2.4.7） */
.logo:focus-visible,
.nav-item:focus-visible,
.theme-toggle:focus-visible,
.collapse-btn:focus-visible,
.icon-btn:focus-visible,
.user-menu:focus-visible,
.tab-item:focus-visible,
.dropdown-item:focus-visible {
  outline: 2px solid var(--color-primary);
  outline-offset: 2px;
}

/* 移动端底部 Tab 栏 */
.tab-bar {
  display: none;
  position: fixed;
  bottom: 0;
  left: 0;
  right: 0;
  height: var(--tabbar-height);
  background: var(--bg-glass);
  backdrop-filter: var(--glass-blur);
  border-top: 1px solid var(--border-subtle);
  z-index: var(--z-dropdown);
  padding: 0 var(--space-2);
}

.tab-item {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 2px;
  padding: 4px;
  border-radius: var(--radius-sm);
  cursor: pointer;
  color: var(--text-muted);
  transition: all var(--transition-fast);
}

.tab-item:hover,
.tab-item.active {
  color: var(--color-primary);
  background: var(--color-primary-soft);
}

.tab-icon {
  font-size: 20px;
}

.tab-label {
  font-size: 10px;
}

/* 移动端适配 */
@media (max-width: 768px) {
  .sidebar {
    transform: translateX(-100%);
    width: 280px !important;
  }

  .app-shell.mobile .sidebar {
    transform: translateX(0);
  }

  .main-area {
    margin-left: 0 !important;
  }

  /* 移动端侧栏仍以固定宽度可见：topbar/content 沿用与桌面同源的偏移令牌 */
  .topbar,
  .content {
    margin-left: var(--sidebar-width);
  }

  .content {
    padding: var(--space-3);
    padding-bottom: calc(var(--tabbar-height) + var(--space-3));
  }

  .topbar-center {
    display: none;
  }

  .tab-bar {
    display: flex;
  }

  .user-name {
    display: none;
  }
}
</style>
