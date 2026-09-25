# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""`_generate_variants` 的解析健壮性与提示词转义（L59，A108 / A109）

这两条是「跳出 config 条带、回扫增强核心」扫出来的，判据来自读 `pipeline.py:158-193`
原文（不是印象）：

- **A108（功能）**：改前判据是 `if response.strip().startswith('['): json.loads(...)`。
  模型只要把数组包在 ```` ```json ```` 围栏里，或前面先说一句「好的，结果如下：」，
  首字符就不是 `[` ⇒ 整段响应被跳过，**静默返回 0 个变体，连 warning 都不发**。
  仓库里本就有更 robust 且各 model adapter 都在用的解析器 `models.base.extract_json_array`
  （直接解析 / 首个 `[` 到末个 `]` 的括号切片），pipeline 是唯一的手搓旁路。这里选它而不是
  `json_extract.extract_json_list`：后者的平衡括号扫描优先找 `{`，遇到「前置说明 + 对象数组」
  会先框住里层那个对象、再因非列表判 `not_list` 而返回空 —— 而流水线的产物恒为一个 JSON
  数组，括号切片正是这个语义下唯一正确的取法。无法解析时 `extract_json_array` 抛
  ModelResponseError，由 `except` 收成一行 warning（不再是无声的 0）。
- **A109（正确性）**：改前把原始 `instruction` 直接插进提示词里的 JSON 字面量
  `原JSON对象如下：[{{"instruction": "{instruction}"}}]`。instruction 里带一个 `"` 或换行，
  发给模型的 prompt 里那段 JSON 结构当场被撑破。改走 `json.dumps`。

两条都**不改**既有的三条契约：`model_backend is None` ⇒ `[]`、`"not json"` ⇒ `[]`、
合法数组 ⇒ 正常解析。本文件另加围栏 / 前置说明 / 含引号换行 / 变体数封顶四组新形状。
"""

import json
from pathlib import Path

import pytest

from augmentor.config import load_config
from augmentor.pipeline import AugmentorPipeline

AI_DIR = Path(__file__).resolve().parent.parent.parent


class RecordingBackend:
    """只实现 `generate` 的最小假后端：记下发出的 prompt，返回预设响应

    `_generate_variants` 只用到 `model_backend.generate(prompt)` 这一个方法，所以不
    必继承 `ModelBackend`（那会牵进缓存 / 退避 / 重试一整套），保持观察面最小。
    """

    def __init__(self, response):
        self._response = response
        self.last_prompt = None

    def generate(self, prompt):
        self.last_prompt = prompt
        return self._response


@pytest.fixture
def pipe():
    config = load_config(str(AI_DIR / "config.yaml"))
    config.augmentation.variants_per_seed = 3
    return AugmentorPipeline(config)


def _seed(instruction="原问题", output="原回答"):
    return {"instruction": instruction, "output": output}


class TestRobustParsing:
    """A108：围栏 / 前置说明 / 尾逗号都不该再把变体吞成 0"""

    def test_plain_array_still_parses(self, pipe):
        pipe.model_backend = RecordingBackend(
            json.dumps([{"instruction": "变体A"}, {"instruction": "变体B"}], ensure_ascii=False))
        got = pipe._generate_variants(_seed())
        assert [v["instruction"] for v in got] == ["变体A", "变体B"]
        assert all(v["output"] == "原回答" and v["input"] == "" for v in got)

    def test_fenced_json_no_longer_yields_zero_variants(self, pipe):
        body = json.dumps([{"instruction": "变体A"}, {"instruction": "变体B"}], ensure_ascii=False)
        pipe.model_backend = RecordingBackend(f"```json\n{body}\n```")
        got = pipe._generate_variants(_seed())
        assert len(got) == 2, "围栏响应被 startswith('[') 判据吞成了 0 个变体"

    def test_leading_prose_before_the_array_no_longer_yields_zero(self, pipe):
        body = json.dumps([{"instruction": "变体A"}], ensure_ascii=False)
        pipe.model_backend = RecordingBackend(f"好的，结果如下：\n{body}")
        got = pipe._generate_variants(_seed())
        assert [v["instruction"] for v in got] == ["变体A"]

    def test_variants_per_seed_cap_still_holds(self, pipe):
        many = json.dumps([{"instruction": f"v{i}"} for i in range(10)], ensure_ascii=False)
        pipe.model_backend = RecordingBackend(many)
        got = pipe._generate_variants(_seed())
        assert len(got) == pipe.config.augmentation.variants_per_seed

    def test_garbage_still_returns_empty(self, pipe):
        """既有的「not json ⇒ []」契约不许被 robust 解析反过来破坏"""
        pipe.model_backend = RecordingBackend("not json at all")
        assert pipe._generate_variants(_seed()) == []


class TestPromptEscaping:
    """A109：instruction 里的引号 / 换行不许撑破发给模型的 prompt JSON 结构"""

    def test_instruction_with_quote_and_newline_survives_the_prompt(self, pipe):
        nasty = '他说："你好\\n世界"\n第二行'
        backend = RecordingBackend(json.dumps([{"instruction": "变体"}], ensure_ascii=False))
        pipe.model_backend = backend
        pipe._generate_variants(_seed(instruction=nasty))
        tail = backend.last_prompt.rsplit("原JSON对象如下：", 1)[1]
        recovered = json.loads(tail)
        assert recovered == [{"instruction": nasty}], (
            "instruction 的引号 / 换行没被正确转义进 prompt")

    def test_no_backslash_literal_leaks_into_prompt_when_instruction_has_backslash(self, pipe):
        """改前手拼会把 `\\n` 当成两字符塞进 JSON；改后 dumps 保证它是可解析的转义"""
        nasty = "路径 C:\\temp\\x"
        backend = RecordingBackend("[]")
        pipe.model_backend = backend
        pipe._generate_variants(_seed(instruction=nasty))
        tail = backend.last_prompt.rsplit("原JSON对象如下：", 1)[1]
        assert json.loads(tail) == [{"instruction": nasty}]


class TestBackendNoneStillEmpty:
    """`model_backend is None ⇒ []` 这条既有契约一字不改"""

    def test_none_backend_returns_empty(self, pipe):
        pipe.model_backend = None
        assert pipe._generate_variants(_seed()) == []
