# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""微型分支收尾 L62：models/base 会话池降级 + 抽象方法兜底、
ModelManager 双重检查/加载成功、ERNIE token 双重检查
"""

import sys
import threading
import time
from types import ModuleType

import pytest

from augmentor.config import ModelConfig
from augmentor.model_manager import ModelManager
from augmentor.models.base import ModelBackend


class _StubBackend(ModelBackend):
    """仅用于测试基类能力的最小实现"""

    def _call_api(self, prompt):
        return prompt


class TestSessionPoolDegradation:
    def test_import_error_falls_back_to_plain_session(self, monkeypatch):
        """HTTPAdapter 不可导入时降级为基础 Session，而不是让整个调用崩溃"""
        import requests

        # 一个缺少 HTTPAdapter 属性的假 requests.adapters 模块，
        # 使 `from requests.adapters import HTTPAdapter` 抛出 ImportError
        bare = ModuleType("requests.adapters")
        monkeypatch.setitem(sys.modules, "requests.adapters", bare)

        backend = _StubBackend(ModelConfig(type="baidu", api_key="k", secret_key="s"))
        session = backend._get_session()
        assert isinstance(session, requests.Session)

    def test_abstract_call_api_returns_none(self):
        """基类 _call_api 的兜底 pass：未覆盖的实例应返回 None 而非报错"""
        bare_class = type("Bare", (ModelBackend,), {})
        bare_class.__abstractmethods__ = frozenset()
        instance = bare_class.__new__(bare_class)
        assert instance._call_api("hi") is None


class _PreemptingLock:
    """进入锁体前抢占设置目标属性，模拟另一线程已完成初始化的竞态"""

    def __init__(self, target_attr, value, manager):
        self._target_attr = target_attr
        self._value = value
        self._manager = manager
        self._real = threading.Lock()

    def __enter__(self):
        setattr(self._manager, self._target_attr, self._value)
        self._real.acquire()
        return self

    def __exit__(self, *exc):
        self._real.release()
        return False


class TestModelManagerRaceBranches:
    @pytest.fixture(autouse=True)
    def _restore_sentence_model(self):
        mm = ModelManager()
        original_model = mm._sentence_model
        original_lock = mm._model_lock
        yield
        mm._sentence_model = original_model
        mm._model_lock = original_lock

    def test_double_check_returns_preempted_value(self):
        """外层读到 None、进锁后已被其他线程写好 → 直接复用（锁内双重检查）"""
        mm = ModelManager()
        mm._sentence_model = None
        mm._model_lock = _PreemptingLock("_sentence_model", "preempted", mm)
        assert mm.get_sentence_model() == "preempted"

    def test_loads_sentence_transformer_success_path(self, monkeypatch):
        class _FakeST:
            def __init__(self, name):
                self.name = name

        fake_st = ModuleType("sentence_transformers")
        fake_st.SentenceTransformer = _FakeST
        monkeypatch.setitem(sys.modules, "sentence_transformers", fake_st)

        mm = ModelManager()
        mm._sentence_model = None
        assert isinstance(mm.get_sentence_model(), _FakeST)
        mm._sentence_model = None


class TestErnieTokenDoubleCheck:
    def test_double_check_returns_valid_token(self):
        """锁外读到无 token、进锁后已被其他线程刷新 → 直接复用"""
        from augmentor.models.ernie import ERNIEBackend

        backend = ERNIEBackend(ModelConfig(
            type="baidu", api_key="k", secret_key="s"
        ))
        backend._access_token = None
        backend._token_expires_at = 0

        class _PreemptBoth(_PreemptingLock):
            def __enter__(self):
                super().__enter__()
                backend._token_expires_at = time.time() + 3600
                return self

        backend._token_lock = _PreemptBoth("_access_token", "pre-token", backend)
        # 进锁后 token 与过期时间均被抢占设置，锁内双重检查应命中
        assert backend._get_access_token() == "pre-token"
