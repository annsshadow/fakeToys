// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

/** R296 P2：论坛默认视图必须是任务导向业务界面（版块导航 / 主题列表 / 发帖区），而非 /api/* 端点调试面板。 */
describe('BBSForum 业务视图契约（R296 P2）', () => {
  const source = readFileSync(resolve(import.meta.dirname, 'BBSForum.vue'), 'utf8')
  const template = source.slice(source.indexOf('<template>'), source.indexOf('</template>'))

  it('提供版块导航 / 主题列表 / 发帖区三个业务区块', () => {
    expect(template).toContain('版块导航')
    expect(template).toContain('主题列表')
    expect(template).toContain('发起新主题')
    for (const s of ['bbs-section-nav', 'bbs-section-list', 'bbs-topic-list', 'bbs-compose-panel']) {
      expect(template, `缺少业务区块: ${s}`).toContain(`data-testid="${s}"`)
    }
  })

  it('发帖区有真实业务表单（标题输入 + 正文文本域 + 提交按钮）', () => {
    const compose = template.slice(template.indexOf('data-testid="bbs-compose-panel"'))
    expect(compose).toContain('<form')
    expect(compose).toContain('data-testid="bbs-compose-title-input"')
    expect(compose).toContain('<textarea')
    expect(compose).toContain('data-testid="bbs-compose-content"')
    expect(compose).toMatch(/<button[^>]*type="submit"[^>]*data-testid="bbs-compose-submit"/)
  })

  it('版块导航与主题列表有列表控件和搜索框', () => {
    expect(template).toContain('data-testid="bbs-section-list"')
    expect(template).toMatch(/<ul|<li v-for|<div v-for/)
    expect(template).toContain('data-testid="bbs-search-input"')
    expect(template).toContain('data-testid="bbs-topic-list"')
    expect(template).toContain('data-testid="bbs-topic-pagination"')
  })

  it('副标题为业务语义描述，不暴露 API 路径', () => {
    expect(template).toContain('data-testid="bbs-subtitle"')
    expect(template).toContain('浏览版块、参与讨论、分享经验')
    expect(template).not.toContain('/api/')
  })

  it('默认视图不渲染 /api/* 端点调试按钮', () => {
    expect(template).not.toMatch(
      /@click="[^"]*(?:loadForums|loadBbsViews|loadBbsControl|loadBbsEntities|loadBbsDeepReads|bbsPost|bbsPut|bbsDel|bbsEntity|bbsSubjectMod)/,
    )
  })
})
