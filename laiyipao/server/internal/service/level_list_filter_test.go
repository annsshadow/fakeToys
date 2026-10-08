package service

import (
	"context"
	"strconv"
	"strings"
	"testing"
)

// 第 111 轮：AdminListLevels 的 chapter / keyword 过滤**真的生效**。
//
// 修前这两个参数根本不在签名里——handler 收了却丢弃，
// 服务端永远回全量 100 关。
//
// 判据（全落在能直接观测的那一层）：
//   - 不过滤      → 行数 == 全表行数
//   - 按章节过滤  → 只剩该章，且行数 == 全表里该章的行数（不多不少）
//   - 按关键词    → 命中数 == 全表里 LIKE 命中的行数
//   - LIKE 转义   → 搜 "%" / "_" 这种通配符字面量时，
//     必须按**字面**匹配，不能把通配符当通配符（否则全表命中）
func TestAdminListLevelsFiltersActuallyFilter(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	full, _, err := ts.AdminListLevels(ctx, 0, "")
	if err != nil {
		t.Fatalf("全量列表失败：%v", err)
	}
	fullN := ts.countLevels(t, ctx, "", "")
	if len(full) != fullN {
		t.Fatalf("无过滤应回全表 %d 行，实际 %d", fullN, len(full))
	}

	// 按章节过滤：第 3 章（冻原）
	for _, ch := range []int{3, 6} {
		got, _, err := ts.AdminListLevels(ctx, ch, "")
		if err != nil {
			t.Fatalf("章节 %d 过滤失败：%v", ch, err)
		}
		want := ts.countLevels(t, ctx, "chapter", strconv.Itoa(ch))
		if len(got) != want {
			t.Errorf("第 %d 章应回 %d 行，实际 %d", ch, want, len(got))
		}
		for _, m := range got {
			if c, _ := m["chapter"].(int); c != ch {
				t.Errorf("第 %d 章过滤混进了第 %d 章的行", ch, c)
			}
		}
	}

	// 按关键词过滤：关卡名形如「冻原 · 41」，用「· 4」只能命中 40s
	kwGot, _, err := ts.AdminListLevels(ctx, 0, "· 4")
	if err != nil {
		t.Fatalf("关键词过滤失败：%v", err)
	}
	kwWant := ts.countLevels(t, ctx, "name_like", "· 4")
	if len(kwGot) != kwWant {
		t.Errorf("关键词「· 4」应命中 %d 行，实际 %d", kwWant, len(kwGot))
	}

	// LIKE 转义：搜通配符字面量「_」，必须 0 命中（关卡名里没有下划线）。
	// 若没转义，「__」会被当通配符 → 全表命中。
	underscore, _, err := ts.AdminListLevels(ctx, 0, "_")
	if err != nil {
		t.Fatalf("关键词过滤失败：%v", err)
	}
	if len(underscore) != 0 {
		t.Errorf("搜通配符字面量「_」应 0 命中（关卡名无下划线），实际 %d —— LIKE 没转义，通配符被当真了", len(underscore))
	}
	pct, _, err := ts.AdminListLevels(ctx, 0, "%")
	if err != nil {
		t.Fatalf("关键词过滤失败：%v", err)
	}
	if len(pct) != 0 {
		t.Errorf("搜通配符字面量「%%」应 0 命中，实际 %d —— LIKE 没转义", len(pct))
	}
}

// countLevels 直接查库数行数（full 或某章或某 LIKE 命中数），作为判据的地面真值。
func (ts *testService) countLevels(t *testing.T, ctx context.Context, mode, val string) int {
	t.Helper()
	var n int
	switch mode {
	case "":
		if err := ts.pool.QueryRow(ctx, `SELECT COUNT(*) FROM levels`).Scan(&n); err != nil {
			t.Fatalf("countLevels() 失败：%v", err)
		}
	case "chapter":
		q := `SELECT COUNT(*) FROM levels WHERE chapter = ` + val
		if err := ts.pool.QueryRow(ctx, q).Scan(&n); err != nil {
			t.Fatalf("countLevels(chapter=%s) 失败：%v", val, err)
		}
	case "name_like":
		// 复现 AdminListLevels 完全相同的转义，作为「该命中多少」的独立真值
		esc := strings.NewReplacer(`\`, `\\`, `%`, `\%`, `_`, `\_`).Replace(val)
		if err := ts.pool.QueryRow(ctx,
			`SELECT COUNT(*) FROM levels WHERE name LIKE $1 ESCAPE '\'::char`,
			"%"+esc+"%").Scan(&n); err != nil {
			t.Fatalf("countLevels(name_like) 失败：%v", err)
		}
	}
	return n
}
