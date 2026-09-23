# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""CLI 核心数据命令集成测试

覆盖 quality / preview / clean / annotate 四个离线命令的端到端行为：
参数组合、输出文件落盘、stdout JSON 结构。
"""

import json
from pathlib import Path

import pytest

from cli import main

AI_DIR = Path(__file__).resolve().parent.parent.parent


@pytest.fixture
def cli_context(tmp_path, monkeypatch):
    """切到含 config.yaml 的 ai 目录，返回 (样本文件, 临时目录)

    Args:
        tmp_path: pytest 临时目录
        monkeypatch: pytest fixture

    Yields:
        (样本数据文件路径, 临时目录)
    """
    items = [
        {"instruction": "如何申请租房？", "input": "", "output": "登录官网申请"},
        {"instruction": "租房多少钱？", "input": "", "output": "按房型定价"},
        {"instruction": "如何退租押金？", "input": "", "output": "满一年后退还"},
        {"instruction": "可以申请月付吗？", "input": "", "output": "支持月付"},
        {"instruction": "租期最短多久？", "input": "", "output": "一个月起租"},
    ]
    monkeypatch.chdir(AI_DIR)
    data_file = tmp_path / "cli_core_data.json"
    data_file.write_text(json.dumps(items, ensure_ascii=False), encoding="utf-8")
    return data_file, tmp_path


def run_cli(argv, extra_argv=None):
    """以给定参数运行 main()，返回 (stdout, 解析结果, exit_code)

    Args:
        argv: 完整参数列表（含程序名）
        extra_argv: 未使用占位，保持签名稳定

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


class TestQualityCommand:
    """quality 命令：评分、过滤、双格式报告"""

    def test_quality_json_report_to_file(self, cli_context):
        """JSON 报告需按 .json 扩展名落盘且可解析"""
        data_file, tmp = cli_context
        report_path = tmp / "quality.json"
        out, parsed, code = run_cli(
            ["cli", "quality", "--input", str(data_file), "--report", str(report_path)]
        )
        assert code is None
        assert report_path.exists()
        report = json.loads(report_path.read_text(encoding="utf-8"))
        assert report["total_samples"] == 5

    def test_quality_md_report_to_file(self, cli_context):
        """.md 扩展名必须生成 markdown 报告而非 JSON"""
        data_file, tmp = cli_context
        report_path = tmp / "quality.md"
        out, parsed, code = run_cli(
            ["cli", "quality", "--input", str(data_file), "--report", str(report_path)]
        )
        assert code is None
        content = report_path.read_text(encoding="utf-8")
        assert content.startswith("#")

    def test_quality_filters_output(self, cli_context):
        """--output 需保存通过质量检查的子集且数量小于原始"""
        data_file, tmp = cli_context
        out_path = tmp / "filtered.json"
        out, parsed, code = run_cli(
            ["cli", "quality", "--input", str(data_file), "--output", str(out_path)]
        )
        assert code is None
        filtered = json.loads(out_path.read_text(encoding="utf-8"))
        assert len(filtered) < 5
        assert all(item.get("output") for item in filtered)

    def test_quality_prints_when_no_output(self, cli_context):
        """不带 --output/--report 时直接打印 JSON 报告"""
        data_file, _ = cli_context
        out, parsed, code = run_cli(["cli", "quality", "--input", str(data_file)])
        assert code is None
        assert parsed["total_samples"] == 5
        assert "pass_rate" in parsed
        assert "score_distribution" in parsed


class TestPreviewCommand:
    """preview 命令：导出格式预览"""

    def test_preview_jsonl_default(self, cli_context):
        """默认 jsonl 格式预览需返回 original + converted 双数据"""
        data_file, _ = cli_context
        out, parsed, code = run_cli(
            ["cli", "preview", "--input", str(data_file), "--size", "2"]
        )
        assert code is None
        assert parsed["format"] == "jsonl"
        assert len(parsed["original_data"]) == 2
        assert len(parsed["converted_data"]) == 2

    def test_preview_custom_format(self, cli_context):
        """指定 alpaca 格式预览需转换字段结构"""
        data_file, _ = cli_context
        out, parsed, code = run_cli(
            ["cli", "preview", "--input", str(data_file), "--format", "alpaca", "--size", "1"]
        )
        assert code is None
        assert parsed["format"] == "alpaca"
        assert len(parsed["converted_data"]) == 1


class TestCleanCommand:
    """clean 命令：数据清洗"""

    def test_clean_removes_urls(self, tmp_path, monkeypatch):
        """含 URL 的条目需被标记噪声并保留（URL 被移除）"""
        items = [
            {"instruction": "请访问 http://example.com 了解更多", "input": "", "output": "好的"},
            {"instruction": "正常问题", "input": "", "output": "正常回答"},
        ]
        monkeypatch.chdir(AI_DIR)
        data_file = tmp_path / "with_url.json"
        data_file.write_text(json.dumps(items, ensure_ascii=False), encoding="utf-8")
        out_path = tmp_path / "cleaned.json"
        out, parsed, code = run_cli(
            ["cli", "clean", "--input", str(data_file), "--output", str(out_path)]
        )
        assert code is None
        cleaned = json.loads(out_path.read_text(encoding="utf-8"))
        joined = json.dumps(cleaned, ensure_ascii=False)
        assert "http://example.com" not in joined
        assert parsed["original_count"] == 2

    def test_clean_summary_counts_consistent(self, cli_context):
        """摘要计数需满足 清洗+移除 == 原始 的不变式"""
        data_file, _ = cli_context
        out_path = Path(data_file.parent / "cleaned2.json")
        out, parsed, code = run_cli(
            ["cli", "clean", "--input", str(data_file), "--output", str(out_path)]
        )
        assert code is None
        assert parsed["cleaned_count"] + parsed["removed_count"] == parsed["original_count"]


class TestAnnotateCommand:
    """annotate 命令：自动标注"""

    def test_annotate_distribution_keys(self, cli_context):
        """标注结果需包含实体数与意图/情感分布"""
        data_file, tmp = cli_context
        out_path = tmp / "annotated.json"
        out, parsed, code = run_cli(
            ["cli", "annotate", "--input", str(data_file), "--output", str(out_path)]
        )
        assert code is None
        assert parsed["total_items"] == 5
        assert "intent_distribution" in parsed
        assert "sentiment_distribution" in parsed
        assert out_path.exists()
