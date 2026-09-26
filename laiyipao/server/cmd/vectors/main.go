// Command vectors 从 Go 侧真相源导出跨端契约文件与测试夹具。
//
// 为什么需要生成器：
//
//	① 契约文件此前是手写的，实践中出过两类问题 ——
//	   写进了 UTF-8 BOM，Go 的 encoding/json 直接拒绝（invalid character 'ï'），
//	   而 Vite 的 JSON 插件会默默吃掉 BOM，于是两端对同一个文件的行为不一致；
//	   手抄数值 inevitably 会漂移。
//	② 引擎冒烟测试此前全用手工构造的关卡，于是「热量吸收态导致战斗永久停摆」
//	   与「刷怪 delay 单位错 50 倍」这两个致命缺陷一路活到端到端试玩才暴露。
//	   真实内容表是唯一能暴露这类问题的夹具。
//
// 用法：
//
//	go run ./cmd/vectors            # 写入全部文件
//	go run ./cmd/vectors -check     # 只校验是否为最新，不写盘（CI 用）
//
// ⚠️ 本工具导出的**只有数据**（反应链数值表、真实关卡/敌人/技能），
// 不含任何「期望值」。期望值必须人工推导后写在 formula_vectors.json 里 ——
// 用实现生成期望值就是自证，测试会永远绿。
package main

import (
	"encoding/json"
	"flag"
	"fmt"
	"os"
	"path/filepath"
	"strconv"

	"github.com/laiyipao/server/internal/domain"
)

type reactionRow struct {
	Key              string `json:"key"`
	Name             string `json:"name"`
	BaseCoef         int    `json:"base_coef"`
	AttackWeightPct  int    `json:"attack_weight_pct"`
	StatusDurationMs int    `json:"status_duration_ms"`
	AoeRadius        int    `json:"aoe_radius"`
	DispelShield     bool   `json:"dispel_shield"`
	AmplifyPct       int    `json:"amplify_pct"`
	Descr            string `json:"descr"`
}

func buildReactions() ([]byte, error) {
	specs := domain.AllReactionSpecs()
	rows := make([]reactionRow, 0, len(specs))
	for _, s := range specs {
		rows = append(rows, reactionRow{
			Key:              string(s.Key),
			Name:             s.Name,
			BaseCoef:         s.BaseCoef,
			AttackWeightPct:  s.AttackWeightPct,
			StatusDurationMs: s.StatusDurationMs,
			AoeRadius:        s.AoeRadius,
			DispelShield:     s.DispelShield,
			AmplifyPct:       s.AmplifyPct,
			Descr:            s.Descr,
		})
	}
	return mustIndent(rows), nil
}

// levelSeeds 是 100 关的种子表。
//
// 存在的理由：客户端曾用
//   expect(levelSeed(1)).toBe(levelSeed(1))
// 来"验证与 Go 侧一致" —— 那是恒等式，且 PHI 常量在测试里本地复制了一份，
// 所以改 Go 侧的实现完全不影响它。实测把 Go 侧的
// 0x9E3779B97F4A7C15 改掉，这条名叫「与 Go 侧一致」的用例照样绿。
//
// 这里导出真值，客户端对字面量断言，才构成真正的跨端锁。
func buildLevelSeeds() ([]byte, error) {
	seeds := make([]string, 0, 100)
	for id := 1; id <= 100; id++ {
		seeds = append(seeds, strconv.FormatInt(domain.GenerateLevel(id).Seed, 10))
	}
	return mustIndent(seeds), nil
}

func mustIndent(v any) []byte {
	raw, err := json.MarshalIndent(v, "", "  ")
	if err != nil {
		panic(err)
	}
	// 显式不加 BOM。encoding/json 不接受 BOM，Vite 却会吃掉它 ——
	// 同一个文件在两端被解析成不同内容是最难查的一类故障。
	return append(raw, '\n')
}

// smokeFixture 是引擎冒烟测试用的真实内容切片。
//
// 只包含第 1 关真正需要的东西，避免把 22 敌人 / 42 技能全导出去
// （客户端已经在 /config 时拿到全量，这里要的是**离线可跑**的最小集）。
type smokeFixture struct {
	Note      string                 `json:"note"`
	Level     domain.GeneratedLevel  `json:"level"`
	Enemies   []domain.SeedEnemy     `json:"enemies"`
	Skills    []domain.SeedSkill     `json:"skills"`
	Composite []domain.SeedSkill     `json:"composite_skills"`
	Equip     []domain.SeedEquipment `json:"equipment"`
}

const smokeLevelID = 1

func buildSmoke() ([]byte, error) {
	gl := domain.GenerateLevel(smokeLevelID)

	// 收集该关实际用到的敌人/技能 id，只导这些。
	usedEnemy := map[int]bool{}
	usedSkill := map[int]bool{}
	for _, w := range gl.Waves {
		for _, sp := range w.Spawns {
			usedEnemy[sp.EnemyID] = true
		}
	}

	all := domain.SeedEnemies
	fx := smokeFixture{
		Note: "由 server/cmd/vectors 从 Go 真相源导出，供 miniapp 引擎冒烟测试使用。" +
			"不要手工编辑：改内容表后重新执行 go run ./cmd/vectors。",
		Level:   gl,
		Enemies: []domain.SeedEnemy{},
		Skills:  []domain.SeedSkill{},
	}
	for _, e := range all {
		if usedEnemy[e.ID] {
			fx.Enemies = append(fx.Enemies, e)
		}
	}
	for _, s := range domain.SeedSkills {
		fx.Skills = append(fx.Skills, s)
		usedSkill[s.ID] = true
	}
	for _, s := range domain.SeedCompositeSkills {
		fx.Composite = append(fx.Composite, s)
	}
	fx.Equip = domain.SeedEquipmentList

	if len(fx.Enemies) == 0 {
		return nil, fmt.Errorf("第 %d 关没有用到任何敌人，夹具无意义", smokeLevelID)
	}
	return mustIndent(fx), nil
}

type target struct {
	path string
	gen  func() ([]byte, error)
	desc string
}

func main() {
	check := flag.Bool("check", false, "只校验是否为最新，不写盘")
	flag.Parse()

	targets := []target{
		{filepath.Join("testdata", "reaction_specs.json"),
			buildReactions,
			fmt.Sprintf("%d 条反应", len(domain.AllReactionSpecs()))},
		{filepath.Join("testdata", "smoke_level1.json"), buildSmoke, "第 1 关真实夹具"},
		{filepath.Join("testdata", "level_seeds.json"), buildLevelSeeds, "100 关种子表"},
	}

	bad := 0
	for _, tg := range targets {
		want, err := tg.gen()
		if err != nil {
			fmt.Fprintln(os.Stderr, "生成", tg.path, "失败:", err)
			bad++
			continue
		}
		if *check {
			got, err := os.ReadFile(tg.path)
			if err != nil {
				fmt.Fprintln(os.Stderr, "缺少", tg.path, "，请运行 go run ./cmd/vectors")
				bad++
				continue
			}
			if string(got) != string(want) {
				fmt.Fprintln(os.Stderr, tg.path, "已过期，请运行 go run ./cmd/vectors 后提交")
				bad++
				continue
			}
			fmt.Printf("OK   %s（%s）\n", tg.path, tg.desc)
			continue
		}
		if err := os.MkdirAll(filepath.Dir(tg.path), 0o755); err != nil {
			fmt.Fprintln(os.Stderr, "创建目录失败:", err)
			bad++
			continue
		}
		if err := os.WriteFile(tg.path, want, 0o644); err != nil {
			fmt.Fprintln(os.Stderr, "写盘失败:", err)
			bad++
			continue
		}
		fmt.Printf("写入 %s（%s）\n", tg.path, tg.desc)
	}
	if bad > 0 {
		os.Exit(1)
	}
}
