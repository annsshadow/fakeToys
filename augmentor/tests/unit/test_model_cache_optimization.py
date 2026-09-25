# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""模型后端缓存与重试行为测试

原文件测试的是一个自定义的 `MockModelBackend`——它复刻了缓存逻辑，
却完全不覆盖生产代码 `ModelBackend.generate`。这里改为直接测试真实实现：
缓存命中、缓存有界（LRU 淘汰）、以及不可重试错误立即放弃。
"""

import os
import subprocess
import sys
from pathlib import Path

import pytest

from augmentor import AugmentorPipeline, load_config
from augmentor.config import ModelConfig
from augmentor.exceptions import ModelGenerateError
from augmentor.models.factory import create_model_backend
from augmentor.models.openai_model import OpenAIBackend


def _backend(api_key="k"):
    return OpenAIBackend(ModelConfig(type="openai", api_key=api_key, model="m"))


def _cfg(model="m", **kwargs):
    return ModelConfig(type="openai", api_key="k", model=model, **kwargs)


class _FakeResponse:
    def __init__(self, status_code, headers=None):
        self.status_code = status_code
        self.headers = headers or {}


class _FakeHTTPError(Exception):
    """模拟 requests HTTPError：状态码挂在 .response 上"""

    def __init__(self, status_code, headers=None):
        super().__init__(f"HTTP {status_code}")
        self.response = _FakeResponse(status_code, headers)


class TestGenerationCache:
    """生成结果缓存"""

    def test_cache_avoids_duplicate_api_calls(self):
        """相同提示第二次必须命中缓存，不再调用 API"""
        backend = _backend()
        calls = []

        def fake_api(prompt):
            calls.append(prompt)
            return f"结果: {prompt}"

        backend._call_api = fake_api

        first = backend.generate("相同提示")
        second = backend.generate("相同提示")

        assert first == second == "结果: 相同提示"
        assert len(calls) == 1, "第二次调用未命中缓存，重复请求了 API"

    def test_cache_hit_still_counts_as_request(self):
        """缓存命中仍应计入请求统计（便于观测真实调用量）"""
        backend = _backend()
        backend._call_api = lambda prompt: "ok"

        backend.generate("p")
        backend.generate("p")

        assert backend.request_count == 2

    def test_different_prompts_are_cached_separately(self):
        backend = _backend()
        backend._call_api = lambda prompt: f"结果: {prompt}"

        backend.generate("提示A")
        backend.generate("提示B")

        assert len(backend._generation_cache) == 2

    def test_cache_is_bounded_by_eviction(self, monkeypatch):
        """缓存必须有界：超出上限时淘汰最旧条目

        原实现是无上限 dict，批量增强上万条数据时每次响应都会永久驻留内存。
        """
        monkeypatch.setattr(OpenAIBackend, "_GENERATION_CACHE_MAX", 4)
        backend = _backend()
        backend._call_api = lambda prompt: f"结果: {prompt}"

        for i in range(10):
            backend.generate(f"提示{i}")

        assert len(backend._generation_cache) <= 4, "缓存未被限制在上限内"
        # 最早的条目应已被淘汰，最新的应仍在。
        # 用 backend._cache_key 而不是 hash(prompt) 取键：键的构造方式属于
        # 实现细节，测试不该复刻它（复刻出来的断言会与实现各自漂移，
        # 且 hash() 带进程随机盐，本来也不能当稳定键用）。
        assert backend._cache_key("提示0") not in backend._generation_cache
        assert backend._cache_key("提示9") in backend._generation_cache

    def test_evicted_entry_is_regenerated(self, monkeypatch):
        """被淘汰的提示再次请求时应重新调用 API（而非返回错误结果）"""
        monkeypatch.setattr(OpenAIBackend, "_GENERATION_CACHE_MAX", 2)
        backend = _backend()
        calls = []

        def fake_api(prompt):
            calls.append(prompt)
            return f"结果: {prompt}"

        backend._call_api = fake_api

        backend.generate("A")
        backend.generate("B")
        backend.generate("C")   # 淘汰 A
        result = backend.generate("A")

        assert result == "结果: A"
        assert calls.count("A") == 2, "被淘汰的条目未重新生成"


class TestGenerateRetryClassification:
    """generate 的错误分类行为"""

    def test_unauthorized_fails_fast(self):
        """401 不可重试：只尝试一次，不浪费时间与配额"""
        backend = _backend()
        calls = []

        def fake_api(prompt):
            calls.append(prompt)
            raise _FakeHTTPError(401)

        backend._call_api = fake_api

        with pytest.raises(ModelGenerateError):
            backend.generate("p", max_retries=5, retry_delay=0)

        assert len(calls) == 1, "401 被错误地重试了"
        assert backend.error_count == 1

    def test_rate_limit_is_retried(self):
        """429 可重试：应重试到成功"""
        backend = _backend()
        calls = []

        def fake_api(prompt):
            calls.append(prompt)
            if len(calls) < 3:
                raise _FakeHTTPError(429, {"Retry-After": "0"})
            return "ok"

        backend._call_api = fake_api

        assert backend.generate("p", max_retries=5, retry_delay=0) == "ok"
        assert len(calls) == 3
        assert backend.error_count == 2

    def test_max_retries_means_total_attempts(self):
        """max_retries 的语义是总尝试次数，不是额外重试次数"""
        backend = _backend()
        calls = []

        def fake_api(prompt):
            calls.append(prompt)
            raise RuntimeError("transient")

        backend._call_api = fake_api

        with pytest.raises(ModelGenerateError):
            backend.generate("p", max_retries=3, retry_delay=0)

        assert len(calls) == 3


class TestResponseCacheIsOptIn:
    """磁盘响应缓存：默认关闭，必须显式开启

    开启意味着把 prompt 与模型响应写进磁盘。这既是磁盘占用问题，
    也是数据落盘的隐私语义问题，所以默认必须是「不落盘」。
    """

    def test_disabled_by_default(self, tmp_path):
        backend = _backend()
        assert backend.response_cache is None, "磁盘缓存默认不应启用"

        backend._call_api = lambda prompt: "结果"
        backend.generate("p")

        assert not list(tmp_path.rglob("*.json")), "未启用磁盘缓存却产生了落盘文件"

    def test_explicit_dir_enables_cache(self, tmp_path):
        backend = OpenAIBackend(_cfg(), response_cache_dir=str(tmp_path / "resp"))
        assert backend.response_cache is not None

    def test_factory_defaults_to_disabled(self):
        backend = create_model_backend(_cfg())
        assert backend.response_cache is None

    def test_factory_forwards_cache_options(self, tmp_path):
        """factory 必须把三个缓存参数透传给具体后端

        否则通过 factory 构造的后端会静默丢掉调用方显式开启的缓存。
        """
        cache_dir = tmp_path / "resp"
        backend = create_model_backend(
            _cfg(),
            response_cache_dir=str(cache_dir),
            response_cache_ttl=123,
            response_cache_max_bytes=4096,
        )

        assert backend.response_cache is not None
        stats = backend.response_cache.stats
        assert stats["max_bytes"] == 4096
        assert stats["cache_dir"] == str(cache_dir)

    def test_unusable_cache_dir_degrades_instead_of_raising(self, tmp_path):
        """缓存目录不可用时降级为不缓存，而不是让后端无法构造

        缓存只是优化手段；因为它建不起来就让整个模型后端不可用是过度耦合。
        """
        blocker = tmp_path / "not_a_dir"
        blocker.write_text("我是文件，不是目录", encoding="utf-8")

        backend = OpenAIBackend(
            _cfg(), response_cache_dir=str(blocker / "resp")  # 父路径是文件 → mkdir 必失败
        )

        assert backend.response_cache is None
        backend._call_api = lambda prompt: "ok"
        assert backend.generate("p") == "ok"


class TestResponseCacheBehavior:
    """磁盘缓存的读写行为"""

    def test_hit_survives_new_instance(self, tmp_path):
        """新实例（模拟进程重启）必须命中磁盘缓存，不再请求 API"""
        cache_dir = tmp_path / "resp"
        calls = []

        def make_backend():
            backend = OpenAIBackend(_cfg(), response_cache_dir=str(cache_dir))

            def fake_api(prompt):
                calls.append(prompt)
                return f"结果: {prompt}"

            backend._call_api = fake_api
            return backend

        assert make_backend().generate("p") == "结果: p"
        assert len(calls) == 1

        # 全新实例：内存缓存为空，只能靠磁盘
        assert make_backend().generate("p") == "结果: p"
        assert len(calls) == 1, "新实例未命中磁盘缓存，重复请求了 API"

    def test_disk_hit_backfills_memory_cache(self, tmp_path):
        """磁盘命中后应回填内存缓存，后续同进程调用不再碰磁盘"""
        cache_dir = tmp_path / "resp"

        first = OpenAIBackend(_cfg(), response_cache_dir=str(cache_dir))
        first._call_api = lambda prompt: "ok"
        first.generate("p")

        second = OpenAIBackend(_cfg(), response_cache_dir=str(cache_dir))
        second._call_api = lambda prompt: pytest.fail("不应再调用 API")
        assert second.generate("p") == "ok"

        assert second._cache_key("p") in second._generation_cache, "磁盘命中未回填内存"

    def test_disk_cache_is_bounded(self, tmp_path):
        """磁盘缓存必须有容量上限——否则只是把内存泄漏换成了磁盘泄漏"""
        backend = OpenAIBackend(
            _cfg(),
            response_cache_dir=str(tmp_path / "resp"),
            response_cache_max_bytes=1024,
        )
        backend._call_api = lambda prompt: "x" * 200

        for i in range(30):
            backend.generate(f"提示{i}")

        stats = backend.response_cache.stats
        assert stats["entries"] < 30, "磁盘缓存未按容量上限淘汰"
        assert stats["total_size_bytes"] <= 1024, "磁盘缓存超出容量上限"

    def test_write_failure_does_not_fail_generation(self, tmp_path, monkeypatch):
        """落盘失败不应把已经成功的生成变成失败"""
        backend = OpenAIBackend(_cfg(), response_cache_dir=str(tmp_path / "resp"))
        backend._call_api = lambda prompt: "ok"

        def boom(key, value, ttl=None):
            raise OSError("磁盘写满")

        monkeypatch.setattr(backend._response_cache, "set", boom)

        assert backend.generate("p") == "ok"

    def test_hit_still_counts_as_request(self, tmp_path):
        """磁盘命中仍应计入请求统计（与内存命中的既有语义保持一致）"""
        cache_dir = tmp_path / "resp"

        first = OpenAIBackend(_cfg(), response_cache_dir=str(cache_dir))
        first._call_api = lambda prompt: "ok"
        first.generate("p")

        second = OpenAIBackend(_cfg(), response_cache_dir=str(cache_dir))
        second._call_api = lambda prompt: pytest.fail("不应再调用 API")
        second.generate("p")

        assert second.request_count == 1


class TestCacheKeyCorrectness:
    """缓存键的三个硬约束：覆盖全部输入、跨进程稳定、与配置区分"""

    def test_key_covers_sampling_params(self):
        """键必须覆盖所有影响输出的输入

        只用 prompt 做键的话，同一后端实例改了 model 或 temperature 之后
        会静默命中上一个配置留下的响应，且不会有任何报错。
        """
        base = OpenAIBackend(_cfg(model="m1"))
        same = OpenAIBackend(_cfg(model="m1"))
        other_model = OpenAIBackend(_cfg(model="m2"))
        other_temp = OpenAIBackend(_cfg(model="m1", temperature=0.1))

        assert base._cache_key("p") == same._cache_key("p")
        assert base._cache_key("p") != other_model._cache_key("p"), "键未区分 model"
        assert base._cache_key("p") != other_temp._cache_key("p"), "键未区分 temperature"

    def test_key_is_stable_across_processes(self):
        """键必须跨进程稳定

        内置 hash() 对 str 带 PYTHONHASHSEED 随机盐，拿它当磁盘键会让缓存
        跨进程永远不命中——表现为「缓存文件越来越多但命中率恒为 0」。
        这里用两个不同 PYTHONHASHSEED 的子进程各算一次键来验证。
        """
        repo_root = Path(__file__).resolve().parents[2]
        script = (
            "from augmentor.config import ModelConfig\n"
            "from augmentor.models.openai_model import OpenAIBackend\n"
            "backend = OpenAIBackend(ModelConfig(type='openai', api_key='k', model='m'))\n"
            "print(backend._cache_key('提示'))\n"
        )

        keys = []
        for seed in ("1", "999"):
            proc = subprocess.run(
                [sys.executable, "-c", script],
                capture_output=True,
                text=True,
                cwd=str(repo_root),
                env=dict(os.environ, PYTHONHASHSEED=seed),
            )
            assert proc.returncode == 0, proc.stderr
            keys.append(proc.stdout.strip())

        assert keys[0] == keys[1], (
            "缓存键在不同 PYTHONHASHSEED 下不一致——键里混进了内置 hash()，"
            "磁盘缓存将永远无法跨进程命中"
        )


class TestPipelineCacheWiring:
    """顶层入口 AugmentorPipeline 必须能把 opt-in 开关透传到工厂

    只把参数加在 factory 上而入口不透传，用户就永远打不开这个能力
    （表现为「参数写了但缓存目录始终是空的」）。
    """

    @staticmethod
    def _capture(monkeypatch):
        captured = {}

        def fake_factory(config, **kwargs):
            captured.update(kwargs)
            return None

        monkeypatch.setattr("augmentor.pipeline.create_model_backend", fake_factory)
        return captured

    @staticmethod
    def _config():
        return load_config(str(Path(__file__).resolve().parents[2] / "config.yaml"))

    def test_pipeline_forwards_cache_options(self, tmp_path, monkeypatch):
        captured = self._capture(monkeypatch)
        config = self._config()

        AugmentorPipeline(
            config,
            response_cache_dir=str(tmp_path / "resp"),
            response_cache_ttl=60,
            response_cache_max_bytes=2048,
        )

        assert captured == {
            "response_cache_dir": str(tmp_path / "resp"),
            "response_cache_ttl": 60,
            "response_cache_max_bytes": 2048,
            # 自 L45 起管道还要翻译重试档位给工厂（A64 接线）。精确字典是故意的：
            # 工厂契约每长一个键，这里就必须显式认领一次，不允许静默扩面。
            "default_attempts": config.augmentation.max_retries,
            "default_retry_delay": config.augmentation.retry_delay,
        }

    def test_pipeline_defaults_to_disabled(self, monkeypatch):
        captured = self._capture(monkeypatch)

        AugmentorPipeline(self._config())

        assert captured["response_cache_dir"] is None, "管道默认不应开启磁盘缓存"


