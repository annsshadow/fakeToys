# -*- coding: utf-8 -*-
"""L96 守卫（A173 关闭）：schema 规格的 `type` 必须归一，归一不了在**构造期**出声

立尺依据（同进程改前 vs 改后复点，Temp/l96q/probe_a173.py，两解释器输出逐字节相同）：
- 改前 `{"type": "str"}` 抛 `TypeError: isinstance() arg 2 must be a type...`，整份校验拿不到判决；
- 改前 `{"type": ("int", "str")}` 遇脏值抛 `AttributeError: 'tuple' object has no attribute '__name__'`；
- 改前 `type_label` 借 `__name__`/`str()` 渲染联合时两解释器分歧：3.14 给 `int | str` 挂了
  `__name__ == "Union"`，3.13 没有，`str(typing.Union[int, str])` 更是两侧异形。

本文件只管**类型规格这一族**（A173），长度/数值/枚举/未知字段那族已在 `test_schema.py`。
"""
import sys
import typing
from pathlib import Path

import pytest

ROOT = Path(__file__).resolve().parents[2]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from augmentor.exceptions import DataValidationError  # noqa: E402
from augmentor.schema import (  # noqa: E402
    TYPE_NAMES,
    DatasetSchema,
    FieldRule,
    resolve_type,
    type_label,
    validate_schema,
)

#: 六档别名 -> (合规值, 不合规值)；不合规值刻意避开 bool，免踩 A175 那一格不对称
ALIAS_CASES = [
    ("str", "abc", 1),
    ("int", 1, "1"),
    ("float", 1.5, "x"),
    ("bool", True, 1),
    ("list", [], {}),
    ("dict", {}, []),
]


class TestTypeNameAliasResolves:
    """规格把类型写成名字时必须等价于写成 Python 类型"""

    def test_alias_table_is_the_six_builtin_names(self):
        # 白名单口径（A173 立案时留的那道「够不够」的问）：本轮定六格，多一格都得先过这条断言
        assert set(TYPE_NAMES) == {"str", "int", "float", "bool", "list", "dict"}
        assert all(isinstance(v, type) for v in TYPE_NAMES.values())

    @pytest.mark.parametrize("alias,ok,bad", ALIAS_CASES)
    def test_alias_resolves_to_the_type_itself(self, alias, ok, bad):
        assert resolve_type(alias) is TYPE_NAMES[alias]

    @pytest.mark.parametrize("alias,ok,bad", ALIAS_CASES)
    def test_alias_spec_verdict_equals_python_type_spec(self, alias, ok, bad):
        """同名两写法必须给同一判决 —— 这一条防「名字解析成了另一个类型」"""
        for value in (ok, bad):
            items = [{"n": value}]
            by_name = validate_schema(items, {"n": {"type": alias}}).to_dict()
            by_type = validate_schema(items, {"n": {"type": TYPE_NAMES[alias]}}).to_dict()
            assert by_name == by_type, (alias, value, by_name, by_type)
        assert validate_schema([{"n": ok}], {"n": {"type": alias}}).is_valid
        # 反空转（A171 口径）：断言「不合规值确实出了判决」而不是只断言两写法相同
        assert not validate_schema([{"n": bad}], {"n": {"type": alias}}).is_valid


class TestBadSpecRaisesAtConstruction:
    """归一不了的规格在 from_spec 当场出声，绝不让一份脏规格走到数据阶段"""

    def test_unknown_type_name_raises_domain_exception_naming_the_field(self):
        with pytest.raises(DataValidationError) as exc:
            DatasetSchema.from_spec({"instruction": {"type": "string"}})
        msg = str(exc.value)
        assert "instruction" in msg          # 文案带字段名，否则多字段规格没法定位
        assert "'string'" in msg             # 带原值
        assert "str" in msg and "int" in msg  # 带可用名字清单

    @pytest.mark.parametrize("garbage", [42, 3.5, object(), {"a": 1}, [1, 2]])
    def test_non_type_object_raises_before_any_item_is_seen(self, garbage):
        """关键形状：**没喂数据就该红** —— 崩点从数据阶段前移到规格阶段"""
        with pytest.raises(DataValidationError):
            DatasetSchema.from_spec({"n": {"type": garbage}})

    def test_it_is_not_a_typeerror_leak(self):
        """改前的症状是 TypeError；这里钉住「不再以 TypeError 出口」"""
        with pytest.raises(DataValidationError) as exc:
            resolve_type("strx", "n")
        assert not isinstance(exc.value, TypeError)
        assert isinstance(exc.value, ValueError)  # DataValidationError 的既有契约

    def test_parameterized_generic_is_rejected_not_crashed(self):
        """`list[str]` 喂给 isinstance 必抛 TypeError，故规格期就拒（本轮实测口径）"""
        with pytest.raises(DataValidationError) as exc:
            DatasetSchema.from_spec({"n": {"type": list[str]}})
        assert "type" in str(exc.value)

    def test_list_containing_none_is_rejected(self):
        with pytest.raises(DataValidationError) as exc:
            resolve_type([int, None], "n")
        assert "None" in str(exc.value)


class TestTupleAndUnionRulesGetVerdict:
    """元组 / 联合规则遇到脏值必须出**判决**（改前这里 AttributeError 崩掉）"""

    def test_tuple_rule_mismatch_message(self):
        errors = DatasetSchema.from_spec({"n": {"type": ("int", "str")}}).validate_item({"n": 1.5})
        assert errors == ["字段 n 类型应为 int/str，实际 float"]

    def test_union_rule_mismatch_message_is_interpreter_stable(self):
        """同进程两写法必须同文案：3.14 与 3.13 对 `int | str` 的 `__name__`/`str()` 异形"""
        pipe = DatasetSchema.from_spec({"n": {"type": int | str}}).validate_item({"n": []})
        typing_union = DatasetSchema.from_spec(
            {"n": {"type": typing.Union[int, str]}}).validate_item({"n": []})
        assert pipe == typing_union == ["字段 n 类型应为 int/str，实际 list"]

    def test_tuple_passes_for_any_member(self):
        schema = DatasetSchema.from_spec({"n": {"type": ["int", "str"]}})
        assert schema.validate_item({"n": 1}) == []
        assert schema.validate_item({"n": "x"}) == []

    def test_name_list_and_mixed_list_equal_tuple_form(self):
        forms = [
            {"n": {"type": ["int", "str"]}},
            {"n": {"type": [int, "str"]}},
            {"n": {"type": ("int", "str")}},
        ]
        verdicts = [validate_schema([{"n": 1.5}], spec).to_dict() for spec in forms]
        assert verdicts[0] == verdicts[1] == verdicts[2]
        assert verdicts[0]["error_count"] == 1  # 反空转：三路都得真判红一次

    def test_type_error_and_other_judgements_coexist(self):
        errors = DatasetSchema.from_spec(
            {"n": {"type": ["int", "str"], "enum": ["a"]}}).validate_item({"n": 1.5})
        assert errors == ["字段 n 类型应为 int/str，实际 float", "字段 n 取值 1.5 不在枚举 ['a']"]

    def test_none_type_rule_still_means_no_type_judgement(self):
        assert resolve_type(None) is None
        assert FieldRule().type is None
        assert DatasetSchema.from_spec({"n": {"type": None}}).validate_item({"n": object()}) == []


class TestLabelRendering:
    """文案层的形状：元组与联合一律 `int/str`，单类型一律本名

    id 必须显式给：pytest 默认按参数对象造 id，而 3.14 的 `int | str` 与
    `typing.Union[int, str]` 渲染成同一个 `Union-int/str`、3.13 不会 —— 于是同一份
    用例在两侧解释器上长出不同节点名，跨解释器 roster 对账当场分叉（A173 那一族
    换了个层出现，本轮实测坐实）。
    """

    @pytest.mark.parametrize("value,label", [
        (int, "int"),
        ((int, str), "int/str"),
        ((int, (str, float)), "int/str/float"),
        (int | str, "int/str"),
        (typing.Union[int, str], "int/str"),
        (type(None), "NoneType"),
        # 最后兜底：既不是元组、没有 args、也不是类时按 str() 渲染。产品路径走不到这里
        # （`resolve_type` 早一步把这种规格拒了），但 `type_label` 是模块级函数，
        # 这一档要么覆盖要么删，不许留成缺数行（本轮实测：留下就是那一格 +1 缺数）。
        (42, "42"),
    ], ids=["int", "tuple2", "tuple-nested", "union-pep604", "union-typing",
            "none", "fallback"])
    def test_label(self, value, label):
        assert type_label(value) == label


class TestNormalizationIsIdempotent:
    """from_spec 归一一次、FieldRule.__post_init__ 再归一一次，第二遍不许改坏第一遍"""

    def test_resolve_twice_is_the_same_object(self):
        union = int | str
        assert resolve_type(resolve_type("int")) is int
        assert resolve_type(resolve_type(("int", "str"))) == (int, str)
        assert resolve_type(resolve_type(union)) is union

    def test_field_rule_direct_construction_normalizes(self):
        """不走 from_spec 的公开入口（FieldRule 是包级导出）同样归一"""
        assert FieldRule(type="float").type is float
        assert FieldRule(type=["int", "str"]).type == (int, str)
        with pytest.raises(DataValidationError):
            FieldRule(type="nope")


class TestBoolAsymmetryPinnedAsCurrent:
    """A175：bool 与 int 家族的**语义**不对称，本轮如实钉住现状，不静默改语义

    `{"type": int}` 拒收 True（专门的 bool 分支），而 `{"type": ("int","str")}` 放过
    True（元组走通用 isinstance，没有 bool 分支）。同一条数据两种写法判决相反。
    文案那半（bool 分支原先缺「实际 X」）本轮已随 A173 一起收掉，见下第一条。
    """

    def test_bool_rejected_by_single_int_rule(self):
        errors = DatasetSchema.from_spec({"n": {"type": int}}).validate_item({"n": True})
        assert errors == ["字段 n 类型应为 int，实际 bool"]

    def test_bool_accepted_by_int_str_tuple_rule(self):
        assert DatasetSchema.from_spec({"n": {"type": ("int", "str")}}).validate_item({"n": True}) == []

    def test_bool_alias_rule_is_strict(self):
        """`"bool"` 名字规则反而是三格里最严的：True 过、1 不过"""
        assert DatasetSchema.from_spec({"n": {"type": "bool"}}).validate_item({"n": True}) == []
        assert DatasetSchema.from_spec({"n": {"type": "bool"}}).validate_item({"n": 1}) != []
