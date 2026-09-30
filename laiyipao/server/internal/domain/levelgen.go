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
	// MaxScore 是「理论满分」，即 star_targets 所依据的那个估算值（1000‰）。
	//
	// 必须单独下发，因为结算裁剪的上限要锚定在**理论满分**上，
	// 而不是 3 星门槛（那只有理论满分的 75%，会让每一次干净通关都被裁剪）。
	// 详见 GenerateLevel 里赋值处的注释。
	MaxScore int64 `json:"max_score"`
}

// ScoreRules 是分数规则。**两端共用同一份数值。**
//
// ⚠️ 这些常量曾经只有 Go 这一份定义，而客户端引擎把同样的数字
// **硬编码了 5 处**（engine.ts 的加分、记事件、地形充能），
// 测试里还��� 2 处。漂移的后果很隐蔽：
//
//	Go 侧用 scorePerDamageUnit 算 star_targets
//	TS 侧用自己那份 /100 累加实际得分
//	两者一旦不一致 → 玩家拿到的星级与实际表现不符
//	而**没有任何测试会发现**，因为两端各自都"自洽"。
//
// 这与 README 工程约束 4（同名不同义）同类：
// 契约不是靠约定维持的，是靠**一个可下发的结构**维持的。
//
// 客户端从 /config 的 score_rules 读这份数据，
// 测试则从夹具 smoke_levels.json 读（由 cmd/vectors 导出）。
type ScoreRules struct {
	// PerDamageUnit 是「每多少伤害算 1 分」。
	PerDamageUnit int64 `json:"per_damage_unit"`
	// OnKillNormal / OnKillBoss 是击杀分。
	OnKillNormal int64 `json:"on_kill_normal"`
	OnKillBoss   int64 `json:"on_kill_boss"`
	// StarTargetRatio 是一/二/三星相对理论满分的占比（千分比）。
	StarTargetRatio [3]int64 `json:"star_target_ratio"`
	// ScoreFullAtSec 是「打满理论满分所需的最短秒数」，速率裁剪的锚点。
	ScoreFullAtSec int64 `json:"score_full_at_sec"`
}

// DefaultScoreRules 返回当前生效的分数规则。
//
// 这里是**唯一定义处**。levelgen 的估算、结算的裁剪、
// cmd/vectors 的契约导出、/config 的下发全部走它，
// 任何人想改数值都必须先改这里。
func DefaultScoreRules() ScoreRules {
	return ScoreRules{
		PerDamageUnit:   scorePerDamageUnit,
		OnKillNormal:    scoreOnKillNormal,
		OnKillBoss:      scoreOnKillBoss,
		StarTargetRatio: StarTargetRatio,
		ScoreFullAtSec:  ScoreFullAtSec,
	}
}

// StarTarget 三档星星门槛：[达到波次, 达到分数]
// StarTargetRatio 是一星/二星/三星相对「理论满分」的占比（千分比）。
//
// ⚠️ 曾经的取值有两个版本，都不对：
//
//	[1000, 1200, 1400] —— 二星以上要求超过理论上限，**数学上不可达**
//	[350, 550, 750]    —— 远低于任何一次干净通关的分数，**3 星白送**
//
// 后者尤其隐蔽：实测默认构筑在 **99/100 关**都能拿 3 星，
// 于是"KeysOnThreeStar"成了摆设。
//
// ## 现在按「漏怪率」标定
//
// 分数 = 击杀分（漏怪就拿不到）+ 伤害分（敌人总会死，≈ 固定）
//
// 关键认识：**伤害分不是可控量**。敌人无论如何都会死，
// 累计伤害 ≈ 敌人总血量 + 过杀，与"打得快不快、好不好"无关。
// 所以提高伤害分权重**不会产生任何梯度**，只是换一个数字 ——
// 我最初打算那么做，查清这一点后放弃了。
//
// 真正可控的量只有**漏怪率**。设漏怪率为 L，则
//
//	得分比 ≈ (击杀分×(1-L) + 伤害分) / (击杀分 + 伤害分)
//
// 代入击杀 500/只、伤害 ≈ 8/只（血量 ×8 后 hp/100）：
//
//	[600] → L ≤ 39%   「过关就行」
//	[850] → L ≤ 15%   「打得干净」
//	[980] → L ≤ 2%    「零漏怪」
//
// 这三档正好是玩家能理解的三个语义，且与关卡无关、可自动换算。
var StarTargetRatio = [3]int64{600, 850, 980}

// TerrainFloorPerChapter 是每章**必定**是地形关的关数（GAME_DESIGN I-4）。
const TerrainFloorPerChapter = 3

// chapterTerrainFor 返回该关是否使用地形，以及地形种类。
//
// 两部分，缺一不可：
//  1. **保底**：每章前 3 关必定是地形关。
//  2. **概率**：其余关卡里约 1/8 也是地形关。
//
// ⚠️ 原规格「约 25% + 每章保底 3 关」是**自相矛盾**的：
// 6 章 × 3 关的保底本身就占 18%，而 18% + 82%×25% = 38.5%，不是 25%。
// 现在按"保底 18% + 概率 12.5%"调和到约 28%，
// 并把 GAME_DESIGN 的表述同步为这个自洽的数字。
//
// ⚠️ 保底曾经写成"概率"，因此根本不是保底：
//
//	switch int(seed&0xFF) % 4 {
//	case 0: return 地形          // 25% 基础分布
//	case 1: if offset < 3 {...}   // 「保底 3 关」
//	}
//
// 保底只在**恰好 1/4 的关卡**里生效，再乘以"本章前 3 关"的条件，
// 期望值是 `3/章长 × 1/4`，对 20 关的章约 0.15 关。
// 第 6 章只有 5 关，期望 0.375 关 —— **数学上就不可能保底 3 关**。
// 实测第 6 章只有 2 关有地形，低于声称的保底。
//
// 而函数名、注释和 GAME_DESIGN 都写着「每章至少 3 关」——
// 一个不成立的承诺比没有承诺更糟：设计文档会被当成已实现的规格。
//
// 现在保底是**结构性的**：前 3 关无条件返回地形关，与散列无关。
func chapterTerrainFor(ch Chapter, levelID int, levelSeed int64) (string, bool) {
	span := ch.EndLevel - ch.StartLevel + 1
	// 1) 保底：每章前 3 关（章长不足 3 则全章）
	offset := int64(levelID - ch.StartLevel)
	if offset < TerrainFloorPerChapter {
		return ch.TerrainKind, true
	}
	if offset < 0 || offset >= int64(span) {
		// 越界说明 levelID 与 ch 不匹配（调用方的 bug）。
		// 这里保守返回无地形，而不是让越界 offset 走进下面的判定。
		return "", false
	}
	// 2) 概率：其余关卡里约 1/8
	if uint32(terrainRoll(levelID))%8 != 0 {
		return "", false
	}
	return ch.TerrainKind, true
}

// terrainRoll 是「这一关要不要地形」的稳定散列。
//
// ⚠️ 这里**必须自己散列 levelID**，不能直接取关卡 seed 的某几位。
//
// 原实现用 `int(seed&0xFF) % 4`（最低 8 位）与 `(seed>>8) % 4`，
// 两者都在**乘法型哈希的弱位**上。实测章内分布（判据是"每章内部"
// 的均匀度，因为地形比例是按章体验的，不是全局体验的）：
//
//	>>0  章内最大偏离 = 1   ch5(4/4/3/4)     尚可
//	>>8  章内最大偏离 = 8   ch4(0/7/13/0)   ← 灾难
//	>>16 章内最大偏离 = 1   ch3(6/5/4/5)     可用
//	>>32 章内最大偏离 = 1   ch1(5/6/5/4)     可用
//
// `>>8` 那一行是关键：第 4 章 20 关的取值是 0/7/13/0，
// 意味着 `>>8 % 4` 在**整章内是同一个值**（只在章边界附近才变）。
// 后果不是"比例偏差 8%"，而是**整章要么全是地形关、要么一个都没有**：
//
//	第 5 章：86~95 关的 (seed>>8)%4 全是 0 → 13/15 关都有地形（87%）
//	第 1 章：整章都是 3     → 除了保底 3 关，其余 17 关一个都没有
//
// 这就是"每章地形关数量忽多忽少"的真正原因 —— 不是概率不够均匀，
// 而是**在用的那几个比特位根本不随关卡号变化**。
//
// 现在改成对 levelID 做一次完整的雪崩（murmur3 的 fmix32 收尾）。
// 取 levelID 而不是 seed 还有第二个好处：这个判定与种子生成方式解耦，
// 将来换 PRNG 不会悄悄改掉 100 关的地形分布。
//
// ⚠️ 诚实说明雪崩步的**实际作用范围**：
// 我试过把雪崩全部去掉（只留 `levelID * 0x9E3779B1`），
// `TestTerrainRollIsUniformWithinChapter` **依然全绿** ——
// 因为「连续整数 × 奇数 再 mod 8」本来就有良好分布
// （奇数与 8 互质，levelID 的低位本就循环覆盖 0..7）。
//
// 所以雪崩**不是**当前 mod-8 判定的关键，它是防御性的：
//   - 若将来模数从 8 改成 10/100（非 2 的幂），弱位就会暴露
//   - 若入参从 levelID 换成稀疏的 levelID（比如 ×7 后的关卡号），
//     只有完整雪崩能保证打散
//
// 保留它的成本是 4 行，收益是"换参数时不会静默退化"。
func terrainRoll(levelID int) uint32 {
	h := uint32(uint64(uint32(levelID)) * 0x9E3779B1)
	h ^= h >> 16
	h *= 0x85EBCA6B
	h ^= h >> 13
	h *= 0xC2B2AE35
	h ^= h >> 16
	return h
}

// chapterOffset 返回关卡在本章内的偏移，值域 [0, span)。
//
// ⚠️ 这里**必须用无符号右移**。
//
// 曾经的写法是 `(levelSeed >> 8) % span`，看起来是标准的"哈希取模"，
// 但 Go 的 `>>` 对有符号整数是**算术右移**（保留符号位），不是逻辑右移：
//
//	seed = -7046029255919282421
//	seed >> 8  →  仍是负数
//	% span     →  **负余数**（Go 的 % 向零截断，-5 % 4 = -1）
//
// 而当时的判据是 `offset < 3`（保底 3 关）——
// **任何负数都满足这个判据**，于是负种子那一半的关卡
// 100% 变成地形关，而不是预期的 3/章长。
//
// 100 关的种子里**恰好 50 个是负数**（PHI 混合哈希均匀分布在 int64 全域），
// 所以实测 100 关里 41 关带地形（41%），而设计意图约 30%。
//
// 之所以能活这么久：没有任何测试断言"100 关里有几关带地形"，
// 而冒烟测试只覆盖 6 关、其中只有 2 关恰好有地形。
//
// `uint64(seed) >> 8` 是逻辑右移，结果必然落在 [0, 2^56)，
// 再 `% span` 必然落在 [0, span)。由 TestChapterOffsetIsUnsigned 守住。
func chapterOffset(levelSeed int64, span int64) int64 {
	if span <= 0 {
		return 0
	}
	return int64(uint64(levelSeed)>>8) % span
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
	// 理论满分必须**单独下发**。
	//
	// ⚠️ 曾经的裁剪上限（scoreCapFor）取的是 fullStarTarget() = StarTargets[2]，
	// 也就是 3 星门槛 = 理论满分的 980‰… 不，是 750‰。
	// 于是上限本身只有理论满分的 75%，而实测默认构筑的正常分数
	// 是理论满分的 130% 以上 —— **每一次干净通关都会被裁剪**：
	//
	//   关 1  实测 19842  裁剪上限 14656  → 被裁到 14656
	//   关 10 实测 22306  裁剪上限 16520  → 被裁
	//   关 25 实测 20589  裁剪上限 15055  → 被裁
	//   关 50 实测 25300  裁剪上限 18446  → 被裁
	//   关 75 实测 31589  裁剪上限 22965  → 被裁
	//   关 100 实测 46085 裁剪上限 35412  → 被裁
	//
	// 后果有两层，都很糟：
	//  1. 打得好的玩家分数被砍，且客户端不知道（本地显示 19842，结算后变 14656）
	//  2. `Clamped` 在**几乎每一次正常胜利**上都为 true，
	//     于是"被裁剪"这个本该用于发现伪造的信号变成了噪声 ——
	//     监控里 90% 的结算都"异常"，真正的伪造反而淹没在里面。
	//
	// 裁剪的锚点必须是**理论满分**（1000‰），不是 3 星门槛。
	// 它的职责是"挡住不可能的分数"，不是"压住优秀玩家的分数"。
	gl.MaxScore = maxScore

	// 地形。必须始终是空数组而非 nil ——
	// json.Marshal(nil slice) 会产出 JSON null，客户端遍历时会炸。
	gl.Terrain = []TerrainPlacement{}
	if kind, ok := chapterTerrainFor(ch, levelID, levelSeed); ok {
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
			// 实测（balance.probe 的「地形生效率」，按真实效果度量）：
			// 6 个油桶的充能峰值是 11~32，而 param 是 22~68 ——
			// **够得着了但累积不够**，没有一个达到阈值。
			//
			// 这是把地形 y 范围从 [380,660] 改到 [600,900]
			// （对齐敌人走廊 [600,940]）之后的结果：改之前充能恒为 0，
			// 改之后能到 param 的 25%~47%。
			//
			// 阈值取 8~23。实测峰值充能 11~32，取这个区间让
			// 「对着油桶集中火力」能点燃而「随手打」不易点燃 ——
			// 保留战术意图，同时保证机制**可用**而不是完全看不到。
			param = 8 + rng.Intn(15)
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
			Kind: kind,
			X:    x,
			// ⚠️ y 必须落在**敌人行走走廊**里，否则地形是死代码。
			//
			// 曾经是 `380 + rng.Intn(280)` → y ∈ [380, 660]，
			// 而敌人出生在 `600 + rng.Intn(340)` → y ∈ [600, 940]（engine.ts）。
			// 两个区间只在 [600, 660] 重叠 —— 约 16%。
			//
			// 后果实测：6 个油桶**一个都没被点燃过**（0%）。
			// 因为火焰弹丸是瞄着**敌人**飞的，敌人�� y≈800 时弹道在 y 800~880，
			// 而油桶在 y≈500，距离 300+ 远超 120 的充能半径。
			// 掩体也受影响（放置 9 个只生效 5 个）——
			// 弹道不经过掩体，自然既不会被挡也不会被打。
			//
			// 改成 600 + rng.Intn(300) → y ∈ [600, 900]，
			// 完整落在敌人走廊内，且留出 100 的下边距（FIELD_H = 1000）。
			// 走廊宽度 300 / 走廊全宽 340 = 88% 覆盖。
			Y:     600 + rng.Intn(300),
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
// enemyScoreUpperBound 返回一只敌人贡献的分数估算（击杀分 + 伤害分）。
//
// ⚠️ 必须用**缩放后**的血量（ScaleEnemy），不能用 SeedEnemies 里的基准值。
//
// 引擎（engine.ts）里 `this.score += Number(res.totalDamage / 100n)`
// 累加的是**实际造成的伤害**，而实际伤害对应的是缩放后的血量（×8）。
// 估算用基准血量就与实际差了 8 倍的伤害分。
//
// 这个不一致本身不会让关卡不可通关（估算偏小 ⇒ 门槛偏低 ⇒ 更容易），
// 但它让「门槛 = 理论满分的固定比例」这个设计**失去意义**：
// 比例是相对估算算的，而估算的分母（伤害分）不对。
func enemyScoreUpperBound(enemyID int) int64 {
	for _, e := range SeedEnemies {
		if e.ID != enemyID {
			continue
		}
		kill := int64(scoreOnKillNormal)
		if e.IsBoss {
			kill = scoreOnKillBoss
		}
		// 与引擎口径对齐：缩放后的血量 ÷ 同一个伤害分母
		scaled := ScaleEnemy(e)
		return kill + (scaled.HP+scaled.ShieldHP)/scorePerDamageUnit
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
