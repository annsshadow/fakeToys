# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L217：`multilingual` 四键的消费证据——`MultilingualSupport` 从零实例变成有产品的路径

**本轮补的是哪一格**：属性面普查（`tests/unit/test_config_reader_census_l217.py`）给出的
现量是 `multilingual` 节四只键全部零读者，根因与 L216 的 vector 同一形状——`augmentor/multilingual.py`
的 `MultilingualSupport` 在产品侧**一个实例都没建过**（`api/`、`augmentor/`、CLI 全树零调用者，
只有测试 import）。能力早就写好了（语言检测、批量翻译、语言分布），缺的是消费者，所以本轮
接的是消费者：`augmentor/language.py` + 两条端点。

**判据一律取行为差**（L214 立的规矩），四条各钉一只键：
- `supported_langs` ⇒ 画像里落在清单外的检测标签跟着它变；翻译目标不在清单内时 400；
- `default_target_lang` ⇒ 请求不带 `target_lang` 时生效的目标语言跟着它变。**喂的是与
  SDK 内置默认（`"en"`）可区间的 `"ja"`**，否则「写死 en」这类缺陷测不出来（L216 的同款教训）；
- `translate_batch_size` ⇒ 批次数量与每批长度跟着它变，且批宽 1 时并发数真的是 1；
- `enabled` ⇒ 翻译路径开不开；闸的位置在**读盘之前**，所以关掉闸的部署不会被一条越界
  文件名拖着去做白名单解析与整份读盘。

**为什么 `mixed` 出现在清单外不算缺陷**：`detect_language` 产出的是检测标签，`mixed`
不对应任何语言码，所以它天然不在 `supported_langs` 里。画像如实列出它，翻译路径也照常
处理混合文本（`translate` 只按目标语言判，源语言只做同语言短路）。
"""

import json
import threading
import time
from pathlib import Path

import pytest

from augmentor import load_config
from augmentor.exceptions import DataValidationError
from augmentor.language import language_profile, translate_dataset

AI_DIR = Path(__file__).resolve().parent.parent.parent

ITEMS = [
    {"instruction": "猫喜欢吃什么食物", "output": "猫喜欢吃鱼肉和猫粮"},
    {"instruction": "狗怎么训练定点排便", "output": "用零食引导到固定地点"},
    {"instruction": "How do I train a hamster to use a wheel?", "output": "Use seeds."},
    {"instruction": "今天的会议安排是什么", "output": "上午十点在产品会议室"},
    {"instruction": "How to write a weekly report", "output": "Progress, risk, plan"},
]

#: 全中文 5 条：并发上界的量测面。混进英文项会被同语言短路吃掉调用数（见
#: `test_same_language_items_never_reach_the_model`），并发读数就不再等于批宽。
ALL_ZH = [
    {"instruction": f"猫喜欢吃什么食物{i}", "output": "猫粮"} for i in range(5)
]


class CountingBackend:
    """会记录「同时在飞几条」的假模型后端

    分批这件事的唯一可见证据就是并发上界：不分批时飞行中的任务数等于清单长度。
    所以这里数的不是调用次数（两种写法调用次数一样），而是并发峰值。
    """

    def __init__(self, delay: float = 0.01):
        self.delay = delay
        self._lock = threading.Lock()
        self._inflight = 0
        self.max_inflight = 0
        self.calls = 0

    def generate(self, prompt: str) -> str:
        with self._lock:
            self._inflight += 1
            self.calls += 1
            self.max_inflight = max(self.max_inflight, self._inflight)
        try:
            time.sleep(self.delay)
            return "TRANSLATED"
        finally:
            with self._lock:
                self._inflight -= 1


def language_items_file(tmp_path, items=None):
    path = tmp_path / "l217_items.json"
    path.write_text(json.dumps(items or ITEMS, ensure_ascii=False), encoding="utf-8")
    return str(path)


def language_config(tmp_path, **overrides):
    """按 `节.键` 覆盖出厂配置，返回一份指向临时目录的 `AppConfig`"""
    config = load_config(str(AI_DIR / "config.yaml"))
    config.versioning.storage_dir = str(tmp_path / "versions")
    for dotted, value in overrides.items():
        section, _, field = dotted.partition(".")
        setattr(getattr(config, section), field, value)
    return config


class TestSupportedLangsIsConsumed:
    """`multilingual.supported_langs` 决定画像的清单外集合与翻译的准入"""

    def test_narrower_list_moves_more_labels_outside(self):
        wide = language_profile(ITEMS, supported_langs=["zh", "en"], default_target_lang="en")
        narrow = language_profile(ITEMS, supported_langs=["zh"], default_target_lang="en")
        assert "en" not in wide["outside_supported"]
        assert "en" in narrow["outside_supported"]
        assert wide["distribution"] == narrow["distribution"]

    def test_distribution_matches_the_detector(self):
        report = language_profile(ITEMS, supported_langs=["zh", "en"], default_target_lang="en")
        assert report["distribution"]["zh"] == 3
        assert report["distribution"]["en"] == 2
        assert report["total_items"] == 5

    def test_target_outside_the_list_is_rejected_before_any_model_call(self):
        """准入门只有一道，住在 SDK：本层不再重复校验（L217 撤闸）

        先在 `translate_dataset` 里写过同判据的第二道闸，注入「摘掉本层闸」时 20 例
        全绿——两存必有一盲，盲的是那道多余的。按仓库「判据住在 SDK/config 层」的惯例
        删掉本层那道，所以这条用例现在验的是 `supported_langs` 真的传到了 SDK：闸生效
        且一条模型调用都没发出。
        """
        backend = CountingBackend()
        with pytest.raises(DataValidationError):
            translate_dataset(
                ITEMS, backend=backend, supported_langs=["zh", "en"],
                default_target_lang="fr", batch_size=2,
            )
        assert backend.calls == 0


class TestDefaultTargetLangIsConsumed:
    """请求省略 `target_lang` 时，生效值来自配置而不是 SDK 内置默认"""

    def test_config_default_reaches_the_translator(self):
        """配置写 `"ja"`、SDK 内置默认是 `"en"`：只有配置真的生效，产物里才会出现 ja。

        取 `ja` 而不是 `en` 做输入是有意的——同名值做校验时，把 `or default_target_lang`
        写成 `or "en"` 的缺陷臂与正确实现读到同一个字符串，判据会空转。
        """
        backend = CountingBackend()
        result = translate_dataset(
            ITEMS, backend=backend, supported_langs=["zh", "en", "ja"],
            default_target_lang="ja", batch_size=3,
        )
        assert result["target_lang"] == "ja"
        assert all(item["language"] == "ja" for item in result["items"])

    def test_request_param_wins_over_config(self):
        backend = CountingBackend()
        result = translate_dataset(
            ITEMS, backend=backend, supported_langs=["zh", "en", "ja"],
            default_target_lang="ja", batch_size=3, target_lang="en",
        )
        assert result["target_lang"] == "en"
        assert all(item["language"] == "en" for item in result["items"])

    def test_keep_original_stays_with_the_sdk(self):
        backend = CountingBackend()
        result = translate_dataset(
            [ITEMS[0]], backend=backend, supported_langs=["zh", "en"],
            default_target_lang="en", batch_size=5, keep_original=False,
        )
        assert "instruction_original" not in result["items"][0]
        assert result["items"][0]["instruction"] == "TRANSLATED"


class TestBatchSizeIsConsumed:
    """`translate_batch_size` 决定切几批、每批多长，进而决定并发上界"""

    def test_chunk_sizes_follow_the_config(self, monkeypatch):
        import augmentor.language as language

        seen = []
        real = language.MultilingualSupport

        class Spy(real):
            def batch_translate(self, items, **kwargs):
                seen.append(len(items))
                return real.batch_translate(self, items, **kwargs)

        monkeypatch.setattr(language, "MultilingualSupport", Spy)
        backend = CountingBackend()
        result = translate_dataset(
            ITEMS * 5, backend=backend, supported_langs=["zh", "en"],
            default_target_lang="en", batch_size=7,
        )
        assert seen == [7, 7, 7, 4]
        assert result["batches"] == 4
        assert result["total"] == 25

    def test_batch_size_one_is_really_sequential(self):
        """批宽 1 ⇒ 并发峰值必须正好 1。不分批的实现拿不到这个读数。"""
        backend = CountingBackend()
        translate_dataset(
            ALL_ZH, backend=backend, supported_langs=["zh", "en"],
            default_target_lang="en", batch_size=1,
        )
        assert backend.max_inflight == 1
        assert backend.calls == 5

    def test_wider_batch_raises_the_concurrency_ceiling(self):
        """批宽 3（小于 SDK 线程池的 4）⇒ 并发峰值正好 3：这时批宽就是上界。

        取 3 而不是 5：`MultilingualSupport.max_workers` 出厂是 4，批宽超过它时并发
        由线程池封顶，读数就不再指认批宽了。
        """
        backend = CountingBackend()
        translate_dataset(
            ALL_ZH, backend=backend, supported_langs=["zh", "en"],
            default_target_lang="en", batch_size=3,
        )
        assert backend.max_inflight == 3

    def test_same_language_items_never_reach_the_model(self):
        """中英混装 5 条里只有 3 条要打模型——同语言短路是 SDK 行为，本轮如实钉住"""
        backend = CountingBackend()
        result = translate_dataset(
            ITEMS, backend=backend, supported_langs=["zh", "en"],
            default_target_lang="en", batch_size=5,
        )
        assert backend.calls == 3
        assert result["total"] == 5
        untranslated = [i for i in result["items"] if i["instruction"].startswith("How")]
        assert len(untranslated) == 2
        assert all(i["instruction"] == i["instruction_original"] for i in untranslated)

    def test_missing_backend_fails_loud_instead_of_returning_the_source_text(self):
        """`backend=None` 必须在分批之前出声

        SDK 的 `batch_translate` 把逐条异常吞成「回退原文」，所以缺失后端若在下游
        出声，端点会带着整份原文回 200——那正是本仓在收的静默冒充成功。
        """
        from augmentor.exceptions import ModelNotConfiguredError

        with pytest.raises(ModelNotConfiguredError):
            translate_dataset(
                ITEMS, backend=None, supported_langs=["zh", "en"],
                default_target_lang="en", batch_size=2,
            )

    def test_empty_dataset_reports_zero_batches(self):
        """整份等式是**键集权威**：回执四键的新增/消失都会在这里红，不只是值对得上"""
        backend = CountingBackend()
        result = translate_dataset(
            [], backend=backend, supported_langs=["zh", "en"],
            default_target_lang="en", batch_size=10,
        )
        assert result == {
            "total": 0, "batches": 0, "batch_size": 10, "target_lang": "en",
            "supported_langs": ["zh", "en"], "items": [],
            "failed": 0, "failed_indices": [], "errors": [], "errors_truncated": False,
        }
        assert backend.calls == 0


class TestEndpointProfile:

    def _client(self, monkeypatch, config, backend=None):
        import api.deps as deps
        from api.main import app
        from fastapi.testclient import TestClient

        stub = type("PipelineStub", (), {})()
        stub.config = config
        stub.model_backend = backend
        monkeypatch.setattr(deps, "_pipeline", stub)
        return TestClient(app)

    def test_echoes_effective_values(self, tmp_path, monkeypatch):
        config = language_config(tmp_path, **{"multilingual.supported_langs": ["zh"]})
        client = self._client(monkeypatch, config)
        response = client.post(
            "/api/dataset/language-profile",
            json={"input_file": language_items_file(tmp_path)},
        )
        assert response.status_code == 200, response.text
        body = response.json()
        assert body["supported_langs"] == ["zh"]
        assert body["default_target_lang"] == config.multilingual.default_target_lang
        assert "en" in body["outside_supported"]
        assert body["total_items"] == 5

    def test_profile_ignores_the_enabled_gate(self, tmp_path, monkeypatch):
        """只读画像不打模型、不花钱，所以闸关着也照样能用（这是设计，不是漏挂）"""
        config = language_config(tmp_path, **{"multilingual.enabled": False})
        client = self._client(monkeypatch, config)
        response = client.post(
            "/api/dataset/language-profile",
            json={"input_file": language_items_file(tmp_path)},
        )
        assert response.status_code == 200, response.text

    def test_text_key_is_honored(self, tmp_path, monkeypatch):
        config = language_config(tmp_path)
        client = self._client(monkeypatch, config)
        response = client.post(
            "/api/dataset/language-profile",
            json={"input_file": language_items_file(tmp_path), "text_key": "output"},
        )
        assert response.json()["distribution"] == {"zh": 3, "en": 2}

    def test_outside_whitelist_input_is_403(self, tmp_path, monkeypatch):
        config = language_config(tmp_path)
        client = self._client(monkeypatch, config)
        response = client.post(
            "/api/dataset/language-profile",
            json={"input_file": r"D:\Windows\not_a_dataset.json"},
        )
        assert response.status_code == 403


    def test_downstream_failure_is_mapped_not_leaked_as_a_stack(self, tmp_path, monkeypatch):
        """读盘之后的意外失败走 `to_http_error`，不是裸栈

        这条钉的是画像路由那句 `except Exception: raise to_http_error(e)`：`ValueError`
        族（含领域异常）映射 400 且原文照回，非 `ValueError` 映射 500 且只回固定文案。
        少了这一支，端点会把下游异常直接抛给 TestClient 变成客户端异常——调用方读到的是
        连接断开而不是一条能行动的错误。
        """
        import api.routes.language as language_route

        def boom(_path):
            raise DataValidationError("数据集形状不对")

        monkeypatch.setattr(language_route, "read_json_file", boom)
        config = language_config(tmp_path)
        client = self._client(monkeypatch, config)
        response = client.post(
            "/api/dataset/language-profile",
            json={"input_file": language_items_file(tmp_path)},
        )
        assert response.status_code == 400, response.text
        assert "数据集形状不对" in response.json()["detail"]


class TestEndpointTranslate:

    def _client(self, monkeypatch, config, backend=None):
        import api.deps as deps
        from api.main import app
        from fastapi.testclient import TestClient

        stub = type("PipelineStub", (), {})()
        stub.config = config
        stub.model_backend = backend
        monkeypatch.setattr(deps, "_pipeline", stub)
        return TestClient(app)

    def test_gate_refuses_before_touching_the_filesystem(self, tmp_path, monkeypatch):
        """闸在读盘之前：连文件名都越界，回的还是 400 而不是 403。

        这条读数才说明「关掉闸的部署不做无谓的白名单解析与整份读盘」是真的，
        而不是只在文档里承诺。
        """
        config = language_config(tmp_path, **{"multilingual.enabled": False})
        client = self._client(monkeypatch, config, backend=CountingBackend())
        response = client.post(
            "/api/dataset/translate",
            json={"input_file": r"D:\Windows\not_a_dataset.json"},
        )
        assert response.status_code == 400
        assert "multilingual.enabled" in response.json()["detail"]

    def test_enabled_config_translates_and_echoes_batching(self, tmp_path, monkeypatch):
        backend = CountingBackend()
        config = language_config(tmp_path, **{
            "multilingual.enabled": True,
            "multilingual.supported_langs": ["zh", "en", "ja"],
            "multilingual.default_target_lang": "ja",
            "multilingual.translate_batch_size": 2,
        })
        client = self._client(monkeypatch, config, backend=backend)
        response = client.post(
            "/api/dataset/translate",
            json={"input_file": language_items_file(tmp_path)},
        )
        assert response.status_code == 200, response.text
        body = response.json()
        assert body["target_lang"] == "ja"
        assert body["batch_size"] == 2
        assert body["batches"] == 3
        assert body["total"] == 5
        assert backend.max_inflight <= 2

    def test_missing_backend_is_400_not_500(self, tmp_path, monkeypatch):
        """`model_backend` 为 None 时 SDK 抛 `ModelNotConfiguredError`（ValueError 子类）"""
        config = language_config(tmp_path, **{"multilingual.enabled": True})
        client = self._client(monkeypatch, config, backend=None)
        response = client.post(
            "/api/dataset/translate",
            json={"input_file": language_items_file(tmp_path)},
        )
        assert response.status_code == 400

    def test_unsupported_target_from_request_is_400(self, tmp_path, monkeypatch):
        config = language_config(tmp_path, **{"multilingual.enabled": True})
        client = self._client(monkeypatch, config, backend=CountingBackend())
        response = client.post(
            "/api/dataset/translate",
            json={"input_file": language_items_file(tmp_path), "target_lang": "de"},
        )
        assert response.status_code == 400
        assert "de" in response.json()["detail"]
