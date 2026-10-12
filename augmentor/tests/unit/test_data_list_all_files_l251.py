# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""`GET /api/data/list` 的 `all_files` 那一档（A304 / L251 收口轮）

**这一档收的是哪一格**：A304 记的是「候选口径窄于白名单」——端点只列
`train_data*.json`，而白名单根目录里 `convert` / `merge` / `sample` 那些由调用方命名的
产物根本进不了下拉。修法①（拍板走的那一条）是**只放开名字这道闸**：扫描范围仍然逐字来自
`allowed_data_roots()`，`all_files=true` 时不再看文件名。

**判据为什么长这样**（三族，缺一族就抓不到对应的漂移）：
1. **默认档零漂移**：不带参数、`false`、`0` 三档的**整条目**（含 `path` 与 `size`）必须逐格
   相同。只比名字不比条目，就抓不到「加档时顺手把 `path` 改成相对路径」这一类。
2. **宽档与白名单同宽**：判据是**集合相等**而不是「包含更多」。写成「宽档 ⊇ 默认档」的尺子
   会被「宽档把根外的 `secret.json` 也端出来」喂饱 —— 而那正是这一族要拦的方向。
3. **宽出去的一份也没越过边界**：`resolve` 后的根外文件、子目录里的文件（`glob` 不递归）、
   重复根去重，三格在**两档**下各量一遍。

**「列表能给的，路由必须能读」这一句在宽档下的现量**（`TestDataListScansWhitelist` 在
`test_api_security.py` 里立的那条承诺）：宽档列出的每一份喂回 `/api/data/load/{name}` 都**不拿
403 也不拿 404**——这一句管的是路径归属，不是内容形态。形态那一格本轮按现量钉：非数组那份拿到
**500 + 固定文案**（`load_data` 的 `except Exception` 把 `assert_dataset_shape` 那句 400 一起
吞了），它是 **A280**「一刀切收敛」那一族的**一格新增现量**，状态码是对外契约所以本轮不动它。
这一格之所以值得单独记：默认档根本不列这类文件，宽档第一次把它端到了列表上。

**契约面与前端面**：`all_files` 必须由 OpenAPI 声明为 `in=query` 的可选布尔且默认 `false`
（键集尺子与 l224 的 query 对账都读这一格）。前端只有系统状态页取宽档，而那一页六处候选下拉
共用同一份 state（备份源文件 / 数据集源文件 / 监控数据文件 / 源数据文件 / 迁移产物 / 流式产物），
所以变宽的是**这一页的六处**；其余 8 处数据文件选择器继续走默认档 —— 这一族用**文本计数**钉，
因为「放宽了这一格」与「顺手把全站的清单都放宽了」在响应面上看不出区别。

跑法：`python -m pytest tests/unit/test_data_list_all_files_l251.py -q --no-cov`
"""

import json
import os
import pathlib

import pytest
from tests.unit.test_wired_dropdown_choices_l226 import needs_web_src, strip_comments

ROOT = pathlib.Path(__file__).resolve().parents[2]
SRC = ROOT / "web" / "src"
API_TS = SRC / "services" / "api.ts"
SYSTEM_TSX = SRC / "pages" / "System.tsx"

WIDE = "/api/data/list?all_files=true"

#: 根目录里的四份顶层 JSON：两份命中默认档的前缀、两份只有宽档才看得见。
#: `nested_only.json` 故意放在**子目录**里：`glob("*.json")` 不递归，所以它两档都不该出现。
DATASET = [{"instruction": "问", "input": "", "output": "答"}]
TOP_LEVEL = ["configish.json", "p.json", "train_data1.json", "train_dataX.json"]


@pytest.fixture
def roots(tmp_path, monkeypatch):
    """白名单根 `ds/`（四份顶层 JSON + 一份子目录里的）与根外 `outside/secret.json`"""
    root = tmp_path / "ds"
    (root / "nested").mkdir(parents=True)
    outside = tmp_path / "outside"
    outside.mkdir()
    (root / "p.json").write_text(json.dumps(DATASET, ensure_ascii=False), encoding="utf-8")
    (root / "train_data1.json").write_text("[]", encoding="utf-8")
    (root / "train_dataX.json").write_text("[]", encoding="utf-8")
    (root / "configish.json").write_text('{"a": 1}', encoding="utf-8")
    (root / "nested" / "nested_only.json").write_text("[]", encoding="utf-8")
    (outside / "secret.json").write_text("[]", encoding="utf-8")
    monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", str(root))
    monkeypatch.chdir(tmp_path)
    from fastapi.testclient import TestClient
    from api.main import app

    return TestClient(app), root, outside


def names(files):
    return [f["name"] for f in files]


class TestDefaultModeIsUnchanged:
    """默认档零漂移：三种写法回**整条目**必须逐格相同"""

    def test_default_lists_only_the_prefix_shape(self, roots):
        http, *_ = roots
        assert names(http.get("/api/data/list").json()["files"]) == [
            "train_data1.json", "train_dataX.json"]

    def test_three_spellings_of_the_default_are_identical(self, roots):
        """`false` 与 `0` 都得落回默认档，且连 `path` / `size` 一起比

        这里不用 `parametrize`：l97 / l98 那两把普查尺子数的是全仓 parametrize 的档位，
        新增一档就得连带重钉它们的读数（L249 立下的「零 parametrize 新增」纪律）。
        """
        http, *_ = roots
        base = http.get("/api/data/list").json()["files"]
        for query in ("", "?all_files=false", "?all_files=0"):
            assert http.get(f"/api/data/list{query}").json()["files"] == base, query


class TestWideModeIsAsWideAsTheWhitelist:
    """宽档判据是**相等**：不多一份根外的，也不少一份根内的"""

    def test_wide_mode_returns_every_top_level_json(self, roots):
        http, *_ = roots
        assert names(http.get(WIDE).json()["files"]) == TOP_LEVEL

    def test_the_response_keys_are_untouched(self, roots):
        """加档没顺手加键：两档每一条都还是 `{name, path, size}` 三键"""
        http, *_ = roots
        for url in ("/api/data/list", WIDE):
            for entry in http.get(url).json()["files"]:
                assert set(entry) == {"name", "path", "size"}, (url, entry)

    def test_the_two_modes_agree_on_the_overlap_entry_for_entry(self, roots):
        """默认档那两条在宽档里逐格相同 ⇒ 宽档是「多列」不是「换算法」"""
        http, *_ = roots
        wide = {f["name"]: f for f in http.get(WIDE).json()["files"]}
        for entry in http.get("/api/data/list").json()["files"]:
            assert wide[entry["name"]] == entry

    def test_neither_mode_walks_into_subdirectories(self, roots):
        """`glob` 不递归：子目录那份两档都不出现（放宽名字 ≠ 放宽深度）"""
        http, *_ = roots
        for url in ("/api/data/list", WIDE):
            assert "nested_only.json" not in names(http.get(url).json()["files"]), url


class TestTheWideModeDoesNotWidenReach:
    """宽的是名字闸，不是路径范围——这一族是本轮唯一的安全面判据"""

    def test_files_outside_the_whitelist_stay_unlisted(self, roots):
        http, _, outside = roots
        assert names(http.get(WIDE).json()["files"]) == TOP_LEVEL
        assert (outside / "secret.json").exists()

    def test_duplicated_roots_are_still_deduplicated(self, roots, tmp_path, monkeypatch):
        """`ds` 与 `ds/.` 同时进白名单：宽档仍四份，不因为走了两遍根就翻倍"""
        root = tmp_path / "ds"
        monkeypatch.setenv(
            "AUGMENTOR_DATA_ROOTS", os.pathsep.join([str(root), str(root / ".")]))
        from fastapi.testclient import TestClient
        from api.main import app

        files = TestClient(app).get(WIDE).json()["files"]
        assert names(files) == TOP_LEVEL

    def test_missing_root_lists_nothing_in_either_mode(self, tmp_path, monkeypatch):
        monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", str(tmp_path / "not_created_yet"))
        from fastapi.testclient import TestClient
        from api.main import app

        http = TestClient(app)
        for url in ("/api/data/list", WIDE):
            response = http.get(url)
            assert response.status_code == 200, url
            assert response.json()["files"] == [], url


class TestTheKnobIsNotCosmetic:
    """两档必须真的分得开，否则这一档只是装饰"""

    def test_the_wide_mode_returns_strictly_more(self, roots):
        http, *_ = roots
        default = set(names(http.get("/api/data/list").json()["files"]))
        wide = set(names(http.get(WIDE).json()["files"]))
        assert default < wide, "宽档与默认档相同 ⇒ all_files 没有接到过滤条件上"
        assert wide - default == {"configish.json", "p.json"}

    def test_an_illegal_value_gets_a_positional_verdict(self, roots):
        """布尔档只有两种真值：`maybe` 由 FastAPI 判 422，而不是悄悄当 true 或当 false"""
        http, *_ = roots
        assert http.get("/api/data/list?all_files=maybe").status_code == 422


class TestTheContractDeclaresTheKnob:
    """OpenAPI 那一格：可选、布尔、默认 false，且这一只端点的 query 面恰好只有它"""

    def test_the_parameter_shape(self):
        from api.main import app

        op = app.openapi()["paths"]["/api/data/list"]["get"]
        params = op.get("parameters") or []
        assert len(params) == 1, params
        param = params[0]
        assert (param["name"], param["in"]) == ("all_files", "query")
        assert param["required"] is False
        schema = param["schema"]
        assert schema["type"] == "boolean"
        assert schema["default"] is False

    def test_the_openapi_still_advertises_one_response_key(self):
        """响应面一字未动：`files` 仍是唯一顶层键"""
        from api.main import app

        op = app.openapi()["paths"]["/api/data/list"]["get"]
        ref = op["responses"]["200"]["content"]["application/json"]["schema"]["$ref"]
        model = app.openapi()["components"]["schemas"][ref.rsplit("/", 1)[-1]]
        assert set(model["properties"]) == {"files"}


class TestWhatTheListHandsBackIsReachable:
    """「列出的都读得到」管的是**路径归属**：宽档列出的每一份喂回 `/load` 都不拿 403/404

    这一句是 `test_api_security.py::TestDataListScansWhitelist` 立的那条承诺在宽档下的续量。
    内容形态那一格**按现量钉**而不是按应有的形状钉：非数组那份拿到的是 500 + 固定文案，而不是
    `assert_dataset_shape` 里那句 400 原话——`load_data` 的收尾分支（`api/routes/data.py:177`）
    用 `except Exception` 兜住了 `raise_internal_error`，而 `HTTPException` 也在它的吞掉范围内。
    那一格属于 **A280** 那一族（一刀切收敛），A280 待拍（状态码是对外契约），所以本轮只把它的
    现量钉下来；谁去收 A280，这一支要同批改。
    """

    def test_no_listed_name_is_forbidden_or_missing(self, roots):
        http, *_ = roots
        for name in names(http.get(WIDE).json()["files"]):
            response = http.get(f"/api/data/load/{name}")
            assert response.status_code not in (403, 404), (name, response.status_code)

    def test_the_dataset_shaped_one_loads_in_full(self, roots):
        """`p.json` 是这一档存在理由里的主用例：只有宽档才看得见它，而它读得出来"""
        http, *_ = roots
        ok = http.get("/api/data/load/p.json")
        assert ok.status_code == 200, ok.json()
        assert ok.json()["items"] == DATASET

    def test_the_wrong_shape_currently_collapses_to_a_flat_500(self, roots):
        """A280 的现量：`configish.json` 拿到 500 + 固定文案，形态那句 400 原话不出现在响应里

        两支分别钉「状态码是 500」与「文案不泄漏」（后者是 A155① 那一格），因为把它们并成
        「非 200」一条既抓不到状态码漂移，也抓不到有人把 `str(exc)` 原样回传。
        """
        from api.deps import INTERNAL_ERROR_DETAIL

        http, *_ = roots
        bad = http.get("/api/data/load/configish.json")
        assert bad.status_code == 500, bad.json()
        assert bad.json()["detail"] == INTERNAL_ERROR_DETAIL, bad.json()
        assert "顶层必须是 JSON 数组" not in bad.json()["detail"]


@needs_web_src
class TestTheFrontendAsksForTheWideModeOnlyWhereItSaysSo:
    """接线面：系统页六处候选共用一只宽档 fetcher，其余 8 处选择器继续吃默认档"""

    def test_api_ts_has_exactly_one_wide_call_and_one_bare_call(self):
        text = strip_comments(API_TS.read_text(encoding="utf-8"))
        assert text.count("'/data/list?all_files=true'") == 1
        assert text.count("api.get('/data/list')") == 1

    def test_the_system_page_is_the_consumer_of_the_wide_fetcher(self):
        page = strip_comments(SYSTEM_TSX.read_text(encoding="utf-8"))
        assert "getBackupSourceCandidates()" in page
        assert "getDataFiles(" not in page, "系统页还留着默认档，就量不出宽档接上了"
        # 六处候选共用 `dataFiles` 这一份 state ⇒ 一次宽档取回，六张卡一起变宽
        assert page.count("options={dataFiles.map") == 6

    def test_the_default_fetcher_still_has_eight_page_consumers(self):
        """其余页面一字未动：默认档的选择器现量 8 处调用"""
        callers = []
        for path in sorted(SRC.rglob("*.tsx")):
            if ".test." in path.name:
                continue
            count = strip_comments(path.read_text(encoding="utf-8")).count("getDataFiles(")
            if count:
                callers.append((path.name, count))
        assert sum(c for _, c in callers) == 8, callers
