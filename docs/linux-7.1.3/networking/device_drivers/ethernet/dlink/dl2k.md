
## D-Link 基于 DL2000 的千兆以太网适配器安
2002 骞?5 鏈?23 鏃。

 - 兼容性列 - 快速安 - 编译驱动
 - 安装驱动
 - 选项参数
 - 配置脚本示例
 - 故障排除


## 兼容性列
适配器支持：

- D-Link DGE-550T 千兆以太网适配器- D-Link DGE-550SX 千兆以太网适配器- 基于 D-Link DL2000 的千兆以太网适配器

该驱动支Linux 内核 2.4.7 及之后版本。我们已在以下环境中测试过它
 . Red Hat v6.2（内核升级到 2.4.7 . Red Hat v7.0（内核升级到 2.4.7 . Red Hat v7.1（内2.4.7 . Red Hat v7.2（内2.4.7-10

## 快速安
```

    1. make all
    2. insmod dl2k.ko
    3. ifconfig eth0 up 10.xxx.xxx.xxx netmask 255.0.0.0
			^^^^^^^^^^^^^^^\	    ^^^^^^^^\
					IP		     NETMASK

```
现在 eth0 应该已激活，你可以用 "ping" 测试它，或用 "ifconfig" 获取更多信息如果测试通过，继续下一步
4. ``cp dl2k.ko /lib/modules/`uname -r`/kernel/drivers/net``
```

	alias eth0 dl2k

```
6. 运行 `depmod` 以更新模块索引7. 运行 `netconfig` `netconf` 创建位于 /etc/sysconfig/network-scripts 的配   脚本 ifcfg-eth0，或者手动创建它
   [- 配置脚本示例]
8. 驱动将在下次启动时自动加载并配置
## 编译驱动

Linux 中，NIC 驱动最常见是配置为可加载模块。构建单体（monolithic）内核的
方法已经过时。该驱动可以编译为单体内核的一部分，但强烈不建议这样做。本节剩余部假设驱动被构建为可加载模块。在 Linux 环境中，最好从源代码重新构建驱动，而不是依预编译版本。这种方法提供了更好的可靠性，因为预编译的驱动可能依赖于某个给Linux
安装中并不存在的库或内核特性
构建 Linux 设备驱动所需3 个文件是 dl2k.c、dl2k.h Makefile。要编译，Linux
安装必须包含 gcc 编译器、内核源代码以及内核头文件。该 Linux 驱动支持 Linux 内核
2.4.7。将文件复制到一个目录，并输入以下命令来编译和链接驱动：

### CD-ROM 椹卞姩鍣。

```

    [root@XXX /] mkdir cdrom
    [root@XXX /] mount -r -t iso9660 -o conv=auto /dev/cdrom /cdrom
    [root@XXX /] cd root
    [root@XXX /root] mkdir dl2k
    [root@XXX /root] cd dl2k
    [root@XXX dl2k] cp /cdrom/linux/dl2k.tgz /root/dl2k
    [root@XXX dl2k] tar xfvz dl2k.tgz
    [root@XXX dl2k] make all

```
### 软盘驱动

```

    [root@XXX /] cd root
    [root@XXX /root] mkdir dl2k
    [root@XXX /root] cd dl2k
    [root@XXX dl2k] mcopy a:/linux/dl2k.tgz /root/dl2k
    [root@XXX dl2k] tar xfvz dl2k.tgz
    [root@XXX dl2k] make all

```
## 安装驱动


### 手动安装


  一旦驱动被编译完成，它必须被加载、启用并绑定到一个协议栈，才能建立网络连接。要
  加载一```

    insmod dl2k.o

  鎴?:

    insmod dl2k.o <可选参	; 添加参数

```
---------------------------------------------------------

```

    insmod dl2k.o media=100mbps_hd

  鎴?:

    insmod dl2k.o media=3

  鎴?:

    insmod dl2k.o media=3,2	; 针对 2 张网
```
---------------------------------------------------------

  请参考下面的 Linux 设备驱动支持的命令行参数列表
  insmod 命令只加载驱动并为其赋予一个形eth0、eth1 等的名称。要NIC 进入
  可操作状态，
```

    ifconfig eth0 up

  最后，要将驱动绑定到活动的协议（例Linux 下的 TCP/IP），输入以下命令::

    ifup eth0

  注意，这仅在系统能够找到包含必要网络信息的配置脚本时才有意义。示例将在下一  给出
  卸载驱动的命令如:

    ifdown eth0
    ifconfig eth0 down
    rmmod dl2k.o

  下面是用于列出当前已加载模块以及查看当前网络配置的命:

    lsmod
    ifconfig


```
### 自动安装

  本节描述如何将驱动安装为在启动时自动加载并配置。以下描述基Red Hat 6.0/7.0
  发行版，但也可以很容易地移植到其他发行版
### Red Hat v6.x/v7.x

  1. dl2k.o 复制到网络模块目录，通常     /lib/modules/2.x.x-xx/net /lib/modules/2.x.x/kernel/drivers/net  2. 找到启动模块配置文件，通常位于
```

	alias ethx dl2k
	options dl2k <可选参

     其中，如NIC 是唯一一个以太网适配器，ethx eth0；如果还安装了一个其     以太网适配器，则为 eth1，依此类推。可选参数列表请参阅上一节中的表格  3. 找到网络配置脚本，通常/etc/sysconfig/network-scripts 目录，并创建一个名     ifcfg-ethx、包含网络信息的配置脚本  4. 注意，对于大多数 Linux 发行版（包括 Red Hat），都提供了一个带有图形用户界面的
     配置工具来执行上述第 2 步和3 步

```
## 参数说明

你可以在不添加任何额外参数的情况下安装此驱动。但是，如果你想要使用扩展功能，则有
必要设置额外的参数。下面是 Linux 设备驱动支持的命令行参数列表

===============================   ==============================================
mtu=packet_size			  指定最大数据包大小。默认为
				  1500銆?
media=media_type		  指定 NIC 工作所在的介质类型				  autosense	自动感知活动介质
				  ===========	=========================
				  10mbps_hd	10Mbps 半双工				  10mbps_fd	10Mbps 全双工				  100mbps_hd	100Mbps 半双工				  100mbps_fd	100Mbps 全双工				  1000mbps_fd	1000Mbps 全双工				  1000mbps_hd	1000Mbps 半双工				  0		自动感知活动介质				  1		10Mbps 半双工				  2		10Mbps 全双工				  3		100Mbps 半双工				  4		100Mbps 全双工				  5          	1000Mbps 半双工				  6          	1000Mbps 全双工				  ===========	=========================

				  默认情况下，NIC 工作在自动感知模式				  1000mbps_fd 1000mbps_hd 类型仅适用				  光纤适配器
vlan=n				  指定 VLAN ID。如vlan=0，则虚拟局域网
				  （VLAN）功能被禁用
jumbo=[0|1]			  指定巨型帧支持。如jumbo=1，则 NIC 接受
				  巨型帧。默认情况下此功能被禁用				  巨型帧通常能提升千兆网络下的性能				  此特性需要远程端支持巨型帧
rx_coalesce=m			  每次中断处理的接收帧数量rx_timeout=n			  接收 DMA 等待中断的时间				  如果设置 rx_coalesce > 0，硬件只在收m
				  个帧时才触发一次中断。硬件在收到 m 个帧				  达到 n * 640 纳秒的超时之前不会触发接				  中断。设置合适的 rx_coalesce rx_timeout
				  可以减少拥塞崩溃和过载，这曾是高速网络的
				  瓶颈
				  例如，rx_coalesce=10 rx_timeout=800				  即硬件在收到 10 个帧512 微秒超时时才
				  触发 1 次中断
tx_coalesce=n			  每次中断处理的发送帧数量				  设置 n > 1 可以减少中断拥塞，但通常会降				  高速网卡的性能。默认为 16
tx_flow=[1|0]			  指定发送流控。如tx_flow=0，则禁用发				  流控，否则驱动自动检测rx_flow=[1|0]			  指定接收流控。如rx_flow=0，则启用接收
				  流控，否则驱动自动检测===============================   ==============================================


## 配置脚本示例

```

    DEVICE=eth0
    USERCTL=no
    ONBOOT=yes
    POOTPROTO=none
    BROADCAST=207.200.5.255
    NETWORK=207.200.5.0
    NETMASK=255.255.255.0
    IPADDR=207.200.5.2


```
## 故障排除

Q1. 源文件每行末尾都包含 ^ M
    确保所有文件都Unix 文件格式（无 LF）。尝试以下命```

	cat dl2k.c | col -b > dl2k.tmp
	mv dl2k.tmp dl2k.c

    鎴?:

	cat dl2k.c | tr -d "\r" > dl2k.tmp
	mv dl2k.tmp dl2k.c

```
Q2：找不到头文件（`*.h`）？

    要编译驱动，你需要内核头文件。安装内核源代码后，头文件通常位于
    /usr/src/linux/include，这Makefile 中配置的默认包含目录。对于某些发行版    /usr/src/include/linux /usr/src/include/asm 下有一份头文件副本，你可以
    在不安装内核源代码的情况下，Makefile 中的 INCLUDEDIR 改为 /usr/include
    注意，RH 7.0 /usr/include 下没有提供正确的头文件，包含这些文件会导致生    错误版本的驱动