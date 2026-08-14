## 内核驱动 fam15h_power


支持的芯片：

- AMD Family 15h 处理器

- AMD Family 16h 处理器

  前缀: 'fam15h_power'

  扫描的地址: PCI 空间

  数据手册:

  - AMD Family 15h 处理器的 BIOS 和内核开发者指南（BKDG）
  - AMD Family 16h 处理器的 BIOS 和内核开发者指南（BKDG）
  - AMD64 架构程序员手册 第 2 卷：系统编程

Author: Andreas Herrmann <herrmann.der.user@googlemail.com>

### 描述


1) 处理器 TDP（热设计功耗，Thermal design power）

在给定的固定频率和电压下，处理器的功耗根据所执行的工作负载而变化。降额功耗（derated power）
是运行特定应用程序时消耗的功率。热设计功耗（TDP）就是降额功耗的一个例子。

该驱动允许通过 TDP 算法读取提供 AMD Family 15h 和 16h 处理器功耗信息的寄存器。

对于 AMD Family 15h 和 16h 处理器，可以使用不同的处理器北桥功能寄存器计算以下功率值：

- BasePwrWatts:
    以瓦特指定处理器为 NB 和核心外部逻辑消耗的最大功率。

- ProcessorPwrWatts:
    以瓦特指定处理器可以支持的最大功率。
- CurrPwrWatts:
    以瓦特指定处理器当前正在消耗的功率。

该驱动提供 ProcessorPwrWatts 和 CurrPwrWatts：

- power1_crit (ProcessorPwrWatts)
- power1_input (CurrPwrWatts)

在多节点处理器上，计算值是针对整个封装（package）的，而不是针对单个节点。因此该驱动仅为
多节点处理器的内部 node0 创建 sysfs 属性。

2) 累积功率机制

该驱动还引入了一种算法，用于计算处理器在测量间隔 Tm 内消耗的平均功率。累积功率机制的特性由
CPUID Fn8000_0007_EDX[^12^] 指示。

- Tsample:
	计算单元功率累加器采样周期

- Tref:
	PTSC 计数器周期

- PTSC:
	性能时间戳计数器

- N:
	计算单元功率累加器采样周期与 PTSC 周期的比率

- Jmax:
	最大计算单元累积功率，由 MaxCpuSwPwrAcc MSR C001007b 指示

- Jx/Jy:
	计算单元累积功率，由 CpuSwPwrAcc MSR C001007a 指示
- Tx/Ty:
	性能时间戳计数器的值，由 CU_PTSC MSR C0010280 指示

- PwrCPUave:
	CPU 平均功率

i. 执行 CPUID Fn8000_0007 以确定 Tsample 与 Tref 的比率。

	N = CPUID Fn8000_0007_ECX[CpuPwrSampleTimeRatio[15:0]] 的值。

ii. 从新的 MSR MaxCpuSwPwrAcc 读取累积能量值的完整范围。

	Jmax = 返回的值。

iii. 在时刻 x，SW 读取 CpuSwPwrAcc MSR 并采样 PTSC。

	Jx = 从 CpuSwPwrAcc 读取的值，Tx = 从 PTSC 读取的值。

iv. 在时刻 y，SW 读取 CpuSwPwrAcc MSR 并采样 PTSC。

	Jy = 从 CpuSwPwrAcc 读取的值，Ty = 从 PTSC 读取的值。

v. 计算一个计算单元在一段时间内的平均功耗
```

	if (Jy < Jx) // 发生了回绕
		Jdelta = (Jy + Jmax) - Jx
	else
		Jdelta = Jy - Jx
	PwrCPUave = N * Jdelta * 1000 / (Ty - Tx)

```
该驱动提供 PwrCPUave 和间隔（默认为 10 毫秒，最大为 1 秒）：

- power1_average (PwrCPUave)
- power1_average_interval (Interval)

power1_average_interval 可以在 /etc/sensors3.conf 文件中更新，如下所示：

chip `fam15h_power-*`
	set power1_average_interval 0.01

然后使用 “sensors -s” 保存它。
