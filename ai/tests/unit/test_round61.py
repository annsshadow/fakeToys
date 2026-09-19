"""第61轮: exceptions异常类增强"""
import pytest
from augmentor.exceptions import (
    AugmentorError, ConfigError, DataError, ModelError, 
    PipelineError, QualityError, VersionError, ExportError,
    CheckpointError, DataFormatError, DataLoadError, DataValidationError,
    DedupError, ModelGenerateError, ModelInitError, ModelNotConfiguredError,
    UnsupportedFormatError
)


ALL_ERRORS = [
    ConfigError, DataError, ModelError, PipelineError, QualityError,
    VersionError, ExportError, CheckpointError, DataFormatError, DataLoadError,
    DataValidationError, DedupError, ModelGenerateError, ModelInitError,
    ModelNotConfiguredError, UnsupportedFormatError
]


class TestExceptions:
    def test_base_error(self):
        e = AugmentorError("test error")
        assert str(e) == "test error"
        assert isinstance(e, Exception)

    def test_all_inherit_from_base(self):
        for cls in ALL_ERRORS:
            assert issubclass(cls, AugmentorError), f"{cls.__name__} should inherit AugmentorError"

    def test_error_with_message(self):
        for cls in ALL_ERRORS:
            e = cls("msg")
            assert str(e) == "msg"

    def test_error_without_message(self):
        for cls in ALL_ERRORS:
            e = cls()
            assert isinstance(e, AugmentorError)
