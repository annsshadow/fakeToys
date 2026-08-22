## 用于读取 z/VM Monitor 记录Linux API


Date  : 2004-Nov-26

Author: Gerald Schaefer (geraldsc@de.ibm.com)



## 描述

本项提供一个新Linux API，其形式为可供用户空间使用的混杂字符（misc char）设备，允许对由 z/VM `*MONITOR` System Service 所收集z/VM Monitor 记录进行读取访问

## 用户需
你希望访问此 API z/VM 客户机（guest）需要进行配置，以允许到 `*MONITOR` 服务IUCV 连接，即其用户条目中需要有 IUCV `*MONITOR` 语句。如果要使用monitor DCSS 受到限制（很可能），你还需NAMESAVE <DCSS NAME> 语句。本项将使用 IUCV 设备驱动来访z/VM 服务，因此你需要一个带IUCV 支持的内核。你还需z/VM 4.4 5.1 版本
能够加载 monitor DCSS 有两种选择（示例假monitor DCSS 起始144 MB、结束于 152 MB）。你可以E 级特权的 CP 命令 Q NSS MAP 来查monitor DCSS 的位置（BEGPAG ENDPAG 的值以 4K 页为单位给出）
同时参见“CP Command and Utility Reference”（SC24-6081-00）以获取关于 DEF STOR Q NSS MAP 命令的更多信息，以及“Saved Segments Planning and Administration”（SC24-6116-00）以获取关于 DCSS 的更多信息
### 绗?1 绉嶆柟妗堬細

你可以使CP 命令 DEF STOR CONFIG 在你的客户机虚拟存储（guest virtual storage）中围绕 DCSS 的地址范围定义一个“内存空洞（memory hole）”
示例：DEF STOR CONFIG 0.140M 200M.200M

这定义了两块存储，第一块大小为 140MB 且起始于地址 0MB，第二块大小200MB 且起始于地址 200MB，总存储为 340MB。注意，第一块应当始终从 0 开始，并且大小至少64MB
### 绗?2 绉嶆柟妗堬細

你的客户机虚拟存储必须在 DCSS 的起始地址之下结束，并且你必须在你parmfile 中用 “mem=内核参数指定一个大DCSS 结束地址的值
```

	DEF STOR 140M

```
这为你的客户机定义了 140MB 的存储大小，参数 “mem=160M被添加到 parmfile 中

## 用户接口

该字符设备实现为一个名“monreader的内核模块，可以通过 modprobe 命令加载，也可以编译进内核。有一个可选的模块（或内核）参“mondcss”，用于指定 monitor DCSS 的名称。如果模块被编译进内核，则可以在 parmfile 中指定内核参“monreader.mondcss=<DCSS NAME>”
如果未指定名称，DCSS 的默认名称为 “MONDCSS”。如果已经有其他用户连接`*MONITOR` 服务（例Performance Toolkit），monitor DCSS 已经被定义，你必须使用同一DCSS。CP 命令 Q MONITOR（E 级特权）会显monitor DCSS 的名称（如果已经定义）以及连接到 `*MONITOR` 服务的用户
关于如何在你z/VM 尚未拥有 monitor DCSS 时创建一个，请参“z/VM Performance一书（SC24-6109-00），你需E 级特权来定义并保存一DCSS
### 示例

```

	modprobe monreader mondcss=MYDCSS

```
这会加载模块并将 DCSS 名称设置“MYDCSS”
### 注意
API 没有提供用于控制 `*MONITOR` 服务的接口，例如指定要收集哪些数据。这可以通过 CP 命令 MONITOR（E 级特权）来完成，详见 “CP Command and Utility Reference”
### 使用 udev 创建设备节点
加载模块后，将创建一个字符设备以及设备节/<udev directory>/monreader
### 不使udev 创建设备节点
如果你的发行版不支持 udev，则设备节点不会被自动创建，你在加载模块后必须手动创建它。因此你需要知道该设备的主设备号和次设备号。这些号码可以在 /sys/class/misc/monreader/dev 中找到
输入 cat /sys/class/misc/monreader/dev 会给出形<major>:<minor> 的输出。设备节点可以通过 mknod 命令创建，输mknod <name> c <major> <minor>，其<name> 是要创建的设备节点的名称
### 示例

```

	# modprobe monreader
	# cat /sys/class/misc/monreader/dev
	10:63
	# mknod /dev/monreader c 10 63

```
这会以默monitor DCSS（MONDCSS）加载模块并创建一个设备节点
### 文件操作
支持以下文件操作：open、release、read、poll。读取有两种可选方式：要么配合轮询（polling）的非阻塞读取，要么不带轮询的阻塞读取。不支持 IOCTL
### 读取
从设备读取会提供一12 字节monitor 控制元素（MCE），其后跟随一组连续的一个或多个 monitor 记录（类似于 CMS 工具 MONWRITE 的输出，但不4K 控制块）。MCE 包含关于后续记录集类型（sample/event 数据）、其中包含的 monitor 域（domain），以及记录集在 monitor DCSS 中起始和结束地址的信息。起始和结束地址可用于确定记录集的大小，结束地址是最后一个数据字节的地址。起始地址需要用来正确处“end-of-frame记录（域 1，记13），即它可以用来确定相对4K 页（frame）边界的记录起始偏移量
关于 monitor 控制元素的布局，请参见 “z/VM Performance文档中的 “Appendix A: `*MONITOR`”。monitor 记录的布局可以在此处找到（z/VM 5.1）：https://www.vm.ibm.com/pubs/mon510/index.html

```

	...
	<读取0 字节>
	<第一MCE>              \
	<第一组记               |
	...                        |- 数据	<最后一MCE>             |
	<最后一组记            /
	<读取0 字节>
	...

```
在一个数据集内部可能存在多于一组的 MCE 及其对应的记录集，每个数据集的结束以一次返回值为 0（读取到 0 字节）的成功读取来标示。在成功读取完一个完整集合（包括结尾0 字节读取）之前，任何接收到的数据都必须被视为无效。因此，你应当总是在处理数据之前把完整的数据集读取进缓冲区
一个数据集的最大大小可以大monitor DCSS 的大小，因此请相应地设计缓冲区，或者使用动态内存分配。monitor DCSS 的大小会在加载模块后打印syslog。你也可以使用（E 级特权）CP 命令 Q NSS MAP 来列出所有可用的段（segment）以及关于它们的信息
与大多数字符设备一样，错误条件通过返回一个负的字节读取数来标示。在这种情况下，errno 变量指示错误条件
EIO     回复失败，读取的数据无效，应用程序应当丢弃自上次成功0 大小读取以来读取的数据EFAULT	copy_to_user 失败，读取的数据无效，应用程序应当丢弃自上次成功0 大小读取以来读取的数据EAGAIN	在非阻塞读取时，如果当前没有可用数据则发生。并没有数据缺失或损坏，只需重试，或者更好地使用轮询来进行非阻塞读取EOVERFLOW	   达到消息上限，自上次成功0 大小读取以来读取的数据是有效的，但后续的记录可能缺失
在最后一种情况（EOVERFLOW）中可能存在缺失的数据，在前两种情况（EIO、EFAULT）中则必然存在缺失的数据。应用程序可以自行决定是继续读取后续数据还是退出
### 打开
只允许一个用户打开该字符设备。如果它已经在使用中，open 函数将失败（返回负值）并将 errno 置为 EBUSY。如果无法建立到 `*MONITOR` 服务IUCV 连接，open 函数也可能失败。在这种情况errno 会被置为 EIO，并且一条带IPUSER SEVER 码的错误消息会被打印syslog。IPUSER SEVER 码在 “z/VM Performance一书的附录 A 中有描述
### 注意
一旦设备被打开，到达的消息就会被接收，并且它们会计入消息上限，即打开设备而不从中读取最终会触发 “达到消息上限错误（EOVERFLOW 错误码）