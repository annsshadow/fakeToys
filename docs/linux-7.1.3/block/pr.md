## 块层对持久预留（Persistent Reservations）的支持


Linux 内核支持一个用户空间接口，用于简化的持久预留，这些预留映射到支持它们的块设备（如 SCSI）。持久预留允许在共享存储设置中将块设备的访问限制为特定的发起端（initiator）

本文档概述受支持ioctl 命令。有关更详细的参考，请参SCSI 主命令（Primary Commands）标准，特别是关于预留以及“PERSISTENT RESERVE IN”和“PERSISTENT RESERVE OUT”命令的章节

所有实现都应确保预留能在断电后继续存在，并覆盖多路径环境中的所有连接。这些行为在 SPC 中是可选的，但会被 Linux 自动应用

### 支持以下类型的预留：


 - PR_WRITE_EXCLUSIVE
	只有拥有该预留的发起端才能写入设备。任何发起端都可以从设备读取

 - PR_EXCLUSIVE_ACCESS
	只有拥有该预留的发起端才能访问设备

 - PR_WRITE_EXCLUSIVE_REG_ONLY
	只有拥有已注册密钥的发起端才能写入设备，任何发起端都可以从设备读取

 - PR_EXCLUSIVE_ACCESS_REG_ONLY
	只有拥有已注册密钥的发起端才能访问设备

 - PR_WRITE_EXCLUSIVE_ALL_REGS

	只有拥有已注册密钥的发起端才能写入设备，任何发起端都可以从设备读取
	所有拥有已注册密钥的发起端都被视为预留持有者
	如果你想使用此类型，请参SPC 规范了解预留持有者的含义

 - PR_EXCLUSIVE_ACCESS_ALL_REGS
	只有拥有已注册密钥的发起端才能访问设备
	所有拥有已注册密钥的发起端都被视为预留持有者
	如果你想使用此类型，请参SPC 规范了解预留持有者的含义


### 支持以下 ioctl


##### 1. IOC_PR_REGISTER


new_key 参数非空时，ioctl 命令注册一个新的预留。如果不存在现有预留，old_key 必须为零；如果要替换现有预留，old_key 必须包含旧的预留密钥

如果 new_key 参数0，它将注销传入 old_key 的现有预留


##### 2. IOC_PR_RESERVE


ioctl 命令预留设备，从而根type 参数限制其他设备的访问。key 参数必须是设备的现有预留密钥，该密钥IOC_PR_REGISTER、IOC_PR_REGISTER_IGNORE、IOC_PR_PREEMPT IOC_PR_PREEMPT_ABORT 命令获取


##### 3. IOC_PR_RELEASE


ioctl 命令释放key flags 指定的预留，从而移除其隐含的任何访问限制


##### 4. IOC_PR_PREEMPT


ioctl 命令释放old_key 引用的现有预留，并用 reservation key new_key 的、指type 的新预留替换它


##### 5. IOC_PR_PREEMPT_ABORT


ioctl 命令的工作方式类似于 IOC_PR_PREEMPT，只是它还会中止通过 old_key 标识的连接发送的任何未完成命令

##### 6. IOC_PR_CLEAR


ioctl 命令注销 key 以及任何使用该设备注册的其他预留密钥，并丢弃任何现有预留


### 标志


所ioctl 都有一flag 字段。目前仅支持一个标志：

 - PR_FL_IGNORE_KEY
	忽略现有的预留密钥。这通常IOC_PR_REGISTER 受支持，某些实现可能对该标志用于 IOC_PR_RESERVE 提供支持

对于所有未知标志，内核将返-EOPNOTSUPP
