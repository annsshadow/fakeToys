## IO-APIC


:作者: Ingo Molnar <mingo@kernel.org>

大多数（全部）符合 Intel-MP 的 SMP 主板都带有所谓的 'IO-APIC'，它是一种
增强的中断控制器。它使我们能够将硬件中断路由到多个 CPU，或 CPU 组。没有
IO-APIC，来自硬件的中断将只会被投递到启动操作系统的那个 CPU（通常是 CPU#0）。

Linux 支持所有符合规范的 SMP 主板，包括带有多个 IO-APIC 的主板。多个
IO-APIC 用于高端服务器中以进一步分散 IRQ 负载。

在某些较旧的主板中存在（少量）已知的故障，这类 bug 通常由内核加以规避。
如果你的符合 MP 规范的 SMP 主板无法启动 Linux，请先查阅 linux-smp 邮件列表
存档。

如果你的机器在使用启用的 IO-APIC IRQ 时能够正常启动，那么你的
```

  hell:~> cat /proc/interrupts
             CPU0
    0:    1360293    IO-APIC-edge  timer
    1:          4    IO-APIC-edge  keyboard
    2:          0          XT-PIC  cascade
   13:          1          XT-PIC  fpu
   14:       1448    IO-APIC-edge  ide0
   16:      28232   IO-APIC-level  Intel EtherExpress Pro 10/100 Ethernet
   17:      51304   IO-APIC-level  eth0
  NMI:          0
  ERR:          0
  hell:~>

```
一些中断仍被列为 'XT PIC'，但这没有问题；这些 IRQ 源中没有一个是性能关键的。

在不太可能的情况下，如果你的主板没有生成可用的 mp-table，你可以使用 pirq=
引导参数来“手工构造”IRQ 条目。不过这并不简单，也无法自动化。一个 /etc/lilo.conf
的示例
```

	append="pirq=15,11,10"

```
实际数字取决于你的系统、你的 PCI 卡以及它们的 PCI 插槽位置。通常 PCI 插槽
在连接到 PCI 芯片组 IRQ 路由设施（传入的 PIRQ1-4
```

               ,-.        ,-.        ,-.        ,-.        ,-.
     PIRQ4 ----| |-.    ,-| |-.    ,-| |-.    ,-| |--------| |
               |S|  \  /  |S|  \  /  |S|  \  /  |S|        |S|
     PIRQ3 ----|l|-. `/---|l|-. `/---|l|-. `/---|l|--------|l|
               |o|  \/    |o|  \/    |o|  \/    |o|        |o|
     PIRQ2 ----|t|-./`----|t|-./`----|t|-./`----|t|--------|t|
               |1| /\     |2| /\     |3| /\     |4|        |5|
     PIRQ1 ----| |-  `----| |-  `----| |-  `----| |--------| |
               `-'        `-'        `-'        `-'        `-'

```
```

                               ,-.
                         INTD--| |
                               |S|
                         INTC--|l|
                               |o|
                         INTB--|t|
                               |x|
                         INTA--| |
                               `-'

```
这些 INTA-D PCI IRQ 始终“对卡本地”，它们的真实含义取决于它们所在的插槽。
如果你看菊花链（daisy chaining）图，插槽 4 中的卡发出 INTA IRQ，它将最终
成为 PCI 芯片组上 PIRQ4 上的一个信号。大多数卡发出 INTA，这在 PIRQ 线之间
创建了最优分布。（合理分配 IRQ 源并非必需，PCI IRQ 可以随意共享，但拥有
非共享的中断对性能有好处）。插槽 5 应该用于显卡，它们通常不使用的
中断，因此它们也不被菊花链连接。

所以，如果你把 SCSI 卡（IRQ11）放在插槽 1，Tulip 卡（IRQ9）放在
```

	append="pirq=11,9"

```
以下脚本尝试从
```

	echo -n pirq=; echo `scanpci | grep T_L | cut -c56-` | sed 's/ /,/g'

```
注意，如果你跳过了一些插槽，或者你的主板没有做默认的菊花链连接，这个脚本
将无法工作。（或者 IO-APIC 的 PIRQ 引脚以某种奇怪的方式连接）。例如，在上
述情况下，如果你把 SCSI
```

	append="pirq=0,9,11"

```
[值 '0' 是一个通用的“占位符”，为空的（或不发出 IRQ 的）插槽保留。]

一般来说，总是有可能找出正确的 pirq= 设置，只需适当排列所有 IRQ 编号……
不过这需要一些时间。一个“不正确”的 pirq 行将导致启动过程挂起，或者设备
无法正常工作（例如，如果它是作为模块插入的）。

如果你有 2 条 PCI 总线，那么你可以使用最多 8 个 pirq 值，尽管这样的主板
往往有不错的配置。
```

	append="pirq=0,0,0,0,0,0,9,11"

```
使用巧妙的试错技术来找出正确的 pirq 行……

祝好运，如果有任何本文档未涵盖的问题，请发邮件至 linux-smp@vger.kernel.org
或 linux-kernel@vger.kernel.org。
