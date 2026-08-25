
## KSMBD - SMB3 内核 Server


KSMBD 一linux 内核 server implements SMB3 协议 内核空间
用于 sharing 文件 在…上 网络.

## KSMBD architecture


The subset 性能 related 操作 belong kernelspace 
the 其他 subset belong 操作 really related 
性能 userspace. 因此, DCE/RPC 管理 具有 historically resulted
进入 一数字 缓冲overflow issues dangerous 安全 bugs 用户
account 管理 implemented 用户空间 作为 ksmbd.mountd.
文件 操作 related 性能 (打开/读取/写入/关闭 )
内核空间 (ksmbd). allows 用于 easier integration VFS
接口 用于 全部 文件 操作.

### ksmbd (内核 daemon)


the server daemon started, starts up 一forker 线程
(ksmbd/接口 name) 初始time 打开 一dedicated 端口 445
用于 listening SMB requests. Whenever clients make 一请求, the Forker
线程 accept the client 连接 fork 一线程 用于 一dedicated
communication channel 之间 the client the server. allows 用于 并行
processing SMB requests(命令) 来自 clients 以及 allowing 用于 
clients make connections. 每个 实例 named ksmbd/1~n(端口 数字)
indicate connected clients. Depending the SMB 请求 types, 每个 
线程 decide pass through the 命令 the 用户空间 (ksmbd.mountd),
currently DCE/RPC 命令 identified handled through the 用户空间.
further utilize the linux 内核, 具有 已经 chosen 进程 the 命令
作为 workitems executed the handlers the ksmbd-io kworker 线程.
allows 用于 multiplexing the handlers 作为 the 内核 takes care initiating
extra worker 线程 the 加载 increased vice versa, the 加载 
decreased destroys the extra worker 线程. 因此, 之后 the 连接 
established the client. Dedicated ksmbd/1..n(端口 数字) takes complete
ownership receiving/parsing SMB 命令. 每个 received 命令 worked
并行 i.e., 那里 多个 client 命令 worked 
并行. 之后 receiving 每个 命令 一separated 内核 workitem prepared
用于 每个 命令 further queued handled ksmbd-io kworkers.
因此, 每个 SMB workitem queued the kworkers. allows the benefit 加载
sharing managed optimally the 默认 内核 optimizing client
性能 handling client 命令 并行.

### ksmbd.mountd (用户空间 daemon)


ksmbd.mountd 一userspace 进程  transfer the 用户 account password 
registered 使用 ksmbd.adduser (part utils 用于 用户空间). Further 
allows sharing information 参数 parsed 来自 smb.conf ksmbd 
内核. 用于 the execution part 具有 一daemon continuously 运行
connected the 内核 接口 使用 netlink 套接 waits 用于 the
requests (dcerpc share/用户 info). handles RPC calls (一最少量
dozen) 大多重要 用于 文件 server 来自 NetShareEnum 
NetServerGetInfo. Complete DCE/RPC 响应 prepared 来自 the 用户空间
passed 在…上 the associated 内核 线程 用于 the client.


## KSMBD 特状


============================== =================================================
特name                   状
============================== =================================================
Dialects                       鍙楁敮鎸? SMB2.1 SMB3.0, SMB3.1.1 dialects
                               (intentionally excludes 安全 vulnerable SMB1
                               dialect).
Auto Negotiation               鍙楁敮鎸。
Compound 请求               受支
Oplock 缓存 Mechanism         受支
SMB2 leases(v1 lease)          鍙楁敮鎸。
Directory leases(v2 lease)     鍙楁敮鎸。
Multi-credits                  鍙楁敮鎸。
NTLM/NTLMv2                    鍙楁敮鎸。
HMAC-SHA256 Signing            鍙楁敮鎸。
Secure negotiate               鍙楁敮鎸。
Signing 鏇存柊                 鍙楁敮鎸。
Pre-authentication integrity   鍙楁敮鎸。
SMB3 encryption(CCM, GCM)      鍙楁敮鎸? (CCM/GCM128 鍜?CCM/GCM256 鍙楁敮鎸。
SMB direct(RDMA)               鍙楁敮鎸。
SMB3 Multi-channel             Partially 受支 Planned implement
                               replay/retry mechanisms 用于 future.
Receive Side Scaling 妯″紡      鍙楁敮鎸。
SMB3.1.1 POSIX extension       鍙楁敮鎸。
ACLs                           Partially 受支 DACLs 可用, SACLs
                               (auditing) planned 用于 the future. 用于
                               ownership (SIDs) ksmbd generates random subauth
                               然后 store disk) 使用 uid/gid
                               get 来自 inode 作为 RID 用于 本地 domain SID.
                               The 电流 acl implementation limited 
                               standalone server, 一domain member.
                               Integration Samba tools 正在 worked 
                               允许 future 支持 用于 运行作为 一domain
                               member.
Kerberos                       鍙楁敮鎸。
Durable handle v1,v2           Planned 用于 future.
Persistent handle              Planned 用于 future.
SMB2 notify                    Planned 用于 future.
Sparse 文件 支持            受支
DCE/RPC 支持                Partially 受支 一少量 calls(NetShareEnumAll,
                               NetServerGetInfo, SAMR, LSARPC) 璇，鏄?needed
                               用于 文件 server handled 通过 netlink 接口
                               来自 ksmbd.mountd. 额外 integration 
                               Samba tools 通过 upcall 正在
                               investigated 允许 支持 用于 额外
                               DCE/RPC 管理 calls (future 支持
                               用于 Witness 协议 e.g.)
ksmbd/nfsd interoperability    Planned 用于 future. The 特ksmbd
                               支持 Leases, Notify, ACLs Share modes.
SMB3.1.1 Compression           Planned 用于 future.
SMB3.1.1 在…上 QUIC             Planned 用于 future.
Signing/Encryption 在…上 RDMA   Planned 用于 future.
SMB3.1.1 GMAC signing 支持  Planned 用于 future.
============================== =================================================


## 如何 运行


1. Download ksmbd-tools(https://github.com/cifsd-team/ksmbd-tools/releases) 鍜。
   compile them.

   - 参README(https://github.com/cifsd-team/ksmbd-tools/blob/master/README.md)
     know 如何 使用 ksmbd.mountd/adduser/addshare/control utils

     $ ./autogen.sh
     $ ./configure --with-rundir=/运行
     $ make && sudo make install

2. 创建 /usr/本地/ksmbd/ksmbd.conf 文件, add SMB share ksmbd.conf 文件.

   - 参ksmbd.conf.示例 ksmbd-utils, 参见 ksmbd.conf manpage
     用于 details configure shares.

        $ man ksmbd.conf

3. 创建 用户/password 用于 SMB share.

   - 参见 ksmbd.adduser manpage.

     $ man ksmbd.adduser
     $ sudo ksmbd.adduser -一<Enter USERNAME 用于 SMB share access>

4. Insert the ksmbd.ko 模块 之后 build 您的 内核. 需加载 the 模块
   ksmbd built 进入 the 内核.

   - Set ksmbd 鍦?menuconfig(e.g. $ make menuconfig)
       [*] 网络 文件 系统  --->
           <M> SMB3 server 支持 (EXPERIMENTAL)

	$ sudo modprobe ksmbd.ko

5. 启动 ksmbd 用户空间 daemon

	$ sudo ksmbd.mountd

6. Access share 来自 Windows Linux 使用 SMB3 client (cifs.ko smbclient samba)

## Shutdown KSMBD


1. kill 用户 内核空间 daemon
	# sudo ksmbd.control -s

## 如何 turn debug print 


每个 layer
/sys/绫?ksmbd-control/debug

1. 启用 全部 component prints
	# sudo ksmbd.control -d "全部"

2. 启用 one the components (smb, auth, vfs, oplock, ipc, conn, rdma)
	# sudo ksmbd.control -d "smb"

3. 显示 什prints 已启
	# cat /sys/绫?ksmbd-control/debug
	  [smb] auth vfs oplock ipc conn [rdma]

4. 禁用 prints:
	try the selected component 一更多, 它是 已禁brackets.
