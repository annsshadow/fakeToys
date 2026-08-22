
## POWER 上的嵌套 KVM


## 简

本文档解释了一个客户机操作系统如何在管理程序实现了相关超级调用（hypercall）的情况下，充当管理程序并通过使用超级调用来运行嵌套客户机。术L0、L1 L2 用于指代不同的软件实体。L0 是处于管理程序模式的实体，通常被称为“宿主机（host）”或“管理程序（hypervisor）”。L1 是一个直接在 L0 之下运行、由 L0 发起并控制的客户机虚拟机。L2 是一个由充当管理程序L1 发起并控制的客户机虚拟机
## 现有 API


Linux/KVM 2018 年起就支持作L0 L1 进行嵌套（Nesting
```

   commit 8e3f5fc1045dc49fd175b978c5457f5f51e7a2ce
   Author: Paul Mackerras <paulus@ozlabs.org>
   Date:   Mon Oct 8 16:31:03 2018 +1100
   KVM: PPC: Book3S HV: Framework and hcall stubs for nested virtualization

```
```
   commit 360cae313702cdd0b90f82c261a8302fecef030a
   Author: Paul Mackerras <paulus@ozlabs.org>
   Date:   Mon Oct 8 16:31:04 2018 +1100
   KVM: PPC: Book3S HV: Nested guest entry via hypercall

```
API 主要借助单一的超级调h_enter_nested() 工作。该调用L1 发出，用以告L0 以给定状态启动一L2 vCPU。随L0 启动这个 L2 并运行，直到达到一L2 退出条件。一L2 退出，L2 的状态就L0 交还L1。每L2 运行时，完整L2 vCPU 状态总是L1 之间来回传递。L0 不在 L2 vCPU 上保留任何状态（除了L0 L1 -> L2 进入L2 -> L1 退出的短暂序列期间）
L0 保留的唯一状态是分区表（partition table）。L1 使用 h_set_partition_table() 超级调用注册它的分区表。L0 持有的关L2 的所有其它状态都是被缓存的状态（例如影子页表）
L1 可以在不事先通知 L0 的情况下运行任何 L2 vCPU。它只需使用 h_enter_nested() 启动 vCPU 即可。L2 vCPU 的创建在每次调用 h_enter_nested() 时隐式完成
在本文档中，我们称这个现API v1 API
## 新的 PAPR API


新的 PAPR API v1 API 的不同之处在于：创建 L2 及其关联vCPU 是显式的。在本文档中，我们称之为 v2 API
h_enter_nested() H_GUEST_VCPU_RUN() 取代。在这之前，L1 必须使用 h_guest_create() 显式地创L2，并使用 h_guest_create_vCPU() 创建任何关联vCPU。获取与设置 vCPU 状态也可以使用 h_guest_{g|s}et 超级调用完成
L1 创建一L2、运行它并删除它的基本执行流程是
- L1 L0 通过 H_GUEST_{G,S}ET_CAPABILITIES() 协商能力（通常L1 启动时）
- L1 请求 L0 H_GUEST_CREATE() 创建一L2，并收到一个令牌（token
- L1 请求 L0 H_GUEST_CREATE_VCPU() 创建一L2 vCPU

- L1 L0 使用 H_GUEST_{G,S}ET() 超级调用沟vCPU 状
- L1 请求 L0 通过运行 H_GUEST_VCPU_RUN() 超级调用来运行该 vCPU

- L1 H_GUEST_DELETE() 删除 L2

关于各个超级调用的更多细节如下：

## HCALL 详情


提供本文档是为了让人API 有一个整体的理解。它并不旨在提供实现一L1 L0 所需的全部细节。更多细节可参考最新版本的 PAPR
所有这HCALL 都由 L1 L0 发出
### H_GUEST_GET_CAPABILITIES()


此调用用于获L0 嵌套管理程序的能力。这包括诸如 CPU 版本之类的能力（例如
```

  H_GUEST_GET_CAPABILITIES(uint64 flags)

  Parameters:
    Input:
      flags: Reserved
    Output:
      R3: Return code
      R4: Hypervisor Supported Capabilities bitmap 1

```
### H_GUEST_SET_CAPABILITIES()


此调用用于将 L1 管理程序的能力告L0。这里传入的标志集合H_GUEST_GET_CAPABILITIES() 相同
通常，先调用 GET，然后再用从 GET 返回的标志子集调SET。这一过程允许 L0 ```

  H_GUEST_SET_CAPABILITIES(uint64 flags,
                           uint64 capabilitiesBitmap1)
  Parameters:
    Input:
      flags: Reserved
      capabilitiesBitmap1: Only capabilities advertised through
                           H_GUEST_GET_CAPABILITIES
    Output:
      R3: Return code
      R4: If R3 = H_P2: The number of invalid bitmaps
      R5: If R3 = H_P2: The index of first invalid bitmap

```
### H_GUEST_CREATE()


此调用用于创建一L2。会返回所创建 L2 的唯一 ID（类似于一LPID），可在后续 HCALL 中使用它```

  H_GUEST_CREATE(uint64 flags,
                 uint64 continueToken);
  Parameters:
    Input:
      flags: Reserved
      continueToken: Initial call set to -1. Subsequent calls,
                     after H_Busy or H_LongBusyOrder has been
                     returned, value that was returned in R4.
    Output:
      R3: Return code. Notable:
        H_Not_Enough_Resources: Unable to create Guest VCPU due to not
        enough Hypervisor memory. See H_GUEST_CREATE_GET_STATE(flags =
        takeOwnershipOfVcpuState)
      R4: If R3 = H_Busy or_H_LongBusyOrder -> continueToken

```
### H_GUEST_CREATE_VCPU()


此调用用于创建一个与 L2 关联vCPU。应当传L2 id（从 H_GUEST_CREATE() 返回）。同时传入的还有一个（对此 L2 而言）唯一vCPUid。这vCPUid ```

  H_GUEST_CREATE_VCPU(uint64 flags,
                      uint64 guestId,
                      uint64 vcpuId);
  Parameters:
    Input:
      flags: Reserved
      guestId: ID obtained from H_GUEST_CREATE
      vcpuId: ID of the vCPU to be created. This must be within the
              range of 0 to 2047
    Output:
      R3: Return code. Notable:
        H_Not_Enough_Resources: Unable to create Guest VCPU due to not
        enough Hypervisor memory. See H_GUEST_CREATE_GET_STATE(flags =
        takeOwnershipOfVcpuState)

```
### H_GUEST_GET_STATE()


此调用用于获取与 L2 关联的状态（客户机级vCPU 特定）。该信息通过客户机状态缓冲区（GSB）传递，它是一种标准格式，如本文档后面所解释，必要细节如下：

这可以获L2 级或 vCPU 特定的信息。L2 级的例子有时基偏移或进程作用域页表信息。vCPU 特定的例子有 GPR VSR。flags 参数中的一个位指明此调用是 L2 级还vCPU 特定的，并且 GSB 中的 ID 必须与之匹配
L1 提供一个指GSB 的指针作为此调用的参数。同时提供的还有与要设置的状态关联的 L2 vCPU ID
L1 只在 GSB 中写ID 与大小。L0 写入
```

  H_GUEST_GET_STATE(uint64 flags,
                           uint64 guestId,
                           uint64 vcpuId,
                           uint64 dataBuffer,
                           uint64 dataBufferSizeInBytes);
  Parameters:
    Input:
      flags:
         Bit 0: getGuestWideState: Request state of the Guest instead
           of an individual VCPU.
         Bit 1: getHostWideState: Request stats of the Host. This causes
           the guestId and vcpuId parameters to be ignored and attempting
           to get the VCPU/Guest state will cause an error.
         Bits 2-63: Reserved
      guestId: ID obtained from H_GUEST_CREATE
      vcpuId: ID of the vCPU pass to H_GUEST_CREATE_VCPU
      dataBuffer: A L1 real address of the GSB.
        If takeOwnershipOfVcpuState, size must be at least the size
        returned by ID=0x0001
      dataBufferSizeInBytes: Size of dataBuffer
    Output:
      R3: Return code
      R4: If R3 = H_Invalid_Element_Id: The array index of the bad
            element ID.
          If R3 = H_Invalid_Element_Size: The array index of the bad
             element size.
          If R3 = H_Invalid_Element_Value: The array index of the bad
             element value.

```
### H_GUEST_SET_STATE()


此调用用于设L2 级或 vCPU 特定L2 状态。该信息通过客户机状态缓冲区（GSB）传递，必要细节如下
这可以设L2 级或 vCPU 特定的信息。L2 级的例子有时基偏移或进程作用域页表信息。vCPU 特定的例子有 GPR VSR。flags 参数中的一个位指明此调用是 L2 级还vCPU 特定的，并且 GSB 中的 ID 必须与之匹配
L1 提供一个指GSB 的指针作为此调用的参数。同时提供的还有与要设置的状态关联的 L2 vCPU ID
L1 GSB 中写入所有值，L0 只读GSB 中的
```

  H_GUEST_SET_STATE(uint64 flags,
                    uint64 guestId,
                    uint64 vcpuId,
                    uint64 dataBuffer,
                    uint64 dataBufferSizeInBytes);
  Parameters:
    Input:
      flags:
         Bit 0: getGuestWideState: Request state of the Guest instead
           of an individual VCPU.
         Bit 1: returnOwnershipOfVcpuState Return Guest VCPU state. See
           GET_STATE takeOwnershipOfVcpuState
         Bits 2-63: Reserved
      guestId: ID obtained from H_GUEST_CREATE
      vcpuId: ID of the vCPU pass to H_GUEST_CREATE_VCPU
      dataBuffer: A L1 real address of the GSB.
        If takeOwnershipOfVcpuState, size must be at least the size
        returned by ID=0x0001
      dataBufferSizeInBytes: Size of dataBuffer
    Output:
      R3: Return code
      R4: If R3 = H_Invalid_Element_Id: The array index of the bad
            element ID.
          If R3 = H_Invalid_Element_Size: The array index of the bad
             element size.
          If R3 = H_Invalid_Element_Value: The array index of the bad
             element value.

```
### H_GUEST_RUN_VCPU()


此调用用于运行一L2 vCPU。L2 vCPU ID 作为参数传入。该 vCPU 以之前使H_GUEST_SET_STATE() 设置的状态运行。当 L2 退出时，L1 将从这个超级调用处恢复执行
这个超级调用还有关联的输入与输出 GSB。与 H_GUEST_{S,G}ET_STATE() 不同，这GSB 指针不是作为超级调用的参数传入的（这样做是出于性能考虑）。这GSB 的位置必须使H_GUEST_SET_STATE() 调用、以 ID 0x0c00 0x0c01（见下表）预先注册
输入 GSB 可能只包含要设置vCPU 特定元素。这GSB 也可以包含零个元素（GSB 4 字节0），如果无需设置任何东西的话
从超级调用退出时，输出缓冲区被填入由 L0 决定的元素。退出的原因包含GPR4 中（NIP 被放GPR4）。返回的元素取决于退出类型。例如，如果退出原因是 L2 执行了一个超级调用（GPR4 = 0xc00），那么 GPR3-12 会被提供在输GSB 中，因为这是服务该超级调用可能需要的状态。如果需要额外的状态，L1 可以调用 H_GUEST_GET_STATE()
要在 L2 中合成中断，当调H_GUEST_RUN_VCPU() 时，L1 可以设置一个标志（作为超级调用参数），L0 就会L2 中合成该中断。或者，L1 也可以使H_GUEST_SET_STATE() 自行合成中断，或
```

  H_GUEST_RUN_VCPU(uint64 flags,
                   uint64 guestId,
                   uint64 vcpuId,
                   uint64 dataBuffer,
                   uint64 dataBufferSizeInBytes);
  Parameters:
    Input:
      flags:
         Bit 0: generateExternalInterrupt: Generate an external interrupt
         Bit 1: generatePrivilegedDoorbell: Generate a Privileged Doorbell
         Bit 2: sendToSystemReset鈥? Generate a System Reset Interrupt
         Bits 3-63: Reserved
      guestId: ID obtained from H_GUEST_CREATE
      vcpuId: ID of the vCPU pass to H_GUEST_CREATE_VCPU
    Output:
      R3: Return code
      R4: If R3 = H_Success: The reason L1 VCPU exited (ie. NIA)
            0x000: The VCPU stopped running for an unspecified reason. An
              example of this is the Hypervisor stopping a VCPU running
              due to an outstanding interrupt for the Host Partition.
            0x980: HDEC
            0xC00: HCALL
            0xE00: HDSI
            0xE20: HISI
            0xE40: HEA
            0xF80: HV Fac Unavail
          If R3 = H_Invalid_Element_Id, H_Invalid_Element_Size, or
            H_Invalid_Element_Value: R4 is offset of the invalid element
            in the input buffer.

```
### H_GUEST_DELETE()


此调用用于删除一L2。所有关联的 vCPU 也会被删除。不提供单独vCPU 删除调用
可以提供一个标志来删除所有客户机。这用于重置
```

  H_GUEST_DELETE(uint64 flags,
                 uint64 guestId)
  Parameters:
    Input:
      flags:
         Bit 0: deleteAllGuests: deletes all guests
         Bits 1-63: Reserved
      guestId: ID obtained from H_GUEST_CREATE
    Output:
      R3: Return code

```
## 客户机状态缓冲区


客户机状态缓冲区（GSB）是 L1 L0 之间通过 H_GUEST_{G,S}ET() H_GUEST_VCPU_RUN() 调用沟L2 状态的主要方法
状态可以与整个 L2 关联（例如时基偏移），也可以与特定的 L2 vCPU 关联（例GPR 状态）。只L2 VCPU 状态可能由 H_GUEST_VCPU_RUN() 设置
GSB 中的所有数据都是大端（big endian）的（与 PAPR 中的标准一致）
客户机状态缓冲区有一个头部，给出元素的数量，随后GSB 元素本身
GSB 头部
+----------+----------+-------------------------------------------+
|  Offset  |  Size    |  Purpose                                  |
|  Bytes   |  Bytes   |                                           |
+==========+==========+===========================================+
|    0     |    4     |  Number of elements                       |
+----------+----------+-------------------------------------------+
|    4     |          |  Guest state buffer elements              |
+----------+----------+-------------------------------------------+

GSB 元素
+----------+----------+-------------------------------------------+
|  Offset  |  Size    |  Purpose                                  |
|  Bytes   |  Bytes   |                                           |
+==========+==========+===========================================+
|    0     |    2     |  ID                                       |
+----------+----------+-------------------------------------------+
|    2     |    2     |  Size of Value                            |
+----------+----------+-------------------------------------------+
|    4     | As above |  Value                                    |
+----------+----------+-------------------------------------------+

GSB 元素中的 ID 指定了要设置什么。这包括架构状态（GPR、VSR、SPR），以及一些关于分区的元数据，如时基偏移与分区作用域页表信息
+--------+-------+----+--------+----------------------------------+
|   ID   | Size  | RW |(H)ost  | Details                          |
|        | Bytes |    |(G)uest |                                  |
|        |       |    |(T)hread|                                  |
|        |       |    |Scope   |                                  |
+========+=======+====+========+==================================+
| 0x0000 |       | RW |   TG   | NOP element                      |
+--------+-------+----+--------+----------------------------------+
| 0x0001 | 0x08  | R  |   G    | Size of L0 vCPU state. See:      |
|        |       |    |        | H_GUEST_GET_STATE:               |
|        |       |    |        | flags = takeOwnershipOfVcpuState |
+--------+-------+----+--------+----------------------------------+
| 0x0002 | 0x08  | R  |   G    | Size Run vCPU out buffer         |
+--------+-------+----+--------+----------------------------------+
| 0x0003 | 0x04  | RW |   G    | Logical PVR                      |
+--------+-------+----+--------+----------------------------------+
| 0x0004 | 0x08  | RW |   G    | TB Offset (L1 relative)          |
+--------+-------+----+--------+----------------------------------+
| 0x0005 | 0x18  | RW |   G    |Partition scoped page tbl info:   |
|        |       |    |        |                                  |
|        |       |    |        |- 0x00 Addr part scope table      |
|        |       |    |        |- 0x08 Num addr bits              |
|        |       |    |        |- 0x10 Size root dir              |
+--------+-------+----+--------+----------------------------------+
| 0x0006 | 0x10  | RW |   G    |Process Table Information:        |
|        |       |    |        |                                  |
|        |       |    |        |- 0x0 Addr proc scope table       |
|        |       |    |        |- 0x8 Table size.                 |
+--------+-------+----+--------+----------------------------------+
| 0x0007-|       |    |        | Reserved                         |
| 0x07FF |       |    |        |                                  |
+--------+-------+----+--------+----------------------------------+
| 0x0800 | 0x08  | R  |   H    | Current usage in bytes of the    |
|        |       |    |        | L0's Guest Management Space      |
|        |       |    |        | for an L1-Lpar.                  |
+--------+-------+----+--------+----------------------------------+
| 0x0801 | 0x08  | R  |   H    | Max bytes available in the       |
|        |       |    |        | L0's Guest Management Space for  |
|        |       |    |        | an L1-Lpar                       |
+--------+-------+----+--------+----------------------------------+
| 0x0802 | 0x08  | R  |   H    | Current usage in bytes of the    |
|        |       |    |        | L0's Guest Page Table Management |
|        |       |    |        | Space for an L1-Lpar             |
+--------+-------+----+--------+----------------------------------+
| 0x0803 | 0x08  | R  |   H    | Max bytes available in the L0's  |
|        |       |    |        | Guest Page Table Management      |
|        |       |    |        | Space for an L1-Lpar             |
+--------+-------+----+--------+----------------------------------+
| 0x0804 | 0x08  | R  |   H    | Cumulative Reclaimed bytes from  |
|        |       |    |        | L0 Guest's Page Table Management |
|        |       |    |        | Space due to overcommit          |
+--------+-------+----+--------+----------------------------------+
| 0x0805-|       |    |        | Reserved                         |
| 0x0BFF |       |    |        |                                  |
+--------+-------+----+--------+----------------------------------+
| 0x0C00 | 0x10  | RW |   T    |Run vCPU Input Buffer:            |
|        |       |    |        |                                  |
|        |       |    |        |- 0x0 Addr of buffer              |
|        |       |    |        |- 0x8 Buffer Size.                |
+--------+-------+----+--------+----------------------------------+
| 0x0C01 | 0x10  | RW |   T    |Run vCPU Output Buffer:           |
|        |       |    |        |                                  |
|        |       |    |        |- 0x0 Addr of buffer              |
|        |       |    |        |- 0x8 Buffer Size.                |
+--------+-------+----+--------+----------------------------------+
| 0x0C02 | 0x08  | RW |   T    | vCPU VPA Address                 |
+--------+-------+----+--------+----------------------------------+
| 0x0C03-|       |    |        | Reserved                         |
| 0x0FFF |       |    |        |                                  |
+--------+-------+----+--------+----------------------------------+
| 0x1000-| 0x08  | RW |   T    | GPR 0-31                         |
| 0x101F |       |    |        |                                  |
+--------+-------+----+--------+----------------------------------+
| 0x1020 |  0x08 | T  |   T    | HDEC expiry TB                   |
+--------+-------+----+--------+----------------------------------+
| 0x1021 | 0x08  | RW |   T    | NIA                              |
+--------+-------+----+--------+----------------------------------+
| 0x1022 | 0x08  | RW |   T    | MSR                              |
+--------+-------+----+--------+----------------------------------+
| 0x1023 | 0x08  | RW |   T    | LR                               |
+--------+-------+----+--------+----------------------------------+
| 0x1024 | 0x08  | RW |   T    | XER                              |
+--------+-------+----+--------+----------------------------------+
| 0x1025 | 0x08  | RW |   T    | CTR                              |
+--------+-------+----+--------+----------------------------------+
| 0x1026 | 0x08  | RW |   T    | CFAR                             |
+--------+-------+----+--------+----------------------------------+
| 0x1027 | 0x08  | RW |   T    | SRR0                             |
+--------+-------+----+--------+----------------------------------+
| 0x1028 | 0x08  | RW |   T    | SRR1                             |
+--------+-------+----+--------+----------------------------------+
| 0x1029 | 0x08  | RW |   T    | DAR                              |
+--------+-------+----+--------+----------------------------------+
| 0x102A | 0x08  | RW |   T    | DEC expiry TB                    |
+--------+-------+----+--------+----------------------------------+
| 0x102B | 0x08  | RW |   T    | VTB                              |
+--------+-------+----+--------+----------------------------------+
| 0x102C | 0x08  | RW |   T    | LPCR                             |
+--------+-------+----+--------+----------------------------------+
| 0x102D | 0x08  | RW |   T    | HFSCR                            |
+--------+-------+----+--------+----------------------------------+
| 0x102E | 0x08  | RW |   T    | FSCR                             |
+--------+-------+----+--------+----------------------------------+
| 0x102F | 0x08  | RW |   T    | FPSCR                            |
+--------+-------+----+--------+----------------------------------+
| 0x1030 | 0x08  | RW |   T    | DAWR0                            |
+--------+-------+----+--------+----------------------------------+
| 0x1031 | 0x08  | RW |   T    | DAWR1                            |
+--------+-------+----+--------+----------------------------------+
| 0x1032 | 0x08  | RW |   T    | CIABR                            |
+--------+-------+----+--------+----------------------------------+
| 0x1033 | 0x08  | RW |   T    | PURR                             |
+--------+-------+----+--------+----------------------------------+
| 0x1034 | 0x08  | RW |   T    | SPURR                            |
+--------+-------+----+--------+----------------------------------+
| 0x1035 | 0x08  | RW |   T    | IC                               |
+--------+-------+----+--------+----------------------------------+
| 0x1036-| 0x08  | RW |   T    | SPRG 0-3                         |
| 0x1039 |       |    |        |                                  |
+--------+-------+----+--------+----------------------------------+
| 0x103A | 0x08  | W  |   T    | PPR                              |
+--------+-------+----+--------+----------------------------------+
| 0x103B | 0x08  | RW |   T    | MMCR 0-3                         |
| 0x103E |       |    |        |                                  |
+--------+-------+----+--------+----------------------------------+
| 0x103F | 0x08  | RW |   T    | MMCRA                            |
+--------+-------+----+--------+----------------------------------+
| 0x1040 | 0x08  | RW |   T    | SIER                             |
+--------+-------+----+--------+----------------------------------+
| 0x1041 | 0x08  | RW |   T    | SIER 2                           |
+--------+-------+----+--------+----------------------------------+
| 0x1042 | 0x08  | RW |   T    | SIER 3                           |
+--------+-------+----+--------+----------------------------------+
| 0x1043 | 0x08  | RW |   T    | BESCR                            |
+--------+-------+----+--------+----------------------------------+
| 0x1044 | 0x08  | RW |   T    | EBBHR                            |
+--------+-------+----+--------+----------------------------------+
| 0x1045 | 0x08  | RW |   T    | EBBRR                            |
+--------+-------+----+--------+----------------------------------+
| 0x1046 | 0x08  | RW |   T    | AMR                              |
+--------+-------+----+--------+----------------------------------+
| 0x1047 | 0x08  | RW |   T    | IAMR                             |
+--------+-------+----+--------+----------------------------------+
| 0x1048 | 0x08  | RW |   T    | AMOR                             |
+--------+-------+----+--------+----------------------------------+
| 0x1049 | 0x08  | RW |   T    | UAMOR                            |
+--------+-------+----+--------+----------------------------------+
| 0x104A | 0x08  | RW |   T    | SDAR                             |
+--------+-------+----+--------+----------------------------------+
| 0x104B | 0x08  | RW |   T    | SIAR                             |
+--------+-------+----+--------+----------------------------------+
| 0x104C | 0x08  | RW |   T    | DSCR                             |
+--------+-------+----+--------+----------------------------------+
| 0x104D | 0x08  | RW |   T    | TAR                              |
+--------+-------+----+--------+----------------------------------+
| 0x104E | 0x08  | RW |   T    | DEXCR                            |
+--------+-------+----+--------+----------------------------------+
| 0x104F | 0x08  | RW |   T    | HDEXCR                           |
+--------+-------+----+--------+----------------------------------+
| 0x1050 | 0x08  | RW |   T    | HASHKEYR                         |
+--------+-------+----+--------+----------------------------------+
| 0x1051 | 0x08  | RW |   T    | HASHPKEYR                        |
+--------+-------+----+--------+----------------------------------+
| 0x1052 | 0x08  | RW |   T    | CTRL                             |
+--------+-------+----+--------+----------------------------------+
| 0x1053 | 0x08  | RW |   T    | DPDES                            |
+--------+-------+----+--------+----------------------------------+
| 0x1054-|       |    |        | Reserved                         |
| 0x1FFF |       |    |        |                                  |
+--------+-------+----+--------+----------------------------------+
| 0x2000 | 0x04  | RW |   T    | CR                               |
+--------+-------+----+--------+----------------------------------+
| 0x2001 | 0x04  | RW |   T    | PIDR                             |
+--------+-------+----+--------+----------------------------------+
| 0x2002 | 0x04  | RW |   T    | DSISR                            |
+--------+-------+----+--------+----------------------------------+
| 0x2003 | 0x04  | RW |   T    | VSCR                             |
+--------+-------+----+--------+----------------------------------+
| 0x2004 | 0x04  | RW |   T    | VRSAVE                           |
+--------+-------+----+--------+----------------------------------+
| 0x2005 | 0x04  | RW |   T    | DAWRX0                           |
+--------+-------+----+--------+----------------------------------+
| 0x2006 | 0x04  | RW |   T    | DAWRX1                           |
+--------+-------+----+--------+----------------------------------+
| 0x2007-| 0x04  | RW |   T    | PMC 1-6                          |
| 0x200c |       |    |        |                                  |
+--------+-------+----+--------+----------------------------------+
| 0x200D | 0x04  | RW |   T    | WORT                             |
+--------+-------+----+--------+----------------------------------+
| 0x200E | 0x04  | RW |   T    | PSPB                             |
+--------+-------+----+--------+----------------------------------+
| 0x200F-|       |    |        | Reserved                         |
| 0x2FFF |       |    |        |                                  |
+--------+-------+----+--------+----------------------------------+
| 0x3000-| 0x10  | RW |   T    | VSR 0-63                         |
| 0x303F |       |    |        |                                  |
+--------+-------+----+--------+----------------------------------+
| 0x3040-|       |    |        | Reserved                         |
| 0xEFFF |       |    |        |                                  |
+--------+-------+----+--------+----------------------------------+
| 0xF000 | 0x08  | R  |   T    | HDAR                             |
+--------+-------+----+--------+----------------------------------+
| 0xF001 | 0x04  | R  |   T    | HDSISR                           |
+--------+-------+----+--------+----------------------------------+
| 0xF002 | 0x04  | R  |   T    | HEIR                             |
+--------+-------+----+--------+----------------------------------+
| 0xF003 | 0x08  | R  |   T    | ASDR                             |
+--------+-------+----+--------+----------------------------------+


## 杂项信息


### 不在 ptregs/hvregs 中的状

v1 API 中，某些状态不ptregs/hvstate 中。这包括向量寄存器与某些 SPR。为了让 L1 L2 设置此状态，L1 h_enter_nested() 调用之前载入这些硬件寄存器，L0 确保它们最终成L2 状态（通过不去触碰它们）
v2 API 移除了这一点，并通过 GSB 显式地设置此状态
### L1 实现细节：缓存状

v1 API 中，所有状态都在每h_enter_nested() 超级调用时从 L1 发往 L0，反之亦然。如L0 当前没有运行任何 L2，L0 就没有关于它们的状态信息。唯一的例外是通过 h_set_partition_table() 注册的分区表的位置
v2 API 改变了这一点，使得 L0 即使在它vCPU 不再运行时也保留 L2 状态。这意味着 L1 只需要在需要修L2 状态、或者它的值已过期时，才与 L0 沟L2 状态。这提供了一个性能优化的机会
当一vCPU H_GUEST_RUN_VCPU() 调用退出时，L1 在内部将所L2 状态标记为无效。这意味着如果 L1 想要知道 L2 状态（比如通过 kvm_get_one_reg() 调用），它需要调H_GUEST_GET_STATE() 来获取该状态。一旦读取，它在 L1 中被标记为有效，直到 L2 再次运行
此外，当 L1 修改 L2 vCPU 状态时，它不需要在 L2 vCPU 再次运行之前把它写入 L0。因此当 L1 更新状态（比如通过 kvm_set_one_reg() 调用）时，它写入一个内部的 L1 副本，并且只L2 通过 H_GUEST_VCPU_RUN() 输入缓冲区再次运行时，才把这个副本刷新到 L0
L1 这种惰性更新状态的做法避免了不必要H_GUEST_{G|S}ET_STATE() 调用