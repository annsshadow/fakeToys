# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""数据分布可视化模块

分工边界（与 `augmentor.visualize_enhanced`）
    本模块的 `DataVisualizer` 由 `AugmentorPipeline` 使用
    （`pipeline.visualize_dataset`）。
    直接对一份数据出图、或用 `VisualizationConfig` 配置样式，用 `visualize_enhanced`。
"""

import json
import logging
from typing import List, Dict, Optional
from pathlib import Path
from collections import Counter
import re

logger = logging.getLogger(__name__)

# 中文词串切分。编译一次放在模块级：`re.findall(字面模式, text)`
# 每次都要走一遍 `re._compile()` 的缓存查找，而 `generate_statistics` 的逐条
# 循环对每条记录各调一次（真实语料 6902 条实测 6902 次）。
# 本模式是 `\u4e00-\u9fa5`，与 `statistics` / `analytics` 的 `\u4e00-\u9fff`
# 不是同一串，合并会改变词频口径（`龥` 与 `鿿` 之间的
# 字归属不同），故各留一份常量，不共用。
_CJK_WORD_PATTERN = re.compile(r'[\u4e00-\u9fa5]+')


class DataVisualizer:
    """数据可视化器"""
    
    def __init__(self, output_dir: str = "visualizations"):
        """初始化可视化器
        
        Args:
            output_dir: 输出目录；构造时不碰盘，首次出图才建
        """
        self.output_dir = Path(output_dir)
        
        self._wordcloud = None
        self._matplotlib = None
    
    def _load_wordcloud(self):
        """延迟加载 wordcloud"""
        if self._wordcloud is None:
            try:
                from wordcloud import WordCloud
                self._wordcloud = WordCloud
                logger.info("加载 wordcloud 成功")
            except ImportError:
                logger.warning("wordcloud 未安装，跳过词云生成")
    
    def _load_matplotlib(self):
        """延迟加载 matplotlib"""
        if self._matplotlib is None:
            try:
                import matplotlib
                matplotlib.use('Agg')  # 非交互式后端
                import matplotlib.pyplot as plt
                self._matplotlib = plt
                logger.info("加载 matplotlib 成功")
            except ImportError:
                logger.warning("matplotlib 未安装，跳过图表生成")
    
    def _output_path(self, output_file: str) -> Path:
        """出图路径，并按需建目录
        
        建目录排在这里而不是构造函数：`matplotlib.savefig` 不会自己建父目录，所以真写图
        时必须有它，而「只是 new 一个可视化器」不该在盘上留下东西 —— 管道构造时会带上这
        一族对象，用户没跑过任何一图，工作目录就已经多了三个空目录。
        """
        self.output_dir.mkdir(parents=True, exist_ok=True)
        return self.output_dir / output_file
    
    def generate_wordcloud(self,
                          items: List[Dict],
                          text_key: str = "instruction",
                          output_file: str = "wordcloud.png") -> str:
        """生成词云图
        
        Args:
            items: 数据列表
            text_key: 文本字段名
            output_file: 输出文件名
        
        Returns:
            输出文件路径
        """
        self._load_wordcloud()
        
        if self._wordcloud is None:
            logger.warning("wordcloud 不可用，跳过词云生成")
            return ""
        
        # 合并所有文本
        text = " ".join([item.get(text_key, "") for item in items])
        
        # 生成词云
        wc = self._wordcloud(
            font_path="C:/Windows/Fonts/simhei.ttf",  # 中文字体
            width=800,
            height=400,
            background_color="white"
        )
        wc.generate(text)
        
        # 保存图片
        output_path = self._output_path(output_file)
        wc.to_file(str(output_path))
        
        logger.info(f"生成词云图: {output_path}")
        return str(output_path)
    
    def generate_length_distribution(self,
                                    items: List[Dict],
                                    text_key: str = "instruction",
                                    output_file: str = "length_distribution.png") -> str:
        """生成长度分布图
        
        Args:
            items: 数据列表
            text_key: 文本字段名
            output_file: 输出文件名
        
        Returns:
            输出文件路径
        """
        self._load_matplotlib()
        
        if self._matplotlib is None:
            logger.warning("matplotlib 不可用，跳过图表生成")
            return ""
        
        # 计算长度
        lengths = [len(item.get(text_key, "")) for item in items]
        
        # 创建直方图
        fig, ax = self._matplotlib.subplots()
        ax.hist(lengths, bins=30, edgecolor='black')
        ax.set_xlabel('文本长度')
        ax.set_ylabel('频次')
        ax.set_title('文本长度分布')
        
        # 保存图片
        output_path = self._output_path(output_file)
        self._matplotlib.savefig(str(output_path), dpi=150, bbox_inches='tight')
        self._matplotlib.close(fig)
        
        logger.info(f"生成长度分布图: {output_path}")
        return str(output_path)
    
    def generate_topic_cluster(self,
                              items: List[Dict],
                              text_key: str = "instruction",
                              output_file: str = "topic_cluster.png") -> str:
        """生成话题聚类图
        
        Args:
            items: 数据列表
            text_key: 文本字段名
            output_file: 输出文件名
        
        Returns:
            输出文件路径
        """
        self._load_matplotlib()
        
        if self._matplotlib is None:
            logger.warning("matplotlib 不可用，跳过图表生成")
            return ""
        
        try:
            from sklearn.feature_extraction.text import TfidfVectorizer
            from sklearn.manifold import TSNE
            import numpy as np
            
            # 提取文本
            texts = [item.get(text_key, "") for item in items]
            
            # TF-IDF 向量化
            vectorizer = TfidfVectorizer(max_features=1000)
            X = vectorizer.fit_transform(texts)
            
            # t-SNE 降维
            tsne = TSNE(n_components=2, random_state=42, perplexity=min(30, len(texts) - 1))
            X_tsne = tsne.fit_transform(X.toarray())
            
            # 绘制散点图
            fig, ax = self._matplotlib.subplots()
            scatter = ax.scatter(X_tsne[:, 0], X_tsne[:, 1], c='blue', alpha=0.6, s=10)
            ax.set_title('话题聚类图 (t-SNE)')
            ax.set_xlabel('t-SNE 维度 1')
            ax.set_ylabel('t-SNE 维度 2')
            
            # 保存图片
            output_path = self._output_path(output_file)
            self._matplotlib.savefig(str(output_path), dpi=150, bbox_inches='tight')
            self._matplotlib.close(fig)
            
            logger.info(f"生成话题聚类图: {output_path}")
            return str(output_path)
            
        except ImportError:
            logger.warning("sklearn 不可用，跳过话题聚类图生成")
            return ""
    
    def generate_quality_distribution(self,
                                     scores: List[float],
                                     output_file: str = "quality_distribution.png") -> str:
        """生成质量评分分布图
        
        Args:
            scores: 质量评分列表
            output_file: 输出文件名
        
        Returns:
            输出文件路径
        """
        self._load_matplotlib()
        
        if self._matplotlib is None:
            logger.warning("matplotlib 不可用，跳过图表生成")
            return ""
        
        # 创建直方图
        fig, ax = self._matplotlib.subplots()
        ax.hist(scores, bins=20, edgecolor='black', color='green')
        ax.set_xlabel('质量评分')
        ax.set_ylabel('频次')
        ax.set_title('质量评分分布')
        ax.axvline(x=0.6, color='red', linestyle='--', label='阈值 (0.6)')
        ax.legend()
        
        # 保存图片
        output_path = self._output_path(output_file)
        self._matplotlib.savefig(str(output_path), dpi=150, bbox_inches='tight')
        self._matplotlib.close(fig)
        
        logger.info(f"生成质量评分分布图: {output_path}")
        return str(output_path)
    
    def generate_timeline(self,
                         progress_data: List[Dict],
                         output_file: str = "timeline.png") -> str:
        """生成时间线图
        
        Args:
            progress_data: 进度数据列表
            output_file: 输出文件名
        
        Returns:
            输出文件路径
        """
        self._load_matplotlib()
        
        if self._matplotlib is None:
            logger.warning("matplotlib 不可用，跳过图表生成")
            return ""
        
        if not progress_data:
            return ""
        
        # 提取数据
        timestamps = [p.get("timestamp", "") for p in progress_data]
        counts = [p.get("count", 0) for p in progress_data]
        
        # 创建折线图
        fig, ax = self._matplotlib.subplots()
        ax.plot(range(len(timestamps)), counts, marker='o')
        ax.set_xlabel('时间点')
        ax.set_ylabel('数据条数')
        ax.set_title('数据增强进度')
        ax.set_xticks(range(len(timestamps)))
        ax.set_xticklabels(timestamps, rotation=45, ha='right')
        
        # 保存图片
        output_path = self._output_path(output_file)
        self._matplotlib.savefig(str(output_path), dpi=150, bbox_inches='tight')
        self._matplotlib.close(fig)
        
        logger.info(f"生成时间线图: {output_path}")
        return str(output_path)
    
    def generate_all_visualizations(self,
                                   items: List[Dict],
                                   text_key: str = "instruction",
                                   scores: Optional[List[float]] = None) -> Dict[str, str]:
        """生成所有可视化图表
        
        Args:
            items: 数据列表
            text_key: 文本字段名
            scores: 质量评分列表
        
        Returns:
            图表文件路径字典
        """
        results = {}
        
        # 词云图
        wordcloud_path = self.generate_wordcloud(items, text_key)
        if wordcloud_path:
            results["wordcloud"] = wordcloud_path
        
        # 长度分布图
        length_dist_path = self.generate_length_distribution(items, text_key)
        if length_dist_path:
            results["length_distribution"] = length_dist_path
        
        # 话题聚类图
        topic_cluster_path = self.generate_topic_cluster(items, text_key)
        if topic_cluster_path:
            results["topic_cluster"] = topic_cluster_path
        
        # 质量评分分布图
        if scores:
            quality_dist_path = self.generate_quality_distribution(scores)
            if quality_dist_path:
                results["quality_distribution"] = quality_dist_path
        
        return results
    
    def generate_statistics(self, items: List[Dict], text_key: str = "instruction") -> Dict:
        """生成统计信息

        委托给模块级 `generate_statistics`：实现只保留一份，实例调用与函数调用
        不会给出不同结果。

        Args:
            items: 数据列表
            text_key: 文本字段名

        Returns:
            统计信息字典
        """
        return generate_statistics(items, text_key)


def generate_statistics(items: List[Dict], text_key: str = "instruction") -> Dict:
    """生成统计信息（模块级便捷函数）

    等价于 `DataVisualizer.generate_statistics`，但**不会构造可视化器实例**——
    `DataVisualizer.__init__` 会 `mkdir` 输出目录，只想拿统计数字的调用方不该
    因此凭空多出一个 `visualizations/` 目录。

    本函数在 `__all__` 里早已声明，此前却只存在于类上，`from augmentor.visualizer
    import *` 会直接 `AttributeError`；这里把它补齐。

    Args:
        items: 数据列表
        text_key: 文本字段名

    Returns:
        统计信息字典
    """
    texts = [item.get(text_key, "") for item in items]
    lengths = [len(t) for t in texts]

    # 词频统计
    all_words = []
    for text in texts:
        words = _CJK_WORD_PATTERN.findall(text)
        all_words.extend(words)

    word_freq = Counter(all_words)

    return {
        "total_items": len(items),
        "avg_length": sum(lengths) / len(lengths) if lengths else 0,
        "min_length": min(lengths) if lengths else 0,
        "max_length": max(lengths) if lengths else 0,
        "unique_words": len(word_freq),
        "top_words": dict(word_freq.most_common(10))
    }


__all__ = [
    "DataVisualizer",
    "DataVisualizationConfig",
    "generate_statistics",
]