package domain

import (
	"encoding/json"
	"errors"
	"strings"
	"testing"
	"time"
)

func newTestToken(levelID int) BattleToken {
	now := time.Now()
	return BattleToken{
		ID: 1, UserID: 1, LevelID: levelID, Seed: 12345,
		IssuedAt: now.Add(-time.Minute), ExpiresAt: now.Add(time.Hour),
	}
}

func winInput(gl GeneratedLevel) SettleInput {
	// 时长必须达到该关的速率锚点（D3 起锚点按关卡推导，不再是固定 60s）——
	// 写死 120s 会让高关卡的分数被裁，星级断言随之失效。
	return SettleInput{
		Result: "win", Score: gl.StarTargets[0], Kills: gl.TotalEnemies(),
		WaveReached: gl.WaveCount, DurationMs: int(speedAnchorSec(gl) * 1000), Shots: 400, Hits: 300,
		Reactions: 80, HPLeft: 400,
	}
}

func testLimits() SettleLimits {
	return SettleLimits{MinDurationMs: 15000}
}

// 防作弊：重放同一份战报必须被拒（token 已使用）。
func TestSettleRejectsUsedToken(t *testing.T) {
	gl := GenerateLevel(1)
	tok := newTestToken(1)
	used := now0()
	tok.UsedAt = &used
	_, err := ValidateSettle(tok, gl, winInput(gl), testLimits(), time.Now())
	if !errors.Is(err, ErrTokenUsed) {
		t.Errorf("已使用的 token 应返回 ErrTokenUsed，实际 %v", err)
	}
}

func TestSettleRejectsExpiredToken(t *testing.T) {
	gl := GenerateLevel(1)
	tok := newTestToken(1)
	tok.ExpiresAt = time.Now().Add(-time.Minute)
	_, err := ValidateSettle(tok, gl, winInput(gl), testLimits(), time.Now())
	if !errors.Is(err, ErrTokenExpired) {
		t.Errorf("过期 token 应返回 ErrTokenExpired，实际 %v", err)
	}
}

func TestSettleRejectsLevelMismatch(t *testing.T) {
	gl := GenerateLevel(5)
	tok := newTestToken(7) // 凭证是第 7 关，结算却是第 5 关
	_, err := ValidateSettle(tok, gl, winInput(gl), testLimits(), time.Now())
	if !errors.Is(err, ErrLevelMismatch) {
		t.Errorf("关卡不符应返回 ErrLevelMismatch，实际 %v", err)
	}
}

// 防刷分：击杀数超过关卡总怪数必须被拒（不是裁剪，是拒绝 —— 静默修正会掩盖问题）。
func TestSettleRejectsImpossibleKills(t *testing.T) {
	gl := GenerateLevel(3)
	tok := newTestToken(3)
	in := winInput(gl)
	in.Kills = gl.TotalEnemies() + 1
	if _, err := ValidateSettle(tok, gl, in, testLimits(), time.Now()); !errors.Is(err, ErrTooManyKills) {
		t.Errorf("超量击杀应返回 ErrTooManyKills，实际 %v", err)
	}
}

func TestSettleRejectsWaveOverflow(t *testing.T) {
	gl := GenerateLevel(3)
	tok := newTestToken(3)
	in := winInput(gl)
	in.WaveReached = gl.WaveCount + 1
	if _, err := ValidateSettle(tok, gl, in, testLimits(), time.Now()); !errors.Is(err, ErrWaveExceeded) {
		t.Errorf("超量波次应返回 ErrWaveExceeded，实际 %v", err)
	}
}

func TestSettleRejectsNegativeNumbers(t *testing.T) {
	gl := GenerateLevel(3)
	tok := newTestToken(3)
	for name, mut := range map[string]func(*SettleInput){
		"负击杀": func(i *SettleInput) { i.Kills = -1 },
		"负波次": func(i *SettleInput) { i.WaveReached = -1 },
		"负时长": func(i *SettleInput) { i.DurationMs = -1 },
		"负发射": func(i *SettleInput) { i.Shots = -1 },
		"负命中": func(i *SettleInput) { i.Hits = -1 },
		"负反应": func(i *SettleInput) { i.Reactions = -1 },
	} {
		in := winInput(gl)
		mut(&in)
		if _, err := ValidateSettle(tok, gl, in, testLimits(), time.Now()); err == nil {
			t.Errorf("%s：应被拒绝", name)
		}
	}
}

func TestSettleRejectsHitRateAbove100(t *testing.T) {
	gl := GenerateLevel(3)
	tok := newTestToken(3)
	in := winInput(gl)
	in.Shots, in.Hits = 10, 20
	if _, err := ValidateSettle(tok, gl, in, testLimits(), time.Now()); !errors.Is(err, ErrInvalidHitRate) {
		t.Errorf("命中>发射应返回 ErrInvalidHitRate，实际 %v", err)
	}
}

// 秒通检测：通关 + 清完全部敌人 + 时长过短 → 拒绝。
func TestSettleRejectsInstantClear(t *testing.T) {
	gl := GenerateLevel(10)
	tok := newTestToken(10)
	in := winInput(gl)
	in.DurationMs = 2000 // 2 秒
	if _, err := ValidateSettle(tok, gl, in, testLimits(), time.Now()); !errors.Is(err, ErrTooShort) {
		t.Errorf("秒通应返回 ErrTooShort，实际 %v", err)
	}
}

// 失败局可以秒退 —— 不能对失败的局做最短时长限制，否则玩家会莫名被拒。
func TestSettleAllowsQuickLose(t *testing.T) {
	gl := GenerateLevel(10)
	tok := newTestToken(10)
	in := SettleInput{
		Result: "lose", Kills: 1, WaveReached: 1, DurationMs: 800, Shots: 5, Hits: 2,
	}
	if _, err := ValidateSettle(tok, gl, in, testLimits(), time.Now()); err != nil {
		t.Errorf("失败局秒退不应被拒：%v", err)
	}
}

// 得分速率超限必须被裁剪（而非拒绝），并记录裁剪原因供后台观察。
func TestSettleClampsScoreRate(t *testing.T) {
	gl := GenerateLevel(10)
	tok := newTestToken(10)
	in := winInput(gl)
	in.Score = 99_999_999
	in.DurationMs = 20000 // 高于最短时长限制，只让"得分速率"这一个变量越界
	res, err := ValidateSettle(tok, gl, in, testLimits(), time.Now())
	if err != nil {
		t.Fatalf("不应报错：%v", err)
	}
	if !res.Clamped {
		t.Error("应标记为已裁剪")
	}
	if res.ClampReason == "" {
		t.Error("裁剪原因不能为空")
	}
	if res.Score >= in.Score {
		t.Errorf("得分应被下调：%d → %d", in.Score, res.Score)
	}
	// 20 秒 × 40000/秒 = 80 万，但仍不能超过满星门槛（更高无意义）
	full := gl.StarTargets[len(gl.StarTargets)-1]
	if res.Score > full {
		t.Errorf("裁剪后得分 %d 仍超过满星门槛 %d", res.Score, full)
	}
}

// 星级必须由服务端按门槛重算，不信任客户端上报。
func TestSettleRecomputesStars(t *testing.T) {
	gl := GenerateLevel(20)
	tok := newTestToken(20)
	in := winInput(gl)
	in.Score = gl.StarTargets[2] // 够三星
	in.Stars = 0                 // 但客户端谎报 0 星
	res, err := ValidateSettle(tok, gl, in, testLimits(), time.Now())
	if err != nil {
		t.Fatalf("不应报错：%v", err)
	}
	if res.Stars != 3 {
		t.Errorf("星级应由服务端重算为 3，实际 %d", res.Stars)
	}
	if !res.Clamped {
		t.Error("星级被修正时应标记为已裁剪")
	}
}

func TestSettleRejectsOverclaimedStars(t *testing.T) {
	gl := GenerateLevel(20)
	tok := newTestToken(20)
	in := winInput(gl)
	in.Score = gl.StarTargets[0] // 只够 1 星
	in.Stars = 3                 // 谎报 3 星
	res, err := ValidateSettle(tok, gl, in, testLimits(), time.Now())
	if err != nil {
		t.Fatalf("不应报错：%v", err)
	}
	if res.Stars != 1 {
		t.Errorf("星级应被修正为 1，实际 %d", res.Stars)
	}
}

// 胜利必须以"清完全部敌人"为准，不信任客户端的 result 字段。
func TestSettleWinRequiresFullClear(t *testing.T) {
	gl := GenerateLevel(12)
	tok := newTestToken(12)
	in := winInput(gl)
	in.Kills = gl.TotalEnemies() - 1 // 还差一只却说通关
	res, err := ValidateSettle(tok, gl, in, testLimits(), time.Now())
	if err != nil {
		t.Fatalf("不应报错：%v", err)
	}
	if res.Win {
		t.Error("未清完敌人却判定通关，这是最典型的刷分路径")
	}
}

// 掉落必须有硬上限，防止利用时长/击杀构造刷收益。
func TestLootIsCapped(t *testing.T) {
	gl := GenerateLevel(60)
	caps := DefaultLootCaps()
	loot := ComputeLoot(gl, 3, 100000, 100000, 1000000)
	if loot["coin"] > caps.MaxCoinPerBattle {
		t.Errorf("金币掉落 %d 超过上限 %d", loot["coin"], caps.MaxCoinPerBattle)
	}
	if loot["keys"] > caps.MaxKeysPerBattle {
		t.Errorf("钥匙掉落 %d 超过上限 %d", loot["keys"], caps.MaxKeysPerBattle)
	}
	// 三星必须给钥匙
	if _, ok := loot["keys"]; !ok {
		t.Error("三星应给钥匙")
	}
	// 低星不该给钥匙
	loot2 := ComputeLoot(gl, 1, 10, 10, 0)
	if _, ok := loot2["keys"]; ok {
		t.Error("非三星不应给钥匙")
	}
}

// 诊断（L-1）：必须能区分"搭配问题"与"硬实力不足"，
// 否则诊断就退化成"升级更快一点"，那还不如不说。
func TestDiagnoseDistinguishesBuildVsPower(t *testing.T) {
	gl := GenerateLevel(30)

	t.Run("没施加任何元素", func(t *testing.T) {
		d := DiagnoseFailure(DiagnoseInput{
			LevelID: 30, FailedTimes: 3, Kills: 50, TotalEnemies: gl.TotalEnemies(),
			ElementsUsed: map[string]int{}, Loadout: []Element{ElementFire},
		}, gl)
		if d.Stage != "no_element" {
			t.Errorf("应诊断为 no_element，实际 %s", d.Stage)
		}
		if d.Confidence < 80 {
			t.Errorf("元素为 0 的置信度应很高，实际 %d", d.Confidence)
		}
	})

	t.Run("元素用得少", func(t *testing.T) {
		d := DiagnoseFailure(DiagnoseInput{
			LevelID: 30, FailedTimes: 3, Kills: 80, TotalEnemies: gl.TotalEnemies(),
			ElementsUsed: map[string]int{"fire": 5}, Loadout: []Element{ElementFire, ElementIce},
		}, gl)
		if d.Stage != "low_reaction" {
			t.Errorf("应诊断为 low_reaction，实际 %s", d.Stage)
		}
	})

	t.Run("元素覆盖窄", func(t *testing.T) {
		d := DiagnoseFailure(DiagnoseInput{
			LevelID: 30, FailedTimes: 3, Kills: 60, TotalEnemies: gl.TotalEnemies(),
			ElementsUsed: map[string]int{"fire": 200, "ice": 200},
			Loadout:      []Element{ElementFire, ElementIce},
		}, gl)
		if d.Stage != "narrow_element" {
			t.Errorf("应诊断为 narrow_element，实际 %s", d.Stage)
		}
	})

	t.Run("硬实力不足", func(t *testing.T) {
		d := DiagnoseFailure(DiagnoseInput{
			LevelID: 30, FailedTimes: 4, Kills: 10, TotalEnemies: gl.TotalEnemies(),
			ElementsUsed: map[string]int{"fire": 300, "ice": 300, "lightning": 300},
			Loadout:      []Element{ElementFire, ElementIce, ElementLightning, ElementCorrosion, ElementKinetic},
		}, gl)
		if d.Stage != "low_power" {
			t.Errorf("应诊断为 low_power，实际 %s", d.Stage)
		}
	})
}

func now0() time.Time { return time.Now() }

// TestDiagnoseJSONShapeIsSnakeCase 钉住 Diagnose 的**序列化形状**。
//
// 这条测试不是冗余的：字段曾经全是无 tag，Go 按字段名原文序列化成
// PascalCase，而客户端读小写 —— 诊断卡整张空白（置信度%、标题、
// 详情、建议全是 undefined）。而三处测试当时全绿：
//   - `stage.test.ts` 的 mock 是**从客户端代码抄的**小写；
//   - `routes_e2e_test.go` 只测状态码；
//   - `e2e.ps1` 取了 `$diag` 但没断言行内字段。
//
// 于是「服务端发 PascalCase / 客户端读小写 / mock 也是小写」三方自洽。
//
// 这正是 ENGINEERING 第 4 条（跨端字段名双向核对）要防而没防住的形态：
// 单端测试的自洽数据证明不了契约。判据必须落在**序列化输出**上。
func TestDiagnoseJSONShapeIsSnakeCase(t *testing.T) {
	// 取一个真实会走到的分支，保证字段有值（空串会让 contains 判定失真）
	d := DiagnoseFailure(DiagnoseInput{
		LevelID: 30, FailedTimes: 3, Kills: 50, TotalEnemies: 100,
		ElementsUsed: map[string]int{}, Loadout: []Element{ElementFire},
	}, GenerateLevel(30))

	raw, err := json.Marshal(d)
	if err != nil {
		t.Fatalf("序列化失败：%v", err)
	}
	got := string(raw)

	for _, want := range []string{`"stage"`, `"title"`, `"detail"`, `"suggestions"`, `"confidence"`} {
		if !strings.Contains(got, want) {
			t.Errorf("Diagnose 序列化输出缺少 %s 键（跨端契约 C1 要求 snake_case）：\n%s\n"+
				"漏 json tag 时 Go 会按字段名原文输出 PascalCase，客户端读到 undefined", want, got)
		}
	}
	for _, bad := range []string{`"Stage"`, `"Title"`, `"Detail"`, `"Suggestions"`, `"Confidence"`} {
		if strings.Contains(got, bad) {
			t.Errorf("Diagnose 序列化输出出现了 %s —— 字段漏了 json tag", bad)
		}
	}
}
