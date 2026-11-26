// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// R302 DR-06-R：流程原始 ID/实例键与 IM 机器会话键只用于检索追踪，不进入用户可见主标题。

/** 原始流程 ID/实例键（work-/task-/UUID/start: 等）。 */
export function isRawProcessKey(value: string): boolean {
  return /^start:/i.test(value) || /^(?:s3-process-|work-|work-reject-|probe-w-|task-|process-|node-|job-|snap-|activity-|reply-|data-)/i.test(value) || /^[0-9a-f][0-9a-f-]{15,}$/i.test(value)
}

/** IM 机器生成的会话键（im-conv-<ts>-<rand> / UUID），不含真人姓名。 */
export function isRawConversationName(name: string): boolean {
  return /^im-conv-[0-9]+-[a-z0-9]+$/i.test(name) || /^[0-9a-f][0-9a-f-]{15,}$/i.test(name)
}

/** 长键仅在副文本折叠显示，中间省略，保留首尾用于追踪。 */
export function shortId(value?: string): string {
  const raw = String(value ?? '')
  if (!raw) return ''
  return raw.length > 24 ? `${raw.slice(0, 12)}…${raw.slice(-8)}` : raw
}

/** 待办主标题：业务名优先；原始键不占据主视觉，回退为可读类型名。 */
export function displayTodoTitle(title?: string, processName?: string, fallback = '新审批'): string {
  const candidates = [title, processName].map((v) => String(v ?? '').trim()).filter(Boolean)
  const readable = candidates.find((v) => !isRawProcessKey(v))
  if (readable) return readable
  const raw = candidates[0] ?? ''
  if (raw.startsWith('work-reject-')) return '退回工作'
  if (raw.startsWith('work-')) return '工作实例'
  if (raw.startsWith('start:')) return '运行闭环'
  return raw ? '流程任务' : fallback
}

/** 原始键副文本：仅在标题字段本身是机器键时展示「流程 · 尾号」。 */
export function foldRawKeySubtext(title?: string): string {
  const raw = String(title ?? '').trim()
  return isRawProcessKey(raw) ? `流程 · ${shortId(raw)}` : ''
}

/** 会话稳定序号：从 id 稳定派生，避免把随机尾号当姓名/主标题。 */
export function conversationOrdinal(id?: string): number {
  const raw = String(id ?? '')
  let hash = 0
  for (let i = 0; i < raw.length; i += 1) hash = (hash * 31 + raw.charCodeAt(i)) >>> 0
  return (hash % 99) + 1
}

/** 机器会话名：类型 + 稳定序号，不能暴露随机尾号。 */
export function displayConversationName(name?: string, type?: string, id?: string): string {
  const raw = String(name ?? '').trim()
  if (!raw) return '未命名会话'
  if (!isRawConversationName(raw)) return raw
  return `${type === 'group' ? '群聊' : '单聊'} ${conversationOrdinal(id)}`
}

/** 会话摘要：真实 lastMessage 优先；只有原始键才引导开始对话。 */
export function displayConversationPreview(lastMessage?: string): string {
  const raw = String(lastMessage ?? '').trim()
  if (!raw || isRawConversationName(raw)) return '暂无消息，点击开始对话'
  return raw
}
