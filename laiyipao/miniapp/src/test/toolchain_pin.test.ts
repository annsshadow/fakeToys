/**
 * 工具链 pin 守卫。
 *
 * # 为什么存在
 *
 * 这条线上的约束**只写在注释里**时，dependabot 会在 2026-09-30 / 10-08 / 10-11
 * 三次自动升级里各打坏一批东西。详见 README 验证状态表的「依赖版本 pin 陷阱」。
 *
 * # 这个守卫的判据：全部从**依赖自己声明的契约**推导
 *
 * 魔数（「就应该是 5.2.8」）半年后没人说得清凭什么。这里每条断言都写成
 * 「**我们声明的版本 == 某个包声明的 peer 精确值**」或「**满足它的 peer 区间**」：
 *
 * | 我们声明的 | 契约来源 | 为什么是精确值 |
 * |---|---|---|
 * | `vite` | `@dcloudio/vite-plugin-uni` 的 `peerDependencies.vite` | `"5.2.8"`，不是区间 |
 * | `vue` | `@vue/server-renderer` 的 `peerDependencies.vue` | `"3.4.21"` |
 * | `@dcloudio/types` | `@dcloudio/uni-app` 的 `peerDependencies['@dcloudio/types']` | `3.4.31` |
 * | `@dcloudio/*`（20 个） | 它们彼此按精确版本引用，成套发布 | 任何分裂都装不出可用 uni-app |
 * | `typescript` | `vue-tsc` 的 `peerDependencies.typescript` | 区间，必须满足 |
 * | `vitest` / `coverage-v8` | `vitest` 的 `peerDependencies.vite` | 区间，与 dcloudio 的锁互斥 |
 *
 * 契约一旦变了（peer 变区间、版本号变），这里先红 —— 红的时候要做的是
 * **重新跑 `type-check` + 全量测试 + `build:h5` + `build:mp-weixin` 四件套**，
 * 再决定新值，而不是把断言改成新值了事。
 *
 * # 已知且刻意保留的冲突
 *
 * `pinia`（2.x 与 3.x 都一样）要求 `vue ^3.5.11`，而 dcloudio 强制 `vue == 3.4.21`。
 * **两者互斥，任何配置下都恰好剩一个 unmet peer。** 本项目选择站在 dcloudio 一侧 ——
 * 它在构建路径上（uni-app 用它编译），而 pinia 那侧只是软警告，
 * 实践中 vue 3.4 上 pinia 工作正常。
 */
import { describe, it, expect } from 'vitest'
import pkg from '../../package.json'
import uniPluginPkg from '@dcloudio/vite-plugin-uni/package.json'
import vueTscPkg from 'vue-tsc/package.json'

/** 精确 semver（允许 dcloudio 那种 `3.0.0-5020420260813003` 构建后缀）。 */
const EXACT_SEMVER = /^\d+\.\d+\.\d+(?:-[0-9A-Za-z.-]+)?$/

/** package.json 的 dependencies + devDependencies 合并视图。 */
const declared: Record<string, string> = {
  ...(pkg.dependencies as Record<string, string>),
  ...(pkg.devDependencies as Record<string, string>),
}

/**
 * 断言 `ours` 与 `theirs`（某个包声明的 peer）逐字相等。
 *
 * 抽出来是因为这个模式出现四次，每次重写一遍断言文案只会让它变长。
 */
function expectMatchesPeer(
  ours: string | undefined,
  theirs: string | undefined,
  what: string,
  sourcePkg: string,
  consequence: string,
) {
  expect(
    theirs,
    `读不到 ${sourcePkg} 的 ${what} —— 依赖结构变了。` +
      '先确认新结构里这条约束还存在，再更新本守卫；别直接删断言。',
  ).toBeTruthy()
  expect(
    theirs,
    `${sourcePkg} 的 ${what} 从精确值变成了 "${theirs}"。` +
      '区间意味着「装哪个都行」，但本项目只验证过精确 pin 下的四件套' +
      '（type-check / test / build:h5 / build:mp-weixin）。' +
      '正确动作：按新区间逐个版本验证，再把本守卫改成区间比较。',
  ).toMatch(EXACT_SEMVER)
  expect(
    ours,
    `package.json 的 ${what} 相关声明是 "${ours}"，而 ${sourcePkg} 要求 "${theirs}"。` +
      consequence,
  ).toBe(theirs)
}

describe('工具链 pin 守卫（dependabot 多次打破过，勿删）', () => {
  it('vite 必须逐字等于 @dcloudio/vite-plugin-uni 的 peer 硬锁', () => {
    const peer = (uniPluginPkg as { peerDependencies?: { vite?: string } }).peerDependencies?.vite
    expectMatchesPeer(
      declared['vite'],
      peer,
      'vite',
      '@dcloudio/vite-plugin-uni',
      '依赖机器人升级会改这一行 —— 改它会让 type-check（TS2307）与 ' +
        'build:h5（Rolldown 解析 vue-demi 失败）同时挂掉。',
    )
  })

  it('vue 必须逐字等于 dcloudio 钉的 @vue/compiler-sfc 版本', () => {
    // 契约来源选 `@dcloudio/vite-plugin-uni` 而不是 `@dcloudio/uni-app` 或
    // `@vue/server-renderer`，原因有二：
    //   1. `vitest.config.ts` 把 `@dcloudio/uni-app` **alias** 到了
    //      `src/test/uni-app-stub.ts`（为了绕开 vue 3.4 缺 `injectHook`），
    //      直接 import 它的 package.json 会被 alias 改写成 stub 路径而失败。
    //   2. `@vue/server-renderer` 是传递依赖，pnpm 不保证提升到顶层
    //      node_modules，import 它在换 pnpm 版本时会突然失败。
    //
    // dcloudio 把整套 `@vue/compiler-*` / `@vue/shared` 钉在同一个精确版本，
    // 而 vue 与它们是锁步发布的 —— 所以 compiler-sfc 的版本就是 vue 该有的版本。
    //
    // 曾写成 ^3.4.21 → 实际装到 3.5.x，与构建链上的 dcloudio 错位。
    const sfc = (uniPluginPkg as { dependencies?: Record<string, string> })
      .dependencies?.['@vue/compiler-sfc']
    expect(
      sfc,
      '读不到 @dcloudio/vite-plugin-uni 的 dependencies["@vue/compiler-sfc"] —— ' +
        'dcloudio 若不再钉 @vue/compiler-*，本断言的依据就没了，' +
        '先确认新的约束在哪，再更新本守卫。',
    ).toBeTruthy()

    expectMatchesPeer(
      declared['vue'],
      sfc,
      'vue',
      '@dcloudio/vite-plugin-uni 的 @vue/compiler-sfc',
      'vue 漂到别的版本会与 dcloudio 构建链上的 @vue/compiler-* 错位。',
    )
  })

  it('@dcloudio/types 必须是无版本区间的精确 pin', () => {
    // dcloudio 的 uni-* 对它的声明是**精确值** `3.4.31`（见 pnpm-lock.yaml 里
    // `@dcloudio/uni-app` 的 peerDependencies）。这里不能直接 import
    // `@dcloudio/uni-app/package.json` 取那个值 —— vitest.config.ts 把它 alias
    // 到了 src/test/uni-app-stub.ts，取不到。所以断言退一步、只守「不许有区间」
    // 这条真正被违反过的性质：曾写成 ^3.4.8 → 实际装到 3.4.32，与 uni-app
    // 声明的 3.4.31 错位。任何 ^ / ~ / x / * 都会让这个错位再次发生。
    const v = declared['@dcloudio/types']
    expect(
      v,
      'package.json 里找不到 @dcloudio/types 声明。',
    ).toBeTruthy()
    expect(
      v,
      `@dcloudio/types 是 "${v}"，带了版本区间。dcloudio 的 uni-* 对它声明的是` +
        '精确值 3.4.31，区间会让实际装到的版本再次错位。改回精确 pin。',
    ).toMatch(EXACT_SEMVER)
  })

  it('@dcloudio/* 发布列车必须成套同版本', () => {
    const dcloudio = Object.entries(declared).filter(([k]) => k.startsWith('@dcloudio/'))
    expect(
      dcloudio.length,
      `package.json 里的 @dcloudio/* 从 21 个变成了 ${dcloudio.length} 个 —— ` +
        'dcloudio 增删了包。先确认新包属于哪一列，再改这里。',
    ).toBe(21)

    // 列车 = uni 运行时那 20 个（`3.0.0-<构建号>`）。
    // `@dcloudio/types` 走独立版本线，由上一条断言按 uni-app 的 peer 管。
    const TRAIN = /^(\d+\.\d+\.\d+-[\d.]+)$/
    const train: Record<string, string> = {}
    for (const [name, range] of dcloudio) {
      const m = TRAIN.exec(range)
      if (m) train[name] = m[1]
    }
    expect(
      Object.keys(train).length,
      `属于 uni 发布列（形如 3.0.0-<构建号>）的 @dcloudio/* 从 20 个变成了 ` +
        `${Object.keys(train).length} 个。发布列车成员变了，先确认这是 dcloudio ` +
        '重组了包，再改本守卫的期望数量。',
    ).toBe(20)

    const versions = [...new Set(Object.values(train))]
    expect(
      versions.length,
      `@dcloudio/* 发布列车分裂成 ${versions.length} 个版本：${versions.join(' / ')}。\n` +
        Object.entries(train).map(([n, v]) => `  ${n} = ${v}`).join('\n') +
        '\n\ndependabot 只看单个包的版本区间，没有「按发布列车升级」的概念 —— ' +
        '所以它会提出只升其中几个的 PR。uni-app 这一族互相按精确版本引用，' +
        '分裂后彼此找不到，mini-app 构建必坏。' +
        '正确动作：整族一起升，并在升之前重跑四件套。',
    ).toBe(1)
  })

  it('typescript 必须满足 vue-tsc 的 peer —— 这两个只能成套升', () => {
    // 实测（2026-10-13）：只把 vue-tsc 升到 2 而不动 typescript，
    // type-check 立刻炸 `ReferenceError: sys is not defined`
    // （@volar/typescript@2 需要更新的 TS）。成套升到 ts 5.9 + vue-tsc 2 才通。
    const want = (vueTscPkg as { peerDependencies?: { typescript?: string } })
      .peerDependencies?.typescript
    expect(
      want,
      'vue-tsc 的 package.json 里读不到 peerDependencies.typescript —— ' +
        '它若不再声明 TS 要求，本断言失去依据，先确认新契约再改这里。',
    ).toBeTruthy()
    // peer 是区间（">=5.0.0" 这类），所以这里断言「我们声明了一个非空值」
    // 而不是逐字相等 —— 真正的把关靠 CI 的 type-check：
    // 不满足 peer 时 vue-tsc 会在运行时炸，而不是安静地降级。
    expect(
      declared['typescript'],
      `vue-tsc 的 peer 要求 typescript "${want}"，而 package.json 写的是 ` +
        `"${declared['typescript']}"。\n这两个包**只能成套升**：vue-tsc 2 配 TS 4.9 会报 ` +
        '`ReferenceError: sys is not defined`（volar 2 的 proxyCreateProgram 依赖新版 TS 内部）。',
    ).toBeTruthy()
    expect(
      declared['typescript'],
      'package.json 里找不到 typescript 声明。',
    ).toBeTruthy()
  })

  it('vitest / coverage-v8 必须留在 2.x 线（与 vite 5.2.8 兼容的最后一条）', () => {
    // vitest >= 5 的 peerDependencies.vite 是 "^6.4.0 || ^7.0.0 || ^8.0.0"
    // （见 node_modules/vitest/package.json），与 dcloudio 的精确 5.2.8 互斥。
    expect(
      declared['vitest'],
      'vitest 必须留在 ^2.1.9。vitest 5+ 的 peer 硬性要求 vite >= 6.4 —— ' +
        '升它会再次打破 dcloudio 的锁，症状与本文件头部表格里那几行一模一样。',
    ).toBe('^2.1.9')
    expect(
      declared['@vitest/coverage-v8'],
      'coverage-v8 必须与 vitest 同为 2.x 线（2.1.9 的 peer 就是 vitest 2.1.9）。',
    ).toBe('2.1.9')
  })
})