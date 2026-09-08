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


const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: '/login', name: 'Login', component: LoginScreen, meta: { requiresAuth: false } },
    {
      path: '/app/:appId', name: 'AppShell', component: AppShell, meta: { requiresAuth: true },
      children: [
        { path: '', redirect: '/app/dashboard' },
        { path: 'dashboard', name: 'Dashboard', component: () => import('./views/Dashboard.vue'), meta: { title: '工作台' } },
        { path: 'org', name: 'OrgViewer', component: () => import('./views/OrgViewer.vue'), meta: { title: '组织架构' } },
        { path: 'process', name: 'ProcessWork', component: () => import('./views/ProcessWork.vue'), meta: { title: '工作流' } },
        { path: 'im', name: 'IMChat', component: () => import('./views/IMChat.vue'), meta: { title: '即时通讯' } },
        { path: 'personal', name: 'Personal', component: () => import('./views/Personal.vue'), meta: { title: '个人中心' } },
        { path: 'settings', name: 'Settings', component: () => import('./views/Settings.vue'), meta: { title: '系统设置' } },
        { path: 'calendar', name: 'CalendarApp', component: () => import('./views/CalendarApp.vue'), meta: { title: '日历' } },
        { path: 'file', name: 'FileManager', component: () => import('./views/FileManager.vue'), meta: { title: '文件管理' } },
        { path: 'bbs', name: 'BBSForum', component: () => import('./views/BBSForum.vue'), meta: { title: '论坛' } },
        { path: 'meeting', name: 'MeetingApp', component: () => import('./views/MeetingApp.vue'), meta: { title: '会议管理' } },
        { path: 'attendance', name: 'AttendanceApp', component: () => import('./views/AttendanceApp.vue'), meta: { title: '考勤管理' } },
        { path: 'query', name: 'QueryManager', component: () => import('./views/QueryManager.vue'), meta: { title: '查询管理' } },
        { path: 'portal', name: 'PortalApp', component: () => import('./views/PortalApp.vue'), meta: { title: '门户管理' } },
        { path: 'hotpic', name: 'HotpicApp', component: () => import('./views/HotpicApp.vue'), meta: { title: '热帖管理' } },
        { path: 'jpush', name: 'JPushApp', component: () => import('./views/JPushApp.vue'), meta: { title: '消息推送' } },
        { path: 'appinfo', name: 'AppInfoApp', component: () => import('./views/AppInfoApp.vue'), meta: { title: '应用管理' } },
        { path: 'category', name: 'CategoryApp', component: () => import('./views/CategoryApp.vue'), meta: { title: '分类管理' } },
        { path: 'mind', name: 'MindApp', component: () => import('./views/MindApp.vue'), meta: { title: '思维导图' } },
        { path: 'document', name: 'DocumentApp', component: () => import('./views/DocumentApp.vue'), meta: { title: '文档管理' } },
        { path: 'program', name: 'ProgramCenterApp', component: () => import('./views/ProgramCenterApp.vue'), meta: { title: '程序中心' } },
        { path: 'queryview', name: 'QueryViewApp', component: () => import('./views/QueryViewApp.vue'), meta: { title: '查询视图' } },
        { path: 'recycle', name: 'RecycleApp', component: () => import('./views/RecycleApp.vue'), meta: { title: '回收站' } },
        { path: 'server', name: 'ServerApp', component: () => import('./views/ServerApp.vue'), meta: { title: '服务器管理' } },
        { path: 'unit', name: 'UnitApp', component: () => import('./views/UnitApp.vue'), meta: { title: '单元管理' } },
        { path: 'form', name: 'FormApp', component: () => import('./views/FormApp.vue'), meta: { title: '表单管理' } },
        { path: 'view', name: 'ViewApp', component: () => import('./views/ViewApp.vue'), meta: { title: '视图管理' } },
        { path: 'fileinfo', name: 'FileInfoApp', component: () => import('./views/FileInfoApp.vue'), meta: { title: '文件信息' } },
        { path: 'ai-chat', name: 'AIChatApp', component: () => import('./views/AIChatApp.vue'), meta: { title: 'AI助手' } },
        { path: 'role', name: 'RoleManager', component: () => import('./views/RoleManager.vue'), meta: { title: '角色管理' } },
        { path: 'process-designer', name: 'ProcessDesigner', component: () => import('./views/ProcessDesigner.vue'), meta: { title: '流程设计器' } },
        { path: 'form-designer', name: 'FormDesigner', component: () => import('./views/FormDesigner.vue'), meta: { title: '表单设计器' } },        { path: 'query-statement-designer', name: 'QueryStatementDesigner', component: () => import('./views/QueryStatementDesigner.vue'), meta: { title: 'SQL语句设计器' } },

        { path: 'query-designer', name: 'QueryDesigner', component: () => import('./views/QueryDesigner.vue'), meta: { title: '查询设计器' } },        { path: 'query-manager-deep', name: 'QueryManagerDeep', component: () => import('./views/QueryManagerDeep.vue'), meta: { title: '查询管理深化' } },

        { path: 'portal-designer', name: 'PortalDesigner', component: () => import('./views/PortalDesigner.vue'), meta: { title: '门户设计器' } },
              { path: 'log-viewer', name: 'LogViewerApp', component: () => import('./views/LogViewerApp.vue'), meta: { title: '日志查看器' } },
        { path: 'selector', name: 'SelectorApp', component: () => import('./views/SelectorApp.vue'), meta: { title: '通用选择器' } },
        { path: 'query-query', name: 'QueryQueryApp', component: () => import('./views/QueryQueryApp.vue'), meta: { title: '查询定义' } },
        { path: 'query-explorer', name: 'QueryExplorerApp', component: () => import('./views/QueryExplorerApp.vue'), meta: { title: '查询浏览器' } },
        { path: 'query-table-designer', name: 'QueryTableDesignerApp', component: () => import('./views/QueryTableDesignerApp.vue'), meta: { title: '表格设计器' } },
        { path: 'query-view-designer', name: 'QueryViewDesignerApp', component: () => import('./views/QueryViewDesignerApp.vue'), meta: { title: '视图设计器' } },
        { path: 'query-stat-designer', name: 'QueryStatDesignerApp', component: () => import('./views/QueryStatDesignerApp.vue'), meta: { title: '统计设计器' } },
        { path: 'query-importer-designer', name: 'QueryImporterDesignerApp', component: () => import('./views/QueryImporterDesignerApp.vue'), meta: { title: '导入设计器' } },
        { path: 'cms-column', name: 'CmsColumnApp', component: () => import('./views/CmsColumnApp.vue'), meta: { title: 'CMS列管理' } },
        { path: 'cms-column-manager', name: 'CmsColumnManagerApp', component: () => import('./views/CmsColumnManagerApp.vue'), meta: { title: 'CMS列管理器' } },
        { path: 'cms-index', name: 'CmsIndexApp', component: () => import('./views/CmsIndexApp.vue'), meta: { title: 'CMS索引设计' } },
        { path: 'process-task-center', name: 'ProcessTaskCenterApp', component: () => import('./views/ProcessTaskCenterApp.vue'), meta: { title: '流程任务中心' } },
        { path: 'process-manager', name: 'ProcessManagerApp', component: () => import('./views/ProcessManagerApp.vue'), meta: { title: '流程实例管理' } },
        { path: 'cms-dict-designer', name: 'CmsDictDesignerApp', component: () => import('./views/CmsDictDesignerApp.vue'), meta: { title: 'CMS字典设计器' } },
        { path: 'cms-form-designer', name: 'CmsFormDesignerApp', component: () => import('./views/CmsFormDesignerApp.vue'), meta: { title: 'CMS表单设计器' } },
        { path: 'cms-view-designer', name: 'CmsViewDesignerApp', component: () => import('./views/CmsViewDesignerApp.vue'), meta: { title: 'CMS视图设计器' } },
        { path: 'cms-script-designer', name: 'CmsScriptDesignerApp', component: () => import('./views/CmsScriptDesignerApp.vue'), meta: { title: 'CMS脚本设计器' } },
        { path: 'cms-xform', name: 'CmsXformApp', component: () => import('./views/CmsXformApp.vue'), meta: { title: 'CMS XForm' } },
        { path: 'cms-module', name: 'CmsModuleApp', component: () => import('./views/CmsModuleApp.vue'), meta: { title: 'CMS模块管理' } },
        { path: 'portal-dict-designer', name: 'PortalDictDesignerApp', component: () => import('./views/PortalDictDesignerApp.vue'), meta: { title: '门户字典设计器' } },
        { path: 'portal-page-designer', name: 'PortalPageDesignerApp', component: () => import('./views/PortalPageDesignerApp.vue'), meta: { title: '门户页面设计器' } },
        { path: 'portal-script-designer', name: 'PortalScriptDesignerApp', component: () => import('./views/PortalScriptDesignerApp.vue'), meta: { title: '门户脚本设计器' } },
        { path: 'portal-widget-designer', name: 'PortalWidgetDesignerApp', component: () => import('./views/PortalWidgetDesignerApp.vue'), meta: { title: '门户组件设计器' } },
        { path: 'process-form-designer', name: 'ProcessFormDesignerApp', component: () => import('./views/ProcessFormDesignerApp.vue'), meta: { title: '流程表单设计器' } },
        { path: 'process-dict-designer', name: 'ProcessDictDesignerApp', component: () => import('./views/ProcessDictDesignerApp.vue'), meta: { title: '流程字典设计器' } },
        { path: 'process-script-designer', name: 'ProcessScriptDesignerApp', component: () => import('./views/ProcessScriptDesignerApp.vue'), meta: { title: '流程脚本设计器' } },
        { path: 'process-xform', name: 'ProcessXformApp', component: () => import('./views/ProcessXformApp.vue'), meta: { title: '流程XForm' } },
        { path: 'process-application', name: 'ProcessApplicationApp', component: () => import('./views/ProcessApplicationApp.vue'), meta: { title: '流程应用管理' } },
        { path: 'service-invoke-designer', name: 'ServiceInvokeDesignerApp', component: () => import('./views/ServiceInvokeDesignerApp.vue'), meta: { title: '服务调用设计器' } },
        { path: 'design-center', name: 'DesignCenterApp', component: () => import('./views/DesignCenterApp.vue'), meta: { title: '设计中心' } },
        { path: 'control-panel', name: 'ControlPanelApp', component: () => import('./views/ControlPanelApp.vue'), meta: { title: '控制面板' } },
        { path: 'config-designer', name: 'ConfigDesignerApp', component: () => import('./views/ConfigDesignerApp.vue'), meta: { title: '配置设计器' } },
        { path: 'find-designer', name: 'FindDesignerApp', component: () => import('./views/FindDesignerApp.vue'), meta: { title: '查找设计器' } },
        { path: 'homepage', name: 'HomepageApp', component: () => import('./views/HomepageApp.vue'), meta: { title: '首页配置' } },
        { path: 'bam', name: 'BamApp', component: () => import('./views/BamApp.vue'), meta: { title: '业务活动监控' } },
        { path: 'collect', name: 'CollectApp', component: () => import('./views/CollectApp.vue'), meta: { title: '收集管理' } },
        { path: 'note', name: 'NoteApp', component: () => import('./views/NoteApp.vue'), meta: { title: '笔记管理' } },
        { path: 'template', name: 'TemplateApp', component: () => import('./views/TemplateApp.vue'), meta: { title: '模板管理' } },
        { path: 'search', name: 'SearchApp', component: () => import('./views/SearchApp.vue'), meta: { title: '全局搜索' } },
        { path: 'pdf-viewer', name: 'PdfViewerApp', component: () => import('./views/PdfViewerApp.vue'), meta: { title: 'PDF查看器' } },
        { path: 'deployment', name: 'DeploymentApp', component: () => import('./views/DeploymentApp.vue'), meta: { title: '部署管理' } },
        { path: 'three-member', name: 'ThreeMemberApp', component: () => import('./views/ThreeMemberApp.vue'), meta: { title: '三方成员管理' } },
        { path: 'face-set', name: 'FaceSetApp', component: () => import('./views/FaceSetApp.vue'), meta: { title: '人脸设置' } },
        { path: 'ann', name: 'AnnApp', component: () => import('./views/AnnApp.vue'), meta: { title: '神经网络AI配置' } },
        { path: 'common', name: 'CommonApp', component: () => import('./views/CommonApp.vue'), meta: { title: '公共组件库' } },
        { path: 'ftsearch', name: 'FtSearchApp', component: () => import('./views/FtSearchApp.vue'), meta: { title: '全文搜索引擎' } },
        { path: 'empty', name: 'EmptyApp', component: () => import('./views/EmptyApp.vue'), meta: { title: '占位页面' } },
],
    },
    { path: '/oauth/callback/:platform', name: 'OAuthCallback', component: OAuthCallback, meta: { requiresAuth: false } },
    { path: '/sso', name: 'SSO', component: () => import('./views/SSO.vue'), meta: { requiresAuth: false } },
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
