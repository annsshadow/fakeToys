
## 内核测试指南

Linux 内核有多种不同的测试工具，因此要知道在什么场景下使用哪一个工具本身就是一个挑战。本文档大致介绍了这些工具之间的差异，以及它们如何相互配合

## 编写和运行测
内核测试大部分是使用 kselftest KUnit 框架编写的。这两个框架都提供了相应的基础设施，以使运行单个测试以及成组的测试更加容易，同时也提供了辅助函数以帮助编写新的测试
如果你希望验证内核的行为——尤其是内核的特定部分——那么你会想要使KUnit kselftest

### KUnit kselftest 的区
KUnit（Documentation/dev-tools/kunit/index.rst）是一套完全运行于内核内部的“白盒”测试系统：因为测试代码是内核的一部分，它可以访问那些没有向用户空间暴露的内部结构和函数
因此 KUnit 测试最适合针对内核中较小的、自包含的部分编写，这些部分可以独立地进行测试。这与“单元”测试的概念十分契合
例如，一KUnit 测试可能会测试某个单独的内核函数（甚至只是该函数中的某一条代码路径，比如一个错误处理分支），而不是把某个特性作为一个整体来测试
这也使得 KUnit 测试的构建和运行非常快，从而可以在开发过程中频繁地运行它们
关于如何编写 KUnit 测试，有一KUnit 测试风格指南，可能会提供进一步的参考，Documentation/dev-tools/kunit/style.rst

kselftest（Documentation/dev-tools/kselftest.rst）则主要实现在用户空间，测试是普通的用户空间脚本或程序
这使得编写更加复杂的测试，或者需要更多地操控整体系统状态的测试（例如派生进程等）变得更加容易。然而，无法kselftest 直接调用内核函数。这意味着只有以某种方式暴露给用户空间的内核功能（例如通过系统调用、设备、文件系统等）才能用 kselftest 进行测试。为了绕过这个限制，一些测试会附带一个配套的内核模块，以暴露更多的信息或功能。不过，如果一个测试主要在甚至完全在内核内部运行，那么 KUnit 可能是更合适的工具
因此，kselftest 非常适合针对完整特性的测试，因为这些特性会向用户空间暴露一个可被测试的接口，而不是暴露实现细节。这与“系统”或“端到端”测试的概念十分契合
例如，所有新增的系统调用都应该附kselftest 测试

## 代码覆盖率工
Linux 内核支持两种不同的代码覆盖率测量工具。它们可以用来验证某个测试是否执行了特定的函数或代码行。这对于确定内核有多少部分正在被测试，以及寻找那些没有被相应测试覆盖到的边界情况非常有用
Documentation/dev-tools/gcov.rst GCC 的覆盖率测试工具，可以配合内核使用以获取全局或按模块统计的覆盖率。与 KCOV 不同，它不记录按任务粒度的覆盖率。覆盖率数据可以debugfs 中读取，并使用常规的 gcov 工具集进行解析
Documentation/dev-tools/kcov.rst 是一个可以编译进内核的特性，用于按任务粒度捕获覆盖率。因此它对于模糊测试（fuzzing）以及那些需要了解代码在单次执行（例如某次系统调用）期间被执行情况的场景非常有用

## 动态分析工
内核还支持一系列动态分析工具，它们试图在问题发生于运行中的内核时检测特定类别的问题。这些工具通常各自寻找不同类型的缺陷，例如非法内存访问、数据竞争之类的并发问题，或者其他像整数溢出这样的未定义行为
下面列出了其中的一些工具：

- kmemleak 检测可能存在的内存泄漏。见 Documentation/dev-tools/kmemleak.rst
- KASAN 检测非法内存访问，例如越界访问和释放后使用（use-after-free）错误。见 Documentation/dev-tools/kasan.rst
- UBSAN 检C 标准所定义的未定义行为，例如整数溢出。见 Documentation/dev-tools/ubsan.rst
- KCSAN 检测数据竞争。见 Documentation/dev-tools/kcsan.rst
- KFENCE 是一个低开销的内存问题检测器，比 KASAN 快得多，并且可以用在生产环境中。见 Documentation/dev-tools/kfence.rst
- lockdep 是一个锁正确性验证器。见 Documentation/locking/lockdep-design.rst
- Runtime Verification（RV，运行时验证）支持检查某个给定子系统的特定行为。见 Documentation/trace/rv/runtime-verification.rst
- 内核中还有其他一些调试插桩手段，其中许多可以lib/Kconfig.debug 中找
这些工具往往把内核作为一个整体来测试，并不会kselftest KUnit 测试那样“通过”。它们可以与 KUnit kselftest 结合使用，方法是：在一个开启了这些工具的内核上运行测试，这样你就可以确信在测试期间没有发生这些错误
其中一些工具与 KUnit kselftest 集成，一旦检测到问题就会使测试自动失败

## 静态分析工
除了测试运行中的内核之外，还可以使用**静态分*工具直接在编译期对内核源代码进行分析。内核中常用的工具可以检查整个源代码树，也可以只检查其中的特定文件。它们使在开发过程中检测和修复问题变得更加容易
Sparse 可以通过执行类型检查、锁检查、值范围检查，以及在进行代码检查时报告各种错误和警告来辅助测试内核。关于如何使用它，详Documentation/dev-tools/sparse.rst 文档页面
Smatch 扩展Sparse，并提供针对编程逻辑错误的额外检查，例如 switch 语句中缺失的 break、在错误检查时忽略了返回值、忘记在错误路径的返回值中设置错误码等。Smatch 还能针对更严重的问题（如整数溢出、空指针解引用和内存泄漏）进行测试。详见项目主http://smatch.sourceforge.net/
Coccinelle 是我们可以使用另一种静态分析器。Coccinelle 经常用于辅助源代码的重构与协同演进，但它也能帮助避免常见代码模式中出现的某些缺陷。可用的测试类型包括 API 测试、内核迭代器正确用法的测试、释放操作健全性的检查、锁行为的分析，以及其他一些已知有助于保持内核使用一致性的测试。详Documentation/dev-tools/coccinelle.rst 文档页面
但要注意，静态分析工具存*误报**。在尝试修复错误和警告之前，需要仔细地评估它们
### 何时使用 Sparse Smatch

Sparse 进行类型检查，例如验证带注解的变量不会导致字节序（endianness）缺陷、检测不当使`__user` 指针的地方，以及分析符号初始化器的兼容性
Smatch 进行流分析，如果允许构建函数数据库，它还会进行跨函数分析。Smatch 试图回答诸如“这个缓冲区是在哪里分配的？它有多大？这个索引能否被用户控制？这个变量是否比那个变量更大？”这样的问题
通常，在 Smatch 中编写检查比Sparse 中编写检查更容易。不过，Sparse Smatch 的检查之间存在一些重叠
### Smatch Coccinelle 的优
Coccinelle 可能最容易用来编写检查。它在预处理器之前工作，因此使用 Coccinelle 检查宏中的缺陷更加容易。Coccinelle 还会为你生成补丁，这是其他工具都做不到的
例如，使Coccinelle 你可以一次性把 `kmalloc(x * size, GFP_KERNEL)` 批量转换`kmalloc_array(x, size, GFP_KERNEL)`，这非常有用。如果你只是制造了一Smatch 警告，然后试图把转换的工作推给各子系统维护者，他们会很恼火。你将不得不逐个争辩每个警告是否真的会溢出
Coccinelle 不分析变量的值，而这正是 Smatch 的强项。另一方面，Coccinelle 允许你用简单的方式做简单的事情