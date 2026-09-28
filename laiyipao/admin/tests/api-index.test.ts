/**
 * api/index.ts 测试：验证每个封装函数的「路径 + 参数 + 请求方法 + 透传」契约。
 *
 * 为什么值得测：这些路径与 server/internal/httpapi/routes.go 一一对应，
 * 路径拼错（少个 s、漏 query）只有运行到对应页面才会暴露；query 组装分支
 * （可选过滤条件缺省时不产生 `?undefined`）也是真实出过的 bug 形态。
 * api 底层被整体替身化 —— 被测对象是 index 层的参数组装，不是 HTTP 传输。
 */
import { beforeEach, describe, expect, it, vi } from 'vitest'

const apiMock = vi.hoisted(() => ({
  get: vi.fn(),
  post: vi.fn(),
  put: vi.fn(),
  del: vi.fn(),
}))

vi.mock('@/api/client', () => ({ api: apiMock }))

import {
  adminLogin,
  adminMe,
  banUser,
  createAnnouncement,
  createRedeemCode,
  fetchAnnouncements,
  fetchBattleDetail,
  fetchBattles,
  fetchDashboard,
  fetchDefenses,
  fetchEconomy,
  fetchEquipment,
  fetchLevelWaves,
  fetchLevels,
  fetchRedeemCodes,
  fetchSkills,
  fetchUsers,
  grantUser,
  regenerateLevels,
  unbanUser,
  updateLevel,
  updateShopItem,
  updateSkill,
  fetchAuditLogs,
} from '@/api'

/** 让 mock 返回「收到的参数」本身：一条断言同时验证参数组装与返回值透传 */
function echoArgs() {
  const f = vi.fn((...args: unknown[]) => Promise.resolve({ __args: args }))
  apiMock.get.mockImplementation(f as never)
  apiMock.post.mockImplementation(f as never)
  apiMock.put.mockImplementation(f as never)
  return f
}

beforeEach(() => {
  apiMock.get.mockReset()
  apiMock.post.mockReset()
  apiMock.put.mockReset()
  apiMock.del.mockReset()
})

describe('api/index 认证与看板', () => {
  it('adminLogin：POST /admin/login，携带账号密码并显式免鉴权', async () => {
    const f = echoArgs()
    await expect(adminLogin('admin', 'pw')).resolves.toEqual({
      __args: ['/admin/login', { username: 'admin', password: 'pw' }, { auth: false }],
    })
    expect(f).toHaveBeenCalledTimes(1)
    expect(apiMock.post).toHaveBeenCalled()
    expect(apiMock.get).not.toHaveBeenCalled()
  })

  it('adminMe：GET /admin/me', async () => {
    echoArgs()
    await expect(adminMe()).resolves.toEqual({ __args: ['/admin/me'] })
  })

  it('fetchDashboard：GET /admin/dashboard', async () => {
    echoArgs()
    await expect(fetchDashboard()).resolves.toEqual({ __args: ['/admin/dashboard'] })
  })
})

describe('api/index 关卡', () => {
  it('fetchLevels：无条件时不拼 query（避免出现 ?chapter=undefined）', async () => {
    echoArgs()
    await expect(fetchLevels()).resolves.toEqual({ __args: ['/admin/levels'] })
  })

  it('fetchLevels：章节与关键词都拼接', async () => {
    echoArgs()
    await expect(fetchLevels({ chapter: 3, keyword: '火' })).resolves.toEqual({
      __args: ['/admin/levels?chapter=3&keyword=%E7%81%AB'],
    })
  })

  it('fetchLevels：chapter=0 是合法过滤值吗？——不是，0 按未选处理被丢弃（现有契约）', async () => {
    echoArgs()
    await fetchLevels({ chapter: 0, keyword: '冰' })
    expect(apiMock.get).toHaveBeenCalledWith('/admin/levels?keyword=%E5%86%B0')
  })

  it('updateLevel：PUT 具体关卡，patch 作为 body', async () => {
    echoArgs()
    await expect(updateLevel(7, { base_hp: 500 })).resolves.toEqual({
      __args: ['/admin/levels/7', { base_hp: 500 }],
    })
  })

  it('regenerateLevels：POST 重新生成', async () => {
    echoArgs()
    await expect(regenerateLevels()).resolves.toEqual({ __args: ['/admin/levels/regenerate'] })
  })

  it('fetchLevelWaves：GET 关卡波次', async () => {
    echoArgs()
    await expect(fetchLevelWaves(42)).resolves.toEqual({ __args: ['/admin/levels/42/waves'] })
  })
})

describe('api/index 技能与装备', () => {
  it('fetchSkills：GET /admin/skills', async () => {
    echoArgs()
    await expect(fetchSkills()).resolves.toEqual({ __args: ['/admin/skills'] })
  })

  it('updateSkill：PUT 具体技能', async () => {
    echoArgs()
    await expect(updateSkill(5, { base_damage: 250 })).resolves.toEqual({
      __args: ['/admin/skills/5', { base_damage: 250 }],
    })
  })

  it('fetchEquipment：GET /admin/equipment', async () => {
    echoArgs()
    await expect(fetchEquipment()).resolves.toEqual({ __args: ['/admin/equipment'] })
  })
})

describe('api/index 用户', () => {
  it('fetchUsers：keyword 缺省、limit/offset 使用默认 50/0', async () => {
    echoArgs()
    await expect(fetchUsers()).resolves.toEqual({ __args: ['/admin/users?limit=50&offset=0'] })
  })

  it('fetchUsers：全参数传递；limit=0 / offset=0 是显式值不被默认覆盖（?? 只兜 null/undefined）', async () => {
    echoArgs()
    await fetchUsers({ keyword: '龙', limit: 0, offset: 0 })
    expect(apiMock.get).toHaveBeenCalledWith('/admin/users?keyword=%E9%BE%99&limit=0&offset=0')
  })

  it('fetchUsers：空字符串 keyword 视为未填', async () => {
    echoArgs()
    await fetchUsers({ keyword: '', limit: 10, offset: 30 })
    expect(apiMock.get).toHaveBeenCalledWith('/admin/users?limit=10&offset=30')
  })

  it('banUser / unbanUser / grantUser', async () => {
    echoArgs()
    await expect(banUser(3, '刷分')).resolves.toEqual({
      __args: ['/admin/users/3/ban', { reason: '刷分' }],
    })
    await expect(unbanUser(3)).resolves.toEqual({ __args: ['/admin/users/3/unban'] })
    await expect(grantUser(3, 'coin', 10000)).resolves.toEqual({
      __args: ['/admin/users/3/grant', { currency: 'coin', amount: 10000 }],
    })
  })
})

describe('api/index 战报与防线', () => {
  it('fetchBattles：无过滤时只带默认 limit=50', async () => {
    echoArgs()
    await expect(fetchBattles()).resolves.toEqual({ __args: ['/admin/battles?limit=50'] })
  })

  it('fetchBattles：user_id / level_id 过滤拼接；user_id=0 视为未填', async () => {
    echoArgs()
    await fetchBattles({ user_id: 9, level_id: 12, limit: 5 })
    expect(apiMock.get).toHaveBeenCalledWith('/admin/battles?user_id=9&level_id=12&limit=5')
    await fetchBattles({ user_id: 0, level_id: 0 })
    expect(apiMock.get).toHaveBeenLastCalledWith('/admin/battles?limit=50')
  })

  it('fetchBattleDetail：GET 单条战报', async () => {
    echoArgs()
    await expect(fetchBattleDetail(88)).resolves.toEqual({ __args: ['/admin/battles/88'] })
  })

  it('fetchDefenses：默认 limit=50，可显式覆盖', async () => {
    echoArgs()
    await expect(fetchDefenses()).resolves.toEqual({ __args: ['/admin/defenses?limit=50'] })
    await expect(fetchDefenses(100)).resolves.toEqual({ __args: ['/admin/defenses?limit=100'] })
  })
})

describe('api/index 经济与运营内容', () => {
  it('fetchEconomy / updateShopItem', async () => {
    echoArgs()
    await expect(fetchEconomy()).resolves.toEqual({ __args: ['/admin/economy'] })
    await expect(updateShopItem(4, { price: 120 })).resolves.toEqual({
      __args: ['/admin/shop/4', { price: 120 }],
    })
  })

  it('公告：列表 + 创建', async () => {
    echoArgs()
    await expect(fetchAnnouncements()).resolves.toEqual({ __args: ['/admin/announcements'] })
    const payload = { title: 'T', body: 'B', published: true }
    await expect(createAnnouncement(payload)).resolves.toEqual({
      __args: ['/admin/announcements', payload],
    })
  })

  it('兑换码：列表 + 创建（过期时间随 payload 透传）', async () => {
    echoArgs()
    await expect(fetchRedeemCodes()).resolves.toEqual({ __args: ['/admin/redeem-codes'] })
    const payload = { code: 'LYP-1', reward: { coin: 100 }, max_uses: 5, expires_at: '2026-12-31T00:00:00Z' }
    await expect(createRedeemCode(payload)).resolves.toEqual({
      __args: ['/admin/redeem-codes', payload],
    })
  })

  it('fetchAuditLogs：默认 limit=100，可覆盖', async () => {
    echoArgs()
    await expect(fetchAuditLogs()).resolves.toEqual({ __args: ['/admin/audit-logs?limit=100'] })
    await expect(fetchAuditLogs(7)).resolves.toEqual({ __args: ['/admin/audit-logs?limit=7'] })
  })
})
