package domain

// 插槽与热量（I-2）在服务端的镜像（R1 第二批移植）。
//
// # 为什么热量值得先移植
//
// `heat_max` 目前是**客户端上报**的，服务端只把上限放宽到 1000（`MaxReportedHeat`）。
// 那 1000 是一个「理论上界」而不是「推导出来的值」——
// 服务端不知道自己算出来的热量应该是多少。
//
// 热量是纯状态机：只依赖 (heat, overheated, overheatRemaining, capBonus,
// decayRemainder, dtMs)，**完全不碰敌人、弹丸、引擎**。
// 所以它可以独立移植、独立验证，而验证之后服务端就能回答
// 「这一局的热量到底该是多少」。
//
// 移植它不等于能校验 `heat_max` —— 那还要知道整局的出招序列，
// 而那要等 tick 循环。但它把「不知道」变成「差最后一层」。
//
// # 移植时最容易错的两处
//
// **① 上限是千分比，不是绝对值。**
// `cap = HEAT_MAX × (1000 + capBonus) / 1000`。写成 `HEAT_MAX + capBonus`
// 的话，capBonus=1000‰ 时上限变成 110 而不是 1100 ——
// 实测后果是过热几乎不再发生、漏怪翻倍（TS 注释里有完整数据）。
//
// **② 衰减必须用「余数累加」。**
// 每 tick 名义衰减 = `10/s × 50ms = 0.5` 点，整数运算会截成 0 ——
// 于是热量只增不减，而过热又要求 `heat >= cap`，
// 两者叠加出一个吸收态：所有技能放不出、过热又永不触发，战斗永久停摆。
//
// 这两条都由 `engine_pure_contract_test.go` 的跨端向量守住。

import (
	"math"
	"strconv"
)

const (
	// ActiveSlots 是主动插槽数。
	ActiveSlots = 4
	// PassiveSlot 是被动插槽的槽位下标。
	PassiveSlot = 4
	// TotalSlots 是总槽位数。
	TotalSlots = 5

	// HeatMax 是热量基准上限（千分比换算前的绝对值）。
	HeatMax int64 = 100
	// OverheatDurationMs 是过热持续毫秒。
	OverheatDurationMs = 2000

	// HeatDecayPerSec 是热量自然衰减（每秒）。
	//
	// 没有衰减是不能玩的：上限 100、单发 20 热量意味着 5 次就过热，
	// 玩家被迫停手 2 秒。5 波次、30+ 敌人的关卡会变成「打 5 下等 2 秒」的循环。
	HeatDecayPerSec int64 = 10

	// DiscardPerWave 是每波免费弃牌次数。
	DiscardPerWave = 1
	// DiscardHeatRefund 是弃一张牌返还的热量。
	DiscardHeatRefund int64 = 1
)

// HeatCap 返回当前热量上限。
//
// ⚠️ `capBonus` 是**千分比**，所以是 `HEAT_MAX × (1000 + capBonus) / 1000`。
//
// 写成 `HEAT_MAX + capBonus`（把千分比当绝对值相加）时，
// capBonus=1000‰ 会让上限变成 110 而不是 1100。实测（100 关全量扫描）：
// 上限涨到 11 倍后过热几乎不再发生，而衰减率是固定的，
// 于是玩家长期停在「热量很高但放不出技能」的死区，漏怪翻 2~3 倍、分数掉 13.8%。
//
// 「提高热量上限」是**有利属性**，出现这种曲线说明单位错了，不是平衡问题。
func (h *HeatSystem) Cap() int64 {
	if h.CapBonus <= 0 {
		return HeatMax
	}
	return HeatMax * (permille + h.CapBonus) / permille
}

// HeatSystem 是热量状态机（镜像 `heatmap.ts` 的同名 class）。
//
// 没有任何方法接收引擎对象 —— 这是它能被独立移植的前提。
type HeatSystem struct {
	// Heat 是当前热量。
	Heat int64
	// Overheated 报告是否处于过热。
	Overheated bool
	// OverheatRemaining 是过热剩余毫秒。
	OverheatRemaining int
	// CapBonus 是上限加成（**千分比**，见 Cap 的注释）。
	CapBonus int64
	// MaxHeatThisBattle 是本局热量峰值，进结算上报。
	MaxHeatThisBattle int64

	// decayRemainder 是衰减的定点余数（千分之一热量）。
	//
	// 恒在 [0,1000) 区间，不存在溢出或无界增长。
	// 它是 tick 序的**确定性函数**（重放时每 tick 同样的 dt 得到同样的余数），
	// 因此**不需要**进回放哈希 —— 与 TS 的结论一致。
	DecayRemainder int64
}

// NewHeatSystem 造一个初始热量系统。
func NewHeatSystem() HeatSystem { return HeatSystem{} }

// TryCast 释放技能，返回是否成功。
//
// 语义：热量**允许正好到上限**，超过则拒绝。
// 因此 `TryCast(cap)` 应当成功，随后 `CheckOverheat` 判定为过热 ——
// 「用尽最后一点热量后被迫过热」正是 I-2 想要的代价设计。
func (h *HeatSystem) TryCast(cost int64) bool {
	if h.Overheated {
		return false
	}
	if h.Heat+cost > h.Cap() {
		return false
	}
	h.Heat += cost
	if h.Heat > h.MaxHeatThisBattle {
		h.MaxHeatThisBattle = h.Heat
	}
	return true
}

// RefundHeat 弃牌返还热量（夹在 0）。
func (h *HeatSystem) RefundHeat(n int64) {
	h.Heat -= n
	if h.Heat < 0 {
		h.Heat = 0
	}
}

// CheckOverheat 在热量达到上限时进入过热，返回是否**新进入**。
func (h *HeatSystem) CheckOverheat() bool {
	if !h.Overheated && h.Heat >= h.Cap() {
		h.Overheated = true
		h.OverheatRemaining = OverheatDurationMs
		return true
	}
	return false
}

// ForceOverheat 强制进入过热（无视 heat 是否到 cap）。
//
// 存在的唯一理由：消除「放不出技能又触发不了过热」的**吸收态**。
// `TryCast` 允许正好到 cap，`CheckOverheat` 要求 >= cap，
// 于是 `heat ∈ (cap-最便宜技能, cap)` 时既放不出也过热不了。
// 自然衰减会最终把它拉出来，但那要等 0.1~0.8 秒且期间完全没有反馈；
// 更要紧的是，任何让衰减失效的改动都会让这个状态变成**永久**的。
// 引擎在每 tick 的自动循环前用 IsStuckFor 兜底。
func (h *HeatSystem) ForceOverheat() bool {
	if h.Overheated {
		return false
	}
	h.Overheated = true
	h.OverheatRemaining = OverheatDurationMs
	return true
}

// AbsorbOverheat 被「隔热护罩」免疫一次过热：清空热量并退出过热状态。
//
// ⚠️ 这里**不能**用 `if h.Overheated { return }` 做前置检查。
//
// 引擎有两条进入过热的路径：
//  1. 正常开火打满 cap 后 `CheckOverheat` → Overheated 已是 true
//  2. 热量死区兜底 → **过热还没开始**，Overheated 仍是 false
//
// 而死区恰恰是护罩最该起作用的场景（热量卡住、动不了）。
// 带前置检查会让第 2 条路径下护盾完全无效。
//
// 另一个必须做的是**清空热量**：若只把 Overheated 置 false，
// 下一 tick 的死区兜底会再次判定「放不出技能」并再次过热 ——
// 变成「每 tick 白烧一层护盾」的空转。
func (h *HeatSystem) AbsorbOverheat() {
	h.Overheated = false
	h.OverheatRemaining = 0
	h.Heat = 0
}

// IsStuckFor 报告当前是否已无法释放任何给定成本的技能。
//
// 供引擎做死区兜底判断。costs 为空返回 false ——
// 「没有任何技能」与「技能都放不下」是两回事，前者不该触发过热。
//
// ⚠️ `cost <= 0` 也返回 false：那是一个免费技能，放得下就不算卡住。
func (h *HeatSystem) IsStuckFor(costs []int64) bool {
	if len(costs) == 0 {
		return false
	}
	for _, c := range costs {
		if c <= 0 {
			return false
		}
		if h.Heat+c <= h.Cap() {
			return false
		}
	}
	return true
}

// Update 推进一帧：先处理过热倒计时，否则做自然衰减。
//
// ⚠️ `dtMs` 是 **float64** 而不是 int，与 TS 的 `update(dtMs: number)` 同形。
//
// 这不是为了「和 TS 看起来一样」，而是**必要**：
// 引擎的步长是小数（16.666ms 之类），而衰减必须先把 dt 取整
// （TS 用 `Math.round`）。若签名是 int，取整就只能发生在**调用点** ——
// 于是「取整规则」散落到每一处调用，而其中一处忘了取整或取整方式不同，
// 两侧的衰减总量就会差一点点，且没有任何东西会报错。
//
// 参数收 float、取整收在本方法内，规则就只有一处。
func (h *HeatSystem) Update(dtMs float64) {
	if h.Overheated {
		h.OverheatRemaining -= int(math.Round(dtMs))
		if h.OverheatRemaining <= 0 {
			h.Overheated = false
			h.OverheatRemaining = 0
			// 过热结束：热量清零，但元素层数不清除（I-2 明确的设计）
			h.Heat = 0
		}
		return
	}
	if h.Heat > 0 {
		// ⚠️ 余数累加，**不能**写成 `(rate * dtMs) / 1000`。
		//
		// 那种写法下 `(10 × 50) / 1000 = 0` —— 整数除法把每 tick 的衰减
		// 彻底截断，衰减恒为 0。于是热量只增不减、只能靠过热清零；
		// 而过热又要求 heat >= cap。两者叠加出吸收态，战斗永久停摆。
		//
		// 余数累加：每 tick 累积 10×50 = 500，攒够 1000 才减 1 点，
		// 于是 1 秒（20 tick）恰好减 10 点，与 HeatDecayPerSec 一致。
		//
		// `math.Round` 与 JS 的 `Math.round` 同为「四舍五入到最近整数、
		// .5 向上」，不是截断。截断会与 TS 差 1，在长局里累积成可见偏差。
		h.DecayRemainder += HeatDecayPerSec * int64(math.Round(dtMs))
		decay := h.DecayRemainder / permille
		h.DecayRemainder -= decay * permille
		h.Heat -= decay
		if h.Heat < 0 {
			h.Heat = 0
		}
	}
}

// CooldownTarget 是 TickCooldown 需要的最小技能面。
//
// 刻意不接收完整的 `EquippedSkill`：本方法只读 cooldownMs / 只写 cooldownRemaining，
// 声明完整类型会强迫调用方与测试方构造十几个无关字段。
// 依赖面越窄越好改。
type CooldownTarget struct {
	CooldownMs        int64
	CooldownRemaining int64
}

// TickCooldown 推进所有技能的冷却（仅在未过热时）。
func (h *HeatSystem) TickCooldown(dtMs float64, skills []CooldownTarget) {
	if h.Overheated {
		return
	}
	d := int64(math.Round(dtMs))
	for i := range skills {
		if skills[i].CooldownRemaining > 0 {
			skills[i].CooldownRemaining -= d
			if skills[i].CooldownRemaining < 0 {
				skills[i].CooldownRemaining = 0
			}
		}
	}
}

// HeatSnapshot 是热量状态的对外表示（镜像 TS 的 `snapshot()`）。
type HeatSnapshot struct {
	Heat       string `json:"heat"`
	Overheated bool   `json:"overheated"`
}

// Snapshot 返回热量快照。
//
// ⚠️ Heat 是**字符串**：int64 可能超过 2^53，JSON number 在 JS 侧解析即失精
// （项目里踩过：种子曾因此变成 `...8032`，断言悄悄通过）。
func (h *HeatSystem) Snapshot() HeatSnapshot {
	return HeatSnapshot{Heat: strconv.FormatInt(h.Heat, 10), Overheated: h.Overheated}
}

// Card 是手牌。
type Card struct {
	ID     string `json:"id"`
	Kind   string `json:"kind"`   // skill / attribute / mechanic
	Rarity string `json:"rarity"` // common / rare / epic
	Name   string `json:"name"`
	Descr  string `json:"descr"`
	// SkillID 是 skill 卡的技能 id；其它卡为 0。
	SkillID int `json:"skill_id,omitempty"`
	// Tier 是 skill 卡的阶数。
	// Tier 是 skill 卡的星级（升格数）。
	Tier int `json:"tier,omitempty"`
	// Element 是卡面元素（skill 卡用；抽牌池按它展示）。
	Element Element `json:"element,omitempty"`
	// Effect 是属性卡的数值效果。
	//
	// 零值 Kind == "" 表示「这张不是属性卡」——
	// `applyCard` 的判据依赖这个区分。
	Effect AttributeEffect `json:"effect,omitempty"`
	// Mechanic 是机制卡的数值效果（同上）。
	Mechanic MechanicEffect `json:"mechanic,omitempty"`
	// hasSkillID 记录「skill_id 字段出现过」。
	//
	// 为什么不直接用 `SkillID != 0`：
	//
	//	`skillId === 0` 曾落进 `applyCard` 最后的隐式 else ——
	//	卡被 consume 掉、emit 了 card_taken，但什么也没发生，
	//	玩家看到「获得技能卡」却没反应。
	//	内容表当前 id 从 1 开始所以不触发，但 id 空间没有
	//	「必须非 0」的保证。
	//
	// 而 JSON 里 `omitempty` 会让 skill_id = 0 的字段**整个消失**，
	// 所以「0」与「没给」在反序列化后本来就不可区分。
	hasSkillID bool
}

// HasSkillID 报告这张卡是否携带 skill_id。
func (c Card) HasSkillID() bool { return c.hasSkillID }

// CardDeck 是手牌管理（镜像 `heatmap.ts` 的同名 class）。
type CardDeck struct {
	Hand []Card
	// DiscardsLeft 是本波剩余免费弃牌次数。
	DiscardsLeft int
}

// NewCardDeck 造一副牌手（弃牌次数为初始值）。
func NewCardDeck() CardDeck {
	return CardDeck{DiscardsLeft: DiscardPerWave}
}

// NewWave 重置每波一次的弃牌机会。
func (d *CardDeck) NewWave() { d.DiscardsLeft = DiscardPerWave }

// SetHand 替换手牌。
func (d *CardDeck) SetHand(cards []Card) { d.Hand = cards }

// Size 返回手牌张数。
func (d *CardDeck) Size() int { return len(d.Hand) }

// Take 按 id 取走一张牌，不存在返回 nil。
func (d *CardDeck) Take(id string) (Card, bool) {
	for i, c := range d.Hand {
		if c.ID == id {
			d.Hand = append(d.Hand[:i], d.Hand[i+1:]...)
			return c, true
		}
	}
	return Card{}, false
}

// DiscardResult 是弃牌的结果。
type DiscardResult struct {
	OK     bool  `json:"ok"`
	Refund int64 `json:"refund"`
}

// Discard 弃一张牌。返回是否成功与返还的热量。
//
// 次数用完则失败，且**不**移除牌 ——
// 否则玩家会在手牌没变少的情况下耗尽弃牌机会。
func (d *CardDeck) Discard(id string) DiscardResult {
	if d.DiscardsLeft <= 0 {
		return DiscardResult{OK: false, Refund: 0}
	}
	// ⚠️ 不看 `card` 的内容，只看「取到了没有」。
	// 若改成「按牌的种类决定能否弃」，那就不是弃牌次数而是牌面限制了。
	if _, ok := d.Take(id); !ok {
		return DiscardResult{OK: false, Refund: 0}
	}
	d.DiscardsLeft--
	return DiscardResult{OK: true, Refund: DiscardHeatRefund}
}

// Drop 移除手牌但**不消耗**弃牌次数。
//
// 跳过（skipCards）需要它：玩家「跳过」不是「弃牌」——
// 否则「跳过」与「逐张弃掉」是同一个操作，DiscardPerWave 的设计就失去意义。
func (d *CardDeck) Drop(id string) bool {
	_, ok := d.Take(id)
	return ok
}
