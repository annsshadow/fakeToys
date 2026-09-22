// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/**
 * 移动端导航守卫。uni-app 使用 pages.json + uni.* 导航（无 vue-router），
 * 故以函数式守卫代替 beforeEach：受保护页在 onShow 中调用 ensureAuthenticated。
 */

import { useSession } from '@/store/session'

export const LOGIN_PAGE = '/pages/login/login'
export const HOME_PAGE = '/pages/index/index'

/** 确保已认证；未认证则重定向到登录页并返回 false。 */
export async function ensureAuthenticated(): Promise<boolean> {
  const session = useSession()
  await session.init()
  if (session.isAuthenticated) return true
  uni.reLaunch({ url: LOGIN_PAGE })
  return false
}

/** 登录成功后跳转到工作台（tabBar 页必须用 switchTab）。 */
export function navigateHome(): void {
  uni.switchTab({ url: HOME_PAGE })
}
