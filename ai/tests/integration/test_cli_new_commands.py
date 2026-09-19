"""CLI 新命令集成测试

profile / outliers / features / auto-config / aggregate 五个新命令
必须能在真实文件输入下完成端到端流程。
"""

import json

import pytest

from cli import main


@pytest.fixture(autouse=True)
def chdir_ai_dir(tmp_path, monkeypatch, sample_items):
    """将工作目录切到含 config.yaml 的目录并写入数据文件

    Args:
        tmp_path: 临时目录
        monkeypatch: pytest fixture
        sample_items: 样本数据
    """
    import os
    import sys
    from pathlib import Path

    ai_dir = Path(__file__).resolve().parent.parent.parent
    monkeypatch.chdir(ai_dir)

    # 写入数据文件
    data_file = tmp_path / "cli_data.json"
    data_file.write_text(
        json.dumps(sample_items, ensure_ascii=False), encoding="utf-8"
    )

    monkeypatch.setattr(sys, "argv", ["cli", "profile", "--input", str(data_file)])
    yield data_file, tmp_path


@pytest.fixture
def sample_items():
    return [
        {"instruction": "如何申请租房？", "input": "", "output": "登录官网申请"},
        {"instruction": "租房多少钱？", "input": "", "output": "按房型定价"},
        {"instruction": "如何退租押金？", "input": "", "output": "满一年后退还"},
        {"instruction": "可以申请月付吗？", "input": "", "output": "支持月付"},
        {"instruction": "租期最短多久？", "input": "", "output": "一个月起租"},
    ]


def run_cli(tmp_path, argv):
    """以指定参数运行 CLI main() 并捕获 stdout

    Args:
        tmp_path: 临时目录
        argv: 参数列表

    Returns:
        (stdout 内容, 解析后的 JSON)
    """
    import sys
    import io
    from contextlib import redirect_stdout

    old_argv = sys.argv
    sys.argv = argv
    buffer = io.StringIO()
    try:
        with redirect_stdout(buffer):
            main()
    finally:
        sys.argv = old_argv
    content = buffer.getvalue()
    try:
        parsed = json.loads(content)
    except json.JSONDecodeError:
        parsed = None
    return content, parsed


class TestProfileCommand:
    def test_profile_prints_report(self, chdir_ai_dir, sample_items, tmp_path):
        data_file, _ = chdir_ai_dir
        content, parsed = run_cli(
            tmp_path, ["cli", "profile", "--input", str(data_file)]
        )
        assert parsed is not None
        assert parsed["total_items"] == 5
        assert "field_completeness" in parsed

    def test_profile_saves_file(self, chdir_ai_dir, sample_items, tmp_path):
        data_file, tmp = chdir_ai_dir
        output = str(tmp / "profile.json")
        run_cli(tmp, ["cli", "profile", "--input", str(data_file), "--output", output])
        with open(output, encoding="utf-8") as f:
            saved = json.load(f)
        assert saved["total_items"] == 5


class TestOutliersCommand:
    def test_outliers_default_zscore(self, chdir_ai_dir, sample_items, tmp_path):
        data_file, _ = chdir_ai_dir
        content, parsed = run_cli(
            tmp_path, ["cli", "outliers", "--input", str(data_file)]
        )
        assert parsed is not None
        assert parsed["method"] == "zscore"
        assert "total_items" in parsed

    def test_outliers_iqr(self, chdir_ai_dir, sample_items, tmp_path):
        data_file, _ = chdir_ai_dir
        content, parsed = run_cli(
            tmp_path, ["cli", "outliers", "--input", str(data_file),
                       "--method", "iqr", "--threshold", "1.5"]
        )
        assert parsed["method"] == "iqr"


class TestFeaturesCommand:
    def test_features_prints_report(self, chdir_ai_dir, sample_items, tmp_path):
        data_file, _ = chdir_ai_dir
        content, parsed = run_cli(
            tmp_path, ["cli", "features", "--input", str(data_file)]
        )
        assert parsed is not None
        assert parsed["total_items"] == 5
        assert "field_features" in parsed
        assert "intent_distribution" in parsed

    def test_features_saves_file(self, chdir_ai_dir, sample_items, tmp_path):
        data_file, tmp = chdir_ai_dir
        output = str(tmp / "features.json")
        run_cli(tmp, ["cli", "features", "--input", str(data_file), "--output", output])
        with open(output, encoding="utf-8") as f:
            saved = json.load(f)
        assert saved["total_items"] == 5


class TestAutoConfigCommand:
    def test_auto_config_recommendation(self, chdir_ai_dir, sample_items, tmp_path):
        data_file, _ = chdir_ai_dir
        content, parsed = run_cli(
            tmp_path, ["cli", "auto-config", "--input", str(data_file)]
        )
        assert parsed is not None
        assert "dedup_threshold" in parsed
        assert "quality_threshold" in parsed
        assert "recommended_sample_size" in parsed


class TestAggregateCommand:
    def test_aggregate_union_writes_output(self, chdir_ai_dir, sample_items, tmp_path):
        data_file, tmp = chdir_ai_dir
        second = tmp / "cli_data2.json"
        second.write_text(
            json.dumps(sample_items[:2], ensure_ascii=False), encoding="utf-8"
        )
        output = str(tmp / "aggregated.json")
        content, parsed = run_cli(
            tmp,
            ["cli", "aggregate", "--inputs", str(data_file), str(second),
             "--output", output, "--strategy", "union"],
        )
        assert parsed is not None
        assert "aggregated_count" in parsed
        with open(output, encoding="utf-8") as f:
            aggregated = json.load(f)
        # 前 2 条与第二个源重复，并集应为 5 条
        assert len(aggregated) == 5

    def test_aggregate_consistent_reports_conflicts(self, chdir_ai_dir, tmp_path):
        data_file, tmp = chdir_ai_dir
        # 构造冲突源：同一问题不同答案
        conflict = tmp / "conflict.json"
        conflict.write_text(
            json.dumps(
                [
                    {"instruction": "如何申请租房？", "output": "不同答案"},
                ],
                ensure_ascii=False,
            ),
            encoding="utf-8",
        )
        output = str(tmp / "agg_conf.json")
        content, parsed = run_cli(
            tmp,
            ["cli", "aggregate", "--inputs", str(data_file), str(conflict),
             "--output", output, "--strategy", "consistent"],
        )
        assert parsed is not None
        # 冲突样本被排除，保留一致部分
        assert "conflicts" in parsed
