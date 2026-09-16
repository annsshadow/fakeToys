"""多语言支持模块

提供语言检测、翻译增强与中英混合处理能力。
"""

import logging
import re
from concurrent.futures import ThreadPoolExecutor, as_completed
from typing import List, Dict, Optional, Any

logger = logging.getLogger(__name__)

CJK_PATTERN = re.compile(r"[\u4e00-\u9fff]")
LATIN_PATTERN = re.compile(r"[A-Za-z]")

# 语言代码到自然语言名称
LANG_NAMES = {
    "zh": "中文",
    "en": "英文",
    "ja": "日文",
    "ko": "韩文",
    "fr": "法文",
    "de": "德文",
    "es": "西班牙文"
}

TRANSLATE_PROMPT_TEMPLATE = (
    "请将下面的内容翻译为{target_lang}，保持原意、语气和格式，"
    "只输出翻译结果，不要任何解释或引号。\n\n{text}"
)


class MultilingualSupport:
    """多语言支持"""

    def __init__(self,
                 model_backend: Optional[Any] = None,
                 supported_langs: Optional[List[str]] = None,
                 max_workers: int = 4):
        """初始化多语言支持

        Args:
            model_backend: 模型后端，用于翻译；为 None 时只能做语言检测
            supported_langs: 支持的语言列表
            max_workers: 批量翻译并发数
        """
        self.model_backend = model_backend
        self.supported_langs = supported_langs or ["zh", "en"]
        self.max_workers = max_workers

    def detect_language(self, text: str) -> str:
        """检测文本语言

        Args:
            text: 待检测文本

        Returns:
            zh / en / mixed / unknown
        """
        if not isinstance(text, str) or not text:
            return "unknown"

        cjk_count = len(CJK_PATTERN.findall(text))
        latin_count = len(LATIN_PATTERN.findall(text))
        total = cjk_count + latin_count

        if total == 0:
            return "unknown"

        cjk_ratio = cjk_count / total
        latin_ratio = latin_count / total

        if cjk_ratio >= 0.8:
            return "zh"
        if latin_ratio >= 0.8:
            return "en"
        if cjk_ratio > 0.1 and latin_ratio > 0.1:
            return "mixed"
        return "other"

    def is_mixed(self, text: str) -> bool:
        """判断文本是否为中英混合

        Args:
            text: 文本

        Returns:
            是否为混合文本
        """
        return self.detect_language(text) == "mixed"

    def _lang_name(self, lang: str) -> str:
        """获取语言的自然语言名称

        Args:
            lang: 语言代码

        Returns:
            语言名称
        """
        return LANG_NAMES.get(lang, lang)

    def translate(self, text: str, target_lang: str = "en") -> str:
        """翻译文本

        Args:
            text: 待翻译文本
            target_lang: 目标语言代码

        Returns:
            翻译结果

        Raises:
            ValueError: 目标语言不受支持
            RuntimeError: 未配置模型后端或翻译失败
        """
        if target_lang not in self.supported_langs:
            raise ValueError(
                f"不支持的目标语言: {target_lang}。支持: {self.supported_langs}"
            )

        if self.model_backend is None:
            raise RuntimeError("翻译需要配置模型后端")

        if not isinstance(text, str) or not text.strip():
            return ""

        source_lang = self.detect_language(text)
        if source_lang == target_lang:
            return text

        prompt = TRANSLATE_PROMPT_TEMPLATE.format(
            target_lang=self._lang_name(target_lang),
            text=text
        )
        return self.model_backend.generate(prompt).strip()

    def batch_translate(self,
                        items: List[Dict],
                        target_lang: str = "en",
                        text_key: str = "instruction",
                        keep_original: bool = True) -> List[Dict]:
        """批量翻译数据

        Args:
            items: 数据列表
            target_lang: 目标语言代码
            text_key: 需要翻译的字段
            keep_original: 是否保留原文字段

        Returns:
            翻译后的数据列表
        """
        if target_lang not in self.supported_langs:
            raise ValueError(
                f"不支持的目标语言: {target_lang}。支持: {self.supported_langs}"
            )

        if not items:
            return []

        results: List[Optional[Dict]] = [None] * len(items)

        def translate_item(index: int, item: Dict) -> tuple:
            translated_text = self.translate(item.get(text_key, ""), target_lang)
            new_item = dict(item)
            if keep_original:
                new_item[f"{text_key}_original"] = item.get(text_key, "")
            new_item[text_key] = translated_text
            new_item["language"] = target_lang
            return index, new_item

        with ThreadPoolExecutor(max_workers=self.max_workers) as executor:
            futures = [
                executor.submit(translate_item, i, item)
                for i, item in enumerate(items)
            ]

            for future in as_completed(futures):
                try:
                    index, new_item = future.result()
                    results[index] = new_item
                except Exception as e:
                    logger.error(f"翻译失败: {e}")

        # 失败项回退为原始数据，保证长度一致
        return [
            results[i] if results[i] is not None else dict(items[i])
            for i in range(len(items))
        ]

    def analyze_languages(self, items: List[Dict], text_key: str = "instruction") -> Dict[str, Any]:
        """统计数据集的语言分布

        Args:
            items: 数据列表
            text_key: 文本字段

        Returns:
            语言分布报告
        """
        distribution: Dict[str, int] = {}
        for item in items:
            lang = self.detect_language(item.get(text_key, ""))
            distribution[lang] = distribution.get(lang, 0) + 1

        total = len(items) if items else 1
        return {
            "total_items": len(items),
            "distribution": distribution,
            "ratios": {k: v / total for k, v in distribution.items()}
        }
