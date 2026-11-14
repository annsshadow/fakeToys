# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L232：数据集血缘那一族三条端点接进系统状态页之后，形状、判决、口径与那道丢字段的坑都不许漂

**改前实测**（`git show HEAD:./web/src/services/api.ts` / 同 `pages/System.tsx`，L232 取证）：

| 事实 | 改前 | 改后 |
| --- | --- | --- |
| 前端调用 `/api/system/dependency` 一族 | **0 处**（3 条全在 l224 的 `UNCONSUMED` 里） | `listDependencyDatasets` / `registerDependencyDataset` / `getDependencyGraph` + 一张「数据集血缘」卡 |
| `POST` 带 `description` / `tags` | **200 且响应回显了这两格**，随后 `GET` 与磁盘 `datasets.json` 里都是空 | 三处一致（响应、GET、磁盘都带着这两格） |

**仓外探针量到的行为矩阵**（白名单根为临时目录，探针两支跑完即删）：

* 注册表不存在时 `GET …/datasets` ⇒ **200** `{"datasets": []}`，不是 404 也不是 null；
  `GET …/graph` ⇒ **200** `{"nodes": [], "edges": [], "issues": []}`。
* 这两支**只读端点原本会把注册表目录建出来**（构造函数 `mkdir(parents=True)`）：探针量到
  请求之前目录不存在、请求之后存在（空目录，此时还没有 `datasets.json`）⇒ 读面写盘
  （**A307**，L249 已把构造的副作用摘掉，现在请求前后目录都不在场）。
* `POST` 最小体 `{name, input_file}` ⇒ 200，回执九键（`dataset_id` / `name` / `description` /
  `path` / `item_count` / `created_at` / `updated_at` / `tags` / `metadata`），与随后 `GET` 的
  元素**逐键相同**（两侧同为 `DatasetInfo.to_dict()`）。`dataset_id` 是 **`md5(name)[:8]`**
  （实测 `dsA` → `8c7f0201`、`dsB` → `0d2cace8`），`path` 是服务端 resolve 后的**绝对路径**，
  `item_count` 是登记时真读文件数出来的（3 条语料 ⇒ 3）。
* 缺 `input_file` ⇒ **422**（L325 / A311 收口后 `detail` 是一句中文，不再是 FastAPI 的
  英文结构）；而其余坏参数一律走中文 400/403/404 ⇒ 错误体**归一**成同一种形状（**A311**）。
* 同名同文件再 `POST` 一次 ⇒ 200，索引与磁盘各 **1** 条（dict 按 id 覆盖，与备份那一族留两条
  索引的形状**不同**），但 `created_at` 被**刷新**成第二次的时间，且没有 409、没有任何
  「已存在被覆盖」的提示；第二次送空 `description` / `tags` 会把旧值**抹成空**（**A306**）。
* `input_file` 四档坏值：不存在 ⇒ **404**「文件不存在」；白名单外绝对路径 ⇒ **403**「路径超出
  允许的数据目录范围」；带 `..` ⇒ **400**「路径包含非法组件」；空串 ⇒ **400**「文件路径 不能为
  空」（label 与「不能为空」之间那个空格是 f-string 原样）。加测一档：顶层不是 JSON 数组 ⇒
  **400**「数据文件的顶层必须是 JSON 数组，当前是对象」。
* `registry_path` 四档（三支判据逐字相同）：省略 ⇒ **200**（走白名单首根下的
  `.dependency_registry`）；显式空串 ⇒ **400**「目录路径 不能为空」（**不**静默回落默认值）；
  `../outside` ⇒ **400**「路径包含非法组件」；白名单外绝对路径 ⇒ **403**；白名单内绝对路径 ⇒
  **200**，且那一份注册表与默认那份**互不可见**（`reg2/datasets.json` 独立）。
* `graph` 的 `nodes` 是 `{id, name, item_count}`、`edges` 是 `{source, target, type}`。悬空边
  进 `issues`：`{"type": "missing_target", "dependency": {…五键…}, "message": "目标数据集不存在:
  ghost12345"}`（对称另有 `missing_source`）。
* **没有环检测**：探针用 SDK 直写 A→B（merged）与 B→A（filtered）成环后 HTTP 读图，`issues`
  仍只有悬空那 1 条，环**零报告** ⇒ 「issues 为空」不能读成「依赖关系没问题」（**A310**）。
* HTTP 面上 dependency 路由**只有三条**：无 DELETE、无注销、无建边入口；SDK
  `DependencyManager` 的 `add_dependency` / `remove_dependency` / `get_dependencies` /
  `validate_dependencies` 四个方法**一个都没有** HTTP 入口，且 SDK 连
  `remove_dataset`/`unregister_dataset` 都没有 ⇒ 登记过的数据集在产品面上删不掉，边只能由
  SDK 建立，纯前端用户的 `edges` 恒为空（**A309**）。
* 「默认注册表目录」在仓内有**三份**：CLI 的 `--registry-path` 默认 `.dependency_registry`
  （相对工作目录）、SDK `DependencyManager.__init__` 形参默认也是 `.dependency_registry`、
  端点走 `default_registry_dir()`＝白名单首个根目录下的 `.dependency_registry`（出厂
  `data/.dependency_registry`）⇒ CLI 登记的数据集页面看不见，反之亦然（**A308**）。
* 三条里只有 `POST` 挂 `Depends(verify_api_key)`，两支 `GET` 都没挂（`route.dependant.dependencies`
  静态读 + 行为双证：设上 `AUGMENTOR_API_KEY` 后无头 `POST` ⇒ **401**「缺少或无效的
  X-API-Key」，带对头 ⇒ 200，两支 `GET` 无头照样 200）；而前端**没有**送这个头的通道，与备份
  那一族同一个缺口（**A312**，同 A302）。
* 写盘走 `atomic_write_json`（同目录 `.name.tmp-<hex>` + `os.replace`，`finally` 清临时件），
  探针跑完整棵树 `.*tmp-*` 残留 **0** ⇒ 是原子的。

跑法：`python -m pytest tests/unit/test_lineage_wiring_l232.py -q --no-cov`
"""

import hashlib
import json
import pathlib
import re

import pytest
from tests.unit.test_transform_wiring_l229 import (
    fetcher_body,
    ts_interface_keys,
)
from tests.unit.test_wired_dropdown_choices_l226 import (
    needs_web_src,
    strip_comments,
)

from api.deps import default_registry_dir, verify_api_key
from api.routes.system_ops import (
    DatasetInfoResponse,
    RegisterDatasetRequest,
)
from api.routes.system_ops import DependencyDatasetListResponse as PyListResponse
from api.routes.system_ops import DependencyGraphResponse as PyGraphResponse

ROOT = pathlib.Path(__file__).resolve().parents[2]
SRC = ROOT / "web" / "src"
PAGE_TSX = SRC / "pages" / "System.tsx"
API_TS = SRC / "services" / "api.ts"
PARSER_PY = ROOT / "augmentor" / "cli" / "parser.py"
DEPENDENCY_PY = ROOT / "augmentor" / "dependency.py"

#: 探针用的 3 条语料：回执里 `item_count 3` 这个读数来自这份内容
ITEMS = [{"instruction": f"q{i}", "input": "", "output": f"a{i}"} for i in range(3)]

#: 这一族的共同前缀。判据**必须**用它而不是 `dependenc`：仓里还有一条
#: `GET /api/system/dependencies`，那是「可选依赖包诊断」，属于另一族（L226 明确拒接，理由是
#: 与 `/api/status` 的 `dependencies` 同一份数据），宽匹配会把它吞进来（L232 首跑实红 2 处）。
FAMILY = "/api/system/dependency/"

#: 三条操作 ⇒ 服务层里那条 fetcher 的名字
FETCHERS = {
    ("GET", "/api/system/dependency/datasets"): "listDependencyDatasets",
    ("POST", "/api/system/dependency/datasets"): "registerDependencyDataset",
    ("GET", "/api/system/dependency/graph"): "getDependencyGraph",
}

#: **L338/A309** 补上的两条写入口。前端服务层尚无对应 fetcher，故不并入 `FETCHERS`
#: （那本名册同时被「三条 fetcher 逐条对路径」与「页面 import/调用」两支尺子读）。
WRITE_ENTRIES = {
    ("DELETE", "/api/system/dependency/datasets/{dataset_id}"),
    ("POST", "/api/system/dependency/edges"),
}


def _page() -> str:
    assert PAGE_TSX.is_file(), "前端源码不在场 ⇒ 本文件的判据全部失效"
    # 规范化空白：Prettier 可能对长 JSX 文本自动折行，把「md5(名称) 的前 8 位」
    # 这类连续短语拆成多行。判据关心文案内容而不是排版，压平空白后再断言。
    return re.sub(r"\s+", " ", strip_comments(PAGE_TSX.read_text(encoding="utf-8")))


def _api_ts() -> str:
    return strip_comments(API_TS.read_text(encoding="utf-8"))


@pytest.fixture
def http_with_registry(tmp_path, monkeypatch):
    """白名单数据根：`ds_a.json`(3 条) + 一份顶层是对象的坏语料；根**外**另有 `outside/`"""
    root = tmp_path / "ds"
    root.mkdir()
    outside = tmp_path / "outside"
    outside.mkdir()
    (root / "ds_a.json").write_text(json.dumps(ITEMS, ensure_ascii=False), encoding="utf-8")
    (root / "notarray.json").write_text(json.dumps({"a": 1}), encoding="utf-8")
    (outside / "secret.json").write_text("[]", encoding="utf-8")
    monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", str(root))
    monkeypatch.setenv("AUGMENTOR_API_KEY", "")
    monkeypatch.chdir(tmp_path)
    from fastapi.testclient import TestClient
    from api.main import app

    return TestClient(app), root, outside, tmp_path


def _disk_registry(root):
    """注册表落盘那份 `datasets.json`（顶层 dict：id → 条目）"""
    path = root / ".dependency_registry" / "datasets.json"
    assert path.is_file(), f"注册表没落盘：{path}"
    return json.loads(path.read_text(encoding="utf-8"))


class TestTheThreeFetchersMatchTheContract:
    """服务层那三条：动词、路径、body 与 query 的分工逐个对上契约"""

    def test_the_three_fetchers_hit_the_three_paths(self):
        for (verb, path), name in FETCHERS.items():
            body = fetcher_body(name)
            # 服务层的路径不带 `/api` 前缀（那一份住在 `baseURL` 里，l224 从源码读它）
            literal = path.removeprefix("/api")
            assert f"api.{verb.lower()}(" in body, f"{name} 不是 {verb} 调用"
            assert f"'{literal}'" in body, f"{name} 里没有 {path}（找的是 {literal}）"

    def test_nothing_else_in_the_service_layer_touches_the_dependency_family(self):
        hits = re.findall(r"api\.(?:get|post|put|delete|patch)\([^)]*?/system/dependency",
                          _api_ts().replace("\n", " "))
        assert len(hits) == 5, f"血缘一族的调用现量 5 处（三条读/登记 + L338 两条写入口），实得 {len(hits)}：{hits}"

    def test_registry_path_is_a_query_param_and_never_a_body_key(self):
        for name in FETCHERS.values():
            body = fetcher_body(name)
            assert "params: { registry_path:" in body, f"{name} 没把 registry_path 当 query 发"
        body = fetcher_body("registerDependencyDataset")
        json_body = re.search(r"const body = (\{.*?\n  \})", body, re.S)
        assert json_body, "registerDependencyDataset 的请求体形状变了 ⇒ 判据要跟着换"
        assert "registry_path" not in json_body.group(1), "它把 registry_path 塞进了请求体"

    def test_the_request_body_carries_exactly_the_contract_keys(self):
        body = fetcher_body("registerDependencyDataset")
        assert set(RegisterDatasetRequest.model_fields) == {
            "name", "input_file", "description", "tags"}
        for key in ("name,", "input_file: inputFile", "description:", "tags:"):
            assert key in body, f"请求体里少了 {key}"

    def test_the_service_layer_writes_no_second_default_for_the_registry_dir(self):
        """`.dependency_registry` 这个字面量只许住在后端（`api/deps.py`、SDK 形参与 CLI 默认），
        服务层**代码**里一份都不写

        页面正文里出现它是**诚实口径**（告诉用户缺省落点是哪、CLI 那份默认不是同一个目录，
        A308），不是第二份默认值 —— 判据只看代码面：`strip_comments` 之后的服务层。
        """
        assert ".dependency_registry" not in _api_ts(), "服务层代码里出现了第二份注册表默认值"
        page = _page()
        assert not re.search(r"registry_?[Pp]ath\w*\s*[:=]\s*['\"]", page), \
            "页面上给 registry_path 写死了值 ⇒ 那就是第二份默认"


class TestTsFaceMatchesThePydanticModels:
    """TS 侧三个接口与线上响应模型逐键相等"""

    def test_dataset_entry_keys(self):
        assert set(ts_interface_keys("DatasetRegistryEntry")) == set(
            DatasetInfoResponse.model_fields)
        assert len(ts_interface_keys("DatasetRegistryEntry")) == 9

    def test_dataset_list_keys(self):
        assert set(ts_interface_keys("DatasetRegistryListResponse")) == set(
            PyListResponse.model_fields)

    def test_graph_keys(self):
        assert set(ts_interface_keys("DependencyGraphResponse")) == set(
            PyGraphResponse.model_fields)
        assert ts_interface_keys("DependencyGraphResponse") == ["nodes", "edges", "issues"]

    def test_the_graph_node_and_edge_shapes_are_pinned(self):
        """图的两格在 pydantic 侧是 `List[Any]`（契约不约束元素），所以形状由这里钉住"""
        assert ts_interface_keys("DependencyGraphNode") == ["id", "name", "item_count"]
        assert ts_interface_keys("DependencyGraphEdge") == ["source", "target", "type"]
        assert ts_interface_keys("DependencyGraphIssue") == ["type", "dependency", "message"]


@needs_web_src
class TestThePageCarriesTheThreeOperations:
    """渲染面上真的看得见：页面导入、调用、卡片标题与那几句实测口径"""

    def test_the_page_imports_and_calls_the_three_fetchers(self):
        page = _page()
        for name in FETCHERS.values():
            assert name in page.split("export default")[0], f"System.tsx 没导入 {name}"
            assert f"{name}(" in page, f"System.tsx 里没有调用 {name}"

    def test_the_card_is_named_after_lineage_not_after_packages(self):
        """本页已有「依赖明细」那张卡讲的是**可选依赖包**，这一族讲的是数据集之间谁派生谁

        两张卡都叫「依赖」用户就分不出，所以标题刻意取「数据集血缘」。这条判据钉的是
        **两个标题同时在**且不同名 —— 只钉新标题的话，把旧卡改名成同一个词也能过。
        """
        page = _page()
        assert 'title="数据集血缘"' in page
        assert 'title="依赖明细"' in page

    def test_candidates_come_from_the_data_list_endpoint(self):
        """L251 换了名：系统页六处候选共用一只宽档 fetcher（A304 收口），这一页的血缘登记也在其中"""
        assert "getBackupSourceCandidates(" in _page()

    def test_the_measured_caveats_stay_on_the_card(self):
        page = _page()
        for phrase in (
            "L338/A309",                 # A309：两条写入口已补
            "POST .../edges",            # A309：建边入口
            "DELETE",                    # A309：注销入口
            "remove_dataset",            # A309：SDK 注销已接进 HTTP
            "依赖成环",                   # A310 / L311：成环现在也报出来了
            "cycle",                     # A310 / L311：那一档的 type 名
            "0 才真的读得成「没问题」",    # A310 / L311：0 才读得成校验通过
            "md5(名称) 的前 8 位",        # A306：id 只由名字算
            "同名再登记回 409",          # A306 / L332：改判 409，原有 created_at 保住
            "回显即丢",                  # A305：本轮收掉的那条
            "与登记这一族端点同一份",    # A308 / L328：CLI 默认已与端点那一份同源
            "构造不再碰磁盘",              # A307：只读端点建目录，L249 收口
            "错误体统一是一句中文",       # A311 / L325：422 与其余 4xx 同一形状了
            "白名单外是 403",             # 白名单档位
            "数据文件的顶层必须是 JSON 数组",  # 这一族独有的那档判决原文
        ):
            assert phrase in page, f"页面上少了这句口径：{phrase}"

    def test_the_page_offers_no_registry_path_input(self):
        """页面刻意不送 `registry_path`：那一格一送就要在服务层写第二份默认（见 A308）"""
        assert "registryPath" not in _page()

    def test_the_tags_input_splits_on_both_commas(self):
        """标签那一格是自由输入，中英文逗号都要当分隔符，空段不许送上去"""
        page = _page()
        # Prettier 会把 `dsTags` 与 `.split(...)` 拆成两行，这里允许任意空白。
        found = re.search(r"tags: dsTags\s*\.split\((.*?)\)", page)
        assert found, "标签那一格不再走 split ⇒ 判据要跟着换形状"
        assert "," in found.group(1) and "，" in found.group(1), found.group(1)
        assert "filter(Boolean)" in page


class TestMeasuredEndpointBehavior:
    """端点上的判决：逐格是探针量到的读数，不是从源码推的想象"""

    def test_missing_registry_lists_an_empty_array(self, http_with_registry):
        http, root, *_ = http_with_registry
        assert not (root / ".dependency_registry").exists()
        r = http.get("/api/system/dependency/datasets")
        assert r.status_code == 200 and r.json() == {"datasets": []}

    def test_the_two_read_endpoints_leave_no_registry_dir(self, http_with_registry):
        """A307 收口（L249）：读面不再写盘 —— 两支 GET 之后注册表目录仍不在场"""
        http, root, *_ = http_with_registry
        registry = root / ".dependency_registry"
        assert not registry.exists()
        assert http.get("/api/system/dependency/datasets").status_code == 200
        assert not registry.exists(), "GET 列表把注册表目录建出来了 ⇒ 读面又写盘了"
        assert http.get("/api/system/dependency/graph").status_code == 200
        assert not registry.exists(), "GET 依赖图把注册表目录建出来了 ⇒ 读面又写盘了"

    def test_empty_graph_has_the_three_keys(self, http_with_registry):
        http, *_ = http_with_registry
        r = http.get("/api/system/dependency/graph")
        assert r.status_code == 200
        assert r.json() == {"nodes": [], "edges": [], "issues": []}

    def test_register_receipt_has_the_nine_keys(self, http_with_registry):
        http, root, *_ = http_with_registry
        r = http.post("/api/system/dependency/datasets",
                      json={"name": "dsA", "input_file": "ds_a.json"})
        assert r.status_code == 200, r.text
        payload = r.json()
        assert set(payload) == set(DatasetInfoResponse.model_fields)
        assert payload["dataset_id"] == hashlib.md5(b"dsA").hexdigest()[:8] == "8c7f0201"
        assert payload["item_count"] == 3
        assert pathlib.Path(payload["path"]).is_absolute()
        assert pathlib.Path(payload["path"]) == (root / "ds_a.json").resolve()

    def test_description_and_tags_reach_the_disk_not_just_the_receipt(self, http_with_registry):
        """A305（本轮收掉的那条）：三处必须一致 —— 响应、随后的 GET、磁盘 `datasets.json`

        改前实测：响应回显 `description="描述B"` / `tags=["t1","t2"]`，而 GET 与磁盘上都是
        空。根因是端点在 `register_dataset()`（内部已落盘）**之后**才改返回对象，改的是内存。
        这条判据只看磁盘与 GET，不看响应 —— 只看响应的尺子在改前也是绿的。
        """
        http, root, *_ = http_with_registry
        body = {"name": "dsB", "input_file": "ds_a.json",
                "description": "描述B", "tags": ["t1", "t2"]}
        assert http.post("/api/system/dependency/datasets", json=body).status_code == 200
        listed = http.get("/api/system/dependency/datasets").json()["datasets"]
        assert len(listed) == 1
        assert listed[0]["description"] == "描述B", "GET 里描述仍是空 ⇒ 改动没生效"
        assert listed[0]["tags"] == ["t1", "t2"], "GET 里标签仍是空 ⇒ 改动没生效"
        disk = _disk_registry(root)
        assert len(disk) == 1
        entry = next(iter(disk.values()))
        assert entry["description"] == "描述B", "磁盘上描述仍是空 ⇒ 只改了回执"
        assert entry["tags"] == ["t1", "t2"], "磁盘上标签仍是空 ⇒ 只改了回执"

    def test_missing_input_file_is_a_chinese_422_now(self, http_with_registry):
        """A311（L325 收口）：缺必填格也回**一句中文** ⇒ 错误体不再双轨

        改前那一格钉的是「英文 422 与中文 4xx 并存」（`detail` 是 FastAPI 那份数组，
        `msg` 逐字 `Field required`）。改后 `detail` 是**字符串** ⇒ 与其余 4xx 同一形状。
        """
        http, *_ = http_with_registry
        r = http.post("/api/system/dependency/datasets", json={"name": "x"})
        assert r.status_code == 422
        detail = r.json()["detail"]
        assert isinstance(detail, str), detail
        assert "参数校验失败" in detail, detail
        assert "input_file" in detail, detail
        assert "缺少必填字段" in detail, detail

    def test_same_name_twice_returns_409_and_keeps_first(self, http_with_registry):
        """A306 / L332：同名再登记回 409，原有 created_at / 描述 / 标签一字不动。"""
        http, root, *_ = http_with_registry
        first = http.post("/api/system/dependency/datasets", json={
            "name": "dsA", "input_file": "ds_a.json",
            "description": "第一份", "tags": ["keep"]}).json()
        again = http.post("/api/system/dependency/datasets", json={
            "name": "dsA", "input_file": "ds_a.json"})
        assert again.status_code == 409, again.text
        assert "已登记" in again.json()["detail"]
        listed = http.get("/api/system/dependency/datasets").json()["datasets"]
        assert len(listed) == 1, "冲突不得新增登记（与备份那一族的形状不同）"
        assert len(_disk_registry(root)) == 1
        kept = listed[0]
        assert kept["created_at"] == first["created_at"], "created_at 不得被刷新"
        assert kept["description"] == "第一份" and kept["tags"] == ["keep"], \
            "第二次不送这两格时旧值必须保留"

    @pytest.mark.parametrize("bad, status, detail", [
        ("nope.json", 404, "文件不存在"),
        ("../outside/secret.json", 400, "路径包含非法组件"),
        ("", 400, "文件路径 不能为空"),
        ("notarray.json", 400, "数据文件的顶层必须是 JSON 数组，当前是对象"),
    ])
    def test_bad_input_file_verdicts(self, http_with_registry, bad, status, detail):
        http, root, outside, *_ = http_with_registry
        r = http.post("/api/system/dependency/datasets",
                      json={"name": "dsX", "input_file": bad})
        assert r.status_code == status, r.text
        assert r.json()["detail"] == detail
        assert sorted(p.name for p in outside.iterdir()) == ["secret.json"], \
            "白名单外多出了文件"
        # 四档都在 `resolve_data_path` / 读文件那一步就被挡下，注册表压根没被构造
        assert not (root / ".dependency_registry" / "datasets.json").exists(), \
            "坏入参那一档竟然落了盘"

    def test_outside_whitelist_is_403_and_writes_nothing(self, http_with_registry):
        http, root, outside, tmp_path = http_with_registry
        r = http.post("/api/system/dependency/datasets", json={
            "name": "dsX", "input_file": str(outside / "secret.json")})
        assert r.status_code == 403
        assert r.json()["detail"] == "路径超出允许的数据目录范围"
        assert sorted(p.name for p in outside.iterdir()) == ["secret.json"]
        assert sorted(p.name for p in tmp_path.iterdir()) == ["ds", "outside"]

    @pytest.mark.parametrize("value, status, detail", [
        ("", 400, "目录路径 不能为空"),
        ("../outside", 400, "路径包含非法组件"),
    ])
    def test_registry_path_bad_verdicts_on_all_three(self, http_with_registry,
                                                     value, status, detail):
        """两支 GET 与那支 POST 对 `registry_path` 的判据逐字相同"""
        http, *_ = http_with_registry
        for call in (lambda: http.get("/api/system/dependency/datasets",
                                      params={"registry_path": value}),
                     lambda: http.get("/api/system/dependency/graph",
                                      params={"registry_path": value}),
                     lambda: http.post("/api/system/dependency/datasets",
                                       json={"name": "dsA", "input_file": "ds_a.json"},
                                       params={"registry_path": value})):
            r = call()
            assert r.status_code == status, r.text
            assert r.json()["detail"] == detail

    def test_registry_path_outside_is_403_and_inside_is_200(self, http_with_registry):
        http, root, outside, *_ = http_with_registry
        for path in ("/api/system/dependency/datasets", "/api/system/dependency/graph"):
            r = http.get(path, params={"registry_path": str(outside)})
            assert r.status_code == 403 and r.json()["detail"] == "路径超出允许的数据目录范围"
        reg2 = root / "reg2"
        r = http.get("/api/system/dependency/datasets", params={"registry_path": str(reg2)})
        assert r.status_code == 200 and r.json() == {"datasets": []}
        assert not reg2.exists(), "白名单内那一档也不该被一次 GET 建出来（A307，L249 收口）"

    def test_an_explicit_registry_is_invisible_to_the_default_one(self, http_with_registry):
        """A308 的行为面：两份注册表互不可见"""
        http, root, *_ = http_with_registry
        reg2 = root / "reg2"
        r = http.post("/api/system/dependency/datasets",
                      json={"name": "dsA2", "input_file": "ds_a.json"},
                      params={"registry_path": str(reg2)})
        assert r.status_code == 200
        assert (reg2 / "datasets.json").is_file()
        default_listed = http.get("/api/system/dependency/datasets").json()["datasets"]
        assert default_listed == [], "默认注册表看见了显式那份的登记 ⇒ A308 要重写"

    def test_graph_reports_a_dangling_edge_but_not_a_cycle(self, http_with_registry):
        """A310：`validate_dependencies` 只查悬空端点，成环零报告"""
        http, root, *_ = http_with_registry
        ids = {}
        for name in ("dsA", "dsB"):
            r = http.post("/api/system/dependency/datasets",
                          json={"name": name, "input_file": "ds_a.json"})
            ids[name] = r.json()["dataset_id"]
        from augmentor.dependency import DependencyManager

        manager = DependencyManager(str(root / ".dependency_registry"))
        manager.add_dependency(ids["dsA"], "ghost12345", "derived")
        manager.add_dependency(ids["dsA"], ids["dsB"], "merged")
        manager.add_dependency(ids["dsB"], ids["dsA"], "filtered")

        payload = http.get("/api/system/dependency/graph").json()
        assert len(payload["nodes"]) == 2 and len(payload["edges"]) == 3
        # A310 / L311：成环现在**报出来**了（改前 issues 里一个字都没有 ⇒
        # 「为空」被读成「没问题」）。悬空端点那一档一字未动，环那档多一格。
        assert [i["type"] for i in payload["issues"]] == ["missing_target", "cycle"]
        assert payload["issues"][0]["message"] == "目标数据集不存在: ghost12345"
        assert set(payload["issues"][0]["dependency"]) == {
            "source_dataset", "target_dataset", "dependency_type", "description", "created_at"}
        # A→B 与 B→A 成环 ⇒ issues 里有一档 `cycle`，且判决点名环上的数据集
        cycles = [i for i in payload["issues"] if i["type"] == "cycle"]
        assert len(cycles) == 1, payload["issues"]
        # 环上那两只是**登记 id**（md5 前 8 位），不是登记名 —— 判决点名的就是它
        assert ids["dsA"] in cycles[0]["message"], cycles[0]
        assert ids["dsB"] in cycles[0]["message"], cycles[0]

    def test_only_the_post_is_key_gated(self, http_with_registry, monkeypatch):
        """A312：静态读 `dependant.dependencies` + 行为双证（设上 key 后无头 POST 是 401）"""
        from api.routes.system_ops import router

        gated, seen = set(), set()
        for route in router.routes:
            path = getattr(route, "path", "")
            if not path.startswith("/api/system/dependency"):
                continue
            verb = sorted(route.methods)[0]
            seen.add((verb, path))
            if any(dep.call is verify_api_key for dep in route.dependant.dependencies):
                gated.add((verb, path))
        assert seen == {(v, p) for v, p in FETCHERS} | WRITE_ENTRIES, sorted(seen)
        assert gated == {("POST", "/api/system/dependency/datasets")} | WRITE_ENTRIES, sorted(gated)

        http, *_ = http_with_registry
        monkeypatch.setenv("AUGMENTOR_API_KEY", "secret-key")
        body = {"name": "authT", "input_file": "ds_a.json"}
        r = http.post("/api/system/dependency/datasets", json=body)
        assert r.status_code == 401 and r.json()["detail"] == "缺少或无效的 X-API-Key"
        assert http.post("/api/system/dependency/datasets", json=body,
                         headers={"X-API-Key": "secret-key"}).status_code == 200
        # 两支 GET 不挂门禁：设了 key 也无头照样 200
        assert http.get("/api/system/dependency/datasets").status_code == 200
        assert http.get("/api/system/dependency/graph").status_code == 200


class TestTheFamilyHasNoProductLevelWriteEntries:
    """A309：三条端点里没有删除、没有注销、没有建边；SDK 那几个方法一个都没上线"""

    def test_the_contract_carries_five_dependency_operations(self):  # L338/A309
        from api.main import app

        ops = {(m.upper(), p) for p, ms in app.openapi()["paths"].items()
               if p.startswith(FAMILY) for m in ms}
        assert ops == {(v, p) for v, p in FETCHERS} | WRITE_ENTRIES, sorted(ops)
        assert [v for v, _ in ops if v == "DELETE"], "注销入口又不见了"

    def test_the_manager_methods_that_never_reached_http(self):
        from augmentor.dependency import DependencyManager

        public = {n for n in vars(DependencyManager) if not n.startswith("_")}
        assert {"add_dependency", "remove_dependency", "get_dependencies",
                "validate_dependencies"} <= public, sorted(public)
        # L338/A309：SDK 补上了 remove_dataset，并由 DELETE 端点接进产品面
        assert "remove_dataset" in public, "SDK 的注销入口又不见了"

        from api.main import app

        # 用 operationId 查而不是 `name not in paths`：FastAPI 的默认 operationId 同时含函数名
        # 与路径，所以「新增一条以这三个方法命名的路由」在这里会红；而在路径串里查
        # `add_dependency` 是恒真的（路径从来不长这样）—— 那是一把不会响的尺子。
        ids = " ".join(op.get("operationId", "")
                       for ops in app.openapi()["paths"].values() for op in ops.values())
        for name in ("add_dependency", "remove_dependency", "get_dependencies"):
            assert name not in ids, f"{name} 竟然有了 HTTP 入口 ⇒ 这条缺口要重新量"

    def test_the_router_carries_exactly_five_dependency_routes(self):  # L338/A309
        from api.routes.system_ops import router

        paths = {getattr(r, "path", "") for r in router.routes}
        assert {p for p in paths if p.startswith(FAMILY)} == (
            {p for _, p in FETCHERS} | {p for _, p in WRITE_ENTRIES})


def _shipped_first_data_root() -> str:
    """**出厂** `config.yaml` 里 `web.data_roots` 的第一项（出厂即 `data`）

    读的是**出厂配置**，不是夹具那份临时数据根 —— 夹具把数据根换到 tmp 目录下，
    那是测试的安排、不是出厂形状；而「CLI 默认与端点默认同源」这句话说的是**出厂形状**。
    """
    import yaml

    config = yaml.safe_load((ROOT / "config.yaml").read_text(encoding="utf-8"))
    roots = config["web"]["data_roots"]
    assert isinstance(roots, list) and roots, roots
    return roots[0]


class TestTheCliAndTheEndpointShareOneDefault:
    """A308：**L328 收口**（用户 2026-11-13 拍板①）—— CLI 与端点那一份**同源**了

    **改前**（本支的旧形状）：三份默认各说各话 —— CLI `--registry-path` 默认
    `.dependency_registry`（CWD 相对）、SDK 形参默认 `.dependency_registry`（CWD 相对）、
    端点省略参数时走 `default_registry_dir()`（白名单首个根目录下的 `.dependency_registry`
    ＝ 出厂 `data/.dependency_registry`）⇒ **CLI 登记的数据集在页面上看不见**（反之亦然）。

    **改后**（用户拍板①）：**CLI 的默认改成 `data/.dependency_registry`** ⇒ 与端点那一份
    **同源**（出厂数据根 `data` 下逐字相等）；**SDK 的库级默认不动**（那一层没有白名单概念
    ⇒ 保持 CWD 相对是库的惯例）。

    **留下的残留（如实记，不藏）**：CLI 这一份是**按出厂数据根 `data` 写死**的 ⇒
    部署把 `web.data_roots` 改成别的名字时，CLI 与端点**又漂开** ⇒
    「让 CLI 读配置取数据根」是**下一档**的候选（那要另拍一次）。
    """

    def test_the_cli_default_is_the_data_root_scoped_one(self):
        parser_src = PARSER_PY.read_text(encoding="utf-8")
        found = re.search(r'"--registry-path",\s*type=str,\s*default="([^"]+)"', parser_src)
        assert found, "CLI 的 --registry-path 默认值换了形状 ⇒ 判据要跟着换"
        assert found.group(1) == "data/.dependency_registry", found.group(1)
        assert not pathlib.Path(found.group(1)).is_absolute(), "CLI 那份默认不再是相对路径"

    def test_the_sdk_default_is_untouched(self):
        """决策的另一半：**SDK 的库级默认不动**（那一层没有白名单概念）"""
        sdk_src = DEPENDENCY_PY.read_text(encoding="utf-8")
        assert 'registry_path: str = ".dependency_registry"' in sdk_src, \
            "SDK 形参默认值换了 ⇒ 决策的另一半被改动了"

    def test_the_endpoint_default_follows_the_whitelist(self, http_with_registry):
        _, root, *_ = http_with_registry
        assert default_registry_dir() == root / ".dependency_registry"
        assert default_registry_dir().is_absolute()

    def test_the_cli_default_and_the_endpoint_default_are_one_directory(self, http_with_registry):
        """**同源那一格**：CLI 那份的默认 == **出厂配置**第一个数据根下的那一份

        这里比的是**出厂 `config.yaml`** 的数据根（不是夹具那份临时目录 —— 夹具把
        数据根换成了 tmp 目录，那是测试的安排、不是出厂形状）⇒ 断言把
        「CLI 默认的父目录名 == 出厂数据根名」**钉成机器可查的一格**：
        哪天有人把 `web.data_roots` 的第一项改了名而没同步 CLI 默认，
        **这一支当场红** ⇒ 那条「残留」从此不是靠人记着。
        """
        _, root, *_ = http_with_registry
        parser_src = PARSER_PY.read_text(encoding="utf-8")
        found = re.search(r'"--registry-path",\s*type=str,\s*default="([^"]+)"', parser_src)
        cli_default = pathlib.Path(found.group(1))
        shipped_root = _shipped_first_data_root()
        assert cli_default == pathlib.Path(shipped_root) / ".dependency_registry", (
            "CLI 默认 %r 与出厂数据根 %r 下的那一份对不上 ⇒ 两份又漂开了"
            % (str(cli_default), shipped_root))

    def test_the_sdk_default_is_still_the_library_convention(self, http_with_registry):
        """**仍然分得清**：SDK 那份仍是库惯例（CWD 相对），与端点那份**不是**同一处"""
        _, root, _, tmp_path = http_with_registry
        from augmentor.dependency import DependencyManager

        import inspect

        signature = inspect.signature(DependencyManager.__init__)
        sdk_default = signature.parameters["registry_path"].default
        assert sdk_default == ".dependency_registry"
        assert (tmp_path / sdk_default).resolve() != default_registry_dir().resolve()
        assert root in default_registry_dir().parents
