"""Config validator module"""

import logging
from typing import Dict, Optional
from pathlib import Path

logger = logging.getLogger(__name__)


class ValidationResult:
    """Validation result"""
    is_valid: bool = True
    errors: list = None

    def __init__(self, is_valid: bool = True):
        self.is_valid = is_valid
        self.errors = []


ValidationSeverity = ValidationResult
Severity = ValidationResult


class ConfigValidator:
    def __init__(self):
        pass

    def validate(self, config_path: str) -> bool:
        path = Path(config_path)
        return path.exists()


def validate_config(config: Dict) -> ValidationResult:
    result = ValidationResult()
    if "models" in config:
        result.is_valid = True
    return result


def validate_config_file(config_path: str) -> Dict:
    validator = ConfigValidator()
    is_valid = validator.validate(config_path)
    return {"valid": is_valid, "path": config_path}
