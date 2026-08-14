
## In-Kernel TLS Handshake


## Overview


Transport Layer 安全 (TLS) 是 一个 Upper Layer 协议 (ULP) 该 runs
在…上 TCP. TLS 提供 end-to-end 数据 integrity 和 confidentiality 在
addition 到 peer authentication.

The 内核's kTLS implementation handles the TLS record subprotocol, 但
执行 不 handle the TLS handshake subprotocol 其 是 使用 到 establish
一个 TLS 会话. 内核 consumers 可 使用 the API 描述 此处 到
请求 TLS 会话 establishment.

存在 若干 可能 ways 到 提供 一个 handshake service 在 the
内核. The API 描述 此处 是 designed 到 hide the details 的 那些
implementations 因此 该 in-kernel TLS consumers 执行 不 需要 到 为
aware 的 如何 the handshake gets 已完成.


## 用户 handshake agent


作为 的 此 writing, 存在 无 TLS handshake implementation 在 the
Linux 内核. 到 提供 一个 handshake service, 一个 handshake agent
(typically 在 用户空间) 是 started 在 每个 网络 namespace 何处 一个
内核 consumer 可能 需要 一个 TLS handshake. Handshake agents listen
用于 事件 sent 来自 the 内核 该 indicate 一个 handshake 请求 是
waiting.

一个 打开 套接字 是 passed 到 一个 handshake agent 通过 一个 netlink 操作,
其 creates 一个 套接字 描述符 在 the agent's 文件 描述符 表.
若 the handshake completes successfully, the handshake agent promotes
the 套接字 到 使用 the TLS ULP 和 sets the 会话 information 使用 the
SOL_TLS 套接字 选项. The handshake agent returns the 套接字 到 the
内核 通过 一个 second netlink 操作.


## 内核 Handshake API


一个 内核 TLS consumer initiates 一个 client-side TLS handshake 在 一个 打开
套接字 由 invoking one 的 the tls_client_hello() 函数. 第一, 它
fills 在 一个 结构体 该 包含 the 参数 的 the 请求:


  结构体 tls_handshake_args {
        结构体 套接字   *ta_sock;
        tls_已完成_func_t ta_已完成;
        void            *ta_数据;
        const char      *ta_peername;
        unsigned int    ta_超时_ms;
        key_串行_t    ta_keyring;
        key_串行_t    ta_my_cert;
        key_串行_t    ta_my_privkey;
        unsigned int    ta_num_peerids;
        key_串行_t    ta_my_peerids[^5^];
  };

The @ta_sock 字段 references 一个 打开 和 connected 套接字. The consumer
必须 hold 一个 参考 在 the 套接字 到 prevent 它 来自 正在 destroyed
同时 the handshake 是 在 progress. The consumer 必须 也 具有
instantiated 一个 结构体 文件 在 sock->文件.


@ta_已完成 包含 一个 回调函数 函数 即 invoked 当 the handshake
具有 completed. Further explanation 的 此 函数 是 在 the "Handshake
Completion" sesction 下文.

The consumer 可 提供 一个 NUL-terminated hostname 在 the @ta_peername
字段 即 sent 作为 part 的 ClientHello. 若 无 peername 是 provided,
the DNS hostname associated 与 the server's IP 地址 是 使用 改为.

The consumer 可 fill 在 the @ta_超时_ms 字段 到 force the servicing
handshake agent 到 exit 之后 一个 数字 的 milliseconds. 此 enables the
套接字 到 为 fully closed 一旦 两者 the 内核 和 the handshake agent
具有 closed 它们的 endpoints.

Authentication material 例如 x.509 certificates, 私有 certificate
keys, 和 pre-shared keys 是 provided 到 the handshake agent 在 keys
该 是 instantiated 由 the consumer 之前 making the handshake
请求. The consumer 可 提供 一个 私有 keyring 即 linked 进入
the handshake agent's 进程 keyring 在 the @ta_keyring 字段 到 prevent
access 的 那些 keys 由 其他 子系统.

到 请求 一个 x.509-authenticated TLS 会话, the consumer fills 在
the @ta_my_cert 和 @ta_my_privkey 字段 与 the 串行 numbers 的
keys containing 一个 x.509 certificate 和 the 私有 key 用于 该
certificate. 然后, 它 invokes 此 函数:


  ret = tls_client_hello_x509(args, gfp_标志);

The 函数 returns zero 当 the handshake 请求 是 在…下 way. 一个
zero return guarantees the 回调函数 函数 @ta_已完成 将 为 invoked
用于 此 套接字. The 函数 returns 一个 negative errno 若 the handshake
可以 不 为 started. 一个 negative errno guarantees the 回调函数 函数
@ta_已完成 将 不 为 invoked 在 此 套接字.


到 initiate 一个 client-side TLS handshake 与 一个 pre-shared key, 使用:


  ret = tls_client_hello_psk(args, gfp_标志);

然而, 在 此 case, the consumer fills 在 the @ta_my_peerids 数组
与 串行 numbers 的 keys containing the peer identities 它 wishes
到 offer, 和 the @ta_num_peerids 字段 与 the 数字 的 数组
条目 它 具有 filled 在. The 其他 字段 是 filled 在 作为 上文.


到 initiate 一个 anonymous client-side TLS handshake 使用:


  ret = tls_client_hello_anon(args, gfp_标志);

The handshake agent presents 无 peer identity information 到 the remote
期间 此 类型 的 handshake. 仅 server authentication (ie the client
verifies the server's identity) 是 performed 期间 the handshake. 从而
the established 会话 uses encryption 仅.


Consumers 该 是 in-kernel servers 使用:


  ret = tls_server_hello_x509(args, gfp_标志);

或


  ret = tls_server_hello_psk(args, gfp_标志);

The 参数 结构体 是 filled 在 作为 上文.


若 the consumer needs 到 cancel the handshake 请求, say, 由于 一个 ^C
或 其他 exigent 事件, the consumer 可 invoke:


  bool tls_handshake_cancel(sock);

此 函数 returns true 若 the handshake 请求 associated 与
@sock 具有 已经 canceled. The consumer's handshake completion 回调函数
将 不 为 invoked. 若 此 函数 returns false, 然后 the consumer's
completion 回调函数 具有 已经 已经 invoked.


## Handshake Completion


当 the handshake agent 具有 completed processing, 它 notifies the
内核 该 the 套接字 可 为 使用 由 the consumer 再次. 在 此 point,
the consumer's handshake completion 回调函数, provided 在 the @ta_已完成
字段 在 the tls_handshake_args 结构体, 是 invoked.

The synopsis 的 此 函数 是:


  typedef void	(**tls_已完成_func_t)(void **数据, int 状态,
                                   key_串行_t peerid);

The consumer 提供 一个 cookie 在 the @ta_数据 字段 的 the
tls_handshake_args 结构体 即 returned 在 the @数据 参数 的
此 回调函数. The consumer uses the cookie 到 match the 回调函数 到 the
线程 waiting 用于 the handshake 到 complete.

The success 状态 的 the handshake 是 returned 通过 the @状态
参数:

+------------+----------------------------------------------+
|  状态    |  meaning                                     |
+============+==============================================+
|  0         |  TLS 会话 established successfully        |
+------------+----------------------------------------------+
|  -EACCESS  |  Remote peer rejected the handshake 或       |
|            |  authentication failed                       |
+------------+----------------------------------------------+
|  -ENOMEM   |  Temporary resource 分配 failure       |
+------------+----------------------------------------------+
|  -EINVAL   |  Consumer provided 一个 invalid 参数       |
+------------+----------------------------------------------+
|  -ENOKEY   |  Missing authentication material             |
+------------+----------------------------------------------+
|  -EIO      |  一个 unexpected fault occurred                |
+------------+----------------------------------------------+

The @peerid 参数 包含 the 串行 数字 的 一个 key containing the
remote peer's identity 或 the 值 TLS_无_PEERID 若 the 会话 是 不
authenticated.

一个 best practice 是 到 关闭 和 destroy the 套接字 immediately 若 the
handshake failed.


### 其他 considerations


同时 一个 handshake 是 在…下 way, the 内核 consumer 必须 alter the
套接字's sk_数据_ready 回调函数 函数 到 ignore 全部 incoming 数据.
一旦 the handshake completion 回调函数 函数 具有 已经 invoked, 正常
receive 操作 可 为 resumed.

一旦 一个 TLS 会话 是 established, the consumer 必须 提供 一个 缓冲区
用于 和 然后 examine the control message (CMSG) 即 part 的 every
后续 sock_recvmsg(). 每个 control message indicates 是否 the
received message 数据 是 TLS record 数据 或 会话 metadata.

参见 tls.rst 用于 details 在 如何 一个 kTLS consumer recognizes incoming
(decrypted) 应用程序 数据, alerts, 和 handshake packets 一旦 the
套接字 具有 已经 promoted 到 使用 the TLS ULP.
