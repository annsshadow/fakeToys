# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""配置与模型 API 路由"""

from typing import Any, Dict, List

from fastapi import APIRouter, Depends, HTTPException
from pydantic import BaseModel, ConfigDict

from augmentor.exceptions import DataValidationError

from ..deps import (
    config_file_path,
    get_pipeline,
    invalidate_config_caches,
    to_http_error,
    verify_api_key,
)
from ..schemas import MessageResponse

router = APIRouter(tags=["config"])


class ConfigResponse(BaseModel):
    """当前生效的配置

    只暴露各分区的**关键项**，不是 `AppConfig` 的完整镜像——密钥等敏感字段
    不会出现在响应里。各分区用 `Dict` 承载，因此新增配置项不需要改这个模型。
    """
    default_model: str
    augmentation: Dict[str, Any]
    quality: Dict[str, Any]
    dedup: Dict[str, Any]
    export: Dict[str, Any]
    vector: Dict[str, Any]
    rag: Dict[str, Any]
    multimodal: Dict[str, Any]


class ModelsResponse(BaseModel):
    """可用模型清单"""
    models: List[str]
    default: str


class ConfigUpdateRequest(BaseModel):
    """配置更新请求

    `extra="allow"` 不是为了「什么都能写」，是为了**报得出来**（A151，与 A86 同构的
    另一半）。A86 修的是节内未知子键被 `hasattr` 静默丢弃，而**节级**一直漏网：
    pydantic 默认 `extra="ignore"` 先把不认识的顶层键吃掉，路由再只遍历那七节，于是
    `{"web": {"max_upload_bytes": 0}}` 实测回 **200 +「配置已保存」+ `ignored_keys: []`**，
    而内存对象与磁盘文件一字未改 —— 比节内拼错更安静，因为连那份干净清单都是空的。
    写入清单本身不变（改 `web` 节牵动端口/白名单/CORS 的生效时机，属于「要不要给这节
    开重启语义」的独立决定），本轮只把静默改成出声。
    """
    model_config = ConfigDict(extra="allow")

    default_model: str | None = None
    augmentation: dict | None = None
    quality: dict | None = None
    dedup: dict | None = None
    export: dict | None = None
    vector: dict | None = None
    rag: dict | None = None
    multimodal: dict | None = None


class ConfigUpdateResponse(MessageResponse):
    """更新配置的结果

    继承 `MessageResponse` 的 `success`/`message` 两键（语义不变），只多一
    `ignored_keys`：请求里**没有写进配置**的键清单（A86 写路径出声，A151 扩到节级）。
    形状两种：`section.key`（节内未知子键）与 `section`（整节不在写入清单里，
    或节名拼错），程序侧按「有没有点」就能分开消费。

    `message` 有两种主干（A152①）：写过东西是「配置已保存…」，一个键都没写成是
    「本次请求未写入任何配置项，config.yaml 保持不变」。**`success` 两边都是
    `True`** —— 它表示「请求被正常处理」，落没落盘看 `ignored_keys` 与文案主干，
    本端点不借 `success` 表达「你的键我一个字都没写进去」。
    """
    ignored_keys: List[str] = []


# `POST /api/config` 的写入清单。配置里 22 个节名（`ConfigValidator.consumed_section_keys()`
# 的 20 节 + `app` / `models` 两个元节）里只有这 7 节能写；名单之外的 15 节
# （`web` / `logging` / `versioning` …）改了要重启、或牵动生效时机（端口、白名单、
# CORS 装配），本端点不碰；A151 起它们会出现在 `ignored_keys` 里而不是静默消失。
# 这份名单与 `ConfigUpdateRequest` 的七个 dict 字段一一对应，由
# `tests/unit/test_upload_ceiling_l87.py::test_writable_list_matches_the_request_schema`
# 钉住。
WRITABLE_SECTIONS = ["augmentation", "quality", "dedup", "export",
                     "vector", "rag", "multimodal"]


@router.get("/api/config", response_model=ConfigResponse, summary="获取配置")
async def get_config():
    """获取配置"""
    try:
        p = get_pipeline()
        return {
            "default_model": p.config.default_model,
            "augmentation": {
                "variants_per_seed": p.config.augmentation.variants_per_seed,
                "num_threads": p.config.augmentation.num_threads
            },
            "quality": {
                "enabled": p.config.quality.enabled,
                "threshold": p.config.quality.threshold
            },
            "dedup": {
                "enabled": p.config.dedup.enabled,
                "threshold": p.config.dedup.threshold
            },
            "export": {
                "default_format": p.config.export.default_format,
                "formats": p.config.export.formats
            },
            "vector": {
                "enabled": p.config.vector.enabled,
                "backend": p.config.vector.backend
            },
            "rag": {
                "enabled": p.config.rag.enabled,
                "default_format": p.config.rag.default_format
            },
            "multimodal": {
                "enabled": p.config.multimodal.enabled
            }
        }
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


@router.post(
    "/api/config",
    response_model=ConfigUpdateResponse,
    summary="更新配置",
)
async def update_config(
    request: ConfigUpdateRequest, _auth: None = Depends(verify_api_key)
):
    """更新配置；只有真的写进了配置对象才持久化到 config.yaml（A152①）"""
    try:
        import dataclasses

        from augmentor.config import apply_section_update, save_config
        from augmentor.config_validator import ConfigValidator

        p = get_pipeline()
        updates = request.model_dump(exclude_none=True)

        # `applied` = 这一次请求是否真的往配置对象里写了东西（A152①）。它决定
        # 要不要落盘，所以口径必须是「写没写」而不是「有没有报错」：`default_model`
        # 是直接赋值（`apply_section_update` 管不到它），节内键则要扣掉被 `hasattr`
        # 丢回去的那些 —— 只提交未知键的请求（`{"augmentation": {"variants_per_item":
        # 3}}`，L88 自己的写面用例就是这个形状）算「没写」。
        applied = False
        if "default_model" in updates:
            p.config.default_model = updates["default_model"]
            applied = True

        # `hasattr` 丢弃未知子键（不崩溃）是既有契约；本轮补的是「丢弃要出声」：
        # 拼错/多余的键被静默跳过却仍回 success，用户看不出「写了没生效」，与读路径
        # 的 A76 同族。`ignored_keys` 是给程序消费的干净清单，`message` 附「是否想写
        # X」给人看；判定、状态码与既有 `success`/`message` 的语义都不变，只加一键。
        #
        # 写入一律经 `apply_section_update`（L73 / A117）而不是裸 `setattr`：判据住在
        # 各节的 `__post_init__` 里，裸 `setattr` 不触发它 ⇒ 坏值能挂到运行中的对象上、
        # 还能被下面的 `save_config` 落盘（当次 200、重启即抛）。该函数要么全落、
        # 要么整批回滚，所以本端点不再存在「内存已改、文件未写」的中间态。
        ignored: List[str] = []
        notes: List[str] = []
        for section in WRITABLE_SECTIONS:
            if section in updates:
                section_config = getattr(p.config, section)
                known = [f.name for f in dataclasses.fields(section_config)]
                dropped = apply_section_update(section_config, updates[section])
                for key in dropped:
                    ignored.append(f"{section}.{key}")
                    notes.append(
                        f"{section}.{key}"
                        + ConfigValidator._suggest(key, sorted(known))
                    )
                # 「一个键都没落」的节不算写入：`apply_section_update` 只把 `hasattr`
                # 判败的键退回来，所以「提交数 > 退回数」就是「至少 setattr 过一次」。
                if len(updates[section]) > len(dropped):
                    applied = True

        # A151：节级漏网。`extra="allow"` 把不认识的顶层节留在 `model_extra` 里，
        # 于是这里能分清两种「没写进去」：配置里**有**这一节但本端点不写（`web` /
        # `logging` 那 15 节，改了要重启或牵动生效时机），以及节名压根拼错。
        # 从前前者被 pydantic 直接吃掉、连 `ignored_keys` 都是空的，实测
        # `{"web": {"max_upload_bytes": 0}}` 回 200「配置已保存」而 live 与磁盘未动。
        known_sections = (set(ConfigValidator.consumed_section_keys())
                          | ConfigValidator.META_TOP_SECTIONS)
        for key in sorted(request.model_extra or {}):
            ignored.append(key)
            if key in known_sections:
                notes.append(f"{key}（本端点不修改该节，请编辑 config.yaml 后重启服务）")
            else:
                notes.append(
                    key + ConfigValidator._suggest(
                        key, sorted(known_sections | set(ConfigUpdateRequest.model_fields)))
                )

        # A152①：只有「真的写了东西」才落盘。`save_config` 是「读现有文件 → 合并 →
        # 整份重写」，而 `yaml.safe_dump` 不回写注释 ⇒ 一次什么都没写成的请求从前也会
        # 把磁盘上那份 `config.yaml` 的注释与排版一起抹掉。那 202 行注释里住的不是文档，
        # 是判据本身：自动保存档「10 倍静默写放大」的实测、`request_timeout` 取 120 而
        # 不是 60 的两侧代价、`max_upload_bytes` 默认 256 MiB 的定标依据。⇒ 空写入连
        # 文件都不碰：字节、mtime、内容缓存三者一律不动，`message` 也不再谎称已保存。
        if applied:
            save_config(p.config, str(config_file_path()))
            # 写盘成功即作废内容类缓存：`save_config` 是整份重序列化，`web` 那两键
            # （数据白名单、上传上限）也一起被重写了一遍，而读它们的缓存按
            # `(路径, mtime_ns)` 建键 ⇒ 「改过配置文件，下一次读必然看见新值」这句话
            # 本来是由操作系统时钟是否跨过一格决定的（本机 tick 实测 0.5 ms）。缓存的
            # 写入方就在本进程，所以这一格只有显式失效能封死；跨进程与「保住 mtime」
            # 的写方管不到，那两格记在 A156 / A158。
            invalidate_config_caches()
            message = "配置已保存，部分配置需要重启服务生效"
        else:
            message = "本次请求未写入任何配置项，config.yaml 保持不变"
        if ignored:
            # 措辞从「未知配置项」改成「未写入的配置项」：A151 之后这份清单里
            # 也可能是**存在但本端点不写**的节（`web`），叫它「未知」是假话。
            message += "（已忽略未写入的配置项：" + "、".join(notes) + "）"
        return {"success": True, "message": message, "ignored_keys": ignored}
    except DataValidationError as e:
        # 越界的新值要的是 400（「你给的这个值不合法」），不是 500（「服务端坏了」）。
        # `DataValidationError` 是 `ValueError` 子类，`to_http_error` 那一档正好译成
        # 400 且回传原文案；必须排在下面的 `Exception` 分支之前，否则一律落 500。
        raise to_http_error(e) from e
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


@router.get("/api/models", response_model=ModelsResponse, summary="列出可用模型")
async def list_models():
    """列出可用模型"""
    try:
        p = get_pipeline()
        models = list(p.config.models.keys())
        return {"models": models, "default": p.config.default_model}
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))
