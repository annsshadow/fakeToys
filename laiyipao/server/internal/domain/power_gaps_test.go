package domain

// power.go 的文案与契合度补漏。
//
// elementName / joinCN 是诊断与后台展示的中文文案生成器；
// 装备契合（synergy）是 I-3 的核心激励：同系装备给反应伤害大加成。
// 文案分支错了不炸服务，但会把「冰」显示成 "ice"——
// 玩家与运营看到的诊断就不再是中文契约的一部分。

import (
	"strings"
	"testing"
)

func TestElementNameCoversAllElements(t *testing.T) {
	for _, c := range []struct {
		e    Element
		want string
	}{
		{ElementFire, "焰"},
		{ElementIce, "冰"},
		{ElementLightning, "电"},
		{ElementCorrosion, "毒"},
		{ElementKinetic, "动能"},
	} {
		if got := elementName(c.e); got != c.want {
			t.Errorf("elementName(%s) = %q，期望 %q", c.e, got, c.want)
		}
	}
	// 兜底：非规范元素（来自客户端报文）原样回显，而不是 panic 或空串
	if got := elementName(Element("bogus")); got != "bogus" {
		t.Errorf("elementName 兜底应原样回显，实际 %q", got)
	}
}

func TestJoinCN(t *testing.T) {
	for _, c := range []struct {
		in   []string
		want string
		why  string
	}{
		{nil, "", "空列表返回空串"},
		{[]string{"焰"}, "焰", "单项不加顿号"},
		{[]string{"焰", "冰"}, "焰、冰", "两项用顿号连接"},
		{[]string{"焰", "冰", "电"}, "焰、冰、电", "三项及以上逐项顿号连接"},
		{[]string{"焰", "冰", "电", "毒"}, "焰、冰、电、毒", "四项同理"},
	} {
		if got := joinCN(c.in); got != c.want {
			t.Errorf("joinCN(%v) = %q，期望 %q（%s）", c.in, got, c.want, c.why)
		}
	}
}

// TestComputeBuildRatingCountsEquipmentSynergy 契合度只统计
// 「与已装备技能同系」的装备。异系装备此前被误计过（或漏计过），
// 这里钉住正反两个方向。
func TestComputeBuildRatingCountsEquipmentSynergy(t *testing.T) {
	in := RatingInput{
		SkillElements: []Element{ElementFire, ElementIce},
		// 2 件同系 + 1 件异系 + 1 件重复同系
		EquipmentElements: []Element{ElementFire, ElementIce, ElementLightning, ElementFire},
	}
	r := ComputeBuildRating(in, DefaultRatingWeights())
	if r.EquipmentSynergy != 3 {
		t.Errorf("同系装备契合度应为 3（含重复计数），实际 %d", r.EquipmentSynergy)
	}
	if r.ElementCoverage != 2 {
		t.Errorf("技能元素覆盖应为 2，实际 %d", r.ElementCoverage)
	}
}

// TestDomainErrorMessage domainError 实现 error 接口——
// Error() 若返回空串，API 层的 400 响应体里会是一条空消息。
func TestDomainErrorMessage(t *testing.T) {
	if got := errUnknownMasteryNode(7).Error(); got == "" {
		t.Error("errUnknownMasteryNode 的 Error() 不应为空")
	} else if !strings.Contains(got, "7") {
		t.Errorf("错误消息应包含节点 id，实际 %q", got)
	}
	for _, err := range []error{
		errMasteryLayerLimit(MasteryNode{Layer: 3, Family: "fire"}),
		errMasteryPrereq(MasteryNode{Layer: 2}),
		errMasteryPoints(4, 3),
	} {
		if err.Error() == "" {
			t.Errorf("%T 的 Error() 不应为空", err)
		}
	}
}
