## 内核驱动 mlxreg-fan


为以下新一Mellanox 系统提供风扇控制
- QMB700，配40x200GbE InfiniBand 端口- MSN3700，配32x200GbE 16x400GbE 以太网端口；
- MSN3410，配6x400GbE 48x50GbE 以太网端口；
- MSN3800，配64x1000GbE 以太网端口；

Author: Vadim Pasternak <vadimp@mellanox.com>

这些是机架顶端（Top of the Rack）系统，配备带有 Mellanox Quantum Spectrume-2 设备Mellanox 交换板。FAN 控制器由可编程设备逻辑实现
可编程设备内默认的寄存器偏移集合如下
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

该设置可以用其他寄存器重新编程
### 描述


该驱动实现了一个简单的接口，用于驱动连接到 PWM 输出和转速计（tachometer）输入的风扇该驱动根据系统配置获PWM 和转速计的寄存器位置，并创建 FAN/PWM hwmon 对象以及一冷却设备。PWM 和转速计通过板载可编程设备感知，该设备导出其寄存器映射。该设备可以连接任何支持寄存器映射的总线类型。单个实例由一PWM 控制、最12 个转速计和一个冷却设创建。它支持的实例数量取决于可编程设备的能力
该驱动通过 hwmon thermal sysfs 接口将风扇暴露给用户空间
### hwmon 子系统中/sys 文件


================= == ===================================================
fan[1-12]_fault   RO 文件，用于转速计 TACH1-TACH12 故障指示
fan[1-12]_input   RO 文件，用于转速计 TACH1-TACH12 输入（单位为 RPMpwm1		  RW 文件，用fan[1-12] 目标占空比（0..255================= == ===================================================

### thermal 子系统中/sys 文件


================= == ====================================================
cur_state	  RW 文件，用于冷却设备的当前冷却状		     ..max_statemax_state	  RO 文件，用于冷却设备的最大冷却状================= == ====================================================
