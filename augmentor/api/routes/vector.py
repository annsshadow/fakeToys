# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""语义索引与向量库状态 API 路由（L216）

这两条端点是 `augmentor/vector/` 那一族后端第一次接到产品入口：在那之前
`create_vector_db` 的调用者只有测试，`config.vector.*` 五只键（`enabled` / `backend` /
`dimension` / `storage_dir` / `collection`）在产品侧一字未读。

**分组**：两条端点都属**只读分析类**，不挂 `verify_api_key`，与其它只读端点一样零配置可用。
「只读」是按后端逐个核过的，不是想当然：`semantic-search` 走内存索引（`storage_dir=None`），
`vector-status` 走 `load()` 面，而 `FAISSDB` 的构造与 `load()` 都不写盘（只有 `persist()` 写）。
`ChromaDB` 相反——它在构造阶段就建目录并注册集合，所以 `augmentor/semantic.py` 的
`EXPLICIT_PERSISTENCE_BACKENDS` 在**构造之前**把它拒掉（`VectorError` → 400）。这一句先于构造，
才是「GET 不写盘」这个承诺成立的原因；否则配置写 `backend: chromadb` 的 GET 会凭空造出目录。
写盘的建库动作住在 `AugmentorPipeline.build_vector_index`，由 `vector.enabled` 决定建不建
（出厂 `false` ⇒ 不建，与本轮之前行为一致），且它只可能由已挂鉴权的增强端点触发。

**判据住在 SDK 层**：`top_k` 的下界由 `VectorDB.search` 那一处 `require_count` 拦下
（`ValueError` → 400），这里不重复校验；`storage_dir` 的「未配置就无法持久化」由
`persist()` 抛 `VectorError`（同样是 `ValueError` 子类 → 400）。
"""

from typing import Any, Dict, List

from fastapi import APIRouter, HTTPException
from pydantic import BaseModel

from ..deps import get_pipeline, read_json_file, resolve_data_path, run_in_thread, to_http_error

router = APIRouter(tags=["dataset"])


class SemanticSearchRequest(BaseModel):
    """语义检索请求

    `text_key` 是参与嵌入的字段名，默认与增强产物同一口径（`instruction`）。
    `top_k` 的非空正整数判据住在 `VectorDB.search`，越界回 400。
    """
    input_file: str
    query: str
    top_k: int = 5
    text_key: str = "instruction"


class SemanticSearchResponse(BaseModel):
    """语义检索结果

    `backend` / `dimension` / `collection` 是**本次生效值**（取自 `config.vector.*`），
    用来让调用方核对「配了什么、算了什么」——`dimension` 一改排名就变，
    不回显生效值就等于让调用方猜。
    """
    query: str
    total: int
    backend: str
    dimension: int
    collection: str
    results: List[Dict[str, Any]]


class VectorStatusResponse(BaseModel):
    """向量库状态

    `enabled` 是「增强结束时自动建库」那只闸的生效值；`exists` 是 `storage_dir` 里
    究竟有没有一份可读回的索引。两者分开设：`enabled=false` 且 `exists=true` 是
    「上一轮开着建过、这一轮关了闸」，`enabled=true` 且 `exists=false` 是「配了还没跑过」。

    `backend` 与另两面（`build_vector_index` 的回执、`semantic-search` 的回显）同义：
    配置选择。`faiss-cpu` 未装时引擎会退化为 numpy 精确检索，那属依赖层事实，
    看 `/api/system/dependencies`。
    """
    enabled: bool
    exists: bool
    count: int
    backend: str
    dimension: int
    collection: str
    storage_dir: str


@router.post(
    "/api/dataset/semantic-search",
    response_model=SemanticSearchResponse,
    summary="语义相似检索",
)
async def semantic_search(request: SemanticSearchRequest):
    """按 `config.vector.*` 的生效值做语义相似检索（内存建库，不落盘）"""
    try:
        from augmentor.semantic import search_similar

        p = get_pipeline()
        vector = p.config.vector
        path = resolve_data_path(request.input_file)
        items = await read_json_file(path)
        results = await run_in_thread(
            search_similar,
            request.query,
            items,
            backend=vector.backend,
            dimension=vector.dimension,
            collection=vector.collection,
            top_k=request.top_k,
            text_key=request.text_key,
        )
        return {
            "query": request.query,
            "total": len(results),
            "backend": vector.backend,
            "dimension": vector.dimension,
            "collection": vector.collection,
            "results": results,
        }
    except HTTPException:
        # 白名单越界（403）等已由 deps 出声，不能被 to_http_error 降级成 500
        raise
    except Exception as e:
        raise to_http_error(e) from e


@router.get(
    "/api/dataset/vector-status",
    response_model=VectorStatusResponse,
    summary="向量库状态",
)
async def vector_status():
    """回显 `config.vector.*` 的生效值，并读回持久化索引是否存在"""
    try:
        from augmentor.semantic import index_status

        p = get_pipeline()
        vector = p.config.vector
        status = await run_in_thread(
            index_status,
            backend=vector.backend,
            dimension=vector.dimension,
            collection=vector.collection,
            storage_dir=vector.storage_dir,
        )
        return {
            "enabled": vector.enabled,
            "exists": status["exists"],
            "count": status["count"],
            "backend": status["backend"],
            "dimension": status["dimension"],
            "collection": status["collection"],
            "storage_dir": status["storage_dir"],
        }
    except HTTPException:
        # 白名单越界（403）等已由 deps 出声，不能被 to_http_error 降级成 500
        raise
    except Exception as e:
        raise to_http_error(e) from e
