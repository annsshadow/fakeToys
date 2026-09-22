# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""数据处理模块

包含数据清洗、自动标注与多模态处理。
"""

from .cleaner import DataCleaner, CleanResult
from .annotator import AutoAnnotator, AnnotationResult
from .image import ImageProcessor, ImageInfo
from .audio import AudioProcessor, AudioInfo
from .multimodal import MultimodalProcessor, MultimodalRecord

__all__ = [
    "DataCleaner",
    "CleanResult",
    "AutoAnnotator",
    "AnnotationResult",
    "ImageProcessor",
    "ImageInfo",
    "AudioProcessor",
    "AudioInfo",
    "MultimodalProcessor",
    "MultimodalRecord"
]
