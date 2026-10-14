# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""自定义异常模块

提供项目统一的异常类型定义。

设计约定（3.0 起）
------------------
所有领域异常都挂在 :class:`AugmentorError` 之下，便于调用方一次性捕获本项目的
全部错误：``except AugmentorError``。

同时，每个具体异常**额外继承一个语义等价的内置异常**（``ValueError`` /
``RuntimeError`` / ``OSError``）。这不是权宜之计，而是 Python 标准库本身的既定
做法——``json.JSONDecodeError`` 继承 ``ValueError``、``UnicodeDecodeError`` 继承
``ValueError``、``socket.timeout`` 继承 ``OSError``。

这样做有两个收益：

1. **参数校验类**错误继续满足 ``isinstance(exc, ValueError)``，**环境/运行时
   类**错误继续满足 ``isinstance(exc, RuntimeError)``，因此既有的
   ``except ValueError`` 调用方与测试不会因为引入领域异常而静默失效；
2. 新代码可以按领域语义精确捕获，例如 ``except ModelGenerateError``。

因此下面每个异常的基类列表都是刻意为之，改动前请确认不会破坏上述两个契约。
"""


class AugmentorError(Exception):
    """增强器基础异常（本项目所有异常的根）

    捕获本项目全部错误：``except AugmentorError``。
    """
    pass


# ---------- 配置 ----------

class ConfigError(AugmentorError, ValueError):
    """配置相关异常（参数非法 / 配置项缺失）"""
    pass


# ---------- 模型 ----------

class ModelError(AugmentorError):
    """模型相关异常"""
    pass


class ModelNotConfiguredError(ModelError, ValueError):
    """模型未配置（缺少 api_key / base_url 等必要凭据）"""
    pass


class ModelInitError(ModelError, ValueError):
    """模型初始化失败"""
    pass


class ModelGenerateError(ModelError, RuntimeError):
    """模型生成失败（API 响应异常、令牌获取失败、响应被截断、重试耗尽）"""
    pass


class ModelResponseError(ModelError, ValueError):
    """模型响应无法解析（无法从响应中提取预期的 JSON 结构）

    与 :class:`ModelGenerateError` 的区别：后者表示「调用没成功」，
    本异常表示「调用成功了，但返回内容不符合预期结构」——属于解析问题。
    """
    pass


# ---------- 数据 ----------

class DataError(AugmentorError):
    """数据相关异常"""
    pass


class DataLoadError(DataError, ValueError):
    """数据加载失败"""
    pass


class DataFormatError(DataError, ValueError):
    """数据格式错误（无法解析、文件损坏、格式不受支持）"""
    pass


class DataValidationError(DataError, ValueError):
    """数据验证失败（字段缺失、长度不一致、取值越界）"""
    pass


# ---------- 质量与去重 ----------

class QualityError(AugmentorError, ValueError):
    """质量检查相关异常"""
    pass


class DedupError(AugmentorError, ValueError):
    """去重相关异常"""
    pass


# ---------- 导出与格式 ----------

class ExportError(AugmentorError, ValueError):
    """导出相关异常"""
    pass


class UnsupportedFormatError(ExportError):
    """不支持的导出/导入格式"""
    pass


# ---------- 流程与状态 ----------

class CheckpointError(AugmentorError, RuntimeError):
    """断点续传相关异常"""
    pass


class StreamError(AugmentorError, RuntimeError):
    """流式处理相关异常（读取器/写入器未打开等状态错误）"""
    pass


class VersionError(AugmentorError, ValueError):
    """版本管理相关异常"""
    pass


class PipelineError(AugmentorError, ValueError):
    """管道流程相关异常"""
    pass


# ---------- 运维 ----------

class BackupError(AugmentorError, ValueError):
    """备份不存在"""
    pass


class BackupIntegrityError(AugmentorError, ValueError):
    """备份文件存在但内容坏掉：校验和与索引里登记的那一格不匹配

    与 :class:`BackupError` 刻意**平级而不是父子**：路由把 ``BackupError`` 整体映射成
    404（「备份不存在」），判据正是「它在库里只有一个抛出点」。恢复一份损坏的备份会
    用坏内容覆盖用户的输出文件，语义是「资源状态冲突」而不是「资源不存在」，所以
    必须是另一个类型；若让它继承 ``BackupError``，那只端点上的 404 映射会顺手把
    409 吞成 404，而这两件事在客户面上完全相反。
    """
    pass


class VectorError(DataError, ValueError):
    """向量库相关异常（维度不匹配、ID 冲突/重复、持久化未配置）"""
    pass


class DatasetTextError(DataValidationError):
    """数据集条目里「被当作文本用的字段」装着非字符串值（A358 / L274）

    单独立一个类型而不是复用 :class:`DataValidationError`，是因为门口那三个收敛函数对
    「SDK 已经判出来的参数错误」给了两种归宿：`to_http_error`（``ValueError`` 支）与
    `raise_boundary_error`（按类型放行）落 **400**，而 `raise_internal_error` 把其余
    一切收敛成 **500** 固定文案。数据集是调用方自己给的文件，它的一条 `instruction`
    是整数时，服务端回答的是「服务器内部错误」——可这是参数缺陷，不是故障。

    把 `DataValidationError` 整体在 `raise_internal_error` 放行会顺带动到 L272 钉住的
    **A280** 那一族（非有限浮点旋钮、`dedup` / `benchmark` 四形全部 500，状态码是对外
    契约、待拍），所以这里按**具名窄口**放行：只有本类的实例换归宿，父类其他成员
    的归宿一字不变。与 :class:`BackupIntegrityError` 刻意与 `BackupError` 平级同族
    ——都是「映射的形状决定必须新立类型」。
    """
    pass
