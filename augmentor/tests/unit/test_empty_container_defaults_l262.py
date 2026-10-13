# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

r"""L262 守卫：`x = x or DEFAULT` 把调用方**显式传的空容器**读成「没传」——普查钉格 + 可达面点名

**为什么立这一条（A343）**：同一族缺陷在本仓有一处**已经判过**的先例——`augmentor/privacy.py`
的 `PiiSanitizer` 在 L188 把三处写法改成 `x if x is not None else DEFAULT`，那一行注释写着
「显式空容器（[] / {}）是『零元素』的合法意图，不是『没传』；旧写法把空容器读成没传而悄悄套上
默认，脱敏范围/字段面被无声放大」。同一判决适用的其余格数此前**没有人量过**：本轮按语法形状
全仓普查（`*.py`，排除 `tests/` 与任何 `test_*.py`），现量 **38 格**，分成四族——

- **A 属性面吞非空容器（10 格）**：构造器里 `self.X = X or <非空容器>`，实例化时传 `[]` 与传
  `None` **完全同值**，由 `TestEmptyIsIndistinguishableFromUnset` 逐格实例化现场证。
- **B 方法体吞非空容器（7 格）**：同一形状出现在方法第一行，吞的是这一次调用的参数。
- **C 空默认幂等（10 格）**：`X or []` 的默认本身就是空容器 ⇒ `[] or []` 还是 `[]`，
  **不构成吞**（`test_class_C_is_provably_idempotent` 逐格验默认值是空字面量，不是靠人判断）。
- **D 非容器默认（11 格）**：默认是标量、对象或一次求值（`MergeConfig()`、`self.strategy`、
  `self._get_all_fields()`、`version_id`）。其中三格在语义上与 A/B 同族（空 ⇒ 换成非空默认），
  本轮**按语法族分开钉**而不是混成一谈：判据是形状，越界的归因会造出假缺陷。

**可达面才是缺陷分级**（`TestReachability` 全部用文本/形状对账，不猜）：

- **HTTP 直送且无人拦（4 条路）**：`POST /api/audit`（`AuditRequest.fields` 无 `min_length`，
  空清单直达 A 族 `DatasetAuditor.fields`）、`POST /api/leakage/check`（同一形状，空清单经
  `detect_leakage` 落到 A 族 `LeakageDetector.fields`）、`POST /api/dataset/stats`（空清单位置参数
  落到 D 族 `calculate::fields` ⇒ 「一列都不统计」变成「全部列」）、`POST /api/dataset/search`
  （空清单位置参数落到 B 族 `EnhancedSearcher.search::fields`）。
- **已经拦住的对照格（2 处）**：`PiiSanitizer` 在**构造器层**改形状（L188），
  `POST /api/dataset/evaluate` 在**路由层**加了 `if not metrics:` ⇒ 400，其注释还写着
  「空列表是假值，传下去会被静默补成三只」——**同一族缺陷在两处被承认，却没有任何一把尺子
  管其余格**，这正是 A343 要钉的缺口。
- **CLI 无空面**：`--fields` / `--rules` 一律 `nargs="+"`，argparse 保证要么 `None` 要么至少一个值
  ⇒ 由 `test_the_CLI_cannot_produce_an_empty_list` 现场验，防止后来人把这一族记成「命令行也能吞」。
- **仓内零调用方传空**：AST 扫全产品码，没有任何一处把空容器字面量喂给这些参数 ⇒ 吞空只在
  **外部输入**（HTTP / 配置 / SDK 用户代码）那一面成立。

**修法留给拍板**（本格只立账、只普查、只钉可达面，不改产品码）：① 逐格改 `is not None`
（L188 判决的推广，A+B 共 17 格，改后「空清单」按零元素语义走，四条 HTTP 路的响应内容会变）；
② 只在 HTTP 门口加封闭校验（`min_length=1`，把空清单判 422，产品内部形状不动）；
③ 维持现状 + 把四条可达路逐条写成「空 ⇒ 全量」的明示契约。三法的对外语义各不相同，
所以 A343 定级 M 待拍，不在代理自开轮里自选。

**本守卫的边界（如实声明）**：普查判据是**语法形状** `X = X or D`（含 `self.X = X or D`），
它量不到「默认值经一层函数求出来才非空」中除 `list(...)` / 大写常量之外的形状（已单列 D 族），
也量不到 `dict(x or {})`、`f(x or [])` 这类**表达式内部**的吞空。注入臂只证明「新的 A 族格会被
数进来、修好的形状不会被数进来」，不证明这四族覆盖全仓所有假值吞并。
"""
import ast
import pathlib
import re

import pytest

ROOT = pathlib.Path(__file__).resolve().parents[2]
SKIP_PARTS = ("/tests/", "test_", "/.venv/", "/web/", "node_modules", "/__pycache__/")

SITE = re.compile(r"^\s*(?:self\.)?([A-Za-z_]\w*)\s*=\s*\1\s+or\s+(.+)$")
EMPTY_DEFAULTS = frozenset({"[]", "{}", "()", "set()", "dict()", "list()", '""', "''"})
NONEMPTY_CALL = re.compile(r"^(list|set|tuple|sorted|dict)\(")
CONSTANT = re.compile(r"^[A-Z_][A-Z_0-9]*$")

MEASURED_TOTAL = 38
MEASURED_A = 10
MEASURED_B = 7
MEASURED_C = 10
MEASURED_D = 11

ROSTER_A = frozenset({
    "augmentor/aggregator.py::DataAggregator.__init__::key_fields",
    "augmentor/aggregator.py::DataAggregator.__init__::value_fields",
    "augmentor/audit.py::DatasetAuditor.__init__::fields",
    "augmentor/benchmark.py::QualityBenchmark.__init__::metrics",
    "augmentor/data/audio.py::AudioProcessor.__init__::supported_extensions",
    "augmentor/data/image.py::ImageProcessor.__init__::supported_extensions",
    "augmentor/evaluation.py::ModelEvaluator.__init__::metrics",
    "augmentor/leakage.py::LeakageDetector.__init__::fields",
    "augmentor/multilingual.py::MultilingualSupport.__init__::supported_langs",
    "augmentor/report.py::ReportGenerator.__init__::buckets",
})
ROSTER_B = frozenset({
    "augmentor/analytics.py::DatasetAnalyzer.analyze::fields",
    "augmentor/cleaner.py::DatasetCleaner.clean::fields",
    "augmentor/cleaner.py::DatasetCleaner.clean::rules",
    "augmentor/compare_enhanced.py::EnhancedComparator.compare::key_fields",
    "augmentor/compare_enhanced.py::EnhancedComparator.diff_datasets::key_fields",
    "augmentor/indexer.py::DatasetIndexer.search::fields",
    "augmentor/search_enhanced.py::EnhancedSearcher.search::fields",
})
ROSTER_C = frozenset({
    "augmentor/aggregator.py::DataAggregator.aggregate_weighted::weights",
    "augmentor/auto_test.py::DatasetTestRunner.add_custom_test::tags",
    "augmentor/dependency.py::DependencyManager.register_dataset::tags",
    "augmentor/dependency.py::DependencyManager.register_dataset::metadata",
    "augmentor/quality_gate.py::QualityGate.__init__::rules",
    "augmentor/report.py::ReportGenerator.generate::dedup_summary",
    "augmentor/tracker.py::ExperimentTracker.start_experiment::metadata",
    "augmentor/version_control.py::DatasetVersionManager.create_version::tags",
    "augmentor/version_control.py::DatasetVersionManager.create_version::metadata",
    "augmentor/versioning.py::VersionManager.create_version::metadata",
})
ROSTER_D = frozenset({
    "augmentor/active_learning.py::ActiveLearningLoop.select_samples::strategy",
    "augmentor/active_learning.py::ActiveLearningLoop.run::strategy",
    "augmentor/auto_test.py::DatasetTestRunner.create_test_suite::test_cases",
    "augmentor/dataset_ops.py::DatasetOperations._merge_and_count::config",
    "augmentor/dataset_ops.py::DatasetOperations.sample::config",
    "augmentor/dataset_ops.py::DatasetOperations.split::config",
    "augmentor/export_enhanced.py::EnhancedExporter.export::options",
    "augmentor/pipeline.py::AugmentorPipeline.__init__::config",
    "augmentor/profiling.py::DataProfiler.__init__::config",
    "augmentor/statistics.py::DatasetStatisticsCalculator.calculate::fields",
    "augmentor/versioning.py::VersionManager.create_version::label",
})


class Site:
    def __init__(self, key, cls, param, default, face, line):
        self.key, self.cls, self.param = key, cls, param
        self.default, self.face, self.line = default, face, line


def classify(text, relname):
    """按语法形状给一份源码里的 `X = X or D` 分族（纯函数，注入臂可直接喂内存文本）"""
    out = []
    cls = "<module>"
    fn = "<module>"
    for i, ln in enumerate(text.replace("\r\n", "\n").split("\n"), 1):
        mc = re.match(r"class (\w+)", ln)
        if mc:
            cls, fn = mc.group(1), "<module>"
        mf = re.match(r"\s*def (\w+)", ln)
        if mf:
            fn = mf.group(1)
        if ln.strip().startswith("#"):
            continue
        m = SITE.match(ln)
        if not m:
            continue
        param, rhs = m.group(1), m.group(2).strip()
        for tok in ("  #", " #"):
            if tok in rhs and rhs.count("(") == rhs.count(")"):
                rhs = rhs.split(tok)[0].strip()
        rhs = rhs.rstrip(",").strip()
        key = rhs.replace(" ", "")
        face = "属性面" if re.search(r"self\." + param + r"\s*=", ln) else "方法体"
        if key in EMPTY_DEFAULTS:
            tag = "C"
        elif key[:1] in "[{" or NONEMPTY_CALL.match(key) or CONSTANT.match(key):
            tag = "A" if face == "属性面" else "B"
        else:
            tag = "D"
        out.append((tag, Site(f"{relname}::{cls}.{fn}::{param}", cls, param, rhs, face, i)))
    return out


def product_files():
    for p in sorted(ROOT.rglob("*.py")):
        s = str(p).replace("\\", "/")
        if any(x in "/" + s for x in SKIP_PARTS):
            continue
        yield s[len(str(ROOT).replace("\\", "/")) + 1:], p.read_bytes().decode("utf-8", "replace")


def census():
    buckets = {"A": {}, "B": {}, "C": {}, "D": {}}
    for rel, text in product_files():
        for tag, site in classify(text, rel):
            buckets[tag][site.key] = site
    return buckets


def read(rel):
    return (ROOT / rel).read_bytes().decode("utf-8", "replace")


def test_the_census_is_the_measured_thirty_eight():
    b = census()
    got = {k: len(v) for k, v in b.items()}
    assert got == {"A": MEASURED_A, "B": MEASURED_B, "C": MEASURED_C, "D": MEASURED_D}, got
    assert sum(got.values()) == MEASURED_TOTAL
    keys = [k for v in b.values() for k in v]
    assert len(keys) == len(set(keys)), "同一 file::class.fn::param 出现两格 ⇒ 名单口径不唯一"


def test_each_roster_is_exact_not_a_ratchet():
    """写成「≤ 某数」的话，新增一格照样绿；这一族的历史恰恰是「以为清了又量出一批」"""
    b = census()
    for tag, pinned in (("A", ROSTER_A), ("B", ROSTER_B), ("C", ROSTER_C), ("D", ROSTER_D)):
        assert set(b[tag]) == pinned, sorted(set(b[tag]) ^ pinned)


def test_class_A_is_the_attribute_face_and_B_is_the_call_face():
    b = census()
    assert all(s.face == "属性面" for s in b["A"].values()), [(k, s.face) for k, s in b["A"].items()]
    assert all(s.face == "方法体" for s in b["B"].values())


def test_class_C_is_provably_idempotent():
    """C 族不算缺陷靠的是「默认值本身是空字面量」这一条可判定的事实，不是人眼说它无害"""
    b = census()
    loose = {k: s.default for k, s in b["C"].items() if s.default.replace(" ", "") not in EMPTY_DEFAULTS}
    assert loose == {}, loose
    assert all(s.default.replace(" ", "") in EMPTY_DEFAULTS for s in b["C"].values())


def test_no_site_in_the_census_uses_the_fixed_shape():
    """`x if x is not None else D` 那一形不进普查：尺子量的是形状，不是文件名"""
    fixed = read("augmentor/privacy.py")
    assert fixed.count("if fields is not None else") == 1
    assert not [s for s in classify(fixed, "augmentor/privacy.py")], classify(fixed, "x")


class TestEmptyIsIndistinguishableFromUnset:
    """A 族十格逐格实例化：传 `[]` 与传 `None` 得到同一个非空默认 ⇒ 空清单这一档没有语义"""

    # argvalues 与 ids 都写成装饰器里的内联字面量：l97 只记「有没有 ids」，l98 还把「值位不是
    # 字面 list」当盲面口径、把「ids 由名字/推导式现造」单独算一档 ⇒ 内联字面量让这一支既不进
    # 盲面也不进 ids-derived 档；两串数量不等时 pytest 在收集期当场报错，不需要另造对账用例。
    @pytest.mark.parametrize(
        "mod,cls_name,attr",
        [
            ("augmentor.aggregator", "DataAggregator", "key_fields"),
            ("augmentor.aggregator", "DataAggregator", "value_fields"),
            ("augmentor.audit", "DatasetAuditor", "fields"),
            ("augmentor.benchmark", "QualityBenchmark", "metrics"),
            ("augmentor.data.audio", "AudioProcessor", "supported_extensions"),
            ("augmentor.data.image", "ImageProcessor", "supported_extensions"),
            ("augmentor.evaluation", "ModelEvaluator", "metrics"),
            ("augmentor.leakage", "LeakageDetector", "fields"),
            ("augmentor.multilingual", "MultilingualSupport", "supported_langs"),
            ("augmentor.report", "ReportGenerator", "buckets"),
        ],
        ids=[
            "DataAggregator.key_fields", "DataAggregator.value_fields",
            "DatasetAuditor.fields", "QualityBenchmark.metrics",
            "AudioProcessor.supported_extensions", "ImageProcessor.supported_extensions",
            "ModelEvaluator.metrics", "LeakageDetector.fields",
            "MultilingualSupport.supported_langs", "ReportGenerator.buckets",
        ],
    )
    def test_empty_list_is_swallowed_into_the_default(self, mod, cls_name, attr):
        import importlib

        cls = getattr(importlib.import_module(mod), cls_name)
        empty = getattr(cls(**{attr: []}), attr)
        unset = getattr(cls(**{attr: None}), attr)
        assert empty == unset, f"{cls_name}.{attr}: 空清单与未传同值 ⇒ 这一档没有独立语义（现状）"
        assert len(empty) > 0, f"{cls_name}.{attr} 的默认非空，才构成「吞空」而不是「幂等」"

    def test_the_pii_sanitizer_is_the_opposite_control(self):
        """同仓已修那一格：传 [] 得到 []，传 None 才得到默认 ⇒ A 族那一档在这里有语义"""
        from augmentor.privacy import PiiSanitizer

        assert PiiSanitizer(fields=[]).fields == []
        assert len(PiiSanitizer(fields=None).fields) > 0


class TestReachability:
    """可达面逐条对账：吞空只有接在外部输入上才是对外缺陷"""

    def test_audit_route_feeds_an_empty_list_straight_into_class_A(self):
        text = read("api/routes/audit.py")
        assert "fields: Optional[List[str]] = None" in text, "请求模型没有 min_length 约束 ⇒ [] 过校验"
        assert "min_length" not in text
        assert "DatasetAuditor(fields=request.fields)" in text

    def test_leakage_route_feeds_an_empty_list_straight_into_class_A(self):
        text = read("api/routes/leakage.py")
        assert "fields: Optional[List[str]] = None" in text
        assert "fields=request.fields" in text
        helper = read("augmentor/leakage.py")
        assert "detector = LeakageDetector(fields=fields, fuzzy_threshold=fuzzy_threshold)" in helper

    def test_stats_route_feeds_an_empty_list_into_a_class_D_method(self):
        text = read("api/routes/dataset_tools.py")
        assert "fields: Optional[List[str]] = None" in text
        assert "calculate_statistics, items, Path(path).stem, request.fields" in text
        calc = read("augmentor/statistics.py")
        assert "fields = fields or self._get_all_fields()" in calc

    def test_search_route_feeds_an_empty_list_into_a_class_B_method(self):
        text = read("api/routes/dataset_tools.py")
        assert "search_dataset," in text and "request.fields," in text
        searcher = read("augmentor/search_enhanced.py")
        assert 'fields = fields or ["instruction", "output", "input"]' in searcher

    def test_the_evaluate_route_already_defends_at_the_door(self):
        """对照格：同一条 metrics 链在路由层加了空清单判决 ⇒ 承认缺陷，但只补了这一处"""
        text = read("api/routes/dataset_tools.py")
        assert "if not metrics:" in text
        assert "评估指标清单为空" in text

    def test_the_CLI_cannot_produce_an_empty_list(self):
        parser = read("augmentor/cli/parser.py")
        for flag in ('"--rules"', '"--fields"', '"--key-fields"'):
            hit = [ln for ln in parser.split("\n") if flag in ln]
            assert hit and all("nargs=\"+\"" in ln or "nargs=4" in ln for ln in hit), hit

    def test_no_caller_inside_the_repo_passes_an_empty_literal(self):
        """吞空只在外部输入面成立：仓内没有任何一处把 [] / {} 直接喂给这些参数"""
        names = {"DatasetAuditor", "LeakageDetector", "detect_leakage", "ModelEvaluator",
                 "QualityBenchmark", "DataAggregator", "ReportGenerator", "MultilingualSupport",
                 "AudioProcessor", "ImageProcessor", "calculate_statistics", "search_dataset"}
        hits = []
        for rel, text in product_files():
            for node in ast.walk(ast.parse(text)):
                if not isinstance(node, ast.Call):
                    continue
                callee = getattr(node.func, "id", None) or getattr(node.func, "attr", None)
                if callee not in names:
                    continue
                args = list(node.args) + [kw.value for kw in node.keywords]
                if any(isinstance(a, (ast.List, ast.Dict)) and not a.elts for a in args):
                    hits.append(f"{rel}:{node.lineno}")
        assert hits == [], hits


class TestTheRulerBites:
    """全部在内存文本上验，磁盘零副作用：证明四族名册不是恒真，也证明修法会立刻被数出来"""

    def test_arm_a_new_attribute_site_lands_in_class_A(self):
        src = ("class K:\n"
               "    def __init__(self, xs=None):\n"
               '        self.xs = xs or ["a"]\n')
        got = classify(src, "synthetic.py")
        assert [t for t, s in got] == ["A"] and got[0][1].key == "synthetic.py::K.__init__::xs"

    def test_arm_b_a_new_method_site_lands_in_class_B(self):
        src = ("class K:\n"
               "    def run(self, xs=None):\n"
               "        xs = xs or [] or [1]\n")
        assert [t for t, _s in classify(src, "synthetic.py")] == ["B"]

    def test_arm_c_the_fixed_shape_is_invisible(self):
        src = ("class K:\n"
               "    def __init__(self, xs=None):\n"
               "        self.xs = xs if xs is not None else [1]\n")
        assert classify(src, "synthetic.py") == []

    def test_arm_d_an_empty_default_is_class_C_not_A(self):
        src = ("class K:\n"
               "    def __init__(self, xs=None):\n"
               "        self.xs = xs or []\n")
        assert [t for t, _s in classify(src, "synthetic.py")] == ["C"]

    def test_arm_e_regressing_the_L188_fix_would_be_counted(self):
        """把 privacy.py 已修那一格在内存里改回旧写法 ⇒ A 族名册当场多出一格，尺子对回退有牙齿"""
        fixed = read("augmentor/privacy.py")
        old = "        self.fields = fields if fields is not None else list(DEFAULT_TEXT_FIELDS)\n"
        backslide = '        self.fields = fields or list(DEFAULT_TEXT_FIELDS)\n'
        assert fixed.count(old) == 1
        assert not classify(fixed, "augmentor/privacy.py")
        got = classify(fixed.replace(old, backslide), "augmentor/privacy.py")
        assert [s.key for tag, s in got if tag == "A"] == [
            "augmentor/privacy.py::PiiSanitizer.__init__::fields"], got
