
## 运行 KUnit 测试的提

## 使用 ``kunit.py run``kunit 工具"

### 从任意目录运

创建一个类似下面这样的 bash 函数会很方便

	function run_kunit() {
	  ( cd "$(git rev-parse --show-toplevel)" && ./tools/testing/kunit/kunit.py run "$@" )
	}

	`kunit.py` 的早期版本（5.6 之前）只有在从内核根目录运行时才工作，因此这里使用了shell `cd`
### 运行测试子集


`kunit.py run` 接受一个可选的 glob 参数来过滤测试。格式为 `"<suite_glob>[.test_glob]"`
假设我们想运sysctl 测试，可以这样：


	$ echo -e 'CONFIG_KUNIT=y\nCONFIG_KUNIT_ALL_TESTS=y' > .kunit/.kunitconfig
	$ ./tools/testing/kunit/kunit.py run 'sysctl*'

我们可以通过以下方式进一步过滤，只运write"测试

	$ echo -e 'CONFIG_KUNIT=y\nCONFIG_KUNIT_ALL_TESTS=y' > .kunit/.kunitconfig
	$ ./tools/testing/kunit/kunit.py run 'sysctl**.**write*'

以这种方式我们付出了构建多于所需测试的成本，但它比摆`.kunitconfig` 文件或注释掉 `kunit_suite` 要容易
不过，如果你想以不那么临时的方式来定义一组测试，下一条提示会很有用
### 定义一组测

`kunit.py run`（以`build` `config`）支持一`--kunitconfig` 标志。因此，如果你有一组想要定期运行的测试（尤其是它们还有其他依赖时），可以为它们创建一个特定的 `.kunitconfig`
例如，kunit 为其测试就有一个：


	$ ./tools/testing/kunit/kunit.py run --kunitconfig=lib/kunit/.kunitconfig

或者，如果你遵循将文件命名`.kunitconfig` 的约定，你可以只传入目录，例如：


	$ ./tools/testing/kunit/kunit.py run --kunitconfig=lib/kunit

	这是一个相对较新的特性（5.12+），因此关于哪些文件应当检入、哪些只保留在本地，我们还没有任何约定。一个配置是否有用到值得提交（并因此必须维护），由你和你的维护者决定
	在父目录和子目录中同时拥`.kunitconfig` 片段是成问题的。有人在讨论在这些文件中添加一import"语句，以便让顶层配置能够运行来自所有子目录的测试。但那将意味着 `.kunitconfig` 文件不再是简单的 .config 片段
	另一种替代方案是kunit 工具自动递归合并配置，但测试在理论上可能依赖于不兼容的选项，因此处理起来会很棘手
### 设置内核命令行参

你可以使`--kernel_args` 来传递任意内核参数，例如

	$ ./tools/testing/kunit/kunit.py run --kernel_args=param=42 --kernel_args=param2=false


### UML 下生成代码覆盖率报告


	TODO(brendanhiggins@google.com): UML gcc 7 及更高版本存在各种问题。你很可能会遇到缺失`.gcda` 文件或编译错误
这与 Documentation/dev-tools/gcov.rst 中记录的获取覆盖率信息的"常规"方式不同
我们可以不启`CONFIG_GCOV_KERNEL=y`，而是设置这些选项

	CONFIG_DEBUG_KERNEL=y
	CONFIG_DEBUG_INFO=y
	CONFIG_DEBUG_INFO_DWARF_TOOLCHAIN_DEFAULT=y
	CONFIG_GCOV=y


将其组合成一个可复制粘贴的命令序列：


	# 将覆盖率选项追加到当前配	$ ./tools/testing/kunit/kunit.py run --kunitconfig=.kunit/ --kunitconfig=tools/testing/kunit/configs/coverage_uml.config
	# 从构建目录（.kunit/）中提取覆盖率信	$ lcov -t "my_kunit_tests" -o coverage.info -c -d .kunit/

	# 从这里开始，过程CONFIG_GCOV_KERNEL=y 时相	# 例如，可以在 tmp 目录中生HTML 报告，如下：
	$ genhtml -o /tmp/coverage_html coverage.info


如果你安装的 gcc 版本不工作，你可以调整步骤：


	$ ./tools/testing/kunit/kunit.py run --make_options=CC=/usr/bin/gcc-6
	$ lcov -t "my_kunit_tests" -o coverage.info -c -d .kunit/ --gcov-tool=/usr/bin/gcov-6

或者，也可以使用基LLVM 的工具链

	# 使用 LLVM 构建并将覆盖率选项追加到当前配	$ ./tools/testing/kunit/kunit.py run --make_options LLVM=1 --kunitconfig=.kunit/ --kunitconfig=tools/testing/kunit/configs/coverage_uml.config
	$ llvm-profdata merge -sparse default.profraw -o default.profdata
	$ llvm-cov export --format=lcov .kunit/vmlinux -instr-profile default.profdata > coverage.info
	# coverage.info 文件lcov 兼容格式，可用于例如生成 HTML 报告
	$ genhtml -o /tmp/coverage_html coverage.info


## 手动运行测试


不使`kunit.py run` 来运行测试也是一个重要的使用场景。目前，如果你想UML 之外的架构上测试，这是你唯一的选择
由于UML 下运行测试相当直接（配置并编译内核，运行 `./linux` 二进制），本节将聚焦于测试非 UML 架构

### 运行内建测试


当将测试设置`=y` 时，测试会作为启动的一部分运行，并TAP 格式将结果打印到 dmesg。因此你只需要像往常一样将测试加入你的 `.config`，构建并启动内核
因此，如果我们用以下配置编译内核

	CONFIG_KUNIT=y
	CONFIG_KUNIT_EXAMPLE_TEST=y

那么我们会看dmesg 中出现类似如下的输出，表明测试已运行并通过

	TAP version 14
	1..1
	    # Subtest: example
	    1..1
	    # example_simple_test: initializing
	    ok 1 - example_simple_test
	ok 1 - example

### 以模块方式运行测

根据测试的不同，你可以将它们构建为可加载模块
例如，我们将之前的配置选项改为


	CONFIG_KUNIT=y
	CONFIG_KUNIT_EXAMPLE_TEST=m

然后在启动进入我们的内核之后，我们可以通过以下方式运行测试

	$ modprobe kunit-example-test

随后它将stdout 打印 TAP 输出
	`modprobe` 在任何测试失败时（截5.13*不会**有非零退出码。但 `kunit.py parse` 会有，见下文
	你也可以设置 `CONFIG_KUNIT=m`，但是，某些特性将不能工作，因此某些测试可能会出错。理想情况下，测试会在其 `Kconfig` 中声明它们依赖于 `KUNIT=y`，但这是一个大多数测试作者不会考虑的边界情况	截至 5.13，唯一的区别是 `current->kunit_test` 将不存在
### 美化打印结果


你可以使`kunit.py parse` 来解dmesg 中的测试输出，并`kunit.py run` 那样熟悉的格式打印结果

	$ ./tools/testing/kunit/kunit.py parse /var/log/dmesg


### 获取每个测试套件的结

无论你如何运行测试，都可以启`CONFIG_KUNIT_DEBUGFS` 来导出每个套件以 TAP 格式呈现的结果：


	CONFIG_KUNIT=y
	CONFIG_KUNIT_EXAMPLE_TEST=m
	CONFIG_KUNIT_DEBUGFS=y

每个套件的结果将暴露`/sys/kernel/debug/kunit/<suite>/results` 下。因此使用我们的示例配置

	$ modprobe kunit-example-test > /dev/null
	$ cat /sys/kernel/debug/kunit/example/results
	... <TAP output> ...

	# 移除模块后，相应的文件会消失
	$ modprobe -r kunit-example-test
	$ cat /sys/kernel/debug/kunit/example/results
	/sys/kernel/debug/kunit/example/results: No such file or directory

### 生成代码覆盖率报

详见 Documentation/dev-tools/gcov.rst 了解如何执行此操作
这里唯一有点 KUnit 特性的建议是，你可能希望将测试构建为模块。这样你可以将测试的覆盖率与启动期间执行的其他代码的覆盖率隔离开，例如：


	# 在运行测试前重置覆盖率计数器	$ echo 0 > /sys/kernel/debug/gcov/reset
	$ modprobe kunit-example-test


## 测试属性与过滤


测试套件和测试用例可以用测试属性（例如测试的速度）来标记。这些属性稍后会打印在测试输出中，并可用于过滤测试执行
### 标记测试属

通过在测试定义中包含一`kunit_attributes` 对象来用属性标记测试
测试用例可以使用 `KUNIT_CASE_ATTR(test_name, attributes)` 宏来定义测试用例，以替代 `KUNIT_CASE(test_name)`

	static const struct kunit_attributes example_attr = {
		.speed = KUNIT_VERY_SLOW,
	};

	static struct kunit_case example_test_cases[] = {
		KUNIT_CASE_ATTR(example_test, example_attr),
	};

	要将一个测试用例标记为慢速，你也可以使用 `KUNIT_CASE_SLOW(test_name)`	这是一个有用的宏，因为 slow 属性是最常用的
测试套件可以通过在套件定义中设置 "attr" 字段来用属性标记

	static const struct kunit_attributes example_attr = {
		.speed = KUNIT_VERY_SLOW,
	};

	static struct kunit_suite example_test_suite = {
		...,
		.attr = example_attr,
	};

	并非 `kunit_attributes` 对象中的所有属性都需要设置。未设置的属性将保持未初始化，并表现得如同该属性被设为 0 NULL。因此，如果一个属性被设为 0，它被视为未设置	这些未设置的属性不会被报告，并可能作为过滤目的的默认值
### 报告属

当用户运行测试时，属性会存在于原始内核输出中（以 KTAP 格式）。注意，对于所有通过的测试，属性默认会kunit.py 输出中隐藏，但可以使`--raw_output` 标志访问原始内核输出。下面是测试用例的测试属性在内核输出中的格式化示例：


	# example_test.speed: slow
	ok 1 example_test

下面是测试套件的测试属性在内核输出中的格式化示例：


	  KTAP version 2
	  # Subtest: example_suite
	  # module: kunit_example_test
	  1..3
	  ...
	ok 1 example_suite

此外，用户可以使用命令行标志 `--list_tests_attr` 输出带有其属性的测试的完整属性报告：


	kunit.py run "example" --list_tests_attr

	在手动运KUnit 时，可以通过传入模块参数 `kunit.action=list_attr` 来访问此报告
### 过滤


用户可以在运行测试时使用 `--filter` 命令行标志来过滤测试。例如：


	kunit.py run --filter speed=slow


你还可以对过滤器使用以下运算符："<">"<=">="!=" "="。例如：


	kunit.py run --filter "speed>slow"

此示例将运行所有速度slow 更快的测试。注意，字符 < > 经常shell 解释，因此可能需要像上面那样加引号或转义
此外，你可以一次使用多个过滤器。只需用逗号分隔过滤器即可。例如：


	kunit.py run --filter "speed>slow, module=kunit_example_test"

	在手动运KUnit 时，你可以通过将过滤器作为模块参数传入来使用此过滤特性：`kunit.filter="speed>slow, speed<=normal"`
被过滤掉的测试将不会运行，也不会出现在测试输出中。你可以使用 `--filter_action=skip` 标志来改为跳过被过滤的测试。这些测试会显示在测试输出中但不会运行。在手动运行 KUnit 时，使用模块参数 `kunit.filter_action=skip` 来启用此特性
### 过滤过程规则


由于套件和测试用例都可以具有属性，过滤期间属性之间可能存在冲突。过滤过程遵循以下规则：

- 过滤始终在单个测试级别进行
- 如果一个测试设置了某个属性，则根据该测试的值进行过滤
- 否则，回退到该套件的值
- 如果两者都未设置，则使用该属性的全局"默认"值
### 当前属性列

`speed`

此属性指示测试执行的速度（测试是慢还是快）
此属性保存为一个枚举，包含以下类别normal"slow" "very_slow"。测试的假定默认速度"normal"。这表示测试花费的时间相对微不足道（少于 1 秒），无论其运行的机器如何。任何比这更慢的测试都可以标记为 "slow" "very_slow"
`KUNIT_CASE_SLOW(test_name)` 可以方便地用于将测试用例的速度设为 "slow"
`module`

此属性指示与测试相关联的模块的名称
此属性自动保存为字符串，并为每个套件打印。测试也可以使用此属性进行过滤
`is_init`

此属性指示测试是否使用了 init 数据或函数
此属性自动保存为布尔值，测试也可以使用此属性进行过滤