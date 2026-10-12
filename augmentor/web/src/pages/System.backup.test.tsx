/**
 * `System` 的数据备份面（L231：A279 第七批接线；L243：A299 校验和闸 + A300 两员入口）
 *
 * 存在理由与 L229 / L230 那两份同构：Python 侧的 `test_backup_wiring_l231.py` 只能证明
 * **源码写了**那六个调用形状与那几句口径，证不出渲染面上真的看得见。这一份钉八件事：
 * ① 列表按服务端回的**原样**渲染，包括索引里两条同 id 的记录（同名创建两次的真实产物）
 *    ——所以行键不能拿 `backup_id` 当唯一键；
 * ② 创建的必填闸、成功回执、失败判决三路各有可见读数，失败时用户看见的是服务端那句
 *    判决原话（403「路径超出允许的数据目录范围」），不是页面自己编的「失败了」；
 * ③ 恢复的必填闸与成功回执，且回执里出现的是 **`output_path`** 那一格的值；
 * ④ 删除走 Popconfirm，确认之后才真发；
 * ⑤ 卡上那几句**诚实口径**逐条留在 DOM 里（A298 / A299 / A300 / A301 / A302 与白名单三档）
 *    ——L243 把其中两句翻了面，所以钉的是新话，旧话（「不拦截恢复」「HTTP 面没有入口」）
 *    在页面上已经不存在了；
 * ⑥ 备份面与候选面各自失败只降级自己，不把服务状态与向量库状态一起拖掉；
 * ⑦ L243 的「校验完整性」三形状（对得上 / 对不上 / 文件丢了）逐格分开显示，且它**点一次
 *    查一次**：挂载时不发，列表一刷新就把上一轮的读数清掉；
 * ⑧ L243 的「清理旧备份」同样走 Popconfirm，送出去的是**保留份数**，下限 10（出厂默认）。
 *
 * 夹具里的 checksum、item_count、file_size 逐字来自探针在真实端点上量到的回执，不是编的。
 * 服务层用 `importOriginal` 做部分替身，`apiErrorDetail` 留真身，理由同前两份用例。
 */
import { afterEach, describe, expect, it, vi } from 'vitest'
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

const fx = vi.hoisted(() => {
  const CHECKSUM = '7c4463a1339d82cc94f5823483e9c23a3990671a9a2e1bcde83e0978148b8166'
  const STAMPED = {
    backup_id: 'backup_20261011_175833',
    timestamp: '2026-10-11T17:58:33.985322',
    source_path: 'D:\\data\\p.json',
    backup_path: 'D:\\data\\.backups\\backup_20261011_175833.json',
    item_count: 5,
    file_size: 231,
    checksum: CHECKSUM,
  }
  const NAMED = { ...STAMPED, backup_id: 'mybak', backup_path: 'D:\\data\\.backups\\mybak.json' }
  // 同名创建两次之后列表的真实形状：三条记录、两条同 id，而磁盘上只有一份 mybak.json。
  // 两条同 id 的 timestamp 是**两次不同的** `datetime.now()`（探针量到 .991804 与 .997820），
  // 页面的行键正是拼了这一格——夹具若把两条写成逐字相同，就测不出行键会不会撞。
  const NAMED_2 = { ...NAMED, timestamp: '2026-10-11T17:58:33.997820' }
  const LIST_DUP = {
    backups: [{ ...NAMED, timestamp: '2026-10-11T17:58:33.991804' }, NAMED_2, STAMPED],
  }
  const CREATE_OK = { ...STAMPED, backup_id: 'mybak' }
  const RESTORE_OK = {
    backup_id: 'mybak',
    output_path: 'D:\\data\\r.json',
    item_count: 5,
  }
  // L243：完整性读数的三种真实形状。夹具逐键抄 `verify_backup_integrity` 的返回：
  // `exists` 判的是「存在且非空」，所以第三条（文件丢了）的 `checksum_match` 必然是
  // false 且无从比 —— 页面把这两种 false 分开显示，不能塌成同一种红。
  const VERIFY_OK = {
    results: [
      {
        backup_id: 'backup_20261011_175833',
        timestamp: '2026-10-11T17:58:33.985322',
        item_count: 5,
        exists: true,
        checksum_match: true,
      },
      {
        backup_id: 'mybak',
        timestamp: '2026-10-11T17:58:33.991804',
        item_count: 5,
        exists: true,
        checksum_match: false,
      },
      {
        backup_id: 'gone',
        timestamp: '2026-10-11T17:58:34.000000',
        item_count: 3,
        exists: false,
        checksum_match: false,
      },
    ],
  }
  const CLEANUP_OK = { deleted_count: 2, max_backups: 1 }
  const FILES = {
    files: [
      { name: 'train_data1.json', path: 'D:\\data\\train_data1.json', size: 2 },
    ],
  }
  return {
    CHECKSUM,
    LIST_DUP,
    CREATE_OK,
    RESTORE_OK,
    VERIFY_OK,
    CLEANUP_OK,
    FILES,
    spies: {
      getServiceStatus: null as unknown as ReturnType<typeof vi.fn>,
      getVectorStatus: null as unknown as ReturnType<typeof vi.fn>,
      listBackups: null as unknown as ReturnType<typeof vi.fn>,
      getDataFiles: null as unknown as ReturnType<typeof vi.fn>,
      createBackup: null as unknown as ReturnType<typeof vi.fn>,
      restoreBackup: null as unknown as ReturnType<typeof vi.fn>,
      deleteBackup: null as unknown as ReturnType<typeof vi.fn>,
      verifyBackups: null as unknown as ReturnType<typeof vi.fn>,
      cleanupBackups: null as unknown as ReturnType<typeof vi.fn>,
      listDependencyDatasets: null as unknown as ReturnType<typeof vi.fn>,
      getDependencyGraph: null as unknown as ReturnType<typeof vi.fn>,
      systemDependencies: null as unknown as ReturnType<typeof vi.fn>,
    },
  }
})

vi.mock('../services/api', async (importOriginal) => {
  const actual = await importOriginal<typeof import('../services/api')>()
  const s = fx.spies
  s.getServiceStatus = vi.fn().mockResolvedValue({
    status: 'ok',
    version: '3.0.0',
    model_default: 'ernie',
    model_available: true,
    model_retry_count: 0,
    model_retry_wait_seconds: 0,
    dependencies: {
      installed: { pandas: true },
      missing: [],
      degraded_features: [],
      all_required_present: true,
      available_count: 1,
    },
  })
  s.getVectorStatus = vi.fn().mockResolvedValue({
    enabled: false,
    exists: false,
    count: 0,
    backend: 'faiss',
    dimension: 768,
    collection: 'augmentor',
    storage_dir: '',
  })
  s.listBackups = vi.fn().mockResolvedValue(fx.LIST_DUP)
  s.getDataFiles = vi.fn().mockResolvedValue(fx.FILES)
  s.createBackup = vi.fn().mockResolvedValue(fx.CREATE_OK)
  s.restoreBackup = vi.fn().mockResolvedValue(fx.RESTORE_OK)
  s.deleteBackup = vi.fn().mockResolvedValue({ success: true })
  s.verifyBackups = vi.fn().mockResolvedValue(fx.VERIFY_OK)
  s.cleanupBackups = vi.fn().mockResolvedValue(fx.CLEANUP_OK)
  // L232 起本页还会取数据集血缘那一面。与本份用例无关，给空回执让它不必逐个显式安排。
  s.listDependencyDatasets = vi.fn().mockResolvedValue({ datasets: [] })
  s.getDependencyGraph = vi.fn().mockResolvedValue({ nodes: [], edges: [], issues: [] })
  // L236 起依赖诊断自取一面。与本份用例无关：**不给回执**（resolved undefined）⇒ 页面
  // 退回综合状态里那一份，渲染结果与 L236 之前逐格相同
  s.systemDependencies = vi.fn()
  return {
    ...actual,
    getServiceStatus: s.getServiceStatus,
    getVectorStatus: s.getVectorStatus,
    listBackups: s.listBackups,
    getDataFiles: s.getDataFiles,
    createBackup: s.createBackup,
    restoreBackup: s.restoreBackup,
    deleteBackup: s.deleteBackup,
    verifyBackups: s.verifyBackups,
    cleanupBackups: s.cleanupBackups,
    listDependencyDatasets: s.listDependencyDatasets,
    getDependencyGraph: s.getDependencyGraph,
    systemDependencies: s.systemDependencies,
  }
})

import System from './System'

const { CHECKSUM, LIST_DUP, CREATE_OK, RESTORE_OK, VERIFY_OK, CLEANUP_OK, FILES, spies } = fx

afterEach(() => {
  cleanup()
  message.destroy()
  // `clearAllMocks` 不清实现，所以逐只把默认回执重新钉回去
  spies.listBackups.mockClear().mockResolvedValue(LIST_DUP)
  spies.getDataFiles.mockClear().mockResolvedValue(FILES)
  spies.createBackup.mockClear().mockResolvedValue(CREATE_OK)
  spies.restoreBackup.mockClear().mockResolvedValue(RESTORE_OK)
  spies.deleteBackup.mockClear().mockResolvedValue({ success: true })
  spies.verifyBackups.mockClear().mockResolvedValue(VERIFY_OK)
  spies.cleanupBackups.mockClear().mockResolvedValue(CLEANUP_OK)
})

/** 等备份列表落地（挂载即取，所以只需等第一次回执渲染出来） */
async function waitBackups() {
  await waitFor(() => expect(spies.listBackups).toHaveBeenCalled())
  expect(await screen.findByText('backup_20261011_175833')).toBeInTheDocument()
}

/**
 * 取备份源那一格的**输入框**
 *
 * antd 的 AutoComplete 把 `aria-label` 同时挂在外层 div 与内层 input 上（实测：
 * `getAllByLabelText` 回两个元素），所以按标签取会撞「多个匹配」。这里显式挑 INPUT。
 */
function sourceInput(): HTMLInputElement {
  const input = screen
    .getAllByLabelText('备份源文件')
    .find((el) => el.tagName === 'INPUT')
  if (!input) {
    throw new Error('备份源文件的输入框没渲染出来')
  }
  return input as HTMLInputElement
}

describe('System 备份列表渲染', () => {
  it('索引里两条同 id 记录都渲出来，行键没有把它们塌成一行', async () => {
    render(<System />)

    await waitBackups()
    // 同名创建两次的真实产物：三条记录里 `mybak` 出现两次
    expect(screen.getAllByText('mybak')).toHaveLength(2)
    expect(screen.getByText('backup_20261011_175833')).toBeInTheDocument()
  })

  it('校验和只显前 12 位，完整值挂在 title 上', async () => {
    const { container } = render(<System />)

    await waitBackups()
    expect(screen.getAllByText(CHECKSUM.slice(0, 12))).toHaveLength(3)
    expect(screen.queryByText(CHECKSUM)).not.toBeInTheDocument()
    const cell = Array.from(container.querySelectorAll('span[title]')).find(
      (el) => el.getAttribute('title') === CHECKSUM
    )
    expect(cell).toBeTruthy()
  })
})

describe('System 创建备份', () => {
  it('未填源文件就点创建：只给一句 warning，不发请求', async () => {
    render(<System />)
    await waitBackups()

    fireEvent.click(screen.getByRole('button', { name: '创建备份' }))

    await waitFor(() =>
      expect(document.body.textContent).toContain('先填要备份的那份服务端数据文件')
    )
    expect(spies.createBackup).not.toHaveBeenCalled()
  })

  it('填了源文件与名称：按 (路径, { name }) 发，回执报条数与 id，列表重取', async () => {
    render(<System />)
    await waitBackups()
    const callsBefore = spies.listBackups.mock.calls.length

    fireEvent.change(sourceInput(), {
      target: { value: 'data/p.json' },
    })
    fireEvent.change(screen.getByLabelText('备份名称'), { target: { value: 'mybak' } })
    fireEvent.click(screen.getByRole('button', { name: '创建备份' }))

    await waitFor(() =>
      expect(spies.createBackup).toHaveBeenCalledWith('data/p.json', { name: 'mybak' })
    )
    await waitFor(() => expect(document.body.textContent).toContain('已备份 5 条 ⇒ mybak'))
    // 创建成功后重新取一次列表，否则页面上看不见刚建的那一条
    await waitFor(() =>
      expect(spies.listBackups.mock.calls.length).toBeGreaterThan(callsBefore)
    )
  })

  it('白名单外被判 403 时，页面上是服务端那句判决而不是「失败了」', async () => {
    spies.createBackup.mockRejectedValue({
      response: { status: 403, data: { detail: '路径超出允许的数据目录范围' } },
    })
    render(<System />)
    await waitBackups()

    fireEvent.change(sourceInput(), {
      target: { value: 'D:\\outside\\secret.json' },
    })
    fireEvent.click(screen.getByRole('button', { name: '创建备份' }))

    await waitFor(() =>
      expect(document.body.textContent).toContain('路径超出允许的数据目录范围')
    )
  })
})

describe('System 恢复与删除', () => {
  it('未填恢复目标就点恢复：只给一句 warning，不发请求', async () => {
    render(<System />)
    await waitBackups()

    // 同 id 有两条记录 ⇒ 每行一个按钮，取第一只
    fireEvent.click(screen.getAllByRole('button', { name: '恢复备份 mybak' })[0])

    await waitFor(() => expect(document.body.textContent).toContain('先填恢复目标文件名'))
    expect(spies.restoreBackup).not.toHaveBeenCalled()
  })

  it('填了目标就按 (id, 目标) 发，回执里出现的是 output_path 那一格的值', async () => {
    render(<System />)
    await waitBackups()

    fireEvent.change(screen.getByLabelText('恢复目标文件名'), {
      target: { value: 'r.json' },
    })
    fireEvent.click(screen.getAllByRole('button', { name: '恢复备份 mybak' })[0])

    await waitFor(() => expect(spies.restoreBackup).toHaveBeenCalledWith('mybak', 'r.json'))
    await waitFor(() =>
      expect(document.body.textContent).toContain('已恢复 5 条 ⇒ D:\\data\\r.json')
    )
  })

  it('删除要先在 Popconfirm 上确认，确认之后才真发并重取列表', async () => {
    render(<System />)
    await waitBackups()
    const callsBefore = spies.listBackups.mock.calls.length

    fireEvent.click(screen.getAllByRole('button', { name: '删除备份 mybak' })[0])
    // 还没确认就不该发
    expect(spies.deleteBackup).not.toHaveBeenCalled()
    fireEvent.click(screen.getByRole('button', { name: 'OK' }))

    await waitFor(() => expect(spies.deleteBackup).toHaveBeenCalledWith('mybak'))
    await waitFor(() => expect(document.body.textContent).toContain('已删除备份 mybak'))
    await waitFor(() =>
      expect(spies.listBackups.mock.calls.length).toBeGreaterThan(callsBefore)
    )
  })
})

describe('System 备份卡的诚实口径（L231 实测，L243 两面翻正）', () => {
  it('卡上那句句脾气逐条留在 DOM 里', async () => {
    render(<System />)
    await waitBackups()

    const text = document.body.textContent || ''
    // A298：同名覆盖 + 索引两条 + 删除按 id 清全部
    expect(text).toContain('覆盖同一份文件')
    expect(text).toContain('删一次两条一起消失')
    // A299：L243 之前这一格写的是「不拦截恢复」，现在翻面成会拦 409。
    // 旧话在页面上已不存在，所以钉新话；顺带钉「一个字节都不写」——那是这一档的
    // 第二维（库里在覆盖输出文件之前抛），只钉 409 会把「拦下了但还是写盘」放过去。
    expect(text).toContain('会拦')
    expect(text).toContain('一个字节都不写')
    expect(text).not.toContain('不拦截恢复')
    // 落点键名（A277 当年订正过文档）
    expect(text).toContain('output_path')
    // A300：SDK 那两员自 L243 起有 HTTP 入口；旧话「HTTP 面没有入口」同样已不在页面上
    expect(text).toContain('现已有 HTTP 入口')
    expect(text).not.toContain('HTTP 面没有入口')
    // A300 的另一半：CLI 那一面自 L244 起补齐六档，页面要说的是「也是六档 + verify 判负退 1」，
    // 旧话「CLI 仍只有四档、没有这两档」不许再出现在页面上（那是 A329 的原文案）
    expect(text).toContain('CLI 自 L244 起也是六档')
    expect(text).toContain('退出码 1')
    expect(text).not.toContain('create / restore / list / delete')
    expect(text).not.toContain('没有这两档')
    // L243：verify 的读数面（逐行哈希、点一次查一次）与份数下限
    expect(text).toContain('索引登记的每一行')
    expect(text).toContain('点一次查一次')
    expect(text).toContain('页面把下限钉在 1')
    // A301：CLI 与页面的「默认目录」不是同一个
    expect(text).toContain('不是同一个目录')
    // A302：设了 API Key 就 401（key 存进「配置管理」页那张卡后由拦截器送出去，L233）
    expect(text).toContain('X-API-Key')
    // 白名单三档与候选口径
    expect(text).toContain('路径包含非法组件')
    expect(text).toContain('train_data*.json')
  })
})

describe('System 校验完整性（L243，A299 的读数面）', () => {
  // 这一只按钮的 accessible name 来自 `aria-label="校验备份完整性"`，不是可见文字
  // 「校验完整性」——`getByRole({ name })` 认前者，所以按可见文字取会撞「找不到元素」。
  // 「清理旧备份」那只恰好两者同名，取法不受影响。
  it('挂载时不发这只请求——它是点一次查一次，不随列表自动刷', async () => {
    render(<System />)
    await waitBackups()

    expect(spies.verifyBackups).not.toHaveBeenCalled()
    expect(screen.queryByText('文件丢了')).not.toBeInTheDocument()
  })

  it('三形状分开显示：对得上 / 对不上 / 文件丢了（后者是「没法比」而不是同一种红）', async () => {
    render(<System />)
    await waitBackups()

    fireEvent.click(screen.getByRole('button', { name: '校验备份完整性' }))

    await waitFor(() => expect(spies.verifyBackups).toHaveBeenCalledTimes(1))
    // 回执那句要 waitFor：同步读 `document.body.textContent` 实测拿不到它，
    // 而表格那一格已经在了（同族前例：`已备份 5 条 ⇒ mybak` 也是这么等的）。
    await waitFor(() =>
      expect(document.body.textContent).toContain('2 只备份的校验和对不上，恢复它们会拿 409')
    )
    const text = document.body.textContent || ''
    // 好 / 坏 / 文件丢了：三只 id 都在，判决三格各就各位
    expect(screen.getByText('gone')).toBeInTheDocument()
    expect(text).toContain('对得上')
    expect(text).toContain('对不上')
    expect(text).toContain('文件丢了')
    // exists=false 时 checksum_match 的 false 无从比 ⇒ 页面单列成「没法比」
    expect(text).toContain('没法比')
    expect(screen.getAllByText('对得上')).toHaveLength(1)
    expect(screen.getAllByText('对不上')).toHaveLength(1)
    expect(screen.getAllByText('没法比')).toHaveLength(1)
  })

  it('全部对得上时回执换成功那一档', async () => {
    spies.verifyBackups.mockResolvedValue({
      results: [{ ...VERIFY_OK.results[0], checksum_match: true }],
    })
    render(<System />)
    await waitBackups()

    fireEvent.click(screen.getByRole('button', { name: '校验备份完整性' }))

    await waitFor(() =>
      expect(document.body.textContent).toContain('已校验 1 只备份，全部对得上')
    )
  })

  it('取不到时只报这一句，备份列表照旧渲染', async () => {
    spies.verifyBackups.mockRejectedValue({
      response: { status: 500, data: { detail: '备份目录读不了' } },
    })
    render(<System />)
    await waitBackups()

    fireEvent.click(screen.getByRole('button', { name: '校验备份完整性' }))

    await waitFor(() => expect(document.body.textContent).toContain('备份目录读不了'))
    // 失败不留半张表：verifyRows 回到 null
    expect(screen.queryByText('gone')).not.toBeInTheDocument()
    expect(screen.getByText('backup_20261011_175833')).toBeInTheDocument()
  })

  it('列表一刷新就把上一轮的读数清掉，不留着旧判决', async () => {
    render(<System />)
    await waitBackups()

    fireEvent.click(screen.getByRole('button', { name: '校验备份完整性' }))
    await waitFor(() => expect(screen.getByText('gone')).toBeInTheDocument())

    fireEvent.change(sourceInput(), { target: { value: 'data/p.json' } })
    fireEvent.click(screen.getByRole('button', { name: '创建备份' }))

    await waitFor(() => expect(spies.createBackup).toHaveBeenCalled())
    // 创建成功后重取列表 ⇒ 走 refreshBackups ⇒ 旧的完整性读数作废
    await waitFor(() => expect(spies.listBackups.mock.calls.length).toBeGreaterThan(1))
    expect(screen.queryByText('gone')).not.toBeInTheDocument()
  })
})

describe('System 清理旧备份（L243，A300 的写面）', () => {
  it('要先在 Popconfirm 上确认，确认之后才发，送的是保留份数', async () => {
    render(<System />)
    await waitBackups()
    const callsBefore = spies.listBackups.mock.calls.length

    fireEvent.click(screen.getByRole('button', { name: '清理旧备份' }))
    expect(spies.cleanupBackups).not.toHaveBeenCalled()

    fireEvent.click(screen.getByRole('button', { name: 'OK' }))

    // 数字框初值 10 = 服务端默认，页面不送 backup_dir（落点由服务端白名单首根决定）
    await waitFor(() => expect(spies.cleanupBackups).toHaveBeenCalledWith({ maxBackups: 10 }))
    await waitFor(() =>
      expect(document.body.textContent).toContain('已删除 2 只旧备份，保留最新 1 只')
    )
    // 删完必须重取列表，否则页面上还挂着已被删掉的那几条
    await waitFor(() =>
      expect(spies.listBackups.mock.calls.length).toBeGreaterThan(callsBefore)
    )
  })

  it('确认框上写的就是当前那格份数', async () => {
    render(<System />)
    await waitBackups()

    fireEvent.change(screen.getByLabelText('保留备份份数'), { target: { value: '3' } })
    fireEvent.click(screen.getByRole('button', { name: '清理旧备份' }))

    expect(await screen.findByText('只保留最新 3 份备份？')).toBeInTheDocument()
    fireEvent.click(screen.getByRole('button', { name: 'OK' }))
    await waitFor(() => expect(spies.cleanupBackups).toHaveBeenCalledWith({ maxBackups: 3 }))
  })

  it('被判 401 时页面上是服务端那句判决原话', async () => {
    spies.cleanupBackups.mockRejectedValue({
      response: { status: 401, data: { detail: '缺少或无效的 X-API-Key' } },
    })
    render(<System />)
    await waitBackups()

    fireEvent.click(screen.getByRole('button', { name: '清理旧备份' }))
    fireEvent.click(screen.getByRole('button', { name: 'OK' }))

    await waitFor(() =>
      expect(document.body.textContent).toContain('缺少或无效的 X-API-Key')
    )
  })
})

describe('System 备份面的失败只降级自己', () => {
  it('列表取不到时服务状态照旧渲染，卡内换成警示', async () => {
    spies.listBackups.mockRejectedValue(new Error('backup down'))

    render(<System />)

    expect(await screen.findByText(/必需依赖齐备/)).toBeInTheDocument()
    expect(await screen.findByText('备份列表未取到')).toBeInTheDocument()
    await waitFor(
      () => expect(document.body.textContent).toContain('获取备份列表失败'),
      { timeout: 4000 }
    )
  })

  it('候选取不到时只报候选那一句，备份列表照旧渲染', async () => {
    spies.getDataFiles.mockRejectedValue({
      response: { status: 500, data: {} },
    })

    render(<System />)

    await waitBackups()
    await waitFor(
      () => expect(document.body.textContent).toContain('获取数据文件列表失败'),
      { timeout: 4000 }
    )
  })
})
