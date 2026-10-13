package service

// 复活币（`revive_token`）在 `StartBattle` 里的消费闭环。
//
// # 为什么这组断言值钱
//
// 复活币此前是「买得到、用不了」的道具，商城里挂了很久。
// 它接上之后，最容易出错的**不是**加没加成，而是下面三件事：
//
//  1. **扣没扣对**：开战扣 1 枚，扣两次就等于吞玩家的道具
//  2. **该扣的时候没扣 / 不该扣的时候扣了**：没有币也扣 = 负余额；
//     有币不扣 = 免费续命
//  3. **加成漏下发**：扣了币却没放大血量 = 玩家白花道具
//
// 三条都只在真实购买 → 开战的链路上才看得见，所以这里走真实路径而不是
// 直接调内部函数。

import (
	"context"
	"testing"

	"github.com/laiyipao/server/internal/domain"
)

// reviveBalance 读当前复活币余额（0 表示没有这一行）。
func reviveBalance(t *testing.T, ts *testService, ctx context.Context, uid int64) int64 {
	t.Helper()
	var bal int64
	err := ts.pool.QueryRow(ctx,
		`SELECT balance FROM user_tokens WHERE user_id = $1 AND token = 'revive_token'`,
		uid).Scan(&bal)
	if err != nil {
		// ErrNoRows 在这里等价于「余额 0」——语义上就是没有币。
		return 0
	}
	return bal
}

// TestReviveTokenGrantsExtraBaseHPAndIsSpent 开战花 1 枚，给**额外当前血量**。
//
// 断言拆成三半，缺一不可：
//
//	ExtraBaseHP > 0        —— 确实给了东西
//	Level.BaseHP == 原值   —— **没有**去改关卡血量上限
//	ExtraBaseHP == N 次漏怪的伤害 —— 语义没漂移
//
// 第二条是第 148–152 轮那个无效实现的根因：抬高 `base_hp` 会同步抬高
// `baseHpMax`，而漏怪伤害以 `baseHpMax` 为分母，加成被整个抵掉
// （实测 100 关 0 关翻盘）。**只断言第一条的话，那个无效实现照样能通过。**
func TestReviveTokenGrantsExtraBaseHPAndIsSpent(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)
	ts.grant(t, ctx, uid, map[string]int64{"energy": 200})
	ts.grant(t, ctx, uid, map[string]int64{"revive_token": 2})

	base := domain.GenerateLevel(1).BaseHP

	resp, err := ts.StartBattle(ctx, uid, 1)
	if err != nil {
		t.Fatalf("开战失败：%v", err)
	}
	if !resp.Revived {
		t.Error("有币时 Revived 应为 true —— 否则客户端不知道要不要提示玩家")
	}

	wantExtra := domain.ReviveExtraBaseHP(domain.GenerateLevel(1))
	if resp.ExtraBaseHP != wantExtra {
		t.Errorf("额外血量 = %d，期望 %d（= %d 次漏怪的伤害）",
			resp.ExtraBaseHP, wantExtra, domain.ReviveExtraLeakCount)
	}
	if resp.ExtraBaseHP <= 0 {
		t.Errorf("额外血量 = %d —— 玩家花了币却什么也没拿到", resp.ExtraBaseHP)
	}
	// 关键：关卡本体必须原样下发。
	if resp.Level.BaseHP != base {
		t.Errorf("下发关卡的 base_hp = %d，已被改成 %d —— "+
			"抬高它会同步抬高漏怪伤害（分母是 baseHpMax），加成被抵掉",
			resp.Level.BaseHP, base)
	}
	if got := reviveBalance(t, ts, ctx, uid); got != 1 {
		t.Errorf("续战令余额 = %d，期望 1（开战扣 1 枚）", got)
	}
}

// TestReviveTokenSpentOncePerBattle 连续开战每次扣 1 枚，不多扣。
//
// 这条专防「在循环里扣」的错：那会让第二场战斗白扣两枚。
func TestReviveTokenSpentOncePerBattle(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)
	ts.grant(t, ctx, uid, map[string]int64{"energy": 500})
	ts.grant(t, ctx, uid, map[string]int64{"revive_token": 2})

	for i := 1; i <= 2; i++ {
		resp, err := ts.StartBattle(ctx, uid, 1)
		if err != nil {
			t.Fatalf("第 %d 场开战失败：%v", i, err)
		}
		if !resp.Revived {
			t.Fatalf("第 %d 场有币却没触发复活加成", i)
		}
		if got, want := reviveBalance(t, ts, ctx, uid), int64(2-i); got != want {
			t.Errorf("第 %d 场后余额 = %d，期望 %d（发了 2 枚，每场扣 1）", i, got, want)
		}
	}

	// 第三场没币了：必须静默降级成普通战斗，而不是报错
	resp, err := ts.StartBattle(ctx, uid, 1)
	if err != nil {
		t.Fatalf("无币时开战应成功（复活是可选增益，不是入场券）：%v", err)
	}
	if resp.Revived {
		t.Error("没有复活币时 Revived 不应为 true —— 那是白送一条命")
	}
	if resp.Level.BaseHP != base(t) {
		t.Errorf("无币时基础血量应为原值 %d，实际 %d", base(t), resp.Level.BaseHP)
	}
	if got := reviveBalance(t, ts, ctx, uid); got != 0 {
		t.Errorf("无币开战不应改变余额，实际 %d", got)
	}
}

func base(t *testing.T) int64 {
	t.Helper()
	return domain.GenerateLevel(1).BaseHP
}

// TestReviveDoesNotMutateGeneratorOutput 加成只改下发的副本，不改生成器输出。
//
// 理由与 domain.WithReviveBonus 的注释同源：生成器是纯函数，
// 就地改会污染同进程后续所有调用 —— 第 150 轮那次 skill_recipes 23505
// 就是这一类（浅拷贝改到了全局内容表）。这里钉住它。
func TestReviveDoesNotMutateGeneratorOutput(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)
	ts.grant(t, ctx, uid, map[string]int64{"energy": 200})
	ts.grant(t, ctx, uid, map[string]int64{"revive_token": 1})

	want := domain.GenerateLevel(1).BaseHP
	if _, err := ts.StartBattle(ctx, uid, 1); err != nil {
		t.Fatalf("开战失败：%v", err)
	}
	if got := domain.GenerateLevel(1).BaseHP; got != want {
		t.Errorf("GenerateLevel(1).BaseHP 被改成了 %d（原值 %d）—— 生成器必须是纯函数",
			got, want)
	}
}
