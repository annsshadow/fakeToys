# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L238（A319 / A320 / A321 / A322）：判决缺失面四条 —— 共引身份、判序、容差、钳制

**本文件量的是「一条权威只住一处」这一维**（A77）。l235 / l236 已经把行为面钉住了
（四档 400、0.99 放行、钳进 [0, 1]、模型 0 次调用、不落盘），那些臂换个写法仍然红。但行为面
有一个共同的盲区：**它测不出「判据抄了两份」**。改前 A319 之所以能把 `[2.0, -0.5, -0.25,
-0.25]` 放进生产，正是因为门口那两行手抄与评分器那两行手抄**逐字节相同、同时漏判**——
两份一致的错在行为测试里看起来完全正常。所以这里量的是身份、判序与「没有第二份」。

五面判据，各对应一种真实失效形状：

1. **共引身份**（`TestOneAuthorityPerRuler`）：`health_score` / `api.routes.quality` 绑定的
   `require_ratio_list` 与 `validation.require_ratio_list` 是**同一个对象**；`api.routes.
   dataset_tools.require_item_field` 同理。`quality_gate` / `language` 是函数体内 import
   （避开循环依赖），所以按「迟到绑定」验：把 `validation.require_item_field` 换成哨身，
   `gate_dataset_health` 与 `translate_dataset` 都必须撞上哨身并报告自己那一格的参数名
   （`text_field` / `text_key`）。抄第二份判据的写法在这一面必然红。
2. **判序**（`TestTheOrderOfVerdicts`）：参数错误在**读盘之前**，字段存在性在**读盘之后**。
   实测「坏权重 + 文件不存在」回 400 而不是 404，「幽灵字段 + 文件不存在」回 404，
   「幽灵字段 + 空数据集」回 400 且说的是空数据集（空判在先），「权重个数错 + 空串字段」
   先报权重。这四档合起来说明门口没有「先读盘再判参数」的实现空间。
3. **容差含端点且量化**（`TestTheToleranceIsRuled`）：`[0.24, 0.25, 0.25, 0.25]` 与
   `[0.2525] * 4` 的十进制差都是 0.01，但 IEEE-754 求和给出 `0.010000000000000009`——
   改前直接 `> 0.01` 判红，即「±0.01 含端点」这句承诺是假话。现在量化到 9 位 ⇒ 0.99 /
   1.005 / 1.01 三档 200，0.98 / 1.02 两档 400，且 400 的 detail 引用
   `WEIGHT_SUM_TOLERANCE` 常数而不是文案里的字面。
4. **两面同文**（`TestTheTwoFacesSayTheSameSentence`）：同一档非法输入，SDK 直构的
   `DataValidationError` 消息与 HTTP 的 `detail` **逐字符相等**，CLI 那条路
   （`run_health_gate`）抛同一句。改前三面三判：字符串权重在直构面是 `TypeError`、
   在 API 面是 422、在门口静默放行。
5. **钳制不是装饰**（`TestTheClampBites`）：一份四项全 1.0 的夹具，默认权重下总分恰好
   1.0；把权重和抬到 1.01（仍容差内）时 `raw = 1.01 > 1.0` 而 `health_score` 仍是 1.0。
   这一支是「钳制那行删掉测试还全绿」的直接反证。

改前 / 改后读数来自两份仓外探针：`Temp/l238_probe2.py`（行为面，L238 写码前跑）与
`Temp/l238_probe3.py`（本文件的身份、判序、钳制三面，逐条对照本文件断言）。

顺带量到一条**新的、本文件不收口**的缺陷（立账 A324）：分割比例那一族仍是手抄容差——
`train_ratio=0.4, val_ratio=0.3, test_ratio=0.29` 这一份完全合法的 40/30/29 切分，
实测和 0.99、浮点差 `0.010000000000000009`，被 `dataset_ops.split` 判成
「分割比例之和必须为1.0，当前为 0.99」；同一处 `ratios` 还不判长度，SDK 直构喂 4 个元素
时通过校验后死在 `ValueError: too many values to unpack (expected 3)`。API 与 CLI 那两面
由三个标量拼 tuple，所以长度那一半只在直构面可达，容差那一半三面都可达。

跑法：`python -m pytest tests/unit/test_decision_gates_l238.py -q --no-cov`
"""

import json
import pathlib
import re
import types

import pytest

import augmentor.health_score as health_score
import augmentor.language as language
import augmentor.quality_gate as quality_gate
import augmentor.validation as validation
from augmentor.exceptions import DataValidationError
from augmentor.health_score import DEFAULT_HEALTH_WEIGHTS, DatasetHealthScore
from augmentor.validation import (
    WEIGHT_SUM_QUANTIZE_DIGITS,
    WEIGHT_SUM_TOLERANCE,
    require_item_field,
    require_ratio_list,
)

ROOT = pathlib.Path(__file__).resolve().parents[2]

#: 4 条、前两条完全重复：默认权重下四项 = 1.0 / 0.75 / 0.8208 / 1.0（l235 已钉）
DUP_ITEMS = [
    {"instruction": "请解释什么是向量数据库，并给出一个使用场景。",
     "output": "向量数据库按相似度检索高维向量。"},
    {"instruction": "请解释什么是向量数据库，并给出一个使用场景。",
     "output": "向量数据库按相似度检索高维向量。"},
    {"instruction": "写一首关于秋天的诗，要求四行以上。",
     "output": "落叶铺满小径，秋天把颜色交给风。"},
    {"instruction": "什么是包含判断，它在检索里有什么用？", "output": "包含判断是子串匹配。"},
]

#: 四项分量**全为 1.0** 的夹具：output 等长（`max == min` 走 quality_balance 的 1.0 支路）、
#: instruction 互不相同（diversity 1.0）、词全不重复（coverage 封顶 1.0）、两列都在
#: （completeness 1.0）。这份夹具的存在就是为了让「权重和 1.01」真的把 raw 顶过 1.0。
PERFECT_ITEMS = [
    {"instruction": "alpha bravo", "output": "1234"},
    {"instruction": "charlie delta", "output": "5678"},
    {"instruction": "echo foxtrot", "output": "90ab"},
]


@pytest.fixture
def http(tmp_path, monkeypatch):
    """白名单数据根指向临时目录：health-gate 只读，但仍不许把夹具落进仓库"""
    root = tmp_path / "ds"
    root.mkdir()
    (root / "dup.json").write_text(json.dumps(DUP_ITEMS, ensure_ascii=False), encoding="utf-8")
    (root / "perfect.json").write_text(
        json.dumps(PERFECT_ITEMS, ensure_ascii=False), encoding="utf-8")
    (root / "empty.json").write_text("[]", encoding="utf-8")
    monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", str(root))
    monkeypatch.setenv("AUGMENTOR_API_KEY", "")
    monkeypatch.chdir(tmp_path)
    from fastapi.testclient import TestClient

    from api.main import app

    return TestClient(app), root


def gate(client, file="dup.json", **body):
    return client.post("/api/quality/health-gate", json={"input_file": file, **body})


def _src(module_name: str) -> str:
    return (ROOT / module_name).read_text(encoding="utf-8")


class TestOneAuthorityPerRuler:
    """第 1 面：判据只有一份，抄第二份在这一面必红"""

    def test_the_weight_ruler_is_the_same_object_in_all_three_bindings(self):
        assert health_score.require_ratio_list is require_ratio_list
        import api.routes.quality as api_quality

        assert api_quality.require_ratio_list is require_ratio_list

    def test_the_field_ruler_is_the_same_object_on_the_http_face(self):
        import api.routes.dataset_tools as api_tools

        assert api_tools.require_item_field is require_item_field

    def test_no_hand_copied_sum_rule_survives_in_the_two_callers(self):
        """门口与评分器都不许再写 `if abs(...)`：改前这里有两行逐字节相同的手抄

        判的是**代码行**（`if` 紧跟 `abs(`），不是散文——两份 docstring 都还在叙述「改前
        是 `abs(sum - 1.0) > 0.01`」，那是历史记录不是实现，把它们算进来会把尺子量歪。
        """
        for module_name in ("augmentor/health_score.py", "api/routes/quality.py"):
            src = _src(module_name)
            assert not re.search(r"^\s*if\s+abs\(", src, re.M), module_name
            assert "require_ratio_list(" in src, module_name

    def test_the_http_wear_for_fields_is_a_coat_not_a_second_rule(self):
        """`_require_any_field` 只做「异常 → 400」的翻译，不重复判存在性"""
        src = _src("api/routes/dataset_tools.py")
        body = src[src.index("def _require_any_field"):src.index("def _as_column")]
        assert "require_item_field(" in body
        assert "in item for item in" not in body, "存在性判据被抄回 HTTP 层了"

    def test_the_two_sdk_callers_resolve_the_field_ruler_through_validation(self, monkeypatch):
        """迟到绑定验身份：换掉 `validation.require_item_field`，两个 SDK 入口都得撞上哨身

        `quality_gate` / `language` 在函数体内 import（顶层 import 会成环），所以它们
        **每次调用都从 validation 模块取一次名字**。这正是本臂能成立的原因，也是
        「抄第二份」在这两面无法隐藏的原因。
        """
        seen = []

        def sentinel(name, items, field, label="数据集"):
            seen.append((name, field, label))
            raise DataValidationError(f"SENTINEL::{name}")

        monkeypatch.setattr(validation, "require_item_field", sentinel)
        with pytest.raises(DataValidationError, match="SENTINEL::text_field"):
            quality_gate.gate_dataset_health(DUP_ITEMS, text_field="instruction")
        with pytest.raises(DataValidationError, match="SENTINEL::text_key"):
            language.translate_dataset(
                DUP_ITEMS, backend=object(), supported_langs=["en", "zh"],
                default_target_lang="en", batch_size=2, text_key="instruction")
        # 参数名按各自那一格报，label 走默认：拼错的人需要知道是哪一格写错了
        assert seen == [("text_field", "instruction", "数据集"),
                        ("text_key", "instruction", "数据集")]

    def test_the_sentinel_arm_is_not_vacuous_the_real_gate_sits_on_the_happy_path(self,
                                                                                  monkeypatch):
        """反恒真：合法字段也必须走过那道闸（否则上面那支的哨身撞的是别处的红）

        哨身换成「数一次的壳」，把真判据原样放进去：`text_field="instruction"` 这一档
        必须恰好调用 1 次并放行，判定结果照常回来。若哪天有人把 `require_item_field` 的
        调用挪进某个只有出错才走的分支，这一支先红。
        """
        real = validation.require_item_field
        seen = []

        def counting(name, items, field, label="数据集"):
            seen.append(name)
            return real(name, items, field, label)

        monkeypatch.setattr(validation, "require_item_field", counting)
        result = quality_gate.gate_dataset_health(DUP_ITEMS, text_field="instruction")
        assert result["gate"]["verdict"] in ("passed", "warned", "failed")
        assert seen == ["text_field"]


class TestTheOrderOfVerdicts:
    """第 2 面：门口判参数、读盘之后判字段，顺序不许反

    反了会怎样：把 `require_item_field` 挪到读盘之前就得凭空造出一份数据来判；把权重校验
    挪到评分之后，参数错误会被路由的兜底 `except Exception` 报成 500——「你写的权重不合法」
    这种用户一改就好的错，绝不该以 500 出场。
    """

    def test_bad_weights_are_rejected_before_the_file_is_even_read(self, http):
        client, _ = http
        r = gate(client, file="no_such.json", weights=[2.0, -0.5, -0.25, -0.25])
        assert r.status_code == 400, r.text
        assert r.json()["detail"] == "weights[0] 必须在 0.0 到 1.0 之间，当前是 2.0"

    def test_a_ghost_field_cannot_be_judged_without_the_file_so_404_wins(self, http):
        client, _ = http
        r = gate(client, file="no_such.json", text_field="no_such_field")
        assert r.status_code == 404, r.text
        assert r.json()["detail"] == "文件不存在"

    def test_the_empty_dataset_verdict_precedes_the_field_verdict(self, http):
        """空数据集时「没有任何一条含该字段」在字面上也成立，但那不是用户的错处"""
        client, _ = http
        r = gate(client, file="empty.json", text_field="no_such_field")
        assert r.status_code == 400, r.text
        assert r.json()["detail"] == "数据集为空，门禁无法给出有意义的判定"

    def test_the_weight_shape_error_wins_over_the_field_error(self, http):
        client, _ = http
        r = gate(client, weights=[0.5, 0.5], text_field="")
        assert r.status_code == 400, r.text
        assert r.json()["detail"] == "weights 必须是 4 个元素，当前 2 个"


class TestTheToleranceIsRuled:
    """第 3 面：A320——「±0.01（含端点）」这句话现在真的有那把尺子"""

    @pytest.mark.parametrize("weights,total", [
        ([0.24, 0.25, 0.25, 0.25], 0.99),
        ([0.25125, 0.25125, 0.25125, 0.25125], 1.005),
        ([0.2525, 0.2525, 0.2525, 0.2525], 1.01),
    ])
    def test_the_endpoints_are_inclusive(self, http, weights, total):
        client, _ = http
        assert sum(weights) == pytest.approx(total, abs=1e-12)
        assert abs(sum(weights) - 1.0) <= WEIGHT_SUM_TOLERANCE + 1e-12
        r = gate(client, weights=weights)
        assert r.status_code == 200, r.text
        assert r.json()["health"]["weights"] == weights
        assert r.json()["health"]["health_score"] <= 1.0

    @pytest.mark.parametrize("weights,total", [
        ([0.245, 0.245, 0.245, 0.245], 0.98),
        ([0.255, 0.255, 0.255, 0.255], 1.02),
    ])
    def test_beyond_the_tolerance_is_a_400_that_reports_the_measured_sum(self, http,
                                                                         weights, total):
        client, _ = http
        r = gate(client, weights=weights)
        assert r.status_code == 400, r.text
        detail = r.json()["detail"]
        assert f"容差 {WEIGHT_SUM_TOLERANCE}，含端点" in detail
        assert detail.endswith(f"当前 {total}"), detail

    def test_the_fp_noise_is_why_the_quantize_digits_exist(self):
        """0.99 这一档的十进制差是 0.01，但浮点求和是 0.010000000000000009 > 0.01

        改前那句 `abs(sum - 1.0) > 0.01` 对「留两位小数凑 1.0」这一种最自然的用法判红。
        这一支钉的是**裸浮点事实**（不是产品行为）：判和之前不量化就必然假拒，
        `WEIGHT_SUM_QUANTIZE_DIGITS` 一旦调小，`test_the_endpoints_are_inclusive` 就红。
        """
        noisy = abs(sum([0.24, 0.25, 0.25, 0.25]) - 1.0)
        assert noisy > WEIGHT_SUM_TOLERANCE
        assert round(noisy, WEIGHT_SUM_QUANTIZE_DIGITS) == WEIGHT_SUM_TOLERANCE

    def test_the_ruler_returns_the_value_and_the_default_is_not_a_shared_list(self):
        assert require_ratio_list("weights", None, length=4) is None
        assert DatasetHealthScore().weights == DEFAULT_HEALTH_WEIGHTS
        DatasetHealthScore().weights[0] = 0.9
        assert DEFAULT_HEALTH_WEIGHTS == [0.25, 0.25, 0.25, 0.25], "默认权重被调用方改写了"


class TestTheTwoFacesSayTheSameSentence:
    """第 4 面：同一档非法输入，SDK / HTTP / CLI 三面同文（改前三面三判）"""

    @pytest.mark.parametrize("field,expected", [
        ("", "text_field 不能是空字符串"),
        ("   ", "text_field 不能是纯空白字符串，当前是 '   '"),
        ("no_such_field", "数据集 里没有任何一条含字段 'no_such_field'（大概率是字段名"
                          "拼错了）；首条实际字段: instruction, output"),
        ("Instruction", "数据集 里没有任何一条含字段 'Instruction'（大概率是字段名"
                        "拼错了）；首条实际字段: instruction, output"),
    ])
    def test_the_four_field_shapes_are_400_on_both_faces_with_one_sentence(self, http,
                                                                          field, expected):
        client, _ = http
        with pytest.raises(DataValidationError) as sdk:
            quality_gate.gate_dataset_health(DUP_ITEMS, text_field=field)
        assert str(sdk.value) == expected
        r = gate(client, text_field=field)
        assert r.status_code == 400, r.text
        assert r.json()["detail"] == expected

    def test_the_cli_face_raises_the_same_sentence_as_the_sdk(self, tmp_path):
        """CLI 走的是 `gate_dataset_health`，所以判据一挪到路由层这一面就哑了"""
        from augmentor.cli.commands.quality import run_health_gate

        src = tmp_path / "dup.json"
        src.write_text(json.dumps(DUP_ITEMS, ensure_ascii=False), encoding="utf-8")
        args = types.SimpleNamespace(input=str(src), text_field="no_such_field", weights=None,
                                     pass_rate=None, pass_rate_min=0.6, duplicate_rate_max=0.3,
                                     completeness_min=0.8, block_on_warning=False)
        with pytest.raises(DataValidationError) as exc:
            run_health_gate(args, types.SimpleNamespace())
        assert "首条实际字段: instruction, output" in str(exc.value)

    @pytest.mark.parametrize("weights,fragment", [
        ([-0.1, 0.4, 0.35, 0.35], "weights[0] 必须在 0.0 到 1.0 之间，当前是 -0.1"),
        ([1.5, -0.2, -0.2, -0.1], "weights[0] 必须在 0.0 到 1.0 之间，当前是 1.5"),
    ])
    def test_the_per_item_bound_is_the_same_sentence_on_both_faces(self, http,
                                                                   weights, fragment):
        client, _ = http
        with pytest.raises(DataValidationError) as sdk:
            DatasetHealthScore(weights=weights)
        assert str(sdk.value) == fragment
        r = gate(client, weights=weights)
        assert r.status_code == 400, r.text
        assert r.json()["detail"] == fragment

    def test_the_http_face_cannot_show_bool_or_str_rejection_and_that_is_recorded(self, http):
        """诚实记录两面之差：pydantic 的 `Optional[List[float]]` 先把布尔吞成 1.0、把
        `'abc'` 拦成 422，所以「布尔逐项拒」与「非数值点名下标」只在直构面可测。

        这一支不是将就——它是 A319 的第 ⑤ 条读数：`[True, 0.0, 0.0, 1.0]` 到门口已经变成
        `[1.0, 0.0, 0.0, 1.0]`，被**和**那一判拦下（`当前 2.0`），而不是被逐项界拦下。
        """
        client, _ = http
        with pytest.raises(DataValidationError, match="不能是布尔"):
            DatasetHealthScore(weights=[True, 0.0, 0.0, 1.0])
        with pytest.raises(DataValidationError, match="必须是数值"):
            DatasetHealthScore(weights=["a", "b", "c", "d"])
        r = gate(client, weights=[True, 0.0, 0.0, 1.0])
        assert r.status_code == 400, r.text
        assert "当前 2.0" in r.json()["detail"]
        assert gate(client, weights=["a", "b", "c", "d"]).status_code == 422

    def test_impact_keeps_its_own_label_but_shares_the_sentence(self, http):
        """`/api/dataset/impact` 是唯一对外措辞变化点：label 说「基线数据集」，判据同一份"""
        client, _ = http
        r = client.post("/api/dataset/impact", json={"before_file": "dup.json",
                                                     "after_file": "dup.json",
                                                     "text_field": "no_such_field"})
        assert r.status_code == 400, r.text
        assert r.json()["detail"].startswith(
            "基线数据集 里没有任何一条含字段 'no_such_field'")


class TestTheClampBites:
    """第 5 面：出口钳制不是摆设——删掉那行 `min/max` 这一支必须红"""

    def test_a_perfect_dataset_scores_exactly_one(self, http):
        client, _ = http
        r = gate(client, file="perfect.json")
        assert r.status_code == 200, r.text
        health = r.json()["health"]
        assert health["health_score"] == 1.0
        assert set(health["metrics"].values()) == {1.0}

    def test_the_tolerance_leftover_is_what_the_clamp_eats(self, http):
        """权重和 1.01（容差内）× 四项全 1.0 ⇒ raw = 1.01，出口必须是 1.0 而不是 1.01"""
        client, _ = http
        weights = [0.2525, 0.2525, 0.2525, 0.2525]
        r = gate(client, file="perfect.json", weights=weights)
        assert r.status_code == 200, r.text
        metrics = r.json()["health"]["metrics"]
        raw = sum(w * metrics[k] for w, k in zip(
            weights, ["completeness", "diversity", "quality_balance", "coverage"]))
        assert raw > 1.0, "这份夹具顶不出越界，钳制臂会退化成恒真"
        assert r.json()["health"]["health_score"] == 1.0

    def test_the_sdk_face_clamps_the_same_way_without_the_http(self):
        assert DatasetHealthScore(weights=[0.2525] * 4).score(PERFECT_ITEMS)["health_score"] \
            == 1.0

    def test_no_legitimate_weight_can_push_the_score_over_one(self, http):
        """值域承诺的反面：逐项 [0,1] + 和 ≈ 1.0 之后，越界只能来自容差那一小截

        改前 `[2.0, -0.5, -0.25, -0.25]` 能顶到 1.17（A319 的实测值），今天这一档是 400；
        这一支把「那一档现在回什么」钉住，而不是只钉「钳制存在」。
        """
        client, _ = http
        r = gate(client, weights=[2.0, -0.5, -0.25, -0.25])
        assert r.status_code == 400, r.text
        assert "weights[0] 必须在 0.0 到 1.0 之间" in r.json()["detail"]
