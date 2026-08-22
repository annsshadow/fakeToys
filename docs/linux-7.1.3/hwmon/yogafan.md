
## 内核驱动 yogafan


支持的设备：

  - Lenovo Yoga、Legion、IdeaPad、Slim、Flex 以及 LOQ 嵌入式控制器
  - 前缀yogafan'
  - 地址：ACPI 句柄（见下文数据库）

作者：Sergio Melas <sergiomelas@gmail.com>

### 描述


本驱动为现代 Lenovo 消费级笔记本提供风扇转速监控。大多数 Lenovo 笔记本不通过标准ISA/LPC 硬件监控芯片提供风扇转速计数据。相反，数据存储在嵌入式控制器（EC）中并通过
ACPI 暴露
该驱动实现了一*限速率滞后（RLLag*滤波器，用于处理 Lenovo EC 固件中低分辨率、有
抖动的采样
### 硬件识别与倍率逻辑


该驱动支持两种不同的 EC 架构。区分在探测阶段通过 DMI 产品系列 quirk 表确定性地完成无需运行时启发式
1. 8 EC 架构（倍率00
   - **系列* Yoga、IdeaPad、Slim、Flex   - **技术细节：** 这些型号为转速计数据分配单个 8 位寄存器。由8 位字段的最大值为 255      BIOS 100 RPM 为单位存储风扇转速（例如2 = 4200 RPM）
2. 16 EC 架构（倍率
   - **系列* Legion、LOQ   - **技术细节：** 高性能游戏型号需要更高的精度来应对超6000 RPM 的风扇。它们使     一16 位字 字节）直接存储原RPM 值
### 婊ゆ尝鍣ㄧ粏鑺。

RLLag 滤波器是一个被动的离散时间一阶滞后模型，它确保：
  - **平滑* 低分辨率的步进增量被平滑1-RPM 的增量  - **压摆率限制：** 通过将变化限制在 1500 RPM/s，防止不真实的读数，以匹配物理风扇的
    惯性  - **轮询无关性：** 滤波器数学基于用户空间读取之间的时间差进行缩放，确保无论轮询频率
    如何都保持一致的物理曲线
### 挂起与恢

该驱动使boottime 时钟（ktime_get_boottime()）来计算采样间隔。这确保了系统挂起期所花费的时间被计入。如果间隔超5 秒（例如笔记本唤醒后），滤波器会自动重置为当前硬值，以防止报告来自睡眠状态之前的"幽灵" RPM 数据
### 用法


该驱动暴露标准的 hwmon sysfs 属性：

===============   ============================
属            描述
fanX_input        过滤后的风扇转速，单位RPM===============   ============================


注意：如果硬件报0 RPM，滤波器会被旁路并立即报0，以确保用户知道风扇已停止

##                  LENOVO 风扇控制器：主参考数据库026

```

 MODEL (DMI PN) | FAMILY / SERIES  | EC OFFSET | FULL ACPI OBJECT PATH          | WIDTH  | MULTiplier
 ----------------------------------------------------------------------------------------------------
 82N7           | Yoga 14cACN      | 0x06      | \_SB.PCI0.LPC0.EC0.FANS        |  8-bit | 100
 80V2 / 81C3    | Yoga 710/720     | 0x06      | \_SB.PCI0.LPC0.EC0.FAN0        |  8-bit | 100
 83E2 / 83DN    | Yoga Pro 7/9     | 0xFE      | \_SB.PCI0.LPC0.EC0.FANS        |  8-bit | 100
 82A2 / 82A3    | Yoga Slim 7      | 0x06      | \_SB.PCI0.LPC0.EC0.FANS        |  8-bit | 100
 81YM / 82FG    | IdeaPad 5        | 0x06      | \_SB.PCI0.LPC0.EC0.FAN0        |  8-bit | 100
 82JW / 82JU    | Legion 5 (AMD)   | 0xFE/0xFF | \_SB.PCI0.LPC0.EC0.FANS (Fan1) | 16-bit | 1
 82JW / 82JU    | Legion 5 (AMD)   | 0xFE/0xFF | \_SB.PCI0.LPC0.EC0.FA2S (Fan2) | 16-bit | 1
 82WQ           | Legion 7i (Int)  | 0xFE/0xFF | \_SB.PCI0.LPC0.EC0.FANS (Fan1) | 16-bit | 1
 82WQ           | Legion 7i (Int)  | 0xFE/0xFF | \_SB.PCI0.LPC0.EC0.FA2S (Fan2) | 16-bit | 1
 82XV / 83DV    | LOQ 15/16        | 0xFE/0xFF | \_SB.PCI0.LPC0.EC0.FANS /FA2S  | 16-bit | 1
 83AK           | ThinkBook G6     | 0x06      | \_SB.PCI0.LPC0.EC0.FAN0        |  8-bit | 100
 81X1           | Flex 5           | 0x06      | \_SB.PCI0.LPC0.EC0.FAN0        |  8-bit | 100
 *Legacy*       | Pre-2020 Models  | 0x06      | \_SB.PCI0.LPC.EC.FAN0          |  8-bit | 100
 ----------------------------------------------------------------------------------------------------

```
方法与识别：

1. DSDT 分析（路径）   使用 'iasl' 分析 BIOS ACPI 表，并与公开 dump 交叉引用。内部标签（FANS、FAN0、FA2S   被映射到 EmbeddedControl OperationRegion 偏移
2. EC 内存映射（偏移）   通过NBFC（NoteBook FanControl）的 XML 逻辑BIOS 固件中的 DSDT Field 定义相匹   进行验证
3. 数据宽度分析（倍率）：
   - 8 位（倍率 100）：Yoga/IdeaPad 的标准。原始值（0-255）   - 16 位（倍率 1）：Legion/LOQ 的标准。两个寄存器xFE/0xFF）

### 参

1. **ACPI 规范（Field Objects）：** 关于OperationRegions 中如何访8 位与 16    字段的文档   https://uefi.org/specs/ACPI/6.5/05_ACPI_Software_Programming_Model.html#field-objects

2. **NBFC 项目* 由社区驱动的、对 Lenovo Legion/LOQ EC 内存映射6 位原始寄存器）的
   逆向工程   https://github.com/hirschmann/nbfc/tree/master/Configs

3. **Linux 内核时间保持 API* 关于 ktime_get_boottime() 以及跨挂起状态处理时间差   文档   https://www.kernel.org/doc/html/latest/core-api/timekeeping.html

4. **Lenovo IdeaPad 笔记本驱动：** 关于 Lenovo 笔记本中基于 DMI 的硬件特性门控的参考   https://github.com/torvalds/linux/blob/master/drivers/platform/x86/lenovo/ideapad-laptop.c
