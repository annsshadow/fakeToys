"""自定义异常模块

提供项目统一的异常类型定义。
"""


class AugmentorError(Exception):
    """增强器基础异常"""
    pass


class ConfigError(AugmentorError):
    """配置相关异常"""
    pass


class ModelError(AugmentorError):
    """模型相关异常"""
    pass


class ModelNotConfiguredError(ModelError):
    """模型未配置"""
    pass


class ModelInitError(ModelError):
    """模型初始化失败"""
    pass


class ModelGenerateError(ModelError):
    """模型生成失败"""
    pass


class DataError(AugmentorError):
    """数据相关异常"""
    pass


class DataLoadError(DataError):
    """数据加载失败"""
    pass


class DataFormatError(DataError):
    """数据格式错误"""
    pass


class DataValidationError(DataError):
    """数据验证失败"""
    pass


class QualityError(AugmentorError):
    """质量检查相关异常"""
    pass


class DedupError(AugmentorError):
    """去重相关异常"""
    pass


class ExportError(AugmentorError):
    """导出相关异常"""
    pass


class UnsupportedFormatError(ExportError):
    """不支持的导出格式"""
    pass


class CheckpointError(AugmentorError):
    """断点续传相关异常"""
    pass


class VersionError(AugmentorError):
    """版本管理相关异常"""
    pass


class PipelineError(AugmentorError):
    """管道流程相关异常"""
    pass
