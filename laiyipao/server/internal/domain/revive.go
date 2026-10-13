package domain

// 复活币 `revive_token` 的效果与数值。
//
// # 为什么不是「+X% 基础血量」
//
// 第 148 轮把它做成 `base_hp × 1500‰`，看起来是战前加成的自然选择。
// 第 152 轮用引擎实跑 100 关证明**任何倍率都不生效**
// （见 `miniapp/src/game/revive_bonus_is_inert.test.ts`）：
//
//	普通怪漏怪伤害 = baseHpMax / LeakDamageDivisor   ← 与血量同步缩放
//	可承受漏怪数   = baseHpMax / (baseHpMax/10) = 10 ← 与倍率无关
//
// 护甲那层 `applyArmor` 是乘法，同样不改变这个比值。
// 把 `base_hp` 抬到 1.5 倍时，玩家血量 1.5B、每次漏怪扣 1.5B/10，
// **可承受的漏怪数还是 10**。玩家花了 5,000 金币什么也没换到。
//
// # 那什么才有效：不被 baseHpMax 缩放的**绝对**血量
//
// 引擎里 `baseHp`（当前血量）与 `baseHpMax`（血量上限）本来就是两个字段，
// 而漏怪伤害只看 `baseHpMax`。所以复活必须加在**当前血量**上，
// 且 `baseHpMax` 保持关卡原值 —— 这才是「多扛 N 次漏怪」。
//
// 语义因此定为「多扛 N 次漏怪」，而不是「血量 +X%」：
// 前者是玩家能理解的单位，且**与关卡血量缩放无关**，跨 100 关都一致。
//
// # 为什么仍然不做局内复活
//
// 局内复活要动战斗状态机、结算路径和 replay 事件流（`replayHash()` 的 prefix）。
// 那是第 148 轮就定下的边界：复活是道具，不是核心战斗改动。
// 战前给绝对血量缓冲不碰这三样。

// LeakDamageDivisor 是「漏一只普通怪扣多少血」的分母。
//
// ⚠️ 必须与 `miniapp/src/game/engine.ts` 的 `LeakDamageDivisor` 完全一致。
// 不同步的后果：服务端按 10 算多扛 3 次，引擎按别的值扣血，
// 玩家实际多扛的次数对不上 —— 而这个偏差**不会报错**，只会让道具效果偏掉。
// 守卫：`revive_contract_test.go`。
const LeakDamageDivisor = 10

// ReviveExtraLeakCount 是复活币让玩家**多扛几次漏怪**。
//
// 取 3 的理由（不是手感）：
//
//	战斗金币产出 `CoinBase 200 + 每星 150 + 每击杀 20`，三星十杀约 850/局
//	续战令售价 5,000 金币 ≈ 6 局
//	多扛 3 次漏怪 ≈ 一关里 30% 的漏怪额度（一关通常漏 10 次以内）
//
// 也就是说：花 6 局的钱，换「本来会漏死的那一波再多挨三次」。
// 够把一次失手救回来，又不够把一关白嫖过去。
//
// ⚠️ 仍**未经真实关卡分布标定**。`gem_balance_pins_test.go` 那种
// 「钉成断言」的写法在这里用不上：这一条要的是玩家实测数据，
// 不是可从代码量出来的量。定这个数的下一步是看线上 `battle_records`
// 的漏怪分布，而不是再改这个常量。
const ReviveExtraLeakCount = 3

// ReviveExtraBaseHP 返回复活币给的**额外当前血量**（绝对值）。
//
// 刻意**不改** `GeneratedLevel.BaseHP`：`BaseHP` 会进 `baseHpMax`，
// 而漏怪伤害以 `baseHpMax` 为分母 —— 一起改就把加成抵掉了（见文件头）。
func ReviveExtraBaseHP(l GeneratedLevel) int64 {
	if l.BaseHP <= 0 {
		return 0
	}
	return l.BaseHP * ReviveExtraLeakCount / LeakDamageDivisor
}
