
## TS-TEE（Trusted Services 项目）


该驱动提供对由 Trusted Services 实现的安全服务的访问。

Trusted Services [^1^] 是一个 TrustedFirmware.org 项目，提供了一套用于在 FF-A [^2^] S-EL0
安全分区（Secure Partition）中开发和部署设备可信根（Root of Trust）服务的框架。该项目托管了
面向 Arm A-profile 设备的 Arm Platform Security Architecture [^3^] 的参考实现。

FF-A 安全分区（SP）可通过 FF-A 驱动 [^4^] 访问，该驱动为本文档所述驱动提供了底层通信。在此
之上使用的是 Trusted Services RPC 协议 [^5^]。要从用户空间使用该驱动，在 [^6^] 处提供了一个
参考实现，它是名为 libts [^7^] 的 Trusted Services 客户端库的一部分。

所有 Trusted Services（TS）SP 拥有相同的 FF-A UUID；它标识的是 TS RPC 协议。一个 TS SP 可以
承载一个或多个服务（例如 PSA Crypto、PSA ITS 等）。一个服务由其服务 UUID 标识；同一类型的服务
不能在同一 SP 中出现两次。在 SP 启动期间，SP 中的每个服务会被分配一个“接口 ID”（interface ID）。
这只是一个简短的 ID，用于简化消息寻址。

通用 TEE 的设计是一次性与可信操作系统（Trusted OS）共享内存，然后该内存可被复用，用来与运行在
可信操作系统上的多个应用程序通信。然而，在 FF-A 的情况下，内存共享是在端点（endpoint）层面工作
的，即内存是与特定的 SP 共享的。用户空间必须能够根据端点 ID 分别与每个 SP 共享内存；因此，为
每一个被发现的 TS SP 注册一个独立的 TEE 设备。打开一个 SP 对应于打开该 TEE 设备并创建一个 TEE
上下文。一个 TS SP 承载一个或多个服务。打开一个服务对应于在给定的 tee_context 中打开一个会话。

```

   User space                  Kernel space                   Secure world
   ~~~~~~~~~~                  ~~~~~~~~~~~~                   ~~~~~~~~~~~~
   +--------+                                               +-------------+
   | Client |                                               | Trusted     |
   +--------+                                               | Services SP |
      /\                                                    +-------------+
      ||                                                          /\
      ||                                                          ||
      ||                                                          ||
      \/                                                          \/
   +-------+                +----------+--------+           +-------------+
   | libts |                |  TEE     | TS-TEE |           |  FF-A SPMC  |
   |       |                |  subsys  | driver |           |   + SPMD    |
   +-------+----------------+----+-----+--------+-----------+-------------+
   |      Generic TEE API        |     |  FF-A  |     TS RPC protocol     |
   |      IOCTL (TEE_IOC_*)      |     | driver |        over FF-A        |
   +-----------------------------+     +--------+-------------------------+

```
## 参考


[^1^] https://www.trustedfirmware.org/projects/trusted-services/

[^2^] https://developer.arm.com/documentation/den0077/

[^3^] https://www.arm.com/architecture/security-features/platform-security

[^4^] drivers/firmware/arm_ffa/

[^5^] https://trusted-services.readthedocs.io/en/v1.0.0/developer/service-access-protocols.html#abi

[^6^] https://git.trustedfirmware.org/TS/trusted-services.git/tree/components/rpc/ts_rpc/caller/linux/ts_rpc_caller_linux.c?h=v1.0.0

[^7^] https://git.trustedfirmware.org/TS/trusted-services.git/tree/deployments/libts/arm-linux/CMakeLists.txt?h=v1.0.0
