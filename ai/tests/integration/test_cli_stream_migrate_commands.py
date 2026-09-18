"""CLI 流式/迁移/监控/自测/依赖命令集成测试

覆盖 stream / migrate / monitor / auto-test / dependency 五个命令，
其中 auto-test 曾因缺少 DatasetTestRunner 导入而 NameError，回归锁定。
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
def stream_dataset(tmp_path, monkeypatch):
    """写入样本数据集并切到 ai 目录（config.yaml 需可解析）

    Args:
        tmp_path: pytest 临时目录
        monkeypatch: pytest fixture

    Returns:
        (数据集文件, 临时目录)
    """
    monkeypatch.chdir(AI_DIR)
    data = tmp_path / "stream.json"
    data.write_text(json.dumps(SAMPLE_ITEMS, ensure_ascii=False), encoding="utf-8")
    return data, tmp_path


class TestStreamCommand:
    def test_stream_quality_operation(self, stream_dataset):
        """quality 操作需过滤低分样本并输出处理报告"""
        data, tmp = stream_dataset
        out_file = tmp / "streamed.json"
        out, err, code = run_cli(
            [
                "cli", "stream", "--input", str(data),
                "--output", str(out_file), "--operation", "quality",
                "--chunk-size", "2",
            ]
        )
        assert code is None, err
        report = json.loads(out)
        assert report["total_input"] == 5
        assert report["total_output"] <= 5
        assert out_file.exists()

    def test_stream_dedup_operation(self, stream_dataset):
        """dedup 操作对无重复数据应全量保留"""
        data, tmp = stream_dataset
        out_file = tmp / "streamed_dedup.json"
        out, err, code = run_cli(
            [
                "cli", "stream", "--input", str(data),
                "--output", str(out_file), "--operation", "dedup",
            ]
        )
        assert code is None, err
        report = json.loads(out)
        assert report["total_output"] == 5


class TestMigrateCommand:
    def test_migrate_default_rules(self, stream_dataset):
        """默认内置规则需完成迁移且无失败条目"""
        data, tmp = stream_dataset
        out_file = tmp / "migrated.json"
        out, err, code = run_cli(
            ["cli", "migrate", "--input", str(data), "--output", str(out_file)]
        )
        assert code is None, err
        assert "成功迁移: 5" in out
        migrated = json.loads(out_file.read_text(encoding="utf-8"))
        assert len(migrated) == 5

    def test_migrate_single_rule(self, stream_dataset):
        """指定单条规则需仅应用该规则（instruction→question 重命名）"""
        data, tmp = stream_dataset
        out_file = tmp / "migrated_rule.json"
        out, err, code = run_cli(
            [
                "cli", "migrate", "--input", str(data),
                "--output", str(out_file), "--rules", "rename_instruction",
            ]
        )
        assert code is None, err
        assert "应用规则: rename_instruction" in out
        migrated = json.loads(out_file.read_text(encoding="utf-8"))
        assert all("question" in item for item in migrated)
        assert all("instruction" not in item for item in migrated)


class TestMonitorCommand:
    def test_monitor_snapshot_metrics(self, stream_dataset):
        """monitor 需输出快照 ID 与三维指标"""
        data, _ = stream_dataset
        out, err, code = run_cli(["cli", "monitor", "--input", str(data)])
        assert code is None, err
        assert "快照ID" in out
        for metric in ("completeness", "diversity", "consistency"):
            assert metric in out

    def test_monitor_saves_report(self, stream_dataset):
        """--output 需写入快照 JSON"""
        data, tmp = stream_dataset
        report_file = tmp / "monitor.json"
        out, err, code = run_cli(
            ["cli", "monitor", "--input", str(data), "--output", str(report_file)]
        )
        assert code is None, err
        assert report_file.exists()


class TestAutoTestCommand:
    """回归：DatasetTestRunner 曾未导入导致 NameError exit 1"""

    def test_auto_test_runs(self, stream_dataset):
        data, _ = stream_dataset
        out, err, code = run_cli(["cli", "auto-test", "--input", str(data)])
        assert code is None, err
        assert "错误" not in err

    def test_auto_test_saves_suite(self, stream_dataset):
        data, tmp = stream_dataset
        out_file = tmp / "suite.json"
        out, err, code = run_cli(
            ["cli", "auto-test", "--input", str(data), "--output", str(out_file)]
        )
        assert code is None, err
        parsed = json.loads(out_file.read_text(encoding="utf-8"))
        assert isinstance(parsed, dict)


class TestDependencyCommand:
    def test_dependency_register_list_validate(self, stream_dataset):
        """注册 → 列表 → 校验全链路，注册表放临时目录"""
        data, tmp = stream_dataset
        registry = tmp / "dep_registry"
        out, err, code = run_cli(
            [
                "cli", "dependency", "--action", "register",
                "--input", str(data), "--name", "myds",
                "--registry-path", str(registry),
            ]
        )
        assert code is None, err
        assert "数据集注册成功" in out

        out2, err2, code2 = run_cli(
            ["cli", "dependency", "--action", "list", "--registry-path", str(registry)]
        )
        assert code2 is None, err2
        assert "myds" in out2

        out3, err3, code3 = run_cli(
            ["cli", "dependency", "--action", "validate", "--registry-path", str(registry)]
        )
        assert code3 is None, err3
        assert "验证通过" in out3

    def test_dependency_register_missing_args(self, stream_dataset):
        """register 缺 --name 需 exit 1"""
        data, tmp = stream_dataset
        out, err, code = run_cli(
            [
                "cli", "dependency", "--action", "register",
                "--input", str(data), "--registry-path", str(tmp / "reg"),
            ]
        )
        assert code == 1
        assert "必需" in err
