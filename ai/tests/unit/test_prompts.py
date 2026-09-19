"""prompts 提示词模板模块测试

覆盖变量替换、缺变量报错、字面花括号保持、注册表渲染与包级导出。
"""

import pytest

from augmentor import (
    PromptRegistry,
    PromptTemplate,
    TemplateError,
    create_template,
)


class TestPromptTemplate:
    def test_render_substitutes_variables(self):
        tpl = create_template("t", "你好 {name}，你的编号是 {id}")
        assert tpl.render(name="小明", id=7) == "你好 小明，你的编号是 7"

    def test_missing_variable_raises_when_declared(self):
        tpl = create_template("t", "Hi {name}", variables=["name"])
        with pytest.raises(TemplateError, match="缺少模板变量"):
            tpl.render()

    def test_undeclared_render_no_strict_check(self):
        tpl = create_template("t", "Hi {name}")
        # 未声明 variables 时不强制校验
        assert tpl.render(name="X") == "Hi X"

    def test_literal_braces_preserved(self):
        tpl = create_template("t", "值 {a} 与字面 {braces} 保持")
        out = tpl.render(a=1)
        assert "{braces}" in out
        assert "值 1" in out

    def test_value_containing_placeholder_not_expanded(self):
        tpl = create_template("t", "前缀 {x}")
        out = tpl.render(x="{y}")
        # 值本身含 {y} 不应二次展开
        assert out == "前缀 {y}"

    def test_with_variables_returns_new_instance(self):
        base = create_template("t", "Q {q}")
        derived = base.with_variables(["q"])
        assert derived is not base
        assert derived.variables == ["q"]
        assert base.variables == []


class TestPromptRegistry:
    def test_register_get_render(self):
        reg = PromptRegistry()
        reg.register(create_template("greet", "Hello {n}", variables=["n"]))
        assert reg.get("greet") is not None
        assert reg.render("greet", n="Ada") == "Hello Ada"
        assert reg.names() == ["greet"]

    def test_render_missing_template_raises(self):
        reg = PromptRegistry()
        with pytest.raises(TemplateError, match="模板不存在"):
            reg.render("nope", n="x")

    def test_get_missing_returns_none(self):
        reg = PromptRegistry()
        assert reg.get("ghost") is None

    def test_register_overrides_same_name(self):
        reg = PromptRegistry()
        reg.register(create_template("a", "old"))
        reg.register(create_template("a", "new"))
        assert reg.get("a").text == "new"

    def test_names_sorted(self):
        reg = PromptRegistry()
        reg.register(create_template("z", "z"))
        reg.register(create_template("a", "a"))
        reg.register(create_template("m", "m"))
        assert reg.names() == ["a", "m", "z"]


class TestPackageExports:
    def test_exports(self):
        import augmentor

        assert augmentor.PromptTemplate is PromptTemplate
        assert augmentor.PromptRegistry is PromptRegistry
        assert augmentor.TemplateError is TemplateError
        assert callable(augmentor.create_template)
