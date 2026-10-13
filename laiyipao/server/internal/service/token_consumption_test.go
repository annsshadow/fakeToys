package service

// 道具消费闭环守卫（对应「C 口径」）。
//
// # 为什么需要它
//
// `walletTokens` 里的道具**买得到**（`seedShop` 里有对应的商品行），
// 但**能不能用**是另一回事。到本轮为止，三个道具里只有一个接通了消费端点。
//
// 症状从来不是报错，而是：
//   - 玩家在商城买了复活币，翻遍任何页面都找不到能用的地方
//   - 后台看到道具流水有入账，运营以为功能正常
//   - 覆盖率很高（`TokenBalance` / `Buy` / `grantWallet` 全都测到了），
//     而覆盖率在这里恰恰是**反向指标**
//
// 写成测试的价值：下一个人往 `walletTokens` 里加第四个道具时，
// **当场**被要求二选一 —— 接上扣减点，或者写明为什么还接不上。
// 这比写在注释里可靠：注释不会被 review 卡住，测试会。

import (
	"os"
	"path/filepath"
	"regexp"
	"strings"
	"testing"
)

// spendCallRe 匹配 `spendToken(ctx, tx, <谁>, "<token>", "<reason>", ...)`。
//
// 为什么按**调用点**判定而不是按「有没有某个函数处理它」：
// 扣减是唯一能证明「用掉了」的动作，而它必然长成一个 spendToken 调用。
// 用别的名字扣（例如直接 UPDATE user_tokens）会在测试里暴露出来 ——
// 那正是我们要的：要么统一走 spendToken，要么在这里改判据并说明理由。
var spendCallRe = regexp.MustCompile(`spendToken\(\s*[\w.]+\s*,\s*[\w.]+\s*,\s*[\w.]+\s*,\s*"([a-z_]+)"`)

// PENDING_TOKENS 是「已上架、尚无消费端点」的道具及其理由。
//
// 理由必须具体到能核对（指明前置条件 / 改动面），且 ≥40 字 ——
// 「暂时没空」这种写法等于把缺口永久合法化，没有任何测试会再提醒。
// 第 150 轮起为空：三种道具全部接通了消费端点。
//
//	revive_token   → StartBattle 开战扣 1 枚 + 本关 base_hp 加成
//	mastery_reset  → POST /mastery/reset
//	gem_wash_token → POST /me/gems/:id/wash
//
// 空表本身不是「完成」的证明 —— `TestEveryWalletTokenIsSpendableOrRegistered`
// 会逐个核对 walletTokens，而 `TestShopDoesNotSellUnspendableTokens`
// 保证「待接通」与「在售」互斥。下一个人往 walletTokens 加新道具时，
// 会被当场要求二选一。
var PENDING_TOKENS = map[string]string{}

// tokensSpentInSource 扫出所有被 spendToken 扣减过的道具名。
func tokensSpentInSource(t *testing.T) map[string][]string {
	t.Helper()
	entries, err := os.ReadDir(".")
	if err != nil {
		t.Fatalf("读包目录失败：%v", err)
	}
	out := map[string][]string{}
	for _, e := range entries {
		name := e.Name()
		if e.IsDir() || !strings.HasSuffix(name, ".go") || strings.HasSuffix(name, "_test.go") {
			continue // 测试里的扣减不算：线上没人走那条路
		}
		src, err := os.ReadFile(filepath.Join(".", name))
		if err != nil {
			t.Fatalf("读 %s 失败：%v", name, err)
		}
		for _, m := range spendCallRe.FindAllStringSubmatch(string(src), -1) {
			out[m[1]] = append(out[m[1]], name)
		}
	}
	return out
}

// TestEveryWalletTokenIsSpendableOrRegistered 每种上架道具都要么有扣减点，
// 要么在 PENDING_TOKENS 里写明原因。
func TestEveryWalletTokenIsSpendableOrRegistered(t *testing.T) {
	spent := tokensSpentInSource(t)

	var orphans []string
	for token := range walletTokens {
		if len(spent[token]) > 0 {
			continue
		}
		if _, ok := PENDING_TOKENS[token]; ok {
			continue
		}
		orphans = append(orphans, token)
	}
	if len(orphans) > 0 {
		t.Errorf("这些道具买得到却没有消费端点，也没登记为待接通：\n  %s\n\n"+
			"玩家花钱买了却在任何地方都用不了，而后台看到的只是「一切正常」。\n"+
			"要么给它接上扣减点（spendToken），要么在 PENDING_TOKENS 里写明"+
			"为什么接不上（要具体到前置条件与改动面）。",
			strings.Join(orphans, "\n  "))
	}
}

// TestPendingTokensHaveNoSpendCall 登记为「待接通」的道具不许已经有扣减点。
//
// 方向相反的守卫：不加这条，某天有人接上了 `revive_token` 却忘了删豁免，
// 于是真正的缺口被一条过期豁免永久掩盖 —— 而 PENDING_TOKENS 里那几行
// 理由会在 review 里显得「仍然成立」。
func TestPendingTokensHaveNoSpendCall(t *testing.T) {
	spent := tokensSpentInSource(t)
	for token, files := range spent {
		if _, ok := PENDING_TOKENS[token]; ok {
			t.Errorf("道具 %q 已经有扣减点（%s），却仍登记为待接通 —— "+
				"请删掉 PENDING_TOKENS 里的条目，否则真正的缺口会被过期豁免掩盖",
				token, strings.Join(files, ", "))
		}
	}
}

// TestPendingTokensAreRealTokens 豁免必须针对真实存在的道具。
//
// 防的是「凭空的豁免」：条目名拼错或道具已下线时，这张表会静默失效。
func TestPendingTokensAreRealTokens(t *testing.T) {
	for token, why := range PENDING_TOKENS {
		if !walletTokens[token] {
			t.Errorf("PENDING_TOKENS 里有 %q，但它不在 walletTokens 里 —— "+
				"道具可能已下线，豁免该删了", token)
		}
		if n := len([]rune(why)); n < 40 {
			t.Errorf("道具 %q 的待接通理由只有 %d 字，太短。"+
				"它必须能回答「为什么接不上」并指出前置条件或改动面，"+
				"否则这个缺口会被永久合法化", token, n)
		}
	}
}

// TestShopDoesNotSellUnspendableTokens 商城**不得**出售待接通的道具。
//
// 为什么能写成断言了（第 150 轮下架机制之后）：
// 此前复活币与洗练石一边「没有消费端点」一边在商城里卖，唯一的处理是
// 在界面文案上说「目前无法使用」—— 那只是**告知**，不是**约束**：
// 后台照样能上架，运营照样能买，玩家照样会投诉。
//
// `seedShop` 里的 `delisted` 标记把它们 enabled 置 false，
// 于是「待接通」与「在售」变成互斥状态，可以被机器判定。
//
// ⚠️ 本守卫是**双向**的，且两条方向的失败都是真缺口：
//   - 待接通却在售 → 玩家花钱买了用不了
//   - 已下架却不待接通 → 要么它其实能用（该上架），要么豁免没登记
//
// 自检只要求「扫到商品种子」，**不要求扫到下架项** ——
// 下架数为 0 恰恰是目标状态，拿它当判据会在全部接通后误报。
func TestShopDoesNotSellUnspendableTokens(t *testing.T) {
	delisted, sold := shopTokensByState(t)
	if len(sold)+len(delisted) == 0 {
		t.Fatal("一条商品都没扫到 —— 判据失效了（seedShop 的写法变了吗？）")
	}
	for token := range delisted {
		if _, pending := PENDING_TOKENS[token]; !pending {
			t.Errorf("商城下架了 %q，但它并不在 PENDING_TOKENS 里 —— "+
				"要么它其实能用（那就该上架），要么该补上待接通理由", token)
		}
	}
	for token := range PENDING_TOKENS {
		if sold[token] {
			t.Errorf("道具 %q 还没有消费端点，却仍在商城出售 —— "+
				"玩家花钱买了却用不了。要么在 seedShop 里给它加 delisted，"+
				"要么把它的消费端点接上。", token)
		}
	}
}

// shopTokensByState 按「是否下架」把商城发放的道具分成两组。
func shopTokensByState(t *testing.T) (delisted, sold map[string]bool) {
	t.Helper()
	raw, err := os.ReadFile(filepath.Join("..", "seeder", "seed.go"))
	if err != nil {
		t.Fatalf("读 seeder/seed.go 失败：%v", err)
	}
	delisted, sold = map[string]bool{}, map[string]bool{}
	for _, line := range strings.Split(string(raw), "\n") {
		line = strings.TrimSpace(line)
		if !strings.HasPrefix(line, "{") {
			continue
		}
		isDelisted := delistedTailRe.MatchString(line)
		if !isDelisted && !soldItemRe.MatchString(line) {
			continue
		}
		for _, g := range shopGrantRe.FindAllStringSubmatch(line, -1) {
			if walletTokens[g[1]] {
				if isDelisted {
					delisted[g[1]] = true
				} else {
					sold[g[1]] = true
				}
			}
		}
	}
	return delisted, sold
}

// soldItemRe 匹配商品定义行**末尾**的 `, false},`（即在售）。
var soldItemRe = regexp.MustCompile(`,\s*false\},?\s*$`)

// shopGrantRe 匹配商品定义里的发放列，例如 `"revive_token": 1}`。
var shopGrantRe = regexp.MustCompile(`"([a-z_]+)":\s*\d+\}`)

// delistedTailRe 匹配 seedShop 里商品定义行**末尾**的 `, true},`。
//
// 为什么只看行尾而不解析整个结构体字面量：
// 商品表的字段顺序（id / code / … / limit / delisted）是个纯排版约定，
// 任何人加一列中间字段都会让按位置的正则当场失效 ——
// 而「失效」的表现是扫不到、于是判定整体挂掉
// （下面那条 `len(delisted) == 0` 的自检就是为这个而留的）。
// 行尾的 `, true},` 则是 delisted 这个字段**本身**的标记，与顺序无关。
var delistedTailRe = regexp.MustCompile(`,\s*true\},?\s*$`)
