
## 简介（Introduction）


ATA over Ethernet（AoE，以太网之上的 ATA）是一种网络协议，提供对 LAN 上块存储的简单访问。

  http://support.coraid.com/documents/AoEr11.txt

适用于 2.6 和 3.x 内核的 EtherDrive(R) HOWTO 位于……

  http://support.coraid.com/support/linux/EtherDrive-2.6-HOWTO.html

其中有许多技巧与提示！请特别参见关于虚拟内存的推荐调优：

  http://support.coraid.com/support/linux/EtherDrive-2.6-HOWTO-5.html#ss5.19

aoetools 是专门配合此驱动工作的用户态程序，可在 sourceforge 上获取。

  http://aoetools.sourceforge.net/

本 Documentation/admin-guide/aoe 目录中的脚本旨在说明该驱动的使用方法；如果你安装了 aoetools，则并不需要它们。


## 创建设备节点（Creating Device Nodes）


  使用 udev 的用户应当会发现块设备节点会被自动创建；但要创建所有必要的设备节点，请使用本目录中 udev.txt 提供的 udev 配置规则。

  有一个 udev-install.sh 脚本，演示了如何在你的系统上安装这些规则。

  还有一个 autoload（自动加载）脚本，演示了如何编辑 /etc/modprobe.d/aoe.conf，以确保 aoe 模块在需要时会被加载。不过，预加载 aoe 模块优于自动加载，因为 AoE 发现需要几秒钟时间。当首次运行 a 命令时 AoE 设备尚不存在、而一秒钟后它又出现时，会令人困惑。

## 使用设备节点（Using Device Nodes）


  "cat /dev/etherd/err" 会阻塞，等待错误诊断输出，例如重传的数据包。

  "echo eth2 eth4 > /dev/etherd/interfaces" 告诉 aoe 驱动将 ATA over Ethernet 流量限制到 eth2 和 eth4。出于安全考虑，应忽略来自不可信网络的 AoE 流量。另请参见下文描述的 aoe_iflist 驱动选项。

  "echo > /dev/etherd/discover" 告诉驱动去发现有哪些 AoE 设备可用。

  将来这些字符设备可能会消失，并被 sysfs 中的对应项取代。使用 aoetools 中的命令可以将用户与这些实现细节隔离开来。

```

	e{shelf}.{slot}
	e{shelf}.{slot}p{part}

  ……因此 "e0.2" 表示第一个机架（shelf 地址为 0）中从左数第三个刀片（slot 2）。这就是整块磁盘。该磁盘的第一个分区将是 "e0.2p1"。

```
## 使用 sysfs（Using sysfs）


  /sys/block 中的每个 aoe 块设备都具有 state、mac 和 netif 等额外属性。当设备已准备好进行 I/O 时，state 属性为 "up"；若已被检测到但不可用，则为 "down"。"down,closewait" 状态表示设备仍处于打开状态，在关闭之前无法再次上线。

  mac 属性是远端 AoE 设备的以太网地址。netif 属性是本地主机上用于与远端 AoE 设备通信的网络接口。

  本目录中有一个脚本可以方便地格式化这些信息。使用 aoetools 的用户应使用 aoe-stat
```

    root@makki root# sh Documentation/admin-guide/aoe/status.sh
       e10.0            eth3              up
       e10.1            eth3              up
       e10.2            eth3              up
       e10.3            eth3              up
       e10.4            eth3              up
       e10.5            eth3              up
       e10.6            eth3              up
       e10.7            eth3              up
       e10.8            eth3              up
       e10.9            eth3              up
        e4.0            eth1              up
        e4.1            eth1              up
        e4.2            eth1              up
        e4.3            eth1              up
        e4.4            eth1              up
        e4.5            eth1              up
        e4.6            eth1              up
        e4.7            eth1              up
        e4.8            eth1              up
        e4.9            eth1              up

  使用 /sys/module/aoe/parameters/aoe_iflist（或者更好地，使用下文讨论的驱动选项）而不是 /dev/etherd/interfaces，将 AoE 流量限制到给定空白符分隔列表中的网络接口。与旧的字符设备不同，sysfs 项既可读也可写。

  设置完允许的接口列表后，触发发现操作是有帮助的。aoetools 软件包为此提供了 aoe-discover 脚本。你也可以直接使用上文描述的 /dev/etherd/discover 特殊文件。

```
## 驱动选项（Driver Options）


  内置 aoe 驱动有一个启动选项以及对应的模块参数 aoe_iflist。如果没有该选项，所有网络接口都可能被用于 ATA over Ethernet。下面是一个示例
```

    modprobe aoe_iflist="eth1 eth3"

```
  aoe_deadsecs 模块参数决定驱动等待 AoE 设备对 AoE 命令作出响应的最大秒数。经过 aoe_deadsecs 秒后，该 AoE 设备将被标记为 "down"。出于测试目的支持取值为零，会使 aoe 驱动永远不断重试 AoE 命令。

  aoe_maxout 模块参数默认值为 128。这是一次性发往某个 AoE 目标的最大未应答数据包数量。

  aoe_dyndevs 模块参数默认值为 1，表示驱动会根据发现顺序为发现的 AoE 目标分配块设备次设备号。在使用动态次设备号的情况下，可以支持更大范围的 AoE 机架与槽位地址。使用 udev 的用户永远无需关心次设备号。使用 aoe_dyndevs=0 则允许使用 aoetools 中的 aoe-mkshelf 脚本、通过静态次设备号方案预先创建设备节点。
