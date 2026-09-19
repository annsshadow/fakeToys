"""CLI 数据操作命令集成测试

覆盖 rag / benchmark / compare / merge 四个命令的端到端行为。
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
def two_datasets(tmp_path, monkeypatch):
    """写入两个（含重叠的）数据集文件

    Args:
        tmp_path: pytest 临时目录
        monkeypatch: pytest fixture

    Returns:
        (数据集A, 数据集B, 临时目录)
    """
    monkeypatch.chdir(AI_DIR)
    a = tmp_path / "dataset_a.json"
    b = tmp_path / "dataset_b.json"
    a.write_text(json.dumps(SAMPLE_ITEMS, ensure_ascii=False), encoding="utf-8")
    b.write_text(json.dumps(SAMPLE_ITEMS[:3], ensure_ascii=False), encoding="utf-8")
    return a, b, tmp_path


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


class TestRagCommand:
    """rag 命令：RAG 训练数据格式转换"""

    def test_rag_llamaindex_format(self, two_datasets):
        """llamaindex 格式需按分块规则拆分长文本"""
        a, _, tmp = two_datasets
        out = tmp / "rag.json"
        out2, parsed, code = run_cli(
            ["cli", "rag", "--input", str(a), "--output", str(out)]
        )
        assert code is None
        records = json.loads(out.read_text(encoding="utf-8"))
        assert len(records) >= len(SAMPLE_ITEMS)
        assert "已转换" in out2

    @pytest.mark.parametrize("fmt", ["langchain", "custom"])
    def test_rag_other_formats(self, two_datasets, fmt):
        """langchain / custom 格式转换需成功落盘且非空"""
        a, _, tmp = two_datasets
        out = tmp / f"rag_{fmt}.json"
        out2, parsed, code = run_cli(
            ["cli", "rag", "--input", str(a), "--output", str(out), "--format", fmt]
        )
        assert code is None
        records = json.loads(out.read_text(encoding="utf-8"))
        assert isinstance(records, list)
        assert len(records) > 0


class TestBenchmarkCommand:
    """benchmark 命令：质量基准与对比"""

    def test_benchmark_prints_metrics(self, two_datasets):
        """默认运行需输出四大基准指标"""
        a, _, _ = two_datasets
        out, parsed, code = run_cli(["cli", "benchmark", "--input", str(a)])
        assert code is None
        assert "pass_rate" in parsed["metrics"]
        assert "duplication_rate" in parsed["metrics"]

    def test_benchmark_save_and_compare_baseline(self, two_datasets):
        """save-baseline 后再跑一次需附带 comparisons 对比结果"""
        a, _, tmp = two_datasets
        baseline = tmp / "baseline.json"
        out2, parsed, code = run_cli(
            [
                "cli", "benchmark", "--input", str(a),
                "--baseline", str(baseline), "--save-baseline",
            ]
        )
        assert code is None
        assert baseline.exists()

        out3, parsed3, code3 = run_cli(
            ["cli", "benchmark", "--input", str(a), "--baseline", str(baseline)]
        )
        assert code3 is None
        assert "comparisons" in parsed3

    def test_benchmark_report_to_file(self, two_datasets):
        """--report 需生成可读的基准报告文件"""
        a, _, tmp = two_datasets
        report = tmp / "bench.md"
        out2, parsed, code = run_cli(
            ["cli", "benchmark", "--input", str(a), "--report", str(report)]
        )
        assert code is None
        assert report.exists()
        assert len(report.read_text(encoding="utf-8")) > 0


class TestCompareCommand:
    """compare 命令：数据集对比"""

    def test_compare_wins_larger_quality(self, two_datasets):
        """B 与 A 相比数量较少，摘要需给出明确结论行"""
        a, b, _ = two_datasets
        out, parsed, code = run_cli(
            ["cli", "compare", "--dataset-a", str(a), "--dataset-b", str(b)]
        )
        assert code is None
        assert "对比" in out or "##" in out

    def test_compare_saves_output(self, two_datasets):
        """--output 需写入对比报告文件"""
        a, b, tmp = two_datasets
        out_file = tmp / "compare.md"
        out, parsed, code = run_cli(
            [
                "cli", "compare",
                "--dataset-a", str(a), "--dataset-b", str(b),
                "--output", str(out_file),
            ]
        )
        assert code is None
        assert out_file.exists()


class TestMergeCommand:
    """merge 命令：多数据集合并"""

    def test_merge_dedup_by_default(self, two_datasets):
        """默认去重：A(5)+B(3 重叠) 合并后应剩 5 条"""
        a, b, tmp = two_datasets
        out_file = tmp / "merged.json"
        out2, parsed, code = run_cli(
            ["cli", "merge", "--inputs", str(a), str(b), "--output", str(out_file)]
        )
        assert code is None
        merged = json.loads(out_file.read_text(encoding="utf-8"))
        assert len(merged) == 5
        assert parsed["removed_duplicates"] >= 3

    def test_merge_without_dedup(self, two_datasets):
        """--no-dedup 时保留全部 8 条"""
        a, b, tmp = two_datasets
        out_file = tmp / "merged_raw.json"
        out2, parsed, code = run_cli(
            [
                "cli", "merge", "--inputs", str(a), str(b),
                "--output", str(out_file), "--no-dedup",
            ]
        )
        assert code is None
        merged = json.loads(out_file.read_text(encoding="utf-8"))
        assert len(merged) == 8
