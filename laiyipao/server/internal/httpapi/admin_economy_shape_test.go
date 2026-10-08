package httpapi

import (
	"testing"
)

// 第 112 轮：钉住 admin/economy 的**字段名契约**。
//
// 后台前端（EconomyView）消费的字段是：
//   flows 行：reason / currency / count / delta（没有 amount / cnt / updated_at）
//   shop  行：price(对象 map) / limit_per_day / enabled（没有 currency / limit / on_sale）
//
// 修前服务端字段一直是这些，**是前端读错了名字**——
// 净流入 KPI 恒 0、价格列恒 0、上架列恒「否」。
// 缺陷在前端，但形状契约没有测试钉住：
// 服务端将来改名（delta→amount）时没有任何信号，
// 前端就会再次悄悄错一次。这条就是那面契约墙。
func TestAdminEconomyFieldContract(t *testing.T) {
	e := newE2E(t)
	_, tok := e.newAdmin(t, "econshape112")
	pid, ptok := e.newPlayer(t, "econshape112p")
	_ = pid
	// 制造至少一条 wallet_flows 流水，让 flows 行非空、键名断言有对象
	if st, _ := e.post(t, "/api/v1/signin", ptok, nil); st != 200 {
		t.Skipf("签到 %d（无可用签到日历时跳过流水断言）", st)
	}

	_, body := e.get(t, "/api/v1/admin/economy", tok)
	flows, _ := body["flows"].([]any)
	shop, _ := body["shop"].([]any)
	if len(flows) == 0 || len(shop) == 0 {
		t.Fatalf("economy 响应缺少 flows/shop 行（%v），契约断言无法建立", body)
	}

	fm, _ := flows[0].(map[string]any)
	for _, k := range []string{"reason", "currency", "count", "delta"} {
		if _, ok := fm[k]; !ok {
			t.Errorf("flows 行缺键 %q —— 后台 KPI 靠 delta 算净流入，键一改名就恒 0：%v", k, fm)
		}
	}
	sm, _ := shop[0].(map[string]any)
	for _, k := range []string{"id", "name", "price", "limit_per_day", "enabled"} {
		if _, ok := sm[k]; !ok {
			t.Errorf("shop 行缺键 %q（后台表格列依赖它）：%v", k, sm)
		}
	}
	if _, ok := sm["price"].(map[string]any); !ok {
		t.Errorf("price 必须是 {货币:数量} 的对象（JSONB map），实际 %T —— "+
			"标量会同时打坏后台渲染与玩家 Buy 的 Unmarshal(map[string]int)", sm["price"])
	}
}
