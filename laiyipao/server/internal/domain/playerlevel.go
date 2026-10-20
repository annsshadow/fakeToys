package domain

// 玩家等级换算。
//
// # 等级此前从未被物化
//
// `user_progress` 只有 `level_exp`（累计经验），**没有「等级」列**；
// 而 `skills.unlock_level`（基础技能 1..30、合成技能 30..62）
// 都以「等级」为门槛 —— 等级解锁落地时必须先有一个**唯一的**换算口径。
//
// # 口径从哪来：与 mastery_points 同源（不是新发明的常数）
//
// `user_progress.mastery_points = 3 + FLOOR(level_exp / 1000)`，
// 迁移注释写着「初始 3，每 10 级 +1」——
// 于是 1000 经验 = 10 级 ⇒ **1 级 = 100 经验**。
//
// # 为什么必须只有一个定义
//
// 同一个量在两处各写一份常量，漂了**不报错** ——
// 「同名不同义」是本项目踩过两次的形态（见工程约束 4）。
// 所以判定门槛时（service 的 SQL）与展示时（客户端）都从这里取分母，
// 谁也不许再写一个 100。
//
// # 等级只升不降
//
// `level_exp` 只增不减（结算时累加），因此等级单调不减 ——
// 这条不变量是熔炼门槛与等级解锁都依赖的前提。
const PlayerLevelDivisor = 100

// PlayerLevel 由累计经验换算玩家等级。
func PlayerLevel(levelExp int64) int {
	return int(levelExp / PlayerLevelDivisor)
}
