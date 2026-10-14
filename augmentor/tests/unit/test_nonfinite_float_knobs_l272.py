# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L272 守卫：非有限浮点（NaN / 无穷）在请求侧 float 旋钮族没有判据

三件事，各自都要能单独喊红：

1. **权威维**：``validation.require_ratio`` 现在逐字判「是不是有限数值」，而不只判 NaN。
   改前无穷是靠闭区间**顺带**挡住的，所以这一族一旦出现「下界判死、上界开放」的旋钮
   （``min_scale_gain`` 的定义域是 [-1, +∞)），``float('inf')`` 就能穿过
   ``not minimum <= value <= maximum`` 一路走进比较式。
2. **落点维**：四只 SDK 咽喉共引同一份权威——``DatasetOperations.sample`` 的 ``ratio``、
   ``gate_dataset_health`` 的三只阈值、``ImpactEvaluator.is_beneficial`` 的
   ``min_scale_gain``、``OutlierDetector`` 的 ``threshold``。判在 SDK 而不是判在端点，
   是因为 CLI 与 API 共用这几层（``quality_gate`` 自己的 docstring 就把这句话写成原则）。
3. **现量维**：14 只 float 旋钮 × 四形（NaN / 无穷 / 5.0 / -1.0）的 HTTP 归宿**整表**钉死。
   其中 ``raise_internal_error`` 那一族（**A280**，待拍）与 422 渲染崩溃那一族
   （**A311** 的新现量）按**观测行为**钉住，不是假装它们合法——谁改这两族，这张表当场红。

改前的三格实测（本轮沙箱复现，逐格见循环日志 L272）：``ratio=inf`` 经
``int(len * inf)`` 抛 ``OverflowError`` 而它**不是** ``ValueError`` 子类 ⇒ ``to_http_error``
落到 500 档；``ratio=-0.5`` 在 systematic 支折成 ``items[:-5]``（「丢最后 5 条」）而
stratified 支折成 0 条，两支都回 **200**；``min_scale_gain=NaN`` 让 ``scale_gain < NaN``
恒 False ⇒ ``beneficial`` 无声判成 true。
"""

import ast
import json
import math
from pathlib import Path

import pytest
from fastapi.testclient import TestClient

from api.deps import INTERNAL_ERROR_DETAIL
from api.main import app
from augmentor.dataset_ops import DatasetOperations, SampleConfig
from augmentor.exceptions import DataValidationError
from augmentor.impact import AugmentationImpact, ImpactEvaluator
from augmentor.outlier import OutlierDetector
from augmentor.quality_gate import GateVerdict, gate_dataset_health
from augmentor.validation import require_ratio

PRODUCT_ROOT = Path(__file__).resolve().parents[2] / "augmentor"

NAN = float("nan")
INF = float("inf")
MINUS_INF = float("-inf")

#: 本轮接线的四只咽喉：`(文件, 权威函数名) -> 该产品文件里的调用点数`。
#: 按文件计数而不是按行号——行号会被插行平移（L217 / L219 / L266 三次同一缺陷类），
#: 而「这一族在这一层有没有判」这一事实不该跟着物理行号走。
WIRING_ROSTER = {
    ("dataset_ops.py", "require_ratio"): 1,
    ("quality_gate.py", "require_ratio"): 3,
    ("impact.py", "require_ratio"): 1,
    ("outlier.py", "require_positive"): 1,
}

#: 现量表：`(路径, 旋钮) -> (NaN, 无穷, 5.0, -1.0)` 四档的 HTTP 归宿。
#: `CRASH` 是「服务端渲染 422 时自己抛 ValueError」——TestClient 把它升成异常，
#: 生产部署（uvicorn + ServerErrorMiddleware）的对应物是「没有 JSON 响应体」。
CENSUS = {
    ("/api/dataset/sample", "ratio"): ("400", "400", "400", "400"),
    ("/api/dataset/split", "train_ratio"): ("400", "400", "400", "400"),
    ("/api/dataset/split", "val_ratio"): ("400", "400", "400", "400"),
    ("/api/dataset/split", "test_ratio"): ("400", "400", "400", "400"),
    ("/api/dataset/search", "fuzzy_threshold"): ("400", "400", "400", "400"),
    ("/api/dataset/impact", "min_scale_gain"): ("400", "400", "200", "200"),
    ("/api/quality/health-gate", "pass_rate"): ("400", "400", "400", "400"),
    ("/api/quality/health-gate", "pass_rate_min"): ("400", "400", "400", "400"),
    ("/api/quality/health-gate", "duplicate_rate_max"): ("400", "400", "400", "400"),
    ("/api/quality/health-gate", "completeness_min"): ("400", "400", "400", "400"),
    ("/api/quality/outliers", "threshold"): ("400", "200", "200", "400"),
    ("/api/quality/dedup", "threshold"): ("500", "500", "500", "500"),
    ("/api/quality/benchmark", "threshold"): ("500", "500", "500", "500"),
    ("/api/leakage/check", "fuzzy_threshold"): ("CRASH", "CRASH", "422", "422"),
}

#: 五档「本轮或既往轮次已经判死」的旋钮：detail 必须点名它自己或它所属的族名。
#: `split` 那一族点的是族名 `ratios`（`require_ratio_list` 的口径），不是三个字段名。
VERDICT_MARKERS = {
    "ratio": "ratio",
    "train_ratio": "ratios",
    "val_ratio": "ratios",
    "test_ratio": "ratios",
    "fuzzy_threshold": "fuzzy",
    "min_scale_gain": "min_scale_gain",
    "pass_rate": "pass_rate",
    "pass_rate_min": "pass_rate_min",
    "duplicate_rate_max": "duplicate_rate_max",
    "completeness_min": "completeness_min",
    "threshold": "threshold",
}

#: 本轮改前是 200 静默错答或 500、改后必须是 400 且判决点名旋钮的那几格。逐格点名，缺一格即红。
SILENT_CELLS_FIXED = (
    ("/api/dataset/sample", "ratio", "inf"),
    ("/api/dataset/sample", "ratio", "-1.0"),
    ("/api/quality/health-gate", "pass_rate_min", "5.0"),
    ("/api/quality/health-gate", "duplicate_rate_max", "5.0"),
    ("/api/quality/health-gate", "completeness_min", "-1.0"),
    ("/api/dataset/impact", "min_scale_gain", "nan"),
    ("/api/quality/outliers", "threshold", "nan"),
)

SHAPE_KEYS = ("nan", "inf", "5.0", "-1.0")
SHAPE_VALUES = {"nan": NAN, "inf": INF, "5.0": 5.0, "-1.0": -1.0}

ITEMS = [{"instruction": f"问句 {i} 号", "output": f"答句 {i} 号",
          "length": 8 + (i % 5), "quality_score": 0.5} for i in range(10)]


def _call_sites(rel_path, func_name):
    """AST 数某产品文件里对某个权威函数的调用点（不含 import 行与函数定义）"""
    tree = ast.parse((PRODUCT_ROOT / rel_path).read_text(encoding="utf-8"))
    hits = 0
    for node in ast.walk(tree):
        if (isinstance(node, ast.Call)
                and getattr(node.func, "id", None) == func_name):
            hits += 1
    return hits


def _write_datasets(tmp_path):
    """将三份数据集落到 pytest 的 tmp 目录（autouse 的 `allow_temp_data_roots` 已把它放进白名单）"""
    files = {}
    for name, payload in (("a.json", ITEMS), ("b.json", ITEMS[:6]),
                          ("empty.json", []), ("out.json", []), ("train.json", ITEMS),
                          ("test.json", ITEMS[:4])):
        path = Path(tmp_path) / name
        path.write_text(json.dumps(payload, ensure_ascii=False), encoding="utf-8")
        files[name.replace(".json", "")] = str(path)
    files["out_dir"] = str(Path(tmp_path) / "split_out")
    Path(files["out_dir"]).mkdir(exist_ok=True)
    return files


def _base_body(path, files):
    """每只端点的最小合法请求体（默认档），现量表在它之上注入坏值"""
    if path == "/api/dataset/sample":
        return {"input_file": files["a"], "output_file": files["out"]}
    if path == "/api/dataset/split":
        return {"input_file": files["a"], "output_dir": files["out_dir"]}
    if path == "/api/dataset/search":
        return {"input_file": files["a"], "query": "问句"}
    if path == "/api/dataset/impact":
        return {"before_file": files["a"], "after_file": files["b"]}
    if path == "/api/leakage/check":
        return {"train_file": files["train"], "test_file": files["test"]}
    return {"input_file": files["a"]}


def _post(client, path, payload):
    """按 `requests` 库的默认口径发体：``json.dumps`` 的 ``allow_nan`` 默认 True

    不能用 ``client.post(json=...)``——httpx 在客户端侧就用 ``allow_nan=False``，
    NaN 那一形连请求都发不出去，量到的会是客户端的拒绝而不是服务端的归宿。
    ``{"x": NaN}`` 是 Python 标准库认为合法、RFC 8259 认为非法的那一档输入，
    真实客户端（``requests``、``urllib`` + ``json.dumps``）默认就这么发。
    """
    body = json.dumps(payload).encode("utf-8")
    try:
        resp = client.post(path, content=body,
                           headers={"Content-Type": "application/json"})
    except Exception as exc:  # noqa: BLE001
        # 422 的渲染路径自己抛 ValueError：TestClient 把它升成异常原样上抛。
        if "Out of range float" in str(exc):
            return "CRASH", str(exc)
        raise
    return str(resp.status_code), resp.text


def _ratio_config(**kwargs):
    return SampleConfig(method=kwargs.pop("method", "random"), **kwargs)


class TestAuthorityRejectsNonFinite:
    """`require_ratio` 的有限性那一维：无穷不再靠闭区间顺带挡住"""

    def test_nan_is_rejected_and_names_the_knob(self):
        with pytest.raises(DataValidationError) as caught:
            require_ratio("ratio", NAN)
        assert "ratio" in str(caught.value)
        assert "NaN" in str(caught.value)

    def test_positive_infinity_is_rejected(self):
        with pytest.raises(DataValidationError):
            require_ratio("threshold", INF)

    def test_negative_infinity_is_rejected(self):
        with pytest.raises(DataValidationError):
            require_ratio("threshold", MINUS_INF)

    def test_infinity_rejected_even_when_the_bound_is_open(self):
        """这一格是本轮改权威的直接理由：开上界时旧写法对 inf 恒放行"""
        with pytest.raises(DataValidationError):
            require_ratio("min_scale_gain", INF, minimum=-1.0, maximum=INF)
        with pytest.raises(DataValidationError):
            require_ratio("min_scale_gain", MINUS_INF, minimum=-1.0, maximum=INF)

    def test_finite_values_pass_and_none_means_unset(self):
        assert require_ratio("ratio", 0.5) == 0.5
        assert require_ratio("ratio", 0.0) == 0.0
        assert require_ratio("ratio", 1.0) == 1.0
        assert require_ratio("ratio", None) is None
        assert require_ratio("gain", 9.0, minimum=-1.0, maximum=INF) == 9.0

    def test_bool_and_str_are_not_numbers(self):
        for bad in (True, False, "0.5"):
            with pytest.raises(DataValidationError) as caught:
                require_ratio("ratio", bad)
            assert "ratio" in str(caught.value)

    def test_finite_out_of_range_keeps_the_bound_wording(self):
        """有限越界那句判决一字未动 ⇒ 本轮只补有限性一维，没有顺手换文案"""
        with pytest.raises(DataValidationError,
                           match=r"必须是 0\.0 到 1\.0 之间的比例，当前是 2\.0"):
            require_ratio("ratio", 2.0)

    def test_nan_wording_is_unchanged_for_closed_bound_sites(self):
        """既有 NaN 判决走「必须是有限数值」这一支，但 NaN 字样与旋钮名都还在"""
        with pytest.raises(DataValidationError) as caught:
            require_ratio("quality.threshold", NAN)
        assert "quality.threshold" in str(caught.value)
        assert "NaN" in str(caught.value)


class TestSampleRatioSink:
    """`DatasetOperations.sample` 的 ratio：与同函数两行之上的 require_count 同一待遇"""

    def setup_method(self):
        self.ops = DatasetOperations()

    def test_ratio_above_one_is_rejected(self):
        for bad in (2.0, 1.5):
            with pytest.raises(DataValidationError, match="ratio"):
                self.ops.sample(ITEMS, _ratio_config(ratio=bad))

    def test_negative_ratio_is_rejected(self):
        """改前 systematic 把 -0.5 折成 `items[:-5]`（5 条）、stratified 折成 0 条，两支都 200"""
        for method in ("random", "systematic", "stratified"):
            for bad in (-0.5, -2.0):
                with pytest.raises(DataValidationError, match="ratio"):
                    self.ops.sample(ITEMS, _ratio_config(method=method, ratio=bad))

    def test_non_finite_ratio_is_rejected(self):
        for bad in (NAN, INF, MINUS_INF):
            with pytest.raises(DataValidationError, match="ratio"):
                self.ops.sample(ITEMS, _ratio_config(ratio=bad))

    def test_bool_ratio_is_rejected(self):
        """改前 `ratio=True` 被当成 1.0 交出全量（实测 10 条 200）"""
        with pytest.raises(DataValidationError, match="ratio"):
            self.ops.sample(ITEMS, _ratio_config(ratio=True))

    def test_zero_ratio_still_means_zero_rows(self):
        """L149 / B219 那一格不回退：0 是「一条都不要」，不是「没传」"""
        assert self.ops.sample(ITEMS, _ratio_config(method="systematic", ratio=0.0)) == []

    def test_in_range_ratio_still_samples(self):
        got = self.ops.sample(ITEMS, _ratio_config(method="systematic", ratio=0.5))
        assert len(got) == 5

    def test_size_knob_unchanged(self):
        assert self.ops.sample(ITEMS, _ratio_config(size=0)) == []
        assert len(self.ops.sample(ITEMS, _ratio_config(size=3))) == 3
        with pytest.raises(DataValidationError, match="size"):
            self.ops.sample(ITEMS, _ratio_config(size=-5))


class TestGateThresholdSinks:
    """`gate_dataset_health` 的三只阈值：门口判了 `pass_rate`，三兄弟此前无人判"""

    def setup_method(self):
        self.weights = [0.25, 0.25, 0.25, 0.25]

    def _gate(self, **kwargs):
        return gate_dataset_health(ITEMS, text_field="instruction",
                                   weights=self.weights, pass_rate=0.9, **kwargs)

    def test_three_thresholds_reject_non_finite(self):
        for key in ("pass_rate_min", "duplicate_rate_max", "completeness_min"):
            for bad in (NAN, INF, MINUS_INF):
                with pytest.raises(DataValidationError, match=key):
                    self._gate(**{key: bad})

    def test_three_thresholds_reject_out_of_range(self):
        for key in ("pass_rate_min", "duplicate_rate_max", "completeness_min"):
            for bad in (5.0, -1.0):
                with pytest.raises(DataValidationError, match=key):
                    self._gate(**{key: bad})

    def test_defaults_still_reach_a_verdict(self):
        report = self._gate()
        assert report["gate"]["verdict"] in (
            GateVerdict.PASSED, GateVerdict.WARNED, GateVerdict.FAILED)

    def test_closed_interval_endpoints_are_legal(self):
        """0.0 与 1.0 是「一条都不许失」与「全部都要过」两档合法口径，且都真的生效

        `pass_rate_min=1.0` 配 `pass_rate=0.9` 必须判 **失败**：这一格证明「写进闭区间
        上端的 1.0」不是被当成没传而回落默认值——那正是 L149 / B219 那一族的缺陷形状。
        """
        assert self._gate(pass_rate_min=1.0)["gate"]["verdict"] == GateVerdict.FAILED
        assert self._gate(pass_rate_min=0.0)["gate"]["verdict"] == GateVerdict.PASSED
        assert self._gate(completeness_min=0.0)["gate"]["verdict"] == GateVerdict.PASSED


class TestImpactSink:
    """`is_beneficial` 的 min_scale_gain：定义域 [-1, +∞)，下界判死、上界开放"""

    def setup_method(self):
        self.evaluator = ImpactEvaluator()
        self.up = AugmentationImpact(before={}, after={},
                                     gains={"scale_gain": 1.0, "dedup_gain": 0.0})

    def _verdict_marker(self, value):
        """改前的形状：NaN 让 `<` 恒 False ⇒ 无声 true。这里直接读那一格返回值"""
        return self.evaluator.is_beneficial(self.up, min_scale_gain=value)

    def test_nan_no_longer_silently_reports_beneficial(self):
        with pytest.raises(DataValidationError, match="min_scale_gain"):
            self._verdict_marker(NAN)

    def test_infinity_is_rejected(self):
        for bad in (INF, MINUS_INF):
            with pytest.raises(DataValidationError, match="min_scale_gain"):
                self._verdict_marker(bad)

    def test_upper_bound_stays_open(self):
        """上界开放是本格的定义域决定的：`scale_gain` 可以任意大，clamp 它是换语义"""
        assert self._verdict_marker(5.0) is False
        assert self._verdict_marker(0.5) is True

    def test_lower_bound_inclusive_and_below_rejected(self):
        assert self._verdict_marker(-1.0) is True
        with pytest.raises(DataValidationError, match="min_scale_gain"):
            self._verdict_marker(-2.0)

    def test_bool_rejected(self):
        """`True` 会被读成 1.0 ⇒ 门槛悄悄抬到「规模必须翻倍」"""
        with pytest.raises(DataValidationError, match="min_scale_gain"):
            self._verdict_marker(True)


class TestOutlierThresholdSink:
    """`OutlierDetector` 的 threshold：手抄的 `threshold <= 0` 换成同族唯一权威"""

    def test_nan_rejected(self):
        """改前 `NaN <= 0` 为假 ⇒ 构造成功，检测在 NaN 阈值上无声跑完并回 200"""
        with pytest.raises(DataValidationError, match="threshold"):
            OutlierDetector(threshold=NAN)

    def test_zero_and_negative_still_rejected(self):
        for bad in (0, -1.0):
            with pytest.raises(ValueError, match="threshold"):
                OutlierDetector(threshold=bad)

    def test_message_still_contains_the_knob_name(self):
        """`tests/unit/test_outlier.py` 用 `match="threshold"` 钉这一格，换权威不能把它削掉"""
        with pytest.raises(DataValidationError) as caught:
            OutlierDetector(threshold=0.0)
        assert "threshold" in str(caught.value)

    def test_positive_infinity_still_accepted_by_family_convention(self):
        """`require_seconds` / `require_positive` 的 docstring 明文写了「正无穷有界可接受」

        本轮不擅自改这一族的对外行为：它需要与 A280 同批拍，所以按观测行为钉住。
        """
        assert OutlierDetector(threshold=INF).threshold == INF

    def test_unknown_method_still_rejected(self):
        with pytest.raises(DataValidationError, match="不支持"):
            OutlierDetector(method="mad")


class TestHttpFace:
    """现量表：14 只旋钮 × 四形的 HTTP 归宿整表相等"""

    def setup_method(self):
        self.client = TestClient(app)

    def _files(self, tmp_path):
        return _write_datasets(tmp_path)

    def test_census_table_is_exact(self, tmp_path):
        files = self._files(tmp_path)
        seen = {}
        for (path, field), _ in CENSUS.items():
            base = _base_body(path, files)
            row = []
            for shape in SHAPE_KEYS:
                body = dict(base)
                body[field] = SHAPE_VALUES[shape]
                code, _ = _post(self.client, path, body)
                row.append(code)
            seen[(path, field)] = tuple(row)
        assert seen == CENSUS

    def test_fixed_cells_carry_a_verdict_naming_the_knob(self, tmp_path):
        files = self._files(tmp_path)
        for path, field, shape in SILENT_CELLS_FIXED:
            body = _base_body(path, files)
            body[field] = SHAPE_VALUES[shape]
            code, text = _post(self.client, path, body)
            assert code == "400", (path, field, shape, text[:120])
            assert VERDICT_MARKERS[field] in text, (path, field, text[:120])

    def test_ratio_shapes_never_return_200(self, tmp_path):
        """改前 systematic / stratified 两支对越界 ratio 一律 200 静默错答"""
        files = self._files(tmp_path)
        for method in ("random", "systematic", "stratified"):
            for shape in SHAPE_KEYS:
                body = _base_body("/api/dataset/sample", files)
                body["method"] = method
                body["ratio"] = SHAPE_VALUES[shape]
                code, text = _post(self.client, "/api/dataset/sample", body)
                assert code == "400", (method, shape, text[:120])

    def test_a280_cells_stay_pinned_as_internal_error(self, tmp_path):
        """`raise_internal_error` 那一族：SDK 有判决但门口把它降成 500（**待拍，本轮不动**）"""
        files = self._files(tmp_path)
        for path in ("/api/quality/dedup", "/api/quality/benchmark"):
            for shape in SHAPE_KEYS:
                body = _base_body(path, files)
                body["threshold"] = SHAPE_VALUES[shape]
                code, text = _post(self.client, path, body)
                assert code == "500", (path, shape, text[:120])
                assert INTERNAL_ERROR_DETAIL in text
                assert "DataValidationError" not in text

    def test_a311_new_cell_render_crash_stays_pinned(self, tmp_path):
        """422 的渲染路径遇到 NaN：整个响应没了（**A311 的新现量，本轮只量不动**）"""
        files = self._files(tmp_path)
        body = _base_body("/api/leakage/check", files)
        body["fuzzy_threshold"] = NAN
        code, text = _post(self.client, "/api/leakage/check", body)
        assert code == "CRASH"
        assert "Out of range float" in text
        # 同一路径的有限越界仍能正常回 422 —— 炸的只是非有限那一形，不是整条端点
        body["fuzzy_threshold"] = 5.0
        assert _post(self.client, "/api/leakage/check", body)[0] == "422"

    def test_finite_in_range_bodies_still_succeed(self, tmp_path):
        """默认档零漂移：好请求仍然 200，本轮的判据没有把合法域收窄"""
        files = self._files(tmp_path)
        checks = (
            ("/api/dataset/sample", {"ratio": 0.5}),
            ("/api/dataset/impact", {"min_scale_gain": 0.1}),
            ("/api/quality/outliers", {"threshold": 2.5}),
            ("/api/quality/health-gate", {"pass_rate_min": 0.7}),
            ("/api/quality/dedup", {}),
        )
        for path, extra in checks:
            body = _base_body(path, files)
            body.update(extra)
            code, text = _post(self.client, path, body)
            assert code == "200", (path, text[:160])


class TestWiringRoster:
    """接线名册：四只咽喉的调用点按文件计数恰好相等"""

    def test_the_roster_is_exact(self):
        seen = {}
        for (rel, func) in WIRING_ROSTER:
            seen[(rel, func)] = _call_sites(rel, func)
        assert seen == WIRING_ROSTER

    def test_product_root_files_exist(self):
        for rel in {rel for (rel, _) in WIRING_ROSTER}:
            assert (PRODUCT_ROOT / rel).is_file(), rel


class TestRulerBites:
    """注入臂：逐格削弱一处接线，当场证明对应的钉子会喊红（全部零写盘）"""

    def test_arm_passthrough_ratio_breaks_the_sample_sink(self, monkeypatch):
        """削掉这一处接线 ⇒ 退回改前的三种后果，`DataValidationError` 一个都不出现"""
        import augmentor.dataset_ops as mod

        monkeypatch.setattr(mod, "require_ratio",
                            lambda name, value, minimum=0.0, maximum=1.0: value)
        ops = DatasetOperations()
        # 改前实测：inf 走 `int(10 * inf)` → OverflowError，它不是 ValueError 子类 ⇒ 端点落 500
        with pytest.raises(OverflowError):
            ops.sample(ITEMS, _ratio_config(ratio=INF))
        # 改前实测：True 被当成 1.0 ⇒ 无声交出全量 10 条
        assert len(ops.sample(ITEMS, _ratio_config(ratio=True))) == 10

    def test_arm_passthrough_ratio_breaks_the_gate_sink(self, monkeypatch):
        """削掉权威 ⇒ 门禁不再拒绝 NaN 阈值，而是拿它算出一份判决（这就是本轮要堵的形状）"""
        import augmentor.validation as mod

        monkeypatch.setattr(mod, "require_ratio",
                            lambda name, value, minimum=0.0, maximum=1.0: value)
        report = gate_dataset_health(ITEMS, text_field="instruction",
                                     weights=[0.25, 0.25, 0.25, 0.25], pass_rate=0.9,
                                     completeness_min=NAN)
        assert "completeness" in report["gate"]["warned_rules"]
        assert report["gate"]["verdict"] == GateVerdict.WARNED

    def test_arm_passthrough_ratio_breaks_the_impact_sink(self, monkeypatch):
        import augmentor.validation as mod

        monkeypatch.setattr(mod, "require_ratio",
                            lambda name, value, minimum=0.0, maximum=1.0: value)
        impact = AugmentationImpact(before={}, after={},
                                    gains={"scale_gain": 1.0, "dedup_gain": 0.0})
        assert ImpactEvaluator().is_beneficial(impact, min_scale_gain=NAN) is True

    def test_arm_old_isnan_only_authority_lets_infinity_through(self, monkeypatch):
        """把权威退回改前那两行（只判 NaN），开上界的旋钮立刻放行 inf"""
        import augmentor.validation as mod

        def _old(name, value, minimum=0.0, maximum=1.0):
            if value is None:
                return None
            if isinstance(value, bool) or not isinstance(value, (int, float)):
                raise DataValidationError(f"{name} 必须是数值")
            if math.isnan(value):
                raise DataValidationError(f"{name} 不能是 NaN")
            if not minimum <= value <= maximum:
                raise DataValidationError(f"{name} 越界")
            return value

        monkeypatch.setattr(mod, "require_ratio", _old)
        impact = AugmentationImpact(before={}, after={},
                                    gains={"scale_gain": 1.0, "dedup_gain": 0.0})
        assert ImpactEvaluator().is_beneficial(impact, min_scale_gain=INF) is False

    def test_arm_passthrough_positive_breaks_the_outlier_sink(self, monkeypatch):
        import augmentor.outlier as mod

        monkeypatch.setattr(mod, "require_positive", lambda name, value: value)
        assert OutlierDetector(threshold=NAN).threshold is NAN

    def test_baseline_after_arms_is_still_wired(self):
        """四支削弱臂都由 monkeypatch 自动还原；这里复验还原后的磁盘态判据仍在"""
        for bad in (NAN, INF):
            with pytest.raises(DataValidationError):
                require_ratio("min_scale_gain", bad, minimum=-1.0, maximum=INF)
        with pytest.raises(DataValidationError):
            OutlierDetector(threshold=NAN)
