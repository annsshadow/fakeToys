import { createApp, type App as VueApp, type Component } from 'vue';
import { createPinia } from 'pinia';
import { createRouter, createWebHistory, type Router } from 'vue-router';
import { setRouter } from './router.js';
import { useSession } from './session.js';
import { themeProvider } from './theme.js';
import type { O2AppOptions } from './types.js';

let _app: VueApp | null = null;
let _router: Router | null = null;

export interface O2AppInitOptions {
  /** 路由配置 */
  routes?: Array<{ path: string; name?: string; component: Component; meta?: Record<string, unknown> }>;
  /** 是否启用认证守卫，默认 true */
  authGuard?: boolean;
  /** 默认主题 */
  theme?: 'dark' | 'light';
  /** API 基础 URL */
  apiBase?: string;
}

export function createO2App(container: string | HTMLElement, options: O2AppInitOptions = {}): { app: VueApp; router: Router; pinia: ReturnType<typeof createPinia> } {
  const el = typeof container === 'string' ? document.querySelector(container) : container;
  if (!el) throw new Error(`Container not found: ${container}`);

  themeProvider.init();
  if (options.theme) themeProvider.setTheme(options.theme);

  const pinia = createPinia();
  const app = createApp({ template: '<router-view />' });
  app.use(pinia);

  const router = createRouter({ history: createWebHistory(), routes: options.routes || [] });
  setRouter(router);
  app.use(router);

  if (options.authGuard !== false) {
    router.beforeEach(async (to) => {
      if (to.meta.requiresAuth !== false) {
        const session = useSession();
        await session.init();
        if (!session.isAuthenticated && to.name !== 'Login' && !to.path.startsWith('/oauth')) {
          return { name: 'Login', query: { redirect: to.fullPath } };
        }
      }
    });
  }

  app.mount(el);
  _app = app;
  _router = router;
  return { app, router, pinia };
}

export function getO2App(): VueApp {
  if (!_app) throw new Error('createO2App() not called yet');
  return _app;
}

export function getO2Router(): Router {
  if (!_router) throw new Error('createO2App() not called yet');
  return _router;
}
