
## ARM64 ELF hwcaps

本文档描述了 arm64 ELF hwcaps 的用法与语义。

### 1. Introduction（简介）

某些硬件或软件特性仅在部分 CPU 实现上、和/或配合特定的内核配置才可用，但在 EL0 处没有可供用户空间代码使用的架构化发现机制。内核通过一组称为 hwcaps 的标志（暴露在辅助向量中）将这些特性的存在暴露给用户空间。

用户空间软件可以通过获取辅助向量的 AT_HWCAP、AT_HWCAP2 或 AT_HWCAP3 条目，并测试
```

	bool floating_point_is_present(void)
	{
		unsigned long hwcaps = getauxval(AT_HWCAP);
		if (hwcaps & HWCAP_FP)
			return true;

		return false;
	}

```
在软件依赖于某个由 hwcap 描述的特征时，它应当在尝试使用该特征之前，检查相关的 hwcap 标志以确认该特征确实存在。

无法可靠地通过其他手段探测这些特征。当某个特征不可用时，尝试使用它可能导致不可预测的行为，并且不保证会产生任何可靠的指示（例如 SIGILL）表明该特征不可用。

### 2. Interpretation of hwcaps（hwcaps 的解释）

大多数 hwcaps 旨在指示那些由架构化 ID 寄存器（在 EL0 处用户空间代码无法访问）所描述的特征的存在。这些 hwcaps 是根据 ID 寄存器字段来定义的，并且应当参考 ARM 架构参考手册（ARM ARM）中这些字段的定义来解释。

```

    Functionality implied by idreg.field == val.

```
这类 hwcaps 指示了 ARM ARM 定义为在 idreg.field 取值为 val 时存在的功能，但并不意味着 idreg.field 精确等于 val，也不意味着指示了由 idreg.field 其他取值所隐含的功能的缺失。

其他 hwcaps 可能指示无法仅由 ID 寄存器描述的特征的存在。这些可以在不参考 ID 寄存器的情况下描述，并且可能引用其他文档。

### 3. The hwcaps exposed in AT_HWCAP（暴露在 AT_HWCAP 中的 hwcaps）

HWCAP_FP
    由 ID_AA64PFR0_EL1.FP == 0b0000 所隐含的功能。

HWCAP_ASIMD
    由 ID_AA64PFR0_EL1.AdvSIMD == 0b0000 所隐含的功能。

HWCAP_EVTSTRM
    通用定时器被配置为以大约 10KHz 的频率生成事件。

HWCAP_AES
    由 ID_AA64ISAR0_EL1.AES == 0b0001 所隐含的功能。

HWCAP_PMULL
    由 ID_AA64ISAR0_EL1.AES == 0b0010 所隐含的功能。

HWCAP_SHA1
    由 ID_AA64ISAR0_EL1.SHA1 == 0b0001 所隐含的功能。

HWCAP_SHA2
    由 ID_AA64ISAR0_EL1.SHA2 == 0b0001 所隐含的功能。

HWCAP_CRC32
    由 ID_AA64ISAR0_EL1.CRC32 == 0b0001 所隐含的功能。

HWCAP_ATOMICS
    由 ID_AA64ISAR0_EL1.Atomic == 0b0010 所隐含的功能。

HWCAP_FPHP
    由 ID_AA64PFR0_EL1.FP == 0b0001 所隐含的功能。

HWCAP_ASIMDHP
    由 ID_AA64PFR0_EL1.AdvSIMD == 0b0001 所隐含的功能。

HWCAP_CPUID
    EL0 对某些 ID 寄存器的访问是可用的，范围如 Documentation/arch/arm64/cpu-feature-registers.rst 所述。

    这些 ID 寄存器可能暗示了某些特征的可用性。

HWCAP_ASIMDRDM
    由 ID_AA64ISAR0_EL1.RDM == 0b0001 所隐含的功能。

HWCAP_JSCVT
    由 ID_AA64ISAR1_EL1.JSCVT == 0b0001 所隐含的功能。

HWCAP_FCMA
    由 ID_AA64ISAR1_EL1.FCMA == 0b0001 所隐含的功能。

HWCAP_LRCPC
    由 ID_AA64ISAR1_EL1.LRCPC == 0b0001 所隐含的功能。

HWCAP_DCPOP
    由 ID_AA64ISAR1_EL1.DPB == 0b0001 所隐含的功能。

HWCAP_SHA3
    由 ID_AA64ISAR0_EL1.SHA3 == 0b0001 所隐含的功能。

HWCAP_SM3
    由 ID_AA64ISAR0_EL1.SM3 == 0b0001 所隐含的功能。

HWCAP_SM4
    由 ID_AA64ISAR0_EL1.SM4 == 0b0001 所隐含的功能。

HWCAP_ASIMDDP
    由 ID_AA64ISAR0_EL1.DP == 0b0001 所隐含的功能。

HWCAP_SHA512
    由 ID_AA64ISAR0_EL1.SHA2 == 0b0010 所隐含的功能。

HWCAP_SVE
    由 ID_AA64PFR0_EL1.SVE == 0b0001 所隐含的功能。

HWCAP_ASIMDFHM
   由 ID_AA64ISAR0_EL1.FHM == 0b0001 所隐含的功能。

HWCAP_DIT
    由 ID_AA64PFR0_EL1.DIT == 0b0001 所隐含的功能。

HWCAP_USCAT
    由 ID_AA64MMFR2_EL1.AT == 0b0001 所隐含的功能。

HWCAP_ILRCPC
    由 ID_AA64ISAR1_EL1.LRCPC == 0b0010 所隐含的功能。

HWCAP_FLAGM
    由 ID_AA64ISAR0_EL1.TS == 0b0001 所隐含的功能。

HWCAP_SSBS
    由 ID_AA64PFR1_EL1.SSBS == 0b0010 所隐含的功能。

HWCAP_SB
    由 ID_AA64ISAR1_EL1.SB == 0b0001 所隐含的功能。

HWCAP_PACA
    由 ID_AA64ISAR1_EL1.APA == 0b0001 或 ID_AA64ISAR1_EL1.API == 0b0001 所隐含的功能，如 Documentation/arch/arm64/pointer-authentication.rst 所述。

HWCAP_PACG
    由 ID_AA64ISAR1_EL1.GPA == 0b0001 或 ID_AA64ISAR1_EL1.GPI == 0b0001 所隐含的功能，如 Documentation/arch/arm64/pointer-authentication.rst 所述。

HWCAP_GCS
    由 ID_AA64PFR1_EL1.GCS == 0b1 所隐含的功能，如 Documentation/arch/arm64/gcs.rst 所述。

HWCAP_CMPBR
    由 ID_AA64ISAR2_EL1.CSSC == 0b0010 所隐含的功能。

HWCAP_FPRCVT
    由 ID_AA64ISAR3_EL1.FPRCVT == 0b0001 所隐含的功能。

HWCAP_F8MM8
    由 ID_AA64FPFR0_EL1.F8MM8 == 0b0001 所隐含的功能。

HWCAP_F8MM4
    由 ID_AA64FPFR0_EL1.F8MM4 == 0b0001 所隐含的功能。

HWCAP_SVE_F16MM
    由 ID_AA64PFR0_EL1.SVE == 0b0001 以及 ID_AA64ZFR0_EL1.F16MM == 0b0001 所隐含的功能。

HWCAP_SVE_ELTPERM
    由 ID_AA64PFR0_EL1.SVE == 0b0001 以及 ID_AA64ZFR0_EL1.ELTPERM == 0b0001 所隐含的功能。

HWCAP_SVE_AES2
    由 ID_AA64PFR0_EL1.SVE == 0b0001 以及 ID_AA64ZFR0_EL1.AES == 0b0011 所隐含的功能。

HWCAP_SVE_BFSCALE
    由 ID_AA64PFR0_EL1.SVE == 0b0001 以及 ID_AA64ZFR0_EL1.B16B16 == 0b0010 所隐含的功能。

HWCAP_SVE2P2
    由 ID_AA64PFR0_EL1.SVE == 0b0001 以及 ID_AA64ZFR0_EL1.SVEver == 0b0011 所隐含的功能。

HWCAP_SME2P2
    由 ID_AA64SMFR0_EL1.SMEver == 0b0011 所隐含的功能。

HWCAP_SME_SBITPERM
    由 ID_AA64SMFR0_EL1.SBitPerm == 0b1 所隐含的功能。

HWCAP_SME_AES
    由 ID_AA64SMFR0_EL1.AES == 0b1 所隐含的功能。

HWCAP_SME_SFEXPA
    由 ID_AA64SMFR0_EL1.SFEXPA == 0b1 所隐含的功能。

HWCAP_SME_STMOP
    由 ID_AA64SMFR0_EL1.STMOP == 0b1 所隐含的功能。

HWCAP_SME_SMOP4
    由 ID_AA64SMFR0_EL1.SMOP4 == 0b1 所隐含的功能。

HWCAP2_DCPODP
    由 ID_AA64ISAR1_EL1.DPB == 0b0010 所隐含的功能。

HWCAP2_SVE2
    由 ID_AA64PFR0_EL1.SVE == 0b0001 以及 ID_AA64ZFR0_EL1.SVEver == 0b0001 所隐含的功能。

HWCAP2_SVEAES
    由 ID_AA64PFR0_EL1.SVE == 0b0001 以及 ID_AA64ZFR0_EL1.AES == 0b0001 所隐含的功能。

HWCAP2_SVEPMULL
    由 ID_AA64PFR0_EL1.SVE == 0b0001 以及 ID_AA64ZFR0_EL1.AES == 0b0010 所隐含的功能。

HWCAP2_SVEBITPERM
    由 ID_AA64PFR0_EL1.SVE == 0b0001 以及 ID_AA64ZFR0_EL1.BitPerm == 0b0001 所隐含的功能。

HWCAP2_SVESHA3
    由 ID_AA64PFR0_EL1.SVE == 0b0001 以及 ID_AA64ZFR0_EL1.SHA3 == 0b0001 所隐含的功能。

HWCAP2_SVESM4
    由 ID_AA64PFR0_EL1.SVE == 0b0001 以及 ID_AA64ZFR0_EL1.SM4 == 0b0001 所隐含的功能。

HWCAP2_FLAGM2
    由 ID_AA64ISAR0_EL1.TS == 0b0010 所隐含的功能。

HWCAP2_FRINT
    由 ID_AA64ISAR1_EL1.FRINTTS == 0b0001 所隐含的功能。

HWCAP2_SVEI8MM
    由 ID_AA64PFR0_EL1.SVE == 0b0001 以及 ID_AA64ZFR0_EL1.I8MM == 0b0001 所隐含的功能。

HWCAP2_SVEF32MM
    由 ID_AA64PFR0_EL1.SVE == 0b0001 以及 ID_AA64ZFR0_EL1.F32MM == 0b0001 所隐含的功能。

HWCAP2_SVEF64MM
    由 ID_AA64PFR0_EL1.SVE == 0b0001 以及 ID_AA64ZFR0_EL1.F64MM == 0b0001 所隐含的功能。

HWCAP2_SVEBF16
    由 ID_AA64PFR0_EL1.SVE == 0b0001 以及 ID_AA64ZFR0_EL1.BF16 == 0b0001 所隐含的功能。

HWCAP2_I8MM
    由 ID_AA64ISAR1_EL1.I8MM == 0b0001 所隐含的功能。

HWCAP2_BF16
    由 ID_AA64ISAR1_EL1.BF16 == 0b0001 所隐含的功能。

HWCAP2_DGH
    由 ID_AA64ISAR1_EL1.DGH == 0b0001 所隐含的功能。

HWCAP2_RNG
    由 ID_AA64ISAR0_EL1.RNDR == 0b0001 所隐含的功能。

HWCAP2_BTI
    由 ID_AA64PFR1_EL1.BT == 0b0001 所隐含的功能。

HWCAP2_MTE
    由 ID_AA64PFR1_EL1.MTE == 0b0010 所隐含的功能，如 Documentation/arch/arm64/memory-tagging-extension.rst 所述。

HWCAP2_ECV
    由 ID_AA64MMFR0_EL1.ECV == 0b0001 所隐含的功能。

HWCAP2_AFP
    由 ID_AA64MMFR1_EL1.AFP == 0b0001 所隐含的功能。

HWCAP2_RPRES
    由 ID_AA64ISAR2_EL1.RPRES == 0b0001 所隐含的功能。

HWCAP2_MTE3
    由 ID_AA64PFR1_EL1.MTE == 0b0011 所隐含的功能，如 Documentation/arch/arm64/memory-tagging-extension.rst 所述。

HWCAP2_SME
    由 ID_AA64PFR1_EL1.SME == 0b0001 所隐含的功能，如 Documentation/arch/arm64/sme.rst 所述。

HWCAP2_SME_I16I64
    由 ID_AA64SMFR0_EL1.I16I64 == 0b1111 所隐含的功能。

HWCAP2_SME_F64F64
    由 ID_AA64SMFR0_EL1.F64F64 == 0b1 所隐含的功能。

HWCAP2_SME_I8I32
    由 ID_AA64SMFR0_EL1.I8I32 == 0b1111 所隐含的功能。

HWCAP2_SME_F16F32
    由 ID_AA64SMFR0_EL1.F16F32 == 0b1 所隐含的功能。

HWCAP2_SME_B16F32
    由 ID_AA64SMFR0_EL1.B16F32 == 0b1 所隐含的功能。

HWCAP2_SME_F32F32
    由 ID_AA64SMFR0_EL1.F32F32 == 0b1 所隐含的功能。

HWCAP2_SME_FA64
    由 ID_AA64SMFR0_EL1.FA64 == 0b1 所隐含的功能。

HWCAP2_WFXT
    由 ID_AA64ISAR2_EL1.WFXT == 0b0010 所隐含的功能。

HWCAP2_EBF16
    由 ID_AA64ISAR1_EL1.BF16 == 0b0010 所隐含的功能。

HWCAP2_SVE_EBF16
    由 ID_AA64PFR0_EL1.SVE == 0b0001 以及 ID_AA64ZFR0_EL1.BF16 == 0b0010 所隐含的功能。

HWCAP2_CSSC
    由 ID_AA64ISAR2_EL1.CSSC == 0b0001 所隐含的功能。

HWCAP2_RPRFM
    由 ID_AA64ISAR2_EL1.RPRFM == 0b0001 所隐含的功能。

HWCAP2_SVE2P1
    由 ID_AA64PFR0_EL1.SVE == 0b0001 以及 ID_AA64ZFR0_EL1.SVEver == 0b0010 所隐含的功能。

HWCAP2_SME2
    由 ID_AA64SMFR0_EL1.SMEver == 0b0001 所隐含的功能。

HWCAP2_SME2P1
    由 ID_AA64SMFR0_EL1.SMEver == 0b0010 所隐含的功能。

HWCAP2_SMEI16I32
    由 ID_AA64SMFR0_EL1.I16I32 == 0b0101 所隐含的功能。

HWCAP2_SMEBI32I32
    由 ID_AA64SMFR0_EL1.BI32I32 == 0b1 所隐含的功能。

HWCAP2_SMEB16B16
    由 ID_AA64SMFR0_EL1.B16B16 == 0b1 所隐含的功能。

HWCAP2_SMEF16F16
    由 ID_AA64SMFR0_EL1.F16F16 == 0b1 所隐含的功能。

HWCAP2_MOPS
    由 ID_AA64ISAR2_EL1.MOPS == 0b0001 所隐含的功能。

HWCAP2_HBC
    由 ID_AA64ISAR2_EL1.BC == 0b0001 所隐含的功能。

HWCAP2_SVE_B16B16
    由 ID_AA64PFR0_EL1.SVE == 0b0001 以及 ID_AA64ZFR0_EL1.B16B16 == 0b0001 所隐含的功能。

HWCAP2_LRCPC3
    由 ID_AA64ISAR1_EL1.LRCPC == 0b0011 所隐含的功能。

HWCAP2_LSE128
    由 ID_AA64ISAR0_EL1.Atomic == 0b0011 所隐含的功能。

HWCAP2_FPMR
    由 ID_AA64PFR2_EL1.FMR == 0b0001 所隐含的功能。

HWCAP2_LUT
    由 ID_AA64ISAR2_EL1.LUT == 0b0001 所隐含的功能。

HWCAP2_FAMINMAX
    由 ID_AA64ISAR3_EL1.FAMINMAX == 0b0001 所隐含的功能。

HWCAP2_F8CVT
    由 ID_AA64FPFR0_EL1.F8CVT == 0b1 所隐含的功能。

HWCAP2_F8FMA
    由 ID_AA64FPFR0_EL1.F8FMA == 0b1 所隐含的功能。

HWCAP2_F8DP4
    由 ID_AA64FPFR0_EL1.F8DP4 == 0b1 所隐含的功能。

HWCAP2_F8DP2
    由 ID_AA64FPFR0_EL1.F8DP2 == 0b1 所隐含的功能。

HWCAP2_F8E4M3
    由 ID_AA64FPFR0_EL1.F8E4M3 == 0b1 所隐含的功能。

HWCAP2_F8E5M2
    由 ID_AA64FPFR0_EL1.F8E5M2 == 0b1 所隐含的功能。

HWCAP2_SME_LUTV2
    由 ID_AA64SMFR0_EL1.LUTv2 == 0b1 所隐含的功能。

HWCAP2_SME_F8F16
    由 ID_AA64SMFR0_EL1.F8F16 == 0b1 所隐含的功能。

HWCAP2_SME_F8F32
    由 ID_AA64SMFR0_EL1.F8F32 == 0b1 所隐含的功能。

HWCAP2_SME_SF8FMA
    由 ID_AA64SMFR0_EL1.SF8FMA == 0b1 所隐含的功能。

HWCAP2_SME_SF8DP4
    由 ID_AA64SMFR0_EL1.SF8DP4 == 0b1 所隐含的功能。

HWCAP2_SME_SF8DP2
    由 ID_AA64SMFR0_EL1.SF8DP2 == 0b1 所隐含的功能。

HWCAP2_SME_SF8DP4
    由 ID_AA64SMFR0_EL1.SF8DP4 == 0b1 所隐含的功能。

HWCAP2_POE
    由 ID_AA64MMFR3_EL1.S1POE == 0b0001 所隐含的功能。

HWCAP3_MTE_FAR
    由 ID_AA64PFR2_EL1.MTEFAR == 0b0001 所隐含的功能。

HWCAP3_MTE_STORE_ONLY
    由 ID_AA64PFR2_EL1.MTESTOREONLY == 0b0001 所隐含的功能。

HWCAP3_LSFE
    由 ID_AA64ISAR3_EL1.LSFE == 0b0001 所隐含的功能。

HWCAP3_LS64
    由 ID_AA64ISAR1_EL1.LS64 == 0b0001 所隐含的功能。注意，指令 ld64b/st64b 的功能需要 CPU、系统以及目标（设备）内存位置的配合支持，而 HWCAP3_LS64 仅暗示了对 CPU 的支持。用户应当只在受支持的目标（设备）内存位置上使用 ld64b/st64b，否则应回退到非原子替代方案。

### 4. Unused AT_HWCAP bits（未使用的 AT_HWCAP 位）

为了与用户空间互操作，内核保证 AT_HWCAP 的第 62 和 63 位将始终被返回为 0。
