package httpapi

// 后台用户详情含防线信息的端到端测试（2026-10-31 新一轮第 31 轮）。

import (
	"context"
	"testing"

	"github.com/gofiber/fiber/v2"
)

func TestE2EAdminUserDetailIncludesDefense(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	_, adminTok := e.newAdmin(t, "ud_def")
	uid, _ := e.newPlayer(t, "ud_def_p")

	// 未设置防线时 defense 为 null
	status, body := e.get(t, "/api/v1/admin/users/"+fmtInt(uid), adminTok)
	if status != fiber.StatusOK {
		t.Fatalf("详情应 200，实际 %d %v", status, body)
	}
	if v, ok := body["defense"]; !ok || v != nil {
		t.Errorf("未设置防线时 defense 应为 null，实际 %v", body["defense"])
	}

	// 设置防线后 defense 非空
	if _, err := e.pool.Exec(context.Background(),
		`INSERT INTO defenses (owner_id, name, power, snapshot, snapshot_hash)
		 VALUES ($1, '我的铁壁', 1000, '{}'::jsonb, 'snap1')`, uid); err != nil {
		t.Fatalf("插入防线失败：%v", err)
	}
	status, body = e.get(t, "/api/v1/admin/users/"+fmtInt(uid), adminTok)
	if status != fiber.StatusOK {
		t.Fatalf("详情应 200，实际 %d %v", status, body)
	}
	defense, ok := body["defense"].(map[string]any)
	if !ok {
		t.Fatalf("有防线时 defense 应为对象，实际 %v", body["defense"])
	}
	if defense["name"] != "我的铁壁" {
		t.Errorf("防线名 = %v，期望 我的铁壁", defense["name"])
	}
	if defense["owner_id"] != float64(uid) {
		t.Errorf("owner_id = %v，期望 %d", defense["owner_id"], uid)
	}
}