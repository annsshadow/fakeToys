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
import MeetingApp from './views/MeetingApp.vue';
import AttendanceApp from './views/AttendanceApp.vue';
import QueryManager from './views/QueryManager.vue';
import AIAssistant from './views/AIAssistant.vue';
import PortalApp from './views/PortalApp.vue';
import HotpicApp from './views/HotpicApp.vue';
import JPushApp from './views/JPushApp.vue';
import AppInfoApp from './views/AppInfoApp.vue';
import CategoryApp from './views/CategoryApp.vue';
import CalendarApp from './views/CalendarApp.vue';
import FileManager from './views/FileManager.vue';
import BBSForum from './views/BBSForum.vue';
import MindApp from './views/MindApp.vue';
import DocumentApp from './views/DocumentApp.vue';
import ProgramCenterApp from './views/ProgramCenterApp.vue';
import QueryViewApp from './views/QueryViewApp.vue';
import RecycleApp from './views/RecycleApp.vue';
import ServerApp from './views/ServerApp.vue';
import UnitApp from './views/UnitApp.vue';
import FormApp from './views/FormApp.vue';
import ViewApp from './views/ViewApp.vue';
import FileInfoApp from './views/FileInfoApp.vue';

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
        { path: 'meeting', name: 'MeetingApp', component: MeetingApp, meta: { title: '会议管理' } },
        { path: 'attendance', name: 'AttendanceApp', component: AttendanceApp, meta: { title: '考勤管理' } },
        { path: 'query', name: 'QueryManager', component: QueryManager, meta: { title: '查询管理' } },
        { path: 'portal', name: 'PortalApp', component: PortalApp, meta: { title: '门户管理' } },
        { path: 'hotpic', name: 'HotpicApp', component: HotpicApp, meta: { title: '热帖管理' } },
        { path: 'jpush', name: 'JPushApp', component: JPushApp, meta: { title: '消息推送' } },
        { path: 'appinfo', name: 'AppInfoApp', component: AppInfoApp, meta: { title: '应用管理' } },
        { path: 'category', name: 'CategoryApp', component: CategoryApp, meta: { title: '分类管理' } },
        { path: 'mind', name: 'MindApp', component: MindApp, meta: { title: '思维导图' } },
        { path: 'document', name: 'DocumentApp', component: DocumentApp, meta: { title: '文档管理' } },
        { path: 'program', name: 'ProgramCenterApp', component: ProgramCenterApp, meta: { title: '程序中心' } },
        { path: 'queryview', name: 'QueryViewApp', component: QueryViewApp, meta: { title: '查询视图' } },
        { path: 'recycle', name: 'RecycleApp', component: RecycleApp, meta: { title: '回收站' } },
        { path: 'server', name: 'ServerApp', component: ServerApp, meta: { title: '服务器管理' } },
        { path: 'unit', name: 'UnitApp', component: UnitApp, meta: { title: '单元管理' } },
        { path: 'form', name: 'FormApp', component: FormApp, meta: { title: '表单管理' } },
        { path: 'view', name: 'ViewApp', component: ViewApp, meta: { title: '视图管理' } },
        { path: 'fileinfo', name: 'FileInfoApp', component: FileInfoApp, meta: { title: '文件信息' } },
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
