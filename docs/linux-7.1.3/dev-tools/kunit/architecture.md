
# KUnit 架构


## KUnit 架构分为两个部分：

- `内核内测试框架`_ 
- `kunit_tool（命令行测试工具）`_

## 内核内测试框架


内核测试库支持使用 KUnit 以 C 语言编写的 KUnit 测试。这些 KUnit 测试是内核代码。KUnit 执行以下任务：

- 组织测试
- 报告测试结果
- 提供测试工具

## 测试用例


测试用例是 KUnit 中的基本单元。KUnit 测试用例被组织成测试套件（suite）。一个 KUnit 测试用例是一个类型为 `void (**)(struct kunit **test)` 的函数。这些测试用例函数被包装在一个名为 struct kunit_case 的结构体中。

	`generate_params` 对于非参数化测试是可选的。

每个 KUnit 测试用例都会接收一个 `struct kunit` 上下文对象，用于跟踪正在运行的测试。KUnit 断言宏和其他 KUnit 工具使用 `struct kunit` 上下文对象。作为一个例外，有两个字段：

- `->priv`：初始化（setup）函数可以用它来存储任意的测试用户数据。

- `->param_value`：它包含可以在参数化测试中检索到的参数值。

## 测试套件


一个 KUnit 套件包含一组测试用例。KUnit 套件由 `struct kunit_suite` 表示。例如：


	static struct kunit_case example_test_cases[] = {
		KUNIT_CASE(example_test_foo),
		KUNIT_CASE(example_test_bar),
		KUNIT_CASE(example_test_baz),
		{}
	};

	static struct kunit_suite example_test_suite = {
		.name = "example",
		.init = example_test_init,
		.exit = example_test_exit,
		.test_cases = example_test_cases,
	};
	kunit_test_suite(example_test_suite);

在上面的例子中，测试套件 `example_test_suite` 运行测试用例 `example_test_foo`、`example_test_bar` 和 `example_test_baz`。在运行测试之前，会调用 `example_test_init`，在运行测试之后，会调用 `example_test_exit`。`kunit_test_suite(example_test_suite)` 将该测试套件注册到 KUnit 测试框架中。

## 执行器（Executor）


KUnit 执行器可以在启动时列出并运行内置的 KUnit 测试。这些测试套件存储在一个名为 `.kunit_test_suites` 的链接器段（linker section）中。相关代码参见 `include/asm-generic/vmlinux.lds.h <https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/tree/include/asm-generic/vmlinux.lds.h?h=v6.0#n950>`_ 中的 `KUNIT_TABLE()` 宏定义。该链接器段由一个指向 `struct kunit_suite` 的指针数组组成，并由 `kunit_test_suites()` 宏填充。KUnit 执行器遍历该链接器段数组，以运行编译进内核的所有测试。

## :alt:	KUnit 套件内存

## KUnit 套件内存图

在内核启动时，KUnit 执行器使用该段的起始和结束地址来遍历并运行所有测试。有关执行器的实现，请参见 `lib/kunit/executor.c <https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/tree/lib/kunit/executor.c>`_。当以内核模块形式构建时，`kunit_test_suites()` 宏会定义一个 `module_init()` 函数，该函数运行编译单元中的所有测试，而不是使用执行器。

在 KUnit 测试中，某些错误类不会影响其他测试或内核的其他部分，每个 KUnit 用例在独立的线程上下文中执行。请参见 `lib/kunit/try-catch.c <https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/tree/lib/kunit/try-catch.c?h=v5.15#n58>`_ 中的 `kunit_try_catch_run()` 函数。

## 断言宏


KUnit 测试使用期望（expectation）/断言（assertion）来验证状态。所有期望/断言的格式为：
`KUNIT_{EXPECT|ASSERT}_<op>[_MSG](kunit, property[, message])`

- `{EXPECT|ASSERT}` 决定该检查是断言还是期望。在失败时，测试流程的区别如下：

 - 对于期望，测试被标记为失败，并记录该失败。

 - 另一方面，断言失败会导致测试用例立即终止。

  - 断言会调用函数：
		  `void __noreturn __kunit_abort(struct kunit *)`。

  - `__kunit_abort` 调用函数：
		  `void __noreturn kunit_try_catch_throw(struct kunit_try_catch *try_catch)`。

  - `kunit_try_catch_throw` 调用函数：
		  `void kthread_complete_and_exit(struct completion *, long) __noreturn;`
		  并终止该特殊线程上下文。

- `<op>` 表示带有以下选项的检查：`TRUE`（所提供的属性具有布尔值 “true”）、`EQ`（所提供的两个属性相等）、`NOT_ERR_OR_NULL`（所提供的指针不为空且不包含 “err” 值）。

- `[_MSG]` 在失败时打印自定义消息。

## 测试结果报告

KUnit 以 KTAP 格式打印测试结果。KTAP 基于 TAP14，参见 Documentation/dev-tools/ktap.rst。KTAP 可与 KUnit 和 Kselftest 配合使用。KUnit 执行器将 KTAP 结果打印到 dmesg 和 debugfs（如果已配置）。

## 参数化测试


每个 KUnit 参数化测试都关联一组参数。该测试会被多次调用，每个参数值调用一次，并且参数存储在 `param_value` 字段中。测试用例包含一个接受生成器函数的 KUNIT_CASE_PARAM() 宏。生成器函数接收前一个参数并返回下一个参数。它还包含一个用于生成基于数组的常见情况生成器的宏。

## kunit_tool（命令行测试工具）


`kunit_tool` 是一个 Python 脚本，位于 `tools/testing/kunit/kunit.py`。它用于配置、构建、执行、解析测试结果，并按正确顺序运行前面所有命令（即配置、构建、执行和解析）。运行 KUnit 测试有两种选择：要么构建一个启用了 KUnit 的内核并手动解析结果（参见 Documentation/dev-tools/kunit/run_manual.rst），要么使用 `kunit_tool`（参见 Documentation/dev-tools/kunit/run_wrapper.rst）。

- `configure` 命令从 `.kunitconfig` 文件（以及任何架构特定的选项）生成内核 `.config`。`qemu_configs` 文件夹中提供的 Python 脚本（例如 `tools/testing/kunit/qemu configs/powerpc.py`）包含特定架构的额外配置选项。它会解析现有的 `.config` 和 `.kunitconfig` 文件，以确保 `.config` 是 `.kunitconfig` 的超集。如果不是，它会将两者合并并运行 `make olddefconfig` 来重新生成 `.config` 文件。然后它检查 `.config` 是否已成为超集。这验证了所有 Kconfig 依赖项都在 `.kunitconfig` 文件中正确指定。`kunit_config.py` 脚本包含解析 Kconfig 的代码。运行 `make olddefconfig` 的代码属于 `kunit_kernel.py` 脚本的一部分。你可以通过以下命令调用此命令：`./tools/testing/kunit/kunit.py config`，并生成 `.config` 文件。
- `build` 在内核树上使用所需选项（取决于架构和某些选项，例如 build_dir）运行 `make`，并报告任何错误。要从当前的 `.config` 构建 KUnit 内核，你可以使用 `build` 参数：`./tools/testing/kunit/kunit.py build`。
- `exec` 命令直接（使用 User-mode Linux 配置）或通过 QEMU 等模拟器执行内核结果。它使用标准输出（stdout）从日志中读取结果，并将其传递给 `parse` 进行解析。如果你已经构建了一个带有内置 KUnit 测试的内核，可以使用 `exec` 参数运行内核并显示测试结果：`./tools/testing/kunit/kunit.py exec`。
- `parse` 从内核日志中提取 KTAP 输出，解析测试结果，并打印摘要。对于失败的测试，会包含任何诊断输出。
