"""第63轮: checkpoint检查点管理"""
import pytest
from augmentor.checkpoint import CheckpointManager


class TestCheckpointManager:
    def test_create(self):
        cm = CheckpointManager()
        assert cm is not None

    def test_has_core_methods(self):
        cm = CheckpointManager()
        assert hasattr(cm, 'save_checkpoint')
        assert hasattr(cm, 'load_checkpoint')
        assert hasattr(cm, 'list_checkpoints')
        assert hasattr(cm, 'create_checkpoint')
        assert hasattr(cm, 'delete_checkpoint')

    def test_has_progress_methods(self):
        cm = CheckpointManager()
        assert hasattr(cm, 'get_progress')
        assert hasattr(cm, 'update_progress')
        assert hasattr(cm, 'get_remaining_indices')
