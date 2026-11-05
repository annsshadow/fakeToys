// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { type ComputedRef, computed } from 'vue'

/**
 * 把 useQuery 的 `data` 变成随刷新更新的列表（R107）。
 *
 * 生成页原写法 `items.value = Array.isArray(data.value) ? ... : []` 只在 setup 执行一次，
 * 之后 `invalidateQueries` / 轮询刷新拿到的新数组不会再回写（用户点「刷新」看到的还是旧列表）。
 * 用 getter 而不是直接传 `data`：`data` 在 setup 里晚于本调用声明，直接读会踩 TDZ。
 */
export function liveQueryItems<T>(readSource: () => T[] | undefined): ComputedRef<T[]> {
  return computed<T[]>(() => {
    const rows = readSource()
    return Array.isArray(rows) ? rows : []
  })
}
