## Dell 远程 BIOS 更新驱动（dell_rbu）


## 目的


本文档演示如何使用 Dell 远程 BIOS 更新驱动在 Dell 服务器和台式机上更新
BIOS 镜像。

## 范围


本文档仅讨论 rbu 驱动的功能。它不涵盖应用程序所需的、用于使 BIOS 能够用
下载到内存中的镜像自我更新的支持。

## 概述


该驱动与 Dell OpenManage 或 Dell 更新包（DUP）配合，用于更新 Dell 服务器
（自 1999 年起销售的服务器）、台式机和笔记本（自 2005 年起销售的产品）上的
BIOS。

请访问 http://support.dell.com 注册，你可以在那里找到关于 OpenManage 和
Dell 更新包（DUP）的信息。

也可以使用 Libsmbios 来更新 Dell 系统上的 BIOS，详情请访问
https://linux.dell.com/libsmbios/。

Dell_RBU 驱动支持使用整体式（monolithic）镜像和数据包式（packetized）镜像
两种方法来更新 BIOS。对于整体式，驱动分配一块连续的物理页，用于存放 BIOS
镜像。对于数据包式，使用该驱动的应用程序将镜像拆分成固定大小的数据包，由
驱动把每个数据包放到连续的物理内存中。驱动还维护一个数据包链表以便回读。

如果 dell_rbu 驱动被卸载，所有已分配的内存都会被释放。

rbu 驱动需要有一个应用程序（如前所述）来通知 BIOS 在下次系统重启时启用更新。

用户不应在下载 BIOS 镜像或更新之后卸载 rbu 驱动。

```

	/sys/class/firmware/dell_rbu/loading
	/sys/class/firmware/dell_rbu/data
	/sys/devices/platform/dell_rbu/image_type
	/sys/devices/platform/dell_rbu/data
	/sys/devices/platform/dell_rbu/packet_size

```
该驱动支持两种更新机制：整体式和数据包式。这些更新机制取决于系统当前运行的
BIOS。大多数 Dell 系统支持整体式更新，即把 BIOS 镜像复制到一块连续的物理
内存中。

在数据包机制下，单块内存可以被拆分成更小的连续内存块，BIOS 镜像被分散到
这些数据包中。

默认情况下，驱动使用整体式内存作为更新类型。这可以通过在驱动加载时指定
加载参数来改为数据包式：
```

	echo packet > /sys/devices/platform/dell_rbu/image_type

```
在数据包更新模式下，必须先给出数据包大小，然后才能发送任何数据包：
```

	echo XXXX > /sys/devices/platform/dell_rbu/packet_size

```
在数据包更新机制中，用户需要创建一个新文件，其中数据包数据首尾相接依次
排列。做法如下：用户创建数据包头，取出一块 BIOS 镜像放在包头旁边；此时，
包头 + BIOS 镜像块加在一起应当与指定的 packet_size 相等。这样就构成了一个
数据包，用户需要从整个 BIOS 镜像文件中创建更多这样的数据包，然后将这些
数据包首尾相接排成一个单独的文件。

随后将该文件复制到 /sys/class/firmware/dell_rbu/data。一旦该文件到达驱动，
驱动就从文件中提取 packet_size 大小的数据，并将其分布到连续、大小为
packet_size 的物理内存空间中。

这种方法确保所有的数据包在一次操作中都能送达驱动。

在整体式更新中，用户只需直接获取 BIOS 镜像（.hdr 文件）并按原样复制到 data
文件，不对 BIOS 镜像本身做任何改动。

按以下步骤下载 BIOS 镜像：

1) echo 1 > /sys/class/firmware/dell_rbu/loading
2) cp bios_image.hdr /sys/class/firmware/dell_rbu/data
3) echo 0 > /sys/class/firmware/dell_rbu/loading

/sys/class/firmware/dell_rbu/ 下的条目会一直保留，直到执行以下操作：

```

	echo -1 > /sys/class/firmware/dell_rbu/loading

```
在完成此步骤之前，驱动无法被卸载。

此外，向 image_type 写入 mono、packet 或 init 都会释放驱动已分配的内存。

如果用户意外地执行了上面的第 1 步和第 3 步而没有执行第 2 步，将导致
/sys/class/firmware/dell_rbu/ 下的条目消失。

```

	echo init > /sys/devices/platform/dell_rbu/image_type

```

此外，驱动还提供 /sys/devices/platform/dell_rbu/data 只读文件，用于回读
已下载的镜像。


   更新完 BIOS 镜像后，用户态应用程序需要执行向 BIOS 发送 BIOS 更新请求的
   代码。这样在下次重启时，BIOS 就知道有新下载的镜像并自我更新。另外，如果
   要更新镜像，不要卸载 rbu 驱动。
