
## Frequently Asked Questions


## How is this different from Autotest, kselftest, and so on?


KUnit 是一个单元测试框架。Autotest、kselftest（以及其他一些）则不是
`单元测试 <https://martinfowler.com/bliki/UnitTest.html>`_ 应当孤立地测试单一的代码单元，因此得名**单元测试**。单元测试应当是粒度最细的测试，并且应当允许对被测代码中的所有可能代码路径进行测试。这只有在被测代码规模较小、且不具有测试无法控制的外部依赖（如硬件）时才可能实现
目前还没有任何不需要将内核安装到测试机器或虚拟机上就能运行的测试框架。所有测试框架都要求测试在用户空间编写，并在被测内核上运行。Autotest、kselftest 以及其他一些框架都是如此，这使得它们都不符合单元测试框架的定义
## Does KUnit support running on architectures other than UML?


支持，大体上可以
在多数情况下，KUnit 核心框架（我们用来编写测试的部分）可以编译到任何体系架构上。它就像内核的另一个组成部分一样被编译，并在内核启动时运行，或在作为模块构建时于模块加载时运行。不过，KUnit Wrapper（`tools/testing/kunit/kunit.py`）这样的基础设施可能不支持某些架构（参见 kunit-on-qemu）
简而言之，是的，你可以在其他架构上运行 KUnit，但这可能比UML 上使KUnit 需要更多的工作
更多信息请参kunit-on-non-uml

## What is the difference between a unit test and other kinds of tests?


Linux 内核现有的大多数测试可以归类为集成测试或端到端测试
- 单元测试应当孤立地测试单一的代码单元。单元测试应当是粒度最细的测试，因此允许对被测代码中的所有可能代码路径进行测试。这只有在被测代码规模较小、且不具有测试无法控制的外部依赖（如硬件）时才可能实现- 集成测试测试一组最小组件（通常只有两三个）之间的交互。例如，有人可能会编写一个集成测试来测试驱动与某块硬件之间的交互，或者测试内核提供的用户空间库与内核本身之间的交互。不过，这类测试大概不会测试整个内核以及硬件交互和用户空间交互- 端到端测试通常从被测代码的角度测试整个系统。例如，有人可能会通过在生产硬件上安装生产配置的内核、配合生产用户空间，然后尝试触发某些依赖于硬件、内核与用户空间之间交互的行为，来为内核编写一个端到端测试
## KUnit is not working, what should I do?


很遗憾，有很多地方可能出问题，但以下是一些可以尝试的方法
1. 使用 `--raw_output` 参数运行 `./tools/testing/kunit/kunit.py run`。这可能会显示被 kunit_tool 解析器隐藏的细节或错误消息2. 与其运行 `kunit.py run`，不如尝试分别运`kunit.py config`、`kunit.py build` `kunit.py exec`。这有助于定位问题出现在哪里。（如果你认为是解析器的问题，可以用 `kunit.py parse` 手动针对 `stdin` 或某个文件运行它。）
3. 直接运行 UML 内核通常能暴kunit_tool 会忽略的问题或错误消息。这应该很简单，只需在构UML 内核（例如通过 `kunit.py build`）后运行 `./vmlinux` 即可。请注意，UML 有一些特殊要求（例如宿主机需要挂tmpfs 文件系统），并且在静态构建且宿主机启用了 KASLR 时曾出现过问题。（在较旧的主机内核上，你可能需要运``setarch `uname -m` -R ./vmlinux`` 来禁KASLR。）
4. 确保内核 .config 中包`CONFIG_KUNIT=y` 以及至少一个测试（例如 `CONFIG_KUNIT_EXAMPLE_TEST=y`）。kunit_tool 会保留其 .config，因此你可以在运`kunit.py run` 后查看使用了什么配置。它还会保留你可能做的任何配置更改，因此你可以用 `make ARCH=um menuconfig` 或类似方式启禁用某些选项，然后重新运kunit_tool5. 尝试在运`kunit.py run` 之前先运`make ARCH=um defconfig`。这可能有助于清理掉任何可能导致问题的残留配置项6. 最后，尝试UML 之外运行 KUnit。KUnit KUnit 测试可以被构建进任何内核，也可以作为模块在运行时加载。这样做应当能让你判断是否是 UML 导致了你所看到的问题。当测试是内建时，它们会在内核启动时执行；而模块在加载时会自动执行相关的测试。测试结果可以从 `/sys/kernel/debug/kunit/<test suite>/results` 收集，并可以`kunit.py parse` 解析。更多细节请参阅 kunit-on-qemu
如果上述技巧都没有帮助，欢迎随时将任何问题邮件发送至 kunit-dev@googlegroups.com