#!/usr/bin/env node
/**
 * 程序化生成战斗音效（2026-10-23）。
 *
 * ## 为什么是「生成文件」而不是 Web Audio 实时合成
 *
 * 微信小程序**没有 `AudioContext`** —— 只有 `uni.createInnerAudioContext()`，
 * 而它需要一个 `src`（包内文件或网络地址）。纯 Web Audio 合成在 H5 能用、
 * 在小程序端等于**静音**，那正是本仓库反复要消灭的
 * 「看起来实现了，其实没生效」。
 *
 * 生成 WAV 文件则两端同构：H5 与小程序都走 `InnerAudioContext`。
 *
 * ## 为什么不用素材站音效
 *
 * 与「美术 100% Canvas 程序化绘制」同一条理由：不引入第三方素材的授权问题。
 * 这里的每个音都是几个正弦/方波 + 指数包络，代码即素材。
 *
 * ## 确定性
 *
 * 噪声用固定种子的 LCG（不用 `Math.random`）。同样的输入必得同样的字节 ——
 * 否则 `--check` 永远红，而一条经常误报的守卫等于没有守卫。
 *
 * 用法：
 *   node scripts/gen-sfx.mjs            # 生成/覆盖 src/static/sfx/*.wav
 *   node scripts/gen-sfx.mjs --check    # 只比对，过期则退出码 1（CI 用）
 */

import { mkdirSync, readFileSync, writeFileSync, existsSync } from 'node:fs'
import { dirname, join, resolve } from 'node:path'
import { fileURLToPath } from 'node:url'

const HERE = dirname(fileURLToPath(import.meta.url))
const OUT_DIR = resolve(HERE, '../src/static/sfx')

const RATE = 22050 // 采样率。22050 足够覆盖这里最高 ~2kHz 的成分，且体积是 44100 的一半
const BITS = 16
const CHANNELS = 1

// ── 合成基元 ──────────────────────────────────────────────────────────

/** 固定种子 LCG：噪声必须可复现，`Math.random` 会让 --check 永远红。 */
function lcg(seed) {
  let s = seed >>> 0
  return () => {
    s = (Math.imul(s, 1664525) + 1013904223) >>> 0
    return s / 4294967296
  }
}

/** 指数衰减包络。`k` 越大衰减越快。 */
function env(t, dur, k = 6) {
  return Math.exp((-k * t) / dur)
}

/** 单音：freq 从 f0 线性滑到 f1，wave 决定谐波结构。 */
function tone({ dur, f0, f1 = f0, wave = 'sine', gain = 1, k = 6 }) {
  const n = Math.round(dur * RATE)
  const out = new Float32Array(n)
  let phase = 0
  for (let i = 0; i < n; i++) {
    const t = i / RATE
    const p = i / n
    const f = f0 + (f1 - f0) * p
    phase += (2 * Math.PI * f) / RATE
    let v
    switch (wave) {
      case 'square':
        v = Math.sin(phase) >= 0 ? 1 : -1
        break
      case 'saw':
        // 只留前 6 次谐波：纯锯齿在小喇叭上会刺耳，且高频混叠
        v = 0
        for (let h = 1; h <= 6; h++) v += Math.sin(phase * h) / h
        v *= 0.55
        break
      default:
        v = Math.sin(phase)
    }
    out[i] = v * gain * env(t, dur, k)
  }
  return out
}

/** 噪声爆：用于"命中"这类没有音高的打击感。 */
function noise({ dur, gain = 1, k = 30, seed = 12345, lowpass = 0.35 }) {
  const n = Math.round(dur * RATE)
  const out = new Float32Array(n)
  const rnd = lcg(seed)
  let prev = 0
  for (let i = 0; i < n; i++) {
    const t = i / RATE
    const raw = rnd() * 2 - 1
    // 一阶低通：把白噪声压成"闷响"，否则在小喇叭上是刺啦声
    prev = prev + lowpass * (raw - prev)
    out[i] = prev * gain * env(t, dur, k)
  }
  return out
}

/** 把多段串起来（每段自带包络）。 */
function seq(parts) {
  const total = parts.reduce((s, p) => s + p.length, 0)
  const out = new Float32Array(total)
  let at = 0
  for (const p of parts) {
    out.set(p, at)
    at += p.length
  }
  return out
}

/** 两段混合（同时发声）。 */
function mix(a, b) {
  const n = Math.max(a.length, b.length)
  const out = new Float32Array(n)
  for (let i = 0; i < n; i++) out[i] = (a[i] ?? 0) + (b[i] ?? 0)
  return out
}

/** 归一化到 -1..1 并做 3ms 淡入淡出，消除首尾的爆音（click）。 */
function finish(samples) {
  let peak = 0
  for (const v of samples) peak = Math.max(peak, Math.abs(v))
  const g = peak > 0 ? 0.85 / peak : 1
  const n = samples.length
  const fade = Math.round(0.003 * RATE)
  const out = new Float32Array(n)
  for (let i = 0; i < n; i++) {
    let f = 1
    if (i < fade) f = i / fade
    else if (i > n - fade) f = Math.max(0, (n - i) / fade)
    out[i] = samples[i] * g * f
  }
  return out
}

// ── 八个音效 ──────────────────────────────────────────────────────────
//
// 音色设计原则：小程序外放是小喇叭，低频几乎放不出来。
// 所以主音都落在 300~1500Hz，低频只作"厚度"存在。

const SOUNDS = {
  /** 开火：短促、带一点下滑，让"连射"听起来有节奏 */
  shot: () => finish(tone({ dur: 0.055, f0: 660, f1: 330, wave: 'square', gain: 0.5, k: 14 })),

  /** 命中：无音高的闷响。必须短（0.04s），否则密集命中会糊成噪声墙 */
  hit: () => finish(noise({ dur: 0.04, gain: 0.9, k: 26, seed: 0x51ed, lowpass: 0.28 })),

  /** 元素反应：上行两音，比普通命中更"亮"，玩家能靠听觉分辨打出了反应 */
  reaction: () =>
    finish(
      seq([
        tone({ dur: 0.06, f0: 520, f1: 620, wave: 'sine', gain: 0.55, k: 8 }),
        tone({ dur: 0.09, f0: 780, f1: 1040, wave: 'sine', gain: 0.5, k: 6 }),
      ]),
    ),

  /** 击杀：下滑的锯齿，收得干净 */
  kill: () => finish(tone({ dur: 0.085, f0: 420, f1: 180, wave: 'saw', gain: 0.55, k: 9 })),

  /** 漏怪：低沉下滑 + 噪声，明确是"坏事" */
  leak: () =>
    finish(
      mix(
        tone({ dur: 0.24, f0: 240, f1: 110, wave: 'sine', gain: 0.8, k: 4.5 }),
        noise({ dur: 0.24, gain: 0.18, k: 7, seed: 0x1ea5, lowpass: 0.12 }),
      ),
    ),

  /** 胜利：大三和弦上行（C-E-G），是游戏里唯一"明亮"的音 */
  win: () =>
    finish(
      seq([
        tone({ dur: 0.1, f0: 523, wave: 'sine', gain: 0.6, k: 3 }),
        tone({ dur: 0.1, f0: 659, wave: 'sine', gain: 0.6, k: 3 }),
        tone({ dur: 0.22, f0: 784, wave: 'sine', gain: 0.7, k: 2.6 }),
      ]),
    ),

  /** 失败：小三度下行，不带噪声（噪声会让"输"听起来像故障） */
  lose: () =>
    finish(
      seq([
        tone({ dur: 0.16, f0: 392, wave: 'sine', gain: 0.65, k: 2.4 }),
        tone({ dur: 0.34, f0: 294, f1: 262, wave: 'sine', gain: 0.7, k: 2.2 }),
      ]),
    ),

  /** UI 点击：极短，只为确认"点到了" */
  tap: () => finish(tone({ dur: 0.022, f0: 1200, f1: 900, wave: 'sine', gain: 0.45, k: 22 })),
}

// ── WAV 封装 ──────────────────────────────────────────────────────────

function toWav(samples) {
  const n = samples.length
  const dataBytes = n * CHANNELS * (BITS / 8)
  const buf = Buffer.alloc(44 + dataBytes)

  buf.write('RIFF', 0, 'ascii')
  buf.writeUInt32LE(36 + dataBytes, 4)
  buf.write('WAVE', 8, 'ascii')
  buf.write('fmt ', 12, 'ascii')
  buf.writeUInt32LE(16, 16) // fmt chunk size
  buf.writeUInt16LE(1, 20) // PCM
  buf.writeUInt16LE(CHANNELS, 22)
  buf.writeUInt32LE(RATE, 24)
  buf.writeUInt32LE(RATE * CHANNELS * (BITS / 8), 28) // byte rate
  buf.writeUInt16LE(CHANNELS * (BITS / 8), 32) // block align
  buf.writeUInt16LE(BITS, 34)
  buf.write('data', 36, 'ascii')
  buf.writeUInt32LE(dataBytes, 40)

  let o = 44
  for (let i = 0; i < n; i++) {
    // 钳位后再转 int16：不钳位时溢出会**回绕**，听感是刺耳的爆音
    const v = Math.max(-1, Math.min(1, samples[i]))
    buf.writeInt16LE(Math.round(v * 32767), o)
    o += 2
  }
  return buf
}

// ── 主流程 ────────────────────────────────────────────────────────────

function main() {
  const check = process.argv.includes('--check')
  mkdirSync(OUT_DIR, { recursive: true })

  const stale = []
  let total = 0
  for (const [name, make] of Object.entries(SOUNDS)) {
    const buf = toWav(make())
    total += buf.length
    const file = join(OUT_DIR, `${name}.wav`)
    if (check) {
      const cur = existsSync(file) ? readFileSync(file) : null
      if (!cur || !cur.equals(buf)) stale.push(name)
    } else {
      writeFileSync(file, buf)
    }
  }

  if (check) {
    if (stale.length) {
      console.error(
        `音效与生成脚本不一致：${stale.join(', ')}\n` +
          `运行 node scripts/gen-sfx.mjs 重新生成（不要手工编辑 wav）。`,
      )
      process.exit(1)
    }
    console.log(`音效已是最新（${Object.keys(SOUNDS).length} 个）`)
    return
  }
  console.log(`已生成 ${Object.keys(SOUNDS).length} 个音效到 ${OUT_DIR}（合计 ${(total / 1024).toFixed(1)} KB）`)
}

main()
