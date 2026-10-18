package domain

// 复活币契约守卫：Go 与 miniapp 的 `LeakDamageDivisor` 必须一致。
//
// # 为什么这是一条跨端契约守卫
//
// 复活的额外血量 = `base_hp × ReviveExtraLeakCount / LeakDamageDivisor`，
// 而引擎（`miniapp/src/game/engine.ts`）扣血用的是**它自己那份**
// `LeakDamageDivisor`。两边不同步时：
//
//	服务端按 10 算「多扛 3 次」→ 下发 base_hp × 3/10
//	引擎按别的值扣血         → 玩家实际多扛的次数对不上
//
// 而这个偏差**不报错**、不 panic，只是道具效果偏掉 ——
// 属于「玩家投诉了但没人查得出来」的那一类。
//
// 判定方式读的是 miniapp 的**源码文本**，不是 import：
// 本包是纯 Go，import TS 是不可能的；而两边各写各的常量
// 恰恰是这条守卫要防的事。

import (
	"os"
	"path/filepath"
	"regexp"
	"strconv"
	"strings"
	"testing"
)

func TestLeakDamageDivisorMatchesEngine(t *testing.T) {
	// 候选路径：仓库里 miniapp 的位置随测试包深度不同而不同。
	var src string
	for _, p := range []string{
		filepath.Join("..", "..", "..", "miniapp", "src", "game", "engine.ts"),
		filepath.Join("..", "..", "..", "..", "miniapp", "src", "game", "engine.ts"),
		filepath.Join("..", "..", "..", "..", "..", "miniapp", "src", "game", "engine.ts"),
	} {
		b, err := os.ReadFile(p)
		if err != nil {
			continue
		}
		src = string(b)
		break
	}
	if src == "" {
		t.Fatal("找不到 miniapp/src/game/engine.ts —— " +
			"这条守卫未执行，不是「通过了」。检查相对路径是否还对得上")
	}

	re := regexp.MustCompile(`export const LeakDamageDivisor = (\d+)n`)
	m := re.FindStringSubmatch(src)
	if m == nil {
		t.Fatal("engine.ts 里找不到 `export const LeakDamageDivisor = <数字>n` —— " +
			"它被改名或改写法了，本守卫已失效（不是「两边一致」）")
	}
	got, err := strconv.Atoi(m[1])
	if err != nil {
		t.Fatalf("解析 engine.ts 的 LeakDamageDivisor %q 失败：%v", m[1], err)
	}
	if got != LeakDamageDivisor {
		t.Errorf("LeakDamageDivisor 两端不一致：Go = %d，engine.ts = %d。\n"+
			"复活的额外血量按 Go 的值算、引擎按 TS 的值扣 —— "+
			"偏差不报错，只会让「多扛 N 次漏怪」实际对不上", LeakDamageDivisor, got)
	}
}

// TestReviveExtraBaseHPMatchesLeakCount 额外血量必须恰好等于 N 次漏怪的伤害。
//
// 这条钉住的是**语义**：`ReviveExtraBaseHP` 不是「某个血量」，
// 而是「`ReviveExtraLeakCount` 次普通怪漏怪的伤害」。语义一漂移，
// 「多扛 3 次」这个玩家能理解的说法就不再成立。
func TestReviveExtraBaseHPMatchesLeakCount(t *testing.T) {
	l := GenerateLevel(1)
	got := ReviveExtraBaseHP(l)

	// 引擎里普通怪：leakDamage = baseHpMax / LeakDamageDivisor（再过 applyArmor）
	perLeak := l.BaseHP / int64(LeakDamageDivisor)
	if want := perLeak * ReviveExtraLeakCount; got != want {
		t.Errorf("ReviveExtraBaseHP = %d，但 %d 次漏怪的伤害 = %d×%d = %d",
			got, ReviveExtraLeakCount, perLeak, ReviveExtraLeakCount, want)
	}

	// 关键不变式：额外血量必须**不**随 baseHpMax 缩放地生效，
	// 所以它只加在当前血量上。数值上表现为「总血量 > 上限」——
	// 这正是「当前 ≠ 上限」的表现形态，也是 applyArmor 分母取上限的原因。
	if l.BaseHP <= 0 {
		t.Fatal("GenerateLevel(1).BaseHP 非正 —— 下面的不变式没有意义")
	}
	if got <= 0 {
		t.Errorf("ReviveExtraBaseHP = %d，必须为正 —— 否则复币什么也不给", got)
	}
}

// TestReviveDoesNotTouchGeneratedLevel 复活**不得**改关卡的 BaseHP。
//
// 这是第 148–152 轮那个无效实现的根因：抬高 `BaseHP` 会同步抬高
// `baseHpMax`，而漏怪伤害以它为分母，于是加成被整个抵掉
// （实测 100 关 0 关翻盘，见 README 已知边界第 16 条）。
//
// 现在 `ReviveExtraBaseHP` 只返回数字、不返回改过的关卡，
// 这条守卫确保它以后也不会被改回「返回一个 BaseHP 被抬高的关卡」。
func TestReviveDoesNotTouchGeneratedLevel(t *testing.T) {
	before := GenerateLevel(1).BaseHP
	_ = ReviveExtraBaseHP(GenerateLevel(1))
	if after := GenerateLevel(1).BaseHP; after != before {
		t.Errorf("调用 ReviveExtraBaseHP 之后 GenerateLevel(1).BaseHP 从 %d 变成 %d —— "+
			"复活一旦改关卡本体，漏怪伤害会跟着涨，加成被抵掉", before, after)
	}

	// 同时钉住新常量存在的理由：倍率常量已被移除，
	// 谁再把它加回来时应当重新读 domain/revive.go 的文件头。
	src, err := os.ReadFile("revive.go")
	if err != nil {
		t.Fatalf("读 revive.go 失败：%v", err)
	}
	if strings.Contains(string(src), "ReviveBaseHPMultPermille") {
		t.Error("revive.go 里又出现了 ReviveBaseHPMultPermille —— " +
			"倍率型复活已实测无效（100 关 0 关翻盘），加回来前先读文件头")
	}
}
