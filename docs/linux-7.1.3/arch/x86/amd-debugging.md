
Debugging AMD Zen systems
+++++++++++++++++++++++++

## 简介


本文档描述了可用于调试 AMD Zen 系统问题的技术。它面向开发者和技术人员，以帮助他们识别和解决问题。

## S3 与 s2idle


在 AMD 系统上，无法同时支持挂起到 RAM（S3）和挂起到空闲（s2idle）。要确认你的系统支持哪种模式，可以查看 `cat /sys/power/mem_sleep`。如果它显示 `s2idle [deep]`，则支持 **S3**；如果显示 `[s2idle]`，则支持 **s2idle**。

在支持 **S3** 的系统上，固件将被用来将所有硬件置于适当的低功耗状态。

在支持 **s2idle** 的系统上，内核将负责将设备转换到适当的低功耗状态。当所有设备都处于适当的低功耗状态时，硬件将转换到硬件休眠状态。

在一个挂起周期之后，你可以通过查看 `cat /sys/power/suspend_stats/last_hw_sleep` 来了解在硬件休眠状态中花费了多少时间。

此流程图说明了 AMD s2idle 挂起流程是如何工作的。


此流程图说明了 AMD s2idle 恢复流程是如何工作的。


## s2idle 调试工具


由于问题可能出现在许多地方，因此已经创建了一个调试工具，位于
`amd-debug-tools <https://git.kernel.org/pub/scm/linux/kernel/git/superm1/amd-debug-tools.git/about/>`_，
它可以帮助测试常见问题并提供建议。

如果你有 s2idle 问题，最好从这里开始，并遵循其发现结果中的说明。如果你仍然有问题，请带着此脚本生成的报告，向
`drm/amd gitlab <https://gitlab.freedesktop.org/drm/amd/-/issues/new?issuable_template=s2idle_BUG_TEMPLATE>`_
提交一个缺陷。

## 来自 IRQ 的伪 s2idle 唤醒


伪唤醒通常会有一个 IRQ 被设置到 `/sys/power/pm_wakeup_irq`。这可以匹配到 `/proc/interrupts` 来确定是什么设备唤醒了系统。

如果这还不足以调试问题，那么可以使用以下 sysfs 文件

```

  # echo 1 | sudo tee /sys/power/pm_debug_messages
  # echo 1 | sudo tee /sys/power/pm_print_times

```
在进行这些更改之后，内核将显示可以回溯到内核 s2idle 循环代码的消息，并在唤醒时显示任何活跃的
GPIO 来源。

如果唤醒是由 ACPI SCI 引起的，可能需要额外的 ACPI 调试

```

  # echo enable | sudo tee /sys/module/acpi/parameters/trace_state
  # echo 1 | sudo tee /sys/module/acpi/parameters/aml_debug_output
  # echo 0x0800000f | sudo tee /sys/module/acpi/parameters/debug_level
  # echo 0xffff0000 | sudo tee /sys/module/acpi/parameters/debug_layer

```
## 来自 GPIO 的伪 s2idle 唤醒


如果在唤醒系统时某个 GPIO 处于活跃状态，理想情况下你应该查看原理图来确定它与什么设备相关联。如果原理图不可用，另一种策略是查看 ACPI _EVT() 条目，以确定当该 GPIO 活跃时会通知什么设备。

举一个假设的例子，假设 GPIO 59 唤醒了系统。你可以查看 SSDT 来确定 GPIO 59 活跃时会通知什么设备。

```

  $ python3 -c "print(hex(59))"
  0x3b

```

```

  $ sudo grep EVT /sys/firmware/acpi/tables/SSDT*
  grep: /sys/firmware/acpi/tables/SSDT27: binary file matches

```

```

  $ sudo cp /sys/firmware/acpi/tables/SSDT27 .
  $ sudo iasl -d SSDT27

```

```

  Case (0x3B)
  {
      M000 (0x393B)
      M460 ("    Notify (\\_SB.PCI0.GP17.XHC1, 0x02)\n", Zero, Zero, Zero, Zero, Zero, Zero)
      Notify (\_SB.PCI0.GP17.XHC1, 0x02) // Device Wake
  }

```
你可以看到，在这种情况下，当 GPIO 59 活跃时会通知设备 `\_SB.PCI0.GP17.XHC1`。显然这是一个 XHCI 控制器，但要更进一步，你可以通过将它与以下内容匹配来确定它是哪个 XHCI 控制器

```

  $ grep "PCI0.GP17.XHC1" /sys/bus/acpi/devices/*/path
  /sys/bus/acpi/devices/device:2d/path:\_SB_.PCI0.GP17.XHC1
  /sys/bus/acpi/devices/device:2e/path:\_SB_.PCI0.GP17.XHC1.RHUB
  /sys/bus/acpi/devices/device:2f/path:\_SB_.PCI0.GP17.XHC1.RHUB.PRT1
  /sys/bus/acpi/devices/device:30/path:\_SB_.PCI0.GP17.XHC1.RHUB.PRT1.CAM0
  /sys/bus/acpi/devices/device:31/path:\_SB_.PCI0.GP17.XHC1.RHUB.PRT1.CAM1
  /sys/bus/acpi/devices/device:32/path:\_SB_.PCI0.GP17.XHC1.RHUB.PRT2
  /sys/bus/acpi/devices/LNXPOWER:0d/path:\_SB_.PCI0.GP17.XHC1.PWRS

```
这里你可以看到它匹配到了 `device:2d`。查看 `physical_node`

```

  $ ls -l /sys/bus/acpi/devices/device:2d/physical_node
  lrwxrwxrwx 1 root root 0 Feb 12 13:22 /sys/bus/acpi/devices/device:2d/physical_node -> ../../../../../pci0000:00/0000:00:08.1/0000:c2:00.4

```
于是真相大白：与此 GPIO 唤醒相关联的 PCI 设备是 `0000:c2:00.4`。

`amd_s2idle.py` 脚本将为你捕获大部分这些工件。

## s2idle PM 调试消息


在 AMD 系统的 s2idle 流程中，ACPI LPS0 驱动负责检查所有 uPEP 约束。未满足 uPEP 约束并不会阻止
s0i3 进入。这意味着如果有一些约束未满足，即使存在某些已知问题，内核仍可能尝试进入 s2idle。

要激活 PM 调试，可以在引导时指定 `pm_debug_messagess` 内核命令行选项，或者写入
`/sys/power/pm_debug_messages`。未满足的约束会显示在内核日志中，并可以通过处理内核环形缓冲区的日志工具（如 `dmesg` 或 `journalctl`）查看。

如果系统在刷新这些消息之前在进出时冻结，一个有用的调试策略是解绑 `amd_pmc` 驱动，以阻止向平台发出开始 s0i3 进入的通知。这将阻止系统在进入或退出时冻结，并让你查看所有失败的

```

  cd /sys/bus/platform/drivers/amd_pmc
  ls | grep AMD | sudo tee unbind

```

```

  ACPI: LPI: Constraint not met; min power state:%s current power state:%s

```
## s2idle 问题的历史示例


为了帮助理解可能发生的问题类型以及如何调试它们，这里提供一些已解决的 s2idle 问题的历史示例。

### 核心离线化（Core offlining）


一位最终用户报告说，将一个核心离线会阻止系统正确进入 s0i3。这通过使用内部 AMD 工具调试来捕获和显示来自硬件的一串指标，显示了核心离线时发生了什么变化。确定的是，硬件没有收到离线核心已进入最深状态的通知，因此它阻止了 CPU 进入最深状态。该问题被调试为一个缺失的命令——在离线时让核心进入 C3 状态。

`commit d6b88ce2eb9d2 ("ACPI: processor idle: Allow playing dead in C3 state") <https://git.kernel.org/torvalds/c/d6b88ce2eb9d2>`_

### 恢复后损坏（Corruption after resume）


Rembrandt 出现的一个大问题是恢复后图形损坏。这是由于 PSP 和驱动职责之间的错位造成的。PSP 会保存和恢复 DMCUB，但驱动假定它需要在恢复时重置 DMCUB。实际上，这种错位在更早的硅片上也存在于，只是没有被观察到。

`commit 79d6b9351f086 ("drm/amd/display: Don't reinitialize DMCUB on s0ix resume") <https://git.kernel.org/torvalds/c/79d6b9351f086>`_

### 连续挂起失败（Back to Back suspends fail）


当使用一个触发 IRQ 来唤醒的唤醒源时，pinctrl-amd 驱动中的一个缺陷可能会捕获到 IRQ 的错误状态，从而阻止系统正确回到睡眠状态。

`commit b8c824a869f22 ("pinctrl: amd: Don't save/restore interrupt status and wake status bits") <https://git.kernel.org/torvalds/c/b8c824a869f22>`_

### 5 分钟后的伪定时器唤醒（Spurious timer based wakeup after 5 minutes）


HPET 曾被用来为系统编程唤醒源，然而这导致了 5 分钟后的伪唤醒。正确使用的闹钟应该是 ACPI 闹钟。

`commit 3d762e21d5637 ("rtc: cmos: Use ACPI alarm for non-Intel x86 systems too") <https://git.kernel.org/torvalds/c/3d762e21d5637>`_

### 恢复后磁盘消失（Disk disappears after resume）


从 s2idle 恢复后，NVME 磁盘会消失。这是由于 BIOS 没有指定 _DSD StorageD3Enable 属性造成的。这导致 NVME 驱动没有在挂起时将磁盘置于预期状态，并在恢复时失败。

`commit e79a10652bbd3 ("ACPI: x86: Force StorageD3Enable on more products") <https://git.kernel.org/torvalds/c/e79a10652bbd3>`_

### 伪 IRQ1（Spurious IRQ1）


许多 Renoir、Lucienne、Cezanne 和 Barcelo 平台存在一个平台固件缺陷，即在 s0i3 恢复期间触发 IRQ1。

该问题已在平台固件中修复，但许多系统不再接收任何平台固件更新。

`commit 8e60615e89321 ("platform/x86/amd: pmc: Disable IRQ1 wakeup for RN/CZN") <https://git.kernel.org/torvalds/c/8e60615e89321>`_

### 硬件超时（Hardware timeout）


硬件除了接受来自 amd-pmc 驱动的值之外，还执行许多操作。由于与硬件的通信路径是一个邮箱，它可能无法足够快地响应。

```

  PM: dpm_run_callback(): acpi_subsys_suspend_noirq+0x0/0x50 returns -110
  amd_pmc AMDI0005:00: PM: failed to suspend noirq: error -110

```
计时问题是通过比较空闲掩码的值来确定的。

`commit 3c3c8e88c8712 ("platform/x86: amd-pmc: Increase the response register timeout") <https://git.kernel.org/torvalds/c/3c3c8e88c8712>`_

### 面板开启时无法进入硬件休眠状态（Failed to reach hardware sleep state with panel on）


在一些 Strix 系统上，观察到某些面板会在内部面板开启时阻止系统进入硬件休眠状态。

尽管面板在挂起期间被关闭，但它暴露了一个计时问题：一个中断导致显示硬件唤醒并阻止了低功耗状态的进入。

`commit 40b8c14936bd2 ("drm/amd/display: Disable unneeded hpd interrupts during dm_init") <https://git.kernel.org/torvalds/c/40b8c14936bd2>`_

## 运行时功耗问题


运行时功耗受许多因素影响，包括但不限于 PCIe 主动状态电源管理（ASPM）的配置、显示亮度、CPU 的 EPP 策略，以及设备的电源管理。

### ASPM


为了获得最佳的运行时功耗，ASPM 应该按照硬件厂商的 BIOS 预期进行编程。为了实现这一点，Linux 内核应该以 `CONFIG_PCIEASPM_DEFAULT` 设为 `y` 的方式编译，并且不应修改 sysfs 文件 `/sys/module/pcie_aspm/parameters/policy`。

最值得注意的是，如果任何设备的 L1.2 没有正确配置，SoC 将无法进入最深的空闲状态。

### EPP 策略


`energy_performance_preference` sysfs 文件可用于为 CPU 设置偏向效率或性能。当它更偏向性能时，与电池续航时间有直接关系。


## BIOS 调试消息


大多数 OEM 机器没有用于输出内核或 BIOS 调试消息的串口 UART。然而 BIOS 调试消息对于理解 BIOS 缺陷以及调用 BIOS AML 的 Linux 内核驱动缺陷很有用。

由于大多数 OEM AMD 系统上的 BIOS 基于 AMD 参考 BIOS，用于导出调试消息的基础设施通常与 AMD 参考 BIOS 相同。

### 手动解析（Manually Parsing）


通常有一个 ACPI 方法 `\M460`，AML 的不同路径会调用它来向 BIOS 串行日志发出一条消息。此方法接受
7 个参数，第一个是字符串，其余是可选的

```

  Method (M460, 7, Serialized)

```

```

  M460 ("  OEM-ASL-PCIe Address (0x%X)._REG (%d %d)  PCSA = %d\n", DADR, Arg0, Arg1, PCSA, Zero, Zero)

```
通常执行时，`\M460` 方法会将附加参数填充到字符串中。为了从 Linux 内核获取这些消息，ACPICA 中
加入了一个钩子，它可以捕获发送给 `\M460` 的**参数**并将其打印到内核环形缓冲区。

```

  extrace-0174 ex_trace_args         :  "  OEM-ASL-PCIe Address (0x%X)._REG (%d %d)  PCSA = %d\n", ec106000, 2, 1, 1, 0, 0

```
为了获取这些消息，你需要以 `CONFIG_ACPI_DEBUG` 编译，然后打开以下 ACPICA 跟踪参数。
这可以在内核命令行或运行时完成：

- `acpi.trace_method_name=\M460`
- `acpi.trace_state=method`

注意：这些在引导时可能非常嘈杂。如果你在内核命令行上打开这些参数，请同时考虑将 `CONFIG_LOG_BUF_SHIFT` 调大到更大的值（如 17），以避免丢失早期引导消息。

### 工具辅助解析（Tool assisted Parsing）


如上所述，手动解析可能很繁琐，尤其是在有大量消息时。为了帮助解决这个问题，已经创建了一个工具，位于
`amd-debug-tools <https://git.kernel.org/pub/scm/linux/kernel/git/superm1/amd-debug-tools.git/about/>`_，
用于帮助解析这些消息。

## 随机重启问题


当发生随机重启时，重启的高层原因存储在一个寄存器中，并会保留到下一次引导。

重启原因分为 6 类：
 - Software induced（软件引发）
 - Power state transition（电源状态转换）
 - Pin induced（引脚引发）
 - Hardware induced（硬件引发）
 - Remote reset（远程复位）
 - Internal CPU event（内部 CPU 事件）

   :header: "Bit", "Type", "Reason"
   :align: left

   "0",  "Pin",      "热引脚 BP_THERMTRIP_L 被触发"
   "1",  "Pin",      "电源按钮被按下了 4 秒"
   "2",  "Pin",      "关机引脚被触发"
   "4",  "Remote",   "接收到远程 ASF 关机命令"
   "9",  "Internal", "内部 CPU 热限制被触发"
   "16", "Pin",      "系统复位引脚 BP_SYS_RST_L 被触发"
   "17", "Software", "软件发出了 PCI 复位"
   "18", "Software", "软件向复位控制寄存器 0xCF9 写入了 0x4"
   "19", "Software", "软件向复位控制寄存器 0xCF9 写入了 0x6"
   "20", "Software", "软件向复位控制寄存器 0xCF9 写入了 0xE"
   "21", "ACPI-state", "发生了 ACPI 电源状态转换"
   "22", "Pin",      "键盘复位引脚 KB_RST_L 被触发"
   "23", "Internal", "发生了内部 CPU 关机事件"
   "24", "Hardware", "系统在失败启动定时器到期前未能引导"
   "25", "Hardware", "硬件看门狗定时器到期"
   "26", "Remote",   "接收到远程 ASF 复位命令"
   "27", "Internal", "一个未纠正错误导致了数据织物（data fabric）同步洪泛事件"
   "29", "Internal", "FCH 和 MP1 未能完成热复位握手"
   "30", "Internal", "发生了奇偶校验错误"
   "31", "Internal", "发生了软件同步洪泛事件"

此信息在内核引导时读取并打印到 syslog 中。当发生随机重启时，此消息有助于确定下一个要调试的组件。
