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
from ..exceptions import DataLoadError

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
        #: A401：最近一次 `process_directory` 收集到的事实（被覆盖 / 被忽略）。
        #: 每次 `process_directory` 调用都会**重写**它 ⇒ 它只描述那一次扫描。
        self.scan_notes: List[str] = []
        #: 上一次扫描产出的记录列表（身份比对用，见 `generate_report` 的 notes 回落）
        self._scan_records: Optional[List[MultimodalRecord]] = None

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

        **A401：两件「静默吃掉」的事现在被记进 `self.scan_notes`**：

        ① **同模态同名互相覆盖**——分组键是 `file_path.stem`、槽位是模态，
           所以 `sample.jpg` 与 `sample.png`（都是 image）会落进**同一个槽位**，
           `sorted` 序**后者胜** ⇒ 前一只**静默消失**。docstring 承诺的是
           「同名文件融合为一条记录」（**跨模态**，那是特性），**同模态**的互相
           覆盖不在那句承诺里 ⇒ 读者以为扫了 4 只、实际只处理了 3 只。
        ② **子目录整只跳过**——只扫顶层（`iterdir()` 的非文件项一律 `continue`），
           目录里若有 `nested/`，里面的媒体**一只都不处理**，而回执里 0 条告警。

        这两件事**都不改处理行为**（只扫顶层、后者胜都是既有语义），改的是
        **回执**：让「被吃掉的东西」在回执里被点名。

        Args:
            directory: 目录路径

        Returns:
            MultimodalRecord 列表
        """
        dir_path = Path(directory)
        if not dir_path.is_dir():
            raise DataLoadError(f"目录不存在: {directory}")

        grouped: Dict[str, Dict[str, Any]] = {}
        notes: List[str] = []

        for file_path in sorted(dir_path.iterdir()):
            if file_path.is_dir():
                # A401：子目录整只跳过 —— 记一笔，别让它静默
                notes.append(f"子目录 {file_path.name}/ 未扫描（只扫顶层文件）")
                continue
            if not file_path.is_file():
                continue

            if self.image_processor.is_supported(str(file_path)):
                slot = "image"
            elif self.audio_processor.is_supported(str(file_path)):
                slot = "audio"
            else:
                # 不在支持的扩展名里 —— 正常的过滤，不是「吃掉东西」，不记
                continue

            bucket = grouped.setdefault(file_path.stem, {})
            if slot in bucket:
                # A401：同模态同名 ⇒ 前一只被覆盖（跨模态不覆盖，那是融合特性）
                notes.append(
                    f"{Path(bucket[slot]).name} 被同名的 {file_path.name} 覆盖"
                    f"（同模态 {slot}，只保留后者）"
                )
            bucket[slot] = str(file_path)

        records = []
        for stem, payload in grouped.items():
            payload.setdefault("text", stem)
            records.append(self.fuse_modalities(payload))

        # A401：这批事实只描述**这一次**扫描 ⇒ 每次扫描都重写
        self.scan_notes = notes
        self._scan_records = records

        logger.info(
            "扫描目录 %s，生成 %d 条多模态记录（%d 条告警）",
            directory, len(records), len(notes),
        )
        return records

    def generate_report(self, records: List[MultimodalRecord],
                        notes: Optional[List[str]] = None) -> Dict[str, Any]:
        """生成多模态处理报告

        **A401：`warning_count` / `warnings` 两格是「被吃掉的东西」的面**——
        同模态同名互相覆盖、子目录整只跳过，这两件事改前在回执里各占 0 条，
        于是「扫了 4 只、只处理了 3 只」这种事实不可见。

        `notes` 省略时回落到**本次扫描**收集到的那批事实（`self.scan_notes`），
        但只在 `records` 正是那一次扫描的产物时才回落（身份比对）⇒ 拿别的
        记录来问报告时不会串上上一批的告警。

        Args:
            records: 多模态记录列表
            notes: 告警清单；`None` 表示「用本次扫描收集到的那批」

        Returns:
            报告字典
        """
        modality_counts: Dict[str, int] = {}
        for record in records:
            for modality in record.modalities:
                modality_counts[modality] = modality_counts.get(modality, 0) + 1

        valid_count = sum(1 for r in records if r.valid)
        errors = [e for r in records for e in r.errors]

        if notes is None:
            notes = self.scan_notes if self._scan_records is records else []
        warnings = list(notes)

        return {
            "total_records": len(records),
            "valid_records": valid_count,
            "invalid_records": len(records) - valid_count,
            "modality_counts": modality_counts,
            "error_count": len(errors),
            "errors": errors[:20],
            # A402：`errors` 那一格硬截 20 而**没有截断旗** ⇒ 「全给了」与「被截了」
            # 两种读法在回执里不可分。加这一格让两者可分（旗为假时 `error_count`
            # 就等于 `len(errors)`）。
            "errors_truncated": len(errors) > 20,
            # A401：告警面。**不截断**（`errors` 那格硬截 20 且无截断旗是另一族
            # 待拍，A402）——这一批的规模由「目录里有多少只文件」决定，而同一条
            # 响应的 `records` 本来就是全量回传，两边口径一致。
            "warning_count": len(warnings),
            "warnings": warnings,
        }
