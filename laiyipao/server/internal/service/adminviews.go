package service

import (
	"time"

	"github.com/laiyipao/server/internal/domain"
)

// ─────────────────────────────────────────────────────────────────────────────
// 后台 API 的响应视图（2026-10-21 新增）
//
// # 为什么要有这个文件
//
// 此前后台 15 个端点的响应是**手写的 `map[string]any`** —— 形状由 SQL 的
// `SELECT` 列表和 `map[string]any{...}` 字面量共同决定，**没有任何类型参与**。
//
// 后果不是「少写了几行」，而是**前后端两侧都没有事实源**：
//
//  1. 后台前端用 `Record<string, unknown>` 承接服务端 JSON，字段名拼错**不报错**；
//  2. 前端测试的 fixture 按**前端形状**写，于是「列恒空」这类缺陷测出来是绿的；
//  3. 实际发生的：`BattlesView` 详情抽屉 14/16 字段为空（服务端给的是
//     `ReplayInfo`，前端按 `AdminBattle` 读）、`DefensesView` 的「属主」列
//     恒空（前端读 `user_id`，服务端给 `owner_id`）、`EquipmentView` 的
//     宝石「效果」与「说明」两列同绑 `descr`。
//
// # 本文件的角色
//
// 这些 struct 是**后台响应形状的唯一事实源**：
//
//   - service 层返回它们（不再返回 map）；
//   - `cmd/tygen` 用反射把它们**生成**成 TypeScript interface；
//   - `cmd/tygen` 的守卫测试保证生成物与本文件同步（改了不重新生成 → 变红）。
//
// 关键性质：**改一个字段名，两端同时红**。这正是「恒空列」这类缺陷的根治点。
//
// # 约定
//
//   - json tag 就是线上 JSON 的键名，**不许在 handler 里再包一层别的名字**；
//   - 可空字段用指针（`*T`）→ 生成 `T | null`；缺省字段用 `omitempty` → 生成 `?`；
//   - 时间统一 `time.Time`（RFC3339 字符串）；可空时间用 `*time.Time`。
//
// ─────────────────────────────────────────────────────────────────────────────

// playerAuthorityGenerator 说明「玩家侧关卡由生成器产出」这一事实。
//
// 第 23 条产品决策（2026-10-20，路线 b）。它是**机器可读**的：
// 后台界面据此提示「记录在案，不影响线上」，而不是把写入回执显示成「已生效」。
const playerAuthorityGenerator = "generator"

// AdminShopUpdateResp 是 `PUT /admin/shop/:id` 的响应。
//
// Updated 列出**实际写入的列**（不是请求里的键）—— 运营能据此确认
// 「哪些字段真的落库了」。
type AdminShopUpdateResp struct {
	ID      int64    `json:"id"`
	Updated []string `json:"updated"`
}

// qualitiesFromDomain 把 `domain.GemQualities`（匿名 struct 切片）转成具名类型。
//
// 为什么要转：`domain.GemQualities` 的元素类型是**匿名 struct**，
// 反射生成 TS 时拿不到名字，只能内联展开；而 `/config` 用的 `QualityInfo`
// 是具名的。两处形状一致，转一下就让两端共用同一个具名类型。
func qualitiesFromDomain() []QualityInfo {
	out := make([]QualityInfo, 0, len(domain.GemQualities))
	for _, q := range domain.GemQualities {
		out = append(out, QualityInfo{Name: q.Name, Count: q.Count, Mult: q.Mult})
	}
	return out
}

// AdminLevelView 是后台关卡列表 / 单关回读共用的行形状。
//
// ⚠️ `Seed` 是**字符串**下发：seed 是 64 位 LCG 值（如 -7046029255919282421），
// 超过 2^53 后 JS Number 直接失真。第 108 轮起就是字符串，生成器会保持这一点。
type AdminLevelView struct {
	ID         int    `json:"id"`
	Chapter    int    `json:"chapter"`
	Name       string `json:"name"`
	Seed       string `json:"seed"`
	BaseHP     int64  `json:"base_hp"`
	WaveCount  int    `json:"wave_count"`
	Difficulty int    `json:"difficulty"`
	EnergyCost int    `json:"energy_cost"`
	// StarTargets 是三星门槛（各档所需分数）。
	StarTargets []int64 `json:"star_targets"`
	// TerrainConfig 是本关地形排布。
	TerrainConfig []domain.TerrainPlacement `json:"terrain_config"`
	IsBoss        bool                      `json:"is_boss"`
	Enabled       bool                      `json:"enabled"`
	// Attempts / Clears / ClearRate / AvgWave 是**真实战报聚合**，不是关卡配置。
	Attempts  int64   `json:"attempts"`
	Clears    int64   `json:"clears"`
	ClearRate float64 `json:"clear_rate"`
	AvgWave   float64 `json:"avg_wave"`
	// PlayerAuthority 与 AppliedToPlayers 说的是同一件事：**这份配置对玩家生效吗**。
	//
	// 第 23 条产品决策（2026-10-20，路线 b：生成器继续权威）：
	// 后台改的只是 `levels` 表这张**规划记录**，玩家侧关卡由纯函数
	// `domain.GenerateLevel` 产出。两个字段让这件事**机器可读**，
	// 后台界面据此提示「记录在案，不影响线上」，
	// 而不是把写入回执显示成「已生效」。
	//
	// ⚠️ 2026-10-21 起**列表行也带这两个字段**（此前只有单关回读带）——
	// 否则运营在列表页看到改后的血量，却只有点开某一关才能知道它不生效。
	PlayerAuthority  string `json:"player_authority"`
	AppliedToPlayers bool   `json:"applied_to_players"`
}

// AdminLevelWaveView 是单关的波次排布。
type AdminLevelWaveView struct {
	WaveIndex int            `json:"wave_index"`
	Spawns    []domain.Spawn `json:"spawns"`
}

// AdminSkillView 是后台技能行（基础技能与合成技能共用同一形状）。
type AdminSkillView struct {
	ID              int    `json:"id"`
	Code            string `json:"code"`
	Name            string `json:"name"`
	Family          string `json:"family"`
	Element         string `json:"element"`
	Kind            string `json:"kind"`
	Descr           string `json:"descr"`
	BaseDamage      int64  `json:"base_damage"`
	HeatCost        int    `json:"heat_cost"`
	CooldownMs      int    `json:"cooldown_ms"`
	Pierce          int    `json:"pierce"`
	AoeRadius       int    `json:"aoe_radius"`
	ApplyElement    string `json:"apply_element"`
	ApplyStacks     int    `json:"apply_stacks"`
	ProjectileSpeed int    `json:"projectile_speed"`
	Chain           int    `json:"chain"`
	UnlockLevel     int    `json:"unlock_level"`
}

// AdminSkillsResp 是 `GET /admin/skills` 的响应。
type AdminSkillsResp struct {
	Items   []AdminSkillView    `json:"items"`
	Recipes []domain.SeedRecipe `json:"recipes"`
}

// AdminEquipmentView 是后台装备行。
type AdminEquipmentView struct {
	ID           int    `json:"id"`
	Code         string `json:"code"`
	Name         string `json:"name"`
	Slot         string `json:"slot"`
	Tier         int    `json:"tier"`
	Element      string `json:"element"`
	Descr        string `json:"descr"`
	BaseArmor    int    `json:"base_armor"`
	BaseBonusPct int    `json:"base_bonus_pct"`
	UnlockLevel  int    `json:"unlock_level"`
}

// AdminEquipmentResp 是 `GET /admin/equipment` 的响应。
//
// ⚠️ 2026-10-21：此前该端点返回 8 个键，而后台前端的 TS 类型只声明了 3 个
// （equipment / gems / skins）—— **敌人抗性、反应表、专精树、构筑权重全被前端丢掉**。
// 现在八个键都在类型里，前端要么用要么显式忽略。
type AdminEquipmentResp struct {
	Equipment      []AdminEquipmentView   `json:"equipment"`
	Gems           []domain.SeedGem       `json:"gems"`
	Qualifications []QualityInfo          `json:"qualifications"`
	Skins          []domain.SeedSkin      `json:"skins"`
	Enemies        []domain.SeedEnemy     `json:"enemies"`
	Reactions      []domain.ReactionSpec  `json:"reactions"`
	Mastery        []domain.MasteryFamily `json:"mastery"`
	RatingWeights  domain.RatingWeights   `json:"rating_weights"`
}

// AdminDefenseView 是后台防线行。
//
// ⚠️ 2026-10-21 新增 `SnapshotHash`：后台「防线值守」页把
// 「快照哈希是否稳定（防线被偷改）」列为**头号关注点**，
// 而服务端此前**根本不返回**这一列 → 该列恒空、头号关注点不可见。
type AdminDefenseView struct {
	ID        int64  `json:"id"`
	OwnerID   int64  `json:"owner_id"`
	OwnerName string `json:"owner_name"`
	Name      string `json:"name"`
	Power     int64  `json:"power"`
	// ElementCoverage 是 5 元素覆盖数（I-3 搭配质量）。
	ElementCoverage int        `json:"element_coverage"`
	MasteryDone     int        `json:"mastery_done"`
	Wins            int        `json:"wins"`
	Losses          int        `json:"losses"`
	ShieldedUntil   *time.Time `json:"shielded_until"`
	UpdatedAt       time.Time  `json:"updated_at"`
	// SnapshotHash 是防线快照的内容哈希 —— 变了 = 防线被偷改过。
	SnapshotHash string `json:"snapshot_hash"`
}

// AdminEconomyFlowView 是钱包流水的一行（按 reason × currency 聚合）。
type AdminEconomyFlowView struct {
	Reason   string `json:"reason"`
	Currency string `json:"currency"`
	Count    int64  `json:"count"`
	Delta    int64  `json:"delta"`
}

// AdminShopItemView 是商城商品行。
type AdminShopItemView struct {
	ID       int    `json:"id"`
	Code     string `json:"code"`
	Name     string `json:"name"`
	Category string `json:"category"`
	// Price 与 Payload 都是「货币 → 数量」的对象（jsonb）。
	Price   map[string]int `json:"price"`
	Payload map[string]int `json:"payload"`
	// LimitPerDay 是每日限购次数（0 = 不限）。
	LimitPerDay int `json:"limit_per_day"`
	SortOrder   int `json:"sort_order"`
	// Enabled = 上架中。2026-10-21 起后台有了这个开关的入口。
	Enabled bool `json:"enabled"`
}

// AdminEconomyResp 是 `GET /admin/economy` 的响应。
type AdminEconomyResp struct {
	Flows []AdminEconomyFlowView `json:"flows"`
	Shop  []AdminShopItemView    `json:"shop"`
}

// AdminAnnouncementView 是公告行。
type AdminAnnouncementView struct {
	ID          int        `json:"id"`
	Title       string     `json:"title"`
	Body        string     `json:"body"`
	Published   bool       `json:"published"`
	PublishedAt *time.Time `json:"published_at"`
	CreatedAt   *time.Time `json:"created_at"`
}

// AdminRedeemCodeView 是兑换码行。
type AdminRedeemCodeView struct {
	ID        int            `json:"id"`
	Code      string         `json:"code"`
	Reward    map[string]int `json:"reward"`
	MaxUses   int            `json:"max_uses"`
	UsedCount int            `json:"used_count"`
	ExpiresAt *time.Time     `json:"expires_at"`
	Enabled   bool           `json:"enabled"`
}

// AdminAuditLogView 是操作审计的一行。
type AdminAuditLogView struct {
	ID       *int64 `json:"id"`
	AdminID  *int64 `json:"admin_id"`
	Username string `json:"username"`
	Action   string `json:"action"`
	Target   string `json:"target"`
	// Detail 是审计明细（形状随 action 而变）→ 生成 `unknown`。
	Detail    any       `json:"detail"`
	CreatedAt time.Time `json:"created_at"`
}
