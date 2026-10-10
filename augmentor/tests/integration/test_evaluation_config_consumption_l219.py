# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L219（A138 ② 档）：`config.evaluation.metrics` 接进 `/api/dataset/evaluate` 的省略支路

**接的是哪一格**：属性面普查（`tests/unit/test_config_reader_census_l217.py`）现量
`evaluation.metrics` 零产品读者——端点一直把 `request.metrics` 直接交给 SDK，而
`ModelEvaluator.__init__` 写死 `metrics or ["bleu", "rouge_l", "similarity"]`，配置那只清单
从未参与。本轮按权威序 **请求参数 > 配置键 > 内置默认** 把它接上。

**为什么这格是零漂移接线而不是行为变更**：出厂配置值与 SDK 回落值**逐字同**
（`config.yaml` 的 `evaluation.metrics` 就是那三只）。所以「按出厂配置部署」的读数一字不变，
只有改过配置的部署会看到差别——判据取的是后者：**改配置 ⇒ 响应的指标集合跟着变**。

**空清单为什么在门口 400**：SDK 那行 `metrics or [内置]` 把空列表当假值，传下去会被**静默补成
三只**——配置说「一个都不算」而响应三个指标齐全。接线恰好让这条死角第一次可达（普查里
`evaluation.metrics` 有读者之后，`[]` 才会真的流到那儿），所以接线与出声必须同轮做，
并由 `TestEmptyAndUnknownConfigLists` 钉住两臂。

**请求侧空列表的口径**：`metrics: []` 与省略**不可区分**（pydantic 层就是 falsy），
按权威序落回配置而不是报错。这条不是猜的，是本轮实测固定下来的语义
（`test_empty_request_list_falls_back_to_config`）。

**没接的那只**：`evaluation.reference_field` 也在这节，但请求模型的默认是硬值 `"output"`，
调用方省略与显式传 `"output"` 在 API 面上同一个形状，要接就得把字段改 `Optional` ——
那是契约变化，另立 A274 拍，不在本轮顺手做。
"""

import json
from pathlib import Path
from types import SimpleNamespace

import pytest

AI_DIR = Path(__file__).resolve().parent.parent.parent

#: 生成侧与参考侧同宽两条，值取得让三种指标互不相同（`bleu` 与 `rouge_l` 在这种
#: 短中文串上会同为 0.0，所以判据一律取**指标集合**而不是数值）
ITEMS = [
    {"output": "租房押金一个月", "reference": "押金为一个月租金"},
    {"output": "合同期一年", "reference": "合同期限一年"},
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

    path = tmp_path / "l219_items.json"
    path.write_text(json.dumps(ITEMS, ensure_ascii=False), encoding="utf-8")
    return SimpleNamespace(
        client=TestClient(app), config=config, pipeline=deps._pipeline, file="l219_items.json"
    )


def evaluate(env, **body):
    """打 `/api/dataset/evaluate`，默认两侧都取 `output`（省略 `metrics` 就是本轮的判据面）"""
    payload = {"generated_file": env.file, "reference_file": env.file,
               "generated_field": "output", "reference_field": "reference"}
    payload.update(body)
    return env.client.post("/api/dataset/evaluate", json=payload)


class TestConfigMetricsIsConsumed:
    """配置那只清单真的决定响应里算哪些指标"""

    def test_narrower_config_narrows_the_response(self, env):
        """只留 `similarity`：响应必须只剩这一项。

        这一臂专打「写死内置三只」——那句 `metrics or [...]` 若还住在路由里，
        配置改成一只也不会有人看见。
        """
        env.config.evaluation.metrics = ["similarity"]
        response = evaluate(env)
        assert response.status_code == 200, response.text
        assert set(response.json()["metrics"]) == {"similarity"}

    def test_request_beats_config(self, env):
        """权威序第一档：请求显式给 `bleu` 时配置那一只 `similarity` 不许掺进来"""
        env.config.evaluation.metrics = ["similarity"]
        response = evaluate(env, metrics=["bleu"])
        assert response.status_code == 200, response.text
        assert set(response.json()["metrics"]) == {"bleu"}

    def test_factory_config_is_the_same_three_metrics(self, env):
        """零漂移钉：出厂配置 + 省略请求 ≡ 显式传出厂那三只

        接线若偷偷改变了出厂部署的读数，这一臂会红——它比「响应恰好有三个键」强，
        因为它把整份 `metrics` 字典拿去和显式清单的结果逐值比。
        """
        factory = list(env.config.evaluation.metrics)
        assert factory == ["bleu", "rouge_l", "similarity"], f"出厂值变了，本臂的等价前提失效: {factory}"

        by_config = evaluate(env)
        by_request = evaluate(env, metrics=factory)
        assert by_config.status_code == 200, by_config.text
        assert by_config.json()["metrics"] == by_request.json()["metrics"]
        assert by_config.json()["sample_count"] == by_request.json()["sample_count"]

    def test_multi_metric_config_order_does_not_leak_into_the_response(self, env):
        """配置只决定**集合**；出厂顺序不该成为对外承诺"""
        env.config.evaluation.metrics = ["similarity", "bleu"]
        response = evaluate(env)
        assert set(response.json()["metrics"]) == {"similarity", "bleu"}

    def test_empty_request_list_falls_back_to_config(self, env):
        """`metrics: []` 与省略不可区分 ⇒ 按权威序落回配置，而不是报错也不是全算"""
        env.config.evaluation.metrics = ["rouge_l"]
        response = evaluate(env, metrics=[])
        assert response.status_code == 200, response.text
        assert set(response.json()["metrics"]) == {"rouge_l"}


class TestEmptyAndUnknownConfigLists:
    """生效值也要过白名单与「非空」判据——接线把配置值第一次送进这条路径"""

    def test_empty_config_list_is_rejected_instead_of_silently_filled(self, env):
        """配置留空：SDK 的 `metrics or [内置]` 会把它补成三只，本轮在门口出声"""
        env.config.evaluation.metrics = []
        response = evaluate(env)
        assert response.status_code == 400, response.text
        detail = response.json()["detail"]
        assert "评估指标清单为空" in detail
        assert "config.evaluation.metrics" in detail

    def test_unknown_metric_in_config_is_a_400_not_a_500(self, env):
        """配置写错指标名：报的是白名单（客户端可修），不是属性错误也不是 500"""
        env.config.evaluation.metrics = ["bleu", "bleu_ish"]
        response = evaluate(env)
        assert response.status_code == 400, response.text
        detail = response.json()["detail"]
        assert "未知的评估指标" in detail
        assert "bleu_ish" in detail
        assert "similarity" in detail, "报错要把可选项一起给出"

    def test_request_side_unknown_metric_still_400(self, env):
        """既有语义不许被接线顺手改掉：请求侧点错名字仍然是 400 白名单"""
        response = evaluate(env, metrics=["nonsense"])
        assert response.status_code == 400, response.text
        assert "未知的评估指标" in response.json()["detail"]
