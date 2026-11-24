<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<template>
  <div class="app-shell" :class="{ 'collapsed': sidebarCollapsed, 'mobile': isMobile, 'drawer-open': mobileDrawerOpen }">
    <!-- 侧边栏 -->
    <aside class="sidebar" :style="{ width: sidebarCollapsed ? 'var(--sidebar-collapsed)' : 'var(--sidebar-width)' }">
      <div class="sidebar-header">
        <div class="logo" role="button" tabindex="0" aria-label="切换侧边栏" @click="toggleSidebar" @keydown.enter="toggleSidebar" @keydown.space.prevent="toggleSidebar">
          <span class="logo-icon">◆</span>
          <span v-show="!sidebarCollapsed" class="logo-text ai-gradient-text">OA4RUST</span>
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

        <!-- R298: 系统管理收敛为单一入口，分组导航移入 AdminLayout（/app/admin/*） -->
        <div class="nav-group">
          <div class="nav-section-label">{{ adminGroup.label }}</div>
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
      </nav>

      <div class="sidebar-footer">
        <button class="theme-toggle" aria-label="切换主题" title="切换主题" @click="toggleTheme">
          <span class="theme-toggle-icon">{{ theme.theme === 'dark' ? '☀' : '☾' }}</span>
          <span v-show="!sidebarCollapsed" class="theme-toggle-label">{{ theme.theme === 'dark' ? '明亮模式' : '深色模式' }}</span>
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
          <nav v-if="!isAdminArea" class="breadcrumb" aria-label="面包屑">
            <span class="breadcrumb-trail">
              <span class="breadcrumb-root">工作台</span>
              <span v-if="currentGroup" class="breadcrumb-separator">/</span>
              <span v-if="currentGroup" class="breadcrumb-group">{{ currentGroup }}</span>
              <span class="breadcrumb-separator">/</span>
              <span class="breadcrumb-current" :aria-current="isAdminArea ? undefined : 'page'">{{ currentTitle }}</span>
            </span>
          </nav>
        </div>

        <div class="topbar-center">
          <div class="search-box">
            <span class="search-icon">⌕</span>
            <input
              v-model="searchQuery"
              placeholder="搜索页面或内容 (⌘K)"
              aria-label="全局搜索"
              data-oa-global-search
              class="search-input"
              @keydown.ctrl.k.prevent="openCommandPalette"
              @keydown.meta.k.prevent="openCommandPalette"
              @keydown.enter.prevent="submitGlobalSearch"
            />
            <button
              type="button"
              class="command-palette-trigger"
              data-testid="command-palette-trigger"
              aria-label="打开命令面板"
              title="命令面板 (⌘K)"
              @click="openCommandPalette"
            >
              ⌘K
            </button>
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

      <!-- R300 B1：活动页签，保留所有原生 router-link 导航语义 -->
      <nav v-if="openTabs.length > 0" class="workspace-tabs" aria-label="已打开的页签" data-testid="workspace-tabs">
        <div v-for="tab in openTabs" :key="tab.path" class="workspace-tab" :class="{ active: tab.path === route.path }">
          <router-link :to="tab.path" class="workspace-tab-link" :aria-current="tab.path === route.path ? 'page' : undefined">
            <span class="workspace-tab-icon">{{ tab.icon }}</span>
            <span class="workspace-tab-label">{{ tab.label }}</span>
          </router-link>
          <button type="button" class="workspace-tab-close" :aria-label="'关闭页签：' + tab.label" @click.prevent.stop="closeTab(tab.path)">×</button>
        </div>
      </nav>

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

    <!-- R300 B1：Ctrl/⌘K 命令面板，键盘可达、Escape 关闭；导航目标全部来自现有注册路由 -->
    <div v-if="showCommandPalette" class="command-palette-backdrop" data-testid="command-palette-backdrop">
      <section
        ref="commandPaletteRoot"
        class="command-palette ai-panel"
        role="dialog"
        aria-modal="true"
        tabindex="-1"
        aria-label="命令面板"
        data-testid="command-palette"
        @keydown.esc.prevent="closeCommandPalette"
        @keydown.down.prevent="moveCommandSelection(1)"
        @keydown.up.prevent="moveCommandSelection(-1)"
        @keydown.enter.prevent="runCommandSelection"
      >
        <div class="command-palette-header">
          <span class="command-palette-glyph">⌘</span>
          <span class="command-palette-title">快速导航</span>
          <span class="command-palette-hint">↑↓ 选择 · Enter 打开 · Esc 关闭</span>
        </div>
        <div class="command-palette-search">
          <input
            v-model="commandQuery"
            data-oa-command-search
            class="command-palette-input"
            aria-label="命令面板搜索"
            placeholder="输入页面名称或关键词…"
            @input="watchCommandQuery"
          />
        </div>
        <div class="command-palette-list" role="listbox" aria-label="可打开的页面">
          <button
            v-for="(cmd, index) in filteredCommands"
            :key="cmd.path"
            type="button"
            class="command-item"
            :class="{ active: index === commandSelection }"
            role="option"
            :aria-selected="index === commandSelection"
            @mouseenter="commandSelection = index"
            @click="runCommand(cmd)"
          >
            <span class="command-item-icon">{{ cmd.icon }}</span>
            <span class="command-item-label">{{ cmd.label }}</span>
            <span class="command-item-group">{{ cmd.group }}</span>
          </button>
          <div v-if="filteredCommands.length === 0" class="command-empty">没有匹配的页面，回车可搜索内容</div>
        </div>
        <div class="command-palette-footer">
          <span>{{ filteredCommands.length }} 个页面</span>
          <span>输入关键词过滤 · 回车交给全局搜索</span>
        </div>
      </section>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useSession, useTheme } from '@oa4rust/sdk'
import { computed, nextTick, onMounted, onUnmounted, ref, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { isNavActive } from './navActive'
import { installFocusTrap, type FocusTrapHandle } from './focusTrap'

const router = useRouter()
const route = useRoute()
const session = useSession()
const { theme, toggleTheme } = useTheme()

const sidebarCollapsed = ref(false)
const showUserMenu = ref(false)
const searchQuery = ref('')
const unreadCount = ref(0)
const isMobile = ref(false)
const mobileDrawerOpen = ref(false)
const openTabPaths = ref<string[]>([])
const showCommandPalette = ref(false)
const commandPaletteRoot = ref<HTMLElement | null>(null)
let commandFocusTrap: FocusTrapHandle | null = null
let commandTrigger: HTMLElement | null = null
const commandQuery = ref('')
const commandSelection = ref(0)

const currentTitle = computed(() => {
  return (route.meta.title as string) || 'OA4Rust'
})

const currentGroup = computed(() => {
  const allGroups = [...navGroups, adminGroup]
  return allGroups.find((group) => group.items.some((item) => item.path === route.path))?.label ?? ''
})

const allNavItems = computed(() => [...navGroups.flatMap((group) => group.items), ...adminGroup.items])

const openTabs = computed(() => {
  return openTabPaths.value
    .map((path) => allNavItems.value.find((item) => item.path === path))
    .filter((item): item is NavItem => Boolean(item))
})

const filteredCommands = computed(() => {
  const keyword = commandQuery.value.trim().toLowerCase()
  const items = allNavItems.value.map((item) => ({
    ...item,
    group: [...navGroups, adminGroup].find((group) => group.items.some((candidate) => candidate.path === item.path))?.label ?? '',
  }))
  if (!keyword) return items
  return items.filter((item) => item.label.toLowerCase().includes(keyword) || item.path.toLowerCase().includes(keyword) || item.group.toLowerCase().includes(keyword))
})

// R302：管理区（/app/admin/*）页名由 AdminLayout h1 单独承载，topbar 不再重复显示。
const isAdminArea = computed(() => route.path.startsWith('/app/admin'))

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

// R298: 管理区入口收敛为单项；六组分组菜单由 AdminLayout 承载（main.ts 嵌套路由）。
const adminGroup: NavGroup = {
  label: '系统管理',
  icon: '⚙',
  admin: true,
  items: [{ path: '/app/admin/control-panel', label: '系统管理', icon: '⚙' }],
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
  if (isMobile.value) {
    mobileDrawerOpen.value = !mobileDrawerOpen.value
    return
  }
  sidebarCollapsed.value = !sidebarCollapsed.value
}

function syncOpenTabs(): void {
  if (route.path === '/app/search' || route.path.startsWith('/app/admin/')) return
  if (!openTabPaths.value.includes(route.path)) openTabPaths.value.push(route.path)
}

function closeTab(path: string): void {
  openTabPaths.value = openTabPaths.value.filter((item) => item !== path)
  if (route.path === path) router.push(openTabPaths.value[0] ?? '/app/dashboard')
}

function openCommandPalette(): void {
  const active = document.activeElement as HTMLElement | null
  commandTrigger = active?.closest?.('[data-testid="command-palette"]') ? null : active
  showCommandPalette.value = true
  commandQuery.value = ''
  commandSelection.value = 0
  searchQuery.value = ''
  void nextTick(() => {
    if (commandPaletteRoot.value) {
      commandFocusTrap = installFocusTrap(commandPaletteRoot.value, commandTrigger)
    }
  })
}

function closeCommandPalette(): void {
  commandFocusTrap?.release()
  commandFocusTrap = null
  showCommandPalette.value = false
}

function moveCommandSelection(delta: number): void {
  const total = filteredCommands.value.length
  if (total === 0) return
  commandSelection.value = (commandSelection.value + delta + total) % total
}

function runCommand(cmd?: { path: string }): void {
  const target = cmd ?? filteredCommands.value[commandSelection.value]
  closeCommandPalette()
  if (target?.path) router.push(target.path)
}

function runCommandSelection(): void {
  if (filteredCommands.value.length > 0) runCommand()
  else submitGlobalSearch()
}

function submitGlobalSearch(): void {
  const query = searchQuery.value.trim() || commandQuery.value.trim()
  closeCommandPalette()
  if (query) router.push({ path: '/app/search', query: { q: query } })
  else router.push('/app/search')
}

function watchCommandQuery(): void {
  commandSelection.value = 0
}

function toggleUserMenu(): void {
  showUserMenu.value = !showUserMenu.value
}

// 仅在焦点位于 user-menu 本身时切换；下拉项（链接/按钮）的 Enter 交给其原生行为，
// 避免键盘打开子菜单时被父级 keydown 冒泡再次关闭。
function toggleUserMenuOnKeydown(e: KeyboardEvent): void {
  if (e.target === e.currentTarget) toggleUserMenu()
}

async function handleLogout(): Promise<void> {
  showUserMenu.value = false
  await session.logout()
  router.replace('/login')
}

function checkMobile(): void {
  isMobile.value = window.innerWidth < 768
}

watch(() => route.path, syncOpenTabs)

onMounted(() => {
  checkMobile()
  syncOpenTabs()
  window.addEventListener('resize', checkMobile)
  document.addEventListener('click', onDocClick)
})

onUnmounted(() => {
  commandFocusTrap?.release()
  commandFocusTrap = null
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
  padding: 0 var(--space-4);
  border-bottom: 1px solid var(--border-subtle);
}

.logo {
  display: flex;
  align-items: center;
  gap: 10px;
  cursor: pointer;
  padding: var(--space-1) var(--space-2);
  border-radius: var(--radius-sm);
  transition: background var(--transition-fast);
}

.logo:hover {
  background: var(--color-primary-soft);
}

.logo-icon {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 28px;
  height: 28px;
  font-size: 16px;
  color: var(--ai-cyan);
  background: var(--ai-gradient-brand);
  border-radius: var(--radius-sm);
  box-shadow: var(--ai-shadow-1);
}

.logo-text {
  font-family: 'Orbitron', sans-serif;
  font-size: 14px;
  font-weight: 600;
  letter-spacing: 2px;
}

.sidebar-nav {
  flex: 1;
  overflow-y: auto;
  overscroll-behavior: contain;
  padding: var(--space-2) 0;
}

.nav-section-label {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px var(--space-4) 4px;
  margin: 4px var(--space-2);
  font-size: 11px;
  font-weight: 700;
  text-transform: uppercase;
  letter-spacing: 1px;
  color: var(--text-muted);
  border-top: 1px solid var(--border-subtle);
  margin-top: var(--space-2);
}

.nav-group {
  display: block;
}

.nav-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 10px var(--space-4);
  margin: 2px var(--space-2);
  border-radius: var(--radius-md);
  cursor: pointer;
  transition: all var(--ai-motion-fast) var(--ai-ease-spring);
  color: var(--text-secondary);
}

.nav-item:hover {
  background: var(--color-primary-soft);
  color: var(--color-primary);
}

.nav-item.active {
  background: var(--ai-gradient-surface);
  color: var(--color-primary);
  /* 内嵌阴影替代 border-left：激活态不再改变盒宽，消除导航项跳动 */
  box-shadow: inset 3px 0 0 var(--ai-indigo), var(--ai-shadow-1);
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
  display: flex;
  align-items: center;
  justify-content: center;
  gap: var(--space-2);
  width: 100%;
  padding: var(--space-2);
  background: var(--ai-glass-bg);
  border: 1px solid var(--ai-glass-border);
  border-radius: var(--radius-md);
  color: var(--text-secondary);
  cursor: pointer;
  font-size: 13px;
  transition: all var(--ai-motion-fast) var(--ai-ease-spring);
}

.theme-toggle:hover {
  border-color: var(--ai-indigo);
  color: var(--ai-cyan);
}

.theme-toggle-icon {
  font-size: 16px;
  color: var(--ai-cyan);
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
  background: var(--ai-glass-bg);
  backdrop-filter: var(--ai-glass-blur);
  -webkit-backdrop-filter: var(--ai-glass-blur);
  border-bottom: 1px solid var(--ai-glass-border);
  position: sticky;
  top: 0;
  z-index: var(--z-sticky);
}

.topbar-left {
  display: flex;
  align-items: center;
  gap: 12px;
  min-width: 0;
}

.collapse-btn {
  background: none;
  border: none;
  color: var(--text-secondary);
  font-size: 20px;
  cursor: pointer;
  padding: var(--space-1) var(--space-2);
  border-radius: var(--radius-sm);
  transition: all var(--ai-motion-fast) var(--ai-ease-spring);
}

.collapse-btn:hover {
  background: var(--color-primary-soft);
  color: var(--color-primary);
}

.breadcrumb {
  min-width: 0;
  font-size: 13px;
  color: var(--text-muted);
}

.breadcrumb-trail {
  display: inline-flex;
  align-items: center;
  gap: var(--space-2);
  min-width: 0;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.breadcrumb-separator {
  color: var(--text-muted);
  opacity: 0.6;
}

.breadcrumb-current {
  color: var(--text-primary);
  font-weight: 600;
}

.topbar-center {
  flex: 1;
  display: flex;
  justify-content: center;
}

.search-box {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  background: var(--ai-glass-bg);
  border: 1px solid var(--ai-glass-border);
  border-radius: var(--radius-lg);
  padding: var(--space-1) var(--space-3);
  width: 320px;
  transition: all var(--ai-motion-fast) var(--ai-ease-spring);
}

.search-box:focus-within {
  border-color: var(--ai-indigo);
  box-shadow: var(--ai-shadow-2);
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

.command-palette-trigger {
  flex-shrink: 0;
  padding: 2px 6px;
  border: 1px solid var(--ai-glass-border);
  border-radius: var(--radius-sm);
  background: var(--bg-elevated);
  color: var(--text-muted);
  font-size: 10px;
  cursor: pointer;
}

.topbar-right {
  display: flex;
  align-items: center;
  gap: var(--space-2);
}

.icon-btn {
  position: relative;
  background: none;
  border: none;
  color: var(--text-secondary);
  font-size: 18px;
  cursor: pointer;
  padding: var(--space-1) var(--space-2);
  border-radius: var(--radius-sm);
  transition: all var(--ai-motion-fast) var(--ai-ease-spring);
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
  gap: var(--space-2);
  padding: var(--space-1) var(--space-2);
  border-radius: var(--radius-md);
  cursor: pointer;
  transition: background var(--ai-motion-fast) var(--ai-ease-spring);
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
  border: 2px solid var(--ai-glass-border);
}

.user-avatar-default {
  background: var(--ai-gradient-brand);
  color: var(--ai-cyan);
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
  margin-top: var(--space-2);
  min-width: 180px;
  padding: var(--space-2);
  display: flex;
  flex-direction: column;
  gap: 2px;
  z-index: var(--z-dropdown);
  animation: dropdown-in var(--duration-fast) var(--ease-out);
  transform-origin: top right;

  /* R294：玻璃表面，消除展开时与底层内容的透明叠影（t5 在案问题） */
  background: var(--ai-glass-bg);
  backdrop-filter: var(--ai-glass-blur);
  -webkit-backdrop-filter: var(--ai-glass-blur);
  border: 1px solid var(--ai-glass-border);
  border-radius: var(--radius-md);
  box-shadow: var(--ai-shadow-2);
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
  gap: var(--space-2);
  padding: var(--space-2) var(--space-3);
  border-radius: var(--radius-sm);
  color: var(--text-secondary);
  text-decoration: none;
  font-size: 13px;
  cursor: pointer;
  border: none;
  background: none;
  width: 100%;
  text-align: left;
  transition: all var(--ai-motion-fast) var(--ai-ease-spring);
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

/* R300 B1：活动页签 */
.workspace-tabs {
  display: flex;
  align-items: center;
  gap: var(--space-1);
  min-height: 34px;
  padding: var(--space-1) var(--space-5);
  overflow-x: auto;
  border-bottom: 1px solid var(--border-subtle);
  background: var(--bg-surface);
}

.workspace-tab {
  display: inline-flex;
  align-items: center;
  flex: 0 0 auto;
  border-radius: var(--radius-sm);
  transition: background var(--ai-motion-fast) var(--ai-ease-spring);
}

.workspace-tab.active {
  background: var(--ai-gradient-surface);
  box-shadow: inset 0 -2px 0 var(--ai-indigo);
}

.workspace-tab-link {
  display: inline-flex;
  align-items: center;
  gap: var(--space-1);
  padding: var(--space-1) var(--space-2);
  color: var(--text-secondary);
  font-size: 12px;
  text-decoration: none;
}

.workspace-tab.active .workspace-tab-link {
  color: var(--color-primary);
  font-weight: 600;
}

.workspace-tab-close {
  border: none;
  background: none;
  color: var(--text-muted);
  font-size: 14px;
  line-height: 1;
  cursor: pointer;
  padding: var(--space-1);
}

/* R300 B1：命令面板 */
.command-palette-backdrop {
  position: fixed;
  inset: 0;
  z-index: var(--z-modal);
  display: flex;
  align-items: flex-start;
  justify-content: center;
  padding-top: 12vh;
  background: var(--ai-cmd-backdrop);
  backdrop-filter: var(--ai-glass-blur);
  -webkit-backdrop-filter: var(--ai-glass-blur);
}

.command-palette {
  width: min(calc(100vw - var(--space-8)), var(--ai-cmd-width));
  max-height: 70vh;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  border: 1px solid var(--ai-glass-border);
  border-radius: var(--ai-cmd-radius);
  background: var(--bg-elevated);
  box-shadow: var(--ai-shadow-3);
  animation: command-in var(--ai-motion-fast) var(--ai-ease-spring);
}

@keyframes command-in {
  from { opacity: 0; transform: translateY(-8px) scale(0.98); }
  to { opacity: 1; transform: translateY(0) scale(1); }
}

.command-palette-header {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  padding: var(--space-3) var(--space-4);
  border-bottom: 1px solid var(--border-subtle);
}

.command-palette-glyph {
  color: var(--ai-cyan);
  font-size: 18px;
}

.command-palette-title {
  color: var(--text-primary);
  font-weight: 600;
}

.command-palette-hint {
  margin-left: auto;
  color: var(--text-muted);
  font-size: 11px;
}

.command-palette-search {
  padding: var(--space-3) var(--space-4) var(--space-1);
}

.command-palette-input {
  width: 100%;
  padding: var(--space-2) var(--space-3);
  border: 1px solid var(--ai-glass-border);
  border-radius: var(--radius-sm);
  background: var(--ai-glass-bg);
  color: var(--text-primary);
  outline: none;
}

.command-palette-input:focus {
  border-color: var(--ai-indigo);
  box-shadow: var(--ai-shadow-1);
}

.command-palette-list {
  flex: 1;
  overflow-y: auto;
  padding: var(--space-1);
}

.command-item {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  width: 100%;
  min-height: var(--ai-cmd-item-height);
  padding: 0 var(--space-3);
  border: none;
  border-radius: var(--radius-sm);
  background: none;
  color: var(--text-secondary);
  cursor: pointer;
  text-align: left;
}

.command-item.active {
  background: var(--ai-gradient-surface);
  color: var(--text-primary);
}

.command-item-icon {
  width: 24px;
  text-align: center;
}

.command-item-group {
  margin-left: auto;
  color: var(--text-muted);
  font-size: 11px;
}

.command-empty {
  padding: var(--space-5);
  color: var(--text-muted);
  text-align: center;
}

.command-palette-footer {
  display: flex;
  justify-content: space-between;
  padding: var(--space-2) var(--space-4);
  border-top: 1px solid var(--border-subtle);
  color: var(--text-muted);
  font-size: 11px;
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
.dropdown-item:focus-visible,
.command-palette-trigger:focus-visible,
.workspace-tab-close:focus-visible,
.command-item:focus-visible,
.command-palette-input:focus-visible {
  outline: 2px solid var(--ai-indigo);
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
  background: var(--ai-glass-bg);
  backdrop-filter: var(--ai-glass-blur);
  border-top: 1px solid var(--ai-glass-border);
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
  padding: var(--space-1);
  border-radius: var(--radius-sm);
  cursor: pointer;
  color: var(--text-muted);
  transition: all var(--ai-motion-fast) var(--ai-ease-spring);
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

  .app-shell.mobile.drawer-open .sidebar {
    transform: translateX(0);
  }

  .main-area {
    margin-left: 0 !important;
  }

  /* R469 移动端侧栏改为抽屉态：默认隐藏、内容回流占满，不保留侧栏占位偏移 */
  .topbar,
  .content {
    margin-left: 0;
  }

  .content {
    padding: var(--space-3);
    padding-bottom: calc(var(--tabbar-height) + var(--space-3));
  }

  .topbar-center {
    display: none;
  }

  .breadcrumb-trail {
    max-width: 45vw;
  }

  .workspace-tabs {
    padding: var(--space-1) var(--space-3);
  }

  .tab-bar {
    display: flex;
  }

  .user-name {
    display: none;
  }
}
</style>
