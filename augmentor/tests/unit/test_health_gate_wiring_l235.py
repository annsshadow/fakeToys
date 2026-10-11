# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L235（A279 第十批）：`POST /api/quality/health-gate` 进质量中心 —— 默认同源、判决如实

**这一轮接的一条**：健康门禁进 `Quality.tsx` 新卡 + 第九个 Tab，并让
`/api/quality/evaluate` 算出来的 `pass_rate` 能一键喂进来（含 **0.0** 那一档 —— 服务层
刻意不写 `|| undefined`，否则「算出来是 0」会被静默降级成「没算这一项」）。消费面因此从
72 涨到 **73**、缺口从 3 收到 **2**（那三个数由 l224 那把尺子钉，本文件不另钉一份）。

**为什么值得单独一把尺子**：这一只端点把三个模块（`DatasetHealthScore` / `ImpactEvaluator` /
`QualityGate`）接成一条判定链，而它的响应里有**四个名单**与**两份指标**，
每一个都很容易被页面读错：`gate.passed` 对 warned 也是 true、`failed_rules` 在
`block_on_warning` 下可以是空的、`skipped_rules` 与 `failed_rules` 是相反的意思、
`errored_rules` 实测恒空。接线只让 l224 的数涨一格，读错一格用户就会把「没算」当「不及格」。

**接线时量出来的三条产品缺陷，本轮立账不修**（改的是判定语义，得单独拍）：

* **A319** 权重不校验非负与上界、`health_score` 也不钳进 [0,1]：同一份数据默认权重下
  0.893 / healthy，`[0,0,0,1]` 变 1.0，`[2.0,-0.5,-0.25,-0.25]` 变 **1.17**；而门禁三条
  规则一条都不看权重 ⇒ 这三档的 verdict 全是 failed，「健康分 healthy」与「门禁阻断」
  同时为真。
* **A320** `abs(sum(weights) - 1.0) > 0.01` 在恰好 0.01 那一档是假的：0.99 与 1.01 的差
  实测都是 `0.010000000000000009 > 0.01` ⇒ 400，而 1.005 放行；docstring 写的「之和 ≈ 1.0」
  与 `HealthGateRequest` 注释里的「4 个元素、之和 ≈ 1.0」都不是它实际的那把尺子。
* **A321** `text_field` 是空串或数据集里不存在的键时**一句判决都没有**：所有条目取到空值，
  实测 `duplicate_rate` 变成 **1.0**，门禁于是在 `duplicate_rate` 那条上判负 —— 一次
  彻头彻尾的假判决，而且它与「数据真的全重复」在响应里长得一模一样。

五面判据，各对应一种真实失效形状：

1. **页面初值 == 契约默认**（`TestThePageInitialsAreTheContractDefaults`）：五个 state 的
   字面初值逐项等于 `HealthGateRequest.model_fields[...].default`；`pass_rate` 与四个权重
   留空（`null`）⇒ 页面没有第二套默认在暗地里生效。权重标签的**顺序**与
   `DatasetHealthScore.__init__` docstring 里那份 `[完整性, 多样性, 质量分布均衡性, 覆盖率]`
   对齐（响应里的 `weights` 也按这个序回显，错位就会把分数记到别的指标头上）。
2. **服务层没有第二套默认**（`TestTheServiceLayerKeepsOneAuthority`）：fetcher 体内不许出现
   任何阈值数字或 `false` 字面；三处哨兵各按其动机钉住 —— `text_field` 用 `|| undefined`
   （躲 A321 那次假判决）、三个阈值与 `block_on_warning` 转 `undefined`（不是 `Optional`，
   送 `null` 实测 **422**）、**`pass_rate` 那一格不许用 `||`**（0 合法且 evaluate 真会回 0.0）。
3. **TS 响应面 == pydantic 模型**（`TestTheTsFaceEqualsThePydanticFace`）：三层嵌套逐键相等
   + 顺序相等，再用真链路把运行时回执的键集对一遍 —— FastAPI 按 `response_model` 过滤，
   模型少写一键该键就静默消失（L27 记过的坑），所以这一面必须两头都量。
4. **判决如实**（`TestTheVerdictsAreWhatThePageSays`）：三档 verdict 各有一支真链路证据
   （failed / warned / passed），外加 `block_on_warning` 那一档「verdict=failed 而
   `failed_rules` 空」、`skipped_rules` 与 `pass_rate=0.0` 的区别、A319/A320/A321 三条缺陷
   的现场、`null` 的不对称（Optional 200 / 非 Optional 422）、四档错误码
   （400 权重两句 / 400 pass_rate 越界 / 400 空数据集 / 404 文件不存在 / 422 缺字段），
   以及「这一只端点不挂 API key 门禁」。
5. **口径如实**（`TestTheHonestProse`）：页面上写着的那几句脾气逐句能被机器对账 ——
   verdict 三档清单与 `GateVerdict` 逐项相等、等级阈值 0.7 / 0.4 从 `health_score.py` 的
   源码里读出来对、A319 那个 1.17、A321 那个 1.0、`block_on_warning` 那句
   「failed_rules 是空的」、阈值无范围闸、`errored_rules` 恒空。

跑法：`python -m pytest tests/unit/test_health_gate_wiring_l235.py -q --no-cov`
"""

import json
import pathlib
import re

import pytest
from tests.unit.test_transform_wiring_l229 import fetcher_body, ts_interface_keys
from tests.unit.test_wired_dropdown_choices_l226 import needs_web_src, strip_comments

from api.routes.quality import (
    GateReportResponse,
    HealthGateRequest,
    HealthMetricsResponse,
    HealthReportResponse,
)
from api.routes.quality import HealthGateResponse as PyHealthGateResponse
from augmentor.quality_gate import GateVerdict

ROOT = pathlib.Path(__file__).resolve().parents[2]
SRC = ROOT / "web" / "src"
QUALITY_TSX = SRC / "pages" / "Quality.tsx"
HEALTH_SCORE_PY = ROOT / "augmentor" / "health_score.py"

#: 夹具 A：4 条，前两条**完全重复**（duplicate_rate 实测 0.5 > 默认上限 0.3 ⇒ 判负）
DUP_ITEMS = [
    {"instruction": "请解释什么是向量数据库，并给出一个使用场景。",
     "output": "向量数据库按相似度检索高维向量。"},
    {"instruction": "请解释什么是向量数据库，并给出一个使用场景。",
     "output": "向量数据库按相似度检索高维向量。"},
    {"instruction": "写一首关于秋天的诗，要求四行以上。",
     "output": "落叶铺满小径，秋天把颜色交给风。"},
    {"instruction": "什么是包含判断，它在检索里有什么用？", "output": "包含判断是子串匹配。"},
]

#: 夹具 B：4 条互不重复（duplicate_rate 0.0），但 output 有一条空、一条缺键
#: ⇒ completeness 实测 0.5 < 默认下限 0.8，且它是 **warning** 级 ⇒ verdict 是 warned
SPARSE_ITEMS = [
    {"instruction": "第一条问题，内容足够长以便通过长度检查。", "output": "第一条答案。"},
    {"instruction": "第二条问题，内容足够长以便通过长度检查。", "output": ""},
    {"instruction": "第三条问题，内容足够长以便通过长度检查。"},
    {"instruction": "第四条问题，内容足够长以便通过长度检查。", "output": "第四条答案。"},
]

#: 两份夹具上的实测读数（仓外探针 `l235_probe6.py`，L235 跑完即删）。
#: 探针按 4 位小数打印，所以 `quality_balance` 那一格用 1e-3 的容差比，其余各项都是精确值。
DUP_HEALTH_SCORE = 0.8927055856521438
DUP_GATE_METRICS = {
    "completeness": 1.0, "diversity": 0.75, "quality_balance": 0.8208,
    "coverage": 1.0, "duplicate_rate": 0.5,
}
SPARSE_HEALTH_SCORE = 0.6250000833333056
#: A319 的现场：权重换掉之后健康分能**超过 1**（同一份数据、同一份 verdict）
WEIGHTS_OVER_ONE = 1.1697944143478562

#: `HealthGateRequest` 的字段默认（这一份由 pydantic 自己说，不在测试里抄第二遍）
FIELD_DEFAULTS = {name: field.default for name, field in HealthGateRequest.model_fields.items()}

STATE_RE = re.compile(
    r"const \[(\w+), set\w+\] = useState(?:<[^>]*>)?\((.*?)\)\s*$", re.M)


def _page() -> str:
    assert QUALITY_TSX.is_file(), "前端源码不在场 ⇒ 本文件的判据全部失效"
    return strip_comments(QUALITY_TSX.read_text(encoding="utf-8"))


def state_initial(name: str) -> str:
    """`const [name, setName] = useState(初值)` 的字面原文

    自己写一份而不复用 l229 的同名工具：那把 `STATE_RE` 的泛型位是 `<[^()]*?>`，
    读不下本页 `useState<(number | null)[]>([null, null, null, null])` 这种泛型里带括号的形状
    （实测匹配不到 ⇒ 会报「页面里没有这个 state」，那是假红）。这里放宽到 `<[^>]*>`：
    泛型里不许出现 `>`，本页面所有 state 都满足，且放宽只多认形状、不改既有读数。
    """
    pairs = dict(STATE_RE.findall(_page()))
    assert name in pairs, f"页面里没有 {name} 这个 state ⇒ 接线被拆了"
    return pairs[name].strip()


def weight_labels() -> list:
    found = re.search(r"const HEALTH_WEIGHT_LABELS = \[(.*?)\]", _page(), re.S)
    assert found, "页面里没有 HEALTH_WEIGHT_LABELS 常量 ⇒ 判据要跟着换形状"
    return re.findall(r"'([^']+)'", found.group(1))


def verdict_copy_keys() -> list:
    found = re.search(r"const GATE_VERDICT_COPY[^=]*= \{(.*?)\n\}", _page(), re.S)
    assert found, "页面里没有 GATE_VERDICT_COPY 常量 ⇒ 判据要跟着换形状"
    return re.findall(r"^\s{2}(\w+):", found.group(1), re.M)


def health_score_level_thresholds() -> tuple:
    """从 `health_score.py` 的源码里读出等级阈值（不抄字面）"""
    src = HEALTH_SCORE_PY.read_text(encoding="utf-8")
    found = re.search(
        r'"level": "healthy" if total >= ([\d.]+) else \("moderate" if total >= ([\d.]+)', src)
    assert found, "health_score.py 里等级那一行变了形状 ⇒ 本条要跟着换形状"
    return float(found.group(1)), float(found.group(2))


@pytest.fixture
def http(tmp_path, monkeypatch):
    """白名单数据根指向临时目录：这一只端点只读，但仍不许把夹具落进仓库"""
    root = tmp_path / "ds"
    root.mkdir()
    (root / "dup.json").write_text(json.dumps(DUP_ITEMS, ensure_ascii=False), encoding="utf-8")
    (root / "sparse.json").write_text(
        json.dumps(SPARSE_ITEMS, ensure_ascii=False), encoding="utf-8")
    (root / "empty.json").write_text("[]", encoding="utf-8")
    monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", str(root))
    monkeypatch.setenv("AUGMENTOR_API_KEY", "")
    monkeypatch.chdir(tmp_path)
    from fastapi.testclient import TestClient

    from api.main import app

    return TestClient(app), root


def gate(client, **body):
    return client.post("/api/quality/health-gate", json={"input_file": "dup.json", **body})


@needs_web_src
class TestThePageInitialsAreTheContractDefaults:
    """页面初值逐项等于 `HealthGateRequest` 的字段默认，留空的两格是真的留空"""

    def test_the_five_defaults_come_from_the_contract(self):
        assert state_initial("gateTextField") == f"'{FIELD_DEFAULTS['text_field']}'"
        assert state_initial("gatePassRateMin") == str(FIELD_DEFAULTS["pass_rate_min"])
        assert state_initial("gateDuplicateMax") == str(FIELD_DEFAULTS["duplicate_rate_max"])
        assert state_initial("gateCompletenessMin") == str(FIELD_DEFAULTS["completeness_min"])
        assert state_initial("gateBlockOnWarning") == (
            "true" if FIELD_DEFAULTS["block_on_warning"] else "false")

    def test_the_contract_defaults_are_what_the_ruler_assumes(self):
        """上面那五支不许空转：契约默认确实是这一组值（改了就要重钉页面初值）"""
        assert FIELD_DEFAULTS["text_field"] == "instruction"
        assert FIELD_DEFAULTS["pass_rate_min"] == 0.6
        assert FIELD_DEFAULTS["duplicate_rate_max"] == 0.3
        assert FIELD_DEFAULTS["completeness_min"] == 0.8
        assert FIELD_DEFAULTS["block_on_warning"] is False
        assert FIELD_DEFAULTS["weights"] is None and FIELD_DEFAULTS["pass_rate"] is None

    def test_pass_rate_and_weights_start_empty(self):
        """留空 = 不传：pass_rate 不传时后端摘掉规则并记进 `skipped_rules`，
        权重不传时用 `DatasetHealthScore` 的默认 `[0.25] × 4`（页面不抄那一份）"""
        assert state_initial("gatePassRate") == "null"
        assert state_initial("gateWeights") == "[null, null, null, null]"

    def test_the_page_does_not_copy_the_default_weights(self):
        """页面不许有第二套默认：调用点一个数字字面都不带

        扫描面只取 `healthGate(file, {…})` 那一块，**不取整页**：用户可见的文案里写着
        「默认权重 [0.25 × 4]」是在解释后端那份默认（A319 那条现场也要靠它才读得懂），
        不是第二份默认。本轮第一次跑就是按整页扫的，被这句如实文案判红 —— 那是尺子的
        范围错了，不是产品错了；权重初值全空由上一支钉住。
        """
        page = _page()
        start = page.index("healthGate(file, {")
        block = page[start:page.index("setGate,")]
        assert not re.search(r"\d", block), f"调用点里出现了数字字面：{block}"
        assert "weights:" in block and "passRate:" in block
        assert "gateWeightsFilled.length === HEALTH_WEIGHT_LABELS.length" in block

    def test_weight_labels_follow_the_authority_order(self):
        """四个标签与 `DatasetHealthScore.__init__` docstring 里那份顺序对齐

        为什么只比「前两字出现在权威名里」而不是逐项相等：页面用的是短名
        （`质量均衡` vs 权威 `质量分布均衡性`），逐项相等会把「同一个指标的简称」判成漂移。
        这一维仍然会响：把顺序换一下、或把某个标签改成别的指标，前两字就对不上了。
        """
        doc = re.search(r"weights: 权重 \[([^\]]+)\]",
                        HEALTH_SCORE_PY.read_text(encoding="utf-8"))
        assert doc, "health_score.py 里权重顺序那份 docstring 变了形状 ⇒ 本条要跟着换形状"
        authority = [w.strip() for w in doc.group(1).split(",")]
        labels = weight_labels()
        assert len(labels) == len(authority) == 4, (labels, authority)
        for label, name in zip(labels, authority):
            assert label[:2] in name, f"页面标签 {label} 与权威第 {labels.index(label)} 项 {name} 不匹配"


@needs_web_src
class TestTheServiceLayerKeepsOneAuthority:
    """fetcher 里没有第二套默认，三处哨兵各按其动机存在"""

    def test_no_threshold_literal_lives_in_the_fetcher(self):
        body = fetcher_body("healthGate")
        assert not re.search(r"\d\.\d", body), f"fetcher 里出现了阈值字面值：{body}"
        assert "false" not in body and "true" not in body, f"fetcher 里写死了布尔默认：{body}"

    def test_text_field_dodges_the_silent_verdict(self):
        """A321：空串 `text_field` 后端一句判决都不给，实测把 duplicate_rate 顶成 1.0"""
        assert "text_field: opts.textField || undefined" in fetcher_body("healthGate")

    def test_the_non_optional_fields_become_undefined_not_null(self):
        """这四个不是 `Optional`，送 `null` 实测 422（axios 的 JSON.stringify 会留下 null）"""
        body = fetcher_body("healthGate")
        for key, opt in (("pass_rate_min", "passRateMin"),
                         ("duplicate_rate_max", "duplicateRateMax"),
                         ("completeness_min", "completenessMin"),
                         ("block_on_warning", "blockOnWarning")):
            assert f"{key}: opts.{opt}," in body, f"{key} 那一格没有原样转发：{body}"
        for name in ("passRateMin", "duplicateRateMax", "completenessMin", "blockOnWarning"):
            assert re.search(rf"{name}\?: (number|boolean)$", body, re.M), f"{name} 不是可选参数"

    def test_pass_rate_must_not_use_the_or_sentinel(self):
        """`0` 是这一格的合法值，而 `/api/quality/evaluate` 真会回 0.0

        用 `|| undefined` 会把「算出来是 0」静默变成「没算这一项」（进 `skipped_rules`），
        正是端点 docstring 里写着要避免的那件事。这一支是**禁令型**：只判 pass_rate 那一行。
        """
        line = [ln for ln in fetcher_body("healthGate").splitlines()
                if ln.strip().startswith("pass_rate:")]
        assert len(line) == 1, f"pass_rate 那一行应恰好一处，实得 {line}"
        assert "||" not in line[0], f"pass_rate 不许用 `||` 哨兵：{line[0]}"
        assert line[0].strip() == "pass_rate: opts.passRate,", line[0]

    def test_weights_are_forwarded_without_a_length_filter(self):
        """个数与和值那两句 400 是该让用户看见的真判决，服务层不许先替它过滤掉"""
        assert "weights: opts.weights," in fetcher_body("healthGate")


@needs_web_src
class TestTheTsFaceEqualsThePydanticFace:
    """三层嵌套逐键相等（含顺序），再用真链路对一遍运行时回执的键集"""

    def test_the_three_levels_match_key_for_key(self):
        assert ts_interface_keys("HealthGateResponse") == ["health", "gate", "skipped_rules"]
        assert set(ts_interface_keys("HealthGateResponse")) == set(PyHealthGateResponse.model_fields)
        assert set(ts_interface_keys("HealthGateReport")) == set(HealthReportResponse.model_fields)
        assert set(ts_interface_keys("HealthGateMetrics")) == set(HealthMetricsResponse.model_fields)
        assert set(ts_interface_keys("HealthGateVerdict")) == set(GateReportResponse.model_fields)

    def test_the_metrics_order_is_the_weighted_order(self):
        """四项指标的顺序 = 权重那四项的顺序，页面按这个序回显权重"""
        assert ts_interface_keys("HealthGateMetrics") == [
            "completeness", "diversity", "quality_balance", "coverage"]
        assert weight_labels() == ["完整性", "多样性", "质量均衡", "覆盖率"]

    def test_the_verdict_keys_include_errored_rules(self):
        """`errored_rules` 必须在 TS 面上：FastAPI 按 response_model 过滤，
        少写一键它就静默消失，而「规则算不出来」正是最该被看见的信号"""
        assert ts_interface_keys("HealthGateVerdict") == [
            "verdict", "passed", "failed_rules", "warned_rules", "errored_rules", "metrics"]

    def test_the_runtime_payload_has_exactly_those_keys(self, http):
        client, _ = http
        r = gate(client)
        assert r.status_code == 200, r.text[:300]
        body = r.json()
        assert set(body) == set(PyHealthGateResponse.model_fields)
        assert set(body["health"]) == set(HealthReportResponse.model_fields)
        assert set(body["gate"]) == set(GateReportResponse.model_fields)
        assert set(body["health"]["metrics"]) == set(HealthMetricsResponse.model_fields)


@needs_web_src
class TestTheVerdictsAreWhatThePageSays:
    """真链路：三档 verdict 各有证据，四条缺陷/脾气各有现场"""

    def test_the_duplicate_fixture_is_failed_on_duplicate_rate(self, http):
        client, _ = http
        body = gate(client).json()
        assert body["gate"]["verdict"] == GateVerdict.FAILED
        assert body["gate"]["passed"] is False
        assert body["gate"]["failed_rules"] == ["duplicate_rate"]
        assert body["gate"]["warned_rules"] == [] and body["gate"]["errored_rules"] == []
        # pass_rate 没传 ⇒ 该规则**没参与判决**，住在 skipped 里而不是 failed 里
        assert body["skipped_rules"] == ["pass_rate"]
        assert "pass_rate" not in body["gate"]["metrics"]
        assert body["health"]["health_score"] == pytest.approx(DUP_HEALTH_SCORE)
        assert body["health"]["level"] == "healthy"
        assert body["health"]["total_samples"] == len(DUP_ITEMS)
        assert body["health"]["weights"] == [0.25, 0.25, 0.25, 0.25]
        for key, value in DUP_GATE_METRICS.items():
            assert body["gate"]["metrics"][key] == pytest.approx(value, abs=1e-3), key

    def test_pass_rate_zero_is_a_verdict_not_a_skip(self, http):
        """A279 第十批接线的动机：0.0 必须**参与**判决（服务层那句 `|| undefined` 禁令的证据）

        `pass_rate: 0.0` 与「不传」在响应里的区别是 skipped_rules 空/非空 +
        failed_rules 多一个名字 —— 用 `||` 就会把前者变成后者。
        """
        client, _ = http
        zero = gate(client, pass_rate=0.0).json()
        assert zero["skipped_rules"] == []
        assert zero["gate"]["failed_rules"] == ["pass_rate", "duplicate_rate"]
        assert zero["gate"]["metrics"]["pass_rate"] == 0.0
        omitted = gate(client).json()
        assert omitted["skipped_rules"] == ["pass_rate"]
        assert omitted["gate"]["failed_rules"] == ["duplicate_rate"]
        passing = gate(client, pass_rate=1.0).json()
        assert passing["skipped_rules"] == []
        assert passing["gate"]["failed_rules"] == ["duplicate_rate"]

    def test_the_sparse_fixture_is_warned_and_still_passed(self, http):
        """warned 那一档：`passed` 仍然是 true ⇒ 页面不能只看 passed"""
        client, _ = http
        r = client.post("/api/quality/health-gate", json={"input_file": "sparse.json"})
        body = r.json()
        assert body["gate"]["verdict"] == GateVerdict.WARNED
        assert body["gate"]["passed"] is True
        assert body["gate"]["failed_rules"] == []
        assert body["gate"]["warned_rules"] == ["completeness"]
        assert body["gate"]["metrics"]["duplicate_rate"] == 0.0
        assert body["health"]["health_score"] == pytest.approx(SPARSE_HEALTH_SCORE)
        assert body["health"]["level"] == "moderate"

    def test_block_on_warning_makes_failed_rules_empty(self, http):
        """verdict=failed 而 `failed_rules` 是空的：判负住在 warned_rules 里

        页面那句文案（「block_on_warning 改的是 verdict 的合成，不是规则的严重级」）
        的现场就在这一支。
        """
        client, _ = http
        body = client.post("/api/quality/health-gate", json={
            "input_file": "sparse.json", "block_on_warning": True}).json()
        assert body["gate"]["verdict"] == GateVerdict.FAILED
        assert body["gate"]["passed"] is False
        assert body["gate"]["failed_rules"] == []
        assert body["gate"]["warned_rules"] == ["completeness"]

    def test_passed_is_reachable_by_loosening_the_duplicate_ceiling(self, http):
        """三档 verdict 都得有真证据：把上限放到 99.0，同一份数据就 passed 了"""
        client, _ = http
        body = gate(client, duplicate_rate_max=99.0).json()
        assert body["gate"]["verdict"] == GateVerdict.PASSED
        assert body["gate"]["passed"] is True
        assert body["gate"]["failed_rules"] == [] and body["gate"]["warned_rules"] == []

    def test_a319_weights_can_push_the_score_over_one_without_touching_the_verdict(self, http):
        client, _ = http
        baseline = gate(client).json()
        all_coverage = gate(client, weights=[0.0, 0.0, 0.0, 1.0]).json()
        negative = gate(client, weights=[2.0, -0.5, -0.25, -0.25]).json()
        assert all_coverage["health"]["health_score"] == pytest.approx(1.0)
        assert negative["health"]["health_score"] == pytest.approx(WEIGHTS_OVER_ONE)
        assert negative["health"]["health_score"] > 1.0, "A319：健康分没有上界钳制"
        for body in (baseline, all_coverage, negative):
            assert body["health"]["level"] == "healthy"
            assert body["gate"]["verdict"] == GateVerdict.FAILED
            assert body["gate"]["metrics"]["duplicate_rate"] == 0.5

    def test_a320_the_sum_tolerance_is_not_what_the_docstring_says(self, http):
        """0.99 与 1.01 都被拒（浮点差是 0.010000000000000009 > 0.01），1.005 放行"""
        client, _ = http
        for weights in ([0.24, 0.25, 0.25, 0.25], [0.26, 0.25, 0.25, 0.25]):
            r = gate(client, weights=weights)
            assert r.status_code == 400, (weights, r.text[:200])
            assert r.json()["detail"] == "权重之和必须为 1.0"
        assert abs(sum([0.24, 0.25, 0.25, 0.25]) - 1.0) > 0.01
        assert gate(client, weights=[0.255, 0.25, 0.25, 0.25]).status_code == 200
        r = gate(client, weights=[0.25, 0.25, 0.25])
        assert r.status_code == 400 and r.json()["detail"] == "权重必须包含 4 个元素"

    def test_a321_a_ghost_text_field_produces_a_fake_duplicate_verdict(self, http):
        """空串与幽灵键都不报错，duplicate_rate 一律 1.0 ⇒ 与「真的全重复」长得一样"""
        client, _ = http
        for text_field in ("", "no_such_field"):
            body = gate(client, text_field=text_field).json()
            assert body["gate"]["metrics"]["duplicate_rate"] == 1.0, text_field
            assert body["gate"]["failed_rules"] == ["duplicate_rate"]
            assert body["health"]["metrics"]["completeness"] == 1.0, "健康分那四项不看 text_field"
        real = gate(client, text_field="output").json()
        assert real["gate"]["metrics"]["duplicate_rate"] == 0.5

    def test_the_three_thresholds_have_no_range_gate(self, http):
        """负数与大于 1 的值全收，判决随阈值单调变化 ⇒ 页面刻意不给这三个框设 min/max"""
        client, _ = http
        assert gate(client, pass_rate=0.0, pass_rate_min=-1.0).json()[
            "gate"]["failed_rules"] == ["duplicate_rate"]
        assert gate(client, pass_rate=1.0, pass_rate_min=5.0).json()[
            "gate"]["failed_rules"] == ["pass_rate", "duplicate_rate"]
        assert gate(client, completeness_min=-3.0).status_code == 200
        assert gate(client, duplicate_rate_max=-0.1).json()[
            "gate"]["failed_rules"] == ["duplicate_rate"]

    def test_null_is_asymmetric_between_optional_and_required_fields(self, http):
        """服务层那四处 `?? undefined` 的证据：非 Optional 送 null 是 422，Optional 是 200"""
        client, _ = http
        assert gate(client, pass_rate=None).status_code == 200
        assert gate(client, weights=None).status_code == 200
        for field, err in (("pass_rate_min", "float_type"),
                           ("duplicate_rate_max", "float_type"),
                           ("completeness_min", "float_type"),
                           ("block_on_warning", "bool_type")):
            r = gate(client, **{field: None})
            assert r.status_code == 422, (field, r.text[:200])
            assert r.json()["detail"][0]["type"] == err, field

    def test_the_four_error_shapes(self, http):
        client, _ = http
        r = gate(client, pass_rate=1.5)
        assert r.status_code == 400 and r.json()["detail"] == "pass_rate 必须在 [0, 1] 区间内"
        r = client.post("/api/quality/health-gate", json={"input_file": "empty.json"})
        assert r.status_code == 400
        assert r.json()["detail"] == "数据集为空，门禁无法给出有意义的判定"
        r = client.post("/api/quality/health-gate", json={"input_file": "nope.json"})
        assert r.status_code == 404 and r.json()["detail"] == "文件不存在"
        r = client.post("/api/quality/health-gate", json={})
        assert r.status_code == 422
        assert r.json()["detail"][0]["type"] == "missing"
        # 四档 detail 都是前端 `apiErrorDetail` 认得的形状（字符串或 pydantic 列表）
        for body in ({"pass_rate": 1.5}, {"input_file": "empty.json"}):
            d = client.post("/api/quality/health-gate",
                            json={"input_file": "dup.json", **body}).json()["detail"]
            assert isinstance(d, str), d

    def test_errored_rules_is_empty_on_every_shape_measured(self, http):
        """页面写着「这一只端点上实测恒空」：把量过的每一档都扫一遍"""
        client, _ = http
        shapes = [
            {"input_file": "dup.json"},
            {"input_file": "dup.json", "pass_rate": 0.0},
            {"input_file": "dup.json", "weights": [0.0, 0.0, 0.0, 1.0]},
            {"input_file": "dup.json", "text_field": ""},
            {"input_file": "dup.json", "duplicate_rate_max": 99.0},
            {"input_file": "sparse.json", "block_on_warning": True},
        ]
        for shape in shapes:
            body = client.post("/api/quality/health-gate", json=shape).json()
            assert body["gate"]["errored_rules"] == [], shape

    def test_this_endpoint_is_not_behind_the_api_key_gate(self, http, monkeypatch):
        """只读、不写盘 ⇒ 门禁升起时仍然 200（l234 那三支只读端点的同一档）"""
        client, _ = http
        from api.deps import API_KEY_ENV

        monkeypatch.setenv(API_KEY_ENV, "s3cret")
        assert gate(client).status_code == 200
        assert API_KEY_ENV == "AUGMENTOR_API_KEY"


@needs_web_src
class TestTheHonestProse:
    """页面上那几句脾气，逐句能被机器对账（不许只是散文）"""

    def test_the_verdict_copy_covers_exactly_the_three_constants(self):
        assert verdict_copy_keys() == [
            GateVerdict.PASSED, GateVerdict.WARNED, GateVerdict.FAILED], verdict_copy_keys()

    def test_the_level_thresholds_on_the_page_come_from_the_source(self):
        healthy, moderate = health_score_level_thresholds()
        assert (healthy, moderate) == (0.7, 0.4)
        assert f"≥{healthy} healthy" in _page() and f"≥{moderate} moderate" in _page()

    def test_the_page_states_the_measured_numbers(self):
        page = _page()
        # A319：健康分能超过 1（实测 1.17）
        assert "1.17" in page and "超过 1" in page
        # A320：容差不是字面那个 0.01
        assert "0.99 与 1.01" in page and "1.005" in page
        # A321：幽灵字段 ⇒ duplicate_rate 1.0
        assert "duplicate_rate 变成 1.0" in page and "先怀疑字段名" in page
        # block_on_warning：failed_rules 空
        assert "failed_rules 是空的" in page
        # skipped 与 failed 是相反的意思
        assert "没参与判决" in page and "不是判负" in page
        # errored_rules 恒空
        assert "恒空" in page
        # 健康分不参与门禁判决
        assert "健康分本身不参与门禁判决" in page

    def test_the_page_admits_the_thresholds_have_no_range_gate(self):
        page = _page()
        assert "三个阈值没有范围闸" in page
        start = page.index("<span>pass_rate ≥</span>")
        end = page.index("<span>健康分权重</span>")
        region = page[start:end]
        assert "min={" not in region and "max={" not in region, region
        # 反证：这一维不是「全页面一刀切禁 min/max」—— pass_rate 那一格确实设了 [0,1] 的闸
        before = page[page.index("<span>通过率</span>"):start]
        assert "min={0}" in before and "max={1}" in before, before

    def test_the_partial_weights_warning_names_the_two_backend_verdicts(self):
        """页面那句「个数必须是 4、之和必须是 1.0」正是后端那两句 400 的原文口径"""
        page = _page()
        assert "gateWeightsPartial" in page and "disabled={gateWeightsPartial}" in page
        assert "个数必须是 4" in page and "之和必须是 1.0" in page
        assert FIELD_DEFAULTS["weights"] is None

    def test_the_fill_in_button_is_gated_on_a_real_evaluation(self):
        """一键喂通过率：没有评估结果时禁用，有了就把 `pass_rate` 原样喂进来（含 0）"""
        page = _page()
        assert "disabled={!evaluation}" in page
        assert "setGatePassRate(evaluation.pass_rate)" in page
