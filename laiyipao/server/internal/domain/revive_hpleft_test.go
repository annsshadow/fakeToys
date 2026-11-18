// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package domain

// 复活币对局的 hp_left 上界守卫。
//
// 服务端 StartBattle 在玩家持有 revive_token 时下发
// extra_base_hp = ReviveExtraBaseHP(gl)（gl.BaseHP × 3/10），
// 客户端引擎把它加在「当前血量」上（baseHp = baseHpMax + extraBaseHp），
// 因此合法复活局的剩余血量最高可达 BaseHP + extra_base_hp。
//
// 但 ValidateSettle 此前把 hp_left 上界写死为 gl.BaseHP，
// 于是「漏怪少于 30% 的复活胜利局」会被无条件 422 拒绝 ——
// 玩家花了复活币，反而结算不了。本文件钉住这个跨端契约。

import (
	"reflect"
	"testing"
	"time"
)

// setTokenRevived 用反射设置 BattleToken.Revived。
//
// 之所以不直接写 tok.Revived = true：本用例是 RED 测试，
// 在修复前 BattleToken **还没有** Revived 字段。直接赋值会编译失败，
// 那只能证明「字段不存在」，证明不了「结算路径拒绝合法血量」这个行为。
// 反射让用例在修复前后都能编译，红线落在真实的行为断言上。
func setTokenRevived(t *testing.T, tok *BattleToken, revived bool) {
	t.Helper()
	v := reflect.ValueOf(tok).Elem().FieldByName("Revived")
	if !v.IsValid() {
		t.Fatal("BattleToken 缺少 Revived 字段 —— 服务端无从得知本局是否用了复活币，" +
			"只能把 hp_left 上界写死为关卡 base_hp，合法复活局必被拒")
	}
	if !v.CanSet() {
		t.Fatal("BattleToken.Revived 不可写")
	}
	v.SetBool(revived)
}

func TestRevivedBattleHPLeftUpperBound(t *testing.T) {
	gl := GenerateLevel(1)
	extra := ReviveExtraBaseHP(gl)
	if extra <= 0 {
		t.Fatalf("ReviveExtraBaseHP(第1关) = %d，必须为正", extra)
	}

	base := settleBase(t, gl) // 一份能通过全部校验的合法上报，HPLeft = BaseHP
	limits := limitsFor(gl)
	now := time.Now()

	// ① 复活局：一点没漏（剩余血量 = BaseHP + 额外血量）必须被接受。
	revivedTok := newTestToken(gl.ID)
	setTokenRevived(t, &revivedTok, true)
	in := base
	in.HPLeft = int(gl.BaseHP + extra)
	if _, err := ValidateSettle(revivedTok, gl, in, limits, now); err != nil {
		t.Errorf("复活局 hp_left=%d（BaseHP %d + 复活额外 %d）应被接受，却报：%v",
			in.HPLeft, gl.BaseHP, extra, err)
	}

	// ② 复活局：上界必须仍然紧，BaseHP + extra + 1 要被拒绝。
	in.HPLeft = int(gl.BaseHP+extra) + 1
	if _, err := ValidateSettle(revivedTok, gl, in, limits, now); err == nil {
		t.Errorf("复活局 hp_left=%d（上界+1）应被拒绝，却通过", in.HPLeft)
	}

	// ③ 非复活局：原有上界不能放宽，BaseHP + 1 仍要被拒绝。
	plainTok := newTestToken(gl.ID)
	setTokenRevived(t, &plainTok, false)
	in = base
	in.HPLeft = int(gl.BaseHP) + 1
	if _, err := ValidateSettle(plainTok, gl, in, limits, now); err == nil {
		t.Errorf("非复活局 hp_left=%d（BaseHP+1）应被拒绝，却通过", in.HPLeft)
	}
}
