package service

import (
	"context"
	"encoding/json"
	"sort"
	"strings"
	"testing"
)

// 商城的 payload key 必须**真的能发货**（第 74 轮）。
//
// # 缺陷：商城 8 件商品里有 3 件永远买不了
//
// `Buy` 用 `grantWallet` 发货，而 `grantWallet` 走 `walletColumns` 白名单：
//
//	var walletColumns = map[string]string{
//	    "coin": "coin", "gem": "gem", "energy": "energy", "keys": "keys",
//	}
//
// 遇到不在表里的 key 就 `ErrBadInput: 未知货币`。
//
// 而种子数据里三个商品的 payload 是**道具名**：
//
//	shop_retry         -> {"revive_token": 1}
//	shop_mastery_reset -> {"mastery_reset": 1}
//	shop_gem_wash      -> {"gem_wash_token": 1}
//
// 于是那三件商品每次购买都在事务里失败并回滚，
// 客户端只看到一句「未知货币 "revive_token"」的 400 ——
// 一个语义完全错误的错误信息（它把「我们没实现这种货币」
// 说成「你请求的货币不存在」）。
//
// # 为什么此前没被发现
//
// 1. `walletColumns` 是**白名单**：新增商品时没人会去看它
// 2. 商城 e2e 测的是**限购与并发**（`concurrency_test.go`），
//    挑的是「第一个 limit_per_day > 0 的商品」——
//    而那恰好是 id=1 `shop_coin_small`，payload 是 `{"coin": 20000}`，能发货
// 3. 三个不可买的商品里有两个 `limit_per_day > 0`，
//    但没有任何测试遍历**所有**商品
//
// # 本文件的判据：结构，不是清单
//
// 扫**数据库里已启用的全部商品**的 price/payload，
// 断言每个 key 都落在 `walletColumns ∪ walletTokens` 里。
//
// ⚠️ 扫的是 DB 而不是 `seed.go` 的源码：
// 运营可以在 admin 后台改商品（`AdminUpdateShopItem` 只改
// name/price/enabled/limit_per_day，payload 改不了），
// 而 DB 是客户端**实际**看到的真相。
// 扫源码会在「DB 与种子不一致」时给出错误的绿灯。
//
// 加一件新商品而忘了在白名单里声明 → 这里会红。

// shopPayloadKeys 是从已启用商品里扫出的 (商品 code, 方向, key) 三元组。
type shopKey struct {
	Code    string
	Field   string // "price" 或 "payload"
	Key     string
	ItemID  int64
	Allowed bool
}

func scanShopKeys(t *testing.T, ts *testService) []shopKey {
	t.Helper()
	ctx := context.Background()
	rows, err := ts.pool.Query(ctx,
		`SELECT id, code, price, payload FROM shop_items WHERE enabled ORDER BY id`)
	if err != nil {
		t.Fatalf("读 shop_items 失败：%v", err)
	}
	defer rows.Close()

	var out []shopKey
	for rows.Next() {
		var id int64
		var code string
		var priceRaw, payloadRaw []byte
		if err := rows.Scan(&id, &code, &priceRaw, &payloadRaw); err != nil {
			t.Fatalf("扫描 shop_items 失败：%v", err)
		}
		for _, field := range []string{"price", "payload"} {
			raw := priceRaw
			if field == "payload" {
				raw = payloadRaw
			}
			var m map[string]int
			if err := json.Unmarshal(raw, &m); err != nil {
				// 运营把 JSON 写坏了 —— 那是另一个测试的事（coverage_final 有覆盖），
				// 这里跳过而不是让它冒充「货币不认识」。
				continue
			}
			for k := range m {
				_, isCol := walletColumns[k]
				out = append(out, shopKey{
					Code: code, Field: field, Key: k, ItemID: id,
					Allowed: isCol || isWalletToken(k),
				})
			}
		}
	}
	if err := rows.Err(); err != nil {
		t.Fatalf("遍历 shop_items 失败：%v", err)
	}
	return out
}

// TestEveryShopPayloadKeyIsSpendable 是本文件的核心。
func TestEveryShopPayloadKeyIsSpendable(t *testing.T) {
	ts := openScratchService(t)
	keys := scanShopKeys(t, ts)
	if len(keys) == 0 {
		t.Fatal("扫不到任何已启用商品的货币字段 —— " +
			"扫描器坏了，本测试会静默报告「全部可发货」")
	}

	var bad []string
	for _, k := range keys {
		if k.Allowed {
			continue
		}
		bad = append(bad, fmtShopKey(k))
	}
	if len(bad) == 0 {
		return
	}
	sort.Strings(bad)
	t.Errorf("以下商品的货币字段用了未声明的 key（%d 项）：\n  %s\n\n"+
		"未声明的货币会让 `Buy` 在事务里返回「未知货币」并回滚 ——\n"+
		"**这件商品永远买不了**，而客户端看到的是一句语义错误的 400。\n"+
		"修法：要么改用 walletColumns 里已有的货币，"+
		"要么在 service.go 的 walletTokens 里显式声明成道具。",
		len(bad), strings.Join(bad, "\n  "))
}

func fmtShopKey(k shopKey) string {
	return k.Code + " (id=" + itoa64(k.ItemID) + ")." + k.Field + "[\"" + k.Key + "\"]"
}

func itoa64(v int64) string {
	if v == 0 {
		return "0"
	}
	neg := v < 0
	if neg {
		v = -v
	}
	var buf [24]byte
	i := len(buf)
	for v > 0 {
		i--
		buf[i] = byte('0' + v%10)
		v /= 10
	}
	if neg {
		i--
		buf[i] = '-'
	}
	return string(buf[i:])
}

// TestBuyGrantsEveryTokenItem 端到端确认「买了真的到账」。
//
// 上一条验的是**声明层**（白名单里有它），这条验的是**行为层**
// （Buy 真的把它写进 user_tokens 并返回正确的 payload）。
func TestBuyGrantsEveryTokenItem(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	rows, err := ts.pool.Query(ctx,
		`SELECT id, code, payload FROM shop_items
		  WHERE enabled
		    AND payload ?| ARRAY['revive_token','mastery_reset','gem_wash_token']
		  ORDER BY id`)
	if err != nil {
		t.Fatalf("读道具商品失败：%v", err)
	}
	type item struct {
		id      int64
		code    string
		payload map[string]int
	}
	var items []item
	for rows.Next() {
		var it item
		var raw []byte
		if err := rows.Scan(&it.id, &it.code, &raw); err != nil {
			t.Fatal(err)
		}
		if err := json.Unmarshal(raw, &it.payload); err != nil {
			t.Fatal(err)
		}
		items = append(items, it)
	}
	rows.Close()
	if err := rows.Err(); err != nil {
		t.Fatal(err)
	}
	if len(items) == 0 {
		t.Fatal("种子里没有 payload 含道具的商品 —— 本测试已经失去意义")
	}

	for _, it := range items {
		t.Run(it.code, func(t *testing.T) {
			uid := ts.newUser(t, ctx)
			// 给足钱：道具商品的 price 用 coin 或 gem，两种都给。
			ts.grant(t, ctx, uid, map[string]int64{"coin": 10_000_000, "gem": 100_000})

			out, err := ts.Buy(ctx, uid, it.id)
			if err != nil {
				t.Fatalf("购买 %s 失败：%v", it.code, err)
			}
			for tok, want := range it.payload {
				if out[tok] != want {
					t.Errorf("Buy 返回的 payload 里 %s = %d，期望 %d", tok, out[tok], want)
				}
				bal, err := ts.Service.TokenBalance(ctx, uid, []string{tok})
				if err != nil {
					t.Fatalf("读道具余额失败：%v", err)
				}
				if bal[tok] != int64(want) {
					t.Errorf("user_tokens 里 %s = %d，期望 %d", tok, bal[tok], want)
				}
				var flowCount int
				if err := ts.pool.QueryRow(ctx,
					`SELECT COUNT(*) FROM wallet_flows
					  WHERE user_id = $1 AND currency = $2 AND delta = $3`,
					uid, tok, int64(want)).Scan(&flowCount); err != nil {
					t.Fatal(err)
				}
				if flowCount != 1 {
					t.Errorf("wallet_flows 里 %s 的进账流水有 %d 条，期望 1 条 —— "+
						"道具的进出必须出现在同一条流里，否则运营看板会显示「道具收入 0」",
						tok, flowCount)
				}
			}
		})
	}
}

// TestUnknownCurrencyIsStillRejected 守住「白名单是白名单」。
//
// 判据不能是「不在 walletColumns 里就当道具」——
// 那样打错字（`{"cino": 1}`）会被静默接受：
// 扣钱照扣、发货照发，只是发的东西没人看得见。
func TestUnknownCurrencyIsStillRejected(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)

	err := ts.DB.Tx(ctx, func(tx txType) error {
		return ts.grantWallet(ctx, tx, uid, map[string]int64{"cino": 1}, "test_unknown", 0)
	})
	if err == nil {
		t.Fatal("未知货币 cino 被放行 —— 它被静默当成了一种道具")
	}
	if !strings.Contains(err.Error(), "cino") {
		t.Errorf("错误信息里应当点名 cino（好让人知道是拼错了），实得 %v", err)
	}
}

// TestTokenUpsertCreatesRowOnFirstGrant 覆盖「道具行还不存在」这条路。
//
// 货币四列都有 DEFAULT 0、注册时就有行；道具是稀疏的。
// 如果这里用的是 `UPDATE` 而不是 UPSERT，第一件道具会**凭空消失**。
func TestTokenUpsertCreatesRowOnFirstGrant(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)

	var before int
	if err := ts.pool.QueryRow(ctx,
		`SELECT COUNT(*) FROM user_tokens WHERE user_id = $1`, uid).Scan(&before); err != nil {
		t.Fatal(err)
	}
	if before != 0 {
		t.Fatalf("新用户不应有任何道具行，实测 %d 行", before)
	}

	ts.grant(t, ctx, uid, map[string]int64{"revive_token": 2})
	ts.grant(t, ctx, uid, map[string]int64{"revive_token": 3})

	bal, err := ts.Service.TokenBalance(ctx, uid, []string{"revive_token"})
	if err != nil {
		t.Fatal(err)
	}
	if bal["revive_token"] != 5 {
		t.Errorf("两次各加 2 与 3 后余额 = %d，期望 5", bal["revive_token"])
	}
}

// TestTokenBalanceReportsUnownedTokensAsZero 让「0 个」与「不存在」可区分。
func TestTokenBalanceReportsUnownedTokensAsZero(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)

	bal, err := ts.Service.TokenBalance(ctx, uid,
		[]string{"revive_token", "mastery_reset"})
	if err != nil {
		t.Fatal(err)
	}
	for _, tok := range []string{"revive_token", "mastery_reset"} {
		v, ok := bal[tok]
		if !ok {
			t.Errorf("%s 没出现在结果里 —— 客户端无法区分「0 个」与「不存在」", tok)
		}
		if v != 0 {
			t.Errorf("%s = %d，期望 0", tok, v)
		}
	}
}
