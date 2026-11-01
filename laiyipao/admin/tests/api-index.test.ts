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
  adminLogout,
  fetchDashboardSoft,
  fetchUserDetail,
  verifyBattle,
  fetchRecomputes,
  fetchRecomputeStats,
  fetchRecomputeMode,
  setRecomputeMode,
  fetchReactions,
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
    // 2026-10-21（P3-6）：重建全部 100 关必须带确认口令 ——
    // 这是最重的写操作（覆盖运营手工调过的所有关卡值，且不可回滚）。
    await expect(regenerateLevels()).resolves.toEqual({
      __args: ['/admin/levels/regenerate', { confirm: 'regenerate' }],
    })
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
    // 2026-10-21（P2-6）：防线列表带 offset ——
    // 此前只能看前 100 条，而 total 是全量 COUNT，
    // 于是「第 N 条防线」不可达（漏掉的还是弱防线，按战力倒序）。
    await expect(fetchDefenses()).resolves.toEqual({
      __args: ['/admin/defenses?limit=50&offset=0'],
    })
    await expect(fetchDefenses(100)).resolves.toEqual({
      __args: ['/admin/defenses?limit=100&offset=0'],
    })
    await expect(fetchDefenses(50, 100)).resolves.toEqual({
      __args: ['/admin/defenses?limit=50&offset=100'],
    })
  })
})

describe('api/index 经济与运营内容', () => {
  it('fetchEconomy / updateShopItem', async () => {
    echoArgs()
    await expect(fetchEconomy()).resolves.toEqual({ __args: ['/admin/economy'] })
    // ⚠️ price 是「货币 → 数量」的 map（服务端 JSONB），不是数字 ——
    // 发数字过去服务端会整单拒绝。
    await expect(updateShopItem(4, { price: { coin: 120 } })).resolves.toEqual({
      __args: ['/admin/shop/4', { price: { coin: 120 } }],
    })
  })

  it('公告：列表 + 创建', async () => {
    echoArgs()
    // 2026-10-21（P2-6）：公告列表带 offset（此前更早的公告不可达）。
    await expect(fetchAnnouncements()).resolves.toEqual({
      __args: ['/admin/announcements?limit=50&offset=0'],
    })
    await expect(fetchAnnouncements(50, 50)).resolves.toEqual({
      __args: ['/admin/announcements?limit=50&offset=50'],
    })
    const payload = { title: 'T', body: 'B', published: true }
    await expect(createAnnouncement(payload)).resolves.toEqual({
      __args: ['/admin/announcements', payload],
    })
  })

  it('兑换码：列表 + 创建（过期时间随 payload 透传）', async () => {
    echoArgs()
    // 2026-10-21（P2-6）：兑换码列表带 offset。
    await expect(fetchRedeemCodes()).resolves.toEqual({
      __args: ['/admin/redeem-codes?limit=50&offset=0'],
    })
    const payload = { code: 'LYP-1', reward: { coin: 100 }, max_uses: 5, expires_at: '2026-12-31T00:00:00Z' }
    await expect(createRedeemCode(payload)).resolves.toEqual({
      __args: ['/admin/redeem-codes', payload],
    })
  })

  it('fetchAuditLogs：默认 limit=100，可覆盖', async () => {
    echoArgs()
    // 2026-10-21（P5-1）：审计日志带 offset（此前只能看最新 100 条）。
  await expect(fetchAuditLogs()).resolves.toEqual({
    __args: ['/admin/audit-logs?limit=100&offset=0'],
  })
    await expect(fetchAuditLogs(7)).resolves.toEqual({
      __args: ['/admin/audit-logs?limit=7&offset=0'],
    })
    // 翻页
    await expect(fetchAuditLogs(7, 14)).resolves.toEqual({
      __args: ['/admin/audit-logs?limit=7&offset=14'],
    })
  })
})

/**
 * 补测：此前 `api/index.ts` 有 9 个导出函数没有任何用例（覆盖率 66%），
 * 恰好是「破坏性操作」与「带默认值的分页参数」两类 —— 也正是路径拼错、
 * 默认值写反后最不容易被发现的地方。
 */
describe('api/index 会话与看板（补测）', () => {
  it('adminLogout：POST /admin/logout（服务端吊销全部会话，不只是清本地）', async () => {
    echoArgs()
    await expect(adminLogout()).resolves.toEqual({ __args: ['/admin/logout'] })
    expect(apiMock.post).toHaveBeenCalledWith('/admin/logout')
    expect(apiMock.get).not.toHaveBeenCalled()
  })

  it('fetchDashboardSoft：GET /admin/dashboard 但带 soft 标记（401 不跳登录）', async () => {
    echoArgs()
    await expect(fetchDashboardSoft()).resolves.toEqual({
      __args: ['/admin/dashboard', { soft: true }],
    })
  })
})

describe('api/index 用户与战斗（补测）', () => {
  it('fetchUserDetail：GET /admin/users/{id}', async () => {
    echoArgs()
    await expect(fetchUserDetail(42)).resolves.toEqual({ __args: ['/admin/users/42'] })
  })

  it('verifyBattle：POST /admin/battles/{id}/verify，body 是 snake_case 的 replay_hash', async () => {
    // 字段名写错（replayHash）服务端会当成缺参 → 校验恒失败，但前端不会报错
    echoArgs()
    await expect(verifyBattle(9, 'abc123')).resolves.toEqual({
      __args: ['/admin/battles/9/verify', { replay_hash: 'abc123' }],
    })
  })
})

describe('api/index 结算重算（补测）', () => {
  it('fetchRecomputes：无条件时只带默认 limit=50', async () => {
    echoArgs()
    await expect(fetchRecomputes()).resolves.toEqual({
      __args: ['/admin/recomputes?limit=50'],
    })
  })

  it('fetchRecomputes：全条件拼接，only_mismatch 用 1/不出现表达', async () => {
    echoArgs()
    await expect(
      fetchRecomputes({ user_id: 7, level_id: 3, limit: 10, offset: 20, only_mismatch: true }),
    ).resolves.toEqual({
      __args: ['/admin/recomputes?user_id=7&level_id=3&only_mismatch=1&limit=10&offset=20'],
    })
  })

  it('fetchRecomputes：only_mismatch=false 不得出现在 query 里', async () => {
    // 传 false 却拼出 only_mismatch=0 会让「只看不符」的语义反转
    echoArgs()
    await fetchRecomputes({ only_mismatch: false })
    expect(apiMock.get).toHaveBeenCalledWith('/admin/recomputes?limit=50')
  })

  it('fetchRecomputeStats：hours 缺省 24，可覆盖', async () => {
    echoArgs()
    await expect(fetchRecomputeStats()).resolves.toEqual({
      __args: ['/admin/recomputes/stats?hours=24'],
    })
    await expect(fetchRecomputeStats(72)).resolves.toEqual({
      __args: ['/admin/recomputes/stats?hours=72'],
    })
  })

  it('fetchRecomputeMode：GET /admin/recomputes/mode', async () => {
    echoArgs()
    await expect(fetchRecomputeMode()).resolves.toEqual({
      __args: ['/admin/recomputes/mode'],
    })
  })

  it('setRecomputeMode：PUT 且必须带 confirm=confirm（防手滑关掉反作弊）', async () => {
    echoArgs()
    await expect(setRecomputeMode('enforce')).resolves.toEqual({
      __args: ['/admin/recomputes/mode', { mode: 'enforce', confirm: 'confirm' }],
    })
    expect(apiMock.put).toHaveBeenCalled()
  })
})

describe('api/index 反应表（补测）', () => {
  it('fetchReactions：GET /admin/reactions 并解包出 reactions 字段', async () => {
    // 与其它函数不同，这里做了 .then 解包：返回的是数组而不是信封，
    // 视图直接 v-for 它 —— 解包写错会让整张图空白。
    apiMock.get.mockResolvedValue({ reactions: [{ key: 'fire', name: '火' }] } as never)
    await expect(fetchReactions()).resolves.toEqual([{ key: 'fire', name: '火' }])
    expect(apiMock.get).toHaveBeenCalledWith('/admin/reactions')
  })
})
