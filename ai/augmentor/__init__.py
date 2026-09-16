"""AI 训练数据增强工具包"""

__version__ = "2.0.0"

from .config import AppConfig, load_config
from .models import create_model_backend, ModelBackend
from .pipeline import AugmentorPipeline
from .quality import QualityScorer
from .dedup import Deduplicator
from .export import Exporter
from .preview import PreviewGenerator, ExportPreview
from .report import ReportGenerator, QualityReport
from .context import ContextAugmentor
from .checkpoint import CheckpointManager
from .versioning import VersionManager
from .sampler import ActiveSampler
from .active_learning import ActiveLearningLoop
from .expander import DomainExpander
from .tracker import ExperimentTracker
from .visualizer import DataVisualizer
from .benchmark import QualityBenchmark
from .multilingual import MultilingualSupport
from .rag import RAGFormatter
from .evaluation import ModelEvaluator

__all__ = [
    "AppConfig",
    "load_config",
    "create_model_backend",
    "ModelBackend",
    "AugmentorPipeline",
    "QualityScorer",
    "Deduplicator",
    "Exporter",
    "PreviewGenerator",
    "ExportPreview",
    "ReportGenerator",
    "QualityReport",
    "ContextAugmentor",
    "CheckpointManager",
    "VersionManager",
    "ActiveSampler",
    "ActiveLearningLoop",
    "DomainExpander",
    "ExperimentTracker",
    "DataVisualizer",
    "QualityBenchmark",
    "MultilingualSupport",
    "RAGFormatter",
    "ModelEvaluator"
]
