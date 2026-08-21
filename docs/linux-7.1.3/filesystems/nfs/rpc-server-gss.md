## 内核 RPC 服务器的 rpcsec_gss 支持


本文档给出用于实现内RPC 服务器（例如 NFS 服务器以NFS 客户端的 NFSv4.0
回调服务器）RPCGSS 认证所依据的标准与协议参考。（但注NFSv4.1 及更高版出于认证目的并不要求客户端充当服务器。）

RPCGSS 在若IETF 文档中进行了规范
 - RFC2203 v1: https://tools.ietf.org/rfc/rfc2203.txt
 - RFC5403 v2: https://tools.ietf.org/rfc/rfc5403.txt

还有第三个版本我们当前尚未实现：

 - RFC7861 v3: https://tools.ietf.org/rfc/rfc7861.txt

## 背景


RPCGSS 认证方法描述了一种为 NFS 执行 GSSAPI 认证的方式。尽GSSAPI 本身
完全与机制无关，但在很多情况NFS 实现仅支KRB5 机制
Linux 内核目前仅支KRB5 机制，并依赖KRB5 特有GSSAPI 扩展
GSSAPI 是一个复杂的库，在内核中完整实现它并不合理。不过，GSSAPI 操作
本质上可分为两部分：

- 初始上下文建- 完整隐私保护（对单个数据包进行签名与加密
前者更为复杂且与策略无关，但对性能不敏感。后者较为简单，但需要非常快
因此，我们在内核中执行每个数据包的完整性保护与隐私保护，而将初始上下文的建立
留给用户空间。我们需要通过 upcall 请求用户空间执行上下文建立
## NFS 鏈嶅姟鍣ㄤ紶缁?Upcall 鏈哄埗


经典 upcall 机制使用一种自定义的、基于文本的 upcall 机制，与一个名rpc.svcgssd 的自定义守护进程通信，该守护进程nfs-utils 包提供
这种 upcall 机制有两个局限：

A) 它只能处理不大于 2KiB 的令
在某Kerberos 部署中，由于附加Kerberos 票据上的各种授权扩展，GSSAPI 令牌
可能相当大，甚至超过 64KiB，这些扩展需要经GSS 层发送以完成上下文建立
B) 它无法妥善处理用户属于数千个以上组的凭据（内核当前的硬上限是 65K 个组），
   因为可发回内核的缓冲区大小（4KiB）受限
## NFS 服务器新 RPC Upcall 机制


较新upcall 机制通过 unix 套接字使RPC，与一个名gss-proxy 的守护进通信，该守护进程由名Gssproxy 的用户空间程序实现
gss_proxy RPC 协议目前记录`此处
<https://fedorahosted.org/gss-proxy/wiki/ProtocolDocumentation>`_銆。
这种 upcall 机制使用内核 rpc 客户端，并通过一个普通的 unix 套接字连接到 gssproxy
用户空间程序。gssproxy 协议不存在传统协议的尺寸限制
## 协商 Upcall 机制


为了提供向后兼容性，内核默认使用传统机制。要切换到新机制，gss-proxy 必须绑定/var/run/gssproxy.sock，然后向 /proc/net/rpc/use-gss-proxy 写入 "1"。如gss-proxy
退出，则必须重复这两个步骤
一旦选择upcall 机制，便无法更改。为了防止被锁定在传统机制中，必须在启动 nfsd
之前执行上述步骤。启nfsd 的一方可以通过读取 /proc/net/rpc/use-gss-proxy 并检其中是否包含 "1" 来保证这一点——该读取操作会阻塞，直到 gss-proxy 完成对该文件的写入