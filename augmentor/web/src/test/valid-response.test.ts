/**
 * `valid-response.ts` 的自证（防生成器静默退化成空转）
 *
 * 两组判据：
 * 1. 表里每一条的样本都能被**自己那个 schema** 吃下 —— 生成器若哪天只吐出 `{}`，
 *    这里当场红，而不是等到 49 个用例各自报「字段 X 缺失」；
 * 2. 没登记的端点**抛错** —— 不许回退成 `{}` 把「没登记」伪装成「校验通过」。
 */
import { describe, expect, it } from 'vitest'

import { ROUTES, responseFor, validSample } from './valid-response'

describe('合法响应生成器', () => {
  it('每个登记端点的样本都能通过自己的 schema', () => {
    for (const [method, pattern, schema] of ROUTES) {
      const sample = validSample(schema)
      const parsed = schema.safeParse(sample)
      const detail = parsed.success ? '' : JSON.stringify(parsed.error.issues)
      expect(parsed.success, `${method} ${pattern}：${detail}`).toBe(true)
    }
  })

  it('样本不是空对象（最小实例要真的长出来）', () => {
    const empty = ROUTES.filter(([, , schema]) => {
      const sample = validSample(schema)
      return typeof sample === 'object' && sample !== null && Object.keys(sample).length === 0
    })
    expect(empty.map(([, pattern]) => pattern)).toEqual([])
  })

  it('路径参数按一段展开，查询串不参与匹配', () => {
    expect(responseFor('GET', '/versions/v1')).toEqual(responseFor('GET', '/versions/v2'))
    expect(() => responseFor('GET', '/versions/v1/data/extra')).toThrow()
    expect(responseFor('GET', '/data/list?all_files=true')).toEqual(
      responseFor('GET', '/data/list')
    )
  })

  it('同一路径不同动词取各自的模型（GET /config 与 POST /config）', () => {
    expect(responseFor('GET', '/config')).not.toEqual(responseFor('POST', '/config'))
  })

  it('没登记的方法/路径抛错，而不是回 {}', () => {
    expect(() => responseFor('GET', '/no/such/endpoint')).toThrow('没有登记合法响应')
    expect(() => responseFor('PATCH', '/config')).toThrow('没有登记合法响应')
  })
})
