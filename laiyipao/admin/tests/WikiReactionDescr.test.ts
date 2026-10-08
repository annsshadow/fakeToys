import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'

/**
 * 后台 wiki 的「反应链一览」表**必须展示 `descr`**（第 87 轮）。
 *
 * # 缺陷：字段被下发、被注释声称「文档站直接展示」，实际被丢弃
 *
 * 服务端 `ReactionSpec.Descr` 的注释写着：
 *
 * > 运营后台的玩法文档站直接展示它，因此必须由服务端下发
 *
 * 而 admin 的 `WikiView.vue` 里：
 *
 * ```ts
 * reactions: Array<{ key, name, base_coef, attack_weight_pct,
 *                    status_duration_ms, aoe_radius,
 *                    dispel_shield, amplify_pct }>   // ← 没有 descr
 * ```
 *
 * 表里有 7 列（系数 / 攻击力权重 / 控制效果 / 受击增伤 / 驱散护盾 / 溅射半径），
 * **没有「说明」列** —— 而同一页的敌人 / 技能 / 复合技能三张表都有。
 *
 * 连带后果比「少一列」严重：第 83 轮把 5 条**错误文案**改对了
 * （「范围伤害」「爆炸」「生成持续火区」「层数传播」「击退并削减护甲」
 * 都不存在），而那些修正**一个都没人看得到**。
 *
 * 我在第 83 轮把它们称为「对外文案」并援引了上面那条注释 ——
 * **结论下得太早**。字段在 JSON 里不等于有人在读它。
 *
 * 这与第 83 轮自己的发现同形：`aoeRadius` 的唯一消费者是屏幕震动。
 * **一个字段的「存在」与「被消费」是两件事，而只有后者有意义。**
 *
 * # 为什么用源码扫描而不是渲染断言
 *
 * 渲染断言能验证「表格里出现了说明文字」，但它需要一份包含
 * 7 条完整反应的数据夹具，而夹具里的 `descr` 又是我们自己写的 ——
 * 夹具说谎时渲染断言照样通过。
 *
 * 扫源码能直接回答「这张表有没有 `descr` 列」，
 * 且判据落在**能直接观测的那一层**（第 78/80/82/83 轮同一条原则）。
 */
describe('wiki 反应表：descr 必须被展示（第 87 轮）', () => {
  const src = readFileSync(resolve(__dirname, '../src/views/WikiView.vue'), 'utf-8')

  /** 取出「反应链一览」那张表所在的源码区间。 */
  function reactionTable(): string {
    const start = src.indexOf('cfg?.reactions')
    expect(start, '找不到绑定 cfg.reactions 的表 —— 结构变了').toBeGreaterThan(0)
    const end = src.indexOf('</el-table>', start)
    expect(end, '反应表没有正常闭合').toBeGreaterThan(start)
    return src.slice(start, end)
  }

  it('反应表里有「说明」列，且绑到 descr', () => {
    const table = reactionTable()
    expect(
      /prop="descr"[^>]*label="说明"/.test(table),
      `反应链一览表里没有 prop="descr" 的「说明」列。\n\n` +
        `当前这张表的列：${table.match(/label="[^"]+"/g)?.join('、')}\n\n` +
        '同一页的敌人 / 技能 / 复合技能三张表都有「说明」列，\n' +
        '只有反应表没有 —— 于是 descr 字段在文档站上是死的。',
    ).toBe(true)
  })

  it('反应表的本地类型声明里含 descr', () => {
    // ⚠️ 两条要分开测。
    //
    // 「有列」与「类型里有字段」是不同的两件事：
    //  只加列不加类型 → vue-tsc 会红（row.descr 不存在）
    //  只加类型不加列 → vue-tsc 全绿，但界面上还是看不到
    //
    // 后者正是本轮的缺陷形态 —— 类型里**没有** descr，
    // 所以连「想渲染都渲染不了」，而且类型检查器完全沉默。
    const m = src.match(/reactions:\s*Array<\{([^}]*)\}>/)
    expect(m, '找不到反应表的本地类型声明').not.toBeNull()
    expect(
      m![1],
      `反应表的本地类型缺少 descr：{${m![1]}}\n\n` +
        '这是本轮缺陷的**根因**：类型里没有 descr，\n' +
        '所以既没人写「说明」列、类型检查器也不会提醒。',
    ).toContain('descr')
  })

  it('四张内容表的「说明」列齐平（少一张就是漂了）', () => {
    // 判据是**计数**而不是「存在某个 el-table-column prop=descr」——
    // 后者在只有一张表加了列时也会通过。
    //
    // ⚠️ 计数会随模板结构调整而失效（把 4 张表合并成 1 张就会红）。
    // 那是**预期内的红**：那时需要重新确认每张表是否都该有说明列。
    // 一个会因为重构而红的守卫，比一个会因为漏一列而红的守卫更可接受 ——
    // 前者要人看一眼，后者默默放过。
    const descrColumns = src.match(/prop="descr"/g) ?? []
    expect(
      descrColumns.length,
      `wiki 里 prop="descr" 的列有 ${descrColumns.length} 处，` +
        `应为 4 处（敌人 / 技能 / 复合技能 / 反应）。\n` +
        '少一列就意味着那一类内容在文档站上没有说明文字。',
    ).toBe(4)
  })
})