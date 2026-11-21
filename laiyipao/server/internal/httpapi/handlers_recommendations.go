// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package httpapi

import (
	"github.com/gofiber/fiber/v2"

	"github.com/laiyipao/server/internal/service"
)

// meRecommendations 是 R6 §1E 用户侧只读推荐入口；鉴权由 me 路由组提供。
func (s *Server) meRecommendations(c *fiber.Ctx) error {
	resp, err := s.Svc.UserRecommendations(c.Context(), userIDFrom(c), c.Query("context", ""))
	if err != nil {
		return err
	}
	return c.JSON(service.NormalizeNilSlices(resp))
}

// adminRecommendations 是 R6 §1E 管理侧只读推荐入口；鉴权由 admin 路由组提供。
func (s *Server) adminRecommendations(c *fiber.Ctx) error {
	resp, err := s.Svc.AdminRecommendations(c.Context(), adminIDFrom(c), c.Params("scope"))
	if err != nil {
		return err
	}
	return c.JSON(service.NormalizeNilSlices(resp))
}
