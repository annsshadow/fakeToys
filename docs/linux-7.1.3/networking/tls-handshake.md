
## In-Kernel TLS Handshake


## Overview


Transport Layer 安全 (TLS) 一Upper Layer 协议 (ULP) runs
在…上 TCP. TLS 提供 end-to-end 数据 integrity confidentiality 
addition 鍒?peer authentication.

The 内核's kTLS implementation handles the TLS record subprotocol, 
执行 handle the TLS handshake subprotocol 使用 establish
一TLS 会话. 内核 consumers 使用 the API 描述 此处 
请求 TLS 会话 establishment.

存在 若干 可能 ways 提供 一handshake service the
内核. The API 描述 此处 designed hide the details 那些
implementations 因此 in-kernel TLS consumers 执行 需
aware 如何 the handshake gets 已完


## 用户 handshake agent


作为 writing, 存在 TLS handshake implementation the
Linux 内核. 提供 一handshake service, 一handshake agent
(typically 用户空间) started 每个 网络 namespace 何处 一
内核 consumer 可能 需一TLS handshake. Handshake agents listen
用于 事件 sent 来自 the 内核 indicate 一handshake 请求 
waiting.

一打开 套接passed 一handshake agent 通过 一netlink 操作,
creates 一套接描述the agent's 文件 描述
鑻?the handshake completes successfully, the handshake agent promotes
the 套接使用 the TLS ULP sets the 会话 information 使用 the
SOL_TLS 套接选项. The handshake agent returns the 套接the
内核 通过 一second netlink 操作.


## 内核 Handshake API


一内核 TLS consumer initiates 一client-side TLS handshake 一打开
套接invoking one the tls_client_hello() 函数. 第一, 
fills 一结构包含 the 参数 the 请求:


  结构tls_handshake_args {
        结构套接  *ta_sock;
        tls_已完成_func_t ta_已完
        void            *ta_数据;
        const char      *ta_peername;
        unsigned int    ta_超时_ms;
        key_串行_t    ta_keyring;
        key_串行_t    ta_my_cert;
        key_串行_t    ta_my_privkey;
        unsigned int    ta_num_peerids;
        key_串行_t    ta_my_peerids[^5^];
  };

The @ta_sock 字段 references 一打开 connected 套接 The consumer
必须 hold 一参the 套接prevent 来自 正在 destroyed
同时 the handshake progress. The consumer 必须 具有
instantiated 一结构文件 sock->文件.


@ta_已完包含 一回调函数 函数 invoked the handshake
具有 completed. Further explanation 函数 the "Handshake
Completion" sesction 下文.

The consumer 提供 一NUL-terminated hostname the @ta_peername
字段 sent 作为 part ClientHello. peername provided,
the DNS hostname associated the server's IP 地址 使用 改为.

The consumer fill the @ta_超时_ms 字段 force the servicing
handshake agent exit 之后 一数字 milliseconds. enables the
套接fully closed 一两the 内核 the handshake agent
具有 closed 它们endpoints.

Authentication material 例如 x.509 certificates, 私有 certificate
keys, 鍜?pre-shared keys 鏄?provided 鍒?the handshake agent 鍦?keys
instantiated the consumer 之前 making the handshake
请求. The consumer 提供 一私有 keyring linked 进入
the handshake agent's 进程 keyring the @ta_keyring 字段 prevent
access 那些 keys 其他 子系

请求 一x.509-authenticated TLS 会话, the consumer fills 
the @ta_my_cert @ta_my_privkey 字段 the 串行 numbers 
keys containing 一x.509 certificate the 私有 key 用于 
certificate. 然后, invokes 函数:


  ret = tls_client_hello_x509(args, gfp_标志);

The 函数 returns zero the handshake 请求 在…下 way. 一
zero return guarantees the 回调函数 函数 @ta_已完invoked
用于 套接 The 函数 returns 一negative errno the handshake
可以 started. 一negative errno guarantees the 回调函数 函数
@ta_已完invoked 套接


initiate 一client-side TLS handshake 一pre-shared key, 使用:


  ret = tls_client_hello_psk(args, gfp_标志);

然 case, the consumer fills the @ta_my_peerids 数组
串行 numbers keys containing the peer identities wishes
offer, the @ta_num_peerids 字段 the 数字 数组
条目 具有 filled  The 其他 字段 filled 作为 上文.


initiate 一anonymous client-side TLS handshake 使用:


  ret = tls_client_hello_anon(args, gfp_标志);

The handshake agent presents 鏃?peer identity information 鍒?the remote
期间 类型 handshake. server authentication (ie the client
verifies the server's identity) performed 期间 the handshake. 从
the established 会话 uses encryption 


Consumers in-kernel servers 使用:


  ret = tls_server_hello_x509(args, gfp_标志);

鎴。


  ret = tls_server_hello_psk(args, gfp_标志);

The 参数 结构filled 作为 上文.


the consumer needs cancel the handshake 请求, say, 由于 一^C
其他 exigent 事件, the consumer invoke:


  bool tls_handshake_cancel(sock);

函数 returns true the handshake 请求 associated 
@sock 具有 已经 canceled. The consumer's handshake completion 回调函数
invoked. 函数 returns false, 然后 the consumer's
completion 回调函数 具有 已经 已经 invoked.


## Handshake Completion


褰?the handshake agent 鍏锋湁 completed processing, 瀹?notifies the
内核 the 套接使用 the consumer 再次. point,
the consumer's handshake completion 回调函数, provided the @ta_已完
字段 the tls_handshake_args 结构 invoked.

The synopsis 函数 


  typedef void	(**tls_已完成_func_t)(void **数据, int 状
                                   key_串行_t peerid);

The consumer 提供 一cookie the @ta_数据 字段 the
tls_handshake_args 结构returned the @数据 参数 
回调函数. The consumer uses the cookie match the 回调函数 the
线程 waiting 用于 the handshake complete.

The success 状the handshake returned 通过 the @状
参数:

+------------+----------------------------------------------+
|  状   |  meaning                                     |
+============+==============================================+
|  0         |  TLS 会话 established successfully        |
+------------+----------------------------------------------+
|  -EACCESS  |  Remote peer rejected the handshake 鎴?      |
|            |  authentication failed                       |
+------------+----------------------------------------------+
|  -ENOMEM   |  Temporary resource 分配 failure       |
+------------+----------------------------------------------+
|  -EINVAL   |  Consumer provided 一invalid 参数       |
+------------+----------------------------------------------+
|  -ENOKEY   |  Missing authentication material             |
+------------+----------------------------------------------+
|  -EIO      |  一unexpected fault occurred                |
+------------+----------------------------------------------+

The @peerid 参数 包含 the 串行 数字 一key containing the
remote peer's identity the TLS_无_PEERID the 会话 
authenticated.

一best practice 关闭 destroy the 套接immediately the
handshake failed.


### 其他 considerations


同时 一handshake 在…下 way, the 内核 consumer 必须 alter the
套接s sk_数据_ready 回调函数 函数 ignore 全部 incoming 数据.
一the handshake completion 回调函数 函数 具有 已经 invoked, 正常
receive 操作 resumed.

一一TLS 会话 established, the consumer 必须 提供 一缓冲
用于 然后 examine the control message (CMSG) part every
后续 sock_recvmsg(). 每个 control message indicates 是否 the
received message 数据 TLS record 数据 会话 metadata.

参见 tls.rst 用于 details 如何 一kTLS consumer recognizes incoming
(decrypted) 应用程序 数据, alerts, handshake packets 一the
套接具有 已经 promoted 使用 the TLS ULP.
