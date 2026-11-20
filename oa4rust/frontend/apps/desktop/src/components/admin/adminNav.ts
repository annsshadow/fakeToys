// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R298 管理区分组侧栏数据：六组覆盖 main.ts 嵌套在 AdminLayout 下的全部 58 条路由。
 * label 与路由 meta.title 同源（adminNav.test.ts 双向校验），侧栏/面包屑/文档标题一致。
 */
export interface AdminNavItem {
  path: string
  label: string
  icon: string
}

export interface AdminNavGroup {
  label: string
  icon: string
  items: AdminNavItem[]
}

export const adminNavGroups: AdminNavGroup[] = [
  {
    label: '组织与权限',
    icon: '🏢',
    items: [
      { path: '/app/admin/role', label: '角色管理', icon: '🔑' },
      { path: '/app/admin/three-member', label: '三方成员管理', icon: '👥' },
      { path: '/app/admin/appinfo', label: '应用管理', icon: '📱' },
    ],
  },
  {
    label: '内容管理',
    icon: '📚',
    items: [
      { path: '/app/admin/document', label: '文档管理', icon: '📄' },
      { path: '/app/admin/category', label: '分类管理', icon: '🗂️' },
      { path: '/app/admin/hotpic', label: '热帖管理', icon: '🔥' },
      { path: '/app/admin/template', label: '模板管理', icon: '🧩' },
      { path: '/app/admin/collect', label: '收集管理', icon: '📥' },
      { path: '/app/admin/form', label: '表单管理', icon: '📋' },
      { path: '/app/admin/view', label: '视图管理', icon: '🪟' },
      { path: '/app/admin/fileinfo', label: '文件信息', icon: '🗃️' },
      { path: '/app/admin/cms-column', label: 'CMS列管理', icon: '🧱' },
      { path: '/app/admin/cms-column-manager', label: 'CMS列管理器', icon: '🧰' },
      { path: '/app/admin/cms-index', label: 'CMS索引设计', icon: '📇' },
      { path: '/app/admin/cms-dict-designer', label: 'CMS字典设计器', icon: '📖' },
      { path: '/app/admin/cms-form-designer', label: 'CMS表单设计器', icon: '🛠️' },
      { path: '/app/admin/cms-view-designer', label: 'CMS视图设计器', icon: '🖼️' },
      { path: '/app/admin/cms-script-designer', label: 'CMS脚本设计器', icon: '📜' },
      { path: '/app/admin/cms-xform', label: 'CMS XForm', icon: '🧾' },
      { path: '/app/admin/cms-module', label: 'CMS模块管理', icon: '📦' },
    ],
  },
  {
    label: '流程管理',
    icon: '🔀',
    items: [
      { path: '/app/admin/process-designer', label: '流程设计器', icon: '🧭' },
      { path: '/app/admin/form-designer', label: '表单设计器', icon: '✏️' },
      { path: '/app/admin/process-task-center', label: '流程任务中心', icon: '📌' },
      { path: '/app/admin/process-manager', label: '流程实例管理', icon: '🗓️' },
      { path: '/app/admin/process-form-designer', label: '流程表单设计器', icon: '📝' },
      { path: '/app/admin/process-dict-designer', label: '流程字典设计器', icon: '📚' },
      { path: '/app/admin/process-script-designer', label: '流程脚本设计器', icon: '⌨️' },
      { path: '/app/admin/process-xform', label: '流程XForm', icon: '🧮' },
      { path: '/app/admin/process-application', label: '流程应用管理', icon: '🚦' },
      { path: '/app/admin/service-invoke-designer', label: '服务调用设计器', icon: '🔗' },
    ],
  },
  {
    label: '查询中心',
    icon: '🔍',
    items: [
      { path: '/app/admin/query', label: '查询管理', icon: '🔎' },
      { path: '/app/admin/queryview', label: '查询视图', icon: '👁️' },
      { path: '/app/admin/query-query', label: '查询定义', icon: '📐' },
      { path: '/app/admin/query-explorer', label: '查询浏览器', icon: '🧪' },
      { path: '/app/admin/query-manager-deep', label: '查询管理深化', icon: '🔬' },
      { path: '/app/admin/query-designer', label: '查询设计器', icon: '🛠️' },
      { path: '/app/admin/query-statement-designer', label: 'SQL语句设计器', icon: '💾' },
      { path: '/app/admin/query-table-designer', label: '表格设计器', icon: '📊' },
      { path: '/app/admin/query-view-designer', label: '视图设计器', icon: '🖥️' },
      { path: '/app/admin/query-stat-designer', label: '统计设计器', icon: '📈' },
      { path: '/app/admin/query-importer-designer', label: '导入设计器', icon: '📤' },
      { path: '/app/admin/find-designer', label: '查找设计器', icon: '🔦' },
    ],
  },
  {
    label: '门户管理',
    icon: '🌐',
    items: [
      { path: '/app/admin/portal', label: '门户管理', icon: '🏠' },
      { path: '/app/admin/portal-designer', label: '门户设计器', icon: '🎨' },
      { path: '/app/admin/portal-dict-designer', label: '门户字典设计器', icon: '📕' },
      { path: '/app/admin/portal-page-designer', label: '门户页面设计器', icon: '📃' },
      { path: '/app/admin/portal-script-designer', label: '门户脚本设计器', icon: '🖊️' },
      { path: '/app/admin/portal-widget-designer', label: '门户组件设计器', icon: '🧷' },
    ],
  },
  {
    label: '系统运维',
    icon: '⚙',
    items: [
      { path: '/app/admin/control-panel', label: '控制面板', icon: '🎛️' },
      { path: '/app/admin/design-center', label: '设计中心', icon: '🏗️' },
      { path: '/app/admin/program', label: '程序中心', icon: '💻' },
      { path: '/app/admin/server', label: '服务器管理', icon: '🖧' },
      { path: '/app/admin/config-designer', label: '配置设计器', icon: '🔧' },
      { path: '/app/admin/deployment', label: '部署管理', icon: '🚀' },
      { path: '/app/admin/log-viewer', label: '日志查看器', icon: '📜' },
      { path: '/app/admin/jpush', label: '消息推送', icon: '📣' },
      { path: '/app/admin/pdf-viewer', label: 'PDF查看器', icon: '📑' },
      { path: '/app/admin/common', label: '公共组件库', icon: '🧲' },
    ],
  },
]
