
## PPP 通用驱动与通道接口


			   Paul Mackerras
			   paulus@samba.org

			      7 Feb 2002

linux-2.4 中的通用 PPP 驱动提供了一个 PPP 实现中通用功能的实现，包括：

- 网络接口单元（ppp0 等）
- 与网络代码的接口
- PPP 多链路：在多条链路之间拆分数据报，以及对接收到的分片进行排序与合并
- 通过 /dev/ppp 字符设备与 pppd 的接口
- 数据包压缩与解压缩
- TCP/IP 头部压缩与解压缩
- 为按需拨号和空闲超时检测网络流量
- 简单的数据包过滤

为了发送和接收 PPP 帧，通用 PPP 驱动调用 PPP `channels`（通道）的服务。一个
PPP 通道封装了一种将 PPP 帧从一台机器传输到另一台机器的机制。一个 PPP 通道
实现在内部可以任意复杂，但它与通用 PPP 代码的接口非常简单：它只需能够发送
PPP 帧、接收 PPP 帧，并选择性地处理 ioctl 请求。目前已有用于异步串行端口、
同步串行端口以及 PPP over ethernet 的 PPP 通道实现。

这种架构使得可以以自然、直接的方式实现 PPP 多链路，方法是允许将多个通道链接
到每个 ppp 网络接口单元。通用层负责在发送时拆分数据报，在接收时重新组合它们。


### PPP 通道 API


参见 include/linux/ppp_channel.h，其中声明了通用 PPP 层与 PPP 通道之间用于
通信的类型和函数。

每个通道必须通过 ppp_channel.ops 指针向通用 PPP 层提供两个函数：

- start_xmit()：当通用层有帧要发送时调用。通道可以选择出于流控原因拒绝该帧。
  在这种情况下，start_xmit() 应返回 0，并且通道应在稍后能再次接受帧时调用
  ppp_output_wakeup() 函数，通用层随后将尝试重传被拒绝的帧。如果帧被接受，
  start_xmit() 函数应返回 1。

- ioctl()：提供一个可供用户空间程序用来控制通道行为某些方面的接口。当用户
  空间程序对一个绑定到该通道的 /dev/ppp 实例执行 ioctl 系统调用时，会调用此
  过程。（通常只有 pppd 会这样做。）

通用 PPP 层向通道提供七个函数：

- ppp_register_channel()：在通道被创建时调用，以通知 PPP 通用层它的存在。例如，
  将串行端口设置为 PPPDISC 线路规程会导致 ppp_async 通道代码调用此函数。

- ppp_unregister_channel()：在通道将要被销毁时调用。例如，当在串行端口上检测到
  挂断时，ppp_async 通道代码会调用此函数。

- ppp_output_wakeup()：由通道在其先前拒绝了对其 start_xmit 函数的调用、且现在
  可以接受更多数据包时调用。

- ppp_input()：由通道在收到一个完整 PPP 帧时调用。

- ppp_input_error()：由通道在检测到某个帧已丢失或被丢弃时调用（例如，因为
  FCS（帧校验序列）错误）。

- ppp_channel_index()：返回由 PPP 通用层分配给该通道的通道索引。通道应提供某种
  方式（例如一个 ioctl）将此索引传回用户空间，因为用户空间需要它来将一个
  /dev/ppp 实例关联到该通道。

- ppp_unit_number()：返回该通道所连接的 ppp 网络接口的单元号，如果通道未连接
  则返回 -1。

将通道连接到 PPP 通用层是由通道代码发起的，而不是由通用层发起。通道应提供某种
方式，供用户级进程独立于 PPP 通用层来控制它。例如，对于 ppp_async 通道，这是由
指向串行端口的文件描述符提供的。

通常，一个用户级进程会初始化底层通信媒介并使其准备好运行 PPP。例如，对于异步
tty，这可能涉及设置 tty 速率和模式、发出调制解调器命令，然后与远程系统进行某种
对话以在那里启用 PPP 服务。我们把这个过程称为 `discovery`（发现）。然后用户级
进程告诉该媒介成为 PPP 通道并向通用 PPP 层注册自己。随后通道必须将分配给它的
通道号报告回用户级进程。从那时起，PPP 守护进程（pppd）中的 PPP 协商代码就可以
接管并执行 PPP 协商，通过 /dev/ppp 接口访问该通道。

在 PPP 通用层的接口处，PPP 帧存储在 skbuff 结构中，并以两字节的 PPP 协议号开头。
该帧**不**包含 0xff `address`（地址）字节或 0x03 `control`（控制）字节，这两个
字节在异步 PPP 中是可选的。也没有任何控制字符的转义，也不包含任何 FCS 或成帧
字符。如果特定媒介需要，那完全由通道代码负责。也就是说，呈现给 start_xmit()
函数的 skbuff 只包含 2 字节协议号和数据，而呈现给 ppp_input() 的 skbuff 也必须是
相同的格式。

通道必须提供一个 ppp_channel 结构实例来表示该通道。通道可以自由地按需使用
`private` 字段。通道应在调用 ppp_register_channel() 之前初始化 `mtu` 和 `hdrlen`
字段，并且在 ppp_unregister_channel() 返回之前不要更改它们。`mtu` 字段表示 PPP
帧数据部分的最大大小，也就是说，它不包含 2 字节协议号。

如果通道在呈现给它用于发送的 skbuff 中需要一些头部空间（即 PPP 帧开始之前
skbuff 数据区中有一些空闲空间），它应将 ppp_channel 结构的 `hdrlen` 字段设置为
所需的头部空间大小。通用 PPP 层会尝试提供那么多头部空间，但通道仍应检查是否有
足够的头部空间，如果没有则复制该 skbuff。

在输入侧，通道理想情况下应在呈现给 ppp_input() 的 skbuff 中提供至少 2 字节的
头部空间。通用 PPP 代码并不要求这一点，但如果这样做会更高效。


### 缓冲与流控


通用 PPP 层被设计为尽量减少它在发送方向上缓冲的数据量。它为 PPP 单元（网络接口
设备）维护一个发送数据包队列，并为每个附着的通道维护一个发送数据包队列。通常
该单元的发送队列最多包含一个数据包；例外情况是 pppd 通过写入 /dev/ppp 发送
数据包，以及核心网络代码在队列停止时调用通用层的 start_xmit() 函数，即当通用层
调用了 netif_stop_queue() 时，这只在发送超时时发生。start_xmit 函数总是接受并
将其被要求发送的包入队。

发送数据包从 PPP 单元发送队列中取出，然后根据情况经受 TCP/IP 头部压缩和数据包
压缩（Deflate 或 BSD-Compress 压缩）。在此之后，数据包不能再被重新排序，因为
解压缩算法依赖于按生成顺序接收压缩数据包。

如果未使用多链路，则该数据包随后被传递给附着通道的 start_xmit() 函数。如果通道
拒绝接收该数据包，通用层会将其保存以供稍后传输。通用层会在通道调用
ppp_output_wakeup() 时，或当核心网络代码再次调用通用层的 start_xmit() 函数时，
再次调用该通道的 start_xmit() 函数。通用层不包含超时和重传逻辑；它依赖于核心
网络代码来实现这一点。

如果使用了多链路，通用层将该数据包拆分为一个或多个分片，并在每个分片上加上
多链路头部。它根据数据包的长度以及当前潜在能够接收分片的通道数量来决定使用
多少分片。如果一个通道当前没有为其排队的待发送分片，则该通道潜在能够接收一个
分片。通道仍可能拒绝某个分片；在这种情况下，该分片被排队等待该通道稍后发送。
这种方案的效果是，更多分片会被分配给带宽更高的通道。这也意味着在轻负载下，
通用层倾向于跨所有通道对大数据包进行分片，从而降低延迟；而在重负载下，数据包
倾向于作为单个分片传输，从而减少分片带来的开销。


### SMP 安全性


通用 PPP 层被设计为 SMP 安全的。在必要时，对内部数据结构的访问周围会使用锁，
以确保其完整性。作为其中的一部分，通用层要求通道遵守某些要求，反过来也向通道
提供某些保证。本质上，要求通道对构成通道与通用层之间通信基础的 ppp_channel
结构提供适当的加锁。这是因为通道提供了 ppp_channel 结构的存储，因此要求通道
保证该存储在适当的时候存在且有效。

通用层要求通道提供以下保证：

- ppp_channel 对象必须从调用 ppp_register_channel() 之时起一直存在，直到
  ppp_unregister_channel() 调用返回之后。

- 在为某通道调用 ppp_unregister_channel() 时，不能有任何线程正处于对该通道的
  ppp_input()、ppp_input_error()、ppp_output_wakeup()、ppp_channel_index() 或
  ppp_unit_number() 的调用之中。

- ppp_register_channel() 和 ppp_unregister_channel() 必须从进程上下文调用，而
  不是从中断或 softirq/BH 上下文调用。

- 其余的通用层函数可以在 softirq/BH 级别调用，但不得从硬件中断处理程序中调用。

- 通用层可能在 softirq/BH 级别调用通道的 start_xmit() 函数，但不会在中断级别
  调用它。因此 start_xmit() 函数不能阻塞。

- 通用层只会在进程上下文中调用通道的 ioctl() 函数。

通用层向通道提供以下保证：

- 当任何线程已经正在执行某通道的 start_xmit() 函数时，通用层不会为该通道调用
  start_xmit() 函数。

- 当任何线程已经正在执行某通道的 ioctl() 函数时，通用层不会为该通道调用
  ioctl() 函数。

- 到 ppp_unregister_channel() 调用返回时，将没有任何线程正在执行从通用层到该
  通道的 start_xmit() 或 ioctl() 函数的调用，并且通用层此后也不会再调用这两个
  函数中的任何一个。


### 与 pppd 的接口


通用 PPP 层导出一个名为 /dev/ppp 的字符设备接口。pppd 用它来控制 PPP 接口单元
和通道。虽然只有一个 /dev/ppp，但每个打开的 /dev/ppp 实例都独立运行，并且可以
关联到一个 PPP 单元或一个 PPP 通道。这是通过使用 file->private_data 字段指向每个
打开的 /dev/ppp 实例的独立对象来实现的。这样便获得了类似于 Solaris 的 clone
open 的效果，使我们能够控制任意数量的 PPP 接口和通道，而无需在 /dev 中塞满
数百个设备名。

当 /dev/ppp 被打开时，会创建一个最初未关联的新实例。使用 ioctl 调用，它可以
随后被关联到一个已有的单元、关联到一个新创建的单元，或关联到一个已有的通道。
关联到一个单元的实例可用于使用 read() 和 write() 系统调用（必要时加上 poll()）
发送和接收 PPP 控制帧。类似地，关联到一个通道的实例可用于在该通道上发送和
接收 PPP 帧。

在多链路术语中，单元代表捆绑（bundle），而通道代表各个物理链路。因此，通过写入
单元（即写入一个关联到该单元的 /dev/ppp 实例）发送的 PPP 帧将受到捆绑级压缩，
并会在各条链路之间分片（如果使用了多链路）。相反，通过写入通道发送的 PPP 帧
将在该通道上原样发送，不带任何多链路头部。

一个通道最初不关联到任何单元。在这种状态下，它可用于 PPP 协商，但不能用于
传输数据数据包。然后它可以通过 ioctl 调用连接到一个 PPP 单元，这样它就能用于
发送和接收该单元的数据数据包。

在一个 /dev/ppp 实例上可用的 ioctl 调用取决于它是未关联、关联到 PPP 接口，还是
关联到 PPP 通道。在未关联实例上可用的 ioctl 调用有：

- PPPIOCNEWUNIT：创建一个新的 PPP 接口，并使此 /dev/ppp 实例成为该接口的
  “所有者”。参数应指向一个 int，若 >= 0 则为期望的单元号，若为 -1 则分配最小的
  未使用单元号。作为接口的所有者意味着，如果此 /dev/ppp 实例被关闭，该接口将被
  关闭。

- PPPIOCATTACH：将此实例关联到一个已有的 PPP 接口。参数应指向一个包含单元号的
  int。这不会使此实例成为该 PPP 接口的所有者。

- PPPIOCATTCHAN：将此实例关联到一个已有的 PPP 通道。参数应指向一个包含通道号的
  int。

在关联到一个通道的 /dev/ppp 实例上可用的 ioctl 调用有：

- PPPIOCCONNECT：将此通道连接到一个 PPP 接口。参数应指向一个包含接口单元号的
  int。如果该通道已连接到一个接口，则返回 EINVAL 错误；如果请求的接口不存在，
  则返回 ENXIO。

- PPPIOCDISCONN：将此通道从其连接的 PPP 接口断开。如果该通道未连接到任何接口，
  则返回 EINVAL 错误。

- PPPIOCBRIDGECHAN：将一个通道与另一个通道桥接。参数应指向一个包含要桥接到的
  通道的通道号的 int。一旦两个通道被桥接，通过一个通道由 ppp_input() 呈现的帧
  会被传递给桥接实例以继续传输。这允许将帧从一个通道交换到另一个通道：例如，将
  PPPoE 帧传入一个 PPPoL2TP 会话。由于通道桥接会中断正常的 ppp_input() 路径，
  给定通道不能同时既作为桥接的一部分，又作为单元的一部分。如果该通道已经是桥接
  或单元的一部分，此 ioctl 会返回 EALREADY 错误；如果请求的通道不存在，则返回
  ENXIO。

- PPPIOCUNBRIDGECHAN：执行 PPPIOCBRIDGECHAN 的逆操作，解除一对通道的桥接。如果
  该通道不构成桥接的一部分，此 ioctl 会返回 EINVAL 错误。

- 所有其他 ioctl 命令都传递给通道的 ioctl() 函数。

在关联到一个接口单元的实例上可用的 ioctl 调用有：

- PPPIOCSMRU：设置接口的 MRU（最大接收单元）。参数应指向一个包含新 MRU 值的
  int。

- PPPIOCSFLAGS：设置控制接口运行的标志。参数应是指向一个包含新标志值的 int 的
  指针。可以设置的标志值中的位是：

	================	========================================
	SC_COMP_TCP		enable transmit TCP header compression
	SC_NO_TCP_CCID		disable connection-id compression for
				TCP header compression
	SC_REJ_COMP_TCP		disable receive TCP header decompression
	SC_CCP_OPEN		Compression Control Protocol (CCP) is
				open, so inspect CCP packets
	SC_CCP_UP		CCP is up, may (de)compress packets
	SC_LOOP_TRAFFIC		send IP traffic to pppd
	SC_MULTILINK		enable PPP multilink fragmentation on
				transmitted packets
	SC_MP_SHORTSEQ		expect short multilink sequence
				numbers on received multilink fragments
	SC_MP_XSHORTSEQ		transmit short multilink sequence nos.
	================	========================================

  这些标志的值在 <linux/ppp-ioctl.h> 中定义。注意，如果未选择 CONFIG_PPP_MULTILINK
  选项，则 SC_MULTILINK、SC_MP_SHORTSEQ 和 SC_MP_XSHORTSEQ 位的值会被忽略。

- PPPIOCGFLAGS：返回接口单元的状态/控制标志的值。参数应指向一个 int，ioctl 将在
  其中存储标志值。除了上面为 PPPIOCSFLAGS 列出的值外，返回的值中可能还会设置
  以下位：

	================	=========================================
	SC_COMP_RUN		CCP compressor is running
	SC_DECOMP_RUN		CCP decompressor is running
	SC_DC_ERROR		CCP decompressor detected non-fatal error
	SC_DC_FERROR		CCP decompressor detected fatal error
	================	=========================================

- PPPIOCSCOMPRESS：设置数据包压缩或解压缩的参数。参数应指向一个 ppp_option_data
  结构（定义于 <linux/ppp-ioctl.h>），其中包含一个指针/长度对，应描述一个包含
  指定压缩方法及其参数的 CCP 选项的内存块。ppp_option_data 结构体还包含一个
  `transmit` 字段。如果它为 0，则该 ioctl 会影响接收路径，否则影响发送路径。

- PPPIOCGUNIT：在参数指向的 int 中返回此接口单元的单元号。

- PPPIOCSDEBUG：将接口的调试标志设置为参数指向的 int 中的值。只使用最低有效位；
  如果为 1，通用层会在其运行期间打印一些调试消息。这仅用于调试通用 PPP 层代码；
  它通常对于搞清楚 PPP 连接为什么失败没有帮助。

- PPPIOCGDEBUG：在参数指向的 int 中返回接口的调试标志。

- PPPIOCGIDLE：返回自上次发送和接收数据数据包以来经过的时间（秒）。参数应指向
  一个 ppp_idle 结构（定义于 <linux/ppp_defs.h>）。如果启用了 CONFIG_PPP_FILTER
  选项，则重置发送和接收空闲定时器的数据包集合被限制为通过 `active`（活跃）数据包
  过滤器的那些。此命令存在两个版本，以处理用户空间期望将时间视为 32 位或 64 位
  time_t 秒的情况。

- PPPIOCSMAXCID：设置 TCP 头部压缩器和解压缩器的最大连接 ID 参数（以及因此的连接
  槽数量）。参数指向的 int 的低 16 位指定压缩器的最大连接 ID。如果该 int 的高
  16 位非零，则它们指定解压缩器的最大连接 ID，否则解压缩器的最大连接 ID 被设置为
  15。

- PPPIOCSNPMODE：设置给定网络协议的网络协议模式。参数应指向一个 npioctl 结构体
  （定义于 <linux/ppp-ioctl.h>）。`protocol` 字段给出要受影响的协议的 PPP 协议号，
  `mode` 字段指定如何处理该协议的数据包：

	=============	==============================================
	NPMODE_PASS	normal operation, transmit and receive packets
	NPMODE_DROP	silently drop packets for this protocol
	NPMODE_ERROR	drop packets and return an error on transmit
	NPMODE_QUEUE	queue up packets for transmit, drop received
			packets
	=============	==============================================

  目前 NPMODE_ERROR 和 NPMODE_QUEUE 与 NPMODE_DROP 效果相同。

- PPPIOCGNPMODE：返回给定协议的网络协议模式。参数应指向一个 npioctl 结构体，其中
  `protocol` 字段被设置为关注协议的 PPP 协议号。返回时，`mode` 字段将被设置为该
  协议的网络协议模式。

- PPPIOCSPASS 和 PPPIOCSACTIVE：设置 `pass` 和 `active` 数据包过滤器。这些 ioctl
  仅在选择了 CONFIG_PPP_FILTER 选项时可用。参数应指向一个 sock_fprog 结构（定义于
  <linux/filter.h>），其中包含该过滤器的已编译 BPF 指令。如果数据包未通过 `pass`
  过滤器，则被丢弃；否则，如果它们未通过 `active` 过滤器，它们会被放行，但不会
  重置发送或接收空闲定时器。

- PPPIOCSMRRU：启用或禁用对接收数据包的多链路处理，并设置多链路 MRRU（最大重建
  接收单元）。参数应指向一个包含新 MRRU 值的 int。如果 MRRU 值为 0，则禁用对接收
  多链路分片的处理。此 ioctl 仅在选择了 CONFIG_PPP_MULTILINK 选项时可用。

Last modified: 7-feb-2002
