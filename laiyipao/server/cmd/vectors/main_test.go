package main

// vectors 生成器的可测部分：三个 JSON 夹具的构造函数。
//
// 为什么值得测：这份数据是跨端契约的唯一真值来源 ——
// 客户端用 reaction_specs.json 断言反应数值、用 level_seeds.json
// 对字面量锁种子、用 smoke_levels.json 跑引擎冒烟。
// 生成逻辑坏了（关卡数少了、敌人过滤错了），客户端测试会大面积误报。
//
// main() 本体（flag 解析、写盘、os.Exit）按豁免处理。

import (
	"bytes"
	"encoding/json"
	"strconv"
	"testing"

	"github.com/laiyipao/server/internal/domain"
)

func TestBuildReactions(t *testing.T) {
	raw, err := buildReactions()
	if err != nil {
		t.Fatalf("buildReactions：%v", err)
	}
	var rows []reactionRow
	if err := json.Unmarshal(raw, &rows); err != nil {
		t.Fatalf("输出不是合法 JSON：%v", err)
	}
	if len(rows) != len(domain.AllReactionSpecs()) {
		t.Fatalf("反应条数应为 %d，实际 %d", len(domain.AllReactionSpecs()), len(rows))
	}
	// 数值必须与 domain 真相源逐条一致
	specs := domain.AllReactionSpecs()
	for i, row := range rows {
		if row.Key != string(specs[i].Key) || row.BaseCoef != specs[i].BaseCoef {
			t.Errorf("第 %d 条反应与 domain 不一致：%+v vs %+v", i, row, specs[i])
		}
	}
}

func TestBuildLevelSeeds(t *testing.T) {
	raw, err := buildLevelSeeds()
	if err != nil {
		t.Fatalf("buildLevelSeeds：%v", err)
	}
	var seeds []string
	if err := json.Unmarshal(raw, &seeds); err != nil {
		t.Fatalf("输出不是合法 JSON：%v", err)
	}
	if len(seeds) != domain.TotalLevels {
		t.Fatalf("应导出 %d 个种子，实际 %d", domain.TotalLevels, len(seeds))
	}
	// 种子必须与生成器逐关一致 —— 客户端对字面量断言，这里漂了它就红
	for id := 1; id <= domain.TotalLevels; id++ {
		want := domain.GenerateLevel(id).Seed
		if seeds[id-1] != strconv.FormatInt(want, 10) {
			t.Errorf("第 %d 关种子漂移：%s vs %d", id, seeds[id-1], want)
		}
	}
}

func TestBuildSmoke(t *testing.T) {
	raw, err := buildSmoke()
	if err != nil {
		t.Fatalf("buildSmoke：%v", err)
	}
	var fx smokeFixture
	if err := json.Unmarshal(raw, &fx); err != nil {
		t.Fatalf("输出不是合法 JSON：%v", err)
	}
	if len(fx.Levels) != domain.TotalLevels {
		t.Errorf("应导出全部 %d 关（难度梯度不能留黑盒），实际 %d", domain.TotalLevels, len(fx.Levels))
	}
	if len(fx.Enemies) == 0 {
		t.Error("敌人切片不应为空（关卡没用到任何敌人，夹具无意义）")
	}
	if len(fx.Skills) == 0 || len(fx.Equip) == 0 {
		t.Error("skills/equipment 不应为空")
	}
	if fx.ScoreRules.OnKillNormal == 0 || fx.SkillRules.MaxLevel == 0 {
		t.Error("score_rules / skill_rules 必须进夹具（跨端守卫依赖）")
	}
	// 敌人过滤：只导真实用到的 —— 用一个已知敌人反查
	used := map[int]bool{}
	for _, gl := range fx.Levels {
		for _, w := range gl.Waves {
			for _, sp := range w.Spawns {
				used[sp.EnemyID] = true
			}
		}
	}
	for _, e := range fx.Enemies {
		if !used[e.ID] {
			t.Errorf("敌人 %d 未被任何关卡使用，过滤逻辑失效", e.ID)
		}
	}
}

// TestMustIndentNoBOM 契约：输出不带 BOM（Go 的 json 拒绝 BOM，
// Vite 却会吃掉 —— 两端解析不一致是最难查的故障）且以换行收尾。
func TestMustIndentNoBOM(t *testing.T) {
	raw := mustIndent(map[string]int{"a": 1})
	if bytes.HasPrefix(raw, []byte{0xEF, 0xBB, 0xBF}) {
		t.Error("输出带 BOM —— Go json 直接拒绝解析")
	}
	if len(raw) == 0 || raw[len(raw)-1] != '\n' {
		t.Error("输出应以换行收尾（diff 友好）")
	}
	if !bytes.Contains(raw, []byte("\n  ")) {
		t.Error("应是缩进格式（人工可读，契约 diff 才有意义）")
	}
}

// TestMustIndentPanic 不可序列化的值必须 panic（内部不变量，不是可恢复错误）。
func TestMustIndentPanic(t *testing.T) {
	defer func() {
		if r := recover(); r == nil {
			t.Error("不可序列化值应 panic")
		}
	}()
	mustIndent(make(chan int))
}
