## OP-TEE（开放可移植可信执行环境，Open Portable Trusted Execution Environment

OP-TEE 驱动处理基于 OP-TEE [^1^] TEE。目前仅支持基于 ARM TrustZone OP-TEE 方案
OP-TEE 通信的最低层构建ARM SMC 调用约定（SMCCC）[^2^] 之上，它OP-TEE SMC 接口 [^3^] 的基础，该接口由驱动在内部使用。在此之上叠放的OP-TEE 消息协议 [^4^]
OP-TEE SMC 接口提供 SMCCC 所需的基本功能以OP-TEE 特有的一些附加功能。最有意思的功能是：

- OPTEE_SMC_FUNCID_CALLS_UID（SMCCC 的一部分）返回版本信息，随后TEE_IOC_VERSION 返回

- OPTEE_SMC_CALL_GET_OS_UUID 返回特定OP-TEE 实现，用于区分，例如，TrustZone OP-TEE 与运行在独立安全协处理器上的 OP-TEE
- OPTEE_SMC_CALL_WITH_ARG 驱动 OP-TEE 消息协议

- OPTEE_SMC_GET_SHM_CONFIG 让驱动和 OP-TEE Linux OP-TEE 之间用于共享内存的内存范围达成一致
GlobalPlatform TEE Client API [^5^] 实现在通用 TEE API 之上
不同组件之间关系的示意图```
      User space                  Kernel                   Secure world
      ~~~~~~~~~~                  ~~~~~~                   ~~~~~~~~~~~~
   +--------+                                             +-------------+
   | Client |                                             | Trusted     |
   +--------+                                             | Application |
      /\                                                  +-------------+
      || +----------+                                           /\
      || |tee-      |                                           ||
      || |supplicant|                                           \/
      || +----------+                                     +-------------+
      \/      /\                                          | TEE Internal|
   +-------+  ||                                          | API         |
   + TEE   |  ||            +--------+--------+           +-------------+
   | Client|  ||            | TEE    | OP-TEE |           | OP-TEE      |
   | API   |  \/            | subsys | driver |           | Trusted OS  |
   +-------+----------------+----+-------+----+-----------+-------------+
   |      Generic TEE API        |       |     OP-TEE MSG               |
   |      IOCTL (TEE_IOC_*)      |       |     SMCCC (OPTEE_SMC_CALL_*) |
   +-----------------------------+       +------------------------------+
```
RPC（远程过程调用，Remote Procedure Call）是来自安全世界对内核驱动或 tee-supplicant 的请求。一RPC OPTEE_SMC_CALL_WITH_ARG 返回的一组特殊范围的 SMCCC 返回值标识。旨在发给内核的 RPC 消息由内核驱动处理。其RPC 消息将被转发tee-supplicant，驱动不再进一步参与，除非切换共享内存缓冲区的表示
### OP-TEE 设备枚举（OP-TEE device enumeration

OP-TEE 提供了一个伪可信应用程序：drivers/tee/optee/device.c，以支持设备枚举。换句话说，OP-TEE 驱动调用该应用程序来检索可作为设备注册TEE 总线上的可信应用程序列表
### OP-TEE 通知（OP-TEE notifications

安全世界可以使用两类通知，使普通世界知晓某个事件
1. 通过 `OPTEE_RPC_CMD_NOTIFICATION` 配合 `OPTEE_RPC_NOTIFICATION_SEND` 参数传递的同步通知2. 通过非安全的边沿触发中断与非安全中断处理程序中的快速调用组合传递的异步通知
同步通知受限于依RPC 来投递，这仅在使`OPTEE_SMC_CALL_WITH_ARG` yielding 调用进入安全世界时可用。这将其排除在安全世界中断处理程序之外
异步通知通过注册OP-TEE 驱动中的非安全边沿触发中断投递给中断处理程序。实际的通知值通过快速调`OPTEE_SMC_GET_ASYNC_NOTIF_VALUE` 获取。请注意，一个中断可以代表多个通知
通知`OPTEE_SMC_ASYNC_NOTIF_VALUE_DO_BOTTOM_HALF` 具有特殊含义。当接收到该值时，意味着普通世界应当发起一yielding 调用 `OPTEE_MSG_CMD_DO_BOTTOM_HALF`。该调用由协助中断处理程序的线程发出。这是安全世界中OP-TEE OS 实现设备驱动上半下半部风格的一个构建模块
### OPTEE_INSECURE_LOAD_IMAGE Kconfig 选项


OPTEE_INSECURE_LOAD_IMAGE Kconfig 选项启用了在内核启动后从内核加载 BL32 OP-TEE 镜像的能力，而不是在内核启动前从固件加载。这还需要在 Arm Trusted Firmware 中启用相应的选项。Arm Trusted Firmware 文档 [^6^] 解释了启用此选项所带来的安全威胁，以及固件和平台层面的缓解措施
使用该选项时，还存在应当解决的、针对内核的额外攻击向量/缓解措施
1. 启动链安全
   - 攻击向量：替rootfs 中的 OP-TEE OS 镜像以获取对系统的控制权
   - 缓解：必须有验证内核rootfs 的启动链安全，否则攻击者可以通过修改 rootfs 中的内容来修改已加载OP-TEE 二进制文件
2. 备用启动模式
   - 攻击向量：使用备用启动模式（即恢复模式）时，OP-TEE 驱动不会被加载，从而留SMC 漏洞
   - 缓解：如果存在备用启动设备的方法（例如恢复模式），应确保在那种模式下应用相同的缓解措施
3. SMC 调用之前的攻击
   - 攻击向量：在发出用于加载 OP-TEE SMC 调用之前执行的代码可能被利用，从而加载一个替换的 OS 镜像
   - 缓解：OP-TEE 驱动必须在任何潜在的攻击向量被打开之前加载。这应包括挂载任何可修改的文件系统、打开网络端口或与外部设备（例USB）通信
4. 阻止加载 OP-TEE SMC 调用
   - 攻击向量：阻止驱动被探测（probe），从而使加载 OP-TEE SMC 调用在期望时未能执行，使其保持开放以便后续执行并加载被修改的 OS
   - 缓解：建议将 OP-TEE 驱动构建为内建（builtin）驱动，而非模块，以防止可能导致模块不被加载的漏洞利用
## 参考（References

[^1^] https://github.com/OP-TEE/optee_os

[^2^] http://infocenter.arm.com/help/topic/com.arm.doc.den0028a/index.html

[^3^] drivers/tee/optee/optee_smc.h

[^4^] drivers/tee/optee/optee_msg.h

[^5^] http://www.globalplatform.org/specificationsdevice.asp look for
    "TEE Client API Specification v1.0" 并点击下载
[^6^] https://trustedfirmware-a.readthedocs.io/en/latest/threat_model/threat_model.html
