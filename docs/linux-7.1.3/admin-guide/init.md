## 解释 “No working init found.” 启动挂起消息

:Authors: Andreas Mohr <andi at lisas period de>
          Cristian Souza <cristianmsbr at gmail period com>

本文档提供了加载 init 二进制文件失败的一些高层原因（大致按执行顺序列出）。

1) **无法挂载根文件系统**：设置 “debug” 内核参数（在 bootloader 配置文件或 CONFIG_CMDLINE 中）
   以获取更详细的内核消息。

2) **init 二进制文件在根文件系统中不存在**：确保你拥有正确的根文件系统类型（且 `root=` 内核参数
   指向正确的分区），所需的驱动（如存储硬件（如 SCSI 或 USB！）和文件系统（ext3、jffs2 等））
   已内建（或作为模块，由 initrd 预加载）。

3) **控制台设备损坏**：可能是 `console= setup` 中存在冲突 --> 初始控制台不可用。例如，某些串口
   控制台由于串口 IRQ 问题（例如缺少基于中断的配置）而不可靠。尝试使用不同的 `console= device`
   或例如 `netconsole=`。

4) **二进制文件存在但依赖不可用**：例如，init 二进制文件所需的库依赖（如 `/lib/ld-linux.so.2`）
   缺失或损坏。使用 `readelf -d <INIT>|grep NEEDED` 找出需要哪些库。

5) **二进制文件无法被加载**：确保二进制文件的架构与你的硬件匹配。例如 i386 与 x86_64 不匹配，或
   尝试在 ARM 硬件上加载 x86。如果你曾尝试在此加载非二进制文件（shell 脚本？），应确保该脚本在其
   shebang 头行（`#!/...`）中指定一个完全可用的解释器（包括其库依赖）。在处理脚本之前，最好先测试
   一个简单的非脚本二进制文件（如 `/bin/sh`）并确认其成功执行。要了解更多信息，可向 `init/main.c`
   添加代码以显示 kernel_execve() 的返回值。

请在你发现新的失败原因时随时扩展本说明（毕竟加载 init 二进制文件是一个关键且艰难的过渡步骤，需要
尽可能无痛），然后向 LKML 提交补丁。进一步的 TODO：

- 通过结构体数组实现各种 `run_init_process()` 调用，从而可以存储 `kernel_execve()` 的结果值，并在
  失败时通过遍历**所有**结果来记录一切（非常重要的可用性修复）。
- 尝试让实现本身整体更有帮助，例如通过在相关位置提供额外的错误消息。
