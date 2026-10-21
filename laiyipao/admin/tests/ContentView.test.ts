/**
 * ContentView.vue 测试。
 *
 * 公告与兑换码页的表单校验全在前端：
 * - 公告标题/正文必填；
 * - 兑换码奖励「货币:数量」解析（半角/全角冒号、逗号分隔、非法项整体拒绝）；
 * - 提交成功后表单重置并重新拉取列表。
 */
import { flushPromises, mount } from '@vue/test-utils'
import ElementPlus from 'element-plus'
import { beforeEach, describe, expect, it, vi } from 'vitest'

const fetchAnnouncements = vi.hoisted(() => vi.fn())
const createAnnouncement = vi.hoisted(() => vi.fn())
const fetchRedeemCodes = vi.hoisted(() => vi.fn())
const createRedeemCode = vi.hoisted(() => vi.fn())
const fetchAuditLogs = vi.hoisted(() => vi.fn())

vi.mock('@/api', () => ({
  fetchAnnouncements,
  createAnnouncement,
  fetchRedeemCodes,
  createRedeemCode,
  fetchAuditLogs,
}))

import ContentView from '@/views/ContentView.vue'

type Row = Record<string, unknown>

async function mountContent(data: { announcements?: Row[]; codes?: Row[]; logs?: Row[] } = {}) {
  fetchAnnouncements.mockResolvedValue({ items: data.announcements ?? [] })
  fetchRedeemCodes.mockResolvedValue({ items: data.codes ?? [] })
  fetchAuditLogs.mockResolvedValue({ items: data.logs ?? [] })
  const wrapper = mount(ContentView, { global: { plugins: [ElementPlus] } })
  await flushPromises()
  return wrapper
}

type Vm = {
  annForm: { title: string; body: string; published: boolean }
  codeForm: { code: string; reward: string; max_uses: number; expires_at: string }
  parseReward: (s: string) => Record<string, number> | null
  rewardText: (v: Record<string, number> | undefined) => string
  submitAnnouncement: () => Promise<void>
  submitCode: () => Promise<void>
  fmtTime: (s: string) => string
}

beforeEach(() => {
  vi.clearAllMocks()
})

describe('ContentView 列表渲染', () => {
  it('表单与页签的真实交互：公告/兑换码输入 v-model、立即发布勾选、页签切换', async () => {
    const wrapper = await mountContent()

    // 公告表单：标题输入、正文 textarea、立即发布勾选（三个 v-model 处理器）
    const title = wrapper.find('input[placeholder="例如：1.2 版本更新说明"]')
    await title.setValue('交互公告')
    const body = wrapper.find('textarea')
    await body.setValue('交互正文')
    const vm = wrapper.vm as unknown as Vm
    expect(vm.annForm).toEqual({ title: '交互公告', body: '交互正文', published: false })
    await wrapper.find('input[type="checkbox"]').setValue(true)
    await flushPromises()
    expect(vm.annForm.published).toBe(true)

    // 兑换码表单：码 / 奖励 / 过期时间三个输入 + 次数上限数字输入
    await wrapper.find('input[placeholder="LYP-2026-XXXX"]').setValue('LYP-VT')
    await wrapper.find('input[placeholder="coin:10000, gem:100"]').setValue('coin:7')
    await wrapper.find('input[placeholder="2026-12-31T23:59:59Z"]').setValue('2026-06-01T00:00:00Z')
    expect(vm.codeForm).toEqual({
      code: 'LYP-VT',
      reward: 'coin:7',
      max_uses: 100,
      expires_at: '2026-06-01T00:00:00Z',
    })

    // 点击「审计日志」页签 → el-tabs 的 update:modelValue
    await wrapper.findAll('.el-tabs__item').find((t) => t.text() === '审计日志')!.trigger('click')
    await flushPromises()
    const vmWithTab = wrapper.vm as unknown as { activeTab: string }
    expect(vmWithTab.activeTab).toBe('logs')
    wrapper.unmount()
  })

  it('次数上限（el-input-number）的 v-model 联动', async () => {
    const { ElInputNumber } = await import('element-plus')
    const wrapper = await mountContent()
    const nums = wrapper.findAllComponents(ElInputNumber)
    expect(nums).toHaveLength(1)
    await nums[0].vm.$emit('update:modelValue', 7)
    await flushPromises()
    const vm = wrapper.vm as unknown as Vm
    expect(vm.codeForm.max_uses).toBe(7)
    wrapper.unmount()
  })

  it('公告状态（已发布/草稿）、兑换码奖励（字符串 JSON/对象/null）、审计日志字段如实渲染', async () => {
    const wrapper = await mountContent({
      // 2026-10-21：与 service.AdminAnnouncementView 一致 ——
      // `published` 是**布尔**（此前 fixture 写的是 1/0，界面靠 num() 兜底，
      // 于是「已发布/草稿」其实是在比数字，不是比布尔）。
      announcements: [
        {
          id: 1,
          title: '维护公告',
          body: '今晚 02:00 维护',
          published: true,
          published_at: '2026-01-02T03:04:05Z',
          created_at: '2026-01-02T03:04:05Z',
        },
        {
          id: 2,
          title: '草稿公告',
          body: '还没发',
          published: false,
          published_at: null,
          created_at: '2026-01-02T03:04:05Z',
        },
      ],
      // 2026-10-21：reward 现在是**对象**（服务端 jsonb map），
      // 不再是 JSON 字符串。
      codes: [
        {
          id: 1,
          code: 'LYP-A',
          reward: { coin: 1000 },
          max_uses: 100,
          used_count: 3,
          expires_at: null,
          enabled: true,
        },
        {
          id: 2,
          code: 'LYP-B',
          reward: { gem: 50 },
          max_uses: 5,
          used_count: 0,
          expires_at: '2026-06-01T00:00:00Z',
          enabled: false,
        },
      ],
      // 第 121 轮：审计日志字段按**服务端真实形状**（AdminListAuditLogs：
      // username / action / target / detail），修前 UI 读的 admin_username /
      // target_type / target_id / reason 全都不存在，四列恒空白。
      logs: [
        {
          id: 1,
          admin_id: 1,
          username: 'admin',
          action: 'grant_currency',
          target: '9',
          detail: { currency: 'coin', amount: -300 },
          created_at: '2026-01-02T03:04:05Z',
        },
      ],
    })
    const text = wrapper.text()
    expect(text).toContain('已发布')
    expect(text).toContain('草稿')
    // 2026-10-21（P5-10）：公告正文与发布时间此前**不显示** ——
    // 服务端一直返回 `body` / `published_at`，运营却看不到自己发了什么。
    expect(text).toContain('今晚 02:00 维护')
    expect(text).toContain('还没发')
    expect(text).toContain('coin×1000')
    expect(text).toContain('gem×50')
    expect(text).toContain('3 / 100')
    expect(text).toContain('可用') // enabled=true
    expect(text).toContain('停用') // enabled=false
    expect(text).toContain('—') // 空时间
    // 审计日志：管理员 / 操作 / 对象 三列按服务端字段渲染
    expect(text).toContain('admin')
    expect(text).toContain('grant_currency')
    // detail 是 JSON 对象，detailText 渲染成可读文本
    expect(text).toContain('coin')
    expect(text).toContain('-300')
    wrapper.unmount()
  })

  /**
   * 2026-10-21：原先这里还直测了 `str` / `num` 两个「什么都往里塞」的兜底。
   * 它们服务的其实是**服务端根本不会发的形状**（`published: 1`、
   * `reward: '{"coin":1000}'` 这种字符串化 jsonb）——
   * 兜底让界面「看起来有数据」，也让 fixture 与线上错开。
   *
   * 现在数据有类型，兜底删了，判据换成「真实字段算对了没有」。
   */
  it('fmtTime 的容错分支直测', async () => {
    const wrapper = await mountContent()
    const vm = wrapper.vm as unknown as Vm
    expect(vm.fmtTime('')).toBe('—')
    // 第 124 轮：fmtTime 渲染成**查看者本地时区**（服务端下发 UTC）。
    // 判据与生产同一算法（Date + 本地分量），与机器时区无关。
    const iso = '2026-01-02T03:04:05Z'
    const d = new Date(iso)
    const p = (n: number) => String(n).padStart(2, '0')
    const localExpected = `${d.getFullYear()}-${p(d.getMonth() + 1)}-${p(d.getDate())} ${p(d.getHours())}:${p(d.getMinutes())}:${p(d.getSeconds())}`
    expect(vm.fmtTime(iso)).toBe(localExpected)
    // 非法输入退回纯展示，不崩
    expect(vm.fmtTime('not-a-time')).toContain('not-a-time')
    wrapper.unmount()
  })

  it('奖励格式解析：半角/全角冒号、多币种、空白项跳过；非法整体拒绝', async () => {
    const wrapper = await mountContent()
    const vm = wrapper.vm as unknown as Vm
    expect(vm.parseReward('coin:10000')).toEqual({ coin: 10000 })
    expect(vm.parseReward('coin:100，gem:200')).toEqual({ coin: 100, gem: 200 })
    expect(vm.parseReward(' coin : 50 , energy:1 ')).toEqual({ coin: 50, energy: 1 })
    expect(vm.parseReward('coin:100, bad-part')).toBeNull() // 任一项非法整体拒绝
    expect(vm.parseReward('coin:abc')).toBeNull()
    expect(vm.parseReward('')).toBeNull()
    expect(vm.parseReward(',,')).toBeNull()
    wrapper.unmount()
  })

  it('rewardText 直接验证：对象与空值两态（服务端现在下发对象）', async () => {
    const wrapper = await mountContent()
    const vm = wrapper.vm as unknown as Vm
    expect(vm.rewardText({ coin: 5 })).toBe('coin×5')
    expect(vm.rewardText({ gem: 2 })).toBe('gem×2')
    expect(vm.rewardText(undefined)).toBe('')
    wrapper.unmount()
  })

  it('加载失败（Promise.all 任一失败）：弹出统一错误消息', async () => {
    fetchAnnouncements.mockRejectedValueOnce(new Error('公告服务挂了'))
    const wrapper = mount(ContentView, { global: { plugins: [ElementPlus] } })
    await flushPromises()
    expect(document.body.textContent).toContain('运营数据加载失败：公告服务挂了')
    wrapper.unmount()
  })
})

describe('ContentView 公告表单', () => {
  it('标题为空：拦截并提示，不提交', async () => {
    const wrapper = await mountContent()
    const vm = wrapper.vm as unknown as Vm
    vm.annForm = { title: '  ', body: '正文', published: false }
    await vm.submitAnnouncement()
    expect(document.body.textContent).toContain('标题与正文都不能为空')
    expect(createAnnouncement).not.toHaveBeenCalled()
    wrapper.unmount()
  })

  it('正文为空：同样拦截（|| 右侧分支）', async () => {
    const wrapper = await mountContent()
    const vm = wrapper.vm as unknown as Vm
    vm.annForm = { title: '标题', body: '', published: false }
    await vm.submitAnnouncement()
    expect(document.body.textContent).toContain('标题与正文都不能为空')
    expect(createAnnouncement).not.toHaveBeenCalled()
    wrapper.unmount()
  })

  it('提交成功：创建后清空表单并重新拉取列表', async () => {
    const wrapper = await mountContent()
    const vm = wrapper.vm as unknown as Vm
    createAnnouncement.mockResolvedValueOnce({ item: {} })
    vm.annForm = { title: '新版本公告', body: '内容', published: true }
    await vm.submitAnnouncement()
    await flushPromises()

    expect(createAnnouncement).toHaveBeenCalledWith({ title: '新版本公告', body: '内容', published: true })
    expect(vm.annForm).toEqual({ title: '', body: '', published: false })
    expect(document.body.textContent).toContain('公告已创建')
    expect(fetchAnnouncements).toHaveBeenCalledTimes(2) // 挂载 + 成功后刷新
    wrapper.unmount()
  })

  it('提交失败：弹出后端错误', async () => {
    const wrapper = await mountContent()
    const vm = wrapper.vm as unknown as Vm
    createAnnouncement.mockRejectedValueOnce(new Error('敏感词拦截'))
    vm.annForm = { title: 'T', body: 'B', published: false }
    await vm.submitAnnouncement()
    await flushPromises()
    expect(document.body.textContent).toContain('敏感词拦截')
    wrapper.unmount()
  })
})

describe('ContentView 兑换码表单', () => {
  it('奖励格式非法：拦截并提示', async () => {
    const wrapper = await mountContent()
    const vm = wrapper.vm as unknown as Vm
    vm.codeForm = { code: 'LYP-X', reward: 'coin-100', max_uses: 5, expires_at: '' }
    await vm.submitCode()
    expect(document.body.textContent).toContain('奖励格式错误，应为「coin:10000, gem:100」')
    expect(createRedeemCode).not.toHaveBeenCalled()
    wrapper.unmount()
  })

  it('兑换码为空：奖励合法也拦截（先验奖励后验码的现有顺序）', async () => {
    const wrapper = await mountContent()
    const vm = wrapper.vm as unknown as Vm
    vm.codeForm = { code: '  ', reward: 'coin:100', max_uses: 5, expires_at: '' }
    await vm.submitCode()
    expect(document.body.textContent).toContain('兑换码不能为空')
    expect(createRedeemCode).not.toHaveBeenCalled()
    wrapper.unmount()
  })

  it('提交成功（带过期时间）：code trim、reward 解析、表单重置、列表刷新', async () => {
    const wrapper = await mountContent()
    const vm = wrapper.vm as unknown as Vm
    createRedeemCode.mockResolvedValueOnce({ item: {} })
    vm.codeForm = {
      code: ' LYP-X ',
      reward: 'coin:100,gem:10',
      max_uses: 5,
      expires_at: '2026-12-31T00:00:00Z',
    }
    await vm.submitCode()
    await flushPromises()

    expect(createRedeemCode).toHaveBeenCalledWith({
      code: 'LYP-X',
      reward: { coin: 100, gem: 10 },
      max_uses: 5,
      expires_at: '2026-12-31T00:00:00Z',
    })
    expect(vm.codeForm).toEqual({ code: '', reward: 'coin:10000', max_uses: 100, expires_at: '' })
    expect(document.body.textContent).toContain('兑换码已创建')
    expect(fetchRedeemCodes).toHaveBeenCalledTimes(2)
    wrapper.unmount()
  })

  it('提交成功（无过期时间）：expires_at 传 undefined 而不是空串', async () => {
    const wrapper = await mountContent()
    const vm = wrapper.vm as unknown as Vm
    createRedeemCode.mockResolvedValueOnce({ item: {} })
    vm.codeForm = { code: 'LYP-Y', reward: 'coin:1', max_uses: 1, expires_at: '' }
    await vm.submitCode()
    await flushPromises()

    expect(createRedeemCode).toHaveBeenCalledWith(
      expect.objectContaining({ code: 'LYP-Y', expires_at: undefined }),
    )
    wrapper.unmount()
  })

  it('提交失败：弹出后端错误', async () => {
    const wrapper = await mountContent()
    const vm = wrapper.vm as unknown as Vm
    createRedeemCode.mockRejectedValueOnce(new Error('兑换码已存在'))
    vm.codeForm = { code: 'LYP-Z', reward: 'coin:1', max_uses: 1, expires_at: '' }
    await vm.submitCode()
    await flushPromises()
    expect(document.body.textContent).toContain('兑换码已存在')
    wrapper.unmount()
  })
})
