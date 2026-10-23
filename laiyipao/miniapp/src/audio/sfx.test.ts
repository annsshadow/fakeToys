// @vitest-environment jsdom
/**
 * 音效模块（audio/sfx.ts）的行为断言（2026-10-23）。
 *
 * 这一组不是「跑通不报错」，而是把三条设计约束各钉一条：
 *
 *   1. 未解锁不播放、**也不排队**（排队会在首次点击后炸出一串音）；
 *   2. 每种音效只建一个 context 并复用（每次播放都建会持续产生 native 对象，
 *      且微信对并发 context 有上限）；
 *   3. 静音持久化（关了就一直是关的）。
 *
 * 为什么值得单独写：音效的失效方式是**完全静默的** —— 没有报错、没有视觉变化，
 * 只有"没有声音"。除了这些断言，没有别的东西会发现它。
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
  __resetSfxForTest,
} from './sfx'

let um: ReturnType<typeof installUniMock>

beforeEach(() => {
  um = installUniMock()
  __resetSfxForTest()
})

describe('audio/sfx', () => {
  it('未解锁 → 播放不创建任何 context，也不报错', () => {
    initSfx()
    expect(isSfxUnlocked()).toBe(false)
    playSfx('shot')
    playSfx('hit')
    expect(um.mock.createInnerAudioContext).not.toHaveBeenCalled()
  })

  it('解锁后 → 同一音效播 5 次只建 1 个 context（复用，不是每次播放都建）', () => {
    unlockSfx()
    for (let i = 0; i < 5; i++) playSfx('shot')
    expect(um.mock.createInnerAudioContext).toHaveBeenCalledTimes(1)
    expect(um.audioInstances.length).toBe(1)
    // 每次播放都真的 play 了（复用不等于只响一次）
    expect(um.audioInstances[0]!.play).toHaveBeenCalledTimes(5)
  })

  it('不同音效各自建 context，src 指向包内 wav', () => {
    unlockSfx()
    playSfx('shot')
    playSfx('win')
    expect(um.mock.createInnerAudioContext).toHaveBeenCalledTimes(2)
    const srcs = um.audioInstances.map((a) => a.src)
    expect(srcs).toContain('/static/sfx/shot.wav')
    expect(srcs).toContain('/static/sfx/win.wav')
  })

  it('连射时先 stop 再 play（否则同一 context 会排队，声音越来越滞后）', () => {
    unlockSfx()
    playSfx('shot')
    playSfx('shot')
    const a = um.audioInstances[0]!
    expect(a.stop).toHaveBeenCalledTimes(2)
    expect(a.play).toHaveBeenCalledTimes(2)
  })

  it('静音 → 不调用 play，但仍不重复创建 context', () => {
    unlockSfx()
    setMuted(true)
    playSfx('hit')
    playSfx('hit')
    expect(um.mock.createInnerAudioContext).not.toHaveBeenCalled()
    expect(isMuted()).toBe(true)
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
    expect(um.mock.createInnerAudioContext).not.toHaveBeenCalled()
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
    destroySfx()
    expect(instances.every((a) => a.destroy.mock.calls.length === 1)).toBe(true)
    expect(isSfxUnlocked()).toBe(false)
    // 释放后再播：未解锁 → 不新建
    playSfx('shot')
    expect(um.mock.createInnerAudioContext).toHaveBeenCalledTimes(2)
  })
})
