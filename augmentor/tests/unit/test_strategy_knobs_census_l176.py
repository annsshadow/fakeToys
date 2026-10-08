# -*- coding: utf-8 -*-
"""L176 / B246：策略旋钮「封闭清单拒」普查棘轮 + 质量报告 format 下沉判据。

L175 的搜索 method 静默回落修复后做同型普查：全仓用户可见的策略/方法旋钮
逐个实探——sample 的 method、outlier 的 method、expander 的 strategy 三处
**已有** else 拒（本次普查实证），唯一缺口是 `save_quality_report` 的 format
（未知 format 静默落 JSON 支，与 L175 同型语义漂移）——收边并把整个普查的
结论**钉成棘轮**：五个旋钮里任何一个未来退化回「静默回落 / 不拒未知值」，
本文件当场红。
"""
import pathlib

import pytest

from augmentor.dataset_ops import DatasetOperations, SampleConfig
from augmentor.exceptions import DataValidationError
from augmentor.expander import DomainExpander
from augmentor.outlier import OutlierDetector
from augmentor.quality_report import QUALITY_REPORT_FORMATS, save_quality_report
from augmentor.search_enhanced import SEARCH_METHODS, search_dataset

_ITEMS = [{"instruction": f"q{i}", "output": f"a{i}"} for i in range(10)]


def test_search_method_rejects_unknown():
    with pytest.raises(DataValidationError):
        search_dataset(_ITEMS, "q", method="not_a_method")


def test_sample_method_rejects_unknown():
    ops = DatasetOperations()
    with pytest.raises(DataValidationError):
        ops.sample(_ITEMS, config=SampleConfig(method="not_a_method", size=3))


def test_outlier_method_rejects_unknown():
    with pytest.raises(DataValidationError):
        OutlierDetector(method="not_a_method")


def test_expander_strategy_rejects_unknown():
    """expander 拒在 expand() 的 else 档（构造器不判）——探针必须走到分发点。"""
    expander = DomainExpander(model_backend=None)
    with pytest.raises(DataValidationError):
        expander.expand(_ITEMS, strategy="not_a_strategy")


def test_quality_report_format_rejects_unknown():
    class _R:
        def to_markdown(self):
            return "# x"

        def to_dict(self):
            return {"x": 1}

    import tempfile
    with tempfile.TemporaryDirectory() as d:
        with pytest.raises(DataValidationError) as ei:
            save_quality_report(_R(), f"{d}/r.out", format="yaml")
        # 文案必须列全合法取值（封闭清单可读性）
        for fmt in QUALITY_REPORT_FORMATS:
            assert fmt in str(ei.value)
        import json as _json
        save_quality_report(_R(), f"{d}/j.json", format="json")
        save_quality_report(_R(), f"{d}/m.md", format="markdown")
        with pytest.raises(DataValidationError):
            save_quality_report(_R(), f"{d}/n.out", format=None)


def test_all_five_knobs_share_one_shape_reject_unknowns():
    """普查棘轮的合体面：五个旋钮 × 各自一个未知值，全部必须出 DataValidationError。

    将来给任何一个旋钮加新取值而忘了同步拒未知值（或把 else 拒改成静默回落），
    这条会红——普查结论不许悄悄退化。
    """
    import tempfile

    probes = [
        ("search", lambda: search_dataset(_ITEMS, "q", method="bogus")),
        ("sample", lambda: DatasetOperations().sample(_ITEMS, config=SampleConfig(method="bogus", size=3))),
        ("outlier", lambda: OutlierDetector(method="bogus")),
        ("expander", lambda: DomainExpander(model_backend=None).expand(_ITEMS, strategy="bogus")),
        ("quality_report", lambda: save_quality_report(_R(), f"{tempfile.mkdtemp()}/r.out", format="bogus")),
    ]
    for name, probe in probes:
        try:
            probe()
        except DataValidationError:
            continue
        raise AssertionError(f"旋钮 {name!r} 退回静默回落了（未知值未被拒）")


class _R:
    def to_markdown(self):
        return "# x"

    def to_dict(self):
        return {"x": 1}
