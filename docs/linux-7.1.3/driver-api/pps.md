
## PPS - Pulse Per Second


Copyright (C) 2007 Rodolfo Giometti <giometti@enneenne.com>

This program is free software; you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation; either version 2 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.



### 概述


LinuxPPS 提供了一个编程接口（API），用于在系统中定义多个 PPS 源。

PPS 意为“每秒脉冲（pulse per second）”，PPS 源就是一个每秒提供一个高精度信号的
设备，应用程序可以利用它来调整系统时钟时间。

一个 PPS 源可以连接到串行端口（通常是接到数据载波检测 Data Carrier Detect 引脚）、
并行端口（ACK 引脚），或者某个专用 CPU 的 GPIO（这在嵌入式系统中很常见）；但在每种
情况下，当一个新的脉冲到达时，系统都必须为它打上时间戳（timestamp）并记录下来供用户
空间使用。

常见的用法是将 NTPD 作为用户空间程序，配合一个 GPS 接收器作为 PPS 源，从而获得与 UTC
保持亚毫秒级同步的墙上时钟时间（wallclock-time）。


### RFC 相关考量


在为 PPS API 实现 RFC 2783 所定义的接口、并使用嵌入式 CPU 的 GPIO 引脚作为连接到信号的
物理链路时，我遇到了一个更深层的问题：

   At startup it needs a file descriptor as argument for the function
   time_pps_create().

这意味着该源必须有一个 /dev/... 条目。对于串行端口和并行端口来说这个假设是可以的，因为
除了（！）采集时间戳（这是 PPS API 的核心任务）之外，你还可以在这些端口上做一些有用的
事情。但对于单一用途的 GPIO 线，这个假设就不成立了。在这种情况下，即便基本的文件相关
功能（如 read() 与 write()）也毫无意义，不应成为使用 PPS API 的前提条件。

如果你考虑到 PPS 源并不总是与 GPS 数据源相连，这个问题就可以简单地解决。

因此你的程序应当检查 GPS 数据源（例如串行端口）是否也是一个 PPS 源；如果不是，它们
应当提供打开另一个设备作为 PPS 源的可能性。

在 LinuxPPS 中，PPS 源就是普通的字符设备，通常映射到 /dev/pps0、/dev/pps1 等文件。


### 使用 USB 转串口设备的 PPS


可以从 USB 转串口设备上获取 PPS。不过，你应该考虑到 USB 协议栈引入的延迟与抖动。用户报告
通过 USB 与 PPS 同步时时钟不稳定，大约在 ±1ms。使用 USB 2.0 时，抖动可能降低到 125 微秒
的量级。

这对于使用 NTP 进行时间服务器同步可能是合适的，因为它有下采样（undersampling）和算法。

如果你的设备没有报告 PPS，你可以检查其驱动是否支持该功能。大多数情况下，你只需要在检查
DCD 状态之后添加对 usb_serial_handle_dcd_change 的调用（参见 ch341 与 pl2303 示例）。


### 编码示例


要将一个 PPS 源注册到内核中，你应该定义一个 struct
```

    static struct pps_source_info pps_ktimer_info = {
	    .name         = "ktimer",
	    .path         = "",
	    .mode         = PPS_CAPTUREASSERT | PPS_OFFSETASSERT |
			    PPS_ECHOASSERT |
			    PPS_CANWAIT | PPS_TSFMT_TSPEC,
	    .echo         = pps_ktimer_echo,
	    .owner        = THIS_MODULE,
    };

```
然后调用函数 pps_register_source()，在你的
```

    source = pps_register_source(&pps_ktimer_info,
			PPS_CAPTUREASSERT | PPS_OFFSETASSERT);

```
```

  int pps_register_source(struct pps_source_info *info, int default_params)

```
其中 “info” 是指向描述某个特定 PPS 源的结构的指针，“default_params” 告诉系统该设备的初始
默认参数应该是什么（显然，这些参数必须是描述驱动能力的 struct pps_source_info 中所定义的
参数的一个子集）。

一旦你将一个新 PPS 源注册到系统中，就可以发出一个 assert 事件（例如在中断处理例程中）
```

    pps_event(source, &ts, PPS_CAPTUREASSERT, ptr)

```
其中 “ts” 是事件的时间戳。

同一个函数还可以运行所定义的 echo 函数（pps_ktimer_echo()，向它传入 “ptr” 指针），如果用户
要求这么做的话……等等。

示例代码请参见 drivers/pps/clients/pps-ktimer.c 文件。


### SYSFS 支持


```

   $ ls /sys/class/pps/
   pps0/  pps1/  pps2/

```
每个目录都是系统中定义的一个 PPS 源的 ID，以及
```

   $ ls -F /sys/class/pps/pps0/
   assert     dev        mode       path       subsystem@
   clear      echo       name       power/     uevent


```
在每个 “assert” 与 “clear” 文件中，你可以找到时间戳和一个
```

   $ cat /sys/class/pps/pps0/assert
   1170026870.983207967#8

```
其中 “#” 之前的是以秒为单位的时间戳；之后的是序列号。其它文件包括：

 - echo：报告该 PPS 源是否具有 echo 函数；

 - mode：报告可用的 PPS 工作模式；

 - name：报告 PPS 源的名称；

 - path：报告 PPS 源的设备路径，即该 PPS 源所连接的设备（如果存在）。


### 测试 PPS 支持


即便没有特定硬件，你也可以为了测试 PPS 支持而使用 pps-ktimer 驱动（参见 PPS 配置菜单中的
客户端子小节）以及你的发行版中 pps-tools 软件包、http://linuxpps.org 或 https://github.com/redlab-i/pps-tools 中提供的用户空间工具。

一旦你启用了 pps-ktimer 的编译，只需 modprobe 它（如果
```

   # modprobe pps-ktimer

```
```

   $ ./ppstest /dev/pps1
   trying PPS source "/dev/pps1"
   found PPS source "/dev/pps1"
   ok, found 1 source(s), now start fetching data...
   source 0 - assert 1186592699.388832443, sequence: 364 - clear  0.000000000, sequence: 0
   source 0 - assert 1186592700.388931295, sequence: 365 - clear  0.000000000, sequence: 0
   source 0 - assert 1186592701.389032765, sequence: 366 - clear  0.000000000, sequence: 0

```
请注意，要编译用户空间程序，你需要 timepps.h 文件。该文件在上述 pps-tools 仓库中可以找到。


### 发生器（Generators）


有时不仅需要捕获 PPS 信号，还需要产生它们。例如，运行一个分布式仿真，它要求计算机的时钟
被非常紧密地同步。

为此，增加了 pps-gen 类。可以通过定义 struct pps_gen_source_info 来向内核注册 PPS 发生器，
如下
```

    static const struct pps_gen_source_info pps_gen_dummy_info = {
            .use_system_clock       = true,
            .get_time               = pps_gen_dummy_get_time,
            .enable                 = pps_gen_dummy_enable,
    };

```
其中 use_system_clock 表明该发生器是否使用系统时钟来产生脉冲，还是使用来自外设设备时钟的
脉冲。方法 get_time() 用于查询存储在发生器时钟中的时间，而方法 enable() 用于启用或禁用
PPS 脉冲的产生。

然后在你的初始化例程中调用函数 pps_gen_register_source()，如下所示，会创建一个新的发生器
```

    pps_gen = pps_gen_register_source(&pps_gen_dummy_info);

```
### 发生器 SYSFS 支持


```

    $ ls /sys/class/pps-gen/
    pps-gen0/  pps-gen1/  pps-gen2/

```
每个目录都是系统中定义的一个 PPS 发生器的 ID，以及
```

    $ ls -F /sys/class/pps-gen/pps-gen0/
    dev  enable  name  power/  subsystem@  system  time  uevent

```
```

    $ echo 1 > /sys/class/pps-gen/pps-gen0/enable

```
### 并行端口发生器


一种做法是发明某些复杂的硬件方案，但这既没必要也未必划算。便宜的做法是在其中一台计算机
（主节点，master）上加载一个 PPS 发生器，在其它计算机（从节点，slave）上加载 PPS 客户端，
并使用非常简单的线缆，例如通过并行端口来传送信号。

```

	pin	name	master      slave
	1	STROBE	  *------     *
	2	D0	  *     |     *
	3	D1	  *     |     *
	4	D2	  *     |     *
	5	D3	  *     |     *
	6	D4	  *     |     *
	7	D5	  *     |     *
	8	D6	  *     |     *
	9	D7	  *     |     *
	10	ACK	  *     ------*
	11	BUSY	  *           *
	12	PE	  *           *
	13	SEL	  *           *
	14	AUTOFD	  *           *
	15	ERROR	  *           *
	16	INIT	  *           *
	17	SELIN	  *           *
	18-25	GND	  *-----------*

```
请注意，并行端口中断只在由高到低的跳变时触发，因此它被用于 PPS 的 assert 边沿。PPS 的 clear
边沿只能通过在中断处理程序中使用轮询（polling）来确定，这实际上可以做得更精确，因为中断
处理的延迟可能相当大且随机。因此当前的 parport PPS 发生器实现（pps_gen_parport 模块）倾向于
使用 clear 边沿来进行时间同步。

clear 边沿的轮询是在关闭中断的情况下进行的，因此最好将 assert 与 clear 边沿之间的延迟选得
尽可能小，以降低系统延迟。但如果太小，从节点将无法捕获 clear 边沿的跳变。30 微秒的默认值在
大多数情况下应该足够好。该延迟可以使用 'delay' pps_gen_parport 模块参数来选择。


### Intel Timed I/O PPS 信号发生器


Intel Timed I/O 是一个高精度设备，出现在 2019 年及更新的 Intel CPU 上，可以产生 PPS 信号。

Timed I/O 与系统时间都由同一个硬件时钟驱动。信号的生成精度约为 20 纳秒。生成的 PPS 信号用于
将外部设备与系统时钟同步。例如，它可以用来与接收由 Timed I/O 设备生成的 PPS 信号的设备共享
你的时钟。有专用的 Timed I/O 引脚用于将 PPS 信号传送到外部设备。

将 Intel Timed I/O 用作 PPS 发生器：

```

        $echo 1 > /sys/class/pps-gen/pps-genx/enable

```
```

        $echo 0 > /sys/class/pps-gen/pps-genx/enable

```
