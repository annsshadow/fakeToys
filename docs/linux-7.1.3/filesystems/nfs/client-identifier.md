
## NFSv4 client identifier


document explains 如何 the NFSv4 协议 identifies client
instances 为了 maintain 文件 打开 状期间
系统 restarts. 一特殊 identifier principal maintained
每个 client. 这些 set administrators, scripts
provided 鐢?site administrators, 鎴?tools provided 鐢?Linux
distributors.

存在 risks 一client's NFSv4 identifier principal
鏄，涓?chosen carefully.


### Introduction


The NFSv4 协议 uses "lease-based 文件 locking". Leases help
NFSv4 servers 提供 文件 guarantees manage 它们
resources.

Simply put, 一NFSv4 server creates 一lease 用于 每个 NFSv4 client.
The server collects 每个 client's 文件 打开 状在…下
the lease 用于 client.

The client responsible 用于 periodically renewing leases.
同时 一lease remains valid, the server holding lease
guarantees the 文件 the client 具有 已创remain place.

一client stops renewing lease (例如, crashes),
the NFSv4 协议 allows the server remove the client's 打开
状之后 一某些 period time. 一client
restarts, indicates servers 打开 状
associated 前一leases longer valid 
destroyed immediately.

此外, 每个 NFSv4 server manages 一persistent 列出 client
leases. 褰?the server restarts 鍜?clients attempt 鍒?recover
它们状 the server uses 列出 distinguish amongst
clients held 状之前 the server restarted clients
sending fresh 打开 requests. enables 文件 
persist safely across server restarts.

### NFSv4 client identifiers


每个 NFSv4 client presents 一identifier NFSv4 servers 因此 
它们 associate the client lease. 每个 client's
identifier consists 鐨?two elements:

  - co_ownerid: 一arbitrary fixed 字符

  - boot verifier: 一64-incarnation verifier enables 一
    server distinguish successive boot epochs the 相同 client.

The NFSv4.0 specification refers 这些 two items 作为 一
"nfs_client_id4". The NFSv4.1 specification refers 这些 two
items 作为 一"client_owner4".

NFSv4 servers tie identifier the principal 安全
flavor the client 使用 presenting  Servers 使用 
principal authorize 后续 lease modification 操作
sent the client. Effectively principal 一third element 
the identifier.

作为 part the identity presented servers, 一good
"co_ownerid" 字符具有 若干 重要 properties:

  - The "co_ownerid" 字符identifies the client 期间 reboot
    recovery, 因此 the 字符persistent across client
    reboots.
  - The "co_ownerid" 字符helps servers distinguish the client
    来自 others, 因此 the 字符globally unique. 注意
    存在 central authority assigns "co_ownerid"
    strings.
  - 因为 通常 appears the 网络 the clear, the
    "co_ownerid" 字符执行 reveal 私有 information 关于
    the client itself.
  - The content the "co_ownerid" 字符set unchanging
    之前 the client attempts NFSv4 mounts 之后 一restart.
  - The NFSv4 协议 places 一1024-byte limit the 大小 the
    "co_ownerid" 字符

### Protecting NFSv4 lease 状


NFSv4 servers utilize the "client_owner4" 作为 描述 上文 
assign 一unique lease 每个 client. 在…下 scheme, 存在
circumstances 何处 clients interfere 每个 其他. 这是
referred 作为 "lease stealing".

distinct clients present the 相同 "co_ownerid" 字符使用
the 相同 principal (例如, AUTH_SYS UID 0), 一server 
unable tell the clients the 相同. 每个 distinct
client presents 一不同 boot verifier, 因此 appears the
server 作为 存在 one client rebooting frequently.
两者都client maintain 打开 状scenario.

distinct clients present the 相同 "co_ownerid" 字符使用
distinct principals, the server likely 允许 the 第一 client
operate normally reject 后续 clients the 相同
"co_ownerid" 字符

一client's "co_ownerid" 字符principal stable,
状recovery 之后 一server client reboot guaranteed.
一client unexpectedly restarts presents 一不同
"co_ownerid" 字符principal the server, the server orphans
the client's 前一打开 状 access 
locked 文件 直到 the server removes the orphaned 状

the server restarts 一client presents 一changed "co_ownerid"
字符principal the server, the server 允许 the
client reclaim 打开 状 give 那些 
其他 clients the meantime. 这是 referred 作为 "
stealing".

Lease stealing stealing increase the potential 用于 denial
service rare cases even 数据 corruption.

### Selecting 一appropriate client identifier


默认情况 the Linux NFSv4 client implementation constructs 
"co_ownerid" 字符starting the words "Linux NFS" followed 
the client's UTS node name (the 相同 node name, incidentally, 
使用 作为 the "machine name" 一AUTH_SYS credential). small
deployments, construction 通常 adequate. 通常, 然
the node name 鐢?itself 鏄，涓?adequately unique, 鍜，鍙?change
unexpectedly. Problematic situations 包含:

  - NFS-root (diskless) clients, 何处 the 本地 DHCP server (
    equivalent) 执行 提供 一unique host name.

  - "Containers" 之内 一单个 Linux host.  每个 container 具有
    一separate 网络 namespace, 执行 使用 the UTS namespace
    提供 一unique host name, 然后 那里 多个 NFS
    client instances the 相同 host name.

  - Clients across 多个 administrative domains access 一
    通用 NFS server. hostnames assigned centrally
    然后 uniqueness cannot guaranteed 除非 一domain name 
    included 鍦?the hostname.

Linux 提供 two mechanisms add uniqueness "co_ownerid"
字符

    nfs.nfs4_unique_id
      模块 参数 set 一arbitrary uniquifier 字符
      通过 the 内核 命令 line, the "nfs" 模块 
      loaded.

    /sys/fs/nfs/net/nfs_client/identifier
      虚拟 文件, 可用 since Linux 5.3, 本地 the
      网络 namespace 它是 accessed 因此 提供
      distinction 涔嬮棿 缃戠粶 namespaces (containers) 褰?the
      hostname remains uniform.

注意 文件 empty name-space creation. the
container 系统 具有 access 一sort per-container identity
然后 uniquifier 使用. 例如, 一uniquifier 可能
formed boot 使用 the container's 内部 identifier:

    sha256sum /绛?machine-id | awk '{print $1}' \\
        > /sys/fs/nfs/net/nfs_client/identifier

### 安全 considerations


The 使用 cryptographic 安全 用于 lease 管理 操作
鏄?strongly encouraged.

NFS Kerberos configured, 一Linux NFSv4 client uses
AUTH_SYS UID 0 作为 the principal part client identity.
配置 insecure, increases the risk 
lease stealing. 然 可能 the choice 用于
client configurations 具有 本地 persistent storage.
"co_ownerid" 字符uniqueness persistence critical 
case.

一Kerberos keytab present 一Linux NFS client, the client
attempts 使用 one the principals keytab 
identifying itself servers. The "sec=" mount 选项 执行 
control behavior. Alternately, 一single-user client 一
Kerberos principal 使用 principal place the client's
host principal.

使用 Kerberos 用于 purpose enables the client server 
使用 the 相同 lease 用于 操作 covered 全部 "sec=" 设置.
Additionally, the Linux NFS client uses the RPCSEC_GSS 安全
flavor 涓?Kerberos 鍜?the integrity QOS 鍒?prevent in-transit
modification 鐨?lease modification requests.

### 额外 notes

The Linux NFSv4 client establishes 一单个 lease 每个 NFSv4
server accesses. NFSv4 mounts 来自 一Linux NFSv4 client 一
特定 server 然后 share lease.

一一client establishes 打开 状 the NFSv4 协议
enables lease 状transition 其他 servers, 以下 数据
具有 已经 migrated. hides 数据 migration completely 来自
运行applications. The Linux NFSv4 client facilitates 状
migration presenting the 相同 "client_owner4" 全部 servers 
encounters.

## 参见 


  - nfs(5)
  - kerberos(7)
  - RFC 7530 用于 the NFSv4.0 specification
  - RFC 8881 用于 the NFSv4.1 specification.
