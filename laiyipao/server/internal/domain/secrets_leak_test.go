package domain

// 密钥泄漏守卫：仓库里**不得**出现真值。
//
// # 为什么需要它
//
// 这个仓库是**开源**的（GitHub 上可见），而部署又用��真域名 / 真 IP /
// 真口令。仓库政策写在 `laiyipao/deploy/README.md`：
//
//	「仓库里**不含**任何真实域名 / IP / 口令。」
//
// 现实是这条政策**靠人记着**：2026-10-21 审计时发现，
// 测试服 IP、真实域名、SSH 私钥路径出现在**两份工作日志**和
// `admin/vite.config.ts` 的注释里 —— 而同一份日志还写着
// 「真实域名只在本地 .local.keys（不入库）」，自相矛盾。
//
// 泄露的**后果**分级：
//
//   - 口令 / 密钥 / 令牌 → 必须轮换，属于事故；
//   - 域名 / IP / 私钥路径 → 是**定位信息**，让人知道该往哪打，
//     但单独不足以进门（进门的钥匙是口令）。
//
// 所以这条守卫盯两类东西：真值（从 .local.keys 读）与定位信息（通用规则）。
//
// # 判据
//
//  1. **真值**：从 `laiyipao/deploy/.local.keys`（不入库）读出
//     DOMAIN / ADMIN_PASS / JWT_SECRET / DB_DSN（并从 DSN 里取出库口令），
//     任一出现在已跟踪文件里 → 红。
//  2. **定位信息**（不依赖 .local.keys，任何时候都生效）：
//     - SSH **私钥文件路径**（`.ssh/` 目录 + 私钥文件名 + `.pem` 后缀）——
//       本仓没有任何合法用途；
//     - `root@` + IP 的**按 IP root 登录** —— 同上。
//
// ⚠️ 这条文件**自己也在这两条规则的射程内**：所以注释里不能写
// 「`.ssh/` 后面跟一个真实的私钥文件名」这样的例子 ——
// 那会让守卫在自己的文件上变红（第一次写这守卫时正是如此，
// 它把本文件里那个举例用的文件名判了出来）。
//
// # 为什么第 2 条不用「禁止一切公网 IP」
//
// 试过：仓库里有大量形如 `1.2.3.4` 的**版本号**与 CGNAT 段
// （`100.64.0.1` 等），一条「禁止公网 IP」的规则会立刻变成噪声，
// 而噪声守卫会被人加白名单绕过 —— 那还不如不做。
//
// # Skip 语义
//
// `.local.keys` 不存在（CI 上就是这样）→ 第 1 条**显式 Skip**，
// 不是「通过」。第 2 条不受影响，照常执行。

import (
	"os"
	"path/filepath"
	"regexp"
	"strings"
	"testing"
)

// findRepoRoot 从当前目录向上找带 .git 的目录。
func findRepoRoot(t *testing.T) string {
	t.Helper()
	dir, err := os.Getwd()
	if err != nil {
		t.Fatalf("取工作目录失败：%v", err)
	}
	for i := 0; i < 8; i++ {
		if _, err := os.Stat(filepath.Join(dir, ".git")); err == nil {
			return dir
		}
		parent := filepath.Dir(dir)
		if parent == dir {
			break
		}
		dir = parent
	}
	t.Skip("找不到仓库根（含 .git 的目录）—— 这条守卫**没有执行**，不是「通过了」")
	return ""
}

// skipDirs 是不扫描的目录：依赖树与构建产物既大又是二进制。
var skipDirs = map[string]bool{
	".git": true, "node_modules": true, "dist": true, "build": true,
	"coverage": true, "coverage-lcov": true, "target": true,
	".next": true, ".nuxt": true, ".pnpm-store": true, ".venv": true,
}

// skipFiles 是不扫描的文件：密钥的**来源**本身（按设计含真值），
// 以及构建产物目录里的东西。
var skipFiles = map[string]bool{
	filepath.Join("laiyipao", "deploy", ".local.keys"):       true,
	filepath.Join("laiyipao", "deploy", "remote-config.env"): true,
}

// isProbablyText 判断是不是文本文件：前 8KB 里有 NUL 字节就不是。
func isProbablyText(path string) bool {
	f, err := os.Open(path)
	if err != nil {
		return false
	}
	defer f.Close()
	buf := make([]byte, 8192)
	n, _ := f.Read(buf)
	return !strings.ContainsRune(string(buf[:n]), 0)
}

// walkTracked 遍历仓库里的文件，跳过依赖树/产物/密钥来源。
func walkTracked(t *testing.T, root string, visit func(rel string, path string)) {
	t.Helper()
	err := filepath.Walk(root, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return nil
		}
		rel, rerr := filepath.Rel(root, path)
		if rerr != nil {
			return nil
		}
		if info.IsDir() {
			if skipDirs[info.Name()] {
				return filepath.SkipDir
			}
			return nil
		}
		if skipFiles[rel] {
			return nil
		}
		visit(rel, path)
		return nil
	})
	if err != nil {
		t.Fatalf("遍历仓库失败：%v", err)
	}
}

// localSecrets 从 .local.keys 读出真值（键名 → 值）。
//
// 解析是「取等号后整行」，不做任何转换 —— 真值原样拿来做子串比对。
func localSecrets(t *testing.T, root string) map[string]string {
	t.Helper()
	out := map[string]string{}
	p := filepath.Join(root, "laiyipao", "deploy", ".local.keys")
	b, err := os.ReadFile(p)
	if err != nil {
		t.Skipf("读不到 %s —— 真值扫描这条守卫**没有执行**，不是「通过了」", p)
	}
	for _, line := range strings.Split(string(b), "\n") {
		line = strings.TrimSpace(line)
		if line == "" || strings.HasPrefix(line, "#") {
			continue
		}
		k, v, ok := strings.Cut(line, "=")
		if !ok {
			continue
		}
		k, v = strings.TrimSpace(k), strings.TrimSpace(v)
		if v == "" {
			continue
		}
		out[k] = v
		// DSN 里还嵌着库口令 —— 它也是真值，且是最容易单独泄露的那个。
		if k == "DB_DSN" {
			if at := strings.LastIndex(v, "@"); at > 0 {
				cred := v[:at]
				if colon := strings.Index(cred, ":"); colon >= 0 {
					if pw := cred[colon+1:]; pw != "" {
						out["DB_PASSWORD"] = pw
					}
				}
			}
		}
	}
	if len(out) == 0 {
		t.Skip(".local.keys 里没解析出任何真值 —— 这条守卫**没有执行**，不是「通过了」")
	}
	return out
}

// TestNoRealSecretsInRepo 守「真值不进仓库」。
func TestNoRealSecretsInRepo(t *testing.T) {
	root := findRepoRoot(t)
	secrets := localSecrets(t, root)

	var hits []string
	walkTracked(t, root, func(rel, path string) {
		if !isProbablyText(path) {
			return
		}
		b, err := os.ReadFile(path)
		if err != nil {
			return
		}
		content := string(b)
		for name, val := range secrets {
			// 太短的值没有比对意义（子串会到处命中）。
			if len(val) < 8 {
				continue
			}
			if strings.Contains(content, val) {
				hits = append(hits, rel+" ← 含有 .local.keys 的 "+name)
			}
		}
	})
	if len(hits) > 0 {
		t.Fatalf("仓库里有 %d 处真值（口令/域名/DSN 等）：\n  %s\n"+
			"这些值来自 laiyipao/deploy/.local.keys（不入库）。"+
			"一旦推上去就是**公开**的，必须立刻轮换并清干净。",
			len(hits), strings.Join(hits, "\n  "))
	}
}

// 定位信息：私钥路径与按 IP 的 root 登录。
//
// 这两条在本仓**没有任何合法用途**，所以可以无条件禁止 ——
// 噪声为零的守卫才有人留着。
var (
	privateKeyPathRe = regexp.MustCompile(`\.ssh/[A-Za-z0-9_.-]+\.pem`)
	rootLoginIPRe    = regexp.MustCompile(`root@[0-9]{1,3}\.[0-9]{1,3}`)
)

// TestNoInfraIdentifiersInRepo 守「定位信息不进仓库」。
func TestNoInfraIdentifiersInRepo(t *testing.T) {
	root := findRepoRoot(t)

	var hits []string
	walkTracked(t, root, func(rel, path string) {
		if !isProbablyText(path) {
			return
		}
		b, err := os.ReadFile(path)
		if err != nil {
			return
		}
		content := string(b)
		if m := privateKeyPathRe.FindString(content); m != "" {
			hits = append(hits, rel+" ← 私钥路径 "+m)
		}
		if m := rootLoginIPRe.FindString(content); m != "" {
			hits = append(hits, rel+" ← root 登录 "+m)
		}
	})
	if len(hits) > 0 {
		t.Fatalf("仓库里有 %d 处定位信息（私钥路径 / root@IP）：\n  %s\n"+
			"它们单独不足以进门（进门的钥匙是口令），但会把「该往哪打」写清楚。\n"+
			"请改成占位符：<YOUR_TEST_DOMAIN> / <TEST_HOST> / ~/.ssh/<KEY>.pem。",
			len(hits), strings.Join(hits, "\n  "))
	}
}
