## 引导 AArch64 Linux

作者：Will Deacon <will.deacon@arm.com>

日期：2012 年 9 月 7 日

本文档基于 Russell King 撰写的 ARM 引导文档，适用于 AArch64 Linux 内核的所有公开版本。

AArch64 异常模型由若干异常级别（EL0 - EL3）构成，其中 EL0、EL1 和 EL2 各自拥有一个安全与非安全副本。EL2 是 hypervisor（虚拟机监视器）级别，EL3 是最高优先级级别，且仅存在于安全模式。两者在架构上均为可选项。

在本文档中，我们使用术语 `boot loader`（引导加载程序）来泛指在控制权移交给 Linux 内核之前，在 CPU 上执行的所有软件。这可能包括安全监视器（secure monitor）和 hypervisor 代码，也可能仅仅是用于准备最小引导环境的少数几条指令。

本质上，引导加载程序至少应提供以下内容：

1. 建立并初始化 RAM
2. 建立设备树（device tree）
3. 解压内核镜像
4. 调用内核镜像

### 1. 建立并初始化 RAM

要求：强制（MANDATORY）

引导加载程序应当找到并初始化内核在系统中用于存储易失性数据的所有 RAM。它以与机器相关的方式完成这项工作。（它可以采用内部算法自动定位并测算所有 RAM 的大小，也可以利用机器中 RAM 的相关信息，或采用引导加载程序设计者认为合适的任何其他方式。）

对于 Arm 机密计算域（Confidential Compute Realms），这包括确保所有受保护 RAM 具有 "RAM" 状态的 Realm IPA 状态（RIPAS）。

### 2. 建立设备树

要求：强制（MANDATORY）

设备树 blob（dtb）必须放置在 8 字节对齐的边界上，且大小不得超过 2 兆字节。由于 dtb 将以最大 2 兆字节大小的块被映射为可缓存（cacheable），因此它不能放置在必须使用特定属性进行映射的任何 2M 区域内。

注意：v4.2 之前的版本还要求 DTB 放置在从内核 Image 下方 text_offset 字节处开始的 512 MB 区域内。

### 3. 解压内核镜像

要求：可选（OPTIONAL）

AArch64 内核目前不提供解压程序，因此如果使用压缩的 Image 目标（例如 Image.gz），则需要由引导加载程序执行解压（gzip 等）。对于未实现此要求的引导加载程序，可使用未压缩的 Image 目标作为替代。

### 4. 调用内核镜像

要求：强制（MANDATORY）

```

  u32 code0;			/* Executable code */
  u32 code1;			/* Executable code */
  u64 text_offset;		/* Image load offset, little endian */
  u64 image_size;		/* Effective Image size, little endian */
  u64 flags;			/* kernel flags, little endian */
  u64 res2	= 0;		/* reserved */
  u64 res3	= 0;		/* reserved */
  u64 res4	= 0;		/* reserved */
  u32 magic	= 0x644d5241;	/* Magic number, little endian, "ARM\x64" */
  u32 res5;			/* reserved (used for PE COFF offset) */


```
头部注记：

- 自 v3.17 起，除非另有说明，所有字段均为小端（little endian）。

- code0/code1 负责跳转到 stext。

- 通过 EFI 引导时，code0/code1 最初会被跳过。res5 是到 PE 头部的偏移量，PE 头部包含 EFI 入口点（efi_stub_entry）。当 stub 完成其工作后，会跳转到 code0 以恢复正常引导流程。

- 在 v3.17 之前，text_offset 的字节序未被规定。在这些情况下 image_size 为零，text_offset 为内核字节序下的 0x80000。当 image_size 非零时，image_size 为小端，必须被遵守。当 image_size 为零时，可假定 text_offset 为 0x80000。

- flags 字段（在 v3.17 中引入）是一个小端 64 位字段，构成如下：

  ============= ===============================================================
  Bit 0		内核字节序。BE 为 1，LE 为 0。
  Bit 1-2	内核页大小。

   - 0 - 未指定。
   - 1 - 4K
   - 2 - 16K
   - 3 - 64K
  Bit 3		内核物理放置位置

			0
			  2MB 对齐的基址应尽可能接近 DRAM 的基址，因为其下方
			  的内存无法通过线性映射访问
			1
			  2MB 对齐的基址，使得从镜像起始处开始计数的所有
			  image_size 字节都位于物理内存的 48 位可寻址范围内
  Bits 4-63	保留。
  ============= ===============================================================

- 当 image_size 为零时，引导加载程序应尝试在内核镜像结束之后，尽可能多地将内存保留给内核使用。所需空间大小取决于所选特性，实际上没有上限。

Image 必须放置在与系统 RAM 中任意位置 2MB 对齐基址相距 text_offset 字节处，并在该处被调用。2MB 对齐基址与镜像起始位置之间的区域对内核没有特殊意义，可用于其他用途。从镜像起始处开始至少 image_size 字节必须可供内核使用。
注意：v4.6 之前的版本无法使用 Image 物理偏移下方的内存，因此建议将 Image 放置得尽可能接近系统 RAM 的起始位置。

如果在引导时将 initrd/initramfs 传递给内核，它必须完整位于一个 1 GB 对齐、最大 32 GB 大小的物理内存窗口内，并且该窗口也要完整覆盖内核 Image。

任何描述给内核的内存（即使是镜像起始位置下方的内存），只要未被标记为从内核保留（例如通过设备树中的 memreserve 区域），都将被视为可供内核使用。

在跳入内核之前，必须满足以下条件：

- 使所有具备 DMA 能力的设备静默（quiesce），以免内存被虚假的网络数据包或磁盘数据破坏。这将为你节省大量调试时间。

- 主 CPU 通用寄存器设置：

    - x0 = 系统 RAM 中设备树 blob（dtb）的物理地址。
    - x1 = 0（保留供将来使用）
    - x2 = 0（保留供将来使用）
    - x3 = 0（保留供将来使用）

- CPU 模式

  所有形式的中断必须在 PSTATE.DAIF（Debug、SError、IRQ 和 FIQ）中被屏蔽。
  CPU 必须处于非安全状态，要么处于 EL2（为访问虚拟化扩展，推荐如此），要么处于 EL1。

- 缓存、MMU

  MMU 必须关闭。

  指令缓存可以开启或关闭，并且不得保留与已加载内核镜像对应的任何陈旧条目。

  与已加载内核镜像对应的地址范围必须清洗（clean）到 PoC（缓存一致性点）。在存在系统缓存或其他缓存已启用的一致主控器的情况下，这通常需要通过按 VA 进行缓存维护，而不是集合/路（set/way）操作。
  遵循按 VA 架构缓存维护操作的系统缓存必须被配置，并可以启用。
  不遵循按 VA 架构缓存维护操作的系统缓存（不推荐）必须被配置并禁用。

- 架构定时器

  CNTFRQ 必须被编程为定时器频率，且 CNTVOFF 必须在所有 CPU 上编程为一致的值。如果在 EL1 进入内核，则 CNTHCTL_EL2 必须在可用时设置 EL1PCTEN（bit 0）。

- 一致性（Coherency）

  所有将由内核引导的 CPU 在进入内核时必须属于同一个一致性域。这可能需要 IMPLEMENTATION DEFINED（实现定义）的初始化，以在每个 CPU 上启用接收维护操作。

- 系统寄存器

  在内核镜像将进入的异常级别及其以下，所有可写的架构系统寄存器必须由更高异常级别的软件进行初始化，以防止在 UNKNOWN（未知）状态下执行。

  对于所有系统：
  - 如果 EL3 存在：

    - 在内核执行的所有 CPU 上，SCR_EL3.FIQ 必须具有相同的值。
    - 只要内核在执行，SCR_EL3.FIQ 的值就必须与引导时的值相同。

  - 如果 EL3 存在且内核在 EL2 进入：

    - SCR_EL3.HCE（bit 8）必须被初始化为 0b1。

  对于需在 v5 模式下使用的、带有 GICv5 中断控制器的系统：

  - 如果内核在 EL1 进入且 EL2 存在：

      - ICH_HFGRTR_EL2.ICC_PPI_ACTIVERn_EL1（bit 20）必须被初始化为 0b1。
      - ICH_HFGRTR_EL2.ICC_PPI_PRIORITYRn_EL1（bit 19）必须被初始化为 0b1。
      - ICH_HFGRTR_EL2.ICC_PPI_PENDRn_EL1（bit 18）必须被初始化为 0b1。
      - ICH_HFGRTR_EL2.ICC_PPI_ENABLERn_EL1（bit 17）必须被初始化为 0b1。
      - ICH_HFGRTR_EL2.ICC_PPI_HMRn_EL1（bit 16）必须被初始化为 0b1。
      - ICH_HFGRTR_EL2.ICC_IAFFIDR_EL1（bit 7）必须被初始化为 0b1。
      - ICH_HFGRTR_EL2.ICC_ICSR_EL1（bit 6）必须被初始化为 0b1。
      - ICH_HFGRTR_EL2.ICC_PCR_EL1（bit 5）必须被初始化为 0b1。
      - ICH_HFGRTR_EL2.ICC_HPPIR_EL1（bit 4）必须被初始化为 0b1。
      - ICH_HFGRTR_EL2.ICC_HAPR_EL1（bit 3）必须被初始化为 0b1。
      - ICH_HFGRTR_EL2.ICC_CR0_EL1（bit 2）必须被初始化为 0b1。
      - ICH_HFGRTR_EL2.ICC_IDRn_EL1（bit 1）必须被初始化为 0b1。
      - ICH_HFGRTR_EL2.ICC_APR_EL1（bit 0）必须被初始化为 0b1。

      - ICH_HFGWTR_EL2.ICC_PPI_ACTIVERn_EL1（bit 20）必须被初始化为 0b1。
      - ICH_HFGWTR_EL2.ICC_PPI_PRIORITYRn_EL1（bit 19）必须被初始化为 0b1。
      - ICH_HFGWTR_EL2.ICC_PPI_PENDRn_EL1（bit 18）必须被初始化为 0b1。
      - ICH_HFGWTR_EL2.ICC_PPI_ENABLERn_EL1（bit 17）必须被初始化为 0b1。
      - ICH_HFGWTR_EL2.ICC_ICSR_EL1（bit 6）必须被初始化为 0b1。
      - ICH_HFGWTR_EL2.ICC_PCR_EL1（bit 5）必须被初始化为 0b1。
      - ICH_HFGWTR_EL2.ICC_CR0_EL1（bit 2）必须被初始化为 0b1。
      - ICH_HFGWTR_EL2.ICC_APR_EL1（bit 0）必须被初始化为 0b1。

      - ICH_HFGITR_EL2.GICRCDNMIA（bit 10）必须被初始化为 0b1。
      - ICH_HFGITR_EL2.GICRCDIA（bit 9）必须被初始化为 0b1。
      - ICH_HFGITR_EL2.GICCDDI（bit 8）必须被初始化为 0b1。
      - ICH_HFGITR_EL2.GICCDEOI（bit 7）必须被初始化为 0b1。
      - ICH_HFGITR_EL2.GICCDHM（bit 6）必须被初始化为 0b1。
      - ICH_HFGITR_EL2.GICCDRCFG（bit 5）必须被初始化为 0b1。
      - ICH_HFGITR_EL2.GICCDPEND（bit 4）必须被初始化为 0b1。
      - ICH_HFGITR_EL2.GICCDAFF（bit 3）必须被初始化为 0b1。
      - ICH_HFGITR_EL2.GICCDPRI（bit 2）必须被初始化为 0b1。
      - ICH_HFGITR_EL2.GICCDDIS（bit 1）必须被初始化为 0b1。
      - ICH_HFGITR_EL2.GICCDEN（bit 0）必须被初始化为 0b1。

  - DT 或 ACPI 表必须描述一个 GICv5 中断控制器。

  对于需在 v3 模式下使用的、带有 GICv3 中断控制器的系统：
  - 如果 EL3 存在：

      - ICC_SRE_EL3.Enable（bit 3）必须被初始化为 0b1。
      - ICC_SRE_EL3.SRE（bit 0）必须被初始化为 0b1。
      - ICC_CTLR_EL3.PMHE（bit 6）必须在内核执行的所有 CPU 上设置为相同的值，并且必须在内核的整个生命周期内保持恒定。

  - 如果内核在 EL1 进入：

      - ICC_SRE_EL2.Enable（bit 3）必须被初始化为 0b1
      - ICC_SRE_EL2.SRE（bit 0）必须被初始化为 0b1。

  - DT 或 ACPI 表必须描述一个 GICv3 中断控制器。

  对于需在兼容（v2）模式下使用的、带有 GICv3 中断控制器的系统：

  - 如果 EL3 存在：

      ICC_SRE_EL3.SRE（bit 0）必须被初始化为 0b0。

  - 如果内核在 EL1 进入：

      ICC_SRE_EL2.SRE（bit 0）必须被初始化为 0b0。

  - DT 或 ACPI 表必须描述一个 GICv2 中断控制器。

  对于带有指针认证（pointer authentication）功能的 CPU：

  - 如果 EL3 存在：

    - SCR_EL3.APK（bit 16）必须被初始化为 0b1
    - SCR_EL3.API（bit 17）必须被初始化为 0b1

  - 如果内核在 EL1 进入：

    - HCR_EL2.APK（bit 40）必须被初始化为 0b1
    - HCR_EL2.API（bit 41）必须被初始化为 0b1

  对于带有活动监视器单元 v1（AMUv1）扩展的 CPU：

  - 如果 EL3 存在：

    - CPTR_EL3.TAM（bit 30）必须被初始化为 0b0
    - CPTR_EL2.TAM（bit 30）必须被初始化为 0b0
    - AMCNTENSET0_EL0 必须被初始化为 0b1111
    - AMCNTENSET1_EL0 必须被初始化为平台相关值，对于存在的每个辅助计数器，在相应位上设置 0b1。

  - 如果内核在 EL1 进入：

    - AMCNTENSET0_EL0 必须被初始化为 0b1111
    - AMCNTENSET1_EL0 必须被初始化为平台相关值，对于存在的每个辅助计数器，在相应位上设置 0b1。

  对于带有细粒度陷阱（Fine Grained Traps，FEAT_FGT）扩展的 CPU：

  - 如果 EL3 存在且内核在 EL2 进入：

    - SCR_EL3.FGTEn（bit 27）必须被初始化为 0b1。

  对于带有细粒度陷阱 2（FEAT_FGT2）扩展的 CPU：

  - 如果 EL3 存在且内核在 EL2 进入：

    - SCR_EL3.FGTEn2（bit 59）必须被初始化为 0b1。

  对于带有 HCRX_EL2 支持（FEAT_HCX）的 CPU：

  - 如果 EL3 存在且内核在 EL2 进入：

    - SCR_EL3.HXEn（bit 38）必须被初始化为 0b1。

  对于带有高级 SIMD 和浮点支持的 CPU：

  - 如果 EL3 存在：

    - CPTR_EL3.TFP（bit 10）必须被初始化为 0b0。

  - 如果 EL2 存在且内核在 EL1 进入：

    - CPTR_EL2.TFP（bit 10）必须被初始化为 0b0。

  对于带有可伸缩向量扩展（Scalable Vector Extension，FEAT_SVE）的 CPU：

  - 如果 EL3 存在：

    - CPTR_EL3.EZ（bit 8）必须被初始化为 0b1。

    - ZCR_EL3.LEN 必须在内核执行的所有 CPU 上初始化为相同的值。

  - 如果内核在 EL1 进入且 EL2 存在：

    - CPTR_EL2.TZ（bit 8）必须被初始化为 0b0。

    - CPTR_EL2.ZEN（bits 17:16）必须被初始化为 0b11。

    - ZCR_EL2.LEN 必须初始化为内核将执行的所有 CPU 上相同的值。

  对于带有可伸缩矩阵扩展（Scalable Matrix Extension，FEAT_SME）的 CPU：

  - 如果 EL3 存在：

    - CPTR_EL3.ESM（bit 12）必须被初始化为 0b1。

    - SCR_EL3.EnTP2（bit 41）必须被初始化为 0b1。

    - SMCR_EL3.LEN 必须初始化为内核将执行的所有 CPU 上相同的值。

 - 如果内核在 EL1 进入且 EL2 存在：

    - CPTR_EL2.TSM（bit 12）必须被初始化为 0b0。

    - CPTR_EL2.SMEN（bits 25:24）必须被初始化为 0b11。

    - SCTLR_EL2.EnTP2（bit 60）必须被初始化为 0b1。

    - SMCR_EL2.LEN 必须初始化为内核将执行的所有 CPU 上相同的值。

    - HFGRTR_EL2.nTPIDR2_EL0（bit 55）必须被初始化为 0b01。

    - HFGWTR_EL2.nTPIDR2_EL0（bit 55）必须被初始化为 0b01。

    - HFGRTR_EL2.nSMPRI_EL1（bit 54）必须被初始化为 0b01。

    - HFGWTR_EL2.nSMPRI_EL1（bit 54）必须被初始化为 0b01。

  对于带有可伸缩矩阵扩展 FA64 特性（FEAT_SME_FA64）的 CPU：

  - 如果 EL3 存在：

    - SMCR_EL3.FA64（bit 31）必须被初始化为 0b1。

 - 如果内核在 EL1 进入且 EL2 存在：

    - SMCR_EL2.FA64（bit 31）必须被初始化为 0b1。

  对于带有内存标记扩展特性（FEAT_MTE2）的 CPU：

  - 如果 EL3 存在：

    - SCR_EL3.ATA（bit 26）必须被初始化为 0b1。

  - 如果内核在 EL1 进入且 EL2 存在：

    - HCR_EL2.ATA（bit 56）必须被初始化为 0b1。

  对于带有可伸缩矩阵扩展版本 2（FEAT_SME2）的 CPU：

  - 如果 EL3 存在：

    - SMCR_EL3.EZT0（bit 30）必须被初始化为 0b1。

 - 如果内核在 EL1 进入且 EL2 存在：

    - SMCR_EL2.EZT0（bit 30）必须被初始化为 0b1。

  对于带有分支记录缓冲区扩展（FEAT_BRBE）的 CPU：

  - 如果 EL3 存在：

    - MDCR_EL3.SBRBE（bits 33:32）必须被初始化为 0b01 或 0b11。

  - 如果内核在 EL1 进入且 EL2 存在：

    - BRBCR_EL2.CC（bit 3）必须被初始化为 0b1。
    - BRBCR_EL2.MPRED（bit 4）必须被初始化为 0b1。

    - HDFGRTR_EL2.nBRBDATA（bit 61）必须被初始化为 0b1。
    - HDFGRTR_EL2.nBRBCTL  （bit 60）必须被初始化为 0b1。
    - HDFGRTR_EL2.nBRBIDR  （bit 59）必须被初始化为 0b1。

    - HDFGWTR_EL2.nBRBDATA（bit 61）必须被初始化为 0b1。
    - HDFGWTR_EL2.nBRBCTL  （bit 60）必须被初始化为 0b1。

    - HFGITR_EL2.nBRBIALL（bit 56）必须被初始化为 0b1。
    - HFGITR_EL2.nBRBINJ  （bit 55）必须被初始化为 0b1。

  对于带有性能监视器扩展（FEAT_PMUv3p9）的 CPU：

 - 如果 EL3 存在：

    - MDCR_EL3.EnPM2（bit 7）必须被初始化为 0b1。

 - 如果内核在 EL1 进入且 EL2 存在：

    - HDFGRTR2_EL2.nPMICNTR_EL0（bit 2）必须被初始化为 0b1。
    - HDFGRTR2_EL2.nPMICFILTR_EL0（bit 3）必须被初始化为 0b1。
    - HDFGRTR2_EL2.nPMUACR_EL1（bit 4）必须被初始化为 0b1。

    - HDFGWTR2_EL2.nPMICNTR_EL0（bit 2）必须被初始化为 0b1。
    - HDFGWTR2_EL2.nPMICFILTR_EL0（bit 3）必须被初始化为 0b1。
    - HDFGWTR2_EL2.nPMUACR_EL1（bit 4）必须被初始化为 0b1。

  对于带有 SPE 数据源过滤（FEAT_SPE_FDS）的 CPU：

  - 如果 EL3 存在：

    - MDCR_EL3.EnPMS3（bit 42）必须被初始化为 0b1。

  - 如果内核在 EL1 进入且 EL2 存在：

    - HDFGRTR2_EL2.nPMSDSFR_EL1（bit 19）必须被初始化为 0b1。
    - HDFGWTR2_EL2.nPMSDSFR_EL1（bit 19）必须被初始化为 0b1。

  对于带有内存复制与内存设置指令（FEAT_MOPS）的 CPU：

  - 如果内核在 EL1 进入且 EL2 存在：

    - HCRX_EL2.MSCEn（bit 11）必须被初始化为 0b1。

    - HCRX_EL2.MCE2（bit 10）必须被初始化为 0b1，且 hypervisor 必须按照 arm64_mops_hyp 中所述处理 MOPS 异常。

  对于带有扩展转换控制寄存器特性（FEAT_TCR2）的 CPU：

  - 如果 EL3 存在：

    - SCR_EL3.TCR2En（bit 43）必须被初始化为 0b1。

 - 如果内核在 EL1 进入且 EL2 存在：

    - HCRX_EL2.TCR2En（bit 14）必须被初始化为 0b1。

  对于带有第 1 阶段权限间接扩展特性（FEAT_S1PIE）的 CPU：

  - 如果 EL3 存在：

    - SCR_EL3.PIEn（bit 45）必须被初始化为 0b1。

  - 如果内核在 EL1 进入且 EL2 存在：

    - HFGRTR_EL2.nPIR_EL1（bit 58）必须被初始化为 0b1。

    - HFGWTR_EL2.nPIR_EL1（bit 58）必须被初始化为 0b1。

    - HFGRTR_EL2.nPIRE0_EL1（bit 57）必须被初始化为 0b1。

    - HFGRWR_EL2.nPIRE0_EL1（bit 57）必须被初始化为 0b1。

 - 对于带有受保护控制栈（Guarded Control Stacks，FEAT_GCS）的 CPU：

  - GCSCR_EL1 必须被初始化为 0。

  - GCSCRE0_EL1 必须被初始化为 0。

  - 如果 EL3 存在：

    - SCR_EL3.GCSEn（bit 39）必须被初始化为 0b1。

  - 如果 EL2 存在：

    - GCSCR_EL2 必须被初始化为 0。

 - 如果内核在 EL1 进入且 EL2 存在：

    - HCRX_EL2.GCSEn 必须被初始化为 0b1。

    - HFGITR_EL2.nGCSEPP（bit 59）必须被初始化为 0b1。

    - HFGITR_EL2.nGCSSTR_EL1（bit 58）必须被初始化为 0b1。

    - HFGITR_EL2.nGCSPUSHM_EL1（bit 57）必须被初始化为 0b1。

    - HFGRTR_EL2.nGCS_EL1（bit 53）必须被初始化为 0b1。

    - HFGRTR_EL2.nGCS_EL0（bit 52）必须被初始化为 0b1。

    - HFGWTR_EL2.nGCS_EL1（bit 53）必须被初始化为 0b1。

    - HFGWTR_EL2.nGCS_EL0（bit 52）必须被初始化为 0b1。

 - 对于带有调试架构（即 FEAT_Debugv8pN，所有版本）的 CPU：

 - 如果 EL3 存在：

   - MDCR_EL3.TDA（bit 9）必须被初始化为 0b0

 - 对于带有 FEAT_PMUv3 的 CPU：

 - 如果 EL3 存在：

   - MDCR_EL3.TPM（bit 6）必须被初始化为 0b0

  对于支持无状态 64 字节加载与存储（FEAT_LS64）的 CPU：

  - 如果内核在 EL1 进入且 EL2 存在：

    - HCRX_EL2.EnALS（bit 1）必须被初始化为 0b1。

  对于支持带状态 64 字节存储（FEAT_LS64_V）的 CPU：

  - 如果内核在 EL1 进入且 EL2 存在：

    - HCRX_EL2.EnASR（bit 2）必须被初始化为 0b1。

上述关于 CPU 模式、缓存、MMU、架构定时器、一致性和系统寄存器的要求适用于所有 CPU。所有 CPU 必须以相同的异常级别进入内核。在文档化的值禁用陷阱（traps）的情况下，只要这些陷阱由更高异常级别透明地处理，如同设置了文档化的值一样，允许启用这些陷阱。

引导加载程序应以如下方式在每个 CPU 上进入内核：

- 主 CPU 必须直接跳转到内核镜像的第一条指令。该 CPU 传递的设备树 blob 必须包含每个 cpu 节点的 'enable-method' 属性。支持的 enable-method 如下所述。

  预期引导加载程序将生成这些设备树属性，并在进入内核之前将其插入 blob 中。

- 带有 "spin-table" enable-method 的 CPU 必须在其 cpu 节点中具有 'cpu-release-addr' 属性。该属性标识一个自然对齐的、零初始化的 64 位内存位置。

  这些 CPU 应当在内核外部的一个保留内存区域（通过设备树中的 /memreserve/ 区域告知内核）中自旋（spin），轮询其 cpu-release-addr 位置，该位置必须包含在保留区域内。可以插入一条 wfe 指令以降低忙循环的额外开销，主 CPU 将发出一条 sev 指令。当 cpu-release-addr 所指向位置被读出非零值时，该 CPU 必须跳转到此值。该值将以单个 64 位小端值写入，因此 CPU 在跳转之前必须将读出的值转换为其本机字节序。

- 带有 "psci" enable method 的 CPU 应保留在内核之外（即，在设备树 memory 节点中描述给内核的内存区域之外，或在设备树中通过 /memreserve/ 区域描述给内核的保留内存区域内）。内核将发出 CPU_ON 调用，如 ARM 文档编号 ARM DEN 0022A（《ARM 处理器上的电源状态协调接口系统软件》）所述，以将 CPU 带入内核。

  设备树应包含 'psci' 节点，如 Documentation/devicetree/bindings/arm/psci.yaml 中所述。

- 次级 CPU 通用寄存器设置

  - x0 = 0（保留供将来使用）
  - x1 = 0（保留供将来使用）
  - x2 = 0（保留供将来使用）
  - x3 = 0（保留供将来使用）
