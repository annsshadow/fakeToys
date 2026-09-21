"""提示词模板管理模块

为数据增强等场景提供可复用的提示词模板：变量替换、缺省占位符、
安全渲染（避免 format 注入），以及简单的模板注册表。
"""

import logging
from dataclasses import dataclass, field
from typing import Any, Dict, List, Optional

logger = logging.getLogger(__name__)


class TemplateError(Exception):
    """模板错误"""


@dataclass
class PromptTemplate:
    """提示词模板"""

    name: str
    text: str
    variables: List[str] = field(default_factory=list)

    def render(self, **kwargs) -> str:
        """渲染模板

        使用显式占位符 {var} 替换；未提供的占位符与字面花括号保持原样。

        Args:
            **kwargs: 变量值

        Returns:
            渲染后的文本

        Raises:
            TemplateError: 声明了 variables 但缺少必需变量
        """
        if self.variables:
            missing = [v for v in self.variables if v not in kwargs]
            if missing:
                raise TemplateError(f"缺少模板变量: {missing}")
        return self._safe_substitute(self.text, kwargs)

    @staticmethod
    def _safe_substitute(text: str, values: Dict[str, Any]) -> str:
        """替换 {key} 占位符；仅替换精确匹配的键，字面花括号不受影响

        Args:
            text: 模板文本
            values: 变量映射

        Returns:
            替换后的文本
        """
        result = text
        for key, value in values.items():
            result = result.replace("{%s}" % key, str(value))
        return result

    def with_variables(self, names: List[str]) -> "PromptTemplate":
        """返回声明了指定变量的新模板（不修改原对象）

        Args:
            names: 变量名列表

        Returns:
            新 PromptTemplate
        """
        return PromptTemplate(name=self.name, text=self.text, variables=list(names))


class PromptRegistry:
    """提示词模板注册表"""

    def __init__(self):
        self._templates: Dict[str, PromptTemplate] = {}

    def register(self, template: PromptTemplate):
        """注册模板

        Args:
            template: 模板实例
        """
        if template.name in self._templates:
            logger.warning("模板 %s 已存在，将被覆盖", template.name)
        self._templates[template.name] = template

    def get(self, name: str) -> Optional[PromptTemplate]:
        """按名获取模板

        Args:
            name: 模板名

        Returns:
            模板或 None
        """
        return self._templates.get(name)

    def names(self) -> List[str]:
        """列出所有模板名

        Returns:
            模板名列表（排序）
        """
        return sorted(self._templates.keys())

    def render(self, name: str, **kwargs) -> str:
        """按名渲染模板

        Args:
            name: 模板名
            **kwargs: 变量

        Returns:
            渲染文本

        Raises:
            TemplateError: 模板不存在
        """
        template = self._templates.get(name)
        if template is None:
            raise TemplateError(f"模板不存在: {name}")
        return template.render(**kwargs)


def create_template(name: str, text: str, variables: Optional[List[str]] = None) -> PromptTemplate:
    """创建模板（便捷函数）

    Args:
        name: 模板名
        text: 模板文本
        variables: 变量名列表（可选）

    Returns:
        PromptTemplate 实例
    """
    return PromptTemplate(name=name, text=text, variables=variables or [])
