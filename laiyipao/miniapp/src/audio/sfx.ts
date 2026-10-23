/**
 * 战斗音效（2026-10-23）。
 *
 * # 为什么走 `InnerAudioContext` 而不是 Web Audio
 *
 * 微信小程序没有 `AudioContext`，只有 `uni.createInnerAudioContext()`。
 * 用 Web Audio 实时合成在 H5 能响、在小程序端**完全静音** ——
 * 那正是本仓库反复要消灭的「看起来实现了，其实没生效」。
 *
 * 所以音源是 `scripts/gen-sfx.mjs` **程序化生成**的 8 个 WAV（见该脚本注释），
 * 两端同构地播同一个文件。素材不是外部下载的，与「美术 100% 程序化绘制」同源。
 *
 * # 三条设计约束（都有对应断言）
 *
 * 1. **未解锁时静默返回，不排队**。
 *    iOS/微信要求音频在用户手势之后才能播。若把未解锁时的播放**排队**，
 *    首次点击后会一次性炸出一串音（一次战斗可能已经攒了几十次射击）。
 *    判据：未 `unlockSfx()` 前 `playSfx()` 不创建任何 context。
 *
 * 2. **每种音效只建一个 context，复用它**。
 *    每次播放都 `createInnerAudioContext()` 会持续产生 native 对象
 *    （不受 JS GC 控制），且微信对并发 context 数量有上限。
 *    判据：同一音效播 5 次，`createInnerAudioContext` 只调用 1 次。
 *
 * 3. **静音要持久化**。
 *    玩家关了音效，下次进来还得是关的。判据：`setMuted(true)` 后新建
 *    实例 `initSfx()` 仍为静音。
 */

/** 八个音效，与 `scripts/gen-sfx.mjs` 的 SOUNDS 键一一对应。 */
export const SFX_NAMES = ['shot', 'hit', 'reaction', 'kill', 'leak', 'win', 'lose', 'tap'] as const

export type SfxName = (typeof SFX_NAMES)[number]

/** 静音偏好的存储键。与 `lyp_battle_speed` 同一命名风格。 */
const MUTED_KEY = 'lyp_sfx_muted'

type AudioCtx = ReturnType<typeof uni.createInnerAudioContext>

const ctxs = new Map<SfxName, AudioCtx>()
let unlocked = false
let muted = false
let initialized = false

/**
 * 音效文件的 URL 前缀。
 *
 * ⚠️ **不能用 `// #ifdef H5` 条件编译**：`vitest.config.ts` 不加载
 * `@dcloudio/vite-plugin-uni`，两分支会同时生效导致重复声明。
 * 也不能写死 `'/static/...'`：H5 的 `manifest.router.base` 是 `"./"`，
 * 部署在子路径时会 404。
 *
 * 这里的取舍：`BASE_URL` 为 `"./"` 或空时退回 `"/"`。
 * 小程序包内绝对路径 `/static/...` 与 H5 dev 的根路径都正确；
 * H5 若要部署在子路径，需要显式设置 vite 的 `base`（而不是靠 `"./"`）。
 */
function assetPrefix(): string {
  const base = (import.meta as { env?: { BASE_URL?: string } }).env?.BASE_URL ?? '/'
  return base === './' || base === '' ? '/' : base
}

/** 读取持久化的静音偏好。幂等，可重复调用。 */
export function initSfx(): void {
  if (initialized) return
  initialized = true
  try {
    muted = uni.getStorageSync(MUTED_KEY) === '1'
  } catch {
    // 存储不可用（隐私模式 / 老基础库）不该让战斗起不来 —— 退回「有声」
    muted = false
  }
}

/**
 * 解锁音频（必须在**用户手势**的调用链里执行）。
 *
 * 只置标志，不预建 context：微信对并发 `InnerAudioContext` 数量有上限，
 * 一次性建 8 个会挤掉真正要用的那一个。context 在首次播放时按需创建。
 */
export function unlockSfx(): void {
  initSfx()
  unlocked = true
}

/** 音频是否已解锁（供测试与调试）。 */
export function isSfxUnlocked(): boolean {
  return unlocked
}

export function isMuted(): boolean {
  initSfx()
  return muted
}

/**
 * 设置静音并持久化。
 *
 * ⚠️ 只影响**后续**播放，不打断正在响的那一个 ——
 * 在音效中途掐断会留下一个半截的爆音。
 */
export function setMuted(v: boolean): void {
  initSfx()
  muted = v
  try {
    uni.setStorageSync(MUTED_KEY, v ? '1' : '0')
  } catch {
    // 写失败只影响下次进来回到「有声」，不影响本次
  }
}

/** 取（或按需创建）某个音效的 context。 */
function ctxFor(name: SfxName): AudioCtx {
  let a = ctxs.get(name)
  if (!a) {
    a = uni.createInnerAudioContext()
    a.src = `${assetPrefix()}static/sfx/${name}.wav`
    // 音量 0.6：外放时满音量的小程序音效会明显发闷，
    // 而且战斗里同一秒可能有十几次命中叠加。
    a.volume = 0.6
    ctxs.set(name, a)
  }
  return a
}

/**
 * 播放一个音效。
 *
 * 未解锁 / 静音时**静默返回**（不排队、不报错）：
 * 报错会污染控制台且无法处理，排队会在解锁瞬间炸出一串音。
 */
export function playSfx(name: SfxName): void {
  initSfx()
  if (!unlocked || muted) return
  try {
    const a = ctxFor(name)
    // 先 stop 再 play：连射时同一个 context 不 stop 会排队，
    // 结果是「射速越快、声音越滞后」。
    a.stop()
    a.play()
  } catch {
    // 播放失败（部分基础库在无音频设备时抛）不该影响战斗循环
  }
}

/**
 * 释放全部 context 并复位解锁态。
 *
 * ⚠️ **产品代码里刻意不调用它**（只有测试用）。理由：
 *
 * 销毁之后新页面要重新等一次用户手势才能播 —— 而本游戏是**自动战斗**，
 * 玩家完全可能进战斗后一次屏幕都不碰。那时音效就是静音，
 * 而「没有声音」不会有任何报错。
 *
 * 代价是模块级最多留 8 个 native 对象（每种音效一个，复用）。
 * 那是可接受的：它们不随时间增长，且小程序退出时随 JS 运行时一起回收。
 */
export function destroySfx(): void {
  for (const a of ctxs.values()) {
    try {
      a.destroy()
    } catch {
      // 已销毁 / 基础库不支持 destroy —— 忽略
    }
  }
  ctxs.clear()
  unlocked = false
}

/**
 * 仅供测试：把模块状态复位。
 *
 * 模块级单例在用例之间会串味（上一个用例解锁过，下一个用例就测不出
 * 「未解锁不播放」）。导出这个函数比每个用例 `vi.resetModules()` 更明确。
 */
export function __resetSfxForTest(): void {
  ctxs.clear()
  unlocked = false
  muted = false
  initialized = false
}
