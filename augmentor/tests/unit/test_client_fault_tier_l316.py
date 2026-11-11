# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L316 守卫：A386 收口 —— 400 档从「`ValueError` 泛档」收成「具名域判决」

**这一轮收的是哪一格**：**A386**（L284 新立；L315 把 A280 那一族处置完毕时点名交接：
「泛档那一格归 A386 与 A155①」）。

**症状（实测可复现，不是推理）**：一份含孤立代理的语料（用 JSON 转义写法 ``\\ud800`` 落盘 ——
文件本身是纯 ASCII，读回来才是 Python 里的孤立代理）打到
`POST /api/dataset/semantic-search` ⇒ **400**，``detail`` 逐字是
``'utf-8' codec can't encode character '\\ud800' in position 0: surrogates not allowed``
⇒ **编码层的内部消息被当成业务判决发给客户端**（泄露实现细节 + 判决类别也错）。

**为什么泛档那一格是错的**：分档门（``api/deps.py::to_http_error``）原先有一支
``isinstance(exc, ValueError)`` ⇒ 400 + ``str(exc)`` **原文照回**。``ValueError`` 是**父类**，
它同时罩住两类东西：① SDK 判出来的**具名域判决**（客户端的错）；② 框架 / 编码层 /
第三方库的**内部异常**（不是客户端的错）。泛档把 ② 也回成 400 判决，并把 ② 的原文发给客户端。

**本轮的拍法**（与 L313 / L314 同一口径：**按类型放行，不按父类放行**）：
400 档只放行**具名**的域判决类型（``CLIENT_FAULT_TYPES``），其余 ``ValueError`` 一律
500 + 固定文案（原文只落服务端日志）。逐处证明的结果写在 ``api/deps.py`` 那段注释与账本
L316 块里。

**名单是量出来的，不是推出来的**（仓外探针，仓内只留结论）：

1. 对 ``augmentor/`` 全包做 AST 普查 ⇒ 生产代码里**没有一处**裸 ``raise ValueError``
   （``augmentor/`` 那一侧由 ``tests/unit/test_exceptions.py`` 的护栏常年钉着；
   ``api/routes/`` 下唯一一处裸 ``raise ValueError`` 在 ``system_ops.py``，本轮改成具名域判决）。
   SDK 抛的域判决共 **21** 个具名类型，逐只点过之后，「客户端的错」那一族就是下面这 **7** 只，
   其余全是服务端自己的问题（配置坏了 / 依赖没装 / 模型没配好 / 后端炸了）⇒ 一律 500。
2. 对全包 ``int()`` / ``float()`` 等可能抛 ``ValueError`` 的转换点普查 **97** 处 ⇒ 逐处看过：
   要么作用于**自己算出来的**数值，要么已被 ``except (TypeError, ValueError)`` 兜住
   （``outlier.py``）⇒ **没有一处**把客户端语料里的字段裸转成数字 ⇒ 泛档那一格在产品面上
   只可能罩住「内部异常」这一类。
3. 逐个端点实测「坏语料」这一档的判决（仓外探针，**40** 条请求）⇒ 客户端的错**另有具名通道**
   （``DatasetTextError`` / ``assert_dataset_shape`` / ``DataFormatError`` …），不必靠泛档兜底。

**四道面各自要能单独喊红**：

1. **门禁面**：源码里**不再有** ``isinstance(exc, ValueError)`` 那一支（AST 形状守卫）；
   ``CLIENT_FAULT_TYPES`` 的成员逐只点名。
2. **判决面**：具名域判决逐只 ⇒ 400 + 原文；裸 ``ValueError`` / ``UnicodeEncodeError`` ⇒
   500 + 固定文案 + **原文不外泄**。
3. **窄口面**：窄口是**承重**的 —— 把它摘掉（``boundary_types=()``）⇒ 具名域判决当场掉进 500。
4. **症状面**：A386 点名的那一处（孤立代理打到语义检索）⇒ 从 400 + 内部原文变成 500 + 固定文案。

**为什么零 parametrize**：`test_parametrize_ids_l97` 与 `test_parametrize_blind_face_l98` 把全仓
parametrize 三面冻成现量数，本守卫走名册字典循环，那两把尺子一格未动。

跑法：`python -m pytest tests/unit/test_client_fault_tier_l316.py -q --no-cov`
"""
import ast
import json
import os
from pathlib import Path

import pytest
from fastapi.testclient import TestClient

from api.deps import CLIENT_FAULT_TYPES, INTERNAL_ERROR_DETAIL, to_http_error
from api.main import app

PRODUCT_ROOT = Path(__file__).resolve().parents[2]
DEPS = PRODUCT_ROOT / "api" / "deps.py"
ROUTES = PRODUCT_ROOT / "api" / "routes"

ITEMS = [{"instruction": "问句", "output": "答句"}]


@pytest.fixture()
def client():
    return TestClient(app, raise_server_exceptions=False)


@pytest.fixture()
def corpus(tmp_path, monkeypatch):
    """一份好数据 + 把临时目录放进路径白名单（零写入仓库 `data/`）"""
    monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", os.pathsep.join([os.getcwd(), str(tmp_path)]))
    path = tmp_path / "a.json"
    path.write_text(json.dumps(ITEMS, ensure_ascii=False), encoding="utf-8")
    return {"ok": path, "dir": tmp_path}


def _surrogate_corpus(tmp_path):
    """一份含孤立代理的语料：JSON 转义写法落盘（文件本身是纯 ASCII）"""
    path = tmp_path / "surrogate.json"
    path.write_text(
        json.dumps([{"instruction": "\ud800", "output": "答"}], ensure_ascii=True),
        encoding="utf-8",
    )
    return path


class TestTheTierIsNarrow:
    """门禁面 + 判决面：400 档只放行具名域判决"""

    def test_a_bare_value_error_is_no_longer_a_client_verdict(self):
        """泛档那一格收掉了：裸 `ValueError` ⇒ 500 + 固定文案，原文不外泄"""
        error = to_http_error(ValueError("内部细节不该外泄"))
        assert error.status_code == 500, error.detail
        assert error.detail == INTERNAL_ERROR_DETAIL, error.detail
        assert "内部细节不该外泄" not in error.detail

    def test_the_allowlist_is_exactly_the_measured_roster(self):
        """名单逐只点名：多一只少一只都当场红（这一份是量出来的，不是推出来的）

        **每一只都有本仓既有契约背书**：`DataLoadError`（缺 pandas / openpyxl 时那句
        「pip install pandas openpyxl」的可行动判决，CLI 侧同形状）与
        `ModelNotConfiguredError`（「翻译需要配置模型后端」）两族在仓里都是 **400**，
        不是 500 —— 这一格是先按「谁的错」推的名单把三支既有守卫顶红之后、按实测改的。
        """
        assert [t.__name__ for t in CLIENT_FAULT_TYPES] == [
            "DataValidationError",
            "DataFormatError",
            "UnsupportedFormatError",
            "QualityError",
            "DedupError",
            "PipelineError",
            "VectorError",
            "DataLoadError",
            "ModelNotConfiguredError",
        ], [t.__name__ for t in CLIENT_FAULT_TYPES]

    def test_each_named_client_fault_still_answers_400_with_its_own_words(self):
        """具名域判决逐只点名：仍是 400 + 原文（这一族一格未动）"""
        for exc_type in CLIENT_FAULT_TYPES:
            error = to_http_error(exc_type("参数不合法"))
            assert error.status_code == 400, exc_type.__name__
            assert error.detail == "参数不合法", exc_type.__name__

    def test_the_door_no_longer_has_a_bare_value_error_branch(self):
        """AST 形状守卫：泛档那一支不许回来（谁把 `isinstance(exc, ValueError)` 加回去 ⇒ 红）

        判据读的是**语法**不是文本：docstring 里那句「原先是 ``isinstance(exc, ValueError)``
        的泛档」是叙述，不是代码 ⇒ 不该被算成一处命中（文本口径会被注释与 docstring 咬出
        假阳性，A56 那条纪律的同一形状）。
        """
        tree = ast.parse(DEPS.read_text(encoding="utf-8"))
        target = None
        for node in ast.walk(tree):
            if isinstance(node, ast.FunctionDef) and node.name == "to_http_error":
                target = node
                break
        assert target is not None, "to_http_error 不见了"
        hits = []
        for node in ast.walk(target):
            if (isinstance(node, ast.Call)
                    and getattr(node.func, "id", None) == "isinstance"
                    and len(node.args) == 2
                    and isinstance(node.args[1], ast.Name)
                    and node.args[1].id == "ValueError"):
                hits.append(node.lineno)
        assert hits == [], f"泛档那一支回来了：ValueError 又被当 400（{hits}）"

    def test_the_internal_ones_now_land_on_the_server_tier(self):
        """内部异常那一族逐只点名：编码层 / 第三方库的 ValueError 一律 500 + 固定文案"""
        for exc in (
            UnicodeEncodeError("utf-8", "\ud800", 0, 1, "surrogates not allowed"),
            ValueError("could not convert string to float: 'abc'"),
        ):
            error = to_http_error(exc)
            assert error.status_code == 500, repr(exc)
            assert error.detail == INTERNAL_ERROR_DETAIL, repr(exc)

    def test_the_server_side_domain_errors_are_not_released(self):
        """服务端自己的那一族逐只点名：配置坏了 / 模型回包坏了 ⇒ 500 + 固定文案"""
        from augmentor.exceptions import (
            ConfigError,
            ModelInitError,
            ModelResponseError,
        )

        for exc_type in (ConfigError, ModelInitError, ModelResponseError):
            error = to_http_error(exc_type("服务端自己的问题"))
            assert error.status_code == 500, exc_type.__name__
            assert error.detail == INTERNAL_ERROR_DETAIL, exc_type.__name__
            assert "服务端自己的问题" not in error.detail, exc_type.__name__


class TestTheNarrowPortIsLoadBearing:
    """窄口面：那只端口承重，摘掉它具名判决当场掉档"""

    def test_dropping_the_port_sends_the_named_verdict_to_the_server_tier(self):
        from augmentor.exceptions import DataValidationError

        error = to_http_error(DataValidationError("参数不合法"), boundary_types=())
        assert error.status_code == 500, error.detail
        assert error.detail == INTERNAL_ERROR_DETAIL

    def test_the_port_can_add_one_named_type(self):
        """逐处证明的那一格：一只类型可以只在这一处被放行（这里用 `ConfigError` 举例）"""
        from augmentor.exceptions import ConfigError

        error = to_http_error(
            ConfigError("total_items 必须为正数"),
            boundary_types=CLIENT_FAULT_TYPES + (ConfigError,),
        )
        assert error.status_code == 400, error.detail
        assert error.detail == "total_items 必须为正数"

    def test_the_config_error_is_not_released_by_default(self):
        """同一个类型在别的门口不许被顺手放行（配置坏了是服务端自己的问题）"""
        from augmentor.exceptions import ConfigError

        error = to_http_error(ConfigError("模型 'x' 未配置"))
        assert error.status_code == 500, error.detail
        assert error.detail == INTERNAL_ERROR_DETAIL


class TestTheNamedSymptomIsGone:
    """症状面：A386 点名的那一处，从 400 + 内部原文变成 500 + 固定文案"""

    def test_the_semantic_search_surrogate_is_now_a_server_fault(self, client, corpus, tmp_path):
        surrogate = _surrogate_corpus(tmp_path)
        response = client.post("/api/dataset/semantic-search",
                               json={"input_file": str(surrogate), "query": "问"})
        assert response.status_code == 500, response.text[:200]
        assert response.json()["detail"] == INTERNAL_ERROR_DETAIL, response.text[:200]
        assert "surrogates not allowed" not in response.text, response.text[:200]

    def test_the_route_code_has_no_bare_value_error_raise(self):
        """那一处裸 `raise ValueError` 已经改成具名域判决（客户端的错仍是 400）"""
        offenders = []
        for path in sorted(ROUTES.glob("*.py")):
            tree = ast.parse(path.read_text(encoding="utf-8"))
            for node in ast.walk(tree):
                if not isinstance(node, ast.Raise) or node.exc is None:
                    continue
                call = node.exc.func if isinstance(node.exc, ast.Call) else node.exc
                if isinstance(call, ast.Name) and call.id == "ValueError":
                    offenders.append(f"{path.name}:{node.lineno}")
        assert not offenders, offenders

    def test_the_invalid_operation_is_still_a_client_verdict(self, client, corpus):
        """客户端把 `operation` 写错 ⇒ 仍是 400 + 那句可行动的判决（不许被收窄成 500）"""
        response = client.post("/api/system/stream", json={
            "input_file": str(corpus["ok"]),
            "output_file": str(corpus["dir"] / "out.json"),
            "operation": "nope"})
        assert response.status_code == 400, response.text[:200]
        assert "不支持的 operation" in response.json()["detail"], response.text[:200]

    def test_the_empty_dataset_auto_config_is_still_a_client_verdict(self, client, corpus):
        """空语料 ⇒ 自动配置那句 `total_items 必须为正数` 仍是 400（改抛具名域判决那一格）"""
        empty = corpus["dir"] / "empty.json"
        empty.write_text("[]", encoding="utf-8")
        response = client.post("/api/dataset/auto-config", json={"input_file": str(empty)})
        assert response.status_code == 400, response.text[:200]
        assert response.json()["detail"] == "total_items 必须为正数", response.text[:200]


class TestTheRosterIsWrittenDown:
    """名册面：35 处分档门逐文件点名 + 窄口没有挂在任何路由上"""

    def test_the_split_door_sites_are_still_where_the_roster_says(self):
        roster = {
            "config.py": 1,
            "data.py": 2,
            "dataset_tools.py": 14,
            "language.py": 2,
            "system_ops.py": 14,
            "vector.py": 2,
        }
        got = {}
        for path in sorted(ROUTES.glob("*.py")):
            tree = ast.parse(path.read_text(encoding="utf-8"))
            hits = 0
            for node in ast.walk(tree):
                if (isinstance(node, ast.Call)
                        and getattr(node.func, "id", None) == "to_http_error"):
                    hits += 1
            if hits:
                got[path.name] = hits
        assert got == roster, got
        assert sum(got.values()) == 35, sum(got.values())

    def test_no_route_releases_a_narrow_port_yet(self):
        """窄口住在门口上、**没有**挂在任何路由上（逐处证明的结果是「默认名单够用」）"""
        sites = []
        for path in sorted(ROUTES.glob("*.py")):
            tree = ast.parse(path.read_text(encoding="utf-8"))
            for node in ast.walk(tree):
                if (isinstance(node, ast.Call)
                        and getattr(node.func, "id", None) == "to_http_error"):
                    for kw in node.keywords:
                        if kw.arg == "boundary_types":
                            sites.append(f"{path.name}:{node.lineno}")
        assert sites == [], sites
