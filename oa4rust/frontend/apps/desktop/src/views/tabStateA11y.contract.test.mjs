/**
 * 第87轮源码级契约：tab/切换按钮的激活态必须对辅助技术可见（WCAG 4.1.2 Name, Role, Value）。
 * 仓内没有 @vue/test-utils，故用 node:test 对 Vue 模板源码做文本契约；
 * 每条断言在"仅靠 CSS class 表示选中态"时都会失败。
 */

import assert from 'node:assert/strict'
import { readFileSync } from 'node:fs'
import { test } from 'node:test'

const read = (file) => readFileSync(new URL(file, import.meta.url), 'utf8')
const linesWith = (src, needle) => src.split(/\r?\n/).filter((line) => line.includes(needle))

test('AttendanceApp.cfg-tab 的 12 个排班配置切换按钮都暴露 aria-pressed', () => {
  const src = read('./AttendanceApp.vue')
  const tabs = linesWith(src, 'class="cfg-tab"')
  assert.equal(tabs.length, 12, 'cfg-tab 按钮数应为 12')
  const keys = [
    'schedule',
    'employee',
    'holiday',
    'workday',
    'admin',
    'importlog',
    'statlog',
    'cycle',
    'v2wp',
    'v2group',
    'v2shift',
    'v2leave',
  ]
  for (const key of keys) {
    const line = tabs.find((l) => l.includes(`moreTab==='${key}'`))
    assert.ok(line, `缺少 cfg-tab 切换按钮: ${key}`)
    assert.match(line, new RegExp(`:aria-pressed="moreTab==='${key}'"`), `cfg-tab[${key}] 未把选中态暴露给辅助技术`)
  }
})

test('BBSForum 论坛主标签与"我的"子标签都暴露 aria-pressed', () => {
  const src = read('./BBSForum.vue')
  const navBlock = src.match(/<nav class="forum-tabs"[\s\S]*?<\/nav>/)?.[0]
  assert.ok(navBlock, '缺少论坛主标签 nav')
  assert.match(navBlock, /:aria-pressed="activeTab === tab\.key"/, '论坛主标签未把激活项暴露给辅助技术')

  for (const sub of ['topics', 'replies']) {
    const line = linesWith(src, `@click="switchMySub('${sub}')"`)[0]
    assert.ok(line, `缺少我的子标签: ${sub}`)
    assert.match(line, new RegExp(`:aria-pressed="mySub==='${sub}'"`), `我的子标签[${sub}] 未把选中态暴露给辅助技术`)
  }
})

test('CalendarApp 日历范围切换暴露 aria-pressed，而动作按钮不被误加', () => {
  const src = read('./CalendarApp.vue')
  for (const scope of ['my', 'public']) {
    const line = linesWith(src, `:class="{on:calScope==='${scope}'}"`)[0]
    assert.ok(line, `缺少日历范围切换按钮: ${scope}`)
    assert.match(
      line,
      new RegExp(`:aria-pressed="calScope==='${scope}'"`),
      `日历范围[${scope}] 未把选中态暴露给辅助技术`,
    )
  }
  for (const action of ['loadCalSettings', 'loadCalCoreEntities', 'loadCalTwin', 'calWrite']) {
    const line = linesWith(src, action).find((l) => l.includes('class="cll-tab"'))
    assert.ok(line, `缺少动作按钮: ${action}`)
    assert.doesNotMatch(line, /aria-pressed/, `动作按钮[${action}] 不应被误加 aria-pressed`)
  }
})
