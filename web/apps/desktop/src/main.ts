import { createApp, h } from 'vue';
import { createPinia } from 'pinia';
import { createRouter, createWebHistory } from 'vue-router';
import { QueryClient, VueQueryPlugin } from '@tanstack/vue-query';
import { createI18n } from 'vue-i18n';
import { createThemeProvider, useSession } from '@oa4rust/sdk';
import { NConfigProvider } from 'naive-ui';
import AppShell from '@oa4rust/ui/components/AppShell.vue';
import LoginScreen from '@oa4rust/ui/components/LoginScreen.vue';
import OAuthCallback from '@oa4rust/ui/views/OAuthCallback.vue';
import Dashboard from './views/Dashboard.vue';
import OrgViewer from './views/OrgViewer.vue';
import ProcessWork from './views/ProcessWork.vue';
import IMChat from './views/IMChat.vue';
import Personal from './views/Personal.vue';
import Settings from './views/Settings.vue';
import SSO from './views/SSO.vue';
import CalendarApp from './views/CalendarApp.vue';
import FileManager from './views/FileManager.vue';
import BBSForum from './views/BBSForum.vue';

const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: '/login', name: 'Login', component: LoginScreen, meta: { requiresAuth: false } },
    {
      path: '/app/:appId', name: 'AppShell', component: AppShell, meta: { requiresAuth: true },
      children: [
        { path: '', redirect: '/app/dashboard' },
        { path: 'dashboard', name: 'Dashboard', component: Dashboard, meta: { title: '工作台' } },
        { path: 'org', name: 'OrgViewer', component: OrgViewer, meta: { title: '组织架构' } },
        { path: 'process', name: 'ProcessWork', component: ProcessWork, meta: { title: '工作流' } },
        { path: 'im', name: 'IMChat', component: IMChat, meta: { title: '即时通讯' } },
        { path: 'personal', name: 'Personal', component: Personal, meta: { title: '个人中心' } },
        { path: 'settings', name: 'Settings', component: Settings, meta: { title: '系统设置' } },
        { path: 'calendar', name: 'CalendarApp', component: CalendarApp, meta: { title: '日历' } },
        { path: 'file', name: 'FileManager', component: FileManager, meta: { title: '文件管理' } },
        { path: 'bbs', name: 'BBSForum', component: BBSForum, meta: { title: '论坛' } },
      ],
    },
    { path: '/oauth/callback/:platform', name: 'OAuthCallback', component: OAuthCallback, meta: { requiresAuth: false } },
    { path: '/sso', name: 'SSO', component: SSO, meta: { requiresAuth: false } },
    { path: '/:pathMatch(.*)*', redirect: '/login' },
  ],
});

router.beforeEach(async (to) => {
  if (to.meta.requiresAuth !== false) {
    const session = useSession();
    await session.init();
    if (!session.isAuthenticated && to.name !== 'Login' && !to.path.startsWith('/oauth')) {
      return { name: 'Login', query: { redirect: to.fullPath } };
    }
  }
});

const i18n = createI18n({ legacy: false, locale: 'zh-cn', fallbackLocale: 'en', messages: {
  'zh-cn': { common: { login: '登录', logout: '退出', confirm: '确认', cancel: '取消', search: '搜索', loading: '加载中...' } },
  en: { common: { login: 'Login', logout: 'Logout', confirm: 'Confirm', cancel: 'Cancel', search: 'Search', loading: 'Loading...' } },
}});

const queryClient = new QueryClient({
  defaultOptions: { queries: { staleTime: 5 * 60 * 1000, retry: 2, refetchOnWindowFocus: false }, mutations: { retry: 1 } },
});

const themeProvider = createThemeProvider();
themeProvider.init();

const app = createApp({ template: '<n-config-provider><router-view /></n-config-provider>', components: { NConfigProvider } });
app.use(createPinia());
app.use(router);
app.use(i18n);
app.use(VueQueryPlugin, { queryClient });
app.mount('#o2-app-root');

import { registerSW } from './registerSW';
registerSW();
console.log('[OA4Rust] App initialized');
