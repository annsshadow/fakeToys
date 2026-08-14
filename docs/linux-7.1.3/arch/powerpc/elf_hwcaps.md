
## POWERPC ELF HWCAPs（硬件能力标志）

本文档描述 POWERPC ELF HWCAPs 的使用语义。


### 1. 简介

某些硬件与软件特性仅在特定的 CPU 上可用，或仅在特定的内核配置下可用。用户空间代码可用的发现机制是 HWCAPs——这是一组由内核在辅助向量（auxiliary vector）中向用户空间暴露的标志位。

用户空间软件可以通过获取辅助向量中的 `AT_HWCAP` 或 `AT_HWCAP2` 项，并测试相应的标志位，来判断某项特性是否可用。

```
	bool floating_point_is_present(void)
	{
		unsigned long HWCAPs = getauxval(AT_HWCAP);
		if (HWCAPs & PPC_FEATURE_HAS_FPU)
			return true;

		return false;
	}
```

依赖某项 HWCAP 所描述特性的软件，应当检查相应的 HWCAP 标志位以确认该特性确实存在，然后再去使用它。

相比主动探测（probing）等手段，HWCAP 是测试特性是否存在的首选方式，因为探测手段可能导致不可预期的行为。

面向特定平台的软件不一定需要测试那些其所依赖的、隐含必备的特性。例如，一个需要 FPU、VMX、VSX 的程序，必须测试相应的 HWCAPs，否则编译器生成的、要求这些特性的代码将无法运行。

### 2. Facilities（设施）

Power ISA 使用术语 "facility"（设施）来描述一类指令、寄存器、中断等。某个 facility 的存在与否，表示该类中相关功能是否可用，具体细节则取决于 ISA 版本。例如，若 VSX facility 可用，则 VSX 指令的使用方式在 v3.0B 与 v3.1B 等 ISA 版本之间会有所不同。

### 3. Categories（类别）

Power ISA v3.0 使用术语 "category"（类别）来描述某些指令类或操作模式，它们可能是可选的、也可能互斥。其确切含义取决于具体的 HWCAP 标志位与上下文。例如，存在 BOOKE 特性意味着实现了 server category。

### 4. HWCAP 分配

HWCAPs 的分配方式在 Power 架构 64 位 ELF V2 ABI 规范中描述（并反映在内核的 uapi 头文件中）。

### 5. HWCAPs exposed AT_HWCAP

PPC_FEATURE_32
32 位 CPU。

PPC_FEATURE_64
64 位 CPU（用户空间运行于 32 位模式）。

PPC_FEATURE_601_INSTR
PowerPC 601 处理器。自提交 f0ed73f3fa2c（"powerpc: 移除 PowerPC 601"）起内核不再使用。

PPC_FEATURE_HAS_ALTIVEC
向量（又称 Altivec、VMX）facility 可用。

PPC_FEATURE_HAS_FPU
浮点 facility 可用。

PPC_FEATURE_HAS_MMU
存在并已启用内存管理单元（MMU）。

PPC_FEATURE_HAS_4xxMAC
40x 或 44x 系列处理器。自提交 732b32daef80（"powerpc: 移除核心支持 40x"）起内核不再使用。

PPC_FEATURE_UNIFIED_CACHE
处理器采用统一的 L1 缓存（指令与数据共享），见于 NXP e200。自提交 39c8bf2b3cc1（"powerpc: Retire e200 核心 (mpc555x processor)"）起内核不再使用。

PPC_FEATURE_HAS_SPE
信号处理引擎（Signal Processing Engine）facility 可用。

PPC_FEATURE_HAS_EFP_SINGLE
嵌入式浮点单精度操作可用。

PPC_FEATURE_HAS_EFP_DOUBLE
嵌入式浮点双精度操作可用。

PPC_FEATURE_NO_TB
timebase facility（mftb 指令）可用。这是 601 特有的 HWCAP；一旦确定处理器为 601（由 HWCAPs 指示），就必须测试该位以使用 timebase。自提交 f0ed73f3fa2c（"powerpc: 移除 PowerPC 601"）起内核不再使用。

PPC_FEATURE_POWER4
POWER4 或 PPC970/FX/MP 处理器。对 POWER4 的支持自提交 471d7ff8b51b（"powerpc/64s: 移除 POWER4 支持"）起已从内核中移除。

PPC_FEATURE_POWER5
POWER5 处理器。

PPC_FEATURE_POWER5_PLUS
POWER5+ 处理器。

PPC_FEATURE_CELL
Cell 处理器。

PPC_FEATURE_BOOKE
处理器实现了嵌入式类别（"BookE"）架构。

PPC_FEATURE_SMT
处理器实现了同步多线程（SMT）。

PPC_FEATURE_ICACHE_SNOOP
处理器的指令缓存与数据缓存一致；为使指令存储与数据存储保持一致以便执行指令序列（如 POWER9 处理器中所述），需要：

```
        sync
        icbi (to any address)
        isync
```

PPC_FEATURE_ARCH_2_05
处理器支持 v2.05 用户态架构。支持更高架构版本的处理器不会设置该特性。

PPC_FEATURE_PA6T
PA6T 处理器。

PPC_FEATURE_HAS_DFP
DFP（十进制浮点）facility 可用。

PPC_FEATURE_POWER6_EXT
POWER6 处理器。

PPC_FEATURE_ARCH_2_06
处理器支持 v2.06 用户态架构。支持更高架构版本的处理器会设置该特性。

PPC_FEATURE_HAS_VSX
VSX facility 可用。

PPC_FEATURE_PSERIES_PERFMON_COMPAT
处理器支持架构定义的 PMU 事件范围 0xE0-0xFF。

PPC_FEATURE_TRUE_LE
处理器支持真正的 little-endian 模式。

PPC_FEATURE_PPC_LE
处理器支持 "PowerPC Little-Endian"，通过地址变换使存储访问表现为 little-endian，但数据以不同格式存储，不适合以该模式运行的其它访问者使用。

### 6. HWCAPs exposed AT_HWCAP2

PPC_FEATURE2_ARCH_2_07
处理器支持 v2.07 用户态架构。支持更高架构版本的处理器会设置该特性。

PPC_FEATURE2_HTM
事务性内存（Transactional Memory）特性可用。

PPC_FEATURE2_DSCR
DSCR facility 可用。

PPC_FEATURE2_EBB
EBB（Event Based Branch）facility 可用。

PPC_FEATURE2_ISEL
isel 指令可用。在 ARCH_2_07 及之后被取代。

PPC_FEATURE2_TAR
TAR facility 可用。

PPC_FEATURE2_VEC_CRYPTO
v2.07 加密指令可用。

PPC_FEATURE2_HTM_NOSC
在事务性状态下发起系统调用将失败，参见 文档/arch/powerpc/syscall64-abi.rst。

PPC_FEATURE2_ARCH_3_00
处理器支持 v3.0B / v3.0C 用户态架构。支持更高架构版本的处理器会设置该特性。

PPC_FEATURE2_HAS_IEEE128
IEEE 128 位二进制浮点，支持 VSX 四精度指令与数据类型。

PPC_FEATURE2_DARN
darn 指令可用。

PPC_FEATURE2_SCV
使用 scv 0 指令进行系统调用，参见 文档/arch/powerpc/syscall64-abi.rst。

PPC_FEATURE2_HTM_NO_SUSPEND
有限的事务性内存 facility 支持（不支持挂起）可用，参见 文档/arch/powerpc/transactional_memory.rst。

PPC_FEATURE2_ARCH_3_1
处理器支持 v3.1 用户态架构。支持更高架构版本的处理器会设置该特性。

PPC_FEATURE2_MMA
MMA facility 可用。
