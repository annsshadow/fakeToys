# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L283：`/api/status` 的降级支必须留日志 —— 全仓 125 支宽 `except` 里唯一落在 HTTP 面的静默

普查口径（`Temp/probe_l283_census.py` / `probe_l283_two.py` 实测）：把「宽 `except` + 既不
记日志也不重抛」的静默落空逐格对着**读侧后果**判：5 支 `except: pass` 全在合法位置（清理
临时件、探测可选依赖等），125 支宽 `except` 里只有 `api/routes/status.py` 的管道读取支**既
不响也不记** —— 而它面向的是「运维看什么」这个用途：状态端点报出 `model_available: false`
时，运维读到的只有「不可用」，`get_pipeline()` 究竟为什么抛（配置坏、目录不可读、依赖缺失）
只存在于那一段被吞掉的异常里，盘上一字未留。同仓的 `deps` 面 500 支走的是「原文只落服务端
日志、不进响应体」，这一格连日志都不留，属于同一族里的漏网，不是另一条主张。

修法克制：只补 `logger.warning`，**不动响应体** —— `status` 字段是否该从 `"ok"` 降级成
`"degraded"` 是对外契约的判决（前端与探针按 `status` 判存活），记为待拍 A381，本守卫只钉
「必须出声」与「响应契约不因补日志而变」。
"""

import logging

import pytest
from fastapi.testclient import TestClient


@pytest.fixture
def client():
    from api.main import app

    return TestClient(app)


def _break_pipeline(monkeypatch):
    """把 `get_pipeline` 换成必然抛错的替身 —— 走的就是那一条降级支"""
    import api.deps as deps
    from api.routes import status as status_module

    def _boom():
        raise RuntimeError("l283 probe: pipeline 构造失败")

    monkeypatch.setattr(deps, "_pipeline", None)
    monkeypatch.setattr(status_module, "get_pipeline", _boom)
    return _boom


def test_pipeline_failure_is_logged(client, monkeypatch, caplog):
    """降级必须有日志，且原文（异常串）落进日志而不进响应体"""
    _break_pipeline(monkeypatch)
    with caplog.at_level(logging.WARNING, logger="api.routes.status"):
        response = client.get("/api/status")

    assert response.status_code == 200
    records = [r for r in caplog.records if r.name == "api.routes.status"]
    assert len(records) == 1
    assert records[0].levelno == logging.WARNING
    message = records[0].getMessage()
    assert "l283 probe: pipeline 构造失败" in message
    # 原文不外泄给调用方：这条异常文本此前只可能出现在响应体里，补日志之后仍不许出现
    assert "l283 probe" not in response.text


def test_degrade_contract_unchanged(client, monkeypatch):
    """响应契约一字不让：降级仍回 200，`model_default` 落 `unknown`、后端判不可用、重试两键为 `None`

    这条是补日志的护栏 —— 修的是「不出声」，不是把状态端点变成失败端点。
    """
    _break_pipeline(monkeypatch)
    response = client.get("/api/status")

    assert response.status_code == 200
    payload = response.json()
    assert payload["model_default"] == "unknown"
    assert payload["model_available"] is False
    assert payload["model_retry_count"] is None
    assert payload["model_retry_wait_seconds"] is None
    assert "status" in payload  # A381 待拍：本格只钉「键仍在」，不钉它的值该不该降级


def test_healthy_pipeline_leaves_no_warning(client, monkeypatch, caplog):
    """反证臂：正常路径不许被这条日志污染 —— 补日志不能变成「每次成功也记一笔降级」

    用替身管道而不是真 `AugmentorPipeline`：这一格判的是「有管道时不响」，真管道要落盘、
    且读的是仓里那份 `config.yaml`，与本断言无关。
    """
    import types

    import api.deps as deps
    from api.routes import status as status_module

    fake = types.SimpleNamespace(
        config=types.SimpleNamespace(default_model="stub-model"),
        model_backend=None,
    )
    monkeypatch.setattr(deps, "_pipeline", None)
    monkeypatch.setattr(status_module, "get_pipeline", lambda: fake)
    with caplog.at_level(logging.WARNING, logger="api.routes.status"):
        response = client.get("/api/status")

    assert response.status_code == 200
    assert response.json()["model_default"] == "stub-model"
    assert [r for r in caplog.records if r.name == "api.routes.status"] == []
