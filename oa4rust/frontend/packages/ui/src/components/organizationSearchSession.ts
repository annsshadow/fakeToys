// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/**
 * 组织选择器搜索会话：判定一次异步搜索的结果是否仍可写回当前视图。
 *
 * 切换搜索类型不会发起新搜索（watch(activeType) 只清空列表等用户重新输入），
 * 此时旧类型的请求仍在飞行。若仅用递增序号判重而不校验类型，旧请求返回时
 * 会把旧类型的结果写进新类型视图——用户切到“人员”却看到“组织”的结果。
 */
export interface OrganizationSearchAttempt {
  readonly token: number
  readonly type: string
}

export function createOrganizationSearchSession() {
  let sequence = 0
  let activeType: string | null = null

  return {
    /** 开启一次搜索，返回本次尝试的令牌与类型。 */
    begin(type: string): OrganizationSearchAttempt {
      sequence += 1
      activeType = type
      return { token: sequence, type }
    },

    /** 用户切换到另一种类型（没有发起新搜索，序号不变）。 */
    switchType(type: string): void {
      activeType = type
    },

    /** 结果是否可写入视图：必须既是最新请求、又属于当前选中类型。 */
    shouldApply(attempt: OrganizationSearchAttempt): boolean {
      return attempt.token === sequence && attempt.type === activeType
    },

    /** 由哪次请求负责收起 loading：只看序号，避免切换类型后按钮永久停在“搜索中”。 */
    isLatest(attempt: OrganizationSearchAttempt): boolean {
      return attempt.token === sequence
    },
  }
}
