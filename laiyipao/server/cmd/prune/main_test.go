package main

// 参数校验的守卫（2026-10-23）。
//
// 这个命令**不需要数据库**就能测它最危险的那部分：参数校验。
// 而参数校验正是「误删」唯一的拦路石 ——
// 一旦 `--days` 允许缺省或允许 1 天，剩下的代码写得再对也没用。
//
// 判据全部是「拒绝」：这里没有一条断言在证明「能删」，全在证明「删不了」。

import (
	"strings"
	"testing"
)

func TestParseOptsRequiresDays(t *testing.T) {
	if _, err := parseOpts(nil); err == nil {
		t.Fatal("不给 --days 必须报错 —— 给默认值等于允许「顺手跑一下」删掉数据")
	}
}

func TestParseOptsRejectsShortRetention(t *testing.T) {
	for _, d := range []string{"--days=1", "--days=6", "--days=-30"} {
		if _, err := parseOpts([]string{d}); err == nil {
			t.Errorf("%s 必须被拒绝（下限 %d 天）", d, minRetentionDays)
		}
	}
}

func TestParseOptsAcceptsMinimum(t *testing.T) {
	o, err := parseOpts([]string{"--days=7"})
	if err != nil {
		t.Fatalf("--days=7 应被接受：%v", err)
	}
	if o.Days != 7 {
		t.Fatalf("Days 应为 7，实际 %d", o.Days)
	}
	// 默认必须是 dry-run
	if o.Apply {
		t.Fatal("默认必须是 dry-run —— --apply 才能真删")
	}
}

func TestParseOptsApplyFlag(t *testing.T) {
	o, err := parseOpts([]string{"--days=30", "--apply"})
	if err != nil {
		t.Fatalf("解析失败：%v", err)
	}
	if !o.Apply {
		t.Fatal("--apply 未生效")
	}
}

func TestParseOptsBatchBounds(t *testing.T) {
	if _, err := parseOpts([]string{"--days=30", "--batch=0"}); err == nil {
		t.Error("--batch=0 必须被拒绝（0 会让删除循环永不推进）")
	}
	if _, err := parseOpts([]string{"--days=30", "--batch=999999"}); err == nil {
		t.Error("--batch 超上限必须被拒绝（长事务会锁表）")
	}
	o, err := parseOpts([]string{"--days=30", "--batch=1000"})
	if err != nil || o.Batch != 1000 {
		t.Fatalf("--batch=1000 应被接受，实际 %+v err=%v", o, err)
	}
}

func TestParseOptsTableWhitelist(t *testing.T) {
	if _, err := parseOpts([]string{"--days=30", "--table=wallet_flows"}); err == nil {
		t.Fatal("wallet_flows 不在白名单 —— 它是经济审计的原始凭证，删了无法追账")
	}
	if _, err := parseOpts([]string{"--days=30", "--table=users"}); err == nil {
		t.Fatal("users 不该出现在清理白名单里")
	}
	o, err := parseOpts([]string{"--days=30", "--table=battle_records"})
	if err != nil {
		t.Fatalf("battle_records 应可单独清理：%v", err)
	}
	if o.Target != "battle_records" {
		t.Fatalf("Target 应为 battle_records，实际 %q", o.Target)
	}
}

// TestPruneTargetsNeverIncludeWalletFlows 白名单的**结构性**守卫。
//
// 上面那条测的是「参数被拒绝」，这条测的是「名单本身没被改坏」——
// 有人往 pruneTargets 里加一张表时，两条会一起拦住。
func TestPruneTargetsNeverIncludeWalletFlows(t *testing.T) {
	for _, p := range pruneTargets {
		if p.table == "wallet_flows" || p.table == "users" || p.table == "user_wallets" {
			t.Fatalf("%s 绝不能进清理白名单：它是账务/身份的主数据", p.table)
		}
		if strings.TrimSpace(p.why) == "" {
			t.Fatalf("%s 缺少 why 说明 —— 运维看到一串表名只会凭感觉加表", p.table)
		}
	}
}
