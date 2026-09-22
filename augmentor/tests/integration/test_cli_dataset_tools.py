"""CLI 数据集工具命令集成测试

覆盖 sample / split / stats / validate / convert / search 命令，
以及 analyze-data / visualize-data 两个曾静默失败的命令的回归。
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
    def test_stats_summary_fields(self, dataset):
        """统计输出需包含总量与长度分布"""
        out2, parsed, code = run_cli(["cli", "stats", "--input", str(dataset)])
        assert code is None
        assert parsed["total"] == 5
        assert parsed["min_length"] <= parsed["avg_length"] <= parsed["max_length"]


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
    def test_search_contains_limit(self, dataset):
        """contains 搜索需按 limit 截断结果（JSON 位于 stdout 的列表段）"""
        out, parsed, code = run_cli(
            [
                "cli", "search", "--input", str(dataset),
                "--query", "租", "--limit", "2",
                "--method", "contains", "--field", "instruction",
            ]
        )
        assert code is None
        assert "找到" in out
        # search 命令先打印两行摘要再输出 JSON 列表，需截取列表段解析
        items = json.loads(out[out.index("["):])
        assert len(items) == 2

    def test_search_exact_no_match(self, dataset):
        """exact 方法搜索不存在的问题应得 0 匹配"""
        out, parsed, code = run_cli(
            [
                "cli", "search", "--input", str(dataset),
                "--query", "不存在的长问题", "--method", "exact",
            ]
        )
        assert code is None
        assert "0" in out


class TestLegacyDataCommandRegression:
    """analyze-data / visualize-data 曾因分发名不匹配而静默无输出"""

    def test_analyze_data_prints_report(self, dataset):
        """analyze-data 需打印数据集大小与质量分数"""
        out, parsed, code = run_cli(
            ["cli", "analyze", "--enhanced", "--input", str(dataset)]
        )
        assert code is None
        assert "数据集大小: 5" in out
        assert "质量分数" in out

    def test_analyze_data_saves_output(self, dataset, tmp_path):
        """analyze-data --output 需写入报告文件"""
        out_file = tmp_path / "analysis.json"
        out, parsed, code = run_cli(
            ["cli", "analyze", "--enhanced", "--input", str(dataset), "--output", str(out_file)]
        )
        assert code is None
        assert out_file.exists()

    def test_visualize_data_json_format(self, dataset):
        """visualize-data --format json 需输出可解析 JSON"""
        out, parsed, code = run_cli(
            ["cli", "visualize", "--enhanced", "--input", str(dataset), "--format", "json"]
        )
        assert code is None
        assert parsed["total_items"] == 5
