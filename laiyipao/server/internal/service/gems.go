// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
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
	"errors"
	"fmt"
	"math/big"

	"github.com/jackc/pgx/v5"

	"github.com/laiyipao/server/internal/domain"
)

// gemInstanceKey 是商城 payload 里表示「造一颗宝石实体」的保留键。
//
// 写成常量而不是裸字符串：它是**跨表契约**（seeder 写、Buy 读），
// 一旦两边拼错，表现是「买了什么都没发生」——
// 而那正是最难排查的一类症状，所以给它个名字并在两边都引用它。
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
//
// ⚠️ `UnscaledGemAffixes` 里的词条**跳过**放大。`element_cap` 是
// 「堆叠到顶就停止累积」的天花板（`domain.MaxElementCap = 8`），
// ×2 只会把总值推过 8 再被下游 clamp 静默吃掉 ——
// 玩家看到「+4 元素上限」，实际一点伤害都没多。
func scaleAffixesByQuality(rows []gemAffixRow, quality string) []gemAffixRow {
	mult := domain.GemQualityMult(quality)
	out := make([]gemAffixRow, 0, len(rows))
	for _, r := range rows {
		if domain.UnscaledGemAffixes[r.Affix] {
			out = append(out, r)
			continue
		}
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
		// ⚠️ 必须带 `AND user_id = $3`，不能只写 `WHERE id = $2`。
		//
		// 当前这条语句看起来是安全的 —— 同一个事务上方刚用
		// `SELECT ... WHERE id = $1 AND user_id = $2 FOR UPDATE` 锁过行，
		// 行锁保证这颗宝石确实属于该玩家。
		//
		// 但那是**调用点**保证的，不是这条语句自己保证的。守卫
		// （ownership_scan_test.go）历史上只扫 SELECT，于是这种
		// 「UPDATE 里没有 user_id」的写法**没有任何东西在看**。
		// 一旦有人给 UPDATE 路径加第二个调用点、或把上面的 FOR UPDATE
		// 挪走，这里就变成一条直接的 IDOR：传别人的 gemID 即可改写
		// 别人的词条，而且返回的是自己视角的宝石列表，看不出异常。
		//
		// 多带一个条件不花任何代价（主键定位已经选中这一行），
		// 却把「不会越权」从调用点的偶然变成语句自己的不变量。
		if _, err := tx.Exec(ctx,
			`UPDATE user_gems SET affixes = $1::jsonb WHERE id = $2 AND user_id = $3`,
			raw, gemID, userID); err != nil {
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

// EquipSlotLimit 是佩戴上限（第 20 轮）。
//
// 战斗消费侧（loadGemsAffixRows → applyLoadoutContribution）把**所有**
// 已佩戴宝石的词条求和 —— 没有上限就是「全戴」：买的每颗宝石都自动生效，
// 「佩戴」退化成「拥有」。6 的具体数值没有公式依据，取的是「部位感」
// 与词条多样性的折中；它是常量而不是配置，因为没有运营调它的场景。
const EquipSlotLimit = 6

// equipGemTx 是 EquipGem / UnequipGem 的事务体（归属 + 行锁共用一份）。
// 返回锁定的这颗宝石的当前 equipped_slot。ErrNotFound = 宝石不存在或不属于该玩家。
func equipGemTx(ctx context.Context, tx pgx.Tx, userID, gemID int64) (string, error) {
	var equipped string
	err := tx.QueryRow(ctx,
		`SELECT equipped_slot FROM user_gems WHERE id = $1 AND user_id = $2 FOR UPDATE`,
		gemID, userID).Scan(&equipped)
	if errors.Is(err, pgx.ErrNoRows) {
		return "", fmt.Errorf("%w: 宝石 %d 不属于该玩家", ErrNotFound, gemID)
	}
	if err != nil {
		return "", fmt.Errorf("lock user_gem: %w", err)
	}
	return equipped, nil
}

// EquipGem 佩戴一颗宝石。
//
// # 这是宝石系统缺失的另一半
//
// `user_gems.equipped_slot` 在本功能之前**全仓零写入** —— 商城卖出的
// `gem_instance` 永远躺在背包里，`loadGemsAffixRows` 的
// `WHERE equipped_slot <> ”` 恒空，宝石词条对**所有玩家**零生效。
// 佩戴写入这个字段后，构筑侧（attacker 派生属性）立刻吃到词条，
// 与洗练共用同一份消费链路，不碰引擎与 replay_hash 的口径。
//
// # 槽位语义与并发
//
//   - 槽位 1..EquipSlotLimit，佩戴时**自动取最小空槽**（玩家只关心
//     「戴几颗」，不关心槽号；equipped_slot 的价值在唯一性约束与排查）。
//
//   - 上限靠**玩家行上的串行化锁**（R102）：事务第一步
//     `SELECT id FROM users WHERE id = $1 FOR UPDATE`，同一玩家的并发佩戴
//     在这里排队。后到者在先到者提交后才开始读已佩戴槽位，计数必然包含
//     先到者 → 超限被拒、槽位不会撞。
//
//     ⚠️ 为什么不能只锁「已佩戴行」：`FOR UPDATE` 锁的是**当前满足条件**
//     的行，而并发请求要佩戴的宝石 `equipped_slot` 还是空串，不在这个集合里
//     —— 它既不被锁，也不在对方语句的快照里（READ COMMITTED 下语句快照
//     在等锁前就定了）。两个请求都读到「已戴 5 颗」，都挑槽 6，各写各的行，
//     最终 7 颗词条生效。R102 注入实测：equipped=7、distinct_slots=6、
//     两条都返回成功。
//
//     这不是「约束能替代」的问题：没有一行计数器可以 CAS，也表达不了
//     「聚合 ≤ 6」；缺陷在锁的**覆盖范围**，所以要锁一个所有写方都先经过、
//     且集合恒定（单行）的点，而不是锁那个会被并发写改变的聚合输入集合。
func (s *Service) EquipGem(ctx context.Context, userID, gemID int64) (*GemView, error) {
	err := s.DB.Tx(ctx, func(tx pgx.Tx) error {
		// R102：同一玩家的佩戴操作串行化在**这一行**上。
		// 必须在读「已佩戴槽位」之前拿：晚一步就回到旧快照，等于没修。
		var lockedID int64
		if err := tx.QueryRow(ctx,
			`SELECT id FROM users WHERE id = $1 FOR UPDATE`, userID).Scan(&lockedID); err != nil {
			if errors.Is(err, pgx.ErrNoRows) {
				return fmt.Errorf("%w: 玩家 %d 不存在", ErrNotFound, userID)
			}
			return fmt.Errorf("lock user for equip: %w", err)
		}

		equipped, err := equipGemTx(ctx, tx, userID, gemID)
		if err != nil {
			return err
		}
		if equipped != "" {
			// 幂等：已佩戴直接返回当前视图，不报错 —— 重复点击不炸。
			return nil
		}
		rows, err := tx.Query(ctx,
			`SELECT equipped_slot FROM user_gems WHERE user_id = $1 AND equipped_slot <> '' FOR UPDATE`,
			userID)
		if err != nil {
			return fmt.Errorf("lock equipped gems: %w", err)
		}
		used := map[string]bool{}
		for rows.Next() {
			var slot string
			if err := rows.Scan(&slot); err != nil {
				rows.Close()
				return fmt.Errorf("scan equipped slots: %w", err)
			}
			used[slot] = true
		}
		rows.Close()
		if err := rows.Err(); err != nil {
			return fmt.Errorf("iterate equipped slots: %w", err)
		}
		if len(used) >= EquipSlotLimit {
			return fmt.Errorf("%w: 最多佩戴 %d 颗宝石", ErrBadInput, EquipSlotLimit)
		}
		slot := ""
		for i := 1; i <= EquipSlotLimit; i++ {
			candidate := fmt.Sprintf("%d", i)
			if !used[candidate] {
				slot = candidate
				break
			}
		}
		if slot == "" {
			// 与上一条互斥（len(used) 已判），走到这里说明槽位计算有 bug —— fail-loud。
			return fmt.Errorf("equip gem: 空槽计算失败（%d 颗已佩戴）", len(used))
		}
		if _, err := tx.Exec(ctx,
			`UPDATE user_gems SET equipped_slot = $1 WHERE id = $2 AND user_id = $3`,
			slot, gemID, userID); err != nil {
			return fmt.Errorf("equip user_gem: %w", err)
		}
		return nil
	})
	if err != nil {
		return nil, err
	}
	return s.gemViewByID(ctx, userID, gemID, "佩戴后")
}

// UnequipGem 卸下一颗宝石（幂等：未佩戴时原样返回）。
func (s *Service) UnequipGem(ctx context.Context, userID, gemID int64) (*GemView, error) {
	err := s.DB.Tx(ctx, func(tx pgx.Tx) error {
		equipped, err := equipGemTx(ctx, tx, userID, gemID)
		if err != nil {
			return err
		}
		if equipped == "" {
			return nil
		}
		if _, err := tx.Exec(ctx,
			`UPDATE user_gems SET equipped_slot = '' WHERE id = $1 AND user_id = $2`,
			gemID, userID); err != nil {
			return fmt.Errorf("unequip user_gem: %w", err)
		}
		return nil
	})
	if err != nil {
		return nil, err
	}
	return s.gemViewByID(ctx, userID, gemID, "卸下后")
}

// gemViewByID 从事务外重读单颗宝石的视图（与 WashGem 的收尾同款：
// 展示字段不必进事务）。
func (s *Service) gemViewByID(ctx context.Context, userID, gemID int64, when string) (*GemView, error) {
	all, err := s.ListGems(ctx, userID)
	if err != nil {
		return nil, err
	}
	for i := range all {
		if all[i].ID == gemID {
			return &all[i], nil
		}
	}
	return nil, fmt.Errorf("%w: 宝石 %d %s不见了", ErrNotFound, gemID, when)
}
