## HVCS IBM “Hypervisor Virtual Console Server”（管理程序虚拟控制台服务器）安装指南


适用于 Linux 内核 2.6.4+

Copyright (C) 2004 IBM Corporation



作者：Ryan S. Arnold <rsa@us.ibm.com>

创建日期：2004 年 3 月 2 日
最后修改：2004 年 8 月 24 日


 1. 驱动简介：
 2. 系统要求
 3. 构建选项：
		3.1  内置：
		3.2  模块：
 4. 安装：
 5. 连接：
 6. 断开连接：
 7. 配置：
 8. 问答：
 9. 报告缺陷：

## 1. 驱动简介：


这是 IBM 管理程序虚拟控制台服务器 “hvcs” 的设备驱动。IBM hvcs 提供
一个 tty 驱动接口，使 Linux 用户空间应用程序能够访问运行在同一分区
Power5 ppc64 系统上的逻辑分区操作系统（Linux 和 AIX）的系统控制台。
该硬件上每个分区使用物理硬件控制台并不现实，因此本驱动通过固件接口
访问虚拟终端设备来使用系统控制台。

## 2. 系统要求：


本设备驱动使用 2.6.4 Linux 内核 API 编写，只能在此版本或更高版本的内核上
构建和运行。

本驱动专为在 IBM Power5 ppc64 硬件上运行而编写，不过在编写时做了一些努力，
将架构相关的固件调用从驱动代码中抽象出来。

Sysfs 必须已在系统上挂载，以便用户能够确定每个 vty-server 关联的主设备号和
次设备号。Sysfs 挂载方法不在本文档讨论范围之内。

## 3. 构建选项：


hvcs 驱动以 tty 驱动的形式注册自己。tty 层按照注册驱动请求的数量动态分配
一块主设备号和次设备号。hvcs 驱动默认向 tty 层请求 64 个这样的主/次设备号，
用于 hvcs 设备节点条目。

如果默认的设备条目数量足够，则可以将本驱动构建进内核。否则，可以通过使用
insmod 参数将驱动作为模块插入来覆盖默认值。

### 3.1 内置：


以下 menuconfig 示例演示了选择将本驱动构建进内核
```

	Device Drivers  --->
		Character devices  --->
			<*> IBM Hypervisor Virtual Console Server Support

```
开始内核 make 过程。

### 3.2 模块：


以下 menuconfig 示例演示了选择将本驱动构建为模块
```

	Device Drivers  --->
		Character devices  --->
			<M> IBM Hypervisor Virtual Console Server Support

```
make 过程将构建以下内核模块：

 - hvcs.ko
 - hvcserver.ko

要以默认分配插入模块，请执行以下命令
```

	insmod hvcserver.ko
	insmod hvcs.ko

```
hvcserver 模块包含架构相关的固件调用，必须首先插入，否则 hvcs 模块将找不到
它期望的某些符号。

要覆盖默认值，请使用如下 insmod 参数（请求 4 个）
```

	insmod hvcs.ko hvcs_parm_num_devs=4

```
insmod 时可指定的设备条目数量有最大值。我们认为目前 1024 是一个合理的服务器
适配器最大允许数量。这始终可以通过在构建前修改源文件中的常量来更改。

注意：insmod 驱动所花费的时间长短似乎与注册驱动请求的 tty 接口数量有关。
```

	rmmod hvcs.ko

```
将 hvcs 作为模块安装的建议方法是使用 depmod 在 /lib/modules/`uname -r`
中构建最新的 modules.dep 文件，然后
```

	modprobe hvcs hvcs_parm_num_devs=4

```
modules.dep 文件指明 hvcserver.ko 需要在 hvcs.ko 之前插入，modprobe 使用该
文件智能地按正确顺序插入模块。

以下 modprobe 命令用于按如下顺序移除 hvcs 和 hvcserver
```

	modprobe -r hvcs

```
## 4. 安装：


tty 层创建包含为 hvcs 驱动分配的主设备号和次设备号的 sysfs 条目。以下 “tree”
片段
```

	sys/
	|-- *other sysfs base dirs*
	|
	|-- class
	|   |-- *other classes of devices*
	|   |
	|   `-- tty
	|       |-- *other tty devices*
	|       |
	|       |-- hvcs0
	|       |   `-- dev
	|       |-- hvcs1
	|       |   `-- dev
	|       |-- hvcs2
	|       |   `-- dev
	|       |-- hvcs3
	|       |   `-- dev
	|       |
	|       |-- *other tty devices*
	|
	|-- *other sysfs base dirs*

```
对于上述示例，以下输出是 cat “dev” 的结果
```

	Pow5:/sys/class/tty/hvcs0/ # cat dev
	254:0

	Pow5:/sys/class/tty/hvcs1/ # cat dev
	254:1

	Pow5:/sys/class/tty/hvcs2/ # cat dev
	254:2

	Pow5:/sys/class/tty/hvcs3/ # cat dev
	254:3

```
读取 “dev” 属性的输出是 tty 层为本驱动使用而分配的字设备主设备号和次设备号。
大多数运行 hvcs 的系统已经创建了设备条目，或者 udev 会自动创建它们。

根据上述示例输出，要手动创建 /dev/hvcs* 节点条目，请执行
```

	mknod /dev/hvcs0 c 254 0
	mknod /dev/hvcs1 c 254 1
	mknod /dev/hvcs2 c 254 2
	mknod /dev/hvcs3 c 254 3

```
使用 mknod 手动创建设备条目会使这些设备节点持久存在。一旦创建，它们将在驱动
insmod 之前就存在。

在插入驱动之前尝试将应用程序连接到 /dev/hvcs* 会产生
```

	"/dev/hvcs*: No such device".

```
注意：仅仅存在设备节点并不意味着该节点已配置了 vty-server 设备。

## 5. 连接

本驱动控制提供 tty 接口的设备，因此用户可以使用任何标准 tty 交互方法（例如
“cat”、“dd”、“echo”）与该设备节点条目交互。但是，本驱动的意图是为 Linux
分区的控制台提供实时的控制台交互，这需要使用能够提供与 tty 设备的双向交互式
I/O 的应用程序。

充当终端模拟器或对所传递数据执行终端类型控制序列转换的应用程序（例如 “minicom”
和 “screen”）不适合提供交互式控制台 I/O。这些程序通常模拟过时的终端类型
（vt100 和 ANSI），并期望入站数据采用这些受支持终端类型之一的形式，但它们要么
不转换、要么不能“充分地”将出站数据转换为调用它们的终端的终端类型（尽管 screen
做了尝试，并且显然可以通过大量的 termcap 调整来配置）。

因此，kermit 和 cu 是推荐用于通过 hvcs 设备与 Linux 控制台交互的两个应用程序。
这些程序只是充当与 tty 设备之间数据传输的通道。它们不要求入站数据采用特定终端
类型的形式，也不会将出站数据转换为特定终端类型。

为了确保控制台应用程序正常运行，必须确保在连接到 /dev/hvcs 控制台后，控制台的
$TERM 环境变量被设置为用于启动交互式 I/O 应用程序的终端模拟器的确切终端类型。
如果使用 xterm 和 kermit 连接到 /dev/hvcs0，当控制台提示符可用时，应在控制台
上 “export TERM=xterm”。这会告诉从控制台调用的 ncurses 应用程序，它们应输出
xterm 能够理解的控制序列。

作为预防措施，hvcs 用户在将 kermit 等应用程序从设备节点断开之前，应始终从会话
中 “exit”。如果不这样做，连接到控制台的下一个用户将继续使用上一个用户已登录的
会话，包括使用上一个用户提供的 $TERM 变量。

vty-server 适配器的热插拔添加和移除会影响用于连接每个 vty-server 适配器的
/dev/hvcs* 节点。为了确定哪个 vty-server 适配器与哪个 /dev/hvcs* 节点相关联，
每个 vty-server sysfs 条目都添加了一个特殊的 sysfs 属性。该条目称为 “index”，
显示它会揭示一个整数，指代用于连接该设备的 /dev/hvcs* 条目。例如，cat 以下
```

	Pow5:/sys/bus/vio/drivers/hvcs/30000004 # cat index
	2

```
这个 '2' 的索引意味着，为了连接到 vty-server 适配器 30000004，用户应与 /dev/hvcs2
交互。

需要注意的是，由于系统的热插拔 I/O 能力，与特定 vty-server 适配器交互的
/dev/hvcs* 条目不能保证在系统重启后保持不变。有关此问题的更多信息，请查看
问答部分。

## 6. 断开连接

作为防止将陈旧数据传递给非预期目标的安全特性，Power5 系统固件在 vty-server 与
vty 之间的连接被切断时会禁用数据获取并丢弃该数据。例如，当 vty-server 在输出
数据到 vty 之后立即与 vty 断开连接时，vty 适配器可能在接收到数据中断到连接被
切断之间的时间间隔不足以在获取被固件禁用之前从固件获取该数据。

当 hvcs 用于服务控制台时，这种行为不是大问题，因为在几乎所有数据写入之后，适配器
都会保持连接很长一段时间。当 hvcs 用作在两个分区之间隧道传输数据的 tty 通道时
[见下文问答]，这就是一个严重问题，因为使用 cat 或 dd 将数据写入设备时 Linux 的
标准行为是打开 tty、发送数据，然后关闭 tty。如果本驱动在 tty 关闭时手动终止
vty-server 连接，这将在目标 vty 有机会获取该数据之前关闭 vty-server 与 vty 的
连接。

此外，仅在模块移除或适配器移除时断开 vty-server 与 vty 是不实际的，因为其他分区
中的其他 vty-server 可能随时需要目标 vty 的使用。

由于这种行为限制，vty-server 与所连接 vty 的断开连接是一个手动过程，使用下面
概述的对 sysfs 属性的写操作；另一方面，vty-server 与 vty 的初始连接由本驱动
自动建立。从不需要手动建立 vty-server 连接。

为了终止 vty-server 与 vty 之间的连接，使用每个 vty-server sysfs 条目中的
“vterm_state” sysfs 属性。读取该属性会显示 vty-server 适配器的当前连接状态。
零表示 vty-server 未连接到 vty。一表示连接处于活动状态。

仅当 vterm_state 先前读取为 '1' 时，向 vterm_state 属性写入 '0'（零）才会断开
vty-server 与目标 vty 之间的 VTERM 连接。如果 vterm_state 读取为 '0'，或者向
vterm_state 属性写入了 '0' 以外的任何值，则写入指令将被忽略。以下示例将展示用于
验证的方法
```

	Pow5:/sys/bus/vio/drivers/hvcs/30000004 # cat vterm_state
	1

	Pow5:/sys/bus/vio/drivers/hvcs/30000004 # echo 0 > vterm_state

	Pow5:/sys/bus/vio/drivers/hvcs/30000004 # cat vterm_state
	0

```
当设备被热插拔移除以及模块被移除时，所有 vty-server 连接都会自动终止。

## 7. 配置

每个 vty-server 在 /sys/devices/vio 目录中有一个 sysfs 条目，该条目在其它几个
sysfs 树目录中被符号链接，特别是在
```

	Pow5:/sys/bus/vio/drivers/hvcs # ls
	.  ..  30000003  30000004  rescan

```
默认情况下，固件会通知 hvcs 驱动 vty-server 的生命周期和伙伴 vty 的移除，但不会
通知伙伴 vty 的添加。由于 HMC 超级管理员可以动态添加伙伴信息，我们为 hvcs 驱动
sysfs 目录提供了 “rescan” 更新属性，该属性会查询固件并更新本驱动管理的所有
vty-server 的伙伴信息。向该属性写入 '1' 会触发更新。一个明确的示例如下：

	Pow5:/sys/bus/vio/drivers/hvcs # echo 1 > rescan

读取该属性会显示 '1' 或 '0' 的状态。一表示更新正在进行中。零表示更新已完成或从未
执行。

此目录中的 vty-server 条目是由固件创建的 32 位分区唯一单元地址。一个 vty-server
sysfs 条目示例如下
```

	Pow5:/sys/bus/vio/drivers/hvcs/30000004 # ls
	.   current_vty   devspec       name          partner_vtys
	..  index         partner_clcs  vterm_state

```
每个条目默认提供一个 “name” 属性。读取 “name” 属性会显示设备类型，如下面所示
```

	Pow5:/sys/bus/vio/drivers/hvcs/30000003 # cat name
	vty-server

```
每个条目默认还提供一个 “devspec” 属性，读取时会显示完整的设备规格，如下面所示
```

	Pow5:/sys/bus/vio/drivers/hvcs/30000004 # cat devspec
	/vdevice/vty-server@30000004

```
每个 vty-server sysfs 目录提供两个只读属性，提供易于解析的伙伴 vty 数据列表：
“partner_vtys” 和
```

	Pow5:/sys/bus/vio/drivers/hvcs/30000004 # cat partner_vtys
	30000000
	30000001
	30000002
	30000000
	30000000

	Pow5:/sys/bus/vio/drivers/hvcs/30000004 # cat partner_clcs
	U5112.428.103048A-V3-C0
	U5112.428.103048A-V3-C2
	U5112.428.103048A-V3-C3
	U5112.428.103048A-V4-C0
	U5112.428.103048A-V5-C0

```
读取 partner_vtys 会返回伙伴 vty 的列表。Vty 单元地址编号仅对每个分区唯一，因此
条目会频繁重复。

读取 partner_clcs 会返回 “融合位置码” 的列表，它们由系统序列号后跟 “-V**” 组成，
其中 '**' 是目标分区号，以及 “-C**”，其中 '**' 是适配器的槽位。第一个 vty 伙伴
对应第一个 clc 项，第二个 vty 伙伴对应第二个 clc 项，依此类推。

一个 vty-server 一次只能连接到一个 vty。条目 “current_vty” 在读取时会打印当前
所选伙伴 vty 的 clc。

可以通过将有效的伙伴 clc 写入该条目来更改 current_vty
```

	Pow5:/sys/bus/vio/drivers/hvcs/30000004 # echo U5112.428.10304
	8A-V4-C0 > current_vty

```
当 vty-server 已连接到 vty 时更改 current_vty 不会影响当前连接。更改在已打开的
连接被释放时生效。

关于 “vterm_state” 属性的信息已在前面名为 “断开连接” 的章节中介绍。

## 8. 问答：


Q：涉及 hvcs 的安全问题有哪些？

A：主要有三个安全问题：

 1. /dev/hvcs* 节点的创建者能够将对设备条目的访问限制为某些用户或组。最好创建
	一个特殊的 hvcs 组权限来提供对系统控制台的访问。

 2. 为了在获取控制台时提供网络安全性，建议用户使用安全方法（例如 SSH）连接到
	托管控制台的分区，或者坐在硬件控制台前。

 3. 使用完控制台后务必退出用户会话，否则下一个 vty-server 连接（可能来自另一个
	分区）将体验到先前已登录的会话。

---------------------------------------------------------------------------

Q：如何多路复用我通过 hvcs 获取的控制台，以便其他人可以看到它：

A：你可以使用 “screen” 直接连接到 /dev/hvcs* 设备，并在你的机器上以控制台组
权限建立一个会话。如前所述，默认情况下 screen 不提供大多数终端模拟器的 termcap
设置来提供从 term 类型 “screen” 到其他类型的充分字符转换。这意味着基于 curses
的程序可能无法在 screen 会话中正确显示。

---------------------------------------------------------------------------

Q：为什么颜色全乱了？
Q：为什么控制字符行为异常或不起作用？
Q：为什么控制台输出全都很奇怪且难以理解？

A：请参阅前面关于 “连接” 的部分，讨论应用程序如何影响字符控制序列的显示。此外，
仅仅因为你使用 xterm 登录到控制台，并不意味着在你之前没有其他人使用 HMC 控制台
（vt320）登录到控制台并让会话保持登录状态。最好的做法是在获取控制台时将 TERM
导出为你的终端模拟器的终端类型。此外，在从控制台断开连接之前务必 “exit” 控制台。
这将确保下一个用户在登录时获得他们自己的 TERM 类型设置。

---------------------------------------------------------------------------

Q：当我尝试将 kermit 连接到 hvcs 设备时得到：
"Sorry, can't open connection: /dev/hvcs*" 发生了什么？

A：某些其它 Power5 控制台机制已连接到该 vty 并且不释放它。你可以尝试通过右键单击
分区然后选择 “close terminal” 从 HMC 强制断开控制台连接。否则，你必须找出拥有
控制台权限的人。有可能你已经使用另一个 kermit 会话打开了控制台却忘了。请查看
Power5 系统的控制台选项，以确定系统控制台可以被占用的多种方式。

OR

A：另一个用户当前可能未将连接方法附加到 /dev/hvcs 设备，但 vterm_state 可能显示
他们仍然建立了 vty-server 连接。他们需要使用 “断开连接” 部分概述的方法释放它，
以便其他人连接到目标 vty。

OR

A：你用于执行 kermit 的用户配置文件可能没有使用 /dev/hvcs* 设备的权限。

OR

A：你可能还没有插入 hvcs.ko 模块，但 /dev/hvcs* 条目仍然存在（在没有 udev 的
系统上）。

OR

A：没有映射到现有 /dev/hvcs* 条目的相应 vty-server 设备。

---------------------------------------------------------------------------

Q：当我尝试将 kermit 连接到 hvcs 设备时得到：
"Sorry, write access to UUCP lockfile directory denied."

A：你指定的 /dev/hvcs* 条目并不存在于你说的地方？也许你还没有插入模块（在使用
udev 的系统上）。

---------------------------------------------------------------------------

Q：如果我已经安装了一个 Linux 分区，能否在该分区上使用 hvcs 为第二个 Linux 分区
的安装提供控制台？

A：可以，前提是你使用 kermit 或 cu 或某些其它不提供终端模拟的程序连接到
/dev/hvcs* 设备。

---------------------------------------------------------------------------

Q：我可以使用本驱动一次连接到多个分区的控制台吗？

A：可以。当然，这意味着必须为该分区配置多个 vty-server，并且每个都必须指向一个
已断开连接的 vty。

---------------------------------------------------------------------------

Q：hvcs 驱动是否支持设备的动态（热插拔）添加？

A：支持，如果你的系统启用了 dlpar 和 hotplug，并且它已被构建进内核，则 hvcs 驱动
被配置为动态处理新设备的添加和未使用设备的移除。

---------------------------------------------------------------------------

Q：出于某种原因，/dev/hvcs* 在重启后没有映射到同一个 vty-server 适配器。发生了
什么？

A：vty-server 适配器到 /dev/hvcs* 条目的分配始终以适配器被暴露的顺序进行。由于
本驱动的热插拔能力，热插拔添加的 vty-server 的分配顺序可能与模块加载时暴露的顺序
不同。如果在两个其它 vty-server 适配器之间的槽位中添加了一个 vty-server 适配器，
则动态添加后重启或重新加载模块可能会导致 /dev/hvcs* 与 vty-server 的耦合关系
改变。请参阅上面的部分，了解如何确定哪个 vty-server 对应哪个 /dev/hvcs* 节点。
提示；查看 vty-server 的 sysfs “index” 属性。

---------------------------------------------------------------------------

Q：我能否将 /dev/hvcs* 用作到另一个分区的通道，并将该分区上的 tty 设备用作管道
的另一端？

A：可以，在 Power5 平台上，hvc_console 驱动为额外的 /dev/hvc* 设备（其中
/dev/hvc0 最有可能是控制台）提供 tty 接口。为了使两个分区之间的 tty 通道工作，
HMC 超级管理员必须使用 HMC gui 为目标分区创建额外的 “serial server”，当目标分区
重新启动时，它将显示为 /dev/hvc*。

HMC 超级管理员随后为当前分区创建额外的 “serial client”，并将其指向目标分区新创建
的 “serial server” 适配器（记住槽位）。这显示为额外的 /dev/hvcs* 设备。

现在，可以在目标系统上配置一个程序来读取或写入 /dev/hvc*，并在当前分区上配置
另一个程序来读取或写入 /dev/hvcs*。现在你在两个分区之间有了一个 tty 通道。

---------------------------------------------------------------------------

## 9. 报告缺陷：


报告缺陷的正确渠道是通过提供你的操作系统的 Linux OS 发行公司，或者将问题发布到
PowerPC 开发邮件列表：

linuxppc-dev@lists.ozlabs.org

此请求旨在提供围绕本驱动的问题和解决方案的、有文档记录且可搜索的公开交流，以造福
所有用户。
