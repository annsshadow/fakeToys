// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/**
 * R538 RED→GREEN 契约：H5 其余 9 页 13 控件补 aria-label，AT 聚焦原生节点 AX name 非空（WCAG 2.1 SC4.1.2 / 3.3.2）。
 * R536 实证：uni-app H5 将 placeholder 编译为 div.uni-input-placeholder，原生 INPUT 无 placeholder 属性，
 * 宿主 uni-input 无 aria-label，增强器 labelHost 无则 return——故只能逐页补 aria-label（R537 增强器透传到聚焦原生控件）。
 * 核心：12 个可达控件（13 中动态字段 326-378 因 stub 空态未渲染据实不纳入）aria-label 精确存在且文案正确。
 * 守卫：main.ts R537 增强器零改；v-model / @tap / @confirm / placeholder / handler / 模板结构 / 样式 / 他页零改。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const src = resolve(import.meta.dirname, '..')
const read = (rel: string): string => readFileSync(resolve(src, rel), 'utf8')

const login = read('pages/login/login.vue')
const search = read('pages/search/search.vue')
const contacts = read('pages/contacts/contacts.vue')
const chat = read('pages/chat/chat.vue')
const mine = read('pages/mine/mine.vue')
const attendance = read('pages/attendance/attendance.vue')
const topic = read('pages/bbs/topic.vue')
const processStart = read('pages/process/start.vue')
const main = read('main.ts')

function lineWith(source: string, needle: string): string {
  const line = source.split('\n').find((l) => l.includes(needle))
  expect(line, '缺少锚点: ' + needle).toBeDefined()
  return line ?? ''
}

describe('R538 H5 其余表单控件可访问名契约', () => {
  // ===== 核心：12 个可达控件 aria-label 精确存在（真父无 → RED）=====
  it('login.vue 账号/密码两控件补 aria-label', () => {
    const account = lineWith(login, 'v-model="credential"')
    expect(account).toContain('aria-label="账号"')
    expect(account).toContain('placeholder="工号 / 用户名"')
    const pwd = lineWith(login, 'v-model="password"')
    expect(pwd).toContain('aria-label="密码"')
    expect(pwd).toContain('placeholder="密码"')
  })

  it('search/contacts/chat 三页搜索与消息控件补 aria-label', () => {
    expect(search).toContain('aria-label="搜索关键词"')
    expect(contacts).toContain('aria-label="通讯录搜索关键词"')
    expect(chat).toContain('aria-label="消息内容"')
  })

  it('mine 改密三框补 aria-label（旧/新/确认）', () => {
    expect(mine).toContain('aria-label="旧密码"')
    expect(mine).toContain('aria-label="新密码"')
    expect(mine).toContain('aria-label="确认密码"')
  })

  it('attendance/topic/processStart 三页四控件补 aria-label', () => {
    expect(attendance).toContain('aria-label="假期日期"')
    expect(attendance).toContain('aria-label="申请事由"')
    expect(topic).toContain('aria-label="回帖内容"')
    expect(processStart).toContain('aria-label="流程标题"')
  })

  // ===== 守卫：业务零改（真父亦须通过）=====
  it('守卫：R537 增强器源码逐字零改（宿主透传逻辑保留不破坏）', () => {
    expect(main).toContain('function installH5NativeInputNameA11y')
    expect(main).toContain('HOST_SELECTOR = \'uni-input, uni-textarea, uni-picker\'')
    expect(main).toContain('NATIVE_SELECTOR = \'input, textarea\'')
    expect(main).toContain('setAttribute(\'aria-label\'')
    expect(main).toContain('closest(HOST_SELECTOR)')
    expect(main).toContain('installH5NativeInputNameA11y()')
  })

  it('守卫：login/search/contacts/chat v-model 与业务事件逐字保留', () => {
    expect(login).toContain('v-model="credential"')
    expect(login).toContain('v-model="password"')
    expect(login).toContain('@tap="submit"')
    expect(search).toContain('v-model="keyword"')
    expect(search).toContain('@confirm="doSearch"')
    expect(search).toContain('@tap="doSearch"')
    expect(contacts).toContain('v-model="keyword"')
    expect(contacts).toContain('@confirm="doSearch"')
    expect(chat).toContain('v-model="text"')
    expect(chat).toContain('@confirm="send"')
    expect(chat).toContain('@tap="send"')
  })

  it('守卫：mine/attendance/topic/processStart 业务事件与占位符逐字保留', () => {
    expect(mine).toContain('v-model="pwdForm.oldPassword"')
    expect(mine).toContain('v-model="pwdForm.newPassword"')
    expect(mine).toContain('v-model="pwdForm.confirmPassword"')
    expect(mine).toContain('@tap="changePassword"')
    expect(mine).toContain('placeholder="新密码（至少 10 位）"')
    expect(attendance).toContain('v-model="leaveDate"')
    expect(attendance).toContain('v-model="leaveReason"')
    expect(attendance).toContain('@tap="applyLeave"')
    expect(attendance).toContain('placeholder="假期日期（如 2026-10-09）"')
    expect(topic).toContain('v-model="text"')
    expect(topic).toContain('@confirm="send"')
    expect(topic).toContain('placeholder="写下你的回帖…"')
    expect(processStart).toContain('v-model="title"')
    expect(processStart).toContain('placeholder="工作标题（必填）"')
  })

  it('守卫：R535 calendar/bbs-new 宿主层正面对照与其他页面零改', () => {
    const cal = read('pages/calendar/calendar.vue')
    const bbs = read('pages/bbs/new.vue')
    expect(cal).toContain('role="button"')
    expect(cal).toContain('aria-label="新建日历"')
    expect(cal).toContain('@tap="toggleCalendarForm"')
    expect(bbs).toContain('aria-label="版块"')
    expect(bbs).toContain('aria-label="标题"')
    expect(bbs).toContain('aria-label="内容"')
  })

  it('核心：aria-label 精确唯一（防重复污染导致 AX 名歧义，真父无 label → RED）', () => {
    const count = (s: string, label: string) => s.split('aria-label="' + label + '"').length - 1
    expect(count(login, '账号')).toBe(1)
    expect(count(login, '密码')).toBe(1)
    expect(count(mine, '旧密码')).toBe(1)
    expect(count(mine, '新密码')).toBe(1)
    expect(count(mine, '确认密码')).toBe(1)
  })

  it('守卫：aria-label 不覆盖 placeholder（二者共存，业务占位符未被替换）', () => {
    expect(lineWith(login, 'v-model="credential"')).toContain('placeholder="工号 / 用户名"')
    expect(lineWith(chat, 'v-model="text"')).toContain('placeholder="发消息…"')
    expect(lineWith(processStart, 'v-model="title"')).toContain('placeholder="工作标题（必填）"')
  })
})
