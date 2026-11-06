// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/**
 * 会话内新消息自动刷新（R140）。
 *
 * 旧实现只在 onLoad 拉一次历史：进入会话后对方发来的消息永远不会出现，
 * 必须退出会话再进来才能看到。这里用「上一次完成后再排下一次」的串行轮询，
 * 慢网下不会堆叠请求；页面隐藏/卸载即停，失败只留可诊断记录不打扰用户。
 */

export const CHAT_POLL_INTERVAL_MS = 5000

/** R144：连续失败后的间隔封顶。断网时不能让 5s 轮询一直打必败请求。 */
export const CHAT_POLL_MAX_INTERVAL_MS = 60_000

/** 轮询间隔下限：误传 0 或负数会变成请求风暴。 */
const MIN_INTERVAL_MS = 1000

/**
 * 连续失败后的下一次轮询间隔：5s → 10s → 20s → 40s → 60s（封顶）。
 * 第 n 次失败对应 `base * 2^(n-1)`；成功一次即由调用方复位为 0。
 */
export function nextPollDelayMs(consecutiveFailures: number, baseIntervalMs = CHAT_POLL_INTERVAL_MS): number {
  const base = Math.max(MIN_INTERVAL_MS, Math.floor(baseIntervalMs))
  if (consecutiveFailures <= 0) return Math.min(CHAT_POLL_MAX_INTERVAL_MS, base)
  return Math.min(CHAT_POLL_MAX_INTERVAL_MS, base * 2 ** (consecutiveFailures - 1))
}

export interface ChatPollHandle {
  stop(): void
}

// R142：可观测计数。页面切换 20 次后必须回到基线，否则就是泄漏。
let activePollers = 0
let activeTimers = 0

export function activeChatLifecycleCounts(): { pollers: number; timers: number } {
  return { pollers: activePollers, timers: activeTimers }
}

export interface ChatPollOptions<T> {
  /** 拉取最新一页消息（后端顺序：新→旧）。 */
  loadLatest: () => Promise<readonly T[]>
  /** 每次成功拉取后回调；页面负责合并与已读。 */
  onLatest: (rows: readonly T[]) => void
  /** 失败时的可诊断记录入口（页面用 console.warn）。第二个参数为连续失败次数。 */
  onError?: (error: unknown, consecutiveFailures: number) => void
  intervalMs?: number
}

export function startChatPolling<T>(options: ChatPollOptions<T>): ChatPollHandle {
  const interval = Math.max(MIN_INTERVAL_MS, Math.floor(options.intervalMs ?? CHAT_POLL_INTERVAL_MS))
  let stopped = false
  let timer: ReturnType<typeof setTimeout> | null = null
  // R144：连续失败计数。任何一次成功都复位，恢复后立刻回到正常节奏。
  let consecutiveFailures = 0
  activePollers += 1

  const tick = async () => {
    try {
      const rows = await options.loadLatest()
      // 停止后丢弃在途结果：页面已经隐藏/卸载，不应再写状态。
      if (!stopped) {
        consecutiveFailures = 0
        options.onLatest(rows)
      }
    } catch (error) {
      consecutiveFailures += 1
      if (!stopped) options.onError?.(error, consecutiveFailures)
    }
    schedule()
  }

  function schedule() {
    if (stopped) return
    activeTimers += 1
    timer = setTimeout(
      () => {
        activeTimers -= 1
        timer = null
        void tick()
      },
      nextPollDelayMs(consecutiveFailures, interval),
    )
  }

  schedule()

  return {
    stop() {
      if (stopped) return
      stopped = true
      if (timer !== null) {
        clearTimeout(timer)
        activeTimers -= 1
        timer = null
      }
      activePollers -= 1
    },
  }
}

function messageKey(message: unknown): string {
  const row = message as { sender?: unknown; content?: unknown }
  const sender = typeof row.sender === 'string' ? row.sender : ''
  const content = typeof row.content === 'string' ? row.content : ''
  return `${sender}\u0000${content}`
}

/**
 * 把最新一页（后端顺序：新→旧）按时间正序追加到当前列表末尾。
 * - 服务器行按 id 去重；
 * - 发送成功的本地占位行（`local-*`）按「发送者 + 内容」被服务器回显一对一替换，
 *   避免轮询把刚发出的消息显示成两条。
 */
export function mergeNewerMessages<T extends { id: string }>(current: readonly T[], latestDesc: readonly T[]): T[] {
  const seen = new Set<string>()
  for (const message of current) {
    if (message?.id) seen.add(message.id)
  }

  const pendingLocals: Array<{ key: string; row: T }> = []
  for (const message of current) {
    if (message?.id?.startsWith('local-')) pendingLocals.push({ key: messageKey(message), row: message })
  }

  const replacedLocals = new Set<T>()
  const append: T[] = []
  for (let i = latestDesc.length - 1; i >= 0; i--) {
    const message = latestDesc[i]
    const id = message?.id
    if (!id || seen.has(id)) continue
    seen.add(id)
    const index = pendingLocals.findIndex((entry) => entry.key === messageKey(message))
    if (index >= 0) {
      replacedLocals.add(pendingLocals[index].row)
      pendingLocals.splice(index, 1)
    }
    append.push(message)
  }

  const kept = replacedLocals.size > 0 ? current.filter((message) => !replacedLocals.has(message)) : [...current]
  return [...kept, ...append]
}
