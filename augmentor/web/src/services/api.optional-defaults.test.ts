/**
 * `services/api.ts` 的**省略分支**契约：调用方不传可选 `opts` 时，请求体/
 * query 里那些可选键必须「不传即不写」——而不是本层自己补一套默认值。
 *
 * 为什么单独立一个文件：这些函数的 `opts = {}` 默认参数分支只有**省略
 * 实参**才走得到。`api.test.ts` 的同名用例一律显式传 opts（那是对的，
 * 它锁的是显式值），于是 default-arg 这一格在 70% 分支门禁下裸奔——
 * L226~L233 把 api.ts 扩到 264 条语句后，分支覆盖跌到 52.38%（CI 实测），
 * 本文件与上述用例各守一半：它守「写了什么」，这里守「没写什么」。
 *
 * 断言方式与 `api.test.ts` 相同：mock `axios.create` 返回的实例，检查
 * `post/get` 收到的实参。`toHaveBeenCalledWith` 的相等语义忽略 undefined
 * 值键，所以「键存在但值为 undefined」与「键不存在」在断言层等价——那
 * 正是 `JSON.stringify` 发出去时的形状（undefined 被丢弃）。
 */
import { beforeEach, describe, expect, it, vi } from 'vitest'

const mocks = vi.hoisted(() => {
  const instance = {
    get: vi.fn(),
    post: vi.fn(),
    put: vi.fn(),
    delete: vi.fn(),
    interceptors: {
      request: { use: vi.fn() },
      response: { use: vi.fn() },
    },
  }
  return { instance, create: vi.fn(() => instance) }
})

vi.mock('axios', () => ({
  default: { create: mocks.create },
}))

import * as api from './api'

const { instance } = mocks

/** 让下一次请求返回给定的 data */
function respondWith(data: unknown) {
  instance.get.mockResolvedValue({ data })
  instance.post.mockResolvedValue({ data })
  instance.put.mockResolvedValue({ data })
  instance.delete.mockResolvedValue({ data })
  return data
}

beforeEach(() => {
  instance.get.mockReset()
  instance.post.mockReset()
  instance.put.mockReset()
  instance.delete.mockReset()
})

describe('省略可选 opts 时不写第二套默认（default-arg 分支契约）', () => {
  it('compareDatasets：三个可选键都不进请求体', async () => {
    respondWith({})
    await api.compareDatasets('a.json', 'b.json')
    expect(instance.post).toHaveBeenCalledWith('/dataset/compare', {
      dataset_a: 'a.json',
      dataset_b: 'b.json',
    })
  })

  it('convertDataset：目标/源格式都不进请求体（source_format 的空串哨兵走 falsy 侧）', async () => {
    respondWith({})
    await api.convertDataset('in.json', 'out.xlsx')
    expect(instance.post).toHaveBeenCalledWith('/dataset/convert', {
      input_file: 'in.json',
      output_file: 'out.xlsx',
    })
  })

  it('mergeDatasets：deduplicate 不传，默认住在服务端请求模型里', async () => {
    respondWith({})
    await api.mergeDatasets(['a.json', 'b.json'], 'merged.json')
    expect(instance.post).toHaveBeenCalledWith('/dataset/merge', {
      inputs: ['a.json', 'b.json'],
      output_file: 'merged.json',
    })
  })

  it('splitDataset：七个旋钮一个都不写', async () => {
    respondWith({})
    await api.splitDataset('in.json', 'out_dir')
    expect(instance.post).toHaveBeenCalledWith('/dataset/split', {
      input_file: 'in.json',
      output_dir: 'out_dir',
    })
  })

  it('sampleDataset：method/size/ratio/seed/stratify_key 都不写', async () => {
    respondWith({})
    await api.sampleDataset('in.json', 'out.json')
    expect(instance.post).toHaveBeenCalledWith('/dataset/sample', {
      input_file: 'in.json',
      output_file: 'out.json',
    })
  })

  it('aggregateDatasets：strategy/target_size 都不写（datasets 是 名称→路径 映射）', async () => {
    respondWith({})
    await api.aggregateDatasets({ a: 'a.json', b: 'b.json' }, 'agg.json')
    expect(instance.post).toHaveBeenCalledWith('/dataset/aggregate', {
      datasets: { a: 'a.json', b: 'b.json' },
      output_file: 'agg.json',
    })
  })

  it('convertToRag：format 的空串哨兵走 falsy 侧，不进 query 语义的请求体', async () => {
    respondWith({})
    await api.convertToRag('in.json', 'out.jsonl')
    expect(instance.post).toHaveBeenCalledWith('/dataset/rag', {
      input_file: 'in.json',
      output_file: 'out.jsonl',
    })
  })

  it('semanticSearch：top_k/text_key 省略，默认由服务端请求模型说话', async () => {
    respondWith({})
    await api.semanticSearch('in.json', '猫')
    expect(instance.post).toHaveBeenCalledWith('/dataset/semantic-search', {
      input_file: 'in.json',
      query: '猫',
    })
  })

  it('registerDependencyDataset：description/tags/registry_path 都不写', async () => {
    respondWith({})
    await api.registerDependencyDataset('my-set', 'a.json')
    expect(instance.post).toHaveBeenCalledWith(
      '/system/dependency/datasets',
      { name: 'my-set', input_file: 'a.json' },
      { params: { registry_path: undefined } },
    )
  })

  it('migrateDataset：rules 的空数组哨兵走 falsy 侧', async () => {
    respondWith({})
    await api.migrateDataset('in.json', 'out.json')
    expect(instance.post).toHaveBeenCalledWith('/system/migrate', {
      input_file: 'in.json',
      output_file: 'out.json',
    })
  })

  it('streamProcess：operation/chunk_size/output_format 都不写', async () => {
    respondWith({})
    await api.streamProcess('in.json', 'out.json')
    expect(instance.post).toHaveBeenCalledWith('/system/stream', {
      input_file: 'in.json',
      output_file: 'out.json',
    })
  })
})

describe('省略可选目录参数时 query 键不出现（listBackups / 血缘两族）', () => {
  it('listBackups 不送 backup_dir，落点由服务端白名单首根决定', async () => {
    respondWith({})
    await api.listBackups()
    expect(instance.get).toHaveBeenCalledWith('/system/backups', {
      params: {},
    })
  })

  it('restoreBackup 不送 backup_dir（目标已存在则整份覆盖，服务端口径）', async () => {
    respondWith({})
    await api.restoreBackup('bk-1', 'restored.json')
    expect(instance.post).toHaveBeenCalledWith(
      '/system/backups/bk-1/restore',
      { output_file: 'restored.json' },
      expect.anything(),
    )
  })

  it('listDependencyDatasets 不送 registry_path', async () => {
    respondWith({})
    await api.listDependencyDatasets()
    expect(instance.get).toHaveBeenCalledWith('/system/dependency/datasets', {
      params: { registry_path: undefined },
    })
  })
})
