"""音频数据处理模块

优先使用 soundfile，未安装时使用标准库 wave 解析 WAV，其余格式退化为基础信息。
"""

import logging
import wave
from dataclasses import dataclass, field
from pathlib import Path
from typing import Dict, Any, List

logger = logging.getLogger(__name__)

# 支持的音频扩展名
SUPPORTED_EXTENSIONS = [".wav", ".mp3", ".flac", ".ogg", ".m4a"]


@dataclass
class AudioInfo:
    """音频元信息"""
    path: str
    format: str
    duration: float = 0.0
    sample_rate: int = 0
    channels: int = 0
    size_bytes: int = 0
    valid: bool = True
    error: str = ""
    metadata: Dict[str, Any] = field(default_factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        """转换为字典

        Returns:
            音频信息字典
        """
        return {
            "path": self.path,
            "format": self.format,
            "duration": self.duration,
            "sample_rate": self.sample_rate,
            "channels": self.channels,
            "size_bytes": self.size_bytes,
            "valid": self.valid,
            "error": self.error,
            "metadata": self.metadata
        }


class AudioProcessor:
    """音频处理器"""

    def __init__(self, supported_extensions: List[str] = None):
        """初始化音频处理器

        Args:
            supported_extensions: 支持的扩展名列表
        """
        self.supported_extensions = supported_extensions or SUPPORTED_EXTENSIONS

    def is_supported(self, path: str) -> bool:
        """判断文件是否为支持的音频格式

        Args:
            path: 文件路径

        Returns:
            是否支持
        """
        return Path(path).suffix.lower() in self.supported_extensions

    def _parse_with_soundfile(self, path: Path) -> Dict[str, Any]:
        """使用 soundfile 解析音频

        Args:
            path: 文件路径

        Returns:
            音频信息字典

        Raises:
            ImportError: soundfile 未安装
        """
        import soundfile as sf

        info = sf.info(str(path))
        return {
            "format": info.format,
            "duration": float(info.duration),
            "sample_rate": int(info.samplerate),
            "channels": int(info.channels),
            "metadata": {"subtype": info.subtype, "parser": "soundfile"}
        }

    def _parse_wav(self, path: Path) -> Dict[str, Any]:
        """使用标准库 wave 解析 WAV

        Args:
            path: 文件路径

        Returns:
            音频信息字典
        """
        with wave.open(str(path), 'rb') as wav:
            frames = wav.getnframes()
            sample_rate = wav.getframerate()
            channels = wav.getnchannels()
            duration = frames / sample_rate if sample_rate else 0.0

            return {
                "format": "WAV",
                "duration": duration,
                "sample_rate": sample_rate,
                "channels": channels,
                "metadata": {
                    "sample_width": wav.getsampwidth(),
                    "frames": frames,
                    "parser": "stdlib"
                }
            }

    def process_audio(self, audio_path: str) -> AudioInfo:
        """处理单个音频

        Args:
            audio_path: 音频路径

        Returns:
            AudioInfo 实例
        """
        path = Path(audio_path)

        if not path.exists():
            return AudioInfo(
                path=str(path), format="", valid=False, error="文件不存在"
            )

        if not self.is_supported(str(path)):
            return AudioInfo(
                path=str(path),
                format=path.suffix.lstrip('.'),
                size_bytes=path.stat().st_size,
                valid=False,
                error=f"不支持的音频格式: {path.suffix}"
            )

        size_bytes = path.stat().st_size
        suffix = path.suffix.lower()

        try:
            try:
                info = self._parse_with_soundfile(path)
            except ImportError:
                if suffix == ".wav":
                    info = self._parse_wav(path)
                else:
                    # 无 soundfile 时仅能提供基础信息
                    info = {
                        "format": suffix.lstrip('.').upper(),
                        "duration": 0.0,
                        "sample_rate": 0,
                        "channels": 0,
                        "metadata": {"parser": "basic"}
                    }
        except Exception as e:
            logger.warning(f"解析音频失败 {audio_path}: {e}")
            return AudioInfo(
                path=str(path),
                format=suffix.lstrip('.'),
                size_bytes=size_bytes,
                valid=False,
                error=str(e)
            )

        return AudioInfo(
            path=str(path),
            format=info.get("format", ""),
            duration=info.get("duration", 0.0),
            sample_rate=info.get("sample_rate", 0),
            channels=info.get("channels", 0),
            size_bytes=size_bytes,
            valid=True,
            metadata=info.get("metadata", {})
        )

    def batch_process(self, paths: List[str]) -> List[AudioInfo]:
        """批量处理音频

        Args:
            paths: 音频路径列表

        Returns:
            AudioInfo 列表
        """
        return [self.process_audio(p) for p in paths]
