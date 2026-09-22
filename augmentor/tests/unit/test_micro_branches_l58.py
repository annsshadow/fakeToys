# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""微型分支收尾：config_validator / context / evaluation 的残留未覆盖分支

每个用例都锁定一条明确的行为契约（读文件失败上报、嵌套结构安全递归、
越界取值报错、并行失败被吞并、边界参数返回 0），而非单纯凑覆盖率。
"""

import builtins

import pytest

from augmentor.config_validator import (
    ConfigValidator,
    ValidationResult,
    validate_config,
)
from augmentor.context import ContextAugmentor
from augmentor.evaluation import _lcs_length, compute_bleu, compute_rouge_l

BASE_VALID = {
    "app": {"name": "x", "version": "1.0"},
    "models": {"default": "ernie"},
}


class TestConfigValidatorBranches:
    def test_validate_file_read_error_reported(self, tmp_path, monkeypatch):
        """open() 抛出非 YAML 异常时，结果应为「读取文件错误」而非崩溃"""
        p = tmp_path / "cfg.yaml"
        p.write_text("app: {name: x, version: '1.0'}\nmodels: {default: ernie}\n",
                     encoding="utf-8")
        real_open = builtins.open

        def failing_open(file, *args, **kwargs):
            if str(file) == str(p):
                raise PermissionError("access denied")
            return real_open(file, *args, **kwargs)

        monkeypatch.setattr(builtins, "open", failing_open)
        result = ConfigValidator().validate_file(str(p))
        assert result.is_valid is False
        assert result.errors[0].path == "file"
        assert "读取文件错误" in result.errors[0].message

    def test_list_of_dicts_nested_recurses_safely(self):
        """未知字段下嵌套 dict 列表应安全递归且不产生误报"""
        config = dict(BASE_VALID, unknown_list=[{"a": 1}, {"b": 2}])
        result = validate_config(config)
        assert result.is_valid is True

    def test_int_above_max_reports_error(self):
        validator = ConfigValidator()
        result = ValidationResult(is_valid=True)
        validator._validate_int(101, {"min": 1, "max": 100},
                                "augmentation.variants_per_seed", result)
        assert result.is_valid is False
        assert "值过大" in result.errors[0].message

    def test_float_above_max_reports_error(self):
        validator = ConfigValidator()
        result = ValidationResult(is_valid=True)
        validator._validate_float(1.5, {"min": 0.0, "max": 1.0},
                                  "quality.threshold", result)
        assert result.is_valid is False
        assert "值过大" in result.errors[0].message


class TestContextParallelFailure:
    def test_parallel_batch_swallows_item_failures(self):
        """并行路径下单项失败应被记录并跳过，而不是炸掉整个批处理"""
        aug = ContextAugmentor(model_backend=None, max_workers=2)

        def boom(item, histories):
            raise RuntimeError("model down")

        aug.generate_multi_turn = boom
        items = [
            {"instruction": "a", "output": "1"},
            {"instruction": "b", "output": "2"},
        ]
        assert aug.batch_generate(items, use_parallel=True) == []


class TestEvaluationEdgeBranches:
    def test_lcs_length_empty_side(self):
        assert _lcs_length([], ["x"]) == 0
        assert _lcs_length(["x"], []) == 0

    def test_bleu_zero_max_n_returns_zero(self):
        assert compute_bleu("你好", "你好", max_n=0) == 0.0

    def test_rouge_l_perfect_match(self):
        assert compute_rouge_l("你好世界", "你好世界") == pytest.approx(1.0)
