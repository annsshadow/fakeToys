# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L220（A274 / A138 ② 档）：`config.evaluation.generated_field` 与 `.reference_field`
接进 `/api/dataset/evaluate` 的省略支路

**接的是哪一格**：属性面普查（`tests/unit/test_config_reader_census_l217.py`）里
`evaluation.reference_field` 是零读者键，理由写得很直白——「参考侧字段由同一条请求模型的
`reference_field` 提供，默认写死在请求侧」。L219 收同节 `metrics` 时把它留在原地，因为
**接不动**：请求默认是硬值 `"output"`、出厂配置也是 `"output"`，两者同值时
`request.reference_field or config.reference_field` 的 `or` 永远落在请求那一档，配置键
即便写了也别想生效。要接只有一条路：把请求侧默认改成 `None`（哨兵），省略才落到配置。

**为什么补 `evaluation.generated_field`**：只接参考侧会出现「参考侧跟配置、生成侧跟硬编码」
两种默认并存，而这两个字段本来就在同一条请求模型上、语义完全对称（A274 点的就是这个口径）。
补一只同构的键，两侧的权威序就一致了。新增键必须有读者，所以它与接线同轮做，不留「写进
schema 却没人读」的新账。

**为什么是零漂移接线而不是行为变更**：两只键的出厂值都是 `"output"`，与接线前请求侧那句
硬编码默认**逐字同** ⇒ 按出厂配置部署的读数一字不变。判据取的是「改配置 ⇒ 取哪一列跟着变」，
再由 `TestFactoryPremise` 把「出厂值就是 `output`」这一前提本身钉住（前提塌了要让等价臂先红，
而不是让它悄悄变成永真）。

**坏字段名一律出声**：配置指向不存在的列 ⇒ `_as_column` 带下标报 400；配置写空串（绕过加载期
`require_string` 直接改属性）⇒ 同一个门口报 `''`。两侧都不静默补空串——静默补空等于凭空造
0 分样本，指标被稀释却看不见来源。

**请求侧空串的口径**：`"generated_field": ""` 与省略**不可区分**（falsy），按权威序落回配置。
这与 L219 给 `metrics: []` 定下的语义同式，不是各自发明的两件事。
"""

import json
from pathlib import Path
from types import SimpleNamespace

import pytest

AI_DIR = Path(__file__).resolve().parent.parent.parent

#: 四条互不相同的列，让「取哪一列」在 `similarity` 上留下可见差别。
#: 值刻意取得两两不同构，避免任何一列与另一列的均值撞车（`test_*_differs_from_*` 臂
#: 的前提由 `TestColumnValuesAreDistinguishable` 机器核对，不靠这里的注释）。
ITEMS = [
    {"output": "押金一个月", "answer": "押金是一个月的租金",
     "reference": "押金为一个月租金", "gold": "完全无关的另一句话"},
    {"output": "合同期一年", "answer": "合同期限是一年",
     "reference": "合同为期整整一年", "gold": "和上面同样无关的文本"},
]


@pytest.fixture
def env(tmp_path, monkeypatch):
    """真实管道 + 客户端，读写面全部指向临时目录，配置从仓库 `config.yaml` 出厂值起"""
    from augmentor import AugmentorPipeline, load_config
    import api.deps as deps
    from api.main import app
    from fastapi.testclient import TestClient

    config = load_config(str(AI_DIR / "config.yaml"))
    config.versioning.storage_dir = str(tmp_path / "versions")
    config.augmentation.output_dir = str(tmp_path / "out")
    config.augmentation.checkpoint_dir = str(tmp_path / "ckpt")
    monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", str(tmp_path))
    monkeypatch.delenv("AUGMENTOR_API_KEY", raising=False)
    monkeypatch.chdir(tmp_path)
    monkeypatch.setattr(deps, "_pipeline", AugmentorPipeline(config))

    path = tmp_path / "l220_items.json"
    path.write_text(json.dumps(ITEMS, ensure_ascii=False), encoding="utf-8")
    return SimpleNamespace(
        client=TestClient(app), config=config, pipeline=deps._pipeline, file="l220_items.json"
    )


def evaluate(env, **body):
    """打 `/api/dataset/evaluate`：只留指标，两侧字段默认省略（本轮的判据面就是省略支路）"""
    payload = {"generated_file": env.file, "reference_file": env.file,
               "metrics": ["similarity"]}
    payload.update(body)
    return env.client.post("/api/dataset/evaluate", json=payload)


class TestColumnValuesAreDistinguishable:
    """四列两两不同文本 ⇒ 「取哪一列」必须能在读数上留下差别

    这是下面所有等价/差别臂的地基：如果 `output` 与 `answer` 的相似度均值本来就相等，
    那些臂会恒真。所以把前提本身钉住，前提塌了先红这里。
    """

    def test_all_four_columns_are_present_in_the_fixture(self, env):
        for index, item in enumerate(ITEMS):
            assert set(item) == {"output", "answer", "reference", "gold"}, f"第 {index} 条: {sorted(item)}"

    def test_different_column_pairs_give_different_scores(self, env):
        seen = {}
        for generated in ("output", "answer"):
            for reference in ("reference", "gold"):
                response = evaluate(env, generated_field=generated, reference_field=reference)
                assert response.status_code == 200, response.text
                seen[(generated, reference)] = response.json()["metrics"]["similarity"]
        assert len({round(v, 12) for v in seen.values()}) == 4, f"有两对列撞车，等价臂会恒真: {seen}"


class TestConfigFieldNamesAreConsumed:
    """配置那两只键真的决定取哪一列"""

    def test_generated_side_follows_config(self, env):
        """省略 `generated_field` ⇒ 取 `config.evaluation.generated_field`"""
        env.config.evaluation.generated_field = "answer"
        by_config = evaluate(env, reference_field="reference")
        assert by_config.status_code == 200, by_config.text
        explicit = evaluate(env, generated_field="answer", reference_field="reference")
        assert by_config.json() == explicit.json()

    def test_generated_side_no_longer_reads_the_hardcoded_output(self, env):
        """专打接线前的形状：那时省略就是 `output`，配置写什么都不算

        与 `test_generated_side_follows_config` 互为反证：同一份配置下，省略支路的读数
        必须**不等于**按 `output` 算出来的那份。
        """
        env.config.evaluation.generated_field = "answer"
        by_config = evaluate(env, reference_field="reference")
        as_output = evaluate(env, generated_field="output", reference_field="reference")
        assert by_config.json()["metrics"] != as_output.json()["metrics"]

    def test_reference_side_follows_config(self, env):
        env.config.evaluation.reference_field = "gold"
        by_config = evaluate(env, generated_field="output")
        assert by_config.status_code == 200, by_config.text
        explicit = evaluate(env, generated_field="output", reference_field="gold")
        assert by_config.json() == explicit.json()
        as_reference = evaluate(env, generated_field="output", reference_field="reference")
        assert by_config.json()["metrics"] != as_reference.json()["metrics"]

    def test_request_beats_config_on_both_sides(self, env):
        """权威序第一档：请求显式给值时配置那两键不许掺进来"""
        env.config.evaluation.generated_field = "answer"
        env.config.evaluation.reference_field = "gold"
        by_request = evaluate(env, generated_field="output", reference_field="reference")
        assert by_request.status_code == 200, by_request.text
        # 省略支路此刻按配置那两列算，读数必须与请求显式那份**不同**
        assert by_request.json()["metrics"] != evaluate(env).json()["metrics"]

    def test_empty_request_string_falls_back_to_config(self, env):
        """`""` 与省略不可区分 ⇒ 落回配置（与 L219 给 `metrics: []` 的语义同式）"""
        env.config.evaluation.generated_field = "answer"
        blank = evaluate(env, generated_field="", reference_field="reference")
        assert blank.status_code == 200, blank.text
        assert blank.json() == evaluate(env, reference_field="reference").json()

    def test_factory_defaults_are_zero_drift(self, env):
        """零漂移钉：出厂配置 + 两侧省略 ≡ 两侧显式传 `output`

        整份响应逐值比（含 `details`），接线若偷偷换了列，这一臂当场红。
        """
        factory_generated = env.config.evaluation.generated_field
        factory_reference = env.config.evaluation.reference_field
        assert (factory_generated, factory_reference) == ("output", "output"), \
            f"出厂值变了，本臂的等价前提失效: {factory_generated} / {factory_reference}"

        by_config = evaluate(env)
        by_request = evaluate(env, generated_field="output", reference_field="output")
        assert by_config.status_code == 200, by_config.text
        assert by_config.json() == by_request.json()


class TestBadConfigFieldNamesAreLoud:
    """配置指向取不到的列时出声，而不是静默补空串"""

    def test_missing_generated_column_reports_the_field_and_index(self, env):
        env.config.evaluation.generated_field = "nope"
        response = evaluate(env, reference_field="reference")
        assert response.status_code == 400, response.text
        detail = response.json()["detail"]
        assert "生成侧" in detail and "nope" in detail and "第 0 条" in detail, detail

    def test_missing_reference_column_reports_the_field_and_index(self, env):
        env.config.evaluation.reference_field = "nope"
        response = evaluate(env, generated_field="output")
        assert response.status_code == 400, response.text
        detail = response.json()["detail"]
        assert "参考侧" in detail and "nope" in detail, detail

    def test_blank_config_column_is_refused_not_padded(self, env):
        """直接改属性绕过加载期 `require_string` 时，门口仍要拒：空串不是「取第一列」"""
        env.config.evaluation.reference_field = ""
        env.config.evaluation.generated_field = "output"
        response = evaluate(env)
        assert response.status_code == 400, response.text
        assert "''" in response.json()["detail"], response.json()["detail"]

    def test_loading_a_blank_config_field_fails_at_the_gate(self):
        """加载期那道闸才是正牌去处：`config.evaluation.reference_field: ''` 不许能加载"""
        from augmentor.config import EvaluationConfig
        from augmentor.validation import DataValidationError

        with pytest.raises(DataValidationError):
            EvaluationConfig(generated_field="output", reference_field="")


class TestFactoryPremise:
    """把等价臂的前提钉成机器断言：出厂两只键就是 `output`"""

    def test_dataclass_default_is_output_for_both(self):
        from augmentor.config import EvaluationConfig

        config = EvaluationConfig()
        assert config.generated_field == "output", config.generated_field
        assert config.reference_field == "output", config.reference_field

    def test_repo_yaml_sets_both_to_output(self):
        import yaml

        loaded = yaml.safe_load((AI_DIR / "config.yaml").read_text(encoding="utf-8"))
        section = loaded["evaluation"]
        assert section["generated_field"] == "output", section
        assert section["reference_field"] == "output", section

    def test_new_key_is_validated_like_its_sibling(self):
        """新键与同节那只同规：类型是字符串且非空，校验器与运行时两侧都有判决"""
        from augmentor.config_validator import ConfigValidator

        known = ConfigValidator.KNOWN_FIELDS
        assert known["evaluation.generated_field"] == {"type": str, "non_empty": True}
        assert known["evaluation.reference_field"] == {"type": str, "non_empty": True}


class TestRequestFieldsAreNowOmissible:
    """契约面：两个字段的「省略」与「传 output」从此是两种不同的事

    A274 预判这一步会动 OpenAPI——字段可空性是定义上的变化，不是推测。这里不靠人眼比对，
    直接从 `app.openapi()` 实推。
    """

    def test_both_fields_are_nullable_with_null_default(self, env):
        from api.main import app

        schema = app.openapi()["components"]["schemas"]["EvaluateRequest"]
        for name in ("generated_field", "reference_field"):
            prop = schema["properties"][name]
            assert prop.get("default") is None, f"{name}: {prop}"
            assert "null" in json.dumps(prop.get("anyOf", prop.get("type"))), f"{name}: {prop}"

    def test_only_the_two_files_remain_required(self, env):
        from api.main import app

        schema = app.openapi()["components"]["schemas"]["EvaluateRequest"]
        assert set(schema["required"]) == {"generated_file", "reference_file"}, schema["required"]
