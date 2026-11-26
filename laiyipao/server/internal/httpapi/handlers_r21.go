// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package httpapi

import "github.com/gofiber/fiber/v2"

// R21 玩家卡点教练端点：只接受 window_days=30（缺省 30），user_id 来自会话。
func (s *Server) meLevelCoach(c *fiber.Ctx) error {
	vals := c.Context().QueryArgs().PeekMulti("window_days")
	if len(vals) > 1 {
		return fiber.NewError(fiber.StatusBadRequest, "参数 window_days 不能重复")
	}
	if len(vals) == 1 && string(vals[0]) != "30" {
		return fiber.NewError(fiber.StatusBadRequest, "window_days 只接受 30")
	}
	res, err := s.Svc.MeLevelCoach(c.Context(), userIDFrom(c))
	if err != nil {
		return err
	}
	return c.JSON(res)
}
