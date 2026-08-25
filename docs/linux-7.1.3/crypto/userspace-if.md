## 用户空间接口


### 简

内核加密 API 中可见于内核空间的概念同样完全适用于用户空间接口。因此，针对内核内使用场景的
内核加密 API 高层讨论在此同样适用
然而，主要的区别在于：用户空间只能作为转换或密码算法的消费者（consumer），而永远不能是提供者（provider）
以下内容涵盖了内核加API 导出的用户空间接口。对此描述的一个可工作示例libkcapi，可[^1^]
获取。该库可被需要内核提供加密服务的用户空间应用程序使用
不过，内核内内核加密 API 的某些方面并不适用于用户空间。这包括同步与异步调用之间的区别。用户空API 调用是完全同步的
[^1^] https://www.chronox.de/libkcapi/index.html

### 用户空间 API 总体说明


内核加密 API 可从用户空间访问。当前，可访问以下密码：

- 消息摘要，包括带密钥的消息摘要（HMAC、CMAC
- 对称密码

- AEAD 密码

- 随机数生成器

该接口通过 socket 类型提供，使AF_ALG 类型。此外，setsockopt 选项类型SOL_ALG。如果用户空头文件尚未导出这些标志，请使用以下宏
```

    #ifndef AF_ALG
    #define AF_ALG 38
    #endif
    #ifndef SOL_ALG
    #define SOL_ALG 279
    #endif


```
密码使用与内核内 API 调用相同的名称来访问。这包括密码的通用名与唯一名命名方案，以及对通用优先级的强制要求
要与内核加密 API 交互，用户空间应用程序必须创建一socket。用户空间使send()/write() 系统调用
族来调用密码操作。密码操作的结果通过 read()/recv() 系统调用族获取
以下 API 调用假设 socket 描述符已由用户空间应用程序打开，并且只讨论内核加密 API 特定的调用
要初始化 socket 接口，消费者必须执行以下顺序：

1. 使用下面针对不同密码类型指定struct sockaddr_alg 参数，创建类型为 AF_ALG socket
2. 使用socket 描述符调bind
3. 使用socket 描述符调accept。accept 系统调用返回一个新文件描述符，用于与该特定密码实例
   交互。当调用 send/write recv/read 系统调用来向内核发送数据或从内核获取数据时必须使用 accept
   返回的文件描述符
### 就地（in-place）密码操

就像内核加密 API 的内核内操作一样，用户空间接口允许就地执行密码操作。这意味着 send/write 系统
调用使用的输入缓冲区read/recv 系统调用使用的输出缓冲区可以是同一个。这对于对称密码操作尤其
有意义，因为这样可以避免将输出数据复制到其最终目的地
另一方面，如果消费者希望将明文和密文保存在不同的内存位置，消费者只需为加密和解密操作提供不同内存指针即可
### 消息摘要 API


用于密码操作的消息摘要类型在调用 bind 系统调用时选择。bind 要求调用者提供一个已填充struct sockaddr 数据结构。该数据结构必须按如下方式填充：

```

    struct sockaddr_alg sa = {
        .salg_family = AF_ALG,
        .salg_type = "hash", /* this selects the hash logic in the kernel */
        .salg_name = "sha1" /* this is the cipher name */
    };


```
salg_type "hash" 适用于消息摘要和带密钥的消息摘要。不过，带密钥的消息摘要通过其相应的
salg_name 引用。有关如何为带密钥的消息摘要设置密钥，请参阅下面解释 setsockopt 接口的说明
使用 send() 系统调用，应用程序提供应当用消息摘要处理的数据。send 系统调用允许指定以下标志
- MSG_MORE：如果设置了此标志，send 系统调用表现得像消息摘要更新函数，此时尚未计算最终的哈希  如果未设置该标志，send 系统调用会立即计算最终的消息摘要
使用 recv() 系统调用，应用程序可以从内核加密 API 读取消息摘要。如果缓冲区对于消息摘要来说太小内核会设MSG_TRUNC 标志
为了设置消息摘要密钥，调用应用程序必须使ALG_SET_KEY ALG_SET_KEY_BY_KEY_SERIAL setsockopt()
选项。如果没有设置密钥，HMAC 操作会在没有密钥引起的初HMAC 状态变更的情况下执行
### 对称密码 API


该操作与消息摘要的讨论非常相似。在初始化期间，struct sockaddr 数据结构必须按如下方式填充：

```

    struct sockaddr_alg sa = {
        .salg_family = AF_ALG,
        .salg_type = "skcipher", /* this selects the symmetric cipher */
        .salg_name = "cbc(aes)" /* this is the cipher name */
    };


```
在使write/send 系统调用族将数据发送到内核之前，消费者必须设置密钥。密钥的设置通过下面setsockopt 调用来描述
使用 sendmsg() 系统调用，应用程序提供应当进行加密或解密处理的数据。此外，IV 通过 sendmsg() 系统
调用提供的数据结构来指定
sendmsg 系统调用struct msghdr 参数被嵌入到 struct cmsghdr 数据结构中。有cmsghdr 数据结构
如何send/recv 系统调用族一起使用，请参recv(2) cmsg(3)。该 cmsghdr 数据结构包含通过单独头实例指定的以下信息
- 使用以下标志之一指定密码操作类型
   - ALG_OP_ENCRYPT - 数据加密

   - ALG_OP_DECRYPT - 数据解密

- 使用标志 ALG_SET_IV 标记IV 信息规范

send 系统调用族允许指定以下标志：

- MSG_MORE：如果设置了此标志，send 系统调用表现得像密码更新函数，期望随后的 send 系统调用提供更多
  输入数据
注意：内核对任何意外数据报告 -EINVAL。调用者必须确保全部数据符/proc/crypto 中针对所选密码给的约束
使用 recv() 系统调用，应用程序可以从内核加密 API 读取密码操作的结果。输出缓冲区必须至少大到足以
容纳加密或解密数据的所有块。如果输出数据大小较小，则只会返回能放入该输出缓冲区大小的块数
### AEAD 密码 API


该操作与对称密码的讨论非常相似。在初始化期间，struct sockaddr 数据结构必须按如下方式填充：

```

    struct sockaddr_alg sa = {
        .salg_family = AF_ALG,
        .salg_type = "aead", /* this selects the symmetric cipher */
        .salg_name = "gcm(aes)" /* this is the cipher name */
    };

```
在使write/send 系统调用族将数据发送到内核之前，消费者必须设置密钥。密钥的设置通过下面setsockopt 调用来描述
此外，在使用 write/send 系统调用族将数据发送到内核之前，消费者必须设置认证标签（authentication
tag）大小。要设置认证标签大小，调用者必须使用下面描述的 setsockopt 调用
使用 sendmsg() 系统调用，应用程序提供应当进行加密或解密处理的数据。此外，IV 通过 sendmsg() 系统
调用提供的数据结构来指定
sendmsg 系统调用struct msghdr 参数被嵌入到 struct cmsghdr 数据结构中。有cmsghdr 数据结构
如何send/recv 系统调用族一起使用，请参recv(2) cmsg(3)。该 cmsghdr 数据结构包含通过单独头实例指定的以下信息
- 使用以下标志之一指定密码操作类型
   - ALG_OP_ENCRYPT - 数据加密

   - ALG_OP_DECRYPT - 数据解密

- 使用标志 ALG_SET_IV 标记IV 信息规范

- 使用标志 ALG_SET_AEAD_ASSOCLEN 指定的关联认证数据（AAD）。AAD 与明密文一起发送给内核。有  内存结构，请参阅下文
send 系统调用族允许指定以下标志：

- MSG_MORE：如果设置了此标志，send 系统调用表现得像密码更新函数，期望随后的 send 系统调用提供更多
  输入数据
注意：内核对任何意外数据报告 -EINVAL。调用者必须确保全部数据符/proc/crypto 中针对所选密码给的约束
使用 recv() 系统调用，应用程序可以从内核加密 API 读取密码操作的结果。输出缓冲区必须至少大到下面内存结构所定义的相同。如果输出数据大小较小，则不执行密码操作
认证的解密操作可能指示完整性错误。这种完整性破坏以 -EBADMSG 错误码标记
#### AEAD 内存结构


AEAD 密码使用在用户空间与内核空间之间作为一个数据流传递的以下信息进行操作
- 明文或密
- 关联认证数据（AAD
- 认证标签

AAD 和认证标签的大小通过 sendmsg setsockopt 调用（参见相关说明）提供。由于内核知道整个数据流大小，它现在能够计算数据流中各个数据分量的正确偏移量
用户空间的调用者必须按以下顺序排列前述信息
- AEAD 加密输入：AAD \|\| 明文

- AEAD 解密输入：AAD \|\| 密文 \|\| 认证标签

用户空间调用者提供的输出缓冲区必须至少大到足以容纳以下数据：

- AEAD 加密输出：密\|\| 认证标签

- AEAD 解密输出：明
### 随机数生成器 API


同样，该操作与其API 非常相似。在初始化期间，struct sockaddr 数据结构必须按如下方式填充：

```

    struct sockaddr_alg sa = {
        .salg_family = AF_ALG,
        .salg_type = "rng", /* this selects the random number generator */
        .salg_name = "drbg_nopr_sha256" /* this is the RNG name */
    };


```
根据 RNG 类型，RNG 必须被播种（seeded）。种子使setsockopt 接口来设置密钥提供。SP800-90A DRBG 不需要种子，但可以被播种。种子在 NIST SP 800-90A 标准中也称为 **Personalization String**（个性化字符串）
使用 read()/recvmsg() 系统调用可以获取随机数。内核在一次调用中最多生128 字节。如果用户空间需更多数据，则必须进行多次 read()/recvmsg() 调用
警告：用户空间调用者可以多次调用前面提到的 accept 系统调用。在这种情况下，返回的文件描述符具有相同
的状态
当内核使CRYPTO_USER_API_RNG_CAVP 选项构建时，启用以下 CAVP 测试接口
- **Entropy**（熵）和 **Nonce**（随机数）的拼接可以通过 ALG_SET_DRBG_ENTROPY setsockopt 接口提供  RNG。设置熵需CAP_SYS_ADMIN 权限
- **Additional Data**（附加数据）可以使用 send()/sendmsg() 系统调用提供，但只能在熵设置之后
### 零拷贝接

除了 send/write/read/recv 系统调用族之外，还可以通过 splice/vmsplice 的零拷贝接口访问 AF_ALG
接口。顾名思义，内核试图避免向内核空间复制的操作
零拷贝操作要求数据在页边界上对齐。非对齐的数据也可以使用，但可能需要内核执行更多操作，从而抵消从
零拷贝接口获得的速度提升
单次零拷贝操作固有的大小限制16 页。如果要发送给 AF_ALG 的数据更多，用户空间必须将输入切分为
最16 页大小的段
零拷贝可以使用以下代码示例（libkcapi 提供了一个完整的工作示例）：

```

    int pipes[2];

    pipe(pipes);
    /* input data in iov */
    vmsplice(pipes[1], iov, iovlen, SPLICE_F_GIFT);
    /* opfd is the file descriptor returned from accept() system call */
    splice(pipes[0], NULL, opfd, NULL, ret, 0);
    read(opfd, out, outlen);


```
### Setsockopt 接口


除了用于发送和检索受密码操作约束的数据的 read/recv send/write 系统调用处理之外，消费者还需设置密码操作的附加信息。此附加信息使用 setsockopt 系统调用设置，该调用必须使用已打开密码的文件描述符
（即 accept 系统调用返回的文件描述符）调用
每次 setsockopt 调用都必须使SOL_ALG 级别
setsockopt 接口允许使用提到optname 设置以下数据
- ALG_SET_KEY -- 设置密钥。密钥设置适用于：

   - skcipher 密码类型（对称密码）

   - hash 密码类型（带密钥的消息摘要）

   - AEAD 密码类型

   - RNG 密码类型以提供种
- ALG_SET_KEY_BY_KEY_SERIAL -- 通过 keyring key_serial_t 设置密钥。此操作ALG_SET_KEY 行为相同  解密的数据从 keyring 密钥复制，并使用该数据作为对称加密的密钥
  传入key_serial_t 必须设置KEY_(POS|USR|GRP|OTH)_SEARCH 权限，否则返-EPERM。支持的密钥类型
  有：user、logon、encrypted trusted
- ALG_SET_AEAD_AUTHSIZE -- 设置 AEAD 密码的认证标签大小。对于加密操作，将生成给定大小的认证标签。对  解密操作，假定所提供的密文包含给定大小的认证标签（参见下面关AEAD 内存布局的章节）
- ALG_SET_DRBG_ENTROPY -- 设置随机数生成器的熵。此选项仅适用RNG 密码类型
### 用户空间 API 示例


请参[^1^] 中的 libkcapi，它提供了对上述 Netlink 内核接口的易用封装。[^1^] 还包含一个调用所libkcapi API 的测试应用程序
[^1^] https://www.chronox.de/libkcapi/index.html
