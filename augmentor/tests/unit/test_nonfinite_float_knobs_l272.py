# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L272 守卫：非有限浮点（NaN / 无穷）在请求侧 float 旋钮族没有判据（**L277 起 `sample.ratio` 连「范围」那一维也判死**）

三件事，各自都要能单独喊红：

1. **权威维**：``validation.require_ratio`` 现在逐字判「是不是有限数值」，而不只判 NaN。
   改前无穷是靠闭区间**顺带**挡住的，所以这一族一旦出现「下界判死、上界开放」的旋钮
   （``min_scale_gain`` 的定义域是 [-1, +∞)），``float('inf')`` 就能穿过
   ``not minimum <= value <= maximum`` 一路走进比较式。
   **L277 起还多判一维**：NaN / 有限性那两判加了 ``isinstance(value, float)`` 前缀 ⇒
   超大整数（``10**400``）不再在 ``math.isfinite`` 上抛裸 ``OverflowError``（经 API 就是 500），
   而是走对 int 精确的那句区间判决。
2. **落点维**：四只 SDK 咽喉共引同一份权威——``DatasetOperations.sample`` 的 ``ratio``、
   ``gate_dataset_health`` 的三只阈值、``ImpactEvaluator.is_beneficial`` 的
   ``min_scale_gain``、``OutlierDetector`` 的 ``threshold``。判在 SDK 而不是判在端点，
   是因为 CLI 与 API 共用这几层（``quality_gate`` 自己的 docstring 就把这句话写成原则）。
3. **现量维**：14 只 float 旋钮 × 四形（NaN / 无穷 / 5.0 / -1.0）的 HTTP 归宿**整表**钉死。
   其中 ``raise_internal_error`` 那一族（**A280**，待拍）按**观测行为**钉住，不是假装它合法——
   谁改这一族，这张表当场红。**422 渲染崩溃那一族（原为 A311 的新现量）已由 L284 收口**：
   `api/main.py` 文末先走 FastAPI 默认的校验失败处理器，渲染抛 ValueError 才把非有限浮点换成
   同名**字符串** token 重渲染 ⇒ 表里那一格从 `CRASH` 翻成 `422`，判据也同步从「洞的快照」
   翻成正判决，并由同一支里的反向臂钉住「摘掉兜底立刻回到 CRASH」。
   **A311 的本体没有收**——「缺必填格走英文 422、其余坏参数走中文 4xx」那一族双轨仍在账上待拍。

**「范围」那一维在 L272 与 L277 两轮的边界不同，必须分开读**：

* **L272（有限性那一维）**：四只咽喉调用权威时把闭区间推到两端（`-float("inf")` /
  `float("inf")`），所以**没有任何一个有限的合法值改了自己的归宿**。当时把「无上界判决」
  这个洞的形状当成契约钉住的三支（本文件的两支 + l230 那一支）是**洞的快照**。
* **L277（范围那一维，只收 `SampleConfig.ratio`）**：``dataset_ops.py`` 把推到两端的实参
  摘掉，改用 ``require_ratio("ratio", config.ratio)`` 的默认 0-1 界 ⇒ `ratio=2.0` / `1.5` /
  `-0.5` / `-1.0` / `-2.0` 在**三种 `method` 上统一**回 `DataValidationError`，端点回 400 且
  detail 点名 `ratio`；改前那三格「同一越界比例三种结局」（random 裸 `ValueError` 的 CPython
  英文原话 / systematic 与 stratified 静默交全量或负数切片交前 N 条）从此不存在。
  判据因此从「反向证据」翻成**更强的正向判决**：既不许 200 静默交产物，也不许退回裸
  `ValueError`（那在 SDK 面就是把调用方踢回 CPython 的英文原话）。
* **另四只旋钮（三只阈值 + `min_scale_gain`）的范围那一维本轮一格未动** —— 仍按改前形状钉住
  （本文件的 `TestGateThresholdSinks.test_three_thresholds_range_face_is_untouched` 与
  `TestImpactSink.test_upper_bound_stays_open` / `test_below_the_semantic_floor_is_still_legal`
  三支，另有 `tests/unit/test_health_gate_wiring_l235.py` 那三支判据单调性），账上归 **A354** 待拍。
  `quality_gate.py` 与 `impact.py` 的站点**必须**继续把界推到两端，这一条由
  `TestOpenBoundRoster` 反向钉住：谁替用户拍了 A354，那一支当场红。

改前的三格实测（本轮沙箱复现，逐格见循环日志 L272）：``ratio=inf`` 经
``int(len * inf)`` 抛 ``OverflowError`` 而它**不是** ``ValueError`` 子类 ⇒ ``to_http_error``
落到 500 档；``ratio=-0.5`` 在 systematic 支折成 ``items[:-5]``（「丢最后 5 条」）而
stratified 支折成 0 条，两支都回 **200**；``min_scale_gain=NaN`` 让 ``scale_gain < NaN``
恒 False ⇒ ``beneficial`` 无声判成 true。**L277 逐格复量**：前两格已在 SDK 门口判死
（第三格属于 A354，仍按观测行为钉住），复现途径只剩「把权威摘掉」——那正是
`TestRulerBites` 那几支注入臂的形状。
"""

import ast
import json
import math
from pathlib import Path

import pytest
from fastapi.exception_handlers import request_validation_exception_handler
from fastapi.exceptions import RequestValidationError
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

#: 每一只 `require_ratio` 咽喉**推到 ±inf 的界实参数**：`(文件, 函数) -> 个数`。
#: L277 只把 `dataset_ops.py` 那一格收回默认 0-1（⇒ 0），另外两站仍必须把界推到两端
#: （范围那一维属 A354 待拍）。写成集合相等而不是 `>=`：这一格同时是双向尺 ——
#: 有人把 A354 顺手拍了（quality_gate / impact 收回实界）或者把 sample 那一格又推回
#: ±inf（洞复活），两种都当场红。它同时是 `TestOpenBoundRoster` 的正控制：识别函数
#: 若瞎掉，`0` 那一格会跟着变成恒真，`6` / `2` 两格立刻兜住它。
OPEN_BOUND_ROSTER = {
    ("dataset_ops.py", "require_ratio"): 0,
    ("quality_gate.py", "require_ratio"): 6,
    ("impact.py", "require_ratio"): 2,
}

#: 现量表：`(路径, 旋钮) -> (NaN, 无穷, 5.0, -1.0)` 四档的 HTTP 归宿。
#: `CRASH` 是「服务端渲染 422 时自己抛 ValueError」——TestClient 把它升成异常，
#: 生产部署（uvicorn + ServerErrorMiddleware）的对应物是「没有 JSON 响应体」。
#: **L284 起全表没有 `CRASH` 这一档了**：`api/main.py` 文末的有限值兜底把最后一格
#: （`/api/leakage/check` 的 `fuzzy_threshold`，非有限两形）翻成 422 + 字符串 token。
#: `_post` 里认 `CRASH` 的那一支**刻意保留**——它现在是反向尺的落点：谁摘掉兜底，
#: 这一格当场以 `CRASH` 的形状回到表里，而不是伪装成一个静默的状态码。
#: **L277 起第一行四格的成因变了**：`5.0` / `-1.0` 此前只是恰好被 random 支的裸
#: `ValueError` 折成 400，现在是 `require_ratio` 的区间判决（见
#: `test_ratio_range_face_is_a_verdict_on_all_three_methods`，它把 detail 文案也钉住）。
CENSUS = {
    ("/api/dataset/sample", "ratio"): ("400", "400", "400", "400"),
    ("/api/dataset/split", "train_ratio"): ("400", "400", "400", "400"),
    ("/api/dataset/split", "val_ratio"): ("400", "400", "400", "400"),
    ("/api/dataset/split", "test_ratio"): ("400", "400", "400", "400"),
    ("/api/dataset/search", "fuzzy_threshold"): ("400", "400", "400", "400"),
    ("/api/dataset/impact", "min_scale_gain"): ("400", "400", "200", "200"),
    ("/api/quality/health-gate", "pass_rate"): ("400", "400", "400", "400"),
    ("/api/quality/health-gate", "pass_rate_min"): ("400", "400", "200", "200"),
    ("/api/quality/health-gate", "duplicate_rate_max"): ("400", "400", "200", "200"),
    ("/api/quality/health-gate", "completeness_min"): ("400", "400", "200", "200"),
    ("/api/quality/outliers", "threshold"): ("400", "200", "200", "400"),
    ("/api/quality/dedup", "threshold"): ("500", "500", "500", "500"),
    ("/api/quality/benchmark", "threshold"): ("500", "500", "500", "500"),
    ("/api/leakage/check", "fuzzy_threshold"): ("422", "422", "422", "422"),
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
#: **L272 那一批只有非有限那一形在这一列里**；**L277 把 `sample.ratio` 的有限越界两格也加了进来**
#: （改前 systematic / stratified 两支对 `5.0` 与 `-1.0` 都静默回 200 交产物）。
#: 其余旋钮的有限越界一格未动（A354 待拍），由 `TestGateThresholdSinks` 与 `TestImpactSink`
#: 那三支按改前形状钉住（落点逐支点名见模块 docstring 第 35-39 行）。
SILENT_CELLS_FIXED = (
    ("/api/dataset/sample", "ratio", "inf"),
    ("/api/dataset/sample", "ratio", "5.0"),
    ("/api/dataset/sample", "ratio", "-1.0"),
    ("/api/dataset/impact", "min_scale_gain", "nan"),
    ("/api/quality/outliers", "threshold", "nan"),
)

#: `sample.ratio` 的范围那一维（L277 收口）：判决必须是**领域异常 + 点名旋钮 + 中文区间原话**。
#: `CPYTHON_ENGLISH` 是改前 random 支交给调用方的那句 CPython 英文原话——它在 SDK 面是裸
#: `ValueError`（`to_http_error` 只能按 `ValueError` 泛档折成 400，detail 里没有旋钮名），
#: 在端点面是一句用户读不懂的英文 ⇒ 两样都算「不可行动」，L277 起一概不许再出现。
RATIO_RANGE_CELLS = (2.0, 1.5, -0.5, -1.0, -2.0, 2, -1)
CPYTHON_ENGLISH = "Sample larger than population or is negative"
RATIO_RANGE_VERDICT = "必须是 0.0 到 1.0 之间的比例"

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


def _is_inf_literal(node):
    """认 `-float("inf")` / `float("inf")` / `float('inf')` / `float('Infinity')` 四种写法

    只认「字面量喂给 `float()`」这一族形状，不认变量：本判据问的是**这一站的界是谁给的**，
    写成变量就出了这一族的射程（那种改动要另立判据，不该由这一格默默放行）。
    """
    if isinstance(node, ast.UnaryOp) and isinstance(node.op, ast.USub):
        node = node.operand
    return (isinstance(node, ast.Call)
            and getattr(node.func, "id", None) == "float"
            and len(node.args) == 1
            and getattr(node.args[0], "value", None) in ("inf", "Infinity"))


def _open_bound_args(rel_path, func_name):
    """数某产品文件里 `require_ratio(...)` 被推到 ±inf 的**界实参**个数（含位置与关键字两种写法）"""
    tree = ast.parse((PRODUCT_ROOT / rel_path).read_text(encoding="utf-8"))
    hits = 0
    for node in ast.walk(tree):
        if not (isinstance(node, ast.Call)
                and getattr(node.func, "id", None) == func_name):
            continue
        values = list(node.args[2:]) + [kw.value for kw in node.keywords]
        hits += sum(1 for v in values if _is_inf_literal(v))
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
        """有限越界那句判决一字未动 ⇒ 补有限性一维时没有顺手换文案

        **L277 起这一句从「反向证据」升成 `sample.ratio` 的正判据**：`dataset_ops.py` 用默认
        0-1 界调权威，所以 `2.0` 经端点就是这句中文判决（旧形状是 CPython 的英文原话，
        见 `CPYTHON_ENGLISH`）。
        """
        with pytest.raises(DataValidationError,
                           match=r"必须是 0\.0 到 1\.0 之间的比例，当前是 2\.0"):
            require_ratio("ratio", 2.0)

    def test_huge_int_is_judged_by_the_interval_instead_of_raising_overflow(self):
        """L277 新增那一维：有限性判据加了 `isinstance(value, float)` 前缀 ⇒ 超大整数走精确区间比较

        改前 `math.isfinite(10**400)` 抛的是裸 `OverflowError`（它**不是** `ValueError` 子类
        ⇒ `to_http_error` 只能落到 500 档，客户端拿到 `INTERNAL_ERROR_DETAIL` 而看不到自己
        写错的那一格）。整数按定义不是 NaN/无穷，而 `minimum <= value <= maximum` 对 int 是
        精确的，所以正确结局是那句中文区间判决。判据写成「必须 `DataValidationError`」而不是
        「不许 `OverflowError`」：后者对「干脆放行超大整数」这种更坏的修法也是绿的。
        """
        for bad in (10 ** 400, -(10 ** 400)):
            with pytest.raises(DataValidationError) as caught:
                require_ratio("ratio", bad)
            assert RATIO_RANGE_VERDICT in str(caught.value), str(caught.value)
            assert "ratio" in str(caught.value)

    def test_nan_wording_is_unchanged_for_closed_bound_sites(self):
        """既有 NaN 判决走「必须是有限数值」这一支，但 NaN 字样与旋钮名都还在"""
        with pytest.raises(DataValidationError) as caught:
            require_ratio("quality.threshold", NAN)
        assert "quality.threshold" in str(caught.value)
        assert "NaN" in str(caught.value)


class TestSampleRatioSink:
    """`DatasetOperations.sample` 的 ratio：L277 起与同函数两行之上的 require_count 同等待遇（两界都判）"""

    def setup_method(self):
        self.ops = DatasetOperations()

    def test_ratio_range_face_is_a_verdict_on_all_three_methods(self):
        """L277 起：`sample.ratio` 的范围那一维判在 SDK 门口，三种 method 一种结局（A292 的这一半已收）

        旧读数（L272 那批按「洞的形状」钉过、本轮按新契约翻面，10 条语料实测）：
        `2.0` / `1.5` ⇒ random 抛 CPython 的英文裸 `ValueError`（**不是** `DataValidationError`）、
        systematic 与 stratified 各静默交整份 10 条；`-0.5` ⇒ systematic 折成前 5 条、
        stratified 交 0 条；`-1.0` / `-2.0` ⇒ 另两支各交 0 条。新契约三格都要验，缺一格即红：
        ① 异常类型必须是领域异常（回退成裸 `ValueError` 就红——那经端点是一句用户读不懂的英文），
        ② 判决必须点名 `ratio` 并带区间原话，③ 一条产物都不许交（静默交产物是改前最坏那两形）。
        整数档（`2` / `-1`）同判：`require_ratio` 的有限性判据加了 `isinstance(value, float)`
        前缀之后，区间比较对 int 是精确的，既不 `OverflowError` 也不放行。
        """
        for method in ("random", "systematic", "stratified"):
            for bad in RATIO_RANGE_CELLS:
                with pytest.raises(DataValidationError) as caught:
                    self.ops.sample(ITEMS, _ratio_config(method=method, ratio=bad))
                text = str(caught.value)
                assert "ratio" in text, (method, bad, text)
                assert RATIO_RANGE_VERDICT in text, (method, bad, text)
                assert CPYTHON_ENGLISH not in text, (method, bad, text)

    def test_out_of_range_ratio_is_judged_at_the_door_not_in_the_dispatch(self, monkeypatch):
        """A292 里最贵的那一格（`-0.5` 在 systematic 支折成 `items[:-5]` 交**前 5 条**）判在门口

        只断言「抛了异常」不够：把判据从 `sample` 门口挪进分发之后，负数切片那一族在
        `stratified` 支仍然能被走到。所以这里把 `_stratified_sample` 换成「一被叫到就红」的哨兵，
        证明判决发生在任何分发之前。改前读数由 `TestRulerBites` 的直通臂现场保留。
        """
        def _never_called(*args, **kwargs):
            raise AssertionError("判决没判在门口：分发被叫到了，负数切片那一族还活着")

        monkeypatch.setattr(DatasetOperations, "_stratified_sample", _never_called)
        for method, bad in (("systematic", -0.5), ("stratified", -0.5),
                            ("systematic", 2.0), ("stratified", 1.5)):
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

    def test_three_thresholds_range_face_is_untouched(self):
        """有限越界本轮一格不收紧：`5.0` / `-1.0` / `99.0` 照旧出一份判决（**A354 待拍**）

        L235 那把尺子逐格量过「判决随阈值单调变化」，收紧范围等于凭空造一条后端
        原本没有的规则并把 200 改成 400 —— 那是对外行为变更，要拍板才动。
        """
        for key, legal in (("pass_rate_min", 5.0), ("pass_rate_min", -1.0),
                           ("duplicate_rate_max", 99.0), ("completeness_min", -3.0),
                           ("completeness_min", 5.0)):
            report = self._gate(**{key: legal})
            assert "gate" in report, (key, legal)

    def test_defaults_still_reach_a_verdict(self):
        report = self._gate()
        assert report["gate"]["verdict"] in (
            GateVerdict.PASSED, GateVerdict.WARNED, GateVerdict.FAILED)

    def test_closed_interval_endpoints_are_legal(self):
        """0.0 与 1.0 是「一条都不许失」与「全部都要过」两档合法口径，且都真的生效

        `pass_rate_min=1.0` 配 `pass_rate=0.9` 必须判 **失败**：这一格证明「送到规则上端的
        1.0」不是被当成没传而回落默认值——那正是 L149 / B219 那一族的缺陷形状。
        """
        assert self._gate(pass_rate_min=1.0)["gate"]["verdict"] == GateVerdict.FAILED
        assert self._gate(pass_rate_min=0.0)["gate"]["verdict"] == GateVerdict.PASSED
        assert self._gate(completeness_min=0.0)["gate"]["verdict"] == GateVerdict.PASSED


class TestImpactSink:
    """`is_beneficial` 的 min_scale_gain：本轮只判有限性，两端都留开（范围那一维属 A354）"""

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

    def test_below_the_semantic_floor_is_still_legal(self):
        """`-1.0` 是 `scale_gain` 的值域下界，但比它更低的门槛**此前就收**：判死它是对外行为变更

        门槛设在值域之下等于「永远放行」，那是一种用法而不是一个笔误 ⇒ A354 待拍。
        """
        assert self._verdict_marker(-1.0) is True
        assert self._verdict_marker(-2.0) is True

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

    def test_rejected_ratio_never_returns_200_on_any_method(self, tmp_path):
        """L272 收的那一半（NaN / 无穷）+ L277 收的另一半（有限越界）：四形 × 三种 `method` 一律 400

        改前两格各有归宿：`nan` / `inf` 在 L272 之前一路走进比较式（`inf` 抛裸
        `OverflowError` ⇒ 500），`5.0` / `-1.0` 在 L277 之前只有 random 支被 CPython 的
        `ValueError` 顺带折成 400，另两支静默回 200 交产物。现在四形在三只 method 上
        都必须是「400 + 点名 ratio 的可行动判决」，一支都不许无声交产物。
        """
        files = self._files(tmp_path)
        for method in ("random", "systematic", "stratified"):
            for shape in SHAPE_KEYS:
                body = _base_body("/api/dataset/sample", files)
                body["method"] = method
                body["ratio"] = SHAPE_VALUES[shape]
                code, text = _post(self.client, "/api/dataset/sample", body)
                assert code == "400", (method, shape, text[:120])
                assert "ratio" in text, (method, shape, text[:120])
                assert CPYTHON_ENGLISH not in text, (method, shape, text[:120])

    def test_ratio_range_face_is_a_verdict_on_all_three_methods(self, tmp_path):
        """L277 翻面的那一支（旧名 `..._is_untouched_...`）：有限越界从此三种 method 一种结局

        旧矩阵（L272 按洞的形状钉的**反向**证据，10 条语料实测）：`5.0` ⇒ random 400 且
        detail 是 CPython 的英文原话、systematic / stratified 各回 200 交整份 10 条；
        `-1.0` ⇒ random 400、另两支各回 200 交 0 条。新契约逐格改成 400，且判据比「状态码
        等于 400」更强，三条都在这一支里：
        ① 状态码恰好 400（不许回退成 200 静默产物，也不许被裸异常折成 500）；
        ② detail **整句**等于那句中文区间判决（回退成 CPython 英文原话虽然也是 400，
           但对客户端不可行动 —— 这一格就是 A292 立账时点名的那句英文）；
        ③ 落盘产物一格未动：`output_file` 指向的文件在整张矩阵跑完之后仍是 `[]`，
           证明 400 之前没有任何一条路径写过产物。
        """
        files = self._files(tmp_path)
        matrix = {
            "random": {"5.0": 5.0, "-1.0": -1.0, "2.0": 2.0},
            "systematic": {"5.0": 5.0, "-1.0": -1.0, "2.0": 2.0},
            "stratified": {"5.0": 5.0, "-1.0": -1.0, "2.0": 2.0},
        }
        for method, shapes in matrix.items():
            for shape, value in shapes.items():
                body = _base_body("/api/dataset/sample", files)
                body["method"] = method
                body["ratio"] = value
                code, text = _post(self.client, "/api/dataset/sample", body)
                assert code == "400", (method, shape, text[:160])
                assert json.loads(text)["detail"] == (
                    f"ratio 必须是 0.0 到 1.0 之间的比例，当前是 {value}"), (method, shape, text)
                assert CPYTHON_ENGLISH not in text, (method, shape, text[:160])
        assert json.loads(Path(files["out"]).read_text(encoding="utf-8")) == []

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
        """422 渲染崩溃那一格 L284 已收口：现量钉「422 + 同名 token」，反向臂钉「摘掉就回 CRASH」

        改前（L272 现量）：`fuzzy_threshold=NaN` 先被模型字段的 `le=1.0` 判错，而 pydantic v2
        每条错误项都带 `input`（回显**解析后的请求体**），starlette 渲染钉 `allow_nan=False`
        ⇒ 错误回执自己在渲染阶段抛 `ValueError` ⇒ 整个响应没了。L284 在 `api/main.py` 文末兜
        了一层，本格的现量（逐格见循环日志 L284）翻成四形全 422，非有限值在 `input` 那一格换成
        `"NaN"` / `"Infinity"` / `"-Infinity"` 三个字符串 token，而 `type` / `msg` / `ctx`
        一字未动 ⇒ 兜底只换了渲染不了的字节，没有换判决。

        判据因此从「洞的快照」升成正判决，但仍留一条反向臂压着：**把处理器换回 FastAPI 默认
        那支**，同一个请求必须当场回到 CRASH。少了这条臂，这一格会退化成「表里写了 422 所以
        永远绿」——那正是 A311 立账时点名的那种不可失败。A311 本体（英文 422 与中文 4xx 双轨）
        不在这一格的射程里，仍待拍。
        """
        files = self._files(tmp_path)
        body = _base_body("/api/leakage/check", files)
        body["fuzzy_threshold"] = NAN
        code, text = _post(self.client, "/api/leakage/check", body)
        assert code == "422", text[:200]
        item = json.loads(text)["detail"][0]
        assert (item["type"], item["ctx"]) == ("less_than_equal", {"le": 1.0}), item
        assert item["input"] == "NaN", item

        for value, echoed in ((INF, "Infinity"), (5.0, 5.0), (-1.0, -1.0)):
            body["fuzzy_threshold"] = value
            got = json.loads(_post(self.client, "/api/leakage/check", body)[1])
            assert got["detail"][0]["input"] == echoed, (value, got)

        saved = app.exception_handlers[RequestValidationError]
        app.exception_handlers[RequestValidationError] = (
            request_validation_exception_handler
        )
        app.middleware_stack = None
        try:
            body["fuzzy_threshold"] = NAN
            code, text = _post(self.client, "/api/leakage/check", body)
            assert code == "CRASH", text[:120]
            assert "Out of range float" in text
        finally:
            app.exception_handlers[RequestValidationError] = saved
            app.middleware_stack = None
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


class TestOpenBoundRoster:
    """开界名册：`require_ratio` 的三站里只有 `sample.ratio` 把界收回默认 0-1

    这一族是**反向尺**，专门盯住「范围那一维」的两类越权改动：

    * 有人替用户拍了 **A354**（把三只阈值 + `min_scale_gain` 的界也收回实界）⇒
      `6` / `2` 两格当场红。那是对外行为变更（把 200 的合法请求改成 400），必须先拍板。
    * 有人把 `sample.ratio` 那一格又推回 ±inf（L277 收的口被回退，A292 的「无上界判决」
      那一半洞复活）⇒ `0` 那一格当场红。

    判据用 AST 而不是 grep 字符串：它问的是**这一站传给权威的界实参长什么样**，
    注释里提一句 `float("inf")` 不该被算成开界，改成变量名传递则属于「另一族判据」。
    `0` 那一格同时是正控制：若识别函数瞎掉（永远返回 0），`6` / `2` 两格立刻兜住它。
    """

    def test_the_open_bound_roster_is_exact(self):
        seen = {}
        for (rel, func) in OPEN_BOUND_ROSTER:
            seen[(rel, func)] = _open_bound_args(rel, func)
        assert seen == OPEN_BOUND_ROSTER

    def test_sample_ratio_site_passes_no_bound_args(self):
        """单点复验：`dataset_ops.py` 那句必须是两参调用，界由权威的默认值给"""
        assert _open_bound_args("dataset_ops.py", "require_ratio") == 0

    def test_a354_sites_still_hand_the_bound_to_both_infinities(self):
        """A354 未拍 ⇒ 这两站的界**必须**仍是 ±inf：6 = 3 只阈值 × 2 端，2 = 1 只 × 2 端"""
        assert _open_bound_args("quality_gate.py", "require_ratio") == 6
        assert _open_bound_args("impact.py", "require_ratio") == 2


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
        # 改前实测（A292 那三格的范围维）：-0.5 在 systematic 支折成 `items[:-5]` = **前 5 条**，
        # 2.0 在 systematic 支折成 `items[::1][:20]` = **整份 10 条**。两格都是「无声交出错的产物」，
        # 一个异常都不抛 ⇒ 本文件对 `sample.ratio` 的正判据（见 TestSampleRatioSink）全靠这两格撑起来。
        got = ops.sample(ITEMS, _ratio_config(method="systematic", ratio=-0.5))
        assert [row["instruction"] for row in got] == [
            row["instruction"] for row in ITEMS[:5]]
        assert len(ops.sample(ITEMS, _ratio_config(method="systematic", ratio=2.0))) == 10
        # 改前实测：同一格在 stratified 支是 0 条 —— 三种结局才是 A292 立的账，别只记一支
        assert ops.sample(ITEMS, _ratio_config(method="stratified", ratio=-0.5)) == []

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
        """五支削弱臂都由 monkeypatch 自动还原；这里复验还原后的磁盘态判据仍在"""
        for bad in (NAN, INF):
            with pytest.raises(DataValidationError):
                require_ratio("min_scale_gain", bad, minimum=-1.0, maximum=INF)
        with pytest.raises(DataValidationError):
            OutlierDetector(threshold=NAN)
        # L277 新增那一格也要复验：还原后超大整数仍是区间判决，不是 OverflowError
        with pytest.raises(DataValidationError, match=RATIO_RANGE_VERDICT):
            require_ratio("ratio", 10 ** 400)
