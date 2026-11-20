// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

/**
 * R296 P2 样板：会议管理默认视图必须是任务导向的办公界面，而不是把后端
 * /api/* 端点平铺成调试按钮的面板。以下断言从模板（<template> 段）抽取，
 * 不触碰 <script> —— API 调用能力保留在脚本层，只是不再渲染为主界面按钮。
 */
describe('MeetingApp 业务视图契约（R296 P2）', () => {
  const source = readFileSync(resolve(import.meta.dirname, 'MeetingApp.vue'), 'utf8')
  const template = source.slice(source.indexOf('<template>'), source.indexOf('</template>'))

  it('默认视图提供「会议室预订 / 我的会议 / 日历视图」三个业务区块', () => {
    expect(template).toContain('会议室预订')
    expect(template).toContain('我的会议')
    expect(template).toContain('日历视图')
    for (const section of ['booking', 'mine', 'calendar']) {
      expect(template, `缺少业务区块 class: ${section}`).toContain(`class="${section}`)
    }
  })

  it('会议室预订区有真实业务控件（表单 + 提交按钮），不是端点触发按钮', () => {
    const booking = template.slice(template.indexOf('class="booking'), template.indexOf('class="mine'))
    expect(booking).toContain('<form')
    expect(booking).toContain('type="date"')
    expect(booking).toMatch(/<button[^>]*type="submit"/)
  })

  it('我的会议区有列表控件，可作为员工日常待办入口', () => {
    const mine = template.slice(template.indexOf('class="mine'), template.indexOf('class="calendar'))
    expect(mine).toMatch(/<ul|<table/)
  })

  it('日历视图区有月历网格容器', () => {
    const calendar = template.slice(template.indexOf('class="calendar'))
    expect(calendar).toContain('calendar-grid')
  })

  it('默认视图不渲染 /api/* 端点调试按钮，副标题不暴露 API 路径', () => {
    expect(template).not.toContain('/api/')
    expect(template).not.toMatch(
      /@click="[^"]*(?:loadMeeting|meetingWrite|meetingMore|meetingLists|meetingAttachment|addBuilding|addRoom)/,
    )
  })
})
