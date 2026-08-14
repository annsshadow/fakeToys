
## Getting Started


此 页 包含 一个 overview 的 the kunit_tool 和 KUnit framework,
teaching 如何 到 运行 existing tests 和 然后 如何 到 写入 一个 简单 test case,
和 covers 通用 problems users face 当 使用 KUnit 用于 the 第一 time.

## Installing Dependencies

KUnit 具有 the 相同 dependencies 作为 the Linux 内核. 只要 您可以
build the 内核, 您可以 运行 KUnit.

## 运行中 tests 与 kunit_tool

kunit_tool 是 一个 Python script, 其 configures 和 builds 一个 内核, runs
tests, 和 formats the test results. 来自 the 内核 repository, 您
可 运行 kunit_tool:


	./tools/testing/kunit/kunit.py 运行

	您 可 参见 the 以下 错误:
	"The source tree 是 不 clean, 请 运行 'make ARCH=um mrproper'"

	此 happens 因为 internally kunit.py specifies `.kunit`
	(默认 选项) 作为 the build directory 在 the 命令 `make O=output/dir`
	through the 参数 `--build_dir`.  Hence, 之前 starting 一个
	out-of-tree build, the source tree 必须 为 clean.

	存在 也 the 相同 caveat mentioned 在 the "Build directory 用于
	the 内核" section 的 the [admin-guide </admin-guide/README>](admin-guide </admin-guide/README>),
	即, 其 使用, 它 必须 为 使用 用于 全部 invocations 的 `make`.
	The good news 是 该 它 可 indeed 为 solved 由 运行中
	`make ARCH=um mrproper`, just 为 aware 该 此 将 删除 the
	电流 配置 和 全部 generated 文件.

若 everything worked correctly, 您 应当 参见 the 以下:


	Configuring KUnit 内核 ...
	Building KUnit 内核 ...
	Starting KUnit 内核 ...

The tests 将 pass 或 fail.

   因为 它是 building 一个 lot 的 sources 用于 the 第一 time,
   the `Building KUnit Kernel` step 可 take 一个 同时.

用于 detailed information 在 此 wrapper, 参见:
Documentation/dev-tools/kunit/运行_wrapper.rst.

### Selecting 其 tests 到 运行


默认情况下, kunit_tool runs 全部 tests reachable 与 minimal 配置,
即, 使用 默认 值 用于 大多数 的 the kconfig 选项.  然而,
您可以 select 其 tests 到 运行 由:

- `Customizing Kconfig`_ 使用 到 compile the 内核, 或
- `Filtering tests by name`_ 到 select specifically 其 compiled tests 到 运行.

#### Customizing Kconfig

一个 good starting point 用于 the `.kunitconfig` 是 the KUnit 默认 配置.
若 您 didn't 运行 `kunit.py run` 尚未, 您可以 generate 它 由 运行中:


	cd $PATH_到_LINUX_REPO
	tools/testing/kunit/kunit.py 配置
	cat .kunit/.kunitconfig

   `.kunitconfig` lives 在 the `--build_dir` 使用 由 kunit.py, 其 是
   `.kunit` 默认情况下.

之前 运行中 the tests, kunit_tool ensures 该 全部 配置 选项
set 在 `.kunitconfig` 是 set 在 the 内核 `.config`. 它 将 warn
您 若 您 具有 不 included dependencies 用于 the 选项 使用.

存在 许多 ways 到 customize the configurations:

一个. Edit `.kunit/.kunitconfig`. The 文件 应当 包含 the 列出 的 kconfig
   选项 必需 到 运行 the desired tests, including 它们的 dependencies.
   您 可 希望 到 remove 配置_KUNIT_全部_TESTS 来自 the `.kunitconfig` 作为
   它 将 启用 一个 数字 的 额外 tests 该 您 可 不 希望.
   若 您 需要 到 运行 在 一个 architecture 其他 比 UML 参见 kunit-on-qemu.

b. 启用 额外 kconfig 选项 在…之上 `.kunit/.kunitconfig`.
```

	./tools/testing/kunit/kunit.py run \
		--kconfig_add CONFIG_LIST_KUNIT_TEST=y

```
c. 提供 the path 的 one 或 更多 .kunitconfig 文件 来自 the tree.
```

	./tools/testing/kunit/kunit.py run \
		--kunitconfig ./fs/fat/.kunitconfig \
		--kunitconfig ./fs/ext4/.kunitconfig

```
d. 若 您 change the `.kunitconfig`, kunit.py 将 trigger 一个 rebuild 的 the
   `.config` 文件. 但 您可以 edit the `.config` 文件 directly 或 与
   tools 类似 `make menuconfig O=.kunit`. 只要 其 一个 superset 的
   `.kunitconfig`, kunit.py won't overwrite 您的 changes.



```

		make savedefconfig O=.kunit
		cp .kunit/defconfig .kunit/.kunitconfig

```
#### Filtering tests 由 name

若 您 希望 到 为 更多 特定 比 Kconfig 可 提供, 它是 也 可能
到 select 其 tests 到 execute 在 boot-time 由 passing 一个 glob filter
(读取 instructions regarding the pattern 在 the manpage `glob(7)`).
若 存在 一个 `"."` (period) 在 the filter, 它 将 为 interpreted 作为 一个
separator 之间 the name 的 the test suite 和 the test case,
否则, 它 将 为 interpreted 作为 the name 的 the test suite.
例如, let's assume 我们 是 使用 the 默认 配置:

一个. inform the name 的 一个 test suite, 类似 `"kunit_executor_test"`,
```

	./tools/testing/kunit/kunit.py run "kunit_executor_test"

```
b. inform the name 的 一个 test case prefixed 由 其 test suite,
```

	./tools/testing/kunit/kunit.py run "example.example_simple_test"

```
c. 使用 wildcard characters (`*?[`) 到 运行 任何 test case 该 matches the pattern,
   类似 `"**.**64*"` 到 运行 test cases containing `"64"` 在 the name inside
```

	./tools/testing/kunit/kunit.py run "*.*64*"

```
## 运行中 Tests 无 the KUnit Wrapper

若 您 执行 不 希望 到 使用 the KUnit Wrapper (例如: 您 希望 code
在…下 test 到 integrate 与 其他 系统, 或 使用 一个 不同/
不受支持 architecture 或 配置), KUnit 可 为 included 在
任何 内核, 和 the results 是 读取 out 和 parsed manually.

   `CONFIG_KUNIT` 应当 不 为 已启用 在 一个 production environment.
   Enabling KUnit disables 内核 Address-Space Layout Randomization
   (KASLR), 和 tests 可 affect the 状态 的 the 内核 在 ways 不
   suitable 用于 production.

### Configuring the 内核

到 启用 KUnit itself, 您 需要 到 启用 the `CONFIG_KUNIT` Kconfig
选项 (在…下 内核 Hacking/内核 Testing 和 Coverage 在
`menuconfig`). 来自 那里, 您可以 启用 任何 KUnit tests. 它们
通常 具有 配置 选项 ending 在 `_KUNIT_TEST`.

KUnit 和 KUnit tests 可 为 compiled 作为 模块. The tests 在 一个 模块
将 运行 当 the 模块 是 loaded.

### 运行中 Tests (无 KUnit Wrapper)

Build 和 运行 您的 内核. 在 the 内核 log, the test 输出 是 printed
out 在 the TAP 格式. 此 将 仅 happen 默认情况下 若 KUnit/tests
是 built-in. 否则 the 模块 将 需要 到 为 loaded.

   一些 lines 和/或 数据 可 get interspersed 在 the TAP 输出.

## Writing 您的 第一 Test

在 您的 内核 repository, let's add 一些 code 该 我们可以 test.

1. 创建 一个 文件 `drivers/misc/example.h`, 其 包含:


	int misc_示例_add(int left, int right);

2. 创建 一个 文件 `drivers/misc/example.c`, 其 包含:


	#包含 <linux/errno.h>

	#包含 "示例.h"

	int misc_示例_add(int left, int right)
	{
		return left + right;
	}

3. Add the 以下 lines 到 `drivers/misc/Kconfig`:


	配置 MISC_示例
		bool "My 示例"

4. Add the 以下 lines 到 `drivers/misc/Makefile`:


	obj-$(配置_MISC_示例) += 示例.o

现在 我们 是 ready 到 写入 the test cases.

1. Add the 下文 test case 在 `drivers/misc/example_test.c`:


	#包含 <kunit/test.h>
	#包含 "示例.h"

	/** 定义 the test cases. **/

	静态 void misc_示例_add_test_基本(结构体 kunit *test)
	{
		KUNIT_EXPECT_EQ(test, 1, misc_示例_add(1, 0));
		KUNIT_EXPECT_EQ(test, 2, misc_示例_add(1, 1));
		KUNIT_EXPECT_EQ(test, 0, misc_示例_add(-1, 1));
		KUNIT_EXPECT_EQ(test, INT_MAX, misc_示例_add(0, INT_MAX));
		KUNIT_EXPECT_EQ(test, -1, misc_示例_add(INT_MAX, INT_MIN));
	}

	静态 void misc_示例_test_failure(结构体 kunit *test)
	{
		KUNIT_FAIL(test, "此 test 从不 passes.");
	}

	静态 结构体 kunit_case misc_示例_test_cases[] = {
		KUNIT_CASE(misc_示例_add_test_基本),
		KUNIT_CASE(misc_示例_test_failure),
		{}
	};

	静态 结构体 kunit_suite misc_示例_test_suite = {
		.name = "misc-example",
		.test_cases = misc_示例_test_cases,
	};
	kunit_test_suite(misc_示例_test_suite);

	模块_LICENSE("GPL");

2. Add the 以下 lines 到 `drivers/misc/Kconfig`:


	配置 MISC_示例_TEST
		tristate "Test 用于 my 示例" 若 !KUNIT_全部_TESTS
		depends 在 MISC_示例 && KUNIT
		默认 KUNIT_全部_TESTS

注意: 若 您的 test 执行 不 支持 正在 built 作为 一个 loadable 模块 (其 是
discouraged), replace tristate 由 bool, 和 depend 在 KUNIT=y 而非 KUNIT.

3. Add the 以下 lines 到 `drivers/misc/Makefile`:


	obj-$(配置_MISC_示例_TEST) += 示例_test.o

4. Add the 以下 lines 到 `.kunit/.kunitconfig`:


	配置_MISC_示例=y
	配置_MISC_示例_TEST=y

5. 运行 the test:


	./tools/testing/kunit/kunit.py 运行

您 应当 参见 the 以下 failure:


	...
	[16:08:57] [PASSED] misc-example:misc_示例_add_test_基本
	[16:08:57] [FAILED] misc-example:misc_示例_test_failure
	[16:08:57] EXPECTATION FAILED 在 驱动/misc/example-test.c:17
	[16:08:57]      此 test 从不 passes.
	...

Congrats! 您 just wrote 您的 第一 KUnit test.

## 接下来 Steps


若 您're interested 在 使用 一些 的 the 更多 高级 特性 的 kunit.py,
take 一个 look 在 Documentation/dev-tools/kunit/运行_wrapper.rst

若 您'd 类似 到 运行 tests 无 使用 kunit.py, check out
Documentation/dev-tools/kunit/运行_manual.rst

用于 更多 information 在 writing KUnit tests (including 一些 通用 techniques
用于 testing 不同 things), 参见 Documentation/dev-tools/kunit/usage.rst
