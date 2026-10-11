# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L236：最后两员接进页面之后，消费面到 75/75——落盘口、零判决与「取法」三面都不许漂

**这一轮收的是哪两员**：`POST /api/dataset/translate`（l224 缺口名单上最后一条挂鉴权的写操作）
与 `GET /api/system/dependencies`（L226 以「指标涨而能力不涨」拒接过的那一条）。用户拍板
「两员都接，做到 75/75」，所以本轮把缺口名单收到 **∅**（名单空了不等于尺子钝了，那一面的
反证住在 `test_api_endpoint_consumption_l224.py` 里，本文件不重复钉）。

**改前 / 改后**（三份仓外探针实测，跑完即删；下面每一格判据都指回其中一条读数）：

| 事实 | 改前 | 改后 |
| --- | --- | --- |
| 前端调用 `/api/dataset/translate` | **0 处** | `translateDataset` + 数据管理页一张卡（三道闸） |
| 前端调用 `/api/system/dependencies` | **0 处** | `systemDependencies` + 系统页「依赖诊断」卡 |
| 请求里送 `output_file` | **200 且盘上零产物**（键被静默丢掉） | 200 + 原子落盘 + 回显已 resolve 的绝对路径 |
| 越界落点 | —— | **403**，且模型调用 **0** 次（落点先解析、再动模型） |
| 系统页依赖明细的门 | `{status && …}`（综合状态拉不动就整块消失） | `{diag && …}`（自取一面 ?? 综合状态里那一份） |

**为什么接 dependencies 不算「指标涨而能力不涨」**：L226 那条裁定在**数据面**上成立——探针实测
两条端点交回的诊断**逐字节相同**（都出自 `augmentor.diagnostics.check_dependencies`）。涨的是
**取法**：`/api/status` 要真构造管道，`/api/system/dependencies` 一次都不碰。决定性的一格是把
`api.routes.status.get_pipeline` 换成抛异常的替身：dependencies 的读数**一格未变**，而 status
退成 `model_default='unknown'`（两支都仍 200）。页面因此能把依赖诊断从整体门里摘出来单独刷新，
这也正是 `TestTheSystemPageOwnsTheDependencyFace` 钉的东西。

**A322（本轮新立，后端未改）**：`text_key` 那一格**零判决**——送空串或送一个数据集里不存在的键
都 200、模型调用 **0** 次、`failed=0`、`errors=[]`，产物里凭空多出 `<key>`、`<key>_original`
与 `language` **三键**（第三键是 `augmentor/multilingual.py` 无条件盖的目标语言戳），而原字段一个字没动。
它比 L221 那族「静默回退原文」更容易骗过去：回执四键全是零值、产物还带着 `language: en`，
从任何一面都读不出「这次什么都没发生」。本轮只在页面上补一道闸把空值拦下、并把「别把 200 且
failed=0 读成翻成了」写在卡上；**后端那一格的判决留给 A322 自己那一轮**，所以本文件把「后端
确实零判决」钉成现量（`TestAThreeTwoTwo`），改后端的那天这两支会红，提醒同步页面文案与卡上那句。

跑法：`python -m pytest tests/unit/test_last_two_wiring_l236.py -q --no-cov`
"""

import json
import pathlib
import re
from types import SimpleNamespace

import pytest
from tests.integration.test_translate_failure_receipt_l221 import (
    SelectiveBackend,
    language_config,
)
from tests.unit.test_transform_wiring_l229 import (
    _page,
    fetcher_body,
    state_initial,
    ts_interface_keys,
)
from tests.unit.test_wired_dropdown_choices_l226 import needs_web_src, strip_comments

from api.routes.language import TranslateRequest, TranslateResponse

ROOT = pathlib.Path(__file__).resolve().parents[2]
SRC = ROOT / "web" / "src"
SYSTEM_TSX = SRC / "pages" / "System.tsx"

#: 3 条中文语料：`食物1` 是部分失败那一支的靶子（与 L221 那份回执用例同一口径）
ITEMS = [{"instruction": f"猫喜欢吃什么食物{i}", "output": "猫粮"} for i in range(3)]

#: `translateDataset` 的请求体字面量——只认 `api.post('/dataset/translate', {…})` 里那一层，
#: 不用「4 空格缩进 + 冒号」去猜：fetcher 的签名里也有一块 4 空格缩进的 `opts: {…}` 类型字面量，
#: 那种读法会把 TS 侧的驼峰名一起抓进来，与契约的蛇形名对不上（首跑实测多出 4 员）
POST_BODY = re.compile(r"api\.post\('/dataset/translate',\s*\{(.*?)\}\)", re.S)
FIELD = re.compile(r"^\s+(\w+):", re.M)


def _system_page() -> str:
    assert SYSTEM_TSX.is_file(), "前端源码不在场 ⇒ 本文件的判据全部失效"
    return strip_comments(SYSTEM_TSX.read_text(encoding="utf-8"))


@pytest.fixture
def http(tmp_path, monkeypatch):
    """白名单数据根 = `tmp_path/ds`（内有 `p.json`），根**外**另有 `tmp_path/outside/`

    管道用替身：翻译那条路径逐条调 `model_backend.generate`，只有替身能数出「花了几次钱」，
    而「模型调用 0 次」正是本文件三格判据（403 先于花钱、清单外目标语言、A322）的读数。
    """
    root = tmp_path / "ds"
    root.mkdir()
    outside = tmp_path / "outside"
    outside.mkdir()
    (root / "p.json").write_text(json.dumps(ITEMS, ensure_ascii=False), encoding="utf-8")
    monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", str(root))
    monkeypatch.chdir(tmp_path)

    import api.deps as deps
    from api.main import app
    from fastapi.testclient import TestClient

    def build(backend, **overrides):
        config = language_config(tmp_path, **{"multilingual.enabled": True, **overrides})
        stub = type("PipelineStub", (), {})()
        stub.config = config
        stub.model_backend = backend
        monkeypatch.setattr(deps, "_pipeline", stub)
        return TestClient(app)

    def landed(name):
        return json.loads((root / name).read_text(encoding="utf-8"))

    return SimpleNamespace(build=build, landed=landed, root=root, outside=outside)


@needs_web_src
class TestThePageDefaultsAreTheContractDefaults:
    """第 1 面：页面那几格初值不许自带第二套默认（权威序：请求参数 > 配置键 > 内置默认）"""

    def test_the_four_inputs_start_at_the_request_defaults(self):
        """四格初值逐项对到 `TranslateRequest.model_fields`：空串对 `None`，其余对同值

        空串 ↔ `None` 不是将就，是探针量过的等价：`target_lang` 送空串与省略都落到
        `multilingual.default_target_lang`（en），`output_file` 送空串与省略都不落盘。所以页面
        「留空」这一档正好对上契约「不给」那一档，不需要在 TS 侧再写一份默认值。
        """
        fields = TranslateRequest.model_fields
        assert fields["target_lang"].default is None
        assert fields["text_key"].default == "instruction"
        assert fields["keep_original"].default is True
        assert fields["output_file"].default is None

        assert state_initial("trOut") == "''"
        assert state_initial("trLang") == "''"
        assert state_initial("trTextKey") == "'instruction'"
        assert state_initial("trKeep") == "true"

    def test_the_billing_ack_has_no_contract_counterpart(self):
        """`trAck` 是页面自己加的那道闸，契约里没有它——不许被读成「后端默认不确认」

        这一格同时钉两处接线：按钮 `disabled={!trAck}`（点不动）与 handler 里的
        `if (!trAck)`（真点到了也不发）。两处拦的是同一件事的两面，少一处就只剩样式。
        """
        assert not any("ack" in name for name in TranslateRequest.model_fields)
        assert state_initial("trAck") == "false"
        page = _page()
        assert "disabled={!trAck}" in page
        assert "if (!trAck)" in page

    def test_ts_response_keys_equal_the_response_model_in_order(self):
        """TS 那份接口逐键、逐序等于响应模型：少写一只键，读数就在页面上凭空消失"""
        assert ts_interface_keys("TranslateResponse") == list(TranslateResponse.model_fields)

    def test_ts_error_sample_keys_equal_what_the_sdk_reports(self):
        """`errors` 的元素只有 `index` / `error` 两键（SDK 侧形状，L221 定标）"""
        assert ts_interface_keys("TranslateErrorSample") == ["index", "error"]

    def test_the_fetcher_sends_exactly_the_contract_fields(self):
        """请求体里那几只键**集合相等**于契约字段：多送一只会被 pydantic 静默忽略，少送一只就少一个旋钮"""
        body = fetcher_body("translateDataset")
        found = POST_BODY.search(body)
        assert found, "translateDataset 不再往 /dataset/translate 发请求体 ⇒ 判据要跟着换形状"
        assert set(FIELD.findall(found.group(1))) == set(TranslateRequest.model_fields)


@needs_web_src
class TestTheLandingPort:
    """第 2 面：A283① 的落盘口——回显、零产物、403 先于花钱、部分失败、契约文档"""

    def test_output_file_lands_on_disk_and_echoes_the_resolved_path(self, http):
        backend = SelectiveBackend()
        client = http.build(backend)

        resp = client.post("/api/dataset/translate", json={
            "input_file": "p.json", "output_file": "translated.json"})

        assert resp.status_code == 200, resp.text
        body = resp.json()
        target = http.root / "translated.json"
        assert target.is_file(), "给了落盘口却没落盘 ⇒ A283 那一格又变回只写不读"
        echo = pathlib.Path(body["output_file"])
        assert echo.is_absolute()
        assert echo.resolve() == target.resolve()
        # 盘上那份与响应里的 items 逐格相同：回执不是「另算一遍」，是同一份产物的两个出口
        assert http.landed("translated.json") == body["items"]
        assert backend.calls == 3

    def test_omitting_it_keeps_the_key_present_and_null_and_writes_nothing(self, http):
        """键**恒在**、值为 null：用「键在不在」区分不了「没落盘」与「落了盘没回显」"""
        client = http.build(SelectiveBackend())
        before = sorted(p.name for p in http.root.iterdir())

        resp = client.post("/api/dataset/translate", json={"input_file": "p.json"})

        assert resp.status_code == 200, resp.text
        body = resp.json()
        assert "output_file" in body
        assert body["output_file"] is None
        assert sorted(p.name for p in http.root.iterdir()) == before

    def test_out_of_root_output_is_403_before_any_model_call(self, http):
        """越界落点 403，且模型调用 **0** 次——花钱路径上的参数错误不该先花一笔才知道"""
        backend = SelectiveBackend()
        client = http.build(backend)
        escaped = http.outside / "escaped.json"

        resp = client.post("/api/dataset/translate", json={
            "input_file": "p.json", "output_file": str(escaped)})

        assert resp.status_code == 403, resp.text
        assert backend.calls == 0
        assert not escaped.exists()
        assert sorted(p.name for p in http.root.iterdir()) == ["p.json"]

    def test_partial_failure_still_200_and_the_disk_copy_carries_source_text(self, http):
        """落盘那份同样混着原文：产物本身不诚实，诚实读数只在回执四键上（L221 的口径延伸）"""
        client = http.build(SelectiveBackend(fail_on=["食物1"]))

        resp = client.post("/api/dataset/translate", json={
            "input_file": "p.json", "output_file": "partial.json"})

        assert resp.status_code == 200, resp.text
        body = resp.json()
        assert body["failed"] == 1
        assert body["failed_indices"] == [1]
        on_disk = http.landed("partial.json")
        assert on_disk == body["items"]
        assert on_disk[1]["instruction"] == "猫喜欢吃什么食物1"
        assert on_disk[0]["instruction"] == "TRANSLATED"

    def test_openapi_declares_the_key_on_both_sides(self):
        """请求与响应两侧都声明，且两侧都**不**必填（只改模型不进 OpenAPI 是另一种「写了没人看得见」）"""
        from api.main import app

        schemas = app.openapi()["components"]["schemas"]
        request = schemas["TranslateRequest"]
        response = schemas["TranslateResponse"]

        assert "output_file" in request["properties"]
        assert "output_file" not in request.get("required", [])
        assert "output_file" in response["properties"]
        assert "output_file" not in response.get("required", [])


@needs_web_src
class TestAThreeTwoTwo:
    """第 3 面：A322 —— `text_key` 后端零判决，页面补的那道闸是唯一判决"""

    def test_empty_text_key_is_a_silent_no_op(self, http):
        """空串 ⇒ 200 / 模型 0 次 / `failed=0` / 产物凭空多三键，而原字段一个字没动

        第三键是**这一格最难看见的部分**（本文件首跑量出来的，探针当时只报了多两键）：
        `augmentor/multilingual.py` 无条件盖 `new_item["language"] = target_lang`，于是一份
        一个字都没翻的产物上照样写着 `language: en`。回执四键全零 + 产物带语言戳 ⇒ 从任何
        一面都读不出「这次什么都没发生」。
        """
        backend = SelectiveBackend()
        client = http.build(backend)

        resp = client.post("/api/dataset/translate", json={
            "input_file": "p.json", "text_key": "", "output_file": "empty_key.json"})

        assert resp.status_code == 200, resp.text
        body = resp.json()
        assert backend.calls == 0
        assert body["failed"] == 0
        assert body["failed_indices"] == []
        assert body["errors"] == []
        on_disk = http.landed("empty_key.json")
        assert set(on_disk[0]) == {"instruction", "output", "", "_original", "language"}
        assert on_disk[0]["instruction"] == "猫喜欢吃什么食物0"
        assert on_disk[0]["language"] == "en"
        assert on_disk[0][""] == ""

    def test_ghost_text_key_is_a_silent_no_op(self, http):
        """不存在的键同档：200 / 0 次 / 回执全零，产物多 `no_such_field`、它的 `_original` 与语言戳"""
        backend = SelectiveBackend()
        client = http.build(backend)

        resp = client.post("/api/dataset/translate", json={
            "input_file": "p.json", "text_key": "no_such_field",
            "output_file": "ghost_key.json"})

        assert resp.status_code == 200, resp.text
        assert backend.calls == 0
        assert resp.json()["failed"] == 0
        on_disk = http.landed("ghost_key.json")
        assert set(on_disk[0]) == {
            "instruction", "output", "no_such_field", "no_such_field_original", "language"}
        assert on_disk[0]["instruction"] == "猫喜欢吃什么食物0"
        assert on_disk[0]["no_such_field"] == ""

    def test_the_page_gate_is_the_only_judgement_and_says_so(self):
        """页面拦空值 + 卡上写明这一格后端零判决；改了后端那天这两句要一起动"""
        page = _page()
        assert "if (!trTextKey.trim())" in page
        assert "A322" in page
        assert "别把「200 且 failed=0」读成翻成了" in page


@needs_web_src
class TestTheSystemPageOwnsTheDependencyFace:
    """第 4 面：dependencies 那一员——数据相同、取法不同，页面因此能单独刷新"""

    def test_both_faces_carry_identical_diagnostics(self, http):
        """两条端点交回的诊断逐格相同（都出自 `check_dependencies`）：L226 的拒接理由在数据面成立"""
        client = http.build(SelectiveBackend())

        status = client.get("/api/status")
        deps = client.get("/api/system/dependencies")

        assert status.status_code == 200, status.text
        assert deps.status_code == 200, deps.text
        assert status.json()["dependencies"] == deps.json()

    def test_dependencies_never_touches_the_pipeline_while_status_degrades(self, http, monkeypatch):
        """决定性的一格：管道炸掉时 dependencies 读数**一格未变**，status 退成 `unknown`

        这就是「涨的是取法不是数据」的实证，也是页面敢把依赖诊断从整体门里摘出来的依据。
        """
        import api.routes.status as status_route

        client = http.build(SelectiveBackend())
        baseline = client.get("/api/system/dependencies").json()

        def boom():
            raise RuntimeError("pipeline is down")

        monkeypatch.setattr(status_route, "get_pipeline", boom)

        still = client.get("/api/system/dependencies")
        degraded = client.get("/api/status")

        assert still.status_code == 200, still.text
        assert still.json() == baseline
        assert degraded.status_code == 200, degraded.text
        assert degraded.json()["model_default"] == "unknown"
        assert degraded.json()["model_available"] is False
        # 综合状态里那一份诊断照旧齐备 ⇒ 页面「自取失败就退回它」这条退路不是空的
        assert degraded.json()["dependencies"] == baseline

    def test_the_fetcher_hits_the_read_path(self):
        assert "api.get('/system/dependencies')" in fetcher_body("systemDependencies")

    def test_the_page_fetches_it_itself_and_keeps_the_fallback(self):
        """页面自取一面，同时留着「退回综合状态里那一份」这条退路"""
        page = _system_page()
        assert "systemDependencies" in page.split("export default")[0], "System.tsx 没导入它"
        assert "await systemDependencies()" in page
        assert "deps ?? status?.dependencies ?? null" in page

    def test_the_detail_cards_are_no_longer_behind_the_status_gate(self):
        """那道整体门没了：明细与降级功能现在的门是 `diag`，综合状态拉不动也看得见"""
        page = _system_page()
        assert "{diag && (" in page
        assert "{status && (" not in page
        assert "刷新依赖诊断" in page
