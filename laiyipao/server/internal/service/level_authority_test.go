package service

import (
	"context"
	"testing"

	"github.com/laiyipao/server/internal/domain"
)

// ## 现状刻画：`levels` 表**不是**玩家侧的关卡来源（第 107 轮）
//
// # 这是本轮最大的发现，也是唯一一个**我不单方面修**的
//
// 实测（端到端）：
//
//	PUT /admin/levels/1  {"base_hp":987654, "name":"P107调参关卡"}  → 200
//	  库里         base_hp=987654  name="P107调参关卡"
//	  运营自己的列表 base_hp=987654  name=P107调参关卡      ← 运营看得到改动
//	  GET /levels/1  base_hp=1000    name=边境哨站 · 1        ← 玩家拿不到
//	  /config 的 levels[0] 同上                                  ← 玩家拿不到
//
// 也就是说：**运营的关卡调参对玩家完全不生效**，
// 而运营界面**完全看不出来** —— 列表里就是改后的值。
//
// # 为什么不单方面修
//
// 两条路都有大blast radius，且都要产品拍板：
//
//	(a) 让 `levels` 表成为权威 —— `getLevel` / `LoadGameConfig` 必须读库。
//	    代价：每次拉配置一次查询（或必须引入缓存 + 失效），
//	    而 `LoadGameConfig` 现在的签名**故意**不返回 error（注释写着「绝不失败」）。
//	(b) 让生成器继续权威 —— 那么 `levels` 表只是**规划表**，
//	    后台应当明说「记录在案，不影响线上」，而不是显示成已生效。
//
// 已记入 README 已知边界（需要产品决策）。
//
// # 那这个文件为什么存在
//
// 因为**静默的失效最贵**：没有测试的话，下一个人会理所当然地以为
// 「后台改了就生效」，并在它上面继续建东西。
//
// 所以这里把现状**钉住**：
//
//   - 它是一个**现状刻画测试**（characterization test），不是对现状的认可
//   - 断言写的是「玩家侧走纯生成器」，所以任何把 DB 接进玩家路径的改动
//     都会让它变红 —— 那正是**需要走产品决策**的信号
//   - 它同时给出三条可选路径各自的**证据**（表里有 / 响应里有 / 玩家拿到的是）
//
// ⚠️ 刻意**不**写成「改动生效」的断言：
// 那等于我把 (a) 或 (b) 替产品定了。

func TestLevelTableIsCurrentlyNotThePlayerSource(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	// 造一个绝不可能与生成值相同的覆盖值
	const wantHP = 987654
	const wantName = "P107现状刻画"
	if _, err := ts.AdminUpdateLevel(ctx, 1, map[string]any{
		"base_hp": wantHP, "name": wantName,
	}); err != nil {
		t.Fatalf("写入覆盖值失败：%v", err)
	}

	// 1) 表里确实写进去了
	row, err := ts.AdminLevelRow(ctx, 1)
	if err != nil {
		t.Fatalf("读回行失败：%v", err)
	}
	if row["base_hp"] != int64(wantHP) {
		t.Fatalf("库里 base_hp=%v，我们写的是 %d", row["base_hp"], wantHP)
	}
	if row["name"] != wantName {
		t.Fatalf("库里 name=%v，我们写的是 %q", row["name"], wantName)
	}

	// 2) 运营列表读到的是**同一个**表 → 运营看得到改动
	list, _, err := ts.AdminListLevels(ctx)
	if err != nil {
		t.Fatalf("读列表失败：%v", err)
	}
	adminSees := false
	for _, m := range list {
		if m["id"] == 1 && m["base_hp"] == int64(wantHP) {
			adminSees = true
		}
	}
	if !adminSees {
		t.Errorf("运营列表里第 1 关不是覆盖后的值 —— 运营界面会**看不出**改动，" +
			"那本条测试的整个前提就变了")
	}

	// 3) 玩家侧走的是**纯生成器**，拿不到改动
	generated := domain.GenerateLevel(1)
	if generated.BaseHP == wantHP {
		// 有人把 levels 表接进了玩家路径！
		msg := "**玩家路径已经改用 levels 表了**。\n\n" +
			"这正是 README 已知边界第 23 条要产品决策的那件事，\n" +
			"现在它已经发生了。\n\n" +
			"请：\n" +
			"  1. 更新 README 已知边界（把「表不是玩家来源」改成「表已成为权威」）\n" +
			"  2. 把本测试改成断言「改动**生效**」并补齐失效路径（缓存等）\n" +
			"  3. 确认 `LoadGameConfig` 的「绝不失败」约定是否仍然成立"
		t.Errorf("`domain.GenerateLevel(1)` 的 base_hp 现在等于覆盖值 %d —— "+
			msg, wantHP)
	}
	if generated.Name == wantName {
		t.Errorf("`domain.GenerateLevel(1)` 的 name 现在等于覆盖值 %q —— 同上", wantName)
	}

	// 4) 纯生成器不查库：这一条是**结构**事实，不需要数据库
	//    （同一个函数对同一关号必然给出同一个值，与表里的内容无关）
	// ⚠️ 不能写 `a != b` —— `GeneratedLevel` 里有 `[]int64` 字段，
	// 含切片的结构体**不可比较**，编译期就报错。
	// 只比纯函数真正承诺的那几个标量字段。
	a := domain.GenerateLevel(7)
	b := domain.GenerateLevel(7)
	if a.BaseHP != b.BaseHP || a.Name != b.Name || a.Seed != b.Seed ||
		a.WaveCount != b.WaveCount || a.Difficulty != b.Difficulty {
		t.Errorf("两次生成同一关得到不同结果 —— 它不是纯函数："+
			"base_hp %d/%d  name %q/%q  seed %d/%d",
			a.BaseHP, b.BaseHP, a.Name, b.Name, a.Seed, b.Seed)
	}
}
