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
})
