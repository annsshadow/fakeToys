package service

// 宝石实例的 roll / 发放 / 洗练。
//
// # 与战斗引擎的关系
//
// 洗练是**纯数据操作**：`user_gems.affixes` 是一段 JSONB，
// 读它的地方（`loadout_attacker.go` / `build_parts.go`）早已存在。
// 所以这一整条线**不碰引擎、不碰 replay_hash、不影响 I-6** ——
// 宝石一到手，构筑侧立刻生效。
//
// # 为什么 roll 发生在服务端
//
// 词条与品质都进 `attacker` 的派生属性，而那些属性进 `replayHash()` 的 prefix。
// 如果由客户端 roll 再上报，玩家就能自己挑最好的词条 ——
// 那不叫洗练，那叫「许愿」。所以 roll 必须在这里，且用的是 CSPRNG。

import (
	"context"
	"crypto/rand"
	"encoding/json"
	"fmt"
	"math/big"

	"github.com/jackc/pgx/v5"

	"github.com/laiyipao/server/internal/domain"
)

// gemInstanceKey 是商城 payload 里表示「造一颗宝石实体」的保留键。
//
// 写成常量而不是裸字符串：它是**跨表契约**（seeder 写、Buy 读），
// 一旦两边拼错，表现是「买了什么都没发生」——
// 而那正是最难排查的一类症状，所以给��个名字并在两边都引用它。
const gemInstanceKey = "gem_instance"

// gemAffixRow 是 `user_gems.affixes` JSONB 里的一项。
type gemAffixRow struct {
	Affix string `json:"affix"`
	Value int64  `json:"value"`
}

// randBelow 返回 [0, n) 的均匀随机整数，用 crypto/rand。
//
// 为什么不用 math/rand：洗练的结果直接进攻击者派生属性 →
// 直接进 replay_hash 的 prefix。用可预测的种子等于把防伪校验的钥匙发出去。
func randBelow(n int64) (int64, error) {
	if n <= 0 {
		return 0, fmt.Errorf("randBelow: n=%d 必须为正", n)
	}
	v, err := rand.Int(rand.Reader, big.NewInt(n))
	if err != nil {
		return 0, err
	}
	return v.Int64(), nil
}

// rollQuality 按 GemQualityWeight 抽一个品质名。
func rollQuality() (string, error) {
	names := make([]string, 0, len(domain.GemQualityWeight))
	total := 0
	for _, q := range domain.GemQualities {
		w := domain.GemQualityWeight[q.Name]
		if w <= 0 {
			continue
		}
		names = append(names, q.Name)
		total += w
	}
	if total <= 0 {
		return "", fmt.Errorf("品质权重全为 0 —— 无从抽取")
	}
	pick, err := randBelow(int64(total))
	if err != nil {
		return "", err
	}
	acc := 0
	for _, q := range domain.GemQualities {
		acc += domain.GemQualityWeight[q.Name]
		if pick < int64(acc) {
			return q.Name, nil
		}
	}
	return names[len(names)-1], nil
}

// rollGemAffixes 为某颗宝石 roll 出指定数量的词条。
//
// 做法：在该宝石的候选池里**不重复**地抽 n 条，每条在
// [min, max] 上取均匀值，再乘以品质系数。
//
// 不重复是必须的：`affixes` 里出现两条同名同值的词条时，
// `loadout_attacker` 会把两者都加上去 —— 数值上是「白送一条」，
// 而玩家看到的是「这颗宝石有两条一样的词条」。
func rollGemAffixes(gemID int, count int) ([]gemAffixRow, error) {
	pool := make([]domain.SeedGemAffix, 0, 4)
	for _, a := range domain.SeedGemAffixes {
		if a.GemID == gemID {
			pool = append(pool, a)
		}
	}
	if len(pool) < count {
		return nil, fmt.Errorf("宝石 %d 的候选词条只有 %d 条，roll %d 条不够",
			gemID, len(pool), count)
	}

	// 部分洗牌（Fisher-Yates 前 count 步）—— 不必打乱全部，够用且省事。
	idx := make([]int, len(pool))
	for i := range idx {
		idx[i] = i
	}
	for i := 0; i < count; i++ {
		j, err := randBelow(int64(len(idx) - i))
		if err != nil {
			return nil, err
		}
		idx[i], idx[i+int(j)] = idx[i+int(j)], idx[i]
	}

	out := make([]gemAffixRow, 0, count)
	used := map[string]bool{}
	for i := 0; i < count; i++ {
		a := pool[idx[i]]
		if used[a.Affix] {
			// 理论上不会发生（候选池里每个 affix 只写一次），
			// 但内容表是人写的，值得在这里挡一道。
			return nil, fmt.Errorf("宝石 %d 的候选池里 %q 重复", gemID, a.Affix)
		}
		used[a.Affix] = true
		span := int64(a.MaxValue - a.MinValue + 1)
		d, err := randBelow(span)
		if err != nil {
			return nil, err
		}
		out = append(out, gemAffixRow{Affix: a.Affix, Value: int64(a.MinValue) + d})
	}
	return out, nil
}

// scaleAffixesByQuality 把基准值放大到该品质。
//
// 放在 roll 之后单独一步（而不是在 roll 里直接乘）是为了让
// 「基准区间」保持可读、可对照：gem_affixes 里存的是 20–60 这种数，
// 玩家在后台看到金宝石 ×2.0 就能自己推算出 40–120。
func scaleAffixesByQuality(rows []gemAffixRow, quality string) []gemAffixRow {
	mult := domain.GemQualityMult(quality)
	out := make([]gemAffixRow, 0, len(rows))
	for _, r := range rows {
		out = append(out, gemAffixRow{Affix: r.Affix, Value: r.Value * mult / 1000})
	}
	return out
}

// rollGemAffixJSON = roll → 按品质放大 → 序列化成 `user_gems.affixes` 的 JSONB。
//
// 抽出来是因为「铸造」与「洗练」都要走这三步，而这三步的顺序有讲究：
// 先按**基准区间** roll 再整体乘品质系数，而不是每个品质各写一套区间。
// 两处各抄一遍的话，改 affix 结构时漏改一处就会写出两种形状的 JSONB，
// 而症状是「有的宝石洗完显示不出来」—— 极难定位。
func rollGemAffixJSON(gemID int, quality string) ([]byte, error) {
	rows, err := rollGemAffixes(gemID, domain.GemQualityAffixCount(quality))
	if err != nil {
		return nil, err
	}
	raw, err := json.Marshal(scaleAffixesByQuality(rows, quality))
	if err != nil {
		return nil, fmt.Errorf("marshal gem affixes: %w", err)
	}
	return raw, nil
}

// mintGem 在库里造出一颗宝石实例，返回它的行 id。
func (s *Service) mintGem(ctx context.Context, tx pgx.Tx, userID int64) (int64, string, error) {
	// 从 SeedGems 里随机挑一种（内容表 8 种等权）。
	pick, err := randBelow(int64(len(domain.SeedGems)))
	if err != nil {
		return 0, "", err
	}
	gem := domain.SeedGems[pick]

	quality, err := rollQuality()
	if err != nil {
		return 0, "", err
	}
	raw, err := rollGemAffixJSON(gem.ID, quality)
	if err != nil {
		return 0, "", err
	}

	var id int64
	err = tx.QueryRow(ctx,
		`INSERT INTO user_gems (user_id, gem_id, quality, affixes)
		 VALUES ($1,$2,$3,$4::jsonb) RETURNING id`,
		userID, gem.ID, quality, raw).Scan(&id)
	if err != nil {
		return 0, "", fmt.Errorf("insert user_gem: %w", err)
	}
	return id, quality, nil
}

// GemView 是玩家可见的一颗宝石。
type GemView struct {
	ID       int64         `json:"id"`
	GemID    int           `json:"gem_id"`
	Code     string        `json:"code"`
	Name     string        `json:"name"`
	Descr    string        `json:"descr"`
	Quality  string        `json:"quality"`
	Affixes  []gemAffixRow `json:"affixes"`
	Equipped string        `json:"equipped_slot"`
}

// ListGems 列出玩家持有的全部宝石实例。
//
// ⚠️ 这是第 150 轮**新增**的读取面：此前 `user_gems` 只有两处内部读取
// （计入攻击者），玩家在界面上根本看不到自己有哪些宝石。
func (s *Service) ListGems(ctx context.Context, userID int64) ([]GemView, error) {
	rows, err := s.pool.Query(ctx, `
		SELECT ug.id, ug.gem_id, g.code, g.name, g.descr,
		       ug.quality, ug.affixes, ug.equipped_slot
		  FROM user_gems ug
		  JOIN gems g ON g.id = ug.gem_id
		 WHERE ug.user_id = $1
		 ORDER BY ug.id`, userID)
	if err != nil {
		return nil, fmt.Errorf("list gems: %w", err)
	}
	defer rows.Close()

	out := []GemView{}
	for rows.Next() {
		var v GemView
		var raw []byte
		if err := rows.Scan(&v.ID, &v.GemID, &v.Code, &v.Name, &v.Descr,
			&v.Quality, &raw, &v.Equipped); err != nil {
			return nil, fmt.Errorf("scan gem: %w", err)
		}
		// 先给一个空切片再 Unmarshal，是为了让 JSONB 里是 `null` 的那颗
		// 宝石序列化成 `[]` 而不是 `null` —— 客户端 `affixes.map(...)`
		// 会直接在 null 上炸，而它是这颗宝石的数据问题，不该整页崩。
		v.Affixes = []gemAffixRow{}
		if err := json.Unmarshal(raw, &v.Affixes); err != nil {
			// 脏数据**报错**而不是跳过那一颗。
			// 「少显示一颗」玩家看不出来，久了就变成内容谜案；
			// 这里炸掉会在日志里留下 gem id，5 分钟能定位。
			// 正常路径下 affixes 全部由本文件写入，解析失败只可能是
			// 内容表改了 affix 结构 —— 那正是需要立刻发现的事。
			return nil, fmt.Errorf("gem %d 的 affixes 解析失败：%w", v.ID, err)
		}
		out = append(out, v)
	}
	if err := rows.Err(); err != nil {
		return nil, fmt.Errorf("iterate gems: %w", err)
	}
	return out, nil
}

// WashGem 消耗 1 枚洗练券，重新 roll 这颗宝石的词条。
//
// 品质**不变** —— 洗的是词条，不是品质。升品质必须是另一条途径，
// 否则「洗练」这个词就名不副实了。
//
// 扣券与重 roll 在**同一事务**：扣了券却 roll 失败会吞道具，
// 而这是最容易发生的一种失败（内容表被改坏导致候选池不足）。
func (s *Service) WashGem(ctx context.Context, userID, gemID int64) (*GemView, error) {
	err := s.DB.Tx(ctx, func(tx pgx.Tx) error {
		var quality string
		var dbGemID int
		if err := tx.QueryRow(ctx,
			`SELECT gem_id, quality FROM user_gems WHERE id = $1 AND user_id = $2 FOR UPDATE`,
			gemID, userID).Scan(&dbGemID, &quality); err != nil {
			if err == pgx.ErrNoRows {
				return fmt.Errorf("%w: 宝石 %d 不属于该玩家", ErrNotFound, gemID)
			}
			return err
		}

		if err := spendToken(ctx, tx, userID, "gem_wash_token", "gem_wash", gemID); err != nil {
			return err
		}

		raw, err := rollGemAffixJSON(dbGemID, quality)
		if err != nil {
			return err
		}
		if _, err := tx.Exec(ctx,
			`UPDATE user_gems SET affixes = $1::jsonb WHERE id = $2`, raw, gemID); err != nil {
			return fmt.Errorf("update user_gem: %w", err)
		}

		return nil
	})
	if err != nil {
		return nil, err
	}
	// 补上展示字段：名称那些不必进事务，也没理由在事务里多查一次。
	all, err := s.ListGems(ctx, userID)
	if err != nil {
		return nil, err
	}
	for i := range all {
		if all[i].ID == gemID {
			return &all[i], nil
		}
	}
	return nil, fmt.Errorf("%w: 宝石 %d 洗练后不见了", ErrNotFound, gemID)
}
