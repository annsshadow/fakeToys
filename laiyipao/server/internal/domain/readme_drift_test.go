package domain

// README 里「迁移数量」这个数字的守卫。
//
// ## 为什么需要这条
//
// README L34 那一行原本写着「建表（8 个 goose 迁移，49 张业务表 + 92 索引）」。
// 实测是 **11 个迁移 / 48 张表 / 93 条索引** —— 三个数字全错。
//
// 而且「8 个」在我这轮动手**之前**就已经错了（当时是 9 个）。
// 也就是说这个数字从加迁移起就没人核对过，漂了 3 次没人发现。
//
// 这类注释最麻烦的地方在于：它读起来像是有人量过，于是没人再去量。
//
// ## 为什么只守「迁移数」这一项
//
// 迁移数**不需要数据库**：直接从**嵌入的迁移集**数 `.sql` 文件。
// 于是这条守卫在任何 CI 里都能跑 —— 而需要连库的守卫会在
// 「测试库没迁移」时静默跳过（本项目反复栽这个）。
//
// 表数与索引数要连库才能数，所以**不**在这里断言。
// 但迁移数一变，这两个数几乎必然变 —— 于是这条守卫会在人
// 不得不重新读那一行的时候提醒他。这是有意的间接保护。
//
// ## 局限（明说）
//
// 判据是**字面量匹配**：只认「N 个 goose 迁移」这个写法。
// 有人把这句话换个说法（改成「迁移共 N 条」）就会绕过。
// 它防的是「加迁移忘了改数字」，不是「文档格式变化」。

import (
	"os"
	"regexp"
	"strconv"
	"strings"
	"testing"

	"github.com/laiyipao/server/migrations"
)

// readmeText 读仓库根的 README。
//
// 找不到时返回错误，由调用方决定 Skip 还是 Fail ——
// **不能**默默当成「通过」，那是本项目最讨厌的形状。
func readmeText(t *testing.T) string {
	t.Helper()
	for _, p := range []string{
		"../../../README.md",
		"../../README.md",
		"../README.md",
		"README.md",
	} {
		if b, err := os.ReadFile(p); err == nil {
			return string(b)
		}
	}
	t.Fatal("找不到 README.md —— 这条守卫未执行，不是「通过了」")
	return ""
}

// migrationCount 数嵌入的迁移文件数。
func migrationCount(t *testing.T) int {
	t.Helper()
	entries, err := migrations.FS.ReadDir(".")
	if err != nil {
		t.Fatalf("读迁移目录失败：%v", err)
	}
	n := 0
	for _, e := range entries {
		if !e.IsDir() && strings.HasSuffix(e.Name(), ".sql") {
			n++
		}
	}
	if n == 0 {
		t.Fatal("迁移文件数为 0 —— 嵌入是否失效？")
	}
	return n
}

func TestReadmeMigrationCountMatchesEmbed(t *testing.T) {
	readme := readmeText(t)
	want := migrationCount(t)

	// 匹配「N 个 goose 迁移」
	re := regexp.MustCompile(`(\d+)\s*个\s*goose\s*迁移`)
	m := re.FindStringSubmatch(readme)
	if m == nil {
		t.Fatalf("README 里找不到「N 个 goose 迁移」这种写法 —— " +
			"本守卫未执行，不是「通过了」。若只是改了措辞，请同步改这条正则。")
	}
	got, err := strconv.Atoi(m[1])
	if err != nil {
		t.Fatalf("解析数字 %q 失败：%v", m[1], err)
	}
	if got != want {
		t.Errorf("README 写「%d 个 goose 迁移」，实际嵌入的迁移文件有 %d 个\n"+
			"（加迁移时忘了改这一行 —— 它已经漂了 3 次没人发现）", got, want)
	}
}
