// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/** R299: AdminLayout 标题隐藏开关的独立 injection key（tsc 不从 .vue 读取导出）。 */
import type { InjectionKey, Ref } from 'vue'
export const ADMIN_LAYOUT_HIDE_TITLE: InjectionKey<Ref<boolean>> = Symbol('ADMIN_LAYOUT_HIDE_TITLE')
/** R300: 页面自带标题隐藏开关：AdminLayout 默认提供，子页 inject 后置 true 隐藏 AdminPageHeader 标题/副标题。 */
export const ADMIN_HIDE_PAGE_TITLE: InjectionKey<Ref<boolean>> = Symbol('ADMIN_HIDE_PAGE_TITLE')
