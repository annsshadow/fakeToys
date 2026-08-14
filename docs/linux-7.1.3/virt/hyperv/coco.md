
## 机密计算（Confidential Computing）VM


Hyper-V 可以创建并运行作为机密计算（Confidential Computing，CoCo）VM 的 Linux 客户机。此类 VM 与物理处理器协作，以更好地保护 VM 内存中数据的机密性和完整性，即使面对可能已被攻破并表现出恶意行为的管理程序（hypervisor/VMM）也是如此。Hyper-V 上的 CoCo VM 共享 Documentation/security/snp-tdx-threat-model.rst 中描述的通用 CoCo VM 威胁模型与安全目标。请注意，Linux 中 Hyper-V 特定的代码将 CoCo VM 称为"isolated VMs"或"isolation VMs"。

Hyper-V 上的 Linux CoCo VM 需要以下部分的协作与交互：

- 支持 CoCo VM 的处理器所在的物理硬件

- 运行支持 CoCo VM 的 Windows/Hyper-V 版本的硬件

- 运行支持作为 CoCo VM 的 Linux 版本的 VM

物理硬件要求如下：

- 带有 SEV-SNP 的 AMD 处理器。Hyper-V 不会运行使用 AMD SME、SEV 或 SEV-ES 加密的客户机 VM，并且此类加密对于 Hyper-V 上的 CoCo VM 来说并不充分。

- 带有 TDX 的 Intel 处理器

要创建 CoCo VM，必须在创建 VM 时向 Hyper-V 指定"Isolated VM"属性。VM 一旦创建，便无法从 CoCo VM 更改为普通 VM，反之亦然。

### 运行模式


Hyper-V CoCo VM 可以运行于两种模式。模式在创建 VM 时选定，在 VM 的生命周期内无法更改。

- 完全 enlightened（fully-enlightened）模式。在此模式下，客户机操作系统被 enlightened，能够理解并管理作为 CoCo VM 运行的各个方面。

- Paravisor 模式。在此模式下，位于客户机与主机之间的 paravisor 层提供一些作为 CoCo VM 运行所需的操作。客户机操作系统所需的 CoCo enlightenment 可以少于 fully-enlightened 情况。

从概念上讲，fully-enlightened 模式与 paravisor 模式可被视为一个光谱上的两个点，该光谱涵盖了作为 CoCo VM 运行所需的客户机 enlightenment 程度。fully-enlightened 模式是光谱的一端。paravisor 模式的完整实现是光谱的另一端，在那一端，作为 CoCo VM 运行的各个方面都由 paravisor 处理，一个对内存加密或 CoCo VM 其他方面一无所知的普通客户机 OS 也能成功运行。然而，Hyper-V 对 paravisor 模式的实现并未走到这一步，而是处于光谱中间的某个位置。CoCo VM 的某些方面由 Hyper-V paravisor 处理，而客户机 OS 必须对另一方面进行 enlightenment。遗憾的是，paravisor 中可能提供的功能/特性没有标准化的枚举，客户机 OS 也没有标准化的机制向 paravisor 查询其提供的功能/特性。paravisor 提供什么的理解是硬编码在客户机 OS 中的。

Paravisor 模式与 `Coconut project`_ 有相似之处，后者旨在提供一个有限的 paravisor，为客户机提供服务，例如虚拟 TPM。然而，Hyper-V paravisor 通常处理的 CoCo VM 方面比目前为 Coconut 设想的更多，因此更接近于光谱中"无需客户机 enlightenment"的一端。


在 CoCo VM 威胁模型中，paravisor 处于客户机安全域中，且必须被客户机 OS 信任。由此推论，hypervisor/VMM 必须像防范潜在恶意的客户机一样，防范潜在恶意的 paravisor。

针对 fully-enlightened 与 paravisor 模式的硬件架构方法因底层处理器而异。

- 对于 AMD SEV-SNP 处理器，在 fully-enlightened 模式下客户机 OS 运行于 VMPL 0，并完全控制客户机上下文。在 paravisor 模式下，客户机 OS 运行于 VMPL 2，而 paravisor 运行于 VMPL 0。运行于 VMPL 0 的 paravisor 拥有客户机 OS（运行于 VMPL 2）所没有的特权。某些操作要求客户机调用 paravisor。此外，在 paravisor 模式下，客户机 OS 按照 SEV-SNP 架构的定义运行于"virtual Top Of Memory"（vTOM）模式。当使用 paravisor 时，此模式简化了客户机对内存加密的管理。

- 对于 Intel TDX 处理器，在 fully-enlightened 模式下客户机 OS 运行于 L1 VM。在 paravisor 模式下，使用 TD 分区。paravisor 运行于 L1 VM，而客户机 OS 运行于嵌套的 L2 VM。

Hyper-V 向客户机暴露一个描述 CoCo 模式的 synthetic MSR。该 MSR 指示底层处理器使用的是 AMD SEV-SNP 还是 Intel TDX，以及是否使用了 paravisor。构建一个能够在任一架构上、以任一模式正常引导并运行的单一 kernel 映像是很直接的。

### Paravisor 影响


运行于 paravisor 模式会影响通用 Linux kernel CoCo VM 功能的以下方面：

- 初始客户机内存设置。在 paravisor 模式下创建新 VM 时，paravisor 先运行，并将客户机物理内存设置为加密。客户机 Linux 进行正常的内存初始化，只是显式地将适当的范围标记为已解密（共享）。在 paravisor 模式下，Linux 不执行在 fully-enlightened 模式下配合 AMD SEV-SNP 特别棘手的早期引导内存设置步骤。

- #VC/#VE 异常处理。在 paravisor 模式下，Hyper-V 将客户机 CoCo VM 配置为将 #VC 和 #VE 异常分别路由到 VMPL 0 和 L1 VM，而不是客户机 Linux。因此，这些异常处理程序不在客户机 Linux 中运行，也不是 paravisor 模式下 Linux 客户机所需的 enlightenment。

- CPUID 标志。AMD SEV-SNP 和 Intel TDX 都在客户机中提供一个 CPUID 标志，指示该 VM 正在使用相应的硬件支持运行。虽然这些 CPUID 标志在 fully-enlightened CoCo VM 中可见，但 paravisor 会过滤掉这些标志，客户机 Linux 看不到它们。在整个 Linux kernel 中，显式测试这些标志的做法大多已被 cc_platform_has() 函数取代，目的是抽象 SEV-SNP 与 TDX 之间的差异。但 cc_platform_has() 抽象也允许 Hyper-V paravisor 配置在即便未设置 CPUID 标志时，有选择地启用 CoCo VM 功能的某些方面。例外是 SEV-SNP 上的早期引导内存设置，它会测试 CPUID SEV-SNP 标志。但 Hyper-V paravisor 模式 VM 中没有该标志，反而达到了不运行 SEV-SNP 特定早期引导内存设置所期望的效果。

- 设备模拟。在 paravisor 模式下，Hyper-V paravisor 提供对 IO-APIC 和 TPM 等设备的模拟。由于模拟发生在 paravisor 的客户机上下文中（而非 hypervisor/VMM 上下文），对这些设备的 MMIO 访问必须是加密引用，而不是 fully-enlightened CoCo VM 中所使用的已解密引用。__ioremap_caller() 函数已被增强，会进行一次回调以检查特定地址范围是否应被视为加密（私有）。参见"is_private_mmio"回调。

- 加密/解密内存转换。在 CoCo VM 中，在加密与解密之间转换客户机内存需要与 hypervisor/VMM 协调。这是通过 __set_memory_enc_pgtable() 调用的回调完成的。在 fully-enlightened 模式下，使用这些回调的普通 SEV-SNP 和 TDX 实现。在 paravisor 模式下，使用 Hyper-V 特定的回调集合。这些回调调用 paravisor，以便 paravisor 能够协调转换并在必要时通知 hypervisor。参见设置这些回调的 hv_vtom_init()。

- 中断注入。在 fully enlightened 模式下，恶意 hypervisor 可能在违反 x86/x64 架构规则的时刻向客户机 OS 注入中断。为了完整保护，客户机 OS 应包含使用 CoCo 能力处理器提供的中断注入管理特性的 enlightenment。在 paravisor 模式下，paravisor 中介对客户机 OS 的中断注入，并确保客户机 OS 只看到"合法"的中断。paravisor 使用 CoCo 能力物理处理器提供的中断注入管理特性，从而将这些复杂性对客户机 OS 屏蔽。

### Hyper-V 超级调用（Hypercalls）


在 fully-enlightened 模式下，Linux 客户机发出的 hypercall 会像在非 CoCo VM 中一样直接路由到 hypervisor。但在 paravisor 模式下，普通 hypercall 会先陷入 paravisor，paravisor 进而可能调用 hypervisor。但 paravisor 在这方面有特殊性，Linux 客户机发出的少数 hypercall 必须始终直接路由到 hypervisor。这些 hypercall 调用点会检测 paravisor 是否存在，并使用特殊的调用序列。例如参见 hv_post_message()。

### 客户机与 Hyper-V 的通信


除了 Linux CoCo VM 中 Linux kernel 对内存加密的通用处理之外，Hyper-V 还有 VMBus 以及使用 Linux 客户机与主机之间共享内存进行通信的 VMBus 设备。该共享内存必须标记为已解密才能启用通信。此外，由于威胁模型包含已遭攻破且潜在恶意的主机，客户机必须防范通过此共享内存向主机泄露任何非预期的数据。

这些 Hyper-V 与 VMBus 内存页被标记为已解密：

- VMBus 监视页（monitor pages）

- 合成中断控制器（SynIC）相关页（除非由 paravisor 提供）

- 每 CPU 的 hypercall 输入和输出页（除非与 paravisor 一起运行）

- VMBus 环形缓冲区。直接映射在 __vmbus_establish_gpadl() 中标记为已解密。在 hv_ringbuffer_init() 中创建的二级映射也必须包含"decrypted"属性。

当客户机向与主机共享的内存写入数据时，必须确保只写入预期的数据。在复制到共享内存之前，填充或 unused 字段必须初始化为零，以免随机 kernel 数据被无意中提供给主机。

类似地，当客户机读取与主机共享的内存时，必须在处理数据之前对其进行验证，以免恶意主机诱使客户机暴露非预期的数据。进行此类验证可能很棘手，因为主机即使在验证进行中或之后也能修改共享内存区域。对于在 VMBus 环形缓冲区中从主机传递给客户机的消息，会验证消息长度，并将消息复制到临时（加密）缓冲区以进行进一步验证和处理。复制会增加少量开销，但这是防范恶意主机的唯一方法。参见 hv_pkt_iter_first()。

许多 VMBus 设备的驱动已通过添加代码来充分验证通过 VMBus 接收的消息而"加固（hardened）"，而不是假设 Hyper-V 在协作运行。此类驱动在 vmbus_devs[] 表中被标记为"allowed_in_isolated"。CoCo VM 中不需要的其他 VMBus 设备驱动尚未加固，它们不允许在 CoCo VM 中加载。参见排除此类设备的 vmbus_is_valid_offer()。

两个 VMBus 设备依赖 Hyper-V 主机进行 DMA 数据传输：用于磁盘 I/O 的 storvsc 和用于网络 I/O 的 netvsc。storvsc 使用普通的 Linux kernel DMA API，因此通过已解密 swiotlb 内存的反弹缓冲（bounce buffering）是隐式完成的。netvsc 有两种数据传输模式。第一种模式经过 netvsc 驱动显式分配的发送和接收缓冲区空间，用于大多数较小的数据包。这些发送和接收缓冲区由 __vmbus_establish_gpadl() 标记为已解密。由于 netvsc 驱动显式地将数据包复制进/出这些缓冲区，加密与解密内存之间的反弹缓冲等效操作已经是数据路径的一部分。第二种模式使用普通的 Linux kernel DMA API，并像 storvsc 一样隐式地通过 swiotlb 内存进行反弹缓冲。

最后，VMBus 虚拟 PCI 驱动在 CoCo VM 中需要特殊处理。Linux PCI 设备驱动使用 Linux PCI 子系统提供的标准 API 访问 PCI 配置空间。在 Hyper-V 上，这些函数直接访问 MMIO 空间，访问会陷入 Hyper-V 进行模拟。但在 CoCo VM 中，内存加密阻止 Hyper-V 读取客户机指令流来模拟该访问。因此在 CoCo VM 中，这些函数必须发起一个 hypercall，以参数显式描述该访问。参见 _hv_pcifront_read_config() 和 _hv_pcifront_write_config() 以及指示使用 hypercall 的"use_calls"标志。

### 机密 VMBus（Confidential VMBus）


机密 VMBus 使机密客户机无需与不可信的主机分区和不可信的 hypervisor 交互。相反，客户机依赖可信的 paravisor 与处理敏感数据的设备通信。硬件（SNP 或 TDX）对客机内存和寄存器状态进行加密，同时使用平台安全处理器对 paravisor 映像进行度量，以确保可信且机密的 computing。

机密 VMBus 在客户机与 paravisor 之间提供安全的通信通道，确保敏感数据通过内存加密和寄存器状态隔离而免受 hypervisor 级别的访问。

机密 VMBus 是机密计算（Confidential Computing，CoCo）VM（在 Hyper-V 术语中又称"Isolated" VM）的扩展。没有机密 VMBus 时，客户机 VMBus 设备驱动（VMBus 术语中的"VSC"）与运行于 Hyper-V 主机上的 VMBus 服务器（VSP）通信。通信必须通过已解密的内存，以便主机能够访问。有了机密 VMBus，一个或多个 VSP 驻留在客户机 VM 中可信的 paravisor 层。由于 paravisor 层也运行于加密内存中，与此类 VSP 通信所用的内存无需解密并因此暴露给 Hyper-V 主机。paravisor 负责在必要时与 Hyper-V 主机安全地通信。

数据直接在 VM 与 vPCI 设备（又称 PCI pass-thru 设备，参见 [vpci](vpci)）之间传输，该设备直接分配给 VTL2 并支持加密内存。在这种情况下，主机分区和 hypervisor 都无法访问该数据。客户机只需与 paravisor 建立 VMBus 连接，用于处理敏感数据的通道，而 paravisor 将与该特定设备通信的细节抽象掉，向客户机提供在 Hyper-V 驱动中已受支持十年的成熟 VSP（Virtual Service Provider）接口。

如果设备不支持加密内存，paravisor 会提供反弹缓冲（bounce-buffering），虽然数据未加密，但后台页不会通过 SLAT 映射到主机分区。尽管并非不可能，但与传统 VMBus 连接（主机分区可直接访问用于通信的内存）相比，主机分区渗透（exfiltrate）数据要困难得多。

下面是传统 VMBus 连接的数据流（`C` 代表客户端或 VSC，`S` 代表服务端或 VSP，`DEVICE` 是物理设备，可能
```
  +---- GUEST ----+       +----- DEVICE ----+        +----- HOST -----+
  |               |       |                 |        |                |
  |               |       |                 |        |                |
  |               |       |                 ==========                |
  |               |       |                 |        |                |
  |               |       |                 |        |                |
  |               |       |                 |        |                |
  +----- C -------+       +-----------------+        +------- S ------+
         ||                                                   ||
         ||                                                   ||
  +------||------------------ VMBus --------------------------||------+
  |                     Interrupts, MMIO                              |
  +-------------------------------------------------------------------+

```
```
  +---- GUEST --------------- VTL0 ------+               +-- DEVICE --+
  |                                      |               |            |
  | +- PARAVISOR --------- VTL2 -----+   |               |            |
  | |     +-- VMBus Relay ------+    ====+================            |
  | |     |   Interrupts, MMIO  |    |   |               |            |
  | |     +-------- S ----------+    |   |               +------------+
  | |               ||               |   |
  | +---------+     ||               |   |
  | |  Linux  |     ||    OpenHCL    |   |
  | |  kernel |     ||               |   |
  | +---- C --+-----||---------------+   |
  |       ||        ||                   |
  +-------++------- C -------------------+               +------------+
          ||                                             |    HOST    |
          ||                                             +---- S -----+
  +-------||----------------- VMBus ---------------------------||-----+
  |                     Interrupts, MMIO                              |
  +-------------------------------------------------------------------+

```
提供机密 VMBus 通道的 VMBus relay 实现，作为 OpenHCL paravisor 的一部分在 OpenVMM 项目中可用。更多信息请参考

  - https://openvmm.dev/，以及
  - https://github.com/microsoft/openvmm

以了解 OpenHCL paravisor。

与 paravisor 一起运行的客户机必须在运行时确定当前 paravisor 是否支持机密 VMBus。x86_64 特定的方法依赖于 CPUID Virtualization Stack leaf；ARM64 实现在运行 ARM CCA 客户机时预期无条件支持机密 VMBus。

机密 VMBus 是整个 VMBus 连接以及所创建的每个 VMBus 通道的一个特征。当建立机密 VMBus 连接时，paravisor 向客户机提供用于 VMBus 设备创建和删除的消息传递路径，并提供每 CPU 的合成中断控制器（SynIC），就像 Hyper-V 主机提供的 SynIC 一样。提供给客户机的每个 VMBus 设备都指示其参与机密 VMBus 的程度。该 offer 指示设备是否使用加密环形缓冲区，以及设备是否对环形缓冲区之外完成的 DMA 使用加密内存。对于使用同一机密 VMBus 连接的不同设备，这些设置可能不同。

尽管这些设置是分开的，但在实践中只会是仅加密环形缓冲区，或同时加密环形缓冲区和外部数据。如果通道由 paravisor 以机密 VMBus 提供，环形缓冲区总是可以加密，因为它严格用于 VTL2 paravisor 与 VTL0 客户机之间的通信。然而，其他内存区域常用于 DMA 等，因此它们需要底层硬件可访问，并且必须未加密（除非设备支持加密内存）。目前，OpenHCL 中没有任何支持加密外部内存的 VSP，但未来版本预期会启用此能力。

由于机密 VMBus 上的某些设备可能需要已解密的环形缓冲区和 DMA 传输，客户机必须与两个 SynIC 交互——一个是 paravisor 提供的，另一个是在不提供机密 VMBus 时由 Hyper-V 主机提供的。中断总是由 paravisor SynIC 发出信号，但客户机必须在两个 SynIC 上检查消息和通道中断。

在机密 VMBus 的情况下，客户机对 SynIC 的常规访问会被 paravisor 拦截（这包括各种 MSR，如 SIMP 和 SIEFP，以及像 HvPostMessage 和 HvSignalEvent 这样的 hypercall）。如果客户机确实想要与 hypervisor 通信，它必须使用特殊机制（SNP 上的 GHCB 页，或 TDX 上的 tdcall）。消息可以是任一种：使用机密 VMBus 时，消息使用 paravisor SynIC；如果客户机选择直接与 hypervisor 通信，则使用 hypervisor SynIC。对于中断信号，某些通道可能运行在主机上（非机密，使用 VMBus relay）并使用 hypervisor SynIC，某些运行在 paravisor 上并使用其 SynIC。RelIDs 由 OpenHCL VMBus 服务器协调，无论通道起源于主机还是 paravisor，都保证唯一。

### load_unaligned_zeropad()


在加密与解密之间转换内存时，set_memory_encrypted() 或 set_memory_decrypted() 的调用者负责确保内存未被使用，且在转换进行期间不被引用。转换有多个步骤，并包含与 Hyper-V 主机的交互。在全部步骤完成之前，内存处于不一致状态。在状态不一致时进行引用可能导致无法干净修复的异常。

然而，kernel 的 load_unaligned_zeropad() 机制可能产生调用者无法阻止的游离引用，因此在 #VC 或 #VE 异常处理程序中有特定代码修复此类情况。但在 Hyper-V 上运行的 CoCo VM 可能被配置为与 paravisor 一起运行，且 #VC 或 #VE 异常被路由到 paravisor。没有架构层面的方法将这些异常转发回客户机 kernel，在这种情况下，#VC/#VE 处理程序中的 load_unaligned_zeropad() 修复代码不会运行。

为避免此问题，用于通知 hypervisor 转换发生的 Hyper-V 特定函数在转换进行期间将页标记为"not present"。如果 load_unaligned_zeropad() 导致游离引用，会生成普通页错误（page fault）而不是 #VC 或 #VE，并且 load_unaligned_zeropad() 基于页错误的处理程序会修复该引用。当加密/解密转换完成时，页会重新标记为"present"。参见 hv_vtom_clear_present() 和 hv_vtom_set_host_visibility()。
