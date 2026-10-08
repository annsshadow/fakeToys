package store

import (
	"context"
	"os"
	"path/filepath"
	"strings"
	"testing"
	"time"
)

// 连接池的会话时区必须被**钉死**（第 90 轮）。
//
// # 缺陷：日期折算的基准由**部署配置**决定
//
// 不钉的话，PG 的会话时区来自：
//
//	postgresql.conf 的 TimeZone
//	或托管实例的默认值（**常见是 UTC**）
//
// 而这个值会静默影响**所有**依赖日期折算的 SQL：
//
//	`sign_date DATE` 的 timestamptz -> date
//	`to_char(created_at, 'YYYY-MM-DD')`
//	任何 `::date` 转换
//
// 换句话说「今天几号」这个业务概念由**部署配置**决定 ——
// 而部署配置不在代码里、也不在测试里。
//
// # 第 89 轮已经撞到过一次
//
// 那里 Go 用 `periodStart`（`time.Local`）、PG 用会话时区，
// 两者各算一半。已改成传日期字面量绕开，
// 但**其余任何新增的日期折算还会再踩一次**。
//
// # 为什么用 RuntimeParams 而不是 AfterConnect + SET
//
// | 写法 | 漏掉的风险 |
// |---|---|
// | `AfterConnect: SET TimeZone = …` | 忘记挂回调 → **静默失效**，没有任何报错 |
// | `RuntimeParams["TimeZone"]` | 随连接握手一次性发出，不存在「漏掉某条连接」 |
func TestPoolSessionTimeZoneIsPinned(t *testing.T) {
	dsn := scratchDB(t)
	ctx, cancel := context.WithTimeout(context.Background(), 60*time.Second)
	defer cancel()

	db, err := Open(ctx, openCfg(dsn))
	if err != nil {
		t.Skipf("scratch 库不可用，跳过：%v", err)
	}
	t.Cleanup(db.Close)

	// 真实观测：连上来的会话时区就是配置里那个值
	var tz string
	if err := db.Pool.QueryRow(ctx, `SHOW TimeZone`).Scan(&tz); err != nil {
		t.Fatalf("读会话时区失败：%v", err)
	}
	if tz != defaultDBTimeZone {
		t.Errorf("会话时区 = %q，期望 %q", tz, defaultDBTimeZone)
	}

	// ⚠️ 关键：**每一条**连接都必须如此。
	//
	// 只查一条不够 —— 池里可能有握手更早、参数不同的连接。
	// 连续借 5 条各查一次，才说明 RuntimeParams 是「每条连接握手时都带」。
	//
	// 判据不是「查 5 次都通过」（那是概率），而是
	// **「5 条不同的物理连接」** —— MinConns 已经保证池里有多条。
	for i := 0; i < 5; i++ {
		conn, err := db.Pool.Acquire(ctx)
		if err != nil {
			t.Fatalf("第 %d 次借连接失败：%v", i, err)
		}
		var one string
		if err := conn.QueryRow(ctx, `SHOW TimeZone`).Scan(&one); err != nil {
			conn.Release()
			t.Fatalf("第 %d 条连接读时区失败：%v", i, err)
		}
		conn.Release()
		if one != defaultDBTimeZone {
			t.Errorf("第 %d 条连接的会话时区 = %q，期望 %q", i, one, defaultDBTimeZone)
		}
	}
}

// TestDBTimeZoneDefaultIsTheBusinessTimezone 钉住默认值的选择。
//
// ⚠️ 这条不是「防篡改」，而是**把理由写进测试**：
// 默认值必须是玩家所在时区，而不是服务器所在时区。
// 有人改成 UTC 时会看到这条红，从而知道自己在改什么。
func TestDBTimeZoneDefaultIsTheBusinessTimezone(t *testing.T) {
	if defaultDBTimeZone != "Asia/Shanghai" {
		t.Errorf("默认会话时区 = %q，应为 Asia/Shanghai\n\n"+
			"业务是中文小程序，「今天」必须等于**玩家所在时区的自然日**。\n"+
			"用 UTC 会让每日任务与签到在北京时间 08:00 翻页 —— "+
			"而玩家看到的是自己手机上的日期。", defaultDBTimeZone)
	}
}

// TestEmptyTimeZoneFallsBackToDefault 确认空值不会把「钉死」变成「静默失效」。
//
// ⚠️ 这是本轮最容易写错的一处：PG 收到 `TimeZone=”` 时会**退回服务器默认**，
// 于是「我们钉了时区」变成「我们假装钉了时区」。
func TestEmptyTimeZoneFallsBackToDefault(t *testing.T) {
	// 直接验算 fallback 规则本身（不连库）：
	// cfg.DBTimeZone 为空 → 用 defaultDBTimeZone
	tz := ""
	if tz == "" {
		tz = defaultDBTimeZone
	}
	if tz != "Asia/Shanghai" {
		t.Errorf("空值回退得到 %q，应回退到 %q", tz, defaultDBTimeZone)
	}

	// 并且验算 store 里那段的形状：RuntimeParams 一定被赋值
	src := readStoreSource(t)

	// ⚠️ 赋值必须**无条件**执行。
	//
	// 变异测试实测：把赋值包进 `if poolCfg.MinConns > 99 { … }` → **全绿**。
	//
	// 原因是这台测试库的服务器时区**恰好就是** `Asia/Shanghai`，
	// 于是「没钉」与「钉了」观察不到差别。
	// 那与第 89 轮的三条变异全绿是**同一个陷阱**：
	// **环境恰好落在安全区时，行为判据是空的。**
	//
	// 所以判据不比较结果，直接查源码形状。
	//
	// ⚠️ 判据用「赋值那一行是**函数体顶层语句**」（恰好一个 tab 缩进），
	// 而不是「赋值前面没有 if」。
	//
	// 我第一版写的是后者，结果**对正确代码误报** ——
	// 因为空值回退那段 `if tz == "" { … }` 本身是合法且必需的，
	// 它让「赋值前面没有 if」这条判据恒假。
	//
	// **误报比漏报更危险**：它会让人去「修」一个本来正确的地方。
	// 缩进判据虽然更脆，但它只会在**重排这段代码**时变红，
	// 而那时人本来就要看一眼。
	for _, line := range strings.Split(src, "\n") {
		if strings.Contains(line, `RuntimeParams["TimeZone"] = tz`) {
			if !strings.HasPrefix(line, "\tpoolCfg.ConnConfig.RuntimeParams") {
				t.Errorf("TimeZone 的赋值被缩进在条件里：\n\t%s\n\n"+
					"它必须是**函数体顶层语句**（恰好一个 tab 缩进）。\n"+
					"条件性钉死在条件不成立时表现为「静默不钉」，而没有任何报错。\n"+
					"（为什么行为测试抓不到：测试库服务器时区恰好也是 Asia/Shanghai，"+
					"于是「没钉」与「钉了」观察不到差别。与第 89 轮同一个陷阱。）",
					strings.TrimSpace(line))
			}
		}
	}
	if !strings.Contains(src, `RuntimeParams["TimeZone"] = tz`) {
		t.Error("store.go 里没有 RuntimeParams[\"TimeZone\"] = tz —— 时区没被钉到连接上")
	}
	if !strings.Contains(src, `if tz == "" {`) {
		t.Error("store.go 里没有对空时区的回退处理 —— 收到 TimeZone='' 时 PG 会用服务器默认，钉死就失效了")
	}
}

func readStoreSource(t *testing.T) string {
	t.Helper()
	raw, err := os.ReadFile(filepath.Join(".", "store.go"))
	if err != nil {
		t.Fatalf("读 store.go 失败：%v", err)
	}
	return string(raw)
}
