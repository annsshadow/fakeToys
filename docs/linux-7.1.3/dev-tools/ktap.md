
## 内核测试任意协议（Kernel Test Anything Protocol，KTAP），版本 1

TAP，即 Test Anything Protocol（测试任意协议），是若干项目用于指定测试结果的格式。其网站规范可在`链接 <https://testanything.org/>`_ 找到。Linux 内核大量使用 TAP 输出作为测试结果然而，内核测试框架对测试结果有特殊需求，与原TAP 规范并不一致。因此，规定了一种“内TAP（KTAP）格式来扩展和改TAP 以支持这些用例。本规范描述了当前内核中所使用的被广泛接受KTAP 格式
KTAP 测试结果描述了一系列测试（可以嵌套：即测试可以拥有子测试），每个测试都可以同时包诊断数据——例如日志行——以及一个最终结果。测试结构和结果是机器可读的，而诊断数据是非结构化
的，用于辅助人工调试
KTAP 输出由四种不同类型的行构成：

- 版本- 计划- 测试用例结果- 诊断
一般而言，有效的 KTAP 输出也应构成有效TAP 输出，但某些信息（尤其是嵌套测试结果）可能会
丢失。另请注意，TAP14 有一份停滞的草案规范，KTAP 在少数地方与之存在差异（特别是“Subtest头部），相关之处将在本文档后面描述
### 版本
所KTAP 格式的结果都以一条“版本行”开头，用来指明结果符合哪个版本(K)TAP 标准
例如
- "KTAP version 1"
- "TAP version 13"
- "TAP version 14"

请注意，KTAP 中，子测试也以版本行开头，表示嵌套测试结果的开始。这与使用单独的 "Subtest"
行的 TAP14 不同
虽然今后合规的测试应使用 "KTAP version 1"，但预计大多数解析器和其他工具为了与现有测试和框兼容，会接受这里列出的其他版本
### 计划
测试计划提供 KTAP 输出中测试（或子测试）的数量
计划行必须遵"1..N" 的格式，其中 N 是测试或子测试的数量。计划行跟在版本行之后，以指示嵌测试的数量
虽然存在测试数量事先未知的情况——此时可以省略测试计划——但强烈建议尽可能提供测试计划
### 测试用例结果
测试用例结果行指示测试的最终状态。它们是必需的，且必须采用以下格式：

```
	<result> <number> [<description>][ # [<directive>] [<diagnostic data>]]
```

result 可以"ok"（表示测试用例通过），"not ok"（表示测试用例失败）
<number> 表示正在执行的测试编号。第一个测试必须为编号 1，之后在同一测试、同一嵌套层级内每额外的子测试编号必须递增 1
description 是对测试的描述，通常是测试的名称，可以是# 或换行符之外的任意字符字符串description 是可选的，但建议使用
directive 和任何诊断数据都是可选的。如果二者存在，必须跟在井号 "#" 之后
directive 是一个关键字，表示测试的结果不同于通过与失败。directive 是可选的，由位于诊断数据
之前的单个关键字组成。如果解析器遇到它不支持directive，应当回退"ok" / "not ok" 的结果
当前接受directive 有：

- "SKIP"，表示测试被跳过（注意，如果使用 SKIP directive，测试用例结果行result 可以"ok"
  "not ok"- "TODO"，表示测试目前预期不会通过，例如因为它所测试的特性已知是损坏的。虽然该 directive 继承  TAP，但不鼓励在内核中使用它- "XFAIL"，表示测试预期会失败。这类似于上面的 "TODO"，并被某kselftest 测试使用- "TIMEOUT"，表示测试已超时（注意，如果使用 TIMEOUT directive，测试用例结果行应为 "not ok"- "ERROR"，表示测试的执行因诊断数据中所包含的特定错误而失败。（注意，如果使ERROR directive  测试用例结果行应"not ok"
诊断数据是一个纯文本字段，包含关于为何产生该结果的任何额外细节。对ERROR 或失败的测试，这
通常是一条错误消息；对于 SKIP 结果，则是对缺失依赖的描述
诊断数据字段是可选的，既没有 directive 也没有任何诊断数据的结果不需要包"#" 字段分隔符
```
	ok 1 test_case_name

```
测试 "test_case_name" 通过了
```
	not ok 1 test_case_name

```
测试 "test_case_name" 失败了
```
	ok 1 test # SKIP necessary dependency unavailable

```
测试 "test" 被跳过，诊断消息"necessary dependency unavailable"
```
	not ok 1 test # TIMEOUT 30 seconds

```
测试 "test" 超时，诊断数据为 "30 seconds"
```
	ok 5 check return code # rcode=0

```
测试 "check return code" 通过了，附带额外的诊断数"rcode=0"
### 诊断
如果测试希望输出任何进一步的信息，应当使用“诊断行”。诊断行是可选的、自由格式的文本，常用于
比最终结果和诊断数据行更详细地描述正在测试的内容以及任何中间结果
诊断行的格式"# <diagnostic_description>"，其中描述可以是任意字符串。诊断行可以出现在测输出的任何位置。作为规则，关于某个测试的诊断行应直接位于该测试的结果行之前
请注意，大多数工具会将未知行（见下）视为诊断行，即使它们不是"#" 开头：这是为了捕获任何其他
可能有助于调试测试的有用内核输出。尽管如此，仍建议测试总是"#" 字符为其任何诊断输出加上
前缀
### 未知
KTAP 输出中可能包含不符合上述四种行格式之一的行。这是允许的，但它们不会影响测试的状态
这是TAP 的一个重要区别。内核测试可能会向系统控制台或日志文件打印消息。这两个目标都可能包来自无关内核或用户空间活动的消息，或来自测试调用的非测试代码的内核消息。测试所调用的内核代很可能并不知道测试正在进行，因此无法将该消息作为诊断消息打印
### 嵌套测试

KTAP 中，测试可以嵌套。这是通过让一个测试在其输出中包含一整套 KTAP 格式的结果来实现的。这
可用于对相关的测试进行分类和分组，或拆分同一测试的不同结果
“父”测试的结果应由其所有子测试的结果组成，以另一KTAP 版本行和测试计划开始，以整体结结束。例如，如果其中一个子测试失败，父测试也应当失败
此外，子测试中的所有行都应缩进。一级缩进是两个空格  "。缩进应从版本行开始，并在父测试的
结果行之前结束
“未知行”不被视为子测试中的行，因此可以缩进也可以不缩进
带有两个嵌套子测试的测试示例
```
	KTAP version 1
	1..1
	  KTAP version 1
	  1..2
	  ok 1 test_1
	  not ok 2 test_2
	# example failed
	not ok 1 example

```
多级嵌套测试的示例格式：

```
	KTAP version 1
	1..2
	  KTAP version 1
	  1..2
	    KTAP version 1
	    1..2
	    not ok 1 test_1
	    ok 2 test_2
	  not ok 1 test_3
	  ok 2 test_4 # SKIP
	not ok 1 example_test_1
	ok 2 example_test_2

```
### TAP KTAP 的主要区
==================================================   =========  ===============
Feature                                              TAP        KTAP
==================================================   =========  ===============
诊断消息中的 yaml json                            ok        不推TODO directive                                        ok        不被识别
允许任意数量的测试被嵌套                            no         yes
"Unknown lines" 属于 "Anything else" 类别           yes        no
"Unknown lines" 是否                                 incorrect  允许
==================================================   =========  ===============

TAP14 规范确实允许嵌套测试，但使用的是形如 "Subtest: <name>" 的行（其<name> 是父测试的名称）而不是使用另一个嵌套的版本行
### KTAP 输出示例

```
	KTAP version 1
	1..1
	  KTAP version 1
	  1..3
	    KTAP version 1
	    1..1
	    # test_1: initializing test_1
	    ok 1 test_1
	  ok 1 example_test_1
	    KTAP version 1
	    1..2
	    ok 1 test_1 # SKIP test_1 skipped
	    ok 2 test_2
	  ok 2 example_test_2
	    KTAP version 1
	    1..3
	    ok 1 test_1
	    # test_2: FAIL
	    not ok 2 test_2
	    ok 3 test_3 # SKIP test_3 skipped
	  not ok 3 example_test_3
	not ok 1 main_test

```
该输出定义了以下层级
一个名"main_test" 的单一测试，它失败，并拥有三个子测试：

- "example_test_1"，通过，拥有一个子测试
   - "test_1"，通过，并输出诊断消息 "test_1: initializing test_1"

- "example_test_2"，通过，拥有两个子测试
   - "test_1"，被跳过，解释为 "test_1 skipped"
   - "test_2"，通过

- "example_test_3"，失败，拥有三个子测
   - "test_1"，通过
   - "test_2"，输出诊断行 "test_2: FAIL"，并失败
   - "test_3"，被跳过，解释为 "test_3 skipped"

请注意，同名的各个子测试并不冲突，因为它们位于不同的父测试中。该输出还体现了一些合理的“冒泡测试结果的规则：如果任何一个子测试失败，则测试失败。被跳过的测试不影响父测试的结果（尽管如其_所有_子测试都被跳过，将一个测试标记为跳过通常是有意义的）
### 另请参阅
- TAP 规范  https://testanything.org/tap-version-13-specification.html
- （停滞的）TAP 版本 14 规范  https://github.com/TestAnything/Specification/blob/tap-14-specification/specification.md
- kselftest 文档  Documentation/dev-tools/kselftest.rst
- KUnit 文档  Documentation/dev-tools/kunit/index.rst
