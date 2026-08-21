
## TPM 事件日志（Event Log

本文档简要介绍什么是 TPM 日志，以及它是如何从前引导固件（preboot firmware）移交给操作系统的
## 简

前引导固件维护一个事件日志，每当有内容被它哈希到任一 PCR 寄存器时，都会向其中添加新条目。事件按其类型分组，并包含被哈希PCR 寄存器的值。通常，前引导固件会对将要移交执行的组件或与启动过程相关的操作进行哈希
此机制的主要应用是远程证明（remote attestation），而它之所以有用的原因[^1^] 的第一节中有精辟的总结
"Attestation is used to provide information about the platform鈥檚 state to a challenger. However, PCR contents are difficult to interpret; therefore, attestation is typically more useful when the PCR contents are accompanied by a measurement log. While not trusted on their own, the measurement log contains a richer set of information than do the PCR contents. The PCR contents are used to provide the validation of the measurement log."

（证明用于向挑战者提供有关平台状态的信息。然而，PCR 内容难以解读；因此，PCR 内容伴随测量日志时，证明通常更有用。测量日志本身虽不可信，但其包含的信息比 PCR 内容更丰富。PCR 内容用于提供对测量日志的验证。）

## UEFI 事件日志


UEFI 提供的事件日志有一些有点奇怪的怪癖
在调ExitBootServices() 之前，Linux EFI stub 将事件日志复制到stub 自身定义的自定义配置表（configuration table）中。遗憾的是，ExitBootServices() 生成的事件最终并未进入该表
固件提供了所谓的 final events 配置表来解决这个问题。在 EFI_TCG2_PROTOCOL.GetEventLog() 第一次被调用之后，事件会被镜像到该表中
这引入了另一个问题：没有任何保证它不会在 Linux EFI stub 运行之前被调用。因此，stub 在仍然运行时需要计算并保存 final events 表的大小到自定义配置表中，以TPM 驱动之后在拼接来自自定义配置表与 final events 表两半的事件日志时跳过这些事件
## 参

- [^1^] https://trustedcomputinggroup.org/resource/pc-client-specific-platform-firmware-profile-specification/
- [^2^] 最终的拼接drivers/char/tpm/eventlog/efi.c 中完