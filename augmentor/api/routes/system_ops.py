# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""系统运维 API 路由

补齐 CLI 里属于「运维 / 治理」而非「数据变换」的独有能力：依赖诊断、配置验证、
质量监控、自动化测试、数据迁移、流式处理、数据集依赖登记，以及备份的
创建 / 列表 / 恢复 / 删除。

这些能力此前只在命令行可用，Web 端因此无法回答「当前环境缺什么依赖」
「这份配置合不合法」「数据集之间是什么依赖关系」这类问题。
"""

import json
from typing import Any, Dict, List, Optional

from fastapi import APIRouter, Depends, HTTPException, Query
from pydantic import BaseModel

from augmentor.exceptions import BackupError

from ..deps import (
    config_file_path,
    default_backup_dir,
    default_registry_dir,
    get_pipeline,
    read_json_file,
    resolve_data_dir,
    resolve_data_path,
    run_in_thread,
    to_http_error,
    verify_api_key,
)
from ..schemas import SuccessResponse

router = APIRouter(tags=["system"])

# ============ 枚举白名单 ============
#
# 这两处的下游都是**静默降级/静默过滤**，必须在 API 边界显式拦下：
#
# * `run_dataset_tests()` 每次调用都新建 `DatasetTestRunner`，其 `_test_suites`
#   初始为空且没有注册入口 —— 传任何套件名都会回退到内置的 `"default"` 套件。
#   既然只有 default 真实存在，就只接受它。
# * `DatasetMigrator.migrate()` 用 `r.rule_id in rules` 过滤，未知规则 id 被
#   **静默丢弃**；全写错时 `rules_applied` 是空数组，等于「迁移跑了个寂寞」，
#   而调用方不看这个字段就发现不了。
#
# 状态码用 400 而非 422：与本项目既有约定一致（见 dataset_tools.py 的同名注释）。
AUTO_TEST_SUITES = ("default",)
MIGRATION_RULES = ("rename_instruction", "rename_output", "flatten_conversations")


# ============ 请求模型 ============

class ValidateConfigRequest(BaseModel):
    """配置验证请求

    `path` 留空时校验**服务自己**正在使用的那份配置（见 `deps.config_file_path`，
    可用 `AUGMENTOR_CONFIG_PATH` 指向别处）；显式传入时按数据白名单校验该路径。
    """
    path: Optional[str] = None


class MonitorRequest(BaseModel):
    """质量监控请求"""
    input_file: str


class AutoTestRequest(BaseModel):
    """自动化测试请求

    `suite` 留空时使用内置默认套件。
    """
    input_file: str
    suite: Optional[str] = None


class MigrateRequest(BaseModel):
    """数据迁移请求

    `rules` 留空时使用 `DatasetMigrator` 的全部内置规则。
    """
    input_file: str
    output_file: str
    rules: Optional[List[str]] = None


class StreamRequest(BaseModel):
    """流式处理请求

    `operation` 决定逐块处理做什么：quality（按质量分过滤）、dedup（去重）、
    clean（清洗后保留）、none（原样搬运）。与 CLI `stream` 的语义一致。
    """
    input_file: str
    output_file: str
    operation: str = "none"
    chunk_size: int = 1000
    output_format: str = "jsonl"


class RegisterDatasetRequest(BaseModel):
    """登记数据集请求"""
    name: str
    input_file: str
    description: str = ""
    tags: Optional[List[str]] = None


class CreateBackupRequest(BaseModel):
    """创建备份请求"""
    input_file: str
    name: Optional[str] = None


class RestoreBackupRequest(BaseModel):
    """恢复备份请求"""
    output_file: str


# ============ 响应模型 ============

class DependenciesResponse(BaseModel):
    """可选依赖诊断（与 `DiagnosticsReport.to_dict()` 的键一一对应）"""
    installed: Dict[str, bool]
    missing: List[str]
    degraded_features: List[str]
    all_required_present: bool
    available_count: int


class ConfigValidationIssue(BaseModel):
    """一条配置问题（与 `ValidationError.to_dict()` 的键一一对应）"""
    path: str
    message: str
    severity: str
    line: Optional[int]


class ConfigValidationResponse(BaseModel):
    """配置验证结果

    `errors` / `warnings` / `info` 是**三条并列的问题列表**（按严重级别分桶），
    `summary` 是三个桶的计数（`{"errors": n, "warnings": n, "info": n}`），
    与 `ConfigValidator` 的返回一致。
    """
    is_valid: bool
    summary: Dict[str, int]
    errors: List[ConfigValidationIssue]
    warnings: List[ConfigValidationIssue]
    info: List[ConfigValidationIssue]


class QualityAlertResponse(BaseModel):
    """一条质量告警（与 `QualityAlert.to_dict()` 的键一一对应）"""
    alert_id: str
    metric_name: str
    current_value: float
    threshold_value: float
    alert_type: str
    severity: str
    message: str
    timestamp: str


class QualitySnapshotResponse(BaseModel):
    """质量快照（与 `QualitySnapshot.to_dict()` 的键一一对应）"""
    snapshot_id: str
    timestamp: str
    metrics: Dict[str, float]
    alerts: List[QualityAlertResponse]


class TestResultResponse(BaseModel):
    """单条测试结果（与 `TestResult.to_dict()` 的键一一对应）"""
    test_id: str
    test_name: str
    passed: bool
    message: str
    execution_time_ms: float
    timestamp: str


class TestSuiteResponse(BaseModel):
    """测试套件结果

    `total_tests` / `passed_tests` / `failed_tests` / `pass_rate` 在
    `TestSuite` 上是**计算属性**而非字段，必须显式列出——漏一个就会被
    response_model 静默裁掉。
    """
    name: str
    description: str
    total_tests: int
    passed_tests: int
    failed_tests: int
    pass_rate: float
    results: List[TestResultResponse]


class MigrationResponse(BaseModel):
    """迁移结果（与 `MigrationResult.to_dict()` 的键一一对应）"""
    migration_id: str
    source_path: str
    target_path: str
    total_items: int
    migrated_items: int
    failed_items: int
    rules_applied: List[str]
    errors: List[Dict[str, Any]]
    timestamp: str


class StreamReportResponse(BaseModel):
    """流式处理报告（与 `StreamProcessor.process()` 的返回键一一对应）"""
    total_input: int
    total_output: int
    processed: int


class DatasetInfoResponse(BaseModel):
    """已登记的数据集（与 `DatasetInfo.to_dict()` 的键一一对应）"""
    dataset_id: str
    name: str
    description: str
    path: str
    item_count: int
    created_at: str
    updated_at: str
    tags: List[str]
    metadata: Dict[str, Any]


class DependencyDatasetListResponse(BaseModel):
    """已登记数据集列表"""
    datasets: List[DatasetInfoResponse]


class DependencyGraphResponse(BaseModel):
    """数据集依赖图 + 校验问题

    CLI 把 `graph` 与 `validate` 分成两个动作，但两者读的是同一份注册表、
    服务于同一个问题（「这批数据集之间的依赖是否成立」），因此合成一个响应：
    调用方一次请求就能同时拿到图与问题清单。
    """
    nodes: List[Any]
    edges: List[Any]
    issues: List[Dict[str, Any]]


class BackupInfoResponse(BaseModel):
    """备份信息（与 `BackupInfo.to_dict()` 的键一一对应）"""
    backup_id: str
    timestamp: str
    source_path: str
    backup_path: str
    item_count: int
    file_size: int
    checksum: str


class BackupListResponse(BaseModel):
    """备份列表"""
    backups: List[BackupInfoResponse]


class RestoreBackupResponse(BaseModel):
    """恢复结果（与 `DatasetBackup.restore()` 的返回键一一对应）"""
    backup_id: str
    output_path: str
    item_count: int


# ============ 依赖与配置 ============

@router.get(
    "/api/system/dependencies",
    response_model=DependenciesResponse,
    summary="可选依赖诊断",
)
async def system_dependencies():
    """报告可选依赖的安装情况与因缺失而降级的能力"""
    from augmentor.diagnostics import check_dependencies

    return check_dependencies().to_dict()


@router.post(
    "/api/system/validate-config",
    response_model=ConfigValidationResponse,
    summary="校验配置文件",
)
async def system_validate_config(request: ValidateConfigRequest):
    """校验一份 YAML 配置，返回按严重级别分桶的问题清单"""
    try:
        from augmentor.config_validator import validate_config_file

        path = (
            config_file_path() if request.path is None else resolve_data_path(request.path)
        )
        result = await run_in_thread(validate_config_file, str(path))
        return result.to_dict()
    except HTTPException:
        raise
    except Exception as e:
        raise to_http_error(e) from e


# ============ 质量监控与测试 ============

@router.post(
    "/api/system/monitor",
    response_model=QualitySnapshotResponse,
    summary="质量监控快照",
)
async def system_monitor(request: MonitorRequest):
    """对数据集做一次质量监控，返回指标快照与触发的告警"""
    try:
        from augmentor.quality_monitor import monitor_quality

        items = await read_json_file(resolve_data_path(request.input_file))
        snapshot = await run_in_thread(monitor_quality, items)
        return snapshot.to_dict()
    except HTTPException:
        raise
    except Exception as e:
        raise to_http_error(e) from e


@router.post(
    "/api/system/auto-test",
    response_model=TestSuiteResponse,
    summary="运行数据集自动化测试",
)
async def system_auto_test(request: AutoTestRequest):
    """对数据集运行内置测试套件，返回逐条结果与通过率"""
    if request.suite is not None and request.suite not in AUTO_TEST_SUITES:
        raise HTTPException(
            status_code=400,
            detail=(
                f"未知的测试套件: {request.suite}"
                f"（当前只支持 {' / '.join(AUTO_TEST_SUITES)}；"
                "自定义套件需要先由调用方注册，本端点暂无注册入口）"
            ),
        )
    try:
        from augmentor.auto_test import run_dataset_tests

        items = await read_json_file(resolve_data_path(request.input_file))
        suite = await run_in_thread(run_dataset_tests, items, request.suite)
        return suite.to_dict()
    except HTTPException:
        raise
    except Exception as e:
        raise to_http_error(e) from e


# ============ 迁移与流式处理 ============

@router.post(
    "/api/system/migrate",
    response_model=MigrationResponse,
    summary="迁移数据集结构",
    dependencies=[Depends(verify_api_key)],
)
async def system_migrate(request: MigrateRequest):
    """按规则把数据集迁移到新的字段结构并落盘"""
    unknown_rules = [r for r in (request.rules or []) if r not in MIGRATION_RULES]
    if unknown_rules:
        raise HTTPException(
            status_code=400,
            detail=(
                f"未知的迁移规则: {', '.join(unknown_rules)}"
                f"（可选 {' / '.join(MIGRATION_RULES)}）"
            ),
        )
    try:
        from augmentor.migration import migrate_file

        input_path = resolve_data_path(request.input_file)
        output_path = resolve_data_path(request.output_file, for_write=True)
        result = await run_in_thread(
            migrate_file, str(input_path), str(output_path), request.rules
        )
        return result.to_dict()
    except HTTPException:
        raise
    except Exception as e:
        raise to_http_error(e) from e


@router.post(
    "/api/system/stream",
    response_model=StreamReportResponse,
    summary="流式处理数据集",
    dependencies=[Depends(verify_api_key)],
)
async def system_stream(request: StreamRequest):
    """分块处理数据集（质量过滤 / 去重 / 清洗 / 原样），内存占用与文件大小无关

    与 `/api/augment/start` 的区别：这条路径**不调用模型**，只做本地规则处理，
    因此同步返回报告而不是丢进后台任务。
    """
    try:
        from augmentor.streaming import StreamAugmentor

        input_path = resolve_data_path(request.input_file)
        output_path = resolve_data_path(request.output_file, for_write=True)
        config = get_pipeline().config

        def run():
            if request.operation == "quality":
                from augmentor.quality import QualityScorer

                scorer = QualityScorer(threshold=config.quality.threshold)

                def process_quality(items):
                    scoring_items = [
                        {
                            "original": item.get("instruction", ""),
                            "generated": item.get("instruction", ""),
                            "output": item.get("output", ""),
                        }
                        for item in items
                    ]
                    scores = scorer.batch_score(scoring_items)
                    return [i for i, s in zip(items, scores) if s.passed]

                processor = process_quality
            elif request.operation == "dedup":
                from augmentor.dedup import Deduplicator

                deduplicator = Deduplicator(threshold=config.dedup.threshold)
                processor = lambda items: deduplicator.deduplicate_and_filter(items)  # noqa: E731
            elif request.operation == "clean":
                from augmentor.data import DataCleaner

                cleaner = DataCleaner()
                processor = lambda items: cleaner.clean(items).items  # noqa: E731
            elif request.operation == "none":
                processor = lambda items: items  # noqa: E731
            else:
                raise ValueError(
                    f"不支持的 operation: {request.operation}"
                    "（可选 quality / dedup / clean / none）"
                )

            augmentor = StreamAugmentor(
                str(input_path),
                str(output_path),
                processor,
                chunk_size=request.chunk_size,
                output_format=request.output_format,
            )
            return augmentor.augment()

        return await run_in_thread(run)
    except HTTPException:
        raise
    except Exception as e:
        raise to_http_error(e) from e


# ============ 数据集依赖登记 ============

REGISTRY_PATH_DESC = (
    "依赖注册表目录。缺省时使用白名单首个根目录下的 `.dependency_registry`"
    "（出厂默认即 `data/.dependency_registry`）；显式传入时该目录必须落在白名单内。"
)


def _dependency_manager(registry_path: Optional[str]):
    """构造依赖管理器（注册表目录受白名单约束，缺省时跟着白名单走）

    只在参数**完全缺席**时用缺省目录；显式传空串（`?registry_path=`）仍是非法入参，
    由 `resolve_data_dir()` 报 400，不能悄悄换成默认值。
    """
    from augmentor.dependency import DependencyManager

    path = default_registry_dir() if registry_path is None else resolve_data_dir(registry_path)
    return DependencyManager(str(path))


@router.get(
    "/api/system/dependency/datasets",
    response_model=DependencyDatasetListResponse,
    summary="已登记数据集列表",
)
async def dependency_datasets(
    registry_path: Optional[str] = Query(None, description=REGISTRY_PATH_DESC),
):
    """列出依赖注册表里登记过的数据集"""
    try:
        manager = _dependency_manager(registry_path)
        return {"datasets": [d.to_dict() for d in manager.list_datasets()]}
    except HTTPException:
        raise
    except Exception as e:
        raise to_http_error(e) from e


@router.post(
    "/api/system/dependency/datasets",
    response_model=DatasetInfoResponse,
    summary="登记数据集",
    dependencies=[Depends(verify_api_key)],
)
async def dependency_register(
    request: RegisterDatasetRequest,
    registry_path: Optional[str] = Query(None, description=REGISTRY_PATH_DESC),
):
    """把一个数据集登记进依赖注册表"""
    try:
        path = resolve_data_path(request.input_file)
        items = await read_json_file(path)

        def run():
            manager = _dependency_manager(registry_path)
            info = manager.register_dataset(request.name, str(path), len(items))
            if request.description:
                info.description = request.description
            if request.tags:
                info.tags = list(request.tags)
            return info.to_dict()

        return await run_in_thread(run)
    except HTTPException:
        raise
    except Exception as e:
        raise to_http_error(e) from e


@router.get(
    "/api/system/dependency/graph",
    response_model=DependencyGraphResponse,
    summary="数据集依赖图与校验问题",
)
async def dependency_graph(
    registry_path: Optional[str] = Query(None, description=REGISTRY_PATH_DESC),
):
    """返回依赖图的节点 / 边，以及依赖关系上的问题清单"""
    try:
        manager = _dependency_manager(registry_path)
        graph = manager.get_dependency_graph()
        issues = manager.validate_dependencies()
        return {
            "nodes": graph.get("nodes", []),
            "edges": graph.get("edges", []),
            "issues": issues,
        }
    except HTTPException:
        raise
    except Exception as e:
        raise to_http_error(e) from e


# ============ 备份 ============

BACKUP_DIR_DESC = (
    "备份目录。缺省时使用白名单首个根目录下的 `.backups`"
    "（出厂默认即 `data/.backups`）；显式传入时该目录必须落在白名单内。"
)


def _resolve_backup_dir(backup_dir: Optional[str]):
    """备份目录：缺省跟着白名单走，显式传空串仍是非法入参（400）"""
    return default_backup_dir() if backup_dir is None else resolve_data_dir(backup_dir)


@router.get(
    "/api/system/backups",
    response_model=BackupListResponse,
    summary="列出备份",
)
async def list_backups_endpoint(
    backup_dir: Optional[str] = Query(None, description=BACKUP_DIR_DESC),
):
    """列出备份目录下的全部备份"""
    try:
        from augmentor.backup import list_backups

        resolved = _resolve_backup_dir(backup_dir)
        return {"backups": list_backups(str(resolved))}
    except HTTPException:
        raise
    except Exception as e:
        raise to_http_error(e) from e


@router.post(
    "/api/system/backups",
    response_model=BackupInfoResponse,
    summary="创建备份",
    dependencies=[Depends(verify_api_key)],
)
async def create_backup_endpoint(
    request: CreateBackupRequest,
    backup_dir: Optional[str] = Query(None, description=BACKUP_DIR_DESC),
):
    """为数据集创建一个带校验和的备份"""
    try:
        from augmentor.backup import create_backup

        source = resolve_data_path(request.input_file)
        resolved_dir = _resolve_backup_dir(backup_dir)
        info = await run_in_thread(
            create_backup, str(source), str(resolved_dir), request.name
        )
        return info.to_dict()
    except HTTPException:
        raise
    except Exception as e:
        raise to_http_error(e) from e


@router.post(
    "/api/system/backups/{backup_id}/restore",
    response_model=RestoreBackupResponse,
    summary="恢复备份",
    dependencies=[Depends(verify_api_key)],
)
async def restore_backup_endpoint(
    backup_id: str,
    request: RestoreBackupRequest,
    backup_dir: Optional[str] = Query(None, description=BACKUP_DIR_DESC),
):
    """把备份恢复到指定路径

    备份 ID 不存在时返回 404，与 ``DELETE /api/system/backups/{backup_id}``
    保持一致 —— 同一个「备份不存在」在同一组路由里不该有两种状态码。
    ``BackupError`` 在库里只有一个抛出点（「备份不存在: xxx」），因此可以
    安全地整体映射成 404。
    """
    try:
        from augmentor.backup import restore_backup

        resolved_dir = _resolve_backup_dir(backup_dir)
        output = resolve_data_path(request.output_file, for_write=True)
        return await run_in_thread(
            restore_backup, backup_id, str(output), str(resolved_dir)
        )
    except HTTPException:
        raise
    except Exception as e:
        raise to_http_error(
            e, not_found="备份不存在", not_found_types=(BackupError,)
        ) from e


@router.delete(
    "/api/system/backups/{backup_id}",
    response_model=SuccessResponse,
    summary="删除备份",
    dependencies=[Depends(verify_api_key)],
)
async def delete_backup_endpoint(
    backup_id: str,
    backup_dir: Optional[str] = Query(None, description=BACKUP_DIR_DESC),
):
    """删除一个备份"""
    try:
        from augmentor.backup import delete_backup

        resolved_dir = _resolve_backup_dir(backup_dir)
        success = await run_in_thread(delete_backup, backup_id, str(resolved_dir))
        if not success:
            raise HTTPException(status_code=404, detail="备份不存在")
        return {"success": True}
    except HTTPException:
        raise
    except Exception as e:
        raise to_http_error(e) from e
