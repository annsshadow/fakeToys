package domain

// 复活币（`revive_token`）的数值与实现。
//
// # 它是「战前加成」，不是「局内复活」
//
// 局内复活要改引擎状态机：致命伤判定、回血点、写进 replay 事件流。
// 战前加成只需要把服务端下发的 `base_hp` 放大一档 —— 玩家体感是
// 「多带了一条命」，而客户端**一行都不用改**（它直接用服务端给的 base_hp）。
//
// # 为什么这不会破坏 I-6（可重放性）
//
// 曾担心「复活改变时间线 → `replay_hash` 变 → 验真失效」。
// 核实后不成立：哈希的 prefix 是 `L${level.id}` + 攻击者派生属性 + 技能段，
// 之后接**完整事件流**。只要服务端在签发战斗令牌时就把加成的 `base_hp`
// 一并下发，重放拿到的是**同一份输入**，跑出的是同一条时间线，哈希自然相等。
//
// 真正的 I-6 杀手只有一种：**结算时按客户端上报的「我复活了几次」扣币**。
// 本实现不涉及 —— 币在开战时就扣，与体力同一时机、同一模型
// （`StartBattle` 里同在一个事务内）。
//
// # 这个数还需要标定
//
// 先取一个保守值。改它只需要动这一行；miniapp 的 `balance.probe.test.ts`
// 里有对应的基准可以复核。

// ReviveBaseHPMultPermille 是复活币的战前加成系数：基础血量乘以它。
const ReviveBaseHPMultPermille = 1500 // +50% 基础血量

// WithReviveBonus 返回把基础血量放大后的关卡副本。
//
// 值传递 + 返回副本：domain 的生成器输出是**纯函数**，不能就地改 ——
// 就地改会污染同进程后续所有调用（第 150 轮那次
// `skill_recipes` 23505 就是这一类：浅拷贝改到了全局内容表）。
func WithReviveBonus(gl GeneratedLevel) GeneratedLevel {
	gl.BaseHP = gl.BaseHP * ReviveBaseHPMultPermille / 1000
	return gl
}
