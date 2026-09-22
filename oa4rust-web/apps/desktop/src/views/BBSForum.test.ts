// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const source = readFileSync(resolve(import.meta.dirname, 'BBSForum.vue'), 'utf8')

describe('BBSForum contracts', () => {
  it('keeps the consumed first-screen section and topic queries', () => {
    expect(source).toContain("api.get('/api/bbs/assemble/control/section/list')")
    expect(source).toContain("queryKey: ['bbs', 'sections']")
    expect(source).toContain("queryKey: ['bbs', 'topics'")
    expect(source).toContain("queryKey: ['bbs', 'replies'")
  })

  it('does not auto-query write handlers on mount', () => {
    expect(source).not.toContain("api.get('/api/bbs/assemble/control/topic/create')")
    expect(source).not.toContain("api.get('/api/bbs/subject/create')")
    expect(source).not.toContain("api.get('/api/bbs/assemble/control/delete/subject')")
    expect(source).not.toContain("api.get('/api/bbs/assemble/control/delete/reply')")
    expect(source).not.toContain("api.get('/api/bbs/assemble/control/delete/forum')")
    expect(source).not.toContain("api.get('/api/bbs/assemble/control/shutup/create')")
    expect(source).not.toContain("api.get('/api/bbs/assemble/control/shutup/delete')")
    expect(source).not.toContain("api.get('/api/bbs/assemble/control/update/control/config')")
    expect(source).not.toContain("api.get('/api/comment/c-1/commend')")
    expect(source).not.toContain("api.get('/api/comment/c-1/uncommend')")
    expect(source).not.toContain("api.get('/api/comment/u3-cmt/commend')")
    expect(source).not.toContain("api.get('/api/comment/u3-cmt/uncommend')")
  })

  it('does not auto-query placeholder ids or module root paths', () => {
    expect(source).not.toContain("api.get('/api/bbs/assemble/control/subject/view/sub-001')")
    expect(source).not.toContain("api.get('/api/bbs/core/entity/subject/subject-001')")
    expect(source).not.toContain("api.get('/api/bbs/core/entity/forum/forum-001')")
    expect(source).not.toContain("api.get('/api/bbs/assemble/control/permission/section/sec-001')")
    expect(source).not.toContain("api.get('/api/bbs/core/entity/section/list/test-forum-id')")
    expect(source).not.toContain("api.get('/api/bbs/core/entity/subject/list/test-section-id')")
    expect(source).not.toContain("api.get('/api/bbs/core/entity/subject/top/test-section-id')")
    expect(source).not.toContain("api.get('/api/bbs/subject/list/1')")
    expect(source).not.toContain("api.get('/api/bbs/forum/view/1')")
    expect(source).not.toContain("api.get('/api/comment/list/i-1/next/10')")
    expect(source).not.toContain("api.get('/api/comment/list/i-1/prev/10')")
    expect(source).not.toContain("api.get('/api/comment/c-1')")
    expect(source).not.toContain("api.get('/api/comment/list/1/size/50')")
    expect(source).not.toContain("api.get('/api/comment/list/1/size/10')")
    expect(source).not.toContain("api.get('/api/bbs')")
    expect(source).not.toContain("api.get('/api/comment')")
    expect(source).not.toContain("api.get('/api/bbs/assemble/control')")
    expect(source).not.toContain("api.get('/api/bbs_assemble_control')")
    expect(source).not.toContain("api.get('/api/bbs/assemble/control/list/reply/filter')")
  })

  it('keeps only the consumed section, topic and reply queries', () => {
    const calls = source.match(/= useQuery\(/g) ?? []
    expect(calls).toHaveLength(3)
    expect(source).not.toContain("api.get('/api/bbs/topic/list')")
    expect(source).not.toContain("api.get('/api/bbs/core/entity/section')")
    expect(source).not.toContain("api.get('/api/bbs/assemble/control/shutup/list')")
    expect(source).not.toContain("api.get('/api/bbs/review/v2/search')")
    expect(source).not.toContain("api.get('/api/bbs/subject/search')")
  })

  it('calls only registered parameterized BBS routes (bare no-arg GET handlers 500 at runtime)', () => {
    // 裸静态路由（如 subject/index/list）的 handler 需 Path((page,count))，
    // 无参注册时 axum 提取必然失败 → 500。列表/搜索/回复必须走 fmt 参数化路由。
    // 钉 URL 字面量本身（与调用换行布局无关）。
    expect(source).toContain('`/api/bbs/assemble/control/subject/index/list/page/${page.value}/count/${pageSize}`')
    expect(source).toContain('`/api/bbs/assemble/control/subject/creamed/list/page/${page.value}/count/${pageSize}`')
    expect(source).toContain('`/api/bbs/assemble/control/subject/search/list/page/1/count/${pageSize}`')
    expect(source).toContain(
      '`/api/bbs/assemble/control/subject/filter/listsubjectinfo/page/${page.value}/count/${pageSize}`',
    )
    expect(source).toContain(
      '`/api/bbs/assemble/control/subject/recommended/list/page/${page.value}/count/${pageSize}`',
    )
    expect(source).toContain("'/api/bbs/assemble/control/reply/filter/list/page/1/count/50'")
    // 版块筛选走 bbs crate 已注册 GET 路由（bbs_subject_info 实表）
    expect(source).toContain('`/api/bbs/subject/list/${selectedSection.value.id}`')
    // 不再调用运行时 500 的裸静态路由
    expect(source).not.toContain("api.get('/api/bbs/assemble/control/list/subjects/index')")
    expect(source).not.toContain("api.post('/api/bbs/assemble/control/subject/search'")
    expect(source).not.toContain('api.post(`/api/bbs/assemble/control/list/subjects/filtered`')
    expect(source).not.toContain('api.post(`/api/bbs/assemble/control/list/reply/filter`')
  })

  it('wires the add-section control to registered section write routes', () => {
    // 版块发布/管理后端已实装（x_bbs_assemble_control_section 硬删），
    // 加号/重命名/删除必须打到这些路由，而非无事件死控件。
    expect(source).toContain('@click="openNewSection()"')
    expect(source).toContain("api.post('/api/bbs/assemble/control/section/create', { name })")
    expect(source).toContain('`/api/bbs/assemble/control/section/save/${sectionModalTarget.value.id}`')
    expect(source).toContain('`/api/bbs/assemble/control/section/delete/${sec.id}`')
  })

  it('covers the personal page (my replies) with the registered user/reply/my route', () => {
    // 论坛个人主页：「我的」页签 = 我的主题（listsubjectinfo）+ 我的回复（user/reply/my）。
    expect(source).toContain('user/reply/my/list/page/${page.value}/count/${pageSize}')
    expect(source).toContain('我的回复')
  })
})
