# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""CLI 数据集工具命令集成测试

覆盖 sample / split / stats / validate / convert / search 命令，
以及 `analyze` / `visualize` 两个曾静默失败的命令的回归
（3.0 起 `--enhanced` 已移除，改为守规范命令名本身）。
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


@pytest.fixture
def dataset(tmp_path, monkeypatch):
    """在临时目录写入 5 条样本数据集

    Args:
        tmp_path: pytest 临时目录
        monkeypatch: pytest fixture

    Returns:
        数据集文件路径
    """
    monkeypatch.chdir(AI_DIR)
    path = tmp_path / "dataset.json"
    path.write_text(json.dumps(SAMPLE_ITEMS, ensure_ascii=False), encoding="utf-8")
    return path


def run_cli(argv):
    """以给定参数运行 main()，返回 (stdout, 解析结果, exit_code)

    Args:
        argv: 完整参数列表（含程序名）

    Returns:
        (stdout 文本, JSON 解析结果或 None, SystemExit 码或 None)
    """
    import io
    import sys
    from contextlib import redirect_stdout

    old_argv = sys.argv
    sys.argv = argv
    buffer = io.StringIO()
    exit_code = None
    try:
        with redirect_stdout(buffer):
            main()
    except SystemExit as exc:
        exit_code = exc.code
    finally:
        sys.argv = old_argv
    content = buffer.getvalue()
    try:
        parsed = json.loads(content)
    except json.JSONDecodeError:
        parsed = None
    return content, parsed, exit_code


class TestSampleCommand:
    def test_sample_by_ratio(self, dataset, tmp_path):
        """按比例采样需产出确定数量（0.4 * 5 = 2 条）"""
        out = tmp_path / "sampled.json"
        out2, parsed, code = run_cli(
            [
                "cli", "sample", "--input", str(dataset),
                "--output", str(out), "--ratio", "0.4", "--seed", "7",
            ]
        )
        assert code is None
        items = json.loads(out.read_text(encoding="utf-8"))
        assert len(items) == 2
        assert parsed["output_count"] == 2

    def test_sample_fixed_size(self, dataset, tmp_path):
        """固定数量采样需严格输出该数量"""
        out = tmp_path / "sampled2.json"
        out2, parsed, code = run_cli(
            [
                "cli", "sample", "--input", str(dataset),
                "--output", str(out), "--size", "3", "--method", "stratified",
            ]
        )
        assert code is None
        assert len(json.loads(out.read_text(encoding="utf-8"))) == 3


class TestSplitCommand:
    def test_split_three_way(self, dataset, tmp_path):
        """0.6/0.2/0.2 三分割需落盘三个文件且计数守恒"""
        out_dir = tmp_path / "splits"
        out2, parsed, code = run_cli(
            [
                "cli", "split", "--input", str(dataset),
                "--output-dir", str(out_dir),
                "--train-ratio", "0.6", "--val-ratio", "0.2", "--test-ratio", "0.2",
            ]
        )
        assert code is None
        splits = parsed["splits"]
        total = sum(s["count"] for s in splits.values())
        assert total == len(SAMPLE_ITEMS)
        for name, meta in splits.items():
            assert Path(meta["file"]).exists()


class TestStatsCommand:
    def test_stats_prints_field_statistics(self, dataset):
        """统计输出需含总量与逐字段统计段（stdout 为可解析 JSON）"""
        out, parsed, code = run_cli(["cli", "stats", "--input", str(dataset)])
        assert code is None
        assert parsed["total_items"] == 5
        assert "instruction" in parsed["field_statistics"]
        assert parsed["field_statistics"]["instruction"]["unique_count"] == 5


class TestValidateCommand:
    def test_validate_strict_preset(self, dataset, tmp_path):
        """strict 预设需生成结果文件并报告通过/未通过状态"""
        out_file = tmp_path / "validation.json"
        out2, parsed, code = run_cli(
            [
                "cli", "validate", "--input", str(dataset),
                "--preset", "strict", "--output", str(out_file),
            ]
        )
        assert code is None
        result = json.loads(out_file.read_text(encoding="utf-8"))
        assert result["total_items"] == 5

    def test_validate_reports_invalid_data(self, dataset, tmp_path):
        """缺字段的脏数据需被 strict 预设判为无效"""
        bad = tmp_path / "bad.json"
        bad.write_text(json.dumps([{"instruction": ""}], ensure_ascii=False), encoding="utf-8")
        out_file = tmp_path / "bad_validation.json"
        out2, parsed, code = run_cli(
            [
                "cli", "validate", "--input", str(bad),
                "--preset", "strict", "--output", str(out_file),
            ]
        )
        assert code is None
        result = json.loads(out_file.read_text(encoding="utf-8"))
        assert result["is_valid"] is False


class TestConvertCommand:
    @pytest.mark.parametrize("fmt", ["csv", "alpaca", "chatml"])
    def test_convert_formats(self, dataset, tmp_path, fmt):
        """主流目标格式转换需成功且条数守恒"""
        out_file = tmp_path / f"converted.{fmt}"
        out2, parsed, code = run_cli(
            [
                "cli", "convert", "--input", str(dataset),
                "--output", str(out_file), "--format", fmt,
            ]
        )
        assert code is None
        assert parsed["input_count"] == 5
        assert parsed["output_count"] == 5
        assert out_file.exists()


class TestSearchCommand:
    def test_search_contains_limit(self, dataset, tmp_path):
        """contains 搜索需按 limit 截断返回条目，且条目以 JSON 列表打在 stdout

        stdout 契约沿用合并前的基础实现：两行摘要 + 匹配条目的 JSON 列表。
        条目就是这个命令的产物，只打印摘要会逼下游改用 `--output` 落盘再读。
        """
        out_file = tmp_path / "search.json"
        out, parsed, code = run_cli(
            [
                "cli", "search", "--input", str(dataset),
                "--query", "租", "--limit", "2",
                "--method", "contains", "--field", "instruction",
                "--output", str(out_file),
            ]
        )
        assert code is None
        assert "找到" in out
        # 摘要两行之后是 JSON 列表，需截取列表段解析
        items = json.loads(out[out.index("["):])
        assert len(items) == 2

        result = json.loads(out_file.read_text(encoding="utf-8"))
        assert result["total_matches"] == 4, "全量匹配数应为 4"
        assert len(result["items"]) == 2, "limit=2 未截断返回条目"

    def test_search_exact_no_match(self, dataset):
        """exact 方法搜索不存在的问题应得 0 匹配"""
        out, parsed, code = run_cli(
            [
                "cli", "search", "--input", str(dataset),
                "--query", "不存在的长问题", "--method", "exact",
            ]
        )
        assert code is None
        assert "找到 0 条" in out


class TestAnalyzeVisualizeRegression:
    """`analyze` / `visualize` 曾因分发名不匹配而静默无输出。

    3.0 起 `--enhanced` 已移除，这里改为守规范命令名本身仍有输出。
    """

    def test_analyze_prints_report(self, dataset):
        out, parsed, code = run_cli(["cli", "analyze", "--input", str(dataset)])
        assert code is None
        assert parsed["dataset_size"] == 5
        assert parsed["scores"]["quality"] >= 0
        # 合并不得丢掉旧基础实现的覆盖分析
        assert parsed["coverage_analysis"]["total_items"] == 5

    def test_analyze_saves_output(self, dataset, tmp_path):
        out_file = tmp_path / "analysis.json"
        out, parsed, code = run_cli(
            ["cli", "analyze", "--input", str(dataset), "--output", str(out_file)]
        )
        assert code is None
        saved = json.loads(out_file.read_text(encoding="utf-8"))
        assert saved["dataset_size"] == 5
        assert "coverage_analysis" in saved

    def test_visualize_report_mode_json(self, dataset, tmp_path):
        """`visualize --output --format json` 需写出可解析 JSON 报告"""
        out_file = tmp_path / "vis.json"
        out, parsed, code = run_cli(
            ["cli", "visualize", "--input", str(dataset),
             "--output", str(out_file), "--format", "json"]
        )
        assert code is None
        report = json.loads(out_file.read_text(encoding="utf-8"))
        assert report["total_items"] == 5
