# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""CLI pipeline 命令与错误路径集成测试

augment / export / analyze / visualize 走管道，需把 config 的版本存储与
日志重定向到临时目录；另覆盖无命令、缺输入文件等错误路径的 exit 1。
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
def pipeline_sandbox(tmp_path, monkeypatch):
    """临时沙箱：config.yaml 的版本存储/日志指向 tmp，并写入数据集

    Args:
        tmp_path: pytest 临时目录
        monkeypatch: pytest fixture

    Returns:
        (数据集文件, 配置文件路径, 临时目录)
    """
    monkeypatch.chdir(tmp_path)
    text = (AI_DIR / "config.yaml").read_text(encoding="utf-8")
    text = (
        text.replace("storage_dir: data/versions",
                      f"storage_dir: {tmp_path / 'versions'}")
        .replace("file: app.log", f"file: {tmp_path / 'app.log'}")
        .replace("baseline_file: data/benchmark_baseline.json",
                 f"baseline_file: {tmp_path / 'baseline.json'}")
    )
    cfg = tmp_path / "config.yaml"
    cfg.write_text(text, encoding="utf-8")
    data = tmp_path / "data.json"
    data.write_text(json.dumps(SAMPLE_ITEMS, ensure_ascii=False), encoding="utf-8")
    return data, cfg, tmp_path


class TestPipelineCommands:
    def test_augment_degrades_without_model(self, pipeline_sandbox):
        """无模型密钥时 augment 需优雅降级（0 变体）而非崩溃"""
        data, cfg, tmp = pipeline_sandbox
        out_file = tmp / "augmented.json"
        out, err, code = run_cli(
            [
                "cli", "--config", str(cfg), "augment",
                "--input", str(data), "--output", str(out_file),
                "--no-quality", "--no-dedup", "--no-checkpoint",
            ]
        )
        assert code is None, err
        report = json.loads(out)
        assert report["input_count"] == 5
        assert report["quality_check"] is False
        assert out_file.exists()

    def test_export_multi_format(self, pipeline_sandbox):
        """export 需按请求格式各产出一个文件"""
        data, cfg, tmp = pipeline_sandbox
        out_dir = tmp / "exports"
        out, err, code = run_cli(
            [
                "cli", "--config", str(cfg), "export",
                "--input", str(data), "--output-dir", str(out_dir),
                "--formats", "jsonl", "csv",
            ]
        )
        assert code is None, err
        files = json.loads(out)
        assert set(files) == {"jsonl", "csv"}
        for path in files.values():
            assert Path(path).exists()

    def test_analyze_dataset(self, pipeline_sandbox):
        """analyze 需输出覆盖分析（类型分布、长度分布）"""
        data, cfg, _ = pipeline_sandbox
        out, err, code = run_cli(
            ["cli", "--config", str(cfg), "analyze", "--input", str(data)]
        )
        assert code is None, err
        report = json.loads(out)
        coverage = report["coverage_analysis"]
        assert coverage["total_items"] == 5
        assert "question_type_distribution" in coverage
        assert "length_distribution" in coverage

    def test_visualize_dataset(self, pipeline_sandbox):
        """visualize 在无图形依赖时需降级输出（不崩溃）"""
        data, cfg, tmp = pipeline_sandbox
        out_dir = tmp / "viz"
        out, err, code = run_cli(
            [
                "cli", "--config", str(cfg), "visualize",
                "--input", str(data), "--output-dir", str(out_dir),
            ]
        )
        assert code is None, err
        json.loads(out)  # 输出必须是可解析 JSON


class TestErrorPaths:
    def test_no_command_exits_1(self, pipeline_sandbox):
        """无子命令需打印帮助并 exit 1"""
        _, cfg, _ = pipeline_sandbox
        out, err, code = run_cli(["cli", "--config", str(cfg)])
        assert code == 1
        assert "命令" in out or "usage" in out.lower()

    def test_missing_input_file_exits_1(self, pipeline_sandbox):
        """输入文件缺失需以 exit 1 报告错误而非 traceback"""
        _, cfg, tmp = pipeline_sandbox
        out, err, code = run_cli(
            [
                "cli", "--config", str(cfg), "quality",
                "--input", str(tmp / "no_such.json"),
            ]
        )
        assert code == 1
        assert "错误" in err
        assert "Traceback" not in err
