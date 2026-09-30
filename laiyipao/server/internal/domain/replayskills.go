package domain

import (
	"fmt"
	"sort"
	"strings"
)

// 回放前缀里的 **S 段**（技能槽配置）的服务端实现。
//
// # 为什么需要它
//
// `replay_hash` 整体**无法**被服务端廉价校验：它是
// `fnv1a64(前缀;事件1;事件2;…)` 的单向哈希，而前缀里的 `A` 段取自
// 客户端的 `currentAttacker()` —— 那个值含**战斗中选卡产生的 buff**，
// 服务端在结算时不知道。
//
// 但 S 段**不含 buff**，它完全由构筑决定（槽位、技能 id、底伤、叠层、热量），
// 服务端同样有这些数据，于是可以独立重算并比对。
//
// 它能抓住：
//   - 伪造底伤（等价于假报技能等级 —— 等级必须烘进底伤）
//   - 上报一套与 `user_skill_slots` 不同的技能
//   - 槽位错位
//
// 它抓不到 `A` 段（攻方系数）—— 那需要模拟，已记入 README 已知边界。

// ReplaySkillSlot 一条「槽位:技能:底伤:叠层:热量」的输入。
//
// BaseDamage 必须是**已含等级加成**的值 —— 客户端在
// `equippedFromSnapshot` 里用 `skillBaseDamageAtLevel` 烘好后放进哈希锚点，
// 这里必须用同一套烘法，否则合法对局会被判为作弊。
type ReplaySkillSlot struct {
	Slot        int
	SkillID     int
	BaseDamage  int64
	ApplyStacks int64
	HeatCost    int64
}

// ReplaySkillsSegment 拼出前缀里的 S 段。
//
// ⚠️ **排序的是格式化之后的字符串，不是按槽位。**
//
// 这是与客户端对齐的关键：TS 侧写的是
// `.map(...).sort().join(',')` —— `Array.prototype.sort()` 默认按
// 字符串码点比较，所以 `slot=10` 会排在 `slot=2` **前面**。
//
// 我第一版写成按 `Slot` 排序，结果 slot ≥ 10 的对局全部被判不匹配。
// 这类错误不会有任何编译期提示，只会在真实玩家的战报上炸。
func ReplaySkillsSegment(items []ReplaySkillSlot) string {
	parts := make([]string, 0, len(items))
	for _, it := range items {
		parts = append(parts, fmt.Sprintf(
			"%d:%d:%d:%d:%d", it.Slot, it.SkillID, it.BaseDamage, it.ApplyStacks, it.HeatCost,
		))
	}
	// ⚠️ 见上：必须排字符串，不能排 Slot。
	sort.Strings(parts)
	return strings.Join(parts, ",")
}

// SkillBaseDamageAtLevel 把等级加成烘进底伤。
//
// 与 TS 的 `skillBaseDamageAtLevel` 同式：
//
//	coef(level) = 1000 + (level-1) * CoefPermille
//	damage     = base * coef / 1000
//
// TS 侧对 `coef == 1000`（即 level 1）有快路径直接返回 base，
// 数值上与通用公式等价，这里不特判。
func SkillBaseDamageAtLevel(rules SkillRules, base int64, level int) int64 {
	coef := rules.CoefPermilleAt(level)
	return base * coef / 1000
}

// BuildReplaySkillsSegment 从「内容表 + 槽位 + 等级」算出 S 段。
//
// slots  是 `slot → skill_id`（未装备的槽位不出现在 map 里）
// levels 是 `skill_id → level`
//
// 未知技能 id **跳过**而不是回落到默认值 —— 回落到「某个默认技能」
// 会给作弊者一份白拿的底伤。客户端的 `equippedFromSnapshot` 同样
// 对查不到内容的技能 `continue`。
func BuildReplaySkillsSegment(
	content []SeedSkill,
	slots map[int]int,
	levels map[int]int,
	rules SkillRules,
) string {
	byID := make(map[int]SeedSkill, len(content))
	for _, s := range content {
		byID[s.ID] = s
	}
	items := make([]ReplaySkillSlot, 0, len(slots))
	for slot, skillID := range slots {
		def, ok := byID[skillID]
		if !ok {
			continue
		}
		lvl := 1
		if v, ok := levels[skillID]; ok {
			lvl = rules.ClampLevel(v)
		}
		items = append(items, ReplaySkillSlot{
			Slot:        slot,
			SkillID:     skillID,
			BaseDamage:  SkillBaseDamageAtLevel(rules, def.BaseDamage, lvl),
			ApplyStacks: def.ApplyStacks,
			HeatCost:    def.HeatCost,
		})
	}
	return ReplaySkillsSegment(items)
}

// ReplaySkillsMaxLen 是 `replay_skills` 的长度上限。
//
// 取值依据：一个 S 段最多 `MaxActiveSlots + 余量` 条，每条形如
// `slot:skillId:baseDamage:applyStacks:heatCost`，其中底伤是 6 位数级别，
// 单条不超过 40 字符。20 条即 800 字符。留到 1024 是为了让
// 「条数超了」与「内容超长」这两个问题能分开报。
const ReplaySkillsMaxLen = 1024

// ReplaySkillsMaxEntries 是条目数上限。
//
// 与 `MaxActiveSlots`（4，另加专精额外槽位）比，
// 留 5 倍余量仍然足以让「多塞了几条」这类问题暴露出来。
const ReplaySkillsMaxEntries = 20
