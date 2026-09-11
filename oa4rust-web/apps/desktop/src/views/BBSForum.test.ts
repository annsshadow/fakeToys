import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const source = readFileSync(resolve(import.meta.dirname, 'BBSForum.vue'), 'utf8')

describe('BBSForum contracts', () => {
  it('keeps the consumed first-screen section and topic queries', () => {
    expect(source).toContain("api.get('/jaxrs/bbs/assemble/control/section/list')")
    expect(source).toContain("queryKey: ['bbs', 'sections']")
    expect(source).toContain("queryKey: ['bbs', 'topics'")
    expect(source).toContain("queryKey: ['bbs', 'replies'")
  })

  it('does not auto-query write handlers on mount', () => {
    expect(source).not.toContain("api.get('/jaxrs/bbs/assemble/control/topic/create')")
    expect(source).not.toContain("api.get('/jaxrs/bbs/subject/create')")
    expect(source).not.toContain("api.get('/jaxrs/bbs/assemble/control/delete/subject')")
    expect(source).not.toContain("api.get('/jaxrs/bbs/assemble/control/delete/reply')")
    expect(source).not.toContain("api.get('/jaxrs/bbs/assemble/control/delete/forum')")
    expect(source).not.toContain("api.get('/jaxrs/bbs/assemble/control/shutup/create')")
    expect(source).not.toContain("api.get('/jaxrs/bbs/assemble/control/shutup/delete')")
    expect(source).not.toContain("api.get('/jaxrs/bbs/assemble/control/update/control/config')")
    expect(source).not.toContain("api.get('/jaxrs/comment/c-1/commend')")
    expect(source).not.toContain("api.get('/jaxrs/comment/c-1/uncommend')")
    expect(source).not.toContain("api.get('/jaxrs/comment/u3-cmt/commend')")
    expect(source).not.toContain("api.get('/jaxrs/comment/u3-cmt/uncommend')")
  })

  it('does not auto-query placeholder ids or module root paths', () => {
    expect(source).not.toContain("api.get('/jaxrs/bbs/assemble/control/subject/view/sub-001')")
    expect(source).not.toContain("api.get('/jaxrs/bbs/core/entity/subject/subject-001')")
    expect(source).not.toContain("api.get('/jaxrs/bbs/core/entity/forum/forum-001')")
    expect(source).not.toContain("api.get('/jaxrs/bbs/assemble/control/permission/section/sec-001')")
    expect(source).not.toContain("api.get('/jaxrs/bbs/core/entity/section/list/test-forum-id')")
    expect(source).not.toContain("api.get('/jaxrs/bbs/core/entity/subject/list/test-section-id')")
    expect(source).not.toContain("api.get('/jaxrs/bbs/core/entity/subject/top/test-section-id')")
    expect(source).not.toContain("api.get('/jaxrs/bbs/subject/list/1')")
    expect(source).not.toContain("api.get('/jaxrs/bbs/forum/view/1')")
    expect(source).not.toContain("api.get('/jaxrs/comment/list/i-1/next/10')")
    expect(source).not.toContain("api.get('/jaxrs/comment/list/i-1/prev/10')")
    expect(source).not.toContain("api.get('/jaxrs/comment/c-1')")
    expect(source).not.toContain("api.get('/jaxrs/comment/list/1/size/50')")
    expect(source).not.toContain("api.get('/jaxrs/comment/list/1/size/10')")
    expect(source).not.toContain("api.get('/jaxrs/bbs')")
    expect(source).not.toContain("api.get('/jaxrs/comment')")
    expect(source).not.toContain("api.get('/jaxrs/bbs/assemble/control')")
    expect(source).not.toContain("api.get('/jaxrs/bbs_assemble_control')")
    expect(source).not.toContain("api.get('/jaxrs/bbs/assemble/control/list/reply/filter')")
  })
})
