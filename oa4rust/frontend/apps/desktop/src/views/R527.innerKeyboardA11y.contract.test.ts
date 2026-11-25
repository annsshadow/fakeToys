// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R527 内层按钮键盘冒泡劫持修复契约（WCAG 2.1 AA / 2.1.1 Keyboard）。
 *
 * R526 甄别判定：MeetingApp.vue 外层 <li role="button" tabindex="0"> 的
 * @keydown.enter / @keydown.space.prevent=viewMeeting 冒泡截获内层七个原生
 * button 的 Enter/Space，键盘用户无法激活任一内层按钮自身动作（2.1.1），
 * Space 反而打开详情 modal。修复给内层七按钮各加 @keydown.enter.stop 与
 * @keydown.space.stop（或父处理器加 target 守卫），阻止 keydown 冒泡，
 * 保留原生 button 键盘激活自身 click。
 *
 * 本测试读 .vue 模板源码：核心断言验证七按钮均带 keydown.stop（或父处理器
 * 带 target 守卫），真父 RED fail、修复后 GREEN pass；守卫断言锁定 @click.stop、
 * 外层 role/tabindex、keydown=viewMeeting、业务函数、日历对照与样式零改，
 * RED 与 GREEN 均须 pass，确保修复不越界改动业务。
 */
import { readFileSync } from 'node:fs'
import path from 'node:path'
import { fileURLToPath } from 'node:url'
import { describe, expect, it } from 'vitest'

const here = path.dirname(fileURLToPath(import.meta.url))
const frontendRoot = path.resolve(here, '../../../..')

function vue(rel: string): string {
  return readFileSync(path.join(frontendRoot, rel), 'utf8')
}

/** 取 li.mc 外层卡片（从 role="button" 的 <li> 起到 </li>）。 */
function cardOuter(source: string): string {
  const at = source.indexOf('<li v-for="m in meetings"')
  expect(at, '未找到会议卡片 <li v-for="m in meetings"').toBeGreaterThan(-1)
  const end = source.indexOf('</li>', at)
  expect(end, '会议卡片无 </li>').toBeGreaterThan(-1)
  return source.slice(at, end + '</li>'.length)
}

/** 取七内层 <button class="bsm">...</button>（不含外层 li）。 */
function innerButtons(source: string): string[] {
  const re = /<button class="bsm"[^>]*>[\s\S]*?<\/button>/g
  const out: string[] = []
  let mm: RegExpExecArray | null
  while ((mm = re.exec(source)) !== null) out.push(mm[0])
  return out
}

describe('R527 内层按钮键盘冒泡劫持修复契约（WCAG 2.1 AA / 2.1.1）', () => {
  const meeting = vue('apps/desktop/src/views/MeetingApp.vue')

  // ---- 核心断言：修复前 RED fail，修复后 GREEN pass ----
  it('七个内层按钮均阻止 Enter/Space keydown 冒泡（或父处理器带 target 守卫）', () => {
    const buttons = innerButtons(meeting)
    expect(buttons.length, '应有七个内层 button.bsm').toBe(7)
    const parentGuarded = /@keydown\.(?:enter|space)(?:\.prevent)?="[^"]*ev\.target\s*===\s*ev\.currentTarget/.test(cardOuter(meeting))
    if (parentGuarded) return
    for (const b of buttons) {
      expect(b, '内层按钮缺 @keydown.enter.stop: ' + b).toMatch(/@keydown\.enter\.stop/)
      expect(b, '内层按钮缺 @keydown.space.stop: ' + b).toMatch(/@keydown\.space\.stop/)
    }
  })

  // ---- 守卫断言：RED 与 GREEN 均须 pass，锁定业务零改 ----
  it('七按钮 @click.stop 处理器逐字零改', () => {
    const handlers = ['joinMeeting(m)', 'showParticipants(m)', 'inviteParticipant(m)', 'manageInvitee(m)', 'modifyMeetingTime(m)', 'markMeetingCompleted(m)', 'showCheckinCode(m)']
    const buttons = innerButtons(meeting)
    expect(buttons.length).toBe(7)
    handlers.forEach((h, i) => expect(buttons[i], '第 ' + (i + 1) + ' 按钮 @click.stop 零改').toContain('@click.stop="' + h + '"'))
  })

  it('七按钮文案逐字零改', () => {
    const labels = ['加入', '参会人', '邀请', '邀请管理', '改时间', '标记完成', '签到码']
    const buttons = innerButtons(meeting)
    handlersForLabels(buttons, labels)
  })

  it('外层卡片 role/tabindex 与 keydown=viewMeeting 零改', () => {
    const card = cardOuter(meeting)
    expect(card).toContain('class="mc modern-card" role="button" tabindex="0"')
    expect(card).toContain('@click="viewMeeting(m)"')
    expect(card).toContain('@keydown.enter="viewMeeting(m)"')
    expect(card).toContain('@keydown.space.prevent="viewMeeting(m)"')
  })

  it('业务函数 viewMeeting / joinMeeting 零改（签名与核心端点）', () => {
    expect(meeting).toContain('async function viewMeeting(m: M) {')
    expect(meeting).toContain('async function joinMeeting(m: M) {')
    expect(meeting).toContain('/accept')
  })

  it('日历对照 div.calendar-event 零改（避免波及他处）', () => {
    expect(meeting).toContain('class="calendar-event" role="button" tabindex="0" @click="viewMeeting(m)" @keydown.enter="viewMeeting(m)" @keydown.space.prevent="viewMeeting(m)"')
  })
})

function handlersForLabels(buttons: string[], labels: string[]): void {
  labels.forEach((label, i) => expect(buttons[i], '第 ' + (i + 1) + ' 按钮文案零改').toContain('>' + label + '</button>'))
}
