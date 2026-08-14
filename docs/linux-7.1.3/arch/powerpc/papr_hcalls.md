
## 超级调用操作码（hcalls）


## 概述

64 位 Power Book3S 平台上的虚拟化基于 PAPR 规范 [^1^]_，该规范描述了客户机
操作系统的运行时环境，以及客户机应如何与 hypervisor 交互以执行特权操作。目前
有两种符合 PAPR 的 hypervisor：

- **IBM PowerVM (PHYP)**：IBM 的专有 hypervisor，支持将 AIX、IBM-i 和 Linux
  作为受支持的客户机（称为逻辑分区或 LPARS）。它支持完整的 PAPR 规范。

- **Qemu/KVM**：支持运行在 PPC64 Linux 宿主机上的 PPC64 Linux 客户机。不过它
  仅实现了 PAPR 规范的一个名为 LoPAPR 的子集 [^2^]_。

在 PPC64 架构上，运行于 PAPR hypervisor 之上的客户机内核称为 **pSeries 客户机**。
pSeries 客户机运行在特权级模式（HV=0）下，每当需要执行 hypervisor 特权操作 [^3^]_
或 hypervisor 管理的其他服务时，都必须向 hypervisor 发出超级调用。

因此，超级调用（hcall）本质上是 pSeries 客户机请求 hypervisor 代表客户机执行特权
操作。客户机发出调用并提供必要的输入操作数。hypervisor 执行完特权操作后，将状态码
和输出操作数返回给客户机。

## HCALL ABI

hcall 的 ABI 规范（pseries 客户机与 PAPR hypervisor 之间）在参考文献 [^2^]_ 的
第 14.5.3 节中描述。切换到 Hypervisor 上下文通过指令 **HVCS** 完成，该指令要求将
hcall 的操作码设置在 **r3** 中，hcall 的任何输入参数在寄存器 **r4-r12** 中提供。
如果需要通过内存缓冲区传递值，存储在该缓冲区中的数据应采用大端字节序。

一旦 hypervisor 处理完 'HVCS' 指令并将控制权返回给客户机，hcall 的返回值可在
**r3** 中获取，任何输出值在寄存器 **r4-r12** 中返回。与输入参数类似，存储在内存
缓冲区中的任何输出值都将采用大端字节序。

PowerPC 架构代码提供了名为 **plpar_hcall_xxx** 的便捷封装函数，定义在架构特定的
头文件中 [^4^]_，用于从作为 pSeries 客户机运行的 Linux 内核中发出 hcall。

## 寄存器约定

任何 hcall 都应遵循 "64-Bit ELF V2 ABI Specification: Power Architecture"[^5^]_
第 2.2.1.1 节中描述的相同寄存器约定。下表汇总了这些约定：

+----------+----------+-------------------------------------------+
| Register |Volatile  |  Purpose                                  |
| Range    |(Y/N)     |                                           |
+==========+==========+===========================================+
|   r0     |    Y     |  可选用途                                 |
+----------+----------+-------------------------------------------+
|   r1     |    N     |  栈指针                                   |
+----------+----------+-------------------------------------------+
|   r2     |    N     |  TOC                                      |
+----------+----------+-------------------------------------------+
|   r3     |    Y     |  hcall 操作码/返回值                       |
+----------+----------+-------------------------------------------+
|  r4-r10  |    Y     |  输入与输出值                             |
+----------+----------+-------------------------------------------+
|   r11    |    Y     |  可选用途/环境指针                         |
+----------+----------+-------------------------------------------+
|   r12    |    Y     |  可选用途/全局入口点处的函数入口地址       |
|          |          |                                           |
+----------+----------+-------------------------------------------+
|   r13    |    N     |  线程指针                                 |
+----------+----------+-------------------------------------------+
|  r14-r31 |    N     |  局部变量                                 |
+----------+----------+-------------------------------------------+
|    LR    |    Y     |  链接寄存器                               |
+----------+----------+-------------------------------------------+
|   CTR    |    Y     |  循环计数器                               |
+----------+----------+-------------------------------------------+
|   XER    |    Y     |  定点异常寄存器                           |
+----------+----------+-------------------------------------------+
|  CR0-1   |    Y     |  条件寄存器字段                           |
+----------+----------+-------------------------------------------+
|  CR2-4   |    N     |  条件寄存器字段                           |
+----------+----------+-------------------------------------------+
|  CR5-7   |    Y     |  条件寄存器字段                           |
+----------+----------+-------------------------------------------+
|  Others  |    N     |                                           |
+----------+----------+-------------------------------------------+

## DRC 与 DRC 索引

```

     DR1                                  Guest
     +--+        +------------+         +---------+
     |  | <----> |            |         |  User   |
     +--+  DRC1  |            |   DRC   |  Space  |
                 |    PAPR    |  Index  +---------+
     DR2         | Hypervisor |         |         |
     +--+        |            | <-----> |  Kernel |
     |  | <----> |            |  Hcall  |         |
     +--+  DRC2  +------------+         +---------+

```
PAPR hypervisor 将 LPAR 可用的共享硬件资源（如 PCI 设备、NVDIMM 等）称为动态资源
（Dynamic Resource，DR）。当 DR 分配给某个 LPAR 时，PHYP 会创建一个名为动态资源
连接器（Dynamic Resource Connector，DRC）的数据结构来管理 LPAR 的访问。LPAR 通过
称为 DRC-Index 的不透明 32 位数值来引用 DRC。DRC-index 值通过设备树（device-tree）
提供给 LPAR，作为与 DR 关联的设备树节点的一个属性存在。

## HCALL 返回值

处理完 hcall 后，hypervisor 在 **r3** 中设置返回值，表示 hcall 成功或失败。若失败，
错误码指示出错原因。这些码在架构特定的头文件中定义和记录 [^4^]_。

在某些情况下，hcall 可能需要很长时间，并且需要多次发出才能被完全处理。这些 hcall
通常会在其参数列表中接受一个不透明值 **continue-token**，返回值为 **H_CONTINUE**
表示 hypervisor 尚未完成对该 hcall 的处理。

为发出此类 hcall，客户机需要在初次调用时设置 **continue-token == 0**，并在每次
后续 hcall 中使用 hypervisor 返回的 **continue-token** 值，直到 hypervisor 返回
一个非 **H_CONTINUE** 的返回值。

## HCALL 操作码

以下是 PHYP 支持的 HCALL 的部分列表。对应的操作码值请查阅架构特定的头文件 [^4^]_：

**H_SCM_READ_METADATA**

| 输入： **drcIndex, offset, buffer-address, numBytesToRead**
| 输出： **numBytesRead**
| 返回值： **H_Success, H_Parameter, H_P2, H_P3, H_Hardware**

给定一个 NVDIMM 的 DRC 索引，从与其关联的元数据区中在指定偏移处读取 N 字节，并复制到
所提供的缓冲区。元数据区存储配置信息，如标签信息、坏块等。元数据区位于 NVDIMM 存储区
带外，因此提供了单独的访问语义。

**H_SCM_WRITE_METADATA**

| 输入： **drcIndex, offset, data, numBytesToWrite**
| 输出： **None**
| 返回值： **H_Success, H_Parameter, H_P2, H_P4, H_Hardware**

给定一个 NVDIMM 的 DRC 索引，在指定偏移处将 N 字节写入与其关联的元数据区，数据来自
所提供的缓冲区。

**H_SCM_BIND_MEM**

| 输入： **drcIndex, startingScmBlockIndex, numScmBlocksToBind,**
| **targetLogicalMemoryAddress, continue-token**
| 输出： **continue-token, targetLogicalMemoryAddress, numScmBlocksToBound**
| 返回值： **H_Success, H_Parameter, H_P2, H_P3, H_P4, H_Overlap,**
| **H_Too_Big, H_P5, H_Busy**

给定一个 NVDIMM 的 DRC 索引，将一段连续的 SCM 块范围
**(startingScmBlockIndex, startingScmBlockIndex+numScmBlocksToBind)** 映射到客户机
物理地址空间中的 **targetLogicalMemoryAddress** 处。如果
**targetLogicalMemoryAddress == 0xFFFFFFFF_FFFFFFFF**，则由 hypervisor 为客户机分配
目标地址。如果客户机对被绑定的 SCM 块存在活跃的 PTE 条目，该 HCALL 可能失败。

**H_SCM_UNBIND_MEM**
| 输入： drcIndex, startingScmLogicalMemoryAddress, numScmBlocksToUnbind
| 输出： numScmBlocksUnbound
| 返回值： **H_Success, H_Parameter, H_P2, H_P3, H_In_Use, H_Overlap,**
| **H_Busy, H_LongBusyOrder1mSec, H_LongBusyOrder10mSec**

给定一个 NVDIMM 的 DRC 索引，从客户机物理地址空间取消映射从
**startingScmLogicalMemoryAddress** 开始的 **numScmBlocksToUnbind** 个 SCM 块。
如果客户机对被解绑的 SCM 块存在活跃的 PTE 条目，该 HCALL 可能失败。

**H_SCM_QUERY_BLOCK_MEM_BINDING**

| 输入： **drcIndex, scmBlockIndex**
| 输出： **Guest-Physical-Address**
| 返回值： **H_Success, H_Parameter, H_P2, H_NotFound**

给定一个 DRC 索引和 SCM 块索引，返回该 SCM 块所映射到的客户机物理地址。

**H_SCM_QUERY_LOGICAL_MEM_BINDING**

| 输入： **Guest-Physical-Address**
| 输出： **drcIndex, scmBlockIndex**
| 返回值： **H_Success, H_Parameter, H_P2, H_NotFound**

给定一个客户机物理地址，返回映射到该地址的 DRC 索引和 SCM 块。

**H_SCM_UNBIND_ALL**

| 输入： **scmTargetScope, drcIndex**
| 输出： **None**
| 返回值： **H_Success, H_Parameter, H_P2, H_P3, H_In_Use, H_Busy,**
| **H_LongBusyOrder1mSec, H_LongBusyOrder10mSec**

根据目标范围，从 LPAR 内存中取消映射属于所有 NVDIMM 的所有 SCM 块，或属于由 drcIndex
标识的单个 NVDIMM 的所有 SCM 块。

**H_SCM_HEALTH**

| 输入： drcIndex
| 输出： **health-bitmap (r4), health-bit-valid-bitmap (r5)**
| 返回值： **H_Success, H_Parameter, H_Hardware**

给定一个 DRC 索引，返回 PMEM 设备的预测性故障和整体健康信息。health-bitmap 中置位的
位指示 PMEM 设备的一个或多个状态（如下表所述），health-bit-valid-bitmap 指示
health-bitmap 中的哪些位有效。位以逆序位序报告，例如值 0xC400000000000000 表示位
0、1 和 5 有效。

健康位图标志：

+------+-----------------------------------------------------------------------+
|  Bit |               Definition                                              |
+======+=======================================================================+
|  00  |  PMEM 设备无法持久化内存内容。如果系统断电，则不会保存任何内容。      |
|      |                                                                       |
+------+-----------------------------------------------------------------------+
|  01  |  PMEM 设备未能持久化内存内容。要么断电时内容未成功保存，要么上电时未  |
|      |  正确恢复。                                                            |
+------+-----------------------------------------------------------------------+
|  02  |  PMEM 设备内容已从先前的 IPL 持久化。上次启动的数据已成功恢复。        |
|      |                                                                       |
+------+-----------------------------------------------------------------------+
|  03  |  PMEM 设备内容未从先前的 IPL 持久化。上次启动没有可恢复的数据。        |
|      |                                                                       |
+------+-----------------------------------------------------------------------+
|  04  |  PMEM 设备剩余内存寿命极低                                            |
+------+-----------------------------------------------------------------------+
|  05  |  由于故障，PMEM 设备将在下次 IPL 时被隔离（garded off）               |
+------+-----------------------------------------------------------------------+
|  06  |  由于当前平台健康状态，PMEM 设备内容无法持久化。硬件故障可能阻止数据  |
|      |  的保存或恢复。                                                        |
+------+-----------------------------------------------------------------------+
|  07  |  在某些条件下 PMEM 设备无法持久化内存内容                             |
+------+-----------------------------------------------------------------------+
|  08  |  PMEM 设备已加密                                                      |
+------+-----------------------------------------------------------------------+
|  09  |  PMEM 设备已成功完成请求的擦除或安全擦除过程。                        |
|      |                                                                       |
+------+-----------------------------------------------------------------------+
|10:63 |  保留 / 未使用                                                        |
+------+-----------------------------------------------------------------------+

**H_SCM_PERFORMANCE_STATS**

| 输入： drcIndex, resultBuffer Addr
| 输出： None
| 返回值：  **H_Success, H_Parameter, H_Unsupported, H_Hardware, H_Authority, H_Privilege**

给定一个 DRC 索引，收集 NVDIMM 的性能统计信息并将其复制到 resultBuffer。

**H_SCM_FLUSH**

| 输入： **drcIndex, continue-token**
| 输出： **continue-token**
| 返回值： **H_SUCCESS, H_Parameter, H_P2, H_BUSY**

给定一个 DRC 索引，将数据刷新到后端 NVDIMM 设备。

当刷新耗时较长时，hcall 返回 H_BUSY，并且需要多次发出该 hcall 才能被完全处理。来自
输出的 **continue-token** 应传入后续发给 hypervisor 的 hcall 的参数列表中，直到 hcall
被完全处理，此时 hypervisor 返回 H_SUCCESS 或其他错误。

**H_HTM**

| 输入： flags, target, operation (op), op-param1, op-param2, op-param3
| 输出： **dumphtmbufferdata**
| 返回值： *H_Success,H_Busy,H_LongBusyOrder,H_Partial,H_Parameter,
		 H_P2,H_P3,H_P4,H_P5,H_P6,H_State,H_Not_Available,H_Authority*

H_HTM 支持硬件跟踪宏（Hardware Trace Macro，HTM）功能及其数据的设置、配置、控制和
转储。HTM 缓冲区存储核心指令、核心 LLAT 和 nest 等功能的跟踪数据。

**H_PKS_GEN_KEY**

| 输入： authorization, objectlabel, objectlabellen, policy, out, outlen
| 输出： **Hypervisor 生成的密钥，或当设置了包装密钥策略时为 None**
| 返回值： *H_SUCCESS, H_Function, H_State, H_R_State, H_Parameter, H_P2,
                H_P3, H_P4, H_P5, H_P6, H_Authority, H_Nomem, H_Busy, H_Resource,
                H_Aborted*

H_PKS_GEN_KEY 用于让 hypervisor 生成一个新随机密钥。该密钥作为对象存储在 Power LPAR
平台密钥库（Platform KeyStore）中，带有提供的对象标签。设置包装密钥策略后，该密钥仅
对 hypervisor 可见，而密钥的标签对用户仍可见。包装密钥的生成仅支持 32 字节的密钥大小。

**H_PKS_WRAP_OBJECT**

| 输入： authorization, wrapkeylabel, wrapkeylabellen, objectwrapflags, in,
|        inlen, out, outlen, continue-token
| 输出： **continue-token, 包装后对象的字节大小, 包装后的对象**
| 返回值： *H_SUCCESS, H_Function, H_State, H_R_State, H_Parameter, H_P2,
                H_P3, H_P4, H_P5, H_P6, H_P7, H_P8, H_P9, H_Authority, H_Invalid_Key,
                H_NOT_FOUND, H_Busy, H_LongBusy, H_Aborted*

H_PKS_WRAP_OBJECT 用于使用存储在 Power LPAR 平台密钥库中的包装密钥对对象进行包装，并将
包装后的对象返回给调用者。调用者提供带有 'wrapping key' 策略设置的包装密钥标签，该密钥
必须已使用 H_PKS_GEN_KEY 预先创建。然后对提供的对象使用包装密钥和附加元数据进行加密，
并将结果返回给调用者。

**H_PKS_UNWRAP_OBJECT**

| 输入： authorization, objectwrapflags, in, inlen, out, outlen, continue-token
| 输出： **continue-token, 解包后对象的字节大小, 解包后的对象**
| 返回值： *H_SUCCESS, H_Function, H_State, H_R_State, H_Parameter, H_P2,
                H_P3, H_P4, H_P5, H_P6, H_P7, H_Authority, H_Unsupported, H_Bad_Data,
                H_NOT_FOUND, H_Invalid_Key, H_Busy, H_LongBusy, H_Aborted*

H_PKS_UNWRAP_OBJECT 用于解包先前使用 H_PKS_WRAP_OBJECT 包装的对象。

## 参考文献

       https://en.wikipedia.org/wiki/Power_Architecture_Platform_Reference
       https://members.openpowerfoundation.org/document/dl/469
       https://openpowerfoundation.org/?resource_lib=power-isa-version-3-0
       https://openpowerfoundation.org/?resource_lib=64-bit-elf-v2-abi-specification-power-architecture
