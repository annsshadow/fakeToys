# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L221（A270）：逐条翻译失败不再冒充成功——SDK / 聚合层 / 端点三面各钉一格

**本轮补的是哪一格**：L217 接消费层时实测坐实，`MultilingualSupport.batch_translate`
把逐条异常 `except Exception as e: logger.error(...)` 吞成「回退原文」，返回形状与全部
成功时**完全相同**。回退原文本身是对的（长度与顺序对齐是调用方的依赖），错的是它无声：
真后端只对部分条目超时/被限流时，产物里混着原文，而 `/api/dataset/translate` 回 200 且
`failed` 无处可查。本轮加的是**失败回执面**，不是新行为。

**三面各自负责一件事，所以三面各有正控制**：
- SDK（`batch_translate`）：下标是**输入下标**，不是 `as_completed` 的完成顺序。这一格
  最容易写错也最容易测漏，所以 `test_index_is_the_input_index_not_completion_order` 把
  失败条刻意做成**最先完成**的那条——按完成顺序记数的实现会拿到 0 而不是 2，当场红。
- 聚合层（`translate_dataset`）：跨批合并必须加批起点 `start`，下标才是全数据集坐标；
  样本上限（20）只在这一层出现，且计数取 SDK 的显式 `failed` 而不是 `len(errors)`。
- 端点：部分失败仍走 200（整批失败由 `ModelNotConfiguredError` 落 400，那是另一档），
  诚实读数在响应体里；`TranslateResponse` 少写一只键，FastAPI 就会把它从响应里静默裁掉。

**为什么样本上限不做成配置键**：`multilingual` 节现量四只键（`enabled` /
`default_target_lang` / `supported_langs` / `translate_batch_size`），加第五只要走
`config_validator` 规格表、`config.yaml`、节注册计数与普查尺子四处同步，而这一只数字
没有任何「按部署不同」的需求——它是响应体尺寸的保护栏，不是能力开关。按仓库「不加投机
配置」的惯例先写死在函数内，将来真有诉求再接（A272 谈的是同一个节的形状问题）。
"""

import json
import time
from pathlib import Path

import pytest

from augmentor import load_config
from augmentor.exceptions import ModelNotConfiguredError
from augmentor.language import translate_dataset
from augmentor.multilingual import MultilingualSupport

AI_DIR = Path(__file__).resolve().parent.parent.parent

#: 回执面新增的四只键——三面共用同一份名字，任一处改名都会让对应断言红。
RECEIPT_KEYS = {"failed", "failed_indices", "errors", "errors_truncated"}


def zh_items(count):
    return [{"instruction": f"猫喜欢吃什么食物{i}", "output": "猫粮"} for i in range(count)]


class SelectiveBackend:
    """按文本内容决定成败的假后端

    失败面必须能**按下标精确指定**，否则「回执说第 3 条没翻」这类主张没法验真。
    失败路径刻意不 sleep：这样被指定失败的那条总是最先完成，凡是把「完成顺序」当
    「输入顺序」的实现都会在这里露出来。
    """

    def __init__(self, fail_on=(), delay=0.0):
        self.fail_on = tuple(fail_on)
        self.delay = delay
        self.calls = 0

    def generate(self, prompt: str) -> str:
        self.calls += 1
        text = prompt.splitlines()[-1]
        for marker in self.fail_on:
            if marker in text:
                raise RuntimeError(f"backend down: {marker}")
        if self.delay:
            time.sleep(self.delay)
        return "TRANSLATED"


def items_file(tmp_path, items):
    path = tmp_path / "l221_items.json"
    path.write_text(json.dumps(items, ensure_ascii=False), encoding="utf-8")
    return str(path)


def language_config(tmp_path, **overrides):
    config = load_config(str(AI_DIR / "config.yaml"))
    config.versioning.storage_dir = str(tmp_path / "versions")
    for dotted, value in overrides.items():
        section, _, field = dotted.partition(".")
        setattr(getattr(config, section), field, value)
    return config


class TestSdkReceipt:
    """SDK 面：`batch_translate` 的回执说「哪几条没翻」，且下标是输入下标"""

    def test_index_is_the_input_index_not_completion_order(self):
        """3 条成功 / 1 条失败，失败那条**最先**返回：按完成顺序记下标会拿到 0"""
        items = [{"instruction": f"猫喜欢吃什么食物{i}"} for i in range(4)]
        backend = SelectiveBackend(fail_on=["食物1"], delay=0.05)
        support = MultilingualSupport(model_backend=backend, max_workers=4)

        report = support.batch_translate(items, target_lang="en")

        assert report["failed"] == 1
        assert report["failed_indices"] == [1]
        assert report["errors"] == [{"index": 1, "error": "RuntimeError: backend down: 食物1"}]

    def test_receipt_is_silent_only_when_nothing_failed(self):
        """全部成功时回执是零值，且 `items` 形状与旧返回一致（消费方不受影响）"""
        items = zh_items(3)
        support = MultilingualSupport(model_backend=SelectiveBackend())

        report = support.batch_translate(items, target_lang="en")

        assert report["failed"] == 0
        assert report["failed_indices"] == []
        assert report["errors"] == []
        assert len(report["items"]) == 3
        assert all(i["instruction"] == "TRANSLATED" for i in report["items"])

    def test_fallback_still_aligns_length_and_keeps_source_text(self):
        """失败条目仍回退原文、长度与顺序不变——本轮保留的行为，不许顺手改掉"""
        items = [{"instruction": f"猫喜欢吃什么食物{i}"} for i in range(3)]
        support = MultilingualSupport(model_backend=SelectiveBackend(fail_on=["食物0", "食物2"]))

        report = support.batch_translate(items, target_lang="en")

        assert [i["instruction"] for i in report["items"]] == [
            "猫喜欢吃什么食物0", "TRANSLATED", "猫喜欢吃什么食物2",
        ]
        assert report["failed_indices"] == [0, 2]

    def test_errors_are_not_truncated_at_the_sdk_layer(self):
        """SDK 的 `errors` 是本批**全量**记录：截断属聚合层，两处都截就把下标也弄丢了"""
        items = zh_items(25)
        support = MultilingualSupport(model_backend=SelectiveBackend(fail_on=["食物"]))

        report = support.batch_translate(items, target_lang="en")

        assert report["failed"] == 25
        assert len(report["errors"]) == 25
        assert report["failed_indices"] == list(range(25))

    def test_does_not_mutate_input(self):
        """回执化之后仍然不改调用方数据（旧守卫，形状迁移时容易顺手写坏）"""
        items = [{"instruction": "猫喜欢吃什么食物"}]
        support = MultilingualSupport(model_backend=SelectiveBackend(fail_on=["猫"]))
        support.batch_translate(items, target_lang="en")

        assert items[0] == {"instruction": "猫喜欢吃什么食物"}


class TestAggregationReceipt:
    """聚合层：跨批下标、样本上限、计数来源"""

    def test_indices_are_dataset_global_across_batches(self):
        """批宽 2 时失败落在第 2、3 批：下标必须是 3/4 而不是批内的 1/0"""
        items = zh_items(5)
        backend = SelectiveBackend(fail_on=["食物3", "食物4"])

        result = translate_dataset(
            items, backend=backend, supported_langs=["zh", "en"],
            default_target_lang="en", batch_size=2,
        )

        assert result["failed_indices"] == [3, 4]
        assert result["failed"] == 2
        assert result["batches"] == 3
        assert result["total"] == 5

    def test_error_sample_is_capped_and_says_so(self):
        """25 条全失败 ⇒ `errors` 只留 20 条样本并声明截断，`failed` 与下标不受样本收窄"""
        items = zh_items(25)
        backend = SelectiveBackend(fail_on=["食物"])

        result = translate_dataset(
            items, backend=backend, supported_langs=["zh", "en"],
            default_target_lang="en", batch_size=10,
        )

        assert result["failed"] == 25
        assert len(result["failed_indices"]) == 25
        assert len(result["errors"]) == 20
        assert result["errors_truncated"] is True

    def test_exactly_the_sample_size_is_not_truncated(self):
        """边界：20 条失败正好装满样本，`errors_truncated` 必须是 False（`>` 而非 `>=`）"""
        items = zh_items(20)
        backend = SelectiveBackend(fail_on=["食物"])

        result = translate_dataset(
            items, backend=backend, supported_langs=["zh", "en"],
            default_target_lang="en", batch_size=5,
        )

        assert result["failed"] == 20
        assert len(result["errors"]) == 20
        assert result["errors_truncated"] is False

    def test_sample_keeps_the_global_indices_it_reports(self):
        """样本里的下标也得是全局坐标，且升序：截断不能把坐标换一套轴"""
        items = zh_items(30)
        backend = SelectiveBackend(fail_on=["食物"])

        result = translate_dataset(
            items, backend=backend, supported_langs=["zh", "en"],
            default_target_lang="en", batch_size=4,
        )

        reported = [e["index"] for e in result["errors"]]
        assert reported == sorted(reported)
        assert reported == result["failed_indices"][:20]

    def test_all_success_receipt_is_zero_and_items_shape_unchanged(self):
        """全成功时聚合层的既有读数一字未变（零漂移正控制）"""
        items = zh_items(6)
        backend = SelectiveBackend()

        result = translate_dataset(
            items, backend=backend, supported_langs=["zh", "en"],
            default_target_lang="en", batch_size=4,
        )

        assert result["failed"] == 0
        assert result["failed_indices"] == []
        assert result["errors"] == []
        assert result["errors_truncated"] is False
        assert result["total"] == 6
        assert result["batches"] == 2
        assert [i["instruction"] for i in result["items"]] == ["TRANSLATED"] * 6

    def test_missing_backend_still_raises_before_any_batch(self):
        """整批失败这一档由本层在分批之前出声（L217 既有语义，回执化不能把它吸进 200）"""
        with pytest.raises(ModelNotConfiguredError):
            translate_dataset(
                zh_items(3), backend=None, supported_langs=["zh", "en"],
                default_target_lang="en", batch_size=2,
            )


class TestEndpointReceipt:
    """端点面：200 带着回执，且四只新键真的出现在响应里"""

    def _client(self, monkeypatch, config, backend=None):
        import api.deps as deps
        from api.main import app
        from fastapi.testclient import TestClient

        stub = type("PipelineStub", (), {})()
        stub.config = config
        stub.model_backend = backend
        monkeypatch.setattr(deps, "_pipeline", stub)
        return TestClient(app)

    def _translate(self, tmp_path, monkeypatch, backend, **overrides):
        config = language_config(tmp_path, **{"multilingual.enabled": True, **overrides})
        client = self._client(monkeypatch, config, backend=backend)
        return client.post(
            "/api/dataset/translate",
            json={"input_file": items_file(tmp_path, zh_items(3))},
        )

    def test_partial_failure_is_200_with_a_loud_receipt(self, tmp_path, monkeypatch):
        """中间那条没翻成：`items` 读起来像成功，回执是唯一能行动的面"""
        response = self._translate(tmp_path, monkeypatch,
                                   SelectiveBackend(fail_on=["食物1"]))

        assert response.status_code == 200, response.text
        body = response.json()
        assert body["failed"] == 1
        assert body["failed_indices"] == [1]
        assert body["errors"][0]["index"] == 1
        assert body["errors"][0]["error"].startswith("RuntimeError:")
        assert body["errors_truncated"] is False
        # 只看 items 会读出一整份译文——这正是回执必须存在的理由
        assert body["items"][1]["instruction"] == "猫喜欢吃什么食物1"
        assert body["items"][0]["instruction"] == "TRANSLATED"

    def test_full_success_reports_zero(self, tmp_path, monkeypatch):
        response = self._translate(tmp_path, monkeypatch, SelectiveBackend())

        assert response.status_code == 200, response.text
        body = response.json()
        assert body["failed"] == 0
        assert body["failed_indices"] == []
        assert body["errors"] == []
        assert body["errors_truncated"] is False

    def test_response_carries_every_receipt_key(self, tmp_path, monkeypatch):
        """`TranslateResponse` 少写任一只键，FastAPI 就把它裁掉：这条钉的是「在响应里」"""
        response = self._translate(tmp_path, monkeypatch,
                                   SelectiveBackend(fail_on=["食物2"]))

        assert RECEIPT_KEYS <= set(response.json())

    def test_openapi_declares_the_receipt(self):
        """契约文档面也得有四键——只改响应模型而没进 OpenAPI 是另一种「写了没人看得见」"""
        from api.main import app

        schema = app.openapi()["components"]["schemas"]["TranslateResponse"]

        assert RECEIPT_KEYS <= set(schema["properties"])
        assert RECEIPT_KEYS <= set(schema["required"])
