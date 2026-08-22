
## The PCI Express Advanced Error Reporting Driver Guide HOWTO


:作 - T. Long Nguyen <tom.l.nguyen@intel.com>
          - Yanmin Zhang <yanmin.zhang@intel.com>

:版权: |copy| 2006 Intel Corporation

## Overview


### About this guide


本指南描PCI Express（PCIe）高级错误报告（Advanced Error Reporting，AER）驱动的基础
知识，并提供有关如何使用它，以及如何使端点设备驱动符PCIe AER 驱动的信息


### What is the PCIe AER Driver?


PCIe 错误信号可以发生PCIe 链路本身上，也可以代表在链路上发起的事务。PCIe 定义了两
错误报告范式：基线能力（baseline capability）和高级错误报告能力。所PCIe 组件都必
提供基线能力，它定义了一组最小错误报告要求。高级错误报告能力通过 PCIe 高级错误报告扩展
能力结构实现，提供更健壮的错误报告

PCIe AER 驱动提供了支PCIe 高级错误报告能力的基础设施。PCIe AER 驱动提供三个基本功能

  - 如果发生错误，收集全面的错误信息
  - 向用户报告错误
  - 执行错误恢复操作

AER 驱动只挂接到支持 PCIe AER 能力Root Port RCEC 上


## User Guide


### Include the PCIe AER Root Driver into the Linux Kernel


PCIe AER 驱动是一个通过 PCIe Port Bus 驱动挂接Root Port 服务驱动。如果用户想使用它，
必须编译该驱动。它CONFIG_PCIEAER 启用，CONFIG_PCIEAER 依赖CONFIG_PCIEPORTBUS

### Load PCIe AER Root Driver


某些系统的固件中带有 AER 支持。在固件处理 AER 的同时启Linux AER 支持会导致不可预
的行为。因此，除非固件通过 ACPI _OSC 方法AER 控制权授予操作系统，否则 Linux 不处
AER 事件。有_OSC 用法的详细信息，请参PCI 固件规范

### AER error output


当捕获到 PCIe AER 错误时，会向控制台输出一条错误消息。如果是可纠正错误（correctable
error），则作为警告消息输出。否则，作为错误消息打印。因此用户可以选择不同的日志级别来
过滤掉可纠正错误消息

```

  0000:50:00.0: PCIe Bus Error: severity=Uncorrectable (Fatal), type=Transaction Layer, (Requester ID)
  0000:50:00.0:   device [8086:0329] error status/mask=00100000/00000000
  0000:50:00.0:    [20] UnsupReq               (First)
  0000:50:00.0:   TLP Header: 0x04000001 0x00200a03 0x05010000 0x00050100

```
在示例中，“Requester ID”指将错误消息发送给 Root Port 的设备的 ID。其他字段请参阅 PCIe
规范

“TLP Header”是引起错误TLP 的原始十六进制格式的前缀/头部。要TLP Header 解码为可
形式，可以使tlp-tool

https://github.com/mmpg-x86/tlp-tool

```

  curl -L https://git.kernel.org/linus/2ca1c94ce0b6 | rtlp-tool --aer

```
### AER Ratelimits


由于每个事务都可能产生错误消息，我们可能会看到大量报告的错误。为了防止多话的设备淹没
控制停滞执行，消息按设备和错误类型（可纠vs. 非致命不可纠正）进行限流。致命错
（包DPC 错误）不受速率限制

AER 使用默认的速率限制：DEFAULT_RATELIMIT_BURST0 个事件）DEFAULT_RATELIMIT_INTERVAL
 秒）内

速率限制sysfs 属性的形式暴露，并且可配置。请参阅
Documentation/ABI/testing/sysfs-bus-pci-devices-aer銆。

### AER Statistics / Counters


当捕获到 PCIe AER 错误时，计数统计信息也以 sysfs 属性的形式暴露，记录于
Documentation/ABI/testing/sysfs-bus-pci-devices-aer銆。

## Developer Guide


要启用错误恢复，软件驱动必须提供回调函数

为了更好地理AER，开发者需要了AER 的工作原理

PCIe 错误分为两类：可纠正错误和不可纠正错误。这种分类基于这些错误的影响，可能导致性能
下降或功能失效

可纠正错误对接口的功能没有任何影响。PCIe 协议可以在不需要任何软件干预或任何数据丢失
情况下恢复。这些错误由硬件检测并纠正

与可纠正错误不同，不可纠正错误会影响接口的功能。不可纠正错误可能导致特定事务或特定 PCIe
链路不可靠。根据这些错误状况，不可纠正错误进一步分为非致命错误（non-fatal error）和致命
错误（fatal error）。非致命错误导致特定事务不可靠，PCIe 链路本身完全正常。另一方面
致命错误导致链路不可靠

当启PCIe 错误报告时，设备捕获到错误后会自动向上面Root Port 发送一条错误消息。Root
Port 在收到错误报告消息后，会在其 AER 能力结构中内部处理并记录该错误消息。被记录的错
信息包括将错误报告代理的 Requester ID 存入错误源识别寄存器，并相应地设Root Error
Status 寄存器的错误位。如果在 Root Error Command 寄存器中启用AER 错误报告，Root Port
在检测到错误时会生成一个中断

注意，上述错误与 PCIe 层级结构和链路有关。这些错误不包括任何设备特定的错误，因为设备特定
错误仍会直接发送给设备驱动

### Provide callbacks


#### PCI error-recovery callbacks


PCIe AER Root 驱动在执行错误恢复操作时，使用错误回调来与所涉及层级结构中的下游设备驱动
协调

数据结构 pci_driver 有一个指err_handler，指pci_error_handlers，后者由几个回调函数
指针组成。除PCIe 特定的部分外（见下文），AER 驱动遵循 pci-error-recovery.rst 中定义的
规则。有关回调的详细定义，请参阅 pci-error-recovery.rst

以下各节说明了何时调用错误回调函数

#### Correctable errors


可纠正错误对接口的功能没有任何影响。PCIe 协议可以在不需要任何软件干预或任何数据丢失
情况下恢复。这些错误不需要任何恢复操作。AER 驱动相应地清除设备的可纠正错误状态寄存器，并
记录这些错误

#### Uncorrectable (non-fatal and fatal) errors


AER 驱动执行一Secondary Bus Reset（次级总线复位）以从不可纠正错误中恢复。复位应用于
发起设备之上的端口：如果发起设备是一个端点（Endpoint），则只复位该端点。另一方面，如果发
设备有从属设备，那些设备也会全部受到复位影响

如果发起设备是一Root Complex Integrated Endpoint（根复合体集成端点），则没有可以应用
Secondary Bus Reset 的端口之上。在这种情况下，AER 驱动改为应用 Function Level Reset（功
级复位）

如果错误消息指示非致命错误，则不需要在上游执行复位。AER 驱动向某个层级结构中关联的所
驱动调用 error_detected(dev, pci_channel_io_normal)
```

  Endpoint <==> Downstream Port B <==> Upstream Port A <==> Root Port

```
如果 Upstream Port A 捕获了一AER 错误，则该层级结构由 Downstream Port B Endpoint 组成

驱动可以返回 PCI_ERS_RESULT_CAN_RECOVER、PCI_ERS_RESULT_DISCONNECT 
PCI_ERS_RESULT_NEED_RESET，具体取决于它是否可以在不复位的情况下恢复、认为设备不可恢复，
需要复位才能恢复。如果所有受影响的驱动都同意可以在不复位的情况下恢复，则跳过复位。只要有一
个驱动请求复位，就会覆盖所有其他驱动

如果错误消息指示致命错误，内核将向某个层级结构中的所有驱动广error_detected(dev,
pci_channel_io_frozen)。然后，必须在上游执行复位。如error_detected 返回
PCI_ERS_RESULT_CAN_RECOVER 表示可以在不复位的情况下恢复，错误处理将进入 mmio_enabled，但
之后仍会执行复位

换句话说，对于非致命错误，驱动可以选择进行复位。但对于致命错误，基于链路不可靠的假设，它们
不能选择不进行复位

### Frequently Asked Questions


问：
  如果 PCIe 设备驱动没有提供错误恢复处理程序（pci_driver->err_handler 等于 NULL），
  发生什么？

答：
  与该驱动关联的设备将无法被恢复。内核将打印出信息性消息来识别不可恢复的设备


## Software error injection


调试 PCIe AER 错误恢复代码相当困难，因为很难触发真实的硬件错误。可以使用基于软件的错误注入
来伪造各PCIe 错误

首先你应在内核配置中启用 PCIe AER 软件错误注入，即你的 .config 中应包含以下项

CONFIG_PCIEAER_INJECT=y or CONFIG_PCIEAER_INJECT=m

用新内核重启或插入模块后，应创建一个名/dev/aer_inject 的设备文件

然后，你需要一个名aer-inject 的用户空间工具，可从以下地址获取

    https://github.com/intel/aer-inject.git

有关 aer-inject 的更多信息可在其源代码中的文档找到
