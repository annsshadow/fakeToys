
## Intel(R) Trace Hub (TH)（英特尔追踪中枢）


### 概述


Intel(R) Trace Hub（TH，英特尔追踪中枢）是一组硬件模块，用于经由多种类型的
追踪输出端口（采用 System Trace Protocol（MIPI STPv2）编码）产生、切换并输出
来自多个硬件和软件源的追踪数据，旨在实现全系统调试。有关该硬件的更多信息，
请参见 Intel(R) Trace Hub 开发者手册 [^1^]。

它由追踪源、追踪目标（输出）以及一个切换器（Global Trace Hub，GTH，全局追踪中枢）
组成。这些设备挂载在各自的总线（"intel_th"）上，可通过 sysfs 属性被探测与配置。

目前支持的 Intel TH 子设备（模块）如下：
  - Software Trace Hub（STH，软件追踪中枢），追踪源，是一个 System Trace
    Module（STM，系统追踪模块）设备，
  - Memory Storage Unit（MSU，内存存储单元），追踪输出，允许将追踪中枢的
    输出存储在系统内存中，
  - Parallel Trace Interface output（PTI，并行追踪接口输出），通过 PTI 端口
    将追踪输出到外部调试主机，
  - Global Trace Hub（GTH，全局追踪中枢），即一个切换器，也是 Intel(R) Trace
    Hub 架构的核心组件。

输出设备的通用属性在
Documentation/ABI/testing/sysfs-bus-intel_th-output-devices 中有说明，其中最
值得关注的是 "active"（激活），用于启用或禁用向该特定输出设备的追踪输出。

GTH 可通过其 "masters"（主设备）属性组将不同的 STP 主设备导向不同的输出端口。
更详细的 GTH 接口说明见
Documentation/ABI/testing/sysfs-bus-intel_th-devices-gth。

STH 注册一个 stm class 设备，并经由它向用户态与内核态的软件追踪源提供接口。
更多信息请参见 Documentation/trace/stm.rst。

MSU 可被配置为将追踪数据采集到系统内存缓冲区中，之后可通过其设备节点以
read() 或 mmap() 接口读取，并导向一个 "software sink"（软件汇聚）驱动，由该
驱动消费数据和/或进一步转发。

总体而言，Intel(R) Trace Hub 运行不需要任何特殊的用户态软件；一切都可以通过
sysfs 属性和设备节点进行配置、启动与采集。

[^1^] https://software.intel.com/sites/default/files/managed/d3/3c/intel-th-developer-manual.pdf

### 总线与子设备


系统中每个 Intel TH 设备都会创建一条属于自身的总线，并分配一个 id 编号，
该编号反映 TH 设备被枚举的顺序。所有 TH 子设备（intel_th 总线上的设备）都以
该 id 开头：0-gth、0-msc0、0-msc1、0-pti、0-sth，其后跟随设备名称以及一个
可选的索引。

输出设备在 /dev/intel_thN 处也会获得一个设备节点，其中 N 为 Intel TH 设备的
id。例如，MSU 的内存缓冲区在分配后可通过 /dev/intel_th0/msc{0,1} 访问。

### 快速示例


```

	$ cat /sys/bus/intel_th/devices/0-msc0/port
	0

```
```

	$ echo 0 > /sys/bus/intel_th/devices/0-gth/masters/33

```
# 在第一个内存缓冲区上分配一个 2 窗口的 multiblock 缓冲区
```

	$ echo multi > /sys/bus/intel_th/devices/0-msc0/mode
	$ echo 64,64 > /sys/bus/intel_th/devices/0-msc0/nr_pages

```
```

	$ echo 1 > /sys/bus/intel_th/devices/0-msc0/wrap

```
```

	$ echo 1 > /sys/bus/intel_th/devices/0-msc0/active

```
# .. 向主设备 33 发送数据，更多细节见 stm.txt ..
# .. 等待追踪数据堆积 ..
```

	$ echo 0 > /sys/bus/intel_th/devices/0-msc0/active

```
```

	$ cat /dev/intel_th0/msc0 > my_stp_trace

```
### 主机调试器模式


可以配置追踪中枢，并从一个通过某条硬件调试接口连接的远程调试主机来控制其
追踪采集；该接口随后既用于控制 Intel Trace Hub，也用于将它的追踪数据传输到
调试主机。

需要告知驱动正在做这样的安排，以便它不去触碰任何采集/端口配置，并避免与
调试主机的配置访问相冲突。在此模式下，驱动唯一执行的活动就是将软件追踪
收集到 Software Trace Hub（一个 stm class 设备）。用户仍须负责建立接收端
解码器能够识别的、合适的 master/channel（主设备/通道）映射。

要启用主机模式，请将 'intel_th' 内核模块的 'host_mode' 参数设为 'y'。intel_th
总线上将不会出现任何虚拟输出设备。同时，'gth' 设备的追踪配置与采集控制属性组
也不会被暴露。'sth' 设备将照常工作。

### 软件汇聚（Software Sinks）


Memory Storage Unit（MSU）驱动提供了一个内核态 API，供其他驱动注册为追踪数据
的软件汇聚。此类驱动可进一步通过其他设备（如 USB 设备控制器或网卡）导出数据。

```
 - 通知软件汇聚某个特定窗口已写满，并"锁定"该窗口（即令其不再可用于追踪
   采集）；发生这种情况时，MSU 驱动会自动切换到缓冲区中的下一个窗口（如果它
   未被锁定），否则将停止追踪采集；
 - 跟踪窗口的"锁定"状态，并为软件汇聚驱动提供一种方式，以便在某个窗口被
   解锁、可再次用于采集追踪数据时通知 MSU 驱动。

```
示例汇聚驱动 msu-sink 演示了软件汇聚的实现。从功能上讲，它只是在窗口一写满
就解锁，使 MSU 以循环缓冲区模式持续运行。与 "multi"（多窗口）模式不同，它会
填满缓冲区中的所有窗口，而非仅第一个。可通过向 "mode" 文件写入 "sink" 来启用
（前提是 msu-sink.ko 已加载）。
