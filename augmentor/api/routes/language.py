# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""多语言 API 路由（L217）

这两条端点是 `augmentor/multilingual.py` 那一族能力第一次接到产品入口：在那之前
`MultilingualSupport` 在产品侧一个实例都没建过，`config.multilingual.*` 四只键
（`enabled` / `default_target_lang` / `supported_langs` / `translate_batch_size`）
全部只写不读。

**鉴权分档是判断，不是照抄**：`language-profile` 属**只读分析类**——纯正则语言检测，
不打模型、不落盘，与 `/api/dataset/stats` 同档，因此不挂 `verify_api_key`。
`translate` 反过来：它逐条调用 `model_backend.generate`，是一条按 token 计费的路径，
与 `/api/augment/start` 同一档 ⇒ 挂鉴权。判据取自「这条请求会不会替调用方花钱/改状态」，
而不是「HTTP 动词是什么」（本仓有 POST 只读的先例，见 `test_api_security` 的白名单）。

**`multilingual.enabled` 住在这里**：出厂 `false`，所以翻译路径默认关闭；`false` 时
先出声再读文件，这样关掉闸的部署不会被一条请求拖着去做白名单解析与整份读盘。
只读画像不受这只闸影响——检测不需要模型，把它一起关掉只会让配置更难核对。

**判据住在 SDK 层**：目标语言是否在清单内由 `MultilingualSupport.batch_translate` 顶部的
`DataValidationError` 判（→ 400），批尺寸下界由 `MultilingualConfig.__post_init__`
的 `require_count` 判，`augmentor/language.py` 和这里都不重复校验。
"""

from typing import Any, Dict, List, Optional

from fastapi import APIRouter, Depends, HTTPException
from pydantic import BaseModel

from ..deps import (
    get_pipeline,
    read_json_file,
    resolve_data_path,
    run_in_thread,
    to_http_error,
    verify_api_key,
)

router = APIRouter(tags=["dataset"])


class LanguageProfileRequest(BaseModel):
    """语言画像请求

    `text_key` 是参与检测的字段名，默认与增强产物同一口径（`instruction`）。
    """
    input_file: str
    text_key: str = "instruction"


class LanguageProfileResponse(BaseModel):
    """语言画像结果

    `supported_langs` / `default_target_lang` 是**本次生效值**（取自
    `config.multilingual.*`）；`outside_supported` 列出落在清单之外的检测标签，
    其中 `mixed` 是检测标签而不是语言码，出现它不构成错误。
    """
    total_items: int
    distribution: Dict[str, int]
    ratios: Dict[str, float]
    outside_supported: List[str]
    supported_langs: List[str]
    default_target_lang: str


class TranslateRequest(BaseModel):
    """批量翻译请求

    `target_lang` 省略时才落到 `multilingual.default_target_lang`（权威序：
    请求参数 > 配置键 > 内置默认）。取值是否在清单内由 SDK 层判。
    """
    input_file: str
    target_lang: Optional[str] = None
    text_key: str = "instruction"
    keep_original: bool = True


class TranslateResponse(BaseModel):
    """批量翻译结果

    `batches` 与 `batch_size` 一起才看得出分批确实发生：25 条 / 批宽 10 是 3 批，
    没分批的话飞行中的翻译任务数就是整份清单的长度。
    """
    total: int
    batches: int
    batch_size: int
    target_lang: str
    supported_langs: List[str]
    items: List[Dict[str, Any]]


@router.post(
    "/api/dataset/language-profile",
    response_model=LanguageProfileResponse,
    summary="语言分布画像",
)
async def language_profile_endpoint(request: LanguageProfileRequest):
    """按 `config.multilingual.*` 的生效值统计数据集语言分布（不打模型、不落盘）"""
    try:
        from augmentor.language import language_profile

        p = get_pipeline()
        multilingual = p.config.multilingual
        path = resolve_data_path(request.input_file)
        items = await read_json_file(path)
        return await run_in_thread(
            language_profile,
            items,
            supported_langs=multilingual.supported_langs,
            default_target_lang=multilingual.default_target_lang,
            text_key=request.text_key,
        )
    except HTTPException:
        # 白名单越界（403）等已由 deps 出声，不能被 to_http_error 降级成 500
        raise
    except Exception as e:
        raise to_http_error(e) from e


@router.post(
    "/api/dataset/translate",
    response_model=TranslateResponse,
    summary="批量翻译数据集",
)
async def translate_endpoint(request: TranslateRequest,
                             _auth: None = Depends(verify_api_key)):
    """按 `multilingual.translate_batch_size` 分批翻译；`enabled` 为 false 时直接拒"""
    try:
        from augmentor.language import translate_dataset

        p = get_pipeline()
        multilingual = p.config.multilingual
        if not multilingual.enabled:
            raise HTTPException(
                status_code=400,
                detail="multilingual.enabled 为 false，翻译路径未启用",
            )
        path = resolve_data_path(request.input_file)
        items = await read_json_file(path)
        return await run_in_thread(
            translate_dataset,
            items,
            backend=p.model_backend,
            supported_langs=multilingual.supported_langs,
            default_target_lang=multilingual.default_target_lang,
            batch_size=multilingual.translate_batch_size,
            target_lang=request.target_lang,
            text_key=request.text_key,
            keep_original=request.keep_original,
        )
    except HTTPException:
        raise
    except Exception as e:
        raise to_http_error(e) from e
