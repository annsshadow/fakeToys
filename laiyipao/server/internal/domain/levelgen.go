package domain

// 关卡种子化生成器。
//
// 100 关不手写：由"章节参数 + 难度曲线 + levelSeed"程序化产出波次与敌种排布，
// 生成结果落库为可逐关手改的配置行。好处：
//   - 扩关零代码改动
//   - 生成过程是纯函数，可单测、可复现
//   - 难度曲线集中在一处，便于调平衡
//
// ⚠️ 生成必须确定性：同样的章节参数 + seed 必须产出完全相同的结果，
// 否则关卡数据无法在客户端/服务端/后台之间保持一致。

// Chapter 章节定义。
type Chapter struct {
	ID            int
	Name          string
	StartLevel    int
	EndLevel      int
	BaseHP        int64     // 防线血量
	Waves         int       // 波次总数
	EnemyIDs      []int     // 本章可用敌人
	NewElements   []Element // 本章新引入元素
	TerrainKind   string    // 本章主导地形
	BossEnemyID   int       // 章节 BOSS，0 表示无
	MaxElement    int64     // 本章元素层数上限
	ArmorPermille int64     // 本章敌人护甲
}

// TotalLevels 首发关卡总数。
const TotalLevels = 100

var chapters = []Chapter{
	{
		ID: 1, Name: "边境哨站", StartLevel: 1, EndLevel: 20,
		BaseHP: 1000, Waves: 5, NewElements: []Element{ElementFire, ElementIce},
		EnemyIDs: []int{1, 2, 3, 4}, TerrainKind: "oil_drum", MaxElement: 2, ArmorPermille: 0,
	},
	{
		ID: 2, Name: "工业废墟", StartLevel: 21, EndLevel: 40,
		BaseHP: 1600, Waves: 6, NewElements: []Element{ElementLightning, ElementKinetic},
		EnemyIDs: []int{1, 2, 3, 5, 6, 7, 8, 9, 10, 11}, TerrainKind: "collapse_wall", MaxElement: 3, ArmorPermille: 50,
	},
	{
		ID: 3, Name: "冻原", StartLevel: 41, EndLevel: 60,
		BaseHP: 2600, Waves: 7, NewElements: []Element{ElementCorrosion},
		EnemyIDs:    []int{1, 2, 3, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15},
		TerrainKind: "charge_tower", MaxElement: 4, ArmorPermille: 100, BossEnemyID: 16,
	},
	{
		ID: 4, Name: "感染矿井", StartLevel: 61, EndLevel: 80,
		BaseHP: 4200, Waves: 8, NewElements: nil,
		EnemyIDs:    []int{1, 2, 3, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 17, 18},
		TerrainKind: "tidal_gate", MaxElement: 5, ArmorPermille: 150,
	},
	{
		ID: 5, Name: "母巢外围", StartLevel: 81, EndLevel: 95,
		BaseHP: 6800, Waves: 9, NewElements: nil,
		EnemyIDs:    []int{1, 2, 3, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 17, 18, 19, 20},
		TerrainKind: "rotor_vane", MaxElement: 6, ArmorPermille: 200, BossEnemyID: 19,
	},
	{
		ID: 6, Name: "母巢核心", StartLevel: 96, EndLevel: 100,
		BaseHP: 10000, Waves: 10, NewElements: nil,
		EnemyIDs:    []int{1, 2, 3, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 17, 18, 19, 20, 21},
		TerrainKind: "charge_tower", MaxElement: 8, ArmorPermille: 250, BossEnemyID: 22,
	},
}

// AllChapters 返回全部章节定义。
func AllChapters() []Chapter {
	out := make([]Chapter, len(chapters))
	copy(out, chapters)
	return out
}

// ChapterOf 返回关卡所属章节。
func ChapterOf(levelID int) Chapter {
	for _, c := range chapters {
		if levelID >= c.StartLevel && levelID <= c.EndLevel {
			return c
		}
	}
	return chapters[len(chapters)-1]
}

// Spawn 是一条刷怪指令。
type Spawn struct {
	EnemyID  int `json:"enemy_id"`
	Count    int `json:"count"`
	Interval int `json:"interval"` // 毫秒，组内每只的间隔
	Delay    int `json:"delay"`    // 毫秒，本组相对波次开始的延迟
}

// Wave 是一个波次的全部刷怪指令。
type Wave struct {
	Index  int     `json:"wave_index"`
	Spawns []Spawn `json:"spawns"`
}

// TerrainPlacement 是关卡内一件可交互地形的位置与参数。
type TerrainPlacement struct {
	Kind  string `json:"kind"` // oil_drum / tidal_gate / rotor_vane / collapse_wall / charge_tower
	X     int    `json:"x"`    // 逻辑坐标 0..1000
	Y     int    `json:"y"`
	Param int    `json:"param"` // 语义随 Kind 变化，见 GAME_DESIGN I-4
}

// GeneratedLevel 是生成器的输出。
type GeneratedLevel struct {
	ID              int                `json:"id"`
	Chapter         int                `json:"chapter"`
	Name            string             `json:"name"`
	Seed            int64              `json:"seed"`
	BaseHP          int64              `json:"base_hp"`
	WaveCount       int                `json:"wave_count"`
	Difficulty      int                `json:"difficulty"` // 千分比
	EnergyCost      int                `json:"energy_cost"`
	StarTargets     []int64            `json:"star_targets"`
	Terrain         []TerrainPlacement `json:"terrain"`
	IsBoss          bool               `json:"is_boss"`
	Waves           []Wave             `json:"waves"`
	ElementCap      int64              `json:"element_cap"`
	ArmorPermille   int64              `json:"armor_permille"`
	MaxReactionTier int64              `json:"max_reaction_tier"`
}

// StarTarget 三档星星门槛：[达到波次, 达到分数]
// StarTargetRatio 是一星/二星/三星相对「理论满分」的占比（千分比）。
//
// ⚠️ 曾经的取值是 [1000, 1200, 1400]，与 permille=1000 配合后意味着：
//
//	1 星 = 100% 满分（要一只不漏、一下不空）
//	2 星 = 120% 满分 ← **要求超过理论上限，数学上不可达**
//	3 星 = 140% 满分 ← 同上
//
// 也就是说两星以上永远拿不到，KeysOnThreeStar 是死配置。
//
// 现在按「漏怪率」反推：分数基本正比于清怪比例（击杀分是大头），
// 于是 35% 满分 ≈ 漏 65% 怪，75% 满分 ≈ 漏 25% 怪 ——
// 对一个体力有限、热量需要规划的关卡，这是「认真打」与「打干净」的分界。
var StarTargetRatio = [3]int64{350, 550, 750}

// chapterTerrainFor 返回该关是否使用地形，以及地形种类。
// 约 25% 的关卡是地形关，每章至少 3 关（GAME_DESIGN I-4）。
func chapterTerrainFor(ch Chapter, levelSeed int64) (string, bool) {
	// 用关卡 seed 的低 8 位决定，保证确定性
	switch int(levelSeed&0xFF) % 4 {
	case 0:
		return ch.TerrainKind, true
	case 1:
		// 每章保底 3 关地形关：取本关在本章内的偏移
		offset := (levelSeed >> 8) % int64(ch.EndLevel-ch.StartLevel+1)
		if offset < 3 {
			return ch.TerrainKind, true
		}
		return "", false
	default:
		return "", false
	}
}

// GenerateLevel 生成单关配置。纯函数：同输入必同输出。
func GenerateLevel(levelID int) GeneratedLevel {
	ch := ChapterOf(levelID)

	// 关卡 seed 由全局常量与关卡号派生，保证跨机器一致。
	// 常量 0x9E3779B97F4A7C15 超过 int64 上限，用 uint64 运算后再折回 int64。
	const phi uint64 = 0x9E3779B97F4A7C15
	levelSeed := int64(uint64(0x5CA1AB1E) ^ (uint64(levelID) * phi))
	rng := newLCG(levelSeed)

	// 章内进度 0..1
	span := ch.EndLevel - ch.StartLevel + 1
	progress := float64(levelID-ch.StartLevel) / float64(span)

	// 全局进度 0..1（跨 100 关）。
	// 难度曲线必须基于全局进度，否则每章开头都会出现难度断崖式下跌
	// （第 20 关 2119 → 第 21 关 350），玩家体验上是"越玩越简单再重新爬"。
	globalProgress := float64(levelID-1) / float64(TotalLevels-1)
	difficulty := int64(1000 + 3200*globalProgress)

	baseHP := ch.BaseHP + int64(progress*float64(ch.BaseHP)*0.5)
	waveCount := ch.Waves + int(progress*3)

	isBoss := ch.BossEnemyID != 0 && (levelID == ch.EndLevel || levelID == ch.EndLevel-4)

	gl := GeneratedLevel{
		ID:              levelID,
		Chapter:         ch.ID,
		Name:            levelName(ch, levelID),
		Seed:            levelSeed,
		BaseHP:          baseHP,
		WaveCount:       waveCount,
		Difficulty:      int(difficulty),
		EnergyCost:      5 + ch.ID,
		ElementCap:      ch.MaxElement,
		ArmorPermille:   ch.ArmorPermille,
		MaxReactionTier: int64(1 + int(progress*2)),
		IsBoss:          isBoss,
	}

	// 理论满分：用于星级门槛与掉落上限（服务端防作弊的基准）
	maxScore := int64(0)
	for w := 0; w < waveCount; w++ {
		wave := generateWave(rng, ch, w, waveCount, levelID, isBoss)
		gl.Waves = append(gl.Waves, wave)
		for _, sp := range wave.Spawns {
			// ⚠️ 单只敌人的分数贡献必须按**引擎的实际给分口径**算，
			// 曾经这里写死 `sp.Count * 2000`（自称"粗略上界"），
			// 而引擎只给两笔：命中时 totalDamage/100、击杀时 500/5000。
			// 一只 hp=100 的普通怪最多拿 500(击杀) + 1(伤害) = 501 分，
			// 门槛却按 2000 算 —— 于是第 1 关：
			//   引擎理论上限 19542，1 星门槛 78000，差 58458。
			// 后果是所有关卡恒 0 星、KeysOnThreeStar 这把钥匙**没有任何合法获取路径**。
			// 更糟的是这类偏差在两端"各自自洽"（Go 只管生成、TS 只管加分），
			// 13 个公式一致性向量全都通过，没有任何测试能发现。
			//
			// 口径常量集中在这里，与 engine.ts 的两处 `this.score +=` 一一对应。
			// 改引擎给分必须同步改这里（由 TestStarTargetsReachable 守住）。
			maxScore += int64(sp.Count) * enemyScoreUpperBound(sp.EnemyID)
		}
	}
	gl.StarTargets = []int64{
		starThreshold(maxScore, StarTargetRatio[0]),
		starThreshold(maxScore, StarTargetRatio[1]),
		starThreshold(maxScore, StarTargetRatio[2]),
	}

	// 地形。必须始终是空数组而非 nil ——
	// json.Marshal(nil slice) 会产出 JSON null，客户端遍历时会炸。
	gl.Terrain = []TerrainPlacement{}
	if kind, ok := chapterTerrainFor(ch, levelSeed); ok {
		gl.Terrain = generateTerrain(rng, kind, levelID)
	}

	return gl
}

// TotalEnemies 返回该关敌人总数，防作弊用来校验 kills 上限。
func (g GeneratedLevel) TotalEnemies() int {
	total := 0
	for _, w := range g.Waves {
		for _, s := range w.Spawns {
			total += s.Count
		}
	}
	return total
}

// generateWave 生成一个波次。波次越靠后，怪越多、越硬、间隔越短。
func generateWave(rng *LCG, ch Chapter, index, total int, levelID int, isBoss bool) Wave {
	progress := float64(index) / float64(maxInt(1, total-1))
	w := Wave{Index: index}

	// 敌人种类数随波次解锁
	poolSize := 1 + int(progress*float64(len(ch.EnemyIDs)-1))
	if poolSize > len(ch.EnemyIDs) {
		poolSize = len(ch.EnemyIDs)
	}

	groups := 1 + int(progress*2.2) // 每波 1~3 组
	for g := 0; g < groups; g++ {
		enemyID := ch.EnemyIDs[rng.Intn(poolSize)]
		// 数量随波次增长
		count := 2 + rng.Intn(3) + int(progress*4)
		if g > 0 {
			count = 1 + rng.Intn(2) + int(progress*2)
		}
		w.Spawns = append(w.Spawns, Spawn{
			EnemyID:  enemyID,
			Count:    count,
			Interval: 900 - int(progress*500),
			Delay:    rng.Intn(1500),
		})
	}

	// BOSS 关的最后一波放 BOSS
	if isBoss && index == total-1 {
		w.Spawns = append(w.Spawns, Spawn{
			EnemyID:  ch.BossEnemyID,
			Count:    1,
			Interval: 0,
			Delay:    0,
		})
	}
	return w
}

// generateTerrain 生成 1~3 件地形。坐标用 0..1000 逻辑空间，
// 客户端按实际画布尺寸线性映射，避免服务端依赖具体分辨率。
func generateTerrain(rng *LCG, kind string, levelID int) []TerrainPlacement {
	count := 1 + rng.Intn(3)
	out := make([]TerrainPlacement, 0, count)
	usedX := map[int]bool{}
	for i := 0; i < count; i++ {
		x := 250 + rng.Intn(650)
		for usedX[x] { // 简单去重，避免地形叠在一起
			x = 250 + rng.Intn(650)
		}
		usedX[x] = true
		// param 的单位必须与「充能的增量」同量级，否则地形永远不触发。
		//
		// ⚠️ 这里曾经差 2~3 个数量级：
		//   油桶 param = 100~180，而每次火焰命中的增量是 totalDamage/100。
		//   血量缩放前单次总伤害约 50（增量 0，因为整除 100 后为 0），
		//   缩放后约 254（增量 2）。于是 100 的阈值要 50 次同元素命中 ——
		//   实测 12 个油桶关卡里 0 个被点燃。
		//   掩体 param = 2000~5000，要 1000~2500 次动能命中，永远不可能。
		//
		// 现在按「一场战斗里合理能打出的次数」标定：
		// 实测第 1 关 270 发 / 约 220 次命中，按元素五分约为每种 44 次，
		// 每次增量 2 → 满打约 88 点。所以阈值取 20~70（4~35 次命中），
		// 留出"必须持续关注某个元素"的空间，又不至于打不出来。
		param := 0
		switch kind {
		case "oil_drum":
			param = 20 + rng.Intn(50) // 引爆所需充能（约 10~35 次火焰命中）
		case "tidal_gate":
			param = 8000 + rng.Intn(4000) // 周期毫秒
		case "rotor_vane":
			param = 30 + rng.Intn(60) // 角速度（度/秒）
		case "collapse_wall":
			param = 30 + rng.Intn(40) // 崩塌所需动能充能（约 15~35 次动能命中）
		case "charge_tower":
			param = 30 + rng.Intn(20) // 蓄满所需击杀数
		}
		out = append(out, TerrainPlacement{
			Kind:  kind,
			X:     x,
			Y:     380 + rng.Intn(280),
			Param: param,
		})
	}
	return out
}

func levelName(ch Chapter, levelID int) string {
	return ch.Name + " · " + itoa(levelID)
}

func starThreshold(maxScore, ratio int64) int64 {
	return maxScore * ratio / permille
}

// 引擎给分口径 —— 必须与 miniapp/src/game/engine.ts 逐字对应。
//
// engine.ts 只有两处加分：
//
//	L759  this.score += Number(res.totalDamage / 100n)   每命中一次
//	L811  this.score += e.isBoss ? 5000 : 500             每击杀一只
//
// 之所以要显式命名成常量：levelgen 的星级门槛与引擎的加分逻辑分属两端，
// 之前唯一的联系是注释里一句"粗略上界"。注释不是契约。
const (
	// scorePerDamageUnit 是「每 100 点伤害记 1 分」里的那个 100。
	scorePerDamageUnit = 100
	// scoreOnKillNormal / scoreOnKillBoss 是击杀分。
	scoreOnKillNormal = 500
	scoreOnKillBoss   = 5000
)

// enemyScoreUpperBound 返回单只敌人能贡献的**理论最高分**。
//
// 击杀分一定拿得到；伤害分是上界 —— 真实对局里
// 一部分伤害会被护甲减免、会被护盾吸收、也打不满血，
// 所以这是「理论满分」而不是「典型得分」，这正是星级门槛该用的量。
//
// 未在敌人表里找到 id 时返回一个保守下界而不是 0：
// 返回 0 会让门槛塌到 0 分（三星白送），宁可偏保守。
func enemyScoreUpperBound(enemyID int) int64 {
	for _, e := range SeedEnemies {
		if e.ID != enemyID {
			continue
		}
		kill := int64(scoreOnKillNormal)
		if e.IsBoss {
			kill = scoreOnKillBoss
		}
		return kill + (e.HP+e.ShieldHP)/scorePerDamageUnit
	}
	return scoreOnKillNormal
}

// LCG 是线性同余伪随机数发生器。
// 选用 LC 而非 mulberry32 是因为 Go 与 TS 都能用完全相同的整数运算实现，
// 从而保证两端生成完全相同的关卡（I-6 确定性的基础）。
type LCG struct {
	state uint64
}

func newLCG(seed int64) *LCG {
	return &LCG{state: uint64(seed) ^ 0x2545F4914F6CDD1D}
}

// Next 返回下一个伪随机数（无符号 64 位）。
func (l *LCG) Next() uint64 {
	// PCG-XSH-RR 的简化版：与 TS 侧 lcg.ts 保持一致
	old := l.state
	l.state = old*6364136223846793005 + 1442695040888963407
	xorshifted := uint64(uint32(((old >> 18) ^ old) >> 27))
	rot := uint64(old >> 59)
	return (xorshifted >> rot) | (xorshifted << ((-rot) & 63))
}

// Intn 返回 [0, n) 内的整数。n <= 0 时返回 0。
func (l *LCG) Intn(n int) int {
	if n <= 0 {
		return 0
	}
	return int(l.Next() % uint64(n))
}

// GenerateAllLevels 生成全部 100 关。
func GenerateAllLevels() []GeneratedLevel {
	out := make([]GeneratedLevel, 0, TotalLevels)
	for id := 1; id <= TotalLevels; id++ {
		out = append(out, GenerateLevel(id))
	}
	return out
}

// --- 内部工具 ---

func maxInt(a, b int) int {
	if a > b {
		return a
	}
	return b
}

func itoa(n int) string {
	if n == 0 {
		return "0"
	}
	neg := n < 0
	if neg {
		n = -n
	}
	var buf [20]byte
	i := len(buf)
	for n > 0 {
		i--
		buf[i] = byte('0' + n%10)
		n /= 10
	}
	if neg {
		i--
		buf[i] = '-'
	}
	return string(buf[i:])
}
