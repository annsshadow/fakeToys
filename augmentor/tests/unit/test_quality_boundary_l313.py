# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L313 守卫：质量那一族按**类型**放行（A280 逐文件收窄第二批）

**这一轮收的是哪一格**：**A280**（L224 立，L311 半修）记的修法①「逐文件收窄」——
本轮收的是**质量那一族**（`api/routes/quality.py` 九处一刀切 500）。

改前的形状（逐格实测）：`POST /api/quality/evaluate` / `dedup` / `report` / `benchmark`
的 `threshold` 四形（NaN / +∞ / 5.0 / −1.0）**全部**回 **500 + 固定文案**，而下游
抛的明明是**具名的域判决**（`quality.threshold` / `dedup.threshold` 那句区间判决）⇒
**调用方自己写错了参数，服务端回一句「服务器内部错误」**。同一族的另外两只端点
（`outliers` / `health-gate`）当时有 `except ValueError` 泛档 ⇒ 同一个域判决在同一个文件里
落两种码；泛档还有第二重代价：它把**内部**异常原文也回成 400 判决（A386 那一族）。

本轮的拍法（**按类型放行，不按 `ValueError` 泛档放行**）：

- 域判决异常（`DataValidationError` 家族）⇒ **400 + 原判决**；
- 具名窄口（去重那一支的 `DedupError`）⇒ **400 + 原判决**；
- 其余异常 ⇒ **500 + 固定文案**（A155① 那句，内部原文只进服务端日志）。

四道面各自要能单独喊红：

1. **名册面**（AST）：`api/routes/quality.py` 里 `raise_internal_error` 的调用点数 **0**、
   `raise_boundary_error` 的调用点数 **9**、`except ValueError` 的臂数 **0** ——
   「一刀切那一族」与「泛档那一族」在这个文件上都已经摘干净。
2. **判决面**（HTTP）：16 格域判决（4 只端点 × 4 形）⇒ 400；`outliers` / `health-gate`
   那几只**原本就 400** 的格子一格未动；缺文件仍 404。
3. **收窄面**（注入）：普通 `ValueError` / `RuntimeError` ⇒ 仍是 **500 + 固定文案**，
   且响应里**不含内部原文**（泛档那一族留下的那条泄漏被摘掉了）。
4. **反向臂**：把去重那一支的窄口摘掉 ⇒ 那 16 格里的 4 格回到 500 ⇒ **窄口是承重的**，
   不是「表里写了 400 所以永远绿」。

**为什么零 parametrize**：`test_parametrize_ids_l97` 与 `test_parametrize_blind_face_l98`
把全仓 parametrize 三面冻成现量数，本守卫走具名用例，那两把尺子一格未动。

跑法：`python -m pytest tests/unit/test_quality_boundary_l313.py -q --no-cov`
"""
import ast
import json
from pathlib import Path

import pytest
from fastapi import HTTPException
from fastapi.testclient import TestClient

from api.deps import INTERNAL_ERROR_DETAIL, raise_boundary_error
from api.main import app
from augmentor.exceptions import DataValidationError, DedupError

PRODUCT_ROOT = Path(__file__).resolve().parents[2]
ROUTES = PRODUCT_ROOT / "api" / "routes" / "quality.py"

#: 四形坏值（NaN / 无穷 / 有限越界上 / 有限越界下）——与 l272 那一族同一口径。
NAN = float("nan")
INF = float("inf")
SHAPES = (("nan", NAN), ("inf", INF), ("5.0", 5.0), ("-1.0", -1.0))

#: 域判决那一族四只端点（`threshold` 是它们共有的那个旋钮）。
DOMAIN_ENDPOINTS = (
    "/api/quality/evaluate",
    "/api/quality/dedup",
    "/api/quality/report",
    "/api/quality/benchmark",
)

ITEMS = [{"instruction": f"问句 {i} 号", "output": f"答句 {i} 号", "length": 8 + (i % 5)}
         for i in range(10)]


def _call_sites(func_name):
    """AST 数 `api/routes/quality.py` 里对某个函数名的调用点（不含 import 行）"""
    tree = ast.parse(ROUTES.read_text(encoding="utf-8"))
    hits = 0
    for node in ast.walk(tree):
        if (isinstance(node, ast.Call)
                and getattr(node.func, "id", None) == func_name):
            hits += 1
    return hits


def _value_error_arms():
    """数 `except ValueError` 那一族的臂数（泛档那一族）"""
    tree = ast.parse(ROUTES.read_text(encoding="utf-8"))
    arms = 0
    for node in ast.walk(tree):
        if not isinstance(node, ast.Try):
            continue
        for handler in node.handlers:
            exc = handler.type
            if exc is None:
                continue
            names = []
            if isinstance(exc, ast.Name):
                names = [exc.id]
            elif isinstance(exc, ast.Tuple):
                names = [e.id for e in exc.elts if isinstance(e, ast.Name)]
            if "ValueError" in names:
                arms += 1
    return arms


@pytest.fixture()
def client():
    return TestClient(app)


@pytest.fixture()
def data_file(tmp_path):
    path = Path(tmp_path) / "a.json"
    path.write_text(json.dumps(ITEMS, ensure_ascii=False), encoding="utf-8")
    return str(path)


def _post(client, path, body):
    """按 `requests` 库的默认口径发体（`json.dumps` 的 `allow_nan` 默认 True）

    不能用 `client.post(json=...)`——httpx 在客户端侧就用 `allow_nan=False`，
    NaN 那一形连请求都发不出去，量到的会是客户端的拒绝而不是服务端的归宿。
    """
    raw = json.dumps(body).encode("utf-8")
    resp = client.post(path, content=raw, headers={"Content-Type": "application/json"})
    return resp.status_code, resp.text


class TestTheOneShotFamilyIsGone:
    """名册面：一刀切那一族与泛档那一族在这个文件上都摘干净了"""

    def test_no_internal_error_call_site_is_left(self):
        assert _call_sites("raise_internal_error") == 0

    def test_every_endpoint_goes_through_the_boundary(self):
        assert _call_sites("raise_boundary_error") == 9

    def test_the_blanket_value_error_arm_is_gone(self):
        """泛档那一族（`except ValueError` ⇒ 400 + 原文）在这个文件上零臂"""
        assert _value_error_arms() == 0


class TestTheDomainVerdictsAreFourHundreds:
    """判决面：16 格域判决 ⇒ 400 + 可行动的判决"""

    def test_evaluate_names_the_knob_and_the_value(self, client, data_file):
        for shape, value in SHAPES:
            code, text = _post(client, "/api/quality/evaluate",
                               {"input_file": data_file, "threshold": value})
            assert code == 400, (shape, text[:160])
            detail = json.loads(text)["detail"]
            assert "quality.threshold" in detail, (shape, detail)
            assert "NaN" in detail or "无穷" in detail or str(value) in detail, (shape, detail)

    def test_report_names_the_knob_and_the_value(self, client, data_file):
        for shape, value in SHAPES:
            code, text = _post(client, "/api/quality/report",
                               {"input_file": data_file, "threshold": value})
            assert code == 400, (shape, text[:160])
            detail = json.loads(text)["detail"]
            assert "quality.threshold" in detail, (shape, detail)

    def test_benchmark_names_the_knob_and_the_value(self, client, data_file):
        for shape, value in SHAPES:
            code, text = _post(client, "/api/quality/benchmark",
                               {"input_file": data_file, "threshold": value})
            assert code == 400, (shape, text[:160])
            detail = json.loads(text)["detail"]
            assert "quality.threshold" in detail, (shape, detail)

    def test_dedup_carries_the_sdk_verdict_verbatim(self, client, data_file):
        """去重那一支的域异常是 `DedupError`（SDK 把 `require_ratio` 的判决折成了它）

        那句文案是 **SDK 的既成契约**（`tests/unit/test_quality_dedup_gates_l76.py` 钉着
        「阈值必须在 0-1 之间」），所以这一格断言的是**它逐字不变** ⇒ 同时也记下它的
        代价：不点名字段名、也不回显实际值（本轮不改 SDK 的公开契约）。
        """
        for shape, value in SHAPES:
            code, text = _post(client, "/api/quality/dedup",
                               {"input_file": data_file, "threshold": value})
            assert code == 400, (shape, text[:160])
            assert json.loads(text)["detail"] == "阈值必须在 0-1 之间", (shape, text[:160])

    def test_the_two_endpoints_that_were_already_four_hundred_did_not_move(self, client, data_file):
        """`outliers` 与 `health-gate` 那几只格子一格未动（改前就 400，改后仍 400）"""
        code, text = _post(client, "/api/quality/outliers",
                           {"input_file": data_file, "method": "mad"})
        assert code == 400, text[:160]
        assert "不支持" in json.loads(text)["detail"], text[:160]

        code, text = _post(client, "/api/quality/outliers",
                           {"input_file": data_file, "threshold": NAN})
        assert code == 400, text[:160]

        code, text = _post(client, "/api/quality/health-gate",
                           {"input_file": data_file, "text_field": "nope"})
        assert code == 400, text[:160]

        code, text = _post(client, "/api/quality/health-gate",
                           {"input_file": data_file, "weights": [0.5]})
        assert code == 400, text[:160]

    def test_the_missing_file_face_is_untouched(self, client, tmp_path):
        code, text = _post(client, "/api/quality/evaluate",
                           {"input_file": str(Path(tmp_path) / "nope.json")})
        assert code == 404, text[:160]

    def test_the_good_body_still_succeeds(self, client, data_file):
        """判据没有把合法域收窄：默认档仍是 200"""
        for path in ("/api/quality/evaluate", "/api/quality/dedup",
                     "/api/quality/report", "/api/quality/clean",
                     "/api/quality/annotate", "/api/quality/benchmark"):
            code, text = _post(client, path, {"input_file": data_file})
            assert code == 200, (path, text[:160])


class TestTheNarrowingStaysNarrow:
    """收窄面：内部异常仍是 500，且内部原文不外泄"""

    def test_a_plain_value_error_is_an_internal_error_now(self, client, data_file, monkeypatch):
        """泛档那一族留下的那条（内部 `ValueError` ⇒ 400 + 原文）被摘掉了"""
        import augmentor.quality as sdk_quality

        class _Boom:
            def __init__(self, *args, **kwargs):
                raise ValueError("内部细节不该外泄")

        # 注入到「评估」那一支的下游（路由里是**函数内** import ⇒ 打到模块属性上即可）
        monkeypatch.setattr(sdk_quality, "QualityScorer", _Boom)
        code, text = _post(client, "/api/quality/evaluate", {"input_file": data_file})
        assert code == 500, text[:160]
        assert json.loads(text)["detail"] == INTERNAL_ERROR_DETAIL, text[:160]
        assert "内部细节不该外泄" not in text, text[:160]

    def test_a_runtime_error_is_an_internal_error(self, client, data_file, monkeypatch):
        import augmentor.quality as sdk_quality

        class _Boom:
            def __init__(self, *args, **kwargs):
                raise RuntimeError("boom")

        monkeypatch.setattr(sdk_quality, "QualityScorer", _Boom)
        code, text = _post(client, "/api/quality/evaluate", {"input_file": data_file})
        assert code == 500, text[:160]
        assert json.loads(text)["detail"] == INTERNAL_ERROR_DETAIL, text[:160]
        assert "boom" not in text, text[:160]


class TestTheReverseArm:
    """反向臂：窄口是承重的 —— 摘掉它，那 4 格立刻回到 500"""

    def test_dropping_the_narrow_port_puts_the_dedup_family_back_to_500(self, client, data_file,
                                                                        monkeypatch):
        import api.routes.quality as quality_module

        def _no_narrow_port(exc, *, not_found="资源不存在", not_found_types=(), boundary_types=()):
            """把 `boundary_types` 忽略掉的那把门口（= 改前那一档）"""
            if not_found_types and isinstance(exc, not_found_types):
                raise HTTPException(status_code=404, detail=not_found)
            if isinstance(exc, DataValidationError):
                raise HTTPException(status_code=400, detail=str(exc))
            from api.deps import raise_internal_error
            raise_internal_error(exc)

        monkeypatch.setattr(quality_module, "raise_boundary_error", _no_narrow_port)
        code, text = _post(client, "/api/quality/dedup",
                           {"input_file": data_file, "threshold": 5.0})
        assert code == 500, text[:160]
        assert json.loads(text)["detail"] == INTERNAL_ERROR_DETAIL, text[:160]


class TestTheBoundaryItself:
    """那把门口的契约：`boundary_types` 与 `not_found_types` 逐格同形"""

    def test_the_domain_family_is_four_hundred(self):
        with pytest.raises(HTTPException) as caught:
            raise_boundary_error(DataValidationError("判决"))
        assert caught.value.status_code == 400
        assert caught.value.detail == "判决"

    def test_the_named_port_is_four_hundred(self):
        with pytest.raises(HTTPException) as caught:
            raise_boundary_error(DedupError("阈值必须在 0-1 之间"),
                                 boundary_types=(DedupError,))
        assert caught.value.status_code == 400
        assert caught.value.detail == "阈值必须在 0-1 之间"

    def test_the_not_found_port_is_four_oh_four(self):
        with pytest.raises(HTTPException) as caught:
            raise_boundary_error(DedupError("不存在"), not_found_types=(DedupError,))
        assert caught.value.status_code == 404
        assert caught.value.detail == "资源不存在"

    def test_everything_else_stays_five_hundred(self):
        with pytest.raises(HTTPException) as caught:
            raise_boundary_error(ValueError("内部细节"))
        assert caught.value.status_code == 500
        assert caught.value.detail == INTERNAL_ERROR_DETAIL
