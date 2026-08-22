## kAFS：AFS 文件系统（AFS FILESYSTEM

 - 概述（Overview） - 用法（Usage） - 挂载点（Mountpoints） - 动态根（Dynamic root） - Proc 文件系统（Proc filesystem） - 单元数据库（The cell database） - 安全（Security） - @sys 替换（The @sys substitution）

## 概述（Overview

该文件系统提供了一个相当简单的、安全的 AFS 文件系统驱动。它仍处于开发中，尚未提供完整的功能集。它所支持的功能包括：

 (*) 安全（目前仅支持 AFS kaserver KerberosIV 票据）
 (*) 文件读写
 (*) 自动挂载（Automounting）
 (*) 本地缓存（通过 fscache）
它尚不支持以AFS 功能
 (*) pioctl() 系统调用

## 编译（Compilation

应通过打开以下内核配置项来启用该文件系统：
```
	CONFIG_AF_RXRPC		- RxRPC 协议传输
	CONFIG_RXKAD		- RxRPC Kerberos 安全处理程序
	CONFIG_AFS_FS		- AFS 文件系统
```
```
	CONFIG_AF_RXRPC_DEBUG	- 允许启用 AF_RXRPC 调试
	CONFIG_AFS_DEBUG	- 允许启用 AFS 调试
```
它们允许通过操作以下内容动态开启调试消息：
```
	/sys/module/af_rxrpc/parameters/debug
	/sys/module/kafs/parameters/debug
```

## 用法（Usage

在插入驱动模块时，必须随同指定根单元（root cell），并附带一```
	modprobe rxrpc
	modprobe kafs rootcell=cambridge.redhat.com:172.16.18.73:172.16.18.91
```
第一个模块是 AF_RXRPC 网络协议驱动。它提供 RxRPC 远程操作协议，也可以从用户空间访问。参见：

	Documentation/networking/rxrpc.rst

第二个模块是 kerberos RxRPC 安全驱动，第三个模块AFS 文件系统实际的文件系统驱动
模块加载后，可以通过如下方式添加更多模块```
	echo add grand.central.org 18.9.48.14:128.2.203.61:130.237.48.87 >/proc/fs/afs/cells
```
其中 "add" 命令的参数是单元的名称，以及该单元内一组卷位置（volume location）服务器，后者以冒号分隔
```
	mount -t afs "%cambridge.redhat.com:root.afs." /afs
	mount -t afs "#cambridge.redhat.com:root.cell." /afs/cambridge
	mount -t afs "#root.afs." /afs
	mount -t afs "#root.cell." /afs/cambridge
```
其中首字符是井号）还是百分号），取决于你究竟是想要一R/W 卷（百分号），还是更倾向 R/O 卷但愿意改用 R/W 卷（井号）
卷的名称可以加上 ".backup" ".readonly" 后缀，以指定仅连接这些类型的卷
单元的名称是可选的，如果在挂载时未给出，则会在 modprobe 时指定的单元中查找该命名卷
可以通过 /proc 添加额外的单元（见后文）

## 挂载点（Mountpoints

AFS 有挂载点（mountpoint）的概念。用 AFS 的术语说，这些是特殊格式的符号链接（与传mount 的“设备名”形式相同）。kAFS 将这些以具有 follow-link 能力（即符号链接语义）的目录形式呈现给用户。如果有人试图访问它们，它们会自动导致目标卷被挂载（如果可能）到该位置
自动挂载的文件系统将在最后一次使用后大约二十分钟被自动卸载。或者，也可以通过 umount() 系统调用直接卸载
手动卸载一AFS 卷会先剔除其上任何空闲的子挂载点。如果全部被剔除，则所请求的卷也会被卸载，否则会返回错EBUSY
管理员可以利用这一点尝试卸载整AFS 树：
```
	umount /afs
```

## 动态根（Dynamic Root

可以通过一个挂载选项创建无服务器的挂载，它仅可用
```
	mount -t afs none /afs -o dyn
```
这会创建一个挂载，其根目录只是一个空目录。试图在该目录中查找一个名称将导致创建一个挂载点```
	ls /afs/grand.central.org/
```

## Proc 文件系统（Proc Filesystem

AFS 模块创建 "/proc/fs/afs/" 目录并填充它
  (*) 一"cells" 文件，列afs 模块当前已知的单元：
```
	[root@andromeda ~]# cat /proc/fs/afs/cells
	USE NAME
	  3 cambridge.redhat.com
```
  (*) 每个单元一个目录，其中包含列出该单元内已知卷位置服务器、卷和活跃服务器的文件：
```
	[root@andromeda ~]# cat /proc/fs/afs/cambridge.redhat.com/servers
	USE ADDR            STATE
	  4 172.16.18.91        0
	[root@andromeda ~]# cat /proc/fs/afs/cambridge.redhat.com/vlservers
	ADDRESS
	172.16.18.91
	[root@andromeda ~]# cat /proc/fs/afs/cambridge.redhat.com/volumes
	USE STT VLID[0]  VLID[1]  VLID[2]  NAME
	  1 Val 20000000 20000001 20000002 root.afs
```

## 单元数据库（The Cell Database

文件系统维护一个内部数据库，记录它知道的所有单元，以及这些单元的卷位置服务器的 IP 地址。系统所属的单元modprobe 时通过 "rootcell=" 参数加入数据库；如果编译进内核，则使用内核命令行上的 "kafs.rootcell=" 参数
```
	echo add CELLNAME VLADDR[:VLADDR][:VLADDR]... >/proc/fs/afs/cells
	echo add grand.central.org 18.9.48.14:128.2.203.61:130.237.48.87 >/proc/fs/afs/cells
```
目前没有其他单元数据库操作可用

## 安全（Security

安全操作通过klog 程序获取一个密钥来发起。一个非常原始的 klog 程序位于
	https://people.redhat.com/~dhowells/rxrpc/klog.c
```
	make klog LDLIBS="-lcrypto -lcrypt -lkrb4 -lkeyutils"
```
```
	./klog
```
假设成功，这会添加一个类型为 RxRPC、以服务和单元命名的密钥，例如："afs@<cellname>"。可以用 keyctl 程序查看它：
```
	[root@andromeda ~]# keyctl show
	Session Keyring
	       -3 --alswrv      0     0  keyring: _ses.3268
		2 --alswrv      0     0   \_ keyring: _uid.0
	111416553 --als--v      0     0   \_ rxrpc: afs@CAMBRIDGE.REDHAT.COM
```
目前，用户名、域（realm）、密码和建议的票据生存期都被编译进程序中
在使AFS 功能之前获取密钥不是必需的，但如果不获取，则所有操作都将受 ACL 的匿名用户部分约束
如果获取了密钥，则拥有该密钥者发出的所AFS 操作（包括挂载和自动挂载）都将使用该密钥进行安全保护
如果一个文件用某个特定密钥打开，然后该文件描述符被传递给一个没有该密钥的进程（可能通过 AF_UNIX 套接字），那么该文件上的操作将使用打开该文件时所用的密钥进行

## @sys 替换（The @sys Substitution

当前网络命名空间的至16 @sys 替换列表可以
```
	[root@andromeda ~]# echo foo amd64_linux_26 >/proc/fs/afs/sysname
```
```
	[root@andromeda ~]# echo >/proc/fs/afs/sysname
```
```
	[root@andromeda ~]# cat /proc/fs/afs/sysname
	foo
	amd64_linux_26
```
进行 @sys 替换时，会按给定顺序尝试列表中的每个元素
默认情况下，该列表将包含一个符"<arch>_linux_26" 模式的项目，其中 amd64 x86_64 的名称