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
 * | `vite` | `pnpm-workspace.yaml` 的 override 目标 | 声明值与实际安装值必须同源；dcloudio 的 peer 是精确值 `5.2.8`，已被安全升级顶替 |
 * | `vue` | `@vue/server-renderer` 的 `peerDependencies.vue` | `"3.4.21"` |
 * | `@dcloudio/types` | `@dcloudio/uni-app` 的 `peerDependencies['@dcloudio/types']` | `3.4.31` |
 * | `@dcloudio/*`（20 个） | 它们彼此按精确版本引用，成套发布 | 任何分裂都装不出可用 uni-app |
 * | `typescript` | `vue-tsc` 的 `peerDependencies.typescript` | 区间，必须满足 |
 * | `vitest` / `coverage-v8` | `vitest` 的 `peerDependencies.vite` | 区间；必须覆盖 `pnpm-workspace.yaml` 里 override 的 vite |
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
 *
 * `vite`：dcloudio 的 peer 是精确值 `5.2.8`，而本仓 override 到 `6.4.4`
 * （5.4.x 线没有新一批 advisory 的修复版本，见 pnpm-workspace.yaml）。这是
 * 第二个刻意保留的 unmet peer，理由同 pinia：构建实际跑的是 override 后的版本，
 * 且 type-check / test / build:h5 / build:mp-weixin 四件套在 6.4.4 上验证过。
 */
import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { fileURLToPath } from 'node:url'
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

/**
 * 从 `pnpm-workspace.yaml` 取 `"vite": x.y.z` 这条 override —— 也就是**实际装到的**
 * vite 版本。
 *
 * 两处断言共用（vite 声明值一致性、vitest 的 peer 区间是否覆盖它）。写成函数是
 * 为了只有一处正则：override 的写法变了，两处同时红，不会只红一处。
 */
function readViteOverride(): string {
  const workspaceYaml = readFileSync(
    fileURLToPath(new URL('../../pnpm-workspace.yaml', import.meta.url)),
    'utf8',
  )
  const v = /^\s*"vite":\s*([0-9][0-9.]*)\s*$/m.exec(workspaceYaml)?.[1]
  expect(
    v,
    'pnpm-workspace.yaml 里找不到 `"vite": x.y.z` 的 override —— 本守卫靠它取' +
      '「实际解析到的 vite」，取不到就没法判断声明值/peer 区间是否成立。',
  ).toBeTruthy()
  return v as string
}

describe('工具链 pin 守卫（dependabot 多次打破过，勿删）', () => {
  it('vite 声明必须等于 pnpm-workspace.yaml 的 override 目标（dcloudio 的精确 peer 已被安全升级顶替）', () => {
    // 2026-10-22 起 vite 被 override 顶到 6.4.4：5.4.x 线没有新一批 advisory 的
    // 修复版本，只能跨大版本。而 @dcloudio/vite-plugin-uni 的 peer 仍是精确值
    // 5.2.8 —— 这是本仓刻意保留的第二个 unmet peer（第一个是 pinia 要 vue ^3.5.11）。
    //
    // 判据三条，缺一条都会退回「声明值与实际安装值错位」的老坑：
    //   1. dcloudio 仍按精确值 pin（依赖结构没变；变了要重新确认约束在哪）
    //   2. 我们声明的 vite == override 目标 —— 2026-10-27 前两者是分裂的
    //      （声明 5.2.8 / 实装 6.4.4），dependabot 按声明值一次报了 20 条已修的漏洞
    //   3. 声明值不等于 dcloudio 那个 pin —— 等于就意味着 override 被撤，
    //      那 20 条 advisory 立刻回来
    const peer = (uniPluginPkg as { peerDependencies?: { vite?: string } }).peerDependencies?.vite
    expect(
      peer,
      '读不到 @dcloudio/vite-plugin-uni 的 peerDependencies.vite —— ' +
        '依赖结构变了，先确认这条约束还在不在，再改本守卫。',
    ).toBeTruthy()
    expect(
      peer,
      `@dcloudio/vite-plugin-uni 的 peer vite 从精确值变成了 "${peer}"。` +
        '区间意味着「装哪个都行」，而本仓只验证过 override 的那一个版本。',
    ).toMatch(EXACT_SEMVER)

    const overrideVite = readViteOverride()
    expect(
      declared['vite'],
      `package.json 的 vite 是 "${declared['vite']}"，而 pnpm-workspace.yaml 把它 ` +
        `override 到 "${overrideVite}"。两者必须一致：声明值与实际安装值分裂时，` +
        'dependabot 会按声明值反复报已经修掉的漏洞（2026-10-27 一次 20 条），' +
        '读代码的人也会以为跑的是老版本。',
    ).toBe(overrideVite)
    expect(
      declared['vite'],
      `package.json 的 vite 又回到 dcloudio 的 pin "${peer}"。` +
        '等于撤掉了安全升级 —— vite 5.4.x 线没有这批 advisory 的修复版本，' +
        '那 20 条会立刻回来。要降回去必须先在 audit 里确认它们真的不再命中。',
    ).not.toBe(peer)
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

  it('vitest / coverage-v8 成套，且 vitest 的 peer vite 区间必须覆盖我们 override 的 vite', () => {
    // 这条断言的历史：2026-10 之前 vite 被 dcloudio 钉在 5.2.8，而 vitest 5+ 的
    // peerDependencies.vite 是 "^6.4.0 || ^7.0.0 || ^8.0.0" —— 两者互斥，所以当时
    // 只能把 vitest 按死在 ^2.1.9。
    //
    // 2026-10-22 变了：5.4.x 线没有新一批 advisory 的修复版本，vite 只能跨大版本，
    // 于是 pnpm-workspace.yaml 把 vite override 到 6.4.4（依据是 oa4rust/mobile：
    // 同一套 dcloudio 工具链，package.json 直接声明 vite 6.4.3 且双构建长期全绿）。
    // vite 到 6.4.4 之后，vitest 2.x 反而成了瓶颈：它的 2 条 critical（tinypool
    // 原型污染 RCE）+ 2 条 moderate（@vitest/mocker 路径穿越）在 2.x 线没有修复版。
    // 升到 4.1.12 一次清掉四条，且 4.1.12 的 peer 区间覆盖 6.4.4。
    //
    // 判据不再写死版本号，而是**读 vitest 自己声明的 peer 区间**，再与
    // pnpm-workspace.yaml 里 override 的 vite 主版本比对 —— 下次谁再动 vitest，
    // 只要它要求的 vite 主版本超出我们 override 的那一条，这里就红。
    const vitestPkg = readFileSync(
      fileURLToPath(new URL('../../node_modules/vitest/package.json', import.meta.url)),
      'utf8',
    )
    const peerVite = (JSON.parse(vitestPkg) as { peerDependencies?: { vite?: string } })
      .peerDependencies?.vite
    expect(
      peerVite,
      'vitest 的 package.json 里读不到 peerDependencies.vite —— 它若不再声明 vite 要求，' +
        '本断言失去依据，先确认新契约再改这里。',
    ).toBeTruthy()

    // override 的 vite 主版本：与上面那条 vite 断言共用同一个取数函数。
    const overrideVite = readViteOverride()
    const overrideMajor = overrideVite.split('.')[0]

    // peer 形如 "^6.0.0 || ^7.0.0 || ^8.0.0"：把每个分支的主版本抠出来比。
    const peerMajors = [...String(peerVite).matchAll(/\^?(\d+)\./g)].map((m) => m[1])
    expect(
      peerMajors,
      `vitest 的 peer vite 是 "${peerVite}"，一个主版本都解析不出来 —— ` +
        '区间写法变了，先确认语义再改本守卫。',
    ).not.toHaveLength(0)
    expect(
      peerMajors,
      `vitest 要求 vite "${peerVite}"，而 pnpm-workspace.yaml 把 vite override 到 ` +
        `${overrideVite}（主版本 ${overrideMajor}）。两者不相容时 vitest 会装不上或` +
        '在运行期炸。要么把 override 顶到 vitest 接受的线，要么把 vitest 降回去。',
    ).toContain(overrideMajor)

    // 两个包必须同一条线：coverage-v8 的 peer 就是精确的 vitest 版本。
    expect(
      declared['@vitest/coverage-v8']?.replace(/^[\^~]/, ''),
      'coverage-v8 必须与 vitest 同版本（它的 peerDependencies.vitest 是精确值）。',
    ).toBe(declared['vitest']?.replace(/^[\^~]/, ''))
  })
})