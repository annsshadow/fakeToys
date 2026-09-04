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
import AIChatApp from './views/AIChatApp.vue';
import RoleManager from './views/RoleManager.vue';
import ProcessDesigner from './views/ProcessDesigner.vue';
import FormDesigner from './views/FormDesigner.vue';
import QueryDesigner from './views/QueryDesigner.vue';
import PortalDesigner from './views/PortalDesigner.vue';


import LogViewerApp from './views/LogViewerApp.vue';
import SelectorApp from './views/SelectorApp.vue';
import QueryQueryApp from './views/QueryQueryApp.vue';
import QueryExplorerApp from './views/QueryExplorerApp.vue';
import QueryTableDesignerApp from './views/QueryTableDesignerApp.vue';
import QueryViewDesignerApp from './views/QueryViewDesignerApp.vue';
import QueryManagerDeep from './views/QueryManagerDeep.vue';
import QueryStatementDesignerApp from './views/QueryStatementDesignerApp.vue';
import QueryStatDesignerApp from './views/QueryStatDesignerApp.vue';
import QueryImporterDesignerApp from './views/QueryImporterDesignerApp.vue';
import CmsColumnApp from './views/CmsColumnApp.vue';
import CmsColumnManagerApp from './views/CmsColumnManagerApp.vue';
import CmsIndexApp from './views/CmsIndexApp.vue';
import ProcessTaskCenterApp from './views/ProcessTaskCenterApp.vue';
import ProcessManagerApp from './views/ProcessManagerApp.vue';
import CmsDictDesignerApp from './views/CmsDictDesignerApp.vue';
import CmsFormDesignerApp from './views/CmsFormDesignerApp.vue';
import CmsViewDesignerApp from './views/CmsViewDesignerApp.vue';
import CmsScriptDesignerApp from './views/CmsScriptDesignerApp.vue';
import CmsXformApp from './views/CmsXformApp.vue';
import CmsModuleApp from './views/CmsModuleApp.vue';
import PortalDictDesignerApp from './views/PortalDictDesignerApp.vue';
import PortalPageDesignerApp from './views/PortalPageDesignerApp.vue';
import PortalScriptDesignerApp from './views/PortalScriptDesignerApp.vue';
import PortalWidgetDesignerApp from './views/PortalWidgetDesignerApp.vue';
import ProcessFormDesignerApp from './views/ProcessFormDesignerApp.vue';
import ProcessDictDesignerApp from './views/ProcessDictDesignerApp.vue';
import ProcessScriptDesignerApp from './views/ProcessScriptDesignerApp.vue';
import ProcessXformApp from './views/ProcessXformApp.vue';
import ProcessApplicationApp from './views/ProcessApplicationApp.vue';
import ServiceInvokeDesignerApp from './views/ServiceInvokeDesignerApp.vue';
import DesignCenterApp from './views/DesignCenterApp.vue';
import ControlPanelApp from './views/ControlPanelApp.vue';
import ConfigDesignerApp from './views/ConfigDesignerApp.vue';
import FindDesignerApp from './views/FindDesignerApp.vue';
import HomepageApp from './views/HomepageApp.vue';
import BamApp from './views/BamApp.vue';
import CollectApp from './views/CollectApp.vue';
import NoteApp from './views/NoteApp.vue';
import TemplateApp from './views/TemplateApp.vue';
import SearchApp from './views/SearchApp.vue';
import PdfViewerApp from './views/PdfViewerApp.vue';
import DeploymentApp from './views/DeploymentApp.vue';
import ThreeMemberApp from './views/ThreeMemberApp.vue';
import FaceSetApp from './views/FaceSetApp.vue';
import AnnApp from './views/AnnApp.vue';
import CommonApp from './views/CommonApp.vue';
import FtSearchApp from './views/FtSearchApp.vue';
import EmptyApp from './views/EmptyApp.vue';
import QueryStatementDesigner from './views/QueryStatementDesigner.vue';
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
        { path: 'ai-chat', name: 'AIChatApp', component: AIChatApp, meta: { title: 'AI助手' } },
        { path: 'role', name: 'RoleManager', component: RoleManager, meta: { title: '角色管理' } },
        { path: 'process-designer', name: 'ProcessDesigner', component: ProcessDesigner, meta: { title: '流程设计器' } },
        { path: 'form-designer', name: 'FormDesigner', component: FormDesigner, meta: { title: '表单设计器' } },        { path: 'query-statement-designer', name: 'QueryStatementDesigner', component: QueryStatementDesigner, meta: { title: 'SQL语句设计器' } },

        { path: 'query-designer', name: 'QueryDesigner', component: QueryDesigner, meta: { title: '查询设计器' } },        { path: 'query-manager-deep', name: 'QueryManagerDeep', component: QueryManagerDeep, meta: { title: '查询管理深化' } },

        { path: 'portal-designer', name: 'PortalDesigner', component: PortalDesigner, meta: { title: '门户设计器' } },
              { path: 'log-viewer', name: 'LogViewerApp', component: LogViewerApp, meta: { title: '日志查看器' } },
        { path: 'selector', name: 'SelectorApp', component: SelectorApp, meta: { title: '通用选择器' } },
        { path: 'query-query', name: 'QueryQueryApp', component: QueryQueryApp, meta: { title: '查询定义' } },
        { path: 'query-explorer', name: 'QueryExplorerApp', component: QueryExplorerApp, meta: { title: '查询浏览器' } },
        { path: 'query-table-designer', name: 'QueryTableDesignerApp', component: QueryTableDesignerApp, meta: { title: '表格设计器' } },
        { path: 'query-view-designer', name: 'QueryViewDesignerApp', component: QueryViewDesignerApp, meta: { title: '视图设计器' } },
        { path: 'query-statement-designer', name: 'QueryStatementDesignerApp', component: QueryStatementDesignerApp, meta: { title: 'SQL设计器' } },
        { path: 'query-stat-designer', name: 'QueryStatDesignerApp', component: QueryStatDesignerApp, meta: { title: '统计设计器' } },
        { path: 'query-importer-designer', name: 'QueryImporterDesignerApp', component: QueryImporterDesignerApp, meta: { title: '导入设计器' } },
        { path: 'cms-column', name: 'CmsColumnApp', component: CmsColumnApp, meta: { title: 'CMS列管理' } },
        { path: 'cms-column-manager', name: 'CmsColumnManagerApp', component: CmsColumnManagerApp, meta: { title: 'CMS列管理器' } },
        { path: 'cms-index', name: 'CmsIndexApp', component: CmsIndexApp, meta: { title: 'CMS索引设计' } },
        { path: 'process-task-center', name: 'ProcessTaskCenterApp', component: ProcessTaskCenterApp, meta: { title: '流程任务中心' } },
        { path: 'process-manager', name: 'ProcessManagerApp', component: ProcessManagerApp, meta: { title: '流程实例管理' } },
        { path: 'cms-dict-designer', name: 'CmsDictDesignerApp', component: CmsDictDesignerApp, meta: { title: 'CMS字典设计器' } },
        { path: 'cms-form-designer', name: 'CmsFormDesignerApp', component: CmsFormDesignerApp, meta: { title: 'CMS表单设计器' } },
        { path: 'cms-view-designer', name: 'CmsViewDesignerApp', component: CmsViewDesignerApp, meta: { title: 'CMS视图设计器' } },
        { path: 'cms-script-designer', name: 'CmsScriptDesignerApp', component: CmsScriptDesignerApp, meta: { title: 'CMS脚本设计器' } },
        { path: 'cms-xform', name: 'CmsXformApp', component: CmsXformApp, meta: { title: 'CMS XForm' } },
        { path: 'cms-module', name: 'CmsModuleApp', component: CmsModuleApp, meta: { title: 'CMS模块管理' } },
        { path: 'portal-dict-designer', name: 'PortalDictDesignerApp', component: PortalDictDesignerApp, meta: { title: '门户字典设计器' } },
        { path: 'portal-page-designer', name: 'PortalPageDesignerApp', component: PortalPageDesignerApp, meta: { title: '门户页面设计器' } },
        { path: 'portal-script-designer', name: 'PortalScriptDesignerApp', component: PortalScriptDesignerApp, meta: { title: '门户脚本设计器' } },
        { path: 'portal-widget-designer', name: 'PortalWidgetDesignerApp', component: PortalWidgetDesignerApp, meta: { title: '门户组件设计器' } },
        { path: 'process-form-designer', name: 'ProcessFormDesignerApp', component: ProcessFormDesignerApp, meta: { title: '流程表单设计器' } },
        { path: 'process-dict-designer', name: 'ProcessDictDesignerApp', component: ProcessDictDesignerApp, meta: { title: '流程字典设计器' } },
        { path: 'process-script-designer', name: 'ProcessScriptDesignerApp', component: ProcessScriptDesignerApp, meta: { title: '流程脚本设计器' } },
        { path: 'process-xform', name: 'ProcessXformApp', component: ProcessXformApp, meta: { title: '流程XForm' } },
        { path: 'process-application', name: 'ProcessApplicationApp', component: ProcessApplicationApp, meta: { title: '流程应用管理' } },
        { path: 'service-invoke-designer', name: 'ServiceInvokeDesignerApp', component: ServiceInvokeDesignerApp, meta: { title: '服务调用设计器' } },
        { path: 'design-center', name: 'DesignCenterApp', component: DesignCenterApp, meta: { title: '设计中心' } },
        { path: 'control-panel', name: 'ControlPanelApp', component: ControlPanelApp, meta: { title: '控制面板' } },
        { path: 'config-designer', name: 'ConfigDesignerApp', component: ConfigDesignerApp, meta: { title: '配置设计器' } },
        { path: 'find-designer', name: 'FindDesignerApp', component: FindDesignerApp, meta: { title: '查找设计器' } },
        { path: 'homepage', name: 'HomepageApp', component: HomepageApp, meta: { title: '首页配置' } },
        { path: 'bam', name: 'BamApp', component: BamApp, meta: { title: '业务活动监控' } },
        { path: 'collect', name: 'CollectApp', component: CollectApp, meta: { title: '收集管理' } },
        { path: 'note', name: 'NoteApp', component: NoteApp, meta: { title: '笔记管理' } },
        { path: 'template', name: 'TemplateApp', component: TemplateApp, meta: { title: '模板管理' } },
        { path: 'search', name: 'SearchApp', component: SearchApp, meta: { title: '全局搜索' } },
        { path: 'pdf-viewer', name: 'PdfViewerApp', component: PdfViewerApp, meta: { title: 'PDF查看器' } },
        { path: 'deployment', name: 'DeploymentApp', component: DeploymentApp, meta: { title: '部署管理' } },
        { path: 'three-member', name: 'ThreeMemberApp', component: ThreeMemberApp, meta: { title: '三方成员管理' } },
        { path: 'face-set', name: 'FaceSetApp', component: FaceSetApp, meta: { title: '人脸设置' } },
        { path: 'ann', name: 'AnnApp', component: AnnApp, meta: { title: '神经网络AI配置' } },
        { path: 'common', name: 'CommonApp', component: CommonApp, meta: { title: '公共组件库' } },
        { path: 'ftsearch', name: 'FtSearchApp', component: FtSearchApp, meta: { title: '全文搜索引擎' } },
        { path: 'empty', name: 'EmptyApp', component: EmptyApp, meta: { title: '占位页面' } },
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
