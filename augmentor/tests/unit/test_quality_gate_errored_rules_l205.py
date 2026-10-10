# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L205（B266③）：门禁必须分开「规则算不出来」与「算出来不及格」

**缺陷**：`QualityGate.run()` 的 `except` 分支把求值失败的规则记成
`satisfied = False`，于是它和真不达标的规则进**同一栏** `failed_rules`。后果：
`verdict=FAILED` 分不清是**数据不达标**还是**规则本身写错了**——而后者该修的是
规则。同族还有 `GateRule.evaluate` 对缺失键直接 `return False`（`metric_key` 拼错
同样读作「不及格」）。

**三面必须一起改**（这是 B266③ 的处置前提，缺一面就比不修更糟）：
1. SDK `GateReport` 新增 `errored_rules` 字段并导出到 `to_dict()`；
2. API `GateReportResponse` 同步声明该键 —— 否则 FastAPI 按 `response_model`
   过滤返回值，新键会被**静默裁掉**（L27 记过的坑），「求值失败」反而看不见了；
3. 本守卫钉住三面一致 + `to_dict()` 的键集 == 响应模型的键集。

**口径决定**：求值失败**不参与判决**（`verdict` 照旧由真不达标的规则决定），但
必须在报告里点名。理由是：一条坏规则让门禁判 FAILED 是「工具坏了」，
该做的是修规则，不是拦数据。
"""

import pytest

from augmentor.quality_gate import GateReport, GateRule, QualityGate


def _rule(name, metric_key="pass_rate", operator=">=", value=0.5,
          severity="error"):
    return GateRule(name=name, metric_key=metric_key, operator=operator,
                    value=value, severity=severity)


class TestErroredRulesAreSeparated:
    """求值失败与不达标必须分成两栏"""

    def test_broken_rule_lands_in_errored_not_failed(self):
        gate = QualityGate(rules=[
            _rule("good", value=0.9),                    # 0.8 >= 0.9 假 ⇒ 真不达标
            _rule("broken", operator=">=", value="x"),   # float >= str ⇒ TypeError
        ])
        report = gate.run({"pass_rate": 0.8})
        assert report.failed_rules == ["good"], report.failed_rules
        assert report.errored_rules == ["broken"], report.errored_rules

    def test_errored_rule_does_not_change_the_verdict(self):
        """算不出来不是「不达标」这个事实 ⇒ 不参与判决"""
        gate = QualityGate(rules=[_rule("broken", value="x")])
        report = gate.run({"pass_rate": 0.8})
        assert report.verdict == "passed", report.verdict
        assert report.errored_rules == ["broken"]

    def test_errored_warning_rule_is_not_a_warning(self):
        """warning 级的坏规则也不算「告警」——它压根没被求值"""
        gate = QualityGate(rules=[
            _rule("w_broken", severity="warning", value="x")])
        report = gate.run({"pass_rate": 0.8})
        assert report.warned_rules == []
        assert report.errored_rules == ["w_broken"]

    def test_missing_metric_key_reads_as_not_satisfied(self):
        """指标键不在 metrics 里 ⇒ `evaluate` 返回 False（仓内既有契约，L205 不改它）

        这一条钉住**边界**：`evaluate` 对缺失键是 `return False` 而不是抛异常，
        所以它进 `failed_rules`（不达标）而不是 `errored_rules`（算不出来）。
        L205 只改「抛异常」那一支的口径，不动这一支——动了它就是行为变更轮。
        """
        gate = QualityGate(rules=[_rule("absent", metric_key="nope")])
        report = gate.run({"pass_rate": 0.8})
        assert report.errored_rules == [], report.errored_rules
        assert report.failed_rules == ["absent"]


class TestHappyPathIsUnchanged:
    """防修过头：没有坏规则时，两栏都该是空的，verdict 照旧"""

    def test_all_pass(self):
        gate = QualityGate(rules=[_rule("a", value=0.5)])
        report = gate.run({"pass_rate": 0.8})
        assert report.verdict == "passed"
        assert report.failed_rules == [] and report.warned_rules == []
        assert report.errored_rules == []

    def test_real_failure_still_fails(self):
        gate = QualityGate(rules=[_rule("a", value=0.9)])
        report = gate.run({"pass_rate": 0.8})
        assert report.verdict == "failed"
        assert report.failed_rules == ["a"]

    def test_warning_still_warns(self):
        gate = QualityGate(rules=[_rule("w", severity="warning", value=0.9)])
        report = gate.run({"pass_rate": 0.8})
        assert report.verdict == "warned"
        assert report.warned_rules == ["w"]


class TestThreeFacesAgree:
    """SDK / to_dict / 响应模型必须声明同一组键"""

    def test_to_dict_exports_the_new_key(self):
        report = GateReport(verdict="failed", errored_rules=["b"])
        assert report.to_dict()["errored_rules"] == ["b"]

    def test_to_dict_keys_are_exactly_the_dataclass_fields(self):
        import dataclasses
        report = GateReport(verdict="passed")
        fields = {f.name for f in dataclasses.fields(report)}
        exported = set(report.to_dict()) - {"passed"}     # passed 是 property
        assert exported == fields, (
            f"to_dict 少导出 {fields - exported} 或多出 {exported - fields}"
        )

    def test_response_model_declares_every_to_dict_key(self):
        """这一条就是 L27 那个坑的守门：模型少一键 ⇒ 响应里静默少一键"""
        from api.routes.quality import GateReportResponse

        model_fields = set(GateReportResponse.model_fields)
        exported = set(GateReport(verdict="passed").to_dict())
        assert model_fields == exported, (
            f"GateReportResponse 与 GateReport.to_dict() 键集不一致："
            f"模型少 {exported - model_fields}（响应里会静默消失）、"
            f"模型多 {model_fields - exported}"
        )
