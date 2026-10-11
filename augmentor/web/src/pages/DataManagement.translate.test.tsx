/**
 * `DataManagement` 的批量翻译面（L236：A279 最后一员 + A283① 落盘口）
 *
 * Python 侧的 `test_last_two_wiring_l236.py` 只能证明**源码写了**那几格初值与那几句口径，
 * 证不出渲染面上真的看得见、也证不出那三道闸真的拦得住。这一份钉四件事：
 * ① 四格初值与按钮的禁用态——初值必须等于契约默认（`TranslateRequest.model_fields`），
 *    否则「什么都不改就点一下」的行为会与后端那一份分家；
 * ② 三道页面级闸各拦得住一次真实点击（未点名落盘 / 字段名清空 / 未勾计费确认），
 *    且**请求 0 次**——这一条路径按 token 计费，闸的意义就是别花钱；
 * ③ 回执三面：落盘绝对路径、`output_file` 为 null 时的「未落盘」、部分失败时的失败回执
 *    （条数 / 下标 / 样本数 / 截断说明都要看得见，缺一样就会把混着原文的产物读成成功）；
 * ④ 失败时用户看见的是服务端那句判决原话，且回执区不出现。
 *
 * 服务层用 `importOriginal` 做部分替身，`apiErrorDetail` 留真身，理由同采样那一份用例。
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
 * 三份夹具都是**探针在真实端点上量到的形状**（L236），不是随手编的：
 * `TR_OK.output_file` 是服务端 resolve 之后的绝对路径（不是请求里那个文件名）；
 * `TR_NODISK.output_file` 是 null——键**恒在**，省略落盘口时值为 null，这是 A283① 的契约；
 * `TR_PARTIAL` 的 `failed: 25 / errors: 20 / errors_truncated: true` 是「全失败但样本收窄」
 * 那一档的真实读数（下标全量 25 条、样本只留 20 条）。
 */
const fx = vi.hoisted(() => {
  const BASE = {
    total: 25,
    batches: 3,
    batch_size: 10,
    target_lang: 'en',
    supported_langs: ['zh', 'en'],
    items: [{ instruction: 'TRANSLATED' }] as Record<string, unknown>[],
  }
  const TR_OK = {
    ...BASE,
    output_file: 'D:\\data\\translated.json',
    failed: 0,
    failed_indices: [] as number[],
    errors: [] as { index: number; error: string }[],
    errors_truncated: false,
  }
  const TR_NODISK = { ...TR_OK, output_file: null }
  const TR_PARTIAL = {
    ...BASE,
    output_file: 'D:\\data\\translated.json',
    failed: 25,
    failed_indices: Array.from({ length: 25 }, (_, i) => i),
    errors: Array.from({ length: 20 }, (_, i) => ({
      index: i,
      error: 'RuntimeError: backend down',
    })),
    errors_truncated: true,
  }
  return {
    TR_OK,
    TR_NODISK,
    TR_PARTIAL,
    spies: {
      getDataFiles: null as unknown as ReturnType<typeof vi.fn>,
      loadData: null as unknown as ReturnType<typeof vi.fn>,
      translateDataset: null as unknown as ReturnType<typeof vi.fn>,
    },
  }
})

vi.mock('../services/api', async (importOriginal) => {
  const actual = await importOriginal<typeof import('../services/api')>()
  fx.spies.getDataFiles = vi
    .fn()
    .mockResolvedValue({ files: [{ name: 'a.json' }, { name: 'b.jsonl' }] })
  fx.spies.loadData = vi.fn().mockResolvedValue({ items: [], total: 0 })
  fx.spies.translateDataset = vi.fn().mockResolvedValue(fx.TR_OK)
  return {
    ...actual,
    getDataFiles: fx.spies.getDataFiles,
    loadData: fx.spies.loadData,
    translateDataset: fx.spies.translateDataset,
  }
})

import DataManagement from './DataManagement'

const { TR_OK, TR_NODISK, TR_PARTIAL, spies } = fx

afterEach(() => {
  cleanup()
  message.destroy()
  spies.translateDataset.mockClear().mockResolvedValue(TR_OK)
})

beforeEach(() => {
  spies.getDataFiles.mockClear()
})

/** 等文件列表落地再点按钮（`selectedFile` 是异步设成 `files[0]` 的，理由见采样那份用例） */
async function waitFiles(container: HTMLElement) {
  await waitFor(
    () =>
      expect((container.querySelector('select') as HTMLSelectElement)?.value).toBe('a.json'),
    { timeout: 4000 },
  )
}

/** antd 的 `Button loading` 会往可及名前面插一个图标名，所以一律按子串正则取 */
const button = () => screen.getByRole('button', { name: /翻译并落盘/ })
const click = () => fireEvent.click(button())

/** 回执不是表单控件，按属性取；读数仍取自它的 textContent */
const labeled = (name: string) => document.querySelector(`[aria-label="${name}"]`)

const ack = () => fireEvent.click(screen.getByRole('checkbox', { name: /按 token 计费/ }))

describe('翻译卡的初值与三道闸', () => {
  it('四格初值等于契约默认，未勾计费确认时按钮是 disabled', () => {
    render(<DataManagement />)

    expect(screen.getByLabelText('翻译输出文件名')).toHaveProperty('value', '')
    expect(screen.getByLabelText('翻译目标语言')).toHaveProperty('value', '')
    expect(screen.getByLabelText('翻译字段名')).toHaveProperty('value', 'instruction')
    expect(screen.getByRole('checkbox', { name: /保留原文到/ })).toBeChecked()
    expect(screen.getByRole('checkbox', { name: /按 token 计费/ })).not.toBeChecked()
    expect(button()).toBeDisabled()
  })

  it('勾上计费确认之后按钮才可点：这一格拦的是「花钱之前先认一次」', () => {
    render(<DataManagement />)

    ack()

    expect(button()).toBeEnabled()
  })

  it('没点名落盘文件 ⇒ 一句提示、请求 0 次（页面比契约更严，A283 立项的原话）', async () => {
    const { container } = render(<DataManagement />)
    await waitFiles(container)

    ack()
    click()

    await waitFor(
      () => expect(document.body.textContent).toContain('产物必须留得住'),
      { timeout: 4000 },
    )
    expect(spies.translateDataset).not.toHaveBeenCalled()
  })

  it('翻译字段清空 ⇒ 拦下且请求 0 次：后端这一格零判决，页面补的是不存在的判决（A322）', async () => {
    const { container } = render(<DataManagement />)
    await waitFiles(container)

    fireEvent.change(screen.getByLabelText('翻译输出文件名'), {
      target: { value: 'translated.json' },
    })
    fireEvent.change(screen.getByLabelText('翻译字段名'), { target: { value: '   ' } })
    ack()
    click()

    await waitFor(
      () => expect(document.body.textContent).toContain('翻译字段不能为空'),
      { timeout: 4000 },
    )
    expect(spies.translateDataset).not.toHaveBeenCalled()
    expect(labeled('翻译回执')).toBeNull()
  })

  it('填齐并勾上确认 ⇒ 发出去的正是那四格，回执报服务端 resolve 之后的绝对落点', async () => {
    const { container } = render(<DataManagement />)
    await waitFiles(container)

    fireEvent.change(screen.getByLabelText('翻译输出文件名'), {
      target: { value: 'translated.json' },
    })
    fireEvent.change(screen.getByLabelText('翻译目标语言'), { target: { value: 'en' } })
    ack()
    click()

    await waitFor(() =>
      expect(spies.translateDataset).toHaveBeenCalledWith('a.json', {
        outputFile: 'translated.json',
        targetLang: 'en',
        textKey: 'instruction',
        keepOriginal: true,
      })
    )
    await waitFor(() => expect(labeled('翻译回执')).not.toBeNull())
    const receipt = labeled('翻译回执')?.textContent || ''
    expect(receipt).toContain('共 25 条')
    expect(receipt).toContain('分 3 批')
    expect(receipt).toContain('批宽 10')
    expect(receipt).toContain('D:\\data\\translated.json')
    // 全成功时不该出现失败回执那一块（它只在 failed > 0 时渲染）
    expect(labeled('翻译失败回执')).toBeNull()
  })
})

describe('翻译回执面', () => {
  it('output_file 为 null ⇒ 回执写「未落盘」，不拿文件名冒充落点', async () => {
    spies.translateDataset.mockResolvedValue(TR_NODISK)
    const { container } = render(<DataManagement />)
    await waitFiles(container)

    fireEvent.change(screen.getByLabelText('翻译输出文件名'), {
      target: { value: 'translated.json' },
    })
    ack()
    click()

    await waitFor(() =>
      expect(labeled('翻译回执')?.textContent).toContain('未落盘（只在本次响应里）')
    )
  })

  it('部分失败 ⇒ 失败回执点名条数、下标、样本数与截断说明（缺一样就会把原文读成译文）', async () => {
    spies.translateDataset.mockResolvedValue(TR_PARTIAL)
    const { container } = render(<DataManagement />)
    await waitFiles(container)

    fireEvent.change(screen.getByLabelText('翻译输出文件名'), {
      target: { value: 'translated.json' },
    })
    ack()
    click()

    await waitFor(() => expect(labeled('翻译失败回执')).not.toBeNull())
    const failed = labeled('翻译失败回执')?.textContent || ''
    expect(failed).toContain('25 条没翻成')
    // 下标只列前 10 个并以省略号收尾：25 条全列会把回执撑成一屏
    expect(failed).toContain('0、1、2、3、4、5、6、7、8、9 …')
    expect(failed).toContain('这些条目在产物里是**原文**')
    expect(failed).toContain('错误样本 20 条')
    expect(failed).toContain('已收窄，计数与下标不受影响')
  })

  it('服务端 400 的判决原话照抄，回执区不出现（multilingual.enabled 出厂 false 那一档）', async () => {
    // 这句 detail 逐字抄自 `api/routes/language.py` 的翻译端点（L236 复核）：页面不翻译、
    // 不改写，用户看见的就是服务端那串字
    spies.translateDataset.mockRejectedValue({
      response: { data: { detail: 'multilingual.enabled 为 false，翻译路径未启用' } },
    })
    const { container } = render(<DataManagement />)
    await waitFiles(container)

    fireEvent.change(screen.getByLabelText('翻译输出文件名'), {
      target: { value: 'translated.json' },
    })
    ack()
    click()

    await waitFor(
      () => expect(document.body.textContent).toContain('翻译路径未启用'),
      { timeout: 4000 },
    )
    expect(labeled('翻译回执')).toBeNull()
  })
})

describe('翻译卡上的诚实口径', () => {
  /**
   * 每一句都是探针在真实端点上量到的形状：少了它们，用户会把「200 且 failed=0」读成
   * 「翻成了」（A322 那一格正是 200 / 0 次调用 / 产物里凭空多两个键），也会把
   * 「留空即不落盘」读成「总会留一份」。
   */
  it('落盘口、403 先于花钱、字段名零判决与样本上限都写在卡上', () => {
    render(<DataManagement />)

    const page = document.body.textContent || ''
    expect(page).toContain('A283')
    expect(page).toContain('A322')
    expect(page).toContain('越界落点回 403 且模型调用 **0** 次')
    expect(page).toContain('已 resolve 的绝对路径')
    expect(page).toContain('errors 只有前 20 条样本')
    expect(page).toContain('language 目标语言戳')
    expect(page).toContain('别把「200 且 failed=0」读成翻成了')
  })
})
