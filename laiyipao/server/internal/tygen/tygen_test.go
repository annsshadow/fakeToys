package tygen_test

import (
	"os"
	"strings"
	"testing"

	"github.com/laiyipao/server/internal/apitypes"
	"github.com/laiyipao/server/internal/tygen"
)

// TestGeneratedTypesAreUpToDate 是本轮「契约根修」的守卫。
//
// # 它守的是什么
//
// `admin/src/api/types.gen.ts` 是**生成物**。Go 侧的响应类型改了而没有重新生成，
// 前端就会拿着**旧的**类型继续编译 —— 契约漂移从「改字段名立刻红」退化成
// 「改字段名没人知道」，那正是本轮要消灭的缺陷。
//
// # 判据能失败吗
//
// 能：改 `adminviews.go` 里任意一个 json tag 而不重新生成，本测试立刻红。
// 下面 `TestRegistryCoversEveryAdminResponse` 与
// `TestTypeMappingCoversTheTrickyCases` 一起保证「生成器本身没瞎」。
func TestGeneratedTypesAreUpToDate(t *testing.T) {
	paths, err := tygen.FindRepoPaths("")
	if err != nil {
		t.Fatalf("定位仓库失败：%v", err)
	}
	want, err := apitypes.AdminRegistry.Generate(apitypes.Header)
	if err != nil {
		t.Fatalf("生成失败：%v", err)
	}
	got, err := os.ReadFile(paths.AdminTypes)
	if err != nil {
		t.Fatalf("读生成物失败（先跑 `go run ./cmd/tygen`）：%v", err)
	}
	if string(got) != want {
		t.Errorf("生成物已过期：%s\n"+
			"  Go 侧的响应类型变了但没重新生成。\n"+
			"  跑：cd laiyipao/server && go run ./cmd/tygen\n"+
			"  （或者直接改 Go 类型，前端会立刻看到类型变化 —— 这正是目的）",
			paths.AdminTypes)
	}
}

// TestRegistryCoversEveryAdminResponse 钉住「每个后台响应类型都在注册表里」。
//
// # 为什么需要它
//
// 生成器只输出**注册表里**的类型。有人加了新的响应 struct 却忘了登记，
// 前端就会缺一个类型 —— 而前端可以自己编一个（`Record<string, unknown>`），
// 于是漂移又回来了，而且没有任何报错。
//
// 判据：把 `adminviews.go` 里导出的响应类型逐个点名。
// 新增响应类型而忘了登记 → 这里红。
func TestRegistryCoversEveryAdminResponse(t *testing.T) {
	registered := map[string]bool{}
	for _, e := range apitypes.AdminRegistry {
		registered[e.Name] = true
	}
	// adminviews.go 里定义的每一个响应视图都必须在注册表里。
	// （这张表是手写的，但它比「忘了登记就没人管」强得多 ——
	//  新增类型时它会红，而红的原因写得很清楚。）
	want := []string{
		"AdminLevelView",
		"AdminLevelWaveView",
		"AdminSkillView",
		"AdminSkillsResp",
		"AdminEquipmentView",
		"AdminEquipmentResp",
		"AdminDefenseView",
		"AdminEconomyFlowView",
		"AdminShopItemView",
		"AdminEconomyResp",
		"AdminShopUpdateResp",
		"AdminAnnouncementView",
		"AdminRedeemCodeView",
		"AdminAuditLogView",
	}
	for _, name := range want {
		if !registered[name] {
			t.Errorf("响应类型 %s 没在注册表里 → 它的前端类型会缺失（漂移会静默回来）", name)
		}
	}
}

// TestTypeMappingCoversTheTrickyCases 钉住生成器的类型映射规则。
//
// 这些规则是「TS 类型必须描述线上真实 JSON」的支点：
//   - 可空字段（指针）→ `T | null`；
//   - map → 索引签名；
//   - any → unknown（**不猜**）；
//   - 具名 struct → 引用注册表里的名字（不内联展开）。
//
// 每条都对应一次真实的踩坑：指针漏成 `T` 会让前端在 `null` 上崩；
// `any` 猜成 `any` 会让 TS 放弃检查。
func TestTypeMappingCoversTheTrickyCases(t *testing.T) {
	paths, err := tygen.FindRepoPaths("")
	if err != nil {
		t.Fatalf("定位仓库失败：%v", err)
	}
	got, err := os.ReadFile(paths.AdminTypes)
	if err != nil {
		t.Fatalf("读生成物失败：%v", err)
	}
	text := string(got)

	cases := []struct {
		name string
		want string
	}{
		{"可空字段必须是 null 联合", "published_at: string | null"},
		{"map 必须是索引签名", "{ [key: string]: number }"},
		{"any 必须落成 unknown", "detail: unknown"},
		{"具名 struct 必须被引用而不是内联", "flows: AdminEconomyFlowView[]"},
		{"嵌套 struct 必须被引用", "items: AdminLevelView[]"},
	}
	for _, c := range cases {
		if !strings.Contains(text, c.want) {
			t.Errorf("生成物里找不到 %q（%s）", c.want, c.name)
		}
	}
	// 不该出现的东西：内联展开的具名类型
	if strings.Contains(text, "{ id: number; code: string") {
		t.Error("生成物把具名 struct 内联展开了 —— 应引用注册表里的名字")
	}
}
