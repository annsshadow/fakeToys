// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

import (
	"os"
	"path/filepath"
	"strconv"
	"strings"
	"testing"
)

// R151 源码契约：`synchronous_commit` 只允许出现在 battle token 签发的那一个事务里。
//
// 本轮（2026-11-07）没有启用它 —— token INSERT 与体力/复活币扣减在同一个
// `StartBattle` 事务（game.go 的 `s.DB.Tx`），把它们一起关掉会违反
// 「不许涉及钱包」的硬约束；拆成两个事务又不会减少 fsync（钱包那一笔仍是 on）。
// 台账 R151 记录了这次裁决。守卫先按「零出现」钉死：任何人把开关搬到别处
// （包括结算、掉落、战报、排行榜、或全局 DSN options），都必须先改这条契约。
//
// 扫描范围：仓库源码/配置（跳过依赖、构建产物与文档）。文档与台账允许讨论该字符串。
var synchronousCommitScannedExt = map[string]bool{
	".go": true, ".sql": true, ".sh": true, ".yml": true, ".yaml": true,
	".ts": true, ".js": true, ".vue": true, ".rs": true, ".toml": true,
}

var synchronousCommitSkippedDirs = map[string]bool{
	".git": true, "node_modules": true, "target": true, "dist": true,
	"build": true, "coverage": true, ".workbuddy": true,
}

// findSynchronousCommit 返回命中位置（文件路径:行号），供正向与反向用例共用。
func findSynchronousCommit(t *testing.T, root string) []string {
	t.Helper()
	var hits []string
	err := filepath.WalkDir(root, func(path string, d os.DirEntry, err error) error {
		if err != nil {
			return err
		}
		if d.IsDir() {
			if synchronousCommitSkippedDirs[d.Name()] {
				return filepath.SkipDir
			}
			return nil
		}
		// 本守卫自身在注释与样本里必然含目标字符串，不参与自检。
		if d.Name() == "synchronous_commit_contract_test.go" {
			return nil
		}
		if !synchronousCommitScannedExt[strings.ToLower(filepath.Ext(path))] {
			return nil
		}
		raw, err := os.ReadFile(path)
		if err != nil {
			return err
		}
		for i, line := range strings.Split(string(raw), "\n") {
			trimmed := strings.TrimSpace(line)
			// 注释行不参与：本仓大量注释会讨论这个开关（例如“synchronous_commit 的成本”），
			// 那不是可执行语句。行注释与块注释续行都从这里跳过。
			if strings.HasPrefix(trimmed, "//") || strings.HasPrefix(trimmed, "*") ||
				strings.HasPrefix(trimmed, "/*") {
				continue
			}
			lower := strings.ToLower(line)
			if !strings.Contains(lower, "synchronous_commit") {
				continue
			}
			// 只抓真正的 SET 语句，避免把普通注释/分段名（“synchronous_commit 的成本全在这里”）
			// 误报成开关。两种写法：`synchronous_commit = off` / `... synchronous_commit TO off`。
			if !strings.Contains(lower, "=") && !strings.Contains(lower, " to ") {
				continue
			}
			hits = append(hits, filepath.ToSlash(path)+":"+strconv.Itoa(i+1))
		}
		return nil
	})
	if err != nil {
		t.Fatalf("扫描 %s 失败：%v", root, err)
	}
	return hits
}

// TestSynchronousCommitOnlyOnBattleTokenInsert 是全仓守卫：
// 当前口径下 `synchronous_commit` 必须为 0 处；若将来按新口径启用，
// 只允许出现在 battle token 签发事务，并在台账 R151 里同步改写判据。
func TestSynchronousCommitOnlyOnBattleTokenInsert(t *testing.T) {
	// go test 的 cwd 是包目录：laiyipao/server/internal/service。
	root := filepath.Join("..", "..", "..", "..")
	hits := findSynchronousCommit(t, root)
	// 唯一白名单：本地压测灌数脚本。它只用于建 1M 行的 perf 库（不进生产、
	// 不承接线上事务），R65 起就靠这一行把灌数时间压下来。
	var unexpected []string
	for _, h := range hits {
		if strings.Contains(h, "laiyipao/server/scripts/perf_seed.sql:") {
			continue
		}
		unexpected = append(unexpected, h)
	}
	if len(unexpected) > 0 {
		t.Fatalf("R151 契约被破坏：`synchronous_commit` 只允许出现在 battle token 签发事务"+
			"（当前未启用，见台账 R151；本地 perf_seed 脚本在白名单内）。命中：\n%s",
			strings.Join(unexpected, "\n"))
	}
}

// TestSynchronousCommitScannerActuallyDetects 是守卫的正向对照：
// 证明扫描器不是恒绿 —— 给一份含目标字符串的样本，必须能报出来。
func TestSynchronousCommitScannerActuallyDetects(t *testing.T) {
	dir := t.TempDir()
	bad := filepath.Join(dir, "bad.go")
	if err := os.WriteFile(bad, []byte("func x() { /* SET LOCAL synchronous_commit = off */ }\n"), 0o600); err != nil {
		t.Fatalf("写样本失败：%v", err)
	}
	hits := findSynchronousCommit(t, dir)
	if len(hits) != 1 || !strings.Contains(hits[0], "bad.go:1") {
		t.Fatalf("扫描器未命中样本（恒绿的守卫等于没有）：%v", hits)
	}
}
