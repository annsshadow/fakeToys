/**
 * LevelsView.vue 测试。
 *
 * 关卡页是写操作最密集的页面：启停、改血量、重新生成 100 关、看波次。
 * 全部走「命令式弹窗确认 → 调 API → 原行合并返回值」的套路，
 * 每个动作的成功/取消/失败三态都直接影响线上关卡数据，因此逐一断言。
 */
import { flushPromises, mount } from '@vue/test-utils'
import { ElDrawer, ElSelect } from 'element-plus'
import ElementPlus from 'element-plus'
import { beforeEach, describe, expect, it, vi } from 'vitest'
import type { AdminLevel } from '@/api'

const fetchLevels = vi.hoisted(() => vi.fn())
const updateLevel = vi.hoisted(() => vi.fn())
const regenerateLevels = vi.hoisted(() => vi.fn())
const fetchLevelWaves = vi.hoisted(() => vi.fn())
const box = vi.hoisted(() => ({ prompt: vi.fn(), confirm: vi.fn() }))

vi.mock('@/api', () => ({ fetchLevels, updateLevel, regenerateLevels, fetchLevelWaves }))
vi.mock('element-plus', async (importOriginal) => {
  const mod = await importOriginal<typeof import('element-plus')>()
  return { ...mod, ElMessageBox: box }
})

import LevelsView from '@/views/LevelsView.vue'

function levelFixture(overrides: Partial<AdminLevel> = {}): AdminLevel {
  return {
    id: 1,
    chapter: 1,
    name: '第1关',
    seed: 's1',
    base_hp: 1000,
    wave_count: 5,
    difficulty: 1200,
    energy_cost: 6,
    is_boss: false,
    enabled: true,
    terrain_config: [{ kind: 'oil_drum' }, { kind: 'mystery_terrain' }],
    star_targets: [60, 120, 180],
    clear_rate: 55,
    avg_wave: 3.2,
    ...overrides,
  }
}

async function mountLevels(items: AdminLevel[] = [], total = items.length) {
  fetchLevels.mockResolvedValue({ items, total })
  const wrapper = mount(LevelsView, { global: { plugins: [ElementPlus] } })
  await flushPromises()
  return wrapper
}

beforeEach(() => {
  vi.clearAllMocks()
})

describe('LevelsView 列表渲染', () => {
  it('加载成功：难度/通关率色阶、地形文案、状态如实渲染', async () => {
    const wrapper = await mountLevels([
      levelFixture({ id: 1, difficulty: 1000, clear_rate: 60 }),
      levelFixture({ id: 2, difficulty: 1500, clear_rate: 30, terrain_config: [] }),
      levelFixture({ id: 3, difficulty: 3000, clear_rate: 15, terrain_config: undefined as unknown as AdminLevel['terrain_config'] }),
      levelFixture({ id: 4, difficulty: 4000, clear_rate: 5, enabled: false }),
    ])
    const text = wrapper.text()
    // 共 N 关
    expect(text).toContain('共 4 关')
    // 难度四档色阶都出现
    for (const d of ['1000‰', '1500‰', '3000‰', '4000‰']) expect(text).toContain(d)
    // 通关率已是百分数，直接展示
    expect(text).toContain('60.0%')
    expect(text).toContain('5.0%')
    // 地形：已知映射 + 未知回退原文 / 空数组与 undefined → 无
    expect(text).toContain('油桶、mystery_terrain')
    expect(text).toContain('无')
    // 状态
    expect(text).toContain('启用')
    expect(text).toContain('停用')
    wrapper.unmount()
  })

  it('查询按钮把章节/关键词传给 fetchLevels；空关键词传 undefined', async () => {
    const wrapper = await mountLevels()
    const vm = wrapper.vm as unknown as { chapter?: number; keyword: string; load: () => void }

    vm.keyword = '火'
    await wrapper.findAll('button').find((b) => b.text() === '查询')!.trigger('click')
    await flushPromises()
    expect(fetchLevels).toHaveBeenLastCalledWith({ chapter: undefined, keyword: '火' })

    vm.chapter = 3
    vm.keyword = ''
    await wrapper.findAll('button').find((b) => b.text() === '查询')!.trigger('click')
    await flushPromises()
    expect(fetchLevels).toHaveBeenLastCalledWith({ chapter: 3, keyword: undefined })
    wrapper.unmount()
  })

  it('加载失败：错误消息弹条 + 不崩溃', async () => {
    fetchLevels.mockRejectedValueOnce(new Error('数据库不可用'))
    const wrapper = mount(LevelsView, { global: { plugins: [ElementPlus] } })
    await flushPromises()
    expect(document.body.textContent).toContain('关卡加载失败：数据库不可用')
    wrapper.unmount()
  })
})

describe('LevelsView 启停开关', () => {
  it('启用成功：合并返回值、按新状态提示「已启用」', async () => {
    const row = levelFixture({ enabled: false })
    updateLevel.mockResolvedValueOnce({ level: { ...row, enabled: true } })
    const wrapper = await mountLevels([row])
    const vm = wrapper.vm as unknown as { toggleEnabled: (r: AdminLevel) => Promise<void> }
    await vm.toggleEnabled(row)
    await flushPromises()

    expect(updateLevel).toHaveBeenCalledWith(1, { enabled: true })
    expect(row.enabled).toBe(true)
    expect(document.body.textContent).toContain('第1关 已启用')
    wrapper.unmount()
  })

  it('停用成功：提示「已停用」', async () => {
    const row = levelFixture({ enabled: true })
    updateLevel.mockResolvedValueOnce({ level: { ...row, enabled: false } })
    const wrapper = await mountLevels([row])
    const vm = wrapper.vm as unknown as { toggleEnabled: (r: AdminLevel) => Promise<void> }
    await vm.toggleEnabled(row)
    await flushPromises()
    expect(document.body.textContent).toContain('第1关 已停用')
    wrapper.unmount()
  })

  it('启停失败：弹出错误消息', async () => {
    updateLevel.mockRejectedValueOnce(new Error('并发冲突'))
    const wrapper = await mountLevels([levelFixture()])
    const vm = wrapper.vm as unknown as { toggleEnabled: (r: AdminLevel) => Promise<void> }
    await vm.toggleEnabled(levelFixture())
    await flushPromises()
    expect(document.body.textContent).toContain('并发冲突')
    wrapper.unmount()
  })
})

describe('LevelsView 改血量', () => {
  it('确认输入：updateLevel 携带 Number 化的新血量，行数据合并', async () => {
    box.prompt.mockResolvedValueOnce({ value: '500' })
    const row = levelFixture({ base_hp: 1000 })
    updateLevel.mockResolvedValueOnce({ level: { ...row, base_hp: 500 } })
    const wrapper = await mountLevels([row])
    const vm = wrapper.vm as unknown as { onEdit: (r: AdminLevel) => Promise<void> }
    await vm.onEdit(row)
    await flushPromises()

    expect(box.prompt).toHaveBeenCalledWith(expect.stringContaining('第1关'), '编辑关卡', expect.anything())
    expect(updateLevel).toHaveBeenCalledWith(1, { base_hp: 500 })
    expect(row.base_hp).toBe(500)
    expect(document.body.textContent).toContain('已保存')
    wrapper.unmount()
  })

  it('弹窗取消：不调用更新接口，也不报错', async () => {
    box.prompt.mockRejectedValueOnce('cancel')
    const wrapper = await mountLevels([levelFixture()])
    const vm = wrapper.vm as unknown as { onEdit: (r: AdminLevel) => Promise<void> }
    await vm.onEdit(levelFixture())
    await flushPromises()
    expect(updateLevel).not.toHaveBeenCalled()
    wrapper.unmount()
  })
})

describe('LevelsView 重新生成 100 关', () => {
  it('确认后重新生成并刷新列表', async () => {
    box.confirm.mockResolvedValueOnce('confirm')
    regenerateLevels.mockResolvedValueOnce({ generated: 100 })
    const wrapper = await mountLevels([levelFixture()])
    const vm = wrapper.vm as unknown as { onRegen: () => Promise<void> }
    await vm.onRegen()
    await flushPromises()

    expect(box.confirm).toHaveBeenCalled()
    expect(regenerateLevels).toHaveBeenCalledTimes(1)
    expect(document.body.textContent).toContain('已重新生成 100 关')
    expect(fetchLevels).toHaveBeenCalledTimes(2) // 挂载一次 + 生成后刷新一次
    wrapper.unmount()
  })

  it('取消确认：直接返回，不触发重新生成', async () => {
    box.confirm.mockRejectedValueOnce('cancel')
    const wrapper = await mountLevels()
    const vm = wrapper.vm as unknown as { onRegen: () => Promise<void> }
    await vm.onRegen()
    await flushPromises()
    expect(regenerateLevels).not.toHaveBeenCalled()
    wrapper.unmount()
  })

  it('重新生成接口失败：弹出错误', async () => {
    box.confirm.mockResolvedValueOnce('confirm')
    regenerateLevels.mockRejectedValueOnce(new Error('生成器异常'))
    const wrapper = await mountLevels()
    const vm = wrapper.vm as unknown as { onRegen: () => Promise<void> }
    await vm.onRegen()
    await flushPromises()
    expect(document.body.textContent).toContain('生成器异常')
    wrapper.unmount()
  })
})

describe('LevelsView 工具栏与操作列交互', () => {
  it('关键词输入与章节下拉的 v-model 双向生效，查询按钮携带筛选参数', async () => {
    const wrapper = await mountLevels()
    // el-select 内部也有 input，必须用 placeholder 精确定位关键词输入框
    const input = wrapper.find('input[placeholder="按名称搜索"]')
    await input.setValue('火')
    expect((input.element as HTMLInputElement).value).toBe('火')

    const select = wrapper.findComponent(ElSelect)
    await select.vm.$emit('update:modelValue', 3)
    await flushPromises()

    await wrapper.findAll('button').find((b) => b.text() === '查询')!.trigger('click')
    await flushPromises()
    expect(fetchLevels).toHaveBeenLastCalledWith({ chapter: 3, keyword: '火' })
    wrapper.unmount()
  })

  it('操作列三个按钮（波次/改血量/启停）逐个点击走真实流程', async () => {
    const row = levelFixture({ id: 2, name: '交互关', enabled: false })
    fetchLevels.mockResolvedValue({ items: [row], total: 1 })
    // 改血量的返回值不带 enabled，避免合并后按钮状态提前翻转
    updateLevel
      .mockResolvedValueOnce({ level: { ...row, base_hp: 500 } })
      .mockResolvedValueOnce({ level: { ...row, enabled: true } })
    fetchLevelWaves.mockResolvedValue({ waves: [] })
    box.prompt.mockResolvedValueOnce({ value: '500' })

    const wrapper = mount(LevelsView, { global: { plugins: [ElementPlus] } })
    await flushPromises()
    const buttons = wrapper.findAll('.el-table__row')[0].findAll('button')
    expect(buttons.map((b) => b.text())).toEqual(['波次', '改血量', '启用'])

    await buttons[0].trigger('click') // 波次
    await flushPromises()
    expect(fetchLevelWaves).toHaveBeenCalledWith(2)

    await buttons[1].trigger('click') // 改血量
    await flushPromises()
    expect(updateLevel).toHaveBeenCalledWith(2, { base_hp: 500 })

    await buttons[2].trigger('click') // 启用
    await flushPromises()
    expect(updateLevel).toHaveBeenLastCalledWith(2, { enabled: true })
    expect(document.body.textContent).toContain('交互关 已启用')
    wrapper.unmount()
  })

  it('抽屉关闭走 v-model 的 onUpdate 处理器', async () => {
    fetchLevelWaves.mockResolvedValue({ waves: [] })
    const wrapper = await mountLevels([levelFixture()])
    const vm = wrapper.vm as unknown as { openWaves: (r: AdminLevel) => Promise<void>; waveDrawer: boolean }
    await vm.openWaves(levelFixture())
    await flushPromises()
    expect(vm.waveDrawer).toBe(true)

    await wrapper.findComponent(ElDrawer).vm.$emit('update:modelValue', false)
    await flushPromises()
    expect(vm.waveDrawer).toBe(false)
    wrapper.unmount()
  })
})

describe('LevelsView 波次抽屉', () => {
  it('打开抽屉加载波次：spawns 摊平为可读文本，非数组显示「—」', async () => {
    fetchLevelWaves.mockResolvedValueOnce({
      waves: [
        { wave_index: 1, spawns: [{ enemy_id: 3, count: 5, interval: 800, delay: 0 }] },
        { wave_index: 2, spawns: 'corrupted' },
      ],
    })
    const wrapper = await mountLevels([levelFixture({ name: '试炼一' })])
    const vm = wrapper.vm as unknown as {
      openWaves: (r: AdminLevel) => Promise<void>
      waveDrawer: boolean
    }
    await vm.openWaves(levelFixture({ id: 7, name: '试炼一' }))
    await flushPromises()

    expect(fetchLevelWaves).toHaveBeenCalledWith(7)
    expect(vm.waveDrawer).toBe(true)
    // el-drawer 默认不 teleport，渲染在组件树内
    const text = wrapper.text()
    expect(text).toContain('试炼一 · 波次编排')
    expect(text).toContain('#3×5（间隔 800ms，延迟 0ms）')
    expect(text).toContain('—')
    wrapper.unmount()
  })

  it('波次接口失败：弹错误且抽屉已打开（用户能看到失败重试入口）', async () => {
    fetchLevelWaves.mockRejectedValueOnce(new Error('波次查询失败'))
    const wrapper = await mountLevels()
    const vm = wrapper.vm as unknown as { openWaves: (r: AdminLevel) => Promise<void>; waveDrawer: boolean }
    await vm.openWaves(levelFixture())
    await flushPromises()
    expect(vm.waveDrawer).toBe(true)
    expect(document.body.textContent).toContain('波次查询失败')
    wrapper.unmount()
  })

  it('抽屉在无目标行时打开不炸：标题回退为空串（防御分支）', async () => {
    const wrapper = await mountLevels()
    const vm = wrapper.vm as unknown as { waveDrawer: boolean; waveTarget: AdminLevel | null }
    vm.waveDrawer = true
    vm.waveTarget = null
    await flushPromises()
    expect(wrapper.text()).toContain('· 波次编排')
    wrapper.unmount()
  })
})
