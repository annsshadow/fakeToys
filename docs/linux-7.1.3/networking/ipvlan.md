
## IPVLAN 驱动使用指南


初始版本：
	Mahesh Bandewar <maheshb AT google.com>

## 1. 简介：

从概念上讲，它与 macvlan 驱动非常相似，主要区别在于使用 L3 在从设备（slave）之间进行多路复用/解复用。这一特性使得主设备与其从设备共享 L2。我是在配合网络命名空间开发这个驱动的，不确定在此之外是否还有其它使用场景。


## 2. 构建与安装：


为了构建该驱动，请选择配置项 CONFIG_IPVLAN。该驱动可以内建到内核中（CONFIG_IPVLAN=y），也可以作为模块构建（CONFIG_IPVLAN=m）。


## 3. 配置：


该驱动没有模块参数，可以使用 IProute2/ip 工具进行配置。
```

    ip link add link <master> name <slave> type ipvlan [ mode MODE ] [ FLAGS ]
       where
	 MODE: l3 (default) | l3s | l2
	 FLAGS: bridge (default) | private | vepa

```
例如：

    (a) 以下命令将创建一个以 eth0 为主设备、模式为
```

	  bash# ip link add link eth0 name ipvl0 type ipvlan
    (b) This command will create IPvlan link in L2 bridge mode::

	  bash# ip link add link eth0 name ipvl0 type ipvlan mode l2 bridge

    (c) This command will create an IPvlan device in L2 private mode::

	  bash# ip link add link eth0 name ipvlan type ipvlan mode l2 private

    (d) This command will create an IPvlan device in L2 vepa mode::

	  bash# ip link add link eth0 name ipvlan type ipvlan mode l2 vepa


```
## 4. 工作模式：


IPvlan 有两种工作模式——L2 和 L3。对于给定的主设备，你可以选择这两种模式之一，该主设备上的所有从设备都将以相同的（所选）模式运行。除了在 L3 模式下从设备不会接收任何多播/广播流量之外，RX 模式几乎相同。L3 模式限制更多，因为路由是从另一个（通常是默认）命名空间控制的。

### 4.1 L2 模式：


在此模式下，TX 处理发生在挂载到从设备的协议栈实例上，数据包被交换并排入主设备以发送出去。在此模式下，从设备也会接收/发送多播和广播（如适用）。

### 4.2 L3 模式：


在此模式下，到 L3 为止的 TX 处理发生在挂载到从设备的协议栈实例上，数据包被切换到主设备的协议栈实例进行 L2 处理和路由，然后再排入出站设备。在此模式下，从设备既不能接收也不能发送多播/广播流量。

### 4.3 L3S 模式：


这与 L3 模式非常相似，区别在于 iptables（连接跟踪）在此模式下可用，因此它是 L3 对称的（L3s）。其性能会略低一些，但这无关紧要，因为你选择此模式而非纯 L3 模式是为了让连接跟踪正常工作。

## 5. 模式标志：


目前提供以下模式标志

### 5.1 bridge（桥接）：


这是默认选项。要将 IPvlan 端口配置为此模式，用户可以选择在命令行上添加该选项，或者不指定任何选项。这是传统模式，从设备之间可以互相通信，也可以通过主设备进行通信。

### 5.2 private（私有）：


如果在命令行上添加此选项，端口将被设置为私有模式。即端口不允许从设备之间互相通信。

### 5.3 vepa：


如果在命令行上添加此选项，端口将被设置为 VEPA 模式。即端口会像 802.1Qbg 中描述的那样，将交换功能卸载到外部实体。
注意：IPvlan 中的 VEPA 模式存在限制。IPvlan 使用主设备的 MAC 地址，因此在此模式下为相邻邻居发出的数据包的源 MAC 和目的 MAC 将相同。这将导致交换机/路由器发送重定向消息。

## 6. 如何选择（macvlan 与 ipvlan）？


这两种设备在许多方面非常相似，具体的使用场景很可能决定选择哪一种设备。如果你的使用场景符合以下某一种情况，则可以选择使用 ipvlan：


(a) 连接到外部交换机/路由器的 Linux 主机配置了策略，每个端口只允许一个 MAC 地址。
(b) 在主设备上创建的虚拟设备数量超过了 MAC 容量，导致网卡进入混杂模式，性能下降成为一个问题。
(c) 如果从设备要被放入敌对/不受信任的网络命名空间，其中从设备上的 L2 可能被更改/滥用。


## 6. 配置示例：


```

  +=============================================================+
  |  Host: host1                                                |
  |                                                             |
  |   +----------------------+      +----------------------+    |
  |   |   NS:ns0             |      |  NS:ns1              |    |
  |   |                      |      |                      |    |
  |   |                      |      |                      |    |
  |   |        ipvl0         |      |         ipvl1        |    |
  |   +----------#-----------+      +-----------#----------+    |
  |              #                              #               |
  |              ################################               |
  |                              # eth0                         |
  +==============================#==============================+


```
```

	ip netns add ns0
	ip netns add ns1

```
```

	ip link add link eth0 ipvl0 type ipvlan mode l2
	ip link add link eth0 ipvl1 type ipvlan mode l2

```
```

	ip link set dev ipvl0 netns ns0
	ip link set dev ipvl1 netns ns1

```
(d) 现在切换到命名空间（ns0 或 ns1）以配置从设备
```

		(1) ip netns exec ns0 bash
		(2) ip link set dev ipvl0 up
		(3) ip link set dev lo up
		(4) ip -4 addr add 127.0.0.1 dev lo
		(5) ip -4 addr add $IPADDR dev ipvl0
		(6) ip -4 route add default via $ROUTER dev ipvl0

	- For ns1::

		(1) ip netns exec ns1 bash
		(2) ip link set dev ipvl1 up
		(3) ip link set dev lo up
		(4) ip -4 addr add 127.0.0.1 dev lo
		(5) ip -4 addr add $IPADDR dev ipvl1
		(6) ip -4 route add default via $ROUTER dev ipvl1

```