## PCI Power Management


Copyright (c) 2010 Rafael J. Wysocki <rjw@sisk.pl>, Novell Inc.

关于 PCI 电源管理相关概念以及 Linux 内核接口的一个概览。基于 Patrick Mochel
<mochel@transmeta.com>（及其他人）此前的工作。

本文档只涵盖 PCI 设备特有的电源管理方面的内容。有关内核设备电源管理接口的
总体说明，请参阅 Documentation/driver-api/pm/devices.rst 和
Documentation/power/runtime_pm.rst。

   1. Hardware and Platform Support for PCI Power Management
   2. PCI Subsystem and Device Power Management
   3. PCI Device Drivers and Power Management
   4. Resources

## 1. Hardware and Platform Support for PCI Power Management


### 1.1. Native and Platform-Based Power Management


一般来说，电源管理是一项通过把设备置于功耗更低（低功耗状态）的状态中，以
牺牲部分功能或性能为代价来节省能量的特性。

通常，当一个设备利用率不足或完全不活动时，它会被置于低功耗状态。然而，当
有必要再次使用该设备时，必须将它恢复回“完全可用”的状态（全功耗状态）。这
可能发生在有数据需要该设备处理时，也可能是由于某个需要该设备处于活动状态的
外部事件（该事件可能由设备自身发出信号）所引起。

PCI 设备可以通过两种方式被置于低功耗状态：使用由 PCI 总线电源管理接口规范
（PCI Bus Power Management Interface Specification）引入的设备能力，或者借助
平台固件（例如 ACPI BIOS）的帮助。对于第一种方式，在下文中称为原生 PCI 电源
管理（native PCI PM），设备电源状态的改变是通过向它的某个标准配置寄存器写入
特定值来实现的。第二种方式需要平台固件提供特殊的方法，供内核用来改变设备的
电源状态。

支持原生 PCI PM 的设备通常可以产生称为电源管理事件（Power Management
Events，PME）的唤醒信号，通知内核有需要设备处于活动状态的外部事件发生。在内核
收到一个 PME 后，应当将被发送该 PME 的设备置于全功耗状态。然而，PCI 总线电源
管理接口规范并没有定义任何将 PME 从设备传送到 CPU 和操作系统内核的标准方法。
这被视为由平台固件来完成该任务，因此，即使一个 PCI 设备被设置为产生 PME，往往
也还需要准备平台固件，以便将来自该设备的 PME 通知给 CPU（例如通过产生中断）。

反过来，如果使用平台固件提供的方法来改变设备的电源状态，通常平台也会提供用于
准备设备产生唤醒信号的方法。然而在那种情况下，由于平台提供的方法依赖于此，往往
还需要使用原生 PCI PM 机制来准备设备产生 PME。

因此，在许多情况下，必须同时使用原生和基于平台的电源管理机制，才能获得期望的
结果。

### 1.2. Native PCI Power Management


PCI 总线电源管理接口规范（PCI PM Spec）是在 PCI 2.1 与 PCI 2.2 规范之间引入的。
它定义了一组用于执行各种电源管理相关操作的标准接口。

PCI PM Spec 的实现对于普通 PCI 设备是可选的，但对于 PCI Express 设备则是必须的。
如果一个设备支持 PCI PM Spec，它在 PCI 配置空间中就有一个 8 字节的电源管理能力
字段。该字段用于描述并控制与原生 PCI 电源管理相关的标准特性。

PCI PM Spec 为设备（D0-D3）和总线（B0-B3）定义了 4 种工作状态。数字越大，设备
或总线在该状态下消耗的功耗越少。然而，数字越大，设备 or 总线返回到全功耗状态
（分别为 D0 或 B0）的延迟也越长。

规范定义了 D3 状态的两种变体。第一种是 D3hot，称为软件可访问的 D3，因为设备可以
被编程进入它。第二种 D3cold，是当设备被移除供电电压（Vcc）时所处状态。无法将 PCI
设备编程进入 D3cold，尽管可能存在一个可编程接口，用于将设备所在的总线置于一个
移除总线上所有设备 Vcc 的状态。

不过，PCI 总线电源管理在撰写本文档时尚未被 Linux 内核支持，因此本文档不涵盖它。

注意，每个 PCI 设备都可以处于全功耗状态（D0）或 D3cold，无论它是否实现了 PCI PM
Spec。除此之外，如果设备实现了 PCI PM Spec，它还必须支持 D3hot 以及 D0。对 D1 和
D2 电源状态的支持是可选的。

支持 PCI PM Spec 的 PCI 设备可以被编程进入任何受支持的低功耗状态（D3cold 除外）。
在 D1-D3hot 状态下，设备的标准配置寄存器必须对软件可访问（即要求设备响应 PCI
配置访问），尽管此时它的 I/O 和内存空间已被禁用。这使得设备可以被编程进入 D0。
因此，内核可以在 D0 和受支持的低功耗状态（D3cold 除外）之间来回切换设备，设备
可能经历的状态转换如下：

+----------------------------+
| Current State | New State  |
+----------------------------+
| D0            | D1, D2, D3 |
+----------------------------+
| D1            | D2, D3     |
+----------------------------+
| D2            | D3         |
+----------------------------+
| D1, D2, D3    | D0         |
+----------------------------+

从 D3cold 到 D0 的转换发生在向设备提供供电电压时（即恢复供电）。在这种情况下，
设备通过完整的上电复位序列返回到 D0，并且像初始上电时一样，由硬件将上电默认值
恢复到设备。

支持 PCI PM Spec 的 PCI 设备可以被编程为在任何电源状态（D0-D3）下产生 PME，但它们
并不要求能够从所有受支持的电源状态产生 PME。特别是，从 D3cold 产生 PME 的能力是
可选的，并且取决于是否存在允许设备保持足够活动以产生唤醒信号的附加电压
（3.3Vaux）。

### 1.3. ACPI Device Power Management


平台固件对 PCI 设备电源管理的支持是系统相关的。然而，如果所讨论的系统符合高级
配置与电源接口（Advanced Configuration and Power Interface，ACPI）规范（如大多数
基于 x86 的系统），它应当实现 ACPI 标准定义的设备电源管理接口。

为此，ACPI BIOS 提供了一些称为“控制方法”（control methods）的特殊函数，内核可以
执行它们来执行特定任务，例如将设备置于低功耗状态。这些控制方法使用一种称为 ACPI
机器语言（AML）的特殊字节码语言编码，并存储在机器的 BIOS 中。内核从 BIOS 加载它们，
并在需要时通过一个 AML 解释器执行，该解释器将 AML 字节码转换为计算和内存或 I/O
空间访问。这样，在理论上，BIOS 编写者可以为内核提供一种以系统相关的方式、依据
系统设计执行操作的方法。

ACPI 控制方法可以分为全局控制方法（不与任何特定设备关联）和设备控制方法（必须为
每个要借助平台处理的设备分别定义）。这意味着，特别是 ACPI 设备控制方法只能用于处理
BIOS 编写者事先已知的设备。用于设备电源管理的 ACPI 方法就属于这一类。

ACPI 规范假定设备可以处于 D0、D1、D2 和 D3 四种电源状态之一，这些状态大致对应于原生
PCI PM 的 D0-D3 状态（尽管 ACPI 没有考虑 D3hot 与 D3cold 之间的差异）。此外，对于
设备的每个电源状态，都有一组必须启用才能将设备置于该状态的电源资源。这些电源资源
使用它们各自的控制方法 _ON 和 _OFF 来控制（即启用或禁用），这些方法必须分别针对每个
电源资源单独定义。

要将设备置于 ACPI 电源状态 Dx（其中 x 是 0 到 3 之间的数字），内核应当 (1) 使用该
电源资源各自的 _ON 控制方法启用设备在该状态下所需的电源资源，并且 (2) 执行为该设备
定义的 _PSx 控制方法。除此之外，如果设备将进入低功耗状态（D1-D3）并应当从该状态产生
唤醒信号，则必须在 _PSx 之前执行为其定义的 _DSW（或 _PSW，在 ACPI 3.0 中被 _DSW
取代）控制方法。在目标电源状态下设备不再需要、且不再被任何其他设备需要的电源资源应当
被禁用（通过执行它们的 _OFF 控制方法）。如果设备当前的电源状态是 D3，则只能以这种方式
进入 D0。

然而，设备的电源状态经常在系统范围的转换（进入睡眠状态或返回工作状态）期间被改变。
ACPI 定义了四种系统睡眠状态 S1、S2、S3 和 S4，并将系统工作状态记为 S0。一般来说，目标
系统睡眠（或工作）状态决定了设备可以被置于的最高功耗（最低编号）状态，内核应当通过
执行设备的 _SxD 控制方法（其中 x 是 0 到 4 之间的数字）来获取此信息。如果要求设备从
目标睡眠状态唤醒系统，则它可以进入的最低功耗（最高编号）状态也由系统的目标状态决定。
内核随后应当使用设备的 _SxW 控制方法来获取该状态的编号。它还应当使用设备的 _PRW 控制
方法来了解需要启用哪些电源资源，才能使设备能够产生唤醒信号。

### 1.4. Wakeup Signaling


PCI 设备产生的唤醒信号，无论是作为原生 PCI PME，还是作为在进入低功耗状态之前执行 _DSW
（或 _PSW）ACPI 控制方法的结果，都必须被捕获并作适当处理。如果它们是在系统处于工作状态
（ACPI S0）时发送的，则应当被转换为中断，以便内核将产生它们的设备置于全功耗状态，并处理
触发它们的事件。反过来，如果它们是在系统睡眠时发送的，则应当导致系统的核心逻辑触发唤醒。

在基于 ACPI 的系统上，普通 PCI 设备发送的唤醒信号会被转换为 ACPI 通用事件（GPE），GPE 是
系统核心逻辑为响应各种需要处理事件而产生的硬件信号。每个 GPE 都与一个或多个潜在相关事件源
关联。特别地，一个 GPE 可以与一个能够发出唤醒信号的 PCI 设备关联。关于 GPE 与事件源之间
连接的信息记录在系统的 ACPI BIOS 中，内核可以从中读取。

如果一个系统 ACPI BIOS 已知的 PCI 设备发出唤醒信号，与它关联的 GPE（如果存在）就会被触发。
与 PCI 桥关联的 GPE 也可能作为对桥下某个设备发出的唤醒信号的响应而被触发（对于根桥也是如此），
并且例如来自系统 ACPI BIOS 未知的设备的原生 PCI PME 就可以通过这种方式处理。

一个 GPE 可能在系统睡眠时（即处于 ACPI S1-S4 某个状态时）被触发，在这种情况下，系统唤醒由其
核心逻辑启动（导致系统唤醒发生的信号源设备之后可能被识别）。用于这种情况的 GPE 称为唤醒 GPE
（wakeup GPE）。

然而，通常 GPE 也会在系统处于工作状态（ACPI S0）时被触发，此时系统的核心逻辑会产生一个系统
控制中断（SCI）来通知内核该事件。接着，SCI 处理程序识别出导致该中断产生的 GPE，进而使内核能够
识别事件的来源（可能是一个发出唤醒信号的 PCI 设备）。用于在系统处于工作状态时通知内核事件的
GPE 称为运行时 GPE（runtime GPE）。

遗憾的是，在非 ACPI 系统上处理普通 PCI 设备发送的唤醒信号没有标准方法，但 PCI Express 设备有
一种方法。即，PCI Express 基础规范引入了一种原生机制，用于将原生 PCI PME 转换为由根端口（root
port）产生的中断。对于普通 PCI 设备，原生 PME 是带外（out-of-band）的，因此它们被单独路由，并且
不需要穿过桥（原则上它们可能被直接路由到系统的核心逻辑），但对于 PCI Express 设备，它们是带内
消息，必须穿过 PCI Express 层次结构，包括从设备到根复合体（Root Complex）路径上的根端口。因此，
可以引入一种机制，使得根端口在收到来自其下某个设备的 PME 消息时产生一个中断。发送该 PME 消息的
设备的 PCI Express 请求者 ID（Requester ID）随后被记录在根端口的某个配置寄存器中，中断处理程序
可以从中读取以识别该设备。[与根复合体集成的 PCI Express 端点发送的 PME 消息不穿过根端口，而是
导致根复合体事件收集器（Root Complex Event Collector，如果有的话）产生中断。]

原则上，原生 PCI Express PME 信号也可以与 GPE 一起在基于 ACPI 的系统上使用，但内核为此必须请求
系统的 ACPI BIOS 释放对根端口配置寄存器的控制权。然而，ACPI BIOS 并不要求允许内核控制这些寄存器，
如果它不这样做，内核就绝不能修改它们的内容。当然，在这种情况下内核无法使用原生 PCI Express PME
信号。

## 2. PCI Subsystem and Device Power Management


### 2.1. Device Power Management Callbacks


PCI 子系统以多种方式参与 PCI 设备的电源管理。首先，它在设备电源管理核心（PM core）与
PCI 设备驱动之间提供了一个中间代码层。具体来说，PCI 子系统的 struct bus_type 对象
pci_bus_type 的 pm 字段指向一个 struct dev_pm_ops 对象 pci_dev_pm_ops，其中包含：
```
  const struct dev_pm_ops pci_dev_pm_ops = {
	.prepare = pci_pm_prepare,
	.complete = pci_pm_complete,
	.suspend = pci_pm_suspend,
	.resume = pci_pm_resume,
	.freeze = pci_pm_freeze,
	.thaw = pci_pm_thaw,
	.poweroff = pci_pm_poweroff,
	.restore = pci_pm_restore,
	.suspend_noirq = pci_pm_suspend_noirq,
	.resume_noirq = pci_pm_resume_noirq,
	.freeze_noirq = pci_pm_freeze_noirq,
	.thaw_noirq = pci_pm_thaw_noirq,
	.poweroff_noirq = pci_pm_poweroff_noirq,
	.restore_noirq = pci_pm_restore_noirq,
	.runtime_suspend = pci_pm_runtime_suspend,
	.runtime_resume = pci_pm_runtime_resume,
	.runtime_idle = pci_pm_runtime_idle,
  };
```
这些回调由 PM core 在与设备电源管理相关的各种情况下执行，而它们又执行 PCI 设备驱动提供的
电源管理回调。它们还执行一些涉及 PCI 设备标准配置寄存器的电源管理操作，这些操作设备驱动无需
了解也无需关心。

表示 PCI 设备的结构体 struct pci_dev 包含若干字段：
```
  struct pci_dev {
	...
	pci_power_t     current_state;  /* Current operating state. */
	int		pm_cap;		/* PM capability offset in the
					   configuration space */
	unsigned int	pme_support:5;	/* Bitmask of states from which PME#
					   can be generated */
	unsigned int	pme_poll:1;	/* Poll device's PME status bit */
	unsigned int	d1_support:1;	/* Low power state D1 is supported */
	unsigned int	d2_support:1;	/* Low power state D2 is supported */
	unsigned int	no_d1d2:1;	/* D1 and D2 are forbidden */
	unsigned int	wakeup_prepared:1;  /* Device prepared for wake up */
	unsigned int	d3hot_delay;	/* D3hot->D0 transition time in ms */
	...
  };
```
它们还间接地使用了嵌入在 struct pci_dev 中的 struct device 的某些字段。

### 2.2. Device Initialization


PCI 子系统与设备电源管理相关的第一项任务是为电源管理准备设备，并初始化为此目的而使用的
struct pci_dev 字段。这发生在 drivers/pci/ 中定义的两个函数 pci_pm_init() 和
pci_acpi_setup() 中。

其中第一个函数检查设备是否支持原生 PCI PM，如果是，则将其电源管理能力结构在配置空间中的
偏移量存储在设备的 struct pci_dev 对象的 pm_cap 字段中。

接下来，该函数检查设备支持哪些 PCI 低功耗状态，以及设备可以从哪些低功耗状态
产生原生 PCI PME。设备的 struct pci_dev 以及嵌入其中的 struct device 的电源管理
字段会相应更新，并且设备产生 PME 的功能被禁用。

第二个函数检查设备是否可以在平台固件（例如 ACPI BIOS）的帮助下被准备为发出
唤醒信号。如果是，该函数会更新嵌入在设备的 struct pci_dev 中的 struct device
的唤醒字段，并使用固件提供的方法来阻止设备发出唤醒信号。

至此，设备已为电源管理做好准备。然而对于无驱动设备，该功能仅限于在系统范围的
转换（进入睡眠状态并返回工作状态）期间执行的一些基本操作。

### 2.3. Runtime Device Power Management


PCI 子系统在 PCI 设备的运行时电源管理中扮演着至关重要的角色。为此它使用了
Documentation/power/runtime_pm.rst 中描述的通用运行时电源管理（runtime PM）框架。
```
	pci_pm_runtime_suspend()
	pci_pm_runtime_resume()
	pci_pm_runtime_idle()
```
这些由核心运行时 PM 例程执行。它还实现了处理处于低功耗状态的 PCI 设备运行时唤醒
信号所需的全部机制，在撰写本文档时，该机制同时适用于第 1 节中描述的原生 PCI
Express PME 信号和基于 ACPI GPE 的唤醒信号。

首先，一个 PCI 设备借助 pm_schedule_suspend() 或 pm_runtime_suspend() 被置于低
功耗状态，或者说被挂起，而对于 PCI 设备，这两个函数会调用
pci_pm_runtime_suspend() 来完成实际工作。要使其工作，设备的驱动必须提供一个
pm->runtime_suspend() 回调（见下文），该回调由 pci_pm_runtime_suspend() 作为
第一步执行。如果驱动的回调成功返回，则设备的标准配置寄存器被保存，设备被准备为
发出唤醒信号，最后它被置于目标低功耗状态。

将设备置入的低功耗状态是它能够从中发出唤醒信号的最低功耗（最高编号）状态。发出
唤醒信号的具体方法是系统相关的，由 PCI 子系统根据设备的报告能力和平台固件来确定。
为了准备设备发出唤醒信号并将其置于所选的低功耗状态，PCI 子系统可以使用平台固件，
也可以使用设备原生的 PCI PM 能力（如果支持的话）。

期望设备驱动的 pm->runtime_suspend() 回调不会尝试准备设备发出唤醒信号或将其置于
低功耗状态。驱动应当把这些任务留给 PCI 子系统，因为它拥有执行这些任务所需的全部
信息。

一个被挂起的设备借助 pm_request_resume() 或 pm_runtime_resume() 被带回“活动”状态，
或者说被恢复，而这两个函数对 PCI 设备都会调用 pci_pm_runtime_resume()。同样，这只有
在设备的驱动提供了 pm->runtime_resume() 回调（见下文）时才有效。然而，在执行驱动的
回调之前，pci_pm_runtime_resume() 会将设备带回全功耗状态，阻止它在那一状态下发出
唤醒信号，并恢复其标准配置寄存器。因此驱动的回调无需担心设备恢复中与 PCI 相关的
方面。

注意，一般来说 pci_pm_runtime_resume() 可能在两种不同情况下被调用。首先，它可能在
设备驱动请求时被调用，例如当有数据需要处理时。其次，它可能作为设备自身发出的唤醒
信号（有时称为“远程唤醒”，remote wakeup）的结果而被调用。当然，为此目的，该唤醒信号
会按第 1 节中描述的某种方式处理，并在源设备被识别之后最终转换为对 PCI 子系统的一个
通知。

由 pm_runtime_idle() 和 pm_request_idle() 为 PCI 设备调用的 pci_pm_runtime_idle()
函数，会执行设备驱动的 pm->runtime_idle() 回调（如果已定义），并且如果该回调没有返回
错误码（或根本不存在），则借助 pm_runtime_suspend() 挂起设备。有时 pci_pm_runtime_idle()
会被 PM core 自动调用（例如，它在设备刚刚被恢复之后就被调用），在这种情况下期望在合理时
挂起设备。然而，通常 PCI 子系统并不真正知道设备是否真的可以被挂起，因此它通过运行设备的
pm->runtime_idle() 回调来让设备驱动做决定。

### 2.4. System-Wide Power Transitions


有几类不同的系统范围电源转换，在 Documentation/driver-api/pm/devices.rst 中有描述。
每一类都要求以特定方式处理设备，PM core 为此执行子系统级的电源管理回调。它们分阶段执行，
使得每个阶段在下一阶段开始之前，对属于给定子系统的每个设备都执行相同的子系统级回调。
这些阶段总是在任务被冻结之后运行。

##### 2.4.1. System Suspend


当系统进入一个会保留内存内容的睡眠状态（例如 ACPI 睡眠状态 S1-S3 之一）时，各阶段为：

	prepare, suspend, suspend_noirq

```
	pci_pm_prepare()
	pci_pm_suspend()
	pci_pm_suspend_noirq()
```
pci_pm_prepare() 例程首先借助 pm_runtime_resume() 将设备置于“完全可用”状态。然后，它
执行设备驱动的 pm->prepare() 回调（如果已定义，即如果驱动的 struct dev_pm_ops 对象存在
且其中的 prepare 指针有效）。

pci_pm_suspend() 例程首先检查设备的驱动是否实现了传统的（legacy）PCI 挂起例程（见第 3 节），
如果是，则执行驱动的 legacy suspend 回调（如果存在）并返回其结果。接下来，如果设备的驱动
没有提供 struct dev_pm_ops 对象（包含指向驱动回调的指针），则调用
pci_pm_default_suspend()，它只是关闭设备的总线主导（bus master）能力并运行
pcibios_disable_device() 将其禁用，除非该设备是桥（PCI 桥被该例程忽略）。接下来，执行设备
驱动的 pm->suspend() 回调（如果已定义），如果失败则返回其结果。最后，调用 pci_fixup_device()
在必要时应用与该设备相关的硬件挂起修正（quirks）。

注意，挂起阶段对 PCI 设备是异步执行的，因此对于任意一对彼此没有已知依赖关系的 PCI 设备
（即设备树中从根桥到叶子设备的路径都不同时包含这两个设备），pci_pm_suspend() 回调可以并行
执行。

pci_pm_suspend_noirq() 例程在 suspend_device_irqs() 被调用之后执行，这意味着在该例程运行期间
不会调用设备驱动的中断处理程序。它首先检查设备的驱动是否实现了传统的 PCI 挂起例程（第 3 节），
如果是，则调用 legacy late suspend 例程并返回其结果（如果驱动的回调尚未保存设备的标准配置
寄存器，则保存它们）。其次，如果设备驱动的 struct dev_pm_ops 对象不存在，则保存设备的标准
配置寄存器并返回成功。否则执行设备驱动的 pm->suspend_noirq() 回调（如果存在），如果失败则返回
其结果。接下来，如果设备的标准配置寄存器尚未被保存（之前执行的某个驱动回调可能已经保存），则
pci_pm_suspend_noirq() 会保存它们，准备设备发出唤醒信号（如有必要）并将其置于低功耗状态。

将设备置入的低功耗状态是它在系统处于目标睡眠状态时能从中发出唤醒信号的最低功耗（最高编号）
状态。与上面描述的运行时 PM 情况一样，发出唤醒信号的机制是系统相关的，由 PCI 子系统决定，
该子系统还负责适当地准备设备从系统的目标睡眠状态发出唤醒信号。

PCI 设备驱动（未实现传统电源管理回调的）通常不应被期望去准备设备发出唤醒信号或将其置于低功耗
状态。然而，如果驱动的某个挂起回调（pm->suspend() 或 pm->suspend_noirq()）保存了设备的标准
配置寄存器，pci_pm_suspend_noirq() 会假定设备已由驱动准备为发出唤醒信号并置于低功耗状态（此时
假定驱动使用了 PCI 子系统为此提供的辅助函数）。并不鼓励 PCI 设备驱动这样做，但在某些罕见情况下
在驱动中这样做可能是最优方案。

##### 2.4.2. System Resume


当系统从保留了内存内容的睡眠状态（例如 ACPI 睡眠状态 S1-S3 之一）转换回工作状态（ACPI S0）
时，各阶段为：

	resume_noirq, resume, complete

分别执行 PCI 总线类型的以下回调：
```
	pci_pm_resume_noirq()
	pci_pm_resume()
	pci_pm_complete()
```
pci_pm_resume_noirq() 例程首先将设备置于全功耗状态，恢复其标准配置寄存器，并在必要时应用与
设备相关的早期恢复硬件修正。这无条件地执行，无论设备的驱动是否实现了传统 PCI 电源管理回调（这样
所有 PCI 设备在恢复期间中断处理程序首次被调用时都处于全功耗状态且它们的标准配置寄存器已被恢复，
从而使内核能够避免由设备仍处于挂起状态的驱动处理共享中断所带来的问题）。如果设备驱动实现了传统的
PCI 电源管理回调（见第 3 节），则执行 legacy early resume 回调并返回其结果。否则，执行设备驱动的
pm->resume_noirq() 回调（如果已定义）并返回其结果。

pci_pm_resume() 例程首先检查设备的标准配置寄存器是否已恢复，如果没有则恢复它们（这只在失败的
挂起过程中的错误路径中才是必要的）。接下来，在必要时应用与设备相关的恢复硬件修正，并且如果设备的
驱动实现了传统 PCI 电源管理回调（见第 3 节），则执行驱动的 legacy resume 回调并返回其结果。否则，
阻塞设备的唤醒信号机制，并执行其驱动的 pm->resume() 回调（如果已定义，然后返回该回调的结果）。

恢复阶段对 PCI 设备是异步执行的，与上面描述的挂起阶段一样，这意味着如果两个 PCI 设备彼此没有已知
的依赖关系，则可以并行对两者执行 pci_pm_resume() 例程。

pci_pm_complete() 例程只执行设备驱动的 pm->complete() 回调（如果已定义）。

##### 2.4.3. System Hibernation


系统休眠（hibernation）比系统挂起更复杂，因为它需要将系统映像（image）创建并写入持久存储介质。
该映像以原子方式创建，并且在此之前所有设备都被静默（quiesced），或者说被冻结（frozen）。

设备的冻结在释放了足够内存之后进行（在撰写本文档时，创建映像要求至少 50% 的系统 RAM 空闲），分为
以下三个阶段：

	prepare, freeze, freeze_noirq

```
	pci_pm_prepare()
	pci_pm_freeze()
	pci_pm_freeze_noirq()
```
这意味着 prepare 阶段与系统挂起完全相同。但另外两个阶段不同。

pci_pm_freeze() 例程与 pci_pm_suspend() 十分相似，但它运行设备驱动的 pm->freeze() 回调（如果已
定义）而不是 pm->suspend()，并且它不应用与挂起相关的硬件修正。对于彼此没有已知依赖关系的不同 PCI
设备，它是异步执行的。

反过来，pci_pm_freeze_noirq() 例程与 pci_pm_suspend_noirq() 相似，但它调用设备驱动的
pm->freeze_noirq() 例程而不是 pm->suspend_noirq()。它也不尝试准备设备发出唤醒信号并将其置于低功耗
状态。不过，如果标准配置寄存器尚未被某个驱动回调保存，它会保存它们。

一旦映像被创建，就必须将其保存。然而此时所有设备都被冻结并且无法处理 I/O，而它们处理 I/O 的能力
显然对于保存映像是必要的。因此它们必须被带回完全可用的状态，这通过以下阶段完成：

	thaw_noirq, thaw, complete

```
	pci_pm_thaw_noirq()
	pci_pm_thaw()
	pci_pm_complete()
```
分别地。

其中第一个，pci_pm_thaw_noirq()，类似于 pci_pm_resume_noirq()。它将设备置于全功耗状态并恢复其标准
配置寄存器。它还执行设备驱动的 pm->thaw_noirq() 回调（如果已定义）而不是 pm->resume_noirq()。

pci_pm_thaw() 例程类似于 pci_pm_resume()，但它运行设备驱动的 pm->thaw() 回调而不是 pm->resume()。
对于彼此没有已知依赖关系的不同 PCI 设备，它是异步执行的。

complete 阶段与系统恢复相同。

在保存映像之后，在进入目标睡眠状态（基于 ACPI 的系统为 ACPI S4）之前，需要关闭设备电源。这分三个
阶段完成：

	prepare, poweroff, poweroff_noirq

其中 prepare 阶段与系统挂起完全相同。另外两个阶段分别类似于 suspend 和 suspend_noirq 阶段。
```
	pci_pm_poweroff()
	pci_pm_poweroff_noirq()
```
分别类似于 pci_pm_suspend() 和 pci_pm_suspend_noirq()，尽管它们不尝试保存设备的标准配置寄存器。

##### 2.4.4. System Restore


系统恢复需要把一个休眠映像加载到内存中，并在恢复休眠前系统活动之前恢复休眠前的内存内容。

如 Documentation/driver-api/pm/devices.rst 所述，休眠映像由内核的一个新实例（称为引导内核，boot
kernel）加载到内存中，而引导内核又由引导加载器以通常方式加载并运行。在引导内核加载了映像之后，它
需要用自己的代码和数据替换映像中存储的“已休眠”内核（称为映像内核，image kernel）的代码和数据。为此，
所有设备都像休眠期间创建映像之前一样被冻结，分为

	prepare, freeze, freeze_noirq

上述阶段。然而，受这些阶段影响的设备只是那些在引导内核中有驱动的设备；其他设备仍将处于引导加载器
遗留给它们的任何状态。

如果恢复休眠前内存内容的操作失败，引导内核将经历上述的“解冻”（thawing）过程，使用 thaw_noirq、thaw
和 complete 阶段（那只会影响到在引导内核中有驱动的设备），然后继续正常运行。

如果休眠前内存内容被成功恢复（这是通常的情况），控制权被传递给映像内核，它随后负责将系统带回工作状态。
为此，它必须恢复设备的休眠前功能，这做得与从内存睡眠状态唤醒很像，尽管它涉及不同的阶段：

	restore_noirq, restore, complete

其中前两个阶段分别类似于上面描述的 resume_noirq 和 resume 阶段，并对应于 PCI 子系统的以下回调：
```
	pci_pm_restore_noirq()
	pci_pm_restore()
```

它们分别执行设备驱动的 pm->restore_noirq() 和 pm->restore() 回调（如果可用）。

complete 阶段执行的方式与系统恢复期间完全相同。

## 3. PCI Device Drivers and Power Management


### 3.1. Power Management Callbacks


PCI 设备驱动通过提供由上述 PCI 子系统电源管理例程执行的回调，以及通过控制其设备的
运行时电源管理，来参与电源管理。

在撰写本文档时，为 PCI 设备驱动定义电源管理回调有两种方式：推荐的一种基于使用
Documentation/driver-api/pm/devices.rst 中描述的 dev_pm_ops 结构体，另一种是“传统”
（legacy）方式，即使用 struct pci_driver 中的 .suspend() 和 .resume() 回调。然而，传统
方式不允许定义运行时电源管理回调，并且对任何新驱动都不太合适。因此本文档不涵盖它
（请参阅源代码以了解更多信息）。

建议所有 PCI 设备驱动都定义一个 struct dev_pm_ops 对象，其中包含将在各种情况下由 PCI
子系统 PM 例程执行的电源管理（PM）回调指针。必须将该驱动的 struct dev_pm_ops 对象的指针
赋给其 struct pci_driver 对象中的 driver.pm 字段。一旦这样做，struct pci_driver 中的“传统”
PM 回调就会被忽略（即使它们不为 NULL）。

struct dev_pm_ops 中的 PM 回调不是强制的，如果它们没有被定义（即 struct dev_pm_ops 的相应
字段未设置），PCI 子系统将以简化的默认方式处理设备。但如果它们被定义了，则期望它们的行为
如以下小节所述。

##### 3.1.1. prepare()


prepare() 回调在系统挂起期间、休眠期间（当即将创建休眠映像时）、保存休眠映像后的关机期间，
以及系统恢复期间（当休眠映像刚刚被加载到内存中时）执行。

只有当驱动的设备有通常可能随时被注册的子设备时，才需要这个回调。在这种情况下，prepare()
回调的作用是阻止该设备的新子设备被注册，直到 resume_noirq()、thaw_noirq() 或 restore_noirq()
回调之一被运行为止。

除此之外，prepare() 回调可以执行一些准备设备被挂起的操作，尽管它不应分配内存（如果挂起设备
需要额外内存，必须提前预分配，例如在 Documentation/driver-api/pm/notifiers.rst 中描述的
suspend/hibernate notifier 中）。

##### 3.1.2. suspend()


suspend() 回调只在系统挂起期间执行，在系统中所有设备的 prepare() 回调都已执行之后。

期望这个回调使设备静默（quiesce）并准备由 PCI 子系统将其置于低功耗状态。PCI 驱动的 suspend()
回调并不要求（实际上甚至不推荐）去保存设备的标准配置寄存器、准备它唤醒系统或将其置于低功耗
状态。所有这些操作都可以很好地由 PCI 子系统在无需驱动参与的情况下完成。

然而，在少数罕见情况下，在 PCI 驱动中执行这些操作是方便的。此时，应分别使用 pci_save_state()、
pci_prepare_to_sleep() 和 pci_set_power_state() 来保存设备的标准配置寄存器、准备系统唤醒（如有
必要）以及将其置于低功耗状态。此外，如果驱动调用了 pci_save_state()，PCI 子系统将不会为其设备
执行 pci_prepare_to_sleep() 或 pci_set_power_state()，因此驱动随后要负责适当地处置该设备。

在 suspend() 回调执行期间，可以调用驱动的中断处理程序来处理来自设备的中断，因此所有依赖驱动
处理中断能力的挂起相关操作都应在此回调中执行。

##### 3.1.3. suspend_noirq()


suspend_noirq() 回调只在系统挂起期间执行，在系统中所有设备的 suspend() 回调都已执行、并且设备
中断已被 PM core 禁用之后。

suspend_noirq() 与 suspend() 的区别在于，在 suspend_noirq() 运行期间不会调用驱动的中断处理程序。
因此 suspend_noirq() 可以执行那些如果在 suspend() 中执行会引发竞态条件的操作。

##### 3.1.4. freeze()


freeze() 回调是休眠特有的，在两种情况下执行：休眠期间，在为创建系统映像做准备、所有设备的
prepare() 回调都已执行之后；以及恢复期间，在系统映像已从持久存储加载到内存、并且所有设备的
prepare() 回调都已执行之后。

这个回调的作用与上述 suspend() 回调的作用类似。事实上，它们只需要在驱动负责将设备置于低功耗
状态的罕见情况下才需要有所不同。

在这些情况下，freeze() 回调不应准备设备系统唤醒或将其置于低功耗状态。不过，它或 freeze_noirq()
应当使用 pci_save_state() 保存设备的标准配置寄存器。

##### 3.1.5. freeze_noirq()


freeze_noirq() 回调是休眠特有的。它在休眠期间，在为创建系统映像做准备、所有设备的 prepare() 和
freeze() 回调都已执行之后执行；并在恢复期间，在系统映像已加载到内存、并且所有设备的 prepare()
和 freeze() 回调都已执行之后执行。它总是在 PM core 禁用设备中断之后执行。

这个回调的作用与上述 suspend_noirq() 回调的作用类似，并且极少需要定义 freeze_noirq()。

freeze_noirq() 与 freeze() 的区别类似于 suspend_noirq() 与 suspend() 的区别。

##### 3.1.6. poweroff()


poweroff() 回调是休眠特有的。它在将休眠映像保存到持久存储之后、系统即将关机时执行。在调用
poweroff() 之前，所有设备的 prepare() 回调都已执行。

这个回调的作用与上述 suspend() 和 freeze() 回调的作用类似，尽管它不需要保存设备寄存器的内容。
特别是，如果驱动想自己将设备置于低功耗状态，而不是让 PCI 子系统来做，poweroff() 回调应当分别使用
pci_prepare_to_sleep() 和 pci_set_power_state() 来准备设备系统唤醒和将其置于低功耗状态，但它无需
保存设备的标准配置寄存器。

##### 3.1.7. poweroff_noirq()


poweroff_noirq() 回调是休眠特有的。它在系统中所有设备的 poweroff() 回调都已执行之后执行。

这个回调的作用与上述 suspend_noirq() 和 freeze_noirq() 回调的作用类似，但它不需要保存设备寄存器的
内容。

poweroff_noirq() 与 poweroff() 的区别类似于 suspend_noirq() 与 suspend() 的区别。

##### 3.1.8. resume_noirq()


resume_noirq() 回调只在系统恢复期间、PM core 已启用非引导 CPU 之后执行。在 resume_noirq() 运行
期间不会调用驱动的中断处理程序，因此这个回调可以执行那些可能与中断处理程序发生竞态的操作。

由于 PCI 子系统在系统恢复的 resume_noirq 阶段无条件将所有设备置于全功耗状态并恢复它们的标准
配置寄存器，resume_noirq() 通常不是必需的。一般来说，它只应用于执行那些如果由 resume() 执行会
导致竞态条件的操作。

##### 3.1.9. resume()


resume() 回调只在系统恢复期间、系统中所有设备的 resume_noirq() 回调都已执行、并且 PM core 已启用
设备中断之后执行。

这个回调负责恢复设备的挂起前配置并将其带回完全可用的状态。在 resume() 返回之后，设备应当能够像
平常一样处理 I/O。

##### 3.1.10. thaw_noirq()


thaw_noirq() 回调是休眠特有的。它在系统映像已创建、并且 PM core 已启用非引导 CPU 之后，在休眠的
thaw_noirq 阶段执行。如果系统恢复期间加载休眠映像失败，它也可能被执行（此时它在启用非引导 CPU
之后执行）。在 thaw_noirq() 运行期间不会调用驱动的中断处理程序。

这个回调的作用与 resume_noirq() 类似。这两个回调的区别在于 thaw_noirq() 是在 freeze() 和
freeze_noirq() 之后执行的，因此一般来说它不需要修改设备寄存器的内容。

##### 3.1.11. thaw()


thaw() 回调是休眠特有的。它在系统中所有设备的 thaw_noirq() 回调都已执行、并且 PM core 已启用设备
中断之后执行。

这个回调负责恢复设备的冻结前配置，使其在 thaw() 返回之后能像平常一样工作。

##### 3.1.12. restore_noirq()


restore_noirq() 回调是休眠特有的。它在休眠的 restore_noirq 阶段执行，此时引导内核已将控制权交给
映像内核，并且映像内核的 PM core 已启用非引导 CPU。

这个回调与 resume_noirq() 类似，唯一的例外是它不能对设备的先前状态做任何假设，即使已知 BIOS（或
通常的平台固件）会在一次挂起-恢复周期中保留该状态。

对于绝大多数 PCI 设备驱动，resume_noirq() 与 restore_noirq() 之间没有区别。

##### 3.1.13. restore()


restore() 回调是休眠特有的。它在系统中所有设备的 restore_noirq() 回调都已执行、并且 PM core 已
启用设备驱动的中断处理程序被调用之后执行。

这个回调与 resume() 类似，就像 restore_noirq() 与 resume_noirq() 类似一样。因此，restore_noirq()
与 restore() 之间的区别类似于 resume_noirq() 与 resume() 之间的区别。

对于绝大多数 PCI 设备驱动，resume() 与 restore() 之间没有区别。

##### 3.1.14. complete()


complete() 回调在以下情况下执行：

  - 系统恢复期间，所有设备的 resume() 回调都已执行之后，
  - 休眠期间，在保存系统映像之前，所有设备的 thaw() 回调都已执行之后，
  - 系统恢复期间，当系统回到其休眠前状态时，所有设备的 restore() 回调都已执行之后。

如果休眠映像加载到内存失败，它也可能被执行（此时它在所有在引导内核中有驱动的设备都已执行 thaw()
回调之后运行）。

这个回调完全是可选的，尽管如果 prepare() 回调执行了需要被撤销的操作，它可能就是必要的。

##### 3.1.15. runtime_suspend()


runtime_suspend() 回调是设备运行时电源管理（runtime PM）特有的。它在设备即将在运行时被挂起（即
被静默并置于低功耗状态）时由 PM core 的运行时 PM 框架执行。

这个回调负责冻结设备并准备将其置于低功耗状态，但它必须允许 PCI 子系统执行挂起设备所需的全部
PCI 相关操作。

##### 3.1.16. runtime_resume()


runtime_resume() 回调是设备运行时 PM 特有的。它在设备即将在运行时被恢复（即置于全功耗状态并被
编程为正常处理 I/O）时由 PM core 的运行时 PM 框架执行。

这个回调负责在设备被 PCI 子系统置于全功耗状态之后恢复设备的正常功能。期望设备在 runtime_resume()
返回之后能够像平常一样处理 I/O。

##### 3.1.17. runtime_idle()


runtime_idle() 回调是设备运行时 PM 特有的。每当根据 PM core 的信息可能希望挂起设备时，它都会由
PM core 的运行时 PM 框架执行。特别是，如果设备恢复是由于某个虚假事件（spurious event）而发生，
它会在 runtime_resume() 返回之后自动执行。

这个回调是可选的，但如果它没有实现，或者它返回 0，PCI 子系统将为设备调用 pm_runtime_suspend()，
而这又会使得驱动的 runtime_suspend() 回调被执行。

##### 3.1.18. Pointing Multiple Callback Pointers to One Routine


尽管原则上前面小节中描述的每个回调都可以定义为一个独立的函数，但常常方便地让 struct dev_pm_ops
的两个或更多成员指向同一个例程。有几种便利宏可用于此目的。

DEFINE_SIMPLE_DEV_PM_OPS() 声明一个 struct dev_pm_ops 对象，其 .suspend()、.freeze() 和
.poweroff() 成员指向一个挂起例程，其 .resume()、.thaw() 和 .restore() 成员指向一个恢复例程。该
struct dev_pm_ops 中的其他函数指针未设置。

DEFINE_RUNTIME_DEV_PM_OPS() 与 DEFINE_SIMPLE_DEV_PM_OPS() 类似，但它额外将 .runtime_resume()
指针设为 pm_runtime_force_resume()，将 .runtime_suspend() 指针设为 pm_runtime_force_suspend()。

SYSTEM_SLEEP_PM_OPS() 可以在 struct dev_pm_ops 的声明内部使用，表示一个挂起例程由 .suspend()、
.freeze() 和 .poweroff() 成员指向，一个恢复例程由 .resume()、.thaw() 和 .restore() 成员指向。

##### 3.1.19. Driver Flags for Power Management


PM core 允许设备驱动设置一些标志，这些标志会影响核心本身以及包括 PCI 总线类型在内的中间层代码
对设备电源管理的处理。这些标志应当在驱动探测（probe）时借助 dev_pm_set_driver_flags() 函数设置
一次，之后不应直接更新。

DPM_FLAG_NO_DIRECT_COMPLETE 标志阻止 PM core 使用 direct-complete 机制——该机制允许在系统挂起
开始时设备处于运行时挂起状态的情况下，跳过设备的 suspend/resume 回调。这也会影响设备的所有祖先，
因此只有在绝对必要时才应使用此标志。

DPM_FLAG_SMART_PREPARE 标志使得 PCI 总线类型仅当设备驱动提供的 ->prepare 回调返回正值时，才从
pci_pm_prepare() 返回一个正值。这使得驱动可以选择

动态地退出使用 direct-complete 机制（而设置 DPM_FLAG_NO_DIRECT_COMPLETE 则表示永久退出）。

DPM_FLAG_SMART_SUSPEND 标志告诉 PCI 总线类型，从驱动的角度看，在系统挂起期间可以安全地将设备
保持在运行时挂起状态。这使得 pci_pm_suspend()、pci_pm_freeze() 和 pci_pm_poweroff() 避免将设备
从运行时挂起中恢复，除非有这样做的 PCI 相关理由。此外，它使得 pci_pm_suspend_late/noirq() 和
pci_pm_poweroff_late/noirq() 在系统范围转换进行中的“late”阶段设备仍处于运行时挂起状态时提前返回。
而且，如果设备在 pci_pm_resume_noirq() 或 pci_pm_restore_noirq() 中处于运行时挂起状态，其运行时
PM 状态会被改为“active”（因为它接下来将被置于 D0）。

设置 DPM_FLAG_MAY_SKIP_RESUME 标志意味着，如果设备在进入工作状态后的系统范围转换之后可以保持在
挂起状态，驱动允许跳过其“noirq”和“early”恢复回调。此标志由 PM core 与设备的 power.may_skip_resume
状态位一起考虑，后者在某些情况下由 pci_pm_suspend_noirq() 设置。如果 PM core 确定应跳过驱动的“noirq”
和“early”恢复回调，dev_pm_skip_resume() 辅助函数将返回“true”，这将导致 pci_pm_resume_noirq() 和
pci_pm_resume_early() 提前返回，而不触碰设备也不执行驱动回调。

### 3.2. Device Runtime Power Management


除了提供设备电源管理回调之外，PCI 设备驱动还负责控制其设备的运行时电源管理（runtime PM）。

PCI 设备的运行时 PM 是可选的，但建议 PCI 设备驱动至少在实现方式有可靠方法来验证设备未被使用时
（例如当网线从以太网适配器上拔下，或没有设备连接到 USB 控制器时）实现它。

为了支持 PCI 运行时 PM，驱动首先需要实现 runtime_suspend() 和 runtime_resume() 回调。它可能还需要
实现 runtime_idle() 回调，以防止设备在 runtime_resume() 回调每次刚返回之后就又被挂起（或者，
runtime_suspend() 回调将不得不检查设备是否真的应该被挂起，并在不是这种情况时返回 -EAGAIN）。

PCI 设备的运行时 PM 默认由 PCI core 启用。PCI 设备驱动无需启用它，也不应尝试这样做。然而，运行
pm_runtime_forbid() 辅助函数的 pci_pm_init() 会阻止它。除此之外，每个 PCI 设备的运行时 PM 使用计数
会在执行设备驱动提供的 probe 回调之前由 local_pci_probe() 递增。

如果一个 PCI 驱动实现了运行时 PM 回调，并打算使用 PM core 和 PCI 子系统提供的运行时 PM 框架，它需要在
其 probe 回调函数中递减设备的运行时 PM 使用计数。如果它不这样做，设备的计数将始终不为零，并且它永远
不会被运行时挂起。最简单的做法是调用 pm_runtime_put_noidle()，但如果驱动想立即调度一个自动挂起，例如，
它可以为此改用 pm_runtime_put_autosuspend()。一般来说，它只需要从 probe 例程中调用一个递减设备使用计数
的函数，就能使设备的运行时 PM 工作。

重要的是要记住，驱动的 runtime_suspend() 回调可能在使用计数被递减之后立即执行，因为用户空间可能已经
通过 sysfs 导致 pm_runtime_allow() 辅助函数运行从而解除了设备运行时 PM 的阻塞，因此驱动必须准备好应对
这种情况。

不过，驱动本身不应调用 pm_runtime_allow()。相反，它应该让用户空间或某些平台相关代码来这样做（如上所述
用户空间可以通过 sysfs 完成），但它必须准备好在 pm_runtime_allow() 一被调用（这可能随时发生，甚至在驱动
加载之前）就正确处理设备的运行时 PM。

当驱动的 remove 回调运行时，它必须平衡在 probe 时对设备运行时 PM 使用计数的递减。为此，如果它在 probe
回调中递减了该计数，就必须在 remove 回调中运行 pm_runtime_get_noresume()。[由于核心在运行驱动的 remove
回调之前会对设备执行运行时恢复并增加设备的使用计数，设备的运行时 PM 在 remove 执行期间实际上是被禁用的，
并且所有递增设备使用计数的运行时 PM 辅助函数此时实际上都等同于 pm_runtime_get_noresume()。]

运行时 PM 框架通过处理挂起或恢复设备的请求，或检查它们是否空闲（在这种情况下随后请求将它们挂起是合理的）
来工作。这些请求由放入电源管理工作队列 pm_wq 的工作项表示。尽管存在少数 PM core 自动排队电源管理请求的
情况（例如，在处理完恢复设备的请求之后，PM core 会自动排队一个检查设备是否空闲的请求），但设备驱动通常
负责为其设备排队电源管理请求。为此它们应使用 PM core 提供的运行时 PM 辅助函数，这些在
Documentation/power/runtime_pm.rst 中有讨论。

设备也可以同步地挂起和恢复，而不将请求放入 pm_wq。在大多数情况下，这同样由其驱动使用 PM core 为此提供的
辅助函数来完成。

有关设备运行时 PM 的更多信息，请参阅 Documentation/power/runtime_pm.rst。

## 4. Resources


PCI Local Bus Specification, Rev. 3.0

PCI Bus Power Management Interface Specification, Rev. 1.2

Advanced Configuration and Power Interface (ACPI) Specification, Rev. 3.0b

PCI Express Base Specification, Rev. 2.0

Documentation/driver-api/pm/devices.rst

Documentation/power/runtime_pm.rst
