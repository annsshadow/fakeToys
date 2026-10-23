// @vitest-environment jsdom
/**
 * 音效模块（audio/sfx.ts）的行为断言（2026-10-23）。
 *
 * 这一组不是「跑通不报错」，而是把四条设计约束各钉一条：
 *
 *   1. 未解锁不播放、**也不排队**（排队会在首次点击后炸出一串音）；
 *   2. 解锁必须在**手势里真的播一次**（H5/iOS 的 autoplay 策略）；
 *   3. 每种音效只建一个 context 并复用（每次播放都建会持续产生 native 对象，
 *      且微信对并发 context 有上限）；
 *   4. 静音持久化（关了就一直是关的）。
 *
 * 为什么值得单独写：音效的失效方式是**完全静默的** —— 没有报错、没有视觉变化，
 * 只有「没有声音」。除了这些断言，没有别的东西会发现它。
 */
import { describe, it, expect, beforeEach } from 'vitest'
import { installUniMock } from '../test/uni-mock'
import {
  initSfx,
  unlockSfx,
  playSfx,
  setMuted,
  isMuted,
  isSfxUnlocked,
  destroySfx,
  assetPrefixFor,
  __resetSfxForTest,
} from './sfx'

let um: ReturnType<typeof installUniMock>

beforeEach(() => {
  um = installUniMock()
  __resetSfxForTest()
})

/** 按 src 统计已创建的 context（unlock 会先建一个 tap，不能直接数总数）。 */
function ctxsWithSrc(fragment: string) {
  return um.audioInstances.filter((a) => a.src.includes(fragment))
}

describe('audio/sfx', () => {
  it('未解锁 → 播放不创建任何 context，也不报错', () => {
    initSfx()
    expect(isSfxUnlocked()).toBe(false)
    playSfx('shot')
    playSfx('hit')
    expect(um.mock.createInnerAudioContext).not.toHaveBeenCalled()
  })

  it('解锁 → 在手势里真的播一次 0 音量的 tap（否则 H5/iOS 永远静音）', () => {
    unlockSfx()
    const tap = ctxsWithSrc('tap.wav')
    expect(tap.length).toBe(1)
    // 关键：不是「置个标志」，而是真的调了 play
    expect(tap[0]!.play).toHaveBeenCalledTimes(1)
    expect(tap[0]!.stop).toHaveBeenCalledTimes(1)
    // 播完音量必须恢复，否则 tap 从此变成静音
    expect(tap[0]!.volume).toBe(0.6)
  })

  it('重复 unlock 只 prime 一次（每次进战斗不该重复播 tap）', () => {
    unlockSfx()
    unlockSfx()
    unlockSfx()
    expect(ctxsWithSrc('tap.wav').length).toBe(1)
    expect(ctxsWithSrc('tap.wav')[0]!.play).toHaveBeenCalledTimes(1)
  })

  it('同一音效播 5 次只建 1 个 context（复用，不是每次播放都建）', () => {
    unlockSfx()
    for (let i = 0; i < 5; i++) playSfx('shot')
    const shots = ctxsWithSrc('shot.wav')
    expect(shots.length).toBe(1)
    // 每次播放都真的 play 了（复用不等于只响一次）
    expect(shots[0]!.play).toHaveBeenCalledTimes(5)
  })

  it('不同音效各自建 context，src 指向包内 wav', () => {
    unlockSfx()
    playSfx('shot')
    playSfx('win')
    expect(ctxsWithSrc('shot.wav').length).toBe(1)
    expect(ctxsWithSrc('win.wav').length).toBe(1)
    expect(ctxsWithSrc('shot.wav')[0]!.src).toMatch(/\/static\/sfx\/shot\.wav$/)
  })

  it('连射时先 stop 再 play（否则同一 context 会排队，声音越来越滞后）', () => {
    unlockSfx()
    playSfx('shot')
    playSfx('shot')
    const a = ctxsWithSrc('shot.wav')[0]!
    expect(a.stop).toHaveBeenCalledTimes(2)
    expect(a.play).toHaveBeenCalledTimes(2)
  })

  it('静音 → 不新增任何播放（解锁时那次 prime 不算）', () => {
    unlockSfx()
    const before = um.audioInstances.reduce((n, a) => n + a.play.mock.calls.length, 0)
    setMuted(true)
    playSfx('hit')
    playSfx('hit')
    const after = um.audioInstances.reduce((n, a) => n + a.play.mock.calls.length, 0)
    expect(after).toBe(before)
    expect(isMuted()).toBe(true)
    // 静音时也不该顺手创建 context
    expect(ctxsWithSrc('hit.wav').length).toBe(0)
  })

  it('静音持久化：写入 storage，重建实例后仍是静音', () => {
    setMuted(true)
    expect(um.mock.setStorageSync).toHaveBeenCalledWith('lyp_sfx_muted', '1')

    // 模拟「下次进入」：模块状态复位，但 storage 还在
    __resetSfxForTest()
    initSfx()
    expect(isMuted()).toBe(true)
    unlockSfx()
    playSfx('tap')
    // 只有 prime 那一次 play，playSfx 没有追加
    expect(ctxsWithSrc('tap.wav')[0]!.play).toHaveBeenCalledTimes(1)
  })

  it('取消静音同样持久化', () => {
    setMuted(true)
    setMuted(false)
    expect(um.mock.setStorageSync).toHaveBeenLastCalledWith('lyp_sfx_muted', '0')
    __resetSfxForTest()
    initSfx()
    expect(isMuted()).toBe(false)
  })

  it('存储读取抛错时退回「有声」，不让战斗起不来', () => {
    um.mock.getStorageSync.mockImplementation(() => {
      throw new Error('storage disabled')
    })
    expect(() => initSfx()).not.toThrow()
    expect(isMuted()).toBe(false)
  })

  it('destroy 释放全部 context 并复位解锁态（新页面要重新手势解锁）', () => {
    unlockSfx()
    playSfx('shot')
    playSfx('kill')
    const instances = [...um.audioInstances]
    expect(instances.length).toBe(3) // tap(prime) + shot + kill

    destroySfx()
    expect(instances.every((a) => a.destroy.mock.calls.length === 1)).toBe(true)
    expect(isSfxUnlocked()).toBe(false)

    // 释放后再播：未解锁 → 不新建
    const before = um.mock.createInnerAudioContext.mock.calls.length
    playSfx('shot')
    expect(um.mock.createInnerAudioContext.mock.calls.length).toBe(before)
  })

  // ── 资源路径（2026-10-23）────────────────────────────────────────
  // 这段逻辑此前是「'./' 归一到 '/'」，于是 H5 部署在子路径时音效 404：
  // JS 与页面都能加载，只有声音去请求了域名根。而 vitest 的 BASE_URL 恒为 '/'，
  // 唯一有分歧的分支**零覆盖**。抽成纯函数后三种取值各断言一次。

  it('assetPrefixFor：根路径 / 相对路径 / 子路径各自原样保留', () => {
    expect(assetPrefixFor('/')).toBe('/')
    // './' 是 uni H5 的默认 base，必须保留相对语义 —— 归一成 '/' 会让子路径部署 404
    expect(assetPrefixFor('./')).toBe('./')
    expect(assetPrefixFor('/game/')).toBe('/game/')
    // 只有空串才回落到根
    expect(assetPrefixFor('')).toBe('/')
  })
})
