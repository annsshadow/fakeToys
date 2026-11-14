# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L345 / A271：阶段闸执行中间件。

出厂十只 ``enabled`` 全为 True ⇒ 本中间件恒放行，行为零漂移。把某只设为 False 时，
只有 :data:`STAGE_ENDPOINTS` 里**明确登记**的那些端点回 400 中文固定文案。

刻意做成纯 ASGI 中间件而不是逐端点 ``Dependencies``：
* 不注册/不改动任何路由，OpenAPI 契约、端点名册、前端 fetcher 全部零位移；
* 一处集中，映射表可测、可回显；
* 判定实时读 ``stage_gates(get_pipeline().config)``，所以 ``POST /api/config``
  热更新之后立刻生效（装配时挂在管道上的 ``stage_gates`` 快照只供排障回显）。

空集合的五节不是遗漏：L341 缩掉 ``expander`` / ``tracker`` 属性、
``active_learning`` / ``frameworks`` 产品侧零实例化、``context`` 只有管道内的
构造点而没有独立端点 —— 它们今天**没有产品消费者可关**，凭空造端点会违反
「不写推测性代码」。解除条件：将来真接出消费者时补进映射表，同批重钉守卫。
"""

import json
import logging

from starlette.responses import Response
from starlette.types import ASGIApp, Receive, Scope, Send

from api import deps

logger = logging.getLogger(__name__)

#: 每只闸关掉的端点。**键集合必须等于 stage_gates() 的十节**（守卫逐格钉住）。
STAGE_ENDPOINTS = {
    "sampler": frozenset({("POST", "/api/dataset/sample")}),
    "rag": frozenset({("POST", "/api/dataset/rag")}),
    "evaluation": frozenset({("POST", "/api/dataset/evaluate")}),
    "multimodal": frozenset({
        ("POST", "/api/multimodal/process"),
        ("POST", "/api/multimodal/scan"),
        ("GET", "/api/multimodal/formats"),
    }),
    "benchmark": frozenset({("POST", "/api/quality/benchmark")}),
    "context": frozenset(),
    "expander": frozenset(),
    "tracker": frozenset(),
    "active_learning": frozenset(),
    "frameworks": frozenset(),
}

#: 反向索引：(METHOD, path) -> section。同一个端点只归一只闸（构造时断言）。
_OWNER = {key: section for section, keys in STAGE_ENDPOINTS.items() for key in keys}
assert len(_OWNER) == sum(len(keys) for keys in STAGE_ENDPOINTS.values()), "端点被多只闸重复登记"

#: 无产品消费者的节，供回显与守卫核对；补映射时这一格会按现量变小。
NO_CONSUMER_SECTIONS = frozenset(
    section for section, keys in STAGE_ENDPOINTS.items() if not keys
)

_DISABLED_TEMPLATE = "能力未启用：{section}.enabled 为 false"


class StageGateMiddleware:
    """按 (method, path) 查闸；关闭时直接回一份 400 JSON，不进路由。"""

    def __init__(self, app: ASGIApp) -> None:
        self.app = app

    async def __call__(self, scope: Scope, receive: Receive, send: Send) -> None:
        if scope["type"] != "http":
            await self.app(scope, receive, send)
            return
        section = _OWNER.get((scope["method"], scope["path"]))
        if section is None:
            await self.app(scope, receive, send)
            return
        try:
            # 实时值：POST /api/config 改的是同一个 config 对象，不能用启动快照。
            gates = deps.stage_gates(deps.get_pipeline().config)
            enabled = gates.get(section, True)
        except Exception as exc:  # fail-open：配置门不该把全站打成 500
            logger.warning("阶段闸读数不可用，放行 %s: %s", section, exc)
            enabled = True
        if enabled:
            await self.app(scope, receive, send)
            return
        body = json.dumps(
            {"detail": _DISABLED_TEMPLATE.format(section=section)}, ensure_ascii=False
        ).encode("utf-8")
        response = Response(body, status_code=400, media_type="application/json")
        await response(scope, receive, send)
