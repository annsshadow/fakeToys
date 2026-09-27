package domain

// 「过量伤害无害」的守卫。
//
// # 背景：一次被完整走完又回滚的调查
//
// 上一轮发现分数体系有个真实的不公平：
// `score = totalDamage/100 + kills*500`，而 `totalDamage`
// **含**打在已死敌人身上的部分（过量伤害 / overkill）。
//
// 实测（miniapp/src/game/elem_coef_shape.test.ts）：
// 元素系数 1000‰ → 3000‰ 期间**击杀数饱和在 818 → 820 不动**，
// 而总伤害涨 39% —— 那 39% 全部是过量伤害，而分数照样照付。
//
// 于是做了完整的「v2 口径」实现（只算有效伤害）+ 计分版本号 + 迁移，
// 结论是**回滚**。理由见下。
//
// # 为什么回滚：这个 exploits 早已被裁剪堵死
//
// `scoreCapFor` 在结算时把分数裁到 `gl.MaxScore`，而
//
//	MaxScore = Σ_敌人 (kill + (hp + shieldHP) / perDamageUnit)
//
// 也就是「击杀分 + 把每个敌人的全部血与盾按分算完」——
// **恰好就是 v2 口径的天花板**。
//
// 实测对照（第 50 关，满配构筑）：
//
//	v1 分数 25496  >  MaxScore 25348   ← 超出理论满分，结算时被裁到 25348
//	v2 分数 25285  <  MaxScore 25348
//
// 也就是说：**v1 本来就打出了超过理论满分的分数，然后被服务端截断。**
// 玩家从「多打一点」那里拿不到任何额外分数 —— 裁剪器就是那道闸。
//
// 所以 v2 修的是一个**收益已被现有机制覆盖**的问题，代价却是：
//
//   - 计分版本号机制（v1/v2 分支）+ 战报新列 + 跨端下发 + 重放按记录选口径
//   - **平衡回归**：实测第 50/57 关「满配 3 星 → 2 星」。
//     原因是被驱散的护盾在「只算吸收量」的口径下一分不给 ——
//     满配反应多、驱散多、白丢的盾血量也多，于是分数**低于**默认构筑。
//     驱散是奖励，让它扣分是反的。
//
// 收益已覆盖、代价真实且有回归 ⇒ 不做。
//
// # 本文件的作用：把上面那个「无害」的前提钉住
//
// 「过量伤害无害」**不是**一句解释，而是一条可检验的不变式：
// 无论客户端上报多大的分数，结算结果都不超过该关的 `MaxScore`。
// 若哪天有人放宽或删掉 `scoreCapFor`，这条会红 ——
// 那时「过量伤害无害」就不再成立，必须重新评估 v2。
//
// 留一条会红的守卫，比留一句「据说不成问题」有用。

import "testing"

// TestScoreIsCappedAtMaxScore 确认结算裁剪的上限**永不超过** MaxScore。
//
// 不变式就是这一条：`scoreCapFor(...) <= gl.MaxScore`。
// 结算时 `res.Score = min(in.Score, scoreCapFor(...))`，
// 所以只要上限不超过 MaxScore，客户端上报多大的分数都拿不到更多。
//
// ## 第一版把这条写成了同义反复
//
// 我原先在测试里**手工重算了一遍** `min(huge, cap)` 再断言等于 cap ——
// 那只证明了「我写的 min 是对的」。若 `scoreCapFor` 本身返回了
// 一个大于 MaxScore 的值，这条测试照样绿。
//
// 直接断言不变式才有效：它引用的是**生产函数本身**的返回值。
func TestScoreIsCappedAtMaxScore(t *testing.T) {
	// 时长从 0 扫到远超 ScoreFullAtSec ——
	// 额度随时长单调放宽，所以最松的情况必然出现在时长最大时。
	for _, id := range []int{1, 25, 50, 75, 100} {
		gl := GenerateLevel(id)
		if gl.MaxScore <= 0 {
			t.Fatalf("第 %d 关 MaxScore = %d，生成器有问题", id, gl.MaxScore)
		}
		for _, sec := range []int64{0, 1, 30, 60, 120, 300, 600, 5000} {
			cap := scoreCapFor(gl, sec)
			if cap > gl.MaxScore {
				t.Fatalf("第 %d 关 sec=%ds 的裁剪上限 %d 超过 MaxScore %d —— "+
					"「过量伤害无害」的前提失效，v2 口径需要重新评估",
					id, sec, cap, gl.MaxScore)
			}
		}
		// 时长足够长时额度应当就是满额（否则裁剪器会误伤干净通关）
		if cap := scoreCapFor(gl, ScoreFullAtSec+1); cap != gl.MaxScore {
			t.Errorf("第 %d 关时长充足时上限 = %d，应为 MaxScore %d（裁剪器会误伤干净通关）",
				id, cap, gl.MaxScore)
		}
	}
}

// TestMaxScoreCoversExactlyOneCleanRun 确认 MaxScore 就是「一次干净通关」的上限。
//
// 这是 v2 天花板与 v1 天花板**重合**的根据：
// 若 MaxScore 明显大于「敌人全部血与盾 + 全部击杀分」，
// 那两者就不是同一个天花板，「裁剪已覆盖 v2 的收益」这个论证就不成立。
func TestMaxScoreCoversExactlyOneCleanRun(t *testing.T) {
	for _, id := range []int{1, 25, 50, 75, 100} {
		gl := GenerateLevel(id)
		// 一个敌人一分不漏地打死、且不多打一分，能拿到的分数
		exact := int64(0)
		for _, w := range gl.Waves {
			for _, sp := range w.Spawns {
				exact += int64(sp.Count) * enemyScoreUpperBound(sp.EnemyID)
			}
		}
		if gl.MaxScore < exact {
			t.Errorf("第 %d 关 MaxScore = %d < 精确上限 %d —— 满分不可达，"+
				"「裁剪已覆盖」的论证不成立", id, gl.MaxScore, exact)
		}
		// 也不该明显大于：差值过大说明上限里掺了别的口径
		// （例如按含过量伤害的历史值算的），那 v2 与 v1 的天花板就不同
		if gl.MaxScore > exact*110/100 {
			t.Errorf("第 %d 关 MaxScore = %d 比精确上限 %d 高出 10%% 以上 —— "+
				"上限疑似按含过量伤害的口径推导，v1/v2 天花板并不重合",
				id, gl.MaxScore, exact)
		}
	}
}
