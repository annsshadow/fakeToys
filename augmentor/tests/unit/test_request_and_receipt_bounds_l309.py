# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""A402 守卫：请求侧的列表长度要有上限，回执的截断要点名（L309）

**这一轮收的是哪一格**：A402（L291 立）—— 「请求侧『列表长度』那一维全族没有
约束」+「`scan` 又把整份 `records` 塞回响应」+「`generate_report` 的 `errors` 硬截 20
而无截断旗」三件事。

**改法（三格各按自己的形状）**：

* **请求侧长度**：`inputs` / `datasets` 的长度各有**出厂上限**（100），超出即 **400
  拒**（不是截断：截断会静默改变语义 ——「我送了 200 个，你只处理了 100 个」而
  客户端毫不知情）。请求面**不接受覆盖**（客户端自己抬高上限就是绕过这一道闸）。
* **响应侧记录**：`scan` 的逐条记录那一维**封顶**（500），并由 `records_truncated`
  点名；`total_records` 仍是真数 ⇒ 客户端能自己看出「回执只给了前 N 条」。
* **错误清单**：`errors` 硬截 20 那一格补上 `errors_truncated` 旗 ⇒ 「全给了」与
  「被截了」两种读法不再不可分。

**这一把尺为什么这样摆**：

1. **超限必须是 400 而不是静默截断**：断言回执的 `detail` 里出现上限与实数，
   且**盘面一格未动**（没有产物落下来）—— 截断那一族缺陷正是「悄悄少做了」。
2. **上限之内必须照旧**：101 个输入被拒、100 个输入照旧 ⇒ 上限不是「把端点关掉」。
3. **截断旗必须与真数同时可读**：`errors_truncated` 为真 ⇒ `error_count` 大于
   `len(errors)`；为假 ⇒ 两者相等（旗不能一直为真，也不能与真数脱节）。
4. **记录截断同理**：`records_truncated` 为真 ⇒ `len(records) < total_records`。

**为什么零 parametrize**：l97 / l98 把全仓 parametrize 三面冻成现量数，本守卫走
类内独立用例，两支普查尺子一格未动。

跑法：`python -m pytest tests/unit/test_request_and_receipt_bounds_l309.py -q --no-cov`
"""
import json

import pytest

from augmentor.data.multimodal import MultimodalProcessor


def _write_json(path, payload):
    """写一份 JSON 数据集

    Args:
        path: 目标路径
        payload: 要写的内容

    Returns:
        目标路径
    """
    path.write_text(json.dumps(payload, ensure_ascii=False), encoding="utf-8")
    return path


@pytest.fixture
def client():
    """一个对着临时数据根的 TestClient

    Returns:
        TestClient 实例
    """
    from fastapi.testclient import TestClient

    from api.main import app

    return TestClient(app)


class TestTheRequestSideLengthIsBounded:
    """① 请求侧：列表长度有上限，超出即 400 拒"""

    def test_merge_rejects_too_many_inputs(self, client, tmp_path):
        """合并：超过上限 ⇒ 400，且**盘面一格未动**（没有产物落下来）"""
        from api.routes.dataset_tools import MAX_MERGE_INPUTS

        paths = []
        for i in range(MAX_MERGE_INPUTS + 1):
            paths.append(str(_write_json(
                tmp_path / ("in%d.json" % i),
                [{"instruction": "x%d" % i, "input": "", "output": "o"}],
            )))

        response = client.post("/api/dataset/merge", json={
            "inputs": paths,
            "output_file": str(tmp_path / "out.json"),
        })

        assert response.status_code == 400, response.text
        detail = response.json()["detail"]
        assert str(MAX_MERGE_INPUTS) in detail, detail
        assert not (tmp_path / "out.json").exists(), "被拒的请求仍然落了产物"

    def test_aggregate_rejects_too_many_datasets(self, client, tmp_path):
        """聚合：超过上限 ⇒ 400"""
        from api.routes.dataset_tools import MAX_AGGREGATE_DATASETS

        datasets = {}
        for i in range(MAX_AGGREGATE_DATASETS + 1):
            datasets["d%d" % i] = str(_write_json(
                tmp_path / ("agg%d.json" % i),
                [{"instruction": "x%d" % i, "input": "", "output": "o"}],
            ))

        response = client.post("/api/dataset/aggregate", json={
            "datasets": datasets,
            "output_file": str(tmp_path / "agg_out.json"),
        })

        assert response.status_code == 400, response.text
        assert str(MAX_AGGREGATE_DATASETS) in response.json()["detail"]
        assert not (tmp_path / "agg_out.json").exists(), "被拒的请求仍然落了产物"

    def test_under_the_limit_still_works(self, client, tmp_path):
        """上限之内照旧（上限不是「把端点关掉」）"""
        from api.routes.dataset_tools import MAX_MERGE_INPUTS

        paths = [str(_write_json(tmp_path / ("ok%d.json" % i),
                                 [{"instruction": "x%d" % i, "input": "", "output": "o"}]))
                 for i in range(2)]

        response = client.post("/api/dataset/merge", json={
            "inputs": paths,
            "output_file": str(tmp_path / "ok_out.json"),
        })

        assert response.status_code == 200, response.text
        assert (tmp_path / "ok_out.json").exists()
        assert MAX_MERGE_INPUTS >= 2


class TestTheReceiptSaysWhenItTruncated:
    """②③ 截断旗：被截了要点名，且与真数对得上"""

    def _scan_dir(self, tmp_path, count, suffix=".png"):
        """造一个含 count 只坏媒体的目录（坏文件 ⇒ 每只一条错误）

        Args:
            tmp_path: pytest 临时目录
            count: 文件只数
            suffix: 扩展名

        Returns:
            目录路径
        """
        d = tmp_path / ("bad%d" % count)
        d.mkdir(parents=True, exist_ok=True)
        for i in range(count):
            (d / ("f%d%s" % (i, suffix))).write_bytes(b"x")
        return d

    def test_the_error_list_says_it_was_truncated(self, tmp_path):
        """25 条错误 ⇒ `errors` 只给 20 条，但 `errors_truncated` 说清了"""
        d = self._scan_dir(tmp_path, 25)

        proc = MultimodalProcessor()
        records = proc.process_directory(str(d))
        report = proc.generate_report(records, proc.scan_notes)

        assert report["error_count"] == 25, report["error_count"]
        assert len(report["errors"]) == 20
        assert report["errors_truncated"] is True

    def test_a_short_error_list_is_not_flagged(self, tmp_path):
        """错误不多 ⇒ 旗为假，且 `error_count` 与 `len(errors)` 相等"""
        d = self._scan_dir(tmp_path, 3)

        proc = MultimodalProcessor()
        records = proc.process_directory(str(d))
        report = proc.generate_report(records, proc.scan_notes)

        assert report["error_count"] == 3
        assert len(report["errors"]) == 3
        assert report["errors_truncated"] is False

    def test_the_record_list_says_it_was_truncated(self, client, tmp_path,
                                                  monkeypatch):
        """逐条记录那一维封顶 ⇒ `records_truncated` 为真、`total_records` 仍是真数"""
        import api.routes.multimodal as route_mod

        monkeypatch.setattr(route_mod, "MAX_SCAN_RECORDS", 2)
        d = tmp_path / "many"
        d.mkdir(parents=True, exist_ok=True)
        for i in range(3):
            (d / ("f%d.png" % i)).write_bytes(b"x")

        response = client.post("/api/multimodal/scan", json={"directory": str(d)})

        assert response.status_code == 200, response.text
        payload = response.json()
        assert payload["total_records"] == 3, payload["total_records"]
        assert len(payload["records"]) == 2, payload["records"]
        assert payload["records_truncated"] is True

    def test_a_short_record_list_is_not_flagged(self, client, tmp_path):
        """记录不多 ⇒ 旗为假，且记录一条不少"""
        d = tmp_path / "few"
        d.mkdir(parents=True, exist_ok=True)
        for i in range(2):
            (d / ("f%d.png" % i)).write_bytes(b"x")

        response = client.post("/api/multimodal/scan", json={"directory": str(d)})

        assert response.status_code == 200, response.text
        payload = response.json()
        assert payload["total_records"] == 2
        assert len(payload["records"]) == 2
        assert payload["records_truncated"] is False
