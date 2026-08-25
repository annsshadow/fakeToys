
## 3Com Vortex 设备驱动


Andrew Morton

2000 骞?4 鏈?30 鏃。

本文档描述了用于 Linux 3Com "Vortex" 设备驱动 3c59x.c 的使用方法与勘误
该驱动由 Donald Becker <becker@scyld.com> 编写
Don 已不再是此版本驱动的主要维护者。请将问题报告给以下一位或多位
- Andrew Morton
- Netdev 邮件列表 <netdev@vger.kernel.org>
- Linux 内核邮件列表 <linux-kernel@vger.kernel.org>

请注意本文档末尾的“报告与诊断问题”一节

自内2.3.99-pre6 起，该驱动已整合3c575 系列 Cardbus 卡的支持，这些卡此前3c575_cb.c 处理
本驱动支持以下硬件：

 - 3c590 Vortex 10Mbps
 - 3c592 EISA 10Mbps Demon/Vortex
 - 3c597 EISA Fast Demon/Vortex
 - 3c595 Vortex 100baseTx
 - 3c595 Vortex 100baseT4
 - 3c595 Vortex 100base-MII
 - 3c900 Boomerang 10baseT
 - 3c900 Boomerang 10Mbps Combo
 - 3c900 Cyclone 10Mbps TPO
 - 3c900 Cyclone 10Mbps Combo
 - 3c900 Cyclone 10Mbps TPC
 - 3c900B-FL Cyclone 10base-FL
 - 3c905 Boomerang 100baseTx
 - 3c905 Boomerang 100baseT4
 - 3c905B Cyclone 100baseTx
 - 3c905B Cyclone 10/100/BNC
 - 3c905B-FX Cyclone 100baseFx
 - 3c905C Tornado
 - 3c920B-EMB-WNM (ATI Radeon 9100 IGP)
 - 3c980 Cyclone
 - 3c980C Python-T
 - 3cSOHO100-TX Hurricane
 - 3c555 Laptop Hurricane
 - 3c556 Laptop Tornado
 - 3c556B Laptop Hurricane
 - 3c575 [Megahertz] 10/100 LAN  CardBus
 - 3c575 Boomerang CardBus
 - 3CCFE575BT Cyclone CardBus
 - 3CCFE575CT Tornado CardBus
 - 3CCFE656 Cyclone CardBus
 - 3CCFEM656B Cyclone+Winmodem CardBus
 - 3CXFEM656C Tornado+Winmodem CardBus
 - 3c450 HomePNA Tornado
 - 3c920 Tornado
 - 3c982 Hydra Dual Port A
 - 3c982 Hydra Dual Port B
 - 3c905B-T4
 - 3c920B-EMB-WNM Tornado

## 模块参数


在加载模块时，可以向驱动提供若干参数。这些参数通常放置`/etc/modprobe.d/*.conf` 中
```
    options 3c59x debug=3 rx_copybreak=300

```

如果你使用的PCMCIA 工具（cardmgr），则参数可能如下：

```
    module "3c59x" opts "debug=3 rx_copybreak=300"

```

支持的参数如下：

debug=N

  其中 N 0 7 之间的数字。大3 的值会在系统日志中产生大量输出。默认值为 debug=1
options=N1,N2,N3,...

  列表中的每个数字为对应的网卡提供一项选项。因此，如果你有两块 3c905 并希望提
```
    options=0x204,0x204

```

  各个选项由若干位字段（bitfield）组成，其含义如下：

  可能的介质类型设
	==	=================================
	0	10baseT
	1	10Mbs AUI
	2	undefined
	3	10base2 (BNC)
	4	100base-TX
	5	100base-FX
	6	MII (Media Independent Interface)
	7	Use default setting from EEPROM
	8       Autonegotiate
	9       External MII
	10      Use default setting from EEPROM
	==	=================================

  在为 'options' 设置生成数值时，上述介质选择值可OR（或相加）上以下值：

  ======  =============================================
  0x8000  Set driver debugging level to 7
  0x4000  Set driver debugging level to 2
  0x0400  Enable Wake-on-LAN
  0x0200  Force full duplex mode.
  0x0010  Bus-master enable bit (Old Vortex cards only)
  ======  =============================================

  例如::

    insmod 3c59x options=0x204

  将强制使用全双工 100base-TX，而不是允许通常的自动协商
global_options=N

  为机器中所3c59x NIC 设置 `options` 参数。上`options` 数组中的条目将覆盖此设置
full_duplex=N1,N2,N3...

  类似'options' 的第 9 位。将对应网卡强制为全双工模式。请优先使用此参数而非 `options` 参数
  事实上，请尽量不要使用它！你最好让自动协商正常工作
global_full_duplex=N1

  为机器中所3c59x NIC 设置全双工模式。上`full_duplex` 数组中的条目将覆盖此设置
flow_ctrl=N1,N2,N3...

  使用 802.3x MAC 层流控com 网卡仅支PAUSE 命令，即如果收到来自链路对端PAUSE 帧，它们会停止发送数据包一小段时间
  驱动仅允许在全双工模式的链路上启用流控
  该功能在 3c905 上似乎不起作用——仅测试3c905B 3c905C
  3com 网卡似乎仅响应发送到保留目的地址 01:80:c2:00:00:01 PAUSE 帧。它们不响应发送到站点 MAC 地址PAUSE 帧
rx_copybreak=M

  驱动预分32 个全尺寸536 字节）网络缓冲区用于接收。当数据包到达时，驱动需要决定是将数据包留在全尺寸缓冲区中，还是分配一个较小的缓冲区并将数据包复制过去
  这是速度/空间之间的权衡
  rx_copybreak 的值用于决定何时进行复制。如果数据包大小小于 rx_copybreak，则复制该数据包。rx_copybreak 的默认值为 200 字节
max_interrupt_work=N

  驱动的中断服务例程在一次调用中可处理许多接收和发送数据包。它在一个循环中完成此操作。max_interrupt_work 的值控制中断服务例程循环的次数。默认值为 32 次循环。如果超过该值，中断服务例程将放弃并生成警告信息“eth0: Too much work in interrupt”
hw_checksums=N1,N2,N3,...

  较新3com NIC 能够在硬件中生成 IPv4、TCP UDP 校验和。Linux 早就使用Rx 校验和功能。“零拷贝”补丁计划用2.4 内核系列，它允许你同时使NIC DMA 分散/聚集（scatter/gather）和发送校验和
  驱动被设置为：在应用 zerocopy 补丁后，所Tornado Cyclone 设备将使S/G Tx 校验和
  提供此模块参数是为了让你能够覆盖该决定。如果你认为 Tx 校验和导致问题，可以使用 `hw_checksums=0` 禁用该功能
  如果你认为你NIC 应当执行 Tx 校验和而驱动未启用它，可以使用 `hw_checksums=1` 强制使用硬件 Tx 校验和
  驱动会在日志中记录一条信息，表明它是否正在使用硬件分聚集和硬Tx 校验和
  分散/聚集和硬件校验和sendfile() 系统调用带来显著的性能提升，但会使 send() 的吞吐量略有下降。对接收效率没有影响
compaq_ioaddr=N,
compaq_irq=N,
compaq_device_id=N

  “用于规Compaq PCI BIOS32 问题的变量”…
watchdog=N

  设置时间长度（以毫秒为单位），超过该时间后内核判定发送器已卡住并需要复位。这主要用于调试目的，尽管在冲突率非常高的局域网上增大该值可能有益。默认值为 5000.0 秒）
enable_wol=N1,N2,N3,...

  为相关接口启Wake-on-LAN 支持。Donald Becker `ether-wake` 应用程序可用于唤醒挂起的机器
  同时启用 NIC 的电源管理支持
global_enable_wol=N

  为机器中所3c59x NIC 设置 enable_wol 模式。上`enable_wol` 数组中的条目将覆盖此设置
### 介质选择


一些较旧的 NIC，如 3c590 3c900 系列，具10base2 AUI 接口
2001 1 月之前，如果10baseT 端口上未检测到活动，该驱动会自动选择 10base2 AUI 端口。随后它会卡10base2 端口上，必须重新加载驱动才能切回 10baseT。这种行为无法通过模块选项覆盖来阻止
较新（当前）版本的驱确实*支持锁定介质类型。因此，如果你使用以下命令加载驱动模块：

	modprobe 3c59x options=0

它将永久选择 10baseT 端口。不会自动选择其他介质类型

### 发送错误，Tx 状态寄存器 82


这是一个常见错误，几乎总是由同一网络上的另一台主机处于全双工模式、而本机处于半双工模式导致。你需要找到那台主机并使其运行在半双工模式，或者将本机修正为全双工模式
作为最后的手段，你可以使用以下命令3c59x 驱动强制为全双工模式
	options 3c59x full_duplex=1

但这应被视为针对损坏网络设备的变通办法，应仅用于无法自动协商的设备

### 附加资源


设备驱动实现细节位于源文件顶部
可在 Don Becker Linux 驱动站点获取额外文档
     http://www.scyld.com/vortex.html

Donald Becker 的驱动开发站点：

     http://www.scyld.com/network.html

Donald vortex-diag 程序可用于检NIC 状态：

     http://www.scyld.com/ethercard_diag.html

Donald mii-diag 程序可用于检查和操作 NIC 的介质无关接口（Media Independent Interface）子系统
     http://www.scyld.com/ethercard_diag.html#mii-diag

Donald wake-on-LAN 页面
     http://www.scyld.com/wakeonlan.html

3Com 用于设置 NIC EEPROM 的基DOS 的应用程序：

	ftp://ftp.3com.com/pub/nic/3c90x/3c90xx2.exe


### 自动协商说明


  驱动使用一分钟的心跳来适应外部局域网环境的变化：链路 UP 时为该值，链路 DOWN 时为 5 秒。这意味着，例如，当一台机器从集线10baseT 局域网拔下、插入交换式 100baseT 局域网时，吞吐量在长达六十秒内会相当糟糕。请耐心等待
  Walter Wong <wcw+@CMU.EDU> 提供Cisco 互操作性说明：

  附带说明，添HAS_NWAY 似乎Cisco 6509 交换机存在共同问题。具体来说，你需要将机器所插端口的生成树参数更改为 'portfast' 模式。否则协商会失败。这是我们注意了一段时间但一直没时间追查的问题
  Cisco 交换机（Jeff Busch <jbusch@deja.com>
```
	interface FastEthernet0/N
	description machinename
	load-interval 30
	spanning-tree portfast

```

    如果自动协商有问题，你可能还需要指"speed 100" "duplex full"（或 "speed 10" "duplex half"）
    WARNING: DO NOT hook up hubs/switches/bridges to these
    specially-configured ports! The switch will become very confused.


### 报告与诊断问

维护者发现，准确而完整的问题报告对于解决驱动问题非常宝贵。我们经常无法复现问题，必须依靠你的耐心和努力来查明问题根源
如果你认为遇到了驱动问题，应采取以下一些步骤：

- 这真的是驱动问题吗？

   排除一些变量：尝试不同的网卡、不同的计算机、不同的线缆、交换机/集线器上的不同端口、不同版本的内核或驱动等
- 好的，是驱动问题
   你需要生成一份报告。通常这是发送给维护者和/netdev@vger.kernel.org 的电子邮件。维护者的电子邮件地址可在驱动源码MAINTAINERS 文件中找到
- 报告的内容会因问题而有很大差异。如果是内核崩溃，则应参'Documentation/admin-guide/reporting-issues.rst'
  但对于大多数问题，提供以下内容很有用
   - 内核版本、驱动版
   - 驱动初始化时生成的横幅信息的副本。例如：

     eth0: 3Com PCI 3c905C Tornado at 0xa400,  00:50:da:6a:88:f0, IRQ 19
     8K byte-wide RAM 5:3 Rx:Tx split, autoselect/Autonegotiate interface.
     MII transceiver found at address 24, status 782d.
     Enabling bus-master transmits and whole-frame receives.

     注意：你必须提供 `debug=2` modprobe 选项才能生成

```
	modprobe 3c59x debug=2
```

   - 如果PCI 设备，提供来'lspci -vx' 的相关输出，例如
```
       00:09.0 Ethernet controller: 3Com Corporation 3c905C-TX [Fast Etherlink] (rev 74)
	       Subsystem: 3Com Corporation: Unknown device 9200
	       Flags: bus master, medium devsel, latency 32, IRQ 19
	       I/O ports at a400 [size=128]
	       Memory at db000000 (32-bit, non-prefetchable) [size=128]
	       Expansion ROM at <unassigned> [disabled] [size=128K]
	       Capabilities: [dc] Power Management version 2
       00: b7 10 00 92 07 00 10 02 74 00 00 02 08 20 00 00
       10: 01 a4 00 00 00 00 00 db 00 00 00 00 00 00 00 00
       20: 00 00 00 00 00 00 00 00 00 00 00 00 b7 10 00 10
       30: 00 00 00 00 dc 00 00 00 00 00 00 00 05 01 0a 0a
```

   - 环境描述0baseT00baseT？全/半双工？交换式还是集线式
   - 你可能向驱动提供的任何额外模块参数
   - 产生的任何内核日志。越多越好。如果这是一个大文件且你要将报告发送给邮件列表，请说明你有该日志文件，但不要发送它。如果你是直接向维护者报告，则直接发送即可
     为确保所有内核日志都可用，请将以下行添加/etc/syslog.conf
```
	 kern.* /var/log/messages
```

     然后重启 syslogd
```
	 /etc/rc.d/init.d/syslog restart
```

     （上述内容可能因你使用的 Linux 发行版而异）
    - 如果你的问题可复现，那就太好了。请尝试以下操作
      1) 提高调试级别。通常通过以下方式完成
	 a) modprobe driver debug=7
	 b) /etc/modprobe.d/driver.conf 中：
	    options driver debug=7

      2) 以更高的调试级别复现问题，将所有日志发送给维护者
      3) Donald Becker 的网<http://www.scyld.com/ethercard_diag.html> 下载你网卡的诊断工具。同时下mii-diag.c 并编译它们
	 a) 在网卡工作正常时运行 'vortex-diag -aaee' 'mii-diag -v'。保存输出
	 b) 在网卡出现故障时运行上述命令。发送两组输出
最后，请保持耐心并做好准备做一些工作。随着维护者提出更多问题、要求更多测试、要求应用补丁等，你最终可能会为此问题工作一周或更长时间。到头来，问题甚至可能仍然没有得到解决