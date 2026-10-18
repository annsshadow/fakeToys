# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L228：stats / features / compare 接线之后，页面上的候选、口径与量纲都不许漂

**改前实测**（`git show HEAD:./web/src/services/api.ts` / 同 `pages/Analysis.tsx`，L228 取证）：

| 事实 | 改前 | 改后 |
| --- | --- | --- |
| 前端调用 `/dataset/stats` | **0 处**（在 l224 的 `UNCONSUMED` 里） | `datasetStats` + 分析页一张卡 |
| 前端调用 `/dataset/features` | **0 处**（同上） | `detectFeatures` + 分析页一张卡 |
| 前端调用 `/dataset/compare` | **0 处**（同上） | `compareDatasets` + 分析页一张卡 |
| 页面上有没有手写字段名清单 | 有（impact / semantic 两张卡各写 `instruction`、`output` 两档） | 新增两张卡的候选**只能来自回执**，本文件钉住「无字面量 option」 |

**为什么这一批要换一条判据**：l226 那把尺子量的是「封闭清单 vs 可 import 的 Python 常数」，
l227 因为**没有**这种常数（`["instruction", "input", "output"]` 只是 `audit.py:69` 等处的
手写字面量，见 **A285**）退到「页面发得出每一档都被端点接受」。这一批连那条都不够用——
实测这两档旋钮给错值时后端**不出声**：

* `POST /api/dataset/stats` 的 `fields:["nope"]` ⇒ **200**，回执里多出一个整套全零的
  **幽灵字段**（十二个键全在，值全 0）；
* `POST /api/dataset/compare` 的 `key_fields:["nope"]` ⇒ **200**，而且更糟：两份毫不相干的
  数据回 `common_items: 1`、`similarity_score: 1.0`（同一条数据默认键回的是 `0` 与 `0.0`）——
  幽灵匹配键把「毫无重叠」**谎报成「完全重叠」**。

所以页面这两处不许出现任何手写字段名，候选只能从回执的键里来；这一条由
`TestCandidatesComeFromTheReceipt` 守（它查的是「那个下拉里没有字面量 option」）。

**一条被探针推翻的想象**（写进来看住它别再回来）：本文件初稿断言「换匹配键会让逐条值差异
从空变成逐条列出」，读了 `augmentor/comparison.py:217` 与 `compare_enhanced.py:289` 才知道
`_compare_fields(items_a, items_b)` 压根不接 `key_fields`——明细永远按 `instruction` 配对、
条目键固定是 `{instruction, value_a, value_b}`，`key_fields` **只**动 `metrics` 那一组数。
`test_key_fields_do_not_move_the_per_field_detail_table` 钉的就是这个不变量。

**探针量到的其余事实**（判据的依据，不是想象）：
* `quality_metrics` 三项**不随 `fields` 变**（实测 `fields:["output"]` 与缺省两跑逐值相等），
  因为 SDK 只遍历 `instruction` 与 `output`（`statistics.py:311`–`333`）⇒ 页面口径栏必须写明
  「与 fields 无关」，否则用户会读成「按所选字段算的质量」；
* 空数据集（`[]`）⇒ `field_statistics` / `content_statistics` / `quality_metrics` **全是 `{}`**，
  `content_statistics` 不是「有结构但值为零」而是两个子键根本不出现 ⇒ TS 类型里两个子键是可选的；
* B 侧 0 条时 `size_ratio` 在 SDK 里是 `float('inf')`（`compare_enhanced.py:190`），
  Pydantic 序列化到线上是 **`null`**（严格 JSON 面安全，但 TS 不能当纯 `number`）⇒ **A288**；
* `similarity_to_a` / `similarity_to_b` 在 `comparison.py` 里**只有 dataclass 默认值与
  `to_dict()` 两处**，全仓无赋值点 ⇒ 恒 `0.0` 的死面，页面不渲它 ⇒ **A287**；
* 两侧同文件 ⇒ `winner == "tie"`、`quality_diff` 四项全 `0.0`（判据是
  `abs(avg_a − avg_b) < 0.01`，`comparison.py:260`）⇒ 三态而不是「有没有胜者」；
* `min_coverage` 出厂 `0.1`（`feature_detect.py:59`）且 `POST /api/dataset/features` 的请求模型
  只有 `input_file`（实测多送 `text_field` 被静默忽略）⇒ 这一档端点不可调，页面不装旋钮。

跑法：`python -m pytest tests/unit/test_stats_features_compare_wiring_l228.py -q --no-cov`
"""

import json
import pathlib
import re

import pytest
from tests.unit.test_wired_dropdown_choices_l226 import (
    needs_web_src,
    strip_comments,
)

from api.routes.dataset_tools import CompareRequest, DatasetFileRequest, StatsRequest

SRC = pathlib.Path(__file__).resolve().parents[2] / "web" / "src"
ANALYSIS_TSX = SRC / "pages" / "Analysis.tsx"
TYPES_TS = SRC / "types" / "api.ts"
API_TS = SRC / "services" / "api.ts"
COMPARISON_PY = (pathlib.Path(__file__).resolve().parents[2]
                 / "augmentor" / "comparison.py")

ITEMS = [
    {"instruction": "请解释什么是向量数据库", "input": "", "output": "向量数据库按相似度检索高维向量。"},
    {"instruction": "写一首关于秋天的诗", "input": "主题：秋天", "output": "落叶铺满小径，秋天把颜色交给风。"},
    {"instruction": "什么是包含判断", "input": "", "output": "包含判断是子串匹配。"},
    {"instruction": "正则怎么用", "input": "", "output": "正则用元字符描述待匹配的形状。"},
]

# output 全空的集：`completeness` 掉到 0.5、`consistency` 掉到 0.0 的那一档
NO_OUTPUT_ITEMS = [
    {"instruction": "甲", "input": "", "output": ""},
    {"instruction": "乙", "input": "", "output": ""},
]

# 同问不同答：instruction 与 R 侧完全相同、output 全部改写。这一对是「匹配键」旋钮的
# 取证夹具：按 instruction 配 ⇒ 交集 4、相似度 1.0；按 output 配 ⇒ 交集 0、相似度 0.0。
# 用 `q.json`（内容与 p 同、只多一条重复）做不到这件事——两档键都会给出高分。
REWRITTEN_ITEMS = [
    {"instruction": i["instruction"], "input": i["input"], "output": f"回答{n}号"}
    for n, i in enumerate(ITEMS)
]

ENTRY_RE = re.compile(r"(\w+):\s*\{\s*label:\s*'([^']+)',\s*caliber:\s*'([^']+)'")
FLAT_MAP_RE = re.compile(r"^\s*(\w+):\s*'([^']+)',?\s*$", re.M)
KEYED_LABEL_RE = re.compile(r"^\s*(\w+):\s*\{\s*label:\s*'([^']+)'", re.M)
KIND_RE = re.compile(r"(\w+):\s*\{[^{}]*kind:\s*'(\w+)'")
TUPLE_ROW_RE = re.compile(r"\['(\w+)',\s*'([^']+)',\s*'(\w+)'\]")
EMPTY_LIST_STATE_RE = re.compile(r"const \[(\w+), set\w+\] = useState<string\[\]>\(\[\]\)")
ASSIGN_RE = re.compile(r"(?:self|result)\.similarity_to_[ab]\s*=[^=]")


def _page() -> str:
    assert ANALYSIS_TSX.is_file(), "前端源码不在场 ⇒ 本文件的判据全部失效"
    return strip_comments(ANALYSIS_TSX.read_text(encoding="utf-8"))


def _const_block(name: str) -> str:
    """取 `const NAME ... = {…}` 或 `= […]` 那一整块（到**行首**的 `}` / `]` 为止）

    非贪婪 + 行首闭括号：块内的 `}` 都跟在一行条目的末尾（`...},`），不会提前收口。
    """
    found = re.search(rf"const {name}[^=]*= [{{\[].*?\n[}}\]]", _page(), re.S)
    assert found, f"Analysis 里没有 {name} 常量 ⇒ 判据要跟着换形状"
    return found.group(0)


def labeled_calibers(name: str) -> dict:
    """`Record<string, {label, caliber}>` 型常量 ⇒ `{键: (标签, 口径)}`"""
    return {key: (label, caliber)
            for key, label, caliber in ENTRY_RE.findall(_const_block(name))}


def flat_labels(name: str) -> dict:
    """`Record<string, string>` 型常量 ⇒ `{键: 标签}`"""
    return {key: label for key, label in FLAT_MAP_RE.findall(
        _const_block(name).split("=", 1)[1])}


def keyed_labels(name: str) -> dict:
    """`Record<string, {label, …}>` 型常量 ⇒ `{键: 标签}`（值不止 label 时 flat_labels 匹配不到）"""
    return {key: label for key, label in KEYED_LABEL_RE.findall(_const_block(name))}


def kinds(name: str) -> dict:
    """带 `kind` 分档的常量 ⇒ `{键: 分档名}`"""
    return {key: kind for key, kind in KIND_RE.findall(_const_block(name))}


def tuple_rows(name: str) -> dict:
    """`[键, 中文名, 量纲]` 三元组清单 ⇒ `{键: (中文名, 量纲)}`"""
    return {key: (label, kind) for key, label, kind in TUPLE_ROW_RE.findall(_const_block(name))}


def select_block(state_name: str) -> str:
    """那个 `value={state_name}` 的 `<Select … />` 片段（候选清单就在它的 `options` 里）"""
    page = _page()
    anchor = f"value={{{state_name}}}"
    assert anchor in page, f"页面里没有 value={{{state_name}}} 的控件 ⇒ 判据要跟着换形状"
    start = page.index(anchor)
    end = page.index("/>", start)
    return page[start:end]


@pytest.fixture
def http_with_datasets(tmp_path, monkeypatch):
    """白名单数据根：`p.json`(4)、`q.json`(5，含一条重复)、`r.json`(同问不同答)、`n.json`(output 全空)、`empty.json`"""
    root = tmp_path / "ds"
    root.mkdir()
    (root / "p.json").write_text(json.dumps(ITEMS, ensure_ascii=False), encoding="utf-8")
    (root / "q.json").write_text(
        json.dumps(ITEMS + [dict(ITEMS[0])], ensure_ascii=False), encoding="utf-8")
    (root / "r.json").write_text(json.dumps(REWRITTEN_ITEMS, ensure_ascii=False),
                                 encoding="utf-8")
    (root / "n.json").write_text(json.dumps(NO_OUTPUT_ITEMS, ensure_ascii=False), encoding="utf-8")
    (root / "empty.json").write_text("[]", encoding="utf-8")
    monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", str(root))
    monkeypatch.chdir(tmp_path)
    from fastapi.testclient import TestClient
    from api.main import app

    return TestClient(app), root


@needs_web_src
class TestWiredDefaults:
    """三只新 fetchers 不许写第二套默认，页面留空＝契约留空"""

    def test_the_three_new_states_start_empty(self):
        """两个多选清单与一个 B 侧选择都从空开始：非空初值等于页面替端点编默认值"""
        declared = set(EMPTY_LIST_STATE_RE.findall(_page()))
        assert {"statsFields", "compareKeyFields"} <= declared, declared
        assert re.search(r"const \[compareFile, setCompareFile\] = useState\(''\)", _page())

    def test_the_contract_defaults_are_none_not_a_field_list(self):
        """`fields` / `key_fields` 在请求模型里的默认是 `None`（＝「由 SDK 决定」而不是「某份清单」）

        这一条是「留空即不发」的前提：如果哪天模型改成 `["instruction"]`，页面留空发
        `undefined` 与发那份清单就不再等价，这条会红着提醒。
        """
        assert StatsRequest.model_fields["fields"].default is None
        assert CompareRequest.model_fields["key_fields"].default is None
        assert DatasetFileRequest.model_fields.keys() == {"input_file"}

    def test_the_service_layer_writes_no_second_default(self):
        """服务层只转实参：默认值的权威必须只有一处（页面、服务层、请求模型各一套就是三套）"""
        src = strip_comments(API_TS.read_text(encoding="utf-8"))
        for name in ("datasetStats", "detectFeatures", "compareDatasets"):
            _, _, body = src.partition(f"export const {name}")
            assert body, f"api.ts 里 {name} 没了"
            head = body[:700]
            assert "= '" not in head and "??" not in head, \
                f"{name} 里出现了第二套默认值"

    def test_empty_opts_really_omit_the_key_on_the_wire(self, http_with_datasets):
        """`fields: undefined` 序列化后这个键不出现 ⇒ 走的是模型的 `None`，不是空列表

        L277（修法①）之后 `fields: []` 与「不发这一键」**不再等价**：留空 ⇒ 全字段（模型的
        `None` 档），显式空清单 ⇒ 零字段作用域（`field_statistics` 是空字典）。两档可区分正是
        这一族要的结果，所以这里钉两件事：① 空清单不再被吞成全量；② 页面留空时线上没有
        `fields` 这一档（拿的还是全量）。
        """
        http, _ = http_with_datasets
        with_list = http.post("/api/dataset/stats",
                              json={"input_file": "p.json", "fields": []}).json()
        omitted = http.post("/api/dataset/stats", json={"input_file": "p.json"}).json()
        assert with_list["field_statistics"] == {}, "空清单又被吞成全字段 ⇒ 修法① 被回退"
        assert set(omitted["field_statistics"]) == {"instruction", "input", "output"}


@needs_web_src
class TestCandidatesComeFromTheReceipt:
    """两个新下拉的候选只能来自回执，页面里不许出现手写字段名 option"""

    @pytest.mark.parametrize("state_name", ["statsFields", "compareKeyFields"])
    def test_no_literal_field_option_in_either_dropdown(self, state_name):
        """两个新下拉的 options 里不许出现一个字面量字段名（写死＝替数据集编清单）"""
        block = select_block(state_name)
        assert "value: '" not in block, \
            f"{state_name} 的下拉里出现了字面量字段名选项: {block}"

    def test_field_candidates_come_from_the_stats_receipt(self):
        """限定字段下拉的候选来自 `field_statistics` 的键，且只用**未限定**那次的回执刷新"""
        page = _page()
        assert re.search(r"options=\{statFieldUniverse\.map", select_block("statsFields")), \
            "限定字段下拉的候选不是来自 statFieldUniverse"
        assert re.search(r"setStatFieldUniverse\(Object\.keys\(result\.field_statistics\)\)",
                         page), "字段清单的来源不是回执的 field_statistics 键集"
        # 刷新必须裹在「本次没限定字段」的闸里，否则勾了 output 之后清单会塌成一格
        assert re.search(r"if \(!statsFields\.length\) \{\s*setStatFieldUniverse", page), \
            "字段清单被限定那次的回执覆盖了 ⇒ 用户会把自己锁死在勾过的那一格"

    def test_key_field_candidates_come_from_the_compare_receipt(self):
        """匹配键下拉的候选来自 `field_comparisons` 的键（两侧字段并集，页面不另编一份）"""
        assert "Object.keys(comparison?.overlap_comparison.field_comparisons" in \
            select_block("compareKeyFields"), "匹配键候选不是从回执的 field_comparisons 来的"

    def test_receipt_derived_candidates_are_all_accepted_by_the_endpoint(self, http_with_datasets):
        """页面发得出的每一档仍须被端点接受（沿用 l227 那条运行时判据）

        权威是**真实回执**：stats 的候选来自 `field_statistics` 的键，compare 的候选来自
        `field_comparisons` 的键。用真实数据把这两集合取出来，逐个发一遍。
        """
        http, _ = http_with_datasets
        stats = http.post("/api/dataset/stats", json={"input_file": "p.json"}).json()
        for field in stats["field_statistics"]:
            response = http.post("/api/dataset/stats",
                                 json={"input_file": "p.json", "fields": [field]})
            assert response.status_code == 200, (field, response.text)
            assert list(response.json()["field_statistics"]) == [field]

        compare = http.post("/api/dataset/compare",
                            json={"dataset_a": "p.json", "dataset_b": "q.json"}).json()
        for field in compare["overlap_comparison"]["field_comparisons"]:
            response = http.post("/api/dataset/compare",
                                 json={"dataset_a": "p.json", "dataset_b": "q.json",
                                       "key_fields": [field]})
            assert response.status_code == 200, (field, response.text)

    def test_ghost_field_name_degrades_silently_which_is_why_the_ui_has_no_list(self,
                                                                                http_with_datasets):
        """`fields:["nope"]` 回 200 并给一整套全零（幽灵字段）⇒ 后端不出声，只能靠候选来源防"""
        http, _ = http_with_datasets
        body = http.post("/api/dataset/stats",
                         json={"input_file": "p.json", "fields": ["nope"]}).json()
        ghost = body["field_statistics"]["nope"]
        assert ghost["total_count"] == 0 or ghost["filled_count"] == 0
        assert ghost["fill_rate"] == 0.0 and ghost["unique_count"] == 0
        assert len(ghost) == 12, f"字段级统计的键集形状变了: {sorted(ghost)}"

    def test_ghost_key_field_inflates_similarity_which_is_the_real_hazard(self,
                                                                         http_with_datasets):
        """`key_fields:["nope"]` 把两份毫不相干的数据谎报成「完全重叠」

        实测 `p.json` vs `n.json`（4 条与 2 条，instruction 一句都不重）：默认键给
        `common_items: 0`、`similarity_score: 0.0`（真话），幽灵键给 `1` 与 `1.0`（谎话）。
        这一条比上一条更硬，因为它是**假阳性**：用户看到的是一份看起来很一致的结论，
        而不是一格零。
        """
        http, _ = http_with_datasets
        payload = {"dataset_a": "p.json", "dataset_b": "n.json"}
        honest = http.post("/api/dataset/compare", json=payload).json(
            )["overlap_comparison"]["metrics"]
        assert honest["common_items"] == 0 and honest["similarity_score"] == 0.0, honest
        ghost = http.post("/api/dataset/compare",
                          json={**payload, "key_fields": ["nope"]}).json(
                          )["overlap_comparison"]["metrics"]
        assert ghost["common_items"] == 1 and ghost["similarity_score"] == 1.0, ghost

    def test_key_fields_actually_move_the_overlap_metrics_so_the_knob_is_not_decoration(
            self, http_with_datasets):
        """同问不同答的一对：`instruction` ⇒ 交集 4 / 相似度 1.0，`output` ⇒ 0 / 0.0

        不发 `key_fields` 的那一跑必须与显式发 `["instruction"]` 的那一跑逐值相等：默认的
        权威住在 SDK（`compare_enhanced.py:126` 的 `key_fields or ["instruction"]`），
        不在请求模型里。
        """
        http, _ = http_with_datasets
        payload = {"dataset_a": "p.json", "dataset_b": "r.json"}
        a = http.post("/api/dataset/compare", json=payload).json(
            )["overlap_comparison"]["metrics"]
        b = http.post("/api/dataset/compare",
                      json={**payload, "key_fields": ["output"]}).json(
                      )["overlap_comparison"]["metrics"]
        assert a["common_items"] == 4 and a["similarity_score"] == 1.0, a
        assert b["common_items"] == 0 and b["similarity_score"] == 0.0, b
        explicit = http.post("/api/dataset/compare",
                             json={**payload, "key_fields": ["instruction"]}).json(
                             )["overlap_comparison"]["metrics"]
        assert explicit == a

    def test_key_fields_do_not_move_the_per_field_detail_table(self, http_with_datasets):
        """匹配键**只**动 `metrics`：`field_comparisons`（含逐条值差异）三跑逐字节不变

        订正记录：本文件初稿把这一档写成「明细跟着匹配键走」，读了 `comparison.py:217`
        与 `compare_enhanced.py:289` 才发现 `_compare_fields(items_a, items_b)` 压根不接
        `key_fields` 参数，配对永远是 `instruction`。页面据此把明细列头渲成条目键而不是
        键名（见 `Analysis.tsx` 字段对照卡）。
        """
        http, _ = http_with_datasets
        payload = {"dataset_a": "p.json", "dataset_b": "r.json"}
        faces = [http.post("/api/dataset/compare", json=req).json()["overlap_comparison"]
                 ["field_comparisons"]
                 for req in (payload, {**payload, "key_fields": ["output"]},
                             {**payload, "key_fields": ["nope"]})]
        assert faces[0] == faces[1] == faces[2]
        assert faces[0]["output"]["value_differences"], "这份夹具本该造出值差异"
        first = faces[0]["output"]["value_differences"][0]
        assert set(first) == {"instruction", "value_a", "value_b"}, first


@needs_web_src
class TestPresentationCoversProducedKeys:
    """页面上每张表的口径/量纲说明必须覆盖真实产出的键，且不许留不再产出的空话"""

    def test_quality_metric_calibers_match_the_produced_key_set(self, http_with_datasets):
        http, _ = http_with_datasets
        produced = set(http.post("/api/dataset/stats",
                                 json={"input_file": "p.json"}).json()["quality_metrics"])
        documented = set(labeled_calibers("STAT_QUALITY_PRESENTATION"))
        assert produced == documented, f"两侧键集不等: 只在产出 {produced - documented} / " \
                                      f"只在说明 {documented - produced}"

    def test_quality_metric_calibers_declare_that_fields_do_not_matter(self, http_with_datasets):
        """三条质量指标只遍历 instruction/output：口径栏必须写明「与 fields 无关」

        依据是实测：`fields:["output"]` 与缺省两跑的 `quality_metrics` 逐值相等。用户勾了
        字段再看质量分，最容易读成「按所选字段算的质量」，所以这句话是判据不是装饰。
        """
        http, _ = http_with_datasets
        whole = http.post("/api/dataset/stats", json={"input_file": "p.json"}).json()
        narrowed = http.post("/api/dataset/stats",
                             json={"input_file": "p.json", "fields": ["output"]}).json()
        assert whole["quality_metrics"] == narrowed["quality_metrics"], (
            whole["quality_metrics"], narrowed["quality_metrics"])
        for key, (_, caliber) in labeled_calibers("STAT_QUALITY_PRESENTATION").items():
            assert "fields" in caliber, (key, caliber)

    def test_overlap_metric_rows_match_the_produced_key_set(self, http_with_datasets):
        """重叠面九项：页面渲染清单与端点产出的键集必须双向相等

        `size_ratio` 也在其中——它在 B 侧为空时线上是 `null`，见
        `TestDeadAndNullableFaces`，所以清单不许因为它「有时不是数」就把它摘掉。
        """
        http, _ = http_with_datasets
        produced = set(http.post("/api/dataset/compare",
                                 json={"dataset_a": "p.json", "dataset_b": "q.json"}).json()
                       ["overlap_comparison"]["metrics"])
        documented = set(tuple_rows("OVERLAP_METRIC_PRESENTATION"))
        assert produced == documented, f"两侧键集不等: 只在产出 {produced - documented} / " \
                                      f"只在说明 {documented - produced}"
        assert len(produced) == 9, produced

    def test_overlap_calibers_split_counts_from_ratios(self):
        """计数、带符号差、0–1 比值三种量纲不许混排（`size_ratio` 的 80% 不是「80 条」）"""
        table = tuple_rows("OVERLAP_METRIC_PRESENTATION")
        assert table["size_a"][1] == "count" and table["size_diff"][1] == "signed"
        assert table["similarity_score"][1] == "ratio"
        assert all(label for label, _ in table.values())

    def test_side_face_labels_match_the_produced_key_sets(self, http_with_datasets):
        """去重面 / 长度面 / 词汇面的内层键必须逐项有中文标签，反向不许留空话"""
        http, _ = http_with_datasets
        quality = http.post("/api/dataset/compare",
                            json={"dataset_a": "p.json", "dataset_b": "q.json"}).json(
                            )["quality_comparison"]
        produced = set()
        for face in ("dedup_a", "length_a", "vocabulary_a"):
            produced |= set(quality[face])
        documented = set(flat_labels("SIDE_METRIC_LABELS"))
        assert produced == documented, f"两侧键集不等: 只在产出 {produced - documented} / " \
                                      f"只在说明 {documented - produced}"

    def test_quality_face_calibers_split_scores_from_rates(self):
        """`pass_rate` 是率、其余三个是分：混渲成 `1` 会被读成「1 条」"""
        table = labeled_calibers("QUALITY_FACE_PRESENTATION")
        assert set(table) == {"avg_score", "pass_rate", "min_score", "max_score"}
        assert all(label and caliber for label, caliber in table.values()), table
        assert kinds("QUALITY_FACE_PRESENTATION") == {
            "avg_score": "score", "pass_rate": "rate",
            "min_score": "score", "max_score": "score"}

    def test_feature_type_labels_cover_the_documented_buckets(self):
        """`_infer_type` 的五档字面量（`feature_detect.py:105` 的 docstring）逐项有中文名

        这一档后端**没有可 import 的常数**（返回值是散在函数里的字面量），所以权威退到
        「函数文档里写明的五档」；哪天后端加一档而页面没跟，回执里会出现原样英文值——
        页面用 `?? entry.feature_type` 兜底不丢数据，但这条守卫要红着提醒。
        """
        documented = flat_labels("FEATURE_TYPE_LABELS")
        assert set(documented) == {"numeric", "text", "boolean", "mixed", "unknown"}, documented


@needs_web_src
class TestDeadAndNullableFaces:
    """两个「写进契约但没人用」与「形状会变」的面，处置必须各有其形"""

    def test_similarity_to_faces_are_never_assigned(self):
        """**A287**：`similarity_to_a` / `similarity_to_b` 只有 dataclass 默认与序列化两处

        全仓（`comparison.py`）找不到对它们的赋值 ⇒ 恒为 `0.0`。这不是「这次的数据恰好 0」：
        两份不同数据的实测回执里它们也是 `0.0`。
        """
        source = COMPARISON_PY.read_text(encoding="utf-8")
        mentions = [line.strip() for line in source.splitlines()
                    if "similarity_to_a" in line or "similarity_to_b" in line]
        assert mentions, "comparison.py 里这两个键没了 ⇒ 本条的判据要跟着换形状"
        assert not ASSIGN_RE.search(source), f"出现了赋值点，死面活了: {mentions}"

    def test_the_page_does_not_render_the_dead_faces(self):
        """页面不渲恒 0 的死面：渲出来等于给用户一条永不移动的假读数"""
        page = _page()
        for dead in ("similarity_to_a", "similarity_to_b"):
            assert dead not in page, f"{dead} 被页面渲染了（它恒 0.0，见 A287）"

    def test_the_dead_faces_are_still_zero_on_two_different_datasets(self, http_with_datasets):
        """行为差取证：两份不同数据也照样 0.0（与上一条的源码扫描互相咬合）"""
        http, _ = http_with_datasets
        quality = http.post("/api/dataset/compare",
                            json={"dataset_a": "p.json", "dataset_b": "n.json"}).json(
                            )["quality_comparison"]
        assert quality["similarity_to_a"] == 0.0
        assert quality["similarity_to_b"] == 0.0
        assert quality["winner"] == "a", "两份数据的质量分本该有别"

    def test_size_ratio_arrives_as_null_when_b_side_is_empty(self, http_with_datasets):
        """**A288**：`float('inf')` 被序列化成 `null`，所以 TS 里它不能是纯 `number`"""
        http, _ = http_with_datasets
        metrics = http.post("/api/dataset/compare",
                           json={"dataset_a": "p.json", "dataset_b": "empty.json"}).json(
                           )["overlap_comparison"]["metrics"]
        assert metrics["size_b"] == 0
        assert metrics["size_ratio"] is None, metrics
        # 线上必须是严格 JSON（浏览器 JSON.parse 不接受 Infinity 字面量）
        raw = http.post("/api/dataset/compare",
                        json={"dataset_a": "p.json", "dataset_b": "empty.json"}).text
        assert "Infinity" not in raw and "NaN" not in raw

    def test_the_types_and_the_page_both_admit_the_null(self):
        """类型面标了 `number | null`、页面有接住 null 的分支：两头都要在"""
        types = TYPES_TS.read_text(encoding="utf-8")
        assert re.search(r"size_ratio:\s*number \| null", types), "TS 类型没接住 null"
        assert re.search(r"raw === null", _page()), "页面没有 null 分支"


@needs_web_src
class TestEndpointErrorShapes:
    """三只端点的失败形状：页面文案能不能真把 detail 端出来"""

    def test_missing_file_is_404_with_a_string_detail_for_all_three(self, http_with_datasets):
        http, _ = http_with_datasets
        for path, payload in (
            ("/api/dataset/stats", {"input_file": "missing.json"}),
            ("/api/dataset/features", {"input_file": "missing.json"}),
            ("/api/dataset/compare", {"dataset_a": "p.json", "dataset_b": "missing.json"}),
        ):
            response = http.post(path, json=payload)
            assert response.status_code == 404, (path, response.text)
            assert isinstance(response.json()["detail"], str), (path, response.json())

    def test_same_pair_is_a_tie_not_an_absent_winner(self, http_with_datasets):
        """`winner` 是三态：同一份文件自己比自己回 `tie` 且差值四项全 0.0"""
        http, _ = http_with_datasets
        quality = http.post("/api/dataset/compare",
                            json={"dataset_a": "p.json", "dataset_b": "p.json"}).json(
                            )["quality_comparison"]
        assert quality["winner"] == "tie"
        assert set(quality["quality_diff"].values()) == {0.0}, quality["quality_diff"]

    def test_the_page_labels_all_three_winner_states(self):
        """三态各有中文标签，未知值不编标签（`??` 兜底那条在页面里）"""
        table = keyed_labels("WINNER_PRESENTATION")
        assert set(table) == {"a", "b", "tie"}, table
        assert all(label for label in table.values())

    def test_empty_dataset_gives_three_empty_dicts_not_zeros(self, http_with_datasets):
        """空集 ⇒ 三个统计字典全是 `{}`（`content_statistics` 的两个子键根本不出现）

        这就是 TS 里 `text_length` / `vocabulary` 标可选的依据，也是页面「无内容而不补 0」
        那条分支的依据。
        """
        http, _ = http_with_datasets
        body = http.post("/api/dataset/stats", json={"input_file": "empty.json"}).json()
        assert body["total_items"] == 0
        assert body["field_statistics"] == {} and body["quality_metrics"] == {}
        assert body["content_statistics"] == {}
        types = TYPES_TS.read_text(encoding="utf-8")
        assert re.search(r"text_length\?:", types), "TS 没把子键标可选，空集回执会让它撒谎"

    def test_the_sparse_threshold_is_not_on_this_endpoint(self, http_with_datasets):
        """`min_coverage` 端点不可调：多送的键被静默忽略，所以页面不装旋钮"""
        http, _ = http_with_datasets
        plain = http.post("/api/dataset/features", json={"input_file": "p.json"}).json()
        with_extra = http.post("/api/dataset/features",
                               json={"input_file": "p.json", "min_coverage": 0.9}).json()
        assert plain == with_extra, "多送的键起了作用 ⇒ 这一档变成可调，页面要跟着加旋钮"
        assert set(plain) == {"total_items", "field_features", "sparse_fields",
                              "intent_distribution"}
