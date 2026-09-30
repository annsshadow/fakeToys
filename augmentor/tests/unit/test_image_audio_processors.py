# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""data/image.py 与 data/audio.py 剩余分支测试

image：文件头解析（png/gif/bmp/jpeg 直接调 _parse_header + 合成字节）、
过小文件/不支持格式错误；audio：无 soundfile 时 wav 标准库解析与
非 wav 基础信息回退、批处理。
"""

import struct
import wave

import pytest

from augmentor.data.image import ImageProcessor
from augmentor.data.audio import AudioProcessor


def _make_png(width, height):
    return (
        b"\x89PNG\r\n\x1a\n"
        + struct.pack(">I", 13)
        + b"IHDR"
        + struct.pack(">II", width, height)
        + b"\x00" * 20
    )


def _make_gif(width, height):
    return b"GIF89a" + struct.pack("<HH", width, height) + b"\x00" * 20


def _make_bmp(width, height):
    return b"\x00" * 18 + struct.pack("<ii", width, height) + b"\x00" * 20


def _make_jpeg(width, height):
    # SOI + APP0(长度16) + SOF0(含尺寸)
    app0 = b"\xff\xe0" + struct.pack(">H", 16) + b"\x00" * 14
    sof0 = (
        b"\xff\xc0"
        + struct.pack(">H", 8)
        + b"\x08"
        + struct.pack(">HH", height, width)
        + b"\x01"
    )
    return b"\xff\xd8" + app0 + sof0 + b"\x00" * 8


class TestImageHeaderParsing:
    def test_png_header(self, tmp_path):
        p = tmp_path / "a.png"
        p.write_bytes(_make_png(640, 480))
        info = ImageProcessor()._parse_header(p)
        assert info["width"] == 640
        assert info["height"] == 480
        assert info["format"] == "PNG"
        assert info["metadata"]["parser"] == "stdlib"

    def test_gif_header(self, tmp_path):
        p = tmp_path / "a.gif"
        p.write_bytes(_make_gif(320, 200))
        info = ImageProcessor()._parse_header(p)
        assert info["width"] == 320
        assert info["height"] == 200

    def test_bmp_header(self, tmp_path):
        p = tmp_path / "a.bmp"
        p.write_bytes(_make_bmp(800, 600))
        info = ImageProcessor()._parse_header(p)
        assert info["width"] == 800
        assert info["height"] == 600

    def test_jpeg_header(self, tmp_path):
        p = tmp_path / "a.jpg"
        p.write_bytes(_make_jpeg(1024, 768))
        info = ImageProcessor()._parse_header(p)
        assert info["width"] == 1024
        assert info["height"] == 768

    def test_jpeg_unparseable_raises(self, tmp_path):
        p = tmp_path / "bad.jpg"
        p.write_bytes(b"\xff\xd8" + b"\x00" * 40)
        with pytest.raises(ValueError, match="无法解析 JPEG"):
            ImageProcessor()._parse_header(p)

    def test_too_small_file_raises(self, tmp_path):
        p = tmp_path / "small.png"
        p.write_bytes(b"\x89PNG")
        with pytest.raises(ValueError, match="文件过小"):
            ImageProcessor()._parse_header(p)

    def test_unsupported_suffix_raises(self, tmp_path):
        p = tmp_path / "a.webp"
        p.write_bytes(b"RIFF")
        with pytest.raises(ValueError, match="不支持"):
            ImageProcessor()._parse_header(p)

    def test_process_image_missing_file(self, tmp_path):
        info = ImageProcessor().process_image(str(tmp_path / "nope.png"))
        assert info.valid is False
        assert "文件不存在" in info.error

    def test_process_image_unsupported_ext(self, tmp_path):
        p = tmp_path / "a.txt"
        p.write_text("not an image", encoding="utf-8")
        info = ImageProcessor().process_image(str(p))
        assert info.valid is False
        assert "不支持" in info.error

    def test_batch_process_mixed(self, tmp_path):
        good = tmp_path / "ok.png"
        good.write_bytes(_make_png(10, 10))
        bad = tmp_path / "ok.txt"
        bad.write_text("x", encoding="utf-8")
        processor = ImageProcessor()
        results = processor.batch_process([str(good), str(bad)])
        assert results[0].valid is True
        assert results[1].valid is False


class TestAudioProcessing:
    def test_wav_standard_library_fallback(self, tmp_path):
        """无 soundfile 时 .wav 需经标准库解析出时长/采样率"""
        p = tmp_path / "a.wav"
        with wave.open(str(p), "wb") as w:
            w.setnchannels(1)
            w.setsampwidth(2)
            w.setframerate(8000)
            w.writeframes(b"\x00\x00" * 8000)  # 1 秒
        processor = AudioProcessor()
        info = processor.process_audio(str(p))
        assert info.valid is True
        assert info.duration == pytest.approx(1.0, abs=0.1)
        assert info.sample_rate == 8000

    def test_non_wav_basic_info_fallback(self, tmp_path):
        """无 soundfile 时非 wav 仅给基础信息且时长为 0"""
        p = tmp_path / "a.mp3"
        p.write_bytes(b"ID3" + b"\x00" * 30)
        processor = AudioProcessor()
        info = processor.process_audio(str(p))
        assert info.valid is True
        assert info.duration == 0.0

    def test_missing_audio_file(self, tmp_path):
        processor = AudioProcessor()
        info = processor.process_audio(str(tmp_path / "nope.wav"))
        assert info.valid is False
        assert "不存在" in info.error or "not found" in info.error.lower()

    def test_unsupported_audio_extension(self, tmp_path):
        p = tmp_path / "a.mid"
        p.write_text("MThd", encoding="utf-8")
        processor = AudioProcessor()
        info = processor.process_audio(str(p))
        assert info.valid is False

    def test_batch_process(self, tmp_path):
        p1 = tmp_path / "a.wav"
        with wave.open(str(p1), "wb") as w:
            w.setnchannels(1)
            w.setsampwidth(2)
            w.setframerate(8000)
            w.writeframes(b"\x00\x00" * 800)
        processor = AudioProcessor()
        results = processor.batch_process([str(p1), str(tmp_path / "b.mid")])
        assert results[0].valid is True
        assert results[1].valid is False
        # to_dict 可序列化
        assert "duration" in results[0].to_dict()

    def test_is_supported(self):
        processor = AudioProcessor()
        assert processor.is_supported("a.wav") is True
        assert processor.is_supported("a.mid") is False
