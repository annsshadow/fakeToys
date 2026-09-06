import { h, createApp, type Component, type DefineComponent } from 'vue';
import type { O2Desktop } from './types.js';

export interface WidgetDefinition {
  name: string;
  component: DefineComponent;
  options?: Record<string, unknown>;
}

const widgets = new Map<string, WidgetDefinition>();

export function defineWidget(
  name: string,
  component: DefineComponent,
  options?: Record<string, unknown>,
): void {
  widgets.set(name, { name, component, options });
}

export function getWidget(name: string): WidgetDefinition | undefined {
  return widgets.get(name);
}

export function defineWidgets(registries: Array<[string, DefineComponent, Record<string, unknown>?]>): void {
  for (const [name, component, options] of registries) {
    defineWidget(name, component, options);
  }
}

export function getAllWidgets(): WidgetDefinition[] {
  return Array.from(widgets.values());
}

export function renderWidget(
  container: HTMLElement,
  name: string,
  options?: Record<string, unknown>,
  desktop?: O2Desktop,
): void {
  const widget = widgets.get(name);
  if (!widget) {
    console.error(`[oa4rust-sdk] Widget "${name}" not found`);
    return;
  }
  container.innerHTML = '';
  const app = createApp(widget.component, {
    ...widget.options,
    ...options,
    desktop,
  });
  app.provide('desktop', desktop ?? null);
  app.provide('widgetName', name);
  app.mount(container);
}

export { defineWidget as loadComponent };
