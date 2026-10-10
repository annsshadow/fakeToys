package httpapi

// `GET /admin/reactions` 的契约测试。
//
// ## 这个端点是为了消除后台的重复
//
// 后台看板（`DashboardView.vue`）原先硬编码了一张
// `REACTION_LABEL: Record<string,string>`，把 7 个反应的中文名逐个写死。
//
// 它当时**恰好是全的**（`domain/elements.go` 里正好 7 个 ReactionKey 常量），
// 所以看不出任何问题 —— 但名字的真源在服务端，
// 连 `ReactionSpec.Name` 的注释都写着「反应的中文名，后台系统/运营看板要用」。
// 复制一份的真实代价是：有人加第 8 个反应而忘了改那张表时，
// 症状**不是报错**，而是图表 y 轴上悄悄出现一个英文 key。
//
// 所以改成后台从本端点取名字。这个端点因此必须**严格等于**领域定义 ——
// 一旦它自己开始「少给一项」或「名字给空」，就等于把重复搬了个地方。
//
// ## 为什么需要数据库
//
// admin 端点在 `requireAdmin` 后面，而鉴权会查 `admin_users` 表确认账号状态，
// 所以**无法脱库测试**。这里沿用仓库既有的 `newE2E` + `newAdmin` 约定。
// 没有 Postgres 时 `newE2E` 会 Skip —— 那一刻这些断言是**未执行**，不是「通过」。

import (
	"testing"

	"github.com/laiyipao/server/internal/domain"
)

// fetchAdminReactions 取一次 /admin/reactions 里的反应条目。
func fetchAdminReactions(t *testing.T) []any {
	t.Helper()
	e := newE2E(t)
	_, adminTok := e.newAdmin(t, "reactions")
	status, body := e.get(t, "/api/v1/admin/reactions", adminTok)
	if status != 200 {
		t.Fatalf("状态码 = %d，期望 200", status)
	}
	list, ok := body["reactions"].([]any)
	if !ok {
		t.Fatalf("响应里没有 reactions 数组（顶层类型 %T）—— "+
			"后台按 `body.reactions` 取值，取不到就会整页退化", body["reactions"])
	}
	return list
}

func TestAdminReactionsMatchesDomain(t *testing.T) {
	got := fetchAdminReactions(t)
	want := domain.AllReactionSpecs()

	// 前提守卫：领域侧若返回空，本测试会退化成「两边都是空 → 相等」，
	// 恒绿。必须先确认领域侧确实有反应。
	if len(want) == 0 {
		t.Fatal("前提不成立：domain.AllReactionSpecs() 返回空 —— " +
			"本测试比较的两边都会是空，断言将毫无意义")
	}
	if len(got) != len(want) {
		t.Fatalf("端点返回 %d 条，领域有 %d 条 —— 后台会缺名字，坐标轴上出现英文 key",
			len(got), len(want))
	}

	// 用 key 索引而不是按顺序比：HTTP 数组的顺序不该成为契约。
	byKey := make(map[string]map[string]any, len(got))
	for _, item := range got {
		m, ok := item.(map[string]any)
		if !ok {
			t.Errorf("条目不是对象：%v", item)
			continue
		}
		k, _ := m["key"].(string)
		if k == "" {
			t.Errorf("有一条缺少 key：%v", m)
			continue
		}
		if _, dup := byKey[k]; dup {
			t.Errorf("key %q 重复出现 —— 后台按 key 建 map 时后者会覆盖前者", k)
		}
		byKey[k] = m
	}

	for _, spec := range want {
		key := string(spec.Key)
		m, ok := byKey[key]
		if !ok {
			t.Errorf("端点缺少反应 %q —— 后台该反应的坐标轴会显示原始 key 而非中文名", key)
			continue
		}
		gotName, _ := m["name"].(string)
		if gotName == "" {
			t.Errorf("反应 %q 的 name 为空 —— 运营看板会显示出一个空白标签", key)
			continue
		}
		if gotName != spec.Name {
			t.Errorf("反应 %q 的 name = %q，领域里是 %q —— 两处已经漂移",
				key, gotName, spec.Name)
		}
	}
}

// TestAdminReactionsIsNotTheHeavyConfig 钉住「不复用 /api/v1/config」这个决定。
//
// 理由是实测数字：config 未压缩 167,124 字节（levels 占 74%），
// 且每次请求都要重算 100 关关卡数据。为 7 个名字去生成 100 关不划算。
//
// 所以响应里**只**应有反应表，不夹带其它大字段。
func TestAdminReactionsIsNotTheHeavyConfig(t *testing.T) {
	got := fetchAdminReactions(t)
	if len(got) == 0 {
		t.Fatal("返回空数组")
	}
	allowed := map[string]bool{
		"key": true, "name": true, "base_coef": true, "attack_weight_pct": true,
		"status_duration_ms": true, "aoe_radius": true, "dispel_shield": true,
		"amplify_pct": true, "armor_shred_permille": true, "knockback": true, "descr": true,
	}
	for _, item := range got {
		m, ok := item.(map[string]any)
		if !ok {
			continue
		}
		for k := range m {
			if !allowed[k] {
				t.Errorf("反应条目里出现了非预期字段 %q —— "+
					"若混进了 /config 的大字段，这个端点就失去了「轻量」的意义", k)
			}
		}
	}
}
