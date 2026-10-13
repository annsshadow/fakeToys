// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { createApp, type DefineComponent } from 'vue'
import type { O2Desktop } from './types.js'

export interface WidgetDefinition {
  name: string
  component: DefineComponent
  options?: Record<string, unknown>
}

const widgets = new Map<string, WidgetDefinition>()

export function defineWidget(name: string, component: DefineComponent, options?: Record<string, unknown>): void {
  widgets.set(name, { name, component, options })
}

export function getWidget(name: string): WidgetDefinition | undefined {
  return widgets.get(name)
}

export function defineWidgets(registries: Array<[string, DefineComponent, Record<string, unknown>?]>): void {
  for (const [name, component, options] of registries) {
    defineWidget(name, component, options)
  }
}

export function getAllWidgets(): WidgetDefinition[] {
  return Array.from(widgets.values())
}

export function renderWidget(
  container: HTMLElement,
  name: string,
  options?: Record<string, unknown>,
  desktop?: O2Desktop,
): void {
  const widget = widgets.get(name)
  if (!widget) {
    console.error(`[oa4rust-sdk] Widget "${name}" not found`)
    return
  }
  // 安全审计（2026-10）：清空容器时走 textContent 而非 innerHTML=''。两者本例都
  // 只清空自身子结点，但 textContent 明确不解析/注入 HTML，杜绝未来误把
  // 不可信字符串赋给 innerHTML 引入 XSS 的退化路径（见 security.test.ts 守护）。
  container.textContent = ''
  const app = createApp(widget.component, {
    ...widget.options,
    ...options,
    desktop,
  })
  app.provide('desktop', desktop ?? null)
  app.provide('widgetName', name)
  app.mount(container)
}

export { defineWidget as loadComponent }
