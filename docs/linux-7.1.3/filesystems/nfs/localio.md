## NFS LOCALIO


## 概述


LOCALIO 辅助 RPC 协议允许 Linux NFS 客户端与服务器可靠地握手，以确定它们是否位于同一台主机上。在 menuconfig 中选择 "NFS client and server support for LOCALIO auxiliary protocol" 以在内核配置中启用 CONFIG_NFS_LOCALIO（同时必须启用 CONFIG_NFS_FS 与 CONFIG_NFSD）。

一旦 NFS 客户端与服务器握手为 "local"（本地），客户端将在读、写和提交操作中绕过网络 RPC 协议。由于绕过了 XDR 与 RPC，这些操作会运行得更快。

LOCALIO 辅助协议的实现使用与 NFS 流量相同的连接，遵循由 NFS ACL 协议扩展所确立的模式。

需要 LOCALIO 辅助协议，才能稳健地发现与其服务器位于本地的客户端。在使用本 LOCALIO 协议之前的某个私有实现中，曾尝试基于 sockaddr 网络地址对所有本地网络接口进行脆弱的匹配。但与 LOCALIO 协议不同，基于 sockaddr 的匹配无法处理 iptables 或容器的使用。

本地客户端与服务器之间的稳健握手仅仅是个开始，这种局部性所支撑的终极用例是：客户端能够直接打开文件并向服务器发起读、写和提交，而无需经过网络。要求是尽可能高效地执行这些环回 NFS 操作，这对于容器用例（例如 kubernetes）尤为有用，因为可以在服务器本地运行 IO 任务。

LOCALIO 通过为读、写和提交绕过 XDR 与 RPC 而带来的性能优势可能极为显著，例如：

fio，时长 20 秒，directio，队列深度 8，16 个 libaio 线程：
  - With LOCALIO:
    4K read:    IOPS=979k,  BW=3825MiB/s (4011MB/s)(74.7GiB/20002msec)
    4K write:   IOPS=165k,  BW=646MiB/s  (678MB/s)(12.6GiB/20002msec)
    128K read:  IOPS=402k,  BW=49.1GiB/s (52.7GB/s)(982GiB/20002msec)
    128K write: IOPS=11.5k, BW=1433MiB/s (1503MB/s)(28.0GiB/20004msec)

  - Without LOCALIO:
    4K read:    IOPS=79.2k, BW=309MiB/s  (324MB/s)(6188MiB/20003msec)
    4K write:   IOPS=59.8k, BW=234MiB/s  (245MB/s)(4671MiB/20002msec)
    128K read:  IOPS=33.9k, BW=4234MiB/s (4440MB/s)(82.7GiB/20004msec)
    128K write: IOPS=11.5k, BW=1434MiB/s (1504MB/s)(28.0GiB/20011msec)

fio，时长 20 秒，directio，队列深度 8，1 个 libaio 线程：
  - With LOCALIO:
    4K read:    IOPS=230k,  BW=898MiB/s  (941MB/s)(17.5GiB/20001msec)
    4K write:   IOPS=22.6k, BW=88.3MiB/s (92.6MB/s)(1766MiB/20001msec)
    128K read:  IOPS=38.8k, BW=4855MiB/s (5091MB/s)(94.8GiB/20001msec)
    128K write: IOPS=11.4k, BW=1428MiB/s (1497MB/s)(27.9GiB/20001msec)

  - Without LOCALIO:
    4K read:    IOPS=77.1k, BW=301MiB/s  (316MB/s)(6022MiB/20001msec)
    4K write:   IOPS=32.8k, BW=128MiB/s  (135MB/s)(2566MiB/20001msec)
    128K read:  IOPS=24.4k, BW=3050MiB/s (3198MB/s)(59.6GiB/20001msec)
    128K write: IOPS=11.4k, BW=1430MiB/s (1500MB/s)(27.9GiB/20001msec)

## 常见问题解答


1. LOCALIO 的用例有哪些？

   a. NFS 客户端与服务器位于同一主机的工作负载可获得更高的 IO 性能。尤其是，运行容器化工作负载时，作业常常发现自己运行在与用于存储的 knfsd 服务器相同的主机上。

2. LOCALIO 有哪些要求？

   a. 尽可能绕过网络 RPC 协议的使用。这包括在 open、读、写和提交操作中绕过 XDR 与 RPC。
   b. 允许客户端与服务器自主发现彼此是否运行于本地，而无需对本地网络拓扑做任何假设。
   c. 通过兼容相关命名空间（例如 network、user、mount）来支持容器的使用。
   d. 支持所有版本的 NFS。NFSv3 尤为重要，因为它在企业中广泛使用，且 pNFS flexfiles 在数据路径上使用了它。

3. 为何 LOCALIO 在判断 NFS 客户端与服务器是否位于同一主机时，不直接比较 IP 地址或主机名？

   由于主要用例之一是容器化工作负载，我们不能假设客户端与服务器之间会共享 IP 地址。这就产生了对握手协议的需求：该协议需要走与 NFS 流量相同的连接，以确认客户端与服务器确实运行在同一主机上。握手使用一个通过线路发送的密钥（secret），如果双方确实位于同一位置，则可通过与共享内核内存中存储的值进行比较来由双方验证。

4. LOCALIO 是否会改善 pNFS flexfiles？

   是的，LOCALIO 对 pNFS flexfiles 形成补充，使其能利用 NFS 客户端与服务器的局部性。让客户端 IO 在尽可能靠近数据存储服务器的位置发起的策略，自然会受益于 LOCALIO 提供的数据路径优化。

5. 为何不开发一种新的 pNFS 布局来启用 LOCALIO？

   可以开发一种新的 pNFS 布局，但那样会把责任推给服务器：在决定发放布局时，服务器必须以某种方式发现客户端位于本地。LOCALIO 所提供的更简单方法更有价值——它让 NFS 客户端协商并利用局部性，而无需以更集中的方式对这种局部性进行更复杂的建模与发现。

6. 让客户端在不使用 RPC 的情况下执行服务端文件 OPEN 有何好处？该好处是否特定于 pNFS？

   无论是否使用 pNFS，避免为文件打开使用 XDR 与 RPC 都对性能有益。尤其是处理小文件时，最好尽可能不通过网络传输，否则可能会削弱甚至抵消"为小文件 I/O 本身避免网络传输"所带来的好处。鉴于 LOCALIO 的要求，当前让客户端在不使用 RPC 的情况下执行服务端文件打开的做法是理想的。若将来要求发生变化，我们可以相应调整。

7. 为何 LOCALIO 仅支持 UNIX 认证（AUTH_UNIX）？

   强认证通常与连接本身绑定。其原理是建立一个由服务器缓存的上下文，该上下文充当发现授权令牌的密钥，随后可被传递给 rpc.mountd 以完成认证过程。另一方面，对于 AUTH_UNIX，通过线路传递的凭据被直接用作 upcall 到 rpc.mountd 时的密钥。这简化了认证过程，因而使 AUTH_UNIX 更易于支持。

8. 对于会转换 RPC 用户 ID 的导出选项（例如 root_squash、all_squash），在 LOCALIO 操作中如何处理？

   转换用户 ID 的导出选项由 nfsd_setuser() 管理，该函数由 nfsd_setuser_and_check_port() 调用，而后者由 __fh_verify() 调用。因此它们对 LOCALIO 的处理方式与非 LOCALIO 完全相同。

9. 鉴于 NFSD 与 NFS 在不同上下文中运行，LOCALIO 如何确保对象生命周期被妥善管理？

   详见下文 "NFS 客户端与服务器互锁" 一节。

## RPC


LOCALIO 辅助 RPC 协议由单个 "UUID_IS_LOCAL" RPC 方法组成，该方法允许 Linux NFS 客户端验证本地 Linux NFS 服务器能否看到客户端生成并在 nfs_common 中提供的 nonce（一次性 UUID）。该协议并非 IETF 标准的一部分，也无此必要，因为它本质上是 Linux 对 Linux 的辅助 RPC 协议，属于实现细节。

UUID_IS_LOCAL 方法以固定 UUID_SIZE（16 字节）对客户端生成的 uuid_t 进行编码。使用固定大小的不透明（opaque）encode 与 decode XDR 方法，而非效率较低的变长方法。

NFS_LOCALIO_PROGRAM 的 RPC 程序号为 400122（由 IANA 分配，参见 https://www.iana.org/assignments/rpc-program-numbers/ ）：Linux Kernel Organization 400122 nfslocalio

```
  /* raw RFC 9562 UUID */
  #define UUID_SIZE 16
  typedef u8 uuid_t<UUID_SIZE>;

  program NFS_LOCALIO_PROGRAM {
      version LOCALIO_V1 {
          void
              NULL(void) = 0;

          void
              UUID_IS_LOCAL(uuid_t) = 1;
      } = 1;
  } = 400122;
```

LOCALIO 使用与 NFS 流量相同的传输连接。因此，LOCALIO 不会向 rpcbind 注册。

## NFS Common 与客户端/服务器握手


fs/nfs_common/nfslocalio.c 提供了若干接口，使 NFS 客户端能够生成 nonce（一次性 UUID）及关联的短生命周期 nfs_uuid_t 结构体，将其注册到 nfs_common 中，供 NFS 服务器后续查找与验证；若匹配，NFS 服务器会填充 nfs_uuid_t 结构体的成员。随后 NFS 客户端使用 nfs_common 将 nfs_uuid_t 从自身的 nfs_uuids 转移到 nfs_common 的 uuids_list 中的 nn->nfsd_serv clients_list。参见：fs/nfs/localio.c:nfs_local_probe()

nfs_common 的 nfs_uuids 列表是 LOCALIO 启用机制的基础，因此它包含指向 nfsd 内存、供客户端直接使用的成员（例如 'net' 是服务器的网络命名空间，客户端可通过它以正确的 rcu 读访问方式访问 nn->nfsd_serv）。正是这种客户端与服务器的同步，使得高级用法以及对象生命周期能够跨越从主机内核的 nfsd 到连接到运行于同一本地主机的 NFS 客户端的每容器 knfsd 实例。

## NFS 客户端与服务器互锁


LOCALIO 提供 nfs_uuid_t 对象及相关接口，以支持正确的网络命名空间（net-ns）与 NFSD 对象引用计数。

LOCALIO 需要引入并使用 NFSD 的 percpu nfsd_net_ref，以将 nfsd_shutdown_net() 与 nfsd_open_local_fh() 互锁，确保每个 net-ns 在 nfsd_open_local_fh() 使用期间不被销毁，这需要更详细的解释：

    nfsd_open_local_fh() 在打开其 nfsd_file 句柄之前会使用
    nfsd_net_try_get()，随后调用方（NFS 客户端）必须在完成其 IO 之后，
    使用 nfsd_file_put_local() 释放该 nfsd_file 及关联的 net-ns
    引用。

    该互锁机制能否正常工作，很大程度上依赖于 nfsd_open_local_fh() 是否具备安全处理如下可能性的能力：NFSD 的 net-ns（以及关联的 nfsd_net）可能已被 nfsd_destroy_serv() 经由 nfsd_shutdown_net() 销毁。

经验证，NFS 客户端与服务器的这一互锁机制修复了一个容易触发的崩溃：当容器中运行、且挂载了 LOCALIO 客户端的 NFSD 实例被关闭时会发生该崩溃。容器及相关 NFSD 重启后，由于 LOCALIO 客户端在尚未对 NFSD 的 net-ns 持有正确引用的情况下就尝试 nfsd_open_local_fh()，客户端会因 NULL 指针解引用而随之崩溃。

## 由 NFS 客户端发起 IO 而非服务器


由于 LOCALIO 着眼于通过协议绕过来实现更高的 IO 性能，必须提供传统 NFS 线路协议（SUNRPC 加 XDR）的替代方案，以访问底层文件系统。

参见 fs/nfs/localio.c:nfs_local_open_fh() 与 fs/nfsd/localio.c:nfsd_open_local_fh()，其中给出了相关接口，有选择地利用 NFS 服务器对象，使位于服务器本地的客户端无需经过网络即可打开文件指针。

客户端的 fs/nfs/localio.c:nfs_local_open_fh() 会调用服务器的 fs/nfsd/localio.c:nfsd_open_local_fh()，并以 RCU 方式谨慎访问相关的 nfsd 网络命名空间与 nn->nfsd_serv。如果 nfsd_open_local_fh() 发现客户端不再看到有效的 nfsd 对象（无论是 struct net 还是 nn->nfsd_serv），它会向 nfs_local_open_fh() 返回 -ENXIO，客户端则会通过再次调用 nfs_local_probe() 来尝试重建所需的 LOCALIO 资源。当容器中运行的 nfsd 实例在 LOCALIO 客户端连接期间重启时，就需要这种恢复。

一旦客户端持有已打开的 nfsd_file 指针，它就会直接向底层本地文件系统（通常由 nfs 服务器完成）发起读、写和提交。因此，对于这些操作，NFS 客户端是向其与 NFS 服务器共享的底层本地文件系统发起 IO。参见：fs/nfs/localio.c:nfs_local_doio() 与 fs/nfs/localio.c:nfs_local_commit()。

对于使用 RPC 向服务器发起 IO 的普通 NFS，如果应用程序使用 O_DIRECT，NFS 客户端会绕过 pagecache，但 NFS 服务器不会。NFS 服务器使用缓冲 IO，使应用程序在向 NFS 客户端发起 IO 时对对齐的要求可以宽松一些。但如果所有应用程序都正确对齐其 IO，则可通过将 'localio_O_DIRECT_semantics' nfs 模块参数设为 Y，将 LOCALIO 配置为从 NFS 客户端到其与 NFS 服务器共享的底层本地文件系统使用端到端的 O_DIRECT 语义，例如：

    echo Y > /sys/module/nfs/parameters/localio_O_DIRECT_semantics

一旦启用，这将使 LOCALIO 使用端到端的 O_DIRECT 语义（但同样，如果应用程序未正确对齐其 IO，这可能导致 IO 失败）。

## 安全性


LOCALIO 仅在使用 UNIX 风格认证（AUTH_UNIX，即 AUTH_SYS）时受支持。

我们注意确保无论使用 LOCALIO 还是常规 NFS 访问，都采用相同的 NFS 安全机制（认证等）。作为传统 NFS 客户端访问 NFS 服务器一部分而建立的 auth_domain，同样用于 LOCALIO。

就容器而言，LOCALIO 让客户端能够访问服务器拥有的网络命名空间。这是为了让客户端能够访问服务器按命名空间划分的 nfsd_net 结构体所必需的。对于传统 NFS，客户端享有同等的访问级别（尽管是通过 SUNRPC 以 NFS 协议的方式）。没有其他命名空间（user、mount 等）被从服务器改动或特意扩展到客户端。

## 模块参数


/sys/module/nfs/parameters/localio_enabled (bool)
控制是否启用 LOCALIO，默认为 Y。如果客户端与服务器位于本地，但 'localio_enabled' 设为 N，则不会使用 LOCALIO。

/sys/module/nfs/parameters/localio_O_DIRECT_semantics (bool)
控制 O_DIRECT 是否向下延伸到底层文件系统，默认为 N。应用程序 IO 必须按逻辑块大小对齐，否则 O_DIRECT 会失败。

/sys/module/nfsv3/parameters/nfs3_localio_probe_throttle (uint)
控制 NFSv3 读、写 IO 是否每 N（nfs3_localio_probe_throttle）次 IO 触发 LOCALIO 的（重新）启用，默认为 0（禁用）。必须为 2 的幂；若管理员配置不当（值过低或非 2 的幂），后果自负。

## 测试


LOCALIO 辅助协议及相关的 NFS LOCALIO 读、写和提交访问，已在各种测试场景下被证明是稳定的：

- 客户端与服务器均位于同一主机。

- 本地与远程客户端、服务器支持的启用组合的所有排列。

- 也针对不支持 LOCALIO 协议的 NFS 存储产品进行了测试。

- 客户端在主机上、服务器在容器内（v3 与 v4.2 均覆盖）。容器测试基于 podman 管理的容器，并包含成功的容器停止/重启场景。

- 将这些测试场景形式化纳入现有测试基础设施的工作正在进行中。初步的常规覆盖由 ktest 对启用了 LOCALIO 的 NFS 环回挂载配置运行 xfstests 提供，并包含 lockdep 与 KASAN 覆盖，参见：
  https://evilpiepirate.org/~testdashboard/ci?user=snitzer&branch=snitm-nfs-next
  https://github.com/koverstreet/ktest

- 已进行各种 kdevops 测试（即 "Chuck's BuildBot"），以定期验证 LOCALIO 改动未对非 LOCALIO 的 NFS 用例造成任何回归。

- Hammerspace 的各种健全性测试在启用 LOCALIO 时全部通过（其中包括大量 pNFS 与 flexfiles 测试）。
