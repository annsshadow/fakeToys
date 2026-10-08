// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import type { QueryClient } from '@tanstack/vue-query'
import { type WatchSource, watch } from 'vue'

/**
 * 查询缓存与登录人的隔离守卫。
 *
 * 视图 queryKey 普遍不含登录人（如 ['Collect', 'list']）：登录人变化
 * （登出 / 切换用户 / 换账号重登）时若保留查询缓存，staleTime 窗口内
 * 上一账号的数据会被直接渲染给当前账号——跨账号数据泄漏。
 * 登录人身份一旦变化即清空全部查询缓存。
 */
export function watchSessionCacheReset(
  queryClient: Pick<QueryClient, 'clear'>,
  currentUserUnique: WatchSource<string | null | undefined>,
): void {
  watch(currentUserUnique, (who, prev) => {
    if (who !== prev) queryClient.clear()
  })
}
