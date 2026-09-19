"""CLI 增强命令族集成测试

覆盖 clean-enhanced / export-enhanced / quality-report / search-enhanced /
stats-enhanced / compare-enhanced 六个增强命令。
"""

import json
from pathlib import Path

import pytest

from cli import main

AI_DIR = Path(__file__).resolve().parent.parent.parent

SAMPLE_ITEMS = [
    {"instruction": "如何申请租房？", "input": "", "output": "登录官网申请"},
    {"instruction": "租房多少钱？", "input": "", "output": "按房型定价"},
    {"instruction": "如何退租押金？", "input": "", "output": "满一年后退还"},
    {"instruction": "可以申请月付吗？", "input": "", "output": "支持月付"},
    {"instruction": "租期最短多久？", "input": "", "output": "一个月起租"},
]
DIRTY_ITEMS = SAMPLE_ITEMS + [
    {},  # 全空条目，remove_empty 规则应移除
]


def run_cli(argv):
    """以给定参数运行 main()，返回 (stdout, stderr, exit_code)

    Args:
        argv: 完整参数列表（含程序名）

    Returns:
        (stdout, stderr, SystemExit 码或 None)
    """
    import io
    import sys
    from contextlib import redirect_stderr, redirect_stdout

    old_argv = sys.argv
    sys.argv = argv
    out, err = io.StringIO(), io.StringIO()
    exit_code = None
    try:
        with redirect_stdout(out), redirect_stderr(err):
            main()
    except SystemExit as exc:
        exit_code = exc.code
    finally:
        sys.argv = old_argv
    return out.getvalue(), err.getvalue(), exit_code


@pytest.fixture
def enhanced_context(tmp_path, monkeypatch):
    """写入干净与脏两个数据集，切到 ai 目录

    Args:
        tmp_path: pytest 临时目录
        monkeypatch: pytest fixture

    Returns:
        (干净数据, 脏数据, 临时目录)
    """
    monkeypatch.chdir(AI_DIR)
    clean = tmp_path / "clean.json"
    dirty = tmp_path / "dirty.json"
    clean.write_text(json.dumps(SAMPLE_ITEMS, ensure_ascii=False), encoding="utf-8")
    dirty.write_text(json.dumps(DIRTY_ITEMS, ensure_ascii=False), encoding="utf-8")
    return clean, dirty, tmp_path


class TestCleanEnhancedCommand:
    def test_clean_enhanced_removes_dirty(self, enhanced_context):
        """脏数据（全空 output）需被清洗规则移除"""
        clean, dirty, tmp = enhanced_context
        out_file = tmp / "cleaned_enh.json"
        out, err, code = run_cli(
            [
                "cli", "clean-enhanced", "--input", str(dirty),
                "--output", str(out_file),
            ]
        )
        assert code is None, err
        items = json.loads(out_file.read_text(encoding="utf-8"))
        assert len(items) < len(DIRTY_ITEMS)

    def test_clean_enhanced_summary_consistent(self, enhanced_context):
        """摘要计数需满足 原始 = 清洗后 + 移除 的不变式"""
        _, dirty, _ = enhanced_context
        out_file = Path(dirty.parent / "cleaned_enh2.json")
        out, err, code = run_cli(
            [
                "cli", "clean-enhanced", "--input", str(dirty),
                "--output", str(out_file),
            ]
        )
        assert code is None, err
        import re
        m_orig = re.search(r"原始数据: (\d+)", out)
        m_clean = re.search(r"清洗后: (\d+)", out)
        m_removed = re.search(r"移除: (\d+)", out)
        assert int(m_orig.group(1)) == int(m_clean.group(1)) + int(m_removed.group(1))


class TestExportEnhancedCommand:
    @pytest.mark.parametrize("fmt", ["csv", "jsonl"])
    def test_export_enhanced_formats(self, enhanced_context, fmt):
        """增强导出需按格式落盘且条数正确"""
        clean, _, tmp = enhanced_context
        out_file = tmp / f"exp_enh.{fmt}"
        out, err, code = run_cli(
            [
                "cli", "export-enhanced", "--input", str(clean),
                "--output", str(out_file), "--format", fmt,
            ]
        )
        assert code is None, err
        assert out_file.exists()
        assert "导出数量: 5" in out

    def test_export_enhanced_max_items(self, enhanced_context):
        """--max-items 需截断导出数量"""
        clean, _, tmp = enhanced_context
        out_file = tmp / "exp_enh_trunc.json"
        out, err, code = run_cli(
            [
                "cli", "export-enhanced", "--input", str(clean),
                "--output", str(out_file), "--max-items", "2",
            ]
        )
        assert code is None, err
        assert "导出数量: 2" in out


class TestQualityReportCommand:
    def test_quality_report_markdown(self, enhanced_context):
        """markdown 格式报告需落盘且含标题"""
        clean, _, tmp = enhanced_context
        out_file = tmp / "qr.md"
        out, err, code = run_cli(
            [
                "cli", "quality-report", "--input", str(clean),
                "--output", str(out_file), "--format", "markdown",
            ]
        )
        assert code is None, err
        assert out_file.exists()
        assert "总体" in out or "评分" in out

    def test_quality_report_threshold_rejects(self, enhanced_context):
        """高阈值（0.99）下小样本应判为未通过"""
        clean, _, tmp = enhanced_context
        out, err, code = run_cli(
            [
                "cli", "quality-report", "--input", str(clean),
                "--threshold", "0.99",
            ]
        )
        assert code is None, err
        assert "未通过" in out or "通过" in out


class TestSearchEnhancedCommand:
    def test_search_enhanced_fuzzy(self, enhanced_context):
        """fuzzy 方法搜索需返回匹配摘要"""
        clean, _, tmp = enhanced_context
        out, err, code = run_cli(
            [
                "cli", "search-enhanced", "--input", str(clean),
                "--query", "租房", "--method", "fuzzy",
            ]
        )
        assert code is None, err
        assert "找到" in out

    def test_search_enhanced_saves_output(self, enhanced_context):
        """--output 需写入可解析的结果 JSON"""
        clean, _, tmp = enhanced_context
        out_file = tmp / "se.json"
        out, err, code = run_cli(
            [
                "cli", "search-enhanced", "--input", str(clean),
                "--query", "租房", "--output", str(out_file),
            ]
        )
        assert code is None, err
        parsed = json.loads(out_file.read_text(encoding="utf-8"))
        assert "total_matches" in parsed


class TestStatsEnhancedCommand:
    def test_stats_enhanced_field_stats(self, enhanced_context):
        """增强统计需输出字段填充率与总质量指标"""
        clean, _, tmp = enhanced_context
        out_file = tmp / "se_stats.json"
        out, err, code = run_cli(
            [
                "cli", "stats-enhanced", "--input", str(clean),
                "--output", str(out_file),
            ]
        )
        assert code is None, err
        assert "数据总量: 5" in out
        parsed = json.loads(out_file.read_text(encoding="utf-8"))
        assert parsed["total_items"] == 5


class TestCompareEnhancedCommand:
    def test_compare_enhanced_identical(self, enhanced_context):
        """同数据集自比较需 100% 相似且共同数=总量"""
        clean, _, tmp = enhanced_context
        out_file = tmp / "ce.json"
        out, err, code = run_cli(
            [
                "cli", "compare-enhanced",
                "--dataset-a", str(clean), "--dataset-b", str(clean),
                "--output", str(out_file),
            ]
        )
        assert code is None, err
        assert "100.00%" in out
        parsed = json.loads(out_file.read_text(encoding="utf-8"))
        assert parsed["metrics"]["common_items"] == 5
        assert parsed["metrics"]["unique_a"] == 0

    def test_compare_enhanced_different(self, enhanced_context):
        """脏 vs 干净需报告 unique_b/unique_a 非零"""
        clean, dirty, tmp = enhanced_context
        out, err, code = run_cli(
            [
                "cli", "compare-enhanced",
                "--dataset-a", str(clean), "--dataset-b", str(dirty),
            ]
        )
        assert code is None, err
        assert "仅在B中: 1" in out
