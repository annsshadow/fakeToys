# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""`DomainExpander` 四处模型响应解析的健壮性（L60，A110 —— A108 的同构孪生）

L59 修了 `pipeline._generate_variants`（A108）的手搓判据
`if response.strip().startswith('[')`。本轮 grep `startswith('[')` 扫出 `expander.py`
里还留着**四处同构**（三个主题生成器 + 一个种子生成器），且后果分两种：

- **主题生成器（`_generate_similar/related/scenario_topics`）**：改前有「按行分割」
  兜底 ⇒ 模型把数组包进 ` ```json ` 围栏或前面先说一句「好的，结果如下：」时首字符不是
  `[`，**掉进行分割、把 "```json" / '["租房","公寓"]' 这类碎片当成主题**塞进结果、污染
  下游（比 A108 的静默 0 更坏，是数据损坏）。
- **种子生成器（`_generate_seeds_for_topic`）**：提示词明说「格式为 JSON 数组」，改前无
  兜底 ⇒ 围栏/散文包裹的合法数组**静默返回 0 条种子、连 warning 都不发**（与 A108 完全同形）。

两条都改走仓库自带的 robust 解析 `models.base.extract_json_array`（直接数组 / 代码围栏 /
前置散文的括号切片），并且**不新增回归**：主题生成器对「解析不出」的两类响应维持改前口径 ——
以 `[` 开头却坏了 ⇒ 返回空（改前 `json.loads` 抛异常被外层 except 收成空），非数组形状 ⇒
照旧按行剥序号。本文件既测新形状（围栏/散文），也**锁死既有契约**（纯文本行列表、坏数组、
非 JSON ⇒ 空、后端异常 ⇒ 空）不许被 robust 解析反过来破坏。
"""

import pytest

from augmentor.expander import DomainExpander


class ScriptedBackend:
    """只实现 `generate` 的假后端：返回预设响应、记下发出的 prompt"""

    def __init__(self, responses):
        self.responses = list(responses)
        self.prompts = []

    def generate(self, prompt):
        self.prompts.append(prompt)
        if not self.responses:
            raise RuntimeError("脚本响应已耗尽")
        response = self.responses.pop(0)
        if isinstance(response, Exception):
            raise response
        return response


@pytest.fixture
def expander_factory():
    def _make(*responses):
        return DomainExpander(ScriptedBackend(list(responses)))
    return _make


class TestTopicListRobustParsing:
    """A110（主题面）：围栏 / 前置散文的 JSON 数组不该再被行切成垃圾主题"""

    def test_fenced_array_no_longer_becomes_junk_topics(self, expander_factory):
        expander = expander_factory('```json\n["主题A", "主题B"]\n```')
        topics = expander._generate_similar_topics(["租金相关"], num_topics=5)
        assert topics == ["主题A", "主题B"], (
            "围栏里的合法数组被行分割成了 ```json / [\"主题A\"...] 等碎片")

    def test_prose_before_array_no_longer_becomes_junk_topics(self, expander_factory):
        expander = expander_factory('好的，结果如下：\n["主题A"]')
        topics = expander._generate_related_topics(["租金"], num_topics=5)
        assert topics == ["主题A"]

    def test_plain_array_still_parses_and_caps(self, expander_factory):
        expander = expander_factory('["A", "B", "C"]')
        assert expander._generate_scenario_topics(["租金"], num_topics=2) == ["A", "B"]

    def test_plain_text_lines_still_split_and_strip(self, expander_factory):
        """既有契约：非数组形状仍按行剥序号，robust 解析不许把它抢走"""
        expander = expander_factory("1. 主题A\n2. 主题B\n3. 主题C")
        assert expander._generate_similar_topics(["租金"], num_topics=2) == ["主题A", "主题B"]

    def test_broken_array_starting_with_bracket_still_returns_empty(self, expander_factory):
        """既有契约 + 非回归：以 `[` 开头却解析不出 = 内容坏了 ⇒ 返回空，
        而不是掉进行分割把碎片当主题（改前也是空：`json.loads` 抛 → 外层 except）。"""
        expander = expander_factory('["主题A", ')
        assert expander._generate_related_topics(["租金"], num_topics=5) == []

    def test_backend_error_still_returns_empty(self, expander_factory):
        expander = expander_factory(RuntimeError("模型不可用"))
        assert expander._generate_scenario_topics(["租金"]) == []

    @pytest.mark.parametrize("method", [
        "_generate_similar_topics",
        "_generate_related_topics",
        "_generate_scenario_topics",
    ])
    def test_the_fix_is_shared_by_all_three_generators(self, expander_factory, method):
        """三处同构一次修好：同一份围栏响应在三个生成器上都该出同样的主题"""
        expander = expander_factory('```json\n["X", "Y"]\n```')
        topics = getattr(expander, method)(["租金相关"], num_topics=5)
        assert topics == ["X", "Y"], f"{method} 没吃到统一的 robust 解析"


class TestSeedGenerationRobustParsing:
    """A110（种子面）：围栏 / 前置散文的合法数组不该再被静默吞成 0 条种子"""

    def test_fenced_seeds_no_longer_yield_zero(self, expander_factory):
        expander = expander_factory('```json\n[{"instruction": "q1", "output": "a1"}]\n```')
        seeds = expander._generate_seeds_for_topic({"topic": "租金相关"}, 3)
        assert seeds == [{"instruction": "q1", "output": "a1"}], (
            "围栏种子被 startswith('[') 判据静默吞成了 0 条")

    def test_prose_before_seed_array_no_longer_yields_zero(self, expander_factory):
        expander = expander_factory('这是生成结果：\n[{"instruction": "q1"}]')
        seeds = expander._generate_seeds_for_topic({"topic": "t"}, 3)
        assert seeds == [{"instruction": "q1"}]

    def test_direct_seed_array_still_works(self, expander_factory):
        expander = expander_factory('[{"instruction": "q1"}, {"instruction": "q2"}]')
        assert len(expander._generate_seeds_for_topic({"topic": "t"}, 1)) == 1

    def test_non_array_still_returns_empty(self, expander_factory):
        """既有契约：真不是数组的响应仍返回空（robust 解析反过来不许无中生有）"""
        expander = expander_factory("这不是 JSON")
        assert expander._generate_seeds_for_topic({"topic": "t"}, 3) == []
