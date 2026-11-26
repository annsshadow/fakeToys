// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package httpapi

import "github.com/gofiber/fiber/v2"

// R19 玩家反应伤害贡献聚合（辅助分析，次数取 reactions_used）。
func (s *Server) meReactionDamageImpact(c *fiber.Ctx) error {
	res, err := s.Svc.MeReactionDamageImpact(c.Context(), userIDFrom(c))
	if err != nil {
		return err
	}
	return c.JSON(res)
}
