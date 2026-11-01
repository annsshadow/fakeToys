/**
 * `DataManagement` 的采样 / 聚合 / RAG 面（L230：A279 第六批接线）
 *
 * 存在理由与 L229 那份同构：Python 侧的 `test_sample_aggregate_rag_wiring_l230.py` 只能证明
 * **源码写了**那份清单与那几句口径，证不出渲染面上真的看得见。这一份钉四件事：
 * ① 三张下拉渲染的项数与初值（方法三项、策略四项、格式「服务端默认」+ 三项）；
 * ② 每个按钮的必填闸、成功回执与失败判决三路各有可见读数——失败时用户看见的是服务端那句
 *    判决原话（L277 起 `ratio` 越界是那句中文区间判决），不是页面自己编的「失败了」；
 * ③ 聚合卡的来源名是页面起的：勾上就出现一个初值等于 stem 的输入框，重名在**发之前**被拦下
 *    （重名会让映射互相覆盖、回执的 `source_counts` 少一格，那是静默丢源）；
 * ④ 五条实测过的**诚实口径**留在卡上（A292 / A293 / A294 / A295 / A296 / A297）。
 *
 * 服务层用 `importOriginal` 做部分替身，`apiErrorDetail` 留真身，理由同上传与转换那两份用例。
 */
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'
import { cleanup, fireEvent, render, screen, waitFor } from '@testing-library/react'
import { message } from 'antd'

window.matchMedia =
  window.matchMedia ||
  ((query: string) =>
    ({
      matches: false,
      media: query,
      onchange: null,
      addListener: () => {},
      removeListener: () => {},
      addEventListener: () => {},
      removeEventListener: () => {},
      dispatchEvent: () => false,
    }) as unknown as MediaQueryList)
class RO {
  observe() {}
  unobserve() {}
  disconnect() {}
}
window.ResizeObserver = window.ResizeObserver || (RO as unknown as typeof ResizeObserver)

/**
 * 夹具与替身同住 `vi.hoisted`，工厂内只用 `fx.xxx` 属性访问（理由见转换那份用例的同名注释）
 *
 * 三份夹具都是**探针在真实端点上量到的形状**，不是随手编的：`AGG_CONFLICT` 逐字来自
 * consistent 策略打两个冲突源的回执（`sources` 里同一个源出现两次），`RAG_OK.format` 是
 * 「省略那一格」时服务端回的**生效值**，`SAMPLE_OK.output_count: 0` 是 `size=0` 那一档
 * 的真实产物（200 + 一份 `[]`）。
 */
const fx = vi.hoisted(() => {
  const SAMPLE_OK = {
    input_file: 'D:\\data\\a.json',
    output_file: 'D:\\data\\sampled.json',
    input_count: 20,
    output_count: 8,
  }
  const SAMPLE_EMPTY = { ...SAMPLE_OK, output_file: 'D:\\data\\zero.json', output_count: 0 }
  const AGG_CONFLICT = {
    output_file: 'D:\\data\\agg.json',
    aggregated_count: 0,
    source_counts: { 甲: 2, 乙: 2 },
    removed_duplicates: 0,
    conflicts: [{ key: '同一个问题', values: ['答案甲', '答案乙'], sources: ['甲', '乙'] }],
  }
  const AGG_OK = {
    output_file: 'D:\\data\\agg.json',
    aggregated_count: 20,
    source_counts: { 甲: 20, 乙: 6 },
    removed_duplicates: 6,
    conflicts: [],
  }
  const RAG_OK = {
    output_file: 'D:\\data\\rag.json',
    format: 'llamaindex',
    input_count: 20,
    record_count: 20,
  }
  return {
    SAMPLE_OK,
    SAMPLE_EMPTY,
    AGG_CONFLICT,
    AGG_OK,
    RAG_OK,
    spies: {
      getDataFiles: null as unknown as ReturnType<typeof vi.fn>,
      loadData: null as unknown as ReturnType<typeof vi.fn>,
      sampleDataset: null as unknown as ReturnType<typeof vi.fn>,
      aggregateDatasets: null as unknown as ReturnType<typeof vi.fn>,
      convertToRag: null as unknown as ReturnType<typeof vi.fn>,
    },
  }
})

vi.mock('../services/api', async (importOriginal) => {
  const actual = await importOriginal<typeof import('../services/api')>()
  fx.spies.getDataFiles = vi
    .fn()
    .mockResolvedValue({ files: [{ name: 'a.json' }, { name: 'b.jsonl' }] })
  fx.spies.loadData = vi.fn().mockResolvedValue({ items: [], total: 0 })
  fx.spies.sampleDataset = vi.fn().mockResolvedValue(fx.SAMPLE_OK)
  fx.spies.aggregateDatasets = vi.fn().mockResolvedValue(fx.AGG_OK)
  fx.spies.convertToRag = vi.fn().mockResolvedValue(fx.RAG_OK)
  return {
    ...actual,
    getDataFiles: fx.spies.getDataFiles,
    loadData: fx.spies.loadData,
    sampleDataset: fx.spies.sampleDataset,
    aggregateDatasets: fx.spies.aggregateDatasets,
    convertToRag: fx.spies.convertToRag,
  }
})

import DataManagement from './DataManagement'

const { SAMPLE_OK, SAMPLE_EMPTY, AGG_CONFLICT, AGG_OK, RAG_OK, spies } = fx

afterEach(() => {
  cleanup()
  message.destroy()
  spies.sampleDataset.mockClear().mockResolvedValue(SAMPLE_OK)
  spies.aggregateDatasets.mockClear().mockResolvedValue(AGG_OK)
  spies.convertToRag.mockClear().mockResolvedValue(RAG_OK)
})

beforeEach(() => {
  spies.getDataFiles.mockClear()
})

/** 等文件列表落地再点按钮（`selectedFile` 是异步设成 `files[0]` 的，理由见转换那份用例） */
async function waitFiles(container: HTMLElement) {
  await waitFor(
    () => expect((container.querySelector('select') as HTMLSelectElement)?.value).toBe('a.json'),
    { timeout: 10000 },
  )
}

/** antd 的 `Button loading` 会往可及名前面插一个图标名，所以一律按子串正则点 */
const click = (name: string) =>
  fireEvent.click(screen.getByRole('button', { name: new RegExp(name) }))

/** 回执不是表单控件，按属性取；读数仍取自它的 textContent */
const labeled = (name: string) => document.querySelector(`[aria-label="${name}"]`)

/**
 * 聚合卡的勾选框：可及名带「聚合来源」前缀
 *
 * 页面上同时有合并卡的勾选框（可及名就是文件名）与聚合卡的勾选框。给聚合那一组加前缀是
 * **刻意**的：不加就有两组同名控件，转换那份用例里的 `getByRole('checkbox', { name: 'a.json' })`
 * 会当场撞上「找到两个」而红——那不是它写错了，是这一批接线引入了同名控件。
 */
const aggCheck = (name: string) =>
  fireEvent.click(screen.getByRole('checkbox', { name: `聚合来源 ${name}` }))

describe('采样卡的渲染面', () => {
  it('方法渲染三项、初值 random；分层字段初值 instruction；三个数值格留空', () => {
    render(<DataManagement />)

    const method = screen.getByLabelText('采样方法')
    expect(Array.from(method.querySelectorAll('option')).map((o) => o.value)).toEqual([
      'random',
      'systematic',
      'stratified',
    ])
    expect(method).toHaveProperty('value', 'random')
    expect(screen.getByLabelText('采样分层字段名')).toHaveProperty('value', 'instruction')
    // 数值格留空 = 那一格不发（契约默认是 None），页面上不许自带一套数字默认
    expect(screen.getByLabelText('采样条数')).toHaveProperty('value', '')
    expect(screen.getByLabelText('采样比例')).toHaveProperty('value', '')
    expect(screen.getByLabelText('采样随机种子')).toHaveProperty('value', '')
  })

  it('点采样发的是选中文件 + 点名的输出 + 方法与分层字段，三个数值格是 undefined', async () => {
    const { container } = render(<DataManagement />)
    await waitFiles(container)

    fireEvent.change(screen.getByLabelText('采样输出文件名'), {
      target: { value: 'sampled.json' },
    })
    click('采样选中的文件')

    await waitFor(() =>
      expect(spies.sampleDataset).toHaveBeenCalledWith('a.json', 'sampled.json', {
        method: 'random',
        size: undefined,
        ratio: undefined,
        seed: undefined,
        stratifyKey: 'instruction',
      }),
    )
    await waitFor(() => expect(labeled('采样回执')?.textContent).toContain('20 条进'))
    expect(labeled('采样回执')?.textContent).toContain('8 条出')
  })

  it('条数填 0 是要发出去的 0，回执照实报「0 条出」而不是把一份空文件藏起来', async () => {
    spies.sampleDataset.mockResolvedValue(SAMPLE_EMPTY)
    const { container } = render(<DataManagement />)
    await waitFiles(container)

    fireEvent.change(screen.getByLabelText('采样条数'), { target: { value: '0' } })
    fireEvent.change(screen.getByLabelText('采样输出文件名'), { target: { value: 'zero.json' } })
    click('采样选中的文件')

    await waitFor(() => expect(spies.sampleDataset).toHaveBeenCalled())
    expect(spies.sampleDataset.mock.calls[0][2].size).toBe(0)
    await waitFor(() => expect(labeled('采样回执')?.textContent).toContain('0 条出'))
  })

  it('采样失败显示服务端那句判决原文（页面不翻译、不改写，也不自己编一句）', async () => {
    // 这句照抄 L277 探针在真实端点上量到的回执：三种 method 同一句中文区间判决。
    // 改前这里 mock 的是 CPython 的英文原话「Sample larger than population or is negative」，
    // 那一格随 A292 的范围维一起收掉了——页面仍然只转述服务端的话，只是话换了。
    spies.sampleDataset.mockRejectedValue({
      response: { data: { detail: 'ratio 必须是 0.0 到 1.0 之间的比例，当前是 2.0' } },
    })
    const { container } = render(<DataManagement />)
    await waitFiles(container)

    fireEvent.change(screen.getByLabelText('采样比例'), { target: { value: '2' } })
    fireEvent.change(screen.getByLabelText('采样输出文件名'), { target: { value: 's.json' } })
    click('采样选中的文件')

    await waitFor(
      () =>
        expect(document.body.textContent).toContain(
          'ratio 必须是 0.0 到 1.0 之间的比例，当前是 2.0',
        ),
      { timeout: 10000 },
    )
    expect(labeled('采样回执')).toBeNull()
  })

  it('没点名输出文件时点对采样不发请求，只给一句要什么的提示', async () => {
    const { container } = render(<DataManagement />)
    await waitFiles(container)

    click('采样选中的文件')

    await waitFor(() => expect(document.body.textContent).toContain('点名输出文件名'), {
      timeout: 10000,
    })
    expect(spies.sampleDataset).not.toHaveBeenCalled()
  })
})

describe('聚合卡的渲染面', () => {
  it('策略渲染四项、初值 union；目标条数留空即不发那一格', async () => {
    const { container } = render(<DataManagement />)
    await waitFiles(container)

    const strategy = screen.getByLabelText('聚合策略')
    expect(Array.from(strategy.querySelectorAll('option')).map((o) => o.value)).toEqual([
      'union',
      'intersection',
      'weighted',
      'consistent',
    ])
    expect(strategy).toHaveProperty('value', 'union')

    aggCheck('a.json')
    fireEvent.change(screen.getByLabelText('聚合输出文件名'), { target: { value: 'agg.json' } })
    click('聚合勾选的来源')

    await waitFor(() => expect(spies.aggregateDatasets).toHaveBeenCalled())
    const [, , opts] = spies.aggregateDatasets.mock.calls[0]
    expect(opts).toEqual({ strategy: 'union', targetSize: undefined })
  })

  it('勾上就出现来源名输入框，初值是文件名去掉扩展名；发出去的是「名字 → 文件」映射', async () => {
    const { container } = render(<DataManagement />)
    await waitFiles(container)

    aggCheck('a.json')
    aggCheck('b.jsonl')
    expect(screen.getByLabelText('来源名 a.json')).toHaveProperty('value', 'a')
    expect(screen.getByLabelText('来源名 b.jsonl')).toHaveProperty('value', 'b')

    fireEvent.change(screen.getByLabelText('聚合输出文件名'), { target: { value: 'agg.json' } })
    click('聚合勾选的来源')

    await waitFor(() =>
      expect(spies.aggregateDatasets).toHaveBeenCalledWith(
        { a: 'a.json', b: 'b.jsonl' },
        'agg.json',
        { strategy: 'union', targetSize: undefined },
      ),
    )
    await waitFor(() => expect(labeled('聚合回执')?.textContent).toContain('20 条'))
    expect(labeled('聚合回执')?.textContent).toContain('删掉 6 条')
    expect(labeled('聚合回执')?.textContent).toContain('甲 20 条 / 乙 6 条')
  })

  it('两个来源改成同一个名字 ⇒ 发之前就拦下，因为重名会让映射互相覆盖', async () => {
    const { container } = render(<DataManagement />)
    await waitFiles(container)

    aggCheck('a.json')
    aggCheck('b.jsonl')
    fireEvent.change(screen.getByLabelText('来源名 b.jsonl'), { target: { value: 'a' } })
    fireEvent.change(screen.getByLabelText('聚合输出文件名'), { target: { value: 'agg.json' } })
    click('聚合勾选的来源')

    await waitFor(() => expect(document.body.textContent).toContain('重复'), { timeout: 10000 })
    expect(spies.aggregateDatasets).not.toHaveBeenCalled()
  })

  it('一个来源都没勾时点对聚合不发请求', async () => {
    const { container } = render(<DataManagement />)
    await waitFiles(container)

    fireEvent.change(screen.getByLabelText('聚合输出文件名'), { target: { value: 'agg.json' } })
    click('聚合勾选的来源')

    await waitFor(() => expect(document.body.textContent).toContain('至少要勾一个来源'), {
      timeout: 10000,
    })
    expect(spies.aggregateDatasets).not.toHaveBeenCalled()
  })

  it('consistent 的冲突明细逐格渲出来：key / values / sources 一个都不许吞（A297）', async () => {
    spies.aggregateDatasets.mockResolvedValue(AGG_CONFLICT)
    const { container } = render(<DataManagement />)
    await waitFiles(container)

    aggCheck('a.json')
    fireEvent.change(screen.getByLabelText('聚合输出文件名'), { target: { value: 'agg.json' } })
    click('聚合勾选的来源')

    await waitFor(() => expect(labeled('聚合回执')).not.toBeNull())
    const receipt = labeled('聚合回执')?.textContent || ''
    expect(receipt).toContain('落盘 0 条')
    expect(receipt).toContain('一致性冲突 1 处')
    expect(receipt).toContain('同一个问题')
    expect(receipt).toContain('答案甲 / 答案乙')
    expect(receipt).toContain('来源 甲 / 乙')
    expect(receipt).toContain('没有')
  })

  it('聚合失败显示服务端的 400 判决（不支持的聚合策略）', async () => {
    spies.aggregateDatasets.mockRejectedValue({
      response: { data: { detail: '不支持的聚合策略: nope' } },
    })
    const { container } = render(<DataManagement />)
    await waitFiles(container)

    aggCheck('a.json')
    fireEvent.change(screen.getByLabelText('聚合输出文件名'), { target: { value: 'agg.json' } })
    click('聚合勾选的来源')

    // 10000 而不是文件里惯常的 4000：这一条走的是**拒绝路径**（antd message.warning），
    // 在 4 核 CI runner 上挂载 + waitFiles + 三次交互很容易吃掉 4s 预算
    // （2026-10-22 实测 5574ms 超时）。断言本身不变，只是把预算给够。
    await waitFor(() => expect(document.body.textContent).toContain('不支持的聚合策略'), {
      timeout: 10000,
    })
    expect(labeled('聚合回执')).toBeNull()
  })
})

describe('RAG 卡的渲染面', () => {
  it('格式渲染「服务端默认」加三项，初值是空串那一档', () => {
    render(<DataManagement />)

    const format = screen.getByLabelText('RAG 格式')
    const options = Array.from(format.querySelectorAll('option')).map((o) => o.value)
    expect(options).toEqual(['', 'llamaindex', 'langchain', 'custom'])
    expect(format).toHaveProperty('value', '')
  })

  it('留空即把空串交给服务层丢；回执报的是服务端回的**生效**格式', async () => {
    const { container } = render(<DataManagement />)
    await waitFiles(container)

    fireEvent.change(screen.getByLabelText('RAG 输出文件名'), { target: { value: 'rag.json' } })
    click('转成 RAG 格式')

    await waitFor(() =>
      expect(spies.convertToRag).toHaveBeenCalledWith('a.json', 'rag.json', { format: '' }),
    )
    await waitFor(() => expect(labeled('RAG 回执')).not.toBeNull())
    const receipt = labeled('RAG 回执')?.textContent || ''
    expect(receipt).toContain('llamaindex')
    expect(receipt).toContain('20 条记录')
    expect(receipt).toContain('输入 20 条')
    expect(receipt).toContain('rag.json')
  })

  it('选了 llamaindex 就发那一档，回执仍照抄服务端回的名字', async () => {
    const { container } = render(<DataManagement />)
    await waitFiles(container)

    fireEvent.change(screen.getByLabelText('RAG 格式'), { target: { value: 'langchain' } })
    fireEvent.change(screen.getByLabelText('RAG 输出文件名'), { target: { value: 'rag.json' } })
    click('转成 RAG 格式')

    await waitFor(() =>
      expect(spies.convertToRag).toHaveBeenCalledWith('a.json', 'rag.json', {
        format: 'langchain',
      }),
    )
    // 夹具回的是 llamaindex（服务端说哪档就是哪档）：页面不许拿自己的选择盖掉回执
    await waitFor(() => expect(labeled('RAG 回执')?.textContent).toContain('llamaindex'))
  })

  it('RAG 转换失败显示服务端那句把三项列全的判决', async () => {
    spies.convertToRag.mockRejectedValue({
      response: {
        data: { detail: "不支持的 RAG 格式: nope。支持: ['llamaindex', 'langchain', 'custom']" },
      },
    })
    const { container } = render(<DataManagement />)
    await waitFiles(container)

    fireEvent.change(screen.getByLabelText('RAG 输出文件名'), { target: { value: 'rag.json' } })
    click('转成 RAG 格式')

    await waitFor(() => expect(document.body.textContent).toContain('不支持的 RAG 格式'), {
      timeout: 10000,
    })
    expect(labeled('RAG 回执')).toBeNull()
  })
})

describe('三张卡上的诚实口径', () => {
  /**
   * 每一句都是探针在真实端点上量到的静默形状：少了它们，用户会把「塌成一个分组」读成
   * 「分层生效了」、把「填了目标条数」读成「任何策略都会截断」。
   */
  it('采样的越界判决、分层折叠与 seed 语义都写在卡上', () => {
    render(<DataManagement />)

    const page = document.body.textContent || ''
    expect(page).toContain('条数赢')
    expect(page).toContain('一律 400')
    expect(page).toContain('一种结局')
    expect(page).not.toContain('没有判据')
    expect(page).toContain('塌成一个分组')
    expect(page).toContain('前 10 个字符')
    expect(page).toContain('A292')
    expect(page).toContain('A296')
  })

  it('聚合的三个不可设旋钮、空转的目标条数与同源自比都写在卡上', () => {
    render(<DataManagement />)

    const page = document.body.textContent || ''
    expect(page).toContain('不可复现')
    expect(page).toContain('完全空转')
    expect(page).toContain('同一个源内部')
    expect(page).toContain('A293')
    expect(page).toContain('A294')
    expect(page).toContain('A297')
  })

  it('RAG 的配置侧旋钮与第四份默认字面量都写在卡上', () => {
    render(<DataManagement />)

    const page = document.body.textContent || ''
    expect(page).toContain('只从服务端配置读')
    expect(page).toContain('静默忽略')
    expect(page).toContain('第四份默认字面量')
    expect(page).toContain('A295')
  })

  it('三条都是写盘端点：回执只在点完之后出现，页面上没有「预览落盘内容」这种东西', async () => {
    const { container } = render(<DataManagement />)
    await waitFiles(container)

    for (const name of ['采样回执', '聚合回执', 'RAG 回执']) {
      expect(labeled(name)).toBeNull()
    }
    expect(document.body.textContent).not.toContain('预览')

    fireEvent.change(screen.getByLabelText('采样输出文件名'), { target: { value: 's.json' } })
    click('采样选中的文件')
    await waitFor(() => expect(labeled('采样回执')).not.toBeNull())
  })
})
