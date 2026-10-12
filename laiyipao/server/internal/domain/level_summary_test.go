package domain

// `LevelSummary`（列表形状）与 `GeneratedLevel`（完整形状）的**字段对拍**。
//
// ## 为什么必须有这个守卫
//
// 第 150 轮把 `/config` 的 `levels` 从 `GeneratedLevel` 换成了 `LevelSummary`
// （去掉 `waves`，实测省掉整个配置包 56% 的体积）。
//
// 代价是**两份结构体并存**，于是新增字段时会漏抄：
// 在 `GeneratedLevel` 加一个 `new_field`，忘了同步到 `LevelSummary` ——
// 于是 `/config` 里那个字段**静默消失**，而：
//
//   - 编译通过（两个类型本就独立）
//   - 所有测试通过（客户端读到的都是 `undefined`，而没人断言它）
//   - 后台看起来一切正常
//
// 症状是「某个字段在某条下发路径上不见了」，而它消失**没有报错**。
//
// 这正是本仓库反复吃过的同一类亏（工程约束 4：跨端字段名必须显式约定）。
// 所以这里用**反射对拍**把它变成结构性不变量：
// `GeneratedLevel` 的每个 json 字段（`waves` 除外）必须在 `LevelSummary`
// 里同名同类型。

import (
	"reflect"
	"testing"
)

// jsonField 收集一个 struct 上带 json tag 的字段，键是 json 名。
func jsonField(t *testing.T, v any) map[string]reflect.Type {
	t.Helper()
	typ := reflect.TypeOf(v)
	if typ.Kind() != reflect.Struct {
		t.Fatalf("%s 不是 struct", typ)
	}
	out := make(map[string]reflect.Type, typ.NumField())
	for i := 0; i < typ.NumField(); i++ {
		f := typ.Field(i)
		tag := f.Tag.Get("json")
		if tag == "" {
			t.Errorf("%s.%s 缺 json tag —— 下发时会按字段名原文（PascalCase）"+
				"序列化，客户端 snake_case 类型读到的全是 undefined",
				typ.Name(), f.Name)
			continue
		}
		name := tag
		for i := 0; i < len(tag); i++ {
			if tag[i] == ',' {
				name = tag[:i]
				break
			}
		}
		if name == "" {
			t.Errorf("%s.%s 的 json tag %q 没有字段名", typ.Name(), f.Name, tag)
			continue
		}
		if _, dup := out[name]; dup {
			t.Errorf("%s 有两个字段映射到 json 名 %q", typ.Name(), name)
		}
		out[name] = f.Type
	}
	return out
}

// wavesFieldName 是唯一允许「只存在于完整形状」的字段。
//
// 写成变量而不是硬编码在比较逻辑里，是为了让它在断言失败时
// 出现在错误信息里 —— 这样「有人给 waves 加了兄弟字段」时，
// 报错会直接告诉你是哪个字段漏了。
const wavesFieldName = "waves"

// TestLevelSummaryCoversEveryGeneratedLevelField 摘要必须覆盖完整形状的每个字段
// （`waves` 除外）。
//
// 判据是「同名 + 同类型」两条都查：
//   - 只查名字 → 有人把 `BaseHP int64` 抄成 `BaseHP int` 时查不出来，
//     而那个错误会让客户端读到一个被 JSON 截断的数值。
func TestLevelSummaryCoversEveryGeneratedLevelField(t *testing.T) {
	full := jsonField(t, GeneratedLevel{})
	sum := jsonField(t, LevelSummary{})

	for name, typ := range full {
		if name == wavesFieldName {
			if _, ok := sum[name]; ok {
				t.Errorf("LevelSummary 不该有 %q —— 它是唯一被故意剔除的字段，"+
					"留着它会让 /config 又大 56%%", name)
			}
			continue
		}
		got, ok := sum[name]
		if !ok {
			t.Errorf("LevelSummary 缺字段 %q —— 它在 GeneratedLevel 里是 %s。"+
				"漏抄的后果是该字段从 /config 静默消失（编译通过、测试全绿）",
				name, typ)
			continue
		}
		if got != typ {
			t.Errorf("LevelSummary.%s 类型 = %s，GeneratedLevel.%s 是 %s —— "+
				"两端 JSON 表示会不一致", name, got, name, typ)
		}
	}
	for name := range sum {
		if _, ok := full[name]; !ok {
			t.Errorf("LevelSummary 有 GeneratedLevel 没有的字段 %q —— "+
				"多出来的字段永远是零值，属于纯粹的死字段", name)
		}
	}
}

// TestSummarizeKeepsEverythingButWaves 转换函数必须逐字段搬运。
//
// 为什么要单独测而不靠上面的反射：反射只保证「字段存在且类型一致」，
// 它**测不出** `Summarize` 里忘了赋值 —— 那会让每个字段都合法地是零值。
// 这正是最容易发生的一类抄写疏漏，所以单独钉一次。
func TestSummarizeKeepsEverythingButWaves(t *testing.T) {
	gl := GenerateLevel(37) // 中段关卡，三层都可能有内容
	got := Summarize(gl)

	if got.ID != gl.ID || got.Chapter != gl.Chapter || got.Name != gl.Name {
		t.Errorf("标识字段没搬过来：%+v", got)
	}
	if got.Seed != gl.Seed || got.BaseHP != gl.BaseHP || got.WaveCount != gl.WaveCount {
		t.Errorf("数值字段没搬过来：%+v", got)
	}
	if got.Difficulty != gl.Difficulty || got.EnergyCost != gl.EnergyCost {
		t.Errorf("难度/体力没搬过来：%+v", got)
	}
	if got.IsBoss != gl.IsBoss || got.MaxScore != gl.MaxScore {
		t.Errorf("BOSS/满分没搬过来：%+v", got)
	}
	if got.ElementCap != gl.ElementCap || got.ArmorPermille != gl.ArmorPermille ||
		got.MaxReactionTier != gl.MaxReactionTier {
		t.Errorf("战斗系数没搬过来：%+v", got)
	}
	if len(got.StarTargets) != len(gl.StarTargets) {
		t.Errorf("star_targets 长度 %d != %d", len(got.StarTargets), len(gl.StarTargets))
	}
	for i := range gl.StarTargets {
		if got.StarTargets[i] != gl.StarTargets[i] {
			t.Errorf("star_targets[%d] = %d，原值 %d", i, got.StarTargets[i], gl.StarTargets[i])
		}
	}
	if len(got.Terrain) != len(gl.Terrain) {
		t.Errorf("terrain 长度 %d != %d", len(got.Terrain), len(gl.Terrain))
	}

	// ⚠️ 关键的一条：`waves` 必须是**真的被丢掉**，而不是被复制。
	// 少这条的话，将来有人「顺手」把 Waves 也搬过来，
	// 体积优化就静默失效了 —— 而症状只是「首屏慢了一点」，不会有人追。
	// 结构体里没有这个字段，所以这里用 JSON 形状断言：
	if _, ok := jsonField(t, got)["waves"]; ok {
		t.Error("摘要里出现了 waves 字段 —— 体积优化已被悄悄回退")
	}
}

// TestSummarizeAllIsSameAsSummarize 批量转换与逐个转换必须等价。
//
// 便宜，但防的是一类真实的错：`SummarizeAll` 写成
// `make([]LevelSummary, len(levels))` + `out[i] = …` 之后索引错位，
// 而那种错误的症状是「第 50 关的数值是第 49 关的」——
// 各字段类型都对，任何静态检查都查不出来。
func TestSummarizeAllIsSameAsSummarize(t *testing.T) {
	all := GenerateAllLevels()
	got := SummarizeAll(all)
	if len(got) != len(all) {
		t.Fatalf("SummarizeAll 长度 %d != %d", len(got), len(all))
	}
	for i := range all {
		want := Summarize(all[i])
		if !reflect.DeepEqual(got[i], want) {
			t.Fatalf("第 %d 条不一致：%+v vs %+v", i, got[i], want)
		}
		if got[i].ID != all[i].ID {
			t.Fatalf("第 %d 条的 id = %d（原值 %d）—— 索引错位了", i, got[i].ID, all[i].ID)
		}
	}
}
