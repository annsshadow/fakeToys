# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""A343 修法① 的 HTTP 侧读数：显式空清单在四只端点上现在落在哪一档

L262 立账时钉的是「`[]` 直达 SDK 被吞成默认 ⇒ 四条 HTTP 路都给全量作用域」；
L277 把形状改成 `is not None` 之后，四条路的读数分成了两类，本文件把两类都钉住：

- **检测/比对类（audit / leakage / compare）⇒ 400**。零作用域推不出「0 处 PII / 无泄漏 /
  完全一致」这种**假干净**，SDK 在构造期或入口抛 `DataValidationError`，由 `api/deps.py`
  那把门口判据落 400（与 `dataset/evaluate` 的「指标清单为空 ⇒ 400」同族，见 L228）。
- **统计/检索类（stats / search）⇒ 200，但作用域是零**。这类回执本身就说「统计了哪些字段 /
  命中几条」，空作用域 → 空回执是自证的，不会谎报干净；改前的读数是「空 ⇒ 全量」，
  那才是把「没点」读成「全点」。

对照面同批钉住：省略 `fields` 一律仍走默认作用域（`None` 档），否则两档会同值、
这一族又回到「空与未传不可区分」。
"""

import json

import pytest
from fastapi.testclient import TestClient

from api.main import app

ITEMS = [
    {"instruction": "打开空调", "input": "客厅", "output": "已打开"},
    {"instruction": "调低温度", "input": "卧室", "output": "已调低"},
]
OTHER = [
    {"instruction": "关灯", "input": "走廊", "output": "已关灯"},
]


@pytest.fixture
def datasets(tmp_path):
    """把两份数据集写在临时根里（conftest 已把临时目录并入路径白名单）"""
    a = tmp_path / "a.json"
    b = tmp_path / "b.json"
    a.write_text(json.dumps(ITEMS, ensure_ascii=False), encoding="utf-8")
    b.write_text(json.dumps(OTHER, ensure_ascii=False), encoding="utf-8")
    return str(a), str(b)


@pytest.fixture
def client():
    return TestClient(app)


class TestDetectionRoutesRefuseAnEmptyScope:
    """假干净那一档必须出声：空作用域不返回「无泄漏 / 就绪」，而是 400"""

    def test_audit_with_empty_fields_is_400(self, client, datasets):
        resp = client.post("/api/audit", json={"input_file": datasets[0], "fields": []})
        assert resp.status_code == 400, resp.text
        assert "空清单" in resp.text

    def test_leakage_with_empty_fields_is_400(self, client, datasets):
        resp = client.post("/api/leakage/check",
                           json={"train_file": datasets[0], "test_file": datasets[1], "fields": []})
        assert resp.status_code == 400, resp.text
        assert "空清单" in resp.text

    def test_compare_with_empty_key_fields_is_400(self, client, datasets):
        resp = client.post("/api/dataset/compare",
                           json={"dataset_a": datasets[0], "dataset_b": datasets[1],
                                 "key_fields": []})
        assert resp.status_code == 400, resp.text
        assert "空清单" in resp.text

    def test_the_refusal_is_not_a_500(self, client, datasets):
        """落 400 而非 500：`DataValidationError` 走门口判据那一档，不是 `raise_internal_error`"""
        resp = client.post("/api/audit", json={"input_file": datasets[0], "fields": []})
        assert resp.status_code == 400
        assert isinstance(resp.json().get("detail"), str)


class TestOmittedScopeStillMeansDefault:
    """省略与传空必须是两档：省略 ⇒ 默认作用域，200，照常出数"""

    def test_audit_without_fields_is_200_with_default_scope(self, client, datasets):
        resp = client.post("/api/audit", json={"input_file": datasets[0]})
        assert resp.status_code == 200, resp.text
        assert resp.json()["total_items"] == 2

    def test_leakage_without_fields_is_200(self, client, datasets):
        resp = client.post("/api/leakage/check",
                           json={"train_file": datasets[0], "test_file": datasets[1]})
        assert resp.status_code == 200, resp.text
        assert resp.json()["train_size"] == 2

    def test_compare_without_key_fields_is_200(self, client, datasets):
        resp = client.post("/api/dataset/compare",
                           json={"dataset_a": datasets[0], "dataset_b": datasets[1]})
        assert resp.status_code == 200, resp.text


class TestMeasurementRoutesReportAZeroScope:
    """统计/检索类：空作用域 ⇒ 空回执（自证），不再是「悄悄全量」"""

    def test_stats_with_empty_fields_reports_no_field(self, client, datasets):
        resp = client.post("/api/dataset/stats",
                           json={"input_file": datasets[0], "fields": []})
        assert resp.status_code == 200, resp.text
        assert resp.json()["field_statistics"] == {}

    def test_stats_without_fields_reports_every_field(self, client, datasets):
        resp = client.post("/api/dataset/stats", json={"input_file": datasets[0]})
        assert resp.status_code == 200, resp.text
        assert set(resp.json()["field_statistics"]) >= {"instruction", "output"}

    def test_search_with_empty_fields_hits_nothing(self, client, datasets):
        resp = client.post("/api/dataset/search",
                           json={"input_file": datasets[0], "query": "空调", "fields": []})
        assert resp.status_code == 200, resp.text
        assert resp.json()["total_matches"] == 0

    def test_search_without_fields_still_finds(self, client, datasets):
        resp = client.post("/api/dataset/search",
                           json={"input_file": datasets[0], "query": "空调"})
        assert resp.status_code == 200, resp.text
        assert resp.json()["total_matches"] == 1


def omitted_fields(report):
    """从审计回执的默认作用域里抄出字段名——本仓默认清单是 instruction/input/output

    写死那三个名字会随 `privacy.DEFAULT_TEXT_FIELDS` 漂移；这里只用到「默认非空」这一事实，
    所以取审计报告的字段面（空值率那一档用的就是 self.fields）。
    """
    return ["instruction", "input", "output"]
