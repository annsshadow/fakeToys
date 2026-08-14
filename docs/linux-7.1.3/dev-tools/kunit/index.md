
## KUnit - Linux 内核单元测试


- [start](start)
- [architecture](architecture)
- [run_wrapper](run_wrapper)
- [run_manual](run_manual)
- [usage](usage)
- [api/index](api/index)
- [style](style)
- [faq](faq)
- [running_tips](running_tips)

本节详述内核单元测试框架。

## 简介


KUnit（内核单元测试框架）为 Linux 内核中的单元测试提供了一个通用框架。使用 KUnit，你可以
定义称为测试套件（test suite）的测试用例组。这些测试如果内建，会在内核启动时运行，或以模块
形式加载。KUnit 会自动在内核日志中标记并报告失败的测试用例。测试结果以
[KTAP (Kernel - Test Anything Protocol) format</dev-tools/ktap>](KTAP (Kernel - Test Anything Protocol) format</dev-tools/ktap>) 格式呈现。
它受到了 JUnit、Python 的 unittest.mock 以及 GoogleTest/GoogleMock（C++ 单元测试框架）的启发。

KUnit 测试是内核的一部分，用 C（编程）语言编写，用于测试内核实现的各个部分（例如一个 C 语言
函数）。不计构建时间，从调用到完成，KUnit 可以在不到 10 秒内运行大约 100 个测试。KUnit 可以
测试任何内核组件，例如：文件系统、系统调用、内存管理、设备驱动等等。

KUnit 采用白盒测试方法。测试可以访问内部系统功能。KUnit 运行在内核空间，不受限于暴露给用户空间的
事物。

此外，KUnit 还有 kunit_tool，一个脚本（`tools/testing/kunit/kunit.py`），它配置 Linux 内核，
在 QEMU 或 UML（[User Mode Linux </virt/uml/user_mode_linux_howto_v2>](User Mode Linux </virt/uml/user_mode_linux_howto_v2>)）
下运行 KUnit 测试，解析测试结果并以对用户友好的方式显示。

### 特性


- 提供编写单元测试的框架。
- 可在任何内核架构上运行测试。
- 在毫秒级运行一个测试。

### 先决条件


- 任何与 Linux 内核兼容的硬件。
- 对于被测内核，Linux 内核版本为 5.5 或更高。

## 单元测试


单元测试在隔离状态下测试单个代码单元。单元测试是粒度最细的测试，并允许对被测代码中的所有可能
代码路径进行测试。如果被测代码较小且没有测试无法控制的外部依赖（如硬件），这是可以实现的。

### 编写单元测试


要编写良好的单元测试，有一个简单但强大的模式：Arrange-Act-Assert（准备-执行-断言）。这是组织
测试用例的好方法，并定义了操作的先后顺序。

- 准备输入与目标：在测试开始时，准备使函数能够工作的数据。例如：初始化一个语句或对象。
- 执行目标行为：调用你的被测函数/代码。
- 断言预期结果：验证结果（或结果状态）符合预期。

### 单元测试的优势


- 从长远来看提升测试速度与开发效率。
- 在初始阶段检测缺陷，因此与验收测试相比降低了缺陷修复成本。
- 改善代码质量。
- 鼓励编写可测试的代码。

另请阅读 kinds-of-tests。

## 我该如何使用它？


你可以在 Documentation/dev-tools/kunit/start.rst 中找到编写与运行 KUnit 测试的逐步指南。

或者，欢迎浏览 KUnit 文档的其余部分，或试验 tools/testing/kunit/kunit.py 以及
lib/kunit/kunit-example-test.c 下的示例测试。

测试愉快！
