package service

// 读本包源码的小工具，供「守卫查询文本本身」的用例使用。
//
// ## 为什么需要读源码而不是只测行为
//
// 有些约束守的是**代码里不该出现某种写法**，而不是「运行结果对不对」。
// 比如「查询不能用 `WHERE col <> '{}'`」——
// 这条约束在数据里没有 null 时，行为上完全看不出差别；
// 只有当某天数据里出现 null 才会暴露，而那时已经在漏数据了。
//
// 把它做成行为测试的代价是：必须先造出脏数据才能测，
// 于是测试依赖数据库状态、依赖迁移是否已应用 ——
// 本项目已经多次因此误判（「测试库没迁移」）。
//
// 读源码的判据不依赖任何外部状态，因此在任何 CI 里都能跑。
//
// ⚠️ 它的局限也要说清楚：**它只认字面量**。
// 如果有人把 `<> '{}'` 换成等价的其它写法（比如 `!= '{}'`），
// 守卫不会红。所以它是「防手滑」而不是「形式化验证」。

import (
	"os"
	"path/filepath"
	"strings"
)

// readServiceSource 读本包的一个源文件（跳过 _test.go），并**剥掉注释**。
//
// 找到时返回**去注释后的代码**。
//
// ## 为什么必须剥注释 —— 这是「文本搜索会误判」的典型
//
// 第一版直接返回原文，结果守卫匹配到了**我自己写的解释性注释**：
// 那条注释为了说明「为什么不能用 `WHERE col <> '{}'`」，
// 恰恰把被禁的写法写在了注释里。
//
// 于是每次修好查询、留下一段解释，守卫就变红 ——
// 而代码其实是对的。守卫生成了「改对代码会被罚」的激励。
//
// 剥法：删掉**整行都是注释**的行（`//` / `*` / `/*` 开头，或以 `*/` 结尾）。
// 真实代码行不会以 `//` 或 `*` 开头，所以这个判据足够精确，
// 且不会误伤字符串里的 `//`（例如 URL）—— 那正是朴素正则会踩的坑。
//
// ## 局限（仍然要说清楚）
//
// 1. **只认字面量**：把 `<> '{}'` 换成等价的 `!= '{}'`，守卫不会红。
// 2. 块注释只处理了「整行都是注释」的情况，行尾块注释 `code /* x */` 不会被剥。
// 它是「防手滑」而不是形式化验证。
//
// 找不到时返回错误而不是 panic —— 由调用方决定是 Skip 还是 Fail。
func readServiceSource(name string) (string, error) {
	var raw string
	found := false
	for _, base := range []string{".", "internal/service", filepath.Join("..", "service")} {
		if b, err := os.ReadFile(filepath.Join(base, name)); err == nil {
			raw = string(b)
			found = true
			break
		}
	}
	if !found {
		return "", os.ErrNotExist
	}
	return stripLineComments(raw), nil
}

// stripLineComments 删掉整行都是注释的行。
func stripLineComments(src string) string {
	lines := strings.Split(src, "\n")
	out := make([]string, 0, len(lines))
	for _, l := range lines {
		t := strings.TrimSpace(l)
		if strings.HasPrefix(t, "//") || strings.HasPrefix(t, "/*") ||
			strings.HasPrefix(t, "*") || strings.HasSuffix(t, "*/") {
			continue
		}
		out = append(out, l)
	}
	return strings.Join(out, "\n")
}

// contains 是 strings.Contains 的本地别名，少打几个字。
func contains(s, sub string) bool { return strings.Contains(s, sub) }
