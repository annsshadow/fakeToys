# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L314 守卫：A280 逐文件收窄（第三批）——那六只文件按类型放行

**这一轮收的是哪一格**：**A280**（L224 立，L311 半修，L313 续半修）记的修法①
「逐文件收窄」——本轮收**第三批**：那六只文件里剩下的 18 处一刀切 500
（`api/routes/data.py` 5 / `api/routes/augment.py` 4 / `api/routes/config.py` 3 /
`api/routes/export.py` 3 / `api/routes/multimodal.py` 2 / `api/routes/privacy.py` 1）。

**拍法沿用 L313**：**按类型放行**，不按 `ValueError` 泛档放行 ⇒

- 域判决（`DataValidationError` 家族）⇒ **400 + 原判决**；
- 具名窄口（模块自己的域异常）⇒ **400 + 原判决**；
- **坏 JSON 文件**（客户端那份文件坏了）⇒ **400 + 那句中文判决**
  （与 `to_http_error` 那一档**逐字同源** ⇒ 判据住一处）；
- 其余异常 ⇒ **500 + 固定文案**（内部原文只进服务端日志）。

**逐处先实测再决定放行口径**（18 处里**收窄 8 格、保持 500 十格**）：

| 文件 | 收窄 | 保持 500（理由） |
| --- | --- | --- |
| `api/routes/data.py` | 3（坏 JSON / 顶层非数组 / 坏 JSON 另一支） | 2（更新与删除：域判决已被 `HTTPException` 接住、写边 `OSError` 是服务端状态） |
| `api/routes/augment.py` | 2（断点删除与进度：坏任务 id） | 2（启动：域判决在 `HTTPException` 里；列表：无客户端入参） |
| `api/routes/config.py` | 0 | 3（域判决那一档已在 `except DataValidationError` 单独接住；其余是服务端配置坏了） |
| `api/routes/export.py` | 2（导出与预览） | 1（格式清单：无客户端入参） |
| `api/routes/multimodal.py` | 1（扫描：目录不可读 / 坏图片 ⇒ 具名窄口） | 1（处理：域判决在 `HTTPException` 里） |
| `api/routes/privacy.py` | 0 | 1（脱敏：域判决在 `HTTPException` 里，SDK 不抛域异常） |

四道面各自要能单独喊红：

1. **名册面**（AST）：六只文件里「一刀切」调用点数 **18 → 10**、「边界口」调用点数本轮 **+8** ⇒
   逐文件点名。
2. **判决面**（HTTP）：本轮翻面的 **8 格**逐格点名（400 + 那句判决）；**一格未动的也逐格点名**
   （缺文件仍 404、合法请求仍 200、目录不可读仍 400）。
3. **收窄面**（注入）：内部异常 ⇒ 仍是 **500 + 固定文案**，且响应里**不含内部原文**。
4. **反向臂**：把具名窄口摘掉 ⇒ 那一族回到 500 ⇒ **窄口是承重的**，不是「表里写了 400 所以永远绿」。

**为什么零 parametrize**：`test_parametrize_ids_l97` 与 `test_parametrize_blind_face_l98`
把全仓 parametrize 三面冻成现量数，本守卫走具名用例，那两把尺子一格未动。

跑法：`python -m pytest tests/unit/test_boundary_narrowing_l314.py -q --no-cov`
"""
import ast
import json
import os
from pathlib import Path

import pytest
from fastapi import HTTPException
from fastapi.testclient import TestClient

from api.deps import INTERNAL_ERROR_DETAIL
from api.main import app
from augmentor.exceptions import DataValidationError, DataFormatError, DataLoadError

PRODUCT_ROOT = Path(__file__).resolve().parents[2]
ROUTES = PRODUCT_ROOT / "api" / "routes"

#: 本轮收窄的那六只文件 ⇒ (一刀切调用点数, 边界口调用点数) 的现量名册。
ROSTER = {
    "data.py": (2, 3),
    "augment.py": (2, 2),
    "config.py": (3, 0),
    "export.py": (1, 3),
    "multimodal.py": (1, 1),
    "privacy.py": (1, 0),
}

#: 本轮**收窄的那 8 格**在名册里的位置（文件名 ⇒ 收窄格数）——逐格点名的骨架。
NARROWED = {
    "data.py": 3,
    "augment.py": 2,
    "export.py": 2,
    "multimodal.py": 1,
}

ITEMS = [{"instruction": "问句 %d 号" % i, "output": "答句 %d 号" % i} for i in range(5)]


def _call_sites(filename, func_name):
    """AST 数某只路由文件里对某个函数名的调用点（不含 import 行）"""
    tree = ast.parse((ROUTES / filename).read_text(encoding="utf-8"))
    hits = 0
    for node in ast.walk(tree):
        if (isinstance(node, ast.Call)
                and getattr(node.func, "id", None) == func_name):
            hits += 1
    return hits


@pytest.fixture()
def client():
    return TestClient(app)


@pytest.fixture()
def corpus(tmp_path, monkeypatch):
    """三份语料：好数据 / 坏 JSON / 顶层不是数组

    白名单要**显式**加上 `tmp_path`（conftest 那只 autouse fixture 只把系统临时目录
    整个放进去，而 `resolve_within_roots` 走的是「相对路径取第一个存在的候选」
    ⇒ 语料必须落在白名单根里才找得到）。
    """
    monkeypatch.setenv("AUGMENTOR_DATA_ROOTS",
                       os.pathsep.join([os.getcwd(), str(tmp_path)]))
    ok = tmp_path / "ok.json"
    ok.write_text(json.dumps(ITEMS, ensure_ascii=False), encoding="utf-8")
    bad = tmp_path / "bad.json"
    bad.write_text("{not json", encoding="utf-8")
    obj = tmp_path / "obj.json"
    obj.write_text(json.dumps({"a": 1}, ensure_ascii=False), encoding="utf-8")
    return {"ok": ok, "bad": bad, "obj": obj}


class TestTheRoster:
    """名册面：一刀切那 18 处收窄到 10 处，边界口新增 8 格"""

    def test_the_roster_is_exact(self):
        got = {}
        for filename in ROSTER:
            got[filename] = (
                _call_sites(filename, "raise_internal_error"),
                _call_sites(filename, "raise_boundary_error"),
            )
        assert got == ROSTER, got

    def test_the_one_shot_family_shrank_to_ten(self):
        total = sum(_call_sites(f, "raise_internal_error") for f in ROSTER)
        assert total == 10, total

    def test_eight_more_sites_go_through_the_boundary(self):
        """本轮新增 8 格边界口（`api/routes/export.py` 那三格里有一格是 L311 就有的）"""
        total = sum(_call_sites(f, "raise_boundary_error") for f in ROSTER)
        assert total == 9, total
        assert sum(NARROWED.values()) == 8

    def test_the_blanket_value_error_arm_is_gone_from_multimodal(self):
        """`api/routes/multimodal.py` 那一处 `except ValueError` 泛档摘掉了（**按类型放行**那一条）

        另注：`data.py` 里那两处 `except (ValueError, UnicodeDecodeError)` **不是**
        门口那一类 —— 它们在一个「这份内容像不像 JSON」的**判断函数**体内
        （`except` ⇒ 返回 `False`），不参与路由的异常归宿。
        """
        src = (ROUTES / "multimodal.py").read_text(encoding="utf-8")
        tree = ast.parse(src)
        arms = 0
        for node in ast.walk(tree):
            if not isinstance(node, ast.Try):
                continue
            for handler in node.handlers:
                exc = handler.type
                names = []
                if isinstance(exc, ast.Name):
                    names = [exc.id]
                elif isinstance(exc, ast.Tuple):
                    names = [e.id for e in exc.elts if isinstance(e, ast.Name)]
                if "ValueError" in names:
                    arms += 1
        assert arms == 0, arms


class TestTheDomainVerdictsAreFourHundreds:
    """判决面：本轮翻面的 8 格，逐格点名（400 + 那句判决）"""

    def test_the_malformed_json_file_is_a_client_error(self, client, corpus):
        """坏 JSON 文件（客户端那份文件坏了）⇒ 400 + 那句中文判决（三处逐格点名）"""
        for path in ("/api/data/load/%s", "/api/analyze/%s", "/api/visualize/%s"):
            resp = client.get(path % corpus["bad"].name)
            assert resp.status_code == 400, (path, resp.text[:160])
            assert "不是合法 JSON" in resp.json()["detail"], (path, resp.text[:160])

    def test_the_wrong_top_level_shape_is_a_client_error(self, client, corpus):
        """顶层不是数组 ⇒ 400 +「顶层必须是 JSON 数组」（A280 / l251 那一格）"""
        resp = client.get("/api/data/load/%s" % corpus["obj"].name)
        assert resp.status_code == 400, resp.text[:160]
        assert "顶层必须是 JSON 数组" in resp.json()["detail"], resp.text[:160]

    def test_a_bad_task_id_is_a_domain_verdict(self, client):
        """坏任务 id（冒号盘符那一档）⇒ 400 + 那句域判决（两处逐格点名）"""
        resp = client.delete("/api/augment/checkpoints/C:evil")
        assert resp.status_code == 400, resp.text[:160]
        assert "不能含冒号" in resp.json()["detail"], resp.text[:160]

        resp = client.get("/api/augment/progress?task_id=C:evil")
        assert resp.status_code == 400, resp.text[:160]
        assert "不能含冒号" in resp.json()["detail"], resp.text[:160]

    def test_the_preview_knobs_are_domain_verdicts(self, client, corpus):
        """预览那一支的两格：非法条数与不支持的格式 ⇒ 400 + 点名那个旋钮/格式"""
        resp = client.post("/api/export/preview",
                           json={"input_file": str(corpus["ok"]), "size": -1})
        assert resp.status_code == 400, resp.text[:160]
        assert "preview_size" in resp.json()["detail"], resp.text[:160]

        resp = client.post("/api/export/preview",
                           json={"input_file": str(corpus["ok"]), "format": "nope"})
        assert resp.status_code == 400, resp.text[:160]
        assert "不支持的导出格式" in resp.json()["detail"], resp.text[:160]

    def test_the_unreadable_directory_is_still_four_hundred(self, client, tmp_path):
        """目录不存在那一格**一格未动**（改前 400、改后仍 400，同一句判决）"""
        resp = client.post("/api/multimodal/scan", json={"directory": str(tmp_path / "nope")})
        assert resp.status_code == 400, resp.text[:160]
        assert "目录不存在" in resp.json()["detail"], resp.text[:160]

    def test_the_missing_file_face_is_untouched(self, client, tmp_path):
        resp = client.get("/api/data/load/%s" % (tmp_path / "nope.json").name)
        assert resp.status_code == 404, resp.text[:160]

    def test_the_good_bodies_still_succeed(self, client, corpus):
        """合法请求仍是 200（判据没有把合法域收窄）"""
        for path in ("/api/data/load/%s", "/api/analyze/%s"):
            resp = client.get(path % corpus["ok"].name)
            assert resp.status_code == 200, (path, resp.text[:160])

        resp = client.post("/api/export/preview",
                           json={"input_file": str(corpus["ok"]), "size": 5})
        assert resp.status_code == 200, resp.text[:160]

        resp = client.post("/api/privacy/sanitize", json={"input_file": str(corpus["ok"])})
        assert resp.status_code == 200, resp.text[:160]


class TestTheNarrowingStaysNarrow:
    """收窄面：内部异常仍是 500 + 固定文案，且内部原文不外泄"""

    def test_an_internal_error_is_still_an_internal_error(self, client, corpus, monkeypatch):
        """分析那一支的下游抛一个内部异常 ⇒ 500 + 固定文案，**原文不外泄**"""
        from api.deps import get_pipeline

        def boom(*args, **kwargs):
            raise RuntimeError("内部细节不该外泄")

        monkeypatch.setattr(get_pipeline(), "analyze_dataset", boom)
        resp = client.get("/api/analyze/%s" % corpus["ok"].name)
        assert resp.status_code == 500, resp.text[:160]
        assert resp.json()["detail"] == INTERNAL_ERROR_DETAIL, resp.text[:160]
        assert "内部细节不该外泄" not in resp.text, resp.text[:160]

    def test_a_plain_value_error_is_an_internal_error_now(self, client, corpus, monkeypatch):
        """**泛档那一族摘掉之后**：内部 `ValueError` ⇒ 500（改前 400 + 原文）"""
        from api.deps import get_pipeline

        def boom(*args, **kwargs):
            raise ValueError("内部细节不该外泄")

        monkeypatch.setattr(get_pipeline(), "analyze_dataset", boom)
        resp = client.get("/api/analyze/%s" % corpus["ok"].name)
        assert resp.status_code == 500, resp.text[:160]
        assert resp.json()["detail"] == INTERNAL_ERROR_DETAIL, resp.text[:160]
        assert "内部细节不该外泄" not in resp.text, resp.text[:160]


class TestTheReverseArm:
    """反向臂：具名窄口是承重的 —— 摘掉它，那一格回到 500"""

    def test_dropping_the_named_port_puts_the_scan_back_to_500(self, client, tmp_path,
                                                               monkeypatch):
        import api.routes.multimodal as route_mod

        def _no_named_port(exc, *, not_found="资源不存在", not_found_types=(),
                           boundary_types=()):
            """把 `boundary_types` 忽略掉的那把门口（= 改前那一档）"""
            if not_found_types and isinstance(exc, not_found_types):
                raise HTTPException(status_code=404, detail=not_found)
            if isinstance(exc, DataValidationError):
                raise HTTPException(status_code=400, detail=str(exc))
            from api.deps import raise_internal_error
            raise_internal_error(exc)

        monkeypatch.setattr(route_mod, "raise_boundary_error", _no_named_port)
        resp = client.post("/api/multimodal/scan", json={"directory": str(tmp_path / "nope")})
        assert resp.status_code == 500, resp.text[:160]
        assert resp.json()["detail"] == INTERNAL_ERROR_DETAIL, resp.text[:160]


class TestTheBoundaryItself:
    """那把门口的契约：坏 JSON 那一档与 `to_http_error` **逐字同源**"""

    def test_the_json_arm_uses_the_shared_chinese_wording(self):
        from api.deps import _json_decode_detail

        exc = json.JSONDecodeError("Expecting value", "{not json", 1)
        assert _json_decode_detail(exc) == "数据文件不是合法 JSON（第 1 行第 2 列）"

    def test_the_json_arm_is_four_hundred_on_the_boundary(self):
        exc = json.JSONDecodeError("Expecting value", "{not json", 1)
        with pytest.raises(HTTPException) as caught:
            from api.deps import raise_boundary_error
            raise_boundary_error(exc)
        assert caught.value.status_code == 400
        assert caught.value.detail == "数据文件不是合法 JSON（第 1 行第 2 列）"

    def test_the_named_port_types_are_the_ones_we_measured(self):
        """本轮那一族具名窄口点的是这两只（目录不可读 / 坏图片）—— 不是父类"""
        assert issubclass(DataLoadError, ValueError)
        assert issubclass(DataFormatError, ValueError)
        assert not issubclass(DataLoadError, DataValidationError)
        assert not issubclass(DataFormatError, DataValidationError)
