<template>
  <div class="app-shell" :class="{ 'collapsed': sidebarCollapsed, 'mobile': isMobile }">
    <!-- 侧边栏 -->
    <aside class="sidebar" :style="{ width: sidebarCollapsed ? '64px' : '240px' }">
      <div class="sidebar-header">
        <div class="logo" @click="sidebarCollapsed = !sidebarCollapsed">
          <span class="logo-icon">◆</span>
          <span v-show="!sidebarCollapsed" class="logo-text">OA4RUST</span>
        </div>
      </div>

      <nav class="sidebar-nav">
        <div
          v-for="item in navItems"
          :key="item.path"
          class="nav-item"
          :class="{ active: isActive(item.path) }"
          @click="navigate(item)"
        >
          <span class="nav-icon">{{ item.icon }}</span>
          <span v-show="!sidebarCollapsed" class="nav-label">{{ item.label }}</span>
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
      <header class="topbar" :style="{ marginLeft: sidebarCollapsed ? '64px' : '240px' }">
        <div class="topbar-left">
          <button class="collapse-btn" @click="sidebarCollapsed = !sidebarCollapsed">
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
          <div class="user-menu" @click="showUserMenu = !showUserMenu">
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
      <main class="content" :style="{ marginLeft: sidebarCollapsed ? '64px' : '240px' }">
        <router-view v-slot="{ Component }">
          <transition name="fade-slide" mode="out-in">
            <component :is="Component" />
          </transition>
        </router-view>
      </main>
    </div>

    <!-- 移动端底部导航 -->
    <nav v-if="isMobile" class="tab-bar">
      <div
        v-for="item in mobileNavItems"
        :key="item.path"
        class="tab-item"
        :class="{ active: isActive(item.path) }"
        @click="navigate(item)"
      >
        <span class="tab-icon">{{ item.icon }}</span>
        <span class="tab-label">{{ item.label }}</span>
      </div>
    </nav>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue';
import { useRouter, useRoute } from 'vue-router';
import { useSession } from '@oa4rust/sdk';
import { useTheme } from '@oa4rust/sdk';

const router = useRouter();
const route = useRoute();
const session = useSession();
const { theme, toggleTheme } = useTheme();

const sidebarCollapsed = ref(false);
const showUserMenu = ref(false);
const searchQuery = ref('');
const unreadCount = ref(0);
const isMobile = ref(false);

const currentTitle = computed(() => {
  return route.meta.title as string || 'OA4Rust';
});

const user = computed(() => session.state.value?.user ?? null);

// PC 端导航项
const navItems = [
  { path: '/app/dashboard', label: '工作台', icon: '🏠' },
  { path: '/app/org', label: '组织', icon: '🏢' },
  { path: '/app/process', label: '工作流', icon: '📋' },
  { path: '/app/im', label: '消息', icon: '💬' },
  { path: '/app/calendar', label: '日历', icon: '📅' },
  { path: '/app/meeting', label: '会议', icon: '👥' },
  { path: '/app/file', label: '文件', icon: '📁' },
  { path: '/app/bbs', label: '论坛', icon: '💭' },
  { path: '/app/admin', label: '管理', icon: '🔧' },
];

// 移动端导航项（精简）
const mobileNavItems = [
  { path: '/app/dashboard', label: '首页', icon: '🏠' },
  { path: '/app/process', label: '待办', icon: '📋' },
  { path: '/app/im', label: '消息', icon: '💬' },
  { path: '/app/personal', label: '我的', icon: '👤' },
];

function isActive(path: string): boolean {
  return route.path.startsWith(path);
}

function navigate(item: { path: string; label: string; icon: string }): void {
  router.push(item.path);
  showUserMenu.value = false;
}

function focusSearch(): void {
  const input = document.querySelector('.search-input') as HTMLInputElement;
  input?.focus();
}

async function handleLogout(): void {
  showUserMenu.value = false;
  await session.logout();
  router.replace('/login');
}

function checkMobile(): void {
  isMobile.value = window.innerWidth < 768;
}

onMounted(() => {
  checkMobile();
  window.addEventListener('resize', checkMobile);
});

onUnmounted(() => {
  window.removeEventListener('resize', checkMobile);
});

// 点击外部关闭菜单
document.addEventListener('click', (e) => {
  const target = e.target as HTMLElement;
  if (!target.closest('.user-menu')) {
    showUserMenu.value = false;
  }
});
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
  width: 240px;
  background: var(--bg-surface);
  border-right: 1px solid var(--border-subtle);
  display: flex;
  flex-direction: column;
  z-index: 100;
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
  padding: 8px 0;
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
  background: var(--color-primary-soft);
  color: var(--color-primary);
  border-left: 3px solid var(--color-primary);
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
  padding: 12px;
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
  margin-left: 240px;
  transition: margin-left var(--transition-normal);
}

/* 顶栏 */
.topbar {
  height: var(--topbar-height);
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 0 20px;
  background: var(--bg-glass);
  backdrop-filter: var(--glass-blur);
  -webkit-backdrop-filter: var(--glass-blur);
  border-bottom: 1px solid var(--border-subtle);
  position: sticky;
  top: 0;
  z-index: 50;
  transition: margin-left var(--transition-normal);
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
  z-index: 200;
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
  padding: 20px;
  margin-left: 0;
  transition: margin-left var(--transition-normal);
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
  z-index: 100;
  padding: 0 8px;
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

  .content {
    padding: 12px;
    padding-bottom: calc(var(--tabbar-height) + 12px);
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
