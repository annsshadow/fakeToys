# -*- coding: utf-8 -*-
"""L172 / B242：全仓 10 条未覆盖语句收尾（配置面死码清除后余下的真缺口）。

删掉 config_validator 的判决维死分支（L171）后，coverage 余下 6 条真实
未覆盖语句，全部是**能造成问题的活分支**——

- `atomic_write_json` / `atomic_write_text` 的 finally 清理：写失败时临时件
  不残留（旧实现的半份窗口防护的一半）；
- `quality_trend` 检疫备份名碰撞：同一秒内两个损坏备份文件时名字不互相覆盖；
- `validation.require_ratio_list` 的非数值元素：`[1, "abc"]` 逐元素拒；
- `config_validator._validate_quality_weights` 的异常承接分支：
  `quality.weights` 给非清单/坏清单时由专项回放（不是被删的死码）接住。
"""
import json
import os
import pathlib

import pytest

from augmentor.atomic_write import atomic_write_json, atomic_write_text
from augmentor.quality_trend import QualityTrendTracker
from augmentor.validation import require_ratio_list
from augmentor.exceptions import DataValidationError
from augmentor.config_validator import ConfigValidator

_AW = pathlib.Path(__file__).resolve().parents[2] / "augmentor" / "atomic_write.py"


def test_json_writer_cleans_up_temp_file_on_failure(tmp_path, monkeypatch):
    """序列化/写盘抛错：目标保持原样、临时件被 finally 清掉。"""
    target = tmp_path / "ds.json"
    target.write_text("[1]", encoding="utf-8")

    class Unserializable:
        def __repr__(self):
            return "Unserializable"

    with pytest.raises(TypeError):
        atomic_write_json(target, [Unserializable()])
    assert target.read_text(encoding="utf-8") == "[1]", "旧内容必须完好"
    assert not [p for p in tmp_path.iterdir() if ".tmp-" in p.name], "临时件必须清掉"


def test_text_writer_cleans_up_temp_file_on_failure(tmp_path, monkeypatch):
    target = tmp_path / "out.jsonl"
    target.write_text("a\n", encoding="utf-8")

    def deny(*args, **kwargs):
        raise PermissionError("替换被拒（超重试上限）")

    monkeypatch.setattr("augmentor.atomic_write._replace_with_retry", deny)
    with pytest.raises(PermissionError):
        atomic_write_text(target, "b\n")
    monkeypatch.undo()
    assert target.read_text(encoding="utf-8") == "a\n"
    assert not [p for p in tmp_path.iterdir() if ".tmp-" in p.name]


def test_quarantine_backup_name_collision_is_not_clobbered(tmp_path):
    """同一秒内两个损坏文件：第二份备份名必须避让（-1/-2 递缀），不覆盖第一份。"""
    path = tmp_path / "trend.json"
    path.write_text("坏一", encoding="utf-8")
    tracker = QualityTrendTracker(storage_path=str(path))
    first = [p for p in tmp_path.iterdir() if ".corrupt-" in p.name]
    assert len(first) == 1, "第一次损坏必须留备份"

    # 制造第二秒内的第二个损坏文件：先把第一份备份读出来当「原件」再造一次损坏
    path.write_text("坏二", encoding="utf-8")
    tracker2 = QualityTrendTracker(storage_path=str(path))
    second = [p for p in tmp_path.iterdir() if ".corrupt-" in p.name]
    assert len(second) > len(first), "第二次损坏必须新增备份而不是覆盖"
    # 两份内容各自保留（没有互相覆盖成同一份）
    bodies = {p.read_text(encoding="utf-8") for p in second}
    assert "坏一" in bodies and "坏二" in bodies, bodies


def test_ratio_list_rejects_non_numeric_elements():
    """非数值元素（字符串）逐元素拒——不只 NaN/bool 两档。"""
    with pytest.raises(DataValidationError) as ei:
        require_ratio_list("quality.weights", [1, "abc", 0])
    assert "[1]" in str(ei.value), str(ei.value)


def test_validate_config_face_catches_bad_weights_through_the_dedicated_replay():
    """`quality.weights` 给坏清单时，validate_config 由**专项回放**（非已删的死码）接住。"""
    result = ConfigValidator().validate_config(
        {"models": {"default": "m", "m": {"type": "openai", "api_key": "k"}},
         "quality": {"weights": ["a", "b"]}}
    )
    weights_errors = [e for e in result.errors if e.path == "quality.weights"]
    assert weights_errors, "坏 weights 必须在 validate_config 面上报"
