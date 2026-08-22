## 閫氱敤 HDLC 灞。

Krzysztof Halasa <khc@pm.waw.pl>


通用 HDLC 层目前支持：

1. 帧中继（Frame Relay，ANSI、CCITT、Cisco 以及LMI
   - 普通（路由）和以太网桥接（以太网设备仿真）接口可以共享同一PVC   - ARP 支持（内核不支持 InARP —在以下地址有一个实验性的 InARP 用户态守护进程：
     http://www.kernel.org/pub/linux/utils/net/hdlc/）
2. 原始 HDLC —可以IP（IPv4）接口或以太网设备仿3. Cisco HDLC
4. PPP
5. X.25（使X.25 例程）
通用 HDLC 只是一个协议驱—它需要针对你特定硬件的低层驱动
使用 HDLC 或帧中继 PVC 的以太网设备仿真兼容 IEEE 802.1Q（VLAN）和 802.1D（以太网桥接）

确保 hdlc.o 和硬件驱动已加载。它应当创建若干“hdlc”（hdlc0 等）网络设备，每WAN 端口一个。你需“sethdlc工具，可从以下地址获取
	http://www.kernel.org/pub/linux/utils/net/hdlc/

```

	gcc -O2 -Wall -o sethdlc sethdlc.c

```
确保你使用的是与内核版本匹配sethdlc
使用 sethdlc 来设置物理接口、时钟速率、所使用HDLC 模式，并在使用帧中继时添所需PVC```

	sethdlc hdlc0 clock int rate 128000
	sethdlc hdlc0 cisco interval 10 timeout 25

```
```

	sethdlc hdlc0 rs232 clock ext
	sethdlc hdlc0 fr lmi ansi
	sethdlc hdlc0 create 99
	ifconfig hdlc0 up
	ifconfig pvc0 localIP pointopoint remoteIP

```
在帧中继模式下，在使pvc 设备之前，先ifconfig 将主 hdlc 设备 up（不要给它分任何 IP 地址）

设置接口
- v35 | rs232 | x21 | t1 | e1
    - 当卡具有软件可选接口时，设置给定端口的物理接口
  loopback
    - 激活硬件回环（仅用于测试）
- clock ext
    - RX 时钟TX 时钟均为外部
- clock int
    - RX 时钟TX 时钟均为内部
- clock txint
    - RX 时钟外部，TX 时钟内部
- clock txfromrx
    - RX 时钟外部，TX 时钟RX 时钟派生
- rate
    - 设置时钟速率（bps）（仅用“int“txint时钟

设置协议
- hdlc - 设置原始 HDLC（仅 IP）模
  nrz / nrzi / fm-mark / fm-space / manchester - 设置传输编码

  no-parity / crc16 / crc16-pr0（预置零CRC16 crc32-itu

  crc16-itu（使ITU-T 多项式的 CRC16 crc16-itu-pr0 - 设置奇偶校验

- hdlc-eth - 使用 HDLC 的以太网设备仿真。奇偶校验和编码同上
- cisco - 设置 Cisco HDLC 模式（支IP、IPv6 IPX
  interval - 保活包之间的时间间隔（秒
  timeout - 在假定链路断开前，距最后一次收到保活包的时间（秒）

- ppp - 设置同步 PPP 模式

- x25 - 设置 X.25 模式

- fr - 帧中继模
  lmi ansi / ccitt / cisco / none - LMI（链路管理）类型

  dce - 帧中DCE（网络侧）LMI，而非默认DTE（用户侧）
  它与时钟毫无关系
  - t391 - 链路完整性验证轮询定时器（秒 用户  - t392 - 轮询验证定时器（秒）- 网络  - n391 - 全状态轮询计数器 - 用户  - n392 - 错误阈- 用户侧和网络  - n393 - 受监控事件计- 用户侧和网络
仅帧中继
- create n | delete n - 添加/删除 DLCI #n PVC 接口  新创建的接口将命名为 pvc0、pvc1 等
- create ether n | delete ether n - 添加一个用于以太网桥接帧的设备。该设备将命名为
  pvceth0、pvceth1 等

### 板级特定问题

```

	insmod n2 hw=io,irq,ram,ports[:io,irq,...]

```
```

	insmod n2 hw=0x300,10,0xD0000,01

```
```

	insmod c101 hw=irq,ram[:irq,...]

```
```

	insmod c101 hw=9,0xdc000

```
```

	n2.hw=io,irq,ram,ports:...

```
```

	c101.hw=irq,ram:...


```
如果你在使用 N2、C101 PLX200SYN 卡时遇到问题，可以执```

	sethdlc hdlc0 private

```
硬件驱动必须在使#define DEBUG_RINGS 编译时构建。将此信息附bug 报告中会很有帮助无论如何，如果在使用中遇到问题，请告诉我
补丁和其它信息见<http://www.kernel.org/pub/linux/utils/net/hdlc/>