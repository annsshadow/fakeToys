
## Texas Instruments CPSW 以太网驱动


## 多队列 & CBS & MQPRIO


cpsw 为每个外部端口提供 3 个 CBS 整形器。本文档基于示例描述了 cpsw 驱动的
MQPRIO 和 CBS Qdisc 卸载配置。它有可能用于音视频桥接（AVB）和时间敏感网络（TSN）。

以下示例在 AM572x EVM 和 BBB 板上进行了测试。

## 测试设置


考虑两个在 dual_emac 模式下运行 cpsw 驱动的 AM572x EVM 示例。

若干前提条件：

- TX 队列必须从具有最高优先级的 txq0 开始定速
- 流量类从 0 开始使用，0 具有最高优先级
- CBS 整形器应与已定速的队列一起使用
- CBS 整形器的带宽必须设置得比潜在的进入速率略大一些，因此所有进入 tx 队列的速率
  必须略小一些
- 由于离散性，实际速率可能会有所不同
- 仅将 skb 优先级映射到 txq 是不够的，还必须用 ip 或 vconfig 工具创建 skb 优先级
  到 l2 优先级的映射
- 类的任何 l2/套接字优先级（0 - 7）都可以使用，但为简单起见使用默认值：3 和 2
- 仅测试了 2 个类：A 和 B，但已检查并可以与更多类一起工作，最多允许 4 个，但只能为
  其中 3 个设置速率

## 示例的测试设置


```

					+-------------------------------+
					|--+                            |
					|  |      Workstation0          |
					|E |  MAC 18:03:73:66:87:42     |
    +-----------------------------+  +--|t |                            |
    |                    | 1  | E |  |  |h |./tsn_listener -d \         |
    |  Target board:     | 0  | t |--+  |0 | 18:03:73:66:87:42 -i eth0 \|
    |  AM572x EVM        | 0  | h |     |  | -s 1500                    |
    |                    | 0  | 0 |     |--+                            |
    |  Only 2 classes:   |Mb  +---|     +-------------------------------+
    |  class A, class B  |        |
    |                    |    +---|     +-------------------------------+
    |                    | 1  | E |     |--+                            |
    |                    | 0  | t |     |  |      Workstation1          |
    |                    | 0  | h |--+  |E |  MAC 20:cf:30:85:7d:fd     |
    |                    |Mb  | 1 |  +--|t |                            |
    +-----------------------------+     |h |./tsn_listener -d \         |
					|0 | 20:cf:30:85:7d:fd -i eth0 \|
					|  | -s 1500                    |
					|--+                            |
					+-------------------------------+



```
### 示例 1：目标板单端口 tx AVB 配置方案


（AM572x evm 的打印和方案，适用于单端口板）

- tc - 流量类
- txq - 发送队列
- p - 优先级
- f - fifo（cpsw fifo）
- S - 已配置整形器

```

    +------------------------------------------------------------------+ u
    | +---------------+  +---------------+  +------+ +------+          | s
    | |               |  |               |  |      | |      |          | e
    | | App 1         |  | App 2         |  | Apps | | Apps |          | r
    | | Class A       |  | Class B       |  | Rest | | Rest |          |
    | | Eth0          |  | Eth0          |  |   |  | |   |  |          | s
    | | VLAN100       |  | VLAN100       |  |   |  | |   |  |          | p
    | | 40 Mb/s       |  | 20 Mb/s       |  |   |  | |   |  |          | a
    | | SO_PRIORITY=3 |  | SO_PRIORITY=2 |  |   |  | |   |  |          | c
    | |   |           |  |   |           |  |   |  | |   |  |          | e
    | +---|-----------+  +---|-----------+  +---|--+ +---|--+          |
    +-----|------------------|------------------|--------|-------------+
	+-+     +------------+                  |        |
	|       |             +-----------------+     +--+
	|       |             |                       |
    +---|-------|-------------|-----------------------|----------------+
    | +----+ +----+ +----+ +----+                   +----+             |
    | | p3 | | p2 | | p1 | | p0 |                   | p0 |             | k
    | \    / \    / \    / \    /                   \    /             | e
    |  \  /   \  /   \  /   \  /                     \  /              | r
    |   \/     \/     \/     \/                       \/               | n
    |    |     |             |                        |                | e
    |    |     |       +-----+                        |                | l
    |    |     |       |                              |                |
    | +----+ +----+ +----+                          +----+             | s
    | |tc0 | |tc1 | |tc2 |                          |tc0 |             | p
    | \    / \    / \    /                          \    /             | a
    |  \  /   \  /   \  /                            \  /              | c
    |   \/     \/     \/                              \/               | e
    |   |      |       +-----+                        |                |
    |   |      |       |     |                        |                |
    |   |      |       |     |                        |                |
    |   |      |       |     |                        |                |
    | +----+ +----+ +----+ +----+                   +----+             |
    | |txq0| |txq1| |txq2| |txq3|                   |txq4|             |
    | \    / \    / \    / \    /                   \    /             |
    |  \  /   \  /   \  /   \  /                     \  /              |
    |   \/     \/     \/     \/                       \/               |
    | +-|------|------|------|--+                  +--|--------------+ |
    | | |      |      |      |  | Eth0.100         |  |     Eth1     | |
    +---|------|------|------|------------------------|----------------+
	|      |      |      |                        |
	p      p      p      p                        |
	3      2      0-1, 4-7  <- L2 priority        |
	|      |      |      |                        |
	|      |      |      |                        |
    +---|------|------|------|------------------------|----------------+
    |   |      |      |      |             |----------+                |
    | +----+ +----+ +----+ +----+       +----+                         |
    | |dma7| |dma6| |dma5| |dma4|       |dma3|                         |
    | \    / \    / \    / \    /       \    /                         | c
    |  \S /   \S /   \  /   \  /         \  /                          | p
    |   \/     \/     \/     \/           \/                           | s
    |   |      |      | +-----            |                            | w
    |   |      |      | |                 |                            |
    |   |      |      | |                 |                            | d
    | +----+ +----+ +----+p            p+----+                         | r
    | |    | |    | |    |o            o|    |                         | i
    | | f3 | | f2 | | f0 |r            r| f0 |                         | v
    | |tc0 | |tc1 | |tc2 |t            t|tc0 |                         | e
    | \CBS / \CBS / \CBS /1            2\CBS /                         | r
    |  \S /   \S /   \  /                \  /                          |
    |   \/     \/     \/                  \/                           |
    +------------------------------------------------------------------+



```
```

	// Add 4 tx queues, for interface Eth0, and 1 tx queue for Eth1
	$ ethtool -L eth0 rx 1 tx 5
	rx unmodified, ignoring

```
```

	// Check if num of queues is set correctly:
	$ ethtool -l eth0
	Channel parameters for eth0:
	Pre-set maximums:
	RX:             8
	TX:             8
	Other:          0
	Combined:       0
	Current hardware settings:
	RX:             1
	TX:             5
	Other:          0
	Combined:       0

```
```

	// TX queues must be rated starting from 0, so set bws for tx0 and tx1
	// Set rates 40 and 20 Mb/s appropriately.
	// Pay attention, real speed can differ a bit due to discreetness.
	// Leave last 2 tx queues not rated.
	$ echo 40 > /sys/class/net/eth0/queues/tx-0/tx_maxrate
	$ echo 20 > /sys/class/net/eth0/queues/tx-1/tx_maxrate

```
```

	// Check maximum rate of tx (cpdma) queues:
	$ cat /sys/class/net/eth0/queues/tx-*/tx_maxrate
	40
	20
	0
	0
	0

```
```

	// Map skb->priority to traffic class:
	// 3pri -> tc0, 2pri -> tc1, (0,1,4-7)pri -> tc2
	// Map traffic class to transmit queue:
	// tc0 -> txq0, tc1 -> txq1, tc2 -> (txq2, txq3)
	$ tc qdisc replace dev eth0 handle 100: parent root mqprio num_tc 3 \
	map 2 2 1 0 2 2 2 2 2 2 2 2 2 2 2 2 queues 1@0 1@1 2@2 hw 1

```
```

	// As two interface sharing same set of tx queues, assign all traffic
	// coming to interface Eth1 to separate queue in order to not mix it
	// with traffic from interface Eth0, so use separate txq to send
	// packets to Eth1, so all prio -> tc0 and tc0 -> txq4
	// Here hw 0, so here still default configuration for eth1 in hw
	$ tc qdisc replace dev eth1 handle 100: parent root mqprio num_tc 1 \
	map 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 queues 1@4 hw 0

```
```

	// Check classes settings
	$ tc -g class show dev eth0
	+---(100:ffe2) mqprio
	|    +---(100:3) mqprio
	|    +---(100:4) mqprio
	|
	+---(100:ffe1) mqprio
	|    +---(100:2) mqprio
	|
	+---(100:ffe0) mqprio
	    +---(100:1) mqprio

	$ tc -g class show dev eth1
	+---(100:ffe0) mqprio
	    +---(100:5) mqprio

```
```

	// Set rate for class A - 41 Mbit (tc0, txq0) using CBS Qdisc
	// Set it +1 Mb for reserve (important!)
	// here only idle slope is important, others arg are ignored
	// Pay attention, real speed can differ a bit due to discreetness
	$ tc qdisc add dev eth0 parent 100:1 cbs locredit -1438 \
	hicredit 62 sendslope -959000 idleslope 41000 offload 1
	net eth0: set FIFO3 bw = 50

```
```

	// Set rate for class B - 21 Mbit (tc1, txq1) using CBS Qdisc:
	// Set it +1 Mb for reserve (important!)
	$ tc qdisc add dev eth0 parent 100:2 cbs locredit -1468 \
	hicredit 65 sendslope -979000 idleslope 21000 offload 1
	net eth0: set FIFO2 bw = 30

```
```

	// Create vlan 100 to map sk->priority to vlan qos
	$ ip link add link eth0 name eth0.100 type vlan id 100
	8021q: 802.1Q VLAN Support v1.8
	8021q: adding VLAN 0 to HW filter on device eth0
	8021q: adding VLAN 0 to HW filter on device eth1
	net eth0: Adding vlanid 100 to vlan filter

```
```

	// Map skb->priority to L2 prio, 1 to 1
	$ ip link set eth0.100 type vlan \
	egress 0:0 1:1 2:2 3:3 4:4 5:5 6:6 7:7

```
```

	// Check egress map for vlan 100
	$ cat /proc/net/vlan/eth0.100
	[...]
	INGRESS priority mappings: 0:0  1:0  2:0  3:0  4:0  5:0  6:0 7:0
	EGRESS priority mappings: 0:0 1:1 2:2 3:3 4:4 5:5 6:6 7:7

```
```

	// Run your appropriate tools with socket option "SO_PRIORITY"
	// to 3 for class A and/or to 2 for class B
	// (I took at https://lore.kernel.org/r/20171017010128.22141-1-vinicius.gomes@intel.com/)
	./tsn_talker -d 18:03:73:66:87:42 -i eth0.100 -p3 -s 1500&
	./tsn_talker -d 18:03:73:66:87:42 -i eth0.100 -p2 -s 1500&

```
```

	// run your listener on workstation (should be in same vlan)
	// (I took at https://lore.kernel.org/r/20171017010128.22141-1-vinicius.gomes@intel.com/)
	./tsn_listener -d 18:03:73:66:87:42 -i enp5s0 -s 1500
	Receiving data rate: 39012 kbps
	Receiving data rate: 39012 kbps
	Receiving data rate: 39012 kbps
	Receiving data rate: 39012 kbps
	Receiving data rate: 39012 kbps
	Receiving data rate: 39012 kbps
	Receiving data rate: 39012 kbps
	Receiving data rate: 39012 kbps
	Receiving data rate: 39012 kbps
	Receiving data rate: 39012 kbps
	Receiving data rate: 39012 kbps
	Receiving data rate: 39000 kbps

```
```

	// Restore default configuration if needed
	$ ip link del eth0.100
	$ tc qdisc del dev eth1 root
	$ tc qdisc del dev eth0 root
	net eth0: Prev FIFO2 is shaped
	net eth0: set FIFO3 bw = 0
	net eth0: set FIFO2 bw = 0
	$ ethtool -L eth0 rx 1 tx 1

```
### 示例 2：目标板双端口 tx AVB 配置方案


（AM572x evm 的打印和方案，仅适用于 dual emac 板）

```

    +------------------------------------------------------------------+ u
    | +----------+  +----------+  +------+  +----------+  +----------+ | s
    | |          |  |          |  |      |  |          |  |          | | e
    | | App 1    |  | App 2    |  | Apps |  | App 3    |  | App 4    | | r
    | | Class A  |  | Class B  |  | Rest |  | Class B  |  | Class A  | |
    | | Eth0     |  | Eth0     |  |   |  |  | Eth1     |  | Eth1     | | s
    | | VLAN100  |  | VLAN100  |  |   |  |  | VLAN100  |  | VLAN100  | | p
    | | 40 Mb/s  |  | 20 Mb/s  |  |   |  |  | 10 Mb/s  |  | 30 Mb/s  | | a
    | | SO_PRI=3 |  | SO_PRI=2 |  |   |  |  | SO_PRI=3 |  | SO_PRI=2 | | c
    | |   |      |  |   |      |  |   |  |  |   |      |  |   |      | | e
    | +---|------+  +---|------+  +---|--+  +---|------+  +---|------+ |
    +-----|-------------|-------------|---------|-------------|--------+
	+-+     +-------+             |         +----------+  +----+
	|       |             +-------+------+             |       |
	|       |             |              |             |       |
    +---|-------|-------------|--------------|-------------|-------|---+
    | +----+ +----+ +----+ +----+          +----+ +----+ +----+ +----+ |
    | | p3 | | p2 | | p1 | | p0 |          | p0 | | p1 | | p2 | | p3 | | k
    | \    / \    / \    / \    /          \    / \    / \    / \    / | e
    |  \  /   \  /   \  /   \  /            \  /   \  /   \  /   \  /  | r
    |   \/     \/     \/     \/              \/     \/     \/     \/   | n
    |   |      |             |                |             |      |   | e
    |   |      |        +----+                +----+        |      |   | l
    |   |      |        |                          |        |      |   |
    | +----+ +----+ +----+                        +----+ +----+ +----+ | s
    | |tc0 | |tc1 | |tc2 |                        |tc2 | |tc1 | |tc0 | | p
    | \    / \    / \    /                        \    / \    / \    / | a
    |  \  /   \  /   \  /                          \  /   \  /   \  /  | c
    |   \/     \/     \/                            \/     \/     \/   | e
    |   |      |       +-----+                +-----+      |       |   |
    |   |      |       |     |                |     |      |       |   |
    |   |      |       |     |                |     |      |       |   |
    |   |      |       |     |    E      E    |     |      |       |   |
    | +----+ +----+ +----+ +----+ t      t +----+ +----+ +----+ +----+ |
    | |txq0| |txq1| |txq4| |txq5| h      h |txq6| |txq7| |txq3| |txq2| |
    | \    / \    / \    / \    / 0      1 \    / \    / \    / \    / |
    |  \  /   \  /   \  /   \  /  .      .  \  /   \  /   \  /   \  /  |
    |   \/     \/     \/     \/   1      1   \/     \/     \/     \/   |
    | +-|------|------|------|--+ 0      0 +-|------|------|------|--+ |
    | | |      |      |      |  | 0      0 | |      |      |      |  | |
    +---|------|------|------|---------------|------|------|------|----+
	|      |      |      |               |      |      |      |
	p      p      p      p               p      p      p      p
	3      2      0-1, 4-7   <-L2 pri->  0-1, 4-7      2      3
	|      |      |      |               |      |      |      |
	|      |      |      |               |      |      |      |
    +---|------|------|------|---------------|------|------|------|----+
    |   |      |      |      |               |      |      |      |    |
    | +----+ +----+ +----+ +----+          +----+ +----+ +----+ +----+ |
    | |dma7| |dma6| |dma3| |dma2|          |dma1| |dma0| |dma4| |dma5| |
    | \    / \    / \    / \    /          \    / \    / \    / \    / | c
    |  \S /   \S /   \  /   \  /            \  /   \  /   \S /   \S /  | p
    |   \/     \/     \/     \/              \/     \/     \/     \/   | s
    |   |      |      | +-----                |      |      |      |   | w
    |   |      |      | |                     +----+ |      |      |   |
    |   |      |      | |                          | |      |      |   | d
    | +----+ +----+ +----+p                      p+----+ +----+ +----+ | r
    | |    | |    | |    |o                      o|    | |    | |    | | i
    | | f3 | | f2 | | f0 |r        CPSW          r| f3 | | f2 | | f0 | | v
    | |tc0 | |tc1 | |tc2 |t                      t|tc0 | |tc1 | |tc2 | | e
    | \CBS / \CBS / \CBS /1                      2\CBS / \CBS / \CBS / | r
    |  \S /   \S /   \  /                          \S /   \S /   \  /  |
    |   \/     \/     \/                            \/     \/     \/   |
    +------------------------------------------------------------------+
    ========================================Eth==========================>

```
```

	// Add 8 tx queues, for interface Eth0, but they are common, so are accessed
	// by two interfaces Eth0 and Eth1.
	$ ethtool -L eth1 rx 1 tx 8
	rx unmodified, ignoring

```
```

	// Check if num of queues is set correctly:
	$ ethtool -l eth0
	Channel parameters for eth0:
	Pre-set maximums:
	RX:             8
	TX:             8
	Other:          0
	Combined:       0
	Current hardware settings:
	RX:             1
	TX:             8
	Other:          0
	Combined:       0

```
```

	// TX queues must be rated starting from 0, so set bws for tx0 and tx1 for Eth0
	// and for tx2 and tx3 for Eth1. That is, rates 40 and 20 Mb/s appropriately
	// for Eth0 and 30 and 10 Mb/s for Eth1.
	// Real speed can differ a bit due to discreetness
	// Leave last 4 tx queues as not rated
	$ echo 40 > /sys/class/net/eth0/queues/tx-0/tx_maxrate
	$ echo 20 > /sys/class/net/eth0/queues/tx-1/tx_maxrate
	$ echo 30 > /sys/class/net/eth1/queues/tx-2/tx_maxrate
	$ echo 10 > /sys/class/net/eth1/queues/tx-3/tx_maxrate

```
```

	// Check maximum rate of tx (cpdma) queues:
	$ cat /sys/class/net/eth0/queues/tx-*/tx_maxrate
	40
	20
	30
	10
	0
	0
	0
	0

```
```

	// Map skb->priority to traffic class for Eth0:
	// 3pri -> tc0, 2pri -> tc1, (0,1,4-7)pri -> tc2
	// Map traffic class to transmit queue:
	// tc0 -> txq0, tc1 -> txq1, tc2 -> (txq4, txq5)
	$ tc qdisc replace dev eth0 handle 100: parent root mqprio num_tc 3 \
	map 2 2 1 0 2 2 2 2 2 2 2 2 2 2 2 2 queues 1@0 1@1 2@4 hw 1

```
```

	// Check classes settings
	$ tc -g class show dev eth0
	+---(100:ffe2) mqprio
	|    +---(100:5) mqprio
	|    +---(100:6) mqprio
	|
	+---(100:ffe1) mqprio
	|    +---(100:2) mqprio
	|
	+---(100:ffe0) mqprio
	    +---(100:1) mqprio

```
```

	// Set rate for class A - 41 Mbit (tc0, txq0) using CBS Qdisc for Eth0
	// here only idle slope is important, others ignored
	// Real speed can differ a bit due to discreetness
	$ tc qdisc add dev eth0 parent 100:1 cbs locredit -1470 \
	hicredit 62 sendslope -959000 idleslope 41000 offload 1
	net eth0: set FIFO3 bw = 50

```
```

	// Set rate for class B - 21 Mbit (tc1, txq1) using CBS Qdisc for Eth0
	$ tc qdisc add dev eth0 parent 100:2 cbs locredit -1470 \
	hicredit 65 sendslope -979000 idleslope 21000 offload 1
	net eth0: set FIFO2 bw = 30

```
```

	// Create vlan 100 to map sk->priority to vlan qos for Eth0
	$ ip link add link eth0 name eth0.100 type vlan id 100
	net eth0: Adding vlanid 100 to vlan filter

```
```

	// Map skb->priority to L2 prio for Eth0.100, one to one
	$ ip link set eth0.100 type vlan \
	egress 0:0 1:1 2:2 3:3 4:4 5:5 6:6 7:7

```
```

	// Check egress map for vlan 100
	$ cat /proc/net/vlan/eth0.100
	[...]
	INGRESS priority mappings: 0:0  1:0  2:0  3:0  4:0  5:0  6:0 7:0
	EGRESS priority mappings: 0:0 1:1 2:2 3:3 4:4 5:5 6:6 7:7

```
```

	// Map skb->priority to traffic class for Eth1:
	// 3pri -> tc0, 2pri -> tc1, (0,1,4-7)pri -> tc2
	// Map traffic class to transmit queue:
	// tc0 -> txq2, tc1 -> txq3, tc2 -> (txq6, txq7)
	$ tc qdisc replace dev eth1 handle 100: parent root mqprio num_tc 3 \
	map 2 2 1 0 2 2 2 2 2 2 2 2 2 2 2 2 queues 1@2 1@3 2@6 hw 1

```
```

	// Check classes settings
	$ tc -g class show dev eth1
	+---(100:ffe2) mqprio
	|    +---(100:7) mqprio
	|    +---(100:8) mqprio
	|
	+---(100:ffe1) mqprio
	|    +---(100:4) mqprio
	|
	+---(100:ffe0) mqprio
	    +---(100:3) mqprio

```
```

	// Set rate for class A - 31 Mbit (tc0, txq2) using CBS Qdisc for Eth1
	// here only idle slope is important, others ignored, but calculated
	// for interface speed - 100Mb for eth1 port.
	// Set it +1 Mb for reserve (important!)
	$ tc qdisc add dev eth1 parent 100:3 cbs locredit -1035 \
	hicredit 465 sendslope -69000 idleslope 31000 offload 1
	net eth1: set FIFO3 bw = 31

```
```

	// Set rate for class B - 11 Mbit (tc1, txq3) using CBS Qdisc for Eth1
	// Set it +1 Mb for reserve (important!)
	$ tc qdisc add dev eth1 parent 100:4 cbs locredit -1335 \
	hicredit 405 sendslope -89000 idleslope 11000 offload 1
	net eth1: set FIFO2 bw = 11

```
```

	// Create vlan 100 to map sk->priority to vlan qos for Eth1
	$ ip link add link eth1 name eth1.100 type vlan id 100
	net eth1: Adding vlanid 100 to vlan filter

```
```

	// Map skb->priority to L2 prio for Eth1.100, one to one
	$ ip link set eth1.100 type vlan \
	egress 0:0 1:1 2:2 3:3 4:4 5:5 6:6 7:7

```
```

	// Check egress map for vlan 100
	$ cat /proc/net/vlan/eth1.100
	[...]
	INGRESS priority mappings: 0:0  1:0  2:0  3:0  4:0  5:0  6:0 7:0
	EGRESS priority mappings: 0:0 1:1 2:2 3:3 4:4 5:5 6:6 7:7

```
```

	// Run appropriate tools with socket option "SO_PRIORITY" to 3
	// for class A and to 2 for class B. For both interfaces
	./tsn_talker -d 18:03:73:66:87:42 -i eth0.100 -p2 -s 1500&
	./tsn_talker -d 18:03:73:66:87:42 -i eth0.100 -p3 -s 1500&
	./tsn_talker -d 20:cf:30:85:7d:fd -i eth1.100 -p2 -s 1500&
	./tsn_talker -d 20:cf:30:85:7d:fd -i eth1.100 -p3 -s 1500&

```
```

	// run your listener on workstation (should be in same vlan)
	// (I took at https://lore.kernel.org/r/20171017010128.22141-1-vinicius.gomes@intel.com/)
	./tsn_listener -d 18:03:73:66:87:42 -i enp5s0 -s 1500
	Receiving data rate: 39012 kbps
	Receiving data rate: 39012 kbps
	Receiving data rate: 39012 kbps
	Receiving data rate: 39012 kbps
	Receiving data rate: 39012 kbps
	Receiving data rate: 39012 kbps
	Receiving data rate: 39012 kbps
	Receiving data rate: 39012 kbps
	Receiving data rate: 39012 kbps
	Receiving data rate: 39012 kbps
	Receiving data rate: 39012 kbps
	Receiving data rate: 39012 kbps
	Receiving data rate: 39000 kbps

```
```

	// Restore default configuration if needed
	$ ip link del eth1.100
	$ ip link del eth0.100
	$ tc qdisc del dev eth1 root
	net eth1: Prev FIFO2 is shaped
	net eth1: set FIFO3 bw = 0
	net eth1: set FIFO2 bw = 0
	$ tc qdisc del dev eth0 root
	net eth0: Prev FIFO2 is shaped
	net eth0: set FIFO3 bw = 0
	net eth0: set FIFO2 bw = 0
	$ ethtool -L eth0 rx 1 tx 1

```
