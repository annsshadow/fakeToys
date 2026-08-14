
## QTEE（Qualcomm 可信执行环境）


QTEE 驱动处理与 Qualcomm TEE [^1^] 的通信。

与 QTEE 的最低层级通信建立在 ARM SMC 调用约定（SMCCC）[^2^] 之上，后者是 QTEE
内部使用的 Secure Channel Manager（SCM）[^3^] 的基础。

在基于 QTEE 的系统中，服务被表示为对象，这些对象带有一系列可被调用以产生结果
（包括其它对象）的操作。

当一个对象托管在 QTEE 内部时，执行其操作被称为“直接调用”（direct invocation）。
QTEE 也可以通过一种称为“回调请求”（callback request）的方法调用托管在非安全
世界的对象。

SCM 提供两个函数来支持直接调用和回调请求：

- QCOM_SCM_SMCINVOKE_INVOKE：用于直接调用。它可以返回一个结果或发起一个
  回调请求。
- QCOM_SCM_SMCINVOKE_CB_RSP：用于提交对先前直接调用触发的回调请求的响应。

QTEE 传输消息 [^4^] 建立在 SCM 驱动函数之上。

一条消息由与 QTEE 共享的两个缓冲区组成：入站缓冲区和出站缓冲区。入站缓冲区
用于直接调用，出站缓冲区用于发起回调请求。下图展示了
```
                                      +---------------------+
                                      |                     v
    +-----------------+-------+-------+------+--------------------------+
    | qcomtee_msg_    |object | buffer       |                          |
    |  object_invoke  |  id   | offset, size |                          | (inbound buffer)
    +-----------------+-------+--------------+--------------------------+
    <---- header -----><---- arguments ------><- in/out buffer payload ->

                                      +-----------+
                                      |           v
    +-----------------+-------+-------+------+----------------------+
    | qcomtee_msg_    |object | buffer       |                      |
    |  callback       |  id   | offset, size |                      | (outbound buffer)
    +-----------------+-------+--------------+----------------------+

```
每个缓冲区以一个头部和一组参数数组开始。

QTEE 传输消息支持四种类型的参数：

- Input Object（IO，输入对象）是当前调用或回调请求的对象参数。
- Output Object（OO，输出对象）是当前调用或回调请求的对象参数。
- Input Buffer（IB，输入缓冲区）是指向入站或出站区域的 (offset, size) 对，
  用于存储当前调用或回调请求的参数。
- Output Buffer（OB，输出缓冲区）是指向入站或出站区域的 (offset, size) 对，
  用于存储来自当前调用或回调请求的参数。

各组件在 QTEE 中相互关系的示意图
```
         User space               Kernel                     Secure world
         ~~~~~~~~~~               ~~~~~~                     ~~~~~~~~~~~~
   +--------+   +----------+                                +--------------+
   | Client |   |callback  |                                | Trusted      |
   +--------+   |server    |                                | Application  |
      /\        +----------+                                +--------------+
      ||  +----------+ /\                                          /\
      ||  |callback  | ||                                          ||
      ||  |server    | ||                                          \/
      ||  +----------+ ||                                   +--------------+
      ||       /\      ||                                   | TEE Internal |
      ||       ||      ||                                   | API          |
      \/       \/      \/   +--------+--------+             +--------------+
   +---------------------+  | TEE    | QTEE   |             | QTEE         |
   |   libqcomtee [5]    |  | subsys | driver |             | Trusted OS   |
   +-------+-------------+--+----+-------+----+-------------+--------------+
   |      Generic TEE API        |       |   QTEE MSG                      |
   |      IOCTL (TEE_IOC_*)      |       |   SMCCC (QCOM_SCM_SMCINVOKE_*)  |
   +-----------------------------+       +---------------------------------+

```
## 参考


[^1^] https://docs.qualcomm.com/bundle/publicresource/topics/80-70015-11/qualcomm-trusted-execution-environment.html

[^2^] http://infocenter.arm.com/help/topic/com.arm.doc.den0028a/index.html

[^3^] drivers/firmware/qcom/qcom_scm.c

[^4^] drivers/tee/qcomtee/qcomtee_msg.h

[^5^] https://github.com/quic/quic-teec
