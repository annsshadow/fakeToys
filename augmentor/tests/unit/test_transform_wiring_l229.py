# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L229：convert / merge / split 三条写盘变换接进数据管理页之后，清单、默认、判决与回执都不许漂

**改前实测**（`git show HEAD:./web/src/services/api.ts` / 同 `pages/DataManagement.tsx`，L229 取证）：

| 事实 | 改前 | 改后 |
| --- | --- | --- |
| 前端调用 `/api/dataset/convert` | **0 处**（在 l224 的 `UNCONSUMED` 里） | `convertDataset` + 一张「格式转换」卡 |
| 前端调用 `/api/dataset/merge` | **0 处**（同上） | `mergeDatasets` + 一张「合并数据集」卡 |
| 前端调用 `/api/dataset/split` | **0 处**（同上） | `splitDataset` + 一张「训练 / 验证 / 测试分割」卡 |
| 页面上有没有第二份目标格式清单 | 没有（此前页面只有上传那张 13 项源格式表） | 新增 12 项写边表，**逐项等于** `converter.OUTPUT_FORMAT_CHOICES` |

**这一批的判据为什么是「清单 + 默认 + 判决 + 回执」四档**：三条都是**写盘**端点，成功面只回
「写到哪、写了多少」，能力全在**旋钮发得对不对**与**拒绝说没说清**。探针（仓库外，跑完即删）
在真实中间件链上量到的形状，逐条是下面这些判据的依据：

* **两张格式表差一格，而且差的是最危险的那一格**：`INPUT_FORMAT_CHOICES`（13，那一行是
  `augmentor/converter.py:91`）与 `OUTPUT_FORMAT_CHOICES`（12，同文件 `:99`）只差一个 `xls`。
  读边认 `.xls`（openpyxl 读得了老 BIFF），写边**刻意不认**——写 `.xls` 会得到「扩展名是 xls、
  字节是 zip」的假容器。端点对 `target_format="xls"` 回 400，且 detail 里**逐条列出那十二个
  合法目标**，所以本文件不比两份字面清单了事，而是把端点自己列出的那份抓回来与页面逐项相等。
* **十二档全落得下去**：`json / jsonl / csv / tsv / alpaca / sharegpt / chatml / llama_factory /
  vicuna / belle` 各配 `.json` 或它本来的扩展名、`excel` 与 `xlsx` 配 `.xlsx`，实测**全 200**、
  `output_count` 恒 6。回执 echo 的是**生效的**内部格式名：请求 `xlsx` 回的是 `excel` ⇒
  页面照抄回执，不照抄自己的选择。
* **空串不是「自动」**：`source_format: ""` 实测 **400**「无法从  转换到 JSON」（两个空格，格式名是空的）。
  请求模型那一格是 `Optional[str] = None`，`""` 合法通过校验却掉进转换器。页面「自动」那档存的
  是空串，服务层必须把它丢成 `undefined`（线上不留这一格）——这不是审美，是**发出去就 400**
  的硬接线。
* **名字与格式不匹配是判决而不是假容器**：`target_format=excel` 配 `bad.json` ⇒ 400，detail 把两种
  处置都点名（改名为 `.xlsx`，或换目标格式）。
* **两条支路的判据不是同一套（A289）**：默认支容差 0.01、分层支 1e-6；负数只有分层支拦（默认支
  `-0.8/0.1/1.7` 实测 **200**，三段变成 `0/0/6`）。两句拒绝的文案也不同：「分割比例之和必须为1.0，
  当前为 0.8999999999999999」对「比例之和必须为 1.0，实际为 1.0050」。
* **幽灵分层键静默**：`stratify_key="nope"` ⇒ **200**，全部条目塌成一个分组，三段数与用默认键时
  **逐格相同**（5/1/0）。这一档不写清就会被读成「分层生效了」，所以页面把那句印在卡上。
* **合并的四档只接了一档（A290）**：`MergeConfig` 有 `deduplicate` / `dedup_threshold` /
  `preserve_order` / `max_items` 四个旋钮，端点源码里只出现 `deduplicate`；SDK 的 `merge_files`
  返回六个键，`MergeResponse` 只暴露五个——`truncated_by_max_items`（与去重删除**分开报**的那一格）
  在线上被丢掉。页面因此只装「开 / 关」一个闸，并把「阈值在这只端点上不可调」写在卡上。
* **三段键恒在**：`1.0/0/0` 实测回 `6/0/0` 且 `val` / `test` 两段照旧出现（`count: 0` 而不是缺键）。

**一条被探针推翻的想象**（写进来看住它别再回来）：本文件初稿打算钉「绝对路径 ⇒ 403」，探针第一次
跑的是 `<白名单根>/o6.json` 这种**在白名单内**的绝对路径——实测 **200**。403 只在路径越出白名单根时
出现（三只端点一致：「路径超出允许的数据目录范围」）。所以判据写成两支：越界绝对路径 ⇒ 403，
名字里带 `..` ⇒ 400「路径包含非法组件」。

跑法：`python -m pytest tests/unit/test_transform_wiring_l229.py -q --no-cov`
"""

import json
import pathlib
import re

import pytest
from tests.unit.test_wired_dropdown_choices_l226 import (
    needs_web_src,
    strip_comments,
)

from api.routes.dataset_tools import (
    ConvertRequest,
    ConvertResponse,
    MergeRequest,
    MergeResponse,
    SplitPart,
    SplitRequest,
    SplitResponse,
)
from augmentor.converter import INPUT_FORMAT_CHOICES, OUTPUT_FORMAT_CHOICES
from augmentor.dataset_ops import MergeConfig

SRC = pathlib.Path(__file__).resolve().parents[2] / "web" / "src"
PAGE_TSX = SRC / "pages" / "DataManagement.tsx"
API_TS = SRC / "services" / "api.ts"
TYPES_TS = SRC / "types" / "api.ts"
DATASET_OPS_PY = pathlib.Path(__file__).resolve().parents[2] / "augmentor" / "dataset_ops.py"

ITEMS = [{"instruction": f"问{i}", "input": "", "output": f"答{i}"} for i in range(6)]
#: q.json 是 p.json 的前四条：两份合并 ⇒ 10 条进；去重 ⇒ 6 条出、删 4 条，不去重 ⇒ 10 条出、删 0 条。
#: 这一对是「去重开关真的动产物」的取证夹具，用两份完全相同的数据做不到（两档删得一样多）。
SUBSET = ITEMS[:4]

#: 探针量到的「这一档目标该配什么扩展名」表；不在表里的目标都配 `.json`（alpaca 等容器格式落盘就是 JSON）
NATURAL_SUFFIX = {"json": ".json", "jsonl": ".jsonl", "csv": ".csv", "tsv": ".tsv",
                  "excel": ".xlsx", "xlsx": ".xlsx"}

STATE_RE = re.compile(r"const \[(\w+), set\w+\] = useState(?:<[^>]*>)?\((.*?)\)")
ENUM_RE = re.compile(r"全部可选目标: ([^）)]+)")


def _page() -> str:
    assert PAGE_TSX.is_file(), "前端源码不在场 ⇒ 本文件的判据全部失效"
    return strip_comments(PAGE_TSX.read_text(encoding="utf-8"))


def ts_list(name: str) -> list:
    """页面那份 `const NAME = ['a', 'b']` ⇒ 逐项保真的清单（顺序也保真）"""
    found = re.search(rf"const {re.escape(name)} = \[(.*?)\]", _page(), re.S)
    assert found, f"DataManagement 里没有 {name} 常量 ⇒ 判据要跟着换形状"
    return re.findall(r"'([^']+)'", found.group(1))


def state_initial(name: str) -> str:
    """`const [name, setName] = useState(...)` 括号里的**字面原文**（不求值，按字面比形状）"""
    pairs = dict(STATE_RE.findall(_page()))
    assert name in pairs, f"页面里没有 {name} 这个 state ⇒ 接线被拆了"
    return pairs[name].strip()


def ts_interface_keys(name: str) -> list:
    """`export interface NAME {…}` 的字段名（TS 侧的响应面）"""
    src = strip_comments(TYPES_TS.read_text(encoding="utf-8"))
    found = re.search(rf"export interface {re.escape(name)} \{{(.*?)\n\}}", src, re.S)
    assert found, f"types/api.ts 里没有 {name} ⇒ 判据要跟着换形状"
    return re.findall(r"^\s{2}(\w+)[?]?:", found.group(1), re.M)


def fetcher_body(name: str) -> str:
    """`api.ts` 里那条 fetcher 的函数体（到下一条 `export const` 为止，不越界到邻居）"""
    src = strip_comments(API_TS.read_text(encoding="utf-8"))
    _, _, tail = src.partition(f"export const {name}")
    assert tail, f"api.ts 里 {name} 没了"
    return tail.split("export const", 1)[0]


def select_block(aria_label: str) -> str:
    """那个 `aria-label="…"` 的原生 `<select …>…</select>` 片段（候选清单就在它的 option 里）"""
    page = _page()
    anchor = f'aria-label="{aria_label}"'
    assert anchor in page, f"页面里没有 aria-label 为 {aria_label} 的控件 ⇒ 判据要跟着换形状"
    start = page.index(anchor)
    return page[start:page.index("</select>", start)]


@pytest.fixture
def http_with_transforms(tmp_path, monkeypatch):
    """白名单数据根：`p.json`(6 条) 与 `q.json`(前 4 条)；根**外**另有一个 `outside/` 目录"""
    root = tmp_path / "ds"
    root.mkdir()
    outside = tmp_path / "outside"
    outside.mkdir()
    (root / "p.json").write_text(json.dumps(ITEMS, ensure_ascii=False), encoding="utf-8")
    (root / "q.json").write_text(json.dumps(SUBSET, ensure_ascii=False), encoding="utf-8")
    monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", str(root))
    monkeypatch.chdir(tmp_path)
    from fastapi.testclient import TestClient
    from api.main import app

    return TestClient(app), root, outside


@needs_web_src
class TestTheTwoFormatListsAreTheAuthorities:
    """两张下拉的清单必须**就是** converter 的两份常数，差的那一格只能是 xls"""

    def test_target_list_equals_output_format_choices(self):
        """写边那张表逐项等于 `OUTPUT_FORMAT_CHOICES`（含顺序）"""
        assert ts_list("CONVERT_TARGET_FORMATS") == list(OUTPUT_FORMAT_CHOICES)

    def test_source_list_reuses_the_upload_authority_and_not_a_second_literal(self):
        """转换卡的源格式**复用** `UPLOAD_FORMAT_CHOICES`，页面里不许出现第二份 13 项字面量

        这一条守的是 A77「一份权威只住一处」：源格式清单已经有 l92 的守卫双向钉着，接线时
        顺手复制一份，将来加一个源格式就会只改一处、另一处静默过期。
        """
        assert _page().count("const UPLOAD_FORMAT_CHOICES") == 1
        block = select_block("转换源格式")
        assert "UPLOAD_FORMAT_CHOICES.map" in block, block
        assert "'alpaca'" not in block

    def test_the_two_lists_differ_by_exactly_xls(self):
        """读边 = 写边 + `xls`，而且只多这一个（多一个就少拒一个假容器）"""
        assert list(INPUT_FORMAT_CHOICES) == list(OUTPUT_FORMAT_CHOICES) + ["xls"]
        assert ts_list("UPLOAD_FORMAT_CHOICES") == list(INPUT_FORMAT_CHOICES)

    def test_xls_is_rejected_by_name_and_the_verdict_enumerates_the_twelve(self,
                                                                           http_with_transforms):
        """页面发得出的十二档全被端点认；`xls` 那一档端点点名拒收并把十二档列全

        detail 里那句「全部可选目标: …」是端点**自己**报的合法清单。把它抓回来与页面那份逐项
        相等，比在测试里再抄一份名单强：加一个格式时两端必须一起动，否则这一支红。
        """
        http, _root, _outside = http_with_transforms
        resp = http.post("/api/dataset/convert", json={
            "input_file": "p.json", "output_file": "a.xlsx", "target_format": "xls"})
        assert resp.status_code == 400
        detail = resp.json()["detail"]
        assert "xls 是老 BIFF 格式，写不出来" in detail
        enumerated = [n.strip() for n in ENUM_RE.search(detail).group(1).split(",")]
        assert enumerated == ts_list("CONVERT_TARGET_FORMATS")

    @pytest.mark.parametrize("target", list(OUTPUT_FORMAT_CHOICES))
    def test_every_target_the_page_offers_is_accepted_by_the_endpoint(self, http_with_transforms,
                                                                     target):
        """UI 能选到的每一档目标都真的落得下去，且回执报的是生效的内部格式名"""
        http, _root, _outside = http_with_transforms
        suffix = NATURAL_SUFFIX.get(target, ".json")
        resp = http.post("/api/dataset/convert", json={
            "input_file": "p.json", "output_file": f"conv_{target}{suffix}",
            "target_format": target})
        assert resp.status_code == 200, resp.json()
        assert resp.json()["target_format"] == ("excel" if target in ("excel", "xlsx") else target)
        assert resp.json()["output_count"] == 6

    def test_mismatched_output_name_is_a_verdict_not_a_mislabeled_container(
            self, http_with_transforms):
        """`target_format=excel` 配 `.json` 名字 ⇒ 400，两种处置都被点名（不落假容器）"""
        http, _root, _outside = http_with_transforms
        detail = http.post("/api/dataset/convert", json={
            "input_file": "p.json", "output_file": "bad.json",
            "target_format": "excel"}).json()["detail"]
        assert "扩展名不是 .xlsx" in detail
        assert "把输出名改成 .xlsx，或换目标格式" in detail


@needs_web_src
class TestWiredDefaultsMatchTheContract:
    """页面初值逐项等于请求模型的默认；必填那几格页面**不给**默认"""

    #: `(页面 state 名, 模型类, 契约字段名)`，按 state 名排序给出，id 用 state 名
    ARMS = sorted([
        ("convertTarget", ConvertRequest, "target_format"),
        ("mergeDedup", MergeRequest, "deduplicate"),
        ("trainRatio", SplitRequest, "train_ratio"),
        ("valRatio", SplitRequest, "val_ratio"),
        ("testRatio", SplitRequest, "test_ratio"),
        ("splitStratify", SplitRequest, "stratify"),
        ("stratifyKey", SplitRequest, "stratify_key"),
        ("splitShuffle", SplitRequest, "shuffle"),
    ])

    @pytest.mark.parametrize("state_name,model,field", ARMS,
                             ids=[arm[0] for arm in ARMS])
    def test_the_initial_value_is_the_contract_default(self, state_name, model, field):
        """初值与模型默认**等值**：页面自带一套默认就是第二权威"""
        literal = state_initial(state_name)
        expected = model.model_fields[field].default
        actual = {"true": True, "false": False, "null": None}.get(literal)
        if actual is None:
            actual = float(literal) if literal.replace(".", "", 1).isdigit() \
                else literal.strip("'\"")
        assert actual == expected, f"{state_name} 初值 {literal!r} 对不上默认 {expected!r}"

    def test_seed_and_source_start_empty_because_their_default_is_none(self):
        """`seed=None` 与 `source_format=None` 在页面是 `null` / 空串：留空＝不发那一格"""
        assert state_initial("splitSeed") == "null"
        assert SplitRequest.model_fields["seed"].default is None
        assert state_initial("convertSource") == "''"
        assert ConvertRequest.model_fields["source_format"].default is None

    def test_required_targets_have_no_default_and_the_page_starts_empty(self):
        """`output_file` / `output_dir` / `inputs` 是必填（模型里根本没有默认），页面也留空"""
        assert ConvertRequest.model_fields["output_file"].is_required()
        assert MergeRequest.model_fields["output_file"].is_required()
        assert SplitRequest.model_fields["output_dir"].is_required()
        for name in ("convertOut", "mergeOut", "splitOutDir"):
            assert state_initial(name) == "''", f"{name} 不该有默认值"
        assert state_initial("mergeInputs") == "[]"

    def test_the_service_layer_writes_no_second_default(self):
        """三条 fetcher 只转实参：默认值的权威必须只有一处（页面、服务层、请求模型各一套就是三套）"""
        for name in ("convertDataset", "mergeDatasets", "splitDataset"):
            body = fetcher_body(name)
            assert "= '" not in body and "??" not in body, f"{name} 里出现了第二套默认值"

    def test_the_empty_source_format_sentinel_is_dropped_in_the_service_layer(self):
        """`source_format: opts.sourceFormat || undefined` 是**唯一**一处默认表达式，而且是必需的

        页面「自动」那档存的是空串，端点收到空串会 400（见下一条），所以服务层必须把空串换成
        `undefined`（序列化后这一格整个不出现）。这里认 `|| undefined` 而不是「没有 `||`」：
        那一处是被量出来的必需接线，不是漂移的第二套默认。
        """
        body = fetcher_body("convertDataset")
        assert body.count("|| undefined") == 1
        assert "source_format: opts.sourceFormat || undefined" in body

    def test_empty_string_source_format_is_not_auto(self, http_with_transforms):
        """实测：`source_format: ""` ⇒ 400，判决里那句「无法从  转换到 …」带一个空格式名

        这一支钉的是「省略」与「显式声明」在 API 面上必须是两种形状：把空串当「自动」发出去，
        用户看到的是一句缺主语的判决（那句文案本身归 **A291**）。
        """
        http, _root, _outside = http_with_transforms
        resp = http.post("/api/dataset/convert", json={
            "input_file": "p.json", "output_file": "e.jsonl", "source_format": ""})
        assert resp.status_code == 400
        assert "无法从  转换到" in resp.json()["detail"]
        assert http.post("/api/dataset/convert", json={
            "input_file": "p.json", "output_file": "e2.jsonl"}).status_code == 200


@needs_web_src
class TestEveryArmTheUiCanSend:
    """页面每个闸发得出的形状：要么被接受并改变产物，要么回一句点名原因的判决"""

    def test_dedup_switch_actually_moves_the_product(self, http_with_transforms):
        """去重开 ⇒ 10 进 6 出删 4；关 ⇒ 10 进 10 出删 0（页面发的是显式 false，不是省略）"""
        http, _root, _outside = http_with_transforms
        on = http.post("/api/dataset/merge", json={
            "inputs": ["p.json", "q.json"], "output_file": "m_on.json",
            "deduplicate": True}).json()
        off = http.post("/api/dataset/merge", json={
            "inputs": ["p.json", "q.json"], "output_file": "m_off.json",
            "deduplicate": False}).json()
        assert (on["input_files"], on["total_input"], on["total_output"],
                on["removed_duplicates"]) == (2, 10, 6, 4)
        assert (off["total_input"], off["total_output"], off["removed_duplicates"]) == (10, 10, 0)

    def test_empty_inputs_is_a_verdict_not_an_empty_file(self, http_with_transforms):
        """一个都没勾 ⇒ 400「inputs 不能为空」（页面的必填闸不是摆设，端点也拦）"""
        http, _root, _outside = http_with_transforms
        resp = http.post("/api/dataset/merge", json={"inputs": [], "output_file": "m.json"})
        assert resp.status_code == 400
        assert resp.json()["detail"] == "inputs 不能为空"

    def test_ratio_tolerance_and_negative_guard_differ_between_the_two_branches(
            self, http_with_transforms):
        """A289 的实测两面：+0.005 默认支过、分层支拒；负数默认支过、分层支拒

        两句拒绝的文案不同，所以判据不能写成「400 就满意」，要逐支点名落在哪一条支路上。
        """
        http, _root, _outside = http_with_transforms
        skewed = {"input_file": "p.json", "train_ratio": 0.805,
                  "val_ratio": 0.1, "test_ratio": 0.1}
        assert http.post("/api/dataset/split", json={
            **skewed, "output_dir": "d1"}).status_code == 200
        stratified = http.post("/api/dataset/split", json={
            **skewed, "output_dir": "d2", "stratify": True, "stratify_key": "instruction"})
        assert stratified.status_code == 400
        assert "实际为 1.0050" in stratified.json()["detail"]

        negative = {"input_file": "p.json", "train_ratio": -0.8,
                    "val_ratio": 0.1, "test_ratio": 1.7}
        loose = http.post("/api/dataset/split", json={**negative, "output_dir": "d3"})
        assert loose.status_code == 200
        assert [loose.json()["splits"][k]["count"]
                for k in ("train", "val", "test")] == [0, 0, 6]
        strict = http.post("/api/dataset/split", json={
            **negative, "output_dir": "d4", "stratify": True, "stratify_key": "instruction"})
        assert strict.status_code == 400
        assert strict.json()["detail"] == "比例不能为负数"

    def test_the_sum_verdict_carries_the_float_artifact_of_the_actual_sum(
            self, http_with_transforms):
        """`0.7/0.1/0.1` 的判决把实数原样报出来（0.8999999999999999）——页面不许替它四舍五入"""
        http, _root, _outside = http_with_transforms
        detail = http.post("/api/dataset/split", json={
            "input_file": "p.json", "output_dir": "d5",
            "train_ratio": 0.7, "val_ratio": 0.1, "test_ratio": 0.1}).json()["detail"]
        assert detail == "分割比例之和必须为1.0，当前为 0.8999999999999999"

    def test_a_ghost_stratify_key_silently_collapses_everything_into_one_group(
            self, http_with_transforms):
        """分层字段给一个不存在的名字 ⇒ 200，且三段数与默认键逐格相同 ⇒ 页面必须把这句说出口"""
        http, _root, _outside = http_with_transforms
        ghost = http.post("/api/dataset/split", json={
            "input_file": "p.json", "output_dir": "g1", "stratify": True,
            "stratify_key": "nope"}).json()["splits"]
        real = http.post("/api/dataset/split", json={
            "input_file": "p.json", "output_dir": "g2", "stratify": True,
            "stratify_key": "instruction"}).json()["splits"]
        assert {k: v["count"] for k, v in ghost.items()} == \
            {k: v["count"] for k, v in real.items()}

    def test_empty_stratify_key_is_a_verdict_while_a_ghost_one_is_not(
            self, http_with_transforms):
        """空字符串拦（400），不存在的名字不拦（200）——这一松一紧是本文件最容易被读反的一处"""
        http, _root, _outside = http_with_transforms
        resp = http.post("/api/dataset/split", json={
            "input_file": "p.json", "output_dir": "k1", "stratify": True, "stratify_key": ""})
        assert resp.status_code == 400
        assert resp.json()["detail"] == "分层分割需要非空的 stratify_key"

    def test_three_segments_are_always_present_even_when_two_are_empty(
            self, http_with_transforms):
        """`1.0/0/0` ⇒ 三段键恒在、值可以是 0 ⇒ TS 面不必把 splits 写成可选"""
        http, _root, _outside = http_with_transforms
        splits = http.post("/api/dataset/split", json={
            "input_file": "p.json", "output_dir": "z1",
            "train_ratio": 1.0, "val_ratio": 0.0, "test_ratio": 0.0}).json()["splits"]
        assert set(splits) == {"train", "val", "test"}
        assert [splits[k]["count"] for k in ("train", "val", "test")] == [6, 0, 0]
        assert set(splits["val"]) == {"file", "count"}

    @pytest.mark.parametrize("path,payloads", [
        ("/api/dataset/convert", {"field": "output_file", "ok": "x.json",
                                  "traversal": "../evil.json"}),
        ("/api/dataset/merge", {"field": "output_file", "ok": "m.json",
                                "traversal": "../evil.json"}),
        ("/api/dataset/split", {"field": "output_dir", "ok": "s", "traversal": "../escape"}),
    ], ids=["convert", "merge", "split"])
    def test_absolute_path_outside_the_roots_is_403_while_dotdot_is_400(
            self, http_with_transforms, path, payloads):
        """越出白名单根的绝对路径 ⇒ 403；名字里带 `..` ⇒ 400「路径包含非法组件」

        两支的分工值得钉死：400 拦的是「组件形状非法」，403 拦的是「解析后不在根里」。把绝对路径
        一律当非法，白名单内的绝对路径就会被误拒（探针第一次就踩在这里，见模块 docstring）。
        """
        http, _root, outside = http_with_transforms
        field = payloads["field"]
        key, source = ("inputs", ["p.json"]) if path == "/api/dataset/merge" \
            else ("input_file", "p.json")
        resp = http.post(path, json={key: source, field: str(outside / payloads["ok"])})
        assert resp.status_code == 403
        assert resp.json()["detail"] == "路径超出允许的数据目录范围"

        traversal = http.post(path, json={key: source, field: payloads["traversal"]})
        assert traversal.status_code == 400
        assert traversal.json()["detail"] == "路径包含非法组件"

    def test_missing_input_is_404_with_a_string_detail_for_all_three(self, http_with_transforms):
        """三只端点的「文件不存在」都是**字符串** detail ⇒ `apiErrorDetail` 的主支不是想象"""
        http, _root, _outside = http_with_transforms
        for path, payload in (
            ("/api/dataset/convert", {"input_file": "nope.json", "output_file": "a.json"}),
            ("/api/dataset/merge", {"inputs": ["nope.json"], "output_file": "b.json"}),
            ("/api/dataset/split", {"input_file": "nope.json", "output_dir": "c1"}),
        ):
            resp = http.post(path, json=payload)
            assert resp.status_code == 404, (path, resp.json())
            assert isinstance(resp.json()["detail"], str)

    def test_empty_output_dir_is_a_verdict(self, http_with_transforms):
        """分割的输出目录留空 ⇒ 400「目录路径 不能为空」（不是「落到当前目录」）"""
        http, _root, _outside = http_with_transforms
        resp = http.post("/api/dataset/split", json={"input_file": "p.json", "output_dir": ""})
        assert resp.status_code == 400
        assert resp.json()["detail"] == "目录路径 不能为空"


@needs_web_src
class TestReceiptCoversEveryResponseKey:
    """响应模型的每一格都要在 TS 面上有名字、在页面回执里被读到——少一格就是吞掉一个读数"""

    FACES = [("ConvertResponse", ConvertResponse, "convertResult"),
             ("MergeResponse", MergeResponse, "mergeResult"),
             ("SplitResponse", SplitResponse, "splitResult")]

    @pytest.mark.parametrize("iface,model,variable", FACES, ids=[f[0] for f in FACES])
    def test_ts_face_matches_the_response_model(self, iface, model, variable):
        """TS 接口的键集合与响应模型的键集合**相等**（多一格是编造，少一格是漏消费）"""
        del variable
        assert set(ts_interface_keys(iface)) == set(model.model_fields)

    @pytest.mark.parametrize("iface,model,variable", FACES, ids=[f[0] for f in FACES])
    def test_every_key_is_read_in_the_page_receipt(self, iface, model, variable):
        """回执把每一格都读出来：写盘端点只回「写到哪、写了多少」，那些数就是全部产品面"""
        del iface
        page = _page()
        for key in model.model_fields:
            assert f"{variable}.{key}" in page, f"{model.__name__}.{key} 在页面上没人读"

    def test_split_part_face_and_the_table_row_derivation(self):
        """`SplitPart` 两键（`file` / `count`）都在 TS 与页面上；表行由 `Object.entries(splits)` 生成"""
        assert set(ts_interface_keys("SplitPart")) == set(SplitPart.model_fields)
        page = _page()
        for key in SplitPart.model_fields:
            assert f"part.{key}" in page, f"SplitPart.{key} 在页面上没人读"
        assert "Object.entries(splitResult.splits)" in page

    def test_the_merge_endpoint_forwards_one_knob_of_four_and_reports_five_of_six(
            self):
        """A290 的两半都钉住：`MergeConfig` 四档只接 `deduplicate`；SDK 六键只上线五键

        这两句是卡上「这一档只控开 / 关」那条口径的证据。少了它们，页面上那个 checkbox 会被读成
        整套去重策略的控制面；把 SDK 的截断计数留在本地，用户看到的 `removed_duplicates` 就可能
        被当成「少掉的条数的全部」（`augmentor/dataset_ops.py:220` 起特意把两件事分开报）。
        """
        assert set(MergeConfig.__dataclass_fields__) == {
            "deduplicate", "dedup_threshold", "preserve_order", "max_items"}
        assert set(MergeResponse.model_fields) == {
            "input_files", "output_file", "total_input", "total_output", "removed_duplicates"}
        assert '"truncated_by_max_items"' in DATASET_OPS_PY.read_text(encoding="utf-8")
        assert "truncated_by_max_items" not in _page()


@needs_web_src
class TestHonestyCopyOnTheThreeCards:
    """三张卡上那几句「这一档其实拦不住 / 其实不可调」的口径，每一句都要有实测陪着"""

    def test_the_xls_asymmetry_is_said_out_loud(self):
        """写边不认 xls 这件事必须写在卡上：两份下拉长得不一样，用户会以为是页面 bug"""
        assert "读边认它，写边刻意不认" in _page()

    def test_the_dedup_threshold_is_declared_not_on_this_endpoint(self):
        """卡上那句「只控开 / 关」点名 A290：不写它，一个开关会被读成整套去重策略"""
        page = _page()
        assert "去重阈值在这只端点上不可调" in page
        assert "A290" in page

    def test_the_two_branch_guards_and_the_ghost_key_are_all_declared(self):
        """A289 的容差差 + 负数只拦一支 + 幽灵分层键静默，三句都在卡上"""
        page = _page()
        assert "默认支是 0.01、分层支是 1e-6" in page
        assert "负数只有分层支会拦" in page
        assert "塌成一个分组" in page

    def test_the_page_names_every_receipt_instead_of_pretending_to_preview(self):
        """三条都是写盘端点：页面只渲回执，不假装「预览」落盘内容（写盘变换类的成文口径）"""
        page = _page()
        for name in ("转换回执", "合并回执", "分割回执"):
            assert f'aria-label="{name}"' in page
        assert "预览转换结果" not in page


def test_the_three_new_fetchers_hit_the_three_write_paths_and_nothing_else():
    """服务层新写的三条只打这三条路径（拼错的端点在这一档就响，不必等 l224 的集合相等）"""
    for name, path in (("convertDataset", "/dataset/convert"),
                       ("mergeDatasets", "/dataset/merge"),
                       ("splitDataset", "/dataset/split")):
        assert f"api.post('{path}'" in fetcher_body(name), f"{name} 打的不是 {path}"
