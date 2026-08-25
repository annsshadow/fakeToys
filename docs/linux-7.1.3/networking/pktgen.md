
## Linux 数据包生成器（packet generator）使用指
启用 CONFIG_NET_PKTGEN 以编译并构建 pktgen，可以内置到内核或作为模块。推荐使用模块；如果需要则 modprobe pktgen。一旦运行，pktgen 会为每个 CPU 创建一个线程，并将亲和性绑定到CPU。监控和控制通过 /proc 完成。最简单的方法是选择一个合适的示例脚本并配置它
```
    ps aux | grep pkt
    root       129  0.3  0.0     0    0 ?        SW    2003 523:20 [kpktgend_0]
    root       130  0.3  0.0     0    0 ?        SW    2003 509:50 [kpktgend_1]

```
```
	/proc/net/pktgen/pgctrl
	/proc/net/pktgen/kpktgend_X
	/proc/net/pktgen/ethX

```
## 为最大性能调优 NIC

默认NIC 设置（可能）并未针对 pktgen 这种人为过载类型的基准测试进行调优，因为这会损害正常使用场景
```
 # ethtool -G ethX tx 1024

```
更大TX 环可以提pktgen 的性能，但在一般情况下它会有害）因TX 环缓冲区可能变得CPU L1/L2 缓存更大）因为它允许NIC 硬件层有更多的排队（这对缓冲区膨胀 bufferbloat 不利）
人们不应匆忙得出 HW TX 环中的数据包/描述符会造成延迟的结论。驱动通常出于各种性能原因而延迟清理环缓冲区，停滞TX 环中的数据包可能只是在等待清理
这个清理问题特别适用于驱ixgbe（Intel 82599 芯片）。该驱动（ixgbe）将 TX+RX 环清理合并，而清理间隔受 ethtool --coalesce 设置中的参数 "rx-usecs" 影响
```
 # ethtool -C ethX rx-usecs 30

```
## 内核线程

Pktgen 为每CPU 创建一个线程，并将亲和性绑定到CPU。这可以通过 proc 文件 /proc/net/pktgen/kpktgend_X 控制
```
 Running:
 Stopped: eth4@0
 Result: OK: add_device=eth4@0

```
最重要的是分配给线程的设备
两个基本的线程命令是
 - add_device DEVICE@NAME -- 添加一个单一设备
 - rem_device_all         -- 移除所有关联的设备

当向线程添加设备时，会创建一个相应的 proc 文件，用于配置该设备。因此，设备名称需要唯一
为了支持将同一设备添加到多个线程（这对多队NIC 很有用），设备命名方案用 "@" 进行了扩展：device@something

"@" 之后的部分可以是任意内容，但通常习惯使用线程号
## 查看设备

Params 部分保存配置信息。Current 部分保存运行统计信息。Result 在一次运行后或在之后打印
```
    /proc/net/pktgen/eth4@0

    Params: count 100000  min_pkt_size: 60  max_pkt_size: 60
	frags: 0  delay: 0  clone_skb: 64  ifname: eth4@0
	flows: 0 flowlen: 0
	queue_map_min: 0  queue_map_max: 0
	dst_min: 192.168.81.2  dst_max:
	src_min:   src_max:
	src_mac: 90:e2:ba:0a:56:b4 dst_mac: 00:1b:21:3c:9d:f8
	udp_src_min: 9  udp_src_max: 109  udp_dst_min: 9  udp_dst_max: 9
	src_mac_count: 0  dst_mac_count: 0
	Flags: UDPSRC_RND  NO_TIMESTAMP  QUEUE_MAP_CPU
    Current:
	pkts-sofar: 100000  errors: 0
	started: 623913381008us  stopped: 623913396439us idle: 25us
	seq_num: 100001  cur_dst_mac_offset: 0  cur_src_mac_offset: 0
	cur_saddr: 192.168.8.3  cur_daddr: 192.168.81.2
	cur_udp_dst: 9  cur_udp_src: 42
	cur_queue_map: 0
	flows: 0
    Result: OK: 15430(c15405+d25) usec, 100000 (60byte,0frags)
    6480562pps 3110Mb/sec (3110669760bps) errors: 0

```
## 配置设备

这是通过 /proc 接口完成的，并且最容易通过示例脚本中定义的 pgset 来完成。你需要指PGDEV 环境变量来使用示例脚本中的函数
```
    export PGDEV=/proc/net/pktgen/eth4@0
    source samples/pktgen/functions.sh

```
```
 pg_ctrl start           starts injection.
 pg_ctrl stop            aborts injection. Also, ^C aborts generator.

 pgset "clone_skb 1"     sets the number of copies of the same packet
 pgset "clone_skb 0"     use single SKB for all transmits
 pgset "burst 8"         uses xmit_more API to queue 8 copies of the same
			 packet and update HW tx queue tail pointer once.
			 "burst 1" is the default
 pgset "pkt_size 9014"   sets packet size to 9014
 pgset "frags 5"         packet will consist of 5 fragments
 pgset "count 200000"    sets number of packets to send, set to zero
			 for continuous sends until explicitly stopped.

 pgset "delay 5000"      adds delay to hard_start_xmit(). nanoseconds

 pgset "dst 10.0.0.1"    sets IP destination address
			 (BEWARE! This generator is very aggressive!)

 pgset "dst_min 10.0.0.1"            Same as dst
 pgset "dst_max 10.0.0.254"          Set the maximum destination IP.
 pgset "src_min 10.0.0.1"            Set the minimum (or only) source IP.
 pgset "src_max 10.0.0.254"          Set the maximum source IP.
 pgset "dst6 fec0::1"     IPV6 destination address
 pgset "src6 fec0::2"     IPV6 source address
 pgset "dstmac 00:00:00:00:00:00"    sets MAC destination address
 pgset "srcmac 00:00:00:00:00:00"    sets MAC source address

 pgset "queue_map_min 0" Sets the min value of tx queue interval
 pgset "queue_map_max 7" Sets the max value of tx queue interval, for multiqueue devices
			 To select queue 1 of a given device,
			 use queue_map_min=1 and queue_map_max=1

 pgset "src_mac_count 1" Sets the number of MACs we'll range through.
			 The 'minimum' MAC is what you set with srcmac.

 pgset "dst_mac_count 1" Sets the number of MACs we'll range through.
			 The 'minimum' MAC is what you set with dstmac.

 pgset "flag [name]"     Set a flag to determine behaviour.  Current flags
			 are: IPSRC_RND # IP source is random (between min/max)
			      IPDST_RND # IP destination is random
			      UDPSRC_RND, UDPDST_RND,
			      MACSRC_RND, MACDST_RND
			      TXSIZE_RND, IPV6,
			      MPLS_RND, VID_RND, SVID_RND
			      FLOW_SEQ,
			      QUEUE_MAP_RND # queue map random
			      QUEUE_MAP_CPU # queue map mirrors smp_processor_id()
			      UDPCSUM,
			      IPSEC # IPsec encapsulation (needs CONFIG_XFRM)
			      NODE_ALLOC # node specific memory allocation
			      NO_TIMESTAMP # disable timestamping
			      SHARED # enable shared SKB
 pgset 'flag ![name]'    Clear a flag to determine behaviour.
			 Note that you might need to use single quote in
			 interactive mode, so that your shell wouldn't expand
			 the specified flag as a history command.

 pgset "spi [SPI_VALUE]" Set specific SA used to transform packet.

 pgset "udp_src_min 9"   set UDP source port min, If < udp_src_max, then
			 cycle through the port range.

 pgset "udp_src_max 9"   set UDP source port max.
 pgset "udp_dst_min 9"   set UDP destination port min, If < udp_dst_max, then
			 cycle through the port range.
 pgset "udp_dst_max 9"   set UDP destination port max.

 pgset "mpls 0001000a,0002000a,0000000a" set MPLS labels (in this example
					 outer label=16,middle label=32,
					 inner label=0 (IPv4 NULL)) Note that
					 there must be no spaces between the
					 arguments. Leading zeros are required.
					 Do not set the bottom of stack bit,
					 that's done automatically. If you do
					 set the bottom of stack bit, that
					 indicates that you want to randomly
					 generate that address and the flag
					 MPLS_RND will be turned on. You
					 can have any mix of random and fixed
					 labels in the label stack.

 pgset "mpls 0"		  turn off mpls (or any invalid argument works too!)

 pgset "vlan_id 77"       set VLAN ID 0-4095
 pgset "vlan_p 3"         set priority bit 0-7 (default 0)
 pgset "vlan_cfi 0"       set canonical format identifier 0-1 (default 0)

 pgset "svlan_id 22"      set SVLAN ID 0-4095
 pgset "svlan_p 3"        set priority bit 0-7 (default 0)
 pgset "svlan_cfi 0"      set canonical format identifier 0-1 (default 0)

 pgset "vlan_id 9999"     > 4095 remove vlan and svlan tags
 pgset "svlan 9999"       > 4095 remove svlan tag


 pgset "tos XX"           set former IPv4 TOS field (e.g. "tos 28" for AF11 no ECN, default 00)
 pgset "traffic_class XX" set former IPv6 TRAFFIC CLASS (e.g. "traffic_class B8" for EF no ECN, default 00)

 pgset "rate 300M"        set rate to 300 Mb/s
 pgset "ratep 1000000"    set rate to 1Mpps

 pgset "xmit_mode netif_receive"  RX inject into stack netif_receive_skb()
				  Works with "burst" but not with "clone_skb".
				  Default xmit_mode is "start_xmit".

```
## 示例脚本

samples/pktgen 目录中包含一pktgen 的教程脚本和辅助工具。辅助文parameters.sh 支持在各示例脚本之间进行简单且一致的参数解析
```
 ./pktgen_sample01_simple.sh -i eth4 -m 00:1B:21:3C:9D:F8 -d 192.168.8.2

```
```
  ./pktgen_sample01_simple.sh [-vx] -i ethX

  -i : ($DEV)       output interface/device (required)
  -s : ($PKT_SIZE)  packet size
  -d : ($DEST_IP)   destination IP. CIDR (e.g. 198.18.0.0/15) is also allowed
  -m : ($DST_MAC)   destination MAC-addr
  -p : ($DST_PORT)  destination PORT range (e.g. 433-444) is also allowed
  -t : ($THREADS)   threads to start
  -f : ($F_THREAD)  index of first thread (zero indexed CPU number)
  -c : ($SKB_CLONE) SKB clones send before alloc new SKB
  -n : ($COUNT)     num messages to send per thread, 0 means indefinitely
  -b : ($BURST)     HW level bursting of SKBs
  -v : ($VERBOSE)   verbose
  -x : ($DEBUG)     debug
  -6 : ($IP6)       IPv6
  -w : ($DELAY)     Tx Delay value (ns)
  -a : ($APPEND)    Script will not reset generator's state, but will append its config

```
所列出的全局变量也在其中。例如，必需的接设备参数 "-i" 设置了变$DEV。复pktgen_sampleXX 脚本并修改它们以适应你自己的需要

## 中断亲和
注意，当向特CPU 添加设备时，同时分配 /proc/irq/XX/smp_affinity 以将 TX 中断绑定到同一 CPU 是个好主意。这减少了释skb 时的缓存抖动（cache bouncing）
此外使用设备标志 QUEUE_MAP_CPU，它SKB TX 队列映射到运行线程的 CPU（直接来smp_processor_id()）
## 启用 IPsec

默认IPsec 转换使用 ESP 封装加传输模
```
    pgset "flag IPSEC"
    pgset "flows 1"

```
为了避免破坏现有的用AH 类型和隧道模式的测试床脚本，你可以使"pgset spi SPI_VALUE" 来指定要采用的转换模式
## 禁用共享 SKB

默认情况下，pktgen 发送的 SKB 是共享的（用户计> 1）
```
	pg_set "flag !SHARED"

```
然而，如果配置"clone_skb" "burst" 参数，skb 仍需要被 pktgen 持有以便进一步访问。因此该 skb 必须是共享的
## 当前命令与配置选项

```
    start
    stop
    reset

```
```
    add_device
    rem_device_all

```
```
    count
    clone_skb
    burst
    debug

    frags
    delay

    src_mac_count
    dst_mac_count

    pkt_size
    min_pkt_size
    max_pkt_size

    queue_map_min
    queue_map_max
    skb_priority

    tos           (ipv4)
    traffic_class (ipv6)

    mpls

    udp_src_min
    udp_src_max

    udp_dst_min
    udp_dst_max

    node

    flag
    IPSRC_RND
    IPDST_RND
    UDPSRC_RND
    UDPDST_RND
    MACSRC_RND
    MACDST_RND
    TXSIZE_RND
    IPV6
    MPLS_RND
    VID_RND
    SVID_RND
    FLOW_SEQ
    QUEUE_MAP_RND
    QUEUE_MAP_CPU
    UDPCSUM
    IPSEC
    NODE_ALLOC
    NO_TIMESTAMP
    SHARED

    spi (ipsec)

    dst_min
    dst_max

    src_min
    src_max

    dst_mac
    src_mac

    clear_counters

    src6
    dst6
    dst6_max
    dst6_min

    flows
    flowlen

    rate
    ratep

    xmit_mode <start_xmit|netif_receive>

    vlan_cfi
    vlan_id
    vlan_p

    svlan_cfi
    svlan_id
    svlan_p

```
参考文献：

- ftp://robur.slu.se/pub/Linux/net-development/pktgen-testing/
- ftp://robur.slu.se/pub/Linux/net-development/pktgen-testing/examples/

Linux-Kongress in Erlangen 2004 的论文- ftp://robur.slu.se/pub/Linux/net-development/pktgen-testing/pktgen_paper.pdf

感谢
Grant Grundler IA-64 parisc 上的测试，Harald Welte、Lennert Buytenhek、Stephen Hemminger、Andi Kleen、Dave Miller 以及许多其他人

Linux 网络开发顺利