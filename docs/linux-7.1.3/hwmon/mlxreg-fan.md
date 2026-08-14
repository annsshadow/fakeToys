## 内核驱动 mlxreg-fan


为以下新一代 Mellanox 系统提供风扇控制：

- QMB700，配备 40x200GbE InfiniBand 端口；
- MSN3700，配备 32x200GbE 或 16x400GbE 以太网端口；
- MSN3410，配备 6x400GbE 加 48x50GbE 以太网端口；
- MSN3800，配备 64x1000GbE 以太网端口；

Author: Vadim Pasternak <vadimp@mellanox.com>

这些是机架顶端（Top of the Rack）系统，配备带有 Mellanox Quantum 或 Spectrume-2 设备的
Mellanox 交换板。FAN 控制器由可编程设备逻辑实现。

可编程设备内默认的寄存器偏移集合如下：

======================= ====
pwm1			0xe3
fan1 (tacho1)		0xe4
fan2 (tacho2)		0xe5
fan3 (tacho3)		0xe6
fan4 (tacho4)		0xe7
fan5 (tacho5)		0xe8
fan6 (tacho6)		0xe9
fan7 (tacho7)		0xea
fan8 (tacho8)		0xeb
fan9 (tacho9)		0xec
fan10 (tacho10)		0xed
fan11 (tacho11)		0xee
fan12 (tacho12)		0xef
======================= ====

该设置可以用其他寄存器重新编程。

### 描述


该驱动实现了一个简单的接口，用于驱动连接到 PWM 输出和转速计（tachometer）输入的风扇。
该驱动根据系统配置获取 PWM 和转速计的寄存器位置，并创建 FAN/PWM 的 hwmon 对象以及一个
冷却设备。PWM 和转速计通过板载可编程设备感知，该设备导出其寄存器映射。该设备可以连接到
任何支持寄存器映射的总线类型。单个实例由一个 PWM 控制、最多 12 个转速计和一个冷却设备
创建。它支持的实例数量取决于可编程设备的能力。

该驱动通过 hwmon 和 thermal 的 sysfs 接口将风扇暴露给用户空间。

### hwmon 子系统中的 /sys 文件


================= == ===================================================
fan[1-12]_fault   RO 文件，用于转速计 TACH1-TACH12 故障指示
fan[1-12]_input   RO 文件，用于转速计 TACH1-TACH12 输入（单位为 RPM）
pwm1		  RW 文件，用于 fan[1-12] 目标占空比（0..255）
================= == ===================================================

### thermal 子系统中的 /sys 文件


================= == ====================================================
cur_state	  RW 文件，用于冷却设备的当前冷却状态
		     （0..max_state）
max_state	  RO 文件，用于冷却设备的最大冷却状态
================= == ====================================================
