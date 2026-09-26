# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""微型分支收尾 L61：data 子包 + csv_excel_import + analytics + audit

- data/cleaner: detect_language 四种文档词表值
- data/audio: soundfile 存在时的解析路径（注入假 soundfile 模块）
- data/multimodal: 目录扫描跳过子目录
- csv_excel_import: pandas 缺失降级（读边 + CSV 写边）；Excel 写边自 L85 起只看
  openpyxl，所以它的缺依赖降级单列一族
- analytics: 长问题洞察分支
- audit: 空数据集判定
"""

import importlib
import importlib.util
import sys
from pathlib import Path
from types import ModuleType, SimpleNamespace

import pytest

from augmentor.analytics import DatasetAnalyzer
from augmentor.audit import audit_dataset
from augmentor.csv_excel_import import export_to_excel
from augmentor.data.audio import AudioProcessor
from augmentor.data.cleaner import DataCleaner
from augmentor.data.multimodal import MultimodalProcessor


class TestDataCleanerLanguage:
    def test_vocabulary_values(self):
        detector = DataCleaner()
        assert detector.detect_language("你好世界") == "zh"
        assert detector.detect_language("hello world") == "en"
        assert detector.detect_language("你好 hello") == "mixed"
        assert detector.detect_language("12345") == "unknown"
        assert detector.detect_language("") == "unknown"


class TestAudioSoundfilePath:
    def test_parse_with_fake_soundfile(self, monkeypatch):
        fake_sf = ModuleType("soundfile")
        fake_sf.info = lambda p: SimpleNamespace(
            format="WAV", duration=2.5, samplerate=16000,
            channels=1, subtype="PCM_16",
        )
        monkeypatch.setitem(sys.modules, "soundfile", fake_sf)
        result = AudioProcessor()._parse_with_soundfile(Path("a.wav"))
        assert result == {
            "format": "WAV",
            "duration": 2.5,
            "sample_rate": 16000,
            "channels": 1,
            "metadata": {"subtype": "PCM_16", "parser": "soundfile"},
        }


class TestMultimodalDirectoryScan:
    def test_subdirectories_skipped(self, tmp_path):
        target = tmp_path / "mm"
        (target / "sub").mkdir(parents=True)
        assert MultimodalProcessor().process_directory(str(target)) == []


def _pandas_installed() -> bool:
    """真实判断 pandas 是否可导入（不看注入的假模块）。"""
    return importlib.util.find_spec("pandas") is not None


class TestPandasMissingFallback:
    def test_has_pandas_false_degrades(self, monkeypatch):
        import augmentor.csv_excel_import as cei

        # 保存真实 pandas，供 finally 恢复；CI 环境（requirements-dev.txt
        # 不含 pandas）本身就没有它，此时该用例仍在验证「缺失即降级」。
        real_pandas = sys.modules.get("pandas")
        was_installed = _pandas_installed()

        monkeypatch.setitem(sys.modules, "pandas", None)
        reloaded = importlib.reload(cei)
        try:
            assert reloaded.HAS_PANDAS is False
            with pytest.raises(ImportError):
                reloaded.import_from_csv("x.csv")
            with pytest.raises(ImportError):
                reloaded.export_to_csv([], "o.csv")
        finally:
            # reload 是原地改全局：必须先恢复 pandas 再 reload，
            # 否则模块留在无 pd 的破损状态污染后续用例
            if real_pandas is not None:
                sys.modules["pandas"] = real_pandas
            else:
                sys.modules.pop("pandas", None)
            importlib.reload(cei)
        # 降级态必须与环境的真实安装情况一致——不能无条件断言 True，
        # 否则在未安装 pandas 的 CI 上必然失败（该断言只在装了的机器上成立）。
        assert cei.HAS_PANDAS is was_installed
        # 注意：`export_to_excel` **不在**这一族里。L85 起它是转接口，实现住在
        # `excel_write`，只看 `HAS_OPENPYXL`，与本模块的 pandas 标志位无关（下面
        # `TestOpenpyxlMissingFallback` 守那一条）。把它们混在一族会造出假守卫。

    def test_export_excel_missing_column_raises(self, tmp_path):
        """缺列校验不依赖 pandas：转接口那一刀在 `excel_columns` 里，两侧解释器都能到"""
        with pytest.raises(ValueError, match="以下列不存在"):
            export_to_excel(
                [{"a": 1}], str(tmp_path / "o.xlsx"), columns=["nope"]
            )


class TestOpenpyxlMissingFallback:
    def test_missing_openpyxl_degrades_before_writing_anything(self, tmp_path, monkeypatch):
        """缺 openpyxl 时写 Excel 要报「装依赖」，且不许留下半成品

        这里用 `monkeypatch.setattr` 改标志位，而不是沿用上一族 reload 模块的写法：
        写边对 `Path("o.xlsx")` 这类相对路径会**真写当前工作目录**，一旦门没拦住就往
        仓库里落文件（L85 之前那两条老用例正是靠相对路径蒙过的 —— 它们绿，但绿得没底）。
        """
        import augmentor.excel_write as excel_write

        out = tmp_path / "o.xlsx"
        monkeypatch.setattr(excel_write, "HAS_OPENPYXL", False)
        with pytest.raises(ImportError, match="openpyxl"):
            excel_write.export_table_to_excel([{"a": 1}], str(out))
        assert not out.exists(), "报错前落了半成品"


class TestAnalyticsLongQuestion:
    def test_long_instruction_insight(self):
        items = [
            {"instruction": "长" * 210, "output": "答" * 210}
            for _ in range(3)
        ]
        report = DatasetAnalyzer(items).analyze()
        assert any("过长" in i.title for i in report.insights)


class TestAuditEmptyDataset:
    def test_empty_items_not_ready(self):
        report = audit_dataset([])
        assert report.total_items == 0
        assert report.ready is False
