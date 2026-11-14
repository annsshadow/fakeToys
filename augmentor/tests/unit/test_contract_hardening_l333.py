# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L333 批次 2 余量四笔的守卫：A353 严格 JSON / A354 范围判据 / A381 降级 503 / A351 坏名字 400

每一支都是**行为面**判据，不是源码字符串普查：把产品码改回旧形状，这里就红
（等价于注入臂，只是不必在内存里 patch 模块）。四笔放在一起是因为它们共用
「对外契约收紧」这一个口径，且每一笔的落地都要求调用方同批同步（题 1）。
"""

import json
import pathlib

import pytest
from fastapi import HTTPException
from fastapi.testclient import TestClient


# ------------------------------------------------------------------ A353 写侧
def test_atomic_write_json_rejects_nan_and_leaves_target_intact(tmp_path):
    """严格 JSON：非有限浮点在临时件上就抛 TypeError，目标文件一字不动"""
    from augmentor.atomic_write import atomic_write_json

    target = tmp_path / "data.json"
    target.write_text('{"v":1}', encoding="utf-8")
    with pytest.raises(ValueError):
        atomic_write_json(target, {"v": float("nan")})
    assert target.read_text(encoding="utf-8") == '{"v":1}', "失败语义：目标保持原样"
    assert not list(tmp_path.glob(".*.tmp-*")), "临时件必须被清掉"


@pytest.mark.parametrize("bad", [float("inf"), float("-inf")])
def test_atomic_write_json_rejects_infinity_on_both_branches(tmp_path, bad):
    """紧凑支与 indent 支都要带 allow_nan=False，不能只修一支"""
    from augmentor.atomic_write import atomic_write_json

    for indent in (None, 2):
        target = tmp_path / f"d{indent}.json"
        with pytest.raises(ValueError):
            atomic_write_json(target, {"v": bad}, indent=indent)
        assert not target.exists(), "序列化失败时不应产出目标文件"


def test_atomic_write_json_still_writes_finite_data(tmp_path):
    """反证臂：合法数据不受影响，别把「严格」做成「一律不写」"""
    from augmentor.atomic_write import atomic_write_json

    target = tmp_path / "ok.json"
    atomic_write_json(target, [{"v": 1.5}])
    assert json.loads(target.read_text(encoding="utf-8")) == [{"v": 1.5}]


# ------------------------------------------------------------------ A353 读侧
@pytest.mark.parametrize("literal", ["NaN", "Infinity", "-Infinity"])
def test_read_items_rejects_nonfinite_literals(tmp_path, literal):
    """读侧同一判据：含非有限字面量的文件不是合法 JSON ⇒ 400，不进统计"""
    from api.deps import read_items

    path = tmp_path / "bad.json"
    path.write_text('[{"v": %s}]' % literal, encoding="utf-8")
    with pytest.raises(HTTPException) as exc:
        read_items(path)
    assert exc.value.status_code == 400
    assert "非有限" in exc.value.detail


def test_read_items_still_reports_syntax_error_with_400(tmp_path):
    """反证臂：普通语法错仍走原来的 JSONDecodeError 映射，没有被新 except 吞成 500"""
    from api.deps import read_items

    path = tmp_path / "broken.json"
    path.write_text("{not json", encoding="utf-8")
    with pytest.raises(HTTPException) as exc:
        read_items(path)
    assert exc.value.status_code == 400


# ------------------------------------------------------------------ A351 名字闸
@pytest.mark.parametrize("name", ["a\x00b.json", "a\nb.json", "a\tb.json", "x" * 256])
def test_impossible_name_is_400_not_404(name):
    """名字根本不可能成立 ⇒ 400；「文件不存在」(404) 只留给合法名字"""
    from api.deps import resolve_within_roots

    with pytest.raises(HTTPException) as exc:
        resolve_within_roots(name, "文件路径")
    assert exc.value.status_code == 400
    assert "文件不存在" not in exc.value.detail


def test_empty_name_keeps_its_own_detail():
    """反证臂：空名字仍回原来的「不能为空」，不被坏名字文案覆盖"""
    from api.deps import resolve_within_roots

    with pytest.raises(HTTPException) as exc:
        resolve_within_roots("   ", "文件路径")
    assert exc.value.status_code == 400
    assert "不能为空" in exc.value.detail


def test_path_with_dotdot_is_still_400():
    """反证臂：既有 `..` 判据未被新判据顶掉"""
    from api.deps import resolve_within_roots

    with pytest.raises(HTTPException) as exc:
        resolve_within_roots("../escape.json", "文件路径")
    assert exc.value.status_code == 400


# ------------------------------------------------------------------ A354 范围判据
def test_gate_thresholds_out_of_range_are_rejected():
    """三只阈值按 [0,1] 收口；改回 ±inf 这里立刻红"""
    from augmentor.quality_gate import gate_dataset_health
    from augmentor.exceptions import DataValidationError

    items = [{"text": "样本一", "quality_score": 0.8}, {"text": "样本二", "quality_score": 0.4}]
    for kw in ({"pass_rate_min": 5.0}, {"pass_rate_min": -1.0},
               {"duplicate_rate_max": 99.0}, {"completeness_min": -3.0}):
        with pytest.raises(DataValidationError):
            gate_dataset_health(items, text_field="text", **kw)


def test_gate_thresholds_at_boundaries_survive():
    """反证臂：0 / 1 两个端点仍合法，别把闭区间做成开区间"""
    from augmentor.quality_gate import gate_dataset_health

    items = [{"text": "样本一", "quality_score": 0.8}]
    for kw in ({"pass_rate_min": 0.0}, {"pass_rate_min": 1.0},
               {"duplicate_rate_max": 0.0}, {"completeness_min": 1.0}):
        gate_dataset_health(items, text_field="text", **kw)


def test_min_scale_gain_below_minus_one_is_rejected():
    """min_scale_gain 的定义域是 [-1,+∞)，低于 -1 等于「永远放行」"""
    from augmentor.impact import ImpactEvaluator, AugmentationImpact
    from augmentor.exceptions import DataValidationError

    impact = AugmentationImpact(before={}, after={}, gains={"scale_gain": 0.2})
    with pytest.raises(DataValidationError):
        ImpactEvaluator().is_beneficial(impact, min_scale_gain=-2.0)
    assert ImpactEvaluator().is_beneficial(impact, min_scale_gain=-1.0) is True


# ------------------------------------------------------------------ A381 降级 503
@pytest.fixture
def client():
    from api.main import app

    return TestClient(app)


def _break_pipeline(monkeypatch):
    import api.deps as deps
    from api.routes import status as status_module

    def _boom():
        raise RuntimeError("l333 probe: pipeline 构造失败")

    monkeypatch.setattr(deps, "_pipeline", None)
    monkeypatch.setattr(status_module, "get_pipeline", _boom)


def test_pipeline_failure_is_503_with_unchanged_body(client, monkeypatch):
    """降级回 503，但响应体键集一字不变；原文仍不进响应体"""
    _break_pipeline(monkeypatch)
    response = client.get("/api/status")
    assert response.status_code == 503
    payload = response.json()
    assert payload["status"] == "ok"
    assert payload["model_default"] == "unknown"
    assert payload["model_available"] is False
    assert "l333 probe" not in response.text
    assert set(payload) == {
        "status", "version", "model_default", "model_available",
        "model_retry_count", "model_retry_wait_seconds", "model_generate_seconds",
        "dependencies",
    }


def test_healthy_pipeline_is_still_200(client):
    """反证臂：健康路径不许被降级标志污染"""
    response = client.get("/api/status")
    assert response.status_code == 200
    assert "status" in response.json()
