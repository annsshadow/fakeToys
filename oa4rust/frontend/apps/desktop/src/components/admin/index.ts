// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/** R298 共享管理原语 barrel：t2-t4 管理页统一从这里导入。 */
export { default as AdminFormSection } from './AdminFormSection.vue'
export { default as AdminLayout } from './AdminLayout.vue'
export { default as AdminPageHeader } from './AdminPageHeader.vue'
export { default as AdminTable } from './AdminTable.vue'
export { default as AdminToolbar } from './AdminToolbar.vue'
export { ADMIN_HIDE_PAGE_TITLE, ADMIN_LAYOUT_HIDE_TITLE } from './adminInjection'
export type { AdminNavGroup, AdminNavItem } from './adminNav'
export { adminNavGroups } from './adminNav'
export { default as ModernUI } from './ModernUI.vue'
