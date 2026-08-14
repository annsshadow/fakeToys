

## Altera 三速以太网 MAC 驱动


Copyright |copy| 2008-2014 Altera Corporation

这是用于 Altera 三速以太网（TSE）控制器、使用 SGDMA 和 MSGDMA 软 DMA IP 组件的驱动。该驱动使用平台
总线获取组件资源。用于测试此驱动的设计是针对 Cyclone(R) V SOC FPGA 板、Cyclone(R) V FPGA 板构建的，
并分别使用 ARM 和 NIOS 处理器主机进行了测试。预期用例是嵌入式系统与外部对等体之间进行简单的通信，
用于状态报告和嵌入式系统的简单配置。

更多信息请访问 www.altera.com 和 www.rocketboards.org。关于此驱动的支持论坛可以在 www.rocketboards.org
找到，用于测试此驱动的设计也可以在那里找到。此驱动的维护者（见 MAINTAINERS）也提供支持。

三速以太网、SGDMA 和 MSGDMA 组件都是软 IP 组件，可以使用 Altera Quartus 工具链组装并构建到 FPGA 中。
构建此驱动所针对的设计使用了 Quartus 13.1 和 14.0。sopc2dts 工具用于为驱动创建设备树，可以在
rocketboards.org 找到。

驱动探测（probe）函数检查设备树，并确定三速以太网实例是使用 SGDMA 还是 MSGDMA 组件。探测函数随后安装
相应的一组 DMA 例程，以初始化、建立发送、接收和中断处理原语，用于各自的配置。

SGDMA 组件在不久的将来（截至本文撰写时 2014 年初的接下来 1-2 年内）将被弃用，转而使用 MSGDMA 组件。
包含 SGDMA 支持是为了现有设计和参考，以防开发者希望支持他们自己的软 DMA 逻辑和驱动支持。任何新设计
都不应使用 SGDMA。

SGDMA 一次只支持单个发送或接收操作，因此与 MSGDMA 软 IP 相比性能较差。有关已知的、有文档记录的 SGDMA
勘误表，请访问 www.altera.com。

目前 SGDMA 和 MSGDMA 均不支持分散/聚集（scatter-gather）DMA。分散/聚集 DMA 将被添加到此驱动的未来维护
更新中。

目前不支持巨型帧（jumbo frame）。

驱动将 PHY 操作限制为 10/100Mbps，并且尚未针对 1Gbps 完全测试。此支持将被添加到未来的维护更新中。

## 1. 内核配置


内核配置选项是 ALTERA_TSE：

 Device Drivers ---> Network device support ---> Ethernet driver support --->
 Altera Triple-Speed Ethernet MAC support (ALTERA_TSE)

## 2. 驱动参数列表


 - debug: 消息级别（0：无输出，16：全部）；
 - dma_rx_num: RX 列表中的描述符数量（默认为 64）；
 - dma_tx_num: TX 列表中的描述符数量（默认为 64）。

## 3. 命令行选项


```

	altera_tse=dma_rx_num:128,dma_tx_num:512

```

## 4. 驱动信息和注意事项


### 4.1. 发送过程

当内核调用驱动的发送例程时，它通过调用底层 DMA 发送例程（SGDMA 或 MSGDMA）建立一个发送描述符，并
启动一个发送操作。一旦发送完成，发送 DMA 逻辑会驱动一个中断。驱动在中断处理链上下文中处理发送完成，
通过回收发送和跟踪所请求的发送操作所需的资源。

### 4.2. 接收过程

驱动将在驱动初始化期间向接收 DMA 逻辑投递接收缓冲区。根据底层 DMA 逻辑（MSGDMA 能够排队接收缓冲区，
SGDMA 无法向 SGDMA 接收逻辑排队接收缓冲区），接收缓冲区可能排队也可能不排队。当收到一个数据包时，DMA
逻辑生成一个中断。驱动通过获取 DMA 接收逻辑状态来处理接收中断，收割接收完成项，直到没有更多接收完成
项可用。

### 4.3. 中断缓和

驱动能够使用 NAPI 为接收操作缓和其 DMA 中断数量。目前不支持发送操作的中断缓和，但将被添加到未来的
维护版本中。

### 4.4) Ethtool 支持

支持 Ethtool。可以使用 ethtool -S ethX 命令获取驱动统计信息和内部错误。可以转储寄存器等。

### 4.5) PHY 支持

驱动与 PAL 兼容，可与 PHY 和 GPHY 设备一起工作。

### 4.7) 源文件列表：

 - Kconfig
 - Makefile
 - altera_tse_main.c: 主网络设备故障驱动
 - altera_tse_ethtool.c: ethtool 支持
 - altera_tse.h: 私有驱动结构和常见定义
 - altera_msgdma.h: MSGDMA 实现函数定义
 - altera_sgdma.h: SGDMA 实现函数定义
 - altera_msgdma.c: MSGDMA 实现
 - altera_sgdma.c: SGDMA 实现
 - altera_sgdmahw.h: SGDMA 寄存器和描述符定义
 - altera_msgdmahw.h: MSGDMA 寄存器和描述符定义
 - altera_utils.c: 驱动工具函数
 - altera_utils.h: 驱动工具函数定义

## 5. 调试信息


驱动导出调试信息，例如内部统计、调试信息、MAC 和 DMA 寄存器等。

用户可以使用 ethtool 支持来获取统计信息：
例如使用：ethtool -S ethX（显示统计计数器）
或查看 MAC 寄存器：例如使用：ethtool -d ethX

开发者还可以使用“debug”模块参数来获取更多调试信息。

## 6. 统计支持


控制器和驱动支持 IEEE 标准定义的统计、RFC 定义的统计以及驱动或 Altera 定义的统计的混合。包含这些统计
标准定义的四个规范如下：

 - IEEE 802.3-2012 - IEEE 以太网标准。
 - RFC 2863，位于 http://www.rfc-editor.org/rfc/rfc2863.txt。
 - RFC 2819，位于 http://www.rfc-editor.org/rfc/rfc2819.txt。
 - Altera 三速以太网用户指南，位于 http://www.altera.com

TSE 和设备驱动支持的统计如下：

"tx_packets" 等价于 IEEE 802.3-2012 第 5.2.2.1.2 节中定义的 aFramesTransmittedOK。此统计是成功发送的
帧的计数。

"rx_packets" 等价于 IEEE 802.3-2012 第 5.2.2.1.5 节中定义的 aFramesReceivedOK。此统计是成功接收到的
帧的计数。此计数不包括任何错误包，例如 CRC 错误、长度错误或对齐错误。

"rx_crc_errors" 等价于 IEEE 802.3-2012 第 5.2.2.1.6 节中定义的 aFrameCheckSequenceErrors。此统计是
在接收时长度为整数个字节且未通过 CRC 检验的帧的计数。

"rx_align_errors" 等价于 IEEE 802.3-2012 第 5.2.2.1.7 节中定义的 aAlignmentErrors。此统计是长度不是整数
个字节且在接收时未通过 CRC 检验的帧的计数。

"tx_bytes" 等价于 IEEE 802.3-2012 第 5.2.2.1.8 节中定义的 aOctetsTransmittedOK。此统计是从接口成功发送的
数据和填充字节的计数。

"rx_bytes" 等价于 IEEE 802.3-2012 第 5.2.2.1.14 节中定义的 aOctetsReceivedOK。此统计是控制器成功接收的
数据和填充字节的计数。

"tx_pause" 等价于 IEEE 802.3-2012 第 30.3.4.2 节中定义的 aPAUSEMACCtrlFramesTransmitted。此统计是从网络
控制器发送的 PAUSE 帧的计数。

"rx_pause" 等价于 IEEE 802.3-2012 第 30.3.4.3 节中定义的 aPAUSEMACCtrlFramesReceived。此统计是网络控制器
接收的 PAUSE 帧的计数。

"rx_errors" 等价于 RFC 2863 中定义的 ifInErrors。此统计是接收到的、因包含错误而无法递交给更高层协议的
数据包的数量。

"tx_errors" 等价于 RFC 2863 中定义的 ifOutErrors。此统计是因错误而无法发送的报文数量。

"rx_unicast" 等价于 RFC 2863 中定义的 ifInUcastPkts。此统计是接收到的、未寻址到广播地址或多播组的报文
数量。

"rx_multicast" 等价于 RFC 2863 中定义的 ifInMulticastPkts。此统计是接收到的、寻址到多播地址组的报文
数量。

"rx_broadcast" 等价于 RFC 2863 中定义的 ifInBroadcastPkts。此统计是接收到的、寻址到广播地址的报文数量。

"tx_discards" 等价于 RFC 2863 中定义的 ifOutDiscards。此统计是即使未检测到错误也未发送的出站数据包数量。
可能发生这种情况的一个例子是为了释放内部缓冲区空间。

"tx_unicast" 等价于 RFC 2863 中定义的 ifOutUcastPkts。此统计计数未寻址到多播组或广播地址的已发送报文
数量。

"tx_multicast" 等价于 RFC 2863 中定义的 ifOutMulticastPkts。此统计计数寻址到多播组的已发送报文数量。

"tx_broadcast" 等价于 RFC 2863 中定义的 ifOutBroadcastPkts。此统计计数寻址到广播地址的已发送报文数量。

"ether_drops" 等价于 RFC 2819 中定义的 etherStatsDropEvents。此统计因缺乏内部控制器资源而丢弃的数据包
数量。

"rx_total_bytes" 等价于 RFC 2819 中定义的 etherStatsOctets。此统计计数控制器接收的总字节数，包括错误和
丢弃的数据包。

"rx_total_packets" 等价于 RFC 2819 中定义的 etherStatsPkts。此统计计数控制器接收的总数据包数，包括错误、
丢弃、单播、多播和广播数据包。

"rx_undersize" 等价于 RFC 2819 中定义的 etherStatsUndersizePkts。此统计计数接收到的、长度小于 64 字节的
格式正确的数据包数量。

"rx_oversize" 等价于 RFC 2819 中定义的 etherStatsOversizePkts。此统计计数接收到的、长度大于 1518 字节的
格式正确的数据包数量。

"rx_64_bytes" 等价于 RFC 2819 中定义的 etherStatsPkts64Octets。此统计计数接收到的、长度为 64 字节的数据包
总数。

"rx_65_127_bytes" 等价于 RFC 2819 中定义的 etherStatsPkts65to127Octets。此统计计数接收到的、长度在 65 到
127 字节（含）之间的数据包总数。

"rx_128_255_bytes" 等价于 RFC 2819 中定义的 etherStatsPkts128to255Octets。此统计计数接收到的、长度在 128 到
255 字节（含）之间的数据包总数。

"rx_256_511_bytes" 等价于 RFC 2819 中定义的 etherStatsPkts256to511Octets。此统计计数接收到的、长度在 256 到
511 字节（含）之间的数据包总数。

"rx_512_1023_bytes" 等价于 RFC 2819 中定义的 etherStatsPkts512to1023Octets。此统计计数接收到的、长度在 512 到
1023 字节（含）之间的数据包总数。

"rx_1024_1518_bytes" 等价于 RFC 2819 中定义的 etherStatsPkts1024to1518Octets。此统计计数接收到的、长度在 1024 到
1518 字节（含）之间的数据包总数。

"rx_gte_1519_bytes" 是 Altera TSE 特有行为定义的统计。此统计计数长度在 1519 和 frm_length 寄存器中配置的最大
帧长度之间接收到的正常和错误帧的数量。更多细节请参阅 Altera TSE 用户指南。

"rx_jabbers" 等价于 RFC 2819 中定义的 etherStatsJabbers。此统计计数接收到的、长度大于 1518 字节、并且具有
整数个字节的错误 CRC（CRC 错误）或非整数个字节的错误 CRC（对齐错误）的数据包总数。

"rx_runts" 等价于 RFC 2819 中定义的 etherStatsFragments。此统计计数接收到的、长度小于 64 字节、并且具有整数
个字节的错误 CRC（CRC 错误）或非整数个字节的错误 CRC（对齐错误）的数据包总数。
