
## SCTP


## SCTP LSM 支持


### 安全钩子（Security Hooks）


```

    security_sctp_assoc_request()
    security_sctp_bind_connect()
    security_sctp_sk_clone()
    security_sctp_assoc_established()

```

这些钩子的用法在 `SCTP SELinux Support`_ 章节中以 SELinux 实现为例进行了描述。

#### security_sctp_assoc_request()


将关联（association）INIT 包的 `@asoc` 和 `@chunk->skb` 传递给安全模块。成功时返回 0，失败
时返回错误。
```

    @asoc - 指向 sctp 关联结构体的指针。
    @skb - 指向关联包 skbuff 的指针。


```

#### security_sctp_bind_connect()


基于 `@optname` 将一个或多个 ipv4/ipv6 地址传递给安全模块进行验证，其结果将是 bind 或 connect
服务，如下面的权限检查表所示。成功时返回 0，失败时返回错误。
```

    @sk      - 指向 sock 结构体的指针。
    @optname - 要验证的选项名称。
    @address - 一个或多个 ipv4 / ipv6 地址。
    @addrlen - 地址的总长度。对每个 ipv4 或 ipv6 地址使用 sizeof(struct sockaddr_in) 或
               sizeof(struct sockaddr_in6) 计算。

  ------------------------------------------------------------------
  |                     BIND 类型检查                              |
  |       @optname             |         @address 包含            |
  |----------------------------|-----------------------------------|
  | SCTP_SOCKOPT_BINDX_ADD     | 一个或多个 ipv4 / ipv6 地址       |
  | SCTP_PRIMARY_ADDR          | 单个 ipv4 或 ipv6 地址            |
  | SCTP_SET_PEER_PRIMARY_ADDR | 单个 ipv4 或 ipv6 地址            |
  ------------------------------------------------------------------

  ------------------------------------------------------------------
  |                   CONNECT 类型检查                             |
  |       @optname             |         @address 包含            |
  |----------------------------|-----------------------------------|
  | SCTP_SOCKOPT_CONNECTX      | 一个或多个 ipv4 / ipv6 地址       |
  | SCTP_PARAM_ADD_IP          | 一个或多个 ipv4 / ipv6 地址       |
  | SCTP_SENDMSG_CONNECT       | 单个 ipv4 或 ipv6 地址            |
  | SCTP_PARAM_SET_PRIMARY     | 单个 ipv4 或 ipv6 地址            |
  ------------------------------------------------------------------

```

```

    SCTP_SOCKOPT_BINDX_ADD - 允许在（可选）调用 bind(3) 之后关联额外的绑定地址。
                             sctp_bindx(3) 在套接字上添加一组绑定地址。

    SCTP_SOCKOPT_CONNECTX - 允许分配多个地址以到达对等端（多宿主，multi-homed）。
                            sctp_connectx(3) 使用多个目标地址在 SCTP 套接字上发起连接。

    SCTP_SENDMSG_CONNECT  - 发起由 sendmsg(2) 或 sctp_sendmsg(3) 在新关联上生成的连接。

    SCTP_PRIMARY_ADDR     - 设置本地主地址。

    SCTP_SET_PEER_PRIMARY_ADDR - 请求对等端将地址设为关联的主地址。

    SCTP_PARAM_ADD_IP          - 启用动态地址重配置时使用，如下所述。
    SCTP_PARAM_SET_PRIMARY     -

```

为了支持动态地址重配置，必须设置以下参数
```

    /proc/sys/net/sctp/addip_enable
    /proc/sys/net/sctp/addip_noauth_enable

```

然后，在启用动态地址重配置时，以下 **_PARAM_** 会被发送给对等端
```

          @optname                      ASCONF 参数
         ----------                    ------------------
    SCTP_SOCKOPT_BINDX_ADD     ->   SCTP_PARAM_ADD_IP
    SCTP_SET_PEER_PRIMARY_ADDR ->   SCTP_PARAM_SET_PRIMARY


```

#### security_sctp_sk_clone()


每当通过 **accept**\(2)（即 TCP 风格套接字）创建新套接字，或套接字被“剥离（peeled off）”
（例如用户空间调用 **sctp_peeloff**\(3)）时调用。
```

    @asoc - 指向当前 sctp 关联结构体的指针。
    @sk - 指向当前 sock 结构体的指针。
    @newsk - 指向新 sock 结构体的指针。


```

#### security_sctp_assoc_established()


当收到 COOKIE ACK 时调用，对等端 secid 将
```

    @asoc - 指向 sctp 关联结构体的指针。
    @skb - 指向 COOKIE ACK 包的 skbuff 的指针。


```

### 用于关联建立的安全钩子


下图展示了在建立关联时使用 `security_sctp_bind_connect()`、`security_sctp_assoc_request()`、
`security_sctp_assoc_established()` 的情况。
```

      SCTP 端点 "A"                                SCTP 端点 "Z"
      =================                                =================
    sctp_sf_do_prm_asoc()
 关联建立可由
 connect(2)、sctp_connectx(3)、
 sendmsg(2) 或 sctp_sendmsg(3) 发起。
 这些将导致对
 security_sctp_bind_connect() 的调用，以
 向 SCTP 对等端 "Z" 发起关联。
         INIT --------------------------------------------->
                                                   sctp_sf_do_5_1B_init()
                                                 响应一个 INIT 块。
                                             SCTP 对等端 "A" 正在请求
                                             一个临时关联。
                                             调用 security_sctp_assoc_request()
                                             以设置对等端标签（若是首次
                                             关联）。
                                             若不是首次关联，则检查
                                             是否允许，若允许则发送：
          <----------------------------------------------- INIT ACK
          |                                  否则记录审计事件并静默
          |                                       丢弃该包。
          |
    COOKIE ECHO ------------------------------------------>
                                                  sctp_sf_do_5_1D_ce()
                                             响应一个 COOKIE ECHO 块。
                                             确认 cookie 并创建一个
                                             永久关联。
                                             调用 security_sctp_assoc_request() 以
                                             执行与 INIT 块响应相同的操作。
          <------------------------------------------- COOKIE ACK
          |                                               |
    sctp_sf_do_5_1E_ca                                    |
 调用 security_sctp_assoc_established()                   |
 以设置对等端标签。                                       |
          |                                               |
          |                              若 SCTP_SOCKET_TCP 或被剥离的
          |                              套接字，则调用 security_sctp_sk_clone()
          |                              以克隆新套接字。
          |                                               |
      ESTABLISHED                                    ESTABLISHED
          |                                               |
    ------------------------------------------------------------------
    |                     关联已建立                                |
    ------------------------------------------------------------------


```

## SCTP SELinux 支持


### 安全钩子（Security Hooks）


上面的 `SCTP LSM Support`_ 章节描述了以下 SCTP 安全
```

    security_sctp_assoc_request()
    security_sctp_bind_connect()
    security_sctp_sk_clone()
    security_sctp_assoc_established()


```

#### security_sctp_assoc_request()


将关联 INIT 包的 `@asoc` 和 `@chunk->skb` 传递给安全模块。成功时返回 0，失败时返回错误。
```

    @asoc - 指向 sctp 关联结构体的指针。
    @skb - 指向关联包 skbuff 的指针。

```

安全模块执行以下操作：
     如果该关联是 `@asoc->base.sk` 上的第一个关联，则将 peer sid 设为 `@skb` 中的值。这将
     确保只有一个 peer sid 被分配给 `@asoc->base.sk`，它可能支持多个关联。

     否则，根据 `@skb peer sid` 验证 `@asoc->base.sk peer_sid`，以确定是否允许该关联。

     将 sctp `@asoc sid` 设为套接字的 sid（来自 `asoc->base.sk`），MLS 部分取自 `@skb peer sid`。
     这将被 SCTP TCP 风格套接字和剥离的连接使用，因为它们会生成一个新的套接字。

     如果配置了 IP 安全选项（CIPSO/CALIPSO），则在该套接字上设置 ip 选项。

#### security_sctp_bind_connect()


基于 `@optname` 检查 ipv4/ipv6 地址所需的权限
```

  ------------------------------------------------------------------
  |                   BIND 权限检查                                |
  |       @optname             |         @address 包含            |
  |----------------------------|-----------------------------------|
  | SCTP_SOCKOPT_BINDX_ADD     | 一个或多个 ipv4 / ipv6 地址       |
  | SCTP_PRIMARY_ADDR          | 单个 ipv4 或 ipv6 地址            |
  | SCTP_SET_PEER_PRIMARY_ADDR | 单个 ipv4 或 ipv6 地址            |
  ------------------------------------------------------------------

  ------------------------------------------------------------------
  |                 CONNECT 权限检查                               |
  |       @optname             |         @address 包含            |
  |----------------------------|-----------------------------------|
  | SCTP_SOCKOPT_CONNECTX      | 一个或多个 ipv4 / ipv6 地址       |
  | SCTP_PARAM_ADD_IP          | 一个或多个 ipv4 / ipv6 地址       |
  | SCTP_SENDMSG_CONNECT       | 单个 ipv4 或 ipv6 地址            |
  | SCTP_PARAM_SET_PRIMARY     | 单个 ipv4 或 ipv6 地址            |
  ------------------------------------------------------------------


```

`SCTP LSM Support`_ 给出了 `@optname` 条目的摘要，并描述了启用动态地址重配置时的 ASCONF 块
处理。

#### security_sctp_sk_clone()


每当通过 **accept**\(2)（即 TCP 风格套接字）创建新套接字，或套接字被“剥离（peeled off）”
（例如用户空间调用 **sctp_peeloff**\(3)）时调用。`security_sctp_sk_clone()` 会将新套接字的
sid 和 peer sid 分别设为 `@asoc sid` 和 `@asoc peer sid` 中包含的值。
```

    @asoc - 指向当前 sctp 关联结构体的指针。
    @sk - 指向当前 sock 结构体的指针。
    @newsk - 指向新 sock 结构体的指针。


```

#### security_sctp_assoc_established()


当收到 COOKIE ACK 时调用，此时设置连接的 peer sid
```

    @asoc - 指向 sctp 关联结构体的指针。
    @skb - 指向 COOKIE ACK 包的 skbuff 的指针。


```

### 策略语句（Policy Statements）


支持 SCTP 的以下类和权限在
```

    class sctp_socket inherits socket { node_bind }

```

```

    policycap extended_socket_class;

```

SELinux 的 SCTP 支持增加了用于连接到特定端口类型的 `name_connect` 权限，以及下文所述章节
解释的 `association` 权限。

如果用户空间工具已更新，SCTP 将支持 `portcon`
```

    portcon sctp 1024-1036 system_u:object_r:sctp_ports_t:s0


```

### SCTP 对等端标签（Peer Labeling）


一个 SCTP 套接字将只分配一个对等端标签。该标签在建立第一个关联时分配。该套接字上的任何
进一步关联，其包的对等端标签都将与套接字的对等端标签比较，只有当它们不同时，才会验证
`association` 权限。这是通过检查套接字 peer sid 与收到的包 peer sid 来确定是否允许该关联。

注意：
   1) 如果未启用对等端标签，则 peer 上下文将始终为 `SECINITSID_UNLABELED`
      （参考策略中的 `unlabeled_t`）。

   2) 由于 SCTP 在单个套接字上可以支持每个端点多个传输地址（多宿主，multi-homing），可以
      配置策略与 NetLabel 为每个传输地址提供不同的对等端标签。由于套接字 peer 标签由第一个
      关联的传输地址决定，建议所有对等端标签保持一致。

   3) **getpeercon**\(3) 可由用户空间用于检索套接字的对等端上下文。

   4) 虽然不是 SCTP 特有的，但要注意：使用 NetLabel 时，如果标签被分配给某个特定接口，而该
      接口“down 掉”，NetLabel 服务将移除该条目。因此要确保网络启动脚本调用 **netlabelctl**\(8)
      来设置所需标签（详见 **netlabel-config**\(8) 辅助脚本）。

   5) NetLabel 的 SCTP 对等端标签规则适用于 https://www.paul-moore.com/blog/t 上标签为
      "netlabel" 的这组帖子中的讨论。

   6) CIPSO 仅支持 IPv4 寻址：`socket(AF_INET, ...)`；CALIPSO 仅支持 IPv6 寻址：
      `socket(AF_INET6, ...)`

      测试 CIPSO/CALIPSO 时请注意以下几点：
         a) 如果 SCTP 包因标签无效而无法投递，CIPSO 会发送一个 ICMP 包。
         b) CALIPSO 不发送 ICMP 包，只是静默丢弃。

   7) IPSEC 不受支持，因为 RFC 3554——sctp/ipsec 支持尚未在用户空间实现（**racoon**\(8) 或
      **ipsec_pluto**\(8)），尽管内核支持 SCTP/IPSEC。
