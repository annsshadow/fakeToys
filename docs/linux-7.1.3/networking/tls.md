
## 内核 TLS

## 概述

传输层安全（Transport Layer Security，TLS）是一种运行于 TCP 之上的上层协
（Upper Layer Protocol，ULP）。TLS 提供端到端的数据完整性与机密性

## 用户接口

### 创建 TLS 连接

首先创建一个新TCP 套接字，在连接建立后设置 TLS ULP

  sock = socket(AF_INET, SOCK_STREAM, 0);
  connect(sock, addr, addrlen);
  setsockopt(sock, SOL_TCP, TCP_ULP, "tls", sizeof("tls"));

设置 TLS ULP 后，我们就可以对 TLS 套接字选项进行设置/获取。当前只有对称加
由内核处理。在 TLS 握手完成后，我们就拥有了将数据路径迁移到内核所需的全部参数
发送和接收分别有独立的套接字选项用于将其迁移到内核

  /** From linux/tls.h **/
  struct tls_crypto_info {
          unsigned short version;
          unsigned short cipher_type;
  };

  struct tls12_crypto_info_aes_gcm_128 {
          struct tls_crypto_info info;
          unsigned char iv[TLS_CIPHER_AES_GCM_128_IV_SIZE];
          unsigned char key[TLS_CIPHER_AES_GCM_128_KEY_SIZE];
          unsigned char salt[TLS_CIPHER_AES_GCM_128_SALT_SIZE];
          unsigned char rec_seq[TLS_CIPHER_AES_GCM_128_REC_SEQ_SIZE];
  };


  struct tls12_crypto_info_aes_gcm_128 crypto_info;

  crypto_info.info.version = TLS_1_2_VERSION;
  crypto_info.info.cipher_type = TLS_CIPHER_AES_GCM_128;
  memcpy(crypto_info.iv, iv_write, TLS_CIPHER_AES_GCM_128_IV_SIZE);
  memcpy(crypto_info.rec_seq, seq_number_write,
					TLS_CIPHER_AES_GCM_128_REC_SEQ_SIZE);
  memcpy(crypto_info.key, cipher_key_write, TLS_CIPHER_AES_GCM_128_KEY_SIZE);
  memcpy(crypto_info.salt, implicit_iv_write, TLS_CIPHER_AES_GCM_128_SALT_SIZE);

  setsockopt(sock, SOL_TLS, TLS_TX, &crypto_info, sizeof(crypto_info));

发送与接收是分别设置的，但设置方式相同，只需使用 TLS_TX TLS_RX 其中之一

### 发TLS 应用数据

在设TLS_TX 套接字选项后，通过该套接字发送的所有应用数据都会使TLS 以及
该套接字选项中提供的参数进行加密。例如，我们可以如下发送一条加密的 hello world
记录

  const char *msg = "hello world\n";
  send(sock, msg, strlen(msg));

如果可能，send() 的数据会直接从用户空间提供的缓冲区加密到内核的加密发送缓冲区中

sendfile 系统调用会以最大长度（2^14）的 TLS 记录发送文件数据

  file = open(filename, O_RDONLY);
  fstat(file, &stat);
  sendfile(sock, file, &offset, stat.st_size);

除非传入 MSG_MORE，否则每send() 调用后都会创建并发TLS 记录。MSG_MORE 
推迟记录的创建，直到不再传入 MSG_MORE 或达到最大记录大小为止

内核需要为加密数据分配缓冲区。该缓冲区在调用 send() 时分配，因此要么整个 send()
调用返回 -ENOMEM（或阻塞等待内存），要么加密一定会成功。如send() 返回 -ENOMEM
且上一次使MSG_MORE 的调用在套接字缓冲区中仍残留有数据，MSG_MORE 的数据会
保留在套接字缓冲区中

### 接收 TLS 应用数据

在设TLS_RX 套接字选项后，所recv 系列的套接字调用都会使用提供TLS 参数
进行解密。必须接收到一个完整的 TLS 记录后才能进行解密

  char buffer[^16384^];
  recv(sock, buffer, 16384);

如果用户的缓冲区足够大，接收到的数据会直接解密到用户缓冲区中，不会发生额外的
分配。如果用户空间缓冲区太小，数据会先在内核中解密再拷贝到用户空间

如果接收到的消息中的 TLS 版本setsockopt 传入的版本不一致，返回 `EINVAL`

如果接收到的消息过大，返`EMSGSIZE`

如果因任何其他原因导致解密失败，返回 `EBADMSG`

### 发TLS 控制消息

除应用数据外，TLS 还有控制消息，例如告警消息（记录类型 21）和握手消息（记录类
22）等。这些消息可以通过 CMSG 提供 TLS 记录类型来经由套接字发送。例如，下面
函数使用类型@record_type 的记录发@length 字节@data

  /** send TLS control message using record_type **/
  static int klts_send_ctrl_message(int sock, unsigned char record_type,
                                    void *data, size_t length)
  {
        struct msghdr msg = {0};
        int cmsg_len = sizeof(record_type);
        struct cmsghdr *cmsg;
        char buf[CMSG_SPACE(cmsg_len)];
        struct iovec msg_iov;   /** Vector of data to send/receive into.  **/

        msg.msg_control = buf;
        msg.msg_controllen = sizeof(buf);
        cmsg = CMSG_FIRSTHDR(&msg);
        cmsg->cmsg_level = SOL_TLS;
        cmsg->cmsg_type = TLS_SET_RECORD_TYPE;
        cmsg->cmsg_len = CMSG_LEN(cmsg_len);
        *CMSG_DATA(cmsg) = record_type;
        msg.msg_controllen = cmsg->cmsg_len;

        msg_iov.iov_base = data;
        msg_iov.iov_len = length;
        msg.msg_iov = &msg_iov;
        msg.msg_iovlen = 1;

        return sendmsg(sock, &msg, 0);
  }

控制消息数据应以未加密形式提供，并由内核加密

### 接收 TLS 控制消息

TLS 控制消息会传入用户空间缓冲区，消息类型通过 cmsg 传递。如果未提供 cmsg 缓冲区，
则接收到控制消息时会返回错误。数据消息可以在未设cmsg 缓冲区的情况下接收

  char buffer[^16384^];
  char cmsg[CMSG_SPACE(sizeof(unsigned char))];
  struct msghdr msg = {0};
  msg.msg_control = cmsg;
  msg.msg_controllen = sizeof(cmsg);

  struct iovec msg_iov;
  msg_iov.iov_base = buffer;
  msg_iov.iov_len = 16384;

  msg.msg_iov = &msg_iov;
  msg.msg_iovlen = 1;

  int ret = recvmsg(sock, &msg, 0 /** flags **/);

  struct cmsghdr *cmsg = CMSG_FIRSTHDR(&msg);
  if (cmsg->cmsg_level == SOL_TLS &&
      cmsg->cmsg_type == TLS_GET_RECORD_TYPE) {
      int record_type = **((unsigned char **)CMSG_DATA(cmsg));
      // Do something with record_type, and control message data in
      // buffer.
      //
      // Note that record_type may be == to application data (23).
  } else {
      // Buffer contains application data.
  }

recv 永远不会返回来自不同类型 TLS 记录混合的数据

### TLS 1.3 密钥更新

TLS 1.3 中，KeyUpdate 握手消息表示发送方正在更新TX 密钥。KeyUpdate 之后发送的
任何消息都会使用新密钥加密。用户空间库可以像提供初始密钥一样，通过 TLS_TX TLS_RX
套接字选项将新密钥传递给内核。TLS 版本和加密套件不能更改

为防止使用错误密钥尝试解密传入记录，当内核接收到 KeyUpdate 消息时会暂停解密，直
通过 TLS_RX 套接字选项提供新密钥为止。在读取KeyUpdate 之后、提供新密钥之前发生
任何读取都会EKEYEXPIRED 失败。在提供新密钥之前，poll() 不会报告来自该套接字的任
读取事件。发送侧没有暂停机制

用户空间应确保所提供crypto_info 已被正确设置。特别是，内核不会检查密nonce 
重用

成功和失败的密钥更新次数分别`TlsTxRekeyOk`、`TlsRxRekeyOk`、`TlsTxRekeyError`
`TlsRxRekeyError` 统计项中跟踪。`TlsRxRekeyReceived` 统计项记录已接收到的 KeyUpdate
握手消息数量

### 集成到用户空TLS 

从高层来看，内核 TLS ULP 是一个用户空TLS 库记录层（record layer）的替代品

OpenSSL 打补丁以使用 ktls 作为记录层的补丁集在
`此处 <https://github.com/Mellanox/openssl/commits/tls_rx2>`_

`一个示<https://github.com/ktls/af_ktls-tool/commits/RX>`_
在握手之后直接使gnutls 调用 send。由于它没有实现完整的记录层，因此不支持控制
消息

### 可选优

如果显式请求，TLS ULP 可以做某些针对特定条件的优化。这些优化要么并非普遍有益，
要么可能影响正确性，因此需要显式开启（opt-in）。所有选项都通过 setsockopt() 
套接字设置，其状态可通过 getsockopt() 以及套接字诊断（`ss`）查看

#### TLS_TX_ZEROCOPY_RO

仅用于设备卸载。允sendfile() 的数据直接传输到 NIC，而无需在内核中拷贝。这样在
启用设备卸载时可以实现真正的零拷贝行为

应用程序必须确保数据在提交与传输完成之间不被修改。换句话说，这主要适用于通过
sendfile() 在套接字上发送的数据是只读的情况

修改数据可能导致原始 TCP 传输TCP 重传使用不同版本的数据。对接收方而言，这看起
就像TLS 记录被篡改，并会导致记录认证失败

#### TLS_RX_EXPECT_NO_PAD

仅用TLS 1.3。期望发送方不对记录进行填充。这样可以在 TLS 1.3 下将数据直接解密
用户空间缓冲区

只有在远端可信的情况下才适合开启此优化，否则它会成为一个将 TLS 处理成本翻倍的
攻击向量

如果解密后的记录发现曾被填充、或不是数据记录，则会再次解密到一个内核缓冲区中，
而不使用零拷贝。此类事件计`TlsDecryptRetry` 统计项

#### TLS_TX_MAX_PAYLOAD_LEN

指定所发TLS 记录明文负载的最大大小

设置此选项后，内核会对所有出TLS 记录强制执行该限制。没有任何明文分片会超过该大小
该选项可用于实TLS Record Size Limit 扩展 [^1^]

- 对于 TLS 1.2，该值直接对应记录大小限制
- 对于 TLS 1.3，该值应设为 record_size_limit - 1，因为记录大小限制为 ContentType
  字段额外包含了一个字节

该选项的有效范围是：TLS 1.2 64 16384 字节，TLS 1.3 63 16384 字节
TLS 1.3 的最小下限更低，是因ContentType 字段额外占用了一个字节

[^1^] https://datatracker.ietf.org/doc/html/rfc8449

## 统计信息

TLS 实现暴露了以下每个命名空间的统计信息（`/proc/net/tls_stat`）：

- `TlsCurrTxSw`, `TlsCurrRxSw` -
  当前已安装、由主机处理加密TX RX 会话数量

- `TlsCurrTxDevice`, `TlsCurrRxDevice` -
  当前已安装、由 NIC 处理加密TX RX 会话数量

- `TlsTxSw`, `TlsRxSw` -
  以主机加密方式打开TX RX 会话数量

- `TlsTxDevice`, `TlsRxDevice` -
  NIC 加密方式打开TX RX 会话数量

- `TlsDecryptError` -
  记录解密失败（例如由于认证标签不正确

- `TlsDeviceRxResync` -
  发送给处理加密NIC RX 重新同步次数

- `TlsDecryptRetry` -
  由于 `TLS_RX_EXPECT_NO_PAD` 预测错误而不得不重新解密RX 记录数量
  注意该计数器也会因非数据记录而递增

- `TlsRxNoPadViolation` -
  由于 `TLS_RX_EXPECT_NO_PAD` 预测错误而不得不重新解密的数RX 记录数量

- `TlsTxRekeyOk`, `TlsRxRekeyOk` -
  现有会话TX RX 成功重新密钥（rekey）的次数

- `TlsTxRekeyError`, `TlsRxRekeyError` -
  现有会话TX RX 重新密钥失败的次

- `TlsRxRekeyReceived` -
  接收到的 KeyUpdate 握手消息数量，要求用户空间提供新RX 密钥
