## 内核驱动 k10temp


支持的设备：

- AMD Family 10h 处理器：

  Socket F：Quad-Core/Six-Core/Embedded Opteron（但见下文）

  Socket AM2+：Quad-Core Opteron、Phenom (II) X3/X4、Athlon X2（但见下文）

  Socket AM3：Quad-Core Opteron、Athlon/Phenom II X2/X3/X4、Sempron II

  Socket S1G3：Athlon II、Sempron、Turion II

- AMD Family 11h 处理器：

  Socket S1G2：Athlon (X2)、Sempron (X2)、Turion X2 (Ultra)

- AMD Family 12h 处理器："Llano"（E2/A4/A6/A8 系列
- AMD Family 14h 处理器："Brazos"（C/E/G/Z 系列
- AMD Family 15h 处理器："Bulldozer"（FX 系列）Trinity"Kaveri"  "Carrizo"Stoney Ridge"Bristol Ridge"

- AMD Family 16h 处理器："Kabini"Mullins"

- AMD Family 17h 处理器："Zen"Zen 2"

- AMD Family 18h 处理器："Hygon Dhyana"

- AMD Family 19h 处理器："Zen 3"

  前缀k10temp'

  扫描的地址：PCI 空间

  数据手册
  AMD Family 10h 处理器的 BIOS 和内核开发人员指南（BKDG）：

    http://support.amd.com/us/Processor_TechDocs/31116.pdf

  AMD Family 11h 处理器的 BIOS 和内核开发人员指南（BKDG）：

    http://support.amd.com/us/Processor_TechDocs/41256.pdf

  AMD Family 12h 处理器的 BIOS 和内核开发人员指南（BKDG）：

    http://support.amd.com/us/Processor_TechDocs/41131.pdf

  AMD Family 14h Models 00h-0Fh 处理器的 BIOS 和内核开发人员指南（BKDG）：

    http://support.amd.com/us/Processor_TechDocs/43170.pdf

  AMD Family 10h 处理器的修订指南
    http://support.amd.com/us/Processor_TechDocs/41322.pdf

  AMD Family 11h 处理器的修订指南
    http://support.amd.com/us/Processor_TechDocs/41788.pdf

  AMD Family 12h 处理器的修订指南
    http://support.amd.com/us/Processor_TechDocs/44739.pdf

  AMD Family 14h Models 00h-0Fh 处理器的修订指南
    http://support.amd.com/us/Processor_TechDocs/47534.pdf

  AMD Family 11h 处理器笔记本用电源与热数据表
    http://support.amd.com/us/Processor_TechDocs/43373.pdf

  AMD Family 10h 服务器与工作站处理器电源与热数据表：

    http://support.amd.com/us/Processor_TechDocs/43374.pdf

  AMD Family 10h 桌面处理器电源与热数据表
    http://support.amd.com/us/Processor_TechDocs/43375.pdf

作者：Clemens Ladisch <clemens@ladisch.de>

### 描述


此驱动允许读AMD Family 10h/11h/12h/14h/15h/16h 处理器的内部温度传感器
所有这些处理器都有一个传感器，但Socket F AM2+ 的处理器上，传感可能返回不一致的值（erratum 319）。除非你指定 "force=1" 模块参数，否驱动将拒绝在这些修订版本上加载
由于技术原因，驱动只能检测主板插槽类型，而非处理器的实际能力。因此，如果
你在 AM2+ 主板上使AM3 处理器，可以安全地使"force=1" 参数
对于早于 Family 17h CPU，有一个温度测量值，sysfs 中作temp1_input
可用。它以摄氏度为单位测量，分辨率为 1/8 度。请注意，它被定义为一个相对```

  Tctl 是处理器温度控制值，由平台用来控制散热系统。Tctl 是一个任意刻度上  非物理温度，以度为单位测量。它***代表像芯片（die）或机箱（case）温  这样的实际物理温度。相反，它指定的是相对于系统必须为处理器指定的最大机  温度和最大热功耗提供最大散热的那一点的处理器温度
```
Tctl 的最大值在文件 temp1_max 中可用
如果 BIOS 已启用硬件温度控制，处理器为避免损坏而自行降频（throttle）的阈temp1_crit temp1_crit_hyst 中可用
在某AMD CPU 上，芯片温度（Tdie）与报告的温度（Tctl）之间存在差异。Tdie
是实际测量的温度，Tctl 用于风扇控制。虽Tctl 始终作为 temp1_input 可用但对于支持它的那CPU，驱动将 Tdie 温度作为 temp2_input 导出
17h 系列型号报告相对温度，驱动旨在补偿并报告真实温度
Family 17h Family 18h CPU 上，额外的温度传感器可能报告 Core Complex
Die（CCD）温度。最8 个这样的温度作为 temp{3..10}_input 报告，标记为
Tccd{1..8}。实际支持取决于 CPU 的具体型号