// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package httpapi

// R7 缺口3：推荐落地收益预估的只读入口。
// 参数：kind=skill|equipment, id=配置里的候选 id, replace=当前已装备的 id。
// 重复参数一律 400 —— 模糊输入不该被静默解析成"第一个"。

import (
	"errors"
	"strconv"

	"github.com/gofiber/fiber/v2"

	"github.com/laiyipao/server/internal/service"
)

func impactSingleQuery(c *fiber.Ctx, key string) (string, error) {
	vals := c.Context().QueryArgs().PeekMulti(key)
	if len(vals) == 0 {
		return "", service.ImpactParamError{Msg: "缺少参数 " + key}
	}
	if len(vals) != 1 {
		return "", service.ImpactParamError{Msg: "参数 " + key + " 不能重复"}
	}
	return string(vals[0]), nil
}

func (s *Server) meRecommendationImpact(c *fiber.Ctx) error {
	kind, err := impactSingleQuery(c, "kind")
	if err != nil {
		return fiber.NewError(fiber.StatusBadRequest, err.Error())
	}
	idRaw, err := impactSingleQuery(c, "id")
	if err != nil {
		return fiber.NewError(fiber.StatusBadRequest, err.Error())
	}
	replaceRaw, err := impactSingleQuery(c, "replace")
	if err != nil {
		return fiber.NewError(fiber.StatusBadRequest, err.Error())
	}
	candidateID, err := strconv.Atoi(idRaw)
	if err != nil {
		return fiber.NewError(fiber.StatusBadRequest, "id 必须是整数")
	}
	replacedID, err := strconv.Atoi(replaceRaw)
	if err != nil {
		return fiber.NewError(fiber.StatusBadRequest, "replace 必须是整数")
	}
	resp, err := s.Svc.RecommendationImpact(c.Context(), userIDFrom(c), kind, candidateID, replacedID)
	if err != nil {
		var pe service.ImpactParamError
		if errors.As(err, &pe) {
			return fiber.NewError(fiber.StatusBadRequest, pe.Error())
		}
		return err
	}
	return c.JSON(service.NormalizeNilSlices(resp))
}
