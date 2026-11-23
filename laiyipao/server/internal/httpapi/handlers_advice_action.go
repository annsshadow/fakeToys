// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package httpapi

// R14：/me/advice/action 的只读入口。只收 element/reaction/source；
// id/replace 只能由服务端从已拥有目录里裁决后写进响应，不能由 query 注入。

import (
	"github.com/gofiber/fiber/v2"

	"github.com/laiyipao/server/internal/service"
)

func adviceOptionalQuery(c *fiber.Ctx, key string) (string, error) {
	vals := c.Context().QueryArgs().PeekMulti(key)
	if len(vals) == 0 {
		return "", nil
	}
	if len(vals) != 1 {
		return "", fiber.NewError(fiber.StatusBadRequest, "参数 "+key+" 不能重复")
	}
	return string(vals[0]), nil
}

func (s *Server) meAdviceAction(c *fiber.Ctx) error {
	element, err := adviceOptionalQuery(c, "element")
	if err != nil {
		return err
	}
	reaction, err := adviceOptionalQuery(c, "reaction")
	if err != nil {
		return err
	}
	source, err := adviceOptionalQuery(c, "source")
	if err != nil {
		return err
	}
	resp, err := s.Svc.AdviceAction(c.Context(), userIDFrom(c), element, reaction, source)
	if err != nil {
		return err
	}
	return c.JSON(service.NormalizeNilSlices(resp))
}
