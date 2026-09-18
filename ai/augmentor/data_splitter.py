"""数据集分割模块"""

from typing import List, Dict
from dataclasses import dataclass


@dataclass
class SplitResult:
    train: List[Dict]
    test: List[Dict]
    split_ratio: float = 0.8


class DataSplitter:
    def split_dataset(self, items: List[Dict], split_ratio: float = 0.8):
        split_idx = int(len(items) * split_ratio)
        return items[:split_idx], items[split_idx:]


def split_dataset(items: List[Dict], split_ratio: float = 0.8):
    return DataSplitter().split_dataset(items, split_ratio)
