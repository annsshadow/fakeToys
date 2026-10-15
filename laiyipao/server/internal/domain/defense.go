package domain

// 防线值守的**纯逻辑**在服务端的镜像（R1 第一批移植）。
//
// # 为什么先移植这些「跑不了引擎的部分」
//
// `defense.ts` 里最有价值的部分不是 `runChallenge`（它要整个引擎），
// 而是它**周围**那一圈纯函数：
//
//   - `validateSnapshot`  —— 挑战前置校验
//   - `isShieldActive`    —— 护盾是否过期
//   - `snapshotDigest`    —— 快照摘要
//   - **种子派生**         —— `defense_id × 1000003 + 分钟桶`，掩到 63 位
//
// 最后一条尤其关键：它意味着**挑战用的种子完全由服务端可算的量决定**，
// 不需要信任客户端报什么。所以即使引擎还没移植，
// 服务端已经能验证「你报的种子对不对」—— 这是 README 记的
// 「防线胜负由客户端裁决」那条已知边界上，**可以先收紧的一小块**。
//
// 剩下的 `runChallenge` 需要 `BattleEngine`，本文件**不碰**它：
// 宁可少移植，也不要移植一个跑不起来的半截。
//
// # 与客户端的契约
//
// `engine_pure_vectors.json` 由 `miniapp/scripts/gen-engine-pure-vectors.ts`
// 从**真实 TS 实现**产出。期望值不由 Go 侧生成 —— 理由同
// `replay_vectors.json`：TS 就是玩家设备上跑的那份代码，
// 它的输出是「合法值」的定义而不是自证。
//
// 守卫在 `engine_pure_contract_test.go`。

import (
	"fmt"
	"sort"
	"strings"
	"time"
)

// DefenseSnapshot 是服务端下发给客户端的防线快照（镜像 defense.ts 的同名接口）。
//
// ⚠️ 它与 `SnapshotHash(raw []byte)` 算的那个哈希**不是同一个东西**：
//   - 服务端那个哈希覆盖 JSON 全字段；
//   - 客户端 `snapshotDigest` 只覆盖 `skills` 与 `works` 两个列表。
//
// 口径不同是**刻意的**（前端那份是轻量本地校验），但也意味着：
// **不能用前端那份摘要去判定「挑战的是同一份构筑」** ——
// 两者覆盖面不同，前者可以相等而构筑已被改过。
type DefenseSnapshot struct {
	Skills       []int64 `json:"skills"`
	Equipment    []int64 `json:"equipment"`
	MasteryNodes []int64 `json:"mastery_nodes"`
	// Works 是工程装置代码：slow_belt / block_wall / tesla_grid。
	Works    []string `json:"works"`
	Elements []string `json:"elements"`
	Mastery  []int64  `json:"mastery"`
}

// ApplyWorks 把工程装置折算成攻方属性加成。
//
// ⚠️ 语义上有个容易读反的地方：**装置是「对方防线的」，
// 但加成给「我方攻方」** —— 因为挑战时装置驱动的是攻方预设强化。
// 注释里写的是「等价手段」而不是直接对应，正是为了说明这一点。
//
// 逐条对应 defense.ts 的 `applyWorks`：
//
//	slow_belt   → ElementCoefPermille += 100
//	block_wall  → ElementCap += 1
//	tesla_grid  → ReactionMultPermille += 200
//
// 未知代码**静默忽略**，与 TS 的 `switch` 无 `default` 分支一致。
// 装置表加了新代码而两端没同步时，症状是「装置没生效」而不是报错 ——
// 这是刻意选的失败形态：宁可少加也不能多加（多加会凭空抬高攻方）。
func ApplyWorks(att Attacker, works []string) Attacker {
	a := att
	for _, code := range works {
		switch code {
		case "slow_belt":
			a.ElementCoefPermille += 100
		case "block_wall":
			a.ElementCap += 1
		case "tesla_grid":
			a.ReactionMultPermille += 200
		}
	}
	return a
}

// DefenseView 是校验所需的最小防线视图。
//
// 只取 `validateSnapshot` 真正读到的字段，而不是把整个 API 响应搬过来 ——
// 搬过来之后每次响应结构变动都要同步改这里，而其中大部分字段
// 校验根本不看。
type DefenseView struct {
	ID            int64
	Snapshot      *DefenseSnapshot
	SnapshotHash  string
	ShieldedUntil *time.Time
}

// ValidateSnapshot 校验对方快照能否被还原。
//
// ⚠️ **顺序有意义**，与 TS 逐条一致：先看快照、再看技能非空、
// 再看哈希、最后才看护盾。改顺序会让同一份输入在两端给出**不同的话术**，
// 而玩家看到的话术是排查问题的第一线索。
//
// 第 2 条（技能非空）与第 3 条（哈希存在）都不是形式检查：
// 拿不到完整快照就跑，得到的胜负毫无意义，却会被服务端记账 ——
// 那是在**污染别人的战绩**。
func ValidateSnapshot(view DefenseView, now time.Time) (bool, string) {
	if view.Snapshot == nil {
		return false, "未获取到对方防线快照，无法模拟挑战"
	}
	if len(view.Snapshot.Skills) == 0 {
		return false, "对方快照里没有任何技能，无法模拟"
	}
	if view.SnapshotHash == "" {
		return false, "对方快照缺少哈希，无法确认挑战的是同一份构筑"
	}
	if IsShieldActive(view.ShieldedUntil, now) {
		return false, "对方护盾尚未过期"
	}
	return true, ""
}

// IsShieldActive 报告护盾是否**仍在生效**。
//
// ⚠️ 判断依据是**时间**，不是「字段是否存在」。
//
// 第 134 轮修过一次：旧代码用 `!!shielded_until` 判存在，
// 而护盾 24h 后 `shielded_until` 仍留在库里（是个过去的时间），
// 于是**已过期的护盾仍显示成「护盾开启」**。
//
// 解析失败返回 false（视为无护盾）：理由与 TS 一致 ——
// 一个时间戳都读不懂的字段，不该被当成「护盾开着」从而**永久禁止挑战**。
// 后者是把一个显示层的小问题变成功能不可用。
func IsShieldActive(until *time.Time, now time.Time) bool {
	if until == nil {
		return false
	}
	return until.After(now)
}

// validElements 是合法元素的全集，与 defense.ts 的 `valid` 数组一致。
//
// 顺序无关（只用 `in` 判断），但**集合必须一致** ——
// 多一个元素会让服务端把非法值也当成合法展示。
var validElements = map[string]bool{
	"fire": true, "ice": true, "lightning": true, "corrosion": true, "kinetic": true,
}

// SnapshotElements 过滤出快照里合法的元素。
//
// ⚠️ 过滤而非报错：快照是服务端生成的，非法值只可能是
// 老数据或运营手改。展示层要的是「能显示的那些」。
func SnapshotElements(snap *DefenseSnapshot) []string {
	if snap == nil {
		return []string{}
	}
	out := []string{}
	for _, e := range snap.Elements {
		if validElements[e] {
			out = append(out, e)
		}
	}
	return out
}

// SnapshotDigest 是客户端那份轻量摘要（镜像 defense.ts 的同名函数）。
//
// ⚠️ 与服务端 `SnapshotHash` 口径**不同**（见 DefenseSnapshot 的注释）。
//
//	排序是必要的：同一个构筑的快照，列表顺序不应影响摘要 ——
//	否则玩家改一次装备顺序就会「看起来像换了一份构筑」。
//
// `skills` 按数值排序、`works` 按字典序排序，与 TS 的
// `sort((a,b)=>a-b)` 与默认 `.sort()` 保持一致。
func SnapshotDigest(works []string, skills []int64) string {
	sk := make([]int64, len(skills))
	copy(sk, skills)
	sort.Slice(sk, func(i, j int) bool { return sk[i] < sk[j] })

	wk := make([]string, len(works))
	copy(wk, works)
	sort.Strings(wk)

	parts := make([]string, len(sk))
	for i, s := range sk {
		parts[i] = fmt.Sprintf("%d", s)
	}
	return hex16(fnv1a64(strings.Join(parts, ",") + "|" + strings.Join(wk, ",")))
}

// DefenseChallengeSeedBase 是种子派生里的质数乘子。
//
// ⚠️ 写死成常量而不是内联字面量：它是**跨端契约的一部分** ——
// 客户端按 `id * 1000003 + minute` 算，服务端要验证它报的种子。
// 一旦有一端改了它，所有历史挑战的种子全部失配，而症状是
// 「挑战报种子不匹配」，看起来像作弊。
const DefenseChallengeSeedBase = 1000003

// DefenseChallengeSeedMask 是种子的位掩码（63 位）。
//
// 取 63 位而不是 64 位，是为了留出符号位 —— Go 侧种子是 int64，
// 而 JS 的 BigInt 无符号运算的结果要与它逐位相同就必须对齐符号语义。
const DefenseChallengeSeedMask uint64 = (1 << 63) - 1

// DeriveChallengeSeed 派生挑战种子。
//
// 种子 = `(defense_id × 1000003 + 分钟桶) & (2^63 − 1)`
// 其中分钟桶 = `floor(now / 60000)`。
//
// # 为什么这条值得单独移植
//
// 它是防线挑战里**唯一不需要引擎就能验证**的量：
// 给定服务端自己知道的 defense_id 与服务端时钟，
// 它能算出「这一分钟里该用哪个种子」，从而校验客户端上报的 seed。
//
// 这不能解决 R2（胜负仍然由客户端裁决），但它把「种子」这一项
// 从「不可验证」变成「可验证」—— 而种子决定整局战局，
// 所以它是收敛 R2 的第一步。
//
// # 为什么掩到 63 位
//
// 负的 defense_id（理论上不会发生）与超大 id 都会溢出 int64。
// 掩码让两端在**任何**输入下都得到同一个非负值，
// 而不是各自溢出成不同的东西。
func DeriveChallengeSeed(defenseID int64, now time.Time) int64 {
	minute := now.UnixMilli() / 60000
	v := defenseID*DefenseChallengeSeedBase + minute
	return int64(uint64(v) & DefenseChallengeSeedMask)
}
