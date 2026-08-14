
## 不使用 kunit_tool 运行测试


如果我们不想使用 kunit_tool（例如：希望与其他系统集成，或在真实硬件上运行测试），
我们可以把 KUnit 包含进任意内核中，读取结果并手动解析。

          测试可能会降低系统的稳定性或安全性。

## 配置内核


KUnit 测试可以脱离 kunit_tool 运行。这在以下情况会很有用：

- 我们有一个现成的内核配置需要测试。
- 需要在真实硬件上运行（或使用 kunit_tool 不支持的模拟器/虚拟机）。
- 希望与某些现有的测试系统集成。

KUnit 通过 `CONFIG_KUNIT` 选项配置，而各个测试也可以通过在我们的 `.config`
中启用它们各自的配置选项来构建。KUnit 测试通常（但不总是）具有以 `_KUNIT_TEST`
结尾的配置选项。大多数测试既可以构建为模块，也可以内建进内核。


	我们可以启用 `KUNIT_ALL_TESTS` 配置选项来自动启用所有依赖已满足的测试。
	这是快速测试当前配置适用的所有内容的良好方式。

	KUnit 可以在启动时启用或禁用，该行为由 kunit.enable 内核参数控制。
	默认情况下，kunit.enable 被设为 1，因为 KUNIT_DEFAULT_ENABLED 默认是启用的。
	为确保测试按预期执行，请确认启动时 kunit.enable=1。

一旦我们构建好内核（和/或模块），运行测试就很简单了。如果测试是内建的，
它们会在内核引导时自动运行。结果会以 TAP 格式写入内核日志（`dmesg`）。

如果测试是作为模块构建的，它们会在模块加载时运行。


	# modprobe example-test

结果会以 TAP 格式出现在 `dmesg` 中。

## debugfs


KUnit 可以通过 debugfs 文件系统从用户空间访问（关于 debugfs 的更多信息，
参见 Documentation/filesystems/debugfs.rst）。

如果启用了 `CONFIG_KUNIT_DEBUGFS`，KUnit 的 debugfs 文件系统挂载在
/sys/kernel/debug/kunit。你可以使用此文件系统执行以下操作。

## 获取测试结果


你可以使用 debugfs 获取 KUnit 测试结果。测试结果可从 debugfs 文件系统中的
以下只读文件中访问：


	/sys/kernel/debug/kunit/<test_suite>/results

测试结果以 KTAP 文档形式打印。注意该文档与内核日志是分开的，因此测试套件的
编号可能不同。

## 在内核引导后运行测试


你可以使用 debugfs 文件系统触发内建测试在引导后运行。要运行测试套件，
你可以使用以下命令向 `/sys/kernel/debug/kunit/<test_suite>/run` 文件写入：


	echo "any string" > /sys/kernel/debugfs/kunit/<test_suite>/run

作为结果，测试套件会运行，结果会打印到内核日志中。

不过，使用了 init 数据的 KUnit 套件无法使用此功能，因为 init 数据可能在
内核引导后被丢弃。使用 init 数据的 KUnit 套件应当用
kunit_test_init_section_suites() 宏来定义。

另外，你不能使用此功能并发运行测试。相反，一个测试会一直等待，直到其他测试
完成或失败才会运行。


	对测试作者而言，要使用此功能，测试需要正确地初始化和/或清理任何数据，
	以便测试能正确地第二次运行。

