
## KSMBD - SMB3 内核 Server


KSMBD 是 一个 linux 内核 server 其 implements SMB3 协议 在 内核空间
用于 sharing 文件 在…上 网络.

## KSMBD architecture


The subset 的 性能 related 操作 belong 在 kernelspace 和
the 其他 subset 其 belong 到 操作 其 是 不 really related 与
性能 在 userspace. 因此, DCE/RPC 管理 该 具有 historically resulted
进入 一个 数字 的 缓冲区 overflow issues 和 dangerous 安全 bugs 和 用户
account 管理 是 implemented 在 用户空间 作为 ksmbd.mountd.
文件 操作 该 是 related 与 性能 (打开/读取/写入/关闭 等.)
在 内核空间 (ksmbd). 此 也 allows 用于 easier integration 与 VFS
接口 用于 全部 文件 操作.

### ksmbd (内核 daemon)


当 the server daemon 是 started, 它 starts up 一个 forker 线程
(ksmbd/接口 name) 在 初始化 time 和 打开 一个 dedicated 端口 445
用于 listening 到 SMB requests. Whenever 新 clients make 一个 请求, the Forker
线程 将 accept the client 连接 和 fork 一个 新 线程 用于 一个 dedicated
communication channel 之间 the client 和 the server. 它 allows 用于 并行
processing 的 SMB requests(命令) 来自 clients 以及 allowing 用于 新
clients 到 make 新 connections. 每个 实例 是 named ksmbd/1~n(端口 数字)
到 indicate connected clients. Depending 在 the SMB 请求 types, 每个 新
线程 可 decide 到 pass through the 命令 到 the 用户空间 (ksmbd.mountd),
currently DCE/RPC 命令 是 identified 到 为 handled through the 用户空间.
到 further utilize the linux 内核, 它 具有 已经 chosen 到 进程 the 命令
作为 workitems 和 到 为 executed 在 the handlers 的 the ksmbd-io kworker 线程.
它 allows 用于 multiplexing 的 the handlers 作为 the 内核 takes care 的 initiating
extra worker 线程 若 the 加载 是 increased 和 vice versa, 若 the 加载 是
decreased 它 destroys the extra worker 线程. 因此, 之后 the 连接 是
established 与 the client. Dedicated ksmbd/1..n(端口 数字) takes complete
ownership 的 receiving/parsing 的 SMB 命令. 每个 received 命令 是 worked
在 并行 i.e., 那里 可 为 多个 client 命令 其 是 worked 在
并行. 之后 receiving 每个 命令 一个 separated 内核 workitem 是 prepared
用于 每个 命令 其 是 further queued 到 为 handled 由 ksmbd-io kworkers.
因此, 每个 SMB workitem 是 queued 到 the kworkers. 此 allows the benefit 的 加载
sharing 到 为 managed optimally 由 the 默认 内核 和 optimizing client
性能 由 handling client 命令 在 并行.

### ksmbd.mountd (用户空间 daemon)


ksmbd.mountd 是 一个 userspace 进程 到, transfer the 用户 account 和 password 该
是 registered 使用 ksmbd.adduser (part 的 utils 用于 用户空间). Further 它
allows sharing information 参数 该 是 parsed 来自 smb.conf 到 ksmbd 在
内核. 用于 the execution part 它 具有 一个 daemon 其 是 continuously 运行中
和 connected 到 the 内核 接口 使用 netlink 套接字, 它 waits 用于 the
requests (dcerpc 和 share/用户 info). 它 handles RPC calls (在 一个 最小 少量
dozen) 该 是 大多数 重要 用于 文件 server 来自 NetShareEnum 和
NetServerGetInfo. Complete DCE/RPC 响应 是 prepared 来自 the 用户空间
和 passed 在…上 到 the associated 内核 线程 用于 the client.


## KSMBD 特性 状态


============================== =================================================
特性 name                   状态
============================== =================================================
Dialects                       受支持. SMB2.1 SMB3.0, SMB3.1.1 dialects
                               (intentionally excludes 安全 vulnerable SMB1
                               dialect).
Auto Negotiation               受支持.
Compound 请求               受支持.
Oplock 缓存 Mechanism         受支持.
SMB2 leases(v1 lease)          受支持.
Directory leases(v2 lease)     受支持.
Multi-credits                  受支持.
NTLM/NTLMv2                    受支持.
HMAC-SHA256 Signing            受支持.
Secure negotiate               受支持.
Signing 更新                 受支持.
Pre-authentication integrity   受支持.
SMB3 encryption(CCM, GCM)      受支持. (CCM/GCM128 和 CCM/GCM256 受支持)
SMB direct(RDMA)               受支持.
SMB3 Multi-channel             Partially 受支持. Planned 到 implement
                               replay/retry mechanisms 用于 future.
Receive Side Scaling 模式      受支持.
SMB3.1.1 POSIX extension       受支持.
ACLs                           Partially 受支持. 仅 DACLs 可用, SACLs
                               (auditing) 是 planned 用于 the future. 用于
                               ownership (SIDs) ksmbd generates random subauth
                               值(然后 store 它 到 disk) 和 使用 uid/gid
                               get 来自 inode 作为 RID 用于 本地 domain SID.
                               The 电流 acl implementation 是 limited 到
                               standalone server, 不 一个 domain member.
                               Integration 与 Samba tools 是 正在 worked 在
                               到 允许 future 支持 用于 运行中 作为 一个 domain
                               member.
Kerberos                       受支持.
Durable handle v1,v2           Planned 用于 future.
Persistent handle              Planned 用于 future.
SMB2 notify                    Planned 用于 future.
Sparse 文件 支持            受支持.
DCE/RPC 支持                Partially 受支持. 一个 少量 calls(NetShareEnumAll,
                               NetServerGetInfo, SAMR, LSARPC) 该 是 needed
                               用于 文件 server handled 通过 netlink 接口
                               来自 ksmbd.mountd. 额外 integration 与
                               Samba tools 和 库 通过 upcall 是 正在
                               investigated 到 允许 支持 用于 额外
                               DCE/RPC 管理 calls (和 future 支持
                               用于 Witness 协议 e.g.)
ksmbd/nfsd interoperability    Planned 用于 future. The 特性 该 ksmbd
                               支持 是 Leases, Notify, ACLs 和 Share modes.
SMB3.1.1 Compression           Planned 用于 future.
SMB3.1.1 在…上 QUIC             Planned 用于 future.
Signing/Encryption 在…上 RDMA   Planned 用于 future.
SMB3.1.1 GMAC signing 支持  Planned 用于 future.
============================== =================================================


## 如何 到 运行


1. Download ksmbd-tools(https://github.com/cifsd-team/ksmbd-tools/releases) 和
   compile them.

   - 参考 到 README(https://github.com/cifsd-team/ksmbd-tools/blob/master/README.md)
     到 know 如何 到 使用 ksmbd.mountd/adduser/addshare/control utils

     $ ./autogen.sh
     $ ./configure --with-rundir=/运行
     $ make && sudo make install

2. 创建 /usr/本地/等/ksmbd/ksmbd.conf 文件, add SMB share 在 ksmbd.conf 文件.

   - 参考 到 ksmbd.conf.示例 在 ksmbd-utils, 参见 ksmbd.conf manpage
     用于 details 到 configure shares.

        $ man ksmbd.conf

3. 创建 用户/password 用于 SMB share.

   - 参见 ksmbd.adduser manpage.

     $ man ksmbd.adduser
     $ sudo ksmbd.adduser -一个 <Enter USERNAME 用于 SMB share access>

4. Insert the ksmbd.ko 模块 之后 您 build 您的 内核. 无 需要 到 加载 the 模块
   若 ksmbd 是 built 进入 the 内核.

   - Set ksmbd 在 menuconfig(e.g. $ make menuconfig)
       [*] 网络 文件 系统  --->
           <M> SMB3 server 支持 (EXPERIMENTAL)

	$ sudo modprobe ksmbd.ko

5. 启动 ksmbd 用户空间 daemon

	$ sudo ksmbd.mountd

6. Access share 来自 Windows 或 Linux 使用 SMB3 client (cifs.ko 或 smbclient 的 samba)

## Shutdown KSMBD


1. kill 用户 和 内核空间 daemon
	# sudo ksmbd.control -s

## 如何 到 turn debug print 在


每个 layer
/sys/类/ksmbd-control/debug

1. 启用 全部 component prints
	# sudo ksmbd.control -d "全部"

2. 启用 one 的 the components (smb, auth, vfs, oplock, ipc, conn, rdma)
	# sudo ksmbd.control -d "smb"

3. 显示 什么 prints 是 已启用.
	# cat /sys/类/ksmbd-control/debug
	  [smb] auth vfs oplock ipc conn [rdma]

4. 禁用 prints:
	若 您 try the selected component 一旦 更多, 它是 已禁用 无 brackets.
