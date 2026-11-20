// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * App 侧栏菜单导航接线（测试服「菜单点不了」实证修复）
 *
 * 线上实证：/augmentor/ 首页 11 个菜单点击后 URL 纹丝不动——menuItems 只有
 * key/icon/label，Menu 没接 onClick，useNavigate 全仓零引用。本用例把
 * 「点菜单 → 路由跳转 → 对应页面渲染 → 选中态跟随」钉成契约。
 *
 * 11 个 lazy 页面全部用轻量替身（各自的行为由 pages/*.test.tsx 管），
 * 这里只关心 App 层的接线。子路径用例用 vi.stubEnv 模拟 VITE_BASE=/augmentor/
 * 的构建形态（Router basename=import.meta.env.BASE_URL）。
 */
import { afterEach, describe, expect, it, vi } from 'vitest'
import { cleanup, fireEvent, render, screen, waitFor } from '@testing-library/react'

window.matchMedia =
  window.matchMedia ||
  ((query: string) =>
    ({
      matches: false,
      media: query,
      onchange: null,
      addListener: () => {},
      removeListener: () => {},
      addEventListener: () => {},
      removeEventListener: () => {},
      dispatchEvent: () => false,
    }) as unknown as MediaQueryList)
class RO {
  observe() {}
  unobserve() {}
  disconnect() {}
}
window.ResizeObserver = window.ResizeObserver || (RO as unknown as typeof ResizeObserver)

// 11 个页面替身：只渲染一个可断言的标记，避免整页挂载的重负载。
vi.mock('./pages/DataManagement', () => ({ default: () => <div>page:data</div> }))
vi.mock('./pages/Dashboard', () => ({ default: () => <div>page:dashboard</div> }))
vi.mock('./pages/Augmentation', () => ({ default: () => <div>page:augment</div> }))
vi.mock('./pages/Quality', () => ({ default: () => <div>page:quality</div> }))
vi.mock('./pages/Security', () => ({ default: () => <div>page:security</div> }))
vi.mock('./pages/Export', () => ({ default: () => <div>page:export</div> }))
vi.mock('./pages/Analysis', () => ({ default: () => <div>page:analysis</div> }))
vi.mock('./pages/Multimodal', () => ({ default: () => <div>page:multimodal</div> }))
vi.mock('./pages/Versions', () => ({ default: () => <div>page:versions</div> }))
vi.mock('./pages/Settings', () => ({ default: () => <div>page:settings</div> }))
vi.mock('./pages/System', () => ({ default: () => <div>page:system</div> }))

import App from './App'

// 菜单文案 → 路由 key → 页面替身标记（与 App.tsx menuItems/Routes 一一对应）
const MENUS: Array<[string, string, string]> = [
  ['仪表盘', '/dashboard', 'page:dashboard'],
  ['数据管理', '/', 'page:data'],
  ['数据增强', '/augment', 'page:augment'],
  ['质量中心', '/quality', 'page:quality'],
  ['数据安全', '/security', 'page:security'],
  ['导出中心', '/export', 'page:export'],
  ['数据分析', '/analysis', 'page:analysis'],
  ['多模态', '/multimodal', 'page:multimodal'],
  ['版本管理', '/versions', 'page:versions'],
  ['配置管理', '/settings', 'page:settings'],
  ['系统状态', '/system', 'page:system'],
]

afterEach(() => {
  cleanup()
  vi.unstubAllEnvs()
  window.history.replaceState(null, '', '/')
})

describe('侧栏菜单点击必须触发路由跳转（根路径部署）', () => {
  it.each(MENUS)('点击「%s」跳到 %s 并渲染对应页面', async (label, route, marker) => {
    // 起点必须不同于目标路由，否则「点了没反应」也能空转通过（数据管理目标就是 /）
    window.history.replaceState(null, '', route === '/system' ? '/' : '/system')
    render(<App />)
    fireEvent.click(screen.getByRole('menuitem', { name: new RegExp(label) }))
    await waitFor(() => {
      expect(window.location.pathname).toBe(route)
      expect(screen.getByText(marker)).toBeInTheDocument()
    })
    // 选中态跟随当前路由
    expect(screen.getByRole('menuitem', { name: new RegExp(label) })).toHaveClass('ant-menu-item-selected')
  })

  it('连续切换菜单：后点的页面覆盖先点的，URL 与选中态一致', async () => {
    window.history.replaceState(null, '', '/')
    render(<App />)
    fireEvent.click(screen.getByRole('menuitem', { name: /质量中心/ }))
    await waitFor(() => expect(screen.getByText('page:quality')).toBeInTheDocument())
    fireEvent.click(screen.getByRole('menuitem', { name: /系统状态/ }))
    await waitFor(() => {
      expect(window.location.pathname).toBe('/system')
      expect(screen.getByText('page:system')).toBeInTheDocument()
    })
    expect(screen.queryByText('page:quality')).not.toBeInTheDocument()
  })
})

describe('子路径部署（basename=/augmentor/）菜单同样可点', () => {
  it('直达 /augmentor/quality 渲染质量中心，点「数据管理」跳到 /augmentor/', async () => {
    vi.stubEnv('BASE_URL', '/augmentor/')
    window.history.replaceState(null, '', '/augmentor/quality')
    render(<App />)
    await waitFor(() => expect(screen.getByText('page:quality')).toBeInTheDocument())
    fireEvent.click(screen.getByRole('menuitem', { name: /数据管理/ }))
    await waitFor(() => {
      expect(window.location.pathname).toBe('/augmentor/')
      expect(screen.getByText('page:data')).toBeInTheDocument()
    })
  })

  it('子路径下点击「仪表盘」URL 带前缀 /augmentor/dashboard', async () => {
    vi.stubEnv('BASE_URL', '/augmentor/')
    window.history.replaceState(null, '', '/augmentor/')
    render(<App />)
    fireEvent.click(screen.getByRole('menuitem', { name: /仪表盘/ }))
    await waitFor(() => {
      expect(window.location.pathname).toBe('/augmentor/dashboard')
      expect(screen.getByText('page:dashboard')).toBeInTheDocument()
    })
  })
})
