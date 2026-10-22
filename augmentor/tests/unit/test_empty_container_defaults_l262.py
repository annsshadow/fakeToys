# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

r"""L262 普查守卫 → L277 按修法① 收口：`x = x or DEFAULT` 把**显式空容器**读成「没传」

**前史（L262 立账）**：同一族缺陷在本仓有一处已判过的先例——`augmentor/privacy.py` 的 `PiiSanitizer`
在 L188 把写法改成 `x if x is not None else DEFAULT`。同一判决适用的其余格数当时没人量过，L262 按语法形状
全仓普查（`*.py`，排除 `tests/` 与 `test_*.py`）现量 **38 格**，分四族（A 属性面 10 / B 方法体 7 /
C 空默认幂等 10 / D 非容器默认 11），并写明「修法留给拍板，本格只立账不改产品码」。

**本轮（L277）**：用户 goal 明文「全面找出并修复验证 augmentor 中所有 bug 和漏洞」⇒ 代理择定 **修法①**
（逐格 `is not None`，即 L188 判决的推广），把 A+B 共 17 格 + D 族里同形状、且被 `POST /api/dataset/stats`
直送的 `statistics.calculate::fields` 一并改掉。普查现量 **20 格 = A 0 / B 0 / C 10 / D 10**：A/B 两族
名册留空不是「尺子废了」，而是**新增一格就红**（`test_each_roster_is_exact_not_a_ratchet` +
`TestTheRulerBites` 的回退臂照旧咬）。

**改完必须配套的真空假值护栏（否则这一族从「无声放大作用域」翻成更坏的「无声清空 / 假干净」）**：

- `DatasetCleaner._remove_empty`：`any(item.get(f) for f in [])` 恒 False ⇒ 零字段作用域会把每一条都判「空」
  而**删光整个数据集** ⇒ 现在原样返回、不记修改。
- `DatasetCleaner._remove_duplicates`：零作用域下每条的键同为 `()` ⇒ 数据集被**压成 1 条** ⇒ 同上。
- `DatasetAuditor(fields=[])` / `LeakageDetector(fields=[])` / `EnhancedComparator.compare(key_fields=[])` /
  `diff_datasets(key_fields=[])`：零作用域报出的「0 处 PII / 无泄漏 / 完全一致」是从**无从检查**推出来的
  假干净 ⇒ 构造期或入口直接 `DataValidationError`（HTTP 面 400），与本文件 L192 未知规则名、L144 假零族同式。

**留在账上的残余（未做，需拍板）**：修法② 的 HTTP 门口 `min_length=1`（422）没加。空清单现在落到 SDK 那一档：
`/api/audit` 与 `/api/leakage/check` 已被上面的 400 挡住，`/api/dataset/stats` 与 `/api/dataset/search`
回 200 但语义从「空 ⇒ 全量」改成「空 ⇒ 零结果 / 零统计」。随包前端不受影响：`web/src/services/api.ts`
的三只调用把可省键写成 `undefined`（JSON 序列化时整键消失 ⇒ 走 `None` 那一档），其注释本来就写着
「页面的『没选』应当真的是没选，而不是替服务端写一个空清单当默认」。

**本守卫的边界（如实声明）**：普查判据是语法形状 `X = X or D`，量不到 `dict(x or {})` / `f(x or [])` 这类
表达式内部的吞空；`FIXED_CELLS` 钉的是「已修那一形在本仓恰好出现在这些行」，不是「全仓再无吞空」。
"""
import ast
import pathlib
import re

import pytest

ROOT = pathlib.Path(__file__).resolve().parents[2]
# `/Temp/` 必须排除：那是**未入仓**的取证草稿目录，里面躺着各轮尺子的历史副本
# （`Temp/l57/head/augmentor/...` 一整棵树）。不排除的话，本地普查把历史副本也算成
# 产品码 —— 2026-10-22 实测 A 族 301 vs 常数 0、名册里整片 Temp 条目，而 CI 的干净
# 克隆没有 Temp、一直是绿的。尺子量的是**当前产品码**，历史副本由 git 负责。
# （这与 a184 的索引口径不矛盾：那边把 Temp 留在索引里，是为了让账本里的取证脚本
#  引用解析成「指向工件」而不是「指向不存在的文件」—— 两把尺子要的东西不同。）
SKIP_PARTS = ("/tests/", "test_", "/.venv/", "/web/", "node_modules", "/__pycache__/", "/Temp/")

SITE = re.compile(r"^\s*(?:self\.)?([A-Za-z_]\w*)\s*=\s*\1\s+or\s+(.+)$")
EMPTY_DEFAULTS = frozenset({"[]", "{}", "()", "set()", "dict()", "list()", '""', "''"})
NONEMPTY_CALL = re.compile(r"^(list|set|tuple|sorted|dict)\(")
CONSTANT = re.compile(r"^[A-Z_][A-Z_0-9]*$")

MEASURED_TOTAL = 20
MEASURED_A = 0
MEASURED_B = 0
MEASURED_C = 10
MEASURED_D = 10

# 修法① 之后 A/B 两族名册为空：留空而非删掉，是为了让「新写一格 `or` 吞空」当场红。
ROSTER_A = frozenset()
ROSTER_B = frozenset()
ROSTER_C = frozenset({
    "augmentor/aggregator.py::DataAggregator.aggregate_weighted::weights",
    "augmentor/auto_test.py::DatasetTestRunner.add_custom_test::tags",
    "augmentor/dependency.py::DependencyManager.register_dataset::metadata",
    "augmentor/dependency.py::DependencyManager.register_dataset::tags",
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
    "augmentor/versioning.py::VersionManager.create_version::label",
})

# 已修那 18 格（去重后 17 条判据：compare_enhanced 的两格同字面量，按「本仓恰好两处」钉）。
# 每条形如 `(文件, strip 后的整行, 应出现次数)`；行内容改动（含回退成 `or`）当场红。
FIXED_CELLS = (
    ("augmentor/aggregator.py", 'self.key_fields = list(key_fields) if key_fields is not None else ["instruction"]', 1),
    ("augmentor/aggregator.py", 'self.value_fields = list(value_fields) if value_fields is not None else ["output"]', 1),
    ("augmentor/audit.py", "self.fields = fields if fields is not None else list(DEFAULT_TEXT_FIELDS)", 1),
    ("augmentor/benchmark.py", "self.metrics = metrics if metrics is not None else list(SUPPORTED_METRICS)", 1),
    ("augmentor/data/audio.py", "self.supported_extensions = (supported_extensions if supported_extensions is not None else SUPPORTED_EXTENSIONS)", 1),
    ("augmentor/data/image.py", "self.supported_extensions = (supported_extensions if supported_extensions is not None else SUPPORTED_EXTENSIONS)", 1),
    ("augmentor/evaluation.py", 'self.metrics = metrics if metrics is not None else ["bleu", "rouge_l", "similarity"]', 1),
    ("augmentor/leakage.py", 'self.fields = fields if fields is not None else ["instruction"]', 1),
    ("augmentor/multilingual.py", 'self.supported_langs = supported_langs if supported_langs is not None else ["zh", "en"]', 1),
    ("augmentor/report.py", "self.buckets = buckets if buckets is not None else DEFAULT_BUCKETS", 1),
    ("augmentor/analytics.py", 'fields = fields if fields is not None else ["instruction", "output", "input"]', 1),
    ("augmentor/cleaner.py", "fields = DEFAULT_CLEAN_FIELDS if fields is None else fields", 1),
    ("augmentor/cleaner.py", "rules = self.rule_names if rules is None else list(rules)", 1),
    ("augmentor/compare_enhanced.py", 'key_fields = key_fields if key_fields is not None else ["instruction"]', 2),
    ("augmentor/indexer.py", 'fields = fields if fields is not None else ["instruction", "output"]', 1),
    ("augmentor/search_enhanced.py", 'fields = fields if fields is not None else ["instruction", "output", "input"]', 1),
    ("augmentor/statistics.py", "fields = fields if fields is not None else self._get_all_fields()", 1),
)

# 现在「空清单」在两族里各有归宿：吞空 ⇒ 套默认；修法① ⇒ 零元素作用域。
# 8 格只改形状（空清单合法、按零作用域走），2 格（audit / leakage）额外在构造期拒空。
HONOUR_ATTRIBUTES = (
    ("augmentor.aggregator", "DataAggregator", "key_fields"),
    ("augmentor.aggregator", "DataAggregator", "value_fields"),
    ("augmentor.benchmark", "QualityBenchmark", "metrics"),
    ("augmentor.data.audio", "AudioProcessor", "supported_extensions"),
    ("augmentor.data.image", "ImageProcessor", "supported_extensions"),
    ("augmentor.evaluation", "ModelEvaluator", "metrics"),
    ("augmentor.multilingual", "MultilingualSupport", "supported_langs"),
    ("augmentor.report", "ReportGenerator", "buckets"),
)

ITEMS = [
    {"instruction": "打开空调", "input": "客厅", "output": "已打开"},
    {"instruction": "调低温度", "input": "卧室", "output": "已调低"},
]


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


def test_the_census_is_the_measured_twenty():
    b = census()
    got = {k: len(v) for k, v in b.items()}
    assert got == {"A": MEASURED_A, "B": MEASURED_B, "C": MEASURED_C, "D": MEASURED_D}, got
    assert sum(got.values()) == MEASURED_TOTAL
    keys = [k for v in b.values() for k in v]
    assert len(keys) == len(set(keys)), "同一 file::class.fn::param 出现两格 ⇒ 名单口径不唯一"


def test_each_roster_is_exact_not_a_ratchet():
    """写成「≤ 某数」的话，新增一格照样绿；A/B 两族现在**恰好零格**，多一格即红"""
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


def test_every_fixed_cell_is_pinned_by_its_exact_line():
    """18 格逐条按整行字面量对账：把任一格改回 `or`、或把默认值抄错，这一支当场红"""
    cache = {}
    for rel, line, count in FIXED_CELLS:
        if rel not in cache:
            cache[rel] = [ln.strip() for ln in read(rel).replace("\r\n", "\n").split("\n")]
        got = [ln for ln in cache[rel] if ln == line]
        assert len(got) == count, (rel, count, len(got), line)


def test_the_census_cannot_see_the_fixed_shape_anywhere():
    """反向对账：已修那 18 格现在**一格都不该**被普查数到（否则 FIX 名册与普查两套口径打架）"""
    b = census()
    fixed_files = {rel for rel, _line, _c in FIXED_CELLS}
    still_counted = {k for tag in ("A", "B") for k in b[tag]
                     if any(k.startswith(rel + "::") for rel in fixed_files)}
    assert still_counted == set(), still_counted


class TestEmptyIsNowHonoured:
    """A 族属性面：传 `[]` 得到 `[]`，传 `None` 才得到默认 ⇒ 「空清单」这一档现在有独立语义"""

    # argvalues 与 ids 都写成装饰器里的内联字面量（l97 / l98 的口径：值位是字面 list、
    # ids 现造会被单列一档）；两串数量不等时 pytest 在收集期当场报错。
    @pytest.mark.parametrize(
        "mod,cls_name,attr",
        [
            ("augmentor.aggregator", "DataAggregator", "key_fields"),
            ("augmentor.aggregator", "DataAggregator", "value_fields"),
            ("augmentor.benchmark", "QualityBenchmark", "metrics"),
            ("augmentor.data.audio", "AudioProcessor", "supported_extensions"),
            ("augmentor.data.image", "ImageProcessor", "supported_extensions"),
            ("augmentor.evaluation", "ModelEvaluator", "metrics"),
            ("augmentor.multilingual", "MultilingualSupport", "supported_langs"),
            ("augmentor.report", "ReportGenerator", "buckets"),
        ],
        ids=[
            "DataAggregator.key_fields", "DataAggregator.value_fields",
            "QualityBenchmark.metrics", "AudioProcessor.supported_extensions",
            "ImageProcessor.supported_extensions", "ModelEvaluator.metrics",
            "MultilingualSupport.supported_langs", "ReportGenerator.buckets",
        ],
    )
    def test_empty_list_now_means_zero_elements(self, mod, cls_name, attr):
        import importlib

        cls = getattr(importlib.import_module(mod), cls_name)
        empty = getattr(cls(**{attr: []}), attr)
        unset = getattr(cls(**{attr: None}), attr)
        assert list(empty) == [], f"{cls_name}.{attr}: 空清单仍被吞成默认 ⇒ 修法① 被回退"
        assert len(list(unset)) > 0, f"{cls_name}.{attr}: 默认变空 ⇒ 省略与传空同值，尺子失效"

    def test_the_pii_sanitizer_is_the_same_control_as_l188(self):
        """同仓最早那一格（L188）与本轮 18 格现在是同一形状：传 [] 得 []，传 None 得默认"""
        from augmentor.privacy import PiiSanitizer

        assert PiiSanitizer(fields=[]).fields == []
        assert len(PiiSanitizer(fields=None).fields) > 0


class TestEmptyScopeRefusedWhereItWouldLie:
    """零作用域推不出「干净」，只推得出「无从检查」⇒ 检测/比对类入口拒空"""

    def test_auditor_refuses_an_empty_field_scope(self):
        from augmentor.audit import DatasetAuditor
        from augmentor.validation import DataValidationError

        with pytest.raises(DataValidationError, match="不能是空清单"):
            DatasetAuditor(fields=[])

    def test_leakage_detector_refuses_an_empty_field_scope(self):
        from augmentor.leakage import LeakageDetector
        from augmentor.validation import DataValidationError

        with pytest.raises(DataValidationError, match="不能是空清单"):
            LeakageDetector(fields=[])

    def test_comparator_refuses_an_empty_key_scope(self):
        from augmentor.compare_enhanced import EnhancedComparator, diff_datasets
        from augmentor.validation import DataValidationError

        with pytest.raises(DataValidationError, match="不能是空清单"):
            EnhancedComparator().compare(ITEMS, ITEMS, key_fields=[])
        with pytest.raises(DataValidationError, match="不能是空清单"):
            diff_datasets(ITEMS, ITEMS, key_fields=[])

    def test_none_still_means_default_for_all_four(self):
        """拒的是显式空清单，不是「没表态」：省略仍然拿到默认作用域（不破坏既有调用）"""
        from augmentor.audit import DatasetAuditor
        from augmentor.compare_enhanced import EnhancedComparator
        from augmentor.leakage import LeakageDetector

        assert len(DatasetAuditor().fields) > 0
        assert len(LeakageDetector().fields) > 0
        assert len(LeakageDetector(fields=None).fields) > 0
        r = EnhancedComparator().compare(ITEMS, ITEMS, key_fields=None)
        assert r.dataset_a_name == "dataset_a"


class TestVacuityGuards:
    """护栏本体：空作用域下的两处真空假值（删光 / 压成一条）与四类零结果读取"""

    def test_remove_empty_with_zero_fields_keeps_everything(self):
        from augmentor.cleaner import DatasetCleaner

        dirty = [{"instruction": "带图", "output": "有答案"}, {"instruction": "只有问", "output": ""}]
        kept, result = DatasetCleaner().clean(dirty, fields=[])
        assert len(kept) == 2, "零字段作用域把每条判成空 ⇒ `_remove_empty` 的真空假值回来了"
        assert result.removed_count == 0

    def test_remove_empty_with_default_fields_still_removes(self):
        """对照臂：护栏不能把规则本身收恒真——作用域内的字段全空时那条照旧被删

        `_remove_empty` 的判据是 `any(item.get(f) for f in fields)`：只要作用域里还有
        一个非空字段就留。所以对照样本必须三个默认字段都空，才是「默认作用域会删、
        零作用域不删」的差。
        """
        from augmentor.cleaner import DatasetCleaner

        dirty = [{"instruction": "带图", "output": "有答案"}, {"instruction": "", "output": "", "input": ""}]
        kept, _result = DatasetCleaner().clean(dirty)
        assert len(kept) == 1

    def test_remove_duplicates_with_zero_fields_does_not_collapse_to_one(self):
        from augmentor.cleaner import DatasetCleaner

        dup = [{"instruction": "a", "output": "1"}, {"instruction": "b", "output": "2"}]
        kept, result = DatasetCleaner().clean(dup, rules=["remove_duplicates"], fields=[])
        assert len(kept) == 2, "空作用域下键同为 `()` ⇒ 两条不同数据被判重复"
        assert result.removed_count == 0

    def test_remove_duplicates_with_default_fields_still_dedups(self):
        from augmentor.cleaner import DatasetCleaner

        dup = [{"instruction": "a", "output": "1"}, {"instruction": "a", "output": "1"}]
        kept, _result = DatasetCleaner().clean(dup, rules=["remove_duplicates"])
        assert len(kept) == 1

    def test_search_with_zero_fields_hits_nothing(self):
        from augmentor.indexer import DatasetIndexer
        from augmentor.search_enhanced import EnhancedSearcher

        assert EnhancedSearcher(ITEMS).search("空调", fields=[]).total_matches == 0
        assert EnhancedSearcher(ITEMS).search("空调").total_matches == 1
        assert DatasetIndexer(ITEMS).search("空调", fields=[]).total_matches == 0

    def test_statistics_with_zero_fields_reports_no_field(self):
        from augmentor.statistics import DatasetStatisticsCalculator

        empty = DatasetStatisticsCalculator(ITEMS).calculate(fields=[])
        unset = DatasetStatisticsCalculator(ITEMS).calculate()
        assert empty.field_statistics == {}, "空清单一律 ⇒ 统计了所有字段（吞空回来了）"
        assert len(unset.field_statistics) > 0


class TestReachability:
    """可达面逐条对账：吞空只有接在外部输入上才是对外缺陷；改完之后四条路各落哪一档"""

    def test_audit_route_still_feeds_request_fields_but_the_sdk_now_refuses_empty(self):
        text = read("api/routes/audit.py")
        assert "fields: Optional[List[str]] = None" in text, "门口没加 min_length（修法② 仍在待拍面）"
        assert "DatasetAuditor(fields=request.fields)" in text
        # 空清单 ⇒ SDK 构造期 DataValidationError ⇒ 路由按校验错误族回 400，不再回「零发现」的假干净

    def test_leakage_route_same_as_audit(self):
        text = read("api/routes/leakage.py")
        assert "fields: Optional[List[str]] = None" in text
        assert "fields=request.fields" in text
        helper = read("augmentor/leakage.py")
        assert "detector = LeakageDetector(fields=fields, fuzzy_threshold=fuzzy_threshold)" in helper

    def test_stats_route_now_means_zero_fields(self):
        text = read("api/routes/dataset_tools.py")
        assert "fields: Optional[List[str]] = None" in text
        assert "calculate_statistics, items, Path(path).stem, request.fields" in text
        calc = read("augmentor/statistics.py")
        assert "fields = fields if fields is not None else self._get_all_fields()" in calc

    def test_search_route_now_means_zero_fields(self):
        text = read("api/routes/dataset_tools.py")
        assert "search_dataset," in text and "request.fields," in text
        searcher = read("augmentor/search_enhanced.py")
        assert 'fields = fields if fields is not None else ["instruction", "output", "input"]' in searcher

    def test_the_evaluate_route_already_defends_at_the_door(self):
        """对照格：同一条 metrics 链在路由层加了空清单判决，本轮把它从「只有这一处」变成通则"""
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
    """全部在内存文本上验，磁盘零副作用：证明四族名册不是恒真，也证明回退会被数出来"""

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

    def test_arm_f_regressing_an_L277_cell_would_be_counted(self):
        """本轮 18 格里任一回退成 `or` ⇒ 普查当场把那一格数回 A 族（名册为空 ⇒ 必红）"""
        fixed = read("augmentor/audit.py")
        old = "        self.fields = fields if fields is not None else list(DEFAULT_TEXT_FIELDS)\n"
        backslide = '        self.fields = fields or list(DEFAULT_TEXT_FIELDS)\n'
        assert fixed.count(old) == 1
        got = classify(fixed.replace(old, backslide), "augmentor/audit.py")
        assert [s.key for tag, s in got if tag == "A"] == [
            "augmentor/audit.py::DatasetAuditor.__init__::fields"], got
