
## AMD 内存加密

安全内存加密（SME，Secure Memory Encryption）和安全加密虚拟化（SEV，Secure Encrypted Virtualization）是 AMD 处理器上的特性。

SME 提供了使用标准 x86 页表将内存中的单个页标记为已加密的能力。被标记为加密的页在从 DRAM 读取时会自动解密，在写入 DRAM 时会自动加密。因此，SME 可用于保护 DRAM 内容免受对系统的物理攻击。

SEV 支持运行加密的虚拟机（VM），其中客户机 VM 的代码和数据受到保护，使得解密版本仅在 VM 自身内部可用。SEV 客户机 VM 具有私有内存和共享内存的概念。私有内存使用客户机专属密钥加密，而共享内存可能使用 hypervisor 密钥加密。当 SME 启用时，hypervisor 密钥与 SME 中使用的密钥相同。

当一个页表项设置了加密位（其位置如何确定见下文）时，该页即被加密。加密位也可以在 cr3 寄存器中指定，从而允许对 PGD 表进行加密。页表的每一级也可以通过在下一级页表对应的页表项中设置加密位来进行加密。这样便可以对整个页表层级进行加密。注意，这意味着仅仅因为在 cr3 中设置了加密位，并不意味着整个层级都被加密。层级中的每个页表项都需要设置加密位才能实现这一点。因此，理论上你可以只在 cr3 中设置加密位以便 PGD 被加密，但不在指向某个 PUD 的 PGD 项中设置加密位，从而导致该 PUD 指向的 PUD 不被加密。

当 SEV 启用时，指令页和客户机页表始终被视为私有的。客户机内部的所有 DMA 操作都必须在共享内存上执行。由于内存加密位在 64 位或 32 位 PAE 模式下由客户机 OS 控制，在所有其他模式下，SEV 硬件会强制将内存加密位设为 1。

对 SME 和 SEV 的支持可以通过 CPUID 指令确定。

```
	0x8000001f[eax]:
		Bit[0] 表示支持 SME
		Bit[1] 表示支持 SEV
	0x8000001f[ebx]:
		Bits[5:0]  用于激活内存加密的页表位编号
		Bits[11:6] 启用内存加密时物理地址空间的缩减位数
			   （这仅影响系统物理地址，不影响客户机物理地址）

```
如果支持 SME，则可以使用 MSR 0xc00100010（MSR_AMD64_SYSCFG）来

```
	0xc0010010:
		Bit[23]   0 = 内存加密特性被禁用
			  1 = 内存加密特性被启用

```
如果支持 SEV，则可以使用 MSR 0xc0010131（MSR_AMD64_SEV）来确定

```
	0xc0010131:
		Bit[0]	  0 = 内存加密未激活
			  1 = 内存加密已激活

```
如果 BIOS 判定启用内存加密带来的物理地址空间缩减（见上文 CPUID 信息）不会与系统的地址空间资源需求冲突，Linux 依赖 BIOS 来设置该位。如果 Linux 启动时该位未被设置，那么 Linux 自身也不会设置它，内存加密将无法使用。

Linux 内核中 SME 的状态可以说明如下：

 - 受支持（Supported）：
	  通过 CPUID 指令确定 CPU 支持 SME。

 - 已启用（Enabled）：
	  受支持，并且 MSR_AMD64_SYSCFG 的 bit 23 已设置。

 - 已激活（Active）：
	  受支持、已启用，并且 Linux 内核正主动将加密位应用到页表项（内核中的 SME 掩码非零）。

SME 也可以在 BIOS 中启用并激活。如果 SME 在 BIOS 中启用并激活，那么所有内存访问都将被加密，也就没有必要再激活 Linux 的内存加密支持。

如果 BIOS 仅仅是启用了 SME（设置了 MSR_AMD64_SYSCFG 的 bit 23），那么可以通过在内核命令行提供 `mem_encrypt=on` 来启用内存加密。但是，如果 BIOS 没有启用 SME，那么即使默认配置为启用，或者指定了 `mem_encrypt=on` 命令行参数，Linux 也将无法激活内存加密。

## 安全嵌套分页（SNP，Secure Nested Paging）

SEV-SNP 引入了新特性（SEV_FEATURES[1:63]），可由 hypervisor 启用以增强安全性。其中部分特性需要客户机侧的实现才能正确工作。下表列出了在各种客户机/hypervisor SNP 特性支持场景下预期的客户机行为。

+-----------------+---------------+---------------+------------------+
| Feature Enabled | Guest needs   | Guest has     | Guest boot       |
| by the HV       | implementation| implementation| behaviour        |
+=================+===============+===============+==================+
|      No         |      No       |      No       |     Boot         |
|                 |               |               |                  |
+-----------------+---------------+---------------+------------------+
|      No         |      Yes      |      No       |     Boot         |
|                 |               |               |                  |
+-----------------+---------------+---------------+------------------+
|      No         |      Yes      |      Yes      |     Boot         |
|                 |               |               |                  |
+-----------------+---------------+---------------+------------------+
|      Yes        |      No       |      No       | Boot with        |
|                 |               |               | feature enabled  |
+-----------------+---------------+---------------+------------------+
|      Yes        |      Yes      |      No       | Graceful boot    |
|                 |               |               | failure          |
+-----------------+---------------+---------------+------------------+
|      Yes        |      Yes      |      Yes      | Boot with        |
|                 |               |               | feature enabled  |
+-----------------+---------------+---------------+------------------+

更多细节见 AMD64 APM[^1^] Vol 2: 15.34.10 SEV_STATUS MSR

## 反向映射表（RMP，Reverse Map Table）

RMP 是系统内存中的一个结构，用于确保系统物理地址与客户机物理地址之间的一一映射。每个可能被分配给客户机的内存页在 RMP 中都有一个条目。

RMP 表可以是内存中连续的一段，也可以是内存中的若干段集合。

### 连续 RMP（Contiguous RMP）

当支持 SEV-SNP 时，存在对这种形式 RMP 的支持：

```
	0x8000001f[eax]:
		Bit[4] 表示支持 SEV-SNP

```
```
        0xc0010132 (RMP_BASE):
                 RMP 第一个字节的系统物理地址

        0xc0010133 (RMP_END):
                 RMP 最后一个字节的系统物理地址

```
硬件要求 RMP_BASE 和（RMP_END + 1）按 8KB 对齐，但 SEV 固件将对齐要求提高到需要 1MB 对齐。

RMP 由一段用于处理器记账的 16KB 区域以及随后的 RMP 条目组成，每个条目大小为 16 字节。RMP 的大小决定了 hypervisor 可以分配给客户机的物理内存范围：

```
        0 到 ((RMP_END + 1 - RMP_BASE - 16KB) / 16B) x 4KB.

```
当前 Linux 的支持依赖 BIOS 为 RMP 分配/预留内存，并正确设置 RMP_BASE 和 RMP_END。Linux 使用 MSR 值来定位 RMP 并确定 RMP 的大小。RMP 必须覆盖全部系统内存，Linux 才能启用 SEV-SNP。

### 分段 RMP（Segmented RMP）

分段 RMP 支持是一种表示 RMP 布局的新方式。最初的 RMP 支持要求 RMP 表在内存中连续。对 RMP 不所在的 NUMA 节点上的 RMP 访问，可能比对 RMP 所在的 NUMA 节点上的访问花费更长时间。分段 RMP 支持允许将 RMP 条目放置在与其所覆盖的内存相同的节点上，从而可能降低访问与该内存相关联的 RMP 条目的延迟。每个 RMP 段覆盖特定范围的系统物理地址。

可以使用 CPUID 确定对这种形式 RMP 的支持：

```
        0x8000001f[eax]:
                Bit[23] 表示支持分段 RMP

```
如果受支持，可以使用 CPUID 找到分段 RMP 属性：

```
        0x80000025[eax]:
                Bits[5:0]  最小支持的 RMP 段大小
                Bits[11:6] 最大支持的 RMP 段大小

        0x80000025[ebx]:
                Bits[9:0]  可缓存的 RMP 段定义数量
                Bit[10]    指示可缓存 RMP 段的数量是否为硬限制

```
```
        0xc0010136 (RMP_CFG):
                Bit[0]     指示是否启用了分段 RMP
                Bits[13:8] 包含一个 RMP 段所覆盖的内存大小
                           （以 2 的幂表示）

```
RMP_CFG MSR 中定义的 RMP 段大小适用于 RMP 的所有段。因此每个 RMP 段覆盖特定范围的系统物理地址。例如，如果 RMP_CFG MSR 值为 0x2401，则 RMP 段覆盖值为 0x24 => 36，意味着一个 RMP 段所覆盖的内存大小为 64GB（1 << 36）。因此第一个 RMP 段覆盖从 0 到 0xF_FFFF_FFFF 的物理地址，第二个 RMP 段覆盖从 0x10_0000_0000 到 0x1F_FFFF_FFFF 的物理地址，依此类推。

当启用分段 RMP 时，RMP_BASE 像现在一样指向 RMP 记账区域（大小为 16K）。但是，在记账区域之后不再紧跟着 RMP 条目，而是有一个 4K 的 RMP 段表（RST，RMP Segment Table）。RST 中的每个条目大小为 8 字节，表示：

```
        Bits[19:0]  映射大小（以 GB 为单位）
                    映射大小可以小于定义的段大小。
                    值为零表示与此段相关联的系统物理地址范围不存在 RMP。
        Bits[51:20] 段物理地址
                    该地址左移 20 位（或在读取时仅被掩码）以形成段的物理地址（1MB 对齐）。

```
RST 可以容纳 512 个段条目，但如果可缓存 RMP 段的数量是一个硬限制（CPUID 0x80000025_EBX[10]），则可以将大小限制为可缓存 RMP 段的数量（CPUID 0x80000025_EBX[9:0]）。

当前 Linux 的支持依赖 BIOS 为分段 RMP（记账区域、RST 以及所有段）分配/预留内存，构建 RST，并正确设置 RMP_BASE、RMP_END 和 RMP_CFG。Linux 使用 MSR 值来定位 RMP 并确定 RMP 段的大小和位置。RMP 必须覆盖全部系统内存，Linux 才能启用 SEV-SNP。

更多细节见 AMD64 APM Vol 2 中的 “15.36.3 Reverse Map Table” 一节，docID: 24593。

## 安全 VM 服务模块（SVSM，Secure VM Service Module）

SNP 提供了一个称为虚拟机特权级别（VMPL，Virtual Machine Privilege Levels）的特性，它定义了客户机软件可以运行的四个特权级别。最特权级别为 0，数值越大的级别特权越低。更多细节见 AMD64 APM Vol 2 中的 “15.35.7 Virtual Machine Privilege Levels” 一节，docID: 24593。

使用该特性时，不同的服务可以运行在不同的保护级别上，独立于客户机 OS 但仍处于安全的 SNP 环境内。它们可以向客户机提供服务，例如 vTPM。

当客户机没有运行在 VMPL0 时，它需要与运行在 VMPL0 的软件通信，以执行特权操作或与安全服务交互。此类特权操作的一个例子是 PVALIDATE，它**必须**在 VMPL0 执行。

在这种场景下，运行在 VMPL0 的软件通常被称为安全 VM 服务模块（SVSM）。对 SVSM 的探测以及与它通信所使用的 API 记录于 “Secure VM Service Module for SEV-SNP Guests”，docID: 58019。

（上述文档的最新版本可以通过使用如 duckduckgo.com 之类的搜索引擎并输入以下内容找到：

  site:amd.com "Secure VM Service Module for SEV-SNP Guests", docID: 58019

作为例子。）
