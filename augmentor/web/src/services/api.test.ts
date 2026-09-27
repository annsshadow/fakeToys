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
    // 不声明格式时传空串：后端 `Form("")` 的默认值就是空串，两种写法同一条代码路径
    expect((body as FormData).get('input_format')).toBe('')
  })

  it('uploadData 把显式声明的源格式放进 input_format', async () => {
    // 这一格钉的是「UI 能选到的那档真的发得出去」：后端「显式声明 > 扩展名」，
    // 而 `<input type=file>` 只能给裸文件名，无扩展名/扩展名与内容不符的进料
    // 全靠这个字段救（A132 ③ 的产品面）。字段名拼错 = 后端默默按扩展名走 = 静默失效。
    respondWith({ ok: true })
    const file = new File(['a,b\n1,2'], 'mystery.dat', { type: 'application/octet-stream' })

    await api.uploadData(file, 'csv')

    const body = instance.post.mock.calls[0][1] as FormData
    expect(body.get('input_format')).toBe('csv')
  })

  it('exportData 透传 input_file / output_dir / formats', async () => {
    respondWith({ ok: true })

    await api.exportData('in.json', 'out', ['jsonl', 'csv'])

    expect(instance.post).toHaveBeenCalledWith(
      '/data/export',
      {
        input_file: 'in.json',
        output_dir: 'out',
        formats: ['jsonl', 'csv'],
      },
      { timeout: api.BULK_REQUEST_TIMEOUT_MS }
    )
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

    expect(instance.post).toHaveBeenCalledWith(
      '/export/batch',
      {
        datasets: { d1: 'a.json' },
        output_dir: 'out',
        formats: ['jsonl'],
      },
      { timeout: api.BULK_REQUEST_TIMEOUT_MS }
    )
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

/**
 * `apiErrorDetail`：把后端的判决文案取回给用户
 *
 * 这一层是唯一有分支的服务层函数，所以它的用例不是「覆盖行」而是**形状对账**：
 * FastAPI 的失败体有两种 detail（4xx 字符串、422 数组），而「取不到就走 fallback」
 * 那一支必须也在内 —— 页面里所有 `catch` 现在都过这个函数，它编出一个
 * `undefined` 或 `[object Object]` 会比原来那句固定文案更糟。
 */
describe('apiErrorDetail', () => {
  /** 造一份 axios 风格的错误：只填 `response.data`，其余一概不需要 */
  const httpError = (data: unknown) => ({ response: { data } })

  it('4xx 的字符串 detail 原样返回，一个字都不改', () => {
    const detail =
      '上传内容整份是一份合法 JSON，但按 csv 解析：名字或 input_format 与内容不符'

    expect(api.apiErrorDetail(httpError({ detail }), '上传失败')).toBe(detail)
  })

  it('422 的 loc/msg 数组取每条 msg 并接起来', () => {
    const err = httpError({
      detail: [
        { type: 'missing', loc: ['body', 'file'], msg: 'Field required' },
        { type: 'extra', loc: ['body', 'oops'], msg: 'Extra inputs' },
      ],
    })

    expect(api.apiErrorDetail(err, '上传失败')).toBe('Field required；Extra inputs')
  })

  it('空字符串 detail 走 fallback（后端回 400 但没文案时不说出空气泡）', () => {
    expect(api.apiErrorDetail(httpError({ detail: '' }), '加载数据失败')).toBe(
      '加载数据失败'
    )
  })

  it('detail 是非字符串非数组（对象、数字）时走 fallback', () => {
    expect(api.apiErrorDetail(httpError({ detail: { code: 7 } }), '保存失败')).toBe(
      '保存失败'
    )
    expect(api.apiErrorDetail(httpError({ detail: 42 }), '删除失败')).toBe('删除失败')
  })

  it('数组里混着没有 msg 的条目：只取字符串 msg，全都没有就 fallback', () => {
    expect(
      api.apiErrorDetail(httpError({ detail: [{ loc: ['body'] }, { msg: '坏字段' }] }), '失败')
    ).toBe('坏字段')
    expect(api.apiErrorDetail(httpError({ detail: [{ loc: ['body'] }] }), '失败')).toBe('失败')
    expect(api.apiErrorDetail(httpError({ detail: [] }), '失败')).toBe('失败')
  })

  it('没有 response 的错误（网络断开、超时、原生 Error）走 fallback', () => {
    expect(api.apiErrorDetail(new Error('timeout of 30000ms exceeded'), '上传失败')).toBe(
      '上传失败'
    )
    expect(api.apiErrorDetail(undefined, '上传失败')).toBe('上传失败')
    expect(api.apiErrorDetail('字符串也能接住', '上传失败')).toBe('上传失败')
  })

  it('响应体是字符串（代理返回的 HTML/纯文本）不会崩', () => {
    expect(
      api.apiErrorDetail(httpError('<html>502 Bad Gateway</html>'), '导出失败')
    ).toBe('导出失败')
  })
})

/**
 * 大块请求的超时档（A167）
 *
 * 这一组钉的不是「参数存在」，而是**一条实测出来的边界**：上传端到端约 0.95 s/MiB，
 * 而实例默认档只有 30 s ⇒ 31 MiB 上下的合法进料（服务端闸是 256 MiB）会被前端自己掐断。
 * 所以三条批量请求必须各带自己的 `timeout`，其余短请求必须**不**跟着放宽 ——
 * 后半句由上面那条实例契约（`timeout: 30000`）守住。
 */
describe('大块请求的每请求超时', () => {
  it('档位是 30 分钟，且远大于实例默认档', () => {
    // 30 min = 闸顶 256 MiB × 实测最慢的 5.2 s/MiB（xlsx ≈ 22 min）再留余量；
    // 这里钉数量级， derivation 的原文在 api.ts 的注释里。
    expect(api.BULK_REQUEST_TIMEOUT_MS).toBe(30 * 60 * 1000)
    expect(api.BULK_REQUEST_TIMEOUT_MS).toBeGreaterThan(30000)
  })

  it('uploadData 把 timeout 作为第三个实参交给 axios', async () => {
    respondWith({ ok: true })

    await api.uploadData(new File(['{}'], 'a.json', { type: 'application/json' }))

    const config = instance.post.mock.calls[0][2] as { timeout: number }
    expect(config.timeout).toBe(api.BULK_REQUEST_TIMEOUT_MS)
  })

  it('exportData / batchExport 同样带档', async () => {
    respondWith({})

    await api.exportData('a.json', 'out', ['jsonl'])
    await api.batchExport({ d: 'a.json' }, 'out', ['jsonl'])

    expect(instance.post.mock.calls[0][2]).toEqual({ timeout: api.BULK_REQUEST_TIMEOUT_MS })
    expect(instance.post.mock.calls[1][2]).toEqual({ timeout: api.BULK_REQUEST_TIMEOUT_MS })
  })

  it('短请求（列表 / 状态）不传 per-request config，继续吃 30 s 默认档', async () => {
    respondWith({})

    await api.getDataFiles()
    await api.getServiceStatus()

    expect(instance.post.mock.calls).toHaveLength(0)
    expect(instance.get.mock.calls[0]).toEqual(['/data/list'])
    expect(instance.get.mock.calls[1]).toEqual(['/status'])
  })
})

/**
 * `isTimeoutError`：把「客户端等不及」和「服务端答错」分开
 *
 * 分开是因为这两件事的后果相反：答错要改输入，等不及可能什么都没错
 * （实测：客户端在 0.60 s 放弃后，服务端照样写完了那份 45 840 001 B 的文件）。
 *
 * 错误对象用 `vi.importActual('axios')` 取**真身** `AxiosError` 来造，形状照
 * `lib/adapters/xhr.js` 的 ontimeout 与 `lib/helpers/composeSignals.js` 的 fetch 档，
 * 而不是我自己编一个 `{ code, message }`：编的形状只能自证，真身的常量若改名会在这里变红。
 */
describe('isTimeoutError', () => {
  /** 真 axios 模块（本文件顶部把 axios 整体换成了替身，所以走 importActual） */
  const realAxios = async () =>
    (await vi.importActual('axios')) as unknown as {
      AxiosError: {
        new (message: string, code?: string): Error
        ECONNABORTED: string
        ETIMEDOUT: string
      }
    }

  it('axios 的两个超时常量与本判据用的是同一串字面量', async () => {
    const { AxiosError } = await realAxios()

    // 若哪天 axios 改了 code 字面量，这条先红，而不是让下面的用例和判据一起自证。
    expect(AxiosError.ECONNABORTED).toBe('ECONNABORTED')
    expect(AxiosError.ETIMEDOUT).toBe('ETIMEDOUT')
  })

  it('xhr/http 档的超时（ECONNABORTED + `timeout of Nms exceeded`）判为超时', async () => {
    const { AxiosError } = await realAxios()
    // 照抄 xhr.js：`'timeout of ' + config.timeout + 'ms exceeded'` + ECONNABORTED
    const err = new AxiosError('timeout of 1800000ms exceeded', 'ECONNABORTED')

    expect(api.isTimeoutError(err)).toBe(true)
  })

  it('fetch 档开了 clarifyTimeoutError（ETIMEDOUT）也判为超时', async () => {
    const { AxiosError } = await realAxios()
    const err = new AxiosError('timeout of 1800000ms exceeded', 'ETIMEDOUT')

    expect(api.isTimeoutError(err)).toBe(true)
  })

  it('用户主动中止同为 ECONNABORTED，但不是超时（这是判据不看 code 就完蛋的那格）', async () => {
    const { AxiosError } = await realAxios()
    // xhr.js 的 abort 分支给的是同一枚 code，只有 message 能把两者分开
    const err = new AxiosError('Request aborted', 'ECONNABORTED')

    expect(api.isTimeoutError(err)).toBe(false)
  })

  it('带 response 的 4xx/5xx 不是超时；没有 code 的原生 Error、null、字符串也不是', () => {
    expect(
      api.isTimeoutError({ response: { status: 504, data: {} }, code: 'ERR_BAD_RESPONSE' })
    ).toBe(false)
    expect(api.isTimeoutError(new Error('timeout of 30000ms exceeded'))).toBe(false)
    expect(api.isTimeoutError(null)).toBe(false)
    expect(api.isTimeoutError(undefined)).toBe(false)
    expect(api.isTimeoutError('timeout')).toBe(false)
    // 对象上没有 message 时不能把 String(undefined) 当成匹配
    expect(api.isTimeoutError({ code: 'ECONNABORTED' })).toBe(false)
  })
})

/**
 * `bulkErrorDetail`：超时说人话，其余照旧把后端的判决透出来
 *
 * 三条路径都要点到：超时走新文案、有 detail 仍走 detail（这次改动不能把 A162 的成果吃掉）、
 * 没有 detail 也没有超时仍走裸 fallback（不能把「刷新确认」那句撒到所有失败上）。
 */
describe('bulkErrorDetail', () => {
  const timeout = { code: 'ECONNABORTED', message: 'timeout of 1800000ms exceeded' }

  it('超时：先复述是哪件事，再说清「服务端可能已经落盘、别急着重试」', () => {
    const text = api.bulkErrorDetail(timeout, '上传失败')

    expect(text).toContain('上传失败')
    expect(text).toContain('客户端已停止等待')
    expect(text).toContain('已把结果落盘')
    expect(text).toContain('请先确认结果再重试')
  })

  it('非超时且有 detail：原样端出后端那句判决（不吃掉 A162 的收口）', () => {
    const detail = '文件超过 256 MiB 上限'

    expect(api.bulkErrorDetail({ response: { data: { detail } } }, '上传失败')).toBe(detail)
  })

  it('非超时且无 detail：只有裸 fallback，不混进超时文案', () => {
    expect(api.bulkErrorDetail(new Error('Network Error'), '导出失败')).toBe('导出失败')
    expect(api.bulkErrorDetail(undefined, '导出失败')).toBe('导出失败')
  })
})
