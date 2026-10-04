# -*- coding: utf-8 -*-
"""L189 / B259：验证预设封闭清单下沉 SDK 直构面 + `rules` 的 falsy 假零。

改前 `DatasetValidator.__init__` 同格两个缺陷：

- ① falsy 假零——`if rules:` 把显式 `rules={}`（「零规则」的合法意图）读成
  「没传」，静默落到 preset 规则面：strict 档传 {} 拿到 basic 规则面
  （valid-but-falsy 值误读族，L144/L147/L149 先例）；
- ② 静默回落——未知 preset（拼错 strict/chat）静默回落 basic，规则面整档
  放宽不出声（checkpoint 症状族语义漂移档；L175/L176 封闭清单族同型）。

API 面（dataset_tools 白名单 400）与 CLI 面（argparse choices）早有收口，
本轮补 SDK 直构面：未知预设拒并列出全部合法取值（A77 清单权威住
`PRESET_RULES`，API 白名单共引派生、CLI 手写字面由本文件钉同一性）。
"""
import pathlib

import pytest

from augmentor.exceptions import DataValidationError
from augmentor.validation import DatasetValidator

ROOT = pathlib.Path(__file__).resolve().parents[2]


def test_empty_rules_dict_means_zero_rules_not_unset():
    """① `rules={}` 是零规则：一条 preset 规则都不套。

    判别探针：「hi」短于 strict 档 `min_instruction_length=5` 本会被 strict
    档标警告；零规则面（`rules={}`）同一数据必须零告警零错误——改前它被
    `if rules:` 判空后静默走 preset，探针无法把 {} 与 preset 区分开。
    """
    data = [{"instruction": "hi", "output": "o"}]
    assert DatasetValidator(preset="strict").validate(data).warning_count > 0
    zero = DatasetValidator(rules={})
    assert zero.rules == {}
    assert zero.rules is not DatasetValidator.PRESET_RULES["basic"]
    result = zero.validate(data)
    assert result.error_count == 0
    assert result.warning_count == 0


def test_none_rules_use_preset_rules():
    """`rules` 缺省（None）+ 已知 preset = 该档规则浅拷贝，且不共享类级字典。"""
    v = DatasetValidator(preset="strict")
    assert v.rules == dict(DatasetValidator.PRESET_RULES["strict"])
    assert v.rules is not DatasetValidator.PRESET_RULES["strict"]
    v.rules["min_instruction_length"] = 999
    assert DatasetValidator.PRESET_RULES["strict"]["min_instruction_length"] == 5


# 字面清单（l97 判据：parametrize 值位必须是字面）
@pytest.mark.parametrize("bad_preset", ["typo", "DEFAULT", "Strict", "basic2", ""])
def test_unknown_presets_are_rejected_with_full_valid_list(bad_preset):
    """② 未知/空 preset 拒并列出全部合法取值——改前静默回落 basic。"""
    with pytest.raises(DataValidationError) as ei:
        DatasetValidator(preset=bad_preset)
    msg = str(ei.value)
    for name in DatasetValidator.PRESET_RULES:
        assert name in msg, f"报错文案必须列出全部合法取值，缺 {name!r}：{msg}"
    if bad_preset:
        assert bad_preset in msg


def test_none_preset_is_rejected():
    """显式传 None 不是「未传」——必填语义拒。"""
    with pytest.raises(DataValidationError):
        DatasetValidator(preset=None)


def test_api_face_whitelist_is_derived_from_sdk_keys():
    """A77 共引钉：API 路由的 `VALIDATION_PRESETS` 由 SDK 面 `PRESET_RULES`
    键派生（不是第二份手写字面清单）；源码级读法防回流成 `("basic", ...)`
    手抄副本。"""
    import api.routes.dataset_tools as tools

    assert tools.VALIDATION_PRESETS == tuple(DatasetValidator.PRESET_RULES)
    src = (ROOT / "api" / "routes" / "dataset_tools.py").read_text(encoding="utf-8")
    assert "VALIDATION_PRESETS = tuple(DatasetValidator.PRESET_RULES)" in src


def test_cli_preset_choices_equal_sdk_presets():
    """CLI 面 `--preset` choices（parser.py 不 import 业务模块、手写字面清单，
    与 EXPORT_FORMATS 同先例）必须与 SDK 面 PRESET_RULES 同一集合——
    任一面新增/退役预设，本例当场红。"""
    from augmentor.cli.parser import build_parser

    def _subparsers_action():
        for action in build_parser()._actions:
            if hasattr(action, "_get_subactions"):
                return action
        raise AssertionError("找不到 subparsers action")

    sub = _subparsers_action().choices["validate"]
    preset_action = next(a for a in sub._actions if a.dest == "preset")
    assert set(preset_action.choices) == set(DatasetValidator.PRESET_RULES)
