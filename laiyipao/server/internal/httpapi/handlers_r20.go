// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package httpapi

// R20 战斗效率趋势聚合入口：me 严格按会话 user_id，admin 全平台。
// window_days 仅接受 30（缺省按 30），重复/非法一律 400。

import (
	"strconv"

	"github.com/gofiber/fiber/v2"

	"github.com/laiyipao/server/internal/service"
)

func battleEfficiencyWindowDays(c *fiber.Ctx) (int, error) {
	vals := c.Context().QueryArgs().PeekMulti("window_days")
	if len(vals) == 0 {
		return 30, nil
	}
	if len(vals) != 1 {
		return 0, service.ImpactParamError{Msg: "参数 window_days 不能重复"}
	}
	n, err := strconv.Atoi(string(vals[0]))
	if err != nil || n != service.BattleEfficiencyWindowDaysValue {
		return 0, service.ImpactParamError{Msg: "window_days 只接受 30"}
	}
	return n, nil
}

func (s *Server) meBattleEfficiency(c *fiber.Ctx) error {
	if _, err := battleEfficiencyWindowDays(c); err != nil {
		return fiber.NewError(fiber.StatusBadRequest, err.Error())
	}
	res, err := s.Svc.MeBattleEfficiency(c.Context(), userIDFrom(c))
	if err != nil {
		return err
	}
	return c.JSON(res)
}

func (s *Server) adminBattleEfficiency(c *fiber.Ctx) error {
	if _, err := battleEfficiencyWindowDays(c); err != nil {
		return fiber.NewError(fiber.StatusBadRequest, err.Error())
	}
	res, err := s.Svc.AdminBattleEfficiency(c.Context())
	if err != nil {
		return err
	}
	return c.JSON(res)
}
