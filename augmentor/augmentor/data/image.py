# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""图像数据处理模块

优先使用 Pillow，未安装时退化为标准库解析文件头，保证离线可用。
"""

import logging
import struct
from dataclasses import dataclass, field
from pathlib import Path
from typing import Dict, Any, List

logger = logging.getLogger(__name__)

# 支持的图像扩展名
SUPPORTED_EXTENSIONS = [".jpg", ".jpeg", ".png", ".bmp", ".gif", ".webp"]


@dataclass
class ImageInfo:
    """图像元信息"""
    path: str
    format: str
    width: int = 0
    height: int = 0
    mode: str = ""
    size_bytes: int = 0
    valid: bool = True
    error: str = ""
    metadata: Dict[str, Any] = field(default_factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        """转换为字典

        Returns:
            图像信息字典
        """
        return {
            "path": self.path,
            "format": self.format,
            "width": self.width,
            "height": self.height,
            "mode": self.mode,
            "size_bytes": self.size_bytes,
            "valid": self.valid,
            "error": self.error,
            "metadata": self.metadata
        }


def _parse_png(data: bytes) -> Dict[str, Any]:
    """解析 PNG 尺寸"""
    width, height = struct.unpack(">II", data[16:24])
    return {"width": width, "height": height}


def _parse_gif(data: bytes) -> Dict[str, Any]:
    """解析 GIF 尺寸"""
    width, height = struct.unpack("<HH", data[6:10])
    return {"width": width, "height": height}


def _parse_bmp(data: bytes) -> Dict[str, Any]:
    """解析 BMP 尺寸"""
    width, height = struct.unpack("<ii", data[18:26])
    return {"width": abs(width), "height": abs(height)}


def _parse_jpeg(data: bytes) -> Dict[str, Any]:
    """解析 JPEG 尺寸（扫描 SOF 标记）"""
    index = 2
    length = len(data)
    while index < length - 9:
        if data[index] != 0xFF:
            index += 1
            continue
        marker = data[index + 1]
        # SOF0-SOF15，排除 DHT(0xC4)、JPG(0xC8)、DAC(0xCC)
        if 0xC0 <= marker <= 0xCF and marker not in (0xC4, 0xC8, 0xCC):
            height, width = struct.unpack(">HH", data[index + 5:index + 9])
            return {"width": width, "height": height}
        segment_length = struct.unpack(">H", data[index + 2:index + 4])[0]
        index += 2 + segment_length
    raise ValueError("无法解析 JPEG 尺寸")


_HEADER_PARSERS = {
    ".png": _parse_png,
    ".gif": _parse_gif,
    ".bmp": _parse_bmp,
    ".jpg": _parse_jpeg,
    ".jpeg": _parse_jpeg
}


class ImageProcessor:
    """图像处理器"""

    def __init__(self, supported_extensions: List[str] = None):
        """初始化图像处理器

        Args:
            supported_extensions: 支持的扩展名列表
        """
        self.supported_extensions = supported_extensions or SUPPORTED_EXTENSIONS

    def is_supported(self, path: str) -> bool:
        """判断文件是否为支持的图像格式

        Args:
            path: 文件路径

        Returns:
            是否支持
        """
        return Path(path).suffix.lower() in self.supported_extensions

    def _parse_with_pillow(self, path: Path) -> Dict[str, Any]:
        """使用 Pillow 解析图像

        Args:
            path: 文件路径

        Returns:
            图像信息字典

        Raises:
            ImportError: Pillow 未安装
        """
        from PIL import Image

        with Image.open(path) as img:
            return {
                "format": img.format or path.suffix.lstrip('.').upper(),
                "width": img.width,
                "height": img.height,
                "mode": img.mode,
                "metadata": {"info_keys": list(img.info.keys())}
            }

    def _parse_header(self, path: Path) -> Dict[str, Any]:
        """使用标准库解析文件头

        Args:
            path: 文件路径

        Returns:
            图像信息字典
        """
        suffix = path.suffix.lower()
        parser = _HEADER_PARSERS.get(suffix)
        if parser is None:
            raise ValueError(f"不支持通过文件头解析的格式: {suffix}")

        with open(path, 'rb') as f:
            data = f.read(65536)

        if len(data) < 26:
            raise ValueError("文件过小，可能已损坏")

        dimensions = parser(data)
        return {
            "format": suffix.lstrip('.').upper(),
            "width": dimensions["width"],
            "height": dimensions["height"],
            "mode": "",
            "metadata": {"parser": "stdlib"}
        }

    def process_image(self, image_path: str) -> ImageInfo:
        """处理单张图像

        Args:
            image_path: 图像路径

        Returns:
            ImageInfo 实例
        """
        path = Path(image_path)

        if not path.exists():
            return ImageInfo(
                path=str(path), format="", valid=False, error="文件不存在"
            )

        if not self.is_supported(str(path)):
            return ImageInfo(
                path=str(path),
                format=path.suffix.lstrip('.'),
                size_bytes=path.stat().st_size,
                valid=False,
                error=f"不支持的图像格式: {path.suffix}"
            )

        size_bytes = path.stat().st_size

        try:
            try:
                info = self._parse_with_pillow(path)
            except Exception:
                info = self._parse_header(path)
        except Exception as e:
            logger.warning(f"解析图像失败 {image_path}: {e}")
            return ImageInfo(
                path=str(path),
                format=path.suffix.lstrip('.'),
                size_bytes=size_bytes,
                valid=False,
                error=str(e)
            )

        return ImageInfo(
            path=str(path),
            format=info.get("format", ""),
            width=info.get("width", 0),
            height=info.get("height", 0),
            mode=info.get("mode", ""),
            size_bytes=size_bytes,
            valid=True,
            metadata=info.get("metadata", {})
        )

    def batch_process(self, paths: List[str]) -> List[ImageInfo]:
        """批量处理图像

        Args:
            paths: 图像路径列表

        Returns:
            ImageInfo 列表
        """
        return [self.process_image(p) for p in paths]
