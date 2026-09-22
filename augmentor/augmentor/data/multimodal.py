# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""多模态数据处理模块

整合图像与音频处理，产出统一的多模态训练数据记录。
"""

import logging
from dataclasses import dataclass, field
from pathlib import Path
from typing import Dict, Any, List, Optional

from .image import ImageProcessor
from .audio import AudioProcessor

logger = logging.getLogger(__name__)


@dataclass
class MultimodalRecord:
    """多模态记录"""
    text: str = ""
    image: Optional[Dict[str, Any]] = None
    audio: Optional[Dict[str, Any]] = None
    modalities: List[str] = field(default_factory=list)
    valid: bool = True
    errors: List[str] = field(default_factory=list)

    def to_dict(self) -> Dict[str, Any]:
        """转换为字典

        Returns:
            记录字典
        """
        return {
            "text": self.text,
            "image": self.image,
            "audio": self.audio,
            "modalities": self.modalities,
            "valid": self.valid,
            "errors": self.errors
        }


class MultimodalProcessor:
    """多模态处理器"""

    def __init__(self,
                 image_extensions: List[str] = None,
                 audio_extensions: List[str] = None):
        """初始化多模态处理器

        Args:
            image_extensions: 支持的图像扩展名
            audio_extensions: 支持的音频扩展名
        """
        self.image_processor = ImageProcessor(image_extensions)
        self.audio_processor = AudioProcessor(audio_extensions)

    def process_image(self, image_path: str) -> Dict[str, Any]:
        """处理图像

        Args:
            image_path: 图像路径

        Returns:
            图像信息字典
        """
        return self.image_processor.process_image(image_path).to_dict()

    def process_audio(self, audio_path: str) -> Dict[str, Any]:
        """处理音频

        Args:
            audio_path: 音频路径

        Returns:
            音频信息字典
        """
        return self.audio_processor.process_audio(audio_path).to_dict()

    def fuse_modalities(self, data: Dict[str, Any]) -> MultimodalRecord:
        """融合多模态数据

        Args:
            data: 包含 text / image / audio 字段的字典

        Returns:
            MultimodalRecord 实例
        """
        record = MultimodalRecord(text=data.get("text", "") or "")
        modalities: List[str] = []

        if record.text:
            modalities.append("text")

        image_path = data.get("image")
        if image_path:
            image_info = self.process_image(image_path)
            record.image = image_info
            if image_info.get("valid"):
                modalities.append("image")
            else:
                record.errors.append(
                    f"图像处理失败: {image_info.get('error', '未知错误')}"
                )

        audio_path = data.get("audio")
        if audio_path:
            audio_info = self.process_audio(audio_path)
            record.audio = audio_info
            if audio_info.get("valid"):
                modalities.append("audio")
            else:
                record.errors.append(
                    f"音频处理失败: {audio_info.get('error', '未知错误')}"
                )

        record.modalities = modalities
        record.valid = bool(modalities) and not record.errors

        return record

    def process_directory(self, directory: str) -> List[MultimodalRecord]:
        """扫描目录并处理其中的多模态文件

        同名文件（如 sample.jpg + sample.wav）会被融合为一条记录。

        Args:
            directory: 目录路径

        Returns:
            MultimodalRecord 列表
        """
        dir_path = Path(directory)
        if not dir_path.is_dir():
            raise ValueError(f"目录不存在: {directory}")

        grouped: Dict[str, Dict[str, Any]] = {}

        for file_path in sorted(dir_path.iterdir()):
            if not file_path.is_file():
                continue

            if self.image_processor.is_supported(str(file_path)):
                grouped.setdefault(file_path.stem, {})["image"] = str(file_path)
            elif self.audio_processor.is_supported(str(file_path)):
                grouped.setdefault(file_path.stem, {})["audio"] = str(file_path)

        records = []
        for stem, payload in grouped.items():
            payload.setdefault("text", stem)
            records.append(self.fuse_modalities(payload))

        logger.info(f"扫描目录 {directory}，生成 {len(records)} 条多模态记录")
        return records

    def generate_report(self, records: List[MultimodalRecord]) -> Dict[str, Any]:
        """生成多模态处理报告

        Args:
            records: 多模态记录列表

        Returns:
            报告字典
        """
        modality_counts: Dict[str, int] = {}
        for record in records:
            for modality in record.modalities:
                modality_counts[modality] = modality_counts.get(modality, 0) + 1

        valid_count = sum(1 for r in records if r.valid)
        errors = [e for r in records for e in r.errors]

        return {
            "total_records": len(records),
            "valid_records": valid_count,
            "invalid_records": len(records) - valid_count,
            "modality_counts": modality_counts,
            "error_count": len(errors),
            "errors": errors[:20]
        }
