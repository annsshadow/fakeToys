// @vitest-environment node
/**
 * 跨端「消费面」守卫：服务端下发的每个字段都必须有真实消费方。
 *
 * ## 为什么需要它
 *
 * 本仓库反复吃同一类亏（工程约束 4 / 15）：
 * 一个字段**存在**、类型正确、编译通过、测试全绿，但**没有人读它**。
 * 而症状从来不是报错，而是
// 「某条信息在线上永远看不到」或者「运营改了它但游戏里没反应」。
//
// 第 150 轮实测：`/config` 的 17 个顶层字段里有 5 个零消费
//（`version` / `recipes` / `gem_qualities` / `skins` / `rating_weights`）。
// 修完之后，靠的是「人肉记得看一眼」—— 那是不可持续的。
// 这个文件把那一眼变成可执行检查。
 *
 * ## 口径与已知豁免
 *
 * 判据：一个 `GameConfig` 的 json 字段名，必须出现在
 * `src/` 下（**排除本文件**）的某个非测试源码里，否则报「无消费方」。
 *
 * 豁免必须写进 `KNOWN_UNCONSUMED` 并附理由，且理由必须具体
 *（不能是「暂时没用」这种会永久烂掉的措辞）。
 * 豁免条目在上方常量里，`TestNoSilentUnconsumedField` 会逐条核对。
 *
 * ## 这条守卫查不到什么（别高估它）
 *
 * 它匹配的是**字段名**（`.recipes` / `'recipes'`），不是「哪个结构的哪个字段」。
 * 同一个名字在别处出现一次就能满足它 —— 例如 `/me` 的 `user_id`
 * 与排行榜条目的 `user_id` 同名，把前者的消费点删掉、只要后者还在，它照样绿。
 *
 * 它能抓住的是「**整个字段名在全仓库消失**」这一类真缺口
 * （第 150 轮实测：`version` / `recipes` / `gem_qualities` / `skins` /
 * `rating_weights` 五个都是这样漏掉的），抓不到「读错结构」的那一类。
 * 后者要靠类型系统与各自的业务断言 —— 那也是为什么本文件之外
 * 仍有大量按行为写的测试。
 */
import { describe, it, expect } from 'vitest'
import { readFileSync, readdirSync, statSync } from 'node:fs'
import { join, relative } from 'node:path'

const SRC = join(__dirname, '..')

/** `/config` 的顶层字段（与 server/internal/service.GameConfig 的 json tag 一一对应）。 */
const CONFIG_FIELDS = [
  'version',
  'levels',
  'enemies',
  'skills',
  'composite_skills',
  'recipes',
  'equipment',
  'gems',
  'gem_qualities',
  'skins',
  'mastery_families',
  'reactions',
  'chapters',
  'rating_weights',
  'score_rules',
  'skill_rules',
  'server_time',
] as const

/**
 * 登录态与个人资料端点的字段（`TokenPair` 与 `/me` 的 json 字段）。
 *
 * 与 `CONFIG_FIELDS` 同一条纪律，但覆盖另一类契约 ——
 * 这些字段的共同特点是「**只影响身份**」：
 * 一个没人读的 `user_id` 会让排行榜不再高亮当前玩家，
 * 一个没人读的 `expires_at` 会让每次息屏回来都白吃一次 401。
 * 症状都不是报错，而是某个功能**静默失效**。
 */
const AUTH_FIELDS = [
  'access_token',
  'refresh_token',
  // 第 150 轮：此前零消费，现已用于「发请求前主动续期」
  'expires_at',
  'nickname',
  'is_guest',
  'user_id',
  'build_rating',
  'power',
  'avatar_url',
] as const

/**
 * 允许「零消费」的字段及其**具体**理由。
 *
 * 空对象 = 全部字段都必须有消费方。
 * 新增条目时必须回答：为什么它在服务端存在却不能被消费？
 * 一个说不出具体理由的条目等于把缺口永久合法化。
 */
const KNOWN_UNCONSUMED: Record<string, string> = {
  equipment:
    '装备目录：2026-10-21（P4-1）起，小程序端从 `/me/equipment` 读装备 —— ' +
    '那个端点一次返回「目录 + 拥有状态 + 装配状态」，而 /config.equipment ' +
    '只有目录。从 /config 再读一遍同一份目录是重复传输。' +
    '该字段的真实消费方是**后台的玩家文档站**（admin 的 WikiView「装备图鉴」页），' +
    '它在另一个包里 —— 本守卫的扫描范围只覆盖 miniapp/src，' +
    '所以这里是「有消费方、但不在本扫描范围内」，不是「没人要」。',
  avatar_url:
    '头像字段：users.avatar_url 只有 DEFAULT（空串，migrations/00001_core.sql:12），' +
    '全仓库没有 UPDATE、没有 INSERT，也没有默认值覆盖 —— 所以它永远是空串。' +
    '首页「我的」页不自动显示它、不缓存它（不落存储 + 不读缓存的字段 = 没人要的字段），' +
    '需要时要另存 + 另读才有存储和读取权。',
}

function walk(dir: string, out: string[] = []): string[] {
  for (const name of readdirSync(dir)) {
    const p = join(dir, name)
    if (statSync(p).isDirectory()) {
      if (name === 'test') continue // 测试基建不算产品消费方
      walk(p, out)
    } else if (/\.(ts|vue)$/.test(name) && !name.includes('.test.')) {
      out.push(p)
    }
  }
  return out
}

/** 全部产品源码的拼接文本（排除测试文件与 src/test 基建）。 */
function productSource(): { text: string; files: string[] } {
  // 排除本文件：它逐字列着全部字段名，若把它算进「消费方」，
  // 这条检查会永远为绿 —— 那是最坏的一种假绿。
  const self = join(__dirname, 'contract_coverage.test.ts')
  const files = walk(SRC).filter((f) => f !== self)
  return { text: files.map((f) => readFileSync(f, 'utf8')).join('\n'), files }
}

describe('跨端消费面（第 150 轮）', () => {
  it('GameConfig 的每个字段都有消费方，或有写明理由的豁免', () => {
    const { text } = productSource()
    const orphans: string[] = []
    for (const f of CONFIG_FIELDS) {
      if (KNOWN_UNCONSUMED[f]) continue
      //
      // ⚠️ 2026-10-21（P6-8）：判据收紧为「**从 config 对象上读**」。
      //
      // 此前是「源码里出现过这个名字就算」——
      // 于是 `reactions` 匹配到了 `settleResult.reactions`（战报结果里的字段），
      // `/config.reactions` 于是**看起来**有人读，而实际上零消费。
      // 这条守卫自己就写下了这个失效形态（见文件头注释），
      // 这次把它修掉。
      //
      // 现在要求：字段是挂在 `config`（或 `cfg`）上的属性。
      const re = new RegExp(`\\b(?:config|cfg)[^\n]{0,60}?\\.${f}\\b`)
      if (!re.test(text)) orphans.push(f)
    }
    expect(
      orphans,
      `这些 /config 字段在全仓库没有任何消费方：\n  ${orphans.join('\n  ')}\n\n` +
        '要么在产品代码里真的读它，要么把「为什么不能读」写进 KNOWN_UNCONSUMED。\n' +
        '「存在但没人读」的字段会让运营改了它却看不到任何反应，且不报任何错。',
    ).toEqual([])
  })

  it('登录态与个人资料端点的每个字段都有消费方，或有写明理由的豁免', () => {
    const { text } = productSource()
    const orphans: string[] = []
    for (const f of AUTH_FIELDS) {
      if (KNOWN_UNCONSUMED[f]) continue
      //
      // ⚠️ 2026-10-21（P6-8）：裸字符串匹配去掉。
      //
      // `'user_id'` 出现在任何地方（哪怕是日志文案）都算「有人读」，
      // 而「有人**读这个字段的值**」才是判据。保留「属性访问」这一种。
      const re = new RegExp(`\\.${f}\\b`)
      if (!re.test(text)) orphans.push(f)
    }
    expect(
      orphans,
      `这些登录态/资料字段在全仓库没有任何消费方：\n  ${orphans.join('\n  ')}\n\n` +
        '身份类字段没人读的症状不是报错，而是功能**静默失效**：' +
        '排行榜不高亮当前玩家、每次息屏回来白吃一次 401。',
    ).toEqual([])
  })

  it('KNOWN_UNCONSUMED 里没有已经有人消费的字段（豁免会过期）', () => {
    const { text } = productSource()
    // ⚠️ 2026-10-21（P6-8）：必须与上面那条**用同一条判据**。
    //
    // 否则两条检查会互相打架：一条按「宽松的名字匹配」说有消费方，
    // 另一条按「结构化匹配」说没有 —— 于是豁免既被要求存在、
    // 又被判为过期，守卫自己卡死。
    const stale = Object.keys(KNOWN_UNCONSUMED).filter((f) => {
      const isConfig = (CONFIG_FIELDS as readonly string[]).includes(f)
      const re = isConfig
        ? new RegExp(`\\b(?:config|cfg)[^\n]{0,60}?\\.${f}\\b`)
        : new RegExp(`\\.${f}\\b`)
      return re.test(text)
    })
    expect(
      stale,
      `这些字段已登记豁免，但代码里已经有人消费它们了：${stale.join(', ')}\n` +
        '过期豁免会让真正的缺口被淹没 —— 请删掉对应条目。',
    ).toEqual([])
  })

  it('豁免条目必须写具体理由，且指向真实文件', () => {
    const { files } = productSource()
    const known: readonly string[] = [...CONFIG_FIELDS, ...AUTH_FIELDS]
    for (const [field, why] of Object.entries(KNOWN_UNCONSUMED)) {
      expect(known, `豁免了不存在的字段 ${field}`).toContain(field)
      expect(
        why.length,
        `字段 ${field} 的豁免理由太短（${why.length} 字）。` +
          '豁免必须能回答「为什么服务端有它而客户端不能读它」，否则它会永久烂在这里。',
      ).toBeGreaterThanOrEqual(20)
      expect(files.length, '产品源码扫描结果为空 —— 守卫本身失效了').toBeGreaterThan(0)
    }
  })

  it('本守卫自身不在扫描范围内（否则它会把所有字段都「消费」掉）', () => {
    const { files } = productSource()
    const scanned = files.map((f) => relative(SRC, f).replace(/\\/g, '/'))
    // 守卫住在 src/test/ 下，而 walk() 跳过 src/test —— 这就是自排除机制。
    // 一旦有人把守卫挪到 src/ 根下，它就会把自己的 CONFIG_FIELDS 常量
    // 算成「消费方」，让整条检查永远为绿（最坏的假绿）。
    expect(
      scanned.some((f) => f.startsWith('test/')),
      `src/test 下的文件不该被计入消费方扫描：${scanned.filter((f) => f.startsWith('test/')).join(', ')}`,
    ).toBe(false)
    expect(
      scanned.some((f) => f.includes('contract_coverage')),
      '本守卫文件出现在扫描结果里 —— 它会把自己的字段常量当成消费方，整条检查将永远为绿',
    ).toBe(false)
    expect(scanned.length, '产品源码扫描结果为空 —— 守卫本身失效了').toBeGreaterThan(0)
  })
})