## 测试风格与命

为了使查找、编写和使用 KUnit 测试尽可能简单，强烈建议按照以下准则对它们进行命名和编写。虽然也可以编写不遵循这些规则的 KUnit 测试，但它们可能破坏某些工具，可能与其他测试冲突，并且可能不会被测试系统自动运行
建议你仅在以下情况下偏离这些准则
1. 将已有已知名称的测试移植KUnit2. 编写如果被自动运行会导致严重问题的测试。例如，非确定性地产生假阳性或假阴性，或运行时间很长
## 子系统、测试套件与测试


为了使测试易于查找，它们被分组为套件（suite）和子系统（subsystem）。一个测试套件（test suite）是一组测试某个内核相关领域的测试。一个子系统（subsystem）是一组测试内核子系统不同部分或某个驱动程序的测试套件
### 瀛愮郴缁。

每个测试套件必须属于一个子系统。子系统是测试同一驱动程序或内核某部分的一个或多个 KUnit 测试套件的集合。测试子系统应与单个内核模块匹配。如果被测代码无法编译为模块，在许多情况下子系统应对应于源树中的目录`MAINTAINERS` 文件中的条目。如果不确定，请遵循类似领域中的测试所设定的约定
测试子系统应以被测代码命名，要么以模块命名（尽可能），要么以被测目录或文件命名。测试子系统命名时应在必要时避免歧义
如果测试子系统名称有多个组成部分，它们应该用下划线分隔*不要**在子系统名称中直接包“test“kunit”，除非我们实际上是在测试其他测试或 kunit 框架本身。例如，子系统可以称为：

`ext4`
  匹配模块和文件系统名`apparmor`
  匹配模块名和 LSM 名`kasan`
  该工具的通用名，路径 `mm/kasan` 的显著部分`snd_hda_codec_hdmi`
  有多个组成部分（`snd`、`hda`、`codec`、`hdmi`），用下划线分隔。匹配模块名
避免使用如下所示的示例名称
`linear-ranges`
  名称应使用下划线而非连字符来分隔单词。推`linear_ranges``qos-kunit-test`
  此名称应使用下划线，并且不应“kunit-test作为后缀。`qos` 作为子系统名也有歧义，因为内核的多个部分都有 `qos` 子系统。`power_qos` 会是更好的名称`pc_parallel_port`
  对应的模块名`parport_pc`，因此该子系统也应命名为 `parport_pc`
         KUnit API 和工具并不显式了解子系统。它们是一种对测试套件进行分类和命名模块的方式，为人类提供了一种简单、一致的方式来查找和运行测试。这在未来可能会改变
### 测试套件


KUnit 测试被分组为测试套件，覆盖被测的特定功能领域。测试套件可以具有为套件中所有测试运行的共享初始化和关闭代码。并非所有子系统都需要拆分为多个测试套件（例如，简单驱动）
测试套件以它们所属的子系统命名。如果一个子系统包含多个套件，应将被测的特定领域附加到子系统名后面，用下划线分隔
如果一个子系统内存在多种使KUnit 的测试（例如，既有单元测试又有集成测试），它们应放入单独的套件中，测试类型作为套件名的最后一个元素。除非这些测试实际存在，否则避免在套件名中使`_test`、`_unittest` 或类似字样
完整的测试套件名（包括子系统名）应指定为 `kunit_suite` 结构体的 `.name` 成员，并构成模块名的基础。例如，测试套件可以包括
`ext4_inode`
  `ext4` 子系统的一部分，测`inode` 领域`kunit_try_catch`
  `kunit` 实现本身的一部分，测`try_catch` 领域`apparmor_property_entry`
  `apparmor` 子系统的一部分，测`property_entry` 领域`kasan`
  `kasan` 子系统只有一个套件，因此套件名与子系统名相同
避免例如以下名称
`ext4_ext4_inode`
  没有理由把子系统名说两遍`property_entry`
  没有子系统名时套件名是有歧义的`kasan_integration_test`
  因为 `kasan` 子系统中只有一个套件，该套件应直接称为 `kasan`。不要冗余地添加 `integration_test`。它应该是一个单独的测试套件。例如，如果添加了单元测试，那么该套件可以命名为 `kasan_unittest` 或类似名称
### 测试用例


单个测试由测试受限代码路径、属性或函数的单个函数组成。在测试输出中，单个测试的结果将作为套件结果的子测试显示
测试应以它们所测试的内容命名。这通常是要测试的函数的名称，加上对正在测试的input或代码路径的描述。由于测试是 C 函数，它们应按照内核编码风格命名和编写
        由于测试本身也是函数，它们的名称不能与内核中的其C 标识符冲突。这可能需要一些有创意的命名。将你的测试函数设为 `static` 以避免污染全局命名空间是个好主意
测试名称示例包括
`unpack_u32_with_null_name`
  测试当传NULL 名称`unpack_u32` 函数`test_list_splice`
  测试 `list_splice` 宏。它有前缀 `test_` 以避免与该宏本身发生名称冲突
如果需要在其测试套件之外引用一个测试，测试*完全限定（fully-qualified*名称应为套件名后跟测试名，用冒号分隔（即 `suite:test`）
## 测试 Kconfig 条目


每个测试套件都应与一Kconfig 条目绑定
Kconfig 条目必须
- 命名`CONFIG_<name>_KUNIT_TEST`：其<name> 是测试套件的名称- 与所测试驱动/子系统的配置条目列在一起，或位[Kernel Hacking]->[Kernel Testing and Coverage] 下- 依赖`CONFIG_KUNIT`- 仅当 `CONFIG_KUNIT_ALL_TESTS` 未启用时可见- 默认值为 `CONFIG_KUNIT_ALL_TESTS`- 在帮助文本中有对 KUnit 的简要描述
如果我们无法满足上述条件（例如，该测试无法构建为模块），测试Kconfig 条目应为三态（tristate）
例如，一Kconfig 条目可能如下所示：


	config FOO_KUNIT_TEST
		tristate "KUnit test for foo" if !KUNIT_ALL_TESTS
		depends on KUNIT
		default KUNIT_ALL_TESTS
		help
		  This builds unit tests for foo.

		  For more information on KUnit and unit tests in general,
		  please refer to the KUnit documentation in Documentation/dev-tools/kunit/.

		  If unsure, say N.


## 测试文件与模块名


KUnit 测试通常编译为单独的模块。为了避免与常规模块冲突，KUnit 模块应以测试套件命名，后`_kunit`（例如，如果 “foobar是核心模块，那么 “foobar_kunitKUnit 测试模块）
测试源文件，无论是编译为单独的模块还是作`#include` 包含在其他源文件中，最好保存在 `tests/` 子目录中，以避免与其他源文件冲突（例如用于制表符补全）
注意，一些现有测试中也使用了 `_test` 后缀。更推荐使用 `_kunit` 后缀，因为它KUnit 与非 KUnit 测试之间的区别更清晰
因此对于常见情况，将包含测试套件的文件命名为 `tests/<suite>_kunit.c`。`tests` 目录应放在与被测代码相同的层级。例如，`lib/string.c` 的测试位`lib/tests/string_kunit.c`
如果套件名包含测试父目录名称的部分或全部，修改源文件名以减少冗余可能是有意义的。例如，`foo_firmware` 套件可以放在 `foo/tests/firmware_kunit.c` 文件中