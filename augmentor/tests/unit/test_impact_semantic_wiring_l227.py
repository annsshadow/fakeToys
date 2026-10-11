# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L227：impact / semantic-search 接线之后，抄来的默认值与响应键集不许漂

**改前实测**（`git show HEAD:./web/src/services/api.ts` / 同 `pages/Analysis.tsx`，L227 取证）：

| 事实 | 改前 | 改后 |
| --- | --- | --- |
| 前端调用 `/dataset/impact` | **0 处**（在 l224 的 `NO_ENTRY` 三条里） | `assessImpact` + 分析页一张卡 |
| 前端调用 `/dataset/semantic-search` | **0 处**（同上） | `semanticSearch` + 分析页一张卡 |
| 页面抄写的四个默认值与请求模型有无对账 | **无** | 本文件 `TestWiredDefaults` 逐条钉 |
| `gains` 四项的中文量纲说明有无权威 | **无** | 本文件按**真实回执的键集**双向对账 |

**为什么要另建一把尺子而不是扩 l226**：l226 管的是「封闭清单」——`SEARCH_METHODS`、
`VALIDATION_PRESETS` 在 Python 侧有可 import 的常数，TSX 那份字面量能逐项比。这两条操作
**没有**这种清单：`text_field` / `text_key` 的合法取值来自数据集本身，SDK 里
`["instruction", "input", "output"]` 只是 `audit.py:68` 与 `privacy.py:76` 的两处手写字面量
（不是常数，引它等于自造一套权威）。所以本文件的权威换了三处来源：
① 默认值 → **请求模型的 `model_fields[...].default`**（页面把默认抄进 `useState`，抄的东西会漂）；
② 增益键集 → **真实回执里的 `gains` 键**（SDK 产出什么，页面就得说明什么，反向也不许留空话）；
③ 快照键集 → **`AugmentationMetrics` 的 dataclass 字段**（唯一豁免 `extra`，豁免理由必须写死）。
字段名那一档不做清单守卫，改做「页面发得出每一档都被端点接受」（`TestEndpointAcceptsWhatTheUiSends`）。

**探针量到的四条事实**（判据的依据，不是想象）：
* `query=""` ⇒ **200**、`total` 等于条目数、每条 `score` 全 `0.0` ⇒ 页面必须在门口拦空串，
  否则用户拿到一份「全都相关」的退化读数；
* `top_k=0` ⇒ **200**、`results=[]` ⇒ 合法的「只要回显、不要命中」，页面 `min` 不许设成 1；
* `text_key="nope"` ⇒ **200**、`text` 全空串（**静默退化**，不报错）⇒ 这一档只能靠
  「只放数据集里真有的字段」来防，不能指望后端出声；
* `text_field="nope"` ⇒ **400** 且 detail 是**字符串**、里面带着首条实际字段 ⇒ 前端
  `apiErrorDetail` 的主支不是想象出来的形状。

跑法：`python -m pytest tests/unit/test_impact_semantic_wiring_l227.py -q --no-cov`
"""

import dataclasses
import json
import pathlib
import re

import pytest
from tests.unit.test_wired_dropdown_choices_l226 import (
    _initial,
    needs_web_src,
    strip_comments,
)

from api.routes.dataset_tools import ImpactRequest
from api.routes.vector import SemanticSearchRequest
from augmentor.impact import AugmentationMetrics

SRC = pathlib.Path(__file__).resolve().parents[2] / "web" / "src"
ANALYSIS_TSX = SRC / "pages" / "Analysis.tsx"
API_TS = SRC / "services" / "api.ts"

# 一份「四字段齐」的数据集：`input` 也在，这样页面哪天放开 `input` 那一档，
# 守卫测的是「端点认不认」而不是「探针的数据凑巧缺这个键」。
ITEMS = [
    {"instruction": "请解释什么是向量数据库", "input": "", "output": "向量数据库按相似度检索高维向量。"},
    {"instruction": "写一首关于秋天的诗", "input": "主题：秋天", "output": "落叶铺满小径，秋天把颜色交给风。"},
    {"instruction": "什么是包含判断", "input": "", "output": "包含判断是子串匹配。"},
    {"instruction": "正则怎么用", "input": "", "output": "正则用元字符描述待匹配的形状。"},
]

GAIN_ENTRY_RE = re.compile(
    r"(\w+):\s*\{\s*label:\s*'([^']+)',\s*caliber:\s*'([^']+)'\s*\}")
SNAPSHOT_ROW_RE = re.compile(r"\['(\w+)',\s*'([^']+)',\s*'(count|decimal)'\]")
INLINE_OPTIONS_RE_TMPL = r"value=\{%s\}[\s\S]*?options=\{\[(.*?)\]\}"
VALUE_RE = re.compile(r"value:\s*'([^']+)'")


def _page() -> str:
    assert ANALYSIS_TSX.is_file(), "前端源码不在场 ⇒ 本文件的判据全部失效"
    return strip_comments(ANALYSIS_TSX.read_text(encoding="utf-8"))


def _num_initial(state_name: str) -> float:
    """读 `const [x, setX] = useState(5)` 里的那个**数字**初值

    l226 的 `_initial` 只认串字面量（那边两个下拉的初值都是串）。这里 `top_k` 与
    `min_scale_gain` 是数，所以补一条同形状的正则，而不是把那边的工具改宽。
    """
    declared = re.search(
        rf"const \[{state_name}, set{state_name[0].upper()}{state_name[1:]}\] = "
        rf"useState\(([-\d.]+)\)",
        _page())
    assert declared, f"{state_name} 的声明形状变了 ⇒ 本条要跟着换形状"
    return float(declared.group(1))


def gains_presentation() -> dict:
    """`GAIN_PRESENTATION` 里声明的 `{键: (中文标签, 量纲)}`"""
    block = re.search(r"const GAIN_PRESENTATION[^=]*= \{(.*?)\n\}", _page(), re.S)
    assert block, "Analysis 里没有 GAIN_PRESENTATION 常量 ⇒ 判据要跟着换形状"
    return {key: (label, caliber)
            for key, label, caliber in GAIN_ENTRY_RE.findall(block.group(1))}


def snapshot_presentation() -> dict:
    """`SNAPSHOT_PRESENTATION` 里声明的 `{键: (标签, 量纲)}`"""
    block = re.search(r"const SNAPSHOT_PRESENTATION[^=]*= \[(.*?)\n\]", _page(), re.S)
    assert block, "Analysis 里没有 SNAPSHOT_PRESENTATION 常量 ⇒ 判据要跟着换形状"
    return {key: (label, kind) for key, label, kind in SNAPSHOT_ROW_RE.findall(block.group(1))}


def inline_options(state_name: str) -> list:
    """`<Select value={x} … options={[…]} />` 里那份**内联**清单的 value 列表

    锚点必须写死成这个 state 名（实测教训）：通配版 `value={(\\w+)}` 会被同一张卡里排在
    前面的 `value={semanticQuery}` 抢走 —— 那一档往后非贪婪匹配到最近的 `options={[`，
    正好把 `value={semanticTextKey}` 整段吞进前一次命中，于是真正的目标永远扫不到。
    """
    pattern = re.compile(INLINE_OPTIONS_RE_TMPL % re.escape(state_name), re.S)
    blocks = pattern.findall(_page())
    assert blocks, f"页面里没有 value={{{state_name}}} 的内联 options ⇒ 判据要跟着换形状"
    values = [v for block in blocks for v in VALUE_RE.findall(block)]
    assert values, f"{state_name} 的 options 里一个 value 都没有: {blocks}"
    return values


@pytest.fixture
def http_with_dataset(tmp_path, monkeypatch):
    """白名单数据根 + `p.json`（4 条）与 `q.json`（5 条，含一条重复）+ 空集（与 L92 同形）"""
    root = tmp_path / "ds"
    root.mkdir()
    (root / "p.json").write_text(json.dumps(ITEMS, ensure_ascii=False), encoding="utf-8")
    (root / "q.json").write_text(
        json.dumps(ITEMS + [dict(ITEMS[0])], ensure_ascii=False), encoding="utf-8")
    (root / "empty.json").write_text("[]", encoding="utf-8")
    monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", str(root))
    monkeypatch.chdir(tmp_path)
    from fastapi.testclient import TestClient
    from api.main import app

    return TestClient(app), root


@needs_web_src
class TestWiredDefaults:
    """页面抄进 `useState` 的四个默认值必须仍等于请求模型里的默认值"""

    def test_scans_are_not_vacuous(self):
        """非空转元断言：四条扫描都真的读到了东西（读不到时上面的正则会先 assert 炸）"""
        assert _initial(ANALYSIS_TSX, "semanticTextKey") == "instruction"
        assert _initial(ANALYSIS_TSX, "impactField") == "instruction"
        assert _num_initial("semanticTopK") == 5
        assert _num_initial("minScaleGain") == 0

    def test_semantic_text_key_default_matches_the_contract(self):
        assert _initial(ANALYSIS_TSX, "semanticTextKey") == \
            SemanticSearchRequest.model_fields["text_key"].default

    def test_top_k_default_matches_the_contract(self):
        assert _num_initial("semanticTopK") == \
            float(SemanticSearchRequest.model_fields["top_k"].default)

    def test_impact_text_field_default_matches_the_contract(self):
        assert _initial(ANALYSIS_TSX, "impactField") == \
            ImpactRequest.model_fields["text_field"].default

    def test_min_scale_gain_default_matches_the_contract(self):
        """门槛漂了等于**默认判决**跟着漂：页面上绿标签的含义与 SDK 口径就不再是一回事"""
        assert _num_initial("minScaleGain") == \
            float(ImpactRequest.model_fields["min_scale_gain"].default)

    def test_the_service_layer_writes_no_second_default(self):
        """服务层只转实参：默认值的权威必须只有一处（页面 useState 与请求模型各一套就是三套）"""
        src = strip_comments(API_TS.read_text(encoding="utf-8"))
        for name in ("assessImpact", "semanticSearch"):
            _, _, body = src.partition(f"export const {name}")
            assert body, f"api.ts 里 {name} 没了"
            head = body[:600]
            assert "= '" not in head and "??" not in head, \
                f"{name} 里出现了第二套默认值"


@needs_web_src
class TestGainAndSnapshotPresentation:
    """页面上的量纲说明必须覆盖 SDK 实际产出的键，且不许留已不产出的空话"""

    def test_every_produced_gain_has_a_documented_caliber(self, http_with_dataset):
        """真实回执里的每一项增益都要在 `GAIN_PRESENTATION` 里有中文量纲

        权威是**响应本身**（`evaluate()` 里没有可 import 的键名常数）。后端加一项而页面
        不跟着加说明时，页面会把它渲染成「未知口径」——不丢数据（vitest 那边钉着），
        但这条守卫要红，提醒那句「未知」是暂时状态而不是终态。
        """
        http, _ = http_with_dataset
        response = http.post("/api/dataset/impact",
                            json={"before_file": "p.json", "after_file": "q.json"})
        assert response.status_code == 200, response.text
        produced = set(response.json()["gains"])
        documented = set(gains_presentation())
        missing = produced - documented
        assert not missing, f"这些增益项在页面上没有量纲说明: {sorted(missing)}"

    def test_no_documented_gain_that_the_sdk_no_longer_produces(self, http_with_dataset):
        """反方向：页面里躺着一条后端不再产出的说明，等于给读者讲一件不存在的事"""
        http, _ = http_with_dataset
        response = http.post("/api/dataset/impact",
                            json={"before_file": "p.json", "after_file": "q.json"})
        produced = set(response.json()["gains"])
        stale = set(gains_presentation()) - produced
        assert not stale, f"这些量纲说明对应的增益项已不再被产出: {sorted(stale)}"

    def test_gain_calibers_say_which_ones_are_ratios(self):
        """前两项是比值、后两项是绝对差（`dedup_gain` 为正=重复率**下降**）：量纲栏不许空"""
        table = gains_presentation()
        assert len(table) >= 4
        assert all(label and caliber for label, caliber in table.values()), table
        assert "绝对差" in table["dedup_gain"][1], table["dedup_gain"]

    def test_snapshot_rows_cover_the_metrics_fields_except_extra(self):
        """`AugmentationMetrics` 的字段除 `extra` 外必须逐项在渲染清单里

        `extra` 是 SDK 留的开放扩展位，页面给它编一个中文标签就是凭空造字段；豁免只有
        这一项，所以哪天 SDK 多一个真字段而页面没加行，这条会当场红。
        """
        fields = {f.name for f in dataclasses.fields(AugmentationMetrics)}
        rows = set(snapshot_presentation())
        assert fields - rows == {"extra"}, f"渲染清单与字段表差在这里: {fields ^ rows}"
        assert not rows - fields, f"渲染清单里有 SDK 不产出的键: {sorted(rows - fields)}"

    def test_the_misnamed_unique_instructions_row_is_labelled_honestly(self):
        """`unique_instructions` 数的其实是「所选字段的唯一值」，标签不许跟着键名说谎"""
        label = snapshot_presentation()["unique_instructions"][0]
        assert "字段" in label, f"标签没说明它是按所选字段数的: {label}"
        assert "指令" not in label, f"选了 output 时它数的是唯一回答，标签不该写死指令: {label}"

    def test_counts_are_not_rendered_with_fake_precision(self):
        """条目数/唯一值数是计数，量纲栏必须是 `count`；长度与比率才是四位小数"""
        table = snapshot_presentation()
        assert table["total_items"][1] == "count"
        assert table["unique_instructions"][1] == "count"
        assert table["duplicate_rate"][1] == "decimal"


@needs_web_src
class TestEndpointAcceptsWhatTheUiSends:
    """页面发得出的每一档，在真实中间件链下的行为与文案形状"""

    @pytest.mark.parametrize("state_name", ["impactField", "semanticTextKey"])
    def test_every_offered_field_name_is_accepted(self, state_name, http_with_dataset):
        """下拉里选得到的字段名必须被端点接受（400 的选项等于骗人）"""
        http, _ = http_with_dataset
        offered = inline_options(state_name)
        assert offered
        for field in offered:
            if state_name == "impactField":
                response = http.post("/api/dataset/impact",
                                     json={"before_file": "p.json", "after_file": "q.json",
                                           "text_field": field})
            else:
                response = http.post("/api/dataset/semantic-search",
                                     json={"input_file": "p.json", "query": "向量",
                                           "text_key": field})
            assert response.status_code == 200, (field, response.text)

    def test_semantic_text_key_actually_changes_what_is_returned(self, http_with_dataset):
        """`text_key` 不是装饰：换成 output 时回显的 `text` 就得是那条的 output 值"""
        http, _ = http_with_dataset
        response = http.post("/api/dataset/semantic-search",
                             json={"input_file": "p.json", "query": "向量", "top_k": 2,
                                   "text_key": "output"})
        results = response.json()["results"]
        assert results, response.text
        for hit in results:
            assert hit["text"] == ITEMS[hit["index"]]["output"], hit

    def test_empty_query_is_servable_which_is_why_the_door_has_a_gate(self, http_with_dataset):
        """门口那条 `请输入查询文本` 的闸是**承重**的：后端对空串回 200 且 score 全 0

        如果哪天后端改成 422/400，这条会红 —— 那时页面门口的闸可以撤，判决交给后端。
        """
        http, _ = http_with_dataset
        response = http.post("/api/dataset/semantic-search",
                             json={"input_file": "p.json", "query": ""})
        assert response.status_code == 200, response.text
        body = response.json()
        assert body["total"] == len(ITEMS) == len(body["results"])
        assert all(hit["score"] == 0.0 for hit in body["results"]), body["results"]

    def test_top_k_zero_is_the_echo_only_mode_the_ui_allows(self, http_with_dataset):
        """`top_k=0` 合法（200、空 results、三个生效值照旧回显）⇒ 页面 `min` 不许设成 1"""
        http, _ = http_with_dataset
        response = http.post("/api/dataset/semantic-search",
                             json={"input_file": "p.json", "query": "向量", "top_k": 0})
        assert response.status_code == 200, response.text
        body = response.json()
        assert body["total"] == 0 and body["results"] == []
        assert body["backend"] and body["dimension"] and body["collection"]

    def test_min_scale_gain_moves_the_verdict_on_an_identical_pair(self, http_with_dataset):
        """同一对文件（零增益）在两个门槛下判决必须不同：旋钮连着判据，不是摆设

        实测：两侧同文件 ⇒ 四项增益全 0.0，`beneficial=true`（门槛 0.0）；门槛抬到 0.5 ⇒
        `beneficial=false`。这条同时钉住「两侧同文件」是合法用法（页面注释里写的那档）。
        """
        http, _ = http_with_dataset
        payload = {"before_file": "p.json", "after_file": "p.json"}
        loose = http.post("/api/dataset/impact", json=payload)
        strict = http.post("/api/dataset/impact",
                           json={**payload, "min_scale_gain": 0.5})
        assert loose.status_code == strict.status_code == 200
        assert set(loose.json()["gains"].values()) == {0.0}
        assert loose.json()["beneficial"] is True
        assert strict.json()["beneficial"] is False

    def test_a_bad_text_field_detail_is_a_string_the_ui_can_show(self, http_with_dataset):
        """字段名拼错时是 400 + **字符串** detail，且文案带着可行动的线索"""
        http, _ = http_with_dataset
        response = http.post("/api/dataset/impact",
                             json={"before_file": "p.json", "after_file": "q.json",
                                   "text_field": "nope"})
        assert response.status_code == 400, response.text
        detail = response.json()["detail"]
        assert isinstance(detail, str), type(detail)
        assert "nope" in detail and "instruction" in detail

    def test_empty_baseline_is_400_not_a_bunch_of_zeros(self, http_with_dataset):
        """空基线的四项增益分母为 0：报 0.0 会被读成「毫无效果」，所以后端出声、页面把判决端出来"""
        http, _ = http_with_dataset
        response = http.post("/api/dataset/impact",
                             json={"before_file": "empty.json", "after_file": "p.json"})
        assert response.status_code == 400, response.text
        detail = response.json()["detail"]
        assert isinstance(detail, str) and "分母" in detail
