package tygen

import (
	"fmt"
	"os"
	"path/filepath"
	"strings"
)

// ModulePath is the Go module path of the server.
const ModulePath = "github.com/laiyipao/server"

// RepoPaths 定位「server 模块根」与「admin 生成的 TS 文件」。
//
// # 为什么不写相对路径
//
// 相对路径（`../../admin/...`）在 `go run` / `go test` / IDE 里工作目录各不相同，
// 早先就因此把文件写到了 `fakeToys/admin/`（仓库根下面多出一个目录）。
// 正确做法是**从 `go.mod` 反推**：找到模块根，admin 就在它旁边。
type RepoPaths struct {
	// ServerRoot 是 server 模块根（里面有 go.mod）。
	ServerRoot string
	// AdminTypes 是生成物的路径。
	AdminTypes string
}

// FindRepoPaths 从 start 向上找 go.mod，返回定位结果。
//
// start 传空表示从当前工作目录开始。
func FindRepoPaths(start string) (RepoPaths, error) {
	dir := start
	if dir == "" {
		wd, err := os.Getwd()
		if err != nil {
			return RepoPaths{}, err
		}
		dir = wd
	}
	abs, err := filepath.Abs(dir)
	if err != nil {
		return RepoPaths{}, err
	}
	for {
		gomod := filepath.Join(abs, "go.mod")
		if data, err := os.ReadFile(gomod); err == nil {
			if strings.Contains(string(data), "module "+ModulePath) {
				return RepoPaths{
					ServerRoot: abs,
					AdminTypes: filepath.Join(abs, "..", "admin", "src", "api", "types.gen.ts"),
				}, nil
			}
		}
		parent := filepath.Dir(abs)
		if parent == abs {
			return RepoPaths{}, fmt.Errorf("tygen: 找不到 module %s 的 go.mod（从 %s 向上找）", ModulePath, dir)
		}
		abs = parent
	}
}
