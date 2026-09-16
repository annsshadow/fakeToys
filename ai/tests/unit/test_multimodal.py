"""多模态数据处理单元测试

图像与音频元信息解析在无第三方库时也必须可用，否则离线环境无法工作。
"""

import struct
import wave
import zlib

import pytest

from augmentor.data import (
    ImageProcessor,
    AudioProcessor,
    MultimodalProcessor,
    MultimodalRecord,
)


def make_png(path, width=2, height=3):
    """生成一张真实的最小 PNG

    Args:
        path: 输出路径
        width: 宽度
        height: 高度
    """

    def chunk(chunk_type, data):
        payload = chunk_type + data
        return (
            struct.pack(">I", len(data))
            + payload
            + struct.pack(">I", zlib.crc32(payload) & 0xFFFFFFFF)
        )

    ihdr = struct.pack(">IIBBBBB", width, height, 8, 2, 0, 0, 0)
    raw = b"".join(b"\x00" + b"\xff\x00\x00" * width for _ in range(height))

    path.write_bytes(
        b"\x89PNG\r\n\x1a\n"
        + chunk(b"IHDR", ihdr)
        + chunk(b"IDAT", zlib.compress(raw))
        + chunk(b"IEND", b"")
    )


def make_wav(path, seconds=0.5, sample_rate=8000, channels=1):
    """生成一个真实的最小 WAV

    Args:
        path: 输出路径
        seconds: 时长
        sample_rate: 采样率
        channels: 声道数
    """
    frames = int(seconds * sample_rate)
    with wave.open(str(path), 'wb') as wav:
        wav.setnchannels(channels)
        wav.setsampwidth(2)
        wav.setframerate(sample_rate)
        wav.writeframes(b"\x00\x00" * frames * channels)


class TestImageProcessor:
    """图像处理"""

    def test_parses_png_dimensions(self, tmp_path):
        """PNG 尺寸解析必须准确，否则图像数据无法被正确引用"""
        image_path = tmp_path / "sample.png"
        make_png(image_path, width=4, height=6)

        info = ImageProcessor().process_image(str(image_path))

        assert info.valid is True
        assert info.width == 4
        assert info.height == 6
        assert info.format == "PNG"
        assert info.size_bytes > 0

    def test_parses_gif_dimensions(self, tmp_path):
        """GIF 使用小端序存储尺寸，需正确解析"""
        image_path = tmp_path / "sample.gif"
        header = b"GIF89a" + struct.pack("<HH", 7, 9) + b"\x00" * 20
        image_path.write_bytes(header)

        info = ImageProcessor().process_image(str(image_path))

        assert info.valid is True
        assert (info.width, info.height) == (7, 9)

    def test_parses_bmp_dimensions(self, tmp_path):
        """BMP 尺寸需支持解析"""
        image_path = tmp_path / "sample.bmp"
        header = b"BM" + b"\x00" * 16 + struct.pack("<ii", 11, 13) + b"\x00" * 8
        image_path.write_bytes(header)

        info = ImageProcessor().process_image(str(image_path))

        assert info.valid is True
        assert (info.width, info.height) == (11, 13)

    def test_missing_file_reported_invalid(self):
        """文件不存在时返回 valid=False 而不是抛异常，便于批量处理"""
        info = ImageProcessor().process_image("not-exists.png")

        assert info.valid is False
        assert "不存在" in info.error

    def test_unsupported_extension(self, tmp_path):
        """不支持的扩展名需明确报错"""
        file_path = tmp_path / "note.txt"
        file_path.write_text("hello", encoding='utf-8')

        info = ImageProcessor().process_image(str(file_path))

        assert info.valid is False
        assert "不支持的图像格式" in info.error

    def test_corrupted_file(self, tmp_path):
        """截断损坏的文件需被识别为无效，而不是返回错误尺寸"""
        file_path = tmp_path / "broken.png"
        file_path.write_bytes(b"\x89PNG\r\n\x1a\n" + b"\x00" * 4)

        info = ImageProcessor().process_image(str(file_path))

        assert info.valid is False
        assert info.error

    def test_is_supported(self):
        """扩展名判定大小写不敏感"""
        processor = ImageProcessor()
        assert processor.is_supported("a.PNG")
        assert processor.is_supported("a.jpg")
        assert not processor.is_supported("a.txt")

    def test_batch_process(self, tmp_path):
        """批量处理结果数量需与输入一致"""
        paths = []
        for index in range(3):
            image_path = tmp_path / f"img{index}.png"
            make_png(image_path, width=index + 1, height=index + 1)
            paths.append(str(image_path))

        results = ImageProcessor().batch_process(paths)

        assert len(results) == 3
        assert [r.width for r in results] == [1, 2, 3]


class TestAudioProcessor:
    """音频处理"""

    def test_parses_wav_metadata(self, tmp_path):
        """WAV 时长/采样率/声道数解析必须准确"""
        audio_path = tmp_path / "sample.wav"
        make_wav(audio_path, seconds=1.0, sample_rate=8000, channels=2)

        info = AudioProcessor().process_audio(str(audio_path))

        assert info.valid is True
        assert info.sample_rate == 8000
        assert info.channels == 2
        assert info.duration == pytest.approx(1.0, abs=0.01)

    def test_missing_file_reported_invalid(self):
        """文件不存在时返回 valid=False"""
        info = AudioProcessor().process_audio("not-exists.wav")

        assert info.valid is False
        assert "不存在" in info.error

    def test_unsupported_extension(self, tmp_path):
        """不支持的扩展名需明确报错"""
        file_path = tmp_path / "note.txt"
        file_path.write_text("hello", encoding='utf-8')

        info = AudioProcessor().process_audio(str(file_path))

        assert info.valid is False
        assert "不支持的音频格式" in info.error

    def test_corrupted_wav(self, tmp_path):
        """损坏的 WAV 需被识别为无效"""
        file_path = tmp_path / "broken.wav"
        file_path.write_bytes(b"RIFF\x00\x00\x00\x00WAVEjunk")

        info = AudioProcessor().process_audio(str(file_path))

        assert info.valid is False

    def test_batch_process(self, tmp_path):
        """批量处理结果数量需与输入一致"""
        paths = []
        for index in range(2):
            audio_path = tmp_path / f"a{index}.wav"
            make_wav(audio_path, seconds=0.2)
            paths.append(str(audio_path))

        results = AudioProcessor().batch_process(paths)
        assert len(results) == 2


class TestMultimodalProcessor:
    """多模态融合"""

    def test_fuse_text_image_audio(self, tmp_path):
        """三种模态齐备时 modalities 需全部列出"""
        image_path = tmp_path / "sample.png"
        audio_path = tmp_path / "sample.wav"
        make_png(image_path)
        make_wav(audio_path)

        record = MultimodalProcessor().fuse_modalities({
            "text": "图文说明",
            "image": str(image_path),
            "audio": str(audio_path)
        })

        assert isinstance(record, MultimodalRecord)
        assert set(record.modalities) == {"text", "image", "audio"}
        assert record.valid is True

    def test_text_only(self):
        """仅有文本时也应生成有效记录"""
        record = MultimodalProcessor().fuse_modalities({"text": "纯文本"})

        assert record.modalities == ["text"]
        assert record.valid is True

    def test_invalid_modality_collects_error(self, tmp_path):
        """模态文件损坏时需记录错误并标记记录无效"""
        record = MultimodalProcessor().fuse_modalities({
            "text": "说明",
            "image": str(tmp_path / "missing.png")
        })

        assert record.valid is False
        assert record.errors
        assert "image" not in record.modalities

    def test_process_directory_groups_same_stem(self, tmp_path):
        """同名图像与音频应被融合为一条记录，这是多模态配对的约定"""
        make_png(tmp_path / "pair.png")
        make_wav(tmp_path / "pair.wav")

        records = MultimodalProcessor().process_directory(str(tmp_path))

        assert len(records) == 1
        assert set(records[0].modalities) == {"text", "image", "audio"}

    def test_process_directory_missing_raises(self):
        """目录不存在时必须报错，避免静默返回空结果"""
        with pytest.raises(ValueError):
            MultimodalProcessor().process_directory("not-a-dir")

    def test_generate_report(self, tmp_path):
        """报告需统计各模态数量与错误数"""
        make_png(tmp_path / "pair.png")

        processor = MultimodalProcessor()
        records = processor.process_directory(str(tmp_path))
        report = processor.generate_report(records)

        assert report["total_records"] == 1
        assert report["modality_counts"]["image"] == 1
        assert report["valid_records"] == 1
