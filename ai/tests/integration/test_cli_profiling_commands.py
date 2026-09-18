"""CLI 画像与聚合命令族测试

profile / outliers / features / auto-config / aggregate 的参数组合、
策略分支、落盘与错误路径。
"""

import json
from pathlib import Path

import pytest

from cli import main

AI_DIR = Path(__file__).resolve().parent.parent.parent


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
def profiling_context(tmp_path, monkeypatch):
    """写入两组长度差异明显的数据集

    Args:
        tmp_path: pytest 临时目录
        monkeypatch: pytest fixture

    Returns:
        (长尾数据文件, 短数据文件, 临时目录)
    """
    monkeypatch.chdir(AI_DIR)
    items = [
        {"instruction": f"问题{c}？", "input": "", "output": "答" * (c + 1)}
        for c in range(1, 21)
    ] + [{"instruction": "超长问题" * 40, "input": "", "output": "长答" * 60}]
    a = tmp_path / "skewed.json"
    b = tmp_path / "short.json"
    a.write_text(json.dumps(items, ensure_ascii=False), encoding="utf-8")
    b.write_text(json.dumps(items[:4], ensure_ascii=False), encoding="utf-8")
    return a, b, tmp_path


class TestProfileCommand:
    def test_profile_saves_json(self, profiling_context):
        a, _, tmp = profiling_context
        out_file = tmp / "profile.json"
        out, err, code = run_cli(
            ["cli", "profile", "--input", str(a), "--output", str(out_file)]
        )
        assert code is None, err
        parsed = json.loads(out_file.read_text(encoding="utf-8"))
        assert parsed["total_items"] == 21

    def test_profile_missing_input_exits_1(self, profiling_context):
        a, _, tmp = profiling_context
        out, err, code = run_cli(
            ["cli", "profile", "--input", str(tmp / "nope.json")]
        )
        assert code == 1
        assert "错误" in err


class TestOutliersCommand:
    def test_outliers_zscore_detects_long_item(self, profiling_context):
        """长度长尾样本需被 zscore 判定为异常"""
        a, _, _ = profiling_context
        out, err, code = run_cli(["cli", "outliers", "--input", str(a)])
        assert code is None, err
        report = json.loads(out)
        assert report["outlier_count"] >= 1

    def test_outliers_iqr_method(self, profiling_context):
        """iqr 方法需正常返回方法标识"""
        a, _, _ = profiling_context
        out, err, code = run_cli(
            ["cli", "outliers", "--input", str(a), "--method", "iqr"]
        )
        assert code is None, err
        report = json.loads(out)
        assert report["method"] == "iqr"

    def test_outliers_saves_output(self, profiling_context):
        """--output 需写入可解析的检测结果"""
        a, _, tmp = profiling_context
        out_file = tmp / "outliers.json"
        out, err, code = run_cli(
            [
                "cli", "outliers", "--input", str(a),
                "--method", "zscore_one_sided", "--threshold", "2.0",
                "--output", str(out_file),
            ]
        )
        assert code is None, err
        parsed = json.loads(out_file.read_text(encoding="utf-8"))
        assert "outlier_count" in parsed


class TestFeaturesCommand:
    def test_features_detection(self, profiling_context):
        """特征检测需输出维度分布"""
        a, _, _ = profiling_context
        out, err, code = run_cli(["cli", "features", "--input", str(a)])
        assert code is None, err
        parsed = json.loads(out)
        assert parsed["total_items"] == 21


class TestAutoConfigCommand:
    def test_auto_config_recommendation(self, profiling_context):
        """自动配置推荐需给出去重阈值等建议"""
        a, _, _ = profiling_context
        out, err, code = run_cli(["cli", "auto-config", "--input", str(a)])
        assert code is None, err
        parsed = json.loads(out)
        assert "dedup" in json.dumps(parsed, ensure_ascii=False)

    def test_auto_config_saves_output(self, profiling_context):
        a, _, tmp = profiling_context
        out_file = tmp / "autocfg.json"
        out, err, code = run_cli(
            ["cli", "auto-config", "--input", str(a), "--output", str(out_file)]
        )
        assert code is None, err
        assert out_file.exists()


class TestAggregateCommand:
    def test_aggregate_union(self, profiling_context):
        a, b, tmp = profiling_context
        out_file = tmp / "agg.json"
        out, err, code = run_cli(
            [
                "cli", "aggregate",
                "--inputs", str(a), str(b),
                "--output", str(out_file), "--strategy", "union",
            ]
        )
        assert code is None, err
        parsed = json.loads(out)
        assert parsed["aggregated_count"] >= 6

    def test_aggregate_consistent_reports_conflicts(self, profiling_context):
        """两数据集同 key 不同 value 需在 consistent 策略下产生冲突记录"""
        a, _, tmp = profiling_context
        conflict_src = tmp / "conflict_src.json"
        conflict_src.write_text(
            json.dumps(
                [
                    {"instruction": "问题1？", "input": "", "output": "完全不同的答案"},
                    {"instruction": "全新问题", "input": "", "output": "新答案"},
                ],
                ensure_ascii=False,
            ),
            encoding="utf-8",
        )
        out_file = tmp / "agg_cons.json"
        out, err, code = run_cli(
            [
                "cli", "aggregate",
                "--inputs", str(a), str(conflict_src),
                "--output", str(out_file), "--strategy", "consistent",
            ]
        )
        assert code is None, err
        parsed = json.loads(out)
        assert len(parsed["conflicts"]) >= 1
