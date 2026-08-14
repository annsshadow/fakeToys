## AML 调试器


:Copyright: |copy| 2016, Intel Corporation
:Author: Lv Zheng <lv.zheng@intel.com>


本文档描述 Linux 内核中内嵌的 AML 调试器的用法。

## 1. 构建调试器


启用 AML 调试器需要以下内核配置项：

```
   CONFIG_ACPI_DEBUGGER=y
   CONFIG_ACPI_DEBUGGER_USER=m

```

用户空间工具可以使用以下命令从内核源码树构建：

```
   $ cd tools
   $ make acpi

```

```
   tools/power/acpi/acpidbg

```

它可以通过运行 "make install"（以具有足够权限的用户）安装到系统目录。

## 2. 启动用户空间调试器接口


在内核以内建调试器启动后，可以通过以下方式启动调试器：

```
   # mount -t debugfs none /sys/kernel/debug
   # modprobe acpi_dbg
   # tools/power/acpi/acpidbg

```

这将进入交互式 AML 调试器环境，在其中可以执行调试器命令。

这些命令记录在“ACPICA Overview and Programmer Reference”中，可从

https://acpica.org/documentation

下载。详细的调试器命令参考位于第 12 章“ACPICA Debugger Reference”。
可以使用 "help" 命令进行快速参考。

## 3. 停止用户空间调试器接口


交互式调试器接口可以通过按 Ctrl+C 或使用以下方式关闭：

```
   # rmmod acpi_dbg

```

如果有一个 acpidbg 实例正在运行，模块的卸载可能会失败。

## 4. 在脚本中运行调试器


在测试脚本中运行 AML 调试器可能很有用。"acpidbg" 以特殊的“批处理”模式
支持这一点。例如，以下命令输出：

```
   # acpidbg -b "namespace"

```
