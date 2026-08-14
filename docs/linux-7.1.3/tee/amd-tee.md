
## AMD-TEE（AMD 的可信执行环境）

AMD-TEE 驱动负责与 AMD 的 TEE 环境进行通信。该 TEE 环境由 AMD Secure
Processor 提供。

AMD Secure Processor（前身称为 Platform Security Processor，简称 PSP）是一颗
专用处理器，具备 ARM TrustZone 技术，以及为支持第三方可信应用（Trusted
Application）而设计的基于软件的可信执行环境（TEE）。目前该功能仅对 APU 启用。

```

                                             |
    x86                                      |
                                             |
 User space            (Kernel space)        |    AMD Secure Processor (PSP)
 ~~~~~~~~~~            ~~~~~~~~~~~~~~        |    ~~~~~~~~~~~~~~~~~~~~~~~~~~
                                             |
 +--------+                                  |       +-------------+
 | Client |                                  |       | Trusted     |
 +--------+                                  |       | Application |
     /\                                      |       +-------------+
     ||                                      |             /\
     ||                                      |             ||
     ||                                      |             \/
     ||                                      |         +----------+
     ||                                      |         |   TEE    |
     ||                                      |         | Internal |
     \/                                      |         |   API    |
 +---------+           +-----------+---------+         +----------+
 | TEE     |           | TEE       | AMD-TEE |         | AMD-TEE  |
 | Client  |           | subsystem | driver  |         | Trusted  |
 | API     |           |           |         |         |   OS     |
 +---------+-----------+----+------+---------+---------+----------+
 |   Generic TEE API        |      | ASP     |      Mailbox       |
 |   IOCTL (TEE_IOC_*)      |      | driver  | Register Protocol  |
 +--------------------------+      +---------+--------------------+

```
在最底层（x86 上），AMD Secure Processor（ASP）驱动使用 CPU 到 PSP 的 mailbox
寄存器向 PSP 提交命令。命令缓冲区的格式对 ASP 驱动是不透明的。它的职责是向安全
处理器提交命令，并将结果返回给 AMD-TEE 驱动。AMD-TEE 驱动与 AMD Secure
Processor 驱动之间的接口可在 [^1^] 中找到。

AMD-TEE 驱动将命令缓冲区负载打包，以便在 TEE 中处理。不同 TEE 命令的命令缓冲
区格式可在 [^2^] 中找到。

AMD-TEE Trusted OS 支持的 TEE 命令包括：

- TEE_CMD_ID_LOAD_TA          - 将一个可信应用（TA）二进制文件加载到 TEE 环境中。
- TEE_CMD_ID_UNLOAD_TA        - 从 TEE 环境中卸载 TA 二进制文件。
- TEE_CMD_ID_OPEN_SESSION     - 与已加载的 TA 打开一个会话。
- TEE_CMD_ID_CLOSE_SESSION    - 关闭与已加载 TA 的会话。
- TEE_CMD_ID_INVOKE_CMD       - 调用已加载 TA 的一个命令。
- TEE_CMD_ID_MAP_SHARED_MEM   - 映射共享内存。
- TEE_CMD_ID_UNMAP_SHARED_MEM - 取消映射共享内存。

AMD-TEE Trusted OS 是运行在 AMD Secure Processor 上的固件。

AMD-TEE 驱动向 TEE 子系统注册自身，并实现以下驱动函数回调：

- get_version - 返回驱动实现 id 与能力（capability）。
- open - 设置驱动上下文数据结构。
- release - 释放驱动资源。
- open_session - 加载 TA 二进制文件并与已加载的 TA 打开会话。
- close_session - 关闭与已加载 TA 的会话并卸载它。
- invoke_func - 调用已加载 TA 的一个命令。

AMD-TEE 不支持 cancel_req 驱动回调。

用户空间（客户端）可以使用 GlobalPlatform TEE Client API [^3^] 与 AMD 的 TEE
通信。AMD 的 TEE 为加载、打开会话、调用命令以及关闭与 TA 的会话提供了一个安全
环境。

## 参考资料

[^1^] include/linux/psp-tee.h

[^2^] drivers/tee/amdtee/amdtee_if.h

[^3^] http://www.globalplatform.org/specificationsdevice.asp look for
    "TEE Client API Specification v1.0" and click download.
