// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/** 月度考勤明细行（只声明页面实际消费的字段；其余字段原样保留）。 */
export interface MyDetailRow {
  id?: string
  date?: string
  status?: string
  [key: string]: unknown
}

/**
 * 后端 /my/detail/list 的 data 在异常信封/网关改写时可能不是数组（如 { list: [...] }
 * 或错误对象）。页面直接 as MyDetailRow[] 会把对象塞进 ref，模板 myDetails.length
 * 变 undefined 导致「暂无考勤明细」永远不显示、v-for 拿到对象渲染出半截数据。
 * 与仓内其它列表（bbs/collect/process/loadList）一致：只接受真数组。
 */
export function toMyDetailRows(value: unknown): MyDetailRow[] {
  return Array.isArray(value) ? (value as MyDetailRow[]) : []
}
