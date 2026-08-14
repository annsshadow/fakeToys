
## CTU CAN FD 驱动


作者：Martin Jerabek <martin.jerabek01@gmail.com>


### 关于 CTU CAN FD IP 核


`CTU CAN FD <https://gitlab.fel.cvut.cz/canbus/ctucanfd_ip_core>`_
是一个用 VHDL 编写的开源软核。
它起源于 2015 年 Ondrej Ille 在
`CTU <https://www.cvut.cz/en>`_ 的
`电气工程学院（FEE） <http://www.fel.cvut.cz/en/>`_ 的
`测量系 <https://meas.fel.cvut.cz/>`_
的项目。

针对基于 Xilinx Zynq SoC 的 MicroZed 板卡的 SocketCAN 驱动
`Vivado 集成 <https://gitlab.fel.cvut.cz/canbus/zynq/zynq-can-sja1000-top>`_
以及基于 Intel Cyclone V 5CSEMA4U23C6 的 DE0-Nano-SoC Terasic 板卡的
`QSys 集成 <https://gitlab.fel.cvut.cz/canbus/intel-soc-ctucanfd>`_
已经完成开发，同时还包括对该核的
`PCIe 集成 <https://gitlab.fel.cvut.cz/canbus/pcie-ctucanfd>`_
的支持。

对于 Zynq，该核通过 APB 系统总线连接，该总线不支持设备枚举，因此
必须在 Device Tree 中指定该设备。这类设备在 kernel 中称为 platform device
（平台设备），由 platform device driver（平台设备驱动）处理。

CTU CAN FD 外设的基本功能模型已被
QEMU 主线接受。参见 QEMU 的 `CAN 仿真支持 <https://www.qemu.org/docs/master/system/devices/can.html>`_
了解 CAN FD 总线、主机连接以及 CTU CAN FD 核的仿真。仿真支持的开发
版本可以从 QEMU 本地开发 `仓库 <https://gitlab.fel.cvut.cz/canbus/qemu-canbus>`_
的 ctu-canfd 分支克隆得到。


### 关于 SocketCAN


SocketCAN 是 Linux 内核中 CAN 设备的标准通用接口。顾名思义，该总线
通过 socket 访问，类似于常见的网络设备。其背后的原理在
`Linux SocketCAN <https://www.kernel.org/doc/html/latest/networking/can.html>`_
中有深入描述。简而言之，它提供了一种
在 CAN 之上实现和使用高层协议的自然方式，
与例如以太网之上运行 UDP/IP 的方式相同。


#### 设备探测（Device probe）


在详细介绍 CAN 总线设备驱动的结构之前，我们先重申一下内核究竟
是如何得知设备存在的。某些总线，如 PCI 或 PCIe，支持设备枚举。也就是说，
系统启动时，会发现总线上的所有设备并读取它们的配置。内核通过其
vendor ID 和 device ID 来识别设备，如果存在为该标识符组合注册的驱动，
就会调用其 probe 方法来为该硬件填充驱动的实例。USB 的情况类似，只不过
它允许设备热插拔。

对于直接内嵌在 SoC 中并连接到内部系统总线（AXI、APB、Avalon 等）
的外设，情况则不同。这些总线不支持枚举，因此内核必须从其他地方
获知设备信息。这正是 Device Tree 的用途所在。


#### 设备树（Device tree）


设备树中的一个条目声明了系统中存在一个设备、它如何被访问（位于
哪条总线上）以及它的配置——寄存器地址、中断等等。此类设备树
的一个示例如下。


```

           / {
               /* ... */
               amba: amba {
                   #address-cells = <1>;
                   #size-cells = <1>;
                   compatible = "simple-bus";

                   CTU_CAN_FD_0: CTU_CAN_FD@43c30000 {
                       compatible = "ctu,ctucanfd";
                       interrupt-parent = <&intc>;
                       interrupts = <0 30 4>;
                       clocks = <&clkc 15>;
                       reg = <0x43c30000 0x10000>;
                   };
               };
           };


```


#### 驱动结构


该驱动可以分为两部分——与平台相关的设备发现与初始化，以及与平台
无关的 CAN 网络设备实现。


##### 平台设备驱动


对于 Zynq，该核通过 AXI 系统总线连接，该总线不支持枚举，因此设备
必须在 Device Tree 中指定。这类设备在 kernel 中称为 **platform device**
（平台设备），由 **platform device driver**（平台设备驱动）\  [^1^]_ 处理。

一个平台设备驱动提供以下内容：

- 一个 **probe** 函数

- 一个 **remove** 函数

- 一张该驱动能够处理的 **compatible**（兼容）设备表

**probe** 函数在设备出现时（或驱动加载时，以较晚者为准）被恰好调用一次。
如果同一个驱动处理多个设备，则会对每个设备调用一次 **probe** 函数。
它的作用是分配并初始化处理设备所需的资源，以及为与平台无关的
层设置底层函数，例如 **read_reg** 和 **write_reg**。
之后，驱动将设备注册到更高层，在本例中注册为 **network device**（网络设备）。

**remove** 函数在设备消失或驱动即将卸载时被调用。它用于释放
在 **probe** 中分配的资源，并将设备从更高层注销。

最后，**compatible** 设备表声明了该驱动能够处理的设备。Device Tree
条目 `compatible` 会与所有 **platform drivers**（平台驱动）的表进行匹配。


           ```c
           /** Match table for OF platform binding **/
           static const struct of_device_id ctucan_of_match[] = {
               { .compatible = "ctu,canfd-2", },
               { .compatible = "ctu,ctucanfd", },
               { /** end of list **/ },
           };
           MODULE_DEVICE_TABLE(of, ctucan_of_match);

           static int ctucan_probe(struct platform_device *pdev);
           static int ctucan_remove(struct platform_device *pdev);

           static struct platform_driver ctucanfd_driver = {
               .probe  = ctucan_probe,
               .remove = ctucan_remove,
               .driver = {
                   .name = DRIVER_NAME,
                   .of_match_table = ctucan_of_match,
               },
           };
           module_platform_driver(ctucanfd_driver);
           ```



##### 网络设备驱动


每个网络设备必须至少支持以下操作：

- 启动设备：`ndo_open`

- 关闭设备：`ndo_close`

- 向设备提交 TX 帧：`ndo_start_xmit`

- 向网络子系统报告 TX 完成与错误：ISR

- 向网络子系统提交 RX 帧：ISR 与 NAPI

事件来源有两种可能：设备和网络子系统。设备事件通常通过中断发出信号，
由中断服务程序（ISR）处理。源自网络子系统的事件处理程序则在
`struct net_device_ops` 中指定。

当设备被启动时，例如通过调用 `ip link set can0 up`，
会调用驱动的 `ndo_open` 函数。它应当校验接口配置并配置和启用设备。
相反的操作是 `ndo_close`，在设备被关闭时调用，无论是显式还是隐式。

当系统需要发送一个帧时，它通过调用 `ndo_start_xmit` 来实现，该函数将
帧入队到设备。如果设备的 HW 队列（FIFO、邮箱或任何实现方式）变满，
`ndo_start_xmit` 的实现会通知网络子系统它应当停止 TX 队列
（通过 `netif_stop_queue`）。之后当设备再次有可用空间并能够入队
另一个帧时，会在 ISR 中重新启用队列。

所有设备事件都在 ISR 中处理，具体包括：

#. **TX 完成**。当设备成功完成一个帧的发送时，该帧会在本地回显。
   发生错误时，则改为向网络子系统发送一个信息性错误帧 [^2^]_。
   在这两种情况下，软件 TX 队列都会被恢复，以便可以发送更多帧。

#. **错误状态**。如果出错（例如设备进入 bus-off 状态或发生 RX 溢出），
   错误计数器会被更新，信息性错误帧会被入队到 SW RX 队列。

#. **RX 缓冲区非空**。在这种情况下，读取 RX 帧并将其入队到 SW RX 队列。
   通常使用 NAPI 作为中间层（参见 ）。


#### NAPI


传入帧的频率可能很高，而每帧都调用中断服务程序的开销会造成显著的
系统负载。Linux 内核中有多种机制来处理这种情况。它们是随着 Linux
内核多年的发展和改进而演进出来的。对于网络设备，当前的标准是
NAPI——**New API（新 API）**。它类似于经典的 top-half/bottom-half
中断处理，即它仅在 ISR 中确认中断，并表明其余处理应在 softirq
上下文中完成。此外，它还提供了在一段时间内 **轮询（poll）** 新帧的可能性。
这有可能避免启用中断、在 ISR 中处理传入 IRQ、重新启用 softirq 以及
将上下文切换回 softirq 这一代价高昂的过程。

更多信息参见 Documentation/networking/napi.rst <napi>。


### 将核集成到 Xilinx Zynq


该核接口的是 Avalon 总线的一个简单子集
（参见 Intel **Avalon Interface Specifications**），
因为它最初用于 Altera FPGA 芯片上，而 Xilinx 原生使用 AXI 接口
（参见 ARM **AMBA AXI and ACE Protocol Specification AXI3,
AXI4, and AXI4-Lite, ACE and ACE-Lite**）。
最明显的解决方案是使用一个 Avalon/AXI 桥或实现某种简单的转换实体。
然而，该核的接口是半双工的，没有握手信号，而 AXI 是全双工的，
具有双向信号。此外，即便是 AXI-Lite 从接口也相当消耗资源，而 CAN
核并不需要 AXI 的灵活性与速度。

因此选择了一个简单得多的总线——APB（Advanced Peripheral Bus，高级外设总线）
（参见 ARM **AMBA APB Protocol Specification**）。
APB-AXI 桥在 Xilinx Vivado 中直接可用，接口适配实体只是一组简单的
组合逻辑赋值。

最后，为了能够将该核作为自定义 IP 包含在框图中，核连同 APB 接口
一起被打包为 Vivado 组件。


### CTU CAN FD 驱动设计


CAN 设备驱动的一般结构已在 中介绍过。接下来的段落将具体提供对 CTU
CAN FD 核驱动的更详细描述。


#### 底层驱动


该核并非仅供 SocketCAN 使用，因此最好拥有一个与 OS 无关的底层驱动。
这个底层驱动随后可以用于 OS 驱动的实现中，或者直接用于裸机或
用户空间应用程序中。另一个优点是，如果硬件略有变化，只需修改
底层驱动即可。

代码 [^3^]_ 部分由工具自动生成，部分由核作者手工编写，并包含论文
作者的贡献。底层驱动支持诸如以下操作：设置位时序、设置控制器模式、
启用/禁用、读取 RX 帧、写入 TX 帧等等。


#### 配置位时序


在 CAN 中，每个位被分为四个段：SYNC、PROP、PHASE1 和 PHASE2。它们的
持续时间以时间量子（Time Quantum）的倍数表示
（详见 `CAN Specification, Version 2.0 <http://esd.cs.ucr.edu/webres/can20.pdf>`_ 第 8 章）。
配置波特率（bitrate）时，所有段的持续时间（以及时间量子）必须根据
波特率和采样点（Sample Point）计算得出。对于 CAN FD，标称波特率
（Nominal bitrate）和数据波特率（Data bitrate）是分别独立计算的。

SocketCAN 相当灵活，既可以通过手动设置所有段的持续时间来提供高度
自定义的配置，也可以通过仅设置波特率和采样点来提供便捷的配置
（如果未指定，甚至会根据 Bosch 建议自动选择）。然而，每个 CAN 控制器
可能具有不同的基准时钟频率和不同的段持续时间寄存器宽度。因此算法
需要持续时间的（以及时钟预分频器的）最小值和最大值，并尝试优化这些
数值以同时满足约束条件和所请求的参数。


           ```c
           struct can_bittiming_const {
               char name[^16^];      /** Name of the CAN controller hardware **/
               __u32 tseg1_min;    /** Time segment 1 = prop_seg + phase_seg1 **/
               __u32 tseg1_max;
               __u32 tseg2_min;    /** Time segment 2 = phase_seg2 **/
               __u32 tseg2_max;
               __u32 sjw_max;      /** Synchronisation jump width **/
               __u32 brp_min;      /** Bit-rate prescaler **/
               __u32 brp_max;
               __u32 brp_inc;
           };
           ```


[lst:can_bittiming_const]

细心的读者会注意到，PROP_SEG 和 PHASE_SEG1 段的持续时间不是分别确定的，
而是先合并，然后默认情况下将得到的 TSEG1 在 PROP_SEG 和 PHASE_SEG1 之间
平均分配。实际上这几乎没有什么影响，因为采样点位于 PHASE_SEG1 和
PHASE_SEG2 之间。然而在 CTU CAN FD 中，`PROP` 和 `PH1` 持续时间寄存器
具有不同宽度（分别为 6 位和 7 位），因此自动计算的值可能会溢出较短的
寄存器，从而必须在两者之间重新分配 [^4^]_。


#### 处理 RX


帧接收在 NAPI 队列中处理，当 RXNE（RX FIFO Not Empty，RX FIFO 非空）
位被置位时，由 ISR 启用。帧被逐个读取，直到 RX FIFO 中没有剩余帧，
或 NAPI 轮询运行达到最大工作配额（参见 ）。然后每帧被传递给网络
接口 RX 队列。

传入的帧可能是 CAN 2.0 帧或 CAN FD 帧。在内核中区分这两者的方法是
分配 `struct can_frame` 或 `struct canfd_frame`，两者大小不同。
在控制器中，关于帧类型的信息存储在 RX FIFO 的第一个字中。

这就给我们带来了一个先有鸡还是先有蛋的问题：我们希望为帧分配 `skb`，
并且只有在分配成功时才从 FIFO 中取出帧；否则将其保留在那里稍后处理。
但是为了能够分配正确的 `skb`，我们必须先从 FIFO 中取出第一个字。有几种
可能的解决方案：

#. 读取该字，然后分配。如果失败，则丢弃帧的其余部分。当系统内存
   不足时，情况本来就很糟糕。

#. 预先始终分配足够大以容纳 FD 帧的 `skb`。然后调整 `skb` 内部，使其
   看起来像是为较小的 CAN 2.0 帧分配的。

#. 增加窥视（peek）FIFO 而非消费该字的选项。

#. 如果分配失败，将读取的字存入驱动的数据中。下次尝试时，使用
   存储的字而不是再次读取。

方案 1 足够简单，但如果我们能做得更好，它就不太令人满意。方案 2
不可接受，因为它需要修改一个完整内核结构的私有状态。略微增加的
内存消耗不过是“蛋糕”上的虚拟樱桃。方案 3 需要不小的硬件改动，
从硬件角度来看也不理想。

方案 4 似乎是一个不错的折中，其缺点是部分帧可能会在 FIFO 中停留
较长时间。尽管如此，RX FIFO 可能只有一个拥有者，因此其他任何人都
不应看到该部分帧（忽略某些特殊的调试场景）。此外，驱动在初始化时
会重置核，因此该部分帧也无法被“收养”。最终选择了方案 4 [^5^]_。


##### 为 RX 帧打时间戳


CTU CAN FD 核会报告帧被接收的确切时间戳。时间戳默认在 EOF 最后一位的
采样点捕获，但可配置为在 SOF 位捕获。时间戳源在核外部，宽度可达 64 位。
在撰写本文时，将时间戳从内核传递到用户空间的功能尚未实现，但计划在
未来完成。


#### 处理 TX


CTU CAN FD 核有 4 个独立的 TX 缓冲区，每个都有自己的状态和优先级。当
核想要发送时，会选择处于 Ready 状态且优先级最高的 TX 缓冲区。

优先级是寄存器 TX_PRIORITY 中的 3 位数值（nibble 对齐）。对于大多数
用例，这应该足够灵活。然而，SocketCAN 仅为传出帧支持一个 FIFO 队列 [^6^]_。
缓冲区优先级可用于模拟 FIFO 行为，方法是为每个缓冲区分配不同的优先级，
并在一帧传输完成后 **轮转（rotating）** 优先级。

除了优先级轮转之外，SW 还必须维护指向由 TX 缓冲区组成的 FIFO 的头尾指针，
以便确定下一个帧应使用哪个缓冲区（`txb_head`）以及哪个缓冲区应是最先
完成的（`txb_tail`）。实际的缓冲区索引（显然）是模 4 的（TX 缓冲区数量），
但指针必须至少宽一位，以便区分 FIFO 满和 FIFO 空——在这种情况下，
`txb\_head \equiv txb\_tail\ (\textrm{mod}\ 4)`。下面给出了如何维护
FIFO 以及优先级轮转的示例


|

+------+---+---+---+---+
| TXB# | 0 | 1 | 2 | 3 |
+======+===+===+===+===+
| Seq  | A | B | C |   |
+------+---+---+---+---+
| Prio | 7 | 6 | 5 | 4 |
+------+---+---+---+---+
|      |   | T |   | H |
+------+---+---+---+---+

|

+------+---+---+---+---+
| TXB# | 0 | 1 | 2 | 3 |
+======+===+===+===+===+
| Seq  |   | B | C |   |
+------+---+---+---+---+
| Prio | 4 | 7 | 6 | 5 |
+------+---+---+---+---+
|      |   | T |   | H |
+------+---+---+---+---+

|

+------+---+---+---+---+----+
| TXB# | 0 | 1 | 2 | 3 | 0’ |
+======+===+===+===+===+====+
| Seq  | E | B | C | D |    |
+------+---+---+---+---+----+
| Prio | 4 | 7 | 6 | 5 |    |
+------+---+---+---+---+----+
|      |   | T |   |   | H  |
+------+---+---+---+---+----+

|

   TX 缓冲区的状态及其可能的转换


##### 为 TX 帧打时间戳


向 TX 缓冲区提交帧时，可以指定该帧应被发送的时间戳。帧的发送可能会
更晚开始，但不会更早。注意，时间戳不参与缓冲区优先级排序——这完全
由上述机制决定。

对基于时间的报文发送的支持最近已被合并到 Linux v4.19
`Time-based packet transmission <https://lwn.net/Articles/748879/>`_，
但这项功能对于 CAN 是否实用仍有待研究。

同样类似于获取 RX 帧的时间戳，该核也支持获取 TX 帧的时间戳——即帧
被成功发送的时间。其细节与为 RX 帧打时间戳非常相似，并在 中描述。


#### 处理 RX 缓冲区溢出


当接收到的帧无法完整放入硬件 RX FIFO 时，RX FIFO 溢出标志（STATUS[DOR]）
会被置位，并触发数据溢出中断（DOI）。在处理该中断时，必须注意先清除
DOR 标志（通过 COMMAND[CDO]），然后再清除 DOI 中断标志。否则，该中断会
立即 [^7^]_ 重新触发。

**注意**：在开发过程中，曾讨论过内部 HW 流水线是否会扰乱这个清除
顺序，以及是否在清除标志和中断之间需要额外的空周期。在 Avalon 接口上，
确实被证明是这样，但 APB 是安全的，因为它使用 2 周期事务。本质上，
DOR 标志会被清除，但在 DOI 清除请求也应用的那个周期（通过将寄存器的
Reset 输入置高），DOI 寄存器的 Preset 输入仍然为高。由于 Set 的优先级
高于 Reset，DOI 标志不会被复位。这已经通过交换 Set/Reset 优先级得到
修复（参见 issue #187）。


#### 报告 Error Passive 与 Bus Off 状态


可能需要在节点达到 **Error Passive**、**Error Warning** 和 **Bus Off** 状态时
进行报告。驱动通过中断（EPI、EWLI）获知错误状态的变化，然后读取错误
计数器来确定核的错误状态。

然而，这里存在一个轻微的竞态条件——状态转换发生（以及中断被触发）
的时间与读取错误计数器的时间之间存在延迟。当收到 EPI 时，节点可能
处于 **Error Passive** 或 **Bus Off** 状态。如果节点进入 **Bus Off**，它显然
会保持该状态直到被复位。否则，节点 **当前或曾经** 处于 **Error Passive**。
然而，也有可能读取到的状态是 **Error Warning** 甚至 **Error Active**。在
这种情况下，是否以及究竟报告什么可能并不明确，但我个人倾向于认为
仍应报告过去的错误状态。类似地，当收到 EWLI 但随后检测到的状态是
**Error Passive** 时，应报告 **Error Passive**。


### CTU CAN FD 驱动源码参考


   :internal:

   :internal:

   :internal:

   :internal:


### CTU CAN FD IP 核与驱动开发致谢


- Odrej Ille <ondrej.ille@gmail.com>

  - 作为 CTU 测量系的学生启动了该项目
  - 多年来为项目投入了大量个人时间与热情
  - 参与了更多受资助的任务

- `Department of Measurement <https://meas.fel.cvut.cz/>`_、
  `Faculty of Electrical Engineering <http://www.fel.cvut.cz/en/>`_、
  `Czech Technical University <https://www.cvut.cz/en>`_

  - 多年来是该项目的主要投资方
  - 在其面向 `Skoda Auto <https://www.skoda-auto.cz/>`_ 的 CAN/CAN FD 诊断框架中使用该项目

- `Digiteq Automotive <https://www.digiteqautomotive.com/en>`_

  - 资助了“CAN FD Open Cores Support Linux Kernel Based Systems”项目
  - 与 CTU 协商并付费以允许公众访问该项目
  - 为这项工作提供了额外资金

- `Department of Control Engineering <https://control.fel.cvut.cz/en>`_、
  `Faculty of Electrical Engineering <http://www.fel.cvut.cz/en/>`_、
  `Czech Technical University <https://www.cvut.cz/en>`_

  - 负责“CAN FD Open Cores Support Linux Kernel Based Systems”项目
  - 提供 GitLab 管理
  - 为持续集成提供虚拟服务器与计算能力
  - 为 HIL 持续集成测试提供硬件

- `PiKRON Ltd. <http://pikron.com/>`_

  - 为启动项目开源准备工作提供了少量资金

- Petr Porazil <porazil@pikron.com>

  - 设计 PCIe 收发器附加板并组装板卡
  - 为基于 MicroZed/Zynq 的系统设计和组装 MZ_APO 基板

- Martin Jerabek <martin.jerabek01@gmail.com>

  - Linux 驱动开发
  - 持续集成平台架构师与 GHDL 更新
  - 论文 `Open-source and Open-hardware CAN FD Protocol Support <https://dspace.cvut.cz/bitstream/handle/10467/80366/F3-DP-2019-Jerabek-Martin-Jerabek-thesis-2019-canfd.pdf>`_

- Jiri Novak <jnovak@fel.cvut.cz>

  - 在 CTU 测量系负责项目的启动、管理与使用

- Pavel Pisa <pisa@cmp.felk.cvut.cz>

  - 发起开源，在 CTU 控制工程系负责项目协调与管理

- Jaroslav Beran<jara.beran@gmail.com>

 - 负责 Intel SoC 的系统集成、核与驱动的测试和更新

- Carsten Emde (`OSADL <https://www.osadl.org/>`_)

 - 提供 OSADL 的专业知识以讨论 IP 核许可
 - 指出了 LGPL 可能的死锁以及 CAN 总线可能的专利问题，这促使 IP 核设计重新授权为类 BSD 许可

- Reiner Zitzmann and Holger Zeltwanger (`CAN in Automation <https://www.can-cia.org/>`_)

 - 提供了建议和帮助以向社区宣传该项目，并邀请我们参加关注 CAN 总线未来发展方向的活动

- Jan Charvat

 - 为 QEMU 实现了 CTU CAN FD 功能模型，该模型已集成到 QEMU 主线（`docs/system/devices/can.rst <https://www.qemu.org/docs/master/system/devices/can.html>`_）
 - 学士论文 Model of CAN FD Communication Controller for QEMU Emulator


### 注释


   其他总线有自己的特定驱动接口来设置设备。

   不要与 CAN Error Frame 混淆。这是一个 `can_frame`，其 `CAN_ERR_FLAG`
   被置位，并在其 `data` 字段中包含一些错误信息。

   可在 CTU CAN FD 仓库
   `<https://gitlab.fel.cvut.cz/canbus/ctucanfd_ip_core>`_ 中获取

   底层驱动函数 `ctucan_hw_set_nom_bittiming` 和
   `ctucan_hw_set_data_bittiming` 就是这样做的。

   在撰写本论文时，方案 1 仍在使用，该修改已排队在 gitlab issue #222 中

   严格来说，自 v4.19 起支持多个 CAN TX 队列
   `can: enable multi-queue for SocketCAN devices <https://lore.kernel.org/patchwork/patch/913526/>`_
   但尚无主线驱动使用它们。

   或者更确切地说，在下一个时钟周期
