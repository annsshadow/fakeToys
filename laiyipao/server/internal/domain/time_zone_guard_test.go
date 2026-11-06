// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package domain

import (
	"fmt"
	"os"
	"path/filepath"
	"regexp"
	"sort"
	"strings"
	"testing"
)

// R102 前哨（**不开轮记录**）：测试夹具不得把宿主时区带进日期/日界断言。
//
// # 为什么要有这条守卫
//
// R92 把业务日界钉在 `config.BusinessTimeZone`（+08:00），服务端
// `periodStart` 按它算「今天 / 本周」。但测试夹具里仍可能用宿主机
// `time.Local` 算日期：开发机 Local=+08:00 时两边恰好一致，看起来没问题；
// CI 容器 Local=UTC，于是 **UTC 16:00–24:00（北京 00:00–08:00）这 8 小时里
// 夹具与服务端差一天**，测试每天有 1/3 的时间是红的。
//
// 这种红长得像 flaky，最容易被「重跑一次过了」糊过去 —— R101 首次推送
// 就被它红过一次（`TestE2ETasksClaim` 报「任务未完成（0/30）」），
// 同一提交在 40 分钟前还是绿的。
//
// # 规则
//
// 扫描 `laiyipao/server` 下所有 `*_test.go`，外加 `cmd/loadtest`、
// `cmd/playertest` 的 Go 文件（它们被测试/压测当夹具引用）。命中下列
// 依赖宿主时区的写法即违规，除非在 `hostTimeZoneAllowed` 里登记了理由：
//
//   - `time.Local`（含 `time.Date(..., time.Local)`、`.In(time.Local)`）；
//   - `.Truncate(...)`（对时间戳做日界/精度截断，尤其 `time.Now().Truncate`）；
//   - `.Location()`（把 `time.Now()` 的宿主时区带进构造 —— R101 的写法）。
//
// 允许的两类必须登记：
//
//  1. 真实时钟冒烟断言（确实要验证「现在」的）；
//  2. 与被测日界无关的时间工具（例如显式 `.UTC()` 后只为精度截断）。
//
// # 防噪声化
//
// 只扫 Go 代码（注释与字符串先被剥离），并对「扫描文件数 / 命中点数」做
// 下限断言：正则写错、遍历路径坏掉时会直接红，而不是悄悄扫 0 个文件
// 然后「通过」。
//
// 破坏实验：把 `battle_service_test.go` 里构造周日的 `businessLocation`
// 改回 `time.Local` → 本条必须红；改回 → 绿（R102 前的实测记录见台账）。

var hostTimeZonePattern = regexp.MustCompile(`time\.Local|\.Truncate\(|\.Location\(\)`)

// hostTimeZoneAllowed 是**显式登记**的豁免，key = "相对路径|该行代码（去首尾空白）"。
//
// 用「行内容」而不是行号做 key：行号会随其它改动漂移，内容变了才需要重新审。
// 每条都必须是「允许的两类」之一，并在 reason 里写清为什么与日界无关。
var hostTimeZoneAllowed = map[string]string{
	"laiyipao/server/internal/service/admin_redeem_codes_batch_test.go|expiresAt := time.Now().Add(24 * time.Hour).UTC().Truncate(time.Second)": "显式 .UTC() 后只为把过期时间戳截到秒，不参与日界/日期断言",
	"laiyipao/server/internal/service/admin_service_test.go|exp := time.Now().UTC().Truncate(time.Microsecond).Add(24 * time.Hour)":             "显式 .UTC() 后只为对齐 timestamptz 微秒精度，不参与日界/日期断言",
	"laiyipao/server/internal/service/admin_timestamps_r133_test.go|stored := raw.Truncate(time.Microsecond)":                                   "夹具时刻显式用 +08:00 构造，只为对齐 timestamptz 微秒精度，不参与日界/日期断言",
}

const (
	// minScannedFiles 是当前仓库里该范围 Go 文件数的下限（实际 ~400）。
	// 遍历/过滤坏掉时会掉到 0，必须红而不是「通过」。
	minScannedFiles = 200
	// minMatched 是当前允许清单里的命中数下限。正则失效时会是 0。
	minMatched = 2
)

// TestFixturesDoNotDependOnHostTimeZone 是全仓静态守卫（见文件头注释）。
func TestFixturesDoNotDependOnHostTimeZone(t *testing.T) {
	root := findRepoRoot(t)

	var (
		scanned   int
		matched   int
		offenders []string
	)
	used := map[string]bool{}

	walkTracked(t, root, func(rel, path string) {
		rel = filepath.ToSlash(rel)
		if !strings.HasPrefix(rel, "laiyipao/server/") || !strings.HasSuffix(rel, ".go") {
			return
		}
		inScope := strings.HasSuffix(rel, "_test.go") ||
			strings.HasPrefix(rel, "laiyipao/server/cmd/loadtest/") ||
			strings.HasPrefix(rel, "laiyipao/server/cmd/playertest/")
		if !inScope {
			return
		}
		scanned++

		b, err := os.ReadFile(path)
		if err != nil {
			t.Fatalf("读 %s 失败：%v", rel, err)
		}
		code := stripGoCommentsAndStrings(string(b))
		for i, line := range strings.Split(code, "\n") {
			if !hostTimeZonePattern.MatchString(line) {
				continue
			}
			matched++
			trimmed := strings.TrimSpace(line)
			key := rel + "|" + trimmed
			if _, ok := hostTimeZoneAllowed[key]; ok {
				used[key] = true
				continue
			}
			offenders = append(offenders,
				fmt.Sprintf("%s:%d  %s", rel, i+1, trimmed))
		}
	})

	if scanned < minScannedFiles {
		t.Fatalf("只扫描到 %d 个 Go 文件（下限 %d）—— 遍历路径或过滤条件坏了，这条守卫**没有真正生效**",
			scanned, minScannedFiles)
	}
	if matched < minMatched {
		t.Fatalf("只命中 %d 处宿主时区写法（下限 %d）—— 正则可能已失效，守卫**没有真正生效**",
			matched, minMatched)
	}

	var stale []string
	for k := range hostTimeZoneAllowed {
		if !used[k] {
			stale = append(stale, k)
		}
	}
	sort.Strings(stale)
	if len(stale) > 0 {
		t.Fatalf("允许清单里有 %d 条已失效（行内容改了或文件没了），必须同步更新：\n%s",
			len(stale), strings.Join(stale, "\n"))
	}

	if len(offenders) > 0 {
		t.Fatalf(`测试夹具不得用宿主时区参与日期/日界断言（命中 %d 处）：

%s

修法：
  - 业务日期/周期起点一律按 config.BusinessTimeZone（service 包里是 businessLocation），
    不要用 time.Local / time.Now().Location() / 宿主 time.Now().Truncate；
  - 确属「真实时钟冒烟」或「与日界无关的时间工具」（如显式 .UTC() 后只为精度截断），
    把该行登记到 hostTimeZoneAllowed 并写清理由。`,
			len(offenders), strings.Join(offenders, "\n"))
	}
}

// stripGoCommentsAndStrings 把注释与字符串/字符字面量替换为空格，保留换行，
// 这样逐行匹配时只看**真正会执行的代码**（注释里的示例、错误文案里的
// "time.Local" 不会造成假红）。
func stripGoCommentsAndStrings(src string) string {
	var b strings.Builder
	b.Grow(len(src))
	n := len(src)
	for i := 0; i < n; {
		c := src[i]
		switch {
		case c == '/' && i+1 < n && src[i+1] == '/':
			for i < n && src[i] != '\n' {
				b.WriteByte(' ')
				i++
			}
		case c == '/' && i+1 < n && src[i+1] == '*':
			b.WriteString("  ")
			i += 2
			for i+1 < n && !(src[i] == '*' && src[i+1] == '/') {
				if src[i] == '\n' {
					b.WriteByte('\n')
				} else {
					b.WriteByte(' ')
				}
				i++
			}
			if i+1 < n {
				b.WriteString("  ")
				i += 2
			}
		case c == '"' || c == '\'' || c == '`':
			quote := c
			b.WriteByte(' ')
			i++
			for i < n {
				if quote != '`' && src[i] == '\\' && i+1 < n {
					b.WriteString("  ")
					i += 2
					continue
				}
				if src[i] == quote {
					b.WriteByte(' ')
					i++
					break
				}
				if src[i] == '\n' {
					b.WriteByte('\n')
				} else {
					b.WriteByte(' ')
				}
				i++
			}
		default:
			b.WriteByte(c)
			i++
		}
	}
	return b.String()
}
