
## NFSv4 client identifier


此 document explains 如何 the NFSv4 协议 identifies client
instances 为了 maintain 文件 打开 和 锁 状态 期间
系统 restarts. 一个 特殊 identifier 和 principal 是 maintained
在 每个 client. 这些 可 为 set 由 administrators, scripts
provided 由 site administrators, 或 tools provided 由 Linux
distributors.

存在 risks 若 一个 client's NFSv4 identifier 和 其 principal
是 不 chosen carefully.


### Introduction


The NFSv4 协议 uses "lease-based 文件 locking". Leases help
NFSv4 servers 提供 文件 锁 guarantees 和 manage 它们的
resources.

Simply put, 一个 NFSv4 server creates 一个 lease 用于 每个 NFSv4 client.
The server collects 每个 client's 文件 打开 和 锁 状态 在…下
the lease 用于 该 client.

The client 是 responsible 用于 periodically renewing 其 leases.
同时 一个 lease remains valid, the server holding 该 lease
guarantees the 文件 锁 the client 具有 已创建 remain 在 place.

若 一个 client stops renewing 其 lease (例如, 若 它 crashes),
the NFSv4 协议 allows the server 到 remove the client's 打开
和 锁 状态 之后 一个 某些 period 的 time. 当 一个 client
restarts, 它 indicates 到 servers 该 打开 和 锁 状态
associated 与 其 前一个 leases 是 无 longer valid 和 可 为
destroyed immediately.

此外, 每个 NFSv4 server manages 一个 persistent 列出 的 client
leases. 当 the server restarts 和 clients attempt 到 recover
它们的 状态, the server uses 此 列出 到 distinguish amongst
clients 该 held 状态 之前 the server restarted 和 clients
sending fresh 打开 和 锁 requests. 此 enables 文件 锁 到
persist safely across server restarts.

### NFSv4 client identifiers


每个 NFSv4 client presents 一个 identifier 到 NFSv4 servers 因此 该
它们 可 associate the client 与 其 lease. 每个 client's
identifier consists 的 two elements:

  - co_ownerid: 一个 arbitrary 但 fixed 字符串.

  - boot verifier: 一个 64-位 incarnation verifier 该 enables 一个
    server 到 distinguish successive boot epochs 的 the 相同 client.

The NFSv4.0 specification refers 到 这些 two items 作为 一个
"nfs_client_id4". The NFSv4.1 specification refers 到 这些 two
items 作为 一个 "client_owner4".

NFSv4 servers tie 此 identifier 到 the principal 和 安全
flavor 该 the client 使用 当 presenting 它. Servers 使用 此
principal 到 authorize 后续 lease modification 操作
sent 由 the client. Effectively 此 principal 是 一个 third element 的
the identifier.

作为 part 的 the identity presented 到 servers, 一个 good
"co_ownerid" 字符串 具有 若干 重要 properties:

  - The "co_ownerid" 字符串 identifies the client 期间 reboot
    recovery, 因此 the 字符串 是 persistent across client
    reboots.
  - The "co_ownerid" 字符串 helps servers distinguish the client
    来自 others, 因此 the 字符串 是 globally unique. 注意
    该 存在 无 central authority 该 assigns "co_ownerid"
    strings.
  - 因为 它 通常 appears 在 the 网络 在 the clear, the
    "co_ownerid" 字符串 执行 不 reveal 私有 information 关于
    the client itself.
  - The content 的 the "co_ownerid" 字符串 是 set 和 unchanging
    之前 the client attempts NFSv4 mounts 之后 一个 restart.
  - The NFSv4 协议 places 一个 1024-byte limit 在 the 大小 的 the
    "co_ownerid" 字符串.

### Protecting NFSv4 lease 状态


NFSv4 servers utilize the "client_owner4" 作为 描述 上文 到
assign 一个 unique lease 到 每个 client. 在…下 此 scheme, 存在
circumstances 何处 clients 可 interfere 与 每个 其他. 这是
referred 到 作为 "lease stealing".

若 distinct clients present the 相同 "co_ownerid" 字符串 和 使用
the 相同 principal (例如, AUTH_SYS 和 UID 0), 一个 server 是
unable 到 tell 该 the clients 是 不 the 相同. 每个 distinct
client presents 一个 不同 boot verifier, 因此 它 appears 到 the
server 作为 若 存在 one client 即 rebooting frequently.
两者都不 client 可 maintain 打开 或 锁 状态 在 此 scenario.

若 distinct clients present the 相同 "co_ownerid" 字符串 和 使用
distinct principals, the server 是 likely 到 允许 the 第一 client
到 operate normally 但 reject 后续 clients 与 the 相同
"co_ownerid" 字符串.

若 一个 client's "co_ownerid" 字符串 或 principal 是 不 stable,
状态 recovery 之后 一个 server 或 client reboot 是 不 guaranteed.
若 一个 client unexpectedly restarts 但 presents 一个 不同
"co_ownerid" 字符串 或 principal 到 the server, the server orphans
the client's 前一个 打开 和 锁 状态. 此 块 access 到
locked 文件 直到 the server removes the orphaned 状态.

若 the server restarts 和 一个 client presents 一个 changed "co_ownerid"
字符串 或 principal 到 the server, the server 将 不 允许 the
client 到 reclaim 其 打开 和 锁 状态, 和 可 give 那些 锁
到 其他 clients 在 the meantime. 这是 referred 到 作为 "锁
stealing".

Lease stealing 和 锁 stealing increase the potential 用于 denial
的 service 和 在 rare cases even 数据 corruption.

### Selecting 一个 appropriate client identifier


默认情况下, the Linux NFSv4 client implementation constructs 其
"co_ownerid" 字符串 starting 与 the words "Linux NFS" followed 由
the client's UTS node name (the 相同 node name, incidentally, 该
是 使用 作为 the "machine name" 在 一个 AUTH_SYS credential). 在 small
deployments, 此 construction 是 通常 adequate. 通常, 然而,
the node name 由 itself 是 不 adequately unique, 和 可 change
unexpectedly. Problematic situations 包含:

  - NFS-root (diskless) clients, 何处 the 本地 DHCP server (或
    equivalent) 执行 不 提供 一个 unique host name.

  - "Containers" 之内 一个 单个 Linux host.  若 每个 container 具有
    一个 separate 网络 namespace, 但 执行 不 使用 the UTS namespace
    到 提供 一个 unique host name, 然后 那里 可 为 多个 NFS
    client instances 与 the 相同 host name.

  - Clients across 多个 administrative domains 该 access 一个
    通用 NFS server. 若 hostnames 是 不 assigned centrally
    然后 uniqueness cannot 为 guaranteed 除非 一个 domain name 是
    included 在 the hostname.

Linux 提供 two mechanisms 到 add uniqueness 到 其 "co_ownerid"
字符串:

    nfs.nfs4_unique_id
      此 模块 参数 可 set 一个 arbitrary uniquifier 字符串
      通过 the 内核 命令 line, 或 当 the "nfs" 模块 是
      loaded.

    /sys/fs/nfs/net/nfs_client/identifier
      此 虚拟 文件, 可用 since Linux 5.3, 是 本地 到 the
      网络 namespace 在 其 它是 accessed 和 因此 可 提供
      distinction 之间 网络 namespaces (containers) 当 the
      hostname remains uniform.

注意 该 此 文件 是 empty 在 name-space creation. 若 the
container 系统 具有 access 到 一些 sort 的 per-container identity
然后 该 uniquifier 可 为 使用. 例如, 一个 uniquifier 可能
为 formed 在 boot 使用 the container's 内部 identifier:

    sha256sum /等/machine-id | awk '{print $1}' \\
        > /sys/fs/nfs/net/nfs_client/identifier

### 安全 considerations


The 使用 的 cryptographic 安全 用于 lease 管理 操作
是 strongly encouraged.

若 NFS 与 Kerberos 是 不 configured, 一个 Linux NFSv4 client uses
AUTH_SYS 和 UID 0 作为 the principal part 的 其 client identity.
此 配置 是 不 仅 insecure, 它 increases the risk 的
lease 和 锁 stealing. 然而, 它 可能 为 the 仅 choice 用于
client configurations 该 具有 无 本地 persistent storage.
"co_ownerid" 字符串 uniqueness 和 persistence 是 critical 在 此
case.

当 一个 Kerberos keytab 是 present 在 一个 Linux NFS client, the client
attempts 到 使用 one 的 the principals 在 该 keytab 当
identifying itself 到 servers. The "sec=" mount 选项 执行 不
control 此 behavior. Alternately, 一个 single-user client 与 一个
Kerberos principal 可 使用 该 principal 在 place 的 the client's
host principal.

使用 Kerberos 用于 此 purpose enables the client 和 server 到
使用 the 相同 lease 用于 操作 covered 由 全部 "sec=" 设置.
Additionally, the Linux NFS client uses the RPCSEC_GSS 安全
flavor 与 Kerberos 和 the integrity QOS 到 prevent in-transit
modification 的 lease modification requests.

### 额外 notes

The Linux NFSv4 client establishes 一个 单个 lease 在 每个 NFSv4
server 它 accesses. NFSv4 mounts 来自 一个 Linux NFSv4 client 的 一个
特定 server 然后 share 该 lease.

一旦 一个 client establishes 打开 和 锁 状态, the NFSv4 协议
enables lease 状态 到 transition 到 其他 servers, 以下 数据
该 具有 已经 migrated. 此 hides 数据 migration completely 来自
运行中 applications. The Linux NFSv4 client facilitates 状态
migration 由 presenting the 相同 "client_owner4" 到 全部 servers 它
encounters.

## 参见 也


  - nfs(5)
  - kerberos(7)
  - RFC 7530 用于 the NFSv4.0 specification
  - RFC 8881 用于 the NFSv4.1 specification.
