/**
 * `DataManagement` 的写盘变换面（L229：convert / merge / split 接线）
 *
 * 存在理由是把 A279 第五批的三件事钉在渲染面上，而不是只钉在源码文本里：
 * ① 两张格式下拉渲染的**确实是**读边十三项与写边十二项（`xls` 只在源那张表里出现），
 *    Python 侧的 `test_transform_wiring_l229.py` 只能证明源码写了那份清单，证不出渲染；
 * ② 每个按钮的必填闸、成功回执与失败判决三路各有可见读数——失败时用户看见的是
 *    服务端那句判决（`路径包含非法组件` / `分割比例之和必须为1.0，当前为 …`），
 *    不是页面自己编的一句「失败了」；
 * ③ 两条实测过的**诚实口径**留在卡上：分层字段给不存在的名字不报错（塌成一个分组），
 *    比例容差与负数判据两条支路不同（账本 A289）。
 *
 * 服务层用 `importOriginal` 做部分替身，`apiErrorDetail` 留真身，理由同上传那份用例。
 */
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'
import { cleanup, fireEvent, render, screen, waitFor } from '@testing-library/react'
import { message } from 'antd'

window.matchMedia =
  window.matchMedia ||
  ((query: string) => ({
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
 * 夹具与替身**同住** `vi.hoisted`，且工厂内只能用 `fx.xxx` 属性访问
 *
 * `vi.mock` 的工厂会被提到文件最顶部执行；`import DataManagement` 同样是提升的 ESM
 * import，所以工厂运行时文件里的顶层 `const` 一个都还没初始化——实测两次红：
 * 先是 `Cannot access 'CONVERT_OK' before initialization`，再把常量搬进 `vi.hoisted` 后
 * 又报 `Cannot access 'spies' before initialization`（因为解构赋值本身也是顶层语句）。
 * 处方：顶层只留 `const fx = vi.hoisted(...)` 这一条被提前的声明，工厂里全部走属性访问；
 * 测试体内再用解构别名（那时顶层语句早已执行完）。
 */
const fx = vi.hoisted(() => {
  const CONVERT_OK = {
    input_file: 'D:\\data\\a.json',
    output_file: 'D:\\data\\out.jsonl',
    source_format: 'json',
    target_format: 'jsonl',
    input_count: 6,
    output_count: 6,
  }
  const MERGE_OK = {
    input_files: 2,
    output_file: 'D:\\data\\merged.json',
    total_input: 10,
    total_output: 8,
    removed_duplicates: 2,
  }
  // 实测形状：三段键**恒在**，`test` 那一段是 `count: 0` 而不是缺键。
  const SPLIT_OK = {
    input_file: 'D:\\data\\a.json',
    output_dir: 'D:\\data\\s1',
    splits: {
      train: { file: 'D:\\data\\s1\\train.json', count: 4 },
      val: { file: 'D:\\data\\s1\\val.json', count: 1 },
      test: { file: 'D:\\data\\s1\\test.json', count: 0 },
    },
  }
  return {
    CONVERT_OK,
    MERGE_OK,
    SPLIT_OK,
    spies: {
      getDataFiles: null as unknown as ReturnType<typeof vi.fn>,
      loadData: null as unknown as ReturnType<typeof vi.fn>,
      convertDataset: null as unknown as ReturnType<typeof vi.fn>,
      mergeDatasets: null as unknown as ReturnType<typeof vi.fn>,
      splitDataset: null as unknown as ReturnType<typeof vi.fn>,
    },
  }
})

vi.mock('../services/api', async (importOriginal) => {
  const actual = await importOriginal<typeof import('../services/api')>()
  fx.spies.getDataFiles = vi
    .fn()
    .mockResolvedValue({ files: [{ name: 'a.json' }, { name: 'b.json' }] })
  fx.spies.loadData = vi.fn().mockResolvedValue({ items: [], total: 0 })
  fx.spies.convertDataset = vi.fn().mockResolvedValue(fx.CONVERT_OK)
  fx.spies.mergeDatasets = vi.fn().mockResolvedValue(fx.MERGE_OK)
  fx.spies.splitDataset = vi.fn().mockResolvedValue(fx.SPLIT_OK)
  return {
    ...actual,
    getDataFiles: fx.spies.getDataFiles,
    loadData: fx.spies.loadData,
    convertDataset: fx.spies.convertDataset,
    mergeDatasets: fx.spies.mergeDatasets,
    splitDataset: fx.spies.splitDataset,
  }
})

import DataManagement from './DataManagement'

const { CONVERT_OK, MERGE_OK, SPLIT_OK, spies } = fx

afterEach(() => {
  cleanup()
  message.destroy()
  for (const key of ['convertDataset', 'mergeDatasets', 'splitDataset'] as const) {
    spies[key].mockClear()
    spies[key].mockResolvedValue(
      key === 'convertDataset' ? CONVERT_OK : key === 'mergeDatasets' ? MERGE_OK : SPLIT_OK
    )
  }
})

beforeEach(() => {
  spies.getDataFiles.mockClear()
})

/** 三个数字输入框按 JSX 顺序：0 train、1 val、2 test、3 seed */
const spinbuttons = () => screen.getAllByRole('spinbutton') as HTMLInputElement[]

/**
 * 按名字点按钮：用**子串**正则而不是精确可及名
 *
 * antd 的 `Button loading` 会在文字前插一个 `aria-label="loading"` 的图标，按钮的可及名
 * 于是变成「loading转换选中的文件」——精确名查不到（实测红两支）。这里刻意**不等** loading
 * 退场：探针量到 `loading` 置回 false 那一次重渲染在本套 jsdom/act 环境里并不总是落地
 * （隔离跑这一支时，500 ms 内三个按钮一直带 loading 图标，而 `loadData` 只被调用过一次），
 * 把测试挂在一个本身不可靠的闸上不如直接接受「带图标的同一按钮」——它仍是同一个控件，
 * `disabled` 实测为 false，点下去真的会走到 handler。
 */
const click = (name: string) =>
  fireEvent.click(screen.getByRole('button', { name: new RegExp(name) }))

/**
 * 回执只按属性取，不走 `getByLabelText`
 *
 * 三张回执都不是表单控件（两张 `div`、一张 antd `Table`），`getByLabelText` 的
 * 标签—控件关联本来也不该覆盖它们；`querySelector('[aria-label=…]')` 证的是
 * 「页面上确实挂了这个名字」，读数仍然取自它的 `textContent`。
 */
const labeled = (name: string) => document.querySelector(`[aria-label="${name}"]`)

/**
 * 等文件列表落地再点按钮
 *
 * `selectedFile` 是 `loadFiles` 里异步设成 `files[0]` 的（`DataManagement.tsx:70`），
 * 刚 `render()` 那一瞬它是空串——此时点任何按钮只会撞上必填闸，测到的是闸而不是接线。
 * 实测第一轮就是这个原因红 5 支（页面回的是「需要选中源文件并点名输出文件名」）。
 */
async function waitFiles(container: HTMLElement) {
  await waitFor(
    () =>
      expect((container.querySelector('select') as HTMLSelectElement)?.value).toBe('a.json'),
    { timeout: 4000 },
  )
}

describe('转换卡的渲染面', () => {
  it('目标格式渲染十二项且不含 xls，源格式渲染「自动」加十三项且含 xls', () => {
    render(<DataManagement />)

    const target = screen.getByLabelText('转换目标格式')
    const targetOptions = Array.from(target.querySelectorAll('option')).map((o) => o.value)
    expect(targetOptions).toHaveLength(12)
    expect(targetOptions).not.toContain('xls')
    expect(targetOptions).toContain('xlsx')
    expect(target).toHaveProperty('value', 'jsonl')

    const source = screen.getByLabelText('转换源格式')
    const sourceOptions = Array.from(source.querySelectorAll('option')).map((o) => o.value)
    expect(sourceOptions).toHaveLength(14)
    expect(sourceOptions).toContain('xls')
    expect(source).toHaveProperty('value', '')
  })

  it('点转换发的是选中文件 + 点名的输出 + 两个格式；选「自动」时把空串交给服务层丢', async () => {
    const { container } = render(<DataManagement />)
    await waitFiles(container)

    fireEvent.change(screen.getByLabelText('转换输出文件名'), {
      target: { value: 'out.jsonl' },
    })
    click('转换选中的文件')

    await waitFor(() =>
      expect(spies.convertDataset).toHaveBeenCalledWith('a.json', 'out.jsonl', {
        targetFormat: 'jsonl',
        sourceFormat: '',
      })
    )
    await waitFor(() => expect(labeled('转换回执')?.textContent).toContain('out.jsonl'))
  })

  it('转换失败显示服务端那句「路径包含非法组件」，并把回执留在空', async () => {
    spies.convertDataset.mockRejectedValue({
      response: { data: { detail: '路径包含非法组件' } },
    })
    const { container } = render(<DataManagement />)
    await waitFiles(container)

    fireEvent.change(screen.getByLabelText('转换输出文件名'), { target: { value: '../evil.json' } })
    click('转换选中的文件')

    await waitFor(() => expect(document.body.textContent).toContain('路径包含非法组件'), {
      timeout: 4000,
    })
    expect(labeled('转换回执')).toBeNull()
  })

  it('没点名输出文件时点对转换不发请求，只给一句要什么的提示', async () => {
    const { container } = render(<DataManagement />)
    await waitFiles(container)

    click('转换选中的文件')

    await waitFor(() => expect(document.body.textContent).toContain('点名输出文件名'), {
      timeout: 4000,
    })
    expect(spies.convertDataset).not.toHaveBeenCalled()
  })
})

describe('合并卡的渲染面', () => {
  const check = (name: string) =>
    fireEvent.click(screen.getByRole('checkbox', { name }))

  it('勾两份文件并点名输出 ⇒ inputs 是两份、去重按默认开', async () => {
    const { container } = render(<DataManagement />)
    await waitFiles(container)

    check('a.json')
    check('b.json')
    fireEvent.change(screen.getByLabelText('合并输出文件名'), { target: { value: 'merged.json' } })
    click('合并勾选的文件')

    await waitFor(() =>
      expect(spies.mergeDatasets).toHaveBeenCalledWith(['a.json', 'b.json'], 'merged.json', {
        deduplicate: true,
      })
    )
    await waitFor(() => expect(labeled('合并回执')).not.toBeNull())
    // 三个数都要看得见：只报「成功」等于把去重删了多少条吞掉。
    const receipt = labeled('合并回执')?.textContent || ''
    expect(receipt).toContain('2 份输入共 10 条')
    expect(receipt).toContain('8 条出')
    expect(receipt).toContain('删掉 2 条')
  })

  it('关掉去重发的是显式 false，不是省略那一格', async () => {
    const { container } = render(<DataManagement />)
    await waitFiles(container)

    check('a.json')
    check('去重')
    fireEvent.change(screen.getByLabelText('合并输出文件名'), { target: { value: 'm.json' } })
    click('合并勾选的文件')

    await waitFor(() =>
      expect(spies.mergeDatasets).toHaveBeenCalledWith(['a.json'], 'm.json', {
        deduplicate: false,
      })
    )
  })

  it('一个都没勾时点对合并不发请求', async () => {
    const { container } = render(<DataManagement />)
    await waitFiles(container)

    fireEvent.change(screen.getByLabelText('合并输出文件名'), { target: { value: 'm.json' } })
    click('合并勾选的文件')

    await waitFor(() => expect(document.body.textContent).toContain('至少要勾一个输入文件'), {
      timeout: 4000,
    })
    expect(spies.mergeDatasets).not.toHaveBeenCalled()
  })

  it('合并失败显示服务端的 404 判决', async () => {
    spies.mergeDatasets.mockRejectedValue({ response: { data: { detail: '文件不存在' } } })
    const { container } = render(<DataManagement />)
    await waitFiles(container)

    check('a.json')
    fireEvent.change(screen.getByLabelText('合并输出文件名'), { target: { value: 'm.json' } })
    click('合并勾选的文件')

    await waitFor(() => expect(document.body.textContent).toContain('文件不存在'), {
      timeout: 4000,
    })
    expect(labeled('合并回执')).toBeNull()
  })
})

describe('分割卡的渲染面', () => {
  it('三段恒在：test 是 0 条也要渲出一行，而不是从表里消失', async () => {
    const { container } = render(<DataManagement />)
    await waitFiles(container)

    fireEvent.change(screen.getByLabelText('分割输出目录'), { target: { value: 's1' } })
    click('分割选中的文件')

    await waitFor(() => expect(labeled('分割回执')).not.toBeNull())
    const rows = Array.from(labeled('分割回执')?.querySelectorAll('tbody tr') || [])
    const text = rows.map((r) => r.textContent)
    expect(text.some((t) => t?.includes('train') && t?.includes('4'))).toBe(true)
    expect(text.some((t) => t?.includes('val') && t?.includes('1'))).toBe(true)
    expect(text.some((t) => t?.includes('test') && t?.includes('0'))).toBe(true)
  })

  it('初值就是契约默认：train/val/test = 0.8/0.1/0.1，seed 空着即不发那一格', async () => {
    const { container } = render(<DataManagement />)
    await waitFiles(container)

    // antd InputNumber 按 step 补小数位显示（step 0.05 ⇒ 两位），所以页面上是 '0.80' 而不是 '0.8'；
    // 发出去的数仍是 number 0.8，下一段断言钉的是那一侧。
    expect(spinbuttons().map((input) => input.value)).toEqual(['0.80', '0.10', '0.10', ''])
    fireEvent.change(screen.getByLabelText('分割输出目录'), { target: { value: 's1' } })
    click('分割选中的文件')

    await waitFor(() => expect(spies.splitDataset).toHaveBeenCalled())
    const opts = spies.splitDataset.mock.calls[0][2]
    expect(opts).toMatchObject({
      trainRatio: 0.8,
      valRatio: 0.1,
      testRatio: 0.1,
      stratify: false,
      stratifyKey: 'instruction',
      shuffle: true,
    })
    expect(opts.seed).toBeUndefined()
  })

  it('比例填 0 是要发出去的 0，不是「没填」', async () => {
    const { container } = render(<DataManagement />)
    await waitFiles(container)

    fireEvent.change(spinbuttons()[2], { target: { value: '0' } })
    fireEvent.change(screen.getByLabelText('分割输出目录'), { target: { value: 's1' } })
    click('分割选中的文件')

    await waitFor(() => expect(spies.splitDataset).toHaveBeenCalled())
    expect(spies.splitDataset.mock.calls[0][2].testRatio).toBe(0)
  })

  it('分割失败显示服务端那句「比例之和必须为1.0」的判决', async () => {
    // 这句照抄探针在真实端点上量到的回执（`0.7/0.1/0.1` ⇒ 浮点artifact 原样出现在判决里）：
    // 夹具用真句子，这条断言证的才是「服务端那串字被整份交给用户」而不是「页面自己编了一句」。
    spies.splitDataset.mockRejectedValue({
      response: { data: { detail: '分割比例之和必须为1.0，当前为 0.8999999999999999' } },
    })
    const { container } = render(<DataManagement />)
    await waitFiles(container)

    fireEvent.change(screen.getByLabelText('分割输出目录'), { target: { value: 's1' } })
    click('分割选中的文件')

    await waitFor(() => expect(document.body.textContent).toContain('分割比例之和必须为1.0'), {
      timeout: 4000,
    })
    expect(labeled('分割回执')).toBeNull()
  })

  /**
   * 两条诚实口径是页面**替用户把坑说出口**：它们各自都是实测过的静默形状，
   * 少了这两句，用户会把「塌成一个分组」读成「分层生效了」。
   */
  it('卡上写着分层字段的静默形状与两条支路的判据差', () => {
    render(<DataManagement />)

    const page = document.body.textContent || ''
    expect(page).toContain('塌成一个分组')
    expect(page).toContain('不报错')
    expect(page).toContain('0.01')
    expect(page).toContain('负数只有分层支会拦')
  })
})
