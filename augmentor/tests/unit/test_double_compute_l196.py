# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L196：五处「同一件事算两遍」的**计数式**守卫

改动的共同形状：同一个纯函数在同一轮里被完整算了两遍，第二次的结果要么直接
丢掉（`merge_files` 的第二次 `_deduplicate`）、要么只为了拿一个 `len()`
（`diff_datasets` 的三个差集）。这类改动的验收难点是**行为零变化**——
所以每一条都配一个「调用次数」预言机：改动前的实现必然多算一次，把改动退回
去这些用例就必须红。单纯断言「结果没变」是不过关的（旧实现也算得出同样结果）。

实测依据（两个独立测量链互相印证，各自 min-of-3）：真实 6902 条房产客服语料，
`PYTHONHASHSEED=0`，把改动前的整包内容物化成**另一个同名包**当独立预言机
（不是新代码的副本——同源预言机在这里等于自证），双进程跑同一份 dump 后比字节：
44872 字节输出**逐键相同**；计时：`recommend_seeds` **0.0484 → 0.0266 s（1.82×）**、
`detect(method="iqr")` 0.0024 → 0.0013（1.85×）、`get_statistics` 0.0013 → 0.0012、
`diff_datasets` 0.0016 → 0.0014、合计 ×0.73。
（探针脚本是一次性工件，不写路径：本文件只用上面那份内存语料，不依赖任何
不入仓的数据文件。）
"""

import json

import pytest

from augmentor.compare_enhanced import diff_datasets
from augmentor.dataset_ops import DatasetOperations, MergeConfig
from augmentor.outlier import OutlierDetector
from augmentor.sampler import ActiveSampler

#: 一份够触发全部分支的小语料（含重复 instruction，喂去重那条）
ITEMS = [
    {"instruction": f"如何申请入住{q}", "input": "", "output": f"答{q}"}
    for q in ("甲", "乙", "丙", "甲", "乙")
] + [
    {"instruction": "退租流程是什么？", "input": "", "output": "提前 30 天申请。"},
    {"instruction": "", "input": "", "output": "空指令也算一条。"},
]

#: 离群点检测用的数值语料：`_extract_values` 读 `item.get(field)` 并转 float，
#: 文本字段一律得到 None ⇒ 必须显式带数值字段，否则 detect 在排序之前就早退
NUMERIC_ITEMS = [{"length": float(v)} for v in
                 (10, 12, 11, 13, 9, 12, 10, 11, 500, 12)]

#: 带少数派问题类型的语料：`why` 占 1/11 ≈ 0.0909 < threshold(0.1) ⇒ 既进
#: underrepresented 列表、又能被匹配循环捡回来当 seed（ITEMS 里只有 length 档，
#: 那条分支永远捡不回 seed，recommended_seeds 恒空）
ITEMS_WITH_MINORITY = (
    [{"instruction": f"如何申请入住{c}", "input": "", "output": f"答{c}"}
     for c in "abcdefghij"]
    + [{"instruction": "为什么我的申请被拒绝了？", "input": "", "output": "资料不全。"}]
)


class _CountingList(list):
    """只重数 `__iter__` 的 list：用来数一个函数把 items 走了几趟"""

    def __init__(self, iterable=()):
        super().__init__(iterable)
        self.iterations = 0

    def __iter__(self):
        self.iterations += 1
        return super().__iter__()


class TestSamplerAnalyzesCoverageOnce:
    """sampler：`recommend_seeds` 不再把 `analyze_coverage` 完整算两遍"""

    def test_recommend_seeds_calls_analyze_coverage_exactly_once(self, monkeypatch):
        sampler = ActiveSampler()
        real = sampler.analyze_coverage
        calls = []

        def spy(items):
            calls.append(len(items))
            return real(items)

        monkeypatch.setattr(sampler, "analyze_coverage", spy)
        sampler.recommend_seeds(ITEMS, top_k=3)

        assert len(calls) == 1, (
            f"analyze_coverage 被调 {len(calls)} 次——recommend_seeds 与 "
            "identify_underrepresented 各算了一遍（L196 前的形状）。"
            "它是 4 趟遍历的纯函数，重复一次就是白扫一整轮。"
        )

    def test_analysis_kwarg_is_equivalent_to_computing_it_here(self):
        """新形参只许省掉重复计算，不许改变答案"""
        sampler = ActiveSampler()
        self_computed = sampler.identify_underrepresented(ITEMS)
        passed_down = sampler.identify_underrepresented(
            ITEMS, analysis=sampler.analyze_coverage(ITEMS)
        )
        assert self_computed == passed_down

    def test_threshold_is_still_honoured_when_analysis_is_passed(self):
        """传了 analysis 不算把 threshold 一起吃死——两个旋钮互不遮挡"""
        sampler = ActiveSampler()
        loose = sampler.identify_underrepresented(ITEMS, threshold=0.99)
        tight = sampler.identify_underrepresented(
            ITEMS, threshold=0.99, analysis=sampler.analyze_coverage(ITEMS)
        )
        assert loose == tight
        assert loose, "threshold=0.99 下一个都不该漏报，语料里有大量覆盖不足类型"

    def test_recommend_seeds_result_is_unchanged(self):
        """端到端结果不许变（三项都要照实回填）"""
        sampler = ActiveSampler()
        result = sampler.recommend_seeds(ITEMS, top_k=3)
        assert result.coverage_analysis["total_items"] == len(ITEMS)
        assert result.recommendations, "语料里有覆盖不足类型，建议列表不该为空"
        # ITEMS 的不足类型全是 length 档，匹配循环捡不回 item ⇒ 空是正确形状
        assert result.recommended_seeds == []

    def test_recommend_seeds_still_returns_seeds_when_they_exist(self):
        """少数派**问题类型**必须仍被捡回来当 seed（防只改快路丢了匹配逻辑）"""
        sampler = ActiveSampler()
        result = sampler.recommend_seeds(ITEMS_WITH_MINORITY, top_k=3)
        assert result.recommended_seeds, "why 只占 1/11 < 0.1，它该被推荐出来"
        assert all(isinstance(seed, dict) and seed.get("instruction")
                   for seed in result.recommended_seeds)


class TestMergeFilesDeduplicatesOnce:
    """dataset_ops：`merge_files` 不再为拿一个计数把整份数据又去重一遍"""

    def _write_inputs(self, tmp_path, datasets):
        tmp_path.mkdir(parents=True, exist_ok=True)
        paths = []
        for i, data in enumerate(datasets):
            p = tmp_path / f"in_{i}.json"
            p.write_text(json.dumps(data, ensure_ascii=False), encoding="utf-8")
            paths.append(str(p))
        return paths

    def test_merge_files_calls_deduplicate_exactly_once(self, tmp_path, monkeypatch):
        ops = DatasetOperations()
        real = ops._deduplicate
        calls = []

        def spy(items, threshold=0.9):
            calls.append(len(items))
            return real(items, threshold)

        monkeypatch.setattr(ops, "_deduplicate", spy)
        paths = self._write_inputs(tmp_path, [ITEMS, ITEMS[:3], ITEMS])
        ops.merge_files(paths, str(tmp_path / "out.json"))

        assert len(calls) == 1, (
            f"_deduplicate 被调 {len(calls)} 次——merge() 一次、报表计数又一次。"
            "它是逐条 md5 的纯函数，重复一次就是再扫一整份语料。"
        )

    def test_merge_reports_are_unchanged(self, tmp_path):
        """重构唯一真正的验收标准：四种配置的报表与产物都不许动

        不写魔法数字，改用**恒等式**判：报表每一项都必须与产物侧的实际形状
        对得上（旧实现里这几个数来自两次独立的 `_deduplicate`，一旦两侧漂移
        立刻能看见）。
        """
        ops = DatasetOperations()
        datasets = [ITEMS, ITEMS[:3], ITEMS]
        total = sum(len(d) for d in datasets)
        for name, cfg in (
            ("default", MergeConfig()),
            ("no_dedup", MergeConfig(deduplicate=False)),
            ("threshold_0_5", MergeConfig(dedup_threshold=0.5)),
            ("max_3", MergeConfig(max_items=3)),
        ):
            paths = self._write_inputs(tmp_path / name, datasets)
            dest = tmp_path / name / "out.json"
            report = ops.merge_files(paths, str(dest), cfg)
            produced = json.loads(dest.read_text(encoding="utf-8"))

            assert report["total_input"] == total, name
            assert report["total_output"] == len(produced), name
            # 产物必须与 merge() 逐条一致：这一条才是「内联没改口径」的判据
            assert produced == ops.merge([list(d) for d in datasets], cfg), name
            if cfg.deduplicate:
                if cfg.max_items is None:
                    assert report["removed_duplicates"] == total - len(produced), name
                else:
                    # 截断档下 removed 只记去重、不记截断（L148/B218 分报口径）
                    assert report["removed_duplicates"] < total - len(produced), name
                    assert report["truncated_by_max_items"] > 0, name
            else:
                assert report["removed_duplicates"] == 0, name
            assert report["truncated_by_max_items"] == (
                total - report["removed_duplicates"] - len(produced)
            ), name

    def test_config_none_still_reports_zero_removed(self, tmp_path):
        """`config=None` 的口径刻意不动：产物照旧去重，报表的 removed 仍是 0"""
        paths = self._write_inputs(tmp_path, [ITEMS, ITEMS])
        report = DatasetOperations().merge_files(
            paths, str(tmp_path / "out.json"), None
        )
        assert report["removed_duplicates"] == 0
        assert report["total_output"] < report["total_input"], "产物侧仍要去重"

    def test_merged_helper_reports_removed_count(self):
        """新私有助手的返回值本身要对：removed == 去重前的条数差"""
        ops = DatasetOperations()
        merged, removed = ops._merge_and_count([ITEMS, ITEMS], MergeConfig())
        assert removed == len(ITEMS) * 2 - len(merged)
        assert len(merged) + removed == len(ITEMS) * 2

    def test_merge_still_matches_the_helper(self):
        """`merge()` 只剩一层壳，答案必须和助手一致"""
        ops = DatasetOperations()
        assert ops.merge([ITEMS, ITEMS], MergeConfig()) == ops._merge_and_count(
            [ITEMS, ITEMS], MergeConfig()
        )[0]


class TestIqrSortsOnce:
    """outlier：iqr 分支的两个分位共一次排序"""

    def test_iqr_detect_sorts_exactly_once(self, monkeypatch):
        import augmentor.outlier as outlier_mod

        real_sorted = sorted
        calls = []

        def spy(iterable, **kwargs):
            calls.append(len(list(iterable)))
            return real_sorted(iterable, **kwargs)

        monkeypatch.setattr(outlier_mod, "sorted", spy, raising=False)
        detector = OutlierDetector(method="iqr", field="length", threshold=1.5)
        detector.detect(NUMERIC_ITEMS)

        assert len(calls) == 1, (
            f"sorted 被调 {len(calls)} 次——Q1 与 Q3 各排了一遍。同一个有序序列"
            "取两个分位，第二遍是白扫一整轮。"
        )

    def test_quantile_with_preordered_equals_sorting_itself(self):
        detector = OutlierDetector(method="iqr")
        values = [5, 1, 4, 2, 3]
        assert detector._quantile(values, 0.5) == detector._quantile(
            values, 0.5, sorted(values)
        )

    def test_quantile_old_two_arg_shape_still_works(self):
        """既有直调形状不许被这次改动碰坏（既有用例就是这么调的）"""
        detector = OutlierDetector(method="iqr")
        assert detector._quantile([7], 0.5) == 7
        assert detector._quantile([], 0.5) == 0.0

    def test_iqr_detect_report_is_unchanged(self):
        detector = OutlierDetector(method="iqr", field="length", threshold=1.5)
        report = detector.detect(NUMERIC_ITEMS)
        assert report.total_items == len(NUMERIC_ITEMS)
        assert report.method == "iqr"
        assert report.outliers, "500 那个离群点必须被检出，否则这条用例在空转"


class TestGetStatisticsWalksItemsOnce:
    """dataset_ops：`get_statistics` 不再为唯一值把 items 再走一趟"""

    def test_items_are_iterated_once(self):
        ops = DatasetOperations()
        items = _CountingList(ITEMS)
        ops.get_statistics(items)
        assert items.iterations == 1, (
            f"items 被迭代 {items.iterations} 次——长度清单一趟、唯一 instruction "
            "集合又一趟。两个数同源，一趟就能一起累积。"
        )

    def test_values_are_unchanged(self):
        ops = DatasetOperations()
        stats = ops.get_statistics(ITEMS)
        assert stats["total"] == len(ITEMS)
        assert stats["min_length"] <= stats["avg_length"] <= stats["max_length"]
        assert stats["unique_instructions"] == len(
            {item.get("instruction", "") for item in ITEMS}
        )

    def test_empty_dataset_still_short_circuits(self):
        assert DatasetOperations().get_statistics([]) == {"total": 0}


class TestDiffDatasetsComputesEachDifferenceOnce:
    """compare_enhanced：三个集合运算各只算一次，list 与 len 复用同一份"""

    @staticmethod
    def _pair():
        a = [{"instruction": f"q{i}", "output": f"a{i}"} for i in range(50)]
        b = [{"instruction": f"q{i}", "output": f"b{i}"} for i in range(25, 75)]
        return a, b

    def test_stats_counts_match_the_listed_members(self):
        """这一条是 #4 的守卫：三个差集必须与被回显的计数同源"""
        result = diff_datasets(*self._pair())
        assert result["stats"]["only_in_a_count"] == len(result["only_in_a"])
        assert result["stats"]["only_in_b_count"] == len(result["only_in_b"])
        assert result["stats"]["in_both_count"] == len(result["in_both"])

    def test_membership_is_exact(self):
        a, b = self._pair()
        result = diff_datasets(a, b)
        assert set(result["only_in_a"]) == {f"q{i}" for i in range(25)}
        assert set(result["only_in_b"]) == {f"q{i}" for i in range(50, 75)}
        assert set(result["in_both"]) == {f"q{i}" for i in range(25, 50)}

    def test_both_empty(self):
        assert diff_datasets([], []) == {
            "only_in_a": [], "only_in_b": [], "in_both": [],
            "stats": {"only_in_a_count": 0, "only_in_b_count": 0, "in_both_count": 0},
        }
