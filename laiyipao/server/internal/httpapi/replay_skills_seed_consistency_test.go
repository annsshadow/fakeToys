// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package httpapi

import (
	"context"
	"testing"

	"github.com/laiyipao/server/internal/domain"
)

// TestSeedSkillsBaseDamageMatchesDB：客户端烘 replay_skills 用的是 /config（进程内
// SeedSkills），服务端重算用的是 skills 表。两边一旦漂移，合法玩家的结算会被
// 当成作弊拒绝（t26 真浏览器实测：上报 100、重算 282）。这里把「种子底伤必须
// 与库里的权威行一致」钉成硬约束；任何测试改了库里的种子必须还原。
func TestSeedSkillsBaseDamageMatchesDB(t *testing.T) {
	e := newE2E(t)
	for _, want := range domain.SeedSkills {
		var got int64
		if err := e.pool.QueryRow(context.Background(),
			`SELECT base_damage FROM skills WHERE id = $1`, want.ID).Scan(&got); err != nil {
			t.Fatalf("查 skills(%d).base_damage 失败：%v", want.ID, err)
		}
		if got != want.BaseDamage {
			t.Errorf("skills(%d).base_damage = %d，种子/config = %d；漂移会让带 replay_skills 的合法结算被拒", want.ID, got, want.BaseDamage)
		}
	}
}

// TestLevel1ClientReplaySkillsIsAccepted：t26 真浏览器里客户端上报的 S 段原样固定
// 在此。第 1 关、技能 1 级的合法回放必须被服务端接受；若被拒，说明重算口径或
// 库里种子已漂移，而不是玩家作弊。
func TestLevel1ClientReplaySkillsIsAccepted(t *testing.T) {
	e := newE2E(t)
	uid, tok := e.newPlayer(t, "legal-s")
	e.exec(t, `UPDATE user_wallets SET energy = 200, energy_updated_at = now() WHERE user_id = $1`, uid)

	const clientSkills = "0:1:100:1:20,1:4:90:1:18,2:7:110:1:24"
	status, body, _ := settle(t, e, tok, clientSkills)
	if status != 200 {
		t.Fatalf("第1关合法 replay_skills 被拒：%d %v（客户端 S 段 %q）", status, body, clientSkills)
	}
}
