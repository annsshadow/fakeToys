# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L315 守卫：A280 的两扇门口逐处处置完毕（一刀切 10 处 + 分档 35 处）

**这一轮收的是哪一格**：**A280**（L224 立，L311/L313/L314 三批逐文件收窄）的最后一格 ——
**「8 个路由文件把下游异常一刀切收敛成 500」这一族**。

**两扇门口、45 处，逐处处置**：

* **一刀切那 10 处**（`raise_internal_error`）：`api/routes/augment.py` 2 / `api/routes/config.py` 3 /
  `api/routes/data.py` 2 / `api/routes/export.py` 1 / `api/routes/multimodal.py` 1 /
  `api/routes/privacy.py` 1。**逐处实测下游的异常类型** ⇒ **每一处的下游都抛不出域判决**
  （域判决早已被 `HTTPException` 那一支接住；写边的 `OSError` 是服务端状态；那几支没有
  客户端入参；配置坏了是服务端自己的问题）⇒ **保持 500 + 固定文案**（A280 记的方向②
  「维持现状并把契约写下来」）。**「资源不存在」那一面早已是 404**（`resolve_data_path`
  对读操作查存在性 ⇒ 404；`resolve_within_roots` 越界 ⇒ 403）。
* **分档那 37 处**（`to_http_error`）：`api/routes/dataset_tools.py` 14 / `api/routes/system_ops.py` 16 /
  `api/routes/language.py` 2 / `api/routes/vector.py` 2 / `api/routes/config.py` 1 /
  `api/routes/data.py` 2。那扇门已经分档：`FileNotFoundError` → 404、`JSONDecodeError` → 400
  （中文判决）、**具名域判决** → 400 + 原文、其余 → 500 + 固定文案 ⇒ **「资源不存在拿不到 404」
  这一格在全仓都不再成立**。

**泛档那一格已在 L316 收掉**（A386）：L315 当时那扇门按父类 `ValueError` 放行 ⇒ 内部异常
也会被回成 400 判决、并把**内部原文**发给客户端；**L316 起那扇门只放行具名域判决**
（`CLIENT_FAULT_TYPES`），裸 `ValueError` 一律 500 + 固定文案 ⇒ 本文件里那一支按现量钉住的
用例已随 L316 改成**正判决**（裸 `ValueError` ⇒ 500 + 固定文案；具名域判决 ⇒ 400 + 原文）。

**四道面各自要能单独喊红**：

1. **名册面**（AST）：一刀切那 10 处与分档那 37 处的**逐文件**调用点数 ⇒ 谁再塞一处一刀切、
   或把某一处挪了地方，这一张名册当场红。
2. **判决面**：一刀切那 10 处**逐处**点名（注入一个内部异常 ⇒ 500 + 固定文案 + **原文不外泄**）；
   分档那扇门的**四档**逐档点名。
3. **契约面**：`docs/augmentor/API.md` 里写明「域判决 400 / 资源不存在 404 / 其余 500 + 固定文案」
   那一段 ⇒ 只钉代码不钉文档，等于把契约留在实现里。

**为什么零 parametrize**：`test_parametrize_ids_l97` 与 `test_parametrize_blind_face_l98`
把全仓 parametrize 三面冻成现量数，本守卫走具名用例，那两把尺子一格未动。

跑法：`python -m pytest tests/unit/test_error_routing_roster_l315.py -q --no-cov`
"""
import ast
import json
import os
from pathlib import Path

import pytest
from fastapi.testclient import TestClient

from api.deps import INTERNAL_ERROR_DETAIL
from api.main import app

PRODUCT_ROOT = Path(__file__).resolve().parents[2]
ROUTES = PRODUCT_ROOT / "api" / "routes"
API_DOC = PRODUCT_ROOT.parent / "docs" / "augmentor" / "API.md"

#: 一刀切那一族（`raise_internal_error`）的**逐文件**现量名册。
ONE_SHOT_ROSTER = {
    "augment.py": 2,
    "config.py": 3,
    "data.py": 2,
    "export.py": 1,
    "multimodal.py": 1,
    "privacy.py": 1,
}

#: 分档那一族（`to_http_error`）的**逐文件**现量名册。
SPLIT_ROSTER = {
    "config.py": 1,
    "data.py": 2,
    "dataset_tools.py": 14,
    "language.py": 2,
    "system_ops.py": 16,  # **L338**：14 → 16
    "vector.py": 2,
}

ITEMS = [{"instruction": "问句 %d 号" % i, "output": "答句 %d 号" % i} for i in range(3)]


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
    return TestClient(app, raise_server_exceptions=False)


@pytest.fixture()
def corpus(tmp_path, monkeypatch):
    """一份好数据 + 把临时目录放进路径白名单（零写入仓库 `data/`）"""
    monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", os.pathsep.join([os.getcwd(), str(tmp_path)]))
    path = tmp_path / "a.json"
    path.write_text(json.dumps(ITEMS, ensure_ascii=False), encoding="utf-8")
    return {"ok": path, "dir": tmp_path}


class TestTheRoutingRoster:
    """名册面：两扇门口一共 45 处，逐文件点名"""

    def test_the_one_shot_family_is_exactly_ten_sites(self):
        got = {fn: _call_sites(fn, "raise_internal_error") for fn in ONE_SHOT_ROSTER}
        assert got == ONE_SHOT_ROSTER, got
        assert sum(got.values()) == 10, sum(got.values())

    def test_the_split_door_family_is_exactly_thirty_five_sites(self):
        got = {fn: _call_sites(fn, "to_http_error") for fn in SPLIT_ROSTER}
        assert got == SPLIT_ROSTER, got
        assert sum(got.values()) == 37, sum(got.values())  # **L338**：35 → 37

    def test_no_route_file_uses_a_catch_all_outside_the_roster(self):
        """名册之外没有第三处一刀切（谁新塞一处 ⇒ 这里当场红）"""
        seen = {}
        for path in sorted(ROUTES.glob("*.py")):
            hits = _call_sites(path.name, "raise_internal_error")
            if hits:
                seen[path.name] = hits
        assert seen == ONE_SHOT_ROSTER, seen


class TestTheOneShotSitesKeepTheirVerdict:
    """判决面：一刀切那 10 处逐处点名（内部异常 ⇒ 500 + 固定文案 + 原文不外泄）"""

    def _assert_internal_error(self, response):
        assert response.status_code == 500, response.text[:200]
        assert response.json()["detail"] == INTERNAL_ERROR_DETAIL, response.text[:200]
        assert "内部细节不该外泄" not in response.text, response.text[:200]

    def test_augment_start(self, client, corpus, monkeypatch):
        import api.routes.augment as route_mod

        def boom():
            raise RuntimeError("内部细节不该外泄")

        monkeypatch.setattr(route_mod, "get_pipeline", boom)
        response = client.post("/api/augment/start", json={
            "input_file": str(corpus["ok"]), "output_file": str(corpus["dir"] / "out.json")})
        self._assert_internal_error(response)

    def test_augment_checkpoint_list(self, client, corpus, monkeypatch):
        import api.routes.augment as route_mod

        def boom():
            raise RuntimeError("内部细节不该外泄")

        monkeypatch.setattr(route_mod, "get_pipeline", boom)
        self._assert_internal_error(client.get("/api/augment/checkpoints"))

    def test_config_read(self, client, corpus, monkeypatch):
        import api.routes.config as route_mod

        def boom():
            raise RuntimeError("内部细节不该外泄")

        monkeypatch.setattr(route_mod, "get_pipeline", boom)
        self._assert_internal_error(client.get("/api/config"))

    def test_config_update(self, client, corpus, monkeypatch):
        import api.routes.config as route_mod

        def boom():
            raise RuntimeError("内部细节不该外泄")

        monkeypatch.setattr(route_mod, "get_pipeline", boom)
        self._assert_internal_error(client.post("/api/config", json={"quality": {"threshold": 0.7}}))

    def test_config_models(self, client, corpus, monkeypatch):
        import api.routes.config as route_mod

        def boom():
            raise RuntimeError("内部细节不该外泄")

        monkeypatch.setattr(route_mod, "get_pipeline", boom)
        self._assert_internal_error(client.get("/api/models"))

    def test_data_update(self, client, corpus, monkeypatch):
        import api.routes.data as route_mod

        def boom(*args, **kwargs):
            raise RuntimeError("内部细节不该外泄")

        monkeypatch.setattr(route_mod, "mutate_json_file", boom)
        response = client.put("/api/data/update/%s?index=0" % corpus["ok"].name,
                              json={"instruction": "x"})
        self._assert_internal_error(response)

    def test_data_delete(self, client, corpus, monkeypatch):
        import api.routes.data as route_mod

        def boom(*args, **kwargs):
            raise RuntimeError("内部细节不该外泄")

        monkeypatch.setattr(route_mod, "mutate_json_file", boom)
        response = client.delete("/api/data/delete/%s?index=0" % corpus["ok"].name)
        self._assert_internal_error(response)

    def test_export_formats(self, client, corpus, monkeypatch):
        import augmentor.export as export_module

        def boom(*args, **kwargs):
            raise RuntimeError("内部细节不该外泄")

        monkeypatch.setattr(export_module, "Exporter", boom)
        self._assert_internal_error(client.get("/api/export/formats"))

    def test_multimodal_process(self, client, corpus, monkeypatch):
        import augmentor.data as data_pkg

        def boom(*args, **kwargs):
            raise RuntimeError("内部细节不该外泄")

        monkeypatch.setattr(data_pkg, "MultimodalProcessor", boom)
        self._assert_internal_error(client.post("/api/multimodal/process", json={"text": "x"}))

    def test_privacy_sanitize(self, client, corpus, monkeypatch):
        import api.routes.privacy as route_mod

        def boom(*args, **kwargs):
            raise RuntimeError("内部细节不该外泄")

        monkeypatch.setattr(route_mod, "PiiSanitizer", boom)
        self._assert_internal_error(
            client.post("/api/privacy/sanitize", json={"input_file": str(corpus["ok"])}))


class TestTheMissingFileFace:
    """「资源不存在」那一面早已是 404（A280 那一行记的那一格）"""

    def test_the_read_paths_answer_404(self, client, corpus):
        response = client.post("/api/privacy/sanitize",
                               json={"input_file": str(corpus["dir"] / "nope.json")})
        assert response.status_code == 404, response.text[:200]

    def test_the_write_paths_answer_403_outside_the_roots(self, client, corpus, tmp_path):
        """越界落点 ⇒ 403（不是 404，也不是 500）"""
        response = client.post("/api/augment/start", json={
            "input_file": str(corpus["ok"]),
            "output_file": str(tmp_path.parent / "outside.json")})
        assert response.status_code == 403, response.text[:200]


class TestTheSplitDoorVerdicts:
    """分档那扇门的四档，逐档点名"""

    def _http_error(self, exc, **kwargs):
        from api.deps import to_http_error
        return to_http_error(exc, **kwargs)

    def test_a_missing_file_is_four_oh_four(self):
        error = self._http_error(FileNotFoundError("no file"))
        assert error.status_code == 404
        assert error.detail == "文件不存在"

    def test_a_broken_json_is_four_hundred_with_the_shared_wording(self):
        error = self._http_error(json.JSONDecodeError("Expecting value", "{not json", 1))
        assert error.status_code == 400
        assert error.detail == "数据文件不是合法 JSON（第 1 行第 2 列）"

    def test_a_value_error_is_four_hundred_with_the_raw_text(self):
        """**A386 / L316 收口**：泛档那一格已经关掉了 ⇒ 裸 `ValueError` 落 500 + 固定文案

        L315 当时是**按现量钉住**这一格（并点名它归 A386 / A155①）⇒ L316 把那一格收掉：
        分档门不再按父类 `ValueError` 放行，只放行**具名**域判决（见下一支）。
        """
        error = self._http_error(ValueError("判决原文"))
        assert error.status_code == 500
        assert error.detail == INTERNAL_ERROR_DETAIL
        assert "判决原文" not in error.detail

    def test_a_named_domain_verdict_is_four_hundred_with_the_raw_text(self):
        """具名域判决仍是 400 + 那句可行动的判决（这一族一格未动）"""
        from augmentor.exceptions import DataValidationError

        error = self._http_error(DataValidationError("判决原文"))
        assert error.status_code == 400
        assert error.detail == "判决原文"

    def test_anything_else_is_five_hundred_with_the_fixed_wording(self):
        error = self._http_error(RuntimeError("内部细节不该外泄"))
        assert error.status_code == 500
        assert error.detail == INTERNAL_ERROR_DETAIL
        assert "内部细节不该外泄" not in error.detail


class TestTheContractIsWrittenDown:
    """契约面：那一段话写在文档里，不只钉在代码里"""

    def test_the_api_doc_states_the_three_tiers(self):
        text = API_DOC.read_text(encoding="utf-8")
        assert "### 错误语义（三档）" in text, "文档里没有那一段「错误语义（三档）」"
        for needle in ("域判决", "404", "500"):
            assert needle in text, needle
        assert "服务器内部错误，详情见服务端日志" in text, "500 档的固定文案没写进文档"
        assert "A386" in text, "泛档那一格（归 A386）必须在文档里点名，不许悄悄留着"
