# R23 规范：备战态构筑反应覆盖诊断

## §0 缺口依据（队长侦察坐实，非编造）
- laiyipao/miniapp/src/pages/skills/skills.vue:5 技能构筑台只显示图鉴/效率榜/推荐；:548-562 确认出战槽来自 store.loadout.slice(0,4) 的写入路径存在，但玩家在备战态看不到「这套构筑能触发哪些反应/补哪种元素补齐」，只能凭直觉。
- laiyipao/miniapp/src/store/game.ts:180-181 loadout 是 4 槽出战配置（0 表示空槽），服务端为权威来源；:245-253 equippedElements 已由 loadout → skillMap.element 派生元素列表，证明数据链现成。
- laiyipao/miniapp/src/game/types.ts:46/:54 配置技能同时下发 element 与 apply_element（引擎 applyElement 字段，见 scripts/probe-terrain.ts:25），二者同源。
- laiyipao/miniapp/src/utils/reactionPairs.ts:15-25 REACTION_PAIRS 由引擎 lookupReaction 对五元素两两穷举反查派生，天然与引擎一致；:9 明确「纯静态派生数据（不落库、不进任何哈希）」。
- laiyipao/miniapp/src/game/elements.ts:21-27 ELEMENT_NAME 提供元素中文名；:290-332 REACTION_TABLE 与 lookupReaction 是反应真相。
- 核心玩法是元素反应（R18 实测蒸汽爆发/过热/超导为伤害主力），但备战界面零覆盖提示 —— 这是核心决策点上的真实缺口。

## §1 数据基础（纯客户端派生）
- 权威构筑槽：store.loadout（4 槽，来自 /me 权威快照，见 game.ts:592-599 loadLoadout / :615-619 保存路径）。
- 槽位元素：对每个非空槽 id，从 store.config.skills 与 composite_skills（见 skills.vue:325-328 allSkillDefs）取技能定义，元素取 skill.element（与 apply_element 同源）。
- 反应真源：REACTION_PAIRS 与 lookupReaction（elements.ts:324-332）。
- 元素名：ELEMENT_NAME（elements.ts:21-27）；反应名用中文名映射，不暴露裸 key 作唯一显示。
- 红线：以上全部为客户端派生，不落库、不进 replay_hash、不改结算；诊断卡只读取不写入 store.loadout。

## §2 算法口径（确定性）
- 构筑覆盖集合 C(E)：对已装备元素多重集 E 两两 distinct 配对 a≠b，k=lookupReaction(a,b)，非空则入集；结果按 REACTION_ORDER（elements.ts:48）稳定排序、去重。
- 补齐建议：对五元素候选中每个元素 e（ELEMENT_ORDER 固定顺序），算新增集合 added(e)=C(E∪{e}) 减 C(E)；取 added 非空且元素个数最大者为最优补齐元素，并列时按 ELEMENT_ORDER 先者。展示 added 内反应中文名。
- 三分枝：
  1. E 无元素（全空槽或技能无元素）→ 诚实空态逐字：当前构筑没有元素技能，无法诊断反应覆盖
  2. 存在 e 使 added 非空 → AI 结论：本套构筑覆盖 X 种反应；补【元素名】可新增【反应名】等 N 种反应
  3. 所有 e 的 added 均为空（已覆盖全部可触发反应）→ 诚实空态逐字：当前构筑已覆盖全部可触发反应，无需补齐
- 边界：引擎 lookupReaction 需要两个不同元素，补**单个**元素要成反应须 E 已有至少 1 个其它元素；E 为空时直接走分支 1 诚实空态，不计算补齐（bestGap 返回 null），绝不编造「补一个元素就能触发反应」的假建议。
- 反应不可交换：lookupReaction 依赖先后，C(E) 取所有有序对，天然覆盖双向；不虚构任何伤害数值。
- 确定性：相同 loadout + config 两次派生结果逐项相等；不引入随机/时间/服务端调用。

## §3 选择器契约
- 卡片根 .r23-coverage-card
- 已覆盖反应标签 .r23-coverage-tag，每个标签带 [data-r23-reaction] 属性（值为 ReactionKey）
- 补齐建议 .r23-gap-suggest
- 诚实空态 .r23-coverage-empty（aria-live=polite / role=status）
- AI 依据与红线注记 .r23-ai-note
- 反应名/元素名用中文映射取名；无元素卡/空构筑只出空态，不编造预判。

## §4 AI 四判据
- 可见汉字「AI」徽标。
- 真实数据结论：由 C(E) 与 added(e) 真实计算得出，必须展示覆盖数 X 与补齐元素/新增反应中文名，不得编造。
- 依据逐字：依据：构筑槽元素 × REACTION_PAIRS(源自引擎 lookupReaction)派生
- 可执行 CTA：去调整出战配置 / 选它补齐。
- 诚实：无可补齐时只显示空态，不强造建议。

## §5 诚实红线（逐字）
- 辅助诊断非伤害承诺，实际触发取决于释放顺序/命中/层数，权威以 replay_hash 为准
- 不落库、不进 replay_hash、不改结算。
- 零真值：域名/IP/JWT/口令/个人路径一律不写入规范与测试，配置值用占位符。

## §6 契约测试
- 位置 laiyipao/miniapp/src/pages/skills/skills_r23_coverage.contract.test.ts
- 守卫：SPEC 数据基础/算法/选择器/AI/红线逐字；引擎真源确定性（C(E) 与补齐差集）；三端不回退。
- 参考实现用 lookupReaction/REACTION_PAIRS 钉死口径，不依赖尚未落地的业务代码；确定性元素集验证（已装备焰+冰→覆盖蒸汽爆发；再补电→新增过热/超导）。

## §7 三端不回退
- miniapp：store.loadout 4 槽写入路径（persistLoadout/saveLoadout）、equippedElements、REACTION_PAIRS、battle.vue 选卡必须保留；诊断卡为增量叠加。
- admin：令牌锚点 v1/v3-r3/r6 保留；无 --r23- 新令牌。
- server：replay_hash 口径不变（S 段构筑快照），诊断纯客户端不触碰。

## §8 分工
- 本规范 + 契约测试（design-lead）；业务实施由后续任务落地；H5 仅在 skills.vue 增诊断卡，不改战斗/结算链路。
