"""schema 数据集结构校验模块测试

覆盖必需键、类型（含 bool 非数值）、枚举、长度/数值范围、
未知字段、空值、结果序列化与包级导出。
"""

import pytest

from augmentor import DatasetSchema, FieldRule, SchemaValidationResult, validate_schema


SPEC = {
    "instruction": {"type": str, "min_length": 4},
    "output": {"type": str},
    "difficulty": {"type": int, "min_value": 1, "max_value": 5},
    "lang": {"type": str, "enum": ["zh", "en"], "required": False},
}


def _valid_item():
    return {"instruction": "如何申请入住？", "output": "登录官网", "difficulty": 3, "lang": "zh"}


class TestValidateItem:
    def test_valid_item_no_errors(self):
        schema = DatasetSchema.from_spec(SPEC)
        assert schema.validate_item(_valid_item()) == []

    def test_missing_required_field(self):
        schema = DatasetSchema.from_spec(SPEC)
        item = _valid_item()
        del item["output"]
        errors = schema.validate_item(item)
        assert any("output" in e for e in errors)

    def test_type_mismatch(self):
        schema = DatasetSchema.from_spec(SPEC)
        item = _valid_item()
        item["difficulty"] = "hard"
        assert any("difficulty" in e and "int" in e for e in schema.validate_item(item))

    def test_bool_not_accepted_as_number(self):
        schema = DatasetSchema.from_spec(SPEC)
        item = _valid_item()
        item["difficulty"] = True
        assert any("difficulty" in e for e in schema.validate_item(item))

    def test_enum_violation(self):
        schema = DatasetSchema.from_spec(SPEC)
        item = _valid_item()
        item["lang"] = "fr"
        assert any("lang" in e and "枚举" in e for e in schema.validate_item(item))

    def test_optional_enum_absent_ok(self):
        schema = DatasetSchema.from_spec(SPEC)
        item = _valid_item()
        del item["lang"]
        assert schema.validate_item(item) == []

    def test_length_violation(self):
        schema = DatasetSchema.from_spec(SPEC)
        item = _valid_item()
        item["instruction"] = "短"
        assert any("instruction" in e and "长度" in e for e in schema.validate_item(item))

    def test_value_range_violation(self):
        schema = DatasetSchema.from_spec(SPEC)
        item = _valid_item()
        item["difficulty"] = 9
        assert any("difficulty" in e and "大于" in e for e in schema.validate_item(item))

    def test_non_dict_item(self):
        schema = DatasetSchema.from_spec(SPEC)
        assert schema.validate_item("not a dict") == ["条目必须是 dict"]


class TestUnknownFields:
    def test_unknown_field_rejected_when_disallowed(self):
        schema = DatasetSchema.from_spec(SPEC, extra_fields_allowed=False)
        item = _valid_item()
        item["extra"] = 1
        assert any("extra" in e for e in schema.validate_item(item))

    def test_unknown_field_allowed_by_default(self):
        schema = DatasetSchema.from_spec(SPEC)
        item = _valid_item()
        item["extra"] = 1
        assert schema.validate_item(item) == []


class TestValidateDataset:
    def test_valid_dataset(self):
        items = [_valid_item(), _valid_item()]
        result = validate_schema(items, SPEC)
        assert isinstance(result, SchemaValidationResult)
        assert result.is_valid is True
        assert result.error_count == 0

    def test_invalid_dataset_reports_indices(self):
        good = _valid_item()
        bad = _valid_item()
        bad["difficulty"] = "x"
        result = validate_schema([good, bad], SPEC)
        assert result.is_valid is False
        assert result.invalid_items[0]["index"] == 1

    def test_result_to_dict(self):
        result = validate_schema([_valid_item()], SPEC)
        d = result.to_dict()
        assert d["is_valid"] is True
        assert d["total_items"] == 1
        assert "errors" in d

    def test_empty_dataset_valid(self):
        result = validate_schema([], SPEC)
        assert result.is_valid is True
        assert result.total_items == 0


class TestSchemaBoundaries:
    def test_null_required_field_error(self):
        schema = DatasetSchema.from_spec({"a": {"type": int, "required": True}})
        assert any("为空" in e for e in schema.validate_item({"a": None}))

    def test_null_optional_field_ok(self):
        schema = DatasetSchema.from_spec({"a": {"type": int, "required": False}})
        assert schema.validate_item({"a": None}) == []

    def test_max_length_violation(self):
        schema = DatasetSchema.from_spec({"s": {"type": str, "max_length": 2}})
        assert any("长度" in e and "大于" in e for e in schema.validate_item({"s": "abc"}))

    def test_min_value_violation(self):
        schema = DatasetSchema.from_spec({"n": {"type": int, "min_value": 0}})
        assert any("小于" in e for e in schema.validate_item({"n": -5}))

    def test_list_min_max_length(self):
        schema = DatasetSchema.from_spec(
            {"tags": {"type": list, "min_length": 1, "max_length": 3}}
        )
        assert schema.validate_item({"tags": ["a", "b"]}) == []
        assert schema.validate_item({"tags": []})
        assert schema.validate_item({"tags": [1, 2, 3, 4]})


class TestFieldRuleDirect:
    def test_rule_defaults(self):
        rule = FieldRule()
        assert rule.required is True
        assert rule.type is None

    def test_package_exports(self):
        import augmentor

        assert augmentor.DatasetSchema is DatasetSchema
        assert augmentor.FieldRule is FieldRule
        assert augmentor.SchemaValidationResult is SchemaValidationResult
        assert callable(augmentor.validate_schema)
