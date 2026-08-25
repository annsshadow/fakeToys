
## 基于 FF-A TPM CRB 驱动


TPM 命令响应缓冲区（CRB）接口是 TCG PC Client Platform TPM Profile (PTP)
规范 [^1^]_ 中定义的一个标TPM 接口。CRB 提供了一组结构化的控制寄存器客户端在TPM 交互时会用到它们，同时还提供了一个用于存TPM 命令与响应的
数据缓冲区。CRB 接口可以在以下位置实现：

- 独立 TPM 芯片中的硬件寄存
- 在内存中，用于运行在隔离环境中的 TPM，其中共享内存允许客户端TPM 交互

Arm A 系列固件框架（FF-A）[^2^]_ 是一份规范，定义了用于以下目的的接口协议
- 将固件划分到运行Arm Secure 世界环境（也称为 TrustZone）中的软件分区中

- 为处于非安全（Non-secure）状态的软件组件（例如操作系统与 Hypervisor）提  标准接口，以便与这些固件通信

TPM 可以作为 FF-A 安全服务来实现。它可以是固TPM，也可能是充当独TPM
芯片代理TPM 服务。基FF-A TPM 将硬件细节（例如总线控制器与片选）操作系统中抽象出来，并且可以保护 locality 4 不被操作系统访问。客户端使用
TCG 定义CRB 接口TPM 服务交互
Arm TPM Service Command Response Buffer Interface Over FF-A [^3^]_ 规范定义客户端可以用来在 CRB 发生更新时发出信号的 FF-A 消息
Linux CRB 驱动FF-A 的交互方式概要如下：

- tpm_crb_ffa 驱动CRB over FF-A 规范中定义的架构TPM 服务 UUID 向内核的
  FF-A 子系统注册
- 如果 FF-A 发现了某TPM 服务，则 tpm_crb_ffa 驱动中的 probe() 函数会运行，
  驱动完成初始化
- Linux CRB 驱动的探测与初始化是由发现通过 ACPI 通告TPM 触发的。CRB 驱动
  可以通过 ACPI 'start' 方法检TPM 的类型。Arm FF-A start 方法定义  TCG ACPI v1.4 [^4^]_ 中
- CRB 驱动执行其常规功能（例如发出 'start' 信号以及 locality 的请释放  时，它会调用 tpm_crb_ffa 驱动中的 tpm_crb_ffa_start() 函数，该函数负责处理
  发往 TPM FF-A 消息
## 参考资

   https://trustedcomputinggroup.org/resource/pc-client-platform-tpm-profile-ptp-specification/
   https://developer.arm.com/documentation/den0077/latest/
   https://developer.arm.com/documentation/den0138/latest/
   https://trustedcomputinggroup.org/resource/tcg-acpi-specification/
