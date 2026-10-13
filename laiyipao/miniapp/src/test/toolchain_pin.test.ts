/**
 * 工具链 pin 守卫（第 148 轮新增）。
 *
 * # 为什么存在
 *
 * `vite.config.ts` 的文件头写着「vite 必须锁定 5.2.8
 * （@dcloudio/vite-plugin-uni 的 peer 硬锁）」—— 但这条约束**只写在注释里**，
 * 没有任何东西在执行它。dependabot 在 2026-09-30 与 2026-10-08 两次自动升级
 * （5.2.8 → 6.4.3 → 8.3.4），每次都把三件事同时打坏：
 *
 * | 症状 | 直接原因 |
 * |---|---|
 * | `pnpm type-check` 报 TS2307 找不到 `vite` | vite 8 只用 `exports` 暴露类型，没有根 `types` 字段；TS 4.9 的 `moduleResolution: Node`（node10）不读 `exports` |
 * | `pnpm build:h5` 失败：Rolldown 无法解析 `vue-demi` | vite 8 换用 rolldown，解析语义与 pinia 2.3.1 的 vue-demi 导入不兼容 |
 * | `npm install` ERESOLVE | dcloudio 的 peer 是**精确值** `5.2.8`，npm 严格执行 peer，pnpm 宽松放过 |
 *
 * 也就是说：README「验证状态」表里「构建 miniapp H5 ✓」这条声明一度是**假的**，
 * 而 `laiyipao-ci.yml` 本来就跑双构建 ——  PR checks 没被设为 required，
 * 破坏被直接合进了 main。
 *
 * # 这个守卫的判据
 *
 * 不是「照样抄一份魔数」——vite 的期望值从 **依赖自己的契约**推导：
 *
 * 1. **结构性**：`@dcloudio/vite-plugin-uni` 的 `peerDependencies.vite` 是精确
 *    版本 `5.2.8`。pin 必须与它**逐字相等**。peer 一旦变成区间或换了版本号，
 *    这里先红 —— 红的时候要做的是重新验证 H5 + mp-weixin 双构建，再决定新 pin，
 *    而不是把断言改成新值了事。
 * 2. **vitest 线**：`vitest >= 5` 的 peer 要求 `vite ^6.4.0 || ^7 || ^8`
 *    （见 node_modules/vitest/package.json），与 dcloudio 的锁**根本矛盾**。
 *    最后一次与 vite 5.2.8 一起验证过的是 `^2.1.9`（coverage-2026-09-28 审计 +
 *    ~147 轮迭代全程使用）。换它也必须重新验证。
 *
 * 判据是「**dependabot 再敢动这两行就红**」，不是「覆盖到了吗」。
 */
import { describe, it, expect } from 'vitest'
import pkg from '../../package.json'
import uniPluginPkg from '@dcloudio/vite-plugin-uni/package.json'

/** 精确 semver：x.y.z，不接受区间/通配符。peer 变成区间时必须人工重新决策。 */
const EXACT_SEMVER = /^\d+\.\d+\.\d+$/

describe('工具链 pin 守卫（dependabot 两次打破过，勿删）', () => {
  it('vite pin 必须逐字等于 @dcloudio/vite-plugin-uni 的 peer 硬锁', () => {
    const peer = (uniPluginPkg as { peerDependencies?: { vite?: string } }).peerDependencies?.vite
    const pinned = pkg.devDependencies?.vite

    expect(
      peer,
      '读不到 @dcloudio/vite-plugin-uni 的 peerDependencies.vite —— ' +
        '依赖结构变了，先确认新结构里这条硬锁还存在，再更新本守卫',
    ).toBeTruthy()

    expect(
      peer,
      `dcloudio 的 peer 从精确版本变成了 "${peer}"。区间意味着「装哪个都行」，` +
        '但本项目只验证过精确 pin 下的双构建。正确动作：重新跑 build:h5 与 ' +
        'build:mp-weixin 确认新区间可用，再把本守卫改成区间比较。',
    ).toMatch(EXACT_SEMVER)

    expect(
      pinned,
      `package.json 的 vite pin 是 "${pinned}"，而 dcloudio 的 peer 硬锁是 "${peer}"。` +
        '依赖机器人的升级 PR 改的是这一行 —— 改它会让 type-check（TS2307）与 ' +
        'build:h5（Rolldown 解析 vue-demi 失败）同时挂掉。',
    ).toBe(peer)
  })

  it('vitest / coverage-v8 必须留在 2.x 线（与 vite 5.2.8 兼容的最后一条）', () => {
    // vitest >= 5 的 peerDependencies.vite 是 "^6.4.0 || ^7.0.0 || ^8.0.0"
    // （见 node_modules/vitest/package.json），与 dcloudio 的精确 5.2.8 互斥。
    // 2.1.9 是与该 vite pin 一同被 100% 覆盖率审计与全部测试验证过的版本。
    expect(
      pkg.devDependencies?.vitest,
      'vitest 必须留在 ^2.1.9。surface 上看它只是个测试运行器，但 vitest 5+ ' +
        '的 peer 硬性要求 vite >= 6.4 —— 升它会再次打破 dcloudio 的锁，' +
        '症状与本守卫头部表格里那三行一模一样。',
    ).toBe('^2.1.9')

    expect(
      pkg.devDependencies?.['@vitest/coverage-v8'],
      'coverage-v8 必须与 vitest 同为 2.x 线（2.1.9 的 peer 就是 vitest 2.1.9）。',
    ).toBe('2.1.9')
  })

  // ── 第 2 轮复查（2026-10-13）新增 ──────────────────────────────────
  //
  // 起因：`git ls-remote --heads origin | grep dependabot` 显示 miniapp
  // 还有 5 个未处理的 dependabot 分支，其中三个是
  //   @dcloudio/uni-mp-{harmony,lark,weixin} → 3.0.0-5020620260917001
  // 而 package.json 里另外 17 个 @dcloudio/* 仍在 3.0.0-5020420260813003。
  //
  // **版本分裂**。这不是「有点风险」，是必然坏：uni-app 这一族是**成套发布**的，
  // 互相按精确版本引用（`@dcloudio/uni-app` 的依赖写死 `@dcloudio/uni-h5@同一个版本号`），
  // 单独抬升其中几个等于让它们互相找不到。dependabot 没有「按发布列车升级」
  // 的概念 —— 它只看单个包的版本区间。
  //
  // 这条守卫的价值在于：**它比 dependabot 配置更根本**。配置只是让机器人别提，
  // 而这条断言保证「即使有人手改了 package.json，只改了一个」也会当场红。
  it('@dcloudio/* 必须成套同版本：分裂的发布列车装不出可用的 uni-app', () => {
    const all: Record<string, string> = {
      ...(pkg.dependencies as Record<string, string>),
      ...(pkg.devDependencies as Record<string, string>),
    }
    const dcloudio = Object.entries(all).filter(([k]) => k.startsWith('@dcloudio/'))

    // 先确认结构没变。数量变了说明 dcloudio 增删了包，本守卫的分组前提要重新确认。
    expect(
      dcloudio.length,
      `package.json 里的 @dcloudio/* 从 21 个变成了 ${dcloudio.length} 个 —— ` +
        'dcloudio 增删了包。本守卫按「成套发布」分组，先确认新包属于哪一列，再改这里。',
    ).toBe(21)

    // 发布列车 = 精确版本。dcloudio 的版本号带构建后缀
    // （`3.0.0-5020420260813003`，预发布标识 + 构建号），所以不能只写
    // `^\d+\.\d+\.\d+$` —— 那会把**全部 20 个**误判成「不在列车上」。
    // 判据是「有没有版本区间符号」：`^` / `~` / `x` / `*` 才算不在列车内。
    const TRAIN = /^(\d+\.\d+\.\d+(?:-[0-9A-Za-z.-]+)?)$/
    const train: Record<string, string> = {}
    const outside: string[] = []
    for (const [name, range] of dcloudio) {
      const m = TRAIN.exec(range)
      if (m) train[name] = m[1]
      else outside.push(`${name}@${range}`)
    }

    // 列车外的包必须显式登记，避免「悄悄多了一批不受约束的包」。
    expect(
      outside,
      `这些 @dcloudio/* 不在精确发布列车上：${outside.join(', ')}。\n` +
        'dcloudio 的发布列车是精确版本；出现区间/通配说明有人给它加了 ^ 或 ~，' +
        '那会让「本守卫只管列车内」这个前提失效。若确认某个包该独立升级，' +
        '把它移进这里显式列出，别让它默认逃出约束。',
    ).toEqual(['@dcloudio/types@^3.4.8'])

    const versions = [...new Set(Object.values(train))]
    expect(
      versions.length,
      `@dcloudio/* 发布列车出现 ${versions.length} 个版本：${versions.join(' / ')}。\n` +
        `分裂的是：\n` +
        Object.entries(train).map(([n, v]) => `  ${n} = ${v}`).join('\n') +
        '\n\ndependabot 只看单个包的版本区间，没有「按发布列车升级」的概念 —— ' +
        '所以它会提出只升其中几个的 PR。uni-app 这一族互相按精确版本引用，' +
        '版本分裂后彼此找不到，mini-app 构建必坏。正确动作：整族一起升，' +
        '并在升之前重跑 build:h5 + build:mp-weixin 双构建。',
    ).toBe(1)
  })
})
