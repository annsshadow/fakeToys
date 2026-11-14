# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later
"""L345 / A271：十只阶段闸的关闸语义。

上一轮只把出厂值对齐成 True 并让闸有读者，关掉并没有效果。本轮补上执行面：
* 出厂全 True ⇒ 七只登记端点一个都不挡（零行为漂移）；
* 关某只 ⇒ 只挡它自己名下的端点，回执是固定中文 400；
* 判定实时读 config，POST /api/config 热更新后不用重启；
* 读数失败 fail-open，配置门不把全站打成 500。
零 parametrize（不扰动量面尺子）。
"""

from types import SimpleNamespace

import pytest
from starlette.applications import Starlette
from starlette.responses import PlainTextResponse
from starlette.routing import Route
from starlette.testclient import TestClient

from api import deps
from api.stage_gate import (
    NO_CONSUMER_SECTIONS,
    STAGE_ENDPOINTS,
    StageGateMiddleware,
    _OWNER,
)
from augmentor.config import AppConfig

SECTIONS = ("context", "sampler", "expander", "tracker", "rag", "evaluation",
            "multimodal", "benchmark", "active_learning", "frameworks")

ALL_ENDPOINTS = sorted(_OWNER)


def _client_and_config(monkeypatch):
    """把闸读数钉在一只可改的 AppConfig 上，routes 只回一句 reached。"""
    cfg = AppConfig()
    calls = []

    def fake_pipeline():
        calls.append(1)
        return SimpleNamespace(config=cfg, stage_gates=deps.stage_gates(cfg))

    monkeypatch.setattr(deps, "get_pipeline", fake_pipeline)
    routes = []
    for method, path in ALL_ENDPOINTS:
        routes.append(Route(path, lambda request: PlainTextResponse("reached"),
                            methods=[method]))
    routes.append(Route("/api/status", lambda request: PlainTextResponse("reached"),
                        methods=["GET"]))
    app = Starlette(routes=routes)
    app.add_middleware(StageGateMiddleware)
    return TestClient(app), cfg, calls


def test_mapping_is_exact_and_all_endpoints_really_exist():
    """映射表逐格钉住：十节、七个端点都在真实 OpenAPI 里、五节明确无消费者。"""
    from api.main import app

    assert set(STAGE_ENDPOINTS) == set(SECTIONS)
    assert len(_OWNER) == 7
    assert set(deps.stage_gates(AppConfig())) == set(SECTIONS)
    assert NO_CONSUMER_SECTIONS == frozenset(
        {"context", "expander", "tracker", "active_learning", "frameworks"}
    ), "无消费者节变了 ⇒ 要么真接了消费者，要么漏了映射，先补证据再改这格"

    spec = app.openapi()
    for method, path in ALL_ENDPOINTS:
        assert path in spec["paths"], f"{path} 已不在 OpenAPI 契约里"
        assert method.lower() in spec["paths"][path], f"{method} {path} 已不在契约里"


def test_default_all_open_leaves_every_mapped_endpoint_alone(monkeypatch):
    """零漂移反证臂：出厂十只全 True，一个 400 都不该出现。"""
    client, cfg, _ = _client_and_config(monkeypatch)
    assert all(deps.stage_gates(cfg).values())
    for method, path in ALL_ENDPOINTS:
        resp = client.request(method, path)
        assert resp.status_code == 200, (method, path, resp.text)
        assert resp.text == "reached"


def test_turning_off_one_gate_only_blocks_its_own_endpoints(monkeypatch):
    """隔离反证臂：关 multimodal 只挡三条多模态端点，其余四只照常进路由。"""
    client, cfg, _ = _client_and_config(monkeypatch)
    cfg.multimodal.enabled = False
    for method, path in ALL_ENDPOINTS:
        resp = client.request(method, path)
        owner = _OWNER[(method, path)]
        expected = 400 if owner == "multimodal" else 200
        assert resp.status_code == expected, (method, path, owner, resp.text)


def test_disabled_response_is_a_fixed_chinese_400(monkeypatch):
    """回执形状：固定文案、不回显请求体、不落部署路径。"""
    client, cfg, _ = _client_and_config(monkeypatch)
    cfg.benchmark.enabled = False
    resp = client.post("/api/quality/benchmark")
    assert resp.status_code == 400
    assert resp.headers["content-type"].startswith("application/json")
    assert resp.json() == {"detail": "能力未启用：benchmark.enabled 为 false"}


def test_unmapped_paths_never_touch_the_gate_reader(monkeypatch):
    """没登记的端点不该付读数成本，也不该被任何一只闸挡住。"""
    client, cfg, calls = _client_and_config(monkeypatch)
    cfg.sampler.enabled = False
    calls.clear()
    assert client.get("/api/status").status_code == 200
    assert calls == [], "未映射路径不该触发 get_pipeline()"


def test_live_config_update_takes_effect_without_restart(monkeypatch):
    """POST /api/config 改的是同一个 config 对象 ⇒ 判定不能用启动快照。"""
    client, cfg, _ = _client_and_config(monkeypatch)
    assert client.post("/api/dataset/rag").status_code == 200
    cfg.rag.enabled = False
    assert client.post("/api/dataset/rag").status_code == 400
    cfg.rag.enabled = True
    assert client.post("/api/dataset/rag").status_code == 200


def test_gate_reader_failure_fails_open(monkeypatch):
    """配置门读不到就放行：宁可漏挡，也不能把全站打成 500。"""
    client, _cfg, _ = _client_and_config(monkeypatch)

    def broken():
        raise RuntimeError("pipeline unavailable")

    monkeypatch.setattr(deps, "get_pipeline", broken)
    assert client.get("/api/multimodal/formats").status_code == 200


def test_real_app_honors_a_closed_gate(monkeypatch):
    """真身臂：不是只测合成 Starlette，真实 api.main.app 上同样生效。"""
    from api.main import app

    cfg = AppConfig()
    monkeypatch.setattr(deps, "get_pipeline",
                        lambda: SimpleNamespace(config=cfg,
                                               stage_gates=deps.stage_gates(cfg)))
    client = TestClient(app)
    assert client.get("/api/multimodal/formats").status_code == 200
    cfg.multimodal.enabled = False
    resp = client.get("/api/multimodal/formats")
    assert resp.status_code == 400, resp.text
    assert "multimodal.enabled" in resp.json()["detail"]
