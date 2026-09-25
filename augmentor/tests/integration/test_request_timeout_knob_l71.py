# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""A74（L71）：单次请求超时可配 —— 六处硬编码收敛成一档旋钮

改前实测（A74 行原文，L47 普查 / L49 就地校正）：六个 `session.post(..., timeout=…)`
**全部硬编码且互不相同** —— claude 60、ernie 换 token 10、ernie 推理 60、gemini 60、
ollama **120**、openai 60。症状不是崩，是「慢模型在用户改不了的地方被掐断」，而
`ReadTimeout` 经 `classify_error` 归为可重试 ⇒ 用户看到的是一串看不出原因的重试。

本文件按缺陷的三层逐个钉：

1. **旋钮真的被消费**（`TestKnobIsConsumed`）：加一个「读了字段但没人用」的旋钮就是
   A64 的同构复发 ⇒ 断言落在 `session.post` 收到的 `timeout` 关键字参数上，而不是
   只断言 `backend._request_timeout`。
2. **三档回落口径**（`TestThreeTierResolution`）：模型条目 > 全局档 > 类默认，
   且显式写的值不被 `or` 读成「没写」。
3. **两侧同判**（`TestConfigYamlRoundTrip`）：`validate-config` 与运行时对同一批越界
   值给同一个判决。下界 1 不是口味而是 `requests` 的硬约束（`test_zero_would_break_
   requests_at_call_time` 直接实测它抛 `ValueError`），判在 1 以下才让「校验绿灯、
   第一次调用当场炸」这条缝无法存在。

ernie 换 token 那一支**故意不跟旋钮走**（A74 明确要求单独拍）⇒
`TestErnieTokenStaysIndependent` 钉住：旋钮拨到 45，token 请求仍走 10，否则一次凭据
错误要干等 45 秒才出声。
"""

import inspect

import pytest

from augmentor.config import (AppConfig, AugmentationConfig,
                              DEFAULT_REQUEST_TIMEOUT, ModelConfig,
                              REQUEST_TIMEOUT_RANGE, load_config, save_config)
from augmentor.exceptions import DataValidationError
from augmentor.models.factory import create_model_backend

BACKEND_TYPES = ["baidu", "openai", "ollama", "claude", "gemini"]
BACKEND_MODULES = ["claude", "ernie", "gemini", "ollama", "openai_model"]


def _cfg(model_type, **extra):
    return ModelConfig(
        type=model_type,
        api_key="k",
        secret_key="s",
        base_url="http://localhost:11434",
        model="m",
        **extra,
    )


class _FakeTokenResponse:
    """ernie 换 token 那一步的最小可用响应（不吃旋钮，但必须成功才走得到推理）"""

    def raise_for_status(self):
        return None

    def json(self):
        return {"access_token": "t", "expires_in": 3600}


class _RecordingSession:
    def __init__(self):
        self.calls = []

    def post(self, url, **kwargs):
        self.calls.append((url, kwargs))
        if "oauth/2.0/token" in url:
            return _FakeTokenResponse()
        # 推理那一次记录完就抛：让五个后端各自的响应解析形状不进入本用例，
        # 要证的是「timeout 这一维真的传给了 requests」，不是「响应能被解析」。
        raise RuntimeError("stop after recording")

    @property
    def inference_timeout(self):
        return self.calls[-1][1]["timeout"]


def _run_call_api(backend, prompt="hello"):
    session = _RecordingSession()
    backend._get_session = lambda: session
    with pytest.raises(RuntimeError, match="stop after recording"):
        backend._call_api(prompt)
    return session


class TestKnobIsConsumed:
    @pytest.mark.parametrize("type_name", BACKEND_TYPES)
    def test_global_knob_reaches_every_backend_post(self, type_name):
        backend = create_model_backend(
            _cfg(type_name), default_request_timeout=45.0
        )
        assert _run_call_api(backend).inference_timeout == 45.0, type_name

    def test_unset_knob_resolves_to_the_single_authority_value(self):
        """没传旋钮时后端落在 `DEFAULT_REQUEST_TIMEOUT`，与配置默认值同一个常数

        改前是六处各写各的两个数（60 / 120），「两处抄同一个数」正是 A77 付过代价的
        错误，所以这里把「配置默认」与「后端类默认」钉成同一个对象。
        """
        assert AugmentationConfig().request_timeout == DEFAULT_REQUEST_TIMEOUT
        assert create_model_backend(_cfg("openai"))._request_timeout \
            == DEFAULT_REQUEST_TIMEOUT

    @pytest.mark.parametrize("name", BACKEND_MODULES)
    def test_call_sites_never_carry_a_bare_timeout_number(self, name):
        """反向棘轮：`timeout=` 右边只许是变量，不许再出现裸数字

        A74 的病根是「同一个维度在六处各写一个数」。这条用例让将来新增第七处硬编码
        当场变红，而不是悄悄多出第三个数字。
        """
        module = __import__(f"augmentor.models.{name}", fromlist=["*"])
        for line in inspect.getsource(module).splitlines():
            if "timeout=" not in line or line.lstrip().startswith("#"):
                continue
            right = line.split("timeout=", 1)[1].strip().rstrip(",)")
            assert not right.replace(".", "").isdigit(), (name, line)


class TestThreeTierResolution:
    """模型条目 > 全局档 > 类默认，且 0 不被读成「没写」"""

    def test_model_entry_overrides_the_global_knob(self):
        backend = create_model_backend(
            _cfg("openai", request_timeout=30.0), default_request_timeout=120.0
        )
        assert backend._request_timeout == 30.0
        assert _run_call_api(backend).inference_timeout == 30.0

    def test_absent_model_entry_falls_back_to_the_global_knob(self):
        backend = create_model_backend(
            _cfg("openai"), default_request_timeout=77.0
        )
        assert backend._request_timeout == 77.0

    def test_both_absent_falls_back_to_class_default(self):
        backend = create_model_backend(_cfg("ollama"))
        assert backend._request_timeout == DEFAULT_REQUEST_TIMEOUT

    @pytest.mark.parametrize("type_name", BACKEND_TYPES)
    def test_factory_forwards_the_knob_to_every_backend(self, type_name):
        """五个后端同一条链路：少一个子类接住参数就会在构造时炸，这里一次扫全"""
        backend = create_model_backend(
            _cfg(type_name), default_request_timeout=45.0
        )
        assert backend._request_timeout == 45.0, type_name

    def test_out_of_range_factory_value_rejected_before_backend_work(self):
        with pytest.raises(DataValidationError, match="default_request_timeout"):
            create_model_backend(_cfg("openai"), default_request_timeout=0.0)
        with pytest.raises(DataValidationError, match="default_request_timeout"):
            create_model_backend(
                _cfg("openai"),
                default_request_timeout=REQUEST_TIMEOUT_RANGE[1] + 1,
            )


class TestConfigYamlRoundTrip:
    def test_yaml_round_trip_for_both_knobs(self, tmp_path):
        """写进 YAML 的两档（全局 + 模型覆盖）都要读得回来

        走「手写真文件 → `load_config`」而不是「构造 → `save_config` → 再读」：
        `save_config` 只持久化 `AppConfig` 的字段集，模型条目要先有内容才写得出内容，
        这里要证的只是**读**这一侧的接线（写了没人读 = A76/A84 那一族缺陷）。
        """
        path = tmp_path / "config.yaml"
        path.write_text(
            "models:\n  default: ernie\n  ernie:\n    type: baidu\n"
            "    request_timeout: 42\n"
            "augmentation:\n  request_timeout: 133\n",
            encoding="utf-8",
        )
        config = load_config(str(path))
        assert config.augmentation.request_timeout == 133.0
        assert config.models["ernie"].request_timeout == 42.0

    def test_shipped_config_yaml_matches_the_code_authority(self):
        """出厂 `config.yaml` 写的那一档必须等于代码里的唯一默认值

        沿用 `test_api_security.py::test_shipped_config_yaml` 的口径：模板与代码各有
        一套数时，「有没有配置文件」就是两种行为。A74 收的正是「文档与代码各有一套数，
        谁也没对齐谁」这条缝 ⇒ 模板自己漂了等于没收。三处（YAML 字面值、dataclass
        默认、`DEFAULT_REQUEST_TIMEOUT`）必须同数。
        """
        import pathlib

        import yaml

        path = pathlib.Path(__file__).resolve().parent.parent.parent / "config.yaml"
        with open(path, encoding="utf-8") as handle:
            raw = yaml.safe_load(handle)

        shipped = raw["augmentation"]["request_timeout"]
        assert shipped == DEFAULT_REQUEST_TIMEOUT
        assert AugmentationConfig().request_timeout == DEFAULT_REQUEST_TIMEOUT
        assert load_config(str(path)).augmentation.request_timeout == shipped

    def test_save_config_round_trips_the_augmentation_knob(self, tmp_path):
        """存盘这一侧同样要接住：`save_config` 按 `AppConfig` 字段集落盘

        少接一步的症状是「改了配置、跑一次没事、再启动回落到默认」，与 A76 那族
        「写了没人读」是同一条缝的反向。
        """
        path = str(tmp_path / "config.yaml")
        config = AppConfig()
        config.augmentation.request_timeout = 77.0
        config.models["ernie"] = ModelConfig(type="baidu", request_timeout=42.0)
        save_config(config, path)
        reloaded = load_config(path)
        assert reloaded.augmentation.request_timeout == 77.0
        assert reloaded.models["ernie"].request_timeout == 42.0

    def test_written_but_empty_model_key_reads_as_none(self, tmp_path):
        """`request_timeout:` 写了键没给值 = 不覆盖全局档，与「没写这个键」同义

        模型节**允许**这个状态（与 `augmentation` 节相反）：那一节的 `_reject_null_fields`
        判的是「字段没有未提供这种状态」，而模型条目天生有「不覆盖」这一档。
        """
        path = tmp_path / "config.yaml"
        path.write_text(
            "models:\n  default: ernie\n  ernie:\n    type: baidu\n"
            "    request_timeout:\n",
            encoding="utf-8",
        )
        assert load_config(str(path)).models["ernie"].request_timeout is None

    def test_augmentation_section_still_rejects_null(self):
        with pytest.raises(DataValidationError, match="request_timeout"):
            AugmentationConfig(request_timeout=None)

    @pytest.mark.parametrize("value", [0, 0.0, -1.0, 600.1, 10**6, float("nan")])
    def test_runtime_and_validator_give_the_same_verdict(self, value):
        from augmentor.config_validator import validate_config

        with pytest.raises(DataValidationError):
            AugmentationConfig(request_timeout=value)
        result = validate_config(
            {"models": {"default": "ernie"},
             "augmentation": {"request_timeout": value}})
        assert not result.is_valid, value

    @pytest.mark.parametrize("value", [1.0, 120.0, 600.0])
    def test_bounds_are_accepted_by_both_sides(self, value):
        from augmentor.config_validator import validate_config

        assert AugmentationConfig(request_timeout=value).request_timeout == value
        result = validate_config(
            {"models": {"default": "ernie"},
             "augmentation": {"request_timeout": value}})
        assert result.is_valid, value

    def test_model_entry_is_judged_by_the_same_range(self):
        """覆盖档与全局档同判据：只有一侧有界就是 A77 那条缝的复发"""
        with pytest.raises(DataValidationError, match="request_timeout"):
            ModelConfig(type="openai", api_key="k", request_timeout=0)
        with pytest.raises(DataValidationError, match="request_timeout"):
            ModelConfig(type="openai", api_key="k",
                        request_timeout=REQUEST_TIMEOUT_RANGE[1] + 1)

    def test_zero_would_break_requests_at_call_time(self):
        """判掉下界的**物证**：`requests` 自己拒 0，且发生在第一次真实调用上

        `timeout=0` 在 SDK 层没有任何夹逼能兜住它，症状就是「校验绿灯的配置跑到第一
        条数据才炸」。这条用例是本仓少见的「直接调真库证一条判据为什么存在」的写法。
        """
        requests = pytest.importorskip("requests")
        with pytest.raises(ValueError, match="less than or equal to 0"):
            requests.get("http://10.255.255.1/", timeout=0)


class TestErnieTokenStaysIndependent:
    def test_token_request_keeps_its_own_timeout_while_inference_follows_knob(self):
        backend = create_model_backend(
            _cfg("baidu"), default_request_timeout=45.0
        )
        session = _run_call_api(backend)
        assert len(session.calls) == 2
        token_url, token_kwargs = session.calls[0]
        inference_url, inference_kwargs = session.calls[1]
        assert "oauth/2.0/token" in token_url
        assert token_kwargs["timeout"] == 10.0
        assert "chat" in inference_url
        assert inference_kwargs["timeout"] == 45.0

    def test_token_timeout_constant_is_not_derived_from_the_knob(self):
        """独立性要有判据：token 档住在本类常数上，源码里不引全局默认常数"""
        from augmentor.models.ernie import ERNIEBackend

        assert ERNIEBackend.TOKEN_REQUEST_TIMEOUT == 10.0
        assert "DEFAULT_REQUEST_TIMEOUT" not in inspect.getsource(ERNIEBackend)
