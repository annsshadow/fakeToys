# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""CLI `health-gate` 子命令集成测试

这条命令的全部价值在**退出码**：CI 里一句 `augmentor health-gate --input ds.json`
就能把「这份数据能不能进训练」变成关卡。所以断言的重心不是 stdout 好不好看，而是
判定与退出码必须一致——判负却返回 0 的关卡等于没有关卡。
"""

import io
import json
from contextlib import redirect_stdout
from pathlib import Path

import pytest

from cli import main

AI_DIR = Path(__file__).resolve().parent.parent.parent

CLEAN = [
    {"instruction": f"问题{i}？", "input": "", "output": f"回答{i}"}
    for i in range(5)
]


def _write(tmp_path, items, name="ds.json"):
    """把条目写成临时数据集并返回路径字符串"""
    path = tmp_path / name
    path.write_text(json.dumps(items, ensure_ascii=False), encoding="utf-8")
    return str(path)


def _run(argv):
    """跑一次 CLI，返回 (退出码, stdout 的 JSON)

    退出码 0 = 走完且门禁放行；1 = 门禁判负或参数/数据非法（`main()` 的兜底）；
    2 = argparse 拒绝。
    """
    import sys

    old_argv = sys.argv
    buffer = io.StringIO()
    code = 0
    try:
        sys.argv = ["cli"] + argv
        with redirect_stdout(buffer):
            main()
    except SystemExit as exc:
        code = exc.code if isinstance(exc.code, int) else 1
    finally:
        sys.argv = old_argv

    content = buffer.getvalue()
    try:
        parsed = json.loads(content)
    except json.JSONDecodeError:
        parsed = None
    return code, parsed


@pytest.fixture(autouse=True)
def _chdir_ai_dir(monkeypatch):
    """命令需要 `config.yaml`，与其余 CLI 测试一样切到仓库根"""
    monkeypatch.chdir(AI_DIR)


class TestHealthGateExitCode:
    def test_healthy_dataset_exits_zero(self, tmp_path):
        code, parsed = _run(["health-gate", "--input", _write(tmp_path, CLEAN)])
        assert code == 0
        assert parsed["gate"]["verdict"] == "passed"
        assert parsed["gate"]["failed_rules"] == []

    def test_failed_gate_exits_one_but_still_prints_detail(self, tmp_path):
        """判负必须退出 1，同时把明细打出来，否则 CI 只知道挂了、不知道挂在哪条

        `--pass-rate 0.1` 是「上游算出的通过率低于门禁下限」的直接用法。
        """
        code, parsed = _run(
            ["health-gate", "--input", _write(tmp_path, CLEAN), "--pass-rate", "0.1"]
        )
        assert code == 1
        assert parsed["gate"]["verdict"] == "failed"
        assert parsed["gate"]["failed_rules"] == ["pass_rate"]
        assert parsed["skipped_rules"] == []

    def test_duplicate_heavy_dataset_fails_on_duplicate_rate(self, tmp_path):
        """重复率超上限时由 `duplicate_rate` 规则拦下（手算 3/4 = 0.75 > 0.3）"""
        items = [
            {"instruction": "同一个问题", "output": "a"},
            {"instruction": "同一个问题", "output": "b"},
            {"instruction": "同一个问题", "output": "c"},
            {"instruction": "另一个问题", "output": "d"},
        ]
        code, parsed = _run(["health-gate", "--input", _write(tmp_path, items)])
        assert code == 1
        assert parsed["gate"]["failed_rules"] == ["duplicate_rate"]
        assert parsed["gate"]["metrics"]["duplicate_rate"] == pytest.approx(0.75)

    def test_warning_does_not_block_unless_asked(self, tmp_path):
        """completeness 是 warning 级：默认放行（退出 0），`--block-on-warning` 才拦

        样本 2 条里 1 条缺 output → completeness 0.5 < 0.8。
        """
        items = [
            {"instruction": "问题一", "output": "回答一"},
            {"instruction": "问题二"},
        ]
        data = _write(tmp_path, items, "partial.json")

        code, parsed = _run(["health-gate", "--input", data])
        assert code == 0
        assert parsed["gate"]["verdict"] == "warned"
        assert parsed["gate"]["warned_rules"] == ["completeness"]

        code, parsed = _run(["health-gate", "--input", data, "--block-on-warning"])
        assert code == 1
        assert parsed["gate"]["verdict"] == "failed"


class TestHealthGateArguments:
    def test_text_field_is_actually_used(self, tmp_path):
        """`--text-field` 必须真的换掉重复率的口径

        instruction 全唯一（0.0）、output 全相同（1.0）：字段写死时两个分支会给出
        同一个答案，这个用例就失去区分力了。
        """
        items = [
            {"instruction": f"独立问题{i}", "output": "同一个回答"}
            for i in range(4)
        ]
        data = _write(tmp_path, items, "same_output.json")

        code, parsed = _run(["health-gate", "--input", data])
        assert code == 0
        assert parsed["gate"]["metrics"]["duplicate_rate"] == 0.0

        code, parsed = _run(["health-gate", "--input", data, "--text-field", "output"])
        assert code == 1
        assert parsed["gate"]["metrics"]["duplicate_rate"] == 1.0

    def test_weights_arity_rejected_by_argparse(self, tmp_path):
        """权重个数由 argparse 挡下（nargs=4），不必等到读文件"""
        code, parsed = _run(
            ["health-gate", "--input", _write(tmp_path, CLEAN),
             "--weights", "0.5", "0.5"]
        )
        assert code == 2
        assert parsed is None

    def test_weights_sum_rejected(self, tmp_path):
        """个数对、和不为 1 时由库层拒绝，CLI 以非零退出而不是默默打分"""
        code, parsed = _run(
            ["health-gate", "--input", _write(tmp_path, CLEAN),
             "--weights", "0.1", "0.2", "0.3", "0.9"]
        )
        assert code == 1
        assert parsed is None

    def test_valid_weights_are_echoed_into_health(self, tmp_path):
        """自定义权重要能一路走到健康分里（否则等价于参数被忽略）"""
        code, parsed = _run(
            ["health-gate", "--input", _write(tmp_path, CLEAN),
             "--weights", "0.1", "0.2", "0.3", "0.4"]
        )
        assert code == 0
        assert parsed["health"]["weights"] == [0.1, 0.2, 0.3, 0.4]

    def test_empty_dataset_is_an_error_not_a_pass(self, tmp_path):
        """空文件不能靠「重复率 0 ≤ 上限」拿到放行"""
        code, parsed = _run(["health-gate", "--input", _write(tmp_path, [])])
        assert code == 1
        assert parsed is None
