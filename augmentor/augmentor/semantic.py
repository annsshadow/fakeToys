# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""语义相似检索：把 `augmentor/vector/` 那一族后端接进产品入口（L216）。

**为什么需要这一层**：`augmentor/vector/`（`FAISSDB` / `ChromaDB` / `create_vector_db`）
从前只有测试在调，产品侧读者为 0，于是 `vector` 节五只键也没有读者。本模块是唯一的生产者，
`config.vector.*` 的取值在这里第一次真的决定产物：

- `backend` ⇒ 选哪个后端类（`faiss` 与 `chromadb` 的元数据规则与 `backend` 属性都不同）；
  落盘与读回两面对**没有** `persist`/`load` 的后端在构造前拒（见 `EXPLICIT_PERSISTENCE_BACKENDS`）；
- `dimension` ⇒ 哈希嵌入的输出宽度，直接改变相似度分值与排名；
- `collection` ⇒ 持久化文件名与向量 ID 的前缀；
- `storage_dir` ⇒ 持久化落点（未配置时 `persist()` 抛 `VectorError`）；
- `enabled` ⇒ 增强结束时是否自动建库并落盘（出厂 `false` ⇒ 不建，等于改前行为）。

**为什么用确定性哈希嵌入而不是真模型**：仓库里可选依赖 `sentence-transformers`
在 CI 与本机都没装，接上模型臂意味着写一条**任何环境都跑不到**的产品路径——那正是本仓
一直在收的「只写不读」面。字符二元组哈希嵌入零依赖、跨进程确定（用 `zlib.crc32`，不用
`hash()`，后者受 `PYTHONHASHSEED` 随机化），所以本模块的每条分支都有现量测试。
真模型嵌入的维度冲突策略已于 **L340/A268** 定为「拒配置」：模型宽度与配置不等即抛 `VectorError`，不忽略/不截断；判据 `assert_model_dimension` 哈希臂今天就在调。真模型臂仍需装包定标，本仓不写不可达路径。
"""

import logging
import zlib
from pathlib import Path
from typing import Any, Dict, List, Optional

import numpy as np

from .exceptions import VectorError
from .vector import create_vector_db
from .vector.base import normalize_vectors

logger = logging.getLogger(__name__)

#: 嵌入用的字符 n-gram 长度。取 2 与 `dedup` 的近似重复判据同一口径：中文逐字成对、
#: 拉丁词内也够密，且对短指令与长文本都不需要分词器。
NGRAM_SIZE = 2


def _ngrams(text: str, n: int = NGRAM_SIZE) -> List[str]:
    """小写、去空白后的字符 n-gram 清单

    短于 `n` 的非空串按整串算，空串产出空清单（嵌入成零向量，检索端不参与排序）。
    """
    cleaned = "".join(ch.lower() for ch in str(text) if not ch.isspace())
    if not cleaned:
        return []
    if len(cleaned) < n:
        return [cleaned]
    return [cleaned[i:i + n] for i in range(len(cleaned) - n + 1)]


#: 有显式持久化面（`persist()` / `load()`）的后端。目前只有 `FAISSDB` 实现这两个方法：
#: `ChromaDB` 由 `PersistentClient` 自管落盘，构造阶段就会 `mkdir` 并注册集合，类里
#: 没有 `persist`/`load`。所以本模块的「落盘建库」与「读回状态」两面对它无定义，
#: 在**构造之前**出声——构造才是那个写入动作，而 `index_status` 服务的是一条 GET。
EXPLICIT_PERSISTENCE_BACKENDS = ("faiss",)


def _require_persistence_surface(backend: str) -> None:
    """落盘与状态读取只对有 `persist`/`load` 面的后端开放

    Raises:
        VectorError: 所选后端没有显式持久化面（今天即 `chromadb`）
    """
    if backend not in EXPLICIT_PERSISTENCE_BACKENDS:
        raise VectorError(
            f"后端 {backend} 没有显式持久化面（persist/load 目前由 FAISSDB 实现），"
            f"无法落盘建库或读回索引状态；可选后端: {EXPLICIT_PERSISTENCE_BACKENDS}"
        )


def embed_texts(texts: List[str], dimension: int) -> List[np.ndarray]:
    """把文本嵌入成 `dimension` 维的 L2 归一化向量

    Args:
        texts: 待嵌入文本
        dimension: 向量宽度，必须是正整数（与 `create_vector_db` 同一道判）

    Returns:
        长度为 `len(texts)` 的一维数组清单，逐个形状为 `(dimension,)`；L340/A268 宽度自检
    """
    if not isinstance(dimension, int) or isinstance(dimension, bool) or dimension <= 0:
        raise VectorError(f"向量维度必须为正整数，当前是 {dimension}")

    rows = np.zeros((len(texts), dimension), dtype=np.float32); assert_model_dimension(rows.shape[1], dimension)  # L340/A268：写出的矩阵宽度必须等于配置
    for i, text in enumerate(texts):
        # crc32 而非 hash()：后者每进程随机化，同一份数据两次构建会出两套向量。
        for ngram in _ngrams(text):
            rows[i, zlib.crc32(ngram.encode("utf-8")) % dimension] += 1.0
    if not len(texts):
        return []
    return list(normalize_vectors(rows))


def _texts_of(items: List[Dict], text_key: str) -> List[str]:
    return [str(item.get(text_key, "") if isinstance(item, dict) else "") for item in items]


def build_index(items: List[Dict],
                *,
                backend: str,
                dimension: int,
                collection: str,
                storage_dir: Optional[str] = None,
                text_key: str = "instruction"):
    """由数据条目构建向量库

    Args:
        items: 数据条目清单
        backend / dimension / collection / storage_dir: 取自 `config.vector.*` 的生效值
        text_key: 参与嵌入的字段名

    Returns:
        已写入全部条目的 `VectorDB` 实例
    """
    texts = _texts_of(items, text_key)
    vectors = embed_texts(texts, dimension)
    db = create_vector_db(
        backend=backend,
        dimension=dimension,
        collection=collection,
        storage_dir=storage_dir,
    )
    # 元数据只放标量：`chromadb` 后端那侧要求标量值（嵌套结构要序列化），
    # 两个后端共用同一份元数据，所以这里就不放整条 item。
    metadata = [{"index": i, "text": text} for i, text in enumerate(texts)]
    db.add_vectors(vectors, metadata)
    return db


def search_similar(query: str,
                   items: List[Dict],
                   *,
                   backend: str,
                   dimension: int,
                   collection: str,
                   top_k: int = 5,
                   text_key: str = "instruction") -> List[Dict[str, Any]]:
    """在内存索引里做语义检索（不落盘）

    Args:
        query: 查询串
        items: 数据条目清单
        backend / dimension / collection: `config.vector.*` 的生效值
        top_k: 返回条数
        text_key: 参与嵌入的字段名

    Returns:
        按相似度降序的结果清单，每项形如
        `{"index": 原条目下标, "score": 相似度, "text": 命中文本, "item": 原条目}`
    """
    db = build_index(
        items,
        backend=backend,
        dimension=dimension,
        collection=collection,
        storage_dir=None,
        text_key=text_key,
    )
    query_vector = embed_texts([query], dimension)[0]
    results = []
    for hit in db.search(query_vector, top_k=top_k):
        index = int(hit["metadata"]["index"])
        results.append({
            "index": index,
            "score": float(hit["score"]),
            "text": hit["metadata"]["text"],
            "item": items[index] if 0 <= index < len(items) else None,
        })
    return results


def persist_index(items: List[Dict],
                  *,
                  backend: str,
                  dimension: int,
                  collection: str,
                  storage_dir: str,
                  text_key: str = "instruction") -> Dict[str, Any]:
    """建库并落盘，返回落点与规模

    `storage_dir` 为空串时 `VectorDB.persist()` 抛 `VectorError`，这里原样向上抛——
    由调用方（管道阶段 / 路由）决定怎么收敛。

    没有显式 `persist()` 面的后端在构造之前就被 `_require_persistence_surface` 拒掉，
    所以「配置写成 chromadb」的收场是一句能力说明，而不是 `AttributeError`。
    """
    _require_persistence_surface(backend)
    db = build_index(
        items,
        backend=backend,
        dimension=dimension,
        collection=collection,
        storage_dir=storage_dir,
        text_key=text_key,
    )
    db.persist()
    return {
        "count": db.count(),
        "backend": backend,
        "dimension": dimension,
        "collection": collection,
        "storage_dir": str(Path(storage_dir)),
    }


def index_status(*, backend: str,
                 dimension: int,
                 collection: str,
                 storage_dir: str) -> Dict[str, Any]:
    """读回持久化索引的状态

    Returns:
        `{"exists": 是否有落盘的索引, "count": 条数, "backend":..., "dimension":...,
        "collection":..., "storage_dir":...}`；不存在时 `count` 为 0。

    `backend` 取的是**配置选择**（`faiss` / `chromadb`），与 `persist_index` 的回执、
    `POST /api/dataset/semantic-search` 的回显同一口径。`FAISSDB.backend` 那个属性报的是
    **实际引擎**：本机没装 `faiss-cpu` 时它是 `"numpy"`（精确检索退化）。退化属依赖层事实，
    权威住在 `/api/system/dependencies`，这里不另立第二个口径——三面带同一只键名就该同义。

    能力闸在构造**之前**：`GET /api/dataset/vector-status` 必须是只读的。L340/A269 起 chromadb
    走 `_chroma_status` 只读探查（不构造客户端、不建目录）：`exists` 由持久化标记判定，
    `count` 恒 0 是「条数不可得」的能力边界；`persist_index` 仍拒 chromadb（无 persist/load 面）。
    """
    if backend == "chromadb":  # L340/A269：只读探查，不构造 PersistentClient
        return _chroma_status(backend, dimension, collection, storage_dir)
    _require_persistence_surface(backend)
    db = create_vector_db(
        backend=backend,
        dimension=dimension,
        collection=collection,
        storage_dir=storage_dir,
    )
    loaded = db.load()
    return {
        "exists": bool(loaded),
        "count": db.count() if loaded else 0,
        "backend": backend,
        "dimension": db.dimension,
        "collection": db.collection,
        "storage_dir": str(Path(storage_dir)),
    }


def assert_model_dimension(model_dimension: int, configured_dimension: int) -> None:
    """L340/A268：模型输出宽度与 `config.vector.dimension` 冲突即拒配置。

    口径是三条候选里最硬的一条：不等就抛，不「忽略配置」也不「静默截断补齐」——
    后两者会让同一份语料在不同配置下悄悄换一套向量，检索排名变了却没有出声。

    今天哈希嵌入把 `dimension` 当输出宽度用，`embed_texts` 用写出的矩阵宽度调本函数；
    将来接真模型（`sentence-transformers`）时，模型给 384 而配置写 768 也必须走这里，
    所以这条策略不是给死路径预留的装饰。

    Raises:
        VectorError: 两侧宽度不等
    """
    if model_dimension != configured_dimension:
        raise VectorError(
            f"模型输出宽度 {model_dimension} 与配置 dimension={configured_dimension} 冲突；"
            "L340/A268 的策略是拒配置，不忽略、不截断补齐"
        )


def _chroma_status(backend: str, dimension: int,
                   collection: str, storage_dir: str) -> Dict[str, Any]:
    """L340/A269：chromadb 的只读状态探查（GET 不落盘）。

    不 import chromadb、不构造 PersistentClient —— 后者在构造阶段就 `mkdir` 并注册
    集合，会把一条 GET 变成写操作。`exists` 只看持久化标记文件；`count` 恒 0，
    因为条数只有真客户端读得到，这里宁可把「不可得」写明，也不编一个假数。
    标记文件是整个 `storage_dir` 一份，不区分 collection，回执里的 collection 只是回显配置。
    """
    root = Path(storage_dir)
    return {
        "exists": (root / "chroma.sqlite3").is_file(),
        "count": 0,
        "backend": backend,
        "dimension": dimension,
        "collection": collection,
        "storage_dir": str(root),
    }
