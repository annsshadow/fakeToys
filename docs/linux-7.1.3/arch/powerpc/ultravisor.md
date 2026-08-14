
## 受保护执行设施（Protected Execution Facility）


    :depth: 3

# 简介


    Protected Execution Facility（PEF，受保护执行设施）是 POWER 9 的一项架构改动，
    用于启用安全虚拟机（SVM）。DD2.3 芯片（PVR=0x004e1203）或更高版本将具备 PEF 能力。
    一个新的 ISA 版本将包含 PEF RFC02487 的改动。

    启用后，PEF 向 POWER 架构添加了一种新的、特权更高的模式，称为 Ultravisor（超级
    监管者）模式。配合这一新模式，还出现了一个新的固件，称为 Protected Execution
    Ultravisor（受保护执行 Ultravisor，简称 Ultravisor）。Ultravisor 模式是 POWER
    架构中特权最高的模式。

	+------------------+
	| Privilege States |
	+==================+
	|  Problem         |
	+------------------+
	|  Supervisor      |
	+------------------+
	|  Hypervisor      |
	+------------------+
	|  Ultravisor      |
	+------------------+

    PEF 保护 SVM 免受 Hypervisor、特权用户以及系统中其他虚拟机的侵害。SVM 在静止状态下
    也受到保护，并且只能由经过授权的机器执行。所有虚拟机都利用 Hypervisor 服务。
    Ultravisor 会过滤 SVM 与 Hypervisor 之间的调用，以确保信息不会意外泄露。除 H_RANDOM
    之外的所有 hypercall（超级调用）都会被反射（reflect）到 Hypervisor。H_RANDOM 不被
    反射，以防止 Hypervisor 影响 SVM 中的随机值。

    为了支持这一点，需要对 CPU 中资源的所有权进行重构。一些先前属于 Hypervisor 特权的
    资源现在改为属于 Ultravisor 特权。

## 硬件


    Hardware（硬件）方面的改动包括以下内容：

    - MSR 中有一个新的位，用于确定当前进程是否在安全模式下运行，即 MSR(S) 位 41。
      MSR(S)=1 时，进程处于安全模式；MSR(s)=0 时，进程处于普通模式。

    - MSR(S) 位只能由 Ultravisor 设置。

    - HRFID 不能用于设置 MSR(S) 位。如果 Hypervisor 需要返回到某个 SVM，它必须使用
      ultracall（超级调用）。它可以确定要返回的 VM 是否是安全的。

    - 有一个新的 Ultravisor 特权寄存器 SMFCTRL，其中有一个使能/禁用位 SMFCTRL(E)。

    - 进程的特权现在由三个 MSR 位 MSR(S, HV, PR) 决定。在下面每个表中，模式按从最低
      特权到最高特权排列。较高特权的模式可以访问较低特权模式的所有资源。

      **安全模式 MSR 设置**

      +---+---+---+---------------+
      | S | HV| PR|Privilege      |
      +===+===+===+===============+
      | 1 | 0 | 1 | Problem       |
      +---+---+---+---------------+
      | 1 | 0 | 0 | Privileged(OS)|
      +---+---+---+---------------+
      | 1 | 1 | 0 | Ultravisor    |
      +---+---+---+---------------+
      | 1 | 1 | 1 | Reserved      |
      +---+---+---+---------------+

      **普通模式 MSR 设置**

      +---+---+---+---------------+
      | S | HV| PR|Privilege      |
      +===+===+===+===============+
      | 0 | 0 | 1 | Problem       |
      +---+---+---+---------------+
      | 0 | 0 | 0 | Privileged(OS)|
      +---+---+---+---------------+
      | 0 | 1 | 0 | Hypervisor    |
      +---+---+---+---------------+
      | 0 | 1 | 1 | Problem (Host)|
      +---+---+---+---------------+

    - 内存被划分为安全内存与普通内存。只有运行在安全模式下的进程才能访问安全内存。

    - 硬件不允许任何未运行在安全模式下的实体访问安全内存。这意味着 Hypervisor 无法
      在不使用 ultracall（请求 Ultravisor）的情况下访问 SVM 的内存。Ultravisor 只会
      允许 Hypervisor 以加密形式看到 SVM 的内存。

    - I/O 系统不允许直接寻址安全内存。这限制 SVM 只能使用虚拟 I/O。

    - 架构允许 SVM 与 Hypervisor 共享不受加密保护的页面。但是，这种共享必须由 SVM 发起。

    - 当进程运行在安全模式时，所有 hypercall（syscall lev=1）都会进入 Ultravisor。

    - 当进程处于安全模式时，所有中断都会进入 Ultravisor。

    - 以下资源已成为 Ultravisor 特权资源，需要 Ultravisor 接口才能进行操控：

      - 处理器配置寄存器（SCOM）。

      - 停止状态（stop state）信息。

      - 调试寄存器 CIABR、DAWR 和 DAWRX，当 SMFCTRL(D) 被设置时。如果 SMFCTRL(D) 未
        设置，则它们在安全模式下不起作用。当被设置时，读写需要一次 Ultravisor 调用，
        否则将导致一次 Hypervisor Emulation Assistance（Hypervisor 仿真辅助）中断。

      - PTCR 与分区表项（分区表位于安全内存中）。尝试写入 PTCR 将导致一次 Hypervisor
        Emulation Assistance 中断。

      - LDBAR（LD Base Address Register，加载基址寄存器）与 IMC（In-Memory Collection，
        内存内采集）非架构寄存器。尝试写入它们将导致一次 Hypervisor Emulation
        Assistance 中断。

      - SVM 的分页、与 Hypervisor 共享 SVM 的内存。（包括 Virtual Processor Area（VPA，
        虚拟处理器区）与虚拟 I/O。）


## 软件/微码


    Software/Microcode（软件/微码）方面的改动包括：

    - SVM 是使用 IBM 提供的（开源）工具从普通 VM 创建的。

    - 所有 SVM 都作为普通 VM 启动，并利用一次 ultracall，即 UV_ESM（Enter Secure Mode，
      进入安全模式）来完成转换。

    - 当进行 UV_ESM ultracall 时，Ultravisor 将 VM 复制到安全内存，解密验证信息，并
      检查 SVM 的完整性。如果完整性检查通过，Ultravisor 将在安全模式下移交控制权。

    - 验证信息包含与 SVM 关联的加密磁盘的口令（pass phrase）。该口令在 SVM 请求时
      提供给它。

    - Ultravisor 不参与保护处于静止状态的 SVM 加密磁盘。

    - 对于外部中断，Ultravisor 保存 SVM 的状态，并将中断反射给 Hypervisor 进行处理。
      对于 hypercall，Ultravisor 向所有 hypercall 不需要的寄存器插入中性状态，然后将
      调用反射给 Hypervisor 处理。H_RANDOM hypercall 由 Ultravisor 执行，不被反射。

    - 为了使虚拟 I/O 工作，必须进行弹跳缓冲（bounce buffering）。

    - Ultravisor 使用 AES（IAPM）来保护 SVM 内存。IAPM 是 AES 的一种模式，可同时提供
      完整性与机密性。

    - 普通页面与安全页面之间数据的移动，由 Hypervisor 中一个新的 HMM 插件与 Ultravisor
      协调完成。

    Ultravisor 向 Hypervisor 与 SVM 提供新的服务。这些服务通过 ultracall 访问。

## 术语


    - Hypercalls（超级调用）：用于向 Hypervisor 请求服务的特殊系统调用。

    - Normal memory（普通内存）：Hypervisor 可访问的内存。

    - Normal page（普通页）：由普通内存支持、可供 Hypervisor 使用的页。

    - Shared page（共享页）：由普通内存支持、Hypervisor/QEMU 与 SVM 均可访问的页
      （即该页在 SVM 与 Hypervisor/QEMU 中都有映射）。

    - Secure memory（安全内存）：仅 Ultravisor 与 SVM 可访问的内存。

    - Secure page（安全页）：由安全内存支持、仅 Ultravisor 与 SVM 可访问的页。

    - SVM：Secure Virtual Machine（安全虚拟机）。

    - Ultracalls（超级调用）：用于向 Ultravisor 请求服务的特殊系统调用。


# Ultravisor 调用 API


    本节描述支持安全虚拟机（SVM）与半虚拟化 KVM 所需的 Ultravisor 调用（ultracall）。
    ultracall 允许 SVM 与 Hypervisor 向 Ultravisor 请求服务，例如访问只能在 Ultravisor
    特权模式下运行时才能访问的寄存器或内存区域。

    需要由 ultracall 提供的特定服务在寄存器 R3 中指定（ultracall 的第一个参数）。
    ultracall 的其他参数（如果有）在寄存器 R4 到 R12 中指定。

    所有 ultracall 的返回值都在寄存器 R3 中。ultracall 的其他输出值（如果有）在寄存器
    R4 到 R12 中返回。这种寄存器用法唯一的例外是下面描述的 `UV_RETURN` ultracall。

    每个 ultracall 返回在特定 ultracall 上下文中适用的特定错误码。不过，与 PowerPC
    Architecture Platform Reference（PAPR，PowerPC 架构平台参考）一样，如果没有为特定
    情况定义具体的错误码，那么 ultracall 将回退到基于错误参数位置（parameter-position
    based）的码，即 U_PARAMETER、U_P2、U_P3 等，取决于可能导致错误的 ultracall 参数。

    一些 ultracall 涉及在 Ultravisor 与 Hypervisor 之间传输一页数据。从安全内存传输到
    普通内存的安全页可以使用动态生成的密钥进行加密。当安全页被传回安全内存时，可以使用
    相同的动态生成密钥进行解密。这些密钥的生成与管理将在单独的文档中说明。

    目前这里只涵盖 Hypervisor 与 SVM 当前已实现并正在使用的 ultracall，但在合理时可以在
    此添加其他 ultracall。

    所有 hypercall/ultracall 的完整规范最终将在 PAPR 规范的 public/OpenPower 版本中
    提供。

```

        If PEF is not enabled, the ultracalls will be redirected to the
        Hypervisor which must handle/fail the calls.

```
## Hypervisor 使用的 Ultracalls


    本节描述 Hypervisor 用于管理 SVM 的虚拟内存管理 ultracall。

### UV_PAGE_OUT


    将一页内容加密并从安全内存移动到普通内存。

#### Syntax


	uint64_t ultracall(const uint64_t UV_PAGE_OUT,
		uint16_t lpid,		/** LPAR ID **/
		uint64_t dest_ra,	/** real address of destination page **/
		uint64_t src_gpa,	/** source guest-physical-address **/
		uint8_t  flags,		/** flags **/
		uint64_t order)		/** page size order **/

#### Return values


    以下值之一：

 - U_SUCCESS	on success.
 - U_PARAMETER	if `lpid` is invalid.
 - U_P2 		if `dest_ra` is invalid.
 - U_P3		if the `src_gpa` address is invalid.
 - U_P4		if any bit in the `flags` is unrecognized
 - U_P5		if the `order` parameter is unsupported.
 - U_FUNCTION	if functionality is not supported.
 - U_BUSY	if page cannot be currently paged-out.

#### Description


    加密一个 secure-page（安全页）的内容，并使其在普通页中可供 Hypervisor 使用。

    默认情况下，源页会从 SVM 的分区作用域页表（partition-scoped page table）中取消映射。
    但 Hypervisor 可以通过在 `flags` 参数中设置 `UV_SNAPSHOT` 标志，向 Ultravisor 提供
    保留该页映射的提示。

    如果源页已经是共享页，则该调用返回 U_SUCCESS，不做任何操作。

#### Use cases


    #. QEMU 尝试访问属于 SVM 的某个地址，但该地址的页帧尚未映射到 QEMU 的地址空间。
       在这种情况下，Hypervisor 将分配一个页帧，将其映射到 QEMU 的地址空间，并发出
       `UV_PAGE_OUT` 调用以取回该页的加密内容。

    #. 当 Ultravisor 安全内存不足，需要换出（page-out）一个 LRU 页时。此时 Ultravisor
       会向 Hypervisor 发出 `H_SVM_PAGE_OUT` hypercall。然后 Hypervisor 将分配一个普通
       页，并发出 `UV_PAGE_OUT` ultracall，Ultravisor 则将该安全页的内容加密并移动到
       普通页中。

    #. 当 Hypervisor 访问 SVM 数据时，Hypervisor 请求 Ultravisor 将相应的页传输到一个
       非安全页，Hypervisor 可以访问该页。不过普通页中的数据将是加密的。

### UV_PAGE_IN


    将一页内容从普通内存移动到安全内存。

#### Syntax


	uint64_t ultracall(const uint64_t UV_PAGE_IN,
		uint16_t lpid,		/** the LPAR ID **/
		uint64_t src_ra,	/** source real address of page **/
		uint64_t dest_gpa,	/** destination guest physical address **/
		uint64_t flags,		/** flags **/
		uint64_t order)		/** page size order **/

#### Return values


    以下值之一：

 - U_SUCCESS	on success.
 - U_BUSY	if page cannot be currently paged-in.
 - U_FUNCTION	if functionality is not supported
 - U_PARAMETER	if `lpid` is invalid.
 - U_P2 		if `src_ra` is invalid.
 - U_P3		if the `dest_gpa` address is invalid.
 - U_P4		if any bit in the `flags` is unrecognized
 - U_P5		if the `order` parameter is unsupported.

#### Description


    将 `src_ra` 标识的页的内容从普通内存移动到安全内存，并将其映射到客户机物理地址
    `dest_gpa`。

    如果 `dest_gpa` 引用一个共享地址，则将该页映射到 SVM 的分区作用域页表中。如果
    `dest_gpa` 不是共享的，则将该页的内容复制到相应的安全页中。根据上下文，在复制前对
    该页进行解密。

    调用者通过 `flags` 参数提供页的属性。`flags` 的有效值为：

 - CACHE_INHIBITED
 - CACHE_ENABLED
 - WRITE_PROTECTION

    在进行 `UV_PAGE_IN` ultracall 之前，Hypervisor 必须将页固定在内存中。

#### Use cases


    #. 当普通 VM 切换到安全模式时，其驻留在普通内存中的所有页都被移动到安全内存中。

    #. 当 SVM 请求与 Hypervisor 共享一页时，Hypervisor 分配一页并告知 Ultravisor。

    #. 当 SVM 访问已被换出（page-out）的安全页时，Ultravisor 调用 Hypervisor 来定位该
       页。定位到该页后，Hypervisor 使用 UV_PAGE_IN 使该页对 Ultravisor 可用。

### UV_PAGE_INVAL


    使 Ultravisor 对一页的映射失效。

#### Syntax


	uint64_t ultracall(const uint64_t UV_PAGE_INVAL,
		uint16_t lpid,		/** the LPAR ID **/
		uint64_t guest_pa,	/** destination guest-physical-address **/
		uint64_t order)		/** page size order **/

#### Return values


    以下值之一：

 - U_SUCCESS	on success.
 - U_PARAMETER	if `lpid` is invalid.
 - U_P2 		if `guest_pa` is invalid (or corresponds to a secure
                        page mapping).
 - U_P3		if the `order` is invalid.
 - U_FUNCTION	if functionality is not supported.
 - U_BUSY	if page cannot be currently invalidated.

#### Description


    此 ultracall 告知 Ultravisor，Hypervisor 中对应于给定客户机物理地址的页映射已失效，
    Ultravisor 不应再访问该页。如果指定的 `guest_pa` 对应于一个安全页，Ultravisor 将
    忽略使其失效的尝试并返回 U_P2。

#### Use cases


    #. 当共享页从 QEMU 的页表中取消映射（可能是因为它被换出到磁盘）时，Ultravisor 需要
       知道该页也不应从它这一侧被访问。


### UV_WRITE_PATE


    验证并写入给定分区的分区表项（PATE）。

#### Syntax


	uint64_t ultracall(const uint64_t UV_WRITE_PATE,
		uint32_t lpid,		/** the LPAR ID **/
		uint64_t dw0		/** the first double word to write **/
		uint64_t dw1)		/** the second double word to write **/

#### Return values


    以下值之一：

 - U_SUCCESS	on success.
 - U_BUSY	if PATE cannot be currently written to.
 - U_FUNCTION	if functionality is not supported.
 - U_PARAMETER	if `lpid` is invalid.
 - U_P2 		if `dw0` is invalid.
 - U_P3		if the `dw1` address is invalid.
 - U_PERMISSION	if the Hypervisor is attempting to change the PATE
			of a secure virtual machine or if called from a
			context other than Hypervisor.

#### Description


    验证并写入给定 LPID 及其分区表项。如果 LPID 已分配并初始化，此调用将导致更改分区表项。

#### Use cases


    #. 分区表驻留在安全内存中，其各项（称为 PATE，Partition Table Entries，分区表项）
       指向 Hypervisor 以及每个虚拟机（包括安全与普通）的分区作用域页表。Hypervisor 在
       分区 0 中运行，其分区作用域页表驻留在普通内存中。

    #. 此 ultracall 允许 Hypervisor 向 Ultravisor 注册 Hypervisor 与其他分区（虚拟机）
       的分区作用域与进程作用域页表项。

    #. 如果现有分区（VM）的 PATE 值发生变化，该分区的 TLB 缓存会被刷新。

    #. Hypervisor 负责分配 LPID。LPID 与其 PATE 项一起注册。Hypervisor 管理普通 VM 的
       PATE 项，并可以随时更改。Ultravisor 管理 SVM 的 PATE 项，不允许 Hypervisor 修改
       它们。

### UV_RETURN


    在处理完被转发（又称 **reflected**，反射）给 Hypervisor 的 hypercall 或中断后，将
    控制权从 Hypervisor 交还给 Ultravisor。

#### Syntax


	uint64_t ultracall(const uint64_t UV_RETURN)

#### Return values


     成功时此调用绝不返回到 Hypervisor。如果 ultracall 不是从 Hypervisor 上下文发出，
     则返回 U_INVALID。

#### Description


    当 SVM 发出 hypercall 或遭遇其他异常时，Ultravisor 通常将异常转发（又称 **reflects**，
    反射）给 Hypervisor。处理完异常后，Hypervisor 使用 `UV_RETURN` ultracall 将控制权
    交还给 SVM。

    进入此 ultracall 时期望的寄存器状态为：

    - 非易失寄存器被恢复为其原始值。
    - 如果从 hypercall 返回，寄存器 R0 包含返回值（**与其他 ultracall 不同**），并且
      寄存器 R4 到 R12 包含 hypercall 的任何输出值。
    - R3 包含 ultracall 编号，即 UV_RETURN。
    - 如果带着合成的中断返回，R2 包含合成的中断号。

#### Use cases


    #. Ultravisor 依赖 Hypervisor 为 SVM 提供若干服务，例如处理 hypercall 与其他异常。
       处理完异常后，Hypervisor 使用 UV_RETURN 将控制权交还给 Ultravisor。

    #. Hypervisor 必须使用此 ultracall 将控制权交还给 SVM。


### UV_REGISTER_MEM_SLOT


    以指定属性注册一个 SVM 地址范围。

#### Syntax


	uint64_t ultracall(const uint64_t UV_REGISTER_MEM_SLOT,
		uint64_t lpid,		/** LPAR ID of the SVM **/
		uint64_t start_gpa,	/** start guest physical address **/
		uint64_t size,		/** size of address range in bytes **/
		uint64_t flags		/** reserved for future expansion **/
		uint16_t slotid)	/** slot identifier **/

#### Return values


    以下值之一：

 - U_SUCCESS	on success.
 - U_PARAMETER	if `lpid` is invalid.
 - U_P2 		if `start_gpa` is invalid.
 - U_P3		if `size` is invalid.
 - U_P4		if any bit in the `flags` is unrecognized.
 - U_P5		if the `slotid` parameter is unsupported.
 - U_PERMISSION	if called from context other than Hypervisor.
 - U_FUNCTION	if functionality is not supported.


#### Description


    为 SVM 注册一个内存范围。该内存范围从客户机物理地址 `start_gpa` 开始，长度为 `size`
    字节。

#### Use cases


    #. 当虚拟机变为安全时，Hypervisor 管理的所有内存槽都进入安全内存。Hypervisor 遍历
       每个内存槽，并向 Ultravisor 注册该槽。Hypervisor 可能会丢弃某些槽，例如用于固件
       （SLOF）的槽。

    #. 当热插拔（hot-plug）新内存时，会注册一个新的内存槽。


### UV_UNREGISTER_MEM_SLOT


    注销先前使用 UV_REGISTER_MEM_SLOT 注册的 SVM 地址范围。

#### Syntax


	uint64_t ultracall(const uint64_t UV_UNREGISTER_MEM_SLOT,
		uint64_t lpid,		/** LPAR ID of the SVM **/
		uint64_t slotid)	/** reservation slotid **/

#### Return values


    以下值之一：

 - U_SUCCESS	on success.
 - U_FUNCTION	if functionality is not supported.
 - U_PARAMETER	if `lpid` is invalid.
 - U_P2 		if `slotid` is invalid.
 - U_PERMISSION	if called from context other than Hypervisor.

#### Description


    释放由 `slotid` 标识的内存槽，并释放分配给该预留的所有资源。

#### Use cases


    #. 内存热移除（hot-remove）。


### UV_SVM_TERMINATE


    终止一个 SVM 并释放其资源。

#### Syntax


	uint64_t ultracall(const uint64_t UV_SVM_TERMINATE,
		uint64_t lpid,		/** LPAR ID of the SVM **/)

#### Return values


    以下值之一：

 - U_SUCCESS	on success.
 - U_FUNCTION	if functionality is not supported.
 - U_PARAMETER	if `lpid` is invalid.
 - U_INVALID	if VM is not secure.
 - U_PERMISSION  if not called from a Hypervisor context.

#### Description


    终止一个 SVM 并释放其所有资源。

#### Use cases


    #. 在终止 SVM 时由 Hypervisor 调用。


## SVM 使用的 Ultracalls


### UV_SHARE_PAGE


    与 Hypervisor 共享一组客户机物理页。

#### Syntax


	uint64_t ultracall(const uint64_t UV_SHARE_PAGE,
		uint64_t gfn,	/** guest page frame number **/
		uint64_t num)	/** number of pages of size PAGE_SIZE **/

#### Return values


    以下值之一：

 - U_SUCCESS	on success.
 - U_FUNCTION	if functionality is not supported.
 - U_INVALID	if the VM is not secure.
 - U_PARAMETER	if `gfn` is invalid.
 - U_P2 		if `num` is invalid.

#### Description


    与 Hypervisor 共享从客户机物理帧号 `gfn` 开始的 `num` 个页。假设页大小为 PAGE_SIZE
    字节。在返回前将页清零。

    如果该地址已由安全页支持，则取消该页的映射，并在 Hypervisor 的帮助下用非安全页
    支持它。如果它还未被任何页支持，则将 PTE 标记为不安全，并在访问该地址时用非安全页
    支持它。如果它已经由非安全页支持，则将页清零并返回。

#### Use cases


    #. Hypervisor 无法访问 SVM 的页，因为它们由安全页支持。因此 SVM 必须显式地向
       Ultravisor 请求能与 Hypervisor 共享的页。

    #. SVM 中需要共享页来支持 virtio 与 Virtual Processor Area（VPA，虚拟处理器区）。


### UV_UNSHARE_PAGE


    将共享的 SVM 页恢复到其初始状态。

#### Syntax


	uint64_t ultracall(const uint64_t UV_UNSHARE_PAGE,
		uint64_t gfn,	/** guest page frame number **/
		uint73 num)	/** number of pages of size PAGE_SIZE**/

#### Return values


    以下值之一：

 - U_SUCCESS	on success.
 - U_FUNCTION	if functionality is not supported.
 - U_INVALID	if VM is not secure.
 - U_PARAMETER	if `gfn` is invalid.
 - U_P2 		if `num` is invalid.

#### Description


    停止与 Hypervisor 共享从 `gfn` 开始的 `num` 个页。假设页大小为 PAGE_SIZE。在返回前
    将页清零。

    如果该地址已由非安全页支持，则取消该页的映射，并用安全页支持它。告知 Hypervisor
    释放对其共享页的引用。如果该地址尚未被页支持，则将 PTE 标记为安全，并在访问该地址
    时用安全页支持它。如果它已经由安全页支持，则将页清零并返回。

#### Use cases


    #. SVM 可能决定取消与 Hypervisor 共享某个页。


### UV_UNSHARE_ALL_PAGES


    取消 SVM 与 Hypervisor 共享的所有页。

#### Syntax


	uint64_t ultracall(const uint64_t UV_UNSHARE_ALL_PAGES)

#### Return values


    以下值之一：

 - U_SUCCESS	on success.
 - U_FUNCTION	if functionality is not supported.
 - U_INVAL	if VM is not secure.

#### Description


    取消与 Hypervisor 共享的所有页。所有被取消共享的页在返回时都被清零。只有由 SVM 显式
    与 Hypervisor 共享的页（使用 UV_SHARE_PAGE ultracall）才会被取消共享。Ultravisor 可能
    在内部与 Hypervisor 共享某些页而无需 SVM 显式请求。这些页不会被此 ultracall 取消
    共享。

#### Use cases


    #. 当使用 `kexec` 引导不同的内核时需要此调用。在 SVM 重置期间也可能需要。

### UV_ESM


    保护虚拟机（**进入安全模式**）。

#### Syntax


	uint64_t ultracall(const uint64_t UV_ESM,
		uint64_t esm_blob_addr,	/** location of the ESM blob **/
		unint64_t fdt)		/** Flattened device tree **/

#### Return values


    以下值之一：

 - U_SUCCESS	on success (including if VM is already secure).
 - U_FUNCTION	if functionality is not supported.
 - U_INVALID	if VM is not secure.
 - U_PARAMETER	if `esm_blob_addr` is invalid.
 - U_P2 		if `fdt` is invalid.
 - U_PERMISSION	if any integrity checks fail.
 - U_RETRY	insufficient memory to create SVM.
 - U_NO_KEY	symmetric key unavailable.

#### Description


    保护虚拟机。成功完成后，在 ESM blob 中指定的地址将控制权交还给虚拟机。

#### Use cases


    #. 普通虚拟机可以选择切换到安全模式。

# Hypervisor 调用 API


    本文档描述支持 Ultravisor 所需的 Hypervisor 调用（hypercall）。Hypercall 是 Hypervisor
    提供给虚拟机与 Ultravisor 的服务。

    这些 hypercall 的寄存器使用方式与 Power Architecture Platform Reference（PAPR）文档
    中定义的其他 hypercall 相同。即在输入时，寄存器 R3 标识所请求的具体服务，寄存器 R4
    到 R11 包含 hypercall 的其他参数（如果有）。在输出时，寄存器 R3 包含返回值，寄存器
    R4 到 R9 包含 hypercall 的任何其他输出值。

    本文档仅涵盖当前已实现/计划用于 Ultravisor 的 hypercall，但在合理时可以在此添加
    其他 hypercall。

    所有 hypercall/ultracall 的完整规范最终将在 PAPR 规范的 public/OpenPower 版本中
    提供。

## 支持 Ultravisor 的 Hypervisor 调用


    以下是一组支持 Ultravisor 所需的 hypercall。

### H_SVM_INIT_START


    开始将普通虚拟机转换为 SVM 的过程。

#### Syntax


	uint64_t hypercall(const uint64_t H_SVM_INIT_START)

#### Return values


    以下值之一：

 - H_SUCCESS	 on success.
        - H_STATE        if the VM is not in a position to switch to secure.

#### Description


    启动保护虚拟机的过程。这涉及与 Ultravisor 协调（使用 ultracall）以在 Ultravisor 中
    为新 SVM 分配资源、将 VM 的页从普通内存传输到安全内存等。当过程完成时，Ultravisor
    发出 H_SVM_INIT_DONE hypercall。

#### Use cases


     #. Ultravisor 使用此 hypercall 告知 Hypervisor 某个 VM 已启动切换到安全模式的过程。


### H_SVM_INIT_DONE


    完成保护 SVM 的过程。

#### Syntax


	uint64_t hypercall(const uint64_t H_SVM_INIT_DONE)

#### Return values


    以下值之一：

 - H_SUCCESS 		on success.
 - H_UNSUPPORTED		if called from the wrong context (e.g.
				from an SVM or before an H_SVM_INIT_START
				hypercall).
 - H_STATE		if the hypervisor could not successfully
                                transition the VM to Secure VM.

#### Description


    完成保护虚拟机的过程。此调用必须在先前的 `H_SVM_INIT_START` hypercall 之后发出。

#### Use cases


    成功保护虚拟机后，Ultravisor 会告知 Hypervisor。Hypervisor 可以使用此调用完成设置
    该虚拟机的内部状态。


### H_SVM_INIT_ABORT


    中止保护 SVM 的过程。

#### Syntax


	uint64_t hypercall(const uint64_t H_SVM_INIT_ABORT)

#### Return values


    以下值之一：

 - H_PARAMETER 		on successfully cleaning up the state,
				Hypervisor will return this value to the
				**guest**, to indicate that the underlying
				UV_ESM ultracall failed.

 - H_STATE		if called after a VM has gone secure (i.e
				H_SVM_INIT_DONE hypercall was successful).

 - H_UNSUPPORTED		if called from a wrong context (e.g. from a
				normal VM).

#### Description


    中止保护虚拟机的过程。此调用必须在先前的 `H_SVM_INIT_START` hypercall 之后、且在
    `H_SVM_INIT_DONE` 调用之前发出。

    进入此 hypercall 时，非易失 GPR 与 FPR 应包含 VM 发出 UV_ESM ultracall 时它们所具有
    的值。此外，`SRR0` 应包含 UV_ESM ultracall 之后那条指令的地址，`SRR1` 应包含用于
    返回到 VM 的 MSR 值。

    此 hypercall 将清理自先前的 `H_SVM_INIT_START` hypercall 以来为该 VM 建立的任何部分
    状态，包括将已换入安全内存的页换出，并发出 `UV_SVM_TERMINATE` ultracall 以终止该 VM。

    清理完部分状态后，控制权返回到 VM（**而非 Ultravisor**），地址为 `SRR0` 所指定，
    MSR 值设置为 `SRR1` 中的值。

#### Use cases


    如果在成功调用 `H_SVM_INIT_START` 之后，Ultravisor 在保护虚拟机时遇到错误，无论是
    由于资源不足还是由于 VM 的安全信息无法被验证，Ultravisor 都会告知 Hypervisor。
    Hypervisor 应使用此调用清理该虚拟机的任何内部状态并返回到 VM。

### H_SVM_PAGE_IN


    将一页内容从普通内存移动到安全内存。

#### Syntax


	uint64_t hypercall(const uint64_t H_SVM_PAGE_IN,
		uint64_t guest_pa,	/** guest-physical-address **/
		uint64_t flags,		/** flags **/
		uint64_t order)		/** page size order **/

#### Return values


    以下值之一：

 - H_SUCCESS	on success.
 - H_PARAMETER	if `guest_pa` is invalid.
 - H_P2		if `flags` is invalid.
 - H_P3		if `order` of page is invalid.

#### Description


    取回属于 VM、位于指定客户机物理地址的页的内容。

    `flags` 中仅有的有效值为：

        - H_PAGE_IN_SHARED 表示将与 Ultravisor 共享该页。

 - H_PAGE_IN_NONSHARED 表示 UV 不再对该页感兴趣。适用于该页为共享页的情况。

    `order` 参数必须对应于配置好的页大小。

#### Use cases


    #. 当普通 VM 变为安全 VM（使用 UV_ESM ultracall）时，Ultravisor 使用此 hypercall
       将 VM 每一页的内容从普通内存移动到安全内存。

    #. Ultravisor 使用此 hypercall 请求 Hypervisor 提供一个可在 SVM 与 Hypervisor 之间
       共享的普通内存页。

    #. Ultravisor 使用此 hypercall 换入（page-in）一个被换出的页。这可在 SVM 触碰一个
       被换出的页时发生。

    #. 如果 SVM 想禁止与 Hypervisor 共享页，它可以告知 Ultravisor 这样做。Ultravisor
       随后将使用此 hypercall 并告知 Hypervisor 它已释放对该普通页的访问。

### H_SVM_PAGE_OUT


    将页的内容移动到普通内存。

#### Syntax


	uint64_t hypercall(const uint64_t H_SVM_PAGE_OUT,
		uint64_t guest_pa,	/** guest-physical-address **/
		uint64_t flags,		/** flags (currently none) **/
		uint64_t order)		/** page size order **/

#### Return values


    以下值之一：

 - H_SUCCESS	on success.
 - H_PARAMETER	if `guest_pa` is invalid.
 - H_P2		if `flags` is invalid.
 - H_P3		if `order` is invalid.

#### Description


    将 `guest_pa` 标识的页的内容移动到普通内存。

    目前 `flags` 未使用，必须设置为 0。`order` 参数必须对应于配置好的页大小。

#### Use cases


    #. 如果 Ultravisor 的安全页不足，它可以使用此 hypercall 将某些安全页的内容移动到
       普通页中。内容将被加密。

# 参考


- `Supporting Protected Computing on IBM Power Architecture <https://developer.ibm.com/articles/l-support-protected-computing/>`_
