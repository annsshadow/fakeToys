// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// t32: H5 关键业务空态引导契约。源码级断言 + 真浏览器实测互补。
import { readFileSync } from 'node:fs'
import { fileURLToPath } from 'node:url'
import { describe, expect, it } from 'vitest'

function src(rel: string): string {
  return readFileSync(fileURLToPath(new URL(rel, import.meta.url)), 'utf8')
}

const GUIDE_CLASS = 'empty-guide'

const cases = [
  {
    name: '文档页附件/文件空态有上传与刷新入口',
    rel: './doc/doc.vue',
    texts: ['还没有附件', '文档库暂无文件'],
    actions: ['@tap="pickFromAlbumOrCamera"', '@tap="pickFromFilePicker"', '@tap="load()"'],
    naked: ['class="tip">暂无附件', 'class="tip">暂无文件'],
  },
  {
    name: '会议页空态有刷新与切换页签入口',
    rel: './meeting/meeting.vue',
    texts: ['换个页签看看或刷新'],
    actions: ['@tap="load()"', "@tap=\"switchTab(tab === 'mine' ? 'wait' : 'mine')\""],
    naked: [],
  },
  {
    name: '收藏页空态有去浏览与刷新入口',
    rel: './collect/collect.vue',
    texts: ['还没有收藏', '去浏览内容'],
    actions: ['@tap="goBrowse"', '@tap="load"'],
    naked: ['class="tip">暂无收藏'],
  },
  {
    name: '浏览页空态有刷新与去搜索入口',
    rel: './browse/browse.vue',
    texts: ['暂无浏览数据', '换个页签'],
    actions: ['@tap="load()"', '@tap="goSearch"'],
    naked: ['class="tip">暂无数据'],
  },
  {
    name: '回收站空态有刷新与去文档入口',
    rel: './recycle/recycle.vue',
    texts: ['回收站为空', '删除的内容'],
    actions: ['@tap="load()"', '@tap="goDoc"'],
    naked: ['class="tip">回收站为空'],
  },
  {
    name: '首页公告空态有刷新公告入口，不再纯空白',
    rel: './index/index.vue',
    texts: ['暂无公告', '刷新公告'],
    actions: ['@tap="loadNotices(true)"'],
    naked: ['class="sec-empty">暂无公告'],
  },
]

describe('t32 H5 空态引导契约', () => {
  for (const c of cases) {
    it(c.name, () => {
      const s = src(c.rel)
      expect(s, c.name + ' 应包含空态引导容器 ' + GUIDE_CLASS).toContain(GUIDE_CLASS)
      for (const text of c.texts) {
        expect(s, c.name + ' 应有引导文案：' + text).toContain(text)
      }
      for (const action of c.actions) {
        expect(s, c.name + ' 应有可点击入口：' + action).toContain(action)
      }
      for (const naked of c.naked) {
        expect(s, c.name + ' 不应残留裸空态：' + naked).not.toContain(naked)
      }
    })
  }

  it('所有目标页空态入口均绑定真实存在的函数', () => {
    const checks: Array<{ rel: string; fn: string }> = [
      { rel: './doc/doc.vue', fn: 'pickFromAlbumOrCamera' },
      { rel: './doc/doc.vue', fn: 'pickFromFilePicker' },
      { rel: './doc/doc.vue', fn: 'load' },
      { rel: './meeting/meeting.vue', fn: 'switchTab' },
      { rel: './meeting/meeting.vue', fn: 'load' },
      { rel: './collect/collect.vue', fn: 'goBrowse' },
      { rel: './collect/collect.vue', fn: 'load' },
      { rel: './browse/browse.vue', fn: 'goSearch' },
      { rel: './browse/browse.vue', fn: 'load' },
      { rel: './recycle/recycle.vue', fn: 'goDoc' },
      { rel: './recycle/recycle.vue', fn: 'load' },
      { rel: './index/index.vue', fn: 'loadNotices' },
    ]
    for (const { rel, fn } of checks) {
      const s = src(rel)
      expect(s, rel + ' 应定义 ' + fn).toMatch(
        new RegExp('(?:async\\s+)?function\\s+' + fn + '\\b|const\\s+' + fn + '\\s*='),
      )
    }
  })
})
