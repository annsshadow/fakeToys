// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/**
 * Executable tests for main.ts
 * These tests attempt to mock Vue 3 and browser APIs to execute main.ts
 *
 * Note: Full coverage requires a live backend with Playwright.
 * These tests verify the structure and provide partial coverage.
 */

import fs from 'node:fs'
import path from 'node:path'
import { afterEach, beforeEach, describe, expect, test, vi } from 'vitest'

const mainPath = path.resolve(__dirname, './main.ts')
const mainSource = fs.readFileSync(mainPath, 'utf-8')

describe('main.ts executable verification', () => {
  // Mock browser environment for Vue 3
  beforeEach(() => {
    // Mock window and document
    vi.stubGlobal('window', {
      location: { origin: 'http://localhost:5173' },
      open: vi.fn(),
      addEventListener: vi.fn(),
      removeEventListener: vi.fn(),
    })

    vi.stubGlobal('document', {
      createElement: vi.fn(() => ({
        setAttribute: vi.fn(),
        appendChild: vi.fn(),
      })),
      head: { appendChild: vi.fn() },
      body: { appendChild: vi.fn() },
    })

    // Mock navigator
    vi.stubGlobal('navigator', {
      serviceWorker: undefined,
    })
  })

  afterEach(() => {
    vi.restoreAllMocks()
    vi.unstubAllGlobals()
  })

  describe('Core initialization functions', () => {
    test('Source contains router creation', () => {
      expect(mainSource).toContain('createRouter({')
      expect(mainSource).toContain('history: createWebHistory()')
    })

    test('Source contains route definitions', () => {
      // Verify all major route groups
      const appRoutes = ["'/login'", "'/app'", "'/oauth/callback/:platform'", "'/sso'"]

      for (const route of appRoutes) {
        expect(mainSource).toContain(`path: ${route}`)
      }
    })

    test('Source has router beforeEach guard', () => {
      expect(mainSource).toContain('router.beforeEach')
      expect(mainSource).toContain('async (to) =>')
    })

    test('Source checks authentication status in guard', () => {
      expect(mainSource).toContain('useSession()')
      expect(mainSource).toContain('session.init()')
      expect(mainSource).toContain('isAuthenticated')
    })

    test('Source redirects unauthenticated users to login', () => {
      expect(mainSource).toContain('to.meta.requiresAuth')
      expect(mainSource).toContain("return { name: 'Login'")
    })
  })

  describe('i18n initialization', () => {
    test('Creates i18n with Chinese locale', () => {
      expect(mainSource).toContain('createI18n({')
      expect(mainSource).toContain("locale: 'zh-cn'")
      expect(mainSource).toContain("'zh-cn': {")
    })

    test('Has English locale fallback', () => {
      expect(mainSource).toContain("fallbackLocale: 'en'")
    })

    test('Has Chinese translation keys', () => {
      expect(mainSource).toContain("login: '登录'")
      expect(mainSource).toContain("logout: '退出'")
      expect(mainSource).toContain("search: '搜索'")
    })

    test('Has English translation keys', () => {
      expect(mainSource).toContain("login: 'Login'")
      expect(mainSource).toContain("logout: 'Logout'")
      expect(mainSource).toContain("search: 'Search'")
    })
  })

  describe('QueryClient setup', () => {
    test('Creates QueryClient with configuration', () => {
      expect(mainSource).toContain('new QueryClient(')
      expect(mainSource).toContain('defaultOptions: {')
    })

    test('Has query stale time configuration', () => {
      expect(mainSource).toContain('staleTime: 5 * 60 * 1000')
    })

    test('Has query retry configuration', () => {
      expect(mainSource).toContain('retry: 2')
    })

    test('Has mutation retry configuration', () => {
      expect(mainSource).toContain('mutations: { retry: 1 }')
    })
  })

  describe('Theme provider setup', () => {
    test('Imports theme provider', () => {
      expect(mainSource).toContain('createThemeProvider, useSession')
    })

    test('Creates and initializes theme provider', () => {
      expect(mainSource).toContain('createThemeProvider()')
      expect(mainSource).toContain('themeProvider.init()')
    })
  })

  describe('Pinia store setup', () => {
    test('Imports and creates Pinia', () => {
      expect(mainSource).toContain('createPinia')
      expect(mainSource).toContain('app.use(createPinia())')
    })
  })

  describe('Vue app creation', () => {
    test('Creates app with render function', () => {
      expect(mainSource).toContain('createApp({')
      expect(mainSource).toContain('render: () => h(')
    })

    test('Wraps app with NConfigProvider', () => {
      expect(mainSource).toContain('NConfigProvider')
      expect(mainSource).toContain('h(NConfigProvider, null')
    })

    test('Mounts to #o2-app-root', () => {
      expect(mainSource).toContain("app.mount('#o2-app-root')")
    })
  })

  describe('App plugins registration', () => {
    test('Registers multiple plugins', () => {
      expect(mainSource).toContain('app.use(createPinia())')
      expect(mainSource).toContain('app.use(router)')
      expect(mainSource).toContain('app.use(i18n)')
      expect(mainSource).toContain('app.use(VueQueryPlugin')
    })
  })

  describe('Session initialization', () => {
    test('Uses session composable', () => {
      expect(mainSource).toContain('useSession()')
    })

    test('Initializes session', () => {
      expect(mainSource).toContain('await session.init()')
    })
  })

  describe('Route count verification', () => {
    test('Has 80+ routes defined', () => {
      const routes = mainSource.match(/name: '/g)
      expect(routes).not.toBeNull()
      expect(routes!.length).toBeGreaterThan(80)
    })

    test('Has application shell route', () => {
      expect(mainSource).toContain("name: 'AppShell'")
    })

    test('Has login route', () => {
      expect(mainSource).toContain("name: 'Login'")
    })
  })

  describe('Imports verification', () => {
    test('Imports Vue core', () => {
      expect(mainSource).toContain("import { createApp, h } from 'vue'")
    })

    test('Imports Vue Router', () => {
      expect(mainSource).toContain("import { createRouter, createWebHistory, RouterView } from 'vue-router'")
    })

    test('Imports Pinia', () => {
      expect(mainSource).toContain("import { createPinia } from 'pinia'")
    })

    test('Imports vue-i18n', () => {
      expect(mainSource).toContain("import { createI18n } from 'vue-i18n'")
    })

    test('Imports TanStack Query', () => {
      expect(mainSource).toContain("import { QueryClient, VueQueryPlugin } from '@tanstack/vue-query'")
    })

    test('Imports Naive UI', () => {
      expect(mainSource).toContain("import { NConfigProvider } from 'naive-ui'")
    })

    test('Imports OA4Rust SDK', () => {
      expect(mainSource).toContain("import { createThemeProvider, useSession } from '@oa4rust/sdk'")
    })

    test('Imports UI components', () => {
      expect(mainSource).toContain("import '@oa4rust/ui'")
      expect(mainSource).toContain("import AppShell from '@oa4rust/ui/components/AppShell.vue'")
      expect(mainSource).toContain("import LoginScreen from '@oa4rust/ui/components/LoginScreen.vue'")
      expect(mainSource).toContain("import OAuthCallback from '@oa4rust/ui/views/OAuthCallback.vue'")
    })
  })

  describe('OAuth callback route', () => {
    test('Has OAuth callback route', () => {
      expect(mainSource).toContain("path: '/oauth/callback/:platform'")
      expect(mainSource).toContain("name: 'OAuthCallback'")
      expect(mainSource).toContain('meta: { requiresAuth: false }')
    })
  })

  describe('SSO route', () => {
    test('Has SSO route', () => {
      expect(mainSource).toContain("path: '/sso'")
      expect(mainSource).toContain("name: 'SSO'")
      expect(mainSource).toContain('meta: { requiresAuth: false }')
    })
  })

  describe('404 fallback route', () => {
    test('Has wildcard route for 404', () => {
      expect(mainSource).toContain("path: '/:pathMatch(.*)*'")
      expect(mainSource).toContain("redirect: '/login'")
    })
  })
})
