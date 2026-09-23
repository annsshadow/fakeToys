/**
 * `services/api.ts` 的契约测试
 *
 * 这一层全是 HTTP 调用，没有分支逻辑，所以测试的价值不在"覆盖行"，
 * 而在**锁定与后端的约定**：URL、HTTP 方法、查询参数名、请求体字段名、默认值。
 * 任何一条被改动都应该让这里变红——因为后端是按这些名字解析的。
 *
 * 断言方式：mock 掉 `axios.create` 返回的实例，检查 `get/post/put/delete`
 * 收到的实参，而不是只断言"被调用过"。
 */
import { beforeEach, describe, expect, it, vi } from 'vitest'

const mocks = vi.hoisted(() => {
  const instance = {
    get: vi.fn(),
    post: vi.fn(),
    put: vi.fn(),
    delete: vi.fn(),
  }
  return { instance, create: vi.fn(() => instance) }
})

vi.mock('axios', () => ({
  default: { create: mocks.create },
}))

import * as api from './api'

const { instance } = mocks
// `api.ts` 在模块加载时就调用了 axios.create，那一刻的实参在任何
// clearAllMocks 之前先取下来，否则会被清掉。
const createArgs = mocks.create.mock.calls[0]

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

describe('axios 实例契约', () => {
  it('baseURL 为 /api、超时 30 秒', () => {
    expect(createArgs).toEqual([{ baseURL: '/api', timeout: 30000 }])
  })
})

describe('数据管理', () => {
  it('getDataFiles 请求 /data/list 并解包 data', async () => {
    const payload = [{ name: 'a.json' }]
    respondWith(payload)

    await expect(api.getDataFiles()).resolves.toBe(payload)
    expect(instance.get).toHaveBeenCalledWith('/data/list')
  })

  it('loadData 把 pageSize 映射为 page_size，并带上 search', async () => {
    respondWith({ items: [] })

    await api.loadData('a.json', 2, 50, '关键词')

    expect(instance.get).toHaveBeenCalledWith('/data/load/a.json', {
      params: { page: 2, page_size: 50, search: '关键词' },
    })
  })

  it('updateDataItem 用 PUT 传请求体、用查询参数传 index', async () => {
    const item = { instruction: 'q', output: 'a' }
    respondWith({ ok: true })

    await api.updateDataItem('a.json', 7, item)

    expect(instance.put).toHaveBeenCalledWith('/data/update/a.json', item, {
      params: { index: 7 },
    })
  })

  it('deleteDataItem 用 DELETE 并带 index 查询参数', async () => {
    respondWith({ ok: true })

    await api.deleteDataItem('a.json', 3)

    expect(instance.delete).toHaveBeenCalledWith('/data/delete/a.json', {
      params: { index: 3 },
    })
  })

  it('uploadData 以 multipart 提交，字段名必须是 file', async () => {
    respondWith({ ok: true })
    const file = new File(['{}'], 'a.json', { type: 'application/json' })

    await api.uploadData(file)

    const [url, body] = instance.post.mock.calls[0]
    expect(url).toBe('/data/upload')
    expect(body).toBeInstanceOf(FormData)
    // 后端按字段名 'file' 取值，改名字就会 422
    expect((body as FormData).get('file')).toBe(file)
  })

  it('exportData 透传 input_file / output_dir / formats', async () => {
    respondWith({ ok: true })

    await api.exportData('in.json', 'out', ['jsonl', 'csv'])

    expect(instance.post).toHaveBeenCalledWith('/data/export', {
      input_file: 'in.json',
      output_dir: 'out',
      formats: ['jsonl', 'csv'],
    })
  })
})

describe('数据增强', () => {
  it('startAugmentation 把 camelCase 参数映射为 snake_case 请求体', async () => {
    respondWith({ task_id: 't1' })

    await api.startAugmentation('in.json', 'out.json', true, false, true)

    expect(instance.post).toHaveBeenCalledWith('/augment/start', {
      input_file: 'in.json',
      output_file: 'out.json',
      use_quality: true,
      use_dedup: false,
      use_checkpoint: true,
    })
  })

  it('getProgress / getCheckpoints 分别请求各自端点', async () => {
    respondWith({})

    await api.getProgress()
    await api.getCheckpoints()

    expect(instance.get).toHaveBeenNthCalledWith(1, '/augment/progress')
    expect(instance.get).toHaveBeenNthCalledWith(2, '/augment/checkpoints')
  })
})

describe('数据分析', () => {
  it('analyzeData / visualizeData 把文件名拼进路径', async () => {
    respondWith({})

    await api.analyzeData('a.json')
    await api.visualizeData('b.json')

    expect(instance.get).toHaveBeenNthCalledWith(1, '/analyze/a.json')
    expect(instance.get).toHaveBeenNthCalledWith(2, '/visualize/b.json')
  })
})

describe('版本管理', () => {
  it('getVersions 请求 /versions', async () => {
    respondWith([])
    await api.getVersions()
    expect(instance.get).toHaveBeenCalledWith('/versions')
  })

  it('createVersion 用查询串传 filename、用请求体传 label/description', async () => {
    respondWith({ id: 'v1' })

    await api.createVersion('a.json', '标签', '说明')

    expect(instance.post).toHaveBeenCalledWith('/versions/create?filename=a.json', {
      label: '标签',
      description: '说明',
    })
  })

  it('getVersion / getVersionData 命中正确的子路径', async () => {
    respondWith({})

    await api.getVersion('v1')
    await api.getVersionData('v1')

    expect(instance.get).toHaveBeenNthCalledWith(1, '/versions/v1')
    expect(instance.get).toHaveBeenNthCalledWith(2, '/versions/v1/data')
  })

  it('diffVersions 传两个版本号', async () => {
    respondWith({})

    await api.diffVersions('v1', 'v2')

    expect(instance.post).toHaveBeenCalledWith('/versions/diff', {
      version1: 'v1',
      version2: 'v2',
    })
  })

  it('rollbackVersion 用 POST 到 rollback 子路径', async () => {
    respondWith({})

    await api.rollbackVersion('v1')

    expect(instance.post).toHaveBeenCalledWith('/versions/v1/rollback')
  })

  it('deleteVersion 用 DELETE', async () => {
    respondWith({})

    await api.deleteVersion('v1')

    expect(instance.delete).toHaveBeenCalledWith('/versions/v1')
  })

  it('getVersionHistory 默认 limit=20，可覆盖', async () => {
    respondWith([])

    await api.getVersionHistory()
    await api.getVersionHistory(5)

    expect(instance.get).toHaveBeenNthCalledWith(1, '/versions/history', { params: { limit: 20 } })
    expect(instance.get).toHaveBeenNthCalledWith(2, '/versions/history', { params: { limit: 5 } })
  })
})

describe('质量评估', () => {
  it('evaluateQuality 默认阈值 0.6', async () => {
    respondWith({})

    await api.evaluateQuality('a.json')

    expect(instance.post).toHaveBeenCalledWith('/quality/evaluate', {
      input_file: 'a.json',
      threshold: 0.6,
    })
  })

  it('dedupData 默认阈值 0.9（比质量阈值更严）', async () => {
    respondWith({})

    await api.dedupData('a.json')

    expect(instance.post).toHaveBeenCalledWith('/quality/dedup', {
      input_file: 'a.json',
      threshold: 0.9,
    })
  })

  it('qualityReport / runBenchmark 默认阈值 0.6，可覆盖', async () => {
    respondWith({})

    await api.qualityReport('a.json')
    await api.runBenchmark('a.json', 0.75)

    expect(instance.post).toHaveBeenNthCalledWith(1, '/quality/report', {
      input_file: 'a.json',
      threshold: 0.6,
    })
    expect(instance.post).toHaveBeenNthCalledWith(2, '/quality/benchmark', {
      input_file: 'a.json',
      threshold: 0.75,
    })
  })

  it('cleanData / annotateData 只传 input_file', async () => {
    respondWith({})

    await api.cleanData('a.json')
    await api.annotateData('a.json')

    expect(instance.post).toHaveBeenNthCalledWith(1, '/quality/clean', { input_file: 'a.json' })
    expect(instance.post).toHaveBeenNthCalledWith(2, '/quality/annotate', { input_file: 'a.json' })
  })
})

describe('导出', () => {
  it('getExportFormats 请求 /export/formats', async () => {
    respondWith([])
    await api.getExportFormats()
    expect(instance.get).toHaveBeenCalledWith('/export/formats')
  })

  it('previewExport 默认 size=5', async () => {
    respondWith({})

    await api.previewExport('a.json', 'alpaca')

    expect(instance.post).toHaveBeenCalledWith('/export/preview', {
      input_file: 'a.json',
      format: 'alpaca',
      size: 5,
    })
  })

  it('batchExport 透传 datasets / output_dir / formats', async () => {
    respondWith({})

    await api.batchExport({ d1: 'a.json' }, 'out', ['jsonl'])

    expect(instance.post).toHaveBeenCalledWith('/export/batch', {
      datasets: { d1: 'a.json' },
      output_dir: 'out',
      formats: ['jsonl'],
    })
  })
})

describe('多模态', () => {
  it('getMultimodalFormats 请求 /multimodal/formats', async () => {
    respondWith([])
    await api.getMultimodalFormats()
    expect(instance.get).toHaveBeenCalledWith('/multimodal/formats')
  })

  it('processMultimodal 同时传 text/image/audio 三个字段', async () => {
    respondWith({})

    await api.processMultimodal('文本', 'img.png', 'a.wav')

    expect(instance.post).toHaveBeenCalledWith('/multimodal/process', {
      text: '文本',
      image: 'img.png',
      audio: 'a.wav',
    })
  })

  it('scanMultimodal 传 directory', async () => {
    respondWith({})

    await api.scanMultimodal('/data/dir')

    expect(instance.post).toHaveBeenCalledWith('/multimodal/scan', { directory: '/data/dir' })
  })
})

describe('配置与健康检查', () => {
  it('getConfig / getModels / healthCheck 各自命中端点', async () => {
    respondWith({})

    await api.getConfig()
    await api.getModels()
    await api.healthCheck()

    expect(instance.get).toHaveBeenNthCalledWith(1, '/config')
    expect(instance.get).toHaveBeenNthCalledWith(2, '/models')
    expect(instance.get).toHaveBeenNthCalledWith(3, '/health')
  })

  it('updateConfig 用 POST 提交整份配置', async () => {
    respondWith({})
    const config = { quality: { threshold: 0.7 } }

    await api.updateConfig(config)

    expect(instance.post).toHaveBeenCalledWith('/config', config)
  })
})

describe('服务状态与演示数据', () => {
  it('getServiceStatus 请求 /status（不是 /health）', async () => {
    respondWith({})

    await api.getServiceStatus()

    expect(instance.get).toHaveBeenCalledWith('/status')
  })

  it('getDemoData 请求 /demo/data 并把裸数组原样返回', async () => {
    // 端点返回的是裸数组，不是 { data: [...] } 包装；解包写错这里会立刻变红
    const payload = [{ instruction: 'q', input: '', output: 'a' }]
    respondWith(payload)

    await expect(api.getDemoData()).resolves.toBe(payload)
    expect(instance.get).toHaveBeenCalledWith('/demo/data')
  })
})

describe('隐私脱敏', () => {
  it('getPiiPatterns 请求 /privacy/patterns', async () => {
    respondWith({})

    await api.getPiiPatterns()

    expect(instance.get).toHaveBeenCalledWith('/privacy/patterns')
  })

  it('sanitizeData 默认不传 fields、include_extra 为 false', async () => {
    respondWith({})

    await api.sanitizeData('a.json')

    expect(instance.post).toHaveBeenCalledWith('/privacy/sanitize', {
      input_file: 'a.json',
      fields: undefined,
      include_extra: false,
    })
  })

  it('sanitizeData 可指定 fields 并打开增强模式', async () => {
    respondWith({})

    await api.sanitizeData('a.json', ['instruction'], true)

    expect(instance.post).toHaveBeenCalledWith('/privacy/sanitize', {
      input_file: 'a.json',
      fields: ['instruction'],
      include_extra: true,
    })
  })
})

describe('泄漏检测与就绪审计', () => {
  it('checkLeakage 传 train_file / test_file，模糊阈值默认 0.8', async () => {
    respondWith({})

    await api.checkLeakage('train.json', 'test.json')

    expect(instance.post).toHaveBeenCalledWith('/leakage/check', {
      train_file: 'train.json',
      test_file: 'test.json',
      fuzzy_threshold: 0.8,
    })
  })

  it('checkLeakage 的模糊阈值可覆盖', async () => {
    respondWith({})

    await api.checkLeakage('train.json', 'test.json', 0.5)

    expect(instance.post).toHaveBeenCalledWith('/leakage/check', {
      train_file: 'train.json',
      test_file: 'test.json',
      fuzzy_threshold: 0.5,
    })
  })

  it('auditDataset 不传参照集时 reference_file 为 undefined', async () => {
    respondWith({})

    await api.auditDataset('a.json')

    expect(instance.post).toHaveBeenCalledWith('/audit', {
      input_file: 'a.json',
      reference_file: undefined,
    })
  })

  it('auditDataset 可带参照集', async () => {
    respondWith({})

    await api.auditDataset('a.json', 'ref.json')

    expect(instance.post).toHaveBeenCalledWith('/audit', {
      input_file: 'a.json',
      reference_file: 'ref.json',
    })
  })
})

describe('离群点与画像', () => {
  it('detectOutliers 默认 method=zscore、threshold=3、field=length', async () => {
    respondWith({})

    await api.detectOutliers('a.json')

    expect(instance.post).toHaveBeenCalledWith('/quality/outliers', {
      input_file: 'a.json',
      method: 'zscore',
      threshold: 3,
      field: 'length',
    })
  })

  it('detectOutliers 的三个参数都可覆盖', async () => {
    respondWith({})

    await api.detectOutliers('a.json', 'iqr', 1.5, 'output')

    expect(instance.post).toHaveBeenCalledWith('/quality/outliers', {
      input_file: 'a.json',
      method: 'iqr',
      threshold: 1.5,
      field: 'output',
    })
  })

  it('profileDataset 只传 input_file（save 交给后端默认值 false）', async () => {
    respondWith({})

    await api.profileDataset('a.json')

    expect(instance.post).toHaveBeenCalledWith('/quality/profiling', {
      input_file: 'a.json',
    })
  })
})
