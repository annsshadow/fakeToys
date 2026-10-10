# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""多语言能力的产品入口（L217）

**为什么需要这一层**：`augmentor/multilingual.py` 的 `MultilingualSupport` 从前在产品侧
一个实例都没建过（`api/`、`augmentor/`、`cli.py` 全树零调用者，只有测试 import），于是
`multilingual` 节四只键全部只写不读。本模块是唯一的生产者，四只键在这里第一次真的决定产物：

- `supported_langs` ⇒ 构造参数，决定哪些目标语言可翻、以及语言画像里哪些检测结果落在清单外；
- `default_target_lang` ⇒ 请求没带 `target_lang` 时真正生效的目标语言；
- `translate_batch_size` ⇒ 一次提交给翻译线程池的条数上界（分批的批宽）；
- `enabled` ⇒ 出厂 `false`，它是「这条会打模型、要花钱的路径开不开」那只闸，判据在路由侧。

**权威序沿用 L214 定稿的那一条**：请求参数 > 配置键 > SDK 内置默认。所以
`target_lang` 省略时才落到 `default_target_lang`，而 `MultilingualSupport.translate` 自己
那句 `target_lang: str = "en"` 只在**本层没传**时才可能生效——本层总是显式传，内置默认
因此拿不到控制权（这一条由注入取证钉：把 `or default_target_lang` 改成写死 `"en"` 立刻红）。

**为什么要分批**：`MultilingualSupport.batch_translate` 把整份清单一次性 submit 给
`ThreadPoolExecutor`，于是一次 5000 条的翻译会在飞行中挂起 5000 个 future。本层按
`translate_batch_size` 切批、逐批调用，把飞行中的任务数压到 `min(批宽, max_workers)` 以内
（`max_workers` 是 `MultilingualSupport` 的构造参数，出厂 4，本轮不动它）；批次之间不并发，
所以模型侧的瞬时压力也是这个上界，而不是一次全放。
"""

import logging
from typing import Any, Dict, List, Optional

from .exceptions import ModelNotConfiguredError
from .multilingual import MultilingualSupport

logger = logging.getLogger(__name__)


def language_profile(items: List[Dict[str, Any]], *, supported_langs: List[str],
                     default_target_lang: str, text_key: str = "instruction") -> Dict[str, Any]:
    """语言分布画像（零模型、纯确定性）

    `detect_language` 会产出 `mixed` 这个**检测标签**而不是语言码，它天然落在
    `supported_langs` 之外，所以 `outside_supported` 里出现 `mixed` 是事实陈述，不是错误：
    混合文本照样能翻（`translate` 只按目标语言判，源语言只做同语言短路）。

    `default_target_lang` 在这里只回显不判事——画像是只读的，用它的人得自己带进请求。
    """
    support = MultilingualSupport(supported_langs=list(supported_langs))
    report = support.analyze_languages(items, text_key=text_key)
    supported = set(support.supported_langs)
    return {
        "total_items": report["total_items"],
        "distribution": report["distribution"],
        "ratios": report["ratios"],
        "outside_supported": sorted(
            lang for lang in report["distribution"] if lang not in supported
        ),
        "supported_langs": list(support.supported_langs),
        "default_target_lang": default_target_lang,
    }


def translate_dataset(items: List[Dict[str, Any]], *, backend: Any,
                      supported_langs: List[str], default_target_lang: str,
                      batch_size: int, target_lang: Optional[str] = None,
                      text_key: str = "instruction",
                      keep_original: bool = True) -> Dict[str, Any]:
    """按 `translate_batch_size` 分批翻译，返回译文与生效值

    Args:
        items: 数据清单
        backend: 模型后端。为 None 时**本层**在分批之前抛 `ModelNotConfiguredError`
            （`ValueError` 子类 ⇒ 400）：SDK 的 `batch_translate` 把逐条异常吞成
            「回退原文」，所以后端缺失若在下游出声，端点会带着整份原文回 200
        supported_langs: `multilingual.supported_langs` 的生效值
        default_target_lang: `multilingual.default_target_lang` 的生效值
        batch_size: `multilingual.translate_batch_size`，一次提交的条数上界。
            下界由 `MultilingualConfig.__post_init__` 的 `require_count` 守住，
            这里不重复校验
        target_lang: 请求侧的目标语言，给了就赢过配置键
        text_key: 参与翻译的字段名
        keep_original: 是否保留原文到 `{text_key}_original`

    Returns:
        `total` / `batches` / `batch_size` / `target_lang` / `supported_langs` / `items`

    Raises:
        ModelNotConfiguredError: backend 为 None

    「最终目标语言在不在 `supported_langs` 里」的判据**不在本层**——SDK 的
    `batch_translate` 顶部就有 `DataValidationError` 这道闸，且它在建线程池之前抛，
    所以越界语言一条模型调用都不会发出。本层曾重复写过一道同判据的闸，摘掉它之后
    测试仍全绿，即两存必有一盲（注入取证见账本 L217），按仓库「判据住在 SDK/config 层」
    的惯例只留下游那一道。
    """
    resolved = target_lang or default_target_lang
    if backend is None:
        raise ModelNotConfiguredError("翻译需要配置模型后端")

    support = MultilingualSupport(model_backend=backend, supported_langs=list(supported_langs))
    translated: List[Dict[str, Any]] = []
    for start in range(0, len(items), batch_size):
        translated.extend(
            support.batch_translate(
                items[start:start + batch_size],
                target_lang=resolved,
                text_key=text_key,
                keep_original=keep_original,
            )
        )
    return {
        "total": len(translated),
        "batches": (len(items) + batch_size - 1) // batch_size if items else 0,
        "batch_size": batch_size,
        "target_lang": resolved,
        "supported_langs": list(supported_langs),
        "items": translated,
    }
