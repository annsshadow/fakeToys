# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""统一异常体系契约测试（3.0 引入）

本文件锁定的不是某个 API 的行为，而是 `augmentor.exceptions` 的**设计契约**——
因为该契约一旦被破坏，调用方的 `except` 就会静默失效：

1. 所有领域异常都必须挂在 `AugmentorError` 之下，
   保证 `except AugmentorError` 能兜住本项目抛出的全部错误；
2. 每个具体领域异常必须**额外继承一个语义等价的内置异常**，
   保证既有的 `except ValueError` / `except RuntimeError` 调用方不会因为
   引入领域异常而漏接；
3. 生产代码必须真的抛领域异常，而不是裸内置异常（防止改造被回退）。
"""

import ast
import pathlib

import pytest

from augmentor.exceptions import (
    AugmentorError,
    BackupError,
    CheckpointError,
    ConfigError,
    DataError,
    DataFormatError,
    DataLoadError,
    DataValidationError,
    DedupError,
    ExportError,
    ModelError,
    ModelGenerateError,
    ModelInitError,
    ModelNotConfiguredError,
    ModelResponseError,
    PipelineError,
    QualityError,
    StreamError,
    UnsupportedFormatError,
    VectorError,
    VersionError,
)

# 领域异常 -> 期望额外继承的内置异常（None 表示只应继承 AugmentorError）
BUILTIN_COMPAT = {
    ConfigError: ValueError,
    ModelNotConfiguredError: ValueError,
    ModelInitError: ValueError,
    ModelGenerateError: RuntimeError,
    ModelResponseError: ValueError,
    DataLoadError: ValueError,
    DataFormatError: ValueError,
    DataValidationError: ValueError,
    QualityError: ValueError,
    DedupError: ValueError,
    ExportError: ValueError,
    UnsupportedFormatError: ValueError,
    CheckpointError: RuntimeError,
    StreamError: RuntimeError,
    VersionError: ValueError,
    PipelineError: ValueError,
    BackupError: ValueError,
    VectorError: ValueError,
    # 中间层抽象类不带内置基类，只作分类用
    ModelError: None,
    DataError: None,
}

# 只继承 AugmentorError、不继承任何内置异常的「纯抽象」异常
ABSTRACT_ONLY = {AugmentorError, ModelError, DataError}


@pytest.mark.parametrize("exc", list(BUILTIN_COMPAT), ids=lambda e: e.__name__)
def test_every_domain_exception_is_augmentor_error(exc):
    """契约 1：任何领域异常都能被 except AugmentorError 捕获"""
    assert issubclass(exc, AugmentorError)


@pytest.mark.parametrize("exc,builtin", list(BUILTIN_COMPAT.items()), ids=lambda v: getattr(v, "__name__", str(v)))
def test_builtin_compat_contract(exc, builtin):
    """契约 2：内置异常兼容性——既有的 except ValueError/RuntimeError 不会漏接"""
    if builtin is None:
        assert not issubclass(exc, (ValueError, RuntimeError, TypeError)), (
            f"{exc.__name__} 是纯抽象异常，不应继承内置异常"
        )
    else:
        assert issubclass(exc, builtin), f"{exc.__name__} 必须继承 {builtin.__name__}"


def test_augmentor_error_root_is_clean():
    """根异常不应绑定任何内置异常，否则层级语义会被污染"""
    assert not issubclass(AugmentorError, (ValueError, RuntimeError, TypeError))
    assert AugmentorError.__bases__ == (Exception,)


def test_unsupported_format_error_is_an_export_error():
    """格式不受支持属于导出错误的特例，层级必须成立"""
    assert issubclass(UnsupportedFormatError, ExportError)
    assert issubclass(VectorError, DataError)


def test_catching_root_covers_all_concrete_exceptions():
    """把每个具体异常抛出来，都能被 except AugmentorError 兜住"""
    for exc in BUILTIN_COMPAT:
        if exc in ABSTRACT_ONLY:
            continue
        with pytest.raises(AugmentorError):
            raise exc("boom")


def test_all_exceptions_exported_from_package():
    """所有领域异常必须能从包顶层导入，否则用户无法按领域捕获"""
    import augmentor

    for exc in BUILTIN_COMPAT:
        assert exc.__name__ in augmentor.__all__, f"{exc.__name__} 未导出到 __all__"
        assert getattr(augmentor, exc.__name__) is exc


# ---------- 契约 3：生产代码真的在抛领域异常 ----------

def test_no_bare_builtin_raises_left_in_package():
    """回归护栏：生产代码不得再出现 raise ValueError/RuntimeError/TypeError

    本项直接编码了 3.0 的改造意图——若有人新增裸内置异常 raise，
    异常层级就会重新出现「定义了但没人用」的退化。
    """
    package_root = pathlib.Path(__file__).resolve().parents[2] / "augmentor"
    banned = {"ValueError", "RuntimeError", "TypeError", "KeyError", "Exception", "AssertionError"}
    offenders = []

    for py_file in sorted(package_root.rglob("*.py")):
        tree = ast.parse(py_file.read_text(encoding="utf-8"), filename=str(py_file))
        for node in ast.walk(tree):
            if not isinstance(node, ast.Raise) or node.exc is None:
                continue
            call = node.exc
            if isinstance(call, ast.Call):
                func = call.func
            else:
                func = call
            if isinstance(func, ast.Name) and func.id in banned:
                rel = py_file.relative_to(package_root.parent)
                offenders.append(f"{rel}:{node.lineno} raise {func.id}")

    assert not offenders, "仍有裸内置异常 raise，请改用 augmentor.exceptions 中的领域异常:\n" + "\n".join(offenders)


# ---------- 行为验证：真实调用路径抛出的是领域异常 ----------

def test_deduplicator_raises_domain_error_and_still_a_value_error():
    """阈值非法既应是 DedupError（领域语义），也仍是 ValueError（向后兼容）"""
    from augmentor.dedup import Deduplicator

    with pytest.raises(DedupError) as info:
        Deduplicator(threshold=1.5)

    assert isinstance(info.value, ValueError)


def test_extract_json_array_raises_model_response_error():
    """响应解析失败属于 ModelResponseError，且仍可被 except ValueError 捕获"""
    from augmentor.models.base import extract_json_array

    with pytest.raises(ModelResponseError) as info:
        extract_json_array("这里没有任何数组")

    assert isinstance(info.value, ValueError)
    assert not isinstance(info.value, RuntimeError)


def test_create_model_backend_raises_config_error():
    """不支持的模型类型属于配置错误，且仍可被 except ValueError 捕获"""
    from augmentor.config import ModelConfig
    from augmentor.models.factory import create_model_backend

    with pytest.raises(ConfigError) as info:
        create_model_backend(ModelConfig(type="不存在的类型"))

    assert isinstance(info.value, ValueError)


def test_translate_without_backend_raises_not_configured():
    """未配置模型后端时必须抛 ModelNotConfiguredError，而不是静默返回原文"""
    from augmentor.multilingual import MultilingualSupport

    with pytest.raises(ModelNotConfiguredError):
        MultilingualSupport().translate("你好", "en")


# ---------- 响应解析器不得泄漏裸 JSONDecodeError ----------

# 四个后端各自实现了一份「从响应中提取 JSON 数组」，它们都声明
# Raises: ModelResponseError，因此即使括号内片段本身是坏 JSON，
# 也必须包成 ModelResponseError——否则 `except ModelResponseError` 会漏接。
_MALFORMED = "说明文字 [1, 2, ,] 结尾文字"


def _backend_extractors():
    """返回 (名称, 可调用对象) 列表；用 __new__ 绕过 __init__（方法不依赖实例状态）"""
    from augmentor.models.base import extract_json_array
    from augmentor.models.ernie import ERNIEBackend
    from augmentor.models.ollama import OllamaBackend
    from augmentor.models.openai_model import OpenAIBackend

    return [
        ("base.extract_json_array", extract_json_array),
        ("OllamaBackend", OllamaBackend.__new__(OllamaBackend).extract_json_from_response),
        ("OpenAIBackend", OpenAIBackend.__new__(OpenAIBackend).extract_json_from_response),
        ("ERNIEBackend", ERNIEBackend.__new__(ERNIEBackend).extract_json_from_response),
    ]


@pytest.mark.parametrize("name,extract", _backend_extractors(), ids=lambda v: v if isinstance(v, str) else "")
def test_malformed_bracket_payload_raises_model_response_error(name, extract):
    """括号内是坏 JSON 时，必须抛 ModelResponseError 而不是裸 JSONDecodeError"""
    with pytest.raises(ModelResponseError):
        extract(_MALFORMED)


@pytest.mark.parametrize("name,extract", _backend_extractors(), ids=lambda v: v if isinstance(v, str) else "")
def test_malformed_payload_is_still_a_value_error(name, extract):
    """向后兼容：ModelResponseError 仍可被 except ValueError 捕获"""
    with pytest.raises(ValueError):
        extract(_MALFORMED)
