# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L230：sample / aggregate / rag 三条写盘操作接进数据管理页之后，清单、默认、判决与回执都不许漂

**改前实测**（`git show HEAD:./web/src/services/api.ts` / 同 `pages/DataManagement.tsx`，L230 取证）：

| 事实 | 改前 | 改后 |
| --- | --- | --- |
| 前端调用 `/api/dataset/sample` | **0 处**（在 l224 的 `UNCONSUMED` 里） | `sampleDataset` + 一张「采样」卡 |
| 前端调用 `/api/dataset/aggregate` | **0 处**（同上） | `aggregateDatasets` + 一张「多源聚合」卡 |
| 前端调用 `/api/dataset/rag` | **0 处**（同上） | `convertToRag` + 一张「转成 RAG 检索格式」卡 |
| 页面上有没有第二份方法 / 策略 / 格式清单 | 没有 | 三份新增，各自**逐项等于**后端那一处权威 |

**这一批的判据为什么仍是「清单 + 默认 + 判决 + 回执」四档**：三条都是**写盘**端点，成功面只回
「写到哪、写了多少」。仓外探针（三支，跑完即删）在真实中间件链上量到的形状，逐条是下面这些判据的依据：

* **三份清单各有各的权威，且都不是「后端有一份常量」那种好办形状**：采样方法三项住在
  `DatasetOperations.sample()` 的 if/elif 分发里（`augmentor/dataset_ops.py:316`、`:318`、`:321`），
  聚合策略四项住在 `DataAggregator.aggregate()` 的同形分发里（`augmentor/aggregator.py:262` 起），
  RAG 格式三项才是真常量（`augmentor/rag.py:17`，并被 `augmentor/config.py:120` 的 `RAG_FORMATS`
  引走）。前两处没有常量可引 ⇒ 本文件用 AST 把分发里逐个 `== "字面量"` 抓回来与页面逐项相等，
  而不是在测试里再抄一份名单（抄一份就等于把「加一支方法要动两处」这件事藏起来）。
  **L277 起上面这几个行号整体平移**（`sample()` 里为 `require_ratio` 与 `is not None` 各插了注释行）：
  本文件对这些引用只作散文说明，判据一律走 AST / 字符串，不数行号（l217 那把尺子把同一缺陷类
  量过三次），所以订正到当前读数而不是删掉。
* **`ratio` 的两维现在都判死，三种 method 一种结局（A292 的这一半在 L277 收口）**：
  `size` 那一格一直有 `require_count`（`augmentor/dataset_ops.py:291`），三支一致 400
  「size 必须是不小于 0 的整数，当前是 -1」；`ratio` 那一格（`:300`）此前只算
  `int(len(items) * ratio)`，**非有限那一维**由 L272 补上、**范围那一维**由 L277 补上
  （改用 `require_ratio("ratio", config.ratio)` 的默认 0-1 界）。
  L277 的改前实测（20 条语料）：`ratio=2` ⇒ random **400**（那句是 CPython 的英文原话
  「Sample larger than population or is negative」）、systematic **200 交全量 20 条**、
  stratified **200 交全量 20 条**；`ratio=-0.5` ⇒ random **400**、systematic **200 交前 10 条**
  （`items[::1][:-10]` 的负数切片）、stratified **200 交 0 条**。
  **改后逐格复量**：`2` / `2.0` / `1.5` / `-0.5` / `-1.0` 在五档 × 三种 `method` 上一律 **400**，
  detail 整句等于「`ratio 必须是 0.0 到 1.0 之间的比例，当前是 <值>`」（pydantic 把 `2` 收进
  `Optional[float]` ⇒ 读数写作 `2.0`），且**盘上不留任何产物**。判据由「反向证据」翻成正向判决，
  是因为「同一越界比例三种结局」这个洞的形状从来不是契约。
  `ratio=0.0001` 仍是合法档：三支一致 200 交 0 条（`int()` 归零，被 `:308` 的短路接住，
  不是三种异常）。
* **分层键取字段值的前 10 个字符（A296）**：`augmentor/dataset_ops.py:357` 是 `value[:10]`。
  决定性的一格（探针第一版**量歪过**，见下面「被推翻的想象」）：三条 `AAAAAAAAAA甲` /
  `AAAAAAAAAA乙` / `BBBBBBBBBB丙` 取 2 条、`seed=7` ⇒ 落盘 `['AAAAAAAAAA乙', 'BBBBBBBBBB丙']`。
  若不折叠，三组各一条、配额该是 `[1,1,0]` ⇒ 落盘甲乙、**丙不在内**。幽灵分层键（字段名不存在）
  实测 **200**，全部塌成一个 `"empty"` 组，取 8 条照给 8 条；空字符串键才是 400
  「分层采样需要非空的 stratify_key」（`augmentor/dataset_ops.py:350`）。
* **聚合端点比 SDK 面窄三个旋钮，且不可复现（A293）**：路由是 `DataAggregator()` 无参构造
  （`api/routes/dataset_tools.py:887`），SDK 的 `key_fields` / `value_fields` / `random_state`
  三个都不可设（`augmentor/aggregator.py:58`）。实测后果：同问不同答的两条 union ⇒ 并成 1 条、
  `removed_duplicates 1`、活下来的是**第一条**（「答案甲」）；weighted 同参两次 ⇒ 落盘内容
  **不相等**。回执那一半是**两份各自手写的键集**：`to_dict()`（`augmentor/aggregator.py:37`）交回
  `total_count / source_counts / removed_duplicates / conflict_count / conflicts` 五键，端点在
  `api/routes/dataset_tools.py:891` 另拼一份（`total_count` 改名 `aggregated_count`、丢掉
  `conflict_count`、多出 `output_file`），conflicts 明细两侧都整份上线——与本组「写盘变换类只回
  写到哪、写了多少」的成文约定相反（实测那份 conflicts 明细 133 字节，四计数响应的十几倍）。
* **`target_size` 只对 weighted 生效（A294）**：实测 `target_size=1` 配 union ⇒ **20 条**、
  配 consistent ⇒ **20 条**、配 weighted ⇒ **1 条**。请求模型与 `docs/augmentor/API.md` 都没写这件事，
  所以页面把那一格标成「只 weighted 吃这一格」。`target_size=0` 是合法档（200 交空产物），
  `-1` 才是 400「target_size 必须是不小于 0 的整数，当前是 -1」；不给时走 SDK 的
  `DEFAULT_TARGET_SIZE = 100`（`augmentor/aggregator.py:22`），够不着就交全量。
* **consistent 连同源自比也算冲突，冲突条目整份不落盘（A297）**：单源两条同问不同答 ⇒
  `aggregated_count 0`、`conflicts` 一项且 `sources` 两个都是「甲」。名字叫「一致性聚合」，
  读起来像跨源比对，实测同源自比同样触发，且这些条目**不进产物**。
* **RAG 那一档站着第四份默认（A295）**：`fmt = request.format or rag_config.default_format or
  "langchain"`（`api/routes/dataset_tools.py:927`）。请求默认 `None`、config 默认 `"llamaindex"`
  （`augmentor/config.py:533`）、SDK 签名默认也是 `"llamaindex"`（`augmentor/rag.py:187`），
  只有路由那第三个字面量是 `"langchain"`，且只在 config 被设成空串时才轮得到它。
  另外实测：请求里**多送** `chunk_size` 一格 ⇒ 200 且产物逐字相同（请求模型没这一格，
  两个切块旋钮只从服务端配置读，`api/routes/dataset_tools.py:929`）；`format` 送空串与省略
  **等价**（两者都回 `llamaindex`）。三档 `record_count` 都等于 `input_count`（短文本不触发切块），
  但落盘键集各不相同：langchain 2 键、llamaindex 6 键、custom 5 键。
* **`size` 与 `ratio` 同给时 size 赢**：实测 `size=3` 与 `ratio=0.9` 同给，三支方法都交 **3 条**；
  `size=999` 超过总条数被**夹**到 20（不是拒绝）；`size=0` 交 200 并在盘上留一份内容为 `[]` 的文件。
* **路径判据三支一致**：白名单根**内**的绝对路径 ⇒ 200；根**外**的绝对路径 ⇒ 403「路径超出允许的
  数据目录范围」；名字里带 `..` ⇒ 400「路径包含非法组件」；输入不存在 ⇒ 404（字符串 detail）。

**一处被自己的夹具推翻的想象**（写进来看住它别再回来）：探针第一版想量 `value[:10]` 的折叠，
夹具写的是 `同一个前缀十个字A` / `…B`——那串**只有 9 个字符**，`[:10]` 压根折不起来，于是那一格
量到的是「三组各一条取前两组」，与折叠毫无关系，读数看起来却像成功了（200 / 2 条）。第二支探针
换成 10 个 `A` + 后缀才量出上面那条决定性读数。同一支探针里另一格也名不副实：标签写着「只有
`input` 不同的两条」，夹具却是 `instruction` 也不同的三条 ⇒ 那一格什么都没证，改由第二支探针的
`sameq.json`（同问不同答）与 `sameall.json`（同问同答不同 input）重新量。

跑法：`python -m pytest tests/unit/test_sample_aggregate_rag_wiring_l230.py -q --no-cov`
"""

import ast
import inspect
import json
import pathlib

import pytest
from tests.unit.test_transform_wiring_l229 import (
    _page,
    fetcher_body,
    select_block,
    state_initial,
    ts_interface_keys,
    ts_list,
)
from tests.unit.test_wired_dropdown_choices_l226 import needs_web_src

from api.routes.dataset_tools import (
    AggregateRequest,
    AggregateResponse,
    RagRequest,
    RagResponse,
    SampleRequest,
    SampleResponse,
)
from augmentor.aggregator import DEFAULT_TARGET_SIZE, AggregationResult, DataAggregator
from augmentor.config import RAG_FORMATS
from augmentor.dataset_ops import SampleConfig
from augmentor.rag import SUPPORTED_FORMATS, RAGFormatter

ROOT = pathlib.Path(__file__).resolve().parents[2]
AGGREGATOR_PY = ROOT / "augmentor" / "aggregator.py"
DATASET_OPS_PY = ROOT / "augmentor" / "dataset_ops.py"
DATASET_TOOLS_PY = ROOT / "api" / "routes" / "dataset_tools.py"
RAG_PY = ROOT / "augmentor" / "rag.py"

#: 20 条、4 类 instruction（每类 5 条），output 各不相同：分层那几支的配额读数靠这份语料
ITEMS = [{"instruction": f"cat{i % 4} 问题 {i}", "input": "", "output": f"答案 {i}"}
         for i in range(20)]
#: 前 6 条：两个源做并 / 交时给出「20 与 6」这一对可区分的读数
SUBSET = ITEMS[:6]
#: 前 10 个字符**真的**相同的两条 + 一条不同前缀：`value[:10]` 折叠的决定性夹具（A296）
COLLAPSE = [{"instruction": "AAAAAAAAAA甲", "input": "", "output": "o1"},
            {"instruction": "AAAAAAAAAA乙", "input": "", "output": "o2"},
            {"instruction": "BBBBBBBBBB丙", "input": "", "output": "o3"}]
#: 同 instruction、不同 output：判重只认 instruction（A293）与同源自比（A297）的夹具
SAME_Q = [{"instruction": "同一个问题", "input": "", "output": "答案甲"},
          {"instruction": "同一个问题", "input": "", "output": "答案乙"}]

#: 三档 RAG 格式落盘的键集（探针实测，逐字抄回）：条数一样，键集三样
RAG_KEY_SETS = {
    "langchain": {"metadata", "page_content"},
    "llamaindex": {"excluded_embed_metadata_keys", "excluded_llm_metadata_keys", "id_",
                   "metadata", "relationships", "text"},
    "custom": {"answer", "context", "id", "metadata", "query"},
}

PATHS = ["/api/dataset/sample", "/api/dataset/aggregate", "/api/dataset/rag"]


def _subject_of(node: ast.AST) -> str:
    """比较式左边那截的点号路径：`config.method` / `strategy`"""
    if isinstance(node, ast.Attribute):
        return f"{_subject_of(node.value)}.{node.attr}"
    if isinstance(node, ast.Name):
        return node.id
    return ""


def dispatch_arms(py_path: pathlib.Path, cls: str, func: str, subject: str) -> list:
    """把一个分发函数里 `subject == "字面量"` 的字符串**按源码顺序**抓回来

    这一支是「后端那份权威没有常量可引」时的读法：采样方法与聚合策略都只以 if/elif 分发的
    形状存在。抓不到东西要抛（数不到 ≠ 干净），与 l97 那把尺子同一口径。
    """
    tree = ast.parse(py_path.read_text(encoding="utf-8"))
    classes = [n for n in ast.walk(tree) if isinstance(n, ast.ClassDef) and n.name == cls]
    if not classes:
        raise LookupError(f"{py_path.name} 里没有类 {cls}")
    funcs = [n for n in classes[0].body
             if isinstance(n, ast.FunctionDef) and n.name == func]
    if not funcs:
        raise LookupError(f"{cls} 里没有方法 {func}")
    hits = [n for n in ast.walk(funcs[0])
            if isinstance(n, ast.Compare) and len(n.ops) == 1
            and isinstance(n.ops[0], ast.Eq) and _subject_of(n.left) == subject
            and isinstance(n.comparators[0], ast.Constant)
            and isinstance(n.comparators[0].value, str)]
    if not hits:
        raise LookupError(
            f"{cls}.{func} 里一个 `{subject} == 「字面量」` 都没读到：这是数不到，不是没有")
    return [n.comparators[0].value for n in sorted(hits, key=lambda n: n.lineno)]


def literal_value(text: str):
    """把 `useState(...)` 括号里的字面原文读成 Python 值（只认页面写得出的那几种形状）"""
    if text in ("true", "false", "null"):
        return {"true": True, "false": False, "null": None}[text]
    if text.replace(".", "", 1).isdigit():
        return float(text)
    return text.strip("'\"")


@pytest.fixture
def http_with_dataset_tools(tmp_path, monkeypatch):
    """白名单数据根：四份语料；根**外**另有一个 `outside/` 目录"""
    root = tmp_path / "ds"
    root.mkdir()
    outside = tmp_path / "outside"
    outside.mkdir()
    for name, payload in (("p.json", ITEMS), ("q.json", SUBSET),
                          ("collapse.json", COLLAPSE), ("sameq.json", SAME_Q)):
        (root / name).write_text(json.dumps(payload, ensure_ascii=False), encoding="utf-8")
    monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", str(root))
    monkeypatch.chdir(tmp_path)
    from fastapi.testclient import TestClient
    from api.main import app

    http = TestClient(app)

    def disk(name: str):
        """落盘产物读回来（写盘端点的产物本身就是判据的一半）"""
        return json.loads((root / name).read_text(encoding="utf-8"))

    return http, root, outside, disk


def sample(http, **kw):
    payload = {"input_file": "p.json", "output_file": "s.json", **kw}
    return http.post("/api/dataset/sample", json=payload)


@needs_web_src
class TestTheRulerProvesItself:
    """AST 那把尺子先自证：读得对、且数不到东西时会抛"""

    def test_it_reads_a_known_dispatch_shape(self, tmp_path):
        probe = tmp_path / "known_shape.py"
        probe.write_text(
            "class K:\n"
            "    def f(self, x, config):\n"
            "        if config.method == 'a':\n"
            "            return 1\n"
            "        elif config.method == 'b':\n"
            "            return 2\n"
            "        elif x == 'not_the_subject':\n"
            "            return 3\n"
            "        raise ValueError(config.method)\n",
            encoding="utf-8")
        assert dispatch_arms(probe, "K", "f", "config.method") == ["a", "b"]

    def test_it_refuses_a_silence_shaped_like_success(self, tmp_path):
        """抓不到分发就抛，而不是回一个空清单让「页面清单 == 后端清单」在空集上成立"""
        probe = tmp_path / "no_arms.py"
        probe.write_text("class K:\n    def f(self, x):\n        return x\n", encoding="utf-8")
        with pytest.raises(LookupError):
            dispatch_arms(probe, "K", "f", "strategy")
        with pytest.raises(LookupError):
            dispatch_arms(probe, "Nope", "f", "strategy")
        with pytest.raises(LookupError):
            dispatch_arms(probe, "K", "nope", "strategy")

    def test_it_reads_the_two_real_dispatches(self):
        """两处真权威各读出预期条数（3 与 4）——读成 0 条的话下面那些相等断言全是空的"""
        assert len(dispatch_arms(DATASET_OPS_PY, "DatasetOperations", "sample",
                                 "config.method")) == 3
        assert len(dispatch_arms(AGGREGATOR_PY, "DataAggregator", "aggregate",
                                 "strategy")) == 4


@needs_web_src
class TestTheThreeListsAreTheAuthorities:
    """三份下拉的清单必须**就是**后端那一处权威，页面里不许站第二份字面量"""

    def test_method_list_equals_the_sample_dispatch_arms(self):
        assert ts_list("SAMPLE_METHODS") == dispatch_arms(
            DATASET_OPS_PY, "DatasetOperations", "sample", "config.method")

    def test_strategy_list_equals_the_aggregate_dispatch_arms(self):
        assert ts_list("AGGREGATE_STRATEGIES") == dispatch_arms(
            AGGREGATOR_PY, "DataAggregator", "aggregate", "strategy")

    def test_format_list_equals_the_one_rag_authority_read_by_three_places(self):
        """RAG 那份是真常量，且已经被 config 引走：页面是第三个读者，不是第二份权威"""
        assert ts_list("RAG_FORMATS") == list(SUPPORTED_FORMATS)
        assert tuple(SUPPORTED_FORMATS) == RAG_FORMATS
        assert RAG_PY.read_text(encoding="utf-8").count("SUPPORTED_FORMATS = [") == 1

    def test_the_three_selects_render_the_lists_and_not_second_literals(self):
        """三个 `<select>` 都 `.map` 那份常量：候选项写死在 option 里就是第二份权威（A77）"""
        for label, name, sample_item in (("采样方法", "SAMPLE_METHODS", "'random'"),
                                         ("聚合策略", "AGGREGATE_STRATEGIES", "'union'"),
                                         ("RAG 格式", "RAG_FORMATS", "'custom'")):
            block = select_block(label)
            assert f"{name}.map" in block, (label, block)
            assert sample_item not in block, (label, block)

    def test_the_rag_select_offers_the_server_default_slot_on_top(self):
        """「服务端默认」那一档是空串，且在三档之外单独给（页面不抄 config 的默认值）"""
        block = select_block("RAG 格式")
        assert '<option value="">服务端默认' in block
        assert "llamaindex" not in block.split("RAG_FORMATS.map")[0].split("<option", 1)[-1]

    @pytest.mark.parametrize("method", ["random", "systematic", "stratified"])
    def test_every_method_the_page_offers_is_accepted(self, http_with_dataset_tools, method):
        """UI 能选到的三种方法都真的落得下去，且条数就是要的那 8 条"""
        http, _root, _outside, disk = http_with_dataset_tools
        resp = sample(http, method=method, size=8, seed=7, stratify_key="instruction",
                      output_file=f"s_{method}.json")
        assert resp.status_code == 200, resp.json()
        assert resp.json()["output_count"] == 8
        assert len(disk(f"s_{method}.json")) == 8

    @pytest.mark.parametrize("strategy", ["union", "intersection", "weighted", "consistent"])
    def test_every_strategy_the_page_offers_is_accepted(self, http_with_dataset_tools, strategy):
        """四档策略都落得下去；`source_counts` 逐源报的是**读进来**的条数"""
        http, _root, _outside, _disk = http_with_dataset_tools
        resp = http.post("/api/dataset/aggregate", json={
            "datasets": {"甲": "p.json", "乙": "q.json"},
            "output_file": f"agg_{strategy}.json", "strategy": strategy})
        assert resp.status_code == 200, resp.json()
        assert resp.json()["source_counts"] == {"甲": 20, "乙": 6}

    @pytest.mark.parametrize("fmt", ["llamaindex", "langchain", "custom"])
    def test_every_format_the_page_offers_is_accepted_and_keys_differ(
            self, http_with_dataset_tools, fmt):
        """三档都落得下去、条数都等于输入条数，但落盘的**键集**三样（探针实测那份表）"""
        http, _root, _outside, disk = http_with_dataset_tools
        resp = http.post("/api/dataset/rag", json={
            "input_file": "p.json", "output_file": f"rag_{fmt}.json", "format": fmt})
        assert resp.status_code == 200, resp.json()
        assert resp.json()["format"] == fmt
        assert resp.json()["record_count"] == resp.json()["input_count"] == 20
        records = disk(f"rag_{fmt}.json")
        assert len(records) == 20
        assert set(records[0]) == RAG_KEY_SETS[fmt]


@needs_web_src
class TestWiredDefaultsMatchTheContract:
    """页面初值逐项等于请求模型的默认；必填那几格页面**不给**默认"""

    def test_the_three_named_defaults_are_the_contract_defaults(self):
        for state_name, model, field in (("sampleMethod", SampleRequest, "method"),
                                         ("sampleKey", SampleRequest, "stratify_key"),
                                         ("aggStrategy", AggregateRequest, "strategy")):
            expected = model.model_fields[field].default
            assert literal_value(state_initial(state_name)) == expected, state_name

    def test_the_five_none_defaults_start_empty_because_omitting_is_a_real_option(self):
        """四个数值格是 `null`、格式那格是空串：留空＝那一格不发，页面不自带一套数字默认"""
        for name, model, field in (("sampleSize", SampleRequest, "size"),
                                   ("sampleRatio", SampleRequest, "ratio"),
                                   ("sampleSeed", SampleRequest, "seed"),
                                   ("aggTargetSize", AggregateRequest, "target_size"),
                                   ("ragFormat", RagRequest, "format")):
            assert model.model_fields[field].default is None, name
            assert state_initial(name) in ("null", "''"), name
        assert state_initial("ragFormat") == "''"

    def test_required_outputs_have_no_default_and_the_page_starts_empty(self):
        for model in (SampleRequest, AggregateRequest, RagRequest):
            assert model.model_fields["output_file"].is_required()
        for name in ("sampleOut", "aggOut", "ragOut"):
            assert state_initial(name) == "''", f"{name} 不该有默认值"
        assert state_initial("aggSelected") == "[]"
        assert state_initial("aggNames") == "{}"

    def test_the_sdk_target_size_default_is_not_copied_into_the_page(self):
        """`DEFAULT_TARGET_SIZE = 100` 只有 SDK 一处：页面留空就是不给，不抄那个数当默认"""
        assert DEFAULT_TARGET_SIZE == 100
        assert state_initial("aggTargetSize") == "null"

    def test_the_service_layer_writes_no_second_default(self):
        """三条 fetcher 只转实参：默认值的权威必须只有一处"""
        for name in ("sampleDataset", "aggregateDatasets", "convertToRag"):
            body = fetcher_body(name)
            assert "= '" not in body and "??" not in body, f"{name} 里出现了第二套默认值"

    def test_the_empty_format_sentinel_is_the_only_default_expression(self):
        """`format: opts.format || undefined` 是唯一一处默认表达式，而且是形状卫生（不是必需）

        实测送空串与省略**等价**（两者都回 `llamaindex`），所以这一处不是为了躲判决，
        是为了让线上只留「省略」一种形状 —— 与转换卡那处 `source_format` 的动机不同
        （那一处送空串会 400）。
        """
        body = fetcher_body("convertToRag")
        assert body.count("|| undefined") == 1
        assert "format: opts.format || undefined" in body

    def test_sending_an_empty_format_is_equivalent_to_omitting_it(
            self, http_with_dataset_tools):
        http, _root, _outside, disk = http_with_dataset_tools
        empty = http.post("/api/dataset/rag", json={
            "input_file": "p.json", "output_file": "rg_empty.json", "format": ""})
        omitted = http.post("/api/dataset/rag", json={
            "input_file": "p.json", "output_file": "rg_omit.json"})
        assert (empty.status_code, omitted.status_code) == (200, 200)
        assert empty.json()["format"] == omitted.json()["format"] == "llamaindex"
        assert disk("rg_empty.json") == disk("rg_omit.json")


@needs_web_src
class TestEveryArmTheUiCanSendForSample:
    """采样卡每个闸发得出的形状：要么被接受并改变产物，要么回一句点名原因的判决"""

    def test_size_wins_over_ratio_on_all_three_methods(self, http_with_dataset_tools):
        http, _root, _outside, _disk = http_with_dataset_tools
        for method in ("random", "systematic", "stratified"):
            resp = sample(http, method=method, size=3, ratio=0.9, seed=7,
                          output_file=f"both_{method}.json")
            assert resp.status_code == 200, resp.json()
            assert resp.json()["output_count"] == 3, method

    def test_size_zero_writes_a_real_empty_file(self, http_with_dataset_tools):
        """`size=0` 是「一条都不要」：200、`output_count 0`、盘上留一份 `[]`（不是不落盘）"""
        http, root, _outside, disk = http_with_dataset_tools
        resp = sample(http, size=0, output_file="zero.json")
        assert resp.status_code == 200
        assert resp.json()["output_count"] == 0
        assert disk("zero.json") == []
        assert (root / "zero.json").read_text(encoding="utf-8").strip() == "[]"

    def test_size_beyond_the_corpus_is_clamped_not_rejected(self, http_with_dataset_tools):
        http, _root, _outside, _disk = http_with_dataset_tools
        resp = sample(http, size=999)
        assert resp.status_code == 200
        assert (resp.json()["input_count"], resp.json()["output_count"]) == (20, 20)

    def test_negative_size_is_the_same_verdict_on_all_three_methods(
            self, http_with_dataset_tools):
        """`size` 那一格有判据（`require_count`），三支一致 —— **L277 起 `ratio` 那一格与它同等待遇**
        （见 `test_out_of_range_ratio_is_one_verdict_on_all_three_methods`；旧形状是「三支三结局」）。

        这一支保留独立判据而不是并入 ratio 那一支：`size` 走 `require_count`、`ratio` 走
        `require_ratio`，是两处判据、两句文案，任何一处回退都该单独喊红。
        """
        http, _root, _outside, _disk = http_with_dataset_tools
        for method in ("random", "systematic", "stratified"):
            resp = sample(http, method=method, size=-1, output_file=f"neg_{method}.json")
            assert resp.status_code == 400, (method, resp.json())
            assert resp.json()["detail"] == "size 必须是不小于 0 的整数，当前是 -1"

    @pytest.mark.parametrize("ratio", [2, 2.0, 1.5, -0.5, -1.0],
                             ids=["ratio=2", "ratio=2.0", "ratio=1.5",
                                  "ratio=-0.5", "ratio=-1.0"])
    def test_out_of_range_ratio_is_one_verdict_on_all_three_methods(
            self, http_with_dataset_tools, ratio):
        """A292 的这一半 L277 已收口：同一个越界比例，三种 method 一种结局（400 + 零产物）

        **这是契约翻面后的重写，不是把断言删松**。旧判据（L230 写下、L272 沿用的**反向**证据）
        钉的是「洞的形状」：`ratio=2` ⇒ random 400 且 detail 是 CPython 的英文原话
        「Sample larger than population or is negative」、systematic 与 stratified 各 200 交全量
        20 条；`ratio=-0.5` ⇒ random 400、systematic 200 交**前 10 条**（负数切片）、
        stratified 200 交 0 条。那句洞从来不是契约，所以新判据比旧的更强，四条都要成立：

        ① 状态码恰好 400（回退成 200 静默产物 ⇒ 红；被裸异常折成 500 ⇒ 也红）；
        ② detail **整句**等于那句中文区间判决 —— 回退成 CPython 英文原话虽然也是 400，
           但对客户端不可行动，那一格就是 A292 立账时点名的原话，所以整句相等而不是包含；
        ③ 三种 method 的归宿**逐字相同**（旧形状最贵的那一格正是「同值三结局」）；
        ④ 400 之前一格产物都没写：`output_file` 指向的文件在盘上不存在。

        `ratio=2`（整数）与 `ratio=2.0` 分列两档：请求模型是 `Optional[float]`，
        pydantic 会把 `2` 收进浮点 ⇒ 读数写作「当前是 2.0」，这一格钉住的是那条 coercion 不漂。
        """
        http, root, _outside, _disk = http_with_dataset_tools
        detail = f"ratio 必须是 0.0 到 1.0 之间的比例，当前是 {float(ratio)}"
        for method in ("random", "systematic", "stratified"):
            name = f"r_{ratio}_{method}.json"
            resp = sample(http, method=method, ratio=ratio, seed=7, output_file=name)
            assert resp.status_code == 400, (method, ratio, resp.json())
            assert resp.json()["detail"] == detail, (method, ratio, resp.json())
            assert "Sample larger than population" not in resp.text, (method, ratio)
            assert not (root / name).exists(), (method, ratio, "400 之前写了盘")

    def test_a_tiny_ratio_rounds_to_zero_on_all_three_methods(self, http_with_dataset_tools):
        """`int()` 归零那一档三支一致交 0 条（被 `dataset_ops.py:308` 的短路接住，不是三种异常）"""
        http, _root, _outside, _disk = http_with_dataset_tools
        for method in ("random", "systematic", "stratified"):
            resp = sample(http, method=method, ratio=0.0001, output_file=f"t_{method}.json")
            assert resp.status_code == 200, (method, resp.json())
            assert resp.json()["output_count"] == 0, method

    def test_seed_is_forwarded_and_its_absence_is_not_reproducible(
            self, http_with_dataset_tools):
        """同 seed 两次逐字相同；不给 seed 三次跑出三份（页面上那句「留空则两次不同」的证据）"""
        http, _root, _outside, disk = http_with_dataset_tools
        for name in ("sd1.json", "sd2.json"):
            assert sample(http, size=8, seed=42, output_file=name).status_code == 200
        assert disk("sd1.json") == disk("sd2.json")

        runs = []
        for i in range(3):
            assert sample(http, size=8, output_file=f"ns{i}.json").status_code == 200
            runs.append(json.dumps(disk(f"ns{i}.json"), ensure_ascii=False, sort_keys=True))
        # 三支互不相同才算不可复现。理论上有极小概率撞上重复（20 取 8 的一对约 1/125970），
        # 三份全同的概率在 1e-15 量级 —— 这一支刻意不钉 seed，钉的就是「没有 seed」这个状态。
        assert len(set(runs)) == 3

    def test_empty_stratify_key_is_a_verdict_while_a_ghost_one_is_not(
            self, http_with_dataset_tools):
        """空字符串拦（400），不存在的名字不拦（200 且塌成一个组）—— 这一松一紧最容易被读反"""
        http, _root, _outside, _disk = http_with_dataset_tools
        empty = sample(http, method="stratified", size=8, stratify_key="",
                       output_file="k_empty.json")
        assert empty.status_code == 400
        assert empty.json()["detail"] == "分层采样需要非空的 stratify_key"
        ghost = sample(http, method="stratified", size=8, stratify_key="nope",
                       output_file="k_ghost.json")
        assert ghost.status_code == 200
        assert ghost.json()["output_count"] == 8

    def test_the_stratify_group_key_collapses_at_ten_chars(self, http_with_dataset_tools):
        """A296 的决定性一格：前 10 个字符相同的两条被当成同一层，配额按**折叠后**的组数摊

        夹具三条 = `AAAAAAAAAA甲` / `AAAAAAAAAA乙` / `BBBBBBBBBB丙`。折叠 ⇒ 两组（2 与 1），
        `largest_remainder(2, [2, 1], caps=[2, 1], minimum_each=1)` 给 `[1, 1]` ⇒ 丙必在内、
        甲乙只留一条。不折叠 ⇒ 三组各一条、`_unit_cap_quota` 给 `[1, 1, 0]` ⇒ 甲乙都在、**丙不在**。
        两种读法的产物集合互斥，所以这一支不靠 seed 也判得住。
        """
        http, _root, _outside, disk = http_with_dataset_tools
        resp = http.post("/api/dataset/sample", json={
            "input_file": "collapse.json", "output_file": "c.json", "method": "stratified",
            "size": 2, "seed": 7, "stratify_key": "instruction"})
        assert resp.status_code == 200, resp.json()
        got = [item["instruction"] for item in disk("c.json")]
        assert "BBBBBBBBBB丙" in got, got
        assert sum(1 for g in got if g.startswith("AAAAAAAAAA")) == 1, got
        assert set(got) <= {"AAAAAAAAAA甲", "AAAAAAAAAA乙", "BBBBBBBBBB丙"}

    def test_unknown_method_is_a_verdict_naming_the_value(self, http_with_dataset_tools):
        http, _root, _outside, _disk = http_with_dataset_tools
        resp = sample(http, method="nope")
        assert resp.status_code == 400
        assert resp.json()["detail"] == "不支持的采样方法: nope"

    def test_the_page_does_not_offer_a_fourth_method_or_hide_one(self):
        """三份清单的**长度**也钉住：少一档是页面藏能力，多一档是页面编能力"""
        assert len(ts_list("SAMPLE_METHODS")) == 3
        assert len(ts_list("AGGREGATE_STRATEGIES")) == 4
        assert len(ts_list("RAG_FORMATS")) == 3
        assert set(SampleConfig.__dataclass_fields__) == {
            "method", "size", "ratio", "seed", "stratify_key"}


@needs_web_src
class TestEveryArmTheUiCanSendForAggregate:
    """聚合卡的闸：三个不可设的旋钮各自留下实测后果，空转的那一格点名说清"""

    def test_key_fields_are_not_settable_so_same_question_different_answer_collapses(
            self, http_with_dataset_tools):
        """A293 的判重那一半：union 把同问不同答并成一条，活下来的是**第一条**"""
        http, _root, _outside, disk = http_with_dataset_tools
        assert not {"key_fields", "value_fields", "random_state"} & set(
            AggregateRequest.model_fields)
        assert "DataAggregator()" in DATASET_TOOLS_PY.read_text(encoding="utf-8")
        resp = http.post("/api/dataset/aggregate", json={
            "datasets": {"甲": "sameq.json"}, "output_file": "k.json", "strategy": "union"})
        assert resp.status_code == 200, resp.json()
        body = resp.json()
        assert (body["aggregated_count"], body["removed_duplicates"]) == (1, 1)
        assert body["source_counts"] == {"甲": 2}
        assert [item["output"] for item in disk("k.json")] == ["答案甲"]

    def test_weighted_is_not_reproducible_because_random_state_is_unreachable(
            self, http_with_dataset_tools):
        """A293 的可复现那一半：同一份请求两次，落盘内容**不相等**（页面上那句「不可复现」的证据）

        理论上两次可能撞上同一份抽样（20 取 5 的一对约 1/15504），所以这一支连跑三次并要求
        至少出现两份不同的产物 —— 三份全同的概率在 1e-11 量级。
        """
        http, _root, _outside, disk = http_with_dataset_tools
        seen = set()
        for i in range(3):
            resp = http.post("/api/dataset/aggregate", json={
                "datasets": {"甲": "p.json"}, "output_file": f"w{i}.json",
                "strategy": "weighted", "target_size": 5})
            assert resp.status_code == 200, resp.json()
            assert resp.json()["aggregated_count"] == 5
            seen.add(json.dumps(disk(f"w{i}.json"), ensure_ascii=False, sort_keys=True))
        assert len(seen) > 1

    @pytest.mark.parametrize("strategy,expected", [("union", 20), ("intersection", 20),
                                                   ("weighted", 1), ("consistent", 20)],
                             ids=["union", "intersection", "weighted", "consistent"])
    def test_target_size_only_bites_the_weighted_branch(self, http_with_dataset_tools,
                                                        strategy, expected):
        """A294 的本体：`target_size=1` 只有 weighted 交 1 条，其余三支**完全空转**交全量"""
        http, _root, _outside, _disk = http_with_dataset_tools
        resp = http.post("/api/dataset/aggregate", json={
            "datasets": {"甲": "p.json"}, "output_file": f"t_{strategy}.json",
            "strategy": strategy, "target_size": 1})
        assert resp.status_code == 200, resp.json()
        assert resp.json()["aggregated_count"] == expected, strategy

    def test_target_size_zero_is_accepted_and_negative_is_a_verdict(
            self, http_with_dataset_tools):
        http, _root, _outside, _disk = http_with_dataset_tools
        zero = http.post("/api/dataset/aggregate", json={
            "datasets": {"甲": "p.json"}, "output_file": "tz.json",
            "strategy": "weighted", "target_size": 0})
        assert zero.status_code == 200
        assert zero.json()["aggregated_count"] == 0
        negative = http.post("/api/dataset/aggregate", json={
            "datasets": {"甲": "p.json"}, "output_file": "tn.json",
            "strategy": "weighted", "target_size": -1})
        assert negative.status_code == 400
        assert negative.json()["detail"] == "target_size 必须是不小于 0 的整数，当前是 -1"

    def test_omitting_target_size_falls_back_to_the_sdk_constant(
            self, http_with_dataset_tools):
        """不给那一格 ⇒ 走 SDK 的 100；语料只有 20 条，够不着就交全量（不是报错）"""
        http, _root, _outside, _disk = http_with_dataset_tools
        resp = http.post("/api/dataset/aggregate", json={
            "datasets": {"甲": "p.json"}, "output_file": "td.json", "strategy": "weighted"})
        assert resp.status_code == 200
        assert resp.json()["aggregated_count"] == 20 < DEFAULT_TARGET_SIZE

    def test_empty_datasets_and_unknown_strategy_are_both_verdicts(
            self, http_with_dataset_tools):
        http, _root, _outside, _disk = http_with_dataset_tools
        empty = http.post("/api/dataset/aggregate", json={
            "datasets": {}, "output_file": "e.json"})
        assert empty.status_code == 400
        assert empty.json()["detail"] == "datasets 不能为空"
        unknown = http.post("/api/dataset/aggregate", json={
            "datasets": {"甲": "p.json"}, "output_file": "u.json", "strategy": "nope"})
        assert unknown.status_code == 400
        assert unknown.json()["detail"] == "不支持的聚合策略: nope"

    def test_two_hand_maintained_receipt_key_sets_drift_and_both_carry_the_detail(
            self, http_with_dataset_tools):
        """A293 的回执那一半：**两份手写的键集各自漂**，而明细两侧都整份上线

        改前想象被实测推翻过一轮：本支初稿钉的是「SDK 的 `to_dict()` 只报 `conflict_count`、
        线上才有明细」——真跑一遍 `to_dict()`（`augmentor/aggregator.py:37`）交回五个键，
        `conflict_count` 与 `conflicts` **都在**。真形状是：端点在 `api/routes/dataset_tools.py:891`
        手拼了一份回执而没调 `to_dict()`，两份键集从此各走各路（`total_count` 改名
        `aggregated_count`、`conflict_count` 在线上被丢掉、`output_file` 只在线上有）。
        判据因此写成「逐键列出两份、点名差在哪」，而不是「一边有明细一边没有」。

        明细整份上线这一格与 `docs/augmentor/API.md` 给 dataset 那组定的「写盘变换类只回写到哪、
        写了多少」相反，所以页面把每条冲突逐字渲出来（不藏），同时在卡上写明它偏了约定。
        """
        sdk_keys = set(AggregationResult(aggregated=[], source_counts={}).to_dict())
        online_keys = set(AggregateResponse.model_fields)
        assert sdk_keys == {"total_count", "source_counts", "removed_duplicates",
                            "conflict_count", "conflicts"}
        assert online_keys == {"output_file", "aggregated_count", "source_counts",
                               "removed_duplicates", "conflicts"}
        assert sdk_keys - online_keys == {"total_count", "conflict_count"}
        assert online_keys - sdk_keys == {"output_file", "aggregated_count"}

        # 端点那一支里根本没有 `to_dict()`：两份键集是各自手写的，不是一份转另一份
        route = DATASET_TOOLS_PY.read_text(encoding="utf-8")
        _, _, tail = route.partition("async def dataset_aggregate")
        assert tail, "dataset_aggregate 端点没了 ⇒ 判据要跟着换形状"
        branch = tail.split("@router.post", 1)[0]
        assert "to_dict()" not in branch

        http, _root, _outside, _disk = http_with_dataset_tools
        resp = http.post("/api/dataset/aggregate", json={
            "datasets": {"甲": "sameq.json", "乙": "sameq.json"},
            "output_file": "cf.json", "strategy": "consistent"})
        assert resp.status_code == 200, resp.json()
        conflicts = resp.json()["conflicts"]
        assert len(conflicts) == 1
        assert set(conflicts[0]) == set(ts_interface_keys("AggregateConflict"))
        assert conflicts[0]["sources"] == ["甲", "甲", "乙", "乙"]
        assert conflicts[0]["values"] == ["答案甲", "答案乙", "答案甲", "答案乙"]

    def test_a_single_source_conflicts_with_itself_and_those_items_never_land(
            self, http_with_dataset_tools):
        """A297：单源同问不同答 ⇒ `aggregated_count 0`、conflicts 一项、两个 source 都是「甲」"""
        http, _root, _outside, disk = http_with_dataset_tools
        resp = http.post("/api/dataset/aggregate", json={
            "datasets": {"甲": "sameq.json"}, "output_file": "self.json",
            "strategy": "consistent"})
        assert resp.status_code == 200, resp.json()
        body = resp.json()
        assert body["aggregated_count"] == 0
        assert body["conflicts"][0]["sources"] == ["甲", "甲"]
        assert disk("self.json") == []

    def test_the_page_names_sources_itself_and_blocks_a_colliding_pair(self):
        """来源名是**页面**起的（契约不给默认），且重名在发之前被拦下

        重名会让映射互相覆盖 ⇒ 回执的 `source_counts` 少一格，那是静默丢源。这一支钉的是
        页面上真有那道拦（不是只钉「页面有个输入框」）。
        """
        page = _page()
        assert "replace(/\\.[^.]+$/, '')" in page
        assert "来源名" in page and "重复" in page
        assert "AggregateRequest" not in page


@needs_web_src
class TestEveryArmTheUiCanSendForRag:
    """RAG 卡的闸：三档格式、第四份默认、以及两个只在服务端配置里的旋钮"""

    def test_invalid_format_is_a_verdict_that_enumerates_the_three(
            self, http_with_dataset_tools):
        """端点自己把三档列全 ⇒ 抓回来与页面那份逐项相等，不在测试里抄第二份名单"""
        http, _root, _outside, _disk = http_with_dataset_tools
        resp = http.post("/api/dataset/rag", json={
            "input_file": "p.json", "output_file": "bad.json", "format": "nope"})
        assert resp.status_code == 400
        detail = resp.json()["detail"]
        assert detail == ("不支持的 RAG 格式: nope。支持: "
                          "['llamaindex', 'langchain', 'custom']")
        assert detail.split("支持: ")[1] == str(ts_list("RAG_FORMATS"))

    def test_extra_chunk_knobs_are_silently_ignored(self, http_with_dataset_tools):
        """请求模型没有 `chunk_size` / `chunk_overlap`，多送 ⇒ 200 且产物逐字相同（静默忽略）"""
        assert not {"chunk_size", "chunk_overlap"} & set(RagRequest.model_fields)
        http, _root, _outside, disk = http_with_dataset_tools
        plain = http.post("/api/dataset/rag", json={
            "input_file": "p.json", "output_file": "c_plain.json", "format": "langchain"})
        extra = http.post("/api/dataset/rag", json={
            "input_file": "p.json", "output_file": "c_extra.json", "format": "langchain",
            "chunk_size": 1, "chunk_overlap": 0})
        assert (plain.status_code, extra.status_code) == (200, 200)
        assert disk("c_plain.json") == disk("c_extra.json")
        assert "chunk_size=rag_config.chunk_size" in \
            DATASET_TOOLS_PY.read_text(encoding="utf-8")

    def test_the_fourth_default_literal_disagrees_with_both_authorities(self):
        """A295：路由那第三个兜底字面量是 `langchain`，config 与 SDK 两处都是 `llamaindex`"""
        route = DATASET_TOOLS_PY.read_text(encoding="utf-8")
        assert 'request.format or rag_config.default_format or "langchain"' in route
        from augmentor.config import RAGConfig
        assert RAGConfig.__dataclass_fields__["default_format"].default == "llamaindex"
        assert inspect.signature(RAGFormatter.format).parameters["fmt"].default == "llamaindex"

    def test_record_count_is_input_count_for_short_texts(self, http_with_dataset_tools):
        """三档都是 20 进 20 出：这一格不能读成「条数 = 块数」的保证（短文本不触发切块）"""
        http, _root, _outside, _disk = http_with_dataset_tools
        for fmt in SUPPORTED_FORMATS:
            resp = http.post("/api/dataset/rag", json={
                "input_file": "p.json", "output_file": f"n_{fmt}.json", "format": fmt})
            body = resp.json()
            assert (body["input_count"], body["record_count"]) == (20, 20), fmt


@needs_web_src
class TestSharedPathAndMissingInputVerdicts:
    """三只端点共用的那一套路径判据与 404 形状（与 L229 那三只逐格相同）"""

    @pytest.mark.parametrize("path,payloads", [
        ("/api/dataset/sample", {"field": "output_file", "ok": "x.json",
                                 "traversal": "../evil.json",
                                 "source": {"input_file": "p.json"}}),
        ("/api/dataset/aggregate", {"field": "output_file", "ok": "a.json",
                                    "traversal": "../evil.json",
                                    "source": {"datasets": {"甲": "p.json"}}}),
        ("/api/dataset/rag", {"field": "output_file", "ok": "r.json",
                              "traversal": "../evil.json",
                              "source": {"input_file": "p.json"}}),
    ], ids=["sample", "aggregate", "rag"])
    def test_inside_absolute_is_200_outside_is_403_and_dotdot_is_400(
            self, http_with_dataset_tools, path, payloads):
        """白名单根**内**的绝对路径 200；根外 403；带 `..` 400 —— 把绝对路径一律当非法就会误拒第一档"""
        http, root, outside, _disk = http_with_dataset_tools
        field, source = payloads["field"], payloads["source"]
        inside = http.post(path, json={**source, field: str(root / payloads["ok"])})
        assert inside.status_code == 200, inside.json()
        escaped = http.post(path, json={**source, field: str(outside / payloads["ok"])})
        assert escaped.status_code == 403
        assert escaped.json()["detail"] == "路径超出允许的数据目录范围"
        traversal = http.post(path, json={**source, field: payloads["traversal"]})
        assert traversal.status_code == 400
        assert traversal.json()["detail"] == "路径包含非法组件"

    def test_missing_input_is_404_with_a_string_detail_for_all_three(
            self, http_with_dataset_tools):
        """三只端点的「文件不存在」都是**字符串** detail ⇒ `apiErrorDetail` 的主支不是想象"""
        http, _root, _outside, _disk = http_with_dataset_tools
        for path, payload in (
            ("/api/dataset/sample", {"input_file": "nope.json", "output_file": "a.json"}),
            ("/api/dataset/aggregate", {"datasets": {"甲": "nope.json"},
                                        "output_file": "b.json"}),
            ("/api/dataset/rag", {"input_file": "nope.json", "output_file": "c.json"}),
        ):
            resp = http.post(path, json=payload)
            assert resp.status_code == 404, (path, resp.json())
            assert isinstance(resp.json()["detail"], str)

    def test_the_three_paths_are_the_only_new_targets(self):
        """三条新 fetcher 只打这三条路径（拼错的端点在这一档就响，不必等 l224 的集合相等）"""
        for name, path in (("sampleDataset", "/dataset/sample"),
                           ("aggregateDatasets", "/dataset/aggregate"),
                           ("convertToRag", "/dataset/rag")):
            assert f"api.post('{path}'" in fetcher_body(name), f"{name} 打的不是 {path}"
        assert PATHS == ["/api/dataset/sample", "/api/dataset/aggregate", "/api/dataset/rag"]


@needs_web_src
class TestReceiptCoversEveryResponseKey:
    """响应模型的每一格都要在 TS 面上有名字、在页面回执里被读到——少一格就是吞掉一个读数"""

    @pytest.mark.parametrize("iface,model,variable", [
        ("SampleResponse", SampleResponse, "sampleResult"),
        ("AggregateResponse", AggregateResponse, "aggResult"),
        ("RagResponse", RagResponse, "ragResult"),
    ], ids=["SampleResponse", "AggregateResponse", "RagResponse"])
    def test_ts_face_matches_the_response_model(self, iface, model, variable):
        del variable
        assert set(ts_interface_keys(iface)) == set(model.model_fields)

    @pytest.mark.parametrize("iface,model,variable", [
        ("SampleResponse", SampleResponse, "sampleResult"),
        ("AggregateResponse", AggregateResponse, "aggResult"),
        ("RagResponse", RagResponse, "ragResult"),
    ], ids=["SampleResponse", "AggregateResponse", "RagResponse"])
    def test_every_key_is_read_in_the_page_receipt(self, iface, model, variable):
        del iface
        page = _page()
        for key in model.model_fields:
            assert f"{variable}.{key}" in page, f"{model.__name__}.{key} 在页面上没人读"

    def test_every_conflict_item_key_is_read_too(self):
        """`conflicts` 那一项的三键也都要被读到：只报「有几处冲突」等于把判决内容吞掉"""
        page = _page()
        for key in ("key", "values", "sources"):
            assert f"conflict.{key}" in page, f"AggregateConflict.{key} 在页面上没人读"
        assert set(ts_interface_keys("AggregateConflict")) == {"key", "values", "sources"}


@needs_web_src
class TestHonestyCopyOnTheThreeCards:
    """三张卡上那几句「这一档其实拦不住 / 其实不可设」的口径，每一句都要有实测陪着"""

    def test_the_sample_card_says_the_ratio_split_and_the_ten_char_collapse(self):
        page = _page()
        assert "条数赢" in page
        assert "一律 400" in page and "一种结局" in page
        assert "负数切片" in page  # 只作为**改前**的取证留在卡上
        assert "没有判据" not in page  # L277 收了 A292 的范围维，这句成了假话
        assert "前 10 个字符" in page
        assert "塌成一个分组" in page
        assert "A292" in page and "A296" in page

    def test_the_aggregate_card_says_the_three_unreachable_knobs_and_the_idle_target(self):
        page = _page()
        assert "不可复现" in page
        assert "完全空转" in page
        assert "同一个源内部" in page
        assert "key_fields" in page and "random_state" in page
        assert "A293" in page and "A294" in page and "A297" in page

    def test_the_rag_card_says_the_config_only_knobs_and_the_fourth_default(self):
        page = _page()
        assert "只从服务端配置读" in page and "静默忽略" in page
        assert "第四份默认字面量" in page
        assert "A295" in page

    def test_the_page_names_every_receipt_instead_of_pretending_to_preview(self):
        page = _page()
        for name in ("采样回执", "聚合回执", "RAG 回执"):
            assert f'aria-label="{name}"' in page
        assert "预览" not in page

    def test_the_source_name_rule_is_declared_as_a_page_choice_not_a_contract(self):
        """来源名的 stem 规则必须写明是**页面**起的：契约不替你取名，用户会以为是服务端默认"""
        assert "来源名由**页面**起" in _page()


# ------------------------------------------------------------------ 文档那一侧
# 这两条路径常量放在使用点旁边：本文件其余常量都在头部，但这一段是 L230 收口时
# 追加的「文档与实测对账」，与前面的接线判据是两回事。
#: 子项目文档已于 2026-10-14 汇总到仓库根 `docs/augmentor/`；本文件自持这一真源
DOCS_DIR = ROOT.parent / "docs" / "augmentor"

API_MD = DOCS_DIR / "API.md"
README_MD = ROOT / "README.md"


def doc_row(path: pathlib.Path, needle: str) -> str:
    """把包含 `needle` 的那**一行**抓回来；抓不到或多于一行都要抛（数不到 ≠ 干净）"""
    hits = [line for line in path.read_text(encoding="utf-8").splitlines() if needle in line]
    if len(hits) != 1:
        raise LookupError(f"{path.name} 里 {needle!r} 命中 {len(hits)} 行，取不出唯一那一行")
    return hits[0]


class TestTheDocRowsSayWhatTheEndpointDoes:
    """`docs/augmentor/API.md` 那两行散文注必须与实测同口径——本轮订正过两处，判据住在这里防回潮

    **改前实测**（L230 收口时逐条量，两句话都与端点行为不符）：

    * aggregate 那行写「`weighted` 策略下 `target_size` 省略会 400（不让 `None` 溜进算术）」
      —— 实测 **200**，`aggregated_count` 20（语料只有 20 条，`DEFAULT_TARGET_SIZE = 100`
      够不着就交全量）。那句「不让 None 溜进算术」描述的是**已被兜住**的那条路径：
      `aggregate()` 透传 `kwargs.get("target_size")`，由 `aggregate_weighted` 统一接住
      `None` 并换成 `DEFAULT_TARGET_SIZE`（`augmentor/aggregator.py:161`）。
    * rag 那行写「`config.rag.default_format`（出厂默认 `langchain`）」—— 实测出厂值是
      **`llamaindex`**（`AppConfig().rag.default_format`），`langchain` 是路由那第三个兜底
      字面量（A295），只在配置被设成空串时才轮得到。省略 `format` 的实测回执也是 `llamaindex`。

    两句都在**同一行**上订正（净行数 0，避免平移历史 `file.py:NNN` 引用）。
    """

    def test_the_aggregate_row_no_longer_claims_a_400_the_endpoint_does_not_give(
            self, http_with_dataset_tools):
        row = doc_row(API_MD, "`/api/dataset/aggregate` | 多源聚合 | **写盘变换类**")
        # 认「原句」而不是认「省略会 400」这四个字：订正说明里要**引用**被推翻的那句，
        # 只认子串就会把订正说明自己判成回潮。
        assert "`target_size` 省略会 400" not in row, "那句被实测推翻的判决还留在文档里"
        assert "不让 `None` 溜进算术" not in row
        assert "省略不是 400" in row and "DEFAULT_TARGET_SIZE" in row
        # 文档那句订正后的话必须与端点当场的行为同口径，而不是「读起来对」
        http, _root, _outside, _disk = http_with_dataset_tools
        resp = http.post("/api/dataset/aggregate", json={
            "datasets": {"甲": "p.json"}, "output_file": "omit.json", "strategy": "weighted"})
        assert resp.status_code == 200, resp.json()
        assert resp.json()["aggregated_count"] == min(DEFAULT_TARGET_SIZE, len(ITEMS))

    def test_the_aggregate_row_declares_the_receipt_deviation(self):
        """`conflicts` 明细整份上线 + 两份键集各自漂，这两件事文档必须点名（不点名就会被读成约定内）"""
        row = doc_row(API_MD, "`/api/dataset/aggregate` | 多源聚合 | **写盘变换类**")
        assert "偏了本组" in row and "conflicts" in row
        assert "to_dict()" in row and "A294" in row and "A297" in row

    def test_the_rag_row_names_the_real_factory_default(self, http_with_dataset_tools):
        from augmentor.config import AppConfig

        row = doc_row(API_MD, "`/api/dataset/rag` | 转换为 RAG 格式 | **写盘变换类**")
        assert "（出厂默认 `langchain`）" not in row, "被实测推翻的那个默认值还留在文档里"
        assert "`llamaindex`" in row and "augmentor/config.py:533" in row and "A295" in row
        assert AppConfig().rag.default_format == "llamaindex"

        http, _root, _outside, _disk = http_with_dataset_tools
        resp = http.post("/api/dataset/rag", json={
            "input_file": "p.json", "output_file": "omit_fmt.json"})
        assert resp.status_code == 200, resp.json()
        assert resp.json()["format"] == "llamaindex"

    def test_the_readme_page_row_lists_the_three_new_cards(self):
        """README 那一行是页面能力的清单：接了三张卡却不写，等于对外少报三个入口"""
        row = doc_row(README_MD, "| `/` | 数据管理 |")
        for phrase in ("采样（三种方法", "多源聚合（四策略", "转成 RAG 检索格式（三档"):
            assert phrase in row, phrase
        # 三张卡各自的诚实口径也要在清单里露一句，否则 README 读起来像「三个都能直接用」
        assert "ratio" in row and "冲突明细逐条渲出" in row
