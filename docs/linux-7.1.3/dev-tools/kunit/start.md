
## Getting Started


包含 一overview the kunit_tool KUnit framework,
teaching 如何 运行 existing tests 然后 如何 写入 一简test case,
covers 通用 problems users face 使用 KUnit 用于 the 第一 time.

## Installing Dependencies

KUnit 具有 the 相同 dependencies 作为 the Linux 内核. 只要 您可
build the 内核, 您可运行 KUnit.

## 运行tests kunit_tool

kunit_tool 一Python script, configures builds 一内核, runs
tests, formats the test results. 来自 the 内核 repository, 
运行 kunit_tool:


	./tools/testing/kunit/kunit.py 运行

	参见 the 以下 错误:
	"The source tree clean, 运行 'make ARCH=um mrproper'"

	姝?happens 鍥犱负 internally kunit.py specifies `.kunit`
	(默认 选项) 作为 the build directory the 命令 `make O=output/dir`
	through the 参数 `--build_dir`.  Hence, 之前 starting 一
	out-of-tree build, the source tree 必须 clean.

	存在 the 相同 caveat mentioned the "Build directory 用于
	the 内核" section the [admin-guide </admin-guide/README>](admin-guide </admin-guide/README>),
	 使用, 必须 使用 用于 全部 invocations `make`.
	The good news indeed solved 运行
	`make ARCH=um mrproper`, just aware 删除 the
	电流 配置 全部 generated 文件.

everything worked correctly, 应当 参见 the 以下:


	Configuring KUnit 内核 ...
	Building KUnit 内核 ...
	Starting KUnit 内核 ...

The tests 灏?pass 鎴?fail.

   因为 它是 building 一lot sources 用于 the 第一 time,
   the `Building KUnit Kernel` step take 一同时.

用于 detailed information wrapper, 参见:
Documentation/dev-tools/kunit/运行_wrapper.rst.

### Selecting tests 运行


默认情况 kunit_tool runs 全部 tests reachable minimal 配置,
 使用 默认 用于 大多the kconfig 选项.  然
您可select tests 运行 

- `Customizing Kconfig`_ 使用 compile the 内核, 
- `Filtering tests by name`_ select specifically compiled tests 运行.

#### Customizing Kconfig

一good starting point 用于 the `.kunitconfig` the KUnit 默认 配置.
didn't 运行 `kunit.py run` 尚未, 您可generate 运行


	cd $PATH_到_LINUX_REPO
	tools/testing/kunit/kunit.py 配置
	cat .kunit/.kunitconfig

   `.kunitconfig` lives the `--build_dir` 使用 kunit.py, 
   `.kunit` 默认情况

之前 运行the tests, kunit_tool ensures 全部 配置 选项
set `.kunitconfig` set the 内核 `.config`. warn
具有 included dependencies 用于 the 选项 使用.

存在 许多 ways customize the configurations:

一 Edit `.kunit/.kunitconfig`. The 文件 应当 包含 the 列出 kconfig
   选项 必需 运行 the desired tests, including 它们dependencies.
   希望 remove 配置_KUNIT_全部_TESTS 来自 the `.kunitconfig` 作为
   启用 一数字 额外 tests 希望.
   需运行 一architecture 其他 UML 参见 kunit-on-qemu.

b. 启用 额外 kconfig 选项 在…之`.kunit/.kunitconfig`.
```

	./tools/testing/kunit/kunit.py run \
		--kconfig_add CONFIG_LIST_KUNIT_TEST=y

```
c. 提供 the path one 更多 .kunitconfig 文件 来自 the tree.
```

	./tools/testing/kunit/kunit.py run \
		--kunitconfig ./fs/fat/.kunitconfig \
		--kunitconfig ./fs/ext4/.kunitconfig

```
d. change the `.kunitconfig`, kunit.py trigger 一rebuild the
   `.config` 文件. 您可edit the `.config` 文件 directly 
   tools 类似 `make menuconfig O=.kunit`. 只要 一superset 
   `.kunitconfig`, kunit.py won't overwrite 您的 changes.



```

		make savedefconfig O=.kunit
		cp .kunit/defconfig .kunit/.kunitconfig

```
#### Filtering tests 鐢?name

希望 更多 特定 Kconfig 提供, 它是 可能
select tests execute boot-time passing 一glob filter
(读取 instructions regarding the pattern the manpage `glob(7)`).
存在 一`"."` (period) the filter, interpreted 作为 一
separator 之间 the name the test suite the test case,
否则, interpreted 作为 the name the test suite.
例如, let's assume 我们 使用 the 默认 配置:

一 inform the name 一test suite, 类似 `"kunit_executor_test"`,
```

	./tools/testing/kunit/kunit.py run "kunit_executor_test"

```
b. inform the name 一test case prefixed test suite,
```

	./tools/testing/kunit/kunit.py run "example.example_simple_test"

```
c. 使用 wildcard characters (`*[`) 运行 任何 test case matches the pattern,
   类似 `"**.**64*"` 运行 test cases containing `"64"` the name inside
```

	./tools/testing/kunit/kunit.py run "*.*64*"

```
## 运行Tests the KUnit Wrapper

执行 希望 使用 the KUnit Wrapper (例如: 希望 code
在…下 test integrate 其他 系统, 使用 一不同/
不受支持 architecture 配置), KUnit included 
任何 内核, the results 读取 out parsed manually.

   `CONFIG_KUNIT` 应当 已启一production environment.
   Enabling KUnit disables 内核 Address-Space Layout Randomization
   (KASLR), tests affect the 状the 内核 ways 
   suitable 用于 production.

### Configuring the 内核

启用 KUnit itself, 需启用 the `CONFIG_KUNIT` Kconfig
选项 (在…下 内核 Hacking/内核 Testing Coverage 
`menuconfig`). 来自 那里, 您可启用 任何 KUnit tests. 它们
通常 具有 配置 选项 ending `_KUNIT_TEST`.

KUnit KUnit tests compiled 作为 模块. The tests 一模块
运行 the 模块 loaded.

### 运行Tests (KUnit Wrapper)

Build 运行 您的 内核. the 内核 log, the test 输出 printed
out the TAP 格式. happen 默认情况KUnit/tests
built-in. 否则 the 模块 需loaded.

   一lines 数据 get interspersed the TAP 输出.

## Writing 您的 第一 Test

您的 内核 repository, let's add 一code 我们可以 test.

1. 创建 一文件 `drivers/misc/example.h`, 包含:


	int misc_示例_add(int left, int right);

2. 创建 一文件 `drivers/misc/example.c`, 包含:


	#包含 <linux/errno.h>

	#包含 "示例.h"

	int misc_示例_add(int left, int right)
	{
		return left + right;
	}

3. Add the 以下 lines `drivers/misc/Kconfig`:


	配置 MISC_示例
		bool "My 示例"

4. Add the 以下 lines `drivers/misc/Makefile`:


	obj-$(配置_MISC_示例) += 示例.o

现在 我们 ready 写入 the test cases.

1. Add the 下文 test case `drivers/misc/example_test.c`:


	#包含 <kunit/test.h>
	#包含 "示例.h"

	/** 定义 the test cases. **/

	静void misc_示例_add_test_基本(结构kunit *test)
	{
		KUNIT_EXPECT_EQ(test, 1, misc_示例_add(1, 0));
		KUNIT_EXPECT_EQ(test, 2, misc_示例_add(1, 1));
		KUNIT_EXPECT_EQ(test, 0, misc_示例_add(-1, 1));
		KUNIT_EXPECT_EQ(test, INT_MAX, misc_示例_add(0, INT_MAX));
		KUNIT_EXPECT_EQ(test, -1, misc_示例_add(INT_MAX, INT_MIN));
	}

	静void misc_示例_test_failure(结构kunit *test)
	{
		KUNIT_FAIL(test, "test 从不 passes.");
	}

	静结构kunit_case misc_示例_test_cases[] = {
		KUNIT_CASE(misc_示例_add_test_基本),
		KUNIT_CASE(misc_示例_test_failure),
		{}
	};

	静结构kunit_suite misc_示例_test_suite = {
		.name = "misc-example",
		.test_cases = misc_示例_test_cases,
	};
	kunit_test_suite(misc_示例_test_suite);

	模块_LICENSE("GPL");

2. Add the 以下 lines `drivers/misc/Kconfig`:


	配置 MISC_示例_TEST
		tristate "Test 用于 my 示例" !KUNIT_全部_TESTS
		depends MISC_示例 && KUNIT
		默认 KUNIT_全部_TESTS

注意: 您的 test 执行 支持 正在 built 作为 一loadable 模块 (
discouraged), replace tristate bool, depend KUNIT=y 而非 KUNIT.

3. Add the 以下 lines `drivers/misc/Makefile`:


	obj-$(配置_MISC_示例_TEST) += 示例_test.o

4. Add the 以下 lines `.kunit/.kunitconfig`:


	配置_MISC_示例=y
	配置_MISC_示例_TEST=y

5. 运行 the test:


	./tools/testing/kunit/kunit.py 运行

应当 参见 the 以下 failure:


	...
	[16:08:57] [PASSED] misc-example:misc_示例_add_test_基本
	[16:08:57] [FAILED] misc-example:misc_示例_test_failure
	[16:08:57] EXPECTATION FAILED 驱动/misc/example-test.c:17
	[16:08:57]      test 从不 passes.
	...

Congrats! just wrote 您的 第一 KUnit test.

## 鎺ヤ笅鏉?Steps


re interested 使用 一the 更多 高级 特kunit.py,
take 一look Documentation/dev-tools/kunit/运行_wrapper.rst

d 类似 运行 tests 使用 kunit.py, check out
Documentation/dev-tools/kunit/运行_manual.rst

用于 更多 information writing KUnit tests (including 一通用 techniques
用于 testing 不同 things), 参见 Documentation/dev-tools/kunit/usage.rst
