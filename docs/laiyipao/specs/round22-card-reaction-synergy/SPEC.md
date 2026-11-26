# R22 选卡反应协同预判 SPEC（round22-card-reaction-synergy）

> 目标：在卡牌选择的核心决策点，告诉玩家「这张卡能和我已装备技能的元素触发什么反应 / 能否补齐新反应」。
> 核心玩法是元素反应，但选卡界面此前零预判——本规范以纯客户端派生补齐，不落库、不进 replay_hash、不改结算。

## 0 缺口与依据（亲读源码，非编造）

- laiyipao/miniapp/src/pages/battle/battle.vue:208-248 选卡界面只显示卡牌元素标签（:239-242 `{{ elementName(c.element) }}`），玩家在决策点看不到反应预判。
- 数据基础全部现成：
  - laiyipao/miniapp/src/utils/reactionPairs.ts:15-25 的 REACTION_PAIRS 由引擎 lookupReaction 对五元素两两穷举反查派生，天然与引擎一致；纯静态派生不落库、不进任何哈希。
  - laiyipao/miniapp/src/game/replay.ts:222 equippedFromSnapshot 产出 EquippedSkill.applyElement（已装备技能实际施加的元素）。
  - laiyipao/miniapp/src/game/heatmap.ts:50-64 手牌 Card.element（有元素卡才有；属性/机制卡为空）。
  - laiyipao/miniapp/src/game/elements.ts:21-27 ELEMENT_NAME（焰/冰/电/毒/动能）、:324-332 lookupReaction、:290-310 REACTION_TABLE。
  - battle.vue:384-388 已 import ELEMENT_NAME/REACTION_ORDER/REACTION_PAIRS、Card/EquippedSkill 类型；:600 hand、:692 showCards、:695-697 elementName 均现成。
- 本规范只定义派生与展示口径，不新增后端字段、不改战斗结算、不改 replay_hash。

## 1 数据基础与派生口径（纯客户端派生）

- 唯一真源：REACTION_PAIRS（源自引擎 lookupReaction），禁止另起一张手写的反应表——禁止编造不存在于引擎的反应或元素对。
- 已装备元素集合 E：取 equippedFromSnapshot 产出的 EquippedSkill.applyElement，过滤空串；同一元素去重。
  - 不得凭卡牌 optimistic UI 猜测；以本局构筑快照派生的 EquippedSkill 为准（与 battle.vue:910-933 同一来源），保证预判与实际构筑一致。
- 手牌卡元素 c.element：仅 skill 卡且带 element 才参与预判；attribute/mechanic 卡（无 element）不预判（诚实）。
- 元素显示名一律用 ELEMENT_NAME；反应显示名一律用 REACTIONS[key].name（引擎真源），禁止硬编码中文反应名。
- 该派生结果是展示线索：不落库、不进 replay_hash、不改结算、不写入任何权威字段。

## 2 算法契约（覆盖集合 + 差集，确定性）

- 现有构筑覆盖反应集合 C(E)：对已装备元素集合 E 中任意两个不同元素 a、b 求 lookupReaction(a,b)，非空者收入集合（顺序无关：REACTION_TABLE 对称，两个方向都查；等价地用 REACTION_PAIRS 判定）。
- 对每张有元素手牌卡（元素 e）计算「加上它后新增的反应」：
  - C(E∪{e}) = 对 E∪{e} 两两 lookupReaction 得到的反应集合；
  - 新增集合 A = C(E∪{e}) − C(E)（差集，按 REACTION_ORDER 稳定排序）；
  - 可搭配元素 B = E 中那些与 e 组合能产生 A 中反应的元素（用 ELEMENT_NAME 显示，稳定去重排序）。
- 卡面文案三分枝（逐字格式）：
  1. A 非空：AI · 新增【X】等N种反应 / 可搭配 焰·冰（X 为 A 首项反应名，N=A.size，可搭配为 B 的元素名以 · 连接）。
  2. A 为空但 C(E∪{e}) 与 C(E) 有交集（只是重复已有反应）：AI · 强化已有【X】（X 为重复反应首项名）。
  3. 无装备元素（E 为空）或手牌卡无元素：不预判，显示诚实空态（见 §3 .r22-synergy-empty），绝不编造反应。
- 差集必须确定性：相同 E 与 e 的输入，两次计算结果逐项相等（unit 测试钉死）。
- 诚实示例（按引擎真相，非任务草稿示例）：已装备焰(fire)+冰(ice) → 覆盖 {蒸汽爆发 steam_burst}；再选电(lightning) → 新增 {过热 overheat（焰×电）, 超导 superconduct（冰×电）}。
  - 注：焰+冰在引擎里是蒸汽爆发 steam_burst，超导 superconduct 需冰+闪电；预判文案必须与引擎 lookupReaction 对齐，不得照抄错误映射。

## 3 选择器与逐字空态契约

- 每张参与预判的手牌卡根节点：.r22-synergy-card（-hand-card 内新增，不改变原点击/弃牌行为）。
- 每条反应预判标签：.r22-synergy-tag，每行带 [data-r22-reaction] 属性（值为 ReactionKey，如 steam_burst）。
- 诚实空态节点：.r22-synergy-empty（E 为空或卡无元素时显示）。
- AI 依据/红线节点：.r22-ai-note。
- 逐字空态：无装备元素时「当前构筑还没有元素技能，无法预判反应」；无元素卡时「该卡无元素，无法预判反应（属性/机制卡）」。
- 诚实红线展示（.r22-ai-note 内逐字包含）：辅助预判非伤害承诺，实际触发取决于释放顺序/命中/层数，权威以 replay_hash 为准。

## 4 AI 四判据（缺一不可）

1. 可见汉字「AI」：卡面预判文案与依据行均带可见的 AI 标识。
2. 真实数据结论：新增/强化反应名与条数必须来自 §2 派生差集，引用引擎真实 ReactionKey 与 ELEMENT_NAME，不得写死夸张结论或虚构伤害数值。
3. 依据行逐字：依据：已装备元素 × 手牌元素，经 REACTION_PAIRS(源自引擎 lookupReaction)派生
4. 可执行 CTA：去装备/选它（点选该卡即装备，或引导去装备页）。

## 5 诚实红线

- 辅助预判非伤害承诺，实际触发取决于释放顺序/命中/层数，权威以 replay_hash 为准。
- 预判只是展示线索：不落库、不进 replay_hash、不改结算、不参与战斗判定。
- 无装备元素或无元素卡时诚实空态，不编造反应、不虚构伤害数值。
- 反应名/元素名只取引擎真源（REACTIONS / ELEMENT_NAME），禁止手写映射漂移。
- 零真值：不得出现口令、令牌、真实域名/IP、个人路径与环境信息。

## 6 契约测试（test-first）

- 契约测试：laiyipao/miniapp/src/pages/battle/battle_r22_synergy.contract.test.ts。
- 守卫 SPEC 逐字契约、源码基线、AI 四判据、诚实红线、tokens 锚点不回退。
- unit 测试用确定性元素集（焰/冰/电）直接验证 REACTION_PAIRS 反应对与差集正确性，不依赖 UI 渲染。

## 7 三端不回退锚点

- miniapp/src/styles/tokens.css 保留 --lyp-token-version: lyp-tokens-v1、--lyp-r3-version: lyp-tokens-v3-r3、--lyp-r6-version: lyp-tokens-r6；禁止新增 --r22- 令牌。
- battle.vue 现有选卡行为（hand、showCards、elementName、takeCard、discardCard）与 REACTION_PAIRS import 必须保留，预判为增量叠加。
- R15-R21 时间线/验真/伤害贡献/效率/关卡教练等既有契约一律不动；本轮零后端改动、零结算改动。

## 8 分工（后续实施任务）

- 实施：在 battle.vue 选卡卡面按 §2 派生并渲染 §3 选择器与文案；纯前端增量，不改引擎/服务端。
- 验证：真浏览器打开选卡界面实测预判文案与实际可触发反应一致；契约测试与 tokens 守卫全绿。
