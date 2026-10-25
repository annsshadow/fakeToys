// 后台玩家详情是**读**端点，不得触发体力 regen 的写（LY-GAP-2 回归钉）。
//
// 2026-10-25（R3）前的形态：AdminUserDetail 走 LoadWallet，而 LoadWallet 会
// 顺手做 `UPDATE user_wallets SET energy = ..., energy_updated_at = ...`。
// 读端点带写副作用有三个具体坏处：
//
//  1. `readonly` 角色的运营账号本应完全无写权（requireWritable 的语义），
//     但这个写发生在 GET 里，绕过了那层门——纸面角色缺陷换了个入口复发；
//  2. 只读副本/读路由下会失败（写语句打不出去）；
//  3. 审计面「谁改了什么」出现没人主动触发的写入。
//
// 修法：改用 LoadWalletReadOnly（同一公式投影、不落库）。
// 本测试是静态钉——只要有人把详情页改回 LoadWallet 就红。
package service

import (
	"os"
	"strings"
	"testing"
)

func TestAdminUserDetailNeverWritesWallet(t *testing.T) {
	src, err := os.ReadFile("adminuserdetail.go")
	if err != nil {
		t.Fatalf("读取 adminuserdetail.go 失败：%v", err)
	}
	text := string(src)

	// ① 不得调用会写库的 LoadWallet
	if strings.Contains(text, "s.LoadWallet(") {
		t.Error("后台详情又走回 s.LoadWallet（会写 user_wallets 做体力恢复）——" +
			"读端点不得写库，请用 s.LoadWalletReadOnly")
	}
	// ② 必须用只读版（防止改了个别的写法绕开 ①）
	if !strings.Contains(text, "s.LoadWalletReadOnly(") {
		t.Error("后台详情未使用 s.LoadWalletReadOnly——钱包段应走只读投影")
	}
}
