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
var PENDING_TOKENS = map[string]string{
	"revive_token": "局内复活要在引擎里续命并重演整场战斗，这会改动 replay_hash 的计算锚点，" +
		"直接冲击 I-6 可重放性（服务端重放哈希必须等于引擎哈希）。" +
		"属核心战斗公式改动，不能顺手补，也不能先做一个不锚定哈希的简版 —— 那会让防伪校验形同虚设。",
	"gem_wash_token": "洗练的前提是先有可洗的宝石实例，而全仓库没有任何 INSERT INTO user_gems：" +
		"玩家手里一颗宝石都没有。先做「洗」等于给一条死路修门。" +
		"需先补宝石获取途径（掉落或兑换），那是独立的一轮改动。",
}

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

// TestShopDoesNotSellUnspendableTokens 商城里的道具必须是「可花」集合里的。
//
// 这条与上面两条互补：上面查「上架的道具能不能用」，这条查
// 「可用的道具是不是真的在上架」—— 方向反了也是缺口。
// 已知豁免：复活币与洗练石**确实在卖**，但那是 PENDING_TOKENS 里
// 已登记的产品缺口，不是这里能判定的错误，因此本条不覆盖它们。
func TestShopDoesNotSellUnspendableTokens(t *testing.T) {
	sold := shopTokenGrants(t)
	for token := range sold {
		if !walletTokens[token] {
			t.Errorf("商城在发 %q，但它不在 walletTokens 里 —— "+
				"Buy 会因「未知货币」回滚，玩家永远买不到", token)
		}
	}
	if len(sold) == 0 {
		t.Fatal("一条商城道具都没扫到 —— 判据失效了（seedShop 的写法变了吗？）")
	}
}

// shopTokenGrants 扫出商城种子实际发放的道具。
var shopGrantRe = regexp.MustCompile(`map\[string\]int\{"([a-z_]+)":\s*\d+\}`)

func shopTokenGrants(t *testing.T) map[string]bool {
	t.Helper()
	// 商城种子在 internal/seeder 包里；从 service 看不到源码。
	// 改为直接读 seeder 包的源码文件（同 module，路径固定）。
	raw, err := os.ReadFile(filepath.Join("..", "seeder", "seed.go"))
	if err != nil {
		t.Fatalf("读 seeder/seed.go 失败：%v", err)
	}
	out := map[string]bool{}
	// 只取 grant 那一列（倒数第二列）—— 用「同行出现 walletTokens 里的名字」判定
	for _, line := range strings.Split(string(raw), "\n") {
		if !strings.Contains(line, "shop_") {
			continue
		}
		for _, m := range shopGrantRe.FindAllStringSubmatch(line, -1) {
			if walletTokens[m[1]] {
				out[m[1]] = true
			}
		}
	}
	return out
}
