# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L226：页面上两张下拉清单，与 Python 侧的三处权威必须逐项相等

**为什么要建这把尺子**：L226 把 `search` / `validate` 接进分析页与质量中心时，方法档与
预设档都做成了 `<Select options={…}>`，而 TSX 那份字面量与 Python 的封闭清单之间
**没有任何编译期连接** —— 后端加一种检索方法、或把某个预设的上限改了，页面不会编译失败，
只会安静地少一档能力（用户选不到）或安静地多发一档非法值（用户选得到但必 400）。
这正是 A162 / L92 那次的形状：能力做完了，产品面只走完一半，而且缺的那一半没人能量出来。

**判据分三层**：

1. **跨语言漂移**（`TestSearchMethodParity` / `TestValidationPresetParity`）：双向相等 +
   顺序相等 + 扫描非空转。权威直接 import Python 常数，与 L92 同口径（搬在 Python 侧，
   因为 `web/@types` 里没有 node，跨语言读文件在 jsdom 侧要先动依赖表）。
2. **标签里的宣称**（`TestPresetLabelClaims`）：label 写了数字与字段名，那些数字**必须**
   就是规则表里的值 —— 否则用户在 UI 上看到的「上限 1000」是一句谎话。这一维只有真读
   `PRESET_RULES` 的值才判得了，比集合相等更严。
3. **端点面回执**（`TestEndpointAcceptsWhatTheUiSends`）：UI 会发的每一档（5 个方法 /
   3 个预设）在真实 TestClient + 真实中间件链下都回 200；清单外的一档必须回 **字符串**
   detail ⇒ 前端 `apiErrorDetail` 的主支不是想象出来的形状。

**与既有尺子的分工**（不重复已有的面）：
* `test_search_method_closed_list_l175.py` 管 SDK/API 共引同一个对象，**没有**管 CLI
  `--method` 那份手写字面（本文件补，见 `test_cli_method_choices_match_the_authority`）；
* `test_validation_preset_closed_list_l189.py` 已经钉了 CLI `--preset` choices 与
  `PRESET_RULES`、以及 `VALIDATION_PRESETS` 由 SDK 派生，本文件**不**再钉那两条，
  只补 TSX 那一头；
* `test_api_endpoint_consumption_l224.py` 数的是「这条操作被调到了吗」，不看下拉清单的
  **内容** —— 它绿着的时候这两个下拉框照样可以整档写错。
"""

import json
import pathlib
import re

import pytest

from api.routes.dataset_tools import SearchRequest, VALIDATION_PRESETS, ValidateRequest
from augmentor.cli.parser import build_parser
from augmentor.search_enhanced import SEARCH_METHODS
from augmentor.validation import DatasetValidator

SRC = pathlib.Path(__file__).resolve().parents[2] / "web" / "src"
ANALYSIS_TSX = SRC / "pages" / "Analysis.tsx"
QUALITY_TSX = SRC / "pages" / "Quality.tsx"

needs_web_src = pytest.mark.skipif(
    not (ANALYSIS_TSX.is_file() and QUALITY_TSX.is_file()),
    reason="前端源码不在场（只带构建产物的安装没有 web/）"
)

VALUE_RE = re.compile(r"value:\s*'([^']+)'")
LABEL_RE = re.compile(r"label:\s*'([^']+)'")


def strip_comments(source: str) -> str:
    """剥掉块注释与 `//` 行注释，只留代码

    口径与 `test_upload_declared_format_l92.py` 的同名工具一致（那边实测过：注释里原样
    写着被判的代码形状会把纯文本扫描骗过去）。本页面的常量上方就有一段解释性注释，
    不剥的话第一项命中可能是它。
    """
    return re.sub(r"/\*[\s\S]*?\*/", "", re.sub(r"^[ \t]*//.*$", "", source, flags=re.M))


def options_from(path: pathlib.Path, const_name: str) -> list:
    """抠出 `[{ value: 'x', label: '…' }, …]` 里的 `(value, label)` 对，保持原序"""
    block = re.search(rf"const {const_name} = \[(.*?)\]", strip_comments(
        path.read_text(encoding="utf-8")), re.S)
    assert block, f"{path.name} 里没有 {const_name} 常量 ⇒ 判据要跟着换形状"
    values = VALUE_RE.findall(block.group(1))
    labels = LABEL_RE.findall(block.group(1))
    assert len(values) == len(labels), f"{const_name} 里有项没写 label: {block.group(1)}"
    return list(zip(values, labels))


def _initial(path: pathlib.Path, state_name: str) -> str:
    """读 `const [x, setX] = useState('初值')` 里的那个字面量"""
    declared = re.search(
        rf"const \[{state_name}, set{state_name[0].upper()}{state_name[1:]}\] = "
        rf"useState\('(.*?)'\)",
        strip_comments(path.read_text(encoding="utf-8")))
    assert declared, f"{path.name} 里 {state_name} 的声明形状变了 ⇒ 本条要跟着换形状"
    return declared.group(1)


@pytest.fixture
def http_with_dataset(tmp_path, monkeypatch):
    """白名单数据根 + 一份 4 条的真实数据集（与 L92 的 fixture 同形）"""
    root = tmp_path / "ds"
    root.mkdir()
    items = [
        {"instruction": "请解释什么是向量数据库", "output": "向量数据库按相似度检索高维向量。"},
        {"instruction": "写一首关于秋天的诗", "output": "落叶铺满小径，秋天把颜色交给风。"},
        {"instruction": "什么是包含", "output": "包含判断是子串匹配。"},
        {"instruction": "正则怎么用", "output": "正则用元字符描述待匹配的形状。"},
    ]
    (root / "l226.json").write_text(json.dumps(items, ensure_ascii=False), encoding="utf-8")
    monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", str(root))
    monkeypatch.chdir(tmp_path)
    from fastapi.testclient import TestClient
    from api.main import app

    return TestClient(app), root


@needs_web_src
class TestSearchMethodParity:
    """分析页的方法下拉与检索侧权威：五档一档都不能差"""

    def _values(self):
        return [v for v, _ in options_from(ANALYSIS_TSX, "SEARCH_METHOD_OPTIONS")]

    def test_the_option_list_is_scanned_and_not_vacuous(self):
        """扫描本身要有效：只抠到一两项说明正则在空转，下面的相等断言就是摆设"""
        values = self._values()
        assert len(values) == len(set(values)) == len(SEARCH_METHODS), values

    def test_nothing_is_offered_that_the_search_side_cannot_run(self):
        """UI 选得到而检索侧不认 ⇒ 选了必 400，等于骗人"""
        extra = [m for m in self._values() if m not in SEARCH_METHODS]
        assert not extra, f"下拉框放开这些方法，但检索侧不认: {extra}"

    def test_every_search_method_can_be_picked_in_the_ui(self):
        """检索侧支持而 UI 选不到 ⇒ 那一档能力对用户不存在"""
        missing = [m for m in SEARCH_METHODS if m not in self._values()]
        assert not missing, f"检索侧支持这些方法，但页面选不到: {missing}"

    def test_the_order_matches_the_authority_table(self):
        """只比顺序不比内容：加一种方法时页面应当**跟着动**，而不是各自排序凑巧相等"""
        assert self._values() == list(SEARCH_METHODS)

    def test_cli_method_choices_match_the_authority(self):
        """CLI 面 `--method` 那份手写字面（parser.py 不 import 业务模块）也是第三个漂移面

        这一条是本文件新长出来的牙齿：`test_search_method_closed_list_l175.py` 钉的是
        SDK 与 API 共引同一个对象，CLI 那份字面清单从没被比过 —— 三个面各写一遍的时候，
        任何一个先动都不会有人报警。
        """
        sub = _subparsers()["search"]
        action = next(a for a in sub._actions if a.dest == "method")
        assert list(action.choices) == list(SEARCH_METHODS)

    def test_the_default_equals_the_contract_default(self):
        """页面初值必须等于请求模型的默认值，而不是清单的第一项

        TSX 那行注释写的是「初值与契约默认同值（`contains`）」，这句话是可判的：
        若两边各说各话，用户一进页面看到「包含子串」而服务端按另一档跑，两档结果的
        差异会直接被读成「检索不稳定」。
        """
        assert _initial(ANALYSIS_TSX, "searchMethod") == SearchRequest.model_fields["method"].default


@needs_web_src
class TestValidationPresetParity:
    """质量中心的预设下拉与验证侧权威：三档一档都不能差"""

    def _values(self):
        return [v for v, _ in options_from(QUALITY_TSX, "VALIDATION_PRESET_OPTIONS")]

    def test_the_option_list_is_scanned_and_not_vacuous(self):
        values = self._values()
        assert len(values) == len(set(values)) == len(VALIDATION_PRESETS), values

    def test_matches_the_api_face_whitelist_the_request_actually_hits(self):
        """相等判据要对着**前端请求真正经过的那道闸**：`VALIDATION_PRESETS`

        路由里 `if request.preset not in VALIDATION_PRESETS` 就是发错的代价；它与
        `PRESET_RULES` 的同一性由 `test_validation_preset_closed_list_l189.py` 钉，
        本文件不重复钉，只补 TSX 这一头。
        """
        assert self._values() == list(VALIDATION_PRESETS)

    def test_every_sdk_preset_can_be_picked_in_the_ui(self):
        missing = [p for p in DatasetValidator.PRESET_RULES if p not in self._values()]
        assert not missing, f"SDK 有这些预设，但页面选不到: {missing}"

    def test_the_default_equals_the_contract_default(self):
        """同上：`basic` 这个初值不是随手写的第一项，它就是路由的默认档

        这条特别有价值，因为 `default` 这个名字曾长期是 `ValidateRequest.preset` 的默认值
        而它根本不存在于预设表（见 L189 与下面那条 400 用例）。
        """
        assert _initial(QUALITY_TSX, "validationPreset") == ValidateRequest.model_fields["preset"].default


@needs_web_src
class TestPresetLabelClaims:
    """label 里写死的数字与字段名，必须就是规则表里的值"""

    def _labels(self):
        return dict(options_from(QUALITY_TSX, "VALIDATION_PRESET_OPTIONS"))

    def test_basic_labels_upper_limits_are_the_real_numbers(self):
        rules = DatasetValidator.PRESET_RULES["basic"]
        label = self._labels()["basic"]
        for number in (rules["max_instruction_length"], rules["max_output_length"]):
            assert str(number) in label, f"label {label!r} 里没有真实上限 {number}"

    def test_strict_label_claims_a_lower_limit_and_forbidden_patterns(self):
        """「加长度下限与禁用片段」这句话的两个承诺"""
        rules = DatasetValidator.PRESET_RULES["strict"]
        assert "min_instruction_length" in rules and "min_output_length" in rules
        assert rules.get("forbidden_patterns"), "strict 档已经不带禁用片段了 ⇒ label 在撒谎"

    def test_chat_label_claims_the_history_field(self):
        rules = DatasetValidator.PRESET_RULES["chat"]
        assert "history" in self._labels()["chat"], "label 少了字段名，本条判据要跟着换"
        assert "history" in rules["optional_fields"] + rules["required_fields"]


class TestEndpointAcceptsWhatTheUiSends:
    """端点回执：UI 会发的每一档真的收，清单外的一档给出前端能显示的判决"""

    @pytest.mark.parametrize("method", ["exact", "contains", "ngram", "fuzzy", "regex"])
    def test_every_listed_method_returns_200(self, http_with_dataset, method):
        http, _ = http_with_dataset
        response = http.post("/api/dataset/search",
                             json={"input_file": "l226.json", "query": "向量",
                                   "method": method})
        assert response.status_code == 200, response.text
        body = response.json()
        assert isinstance(body["total_matches"], int)
        assert body["method"] == method

    @pytest.mark.parametrize("preset", ["basic", "strict", "chat"])
    def test_every_listed_preset_returns_200(self, http_with_dataset, preset):
        http, _ = http_with_dataset
        response = http.post("/api/dataset/validate",
                             json={"input_file": "l226.json", "preset": preset})
        assert response.status_code == 200, response.text
        assert response.json()["total_items"] == 4

    def test_a_method_outside_the_list_is_a_string_detail_the_ui_can_show(self, http_with_dataset):
        http, _ = http_with_dataset
        response = http.post("/api/dataset/search",
                             json={"input_file": "l226.json", "query": "向量",
                                   "method": "substring"})
        assert response.status_code == 400, response.text
        detail = response.json()["detail"]
        assert isinstance(detail, str), type(detail)
        assert "substring" in detail and "ngram" in detail, detail

    def test_a_preset_outside_the_list_is_a_string_detail_the_ui_can_show(self, http_with_dataset):
        """"default" 是历史事故名：它根本不存在于预设表，却曾长期是路由的默认值"""
        http, _ = http_with_dataset
        response = http.post("/api/dataset/validate",
                             json={"input_file": "l226.json", "preset": "default"})
        assert response.status_code == 400, response.text
        detail = response.json()["detail"]
        assert isinstance(detail, str), type(detail)
        assert "default" in detail and "strict" in detail, detail


def _subparsers():
    for action in build_parser()._actions:
        if getattr(action, "choices", None) and "search" in action.choices:
            return action.choices
    raise AssertionError("找不到 subparsers action 或 search 子命令")
