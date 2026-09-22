# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""CLI `pipeline` 组命令（由 cli.py 拆出，逻辑未改）"""

from ..io import _print
from augmentor import AugmentorPipeline


# ============ 增强 ============
def run_augment(args, config):
    """`augment` 子命令"""
    pipeline = AugmentorPipeline(config)
    result = pipeline.augment_dataset(
        args.input,
        args.output,
        use_checkpoint=not args.no_checkpoint,
        use_quality_check=not args.no_quality,
        use_dedup=not args.no_dedup
    )
    _print(result)


# ============ 流式处理 ============
def run_stream(args, config):
    """`stream` 子命令"""
    from augmentor.streaming import StreamAugmentor
    from augmentor.quality import QualityScorer
    from augmentor.dedup import Deduplicator

    def get_processor(operation: str):
        if operation == "quality":
            scorer = QualityScorer(threshold=config.quality.threshold)
            def process(items):
                scoring_items = [
                    {
                        "original": item.get("instruction", ""),
                        "generated": item.get("instruction", ""),
                        "output": item.get("output", "")
                    }
                    for item in items
                ]
                scores = scorer.batch_score(scoring_items)
                return [
                    item for item, score in zip(items, scores) if score.passed
                ]
            return process
        elif operation == "dedup":
            deduplicator = Deduplicator(threshold=config.dedup.threshold)
            return lambda items: deduplicator.deduplicate_and_filter(items)
        elif operation == "clean":
            from augmentor.data import DataCleaner
            cleaner = DataCleaner()
            def process(items):
                result = cleaner.clean(items)
                return result.items
            return process
        else:
            return lambda items: items

    processor = get_processor(args.operation)
    augmentor = StreamAugmentor(
        input_file=args.input,
        output_file=args.output,
        processor=processor,
        chunk_size=args.chunk_size
    )
    result = augmentor.augment()
    _print(result)
