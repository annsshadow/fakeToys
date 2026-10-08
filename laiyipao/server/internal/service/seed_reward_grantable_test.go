package service

import (
	"context"
	"encoding/json"
	"strconv"
	"strings"
	"testing"
)

// 种子里的任务奖励必须是**可发放的货币或道具**（第 96 轮）。
//
// # 缺陷形态：种子里出现一个没有任何发放端的键
//
// 钱包只有两条发放通道：
//
//   - `walletColumns`：coin / gem / energy / keys（`user_wallets` 的列）
//   - `walletTokens`：revive_token / mastery_reset / gem_wash_token（第 74 轮加的道具）
//
// 而 `grantWallet` 对**不在这两份名单里**的键返回「未知货币」。
// 于是：种子里写错一个键名 → 该任务的奖励**永远发不出去**，
// 而任务在界面上显示「完成」、按钮亮着、点下去报错。
//
// 与第 74 轮同族（商城 8 件里 3 件买不了）：
// 那是 payload 用道具名而发货只认货币列；
// 这次是**奖励**里出现了不可发放的键。
//
// # 为什么这条值得单独守
//
// 第 74 轮的修法是「把 `walletTokens` 写成显式名单而非
// 「不在 walletColumns 里就算道具」」，那条名单**只被 Buy 用**。
// 任务的奖励走的是另一条路（`ClaimTask` → `grantWallet`），
// 两边共用同一份名单，但**没人检查种子里的键在名单里**。
//
// # 判据：种子里每个任务的奖励键都必须在 walletColumns ∪ walletTokens
//
// ⚠️ 用 Go 里的 `walletColumns` / `walletTokens` 而不是重新列一份清单 ——
// 复制一份清单就是「测副本」（第 93 轮踩过）。
func TestSeededTaskRewardsAreGrantable(t *testing.T) {
	scratch := openScratchService(t)
	ctx := context.Background()

	rows, err := scratch.pool.Query(ctx,
		`SELECT code, reward FROM tasks WHERE enabled ORDER BY code`)
	if err != nil {
		t.Fatalf("读任务失败：%v", err)
	}
	defer rows.Close()

	bad := []string{}
	n := 0
	for rows.Next() {
		var code string
		var raw []byte
		if err := rows.Scan(&code, &raw); err != nil {
			t.Fatal(err)
		}
		n++

		var reward map[string]int64
		if err := jsonUnmarshal(raw, &reward); err != nil {
			t.Errorf("%s 的 reward 不是合法 JSON：%v", code, err)
			continue
		}
		if len(reward) == 0 {
			bad = append(bad, code+": 奖励是空的")
			continue
		}
		for k, v := range reward {
			if _, ok := walletColumns[k]; !ok && !isWalletToken(k) {
				bad = append(bad, code+": 奖励里有不可发放的键 "+k)
			}
			if v <= 0 {
				bad = append(bad, code+": 奖励键 "+k+" 的值是 "+itoa(v)+"（必须为正）")
			}
		}
	}
	if err := rows.Err(); err != nil {
		t.Fatal(err)
	}
	if n == 0 {
		t.Skip("种子里没有任务 —— 判据前提不成立")
	}
	if len(bad) > 0 {
		t.Errorf("以下任务的奖励有问题：\n  %s\n\n"+
			"不可发放的键会让 `grantWallet` 返回「未知货币」，"+
			"于是任务显示「完成」、按钮亮着、点下去报错。\n"+
			"合法键只有两处来源：`walletColumns`（coin/gem/energy/keys）"+
			"与 `walletTokens`（道具白名单）。",
			joinLines(bad))
	}
}

// TestSignInCalendarRewardsAreGrantable 是同一判据在**签到日历**上的应用。
//
// ⚠️ 与任务分开测：它们是两张表、两条发放路径，
// 合成一条断言会让「哪张表出的问题」在失败信息里丢失。
func TestSignInCalendarRewardsAreGrantable(t *testing.T) {
	scratch := openScratchService(t)
	ctx := context.Background()

	rows, err := scratch.pool.Query(ctx,
		`SELECT day_index, reward FROM sign_in_calendar ORDER BY day_index`)
	if err != nil {
		t.Fatalf("读签到日历失败：%v", err)
	}
	defer rows.Close()

	bad := []string{}
	n := 0
	for rows.Next() {
		var day int
		var raw []byte
		if err := rows.Scan(&day, &raw); err != nil {
			t.Fatal(err)
		}
		n++
		var reward map[string]int64
		if err := jsonUnmarshal(raw, &reward); err != nil {
			t.Errorf("第 %d 天的 reward 不是合法 JSON：%v", day, err)
			continue
		}
		if len(reward) == 0 {
			bad = append(bad, dayName(day)+": 奖励是空的")
			continue
		}
		for k, v := range reward {
			if _, ok := walletColumns[k]; !ok && !isWalletToken(k) {
				bad = append(bad, dayName(day)+": 奖励里有不可发放的键 "+k)
			}
			if v <= 0 {
				bad = append(bad, dayName(day)+": 奖励键 "+k+" 的值是 "+itoa(v)+"（必须为正）")
			}
		}
	}
	if err := rows.Err(); err != nil {
		t.Fatal(err)
	}
	if n == 0 {
		t.Skip("签到日历是空的 —— 判据前提不成立")
	}
	if len(bad) > 0 {
		t.Errorf("以下签到奖励有问题：\n  %s\n\n"+
			"不可发放的键会让 `grantWallet` 返回「未知货币」—— "+
			"玩家看到「签到成功」却什么也没拿到（错误被吞成成功）。",
			joinLines(bad))
	}
}

/**
 * 下面三个 helper 只是为了让断言信息读起来像一句话。
 * 它们**不含任何判据** —— 判据在测试体里。
 *
 * 抽出来的唯一理由是：`%s` 里拼 `fmt.Sprintf` 会让失败信息
 * 变成一长串转义字符，而这里要读的是「哪个任务的哪个键不对」。
 */
func jsonUnmarshal(raw []byte, v any) error { return json.Unmarshal(raw, v) }

func joinLines(xs []string) string { return strings.Join(xs, "\n  ") }

func dayName(d int) string { return "第 " + strconv.Itoa(d) + " 天" }

func itoa(i int64) string { return strconv.FormatInt(i, 10) }
