
## KVM（基于内核的虚拟机）API 权威文档


## 1. General description


kvm API 围绕可以发送给各类文件描述符的不同种类的 ioctl 构建。最初的一次 open("/dev/kvm") 获取一个指向 kvm 子系统的句柄；该句柄可用于发出系统 ioctl。在此句柄上执行 KVM_CREATE_VM ioctl 将创建一个 VM 文件描述符，可用于发出 VM ioctl。在 VM fd 上执行 KVM_CREATE_VCPU 或 KVM_CREATE_DEVICE ioctl 将创建一个虚拟 cpu 或设备，并返回指向新资源的文件描述符。

换句话说，kvm API 是一组发送给不同种类文件描述符的 ioctl，用于控制虚拟机的各个方面。根据接受它们的文件描述符，ioctl 属于以下类别：

 - System ioctls: These query and set global attributes which affect the
   whole kvm subsystem.  In addition a system ioctl is used to create
   virtual machines.

 - VM ioctls: These query and set attributes that affect an entire virtual
   machine, for example memory layout.  In addition a VM ioctl is used to
   create virtual cpus (vcpus) and devices.

   VM ioctls must be issued from the same process (address space) that was
   used to create the VM.

 - vcpu ioctls: These query and set attributes that control the operation
   of a single virtual cpu.

   vcpu ioctls should be issued from the same thread that was used to create
   the vcpu, except for asynchronous vcpu ioctl that are marked as such in
   the documentation.  Otherwise, the first ioctl after switching threads
   could see a performance impact.

 - device ioctls: These query and set attributes that control the operation
   of a single device.

   device ioctls must be issued from the same process (address space) that
   was used to create the VM.

虽然大多数 ioctl 是特定于某一种文件描述符的，但在某些情况下，同一个 ioctl 可以属于多个类别。

KVM API 是随着时间推移成长起来的。因此，KVM 定义了许多形如 `KVM_CAP_*` 的常量，每个对应由一个或多个 ioctl 提供的一组功能。这些"能力"（capabilities）的可用性可以通过 KVM_CHECK_EXTENSION <KVM_CHECK_EXTENSION> 来检查。对于希望获得其功能的 VM 或 VCPU，某些能力还需要被启用（参见 cap_enable 和 cap_enable_vm）。


## 2. Restrictions


一般而言，文件描述符可以通过 fork() 和 unix 域套接字的 SCM_RIGHTS 设施在进程间迁移。这类技巧明确不受 kvm 支持。虽然它们不会对宿主机造成损害，但其实际行为不被 API 保证。有关 KVM 支持的 ioctl 使用模型详情，请参阅"General description"。

需要注意的是，尽管 VM ioctl 只能从创建该 VM 的进程发出，但 VM 的生命周期与其文件描述符相关联，而非与其创建者（进程）相关联。换句话说，VM 及其资源（**包括关联的地址空间**）在对该 VM 文件描述符的最后一个引用被释放之前不会被释放。例如，如果在 ioctl(KVM_CREATE_VM) 之后执行 fork()，则该 VM 在父（原始）进程及其子进程都释放了它们对 VM 文件描述符的引用之前不会被释放。

由于 VM 的资源在其文件描述符的最后一个引用被释放之前不会被释放，因此强烈不建议在未仔细考虑的情况下通过 fork()、dup() 等方式创建对 VM 的额外引用，这可能会产生不希望的副作用，例如 VM 关闭时，由 VM 进程及其代表分配的内存可能不会被释放/记账。


## 3. Extensions


自 Linux 2.6.22 起，KVM ABI 已经稳定：不允许任何不向后兼容的变更。然而，存在一个扩展设施，允许查询和使用对 API 的向后兼容扩展。

扩展机制并非基于 Linux 版本号。相反，kvm 定义扩展标识符，并提供一个设施来查询某个特定的扩展标识符是否可用。如果可用，则有一组 ioctl 可供应用程序使用。


## 4. API description


本节描述可用于控制 kvm 客户机的 ioctl。对于每个 ioctl，除描述外还提供以下信息：

  Capability:
      which KVM extension provides this ioctl.  Can be 'basic',
      which means that is will be provided by any kernel that supports
      API version 12 (see KVM_GET_API_VERSION <KVM_GET_API_VERSION>),
      or a KVM_CAP_xyz constant that can be checked with
      KVM_CHECK_EXTENSION <KVM_CHECK_EXTENSION>.

  Architectures:
      which instruction set architectures provide this ioctl.
      x86 includes both i386 and x86_64.

  Type:
      system, vm, or vcpu.

  Parameters:
      what parameters are accepted by the ioctl.

  Returns:
      the return value.  General error numbers (EBADF, ENOMEM, EINVAL)
      are not detailed, but errors with specific meanings are.



### 4.1 KVM_GET_API_VERSION



:Capability: basic
:Architectures: all
:Type: system ioctl
:Parameters: none
:Returns: the constant KVM_API_VERSION (=12)

这会将 API 版本标识为稳定的 kvm API。预计该数字不会变化。不过，Linux 2.6.20 和 2.6.21 报告的是更早的版本；这些版本没有文档且不受支持。如果 KVM_GET_API_VERSION 返回的值不是 12，应用程序应当拒绝运行。如果此项检查通过，所有被描述为 'basic' 的 ioctl 都将可用。


### 4.2 KVM_CREATE_VM



:Capability: basic
:Architectures: all
:Type: system ioctl
:Parameters: machine type identifier (KVM_VM_*)
:Returns: a VM fd that can be used to control the new virtual machine.

新 VM 没有虚拟 cpu，也没有内存。你可能希望将 0 用作机器类型。

##### X86:



受支持的 X86 VM 类型可以通过 KVM_CAP_VM_TYPES 查询。

##### S390:



为了在 S390 上创建用户控制的虚拟机，请检查 KVM_CAP_S390_UCONTROL，并以特权用户（CAP_SYS_ADMIN）使用标志 KVM_VM_S390_UCONTROL。

##### MIPS:



要在 MIPS 上使用硬件辅助虚拟化（VZ ASE），而非默认的陷入并模拟（trap & emulate）实现（该实现会改变虚拟内存布局以适配用户模式），请检查 KVM_CAP_MIPS_VZ 并使用标志 KVM_VM_MIPS_VZ。

##### ARM64:



在 arm64 上，VM 的物理地址大小（IPA 大小限制）默认限制为 40 位。如果宿主机支持 KVM_CAP_ARM_VM_IPA_SIZE 扩展，该限制可配置。受支持时，使用 KVM_VM_TYPE_ARM_IPA_SIZE(IPA_Bits) 在机器类型标识符中设置大小，其中 IPA_Bits 是 VM 使用的任何物理地址的最大宽度。IPA_Bits 被编码在机器类型标识符的 bits[7-0] 中。

```
    vm_fd = ioctl(dev_fd, KVM_CREATE_VM, KVM_VM_TYPE_ARM_IPA_SIZE(48));
```

所请求的大小（IPA_Bits）必须满足：

 ==   =========================================================
  0   Implies default size, 40bits (for backward compatibility)
  N   Implies N bits, where N is a positive integer such that,
      32 <= N <= Host_IPA_Limit
 ==   =========================================================

Host_IPA_Limit 是宿主机上 IPA_Bits 可能的最大值，取决于 CPU 能力和内核配置。该限制可以通过运行时调用 KVM_CHECK_EXTENSION ioctl() 的 KVM_CAP_ARM_VM_IPA_SIZE 获取。

如果所请求的 IPA 大小（无论是隐式还是显式）在宿主机上不受支持，VM 的创建将失败。

请注意，配置 IPA 大小不会影响客户机 CPU 在 ID_AA64MMFR0_EL1[PARange] 中暴露的能力。它只影响由 stage2 级别（客户机物理地址到宿主机物理地址转换）所转换的地址大小。


### 4.3 KVM_GET_MSR_INDEX_LIST, KVM_GET_MSR_FEATURE_INDEX_LIST



:Capability: basic, KVM_CAP_GET_MSR_FEATURES for KVM_GET_MSR_FEATURE_INDEX_LIST
:Architectures: x86
:Type: system ioctl
:Parameters: struct kvm_msr_list (in/out)
:Returns: 0 on success; -1 on error

错误：

  ======     ============================================================
  EFAULT     msr 索引列表无法被读取或写入
  E2BIG      msr 索引列表太大，无法放入用户指定的数组中
  ======     ============================================================

```
  struct kvm_msr_list {
	__u32 nmsrs; /* number of msrs in entries */
	__u32 indices[0];
  };
```

用户用 nmsrs 填入 indices 数组的大小，作为回报 kvm 调整 nmsrs 以反映实际的 msr 数量，并用其编号填充 indices 数组。

KVM_GET_MSR_INDEX_LIST 返回受支持的客户机 msr。该列表随 kvm 版本和宿主机处理器而变，除此之外不会改变。

注意：如果 kvm 表明支持 MCE（KVM_CAP_MCE），则 MCE bank MSR 不会在 MSR 列表中返回，因为不同的 vcpu 可能拥有不同数量的 bank，这通过 KVM_X86_SETUP_MCE ioctl 设置。

KVM_GET_MSR_FEATURE_INDEX_LIST 返回可以传递给 KVM_GET_MSRS 系统 ioctl 的 MSR 列表。这让用户空间能够探测通过 MSR 暴露的宿主机能力及处理器特性（例如 VMX 能力）。该列表也随 kvm 版本和宿主机处理器而变，除此之外不会改变。



### 4.4 KVM_CHECK_EXTENSION



:Capability: basic, KVM_CAP_CHECK_EXTENSION_VM for vm ioctl
:Architectures: all
:Type: system ioctl, vm ioctl
:Parameters: extension identifier (KVM_CAP_*)
:Returns: 0 if unsupported; 1 (or some other positive integer) if supported

该 API 允许应用程序查询核心 kvm API 的扩展。用户空间传递一个扩展标识符（整数）并接收一个描述扩展可用性的整数。通常 0 表示否，1 表示是，但某些扩展可能在整数返回值中报告额外信息。

根据其初始化方式，不同的 VM 可能具有不同的能力。因此建议使用 vm ioctl 来查询能力（在 vm fd 上通过 KVM_CAP_CHECK_EXTENSION_VM 可用）。

### 4.5 KVM_GET_VCPU_MMAP_SIZE



:Capability: basic
:Architectures: all
:Type: system ioctl
:Parameters: none
:Returns: size of vcpu mmap area, in bytes

KVM_RUN ioctl（参见前文）通过共享内存区域与用户空间通信。该 ioctl 返回该区域的大小。详情请参阅 KVM_RUN 文档。

除了 KVM_RUN 通信区域的大小外，VCPU 文件描述符的其他区域也可以被 mmap，包括：

- if KVM_CAP_COALESCED_MMIO is available, a page at
  KVM_COALESCED_MMIO_PAGE_OFFSET * PAGE_SIZE; for historical reasons,
  this page is included in the result of KVM_GET_VCPU_MMAP_SIZE.
  KVM_CAP_COALESCED_MMIO is not documented yet.

- if KVM_CAP_DIRTY_LOG_RING is available, a number of pages at
  KVM_DIRTY_LOG_PAGE_OFFSET * PAGE_SIZE.  For more information on
  KVM_CAP_DIRTY_LOG_RING, see KVM_CAP_DIRTY_LOG_RING.


### 4.7 KVM_CREATE_VCPU



:Capability: basic
:Architectures: all
:Type: vm ioctl
:Parameters: vcpu id (apic id on x86)
:Returns: vcpu fd on success, -1 on error

该 API 向虚拟机添加一个 vcpu。添加数量不得超过 max_vcpus。vcpu id 是范围 [0, max_vcpu_id) 内的整数。

建议的 max_vcpus 值可以通过运行时调用 KVM_CHECK_EXTENSION ioctl() 的 KVM_CAP_NR_VCPUS 获取。max_vcpus 可能的最大值可以通过运行时调用 KVM_CHECK_EXTENSION ioctl() 的 KVM_CAP_MAX_VCPUS 获取。

如果 KVM_CAP_NR_VCPUS 不存在，你应当假定 max_vcpus 最多为 4 个 cpu。如果 KVM_CAP_MAX_VCPUS 不存在，你应当假定 max_vcpus 与 KVM_CAP_NR_VCPUS 返回的值相同。

max_vcpu_id 可能的最大值可以通过运行时调用 KVM_CHECK_EXTENSION ioctl() 的 KVM_CAP_MAX_VCPU_ID 获取。

如果 KVM_CAP_MAX_VCPU_ID 不存在，你应当假定 max_vcpu_id 与 KVM_CAP_MAX_VCPUS 返回的值相同。

在使用 book3s_hv 模式的 powerpc 上，vcpu 被映射到由一个或多个虚拟 CPU 核组成的虚拟线程中。（这是因为硬件要求一个 CPU 核中的所有硬件线程都处于同一分区中。）KVM_CAP_PPC_SMT 能力表示每个虚拟核（vcore）的 vcpu 数量。vcore id 由 vcpu id 除以每个 vcore 的 vcpu 数量得到。给定 vcore 中的 vcpu 始终彼此位于同一物理核中（尽管可能随时间切换到不同的物理核）。用户空间可以通过分配 vcpu id 来控制客户机的线程（SMT）模式。例如，如果用户空间希望客户机 vcpu 是单线程的，它应当使所有 vcpu id 都是每个 vcore 的 vcpu 数量的倍数。

对于使用 S390 用户控制虚拟机创建的虚拟 cpu，得到的 vcpu fd 可以在页偏移 KVM_S390_SIE_PAGE_OFFSET 处进行内存映射，以获取虚拟 cpu 硬件控制块的内存映射。


### 4.8 KVM_GET_DIRTY_LOG



:Capability: basic
:Architectures: all
:Type: vm ioctl
:Parameters: struct kvm_dirty_log (in/out)
:Returns: 0 on success, -1 on error

```
  /* for KVM_GET_DIRTY_LOG */
  struct kvm_dirty_log {
	__u32 slot;
	__u32 padding;
	union {
		void __user *dirty_bitmap; /* one bit per page */
		__u64 padding;
	};
  };
```

给定一个内存槽，返回一个位图，包含自上次调用该 ioctl 以来被弄脏的所有页。第 0 位对应内存槽中的第一页。请确保整个结构体被清零，以避免填充问题。

如果 KVM_CAP_MULTI_ADDRESS_SPACE 可用，slot 字段的 16-31 位指定了你想要返回脏位图的地址空间。有关 slot 字段用法的详情，请参阅 KVM_SET_USER_MEMORY_REGION。

脏位图中的位会在 ioctl 返回之前被清零，除非启用了 KVM_CAP_MANUAL_DIRTY_LOG_PROTECT2。更多信息请参阅该能力的描述。

注意，Xen shared_info 页（如果已配置）应始终被视为脏页。KVM 不会显式地将其标记为脏。


### 4.10 KVM_RUN



:Capability: basic
:Architectures: all
:Type: vcpu ioctl
:Parameters: none
:Returns: 0 on success, -1 on error

错误：

  =======    ==============================================================
  EINTR      an unmasked signal is pending
  ENOEXEC    the vcpu hasn't been initialized or the guest tried to execute
             instructions from device memory (arm64)
  ENOSYS     data abort outside memslots with no syndrome info and
             KVM_CAP_ARM_NISV_TO_USER not enabled (arm64)
  EPERM      SVE feature set but not finalized (arm64)
  =======    ==============================================================

该 ioctl 用于运行一个客户机虚拟 cpu。虽然没有显式参数，但存在一个隐式参数块，可以通过以 KVM_GET_VCPU_MMAP_SIZE 给定大小对 vcpu fd 在偏移 0 处进行 mmap() 获得。该参数块被格式化为 'struct kvm_run'（见下文）。


### 4.11 KVM_GET_REGS



:Capability: basic
:Architectures: all except arm64
:Type: vcpu ioctl
:Parameters: struct kvm_regs (out)
:Returns: 0 on success, -1 on error

从 vcpu 读取通用寄存器。

```
  /* x86 */
  struct kvm_regs {
	/* out (KVM_GET_REGS) / in (KVM_SET_REGS) */
	__u64 rax, rbx, rcx, rdx;
	__u64 rsi, rdi, rsp, rbp;
	__u64 r8,  r9,  r10, r11;
	__u64 r12, r13, r14, r15;
	__u64 rip, rflags;
  };

  /* mips */
  struct kvm_regs {
	/* out (KVM_GET_REGS) / in (KVM_SET_REGS) */
	__u64 gpr[32];
	__u64 hi;
	__u64 lo;
	__u64 pc;
  };

  /* LoongArch */
  struct kvm_regs {
	/* out (KVM_GET_REGS) / in (KVM_SET_REGS) */
	unsigned long gpr[32];
	unsigned long pc;
  };


```
### 4.12 KVM_SET_REGS



:Capability: basic
:Architectures: all except arm64
:Type: vcpu ioctl
:Parameters: struct kvm_regs (in)
:Returns: 0 on success, -1 on error

将通用寄存器写入 vcpu。

See KVM_GET_REGS for the data structure.


### 4.13 KVM_GET_SREGS



:Capability: basic
:Architectures: x86, ppc
:Type: vcpu ioctl
:Parameters: struct kvm_sregs (out)
:Returns: 0 on success, -1 on error

从 vcpu 读取特殊寄存器。

```
  /* x86 */
  struct kvm_sregs {
	struct kvm_segment cs, ds, es, fs, gs, ss;
	struct kvm_segment tr, ldt;
	struct kvm_dtable gdt, idt;
	__u64 cr0, cr2, cr3, cr4, cr8;
	__u64 efer;
	__u64 apic_base;
	__u64 interrupt_bitmap[(KVM_NR_INTERRUPTS + 63) / 64];
  };

  /* ppc -- see arch/powerpc/include/uapi/asm/kvm.h */

```

interrupt_bitmap 是挂起外部中断的位图。最多只能设置一位。该中断已被 APIC 确认，但尚未被注入到 cpu 核中。


### 4.14 KVM_SET_SREGS



:Capability: basic
:Architectures: x86, ppc
:Type: vcpu ioctl
:Parameters: struct kvm_sregs (in)
:Returns: 0 on success, -1 on error

将特殊寄存器写入 vcpu。See KVM_GET_SREGS for the data structures.


### 4.15 KVM_TRANSLATE



:Capability: basic
:Architectures: x86
:Type: vcpu ioctl
:Parameters: struct kvm_translation (in/out)
:Returns: 0 on success, -1 on error

根据 vcpu 当前地址转换模式翻译一个虚拟地址。

```
  struct kvm_translation {
	/* in */
	__u64 linear_address;

	/* out */
	__u64 physical_address;
	__u8  valid;
	__u8  writeable;
	__u8  usermode;
	__u8  pad[5];
  };


```
### 4.16 KVM_INTERRUPT



:Capability: basic
:Architectures: x86, ppc, mips, riscv, loongarch
:Type: vcpu ioctl
:Parameters: struct kvm_interrupt (in)
:Returns: 0 on success, negative on failure.

将待注入的硬件中断向量排入队列。

```
  /* for KVM_INTERRUPT */
  struct kvm_interrupt {
	/* in */
	__u32 irq;
  };
```

##### X86:



:Returns:

	========= ===================================
	  0       on success,
	 -EEXIST  if an interrupt is already enqueued
	 -EINVAL  the irq number is invalid
	 -ENXIO   if the PIC is in the kernel
	 -EFAULT  if the pointer is invalid
	========= ===================================

注意，'irq' 是中断向量，而非中断引脚或线路。如果未使用内核态 PIC，该 ioctl 很有用。

##### PPC:



将待注入的外部中断排入队列。该 ioctl 被重载为 3 个不同的 irq 值：

a) KVM_INTERRUPT_SET

   一旦客户机准备好接收中断，就将边沿型外部中断注入到客户机中。注入后，中断即完成。

b) KVM_INTERRUPT_UNSET

   这会取消任何挂起的中断。

   Only available with KVM_CAP_PPC_UNSET_IRQ.

c) KVM_INTERRUPT_SET_LEVEL

   这将电平型外部中断注入到客户机上下文中。中断保持挂起，直到触发带有 KVM_INTERRUPT_UNSET 的特定 ioctl。

   Only available with KVM_CAP_PPC_IRQ_LEVEL.

注意，除上述声明的值之外的任何 'irq' 值都是无效的，并会导致意外行为。

This is an asynchronous vcpu ioctl and can be invoked from any thread.

##### MIPS:



将待注入虚拟 CPU 的外部中断排入队列。负的 interrupt 号会将中断出队。

This is an asynchronous vcpu ioctl and can be invoked from any thread.

##### RISC-V:



将待注入虚拟 CPU 的外部中断排入队列。该 ioctl 被重载为 2 个不同的 irq 值：

a) KVM_INTERRUPT_SET

   这为虚拟 CPU 设置外部中断，它将在就绪后接收。

b) KVM_INTERRUPT_UNSET

   这会清除虚拟 CPU 的挂起外部中断。

This is an asynchronous vcpu ioctl and can be invoked from any thread.

##### LOONGARCH:



将待注入虚拟 CPU 的外部中断排入队列。负的 interrupt 号会将中断出队。

This is an asynchronous vcpu ioctl and can be invoked from any thread.


### 4.18 KVM_GET_MSRS



:Capability: basic (vcpu), KVM_CAP_GET_MSR_FEATURES (system)
:Architectures: x86
:Type: system ioctl, vcpu ioctl
:Parameters: struct kvm_msrs (in/out)
:Returns: number of msrs successfully returned;
          -1 on error

当用作系统 ioctl 时：读取 VM 可用的基于 MSR 的特性的值。这类似于 KVM_GET_SUPPORTED_CPUID，但它返回 MSR 索引和值。基于 MSR 的特性列表可以通过系统 ioctl 中的 KVM_GET_MSR_FEATURE_INDEX_LIST 获取。

当用作 vcpu ioctl 时：从 vcpu 读取模型特定寄存器。受支持的 msr 索引可以通过系统 ioctl 中的 KVM_GET_MSR_INDEX_LIST 获取。

```
  struct kvm_msrs {
	__u32 nmsrs; /* number of msrs in entries */
	__u32 pad;

	struct kvm_msr_entry entries[0];
  };

  struct kvm_msr_entry {
	__u32 index;
	__u32 reserved;
	__u64 data;
  };
```

应用程序代码应设置 'nmsrs' 成员（表示 entries 数组的大小）以及每个数组条目的 'index' 成员。kvm 将填充 'data' 成员。


### 4.19 KVM_SET_MSRS



:Capability: basic
:Architectures: x86
:Type: vcpu ioctl
:Parameters: struct kvm_msrs (in)
:Returns: number of msrs successfully set (see below), -1 on error

将模型特定寄存器写入 vcpu。数据结构请参阅 KVM_GET_MSRS。

应用程序代码应设置 'nmsrs' 成员（表示 entries 数组的大小），以及每个数组条目的 'index' 和 'data' 成员。

它会尝试逐一设置数组 entries[] 中的 MSR。如果设置某个 MSR 失败（例如，由于设置了保留位、KVM 不支持/不模拟该 MSR 等），它会停止处理 MSR 列表，并返回已成功设置的 MSR 数量。


### 4.20 KVM_SET_CPUID



:Capability: basic
:Architectures: x86
:Type: vcpu ioctl
:Parameters: struct kvm_cpuid (in)
:Returns: 0 on success, -1 on error

定义 vcpu 对 cpuid 指令的响应。如果可用，应用程序应使用 KVM_SET_CPUID2 ioctl。

注意事项（Caveat emptor）：
  - 如果该 IOCTL 失败，KVM 不保证先前的有效 CPUID 配置（如果存在）未被破坏。用户空间可以通过 KVM_GET_CPUID2 获取结果 CPUID 配置的副本。
  - 在 KVM_RUN 之后使用 KVM_SET_CPUID{,2}，即在运行客户机之后更改客户机 vCPU 模型，可能导致客户机不稳定。
  - 使用异构的 CPUID 配置（APIC ID、拓扑等除外）可能导致客户机不稳定。

```
  struct kvm_cpuid_entry {
	__u32 function;
	__u32 eax;
	__u32 ebx;
	__u32 ecx;
	__u32 edx;
	__u32 padding;
  };

  /* for KVM_SET_CPUID */
  struct kvm_cpuid {
	__u32 nent;
	__u32 padding;
	struct kvm_cpuid_entry entries[0];
  };


```
### 4.21 KVM_SET_SIGNAL_MASK



:Capability: basic
:Architectures: all
:Type: vcpu ioctl
:Parameters: struct kvm_signal_mask (in)
:Returns: 0 on success, -1 on error

定义在执行 KVM_RUN 期间被阻塞的信号。该信号掩码临时覆盖线程的信号掩码。收到的任何未阻塞信号（SIGKILL 和 SIGSTOP 除外，它们保留传统行为）将导致 KVM_RUN 以 -EINTR 返回。

注意，只有当该信号未被原始信号掩码阻塞时才会被投递。

```
  /* for KVM_SET_SIGNAL_MASK */
  struct kvm_signal_mask {
	__u32 len;
	__u8  sigset[0];
  };


```
### 4.22 KVM_GET_FPU



:Capability: basic
:Architectures: x86, loongarch
:Type: vcpu ioctl
:Parameters: struct kvm_fpu (out)
:Returns: 0 on success, -1 on error

从 vcpu 读取浮点状态。

```
  /* x86: for KVM_GET_FPU and KVM_SET_FPU */
  struct kvm_fpu {
	__u8  fpr[8][16];
	__u16 fcw;
	__u16 fsw;
	__u8  ftwx;  /* in fxsave format */
	__u8  pad1;
	__u16 last_opcode;
	__u64 last_ip;
	__u64 last_dp;
	__u8  xmm[16][16];
	__u32 mxcsr;
	__u32 pad2;
  };

  /* LoongArch: for KVM_GET_FPU and KVM_SET_FPU */
  struct kvm_fpu {
	__u32 fcsr;
	__u64 fcc;
	struct kvm_fpureg {
		__u64 val64[4];
	}fpr[32];
  };


```
### 4.23 KVM_SET_FPU



:Capability: basic
:Architectures: x86, loongarch
:Type: vcpu ioctl
:Parameters: struct kvm_fpu (in)
:Returns: 0 on success, -1 on error

将浮点状态写入 vcpu。

```
  /* x86: for KVM_GET_FPU and KVM_SET_FPU */
  struct kvm_fpu {
	__u8  fpr[8][16];
	__u16 fcw;
	__u16 fsw;
	__u8  ftwx;  /* in fxsave format */
	__u8  pad1;
	__u16 last_opcode;
	__u64 last_ip;
	__u64 last_dp;
	__u8  xmm[16][16];
	__u32 mxcsr;
	__u32 pad2;
  };

  /* LoongArch: for KVM_GET_FPU and KVM_SET_FPU */
  struct kvm_fpu {
	__u32 fcsr;
	__u64 fcc;
	struct kvm_fpureg {
		__u64 val64[4];
	}fpr[32];
  };


```
### 4.24 KVM_CREATE_IRQCHIP



:Capability: KVM_CAP_IRQCHIP, KVM_CAP_S390_IRQCHIP (s390)
:Architectures: x86, arm64, s390
:Type: vm ioctl
:Parameters: none
:Returns: 0 on success, -1 on error

在内核中创建一个中断控制器模型。在 x86 上，创建一个虚拟 ioapic、一个虚拟 PIC（两个嵌套的 PIC），并配置未来的 vcpu 拥有本地 APIC。GSI 0-15 的 IRQ 路由同时指向 PIC 和 IOAPIC；GSI 16-23 仅指向 IOAPIC。在 arm64 上，创建一个 GICv2。任何其他 GIC 版本都需要使用 KVM_CREATE_DEVICE，它也支持创建 GICv2。对于 GICv2，推荐使用 KVM_CREATE_DEVICE 而非 KVM_CREATE_IRQCHIP。在 s390 上，创建一个虚拟的 irq 路由表。

注意，在 s390 上，在使用 KVM_CREATE_IRQCHIP 之前需要先启用 KVM_CAP_S390_IRQCHIP vm 能力。


### 4.25 KVM_IRQ_LINE



:Capability: KVM_CAP_IRQCHIP
:Architectures: x86, arm64
:Type: vm ioctl
:Parameters: struct kvm_irq_level
:Returns: 0 on success, -1 on error

设置内核中断控制器模型中 GSI 输入的电平。在某些架构上，要求已预先使用 KVM_CREATE_IRQCHIP 创建了中断控制器模型。注意，边沿触发的中断要求电平先置为 1 再置回 0。

在真实硬件上，中断引脚可以是低电平有效或高电平有效。这对于 struct kvm_irq_level 的 level 字段没有影响：1 始终表示有效（asserted），0 表示无效（deasserted）。

x86 允许操作系统为电平触发中断编程中断极性（低电平有效/高电平有效），KVM 过去也会考虑极性。然而，由于在低电平有效中断处理中的代码腐化（bitrot），上述约定现在在 x86 上也有效。这由 KVM_CAP_X86_IOAPIC_POLARITY_IGNORED 发出信号。用户空间不应将中断以低电平有效的方式呈现给客户机，除非存在该能力（或者当然，除非它没有使用内核态 irqchip）。

arm64 可以在 CPU 级别或在内核态 irqchip（GIC）处发出中断信号，并且对于内核态 irqchip，可以告知 GIC 使用为特定 cpu 指定的 PPI。irq 字段的解释如下：

```
  bits:  |  31 ... 28  | 27 ... 24 | 23  ... 16 | 15 ... 0 |
  field: | vcpu2_index | irq_type  | vcpu_index |  irq_id  |
```

irq_type 字段具有以下取值：

- KVM_ARM_IRQ_TYPE_CPU:
	       out-of-kernel GIC: irq_id 0 is IRQ, irq_id 1 is FIQ
- KVM_ARM_IRQ_TYPE_SPI:
	       in-kernel GICv2/GICv3: SPI, irq_id between 32 and 1019 (incl.)
               (the vcpu_index field is ignored)
	       in-kernel GICv5: SPI, irq_id between 0 and 65535 (incl.)
- KVM_ARM_IRQ_TYPE_PPI:
	       in-kernel GICv2/GICv3: PPI, irq_id between 16 and 31 (incl.)
	       in-kernel GICv5: PPI, irq_id between 0 and 127 (incl.)

（因此 irq_id 字段恰好对应于 ARM GIC 规范中的 IRQ ID）

在这两种情况下，level 都用于置位/清除该线路。

当支持 KVM_CAP_ARM_IRQ_LINE_LAYOUT_2 时，目标 vcpu 被标识为 (256 * vcpu2_index + vcpu_index)。否则，vcpu2_index 必须为零。

注意，在 arm64 上，KVM_CAP_IRQCHIP 能力仅决定内核态 irqchip 的中断注入。KVM_IRQ_LINE 始终可用于用户空间中断控制器。

```
  struct kvm_irq_level {
	union {
		__u32 irq;     /* GSI */
		__s32 status;  /* not used for KVM_IRQ_LEVEL */
	};
	__u32 level;           /* 0 or 1 */
  };

```
### 4.26 KVM_GET_IRQCHIP



:Capability: KVM_CAP_IRQCHIP
:Architectures: x86
:Type: vm ioctl
:Parameters: struct kvm_irqchip (in/out)
:Returns: 0 on success, -1 on error

将使用 KVM_CREATE_IRQCHIP 创建的内核中断控制器的状态读入调用者提供的缓冲区。

```
  struct kvm_irqchip {
	__u32 chip_id;  /* 0 = PIC1, 1 = PIC2, 2 = IOAPIC */
	__u32 pad;
        union {
		char dummy[512];  /* reserving space */
		struct kvm_pic_state pic;
		struct kvm_ioapic_state ioapic;
	} chip;
  };

```
### 4.27 KVM_SET_IRQCHIP



:Capability: KVM_CAP_IRQCHIP
:Architectures: x86
:Type: vm ioctl
:Parameters: struct kvm_irqchip (in)
:Returns: 0 on success, -1 on error

从调用者提供的缓冲区设置使用 KVM_CREATE_IRQCHIP 创建的内核中断控制器的状态。

```
  struct kvm_irqchip {
	__u32 chip_id;  /* 0 = PIC1, 1 = PIC2, 2 = IOAPIC */
	__u32 pad;
        union {
		char dummy[512];  /* reserving space */
		struct kvm_pic_state pic;
		struct kvm_ioapic_state ioapic;
	} chip;
  };

```
### 4.28 KVM_XEN_HVM_CONFIG



:Capability: KVM_CAP_XEN_HVM
:Architectures: x86
:Type: vm ioctl
:Parameters: struct kvm_xen_hvm_config (in)
:Returns: 0 on success, -1 on error

设置 Xen HVM 客户机用于初始化其超级调用页的 MSR，并提供用户空间中超级调用 blob 的起始地址和大小。当客户机写入该 MSR 时，kvm 会将一个 blob 页（32 位或 64 位，取决于 vcpu 模式）复制到客户机内存中。

MSR 索引必须位于 [0x40000000, 0x4fffffff] 范围内，即必须位于非官方为虚拟机监控器保留的范围内。最小值和最大值通过 KVM_XEN_MSR_MIN_INDEX 和 KVM_XEN_MSR_MAX_INDEX 枚举。

```
  struct kvm_xen_hvm_config {
	__u32 flags;
	__u32 msr;
	__u64 blob_addr_32;
	__u64 blob_addr_64;
	__u8 blob_size_32;
	__u8 blob_size_64;
	__u8 pad2[30];
  };
```

如果 KVM_CAP_XEN_HVM 检查返回了某些标志，则可以将它们设置在该 ioctl 的 flags 字段中：

KVM_XEN_HVM_CONFIG_INTERCEPT_HCALL 标志请求 KVM 自动生成超级调用页的内容；超级调用将被拦截并通过 KVM_EXIT_XEN 传递给用户空间。在这种情况下，所有 blob 大小和地址字段必须为零。

KVM_XEN_HVM_CONFIG_EVTCHN_SEND 标志向 KVM 表明，用户空间将始终使用 KVM_XEN_HVM_EVTCHN_SEND ioctl 来投递事件通道中断，而不是直接操作客户机的 shared_info 结构。反过来，这可能允许 KVM 启用诸如拦截 SCHEDOP_poll 超级调用以加速客户机的 PV 自旋锁操作等特性。即使被广告了该能力，用户空间仍可使用该 ioctl 来投递事件，即使用户空间没有发送它将始终这样做的指示。

目前，struct kvm_xen_hvm_config 中没有其他有效标志。

### 4.29 KVM_GET_CLOCK



:Capability: KVM_CAP_ADJUST_CLOCK
:Architectures: x86
:Type: vm ioctl
:Parameters: struct kvm_clock_data (out)
:Returns: 0 on success, -1 on error

获取当前客户机所看到的 kvmclock 的当前时间戳。结合 KVM_SET_CLOCK，它用于在迁移等场景中确保单调性。

当将 KVM_CAP_ADJUST_CLOCK 传递给 KVM_CHECK_EXTENSION 时，它返回 KVM 可在 struct kvm_clock_data 的 flag 成员中返回的一组位。

定义了以下标志：

KVM_CLOCK_TSC_STABLE
  如果置位，返回的值是调用 KVM_GET_CLOCK 那一刻所有 VCPU 所看到的精确 kvmclock 值。
  如果清零，返回的值只是 CLOCK_MONOTONIC 加上一个常量偏移；该偏移可以通过 KVM_SET_CLOCK 修改。KVM 会尝试让所有 VCPU 跟随此时钟，但由于宿主 TSC 不稳定，每个 VCPU 读取的精确值可能不同。

KVM_CLOCK_REALTIME
  如果置位，kvm_clock_data 结构中的 `realtime` 字段会被填充为调用 KVM_GET_CLOCK 那一刻宿主机实时时钟源的值。如果清零，则 `realtime` 字段不包含值。

KVM_CLOCK_HOST_TSC
  如果置位，kvm_clock_data 结构中的 `host_tsc` 字段会被填充为调用 KVM_GET_CLOCK 那一刻宿主机时间戳计数器（TSC）的值。如果清零，则 `host_tsc` 字段不包含值。

```
  struct kvm_clock_data {
	__u64 clock;  /* kvmclock current value */
	__u32 flags;
	__u32 pad0;
	__u64 realtime;
	__u64 host_tsc;
	__u32 pad[4];
  };

```
### 4.30 KVM_SET_CLOCK



:Capability: KVM_CAP_ADJUST_CLOCK
:Architectures: x86
:Type: vm ioctl
:Parameters: struct kvm_clock_data (in)
:Returns: 0 on success, -1 on error

将 kvmclock 的当前时间戳设置为参数中指定的值。结合 KVM_GET_CLOCK，它用于在迁移等场景中确保单调性。

可以传递以下标志：

KVM_CLOCK_REALTIME
  如果置位，KVM 会将 `realtime` 字段的值与调用 KVM_SET_CLOCK 那一刻宿主机实时时钟源的值进行比较。经过时间的差值会被加到最终提供给客户机的 kvmclock 值中。

`KVM_GET_CLOCK` 返回的其他标志会被接受但被忽略。

```
  struct kvm_clock_data {
	__u64 clock;  /* kvmclock current value */
	__u32 flags;
	__u32 pad0;
	__u64 realtime;
	__u64 host_tsc;
	__u32 pad[4];
  };

```
### 4.31 KVM_GET_VCPU_EVENTS



:Capability: KVM_CAP_VCPU_EVENTS
:Extended by: KVM_CAP_INTR_SHADOW
:Architectures: x86, arm64
:Type: vcpu ioctl
:Parameters: struct kvm_vcpu_events (out)
:Returns: 0 on success, -1 on error

##### X86:



获取当前挂起的异常、中断和 NMI 以及 vcpu 的相关状态。

```
  struct kvm_vcpu_events {
	struct {
		__u8 injected;
		__u8 nr;
		__u8 has_error_code;
		__u8 pending;
		__u32 error_code;
	} exception;
	struct {
		__u8 injected;
		__u8 nr;
		__u8 soft;
		__u8 shadow;
	} interrupt;
	struct {
		__u8 injected;
		__u8 pending;
		__u8 masked;
		__u8 pad;
	} nmi;
	__u32 sipi_vector;
	__u32 flags;
	struct {
		__u8 smm;
		__u8 pending;
		__u8 smm_inside_nmi;
		__u8 latched_init;
	} smi;
	__u8 reserved[27];
	__u8 exception_has_payload;
	__u64 exception_payload;
  };
```

在 flags 字段中定义了以下位：

- KVM_VCPUEVENT_VALID_SHADOW 可被置位以表明 interrupt.shadow 包含有效状态。

- KVM_VCPUEVENT_VALID_SMM 可被置位以表明 smi 包含有效状态。

- KVM_VCPUEVENT_VALID_PAYLOAD 可被置位以表明 exception_has_payload、exception_payload 和 exception.pending 字段包含有效状态。只要启用了 KVM_CAP_EXCEPTION_PAYLOAD，该位就会被置位。

- KVM_VCPUEVENT_VALID_TRIPLE_FAULT 可被置位以表明 triple_fault_pending 字段包含有效状态。只要启用了 KVM_CAP_X86_TRIPLE_FAULT_EVENT，该位就会被置位。

##### ARM64:



如果客户机以某种方式访问由宿主内核模拟的设备，而真实设备会因此生成物理 SError，KVM 可能会为该 VCPU 使一个虚拟 SError 挂起。该系统错误中断保持挂起，直到客户机通过解除 PSTATE.A 屏蔽来接受该异常。

运行 VCPU 可能导致它接受挂起的 SError，或进行导致 SError 挂起的访问。事件的描述仅在 VPCU 未运行时有效。

该 API 提供了一种读写为客户机不可见的挂起"event"状态的方法。要保存、恢复或迁移 VCPU，可以使用此 GET/SET API 读取然后写入表示该状态的结构体，以及与其它客户机可见的寄存器一起。无法"取消"一个已挂起的 SError。

在用户空间模拟的设备也可能希望生成 SError。为此，事件结构体可以由用户空间填充。应首先读取当前状态，以确保没有现有的 SError 挂起。如果存在现有的 SError 挂起，则应遵循架构的"Multiple SError interrupts"规则。（DDI0587.a "ARM Reliability, Availability, and Serviceability (RAS) Specification" 的 2.5.3 节）。

SError 异常始终有一个 ESR 值。某些 CPU 能够指定虚拟 SError 的 ESR 值应该是什么。这些系统会广告 KVM_CAP_ARM_INJECT_SERROR_ESR。在这种情况下，读取时 exception.has_esr 始终具有非零值，而使 SError 挂起的代理应指定 exception.serror_esr 低 24 位中的 ISS 字段。如果系统支持 KVM_CAP_ARM_INJECT_SERROR_ESR，但用户空间将事件设置为 exception.has_esr 为零，KVM 会选择一个 ESR。

在不支持该能力的系统上指定 exception.has_esr 将返回 -EINVAL。设置 exception.serror_esr 低 24 位之外的任何内容将返回 -EINVAL。

无法读回挂起的外部中止（通过 KVM_SET_VCPU_EVENTS 或其他方式注入），因为此类异常总是直接投递到虚拟 CPU。

在尚未初始化的 vCPU 上调用此 ioctl 将返回 -ENOEXEC。

```
  struct kvm_vcpu_events {
	struct {
		__u8 serror_pending;
		__u8 serror_has_esr;
		__u8 ext_dabt_pending;
		/* Align it to 8 bytes */
		__u8 pad[5];
		__u64 serror_esr;
	} exception;
	__u32 reserved[12];
  };
```
### 4.32 KVM_SET_VCPU_EVENTS



:Capability: KVM_CAP_VCPU_EVENTS
:Extended by: KVM_CAP_INTR_SHADOW
:Architectures: x86, arm64
:Type: vcpu ioctl
:Parameters: struct kvm_vcpu_events (in)
:Returns: 0 on success, -1 on error

##### X86:



设置挂起的异常、中断、NMI 以及 vcpu 的相关状态。

See KVM_GET_VCPU_EVENTS for the data structure.

可能被运行的 VCPU 异步修改的字段可以从更新中排除。这些字段是 nmi.pending、sipi_vector、smi.smm、smi.pending。保持 flags 字段中相应的位被清零，以抑制覆盖当前内核态状态。这些位是：

===============================  ==================================
KVM_VCPUEVENT_VALID_NMI_PENDING  transfer nmi.pending to the kernel
KVM_VCPUEVENT_VALID_SIPI_VECTOR  transfer sipi_vector
KVM_VCPUEVENT_VALID_SMM          transfer the smi sub-struct.
===============================  ==================================

如果 KVM_CAP_INTR_SHADOW 可用，则可以在 flags 字段中设置 KVM_VCPUEVENT_VALID_SHADOW，以表明 interrupt.shadow 包含有效状态并应被写入 VCPU。

只有在 KVM_CAP_X86_SMM 可用时才能设置 KVM_VCPUEVENT_VALID_SMM。

如果启用了 KVM_CAP_EXCEPTION_PAYLOAD，则可以在 flags 字段中设置 KVM_VCPUEVENT_VALID_PAYLOAD，以表明 exception_has_payload、exception_payload 和 exception.pending 字段包含有效状态并应被写入 VCPU。

如果启用了 KVM_CAP_X86_TRIPLE_FAULT_EVENT，则可以在 flags 字段中设置 KVM_VCPUEVENT_VALID_TRIPLE_FAULT，以表明 triple_fault 字段包含有效状态并应被写入 VCPU。

##### ARM64:



用户空间可能需要向客户机注入多种类型的事件。

设置此 VCPU 挂起的 SError 异常状态。无法"取消"一个已挂起的 SError。

如果客户机对 I/O 内存进行了用户空间无法处理的访问，例如由于缺少指令综合征（syndrome）解码信息，或者因为在被访问的 IPA 处没有映射设备，那么用户空间可以请内核使用来自 VCPU 退出故障的地址注入一个外部中止。在不是 KVM_EXIT_MMIO、KVM_EXIT_ARM_NISV 或 KVM_EXIT_ARM_LDST64B 的退出之后设置 ext_dabt_pending 是一种编程错误。此特性仅在系统支持 KVM_CAP_ARM_INJECT_EXT_DABT 时可用。这是一个辅助设施，为不同用户空间实现在如何向客户机报告上述情况的访问方面提供一致性。尽管如此，用户空间仍然可以通过使用 KVM_SET_ONE_REG API 操作各个寄存器来模拟所有 Arm 异常。

See KVM_GET_VCPU_EVENTS for the data structure.

在尚未初始化的 vCPU 上调用此 ioctl 将返回 -ENOEXEC。

### 4.33 KVM_GET_DEBUGREGS



:Capability: KVM_CAP_DEBUGREGS
:Architectures: x86
:Type: vcpu ioctl
:Parameters: struct kvm_debugregs (out)
:Returns: 0 on success, -1 on error

从 vcpu 读取调试寄存器。

```
  struct kvm_debugregs {
	__u64 db[4];
	__u64 dr6;
	__u64 dr7;
	__u64 flags;
	__u64 reserved[9];
  };


```
### 4.34 KVM_SET_DEBUGREGS



:Capability: KVM_CAP_DEBUGREGS
:Architectures: x86
:Type: vcpu ioctl
:Parameters: struct kvm_debugregs (in)
:Returns: 0 on success, -1 on error

将调试寄存器写入 vcpu。

See KVM_GET_DEBUGREGS for the data structure. The flags field is unused yet and must be cleared on entry.### 4.35 KVM_SET_USER_MEMORY_REGION


:Capability: KVM_CAP_USER_MEMORY
:Architectures: all
:Type: vm ioctl
:Parameters: struct kvm_userspace_memory_region (in)
:Returns: 0 on success, -1 on error

```

  struct kvm_userspace_memory_region {
	__u32 slot;
	__u32 flags;
	__u64 guest_phys_addr;
	__u64 memory_size; /* bytes */
	__u64 userspace_addr; /* start of the userspace allocated memory */
  };

  /* for kvm_userspace_memory_region::flags */
  #define KVM_MEM_LOG_DIRTY_PAGES	(1UL << 0)
  #define KVM_MEM_READONLY	(1UL << 1)

```
该 ioctl 允许用户创建、修改或删除一个客户机物理内存槽。"slot" 的 0-15 位指定槽 id，该值应小于每个
VM 所支持的最大用户内存槽数量。最大允许的槽数量可通过 KVM_CAP_NR_MEMSLOTS 查询。
槽在客户机物理地址空间中不得重叠。

如果 KVM_CAP_MULTI_ADDRESS_SPACE 可用，"slot" 的 16-31 位指定被修改的地址空间。它们必须小于
KVM_CHECK_EXTENSION 针对 KVM_CAP_MULTI_ADDRESS_SPACE 能力返回的值。不同地址空间中的槽彼此无关；
关于槽重叠的限制仅适用于各自的地址空间内部。

删除槽的方法是令 memory_size 为零。当修改一个已存在的槽时，它可以在客户机物理内存空间中移动，
或其 flags 可以被修改，但大小不可被调整。

该区域的内存从 userspace_addr 字段所指向的地址处开始获取，该地址必须指向整个内存槽大小范围内
用户可寻址的内存。任何对象都可以作为这块内存的后备，包括匿名内存、普通文件以及 hugetlbfs。内存区域
后备的变化会自动反映到客户机中。例如，影响该区域的 mmap() 会立刻变得对客户机可见。另一个例子是
madvise(MADV_DROP)。

在支持某种地址标记（address tagging）形式架构上，userspace_addr 必须是未标记的（untagged）地址。

建议 guest_phys_addr 与 userspace_addr 的低 21 位保持一致。这样可以让客户机中的大页由宿主机中的
大页作为后备。

flags 字段支持两个标志：KVM_MEM_LOG_DIRTY_PAGES 与 KVM_MEM_READONLY。前者可被设置以指示 KVM 跟踪
槽内内存的写入情况。如何使用它可参见 KVM_GET_DIRTY_LOG ioctl。若 KVM_CAP_READONLY_MEM 能力允许，
后者可被设置以使新槽变为只读。在这种情况下，对该内存的写入会被作为 KVM_EXIT_MMIO 退出上报给用户空间。

对于 TDX 客户机，删除/移动内存区域会丢失客户机内存内容。不支持只读区域。仅支持 as-id 0。

注意：在 arm64 上，当槽具有 KVM_MEM_READONLY 标志时，由页表遍历器（page-table walker）产生的写入
（例如用于更新 Access 和 Dirty 标志）永远不会导致 KVM_EXIT_MMIO 退出。这是因为 KVM 无法提供页表遍历器
将要写入的数据，从而无法模拟该访问。取而代之，会向客户机注入一个异常（如果页表更新的起因是加载或
存储，则为数据异常 data abort；如果是指令获取，则为指令异常 instruction abort）。

##### S390:


如果 VM 设置了 KVM_VM_S390_UCONTROL 标志，则返回 -EINVAL 或 -EEXIST。
如果是在受保护的 VM 上调用，则返回 -EINVAL。

### 4.36 KVM_SET_TSS_ADDR


:Capability: KVM_CAP_SET_TSS_ADDR
:Architectures: x86
:Type: vm ioctl
:Parameters: unsigned long tss_address (in)
:Returns: 0 on success, -1 on error

该 ioctl 定义客户机物理地址空间中一个三页区域的物理地址。该区域必须位于客户机物理地址空间的前
4GB 之内，且不能与任何内存槽或任何 mmio 地址冲突。如果客户机访问该内存区域，可能会发生故障。

在基于 Intel 的主机上，该 ioctl 是必需的。在 Intel 硬件上需要它，是因为虚拟化实现中的一个
怪异之处（参见尚未面世的 internals 文档）。



### 4.37 KVM_ENABLE_CAP


:Capability: KVM_CAP_ENABLE_CAP
:Architectures: mips, ppc, s390, x86, loongarch
:Type: vcpu ioctl
:Parameters: struct kvm_enable_cap (in)
:Returns: 0 on success; -1 on error

:Capability: KVM_CAP_ENABLE_CAP_VM
:Architectures: all
:Type: vm ioctl
:Parameters: struct kvm_enable_cap (in)
:Returns: 0 on success; -1 on error


   并非所有扩展都默认启用。通过此 ioctl，应用程序可以启用一个扩展，使其对客户机可用。

在不支持此 ioctl 的系统上，它总是失败。在支持它的系统上，它只对那些支持被启用的扩展有效。

要检查某个能力是否可以被启用，应当使用 KVM_CHECK_EXTENSION ioctl。

```

  struct kvm_enable_cap {
       /* in */
       __u32 cap;

```
要被启用的能力。

```

       __u32 flags;

```
一个指示未来增强的位域。目前必须为 0。

```

       __u64 args[4];

```
启用某个特性所需的参数。如果一个特性需要初始值才能正常工作，这里就是放置它们的地方。

```

       __u8  pad[64];
  };

```
vcpu ioctl 应用于 vcpu 特定的能力，vm ioctl 应用于 VM 范围的能力。

### 4.38 KVM_GET_MP_STATE


:Capability: KVM_CAP_MP_STATE
:Architectures: x86, s390, arm64, riscv, loongarch
:Type: vcpu ioctl
:Parameters: struct kvm_mp_state (out)
:Returns: 0 on success; -1 on error

```

  struct kvm_mp_state {
	__u32 mp_state;
  };

```
返回 vcpu 当前的"多处理器状态"（尽管在单处理器客户机上也有效）。

可能的值如下：

   ==========================    ===============================================
   KVM_MP_STATE_RUNNABLE         the vcpu is currently running
                                 [x86,arm64,riscv,loongarch]
   KVM_MP_STATE_UNINITIALIZED    the vcpu is an application processor (AP)
                                 which has not yet received an INIT signal [x86]
   KVM_MP_STATE_INIT_RECEIVED    the vcpu has received an INIT signal, and is
                                 now ready for a SIPI [x86]
   KVM_MP_STATE_HALTED           the vcpu has executed a HLT instruction and
                                 is waiting for an interrupt [x86]
   KVM_MP_STATE_SIPI_RECEIVED    the vcpu has just received a SIPI (vector
                                 accessible via KVM_GET_VCPU_EVENTS) [x86]
   KVM_MP_STATE_STOPPED          the vcpu is stopped [s390,arm64,riscv]
   KVM_MP_STATE_CHECK_STOP       the vcpu is in a special error state [s390]
   KVM_MP_STATE_OPERATING        the vcpu is operating (running or halted)
                                 [s390]
   KVM_MP_STATE_LOAD             the vcpu is in a special load/startup state
                                 [s390]
   KVM_MP_STATE_SUSPENDED        the vcpu is in a suspend state and is waiting
                                 for a wakeup event [arm64]
   ==========================    ===============================================

在 x86 上，此 ioctl 仅在 KVM_CREATE_IRQCHIP 之后才有用。如果没有内核态 irqchip，多处理器状态
必须在这些架构上由用户空间维护。

##### For arm64:


如果 vCPU 处于 KVM_MP_STATE_SUSPENDED 状态，KVM 会模拟 WFI 指令的架构化执行。

如果识别到一个唤醒事件，KVM 会退出到用户空间，产生一个 KVM_SYSTEM_EVENT 退出，其中事件类型为
KVM_SYSTEM_EVENT_WAKEUP。如果用户空间希望响应此唤醒，它必须将 vCPU 的 MP 状态设置为
KVM_MP_STATE_RUNNABLE。如果不这样做，KVM 会在后续对 KVM_RUN 的调用中继续等待唤醒事件。


     如果用户空间打算将 vCPU 保持在 SUSPENDED 状态，强烈建议用户空间采取行动抑制唤醒事件
     （例如屏蔽某个中断）。否则，后续对 KVM_RUN 的调用会立即以 KVM_SYSTEM_EVENT_WAKEUP 事件退出，
     并无意中浪费 CPU 周期。

     此外，如果用户空间采取行动抑制了唤醒事件，强烈建议它在 vCPU 再次变为 RUNNABLE 时将其
     恢复到原始状态。例如，如果用户空间屏蔽了一个挂起的中断来抑制唤醒，那么在将控制权交还给
     客户机之前，应解除该中断的屏蔽。

##### For riscv:


唯一有效的状态是 KVM_MP_STATE_STOPPED 与 KVM_MP_STATE_RUNNABLE，它们反映 vcpu 是否被暂停。

在 LoongArch 上，仅使用 KVM_MP_STATE_RUNNABLE 状态来反映 vcpu 是否可运行。

### 4.39 KVM_SET_MP_STATE


:Capability: KVM_CAP_MP_STATE
:Architectures: x86, s390, arm64, riscv, loongarch
:Type: vcpu ioctl
:Parameters: struct kvm_mp_state (in)
:Returns: 0 on success; -1 on error

设置 vcpu 当前的"多处理器状态"；参数说明参见 KVM_GET_MP_STATE。

在 x86 上，此 ioctl 仅在 KVM_CREATE_IRQCHIP 之后才有用。如果没有内核态 irqchip，多处理器状态
必须在这些架构上由用户空间维护。

##### For arm64/riscv:


唯一有效的状态是 KVM_MP_STATE_STOPPED 与 KVM_MP_STATE_RUNNABLE，它们反映 vcpu 是否应被暂停。

在 LoongArch 上，仅使用 KVM_MP_STATE_RUNNABLE 状态来反映 vcpu 是否可运行。

### 4.40 KVM_SET_IDENTITY_MAP_ADDR


:Capability: KVM_CAP_SET_IDENTITY_MAP_ADDR
:Architectures: x86
:Type: vm ioctl
:Parameters: unsigned long identity (in)
:Returns: 0 on success, -1 on error

该 ioctl 定义客户机物理地址空间中一个单页区域的物理地址。该区域必须位于客户机物理地址空间的
前 4GB 之内，且不能与任何内存槽或任何 mmio 地址冲突。如果客户机访问该内存区域，可能会发生故障。

将地址设置为 0 会导致该地址被重置为默认值（0xfffbc000）。

在基于 Intel 的主机上，该 ioctl 是必需的。在 Intel 硬件上需要它，是因为虚拟化实现中的一个
怪异之处（参见尚未面世的 internals 文档）。

如果有任何 VCPU 已经被创建，则会失败。

### 4.41 KVM_SET_BOOT_CPU_ID


:Capability: KVM_CAP_SET_BOOT_CPU_ID
:Architectures: x86
:Type: vm ioctl
:Parameters: unsigned long vcpu_id
:Returns: 0 on success, -1 on error

定义哪个 vcpu 是引导处理器（Bootstrap Processor，BSP）。取值与 KVM_CREATE_VCPU 中的 vcpu id 相同。
如果未调用此 ioctl，则默认是 vcpu 0。此 ioctl 必须在 vcpu 创建之前调用，否则会返回 EBUSY 错误。


### 4.42 KVM_GET_XSAVE


:Capability: KVM_CAP_XSAVE
:Architectures: x86
:Type: vcpu ioctl
:Parameters: struct kvm_xsave (out)
:Returns: 0 on success, -1 on error


```

  struct kvm_xsave {
	__u32 region[1024];
	__u32 extra[0];
  };

```
该 ioctl 会将当前 vcpu 的 xsave 结构体复制到用户空间。


### 4.43 KVM_SET_XSAVE


:Capability: KVM_CAP_XSAVE and KVM_CAP_XSAVE2
:Architectures: x86
:Type: vcpu ioctl
:Parameters: struct kvm_xsave (in)
:Returns: 0 on success, -1 on error

```


  struct kvm_xsave {
	__u32 region[1024];
	__u32 extra[0];
  };

```
该 ioctl 会将用户空间的 xsave 结构体复制到内核。它复制的字节数等于 KVM_CHECK_EXTENSION(KVM_CAP_XSAVE2)
在 vm 文件描述符上调用时返回的值。KVM_CHECK_EXTENSION(KVM_CAP_XSAVE2) 返回的大小值总是至少为 4096。
目前，只有当某个动态特性已通过 `arch_prctl()` 启用时它才会大于 4096，但这在未来可能会改变。

struct kvm_xsave 中各状态保存区域的偏移量遵循宿主机上 CPUID 叶子 0xD 的内容。


### 4.44 KVM_GET_XCRS


:Capability: KVM_CAP_XCRS
:Architectures: x86
:Type: vcpu ioctl
:Parameters: struct kvm_xcrs (out)
:Returns: 0 on success, -1 on error

```

  struct kvm_xcr {
	__u32 xcr;
	__u32 reserved;
	__u64 value;
  };

  struct kvm_xcrs {
	__u32 nr_xcrs;
	__u32 flags;
	struct kvm_xcr xcrs[KVM_MAX_XCRS];
	__u64 padding[16];
  };

```
该 ioctl 会将当前 vcpu 的 xcrs 复制到用户空间。


### 4.45 KVM_SET_XCRS


:Capability: KVM_CAP_XCRS
:Architectures: x86
:Type: vcpu ioctl
:Parameters: struct kvm_xcrs (in)
:Returns: 0 on success, -1 on error

```

  struct kvm_xcr {
	__u32 xcr;
	__u32 reserved;
	__u64 value;
  };

  struct kvm_xcrs {
	__u32 nr_xcrs;
	__u32 flags;
	struct kvm_xcr xcrs[KVM_MAX_XCRS];
	__u64 padding[16];
  };

```
该 ioctl 会将 vcpu 的 xcr 设置为用户空间指定的值。


### 4.46 KVM_GET_SUPPORTED_CPUID


:Capability: KVM_CAP_EXT_CPUID
:Architectures: x86
:Type: system ioctl
:Parameters: struct kvm_cpuid2 (in/out)
:Returns: 0 on success, -1 on error

```

  struct kvm_cpuid2 {
	__u32 nent;
	__u32 padding;
	struct kvm_cpuid_entry2 entries[0];
  };

  #define KVM_CPUID_FLAG_SIGNIFCANT_INDEX		BIT(0)
  #define KVM_CPUID_FLAG_STATEFUL_FUNC		BIT(1) /* deprecated */
  #define KVM_CPUID_FLAG_STATE_READ_NEXT		BIT(2) /* deprecated */

  struct kvm_cpuid_entry2 {
	__u32 function;
	__u32 index;
	__u32 flags;
	__u32 eax;
	__u32 ebx;
	__u32 ecx;
	__u32 edx;
	__u32 padding[3];
  };

```
该 ioctl 返回在默认配置下由硬件和 kvm 都支持的 x86 cpuid 特性。用户空间可以使用该 ioctl 返回的
信息来构造与硬件、内核以及用户空间能力一致的 cpuid 信息（用于 KVM_SET_CPUID2），并与用户需求一致
（例如，用户可能希望约束 cpuid 以模拟较旧的硬件，或为了在集群中保持一致的特性）。

动态启用的特性位需要在调用此 ioctl 之前通过 `arch_prctl()` 请求。未被请求的特性位不会包含在结果中。

注意，某些能力（如 KVM_CAP_X86_DISABLE_EXITS）可能会暴露 kvm 在默认配置下不支持的 cpuid 特性
（例如 MONITOR）。如果用户空间启用了此类能力，它负责适当地修改此 ioctl 的结果。

用户空间调用 KVM_GET_SUPPORTED_CPUID 时，需传入一个 kvm_cpuid2 结构体，其 'nent' 字段指示可变长
数组 'entries' 中的条目数量。如果条目数量太少而无法描述 cpu 能力，会返回错误（E2BIG）。如果数量
过多，'nent' 字段会被调整并返回一个错误（ENOMEM）。如果数量恰好合适，'nent' 字段会被调整为
'entries' 数组中有效条目的数量，并随后被填充。

返回的条目是 cpuid 指令返回的主机 cpuid，其中未知或不支持的特性被屏蔽。某些特性（例如 x2apic）可能
不在主机 cpu 中，但如果 kvm 能够高效地模拟它们，则会被 kvm 暴露出来。每个条目中的字段定义如下：

  function:
         用于获取该条目的 eax 值

  index:
         用于获取该条目的 ecx 值（针对受 ecx 影响的条目）

  flags:
     以下零个或多个的按位或：

        KVM_CPUID_FLAG_SIGNIFCANT_INDEX:
           表示 index 字段有效

   eax, ebx, ecx, edx:
         该 function/index 组合下 cpuid 指令返回的值

x2APIC（CPUID 叶子 1，ecx[21]）和 TSC deadline 定时器（CPUID 叶子 1，ecx[24]）可能作为 true 返回，
但它们依赖于 KVM_CREATE_IRQCHIP 的内核态
```

  ioctl(KVM_CHECK_EXTENSION, KVM_CAP_TSC_DEADLINE_TIMER)

```
来实现；如果它返回 true 且你使用了 KVM_CREATE_IRQCHIP，或者你在用户空间模拟了该特性，那么你就可以
为 KVM_SET_CPUID2 启用该特性。

在 KVM_SET_CPUID2 中启用 x2APIC 需要 KVM_CREATE_IRQCHIP，因为 KVM 不支持将 x2APIC MSR 访问转发到
用户空间，即 KVM 不支持在用户空间模拟 x2APIC。

### 4.47 KVM_PPC_GET_PVINFO


:Capability: KVM_CAP_PPC_GET_PVINFO
:Architectures: ppc
:Type: vm ioctl
:Parameters: struct kvm_ppc_pvinfo (out)
:Returns: 0 on success, !0 on error

```

  struct kvm_ppc_pvinfo {
	__u32 flags;
	__u32 hcall[4];
	__u8  pad[108];
  };

```
该 ioctl 从 vm 上下文中获取需要借助设备树或其他方式传递给客户机的 PV 特定信息。

hcall 数组定义了构成一次 hypercall 的 4 条指令。

如果以后该结构体添加了任何附加字段，会在 flags 位图中设置对应于该附加信息的一个位。

```

   /* the host supports the ePAPR idle hcall
   #define KVM_PPC_PVINFO_FLAGS_EV_IDLE   (1<<0)

```
### 4.52 KVM_SET_GSI_ROUTING


:Capability: KVM_CAP_IRQ_ROUTING
:Architectures: x86 s390 arm64
:Type: vm ioctl
:Parameters: struct kvm_irq_routing (in)
:Returns: 0 on success, -1 on error

设置 GSI 路由表条目，覆盖任何先前设置的条目。

在 arm64 上，GSI 路由有以下限制：

- GSI 路由不适用于 KVM_IRQ_LINE，而只适用于 KVM_IRQFD。

```

  struct kvm_irq_routing {
	__u32 nr;
	__u32 flags;
	struct kvm_irq_routing_entry entries[0];
  };

```
目前未指定任何标志，相应字段必须设置为零。

```

  struct kvm_irq_routing_entry {
	__u32 gsi;
	__u32 type;
	__u32 flags;
	__u32 pad;
	union {
		struct kvm_irq_routing_irqchip irqchip;
		struct kvm_irq_routing_msi msi;
		struct kvm_irq_routing_s390_adapter adapter;
		struct kvm_irq_routing_hv_sint hv_sint;
		struct kvm_irq_routing_xen_evtchn xen_evtchn;
		__u32 pad[8];
	} u;
  };

  /* gsi routing entry types */
  #define KVM_IRQ_ROUTING_IRQCHIP 1
  #define KVM_IRQ_ROUTING_MSI 2
  #define KVM_IRQ_ROUTING_S390_ADAPTER 3
  #define KVM_IRQ_ROUTING_HV_SINT 4
  #define KVM_IRQ_ROUTING_XEN_EVTCHN 5

```
在 s390 上，向 ucontrol VM 添加 KVM_IRQ_ROUTING_S390_ADAPTER 会以 -EINVAL 错误被拒绝。

flags:

- KVM_MSI_VALID_DEVID：与 KVM_IRQ_ROUTING_MSI 路由条目类型一起使用，表示 devid 字段包含一个
  有效值。每 VM 的 KVM_CAP_MSI_DEVID 能力用于通告需要提供设备 ID 的要求。如果该能力不可用，
  用户空间绝不应设置 KVM_MSI_VALID_DEVID 标志，否则 ioctl 可能会失败。
- 否则为零

```

  struct kvm_irq_routing_irqchip {
	__u32 irqchip;
	__u32 pin;
  };

  struct kvm_irq_routing_msi {
	__u32 address_lo;
	__u32 address_hi;
	__u32 data;
	union {
		__u32 pad;
		__u32 devid;
	};
  };

```
如果设置了 KVM_MSI_VALID_DEVID，则 devid 包含写入 MSI 消息的设备的唯一设备标识符。对于 PCI，
这通常是低 16 位中的 BDF 标识符。

在 x86 上，除非启用了 KVM_CAP_X2APIC_API 能力的 KVM_X2APIC_API_USE_32BIT_IDS 特性，否则 address_hi
会被忽略。如果启用，address_hi 的 31-8 位提供目的 id 的 31-8 位。address_hi 的 7-0 位必须为零。

```

  struct kvm_irq_routing_s390_adapter {
	__u64 ind_addr;
	__u64 summary_addr;
	__u64 ind_offset;
	__u32 summary_offset;
	__u32 adapter_id;
  };

  struct kvm_irq_routing_hv_sint {
	__u32 vcpu;
	__u32 sint;
  };

  struct kvm_irq_routing_xen_evtchn {
	__u32 port;
	__u32 vcpu;
	__u32 priority;
  };


```
当 KVM_CAP_XEN_HVM 在其支持特性指示中包含 KVM_XEN_HVM_CONFIG_EVTCHN_2LEVEL 位时，支持路由到 Xen
事件通道。尽管存在 priority 字段，但目前仅支持值 KVM_XEN_HVM_CONFIG_EVTCHN_2LEVEL，这意味着通过
两级事件通道投递。未来可能会添加 FIFO 事件通道支持。


### 4.55 KVM_SET_TSC_KHZ


:Capability: KVM_CAP_TSC_CONTROL / KVM_CAP_VM_TSC_CONTROL
:Architectures: x86
:Type: vcpu ioctl / vm ioctl
:Parameters: virtual tsc_khz
:Returns: 0 on success, -1 on error

指定虚拟机的 tsc 频率。频率的单位是 KHz。

如果通告了 KVM_CAP_VM_TSC_CONTROL 能力，它也可以作为 vm ioctl 使用，以设置随后创建的 vCPU 的
初始 tsc 频率。注意，vm ioctl 仅允许在创建 vCPU 之前使用。

对于 TSC 受保护的机密计算（CoCo）VM（其 TSC 频率在 VM 范围配置一次并在 VM 生命周期内保持不变），
应使用 vm ioctl 来配置 TSC 频率，vcpu ioctl 不被支持。

此类 CoCo VM 的例子：TDX 客户机。

### 4.56 KVM_GET_TSC_KHZ


:Capability: KVM_CAP_GET_TSC_KHZ / KVM_CAP_VM_TSC_CONTROL
:Architectures: x86
:Type: vcpu ioctl / vm ioctl
:Parameters: none
:Returns: virtual tsc-khz on success, negative value on error

返回客户机的 tsc 频率。返回值的单位是 KHz。如果宿主机具有不稳定的 tsc，该 ioctl 会返回 -EIO
作为错误。


### 4.57 KVM_GET_LAPIC


:Capability: KVM_CAP_IRQCHIP
:Architectures: x86
:Type: vcpu ioctl
:Parameters: struct kvm_lapic_state (out)
:Returns: 0 on success, -1 on error

```

  #define KVM_APIC_REG_SIZE 0x400
  struct kvm_lapic_state {
	char regs[KVM_APIC_REG_SIZE];
  };

```
读取 Local APIC 寄存器并将其复制到输入参数中。数据格式和布局与架构手册中记录的一致。

如果启用了 KVM_CAP_X2APIC_API 的 KVM_X2APIC_API_USE_32BIT_IDS 特性，那么 APIC_ID 寄存器的格式
取决于其 VCPU 的 APIC 模式（由 MSR_IA32_APICBASE 报告）。x2APIC 将 APIC ID 存储在 APIC_ID 寄存器
（字节 32-35）中。xAPIC 仅允许一个 8 位的 APIC ID，存储在 APIC 寄存器的 31-24 位，或等效地存储在
struct kvm_lapic_state 的 regs 字段的字节 35 中。因此 KVM_GET_LAPIC 必须在 MSR_IA32_APICBASE 已
通过 KVM_SET_MSR 设置之后调用。

如果禁用了 KVM_X2APIC_API_USE_32BIT_IDS 特性，struct kvm_lapic_state 始终使用 xAPIC 格式。


### 4.58 KVM_SET_LAPIC


:Capability: KVM_CAP_IRQCHIP
:Architectures: x86
:Type: vcpu ioctl
:Parameters: struct kvm_lapic_state (in)
:Returns: 0 on success, -1 on error

```

  #define KVM_APIC_REG_SIZE 0x400
  struct kvm_lapic_state {
	char regs[KVM_APIC_REG_SIZE];
  };

```
将输入参数复制到 Local APIC 寄存器中。数据格式和布局与架构手册中记录的一致。

APIC ID 寄存器的格式（struct kvm_lapic_state 的 regs 字段的字节 32-35）取决于 KVM_CAP_X2APIC_API
能力的状态。参见 KVM_GET_LAPIC 中的说明。


### 4.59 KVM_IOEVENTFD


:Capability: KVM_CAP_IOEVENTFD
:Architectures: all
:Type: vm ioctl
:Parameters: struct kvm_ioeventfd (in)
:Returns: 0 on success, !0 on error

该 ioctl 将 ioeventfd 附加或分离到客户机内一个合法的 pio/mmio 地址。对注册地址的客户机写入将
触发所提供的事件，而不是导致一次退出。

```

  struct kvm_ioeventfd {
	__u64 datamatch;
	__u64 addr;        /* legal pio/mmio address */
	__u32 len;         /* 0, 1, 2, 4, or 8 bytes    */
	__s32 fd;
	__u32 flags;
	__u8  pad[36];
  };

```
对于 s390 上 virtio-ccw 设备的特殊情况，ioevent 匹配的是一个子通道/virtqueue 元组，而不是地址。

```

  #define KVM_IOEVENTFD_FLAG_DATAMATCH (1 << kvm_ioeventfd_flag_nr_datamatch)
  #define KVM_IOEVENTFD_FLAG_PIO       (1 << kvm_ioeventfd_flag_nr_pio)
  #define KVM_IOEVENTFD_FLAG_DEASSIGN  (1 << kvm_ioeventfd_flag_nr_deassign)
  #define KVM_IOEVENTFD_FLAG_VIRTIO_CCW_NOTIFY \
	(1 << kvm_ioeventfd_flag_nr_virtio_ccw_notify)

```
如果设置了 datamatch 标志，则只有当写入注册地址的值等于 struct kvm_ioeventfd 中的 datamatch 时，
才会触发该事件。

对于 virtio-ccw 设备，addr 包含子通道 id，datamatch 包含 virtqueue 索引。

借助 KVM_CAP_IOEVENTFD_ANY_LENGTH，允许长度为 0 的 ioeventfd，内核将忽略客户机写入的长度，并可能
获得更快的 vmexit。这种加速可能只适用于特定架构，但 ioeventfd 在任何情况下都能工作。

### 4.60 KVM_DIRTY_TLB


:Capability: KVM_CAP_SW_TLB
:Architectures: ppc
:Type: vcpu ioctl
:Parameters: struct kvm_dirty_tlb (in)
:Returns: 0 on success, -1 on error

```

  struct kvm_dirty_tlb {
	__u64 bitmap;
	__u32 num_dirty;
  };

```
每当用户空间更改了共享 TLB 中的一个条目时，必须在关联的 vcpu 上调用 KVM_RUN 之前调用此 ioctl。

"bitmap" 字段是一个数组的用户空间地址。该数组由若干位组成，位数等于由上次成功调用
`KVM_ENABLE_CAP(KVM_CAP_SW_TLB)` 确定的 TLB 条目总数，向上舍入到最接近的 64 的倍数。

每一位对应一个 TLB 条目，顺序与共享 TLB 数组中的顺序相同。

该数组为小端序：位 0 是第一个字节的最低有效位，位 8 是第二个字节的最低有效位，依此类推。这避免了
因字长不同而带来的任何复杂性。

"num_dirty" 字段是给 KVM 的一个性能提示，用于判断它是否应该跳过处理位图而直接使所有内容失效。它
必须设置为位图中被置位的位数。


### 4.62 KVM_CREATE_SPAPR_TCE


:Capability: KVM_CAP_SPAPR_TCE
:Architectures: powerpc
:Type: vm ioctl
:Parameters: struct kvm_create_spapr_tce (in)
:Returns: file descriptor for manipulating the created TCE table

这将创建一个虚拟 TCE（转换控制条目 translation control entry）表，它是 PAPR 风格虚拟 I/O 的
IOMMU。它用于将虚拟 I/O 中使用的逻辑地址转换为客户机物理地址，并为 PAPR 虚拟 I/O 提供分散/聚集
（scatter/gather）能力。

```

  /* for KVM_CAP_SPAPR_TCE */
  struct kvm_create_spapr_tce {
	__u64 liobn;
	__u32 window_size;
  };

```
liobn 字段给出了要为其创建 TCE 表的逻辑 IO 总线号。window_size 字段指定了该 TCE 表将转换的 DMA
窗口大小——该表将为 DMA 窗口的每 4kiB 包含一个 64 位的 TCE 条目。

当客户机对已经使用此 ioctl() 创建了 TCE 表的 liobn 发出 H_PUT_TCE hcall 时，内核将在实模式下处理
它，更新 TCE 表。针对其他 liobn 的 H_PUT_TCE 调用会导致 vm 退出，必须由用户空间处理。

返回值是一个文件描述符，可以传递给 mmap(2) 以将创建的 TCE 表映射到用户空间。这允许用户空间读取
由内核处理的 H_PUT_TCE 调用所写入的条目，也允许用户空间直接更新 TCE 表，这在某些情况下很有用。


### 4.64 KVM_NMI


:Capability: KVM_CAP_USER_NMI
:Architectures: x86
:Type: vcpu ioctl
:Parameters: none
:Returns: 0 on success, -1 on error

在线程的 vcpu 上排队一个 NMI。注意，这仅在未调用 KVM_CREATE_IRQCHIP 时有明确定义，因为这是虚拟
cpu 核心与虚拟 Local APIC 之间的接口。在调用 KVM_CREATE_IRQCHIP 之后，该接口完全在内核中模拟。

要使用它来配合 KVM_CREATE_IRQCHIP 模拟 LINT1 输入，请使用以下算法：

  - 暂停 vcpu
  - 读取 Local APIC 的状态（KVM_GET_LAPIC）
  - 检查更改 LINT1 是否会排队一个 NMI（参见 LINT1 的 LVT 条目）
  - 如果是，发出 KVM_NMI
  - 恢复 vcpu

某些客户机将 LINT1 NMI 输入配置为引发 panic，以协助调试。


### 4.65 KVM_S390_UCAS_MAP


:Capability: KVM_CAP_S390_UCONTROL
:Architectures: s390
:Type: vcpu ioctl
:Parameters: struct kvm_s390_ucas_mapping (in)
:Returns: 0 in case of success

```

	struct kvm_s390_ucas_mapping {
		__u64 user_addr;
		__u64 vcpu_addr;
		__u64 length;
	};

```
该 ioctl 将从 "user_addr" 开始、长度为 "length" 的内存映射到从 "vcpu_addr" 开始的 vcpu 地址空间。
所有参数都需要按 1 兆字节对齐。


### 4.66 KVM_S390_UCAS_UNMAP


:Capability: KVM_CAP_S390_UCONTROL
:Architectures: s390
:Type: vcpu ioctl
:Parameters: struct kvm_s390_ucas_mapping (in)
:Returns: 0 in case of success

```

	struct kvm_s390_ucas_mapping {
		__u64 user_addr;
		__u64 vcpu_addr;
		__u64 length;
	};

```
该 ioctl 取消映射从 "vcpu_addr" 开始、长度为 "length" 的 vcpu 地址空间中的内存。"user_addr" 字段
被忽略。所有参数都需要按 1 兆字节对齐。


### 4.67 KVM_S390_VCPU_FAULT


:Capability: KVM_CAP_S390_UCONTROL
:Architectures: s390
:Type: vcpu ioctl
:Parameters: vcpu absolute address (in)
:Returns: 0 in case of success

该调用会在虚拟 cpu 的地址空间（对于用户控制的虚拟机）或虚拟机的地址空间（对于常规虚拟机）上
创建一个页表条目。这仅对次要缺页（minor fault）有效，因此建议事先通过用户页表访问相关内存页。
这对于处理用户控制虚拟机的有效性拦截（validity intercept）非常有用，可在调用 KVM_RUN ioctl 之前
将虚拟 cpu 的 lowcore 页缺页装入。
### 4.68 KVM_SET_ONE_REG


:Capability: KVM_CAP_ONE_REG
:Architectures: all
:Type: vcpu ioctl
:Parameters: struct kvm_one_reg (in)
:Returns: 0 on success, negative value on failure

错误码：

  ======   ============================================================
  ENOENT   没有该寄存器
  EINVAL   无效的寄存器 ID，或没有该寄存器，或与 s390 上受保护虚拟化
           模式下的 VM 一起使用
  EPERM    (arm64) 在 vcpu 定稿（finalization）之前不允许访问该寄存器
  EBUSY    (riscv) vcpu 至少运行过一次之后不允许更改寄存器值
  ======   ============================================================

（这些错误码仅供参考：不要依赖在特定情况下返回特定的错误码。）

```

  struct kvm_one_reg {
       __u64 id;
       __u64 addr;
 };

```
使用该 ioctl，可以通过传入的 struct kvm_one_reg 将单个 vcpu 寄存器设置为用户空间指定的特定值，
其中 id 指代如下所述的寄存器标识符，addr 是指向相应大小变量的指针。寄存器可以架构无关，
也可以架构相关。每种都有各自的操作范围和各自的常量与宽度。要追踪已实现的寄存器，请参见
以下列表：

  ======= =============================== ============
  Arch              Register              Width (bits)
  ======= =============================== ============
  PPC     KVM_REG_PPC_HIOR                64
  PPC     KVM_REG_PPC_IAC1                64
  PPC     KVM_REG_PPC_IAC2                64
  PPC     KVM_REG_PPC_IAC3                64
  PPC     KVM_REG_PPC_IAC4                64
  PPC     KVM_REG_PPC_DAC1                64
  PPC     KVM_REG_PPC_DAC2                64
  PPC     KVM_REG_PPC_DABR                64
  PPC     KVM_REG_PPC_DSCR                64
  PPC     KVM_REG_PPC_PURR                64
  PPC     KVM_REG_PPC_SPURR               64
  PPC     KVM_REG_PPC_DAR                 64
  PPC     KVM_REG_PPC_DSISR               32
  PPC     KVM_REG_PPC_AMR                 64
  PPC     KVM_REG_PPC_UAMOR               64
  PPC     KVM_REG_PPC_MMCR0               64
  PPC     KVM_REG_PPC_MMCR1               64
  PPC     KVM_REG_PPC_MMCRA               64
  PPC     KVM_REG_PPC_MMCR2               64
  PPC     KVM_REG_PPC_MMCRS               64
  PPC     KVM_REG_PPC_MMCR3               64
  PPC     KVM_REG_PPC_SIAR                64
  PPC     KVM_REG_PPC_SDAR                64
  PPC     KVM_REG_PPC_SIER                64
  PPC     KVM_REG_PPC_SIER2               64
  PPC     KVM_REG_PPC_SIER3               64
  PPC     KVM_REG_PPC_PMC1                32
  PPC     KVM_REG_PPC_PMC2                32
  PPC     KVM_REG_PPC_PMC3                32
  PPC     KVM_REG_PPC_PMC4                32
  PPC     KVM_REG_PPC_PMC5                32
  PPC     KVM_REG_PPC_PMC6                32
  PPC     KVM_REG_PPC_PMC7                32
  PPC     KVM_REG_PPC_PMC8                32
  PPC     KVM_REG_PPC_FPR0                64
  ...
  PPC     KVM_REG_PPC_FPR31               64
  PPC     KVM_REG_PPC_VR0                 128
  ...
  PPC     KVM_REG_PPC_VR31                128
  PPC     KVM_REG_PPC_VSR0                128
  ...
  PPC     KVM_REG_PPC_VSR31               128
  PPC     KVM_REG_PPC_FPSCR               64
  PPC     KVM_REG_PPC_VSCR                32
  PPC     KVM_REG_PPC_VPA_ADDR            64
  PPC     KVM_REG_PPC_VPA_SLB             128
  PPC     KVM_REG_PPC_VPA_DTL             128
  PPC     KVM_REG_PPC_EPCR                32
  PPC     KVM_REG_PPC_EPR                 32
  PPC     KVM_REG_PPC_TCR                 32
  PPC     KVM_REG_PPC_TSR                 32
  PPC     KVM_REG_PPC_OR_TSR              32
  PPC     KVM_REG_PPC_CLEAR_TSR           32
  PPC     KVM_REG_PPC_MAS0                32
  PPC     KVM_REG_PPC_MAS1                32
  PPC     KVM_REG_PPC_MAS2                64
  PPC     KVM_REG_PPC_MAS7_3              64
  PPC     KVM_REG_PPC_MAS4                32
  PPC     KVM_REG_PPC_MAS6                32
  PPC     KVM_REG_PPC_MMUCFG              32
  PPC     KVM_REG_PPC_TLB0CFG             32
  PPC     KVM_REG_PPC_TLB1CFG             32
  PPC     KVM_REG_PPC_TLB2CFG             32
  PPC     KVM_REG_PPC_TLB3CFG             32
  PPC     KVM_REG_PPC_TLB0PS              32
  PPC     KVM_REG_PPC_TLB1PS              32
  PPC     KVM_REG_PPC_TLB2PS              32
  PPC     KVM_REG_PPC_TLB3PS              32
  PPC     KVM_REG_PPC_EPTCFG              32
  PPC     KVM_REG_PPC_ICP_STATE           64
  PPC     KVM_REG_PPC_VP_STATE            128
  PPC     KVM_REG_PPC_TB_OFFSET           64
  PPC     KVM_REG_PPC_SPMC1               32
  PPC     KVM_REG_PPC_SPMC2               32
  PPC     KVM_REG_PPC_IAMR                64
  PPC     KVM_REG_PPC_TFHAR               64
  PPC     KVM_REG_PPC_TFIAR               64
  PPC     KVM_REG_PPC_TEXASR              64
  PPC     KVM_REG_PPC_FSCR                64
  PPC     KVM_REG_PPC_PSPB                32
  PPC     KVM_REG_PPC_EBBHR               64
  PPC     KVM_REG_PPC_EBBRR               64
  PPC     KVM_REG_PPC_BESCR               64
  PPC     KVM_REG_PPC_TAR                 64
  PPC     KVM_REG_PPC_DPDES               64
  PPC     KVM_REG_PPC_DAWR                64
  PPC     KVM_REG_PPC_DAWRX               64
  PPC     KVM_REG_PPC_CIABR               64
  PPC     KVM_REG_PPC_IC                  64
  PPC     KVM_REG_PPC_VTB                 64
  PPC     KVM_REG_PPC_CSIGR               64
  PPC     KVM_REG_PPC_TACR                64
  PPC     KVM_REG_PPC_TCSCR               64
  PPC     KVM_REG_PPC_PID                 64
  PPC     KVM_REG_PPC_ACOP                64
  PPC     KVM_REG_PPC_VRSAVE              32
  PPC     KVM_REG_PPC_LPCR                32
  PPC     KVM_REG_PPC_LPCR_64             64
  PPC     KVM_REG_PPC_PPR                 64
  PPC     KVM_REG_PPC_ARCH_COMPAT         32
  PPC     KVM_REG_PPC_DABRX               32
  PPC     KVM_REG_PPC_WORT                64
  PPC	  KVM_REG_PPC_SPRG9               64
  PPC	  KVM_REG_PPC_DBSR                32
  PPC     KVM_REG_PPC_TIDR                64
  PPC     KVM_REG_PPC_PSSCR               64
  PPC     KVM_REG_PPC_DEC_EXPIRY          64
  PPC     KVM_REG_PPC_PTCR                64
  PPC     KVM_REG_PPC_HASHKEYR            64
  PPC     KVM_REG_PPC_HASHPKEYR           64
  PPC     KVM_REG_PPC_DAWR1               64
  PPC     KVM_REG_PPC_DAWRX1              64
  PPC     KVM_REG_PPC_DEXCR               64
  PPC     KVM_REG_PPC_TM_GPR0             64
  ...
  PPC     KVM_REG_PPC_TM_GPR31            64
  PPC     KVM_REG_PPC_TM_VSR0             128
  ...
  PPC     KVM_REG_PPC_TM_VSR63            128
  PPC     KVM_REG_PPC_TM_CR               64
  PPC     KVM_REG_PPC_TM_LR               64
  PPC     KVM_REG_PPC_TM_CTR              64
  PPC     KVM_REG_PPC_TM_FPSCR            64
  PPC     KVM_REG_PPC_TM_AMR              64
  PPC     KVM_REG_PPC_TM_PPR              64
  PPC     KVM_REG_PPC_TM_VRSAVE           64
  PPC     KVM_REG_PPC_TM_VSCR             32
  PPC     KVM_REG_PPC_TM_DSCR             64
  PPC     KVM_REG_PPC_TM_TAR              64
  PPC     KVM_REG_PPC_TM_XER              64

  MIPS    KVM_REG_MIPS_R0                 64
  ...
  MIPS    KVM_REG_MIPS_R31                64
  MIPS    KVM_REG_MIPS_HI                 64
  MIPS    KVM_REG_MIPS_LO                 64
  MIPS    KVM_REG_MIPS_PC                 64
  MIPS    KVM_REG_MIPS_CP0_INDEX          32
  MIPS    KVM_REG_MIPS_CP0_ENTRYLO0       64
  MIPS    KVM_REG_MIPS_CP0_ENTRYLO1       64
  MIPS    KVM_REG_MIPS_CP0_CONTEXT        64
  MIPS    KVM_REG_MIPS_CP0_CONTEXTCONFIG  32
  MIPS    KVM_REG_MIPS_CP0_USERLOCAL      64
  MIPS    KVM_REG_MIPS_CP0_XCONTEXTCONFIG 64
  MIPS    KVM_REG_MIPS_CP0_PAGEMASK       32
  MIPS    KVM_REG_MIPS_CP0_PAGEGRAIN      32
  MIPS    KVM_REG_MIPS_CP0_SEGCTL0        64
  MIPS    KVM_REG_MIPS_CP0_SEGCTL1        64
  MIPS    KVM_REG_MIPS_CP0_SEGCTL2        64
  MIPS    KVM_REG_MIPS_CP0_PWBASE         64
  MIPS    KVM_REG_MIPS_CP0_PWFIELD        64
  MIPS    KVM_REG_MIPS_CP0_PWSIZE         64
  MIPS    KVM_REG_MIPS_CP0_WIRED          32
  MIPS    KVM_REG_MIPS_CP0_PWCTL          32
  MIPS    KVM_REG_MIPS_CP0_HWRENA         32
  MIPS    KVM_REG_MIPS_CP0_BADVADDR       64
  MIPS    KVM_REG_MIPS_CP0_BADINSTR       32
  MIPS    KVM_REG_MIPS_CP0_BADINSTRP      32
  MIPS    KVM_REG_MIPS_CP0_COUNT          32
  MIPS    KVM_REG_MIPS_CP0_ENTRYHI        64
  MIPS    KVM_REG_MIPS_CP0_COMPARE        32
  MIPS    KVM_REG_MIPS_CP0_STATUS         32
  MIPS    KVM_REG_MIPS_CP0_INTCTL         32
  MIPS    KVM_REG_MIPS_CP0_CAUSE          32
  MIPS    KVM_REG_MIPS_CP0_EPC            64
  MIPS    KVM_REG_MIPS_CP0_PRID           32
  MIPS    KVM_REG_MIPS_CP0_EBASE          64
  MIPS    KVM_REG_MIPS_CP0_CONFIG         32
  MIPS    KVM_REG_MIPS_CP0_CONFIG1        32
  MIPS    KVM_REG_MIPS_CP0_CONFIG2        32
  MIPS    KVM_REG_MIPS_CP0_CONFIG3        32
  MIPS    KVM_REG_MIPS_CP0_CONFIG4        32
  MIPS    KVM_REG_MIPS_CP0_CONFIG5        32
  MIPS    KVM_REG_MIPS_CP0_CONFIG7        32
  MIPS    KVM_REG_MIPS_CP0_XCONTEXT       64
  MIPS    KVM_REG_MIPS_CP0_ERROREPC       64
  MIPS    KVM_REG_MIPS_CP0_KSCRATCH1      64
  MIPS    KVM_REG_MIPS_CP0_KSCRATCH2      64
  MIPS    KVM_REG_MIPS_CP0_KSCRATCH3      64
  MIPS    KVM_REG_MIPS_CP0_KSCRATCH4      64
  MIPS    KVM_REG_MIPS_CP0_KSCRATCH5      64
  MIPS    KVM_REG_MIPS_CP0_KSCRATCH6      64
  MIPS    KVM_REG_MIPS_CP0_MAAR(0..63)    64
  MIPS    KVM_REG_MIPS_COUNT_CTL          64
  MIPS    KVM_REG_MIPS_COUNT_RESUME       64
  MIPS    KVM_REG_MIPS_COUNT_HZ           64
  MIPS    KVM_REG_MIPS_FPR_32(0..31)      32
  MIPS    KVM_REG_MIPS_FPR_64(0..31)      64
  MIPS    KVM_REG_MIPS_VEC_128(0..31)     128
  MIPS    KVM_REG_MIPS_FCR_IR             32
  MIPS    KVM_REG_MIPS_FCR_CSR            32
  MIPS    KVM_REG_MIPS_MSA_IR             32
  MIPS    KVM_REG_MIPS_MSA_CSR            32
  ======= =============================== ============

ARM 寄存器映射使用低 32 位。其中的高 16 位是寄存器组类型，或协处理器编号：

```

  0x4020 0000 0010 <index into the kvm_regs struct:16>

```
```

  0x4020 0000 000F <zero:1> <crn:4> <crm:4> <opc1:4> <opc2:3>

```
```

  0x4030 0000 000F <zero:1> <zero:4> <crm:4> <opc1:4> <zero:3>

```
```

  0x4020 0000 0011 00 <csselr:8>

```
```

  0x4020 0000 0012 1 <regno:12>

```
```

  0x4030 0000 0012 0 <regno:12>

```
```

  0x4030 0000 0014 <regno:16>


```
arm64 寄存器映射使用低 32 位。其中的高 16 位是寄存器组类型，或协处理器编号：

arm64 核心/FP-SIMD 寄存器具有以下 id 位模式。注意，访问大小是可变的，因为 kvm_regs 结构体
包含从 32 到 128 位不等的元素。index 是一个 32 位的
```

  0x60x0 0000 0010 <index into the kvm_regs struct:16>

```
具体来说：

======================= ========= ===== =======================================
    Encoding            Register  Bits  kvm_regs member
======================= ========= ===== =======================================
  0x6030 0000 0010 0000 X0          64  regs.regs[^0^]
  0x6030 0000 0010 0002 X1          64  regs.regs[^1^]
  ...
  0x6030 0000 0010 003c X30         64  regs.regs[^30^]
  0x6030 0000 0010 003e SP          64  regs.sp
  0x6030 0000 0010 0040 PC          64  regs.pc
  0x6030 0000 0010 0042 PSTATE      64  regs.pstate
  0x6030 0000 0010 0044 SP_EL1      64  sp_el1
  0x6030 0000 0010 0046 ELR_EL1     64  elr_el1
  0x6030 0000 0010 0048 SPSR_EL1    64  spsr[KVM_SPSR_EL1] (alias SPSR_SVC)
  0x6030 0000 0010 004a SPSR_ABT    64  spsr[KVM_SPSR_ABT]
  0x6030 0000 0010 004c SPSR_UND    64  spsr[KVM_SPSR_UND]
  0x6030 0000 0010 004e SPSR_IRQ    64  spsr[KVM_SPSR_IRQ]
  0x6030 0000 0010 0050 SPSR_FIQ    64  spsr[KVM_SPSR_FIQ]
  0x6040 0000 0010 0054 V0         128  fp_regs.vregs[^0^]    [^1^]_
  0x6040 0000 0010 0058 V1         128  fp_regs.vregs[^1^]    [^1^]_
  ...
  0x6040 0000 0010 00d0 V31        128  fp_regs.vregs[^31^]   [^1^]_
  0x6020 0000 0010 00d4 FPSR        32  fp_regs.fpsr
  0x6020 0000 0010 00d5 FPCR        32  fp_regs.fpcr
======================= ========= ===== =======================================

       KVM_ARM_VCPU_INIT。

       对于已启用 SVE 的 vcpu（见下文），可以通过相应 SVE Zn 寄存器的位 [127:0]
       访问等价的寄存器内容。

```

  0x6020 0000 0011 00 <csselr:8>

```
```

  0x6030 0000 0013 <op0:2> <op1:3> <crn:4> <crm:4> <op2:3>

```

     有两个系统寄存器 ID 不遵循指定的模式。它们是 KVM_REG_ARM_TIMER_CVAL 和
     KVM_REG_ARM_TIMER_CNT，分别映射到系统寄存器 CNTV_CVAL_EL0 和 CNTVCT_EL0。
     这两个的值被意外地交换了，这意味着 TIMER_CVAL 派生自 CNTVCT_EL0 的寄存器编码，
     而 TIMER_CNT 派生自 CNTV_CVAL_EL0 的寄存器编码。由于这是 API，必须保持现状。

```

  0x6030 0000 0014 <regno:16>

```
```

  0x6080 0000 0015 00 <n:5> <slice:5>   Zn bits[2048*slice + 2047 : 2048*slice]
  0x6050 0000 0015 04 <n:4> <slice:5>   Pn bits[256*slice + 255 : 256*slice]
  0x6050 0000 0015 060 <slice:5>        FFR bits[256*slice + 255 : 256*slice]
  0x6060 0000 0015 ffff                 KVM_REG_ARM64_SVE_VLS pseudo-register

```
当 2048 * slice >= 128 * max_vq 时，访问该寄存器 ID 会失败并返回 ENOENT。max_vq 是 vcpu 支持的
最大向量长度（以 128 位四字为单位）：见下文的 [^2^]_。

这些寄存器只能在启用了 SVE 的 vcpu 上访问。详见 KVM_ARM_VCPU_INIT。

此外，除了 KVM_REG_ARM64_SVE_VLS 之外，在 vcpu 的 SVE 配置通过
KVM_ARM_VCPU_FINALIZE(KVM_ARM_VCPU_SVE) 定稿之前，无法访问这些寄存器。关于此过程的更多信息，
请参见 KVM_ARM_VCPU_INIT 和 KVM_ARM_VCPU_FINALIZE。

KVM_REG_ARM64_SVE_VLS 是一个伪寄存器，允许用户空间发现并配置 vcpu 所支持的向量长度集合。
通过 KVM_GET_ONE_REG 或 KVM_SET_ONE_REG 在用户内存之间传输时，该寄存器的值为
__u64[KVM_ARM64_SVE_VLS_WORDS] 类型，并将向量长度集合编码为
```

  __u64 vector_lengths[KVM_ARM64_SVE_VLS_WORDS];

  if (vq >= SVE_VQ_MIN && vq <= SVE_VQ_MAX &&
      ((vector_lengths[(vq - KVM_ARM64_SVE_VQ_MIN) / 64] >>
		((vq - KVM_ARM64_SVE_VQ_MIN) % 64)) & 1))
	/* Vector length vq * 16 bytes supported */
  else
	/* Vector length vq * 16 bytes not supported */

```
       max_vq。这是该 vcpu 上客户机可用的最大向量长度，并决定了通过此 ioctl 接口可见的
       寄存器切片。

（关于 "vq" 命名法的解释，请参见 Documentation/arch/arm64/sve.rst。）

KVM_REG_ARM64_SVE_VLS 仅在 KVM_ARM_VCPU_INIT 之后可访问。KVM_ARM_VCPU_INIT 将其初始化为
宿主机支持的最佳向量长度集合。

用户空间随后可以根据需要修改它，直到 vcpu 的 SVE 配置通过
KVM_ARM_VCPU_FINALIZE(KVM_ARM_VCPU_SVE) 定稿为止。

除了简单地从宿主机集合中移除所有超过某个值的向量长度之外，对任意选定向量长度集合的支持
依赖于硬件，可能不可用。尝试通过 KVM_SET_ONE_REG 配置无效的向量长度集合会以 EINVAL 失败。

在 vcpu 的 SVE 配置定稿之后，进一步写入该寄存器的尝试会以 EPERM 失败。

```

  0x6030 0000 0016 <regno:16>

```
位图特性固件寄存器暴露了可供用户空间配置的 hypercall 服务。置位的位对应于可供客户机访问的
服务。默认情况下，KVM 在 VM 初始化期间设置所有受支持的位。用户空间可以通过 KVM_GET_ONE_REG
发现可用的服务，并通过 KVM_SET_ONE_REG 写回它希望客户机看到的、对应于相应特性的位图。

注意：一旦 VM 的任何 vCPU 至少运行过一次，这些寄存器就变为不可变的。在这种情况下，
KVM_SET_ONE_REG 会向用户空间返回 -EBUSY。

（更多细节请参见 Documentation/virt/kvm/arm/hypercalls.rst。）


MIPS 寄存器映射使用低 32 位。其中的高 16 位是寄存器组类型：

```

  0x7030 0000 0000 <reg:16>

```
MIPS CP0 寄存器（见上文 KVM_REG_MIPS_CP0_*）具有以下 id 位
```

  0x7020 0000 0001 00 <reg:5> <sel:3>   (32-bit)
  0x7030 0000 0001 00 <reg:5> <sel:3>   (64-bit)

```
注意：KVM_REG_MIPS_CP0_ENTRYLO0 和 KVM_REG_MIPS_CP0_ENTRYLO1 是 EntryLo 寄存器的 MIPS64 版本，
无论宿主机硬件、宿主机内核、客户机的字长如何，也无论客户机中是否存在 XPA，即 RI 和 XI 位
（如果存在）分别位于位 63 和位 62，PFNX 字段从位 30 开始。

MIPS MAAR（见上文 KVM_REG_MIPS_CP0_MAAR(*)）具有以下 id 位
```

  0x7030 0000 0001 01 <reg:8>

```
```

  0x7030 0000 0002 <reg:16>

```
MIPS FPU 寄存器（见上文 KVM_REG_MIPS_FPR_{32,64}()）根据所访问寄存器的大小具有不同的 id 位模式。
它们始终依据当前客户机 FPU 模式（Status.FR 和 Config5.FRE）进行访问，即客户机所见的方式；
如果客户机 FPU 模式发生改变，它们会变得不可预测。MIPS SIMD 架构（MSA）向量寄存器
（见上文 KVM_REG_MIPS_VEC_128()）具有类似的模式，因为它们
```

  0x7020 0000 0003 00 <0:3> <reg:5> (32-bit FPU registers)
  0x7030 0000 0003 00 <0:3> <reg:5> (64-bit FPU registers)
  0x7040 0000 0003 00 <0:3> <reg:5> (128-bit MSA vector registers)

```
MIPS FPU 控制寄存器（见上文 KVM_REG_MIPS_FCR_{IR,CSR}）具有
```

  0x7020 0000 0003 01 <0:3> <reg:5>

```
MIPS MSA 控制寄存器（见上文 KVM_REG_MIPS_MSA_{IR,CSR}）具有
```

  0x7020 0000 0003 02 <0:3> <reg:5>

```
RISC-V 寄存器映射使用低 32 位。其中的高 8 位是寄存器组类型。

RISC-V 配置寄存器用于配置客户机 VCPU，它具有
```

  0x8020 0000 01 <index into the kvm_riscv_config struct:24> (32bit Host)
  0x8030 0000 01 <index into the kvm_riscv_config struct:24> (64bit Host)

```
以下是 RISC-V 配置寄存器：

======================= ========= =============================================
    Encoding            Register  Description
======================= ========= =============================================
  0x80x0 0000 0100 0000 isa       ISA feature bitmap of Guest VCPU
======================= ========= =============================================

isa 配置寄存器可以随时读取，但只能在客户机 VCPU 运行之前写入。默认情况下，它具有与底层宿主机
匹配的 ISA 特性位。

RISC-V 核心寄存器表示客户机 VCPU 的一般执行状态
```

  0x8020 0000 02 <index into the kvm_riscv_core struct:24> (32bit Host)
  0x8030 0000 02 <index into the kvm_riscv_core struct:24> (64bit Host)

```
以下是 RISC-V 核心寄存器：

======================= ========= =============================================
    Encoding            Register  Description
======================= ========= =============================================
  0x80x0 0000 0200 0000 regs.pc   Program counter
  0x80x0 0000 0200 0001 regs.ra   Return address
  0x80x0 0000 0200 0002 regs.sp   Stack pointer
  0x80x0 0000 0200 0003 regs.gp   Global pointer
  0x80x0 0000 0200 0004 regs.tp   Task pointer
  0x80x0 0000 0200 0005 regs.t0   Caller saved register 0
  0x80x0 0000 0200 0006 regs.t1   Caller saved register 1
  0x80x0 0000 0200 0007 regs.t2   Caller saved register 2
  0x80x0 0000 0200 0008 regs.s0   Callee saved register 0
  0x80x0 0000 0200 0009 regs.s1   Callee saved register 1
  0x80x0 0000 0200 000a regs.a0   Function argument (or return value) 0
  0x80x0 0000 0200 000b regs.a1   Function argument (or return value) 1
  0x80x0 0000 0200 000c regs.a2   Function argument 2
  0x80x0 0000 0200 000d regs.a3   Function argument 3
  0x80x0 0000 0200 000e regs.a4   Function argument 4
  0x80x0 0000 0200 000f regs.a5   Function argument 5
  0x80x0 0000 0200 0010 regs.a6   Function argument 6
  0x80x0 0000 0200 0011 regs.a7   Function argument 7
  0x80x0 0000 0200 0012 regs.s2   Callee saved register 2
  0x80x0 0000 0200 0013 regs.s3   Callee saved register 3
  0x80x0 0000 0200 0014 regs.s4   Callee saved register 4
  0x80x0 0000 0200 0015 regs.s5   Callee saved register 5
  0x80x0 0000 0200 0016 regs.s6   Callee saved register 6
  0x80x0 0000 0200 0017 regs.s7   Callee saved register 7
  0x80x0 0000 0200 0018 regs.s8   Callee saved register 8
  0x80x0 0000 0200 0019 regs.s9   Callee saved register 9
  0x80x0 0000 0200 001a regs.s10  Callee saved register 10
  0x80x0 0000 0200 001b regs.s11  Callee saved register 11
  0x80x0 0000 0200 001c regs.t3   Caller saved register 3
  0x80x0 0000 0200 001d regs.t4   Caller saved register 4
  0x80x0 0000 0200 001e regs.t5   Caller saved register 5
  0x80x0 0000 0200 001f regs.t6   Caller saved register 6
  0x80x0 0000 0200 0020 mode      Privilege mode (1 = S-mode or 0 = U-mode)
======================= ========= =============================================

RISC-V csr 寄存器表示监督者模式的控制/状态寄存器
```

  0x8020 0000 03 <index into the kvm_riscv_csr struct:24> (32bit Host)
  0x8030 0000 03 <index into the kvm_riscv_csr struct:24> (64bit Host)

```
以下是 RISC-V csr 寄存器：

======================= ========= =============================================
    Encoding            Register  Description
======================= ========= =============================================
  0x80x0 0000 0300 0000 sstatus   Supervisor status
  0x80x0 0000 0300 0001 sie       Supervisor interrupt enable
  0x80x0 0000 0300 0002 stvec     Supervisor trap vector base
  0x80x0 0000 0300 0003 sscratch  Supervisor scratch register
  0x80x0 0000 0300 0004 sepc      Supervisor exception program counter
  0x80x0 0000 0300 0005 scause    Supervisor trap cause
  0x80x0 0000 0300 0006 stval     Supervisor bad address or instruction
  0x80x0 0000 0300 0007 sip       Supervisor interrupt pending
  0x80x0 0000 0300 0008 satp      Supervisor address translation and protection
======================= ========= =============================================

RISC-V 定时器寄存器表示客户机 VCPU 的定时器状态，它具有
```

  0x8030 0000 04 <index into the kvm_riscv_timer struct:24>

```
以下是 RISC-V 定时器寄存器：

======================= ========= =============================================
    Encoding            Register  Description
======================= ========= =============================================
  0x8030 0000 0400 0000 frequency Time base frequency (read-only)
  0x8030 0000 0400 0001 time      Time value visible to Guest
  0x8030 0000 0400 0002 compare   Time compare programmed by Guest
  0x8030 0000 0400 0003 state     Time compare state (1 = ON or 0 = OFF)
======================= ========= =============================================

RISC-V F-extension 寄存器表示单精度浮点
```

  0x8020 0000 05 <index into the __riscv_f_ext_state struct:24>

```
以下是 RISC-V F-extension 寄存器：

======================= ========= =============================================
    Encoding            Register  Description
======================= ========= =============================================
  0x8020 0000 0500 0000 f[^0^]      Floating point register 0
  ...
  0x8020 0000 0500 001f f[^31^]     Floating point register 31
  0x8020 0000 0500 0020 fcsr      Floating point control and status register
======================= ========= =============================================

RISC-V D-extension 寄存器表示双精度浮点
```

  0x8020 0000 06 <index into the __riscv_d_ext_state struct:24> (fcsr)
  0x8030 0000 06 <index into the __riscv_d_ext_state struct:24> (non-fcsr)

```
以下是 RISC-V D-extension 寄存器：

======================= ========= =============================================
    Encoding            Register  Description
======================= ========= =============================================
  0x8030 0000 0600 0000 f[^0^]      Floating point register 0
  ...
  0x8030 0000 0600 001f f[^31^]     Floating point register 31
  0x8020 0000 0600 0020 fcsr      Floating point control and status register
======================= ========= =============================================

LoongArch 寄存器映射使用低 32 位。其中的高 16 位是寄存器组类型。

LoongArch csr 寄存器用于控制客户机 cpu 或获取客户机状态
```

  0x9030 0000 0001 00 <reg:5> <sel:3>   (64-bit)

```
LoongArch KVM 控制寄存器用于实现一些新定义的功能
```

  0x9030 0000 0002 <reg:16>

```
```

  0x2030 0002 <msr number:32>

```
以下是 x86 的 KVM 定义寄存器：

======================= ========= =============================================
    Encoding            Register  Description
======================= ========= =============================================
  0x2030 0003 0000 0000 SSP       Shadow Stack Pointer
======================= ========= =============================================

### 4.69 KVM_GET_ONE_REG


:Capability: KVM_CAP_ONE_REG
:Architectures: all
:Type: vcpu ioctl
:Parameters: struct kvm_one_reg (in and out)
:Returns: 0 on success, negative value on failure

错误码包括：

  ======== ============================================================
  ENOENT   没有该寄存器
  EINVAL   无效的寄存器 ID，或没有该寄存器，或与 s390 上受保护虚拟化
           模式下的 VM 一起使用
  EPERM    (arm64) 在 vcpu 定稿（finalization）之前不允许访问该寄存器
  ======== ============================================================

（这些错误码仅供参考：不要依赖在特定情况下返回特定的错误码。）

该 ioctl 允许接收 vcpu 中实现的单个寄存器的值。要读取的寄存器由传入的 kvm_one_reg 结构体的
"id" 字段指示。成功时，寄存器值可以在 "addr" 指向的内存位置找到。

使用该接口可访问的寄存器列表与 4.68 中的列表相同。


### 4.70 KVM_KVMCLOCK_CTRL


:Capability: KVM_CAP_KVMCLOCK_CTRL
:Architectures: Any that implement pvclocks (currently x86 only)
:Type: vcpu ioctl
:Parameters: None
:Returns: 0 on success, -1 on error

该 ioctl 设置一个对客户机可访问的标志，指示指定的 vCPU 已被宿主机用户空间暂停。

宿主机将在 pvclock 结构体中设置一个标志，该标志由 soft lockup 看门狗检查。该标志是客户机与
宿主机之间共享的 pvclock 结构体的一部分，具体是 pvclock_vcpu_time_info 结构体的 flags 字段的
第二位。它由宿主机独占设置，由客户机独占读取/清除。客户机检查和清除该标志的操作必须是原子
操作，因此必须使用 load-link/store-conditional 或等价指令。客户机在两种情况下会清除该标志：
当 soft lockup 看门狗定时器重置自身时，或当检测到 soft lockup 时。该 ioctl 可以在暂停 vcpu 之后、
但在其恢复之前的任何时间调用。


### 4.71 KVM_SIGNAL_MSI


:Capability: KVM_CAP_SIGNAL_MSI
:Architectures: x86 arm64
:Type: vm ioctl
:Parameters: struct kvm_msi (in)
:Returns: >0 on delivery, 0 if guest blocked the MSI, and -1 on error

直接注入一条 MSI 消息。仅在能处理 MSI 消息的内核态 irqchip 下有效。

```

  struct kvm_msi {
	__u32 address_lo;
	__u32 address_hi;
	__u32 data;
	__u32 flags;
	__u32 devid;
	__u8  pad[12];
  };

```
flags:
  KVM_MSI_VALID_DEVID：devid 包含一个有效值。每 VM 的 KVM_CAP_MSI_DEVID 能力用于通告需要提供
  设备 ID 的要求。如果该能力不可用，用户空间绝不应设置 KVM_MSI_VALID_DEVID 标志，否则 ioctl
  可能会失败。

如果设置了 KVM_MSI_VALID_DEVID，则 devid 包含写入 MSI 消息的设备的唯一设备标识符。对于 PCI，
这通常是低 16 位中的 BDF 标识符。

在 x86 上，除非启用了 KVM_CAP_X2APIC_API 能力的 KVM_X2APIC_API_USE_32BIT_IDS 特性，否则 address_hi
会被忽略。如果启用，address_hi 的 31-8 位提供目的 id 的 31-8 位。address_hi 的 7-0 位必须为零。


### 4.71 KVM_CREATE_PIT2


:Capability: KVM_CAP_PIT2
:Architectures: x86
:Type: vm ioctl
:Parameters: struct kvm_pit_config (in)
:Returns: 0 on success, -1 on error

为 i8254 PIT 创建一个内核态设备模型。该调用仅在通过 KVM_CREATE_IRQCHIP 启用内核态 irqchip
支持之后才有效。以下
```

  struct kvm_pit_config {
	__u32 flags;
	__u32 pad[15];
  };

```
```

  #define KVM_PIT_SPEAKER_DUMMY     1 /* emulate speaker port stub */

```
PIT 定时器中断可以使用一个每 VM 的内核线程来注入。如果它
```

  kvm-pit/<owner-process-pid>

```
在运行具有高优先级的客户机时，可能需要相应地调整该线程的调度参数。

此 IOCTL 取代了已过时的 KVM_CREATE_PIT。


### 4.72 KVM_GET_PIT2


:Capability: KVM_CAP_PIT_STATE2
:Architectures: x86
:Type: vm ioctl
:Parameters: struct kvm_pit_state2 (out)
:Returns: 0 on success, -1 on error

获取内核态 PIT 模型的状态。仅在
```

  struct kvm_pit_state2 {
	struct kvm_pit_channel_state channels[3];
	__u32 flags;
	__u32 reserved[9];
  };

```
```

  /* disable PIT in HPET legacy mode */
  #define KVM_PIT_FLAGS_HPET_LEGACY     0x00000001
  /* speaker port data bit enabled */
  #define KVM_PIT_FLAGS_SPEAKER_DATA_ON 0x00000002

```
此 IOCTL 取代了已过时的 KVM_GET_PIT。


### 4.73 KVM_SET_PIT2


:Capability: KVM_CAP_PIT_STATE2
:Architectures: x86
:Type: vm ioctl
:Parameters: struct kvm_pit_state2 (in)
:Returns: 0 on success, -1 on error

设置内核态 PIT 模型的状态。仅在 KVM_CREATE_PIT2 之后有效。关于 struct kvm_pit_state2 的细节
请参见 KVM_GET_PIT2。

  `KVM_SET_PIT2` 严格遵守 Intel 8254 PIT 的规范。例如，`struct kvm_pit_channel_state` 中
  `count` 值为 0 被解释为 65536，即最大计数值。参考 `Intel 8254 programmable interval
  timer <https://www.scs.stanford.edu/10wi-cs140/pintos/specs/8254.pdf>`_。

此 IOCTL 取代了已过时的 KVM_SET_PIT。


### 4.74 KVM_PPC_GET_SMMU_INFO


:Capability: KVM_CAP_PPC_GET_SMMU_INFO
:Architectures: powerpc
:Type: vm ioctl
:Parameters: None
:Returns: 0 on success, -1 on error

该 ioctl 填充并返回一个描述 KVM 支持的"服务器"级 MMU 模拟特性的结构体。用户空间反过来可以用它
为客机操作系统生成适当的设备树属性。

该结构体包含一些全局信息，后面跟着一个
```

      struct kvm_ppc_smmu_info {
	     __u64 flags;
	     __u32 slb_size;
	     __u32 pad;
	     struct kvm_ppc_one_seg_page_size sps[KVM_PPC_PAGE_SIZES_MAX_SZ];
      };

```
支持的标志如下：

    - KVM_PPC_PAGE_SIZES_REAL:
        当设置该标志时，客户机页大小必须"适配"后备存储的页大小。当未设置时，列表中的任何页大小
        都可以使用，而不管它们如何由用户空间作为后备。

    - KVM_PPC_1T_SEGMENTS
        除了标准的 256M 段之外，模拟的 MMU 还支持 1T 段。

    - KVM_PPC_NO_HASH
	该标志表示 KVM 不支持 HPT 客户机，因此所有客户机必须使用 radix MMU 模式。

"slb_size" 字段指示支持多少个 SLB 条目。

"sps" 数组包含 8 个条目，按递增顺序指示段支持的基页大小。每个条目定义为
```

   struct kvm_ppc_one_seg_page_size {
	__u32 page_shift;	/* Base page shift of segment (or 0) */
	__u32 slb_enc;		/* SLB encoding for BookS */
	struct kvm_ppc_one_page_size enc[KVM_PPC_PAGE_SIZES_MAX_SZ];
   };

```
"page_shift" 为 0 的条目未被使用。由于数组按递增顺序组织，遇到此类条目时查找即可停止。

"slb_enc" 字段提供在 SLB 中用于该页大小的编码。这些位的位置使得该值可以直接按位或到 slbmte
指令的 "vsid" 参数中。

"enc" 数组是一个列表，针对每个段基页大小提供受支持的实际页大小列表（只能大于或等于基页大小），
以及哈希 PTE 中的相应编码。类似地，该数组是 8 个条目，按递增大小排序，而 "0" 偏移的条目
```

   struct kvm_ppc_one_page_size {
	__u32 page_shift;	/* Page shift (or 0) */
	__u32 pte_enc;		/* Encoding in the HPTE (>>12) */
   };

```
"pte_enc" 字段提供一个值，可以按位或到哈希 PTE 的 RPN 字段中（即，需要先左移 12 位才能按位或
到哈希 PTE 的第二个双字中）。

### 4.75 KVM_IRQFD


:Capability: KVM_CAP_IRQFD
:Architectures: x86 s390 arm64
:Type: vm ioctl
:Parameters: struct kvm_irqfd (in)
:Returns: 0 on success, -1 on error

允许设置一个 eventfd 以直接触发一次客户机中断。kvm_irqfd.fd 指定用作 eventfd 的文件描述符，
kvm_irqfd.gsi 指定由此事件切换的 irqchip 引脚。当 eventfd 上触发一个事件时，会使用指定的 gsi
引脚向客户机注入一个中断。使用 KVM_IRQFD_FLAG_DEASSIGN 标志并同时指定 kvm_irqfd.fd 和
kvm_irqfd.gsi，可以移除该 irqfd。

借助 KVM_CAP_IRQFD_RESAMPLE，KVM_IRQFD 支持去断言（de-assert）和通知机制，从而允许模拟基于
irqfd 的电平触发中断。当设置 KVM_IRQFD_FLAG_RESAMPLE 时，用户必须在 kvm_irqfd.resamplefd 字段
中传入一个额外的 eventfd。在重采样模式下，通过 kvm_irq.fd 投递中断会断言 irqchip 中指定的 gsi。
当 irqchip 被重采样时（例如来自 EOI），gsi 被去断言，并通过 kvm_irqfd.resamplefd 通知用户。是否
重新排队该中断，由用户负责，前提是使用它的设备仍需要服务。注意，关闭 resamplefd 不足以禁用
该 irqfd。KVM_IRQFD_FLAG_RESAMPLE 仅在分配时需要，而不必与 KVM_IRQFD_FLAG_DEASSIGN 一起指定。

在 arm64 上，由于支持 gsi 路由，可能发生以下情况：

- 如果没有与该 gsi 关联的路由条目，注入失败
- 如果该 gsi 关联到 irqchip 路由条目，irqchip.pin + 32 对应于被注入的 SPI ID
- 如果该 gsi 关联到 MSI 路由条目，MSI 消息和设备 ID 被转换为一个 LPI（支持仅限于 GICv3 ITS
  的内核态模拟）

### 4.76 KVM_PPC_ALLOCATE_HTAB


:Capability: KVM_CAP_PPC_ALLOC_HTAB
:Architectures: powerpc
:Type: vm ioctl
:Parameters: Pointer to u32 containing hash table order (in/out)
:Returns: 0 on success, -1 on error

该 ioctl 请求宿主机内核使用 PAPR 半虚拟化接口为客户机分配一个 MMU 哈希表。这仅在内核配置为使用
Book 3S HV 风格的虚拟化时才起作用。否则该能力不存在，ioctl 返回 ENOTTY 错误。本说明的其余部分
假设为 Book 3S HV。

调用此 ioctl 时不能有正在运行的 vcpu；如果有，它将不执行任何操作并返回 EBUSY 错误。

参数是一个指向 32 位无符号整数变量的指针，该变量包含所需哈希表大小（以 2 为底的对数）的阶（order），
其取值范围必须在 18 到 46 之间。在 ioctl 成功返回时，该值不会被内核改变。

如果当任何 vcpu 被要求运行（通过 KVM_RUN ioctl）时尚未分配哈希表，宿主机内核将分配一个默认大小
的哈希表（16 MB）。

如果在哈希表已分配的情况下调用此 ioctl，且阶与现有哈希表不同，则会释放现有哈希表并分配一个新的。
如果在哈希表已分配且阶与指定相同时调用此 ioctl，内核将清空现有哈希表（将所有 HPTE 置零）。无论
哪种情况，如果客户机使用了虚拟化实模式区域（VRMA）设施，内核将在任何 vcpu 的下一次 KVM_RUN 时
重新创建 VMRA HPTE。

### 4.77 KVM_S390_INTERRUPT


:Capability: basic
:Architectures: s390
:Type: vm ioctl, vcpu ioctl
:Parameters: struct kvm_s390_interrupt (in)
:Returns: 0 on success, -1 on error

允许向客户机注入一个中断。根据中断类型，中断可以是浮动的（vm ioctl）或每 cpu 的（vcpu ioctl）。

```

  struct kvm_s390_interrupt {
	__u32 type;
	__u32 parm;
	__u64 parm64;
  };

```
type 可以是以下之一：

KVM_S390_SIGP_STOP (vcpu)
    - sigp 停止；可选标志在 parm 中
KVM_S390_PROGRAM_INT (vcpu)
    - 程序检查；code 在 parm 中
KVM_S390_SIGP_SET_PREFIX (vcpu)
    - sigp 设置前缀；前缀地址在 parm 中
KVM_S390_RESTART (vcpu)
    - 重启
KVM_S390_INT_CLOCK_COMP (vcpu)
    - 时钟比较器中断
KVM_S390_INT_CPU_TIMER (vcpu)
    - CPU 定时器中断
KVM_S390_INT_VIRTIO (vm)
    - virtio 外部中断；外部中断参数在 parm 和 parm64 中
KVM_S390_INT_SERVICE (vm)
    - sclp 外部中断；sclp 参数在 parm 中
KVM_S390_INT_EMERGENCY (vcpu)
    - sigp 紧急；源 cpu 在 parm 中
KVM_S390_INT_EXTERNAL_CALL (vcpu)
    - sigp 外部调用；源 cpu 在 parm 中
KVM_S390_INT_IO(ai,cssid,ssid,schid) (vm)
    - 复合值，指示一个 I/O 中断（ai - 适配器中断；cssid,ssid,schid - 子通道）；
      I/O 中断参数在 parm（子通道）和 parm64（intparm，中断子类）中
KVM_S390_MCHK (vm, vcpu)
    - 机器检查中断；cr 14 位在 parm 中，机器检查中断码在 parm64 中（注意，需要额外负载的
      机器检查不受此 ioctl 支持）

这是一个异步的 vcpu ioctl，可以从任何线程调用。

### 4.78 KVM_PPC_GET_HTAB_FD


:Capability: KVM_CAP_PPC_HTAB_FD
:Architectures: powerpc
:Type: vm ioctl
:Parameters: Pointer to struct kvm_get_htab_fd (in)
:Returns: file descriptor number (>= 0) on success, -1 on error

该 ioctl 返回一个文件描述符，可用于读出客户机哈希页表（HPT）中的条目，或写入条目以初始化 HPT。
仅当参数的 flags 字段中设置了 KVM_GET_HTAB_WRITE 位时，返回的 fd 才可写；仅当该位清零时，才可
读。参数结构体如下
```

  /* For KVM_PPC_GET_HTAB_FD */
  struct kvm_get_htab_fd {
	__u64	flags;
	__u64	start_index;
	__u64	reserved[2];
  };

  /* Values for kvm_get_htab_fd.flags */
  #define KVM_GET_HTAB_BOLTED_ONLY	((__u64)0x1)
  #define KVM_GET_HTAB_WRITE		((__u64)0x2)

```
'start_index' 字段给出 HPT 中开始读取的条目的索引。写入时忽略该字段。

对 fd 的读取最初会提供所有"有趣"的 HPT 条目的信息。如果设置了 KVM_GET_HTAB_BOLTED_ONLY 位，
有趣的条目是那些置位了 bolted 位的条目；否则是所有条目。到达 HPT 末尾时，read() 会返回。如果
再次对 fd 调用 read()，它会从 HPT 开头重新开始，但只返回自上次读取以来发生变化的 HPT 条目。

读取或写入的数据结构为一个头部（8 字节），后跟一系列有效的 HPT 条目（每条 16 字节）。头部指示
有多少个有效 HPT 条目，以及有效条目之后跟随多少个无效条目。无效条目不被显式表示
```

  struct kvm_get_htab_header {
	__u32	index;
	__u16	n_valid;
	__u16	n_invalid;
  };

```
对 fd 的写入从头部中给出的索引处创建 HPT 条目；先是 'n_valid' 个来自写入数据的有效条目，然后是
'n_invalid' 个无效条目，使找到的任何先前有效条目失效。

### 4.79 KVM_CREATE_DEVICE


:Capability: KVM_CAP_DEVICE_CTRL
:Architectures: all
:Type: vm ioctl
:Parameters: struct kvm_create_device (in/out)
:Returns: 0 on success, -1 on error

错误码：

  ======  =======================================================
  ENODEV  设备类型未知或不被支持
  EEXIST  设备已创建，且该类型的设备可能不会实例化多次
  ======  =======================================================

  其他错误条件可能由各个设备类型定义，或具有其标准含义。

在内核中创建一个模拟设备。在 fd 中返回的文件描述符可用于 KVM_SET/GET/HAS_DEVICE_ATTR。

如果设置了 KVM_CREATE_DEVICE_TEST 标志，则只测试设备类型是否受支持（不一定是它能否在当前 vm 中
创建）。

各个设备不应定义标志。属性应用于指定任何不被设备类型编号所暗示的行为。

```

  struct kvm_create_device {
	__u32	type;	/* in: KVM_DEV_TYPE_xxx */
	__u32	fd;	/* out: device handle */
	__u32	flags;	/* in: KVM_CREATE_DEVICE_xxx */
  };

```
### 4.80 KVM_SET_DEVICE_ATTR/KVM_GET_DEVICE_ATTR


:Capability: KVM_CAP_DEVICE_CTRL, KVM_CAP_VM_ATTRIBUTES for vm device,
             KVM_CAP_VCPU_ATTRIBUTES for vcpu device
             KVM_CAP_SYS_ATTRIBUTES for system (/dev/kvm) device (no set)
:Architectures: x86, arm64, s390
:Type: device ioctl, vm ioctl, vcpu ioctl
:Parameters: struct kvm_device_attr
:Returns: 0 on success, -1 on error

错误码：

  =====   =============================================================
  ENXIO   该组或属性对此外设未知/不受支持，或缺少硬件支持。
  EPERM   该属性（当前）不能以这种方式访问
          （例如只读属性，或仅在设备处于不同状态时才有意义的属性）
  =====   =============================================================

  其他错误条件可能由各个设备类型定义。

获取/设置指定的设备配置和/或状态片段。其语义是设备相关的。请参见 "devices" 目录中的各个
设备文档。与 ONE_REG 一样，传输数据的大小由特定属性定义。

```

  struct kvm_device_attr {
	__u32	flags;		/* no flags currently defined */
	__u32	group;		/* device-defined */
	__u64	attr;		/* group-defined */
	__u64	addr;		/* userspace address of attr data */
  };

```
### 4.81 KVM_HAS_DEVICE_ATTR


:Capability: KVM_CAP_DEVICE_CTRL, KVM_CAP_VM_ATTRIBUTES for vm device,
             KVM_CAP_VCPU_ATTRIBUTES for vcpu device
             KVM_CAP_SYS_ATTRIBUTES for system (/dev/kvm) device
:Type: device ioctl, vm ioctl, vcpu ioctl
:Parameters: struct kvm_device_attr
:Returns: 0 on success, -1 on error

错误码：

  =====   =============================================================
  ENXIO   该组或属性对此外设未知/不受支持，或缺少硬件支持。
  =====   =============================================================

测试一个设备是否支持特定属性。成功返回表示已实现该属性。它并不一定表示该属性可以在设备
当前状态下被读取或写入。"addr" 被忽略。


### 4.82 KVM_ARM_VCPU_INIT


:Capability: basic
:Architectures: arm64
:Type: vcpu ioctl
:Parameters: struct kvm_vcpu_init (in)
:Returns: 0 on success; -1 on error

错误码：

  ======     =================================================================
  EINVAL    目标未知，或特性组合无效。
  ENOENT    指定的某个特性位未知。
  ======     =================================================================

该 ioctl 告诉 KVM 要向客户机呈现什么类型的 CPU，以及它应具有哪些可选特性。这将使 cpu 寄存器
重置为它们的初始值。如果未调用它，KVM_RUN 将对该 vcpu 返回 ENOEXEC。

初始值定义为：
 - 处理器状态：
  - AArch64：EL1h，D、A、I 和 F 位置位。所有其他位清零。
  - AArch32：SVC，A、I 和 F 位置位。所有其他位清零。
 - 通用寄存器，包括 PC 和 SP：置为 0
 - FPSIMD/NEON 寄存器：置为 0
 - SVE 寄存器：置为 0
 - 系统寄存器：重置为架构定义的初始值，即针对 EL1（或 SVC）或 EL2（在启用 EL2 的情况下）
   的热复位值。

注意，由于某些寄存器反映机器拓扑，所有 vcpu 都应在此 ioctl 调用之前创建。

用户空间可以对给定的 vcpu 多次调用此函数，包括在 vcpu 运行之后。这将把 vcpu 重置为其初始状态。
初始调用之后的所有调用必须使用相同的目标以及相同的特性标志集合，否则将返回 EINVAL。

可能的特性：

 - KVM_ARM_VCPU_POWER_OFF：以断电状态启动 CPU。
	  依赖于 KVM_CAP_ARM_PSCI。如果未设置，则在调用 KVM_RUN 时 CPU 将上电并
	  执行客户机代码。
 - KVM_ARM_VCPU_EL1_32BIT：以 32 位模式启动 CPU。
	  依赖于 KVM_CAP_ARM_EL1_32BIT（仅 arm64）。
 - KVM_ARM_VCPU_PSCI_0_2：为该 CPU 模拟 PSCI v0.2（或与 v0.2 向后兼容的未来修订版）。
	  依赖于 KVM_CAP_ARM_PSCI_0_2。
 - KVM_ARM_VCPU_PMU_V3：为该 CPU 模拟 PMUv3。
	  依赖于 KVM_CAP_ARM_PMU_V3。

 - KVM_ARM_VCPU_PTRAUTH_ADDRESS：启用地址指针认证，仅适用于 arm64。
	  依赖于 KVM_CAP_ARM_PTRAUTH_ADDRESS。
	  如果 KVM_CAP_ARM_PTRAUTH_ADDRESS 和 KVM_CAP_ARM_PTRAUTH_GENERIC 都存在，
	  则必须同时请求 KVM_ARM_VCPU_PTRAUTH_ADDRESS 和 KVM_ARM_VCPU_PTRAUTH_GENERIC，
	  或者两者都不请求。

 - KVM_ARM_VCPU_PTRAUTH_GENERIC：启用通用指针认证，仅适用于 arm64。
	  依赖于 KVM_CAP_ARM_PTRAUTH_GENERIC。
	  如果 KVM_CAP_ARM_PTRAUTH_ADDRESS 和 KVM_CAP_ARM_PTRAUTH_GENERIC 都存在，
	  则必须同时请求 KVM_ARM_VCPU_PTRAUTH_ADDRESS 和 KVM_ARM_VCPU_PTRAUTH_GENERIC，
	  或者两者都不请求。

 - KVM_ARM_VCPU_SVE：为 CPU 启用 SVE（仅 arm64）。
	  依赖于 KVM_CAP_ARM_SVE。
	  需要 KVM_ARM_VCPU_FINALIZE(KVM_ARM_VCPU_SVE)：

    - 在 KVM_ARM_VCPU_INIT 之后：

       - 可以使用 KVM_GET_ONE_REG 读取 KVM_REG_ARM64_SVE_VLS：该伪寄存器的初始值指示
	      在此宿主机上 vcpu 可能的最佳向量长度集合。

    - 在 KVM_ARM_VCPU_FINALIZE(KVM_ARM_VCPU_SVE) 之前：

       - KVM_RUN 和 KVM_GET_REG_LIST 不可用；

       - 不能使用 KVM_GET_ONE_REG 和 KVM_SET_ONE_REG 访问可伸缩的架构 SVE 寄存器
	        KVM_REG_ARM64_SVE_ZREG()、KVM_REG_ARM64_SVE_PREG() 或
	        KVM_REG_ARM64_SVE_FFR；

       - 可以选择使用 KVM_SET_ONE_REG 写入 KVM_REG_ARM64_SVE_VLS，以修改 vcpu
	       可用的向量长度集合。

    - 在 KVM_ARM_VCPU_FINALIZE(KVM_ARM_VCPU_SVE) 之后：

       - KVM_REG_ARM64_SVE_VLS 伪寄存器变为不可变，不能再使用 KVM_SET_ONE_REG 写入。

 - KVM_ARM_VCPU_HAS_EL2：启用嵌套虚拟化支持，从 EL2 而不是 EL1 启动客户机。
	  依赖于 KVM_CAP_ARM_EL2。
	  除非同时设置了 KVM_ARM_VCPU_HAS_EL2_E2H0，否则 VM 以 HCR_EL2.E2H 为 RES1（VHE）
	  的方式运行。

 - KVM_ARM_VCPU_HAS_EL2_E2H0：将嵌套虚拟化支持限制为 HCR_EL2.E2H 为 RES0（非 VHE）。
	  依赖于 KVM_CAP_ARM_EL2_E2H0。
	  还必须设置 KVM_ARM_VCPU_HAS_EL2。

### 4.83 KVM_ARM_PREFERRED_TARGET


:Capability: basic
:Architectures: arm64
:Type: vm ioctl
:Parameters: struct kvm_vcpu_init (out)
:Returns: 0 on success; -1 on error

错误码：

  ======     ==========================================
  ENODEV     宿主机没有可用的首选目标
  ======     ==========================================

该 ioctl 查询 KVM 在底层宿主机上可模拟的首选 CPU 目标类型。

该 ioctl 返回 struct kvm_vcpu_init 实例，其中包含有关首选 CPU 目标类型及其推荐特性的信息。
如果首选目标建议设置这些特性，则返回的 kvm_vcpu_init->features 位图会置上相应的特性位，但
这并非强制要求。

该 ioctl 返回的信息可用于准备 struct kvm_vcpu_init 实例以用于 KVM_ARM_VCPU_INIT ioctl，
从而生成与底层宿主机匹配的 VCPU。


### 4.84 KVM_GET_REG_LIST


:Capability: basic
:Architectures: arm64, mips, riscv, x86 (if KVM_CAP_ONE_REG)
:Type: vcpu ioctl
:Parameters: struct kvm_reg_list (in/out)
:Returns: 0 on success; -1 on error

错误码：

  =====      ==============================================================
  E2BIG      reg 索引列表太大，无法放入用户指定的数组中（所需的数量将被写入 n）。
  =====      ==============================================================

```

  struct kvm_reg_list {
	__u64 n; /* number of registers in reg[] */
	__u64 reg[0];
  };

```
该 ioctl 返回受 KVM_GET_ONE_REG/KVM_SET_ONE_REG 调用支持的客户机寄存器。

注意，由于历史原因（说白了就是没人关心），s390 不支持 KVM_GET_REG_LIST。在内核 4.x 及更新
版本中的寄存器集合为：

- KVM_REG_S390_TODPR

- KVM_REG_S390_EPOCHDIFF

- KVM_REG_S390_CPU_TIMER

- KVM_REG_S390_CLOCK_COMP

- KVM_REG_S390_PFTOKEN

- KVM_REG_S390_PFCOMPARE

- KVM_REG_S390_PFSELECT

- KVM_REG_S390_PP

- KVM_REG_S390_GBEA

注意，对于 x86，由 KVM_GET_MSR_INDEX_LIST 枚举的所有 MSR 都作为 KVM_X86_REG_TYPE_MSR 类型
受支持，但不会通过 KVM_GET_REG_LIST 枚举。

### 4.85 KVM_ARM_SET_DEVICE_ADDR (deprecated)


:Capability: KVM_CAP_ARM_SET_DEVICE_ADDR
:Architectures: arm64
:Type: vm ioctl
:Parameters: struct kvm_arm_device_address (in)
:Returns: 0 on success, -1 on error

错误码：

  ======  ============================================
  ENODEV  设备 id 未知
  ENXIO   当前系统不支持该设备
  EEXIST  地址已设置
  E2BIG   地址超出客户机物理地址空间
  EBUSY   地址与其他设备范围重叠
  ======  ============================================

```

  struct kvm_arm_device_addr {
	__u64 id;
	__u64 addr;
  };

```
在客户机物理地址空间中指定一个设备地址，客户机可以在该地址访问模拟或直通的、宿主机内核
需要知晓的设备。id 字段是特定设备的一个架构相关标识符。

arm64 将 id 字段分为两部分：一个设备 id 和一个
```

  bits:  | 63        ...       32 | 31    ...    16 | 15    ...    0 |
  field: |        0x00000000      |     device id   |  addr type id  |

```
arm64 目前仅在使用内核态 GIC 支持硬件 VGIC 特性时才需要它，使用 KVM_ARM_DEVICE_VGIC_V2
作为设备 id。在为客户的 VGIC 虚拟 CPU 和分发器（distributor）接口映射设置基址时，必须在
调用 KVM_CREATE_IRQCHIP 之后、但在任何 VCPU 上调用 KVM_RUN 之前调用该 ioctl。对任何基址
两次调用此 ioctl 将返回 -EEXIST。

注意，此 IOCTL 已废弃，应使用更灵活的 SET/GET_DEVICE_ATTR API 代替。


### 4.86 KVM_PPC_RTAS_DEFINE_TOKEN


:Capability: KVM_CAP_PPC_RTAS
:Architectures: ppc
:Type: vm ioctl
:Parameters: struct kvm_rtas_token_args
:Returns: 0 on success, -1 on error

为 RTAS（Run Time Abstraction Services，运行时抽象服务）服务定义一个令牌值，以允许它在内核中
被处理。参数结构体给出服务的名称，该名称必须是具有内核侧实现的服务名称。如果令牌值非零，它将
与该服务关联，客户机随后指定该令牌的 RTAS 调用将由内核处理。如果令牌值为 0，则与该服务关联的
任何令牌都将被遗忘，客户机随后针对该服务的 RTAS 调用将被传递给用户空间处理。

### 4.87 KVM_SET_GUEST_DEBUG


:Capability: KVM_CAP_SET_GUEST_DEBUG
:Architectures: x86, s390, ppc, arm64
:Type: vcpu ioctl
:Parameters: struct kvm_guest_debug (in)
:Returns: 0 on success; -1 on error

```

  struct kvm_guest_debug {
       __u32 control;
       __u32 pad;
       struct kvm_guest_debug_arch arch;
  };

```
设置处理器特定的调试寄存器，并配置 vcpu 以处理客户机调试事件。结构体有两部分，第一部分是一个
控制位域，指示运行时处理的调试事件类型。通用控制位如下：

  - KVM_GUESTDBG_ENABLE:        启用客户机调试
  - KVM_GUESTDBG_SINGLESTEP:    下一次运行应单步执行

control 字段的高 16 位是架构相关的控制标志，可包括以下：

  - KVM_GUESTDBG_USE_SW_BP:     使用软件断点 [x86, arm64]
  - KVM_GUESTDBG_USE_HW_BP:     使用硬件断点 [x86, s390]
  - KVM_GUESTDBG_USE_HW:        使用硬件调试事件 [arm64]
  - KVM_GUESTDBG_INJECT_DB:     注入 DB 类型异常 [x86]
  - KVM_GUESTDBG_INJECT_BP:     注入 BP 类型异常 [x86]
  - KVM_GUESTDBG_EXIT_PENDING:  触发立即的客户机退出 [s390]
  - KVM_GUESTDBG_BLOCKIRQ:      避免注入中断/NMI/SMI [x86]

例如，KVM_GUESTDBG_USE_SW_BP 表示内存中启用了软件断点，因此我们需要确保正确捕获断点异常，
并且 KVM 运行循环在断点处退出，而不是继续运行到正常的客户机向量。对于 KVM_GUESTDBG_USE_HW_BP，
我们需要确保客户机 vCPU 的架构相关寄存器被更新为正确的（提供的）值。

结构体的第二部分是架构相关的，通常包含一组调试寄存器。

对于 arm64，调试寄存器的数量是实现定义的，可以通过查询 KVM_CAP_GUEST_DEBUG_HW_BPS 和
KVM_CAP_GUEST_DEBUG_HW_WPS 能力来确定，这两个能力返回一个正数，指示受支持的寄存器数量。

对于 ppc，KVM_CAP_PPC_GUEST_DEBUG_SSTEP 能力指示是否支持单步调试事件
（KVM_GUESTDBG_SINGLESTEP）。

在受支持的情况下，KVM_CAP_SET_GUEST_DEBUG2 能力指示 control 字段中受支持的 KVM_GUESTDBG_* 位。

当调试事件以 KVM_EXIT_DEBUG 原因退出主运行循环时，kvm_run 结构体的 kvm_debug_exit_arch 部分
包含架构相关的调试信息。

### 4.88 KVM_GET_EMULATED_CPUID


:Capability: KVM_CAP_EXT_EMUL_CPUID
:Architectures: x86
:Type: system ioctl
:Parameters: struct kvm_cpuid2 (in/out)
:Returns: 0 on success, -1 on error

```

  struct kvm_cpuid2 {
	__u32 nent;
	__u32 flags;
	struct kvm_cpuid_entry2 entries[0];
  };

```
member 'flags' 字段用于从用户空间传递标志。

```

  #define KVM_CPUID_FLAG_SIGNIFCANT_INDEX		BIT(0)
  #define KVM_CPUID_FLAG_STATEFUL_FUNC		BIT(1) /* deprecated */
  #define KVM_CPUID_FLAG_STATE_READ_NEXT		BIT(2) /* deprecated */

  struct kvm_cpuid_entry2 {
	__u32 function;
	__u32 index;
	__u32 flags;
	__u32 eax;
	__u32 ebx;
	__u32 ecx;
	__u32 edx;
	__u32 padding[3];
  };

```
该 ioctl 返回由 kvm 模拟的 x86 cpuid 特性。用户空间可以使用该 ioctl 返回的信息来查询哪些特性
是由 kvm 模拟的，而不是原生存在的。

用户空间通过传入一个 kvm_cpuid2 结构体来调用 KVM_GET_EMULATED_CPUID，其中 'nent' 字段指示
可变长数组 'entries' 中的条目数量。如果条目数量太少而无法描述 cpu 能力，会返回错误（E2BIG）。
如果数量过多，'nent' 字段会被调整并返回一个错误（ENOMEM）。如果数量恰好合适，'nent' 字段会被
调整为 'entries' 数组中有效条目的数量，并随后被填充。

返回的条目是 kvm 模拟的各个特性的 CPUID 位集合，由 CPUID 指令返回，其中未知或不支持的特性位
被清零。

例如，像 x2apic 这样的特性可能不在主机 cpu 中，但因为可以被高效模拟而在 KVM_GET_SUPPORTED_CPUID
中由 kvm 暴露，因此不包含在此处。

每个条目中的字段定义如下：

  function:
	 用于获取该条目的 eax 值
  index:
	 用于获取该条目的 ecx 值（针对受 ecx 影响的条目）
  flags:
    以下零个或多个的按位或：

        KVM_CPUID_FLAG_SIGNIFCANT_INDEX:
           表示 index 字段有效

   eax, ebx, ecx, edx:

         该 function/index 组合下 cpuid 指令返回的值

### 4.89 KVM_S390_MEM_OP


:Capability: KVM_CAP_S390_MEM_OP, KVM_CAP_S390_PROTECTED, KVM_CAP_S390_MEM_OP_EXTENSION
:Architectures: s390
:Type: vm ioctl, vcpu ioctl
:Parameters: struct kvm_s390_mem_op (in)
:Returns: = 0 on success,
          < 0 on generic error (e.g. -EFAULT or -ENOMEM),
          16 bit program exception code if the access causes such an exception

从/向 VM 的内存读取或写入数据。KVM_CAP_S390_MEM_OP_EXTENSION 能力指定了受支持的功能。

```

  struct kvm_s390_mem_op {
	__u64 gaddr;		/* the guest address */
	__u64 flags;		/* flags */
	__u32 size;		/* amount of bytes */
	__u32 op;		/* type of operation */
	__u64 buf;		/* buffer in userspace */
	union {
		struct {
			__u8 ar;	/* the access register number */
			__u8 key;	/* access key, ignored if flag unset */
			__u8 pad1[6];	/* ignored */
			__u64 old_addr;	/* ignored if flag unset */
		};
		__u32 sida_offset; /* offset into the sida */
		__u8 reserved[32]; /* ignored */
	};
  };

```
内存区域的起始地址必须在 "gaddr" 字段中指定，区域的长度在 "size" 字段中（不能为 0）。"size"
的最大值可以通过检查 KVM_CAP_S390_MEM_OP 能力获得。"buf" 是用户空间应用程序提供的缓冲区，对于
读访问，读取的数据应写入该缓冲区；对于写访问，要写入的数据存储在该缓冲区中。"reserved" 字段
用于未来的扩展。保留和未使用的值会被忽略。添加成员的未来扩展必须引入新的标志。

操作类型在 "op" 字段中指定。可修改其行为的标志可以在 "flags" 字段中设置。未定义的标志位必须
置为 0。

可能的操作有：
  - `KVM_S390_MEMOP_LOGICAL_READ`
  - `KVM_S390_MEMOP_LOGICAL_WRITE`
  - `KVM_S390_MEMOP_ABSOLUTE_READ`
  - `KVM_S390_MEMOP_ABSOLUTE_WRITE`
  - `KVM_S390_MEMOP_SIDA_READ`
  - `KVM_S390_MEMOP_SIDA_WRITE`
  - `KVM_S390_MEMOP_ABSOLUTE_CMPXCHG`

##### Logical read/write（逻辑读/写）：


访问逻辑内存，即根据 VCPU 的状态将给定的客户机地址转换为绝对地址，并使用该绝对地址作为访问的
目标。"ar" 指定要使用的访问寄存器编号；有效范围是 0..15。逻辑访问仅允许用于 VCPU ioctl。逻辑
访问仅允许用于非受保护的客户机。

受支持的标志：
  - `KVM_S390_MEMOP_F_CHECK_ONLY`
  - `KVM_S390_MEMOP_F_INJECT_EXCEPTION`
  - `KVM_S390_MEMOP_F_SKEY_PROTECTION`

可以设置 KVM_S390_MEMOP_F_CHECK_ONLY 标志，以检查相应的内存访问是否会导致访问异常；但是，
不会对目标处内存中的数据进行实际访问。在这种情况下，"buf" 未被使用，可以为 NULL。

如果在访问期间发生了访问异常（或在 KVM_S390_MEMOP_F_CHECK_ONLY 情况下将会发生），ioctl 返回
一个正的错误号，指示异常的类型。如果设置了标志 KVM_S390_MEMOP_F_INJECT_EXCEPTION，该异常也会
直接在相应的 VCPU 上引发。在保护异常的情况下，除非另有说明，注入的翻译异常标识符（TEID）表示
抑制（suppression）。

如果设置了 KVM_S390_MEMOP_F_SKEY_PROTECTION 标志，存储键保护也会生效，并可能在访问因 "key"
指定的访问键而被禁止时导致异常；有效范围是 0..15。KVM_S390_MEMOP_F_SKEY_PROTECTION 在
KVM_CAP_S390_MEM_OP_EXTENSION 大于 0 时可用。由于被访问的内存可能跨越多个页，而这些页可能具有
不同的存储键，因此有可能在内存已被修改之后才发生保护异常。在这种情况下，如果注入了异常，TEID
不会指示抑制。

##### Absolute read/write（绝对读/写）：


访问绝对内存。该操作旨在与 KVM_S390_MEMOP_F_SKEY_PROTECTION 标志一起使用，以允许在一个操作中
访问内存并执行存储键保护所需的检查（相对于用户空间获取存储键、执行检查、然后访问内存，这可能会
在检查和访问之间产生延迟）。如果 KVM_CAP_S390_MEM_OP_EXTENSION 设置了
KVM_S390_MEMOP_EXTENSION_CAP_BASE 位，则绝对访问允许用于 VM ioctl。目前绝对访问不允许用于 VCPU
ioctl。绝对访问仅允许用于非受保护的客户机。

受支持的标志：
  - `KVM_S390_MEMOP_F_CHECK_ONLY`
  - `KVM_S390_MEMOP_F_SKEY_PROTECTION`

与逻辑访问共有的标志的语义与逻辑访问相同。

##### Absolute cmpxchg（绝对比较交换）：


对客户机绝对内存执行 cmpxchg。旨在与 KVM_S390_MEMOP_F_SKEY_PROTECTION 标志一起使用。与无条件
写入不同，仅当目标位置包含 "old_addr" 指向的值时才会发生访问。这作为一次原子 cmpxchg 执行，
长度由 "size" 参数指定。"size" 必须是 2 的幂，最大为 16（含）。如果因为目标值与新值不匹配而
未发生交换，则 "old_addr" 指向的值会被替换为目标值。用户空间可以通过检查是否发生了这种替换来
判断交换是否发生。如果 KVM_CAP_S390_MEM_OP_EXTENSION 设置了
KVM_S390_MEMOP_EXTENSION_CAP_CMPXCHG 标志，则 cmpxchg 操作允许用于 VM ioctl。

受支持的标志：
  - `KVM_S390_MEMOP_F_SKEY_PROTECTION`

##### SIDA read/write（SIDA 读/写）：


访问安全指令数据区（secure instruction data area），其中包含受保护客户机进行指令模拟所需的
内存操作数。SIDA 访问在 KVM_CAP_S390_PROTECTED 能力可用时提供。SIDA 访问仅允许用于 VCPU
ioctl。SIDA 访问仅允许用于受保护的客户机。

不支持任何标志。

### 4.90 KVM_S390_GET_SKEYS


:Capability: KVM_CAP_S390_SKEYS
:Architectures: s390
:Type: vm ioctl
:Parameters: struct kvm_s390_skeys
:Returns: 0 on success, KVM_S390_GET_SKEYS_NONE if guest is not using storage
          keys, negative value on error

该 ioctl 用于在 s390 上获取客户机存储键的值
```

  struct kvm_s390_skeys {
	__u64 start_gfn;
	__u64 count;
	__u64 skeydata_addr;
	__u32 flags;
	__u32 reserved[9];
  };

```
start_gfn 字段是你要获取其存储键的第一个客户机帧的编号。

count 字段是要获取其存储键的连续帧的数量（从 start_gfn 开始）。count 字段必须至少为 1，允许
的最大值定义为 KVM_S390_SKEYS_MAX。超出此范围的值将导致 ioctl 返回 -EINVAL。

skeydata_addr 字段是足以容纳 count 字节的缓冲区的地址。该缓冲区将被 ioctl 填入存储键数据。

### 4.91 KVM_S390_SET_SKEYS


:Capability: KVM_CAP_S390_SKEYS
:Architectures: s390
:Type: vm ioctl
:Parameters: struct kvm_s390_skeys
:Returns: 0 on success, negative value on error

该 ioctl 用于在 s390 架构上设置客户机存储键的值。该 ioctl 通过 kvm_s390_skeys 结构体接收参数。
结构体定义请参见 KVM_S390_GET_SKEYS 一节。

start_gfn 字段是你要设置其存储键的第一个客户机帧的编号。

count 字段是要获取其存储键的连续帧的数量（从 start_gfn 开始）。count 字段必须至少为 1，允许
的最大值定义为 KVM_S390_SKEYS_MAX。超出此范围的值将导致 ioctl 返回 -EINVAL。

skeydata_addr 字段是包含 count 字节存储键的缓冲区的地址。缓冲区中的每个字节将被设置为从
start_gfn 开始、共 count 个帧中每个帧的存储键。

注意：如果在给定的数据中发现任何架构无效的键值，ioctl 将返回 -EINVAL。

### 4.92 KVM_S390_IRQ


:Capability: KVM_CAP_S390_INJECT_IRQ
:Architectures: s390
:Type: vcpu ioctl
:Parameters: struct kvm_s390_irq (in)
:Returns: 0 on success, -1 on error

错误码：

  ======  =================================================================
  EINVAL  中断类型无效
          type 为 KVM_S390_SIGP_STOP 且 flag 参数为无效值，
          type 为 KVM_S390_INT_EXTERNAL_CALL 且 code 大于
          VCPU 的最大数量
  EBUSY   type 为 KVM_S390_SIGP_SET_PREFIX 且 vcpu 未停止，
          type 为 KVM_S390_SIGP_STOP 且已有一个 stop 中断挂起，
          type 为 KVM_S390_INT_EXTERNAL_CALL 且已有一个外部调用中断
          挂起
  ======  =================================================================

允许向客户机注入一个中断。

使用 struct kvm_s390_irq 作为参数可以注入无法通过 KVM_S390_INTERRUPT 注入的额外负载。

```

  struct kvm_s390_irq {
	__u64 type;
	union {
		struct kvm_s390_io_info io;
		struct kvm_s390_ext_info ext;
		struct kvm_s390_pgm_info pgm;
		struct kvm_s390_emerg_info emerg;
		struct kvm_s390_extcall_info extcall;
		struct kvm_s390_prefix_info prefix;
		struct kvm_s390_stop_info stop;
		struct kvm_s390_mchk_info mchk;
		char reserved[64];
	} u;
  };

```
type 可以是以下之一：

- KVM_S390_SIGP_STOP - sigp 停止；参数在 .stop 中
- KVM_S390_PROGRAM_INT - 程序检查；参数在 .pgm 中
- KVM_S390_SIGP_SET_PREFIX - sigp 设置前缀；参数在 .prefix 中
- KVM_S390_RESTART - 重启；无参数
- KVM_S390_INT_CLOCK_COMP - 时钟比较器中断；无参数
- KVM_S390_INT_CPU_TIMER - CPU 定时器中断；无参数
- KVM_S390_INT_EMERGENCY - sigp 紧急；参数在 .emerg 中
- KVM_S390_INT_EXTERNAL_CALL - sigp 外部调用；参数在 .extcall 中
- KVM_S390_MCHK - 机器检查中断；参数在 .mchk 中

这是一个异步的 vcpu ioctl，可以从任何线程调用。

### 4.94 KVM_S390_GET_IRQ_STATE


:Capability: KVM_CAP_S390_IRQ_STATE
:Architectures: s390
:Type: vcpu ioctl
:Parameters: struct kvm_s390_irq_state (out)
:Returns: >= number of bytes copied into buffer,
          -EINVAL if buffer size is 0,
          -ENOBUFS if buffer size is too small to fit all pending interrupts,
          -EFAULT if the buffer address was invalid

该 ioctl 允许用户空间在单个缓冲区中检索当前所有挂起中断的完整状态。用例包括迁移和自省。参数
结构体包含
```

  struct kvm_s390_irq_state {
	__u64 buf;
	__u32 flags;        /* will stay unused for compatibility reasons */
	__u32 len;
	__u32 reserved[4];  /* will stay unused for compatibility reasons */
  };

```
用户空间传入上述结构体，对于每个挂起的中断，一个 struct kvm_s390_irq 会被复制到提供的缓冲区中。

该结构体包含一个 flags 字段和一个 reserved 字段，用于未来的扩展。由于内核从未检查 flags == 0，
而 QEMU 也从未预清零 flags 和 reserved，因此未来如果不破坏兼容性，就无法使用这些字段。

如果返回 -ENOBUFS，则提供的缓冲区太小，用户空间可以使用更大的缓冲区重试。

### 4.95 KVM_S390_SET_IRQ_STATE


:Capability: KVM_CAP_S390_IRQ_STATE
:Architectures: s390
:Type: vcpu ioctl
:Parameters: struct kvm_s390_irq_state (in)
:Returns: 0 on success,
          -EFAULT if the buffer address was invalid,
          -EINVAL for an invalid buffer length (see below),
          -EBUSY if there were already interrupts pending,
          errors occurring when actually injecting the
          interrupt. See KVM_S390_IRQ.

该 ioctl 允许用户空间设置当前为该 vcpu 挂起的所有 cpu 本地中断的完整状态。它旨在用于迁移后
恢复中断状态。输入参数是一个用户空间缓冲区
```

  struct kvm_s390_irq_state {
	__u64 buf;
	__u32 flags;        /* will stay unused for compatibility reasons */
	__u32 len;
	__u32 reserved[4];  /* will stay unused for compatibility reasons */
  };

```
关于 flags 和 reserved 的限制同样适用。（见 KVM_S390_GET_IRQ_STATE）

buf 引用的用户空间内存包含每个要注入到客户机的中断对应的一个 struct kvm_s390_irq。

如果其中某个中断由于某种原因无法注入，ioctl 会中止。

len 必须是 sizeof(struct kvm_s390_irq) 的倍数。它必须 > 0，且不得超过
(max_vcpus + 32) * sizeof(struct kvm_s390_irq)，即可能挂起的 cpu 本地中断的最大数量。

### 4.96 KVM_SMI


:Capability: KVM_CAP_X86_SMM
:Architectures: x86
:Type: vcpu ioctl
:Parameters: none
:Returns: 0 on success, -1 on error

在线程的 vcpu 上排队一个 SMI。

### 4.97 KVM_X86_SET_MSR_FILTER


:Capability: KVM_CAP_X86_MSR_FILTER
:Architectures: x86
:Type: vm ioctl
:Parameters: struct kvm_msr_filter
:Returns: 0 on success, < 0 on error

```

  struct kvm_msr_filter_range {
  #define KVM_MSR_FILTER_READ  (1 << 0)
  #define KVM_MSR_FILTER_WRITE (1 << 1)
	__u32 flags;
	__u32 nmsrs; /* number of msrs in bitmap */
	__u32 base;  /* MSR index the bitmap starts at */
	__u8 *bitmap; /* a 1 bit allows the operations in flags, 0 denies */
  };

  #define KVM_MSR_FILTER_MAX_RANGES 16
  struct kvm_msr_filter {
  #define KVM_MSR_FILTER_DEFAULT_ALLOW (0 << 0)
  #define KVM_MSR_FILTER_DEFAULT_DENY  (1 << 0)
	__u32 flags;
	struct kvm_msr_filter_range ranges[KVM_MSR_FILTER_MAX_RANGES];
  };

```
`struct kvm_msr_filter_range` 的 flags 值：

`KVM_MSR_FILTER_READ`

  使用给定的位图过滤对 MSR 的读访问。位图中为 0 表示应拒绝读访问，为 1 表示无论默认过滤器
  动作如何，都应允许对特定 MSR 的读访问。

`KVM_MSR_FILTER_WRITE`

  使用给定的位图过滤对 MSR 的写访问。位图中为 0 表示应拒绝写访问，为 1 表示无论默认过滤器
  动作如何，都应允许对特定 MSR 的写访问。

`struct kvm_msr_filter` 的 flags 值：

`KVM_MSR_FILTER_DEFAULT_ALLOW`

  如果没有过滤范围匹配正在被访问的 MSR 索引，KVM 默认允许对所有 MSR 的访问。

`KVM_MSR_FILTER_DEFAULT_DENY`

  如果没有过滤范围匹配正在被访问的 MSR 索引，KVM 默认拒绝对所有 MSR 的访问。

该 ioctl 允许用户空间定义最多 16 个 MSR 范围位图，以拒绝通常被 KVM 允许的客机 MSR 访问。如果
某个 MSR 未被特定范围覆盖，则应用"默认"过滤行为。每个位图范围覆盖 [base .. base+nmsrs) 范围内的
MSR。

如果 MSR 访问被用户空间拒绝，由此产生的 KVM 行为取决于是否启用了
KVM_CAP_X86_USER_SPACE_MSR 的 KVM_MSR_EXIT_REASON_FILTER。如果启用了 KVM_MSR_EXIT_REASON_FILTER，
KVM 在被拒绝的访问上会退出到用户空间，即用户空间实际上拦截了该 MSR 访问。如果未启用
KVM_MSR_EXIT_REASON_FILTER，KVM 会在被拒绝的访问上向客户机注入一个 #GP。注意，如果在 VMX 转换
期间模拟 MSR 加载/存储时 MSR 访问被拒绝，KVM 会忽略 KVM_MSR_EXIT_REASON_FILTER。完整细节请参见
下面的警告。

如果 MSR 访问被用户空间允许，KVM 将根据 vCPU 模型模拟和/或虚拟化该访问。注意，如果访问被用户空间
允许，KVM 最终仍可能注入 #GP，例如 KVM 不支持该 MSR，或者为了遵循该 MSR 的架构行为。

默认情况下，KVM 以 KVM_MSR_FILTER_DEFAULT_ALLOW 模式运行，且没有 MSR 范围过滤器。

使用一组空范围（所有 nmsrs == 0）调用此 ioctl 会禁用 MSR 过滤。在该模式下，`KVM_MSR_FILTER_DEFAULT_DENY`
无效并会导致错误。

   MSR 访问作为指令执行（模拟或原生）的副作用不会被过滤，因为硬件在 RDMSR 和 WRMSR 之外不遵循
   MSR 位图，而 KVM 在模拟指令时会模仿该行为，以避免与硬件产生无意义的偏差。例如，RDPID 读取
   MSR_TSC_AUX，SYSENTER 读取 SYSENTER MSR，等等。

   MSR 通过专用 VMCS 字段加载/存储的，不会作为 VM-Enter/VM-Exit 模拟的一部分被过滤。

   MSR 通过 VMX 的加载/存储列表加载/存储的，会作为 VM-Enter/VM-Exit 模拟的一部分被过滤。如果
   在 VM-Enter 时 MSR 访问被拒绝，KVM 会合成一个一致性检查 VM-Exit（EXIT_REASON_MSR_LOAD_FAIL）。
   如果在 VM-Exit 时 MSR 访问被拒绝，KVM 会合成一个 VM-Abort。简而言之，KVM 扩展了 Intel 的
   架构列表，列出那些无法通过 VM-Enter/VM-Exit MSR 列表加载/保存的 MSR。平台所有者有责任将任何
   此类限制传达给其最终用户。

   x2APIC MSR 访问无法被过滤（KVM 会静默忽略覆盖任何 x2APIC MSR 的过滤器）。

注意，在 vCPU 运行时调用此 ioctl 本质上是竞态的。但是，KVM 确实保证 vCPU 将看到先前的过滤器
或新的过滤器之一，例如，在旧过滤器和新过滤器中具有相同设置的 MSR 将具有确定性的行为。

类似地，如果用户空间希望在拒绝的访问上进行拦截，必须在激活任何过滤器之前启用
KVM_MSR_EXIT_REASON_FILTER，并在所有过滤器停用之后才将其关闭。否则可能导致 KVM 注入 #GP 而不是
退出到用户空间。

### 4.98 KVM_CREATE_SPAPR_TCE_64


:Capability: KVM_CAP_SPAPR_TCE_64
:Architectures: powerpc
:Type: vm ioctl
:Parameters: struct kvm_create_spapr_tce_64 (in)
:Returns: file descriptor for manipulating the created TCE table

这是 KVM_CAP_SPAPR_TCE 的扩展，后者仅支持 32 位窗口，在 4.62 KVM_CREATE_SPAPR_TCE 中描述。

```

  /* for KVM_CAP_SPAPR_TCE_64 */
  struct kvm_create_spapr_tce_64 {
	__u64 liobn;
	__u32 page_shift;
	__u32 flags;
	__u64 offset;	/* in pages */
	__u64 size; 	/* in pages */
  };

```
该扩展的目的是支持一个额外的、具有可变页大小的更大 DMA 窗口。KVM_CREATE_SPAPR_TCE_64 接收
一个 64 位的窗口大小、一个 IOMMU 页偏移（page shift）以及相应 DMA 窗口的总线偏移（bus offset），
@size 和 @offset 是 IOMMU 页的数量。

@flags 目前未被使用。

其余功能与 KVM_CREATE_SPAPR_TCE 相同。

### 4.99 KVM_REINJECT_CONTROL


:Capability: KVM_CAP_REINJECT_CONTROL
:Architectures: x86
:Type: vm ioctl
:Parameters: struct kvm_reinject_control (in)
:Returns: 0 on success,
         -EFAULT if struct kvm_reinject_control cannot be read,
         -ENXIO if KVM_CREATE_PIT or KVM_CREATE_PIT2 didn't succeed earlier.

i8254（PIT）有两种模式，reinject 和 !reinject。默认是 reinject，即 KVM 排队已流逝的 i8254
tick 并监控 i8254 注入的中断的完成。reinject 模式会在没有来自 i8254 的挂起中断时出队一个 tick
并注入其中断。!reinject 模式在 tick 到达时立即注入中断。

```

  struct kvm_reinject_control {
	__u8 pit_reinject;
	__u8 reserved[31];
  };

```
除非运行使用 PIT 进行定时的旧操作系统（例如 Linux 2.4.x），否则建议使用 pit_reinject = 0
（!reinject 模式）。

### 4.100 KVM_PPC_CONFIGURE_V3_MMU


:Capability: KVM_CAP_PPC_MMU_RADIX or KVM_CAP_PPC_MMU_HASH_V3
:Architectures: ppc
:Type: vm ioctl
:Parameters: struct kvm_ppc_mmuv3_cfg (in)
:Returns: 0 on success,
         -EFAULT if struct kvm_ppc_mmuv3_cfg cannot be read,
         -EINVAL if the configuration is invalid

该 ioctl 控制客户机是使用 radix 还是 HPT（哈希页表）转换，并设置指向客户机进程表的指针。

```

  struct kvm_ppc_mmuv3_cfg {
	__u64	flags;
	__u64	process_table;
  };

```
可以在 flags 中设置两个位：KVM_PPC_MMUV3_RADIX 和 KVM_PPC_MMUV3_GTSE。KVM_PPC_MMUV3_RADIX 如果
置位，则将客户机配置为使用 radix 树转换；如果清零，则使用 HPT 转换。KVM_PPC_MMUV3_GTSE 如果
置位且 KVM 允许，则将客户机配置为能够使用全局 TLB 和 SLB 失效指令；如果清零，客户机不得使用
这些指令。

process_table 字段指定客户机进程表的地址和大小，该表位于客户机空间中。该字段的格式为分区表项
（partition table entry）的第二个双字，如 Power ISA V3.00 第 III 册 5.7.6.1 节所定义。

### 4.101 KVM_PPC_GET_RMMU_INFO


:Capability: KVM_CAP_PPC_MMU_RADIX
:Architectures: ppc
:Type: vm ioctl
:Parameters: struct kvm_ppc_rmmu_info (out)
:Returns: 0 on success,
	 -EFAULT if struct kvm_ppc_rmmu_info cannot be written,
	 -EINVAL if no useful information can be returned

该 ioctl 返回一个结构体，其中包含两样东西：(a) 一个包含受支持的 radix 树几何布局的列表，以及
(b) 一个将页大小映射到 tlbie（TLB 失效条目）指令的 "AP"（实际页大小）字段的列表。

```

  struct kvm_ppc_rmmu_info {
	struct kvm_ppc_radix_geom {
		__u8	page_shift;
		__u8	level_bits[4];
		__u8	pad[3];
	}	geometries[8];
	__u32	ap_encodings[8];
  };

```
geometries[] 字段给出最多 8 种受支持的 radix 页表几何布局，以最小页大小以 2 为底的对数，以及
从 PTE 级到 PGD 级（按此顺序）树每一级索引的位数表示。任何未使用的条目在 page_shift 字段中为 0。

ap_encodings 给出受支持的页大小及其 AP 字段编码，以 AP 值位于高 3 位、页大小以 2 为底的对数
位于低 6 位进行编码。

### 4.102 KVM_PPC_RESIZE_HPT_PREPARE


:Capability: KVM_CAP_SPAPR_RESIZE_HPT
:Architectures: powerpc
:Type: vm ioctl
:Parameters: struct kvm_ppc_resize_hpt (in)
:Returns: 0 on successful completion,
	 >0 if a new HPT is being prepared, the value is an estimated
         number of milliseconds until preparation is complete,
         -EFAULT if struct kvm_reinject_control cannot be read,
	 -EINVAL if the supplied shift or flags are invalid,
	 -ENOMEM if unable to allocate the new HPT,

用于实现 PAPR 扩展，以在运行时调整客户机哈希页表（HPT）的大小。具体来说，它启动、停止或监视
为客户机准备一个新的潜在 HPT，实质上实现了 H_RESIZE_HPT_PREPARE hypercall。

```

  struct kvm_ppc_resize_hpt {
	__u64 flags;
	__u32 shift;
	__u32 pad;
  };

```
如果在客户机没有挂起的 HPT 时以 shift > 0 调用，这将开始准备一个新的、大小为 2^(shift) 字节的
挂起 HPT。然后它返回一个正整数，表示距离准备完成估计的毫秒数。

如果在存在挂起的 HPT 但其大小与参数中请求的不匹配时调用，则丢弃现有的挂起 HPT，并按上述方式
创建一个新的。

如果在存在请求大小的挂起 HPT 时调用，将：

  - 如果挂起 HPT 的准备已完成，返回 0
  - 如果挂起 HPT 的准备已失败，返回错误码，然后丢弃挂起的 HPT
  - 如果挂起 HPT 的准备仍在进行中，返回距离准备完成估计的毫秒数

如果以 shift == 0 调用，则丢弃任何当前挂起的 HPT 并返回 0（即取消任何正在进行的准备）。

flags 保留用于未来的扩展，目前设置 flags 中的任何位都将导致 -EINVAL。

通常这将使用相同的参数重复调用，直到它返回 <= 0。第一次调用将启动准备，后续调用将监视准备，
直到完成或失败。

### 4.103 KVM_PPC_RESIZE_HPT_COMMIT


:Capability: KVM_CAP_SPAPR_RESIZE_HPT
:Architectures: powerpc
:Type: vm ioctl
:Parameters: struct kvm_ppc_resize_hpt (in)
:Returns: 0 on successful completion,
         -EFAULT if struct kvm_reinject_control cannot be read,
	 -EINVAL if the supplied shift or flags are invalid,
	 -ENXIO is there is no pending HPT, or the pending HPT doesn't
         have the requested size,
	 -EBUSY if the pending HPT is not fully prepared,
	 -ENOSPC if there was a hash collision when moving existing
         HPT entries to the new HPT,
	 -EIO on other error conditions

用于实现 PAPR 扩展，以在运行时调整客户机哈希页表（HPT）的大小。具体来说，它请求将客户机转移到
使用新的 HPT 工作，实质上实现了 H_RESIZE_HPT_COMMIT hypercall。

```

  struct kvm_ppc_resize_hpt {
	__u64 flags;
	__u32 shift;
	__u32 pad;
  };

```
这只应在 KVM_PPC_RESIZE_HPT_PREPARE 以相同参数返回 0 之后调用。在其他情况下，
KVM_PPC_RESIZE_HPT_COMMIT 将返回错误（通常是 -ENXIO 或 -EBUSY，但如果准备已开始但失败了，
也可能返回其他错误）。

如果客户机尚未使自己处于静止（quiescent）状态（即没有 vcpu 会进行启用 MMU 的内存访问），这
对客户机的影响将是未定义的。

成功完成后，挂起的 HPT 将成为客户机的活动 HPT，而先前的 HPT 将被丢弃。

失败时，客户机仍将在其先前的 HPT 上运行。

### 4.104 KVM_X86_GET_MCE_CAP_SUPPORTED


:Capability: KVM_CAP_MCE
:Architectures: x86
:Type: system ioctl
:Parameters: u64 mce_cap (out)
:Returns: 0 on success, -1 on error

返回受支持的 MCE 能力。u64 mce_cap 参数与 MSR_IA32_MCG_CAP 寄存器具有相同的格式。受支持的
能力会将其相应的位置位。
### 4.105 KVM_X86_SETUP_MCE


:Capability: KVM_CAP_MCE
:Architectures: x86
:Type: vcpu ioctl
:Parameters: u64 mcg_cap (in)
:Returns: 0 on success,
         -EFAULT if u64 mcg_cap cannot be read,
         -EINVAL if the requested number of banks is invalid,
         -EINVAL if requested MCE capability is not supported.

初始化以供使用的 MCE 支持。u64 mcg_cap 参数与 MSR_IA32_MCG_CAP 寄存器具有相同的格式，并指定
应启用哪些能力。受支持的最大错误报告（error-reporting）bank 数量可以在检查 KVM_CAP_MCE 时获取。
受支持的能力可以通过 KVM_X86_GET_MCE_CAP_SUPPORTED 获取。

### 4.106 KVM_X86_SET_MCE


:Capability: KVM_CAP_MCE
:Architectures: x86
:Type: vcpu ioctl
:Parameters: struct kvm_x86_mce (in)
:Returns: 0 on success,
         -EFAULT if struct kvm_x86_mce cannot be read,
         -EINVAL if the bank number is invalid,
         -EINVAL if VAL bit is not set in status field.

向客户机注入一个机器检查错误（MCE）。输入
```

  struct kvm_x86_mce {
	__u64 status;
	__u64 addr;
	__u64 misc;
	__u64 mcg_status;
	__u8 bank;
	__u8 pad1[7];
	__u64 pad2[3];
  };

```
如果报告的 MCE 是一个未纠正的错误（uncorrected error），KVM 会将其作为 MCE 异常注入客户机。如果
客户机 MCG_STATUS 寄存器报告 MCE 正在进行中，KVM 会导致一个 KVM_EXIT_SHUTDOWN vmexit。

否则，如果 MCE 是一个已纠正的错误（corrected error），KVM 只会将其存储在相应的 bank 中（前提
是该 bank 没有持有一个先前报告的未纠正错误）。

### 4.107 KVM_S390_GET_CMMA_BITS


:Capability: KVM_CAP_S390_CMMA_MIGRATION
:Architectures: s390
:Type: vm ioctl
:Parameters: struct kvm_s390_cmma_log (in, out)
:Returns: 0 on success, a negative value on error

错误码：

  ======     =============================================================
  ENOMEM     无法分配足够的内存来完成任务
  ENXIO      如果 CMMA 未启用
  EINVAL     如果未设置 KVM_S390_CMMA_PEEK 但迁移模式未启用
  EINVAL     如果未设置 KVM_S390_CMMA_PEEK 但脏页跟踪已被禁用
             （因此迁移模式被自动禁用）
  EFAULT     如果用户空间地址无效，或地址没有对应的页表
             （例如使用大页时）。
  ======     =============================================================

该 ioctl 用于在 s390 架构上获取 CMMA 位的值。它适用于两种场景：

- 在实时迁移期间保存 CMMA 值。实时迁移需要通过 KVM_REQ_START_MIGRATION VM 属性启用。
- 通过设置了标志 KVM_S390_CMMA_PEEK 来非破坏性地查看 CMMA 值。

该 ioctl 通过 kvm_s390_cmma_log 结构体接收参数。所需的值被写入一个缓冲区，其位置通过
kvm_s390_cmma_log 结构体中的 "values" 成员指示。输入结构体中的值也会根据需要更新。

每个 CMMA 值占用一个字节。

```

  struct kvm_s390_cmma_log {
	__u64 start_gfn;
	__u32 count;
	__u32 flags;
	union {
		__u64 remaining;
		__u64 mask;
	};
	__u64 values;
  };

```
start_gfn 是要获取其 CMMA 值的第一个客户机帧的编号，

count 是缓冲区长度的字节数，

values 指向将结果写入其中的缓冲区。

如果 count 大于 KVM_S390_SKEYS_MAX，则被视为 KVM_S390_SKEYS_MAX。为了与其他 ioctl 保持一致，
复用 KVM_S390_SKEYS_MAX。

结果被写入 values 字段指向的缓冲区中，并且输入参数的值按如下方式更新。

根据标志的不同，会执行不同的操作。到目前为止唯一受支持的标志是 KVM_S390_CMMA_PEEK。

如果未设置 KVM_S390_CMMA_PEEK，默认行为是：
start_gfn 将指示其 CMMA 位为脏的第一个页帧。它不一定与作为输入传入的相同，因为会跳过干净页。

count 将指示缓冲区中实际写入的字节数。它（而且往往）会小于输入值，因为缓冲区只填充到找到 16 字节
干净值为止（这些值随后不会被复制到缓冲区中）。由于一个 CMMA 迁移块需要基地址和长度，总共 16 字节，
所以只要干净数据的大小不超过头部的大小，我们就会在后面有一些脏数据的情况下发回一些干净数据。这
允许以更多地往返用户空间为代价，最小化要保存或通过网络传输的数据量。ioctl 的下一次调用将跳过所有
干净值，可能节省的不仅仅是找到的 16 字节。

如果设置了 KVM_S390_CMMA_PEEK：
即使不在迁移模式下，也会读取现有的存储属性，并且不执行其他操作；

输出的 start_gfn 将等于输入的 start_gfn，

输出的 count 将等于输入的 count，除非已到达内存末尾。

在这两种情况下：
"remaining" 字段将指示仍然剩余的脏 CMMA 值的总数，或者如果设置了 KVM_S390_CMMA_PEEK 且未启用
迁移模式则为 0。

mask 未被使用。

values 指向将存储结果的用户空间缓冲区。

### 4.108 KVM_S390_SET_CMMA_BITS


:Capability: KVM_CAP_S390_CMMA_MIGRATION
:Architectures: s390
:Type: vm ioctl
:Parameters: struct kvm_s390_cmma_log (in)
:Returns: 0 on success, a negative value on error

该 ioctl 用于在 s390 架构上设置 CMMA 位的值。它旨在实时迁移期间用于恢复 CMMA 值，但其使用没有
限制。该 ioctl 通过 kvm_s390_cmma_values 结构体接收参数。每个 CMMA 值占用一个字节。

```

  struct kvm_s390_cmma_log {
	__u64 start_gfn;
	__u32 count;
	__u32 flags;
	union {
		__u64 remaining;
		__u64 mask;
 	};
	__u64 values;
  };

```
start_gfn 指示起始的客户机帧编号，

count 指示缓冲区中要考虑多少个值，

flags 未被使用，必须为 0。

mask 指示要考虑哪些 PGSTE 位。

remaining 未被使用。

values 指向用户空间中存储这些值的缓冲区。

如果无法分配足够的内存来完成任务，该 ioctl 可能以 -ENOMEM 失败；如果 CMMA 未启用，以 -ENXIO
失败；如果 count 字段过大（例如超过 KVM_S390_CMMA_SIZE_MAX）或 flags 字段不为 0，以 -EINVAL
失败；如果用户空间地址无效、写入了无效页（例如内存末尾之后）或地址没有对应的页表（例如使用大页时），
以 -EFAULT 失败。

### 4.109 KVM_PPC_GET_CPU_CHAR


:Capability: KVM_CAP_PPC_GET_CPU_CHAR
:Architectures: powerpc
:Type: vm ioctl
:Parameters: struct kvm_ppc_cpu_char (out)
:Returns: 0 on successful completion,
	 -EFAULT if struct kvm_ppc_cpu_char cannot be written

该 ioctl 向用户空间提供有关 CPU 某些特性的信息，这些特性与指令的推测执行以及推测执行可能导致的
信息泄漏有关（参见 CVE-2017-5715、CVE-2017-5753 和 CVE-2017-5754）。信息位于
```

  struct kvm_ppc_cpu_char {
	__u64	character;		/* characteristics of the CPU */
	__u64	behaviour;		/* recommended software behaviour */
	__u64	character_mask;		/* valid bits in character */
	__u64	behaviour_mask;		/* valid bits in behaviour */
  };

```
为了可扩展性，character_mask 和 behaviour_mask 字段指示 character 和 behaviour 中的哪些位已由
内核填充。如果将来定义的位集合被扩展，用户空间将能够判断它是否运行在知晓新位的内核上。

character 字段描述有助于防止无意信息泄露的 CPU 属性 —— 具体来说，是否存在用于刷新失效（flash-invalidate）
L1 数据缓存的指令（ori 30,30,0 或 mtspr SPRN_TRIG2,rN），L1 数据缓存是否设置为一种模式（其中
条目只能由创建它们的线程使用），bcctr[l] 指令是否能防止推测执行，以及是否提供推测屏障指令
（ori 31,31,0）。

behaviour 字段描述软件为防止无意信息泄露而应采取的操作，从而描述硬件受哪些漏洞影响；具体来说，
从内核返回用户模式时是否应刷新 L1 数据缓存，以及是否应在数组边界检查和数组访问之间放置推测屏障。

这些字段使用与新的 H_GET_CPU_CHARACTERISTICS hypercall 相同的位定义。

### 4.110 KVM_MEMORY_ENCRYPT_OP


:Capability: basic
:Architectures: x86
:Type: vm ioctl, vcpu ioctl
:Parameters: an opaque platform specific structure (in/out)
:Returns: 0 on success; -1 on error

如果平台支持创建加密的 VM，则可以使用此 ioctl 发出特定于平台的、用于管理这些加密 VM 的内存加密
命令。

目前，此 ioctl 用于发出 AMD 处理器上的安全加密虚拟化（SEV）命令和 Intel 处理器上的信任域扩展
（TDX）命令。详细的命令定义在 Documentation/virt/kvm/x86/amd-memory-encryption.rst 和
Documentation/virt/kvm/x86/intel-tdx.rst 中。

### 4.111 KVM_MEMORY_ENCRYPT_REG_REGION


:Capability: basic
:Architectures: x86
:Type: system
:Parameters: struct kvm_enc_region (in)
:Returns: 0 on success; -1 on error

该 ioctl 可用于注册一个可能包含加密数据的客户机内存区域（例如客户机 RAM、SMRAM 等）。

它用于启用 SEV 的客户机中。当启用加密时，客户机内存区域可能包含加密数据。SEV 内存加密引擎使用
一种调整（tweak）机制，使得两个相同的明文页，即使位于不同位置，也会具有不同的密文。因此交换或
移动这些页的密文不会导致明文被交换。因此，为 SEV 客户机重定位（或迁移）物理后备页将需要一些额外
的步骤。

注意：当前的 SEV 密钥管理规范没有提供交换或迁移（移动）密文页的命令。因此，目前我们固定（pin）
通过此 ioctl 注册的客户机内存区域。

### 4.112 KVM_MEMORY_ENCRYPT_UNREG_REGION


:Capability: basic
:Architectures: x86
:Type: system
:Parameters: struct kvm_enc_region (in)
:Returns: 0 on success; -1 on error

该 ioctl 可用于注销上述通过 KVM_MEMORY_ENCRYPT_REG_REGION ioctl 注册的客户机内存区域。

### 4.113 KVM_HYPERV_EVENTFD


:Capability: KVM_CAP_HYPERV_EVENTFD
:Architectures: x86
:Type: vm ioctl
:Parameters: struct kvm_hyperv_eventfd (in)

该 ioctl（注销）注册一个 eventfd，以通过 SIGNAL_EVENT hypercall 从客户机接收关于指定 Hyper-V
连接 id 的通知，而不会导致用户退出。带有非零事件标志号（位 24-31）的 SIGNAL_EVENT hypercall 仍会
触发 KVM_EXIT_HYPERV_HCALL 用户退出。

```

  struct kvm_hyperv_eventfd {
	__u32 conn_id;
	__s32 fd;
	__u32 flags;
	__u32 padding[3];
  };

```
```

  #define KVM_HYPERV_CONN_ID_MASK		0x00ffffff

```
```

  #define KVM_HYPERV_EVENTFD_DEASSIGN	(1 << 0)

```
:Returns: 0 on success,
 	  -EINVAL if conn_id or flags is outside the allowed range,
	  -ENOENT on deassign if the conn_id isn't registered,
	  -EEXIST on assign if the conn_id is already registered

### 4.114 KVM_GET_NESTED_STATE


:Capability: KVM_CAP_NESTED_STATE
:Architectures: x86
:Type: vcpu ioctl
:Parameters: struct kvm_nested_state (in/out)
:Returns: 0 on success, -1 on error

错误码：

  =====      =============================================================
  E2BIG     总状态大小超过了用户指定的 'size' 值；所需的大小将被写入 size。
  =====      =============================================================

```

  struct kvm_nested_state {
	__u16 flags;
	__u16 format;
	__u32 size;

	union {
		struct kvm_vmx_nested_state_hdr vmx;
		struct kvm_svm_nested_state_hdr svm;

		/* Pad the header to 128 bytes.  */
		__u8 pad[120];
	} hdr;

	union {
		struct kvm_vmx_nested_state_data vmx[0];
		struct kvm_svm_nested_state_data svm[0];
	} data;
  };

  #define KVM_STATE_NESTED_GUEST_MODE		0x00000001
  #define KVM_STATE_NESTED_RUN_PENDING		0x00000002
  #define KVM_STATE_NESTED_EVMCS		0x00000004

  #define KVM_STATE_NESTED_FORMAT_VMX		0
  #define KVM_STATE_NESTED_FORMAT_SVM		1

  #define KVM_STATE_NESTED_VMX_VMCS_SIZE	0x1000

  #define KVM_STATE_NESTED_VMX_SMM_GUEST_MODE	0x00000001
  #define KVM_STATE_NESTED_VMX_SMM_VMXON	0x00000002

  #define KVM_STATE_VMX_PREEMPTION_TIMER_DEADLINE 0x00000001

  struct kvm_vmx_nested_state_hdr {
	__u64 vmxon_pa;
	__u64 vmcs12_pa;

	struct {
		__u16 flags;
	} smm;

	__u32 flags;
	__u64 preemption_timer_deadline;
  };

  struct kvm_vmx_nested_state_data {
	__u8 vmcs12[KVM_STATE_NESTED_VMX_VMCS_SIZE];
	__u8 shadow_vmcs12[KVM_STATE_NESTED_VMX_VMCS_SIZE];
  };

```
该 ioctl 将 vcpu 的嵌套虚拟化状态从内核复制到用户空间。

状态的最大大小可以通过向 KVM_CHECK_EXTENSION ioctl() 传入 KVM_CAP_NESTED_STATE 获取。

### 4.115 KVM_SET_NESTED_STATE


:Capability: KVM_CAP_NESTED_STATE
:Architectures: x86
:Type: vcpu ioctl
:Parameters: struct kvm_nested_state (in)
:Returns: 0 on success, -1 on error

这将 vcpu 的 kvm_nested_state 结构体从用户空间复制到内核。关于 struct kvm_nested_state 的定义，
请参见 KVM_GET_NESTED_STATE。

### 4.116 KVM_(UN)REGISTER_COALESCED_MMIO


:Capability: KVM_CAP_COALESCED_MMIO (for coalesced mmio)
	     KVM_CAP_COALESCED_PIO (for coalesced pio)
:Architectures: all
:Type: vm ioctl
:Parameters: struct kvm_coalesced_mmio_zone
:Returns: 0 on success, < 0 on error

合并 I/O（Coalesced I/O）是一种性能优化，它推迟硬件寄存器写入的模拟，从而避免用户空间退出。它
通常用于减少模拟频繁访问的硬件寄存器的开销。

当硬件寄存器被配置为合并 I/O 时，写访问不会退出到用户空间，其值被记录在一个内核与用户空间之间
共享的环形缓冲区中。

如果对硬件寄存器的一次或多次写访问可以推迟到对同一设备上另一个硬件寄存器的读或写，则使用合并
I/O。最后一次访问将导致 vmexit，用户空间将在模拟它之前处理来自环形缓冲区的访问。这将避免在重复
写入时退出到用户空间。

合并 pio 基于合并 mmio。合并 mmio 与合并 pio 之间几乎没有区别，只是合并 pio 记录对 I/O 端口的
访问。

### 4.117 KVM_CLEAR_DIRTY_LOG


:Capability: KVM_CAP_MANUAL_DIRTY_LOG_PROTECT2
:Architectures: x86, arm64, mips
:Type: vm ioctl
:Parameters: struct kvm_clear_dirty_log (in)
:Returns: 0 on success, -1 on error

```

  /* for KVM_CLEAR_DIRTY_LOG */
  struct kvm_clear_dirty_log {
	__u32 slot;
	__u32 num_pages;
	__u64 first_page;
	union {
		void __user *dirty_bitmap; /* one bit per page */
		__u64 padding;
	};
  };

```
该 ioctl 根据 struct kvm_clear_dirty_log 的 dirty_bitmap 字段中传入的位图，清除内存槽中页的
脏状态。位图的位 0 对应于内存槽中的页 "first_page"，num_pages 是输入位图的大小（以位为单位）。
first_page 必须是 64 的倍数；除非 first_page + num_pages 等于内存槽的大小，否则 num_pages 也
必须是 64 的倍数。对于输入位图中每个被置位的位，相应的页在 KVM 的脏位图中被标记为"干净"，并且
为该页重新启用脏页跟踪（例如通过写保护，或清除页表项中的脏位）。

如果 KVM_CAP_MULTI_ADDRESS_SPACE 可用，slot 字段的 16-31 位指定要清除脏状态的地址空间。关于
slot 字段的用法细节，请参见 KVM_SET_USER_MEMORY_REGION。

当启用了 KVM_CAP_MANUAL_DIRTY_LOG_PROTECT2 时，此 ioctl 最有用；更多信息请参见该能力的描述。
但是，只要 KVM_CHECK_EXTENSION 确认 KVM_CAP_MANUAL_DIRTY_LOG_PROTECT2 存在，它就可以始终被使用。

### 4.118 KVM_GET_SUPPORTED_HV_CPUID


:Capability: KVM_CAP_HYPERV_CPUID (vcpu), KVM_CAP_SYS_HYPERV_CPUID (system)
:Architectures: x86
:Type: system ioctl, vcpu ioctl
:Parameters: struct kvm_cpuid2 (in/out)
:Returns: 0 on success, -1 on error

```

  struct kvm_cpuid2 {
	__u32 nent;
	__u32 padding;
	struct kvm_cpuid_entry2 entries[0];
  };

  struct kvm_cpuid_entry2 {
	__u32 function;
	__u32 index;
	__u32 flags;
	__u32 eax;
	__u32 ebx;
	__u32 ecx;
	__u32 edx;
	__u32 padding[3];
  };

```
该 ioctl 返回 KVM 中与 Hyper-V 模拟相关的 x86 cpuid 特性叶子（leaf）。用户空间可以使用该 ioctl
返回的信息来构造呈现给使用 Hyper-V 增强（enlightenment）的客户机（例如 Windows 或 Hyper-V 客户机）
的 cpuid 信息。

此 ioctl 返回的 CPUID 特性叶子由 Hyper-V 顶层功能规范（TLFS）定义。这些叶子无法通过
KVM_GET_SUPPORTED_CPUID ioctl 获取，因为其中一些与 KVM 特性叶子（0x40000000、0x40000001）相交。

目前，返回以下 CPUID 叶子列表：

 - HYPERV_CPUID_VENDOR_AND_MAX_FUNCTIONS
 - HYPERV_CPUID_INTERFACE
 - HYPERV_CPUID_VERSION
 - HYPERV_CPUID_FEATURES
 - HYPERV_CPUID_ENLIGHTMENT_INFO
 - HYPERV_CPUID_IMPLEMENT_LIMITS
 - HYPERV_CPUID_NESTED_FEATURES
 - HYPERV_CPUID_SYNDBG_VENDOR_AND_MAX_FUNCTIONS
 - HYPERV_CPUID_SYNDBG_INTERFACE
 - HYPERV_CPUID_SYNDBG_PLATFORM_CAPABILITIES

用户空间通过传入一个 kvm_cpuid2 结构体来调用 KVM_GET_SUPPORTED_HV_CPUID，其中 'nent' 字段指示
可变长数组 'entries' 中的条目数量。如果条目数量太少而无法描述所有 Hyper-V 特性叶子，会返回错误
（E2BIG）。如果数量大于或等于 Hyper-V 特性叶子的数量，'nent' 字段会被调整为 'entries' 数组中
有效条目的数量，并随后被填充。

'struct kvm_cpuid_entry2' 中的 'index' 和 'flags' 字段目前保留，用户空间不应期望在那里获得任何
特定值。

注意，KVM_GET_SUPPORTED_HV_CPUID 的 vcpu 版本目前已被废弃。与无条件暴露所有受支持特性位的系统
ioctl 不同，vcpu 版本有以下怪异之处：

- HYPERV_CPUID_NESTED_FEATURES 叶子和 HV_X64_ENLIGHTENED_VMCS_RECOMMENDED 特性位仅在相应的
  vCPU 先前启用了 Enlightened VMCS（KVM_CAP_HYPERV_ENLIGHTENED_VMCS）时才会暴露。
- HV_STIMER_DIRECT_MODE_AVAILABLE 位仅在具有内核态 LAPIC 时才暴露。
  （假定已调用 KVM_CREATE_IRQCHIP。）

### 4.119 KVM_ARM_VCPU_FINALIZE


:Architectures: arm64
:Type: vcpu ioctl
:Parameters: int feature (in)
:Returns: 0 on success, -1 on error

错误码：

  ======     ==============================================================
  EPERM      特性未启用、需要配置，或已经定稿
  EINVAL     特性未知或不存在
  ======     ==============================================================

feature 的已识别值：

  =====      ===========================================
  arm64      KVM_ARM_VCPU_SVE (requires KVM_CAP_ARM_SVE)
  =====      ===========================================

定稿（finalize）指定 vcpu 特性的配置。

vcpu 必须已经通过一次成功的 KVM_ARM_VCPU_INIT <KVM_ARM_VCPU_INIT> 调用（在 features[] 中设置了
相应的标志）完成了初始化，启用了受影响的特性。

对于受影响的 vcpu 特性，这是在 vcpu 完全可用之前必须执行的强制性步骤。

在 KVM_ARM_VCPU_INIT 和 KVM_ARM_VCPU_FINALIZE 之间，可以通过使用诸如 KVM_SET_ONE_REG 之类的
ioctl 来配置该特性。应执行的确切配置以及如何执行是特性相关的。

其他依赖于特定特性被定稿的调用，例如 KVM_RUN、KVM_GET_REG_LIST、KVM_GET_ONE_REG 和
KVM_SET_ONE_REG，除非该特性已经通过 KVM_ARM_VCPU_FINALIZE 调用定稿，否则将以 -EPERM 失败。

需要使用此 ioctl 定稿的 vcpu 特性的细节，请参见 KVM_ARM_VCPU_INIT。

### 4.120 KVM_SET_PMU_EVENT_FILTER


:Capability: KVM_CAP_PMU_EVENT_FILTER
:Architectures: x86
:Type: vm ioctl
:Parameters: struct kvm_pmu_event_filter (in)
:Returns: 0 on success, -1 on error

错误码：

  ======     ============================================================
  EFAULT     args[^0^] cannot be accessed
  EINVAL     args[^0^] contains invalid data in the filter or filter events
  E2BIG      nevents is too large
  EBUSY      not enough memory to allocate the filter
  ======     ============================================================

```

  struct kvm_pmu_event_filter {
	__u32 action;
	__u32 nevents;
	__u32 fixed_counter_bitmap;
	__u32 flags;
	__u32 pad[4];
	__u64 events[0];
  };

```
该 ioctl 通过限制允许的 event select 和 unit mask 组合，来限制客户机可以编程的 PMU 事件集合。

参数持有一个将被允许或拒绝的过滤事件列表。

过滤事件只控制通用计数器；固定用途计数器由 fixed_counter_bitmap 控制。

```

```
`0`

要使用此模式，请清空 'flags' 字段。

在此模式下，每个事件将包含一个 event select + unit mask。

当客户机尝试编程 PMU 时，客户机的 event select + unit mask 会与过滤事件进行比较，以确定客户机
是否应具有访问权限。

`KVM_PMU_EVENT_FLAG_MASKED_EVENTS`
:Capability: KVM_CAP_PMU_EVENT_MASKED_EVENTS

在此模式下，每个过滤事件将包含一个 event select、mask、match 和
```

  KVM_PMU_ENCODE_MASKED_ENTRY()

```
```

  Bits   Description
  ----   -----------
  7:0    event select (low bits)
  15:8   umask match
  31:16  unused
  35:32  event select (high bits)
  36:54  unused
  55     exclude bit
  63:56  umask mask

```
当客户机尝试编程 PMU 时，按以下步骤确定客户机是否应具有访问权限：

 1. 将客户机的 event select 与过滤事件进行匹配。
 2. 如果找到匹配，将客户机的 unit mask 与所包含过滤事件的 mask 和 match 值进行匹配。
    I.e. (unit mask & mask) == match && !exclude。
 3. 如果找到匹配，将客户机的 unit mask 与所排除过滤事件的 mask 和 match 值进行匹配。
    I.e. (unit mask & mask) == match && exclude。
 4.
   a. 如果找到包含匹配且未找到排除匹配，则过滤该事件。
   b. 对于所有其他情况，不过滤该事件。
 5.
   a. 如果事件被过滤且它是允许列表，则允许客户机编程该事件。
   b. 如果事件被过滤且它是拒绝列表，则不允许客户机编程该事件。

设置新的 pmu 事件过滤器时，如果设置了任何未使用字段，或者在 Intel 上调用时设置了 event select
中的任何高位（35:32），将返回 -EINVAL。

```

  #define KVM_PMU_EVENT_ALLOW 0
  #define KVM_PMU_EVENT_DENY 1

```
通过此 API，KVM 用户空间还可以通过配置 "action" 和 "fixed_counter_bitmap" 字段来控制 VM 的
固定计数器的行为（如果有）。

具体来说，KVM 在确定是否
```

  FixCtr[i]_is_allowed = (action == ALLOW) && (bitmap & BIT(i)) ||
    (action == DENY) && !(bitmap & BIT(i));
  FixCtr[i]_is_denied = !FixCtr[i]_is_allowed;

```
KVM 总是使用 fixed_counter_bitmap，确保 fixed_counter_bitmap 设置正确是用户空间的责任，例如，如果
用户空间想要定义一个只影响通用计数器的过滤器。

注意，"events" 字段也适用于固定计数器的硬编码 event_select 和 unit_mask 值。"fixed_counter_bitmap"
的优先级高于 "events"，如果两者之间存在矛盾。

### 4.121 KVM_PPC_SVM_OFF


:Capability: basic
:Architectures: powerpc
:Type: vm ioctl
:Parameters: none
:Returns: 0 on successful completion,

错误码：

  ======     ================================================================
  EINVAL     如果 ultravisor 未能终止安全客户机
  ENOMEM     如果 hypervisor 未能为客户机分配新的 radix 页表
  ======     ================================================================

该 ioctl 用于关闭客户机的安全模式，或将客户机从安全模式转换到正常模式。这在客户机被重置时调用。
如果针对正常客户机调用，则没有效果。

该 ioctl 发出一个 ultravisor 调用来终止安全客户机，解除 VPA 页的固定，并释放所有由 hypervisor
用于跟踪安全页的设备页。

### 4.122 KVM_S390_NORMAL_RESET


:Capability: KVM_CAP_S390_VCPU_RESETS
:Architectures: s390
:Type: vcpu ioctl
:Parameters: none
:Returns: 0

该 ioctl 根据 POP（Principles Of Operation，操作原理）中的 cpu 重置定义重置 VCPU 寄存器和控制结构。

### 4.123 KVM_S390_INITIAL_RESET


:Capability: basic
:Architectures: s390
:Type: vcpu ioctl
:Parameters: none
:Returns: 0

该 ioctl 根据 POP 中的初始 cpu 重置定义重置 VCPU 寄存器和控制结构。但是，cpu 不会被置于 ESA 模式。
此重置是正常重置的超集。

### 4.124 KVM_S390_CLEAR_RESET


:Capability: KVM_CAP_S390_VCPU_RESETS
:Architectures: s390
:Type: vcpu ioctl
:Parameters: none
:Returns: 0

该 ioctl 根据 POP 中的清除 cpu 重置定义重置 VCPU 寄存器和控制结构。但是，cpu 不会被置于 ESA 模式。
此重置是初始重置的超集。


### 4.125 KVM_S390_PV_COMMAND


:Capability: KVM_CAP_S390_PROTECTED
:Architectures: s390
:Type: vm ioctl
:Parameters: struct kvm_pv_cmd
:Returns: 0 on success, < 0 on error

```

  struct kvm_pv_cmd {
	__u32 cmd;	/* Command to be executed */
	__u16 rc;	/* Ultravisor return code */
	__u16 rrc;	/* Ultravisor return reason code */
	__u64 data;	/* Data or address */
	__u32 flags;    /* flags for future extensions. Must be 0 for now */
	__u32 reserved[3];
  };

```
**Ultravisor 返回码（Ultravisor return codes）**
如果为了实现命令预期的结果而执行了 Ultravisor 调用，则由内核提供 Ultravisor 返回（原因）码。因此
它们与 IOCTL 返回码无关。如果 KVM 改变了 `rc`，其值将始终大于 0，因此建议在发出 PV 命令之前将其
设置为 0，以便能够检测到 `rc` 的变化。

**cmd 值：**

KVM_PV_ENABLE
  分配内存并将 VM 注册到 Ultravisor，从而将内存捐赠给 Ultravisor，使其成为 KVM 不可访问的。
  所有现有的 CPU 都被转换为受保护的 CPU。在此命令成功之后，任何通过热插拔添加的 CPU 在创建时
  也会变成受保护的。

  错误码：

  =====      =============================
  EINTR      存在未屏蔽的挂起信号
  =====      =============================

KVM_PV_DISABLE
  从 Ultravisor 注销 VM，并回收捐赠给 Ultravisor 的内存，使其重新可被内核使用。所有注册的 VCPU
  都被转换回非受保护的。如果先前的一个受保护 VM 已经通过 KVM_PV_ASYNC_CLEANUP_PREPARE 准备好进行
  异步拆除，并且随后没有通过 KVM_PV_ASYNC_CLEANUP_PERFORM 拆除，则它将在本次调用中与当前的受保护
  VM 一起被拆除。

KVM_PV_VM_SET_SEC_PARMS
  将镜像头从 VM 内存传递给 Ultravisor，以准备镜像的解包和验证。

KVM_PV_VM_UNPACK
  解包（保护和解密）加密启动镜像的一页。

KVM_PV_VM_VERIFY
  验证解包镜像的完整性。只有它成功，才允许 KVM 启动受保护的 VCPU。

KVM_PV_INFO
  :Capability: KVM_CAP_S390_PROTECTED_DUMP

  提供一个 API，通过子命令向用户空间提供 Ultravisor 相关数据。len_max 是用户空间缓冲区的大小，
  len_written 是 KVM 指示实际写入该缓冲区的字节数。如果将来添加更多响应字段，len_written 可用于
  确定有效字段。

```

     enum pv_cmd_info_id {
	KVM_PV_INFO_VM,
	KVM_PV_INFO_DUMP,
     };

     struct kvm_s390_pv_info_header {
	__u32 id;
	__u32 len_max;
	__u32 len_written;
	__u32 reserved;
     };

     struct kvm_s390_pv_info {
	struct kvm_s390_pv_info_header header;
	struct kvm_s390_pv_info_dump dump;
	struct kvm_s390_pv_info_vm vm;
     };

```
**子命令：**

  KVM_PV_INFO_VM
    此子命令为 PV 宿主机提供基本的 Ultravisor 信息。这些值也可能作为文件导出在 sysfs 固件 UV
    查询接口中，但在此 API 中程序更容易获取。

    inst_calls 和 feature_indication 成员提供已安装的 UV 调用和 UV 的其他特性指示。

    max_* 成员提供关于 PV vCPU、PV 客户机和 PV 客户机内存大小最大值的信息。

```

      struct kvm_s390_pv_info_vm {
	__u64 inst_calls_list[4];
	__u64 max_cpus;
	__u64 max_guests;
	__u64 max_guest_addr;
	__u64 feature_indication;
      };


  KVM_PV_INFO_DUMP
    此子命令提供与转储 PV 客户机相关的信息。

    ::

      struct kvm_s390_pv_info_dump {
	__u64 dump_cpu_buffer_len;
	__u64 dump_config_mem_buffer_per_1m;
	__u64 dump_config_finalize_len;
      };

```
KVM_PV_DUMP
  :Capability: KVM_CAP_S390_PROTECTED_DUMP

  提供一个 API，提供有助于转储受保护 VM 的调用。

```

    struct kvm_s390_pv_dmp {
      __u64 subcmd;
      __u64 buff_addr;
      __u64 buff_len;
      __u64 gaddr;		/* For dump storage state */
    };

  **子命令：**

  KVM_PV_DUMP_INIT
    初始化受保护 VM 的转储过程。如果此调用不成功，所有其他子命令将以 -EINVAL 失败。如果
    转储过程尚未完成，此子命令将返回 -EINVAL。

    并非所有 PV vm 都可以被转储，所有者需要在 SE 头中设置 `dump allowed` PCF 位 34 以允许转储。

  KVM_PV_DUMP_CONFIG_STOR_STATE
     存储 `buff_len` 字节的调整（tweak）组件值，从绝对客户机地址（`gaddr`）指定的 1MB 块开始。
     `buff_len` 需要与 `conf_dump_storage_state_len` 对齐，且至少 >= dump uv_info 数据提供的
     `conf_dump_storage_state_len` 值。即使返回了错误 rc，buff_user 也可能被写入。例如，如果我们
     在写入第一页数据后遇到缺页。

  KVM_PV_DUMP_COMPLETE
    如果子命令成功，它将完成转储过程，并允许再次调用 KVM_PV_DUMP_INIT。

    成功时，`conf_dump_finalize_len` 字节的完成数据将被存储到 `buff_addr`。完成数据包含密钥派生
    种子、IV、调整随机数和加密密钥，以及认证标签，所有这些都需要在以后解密转储时使用。

```
KVM_PV_ASYNC_CLEANUP_PREPARE
  :Capability: KVM_CAP_S390_PROTECTED_ASYNC_DISABLE

  为当前的受保护 VM 准备异步拆除。当前受保护 VM 使用的大多数资源将被搁置，以供后续异步拆除。当前
  受保护 VM 随后将立即作为非受保护的 VM 恢复执行。任何时刻最多只能有一个受保护 VM 被准备好进行
  异步拆除。如果某个受保护 VM 已经准备好拆除，而没有随后调用 KVM_PV_ASYNC_CLEANUP_PERFORM，则此
  调用将失败。在这种情况下，用户空间进程应发出一个正常的 KVM_PV_DISABLE。通过此调用搁置的资源
  需要通过后续调用 KVM_PV_ASYNC_CLEANUP_PERFORM 或 KVM_PV_DISABLE 来清理，否则它们将在 KVM 终止
  时被清理。一旦清理开始，即 KVM_PV_ASYNC_CLEANUP_PERFORM 完成之前，就可以再次调用
  KVM_PV_ASYNC_CLEANUP_PREPARE。

KVM_PV_ASYNC_CLEANUP_PERFORM
  :Capability: KVM_CAP_S390_PROTECTED_ASYNC_DISABLE

  拆除先前通过 KVM_PV_ASYNC_CLEANUP_PREPARE 准备好拆除的受保护 VM。搁置的资源将在此命令执行期间
  被释放。此 PV 命令理想情况下应由用户空间从单独的线程发出。如果收到致命信号（或进程自然终止），
  该命令将立即终止而不完成，正常的 KVM 关闭过程将负责清理所有剩余的受保护 VM，包括那些拆除被
  进程终止中断的 VM。

### 4.126 KVM_XEN_HVM_SET_ATTR


:Capability: KVM_CAP_XEN_HVM / KVM_XEN_HVM_CONFIG_SHARED_INFO
:Architectures: x86
:Type: vm ioctl
:Parameters: struct kvm_xen_hvm_attr
:Returns: 0 on success, < 0 on error

```

  struct kvm_xen_hvm_attr {
	__u16 type;
	__u16 pad[3];
	union {
		__u8 long_mode;
		__u8 vector;
		__u8 runstate_update_flag;
		union {
			__u64 gfn;
			__u64 hva;
		} shared_info;
		struct {
			__u32 send_port;
			__u32 type; /* EVTCHNSTAT_ipi / EVTCHNSTAT_interdomain */
			__u32 flags;
			union {
				struct {
					__u32 port;
					__u32 vcpu;
					__u32 priority;
				} port;
				struct {
					__u32 port; /* Zero for eventfd */
					__s32 fd;
				} eventfd;
				__u32 padding[4];
			} deliver;
		} evtchn;
		__u32 xen_version;
		__u64 pad[8];
	} u;
  };

```
type 值：

KVM_XEN_ATTR_TYPE_LONG_MODE
  将 VM 的 ABI 模式设置为 32 位或 64 位（长模式）。这决定了暴露给 VM 的 shared_info 页的布局。

KVM_XEN_ATTR_TYPE_SHARED_INFO
  设置 Xen shared_info 页所在的客户机物理帧号。注意，尽管 Xen 将前 32 个 vCPU 的 vcpu_info 放在
  shared_info 页中，但 KVM 不会自动这样做，而是要求即使给定 vCPU 的 vcpu_info 位于 shared_info
  页中的"默认"位置时，也要显式使用 KVM_XEN_VCPU_ATTR_TYPE_VCPU_INFO 或
  KVM_XEN_VCPU_ATTR_TYPE_VCPU_INFO_HVA。这是因为 KVM 可能不知道用作 vcpu_info[] 数组索引的 Xen
  CPU id，因此可能不知道正确的默认位置。

  注意，shared_info 页可能被 KVM 持续写入；除其他内容外，它包含用于向 Xen 客户机投递中断的事件通道
  位图。它免于脏页跟踪机制 —— 每次向客户机投递一个事件通道中断时，KVM 不会显式将该页标记为脏！
  因此，如果任何 vCPU 一直在运行，或者任何事件通道中断可以被路由到客户机，用户空间应始终假定指定的
  GFN 是脏的。

  将 gfn 设置为 KVM_XEN_INVALID_GFN 将禁用 shared_info 页。

KVM_XEN_ATTR_TYPE_SHARED_INFO_HVA
  如果在 Xen 能力中也设置了 KVM_XEN_HVM_CONFIG_SHARED_INFO_HVA 标志，则可以使用此属性来设置
  shared_info 页所在的用户空间地址，无论它映射在客户机物理地址空间的何处，该地址在 VMM 中始终是
  固定的。应优先使用此属性而不是 KVM_XEN_ATTR_TYPE_SHARED_INFO，因为它避免在页被重新映射到客户机
  物理地址空间时对内部缓存进行不必要的失效。

  将 hva 设置为零将禁用 shared_info 页。

KVM_XEN_ATTR_TYPE_UPCALL_VECTOR
  设置用于投递 Xen 事件通道 upcall 的异常向量。这是由 hypervisor 直接注入的、VM 范围的向量（不
  通过本地 APIC），通常由客户机通过 HVM_PARAM_CALLBACK_IRQ 配置。可以通过将其设置为零来再次禁用
  （例如对于客户机 SHUTDOWN_soft_reset）。

KVM_XEN_ATTR_TYPE_EVTCHN
  当 KVM_CAP_XEN_HVM ioctl 指示支持 KVM_XEN_HVM_CONFIG_EVTCHN_SEND 特性时，此属性可用。它配置
  一个出站端口号，用于拦截来自客户机的 EVTCHNOP_send 请求。给定的发送端口号可以被定向回客户机
  上指定的 vCPU（通过 APIC ID）/端口/优先级，或触发 eventfd 上的事件。可以通过在后续调用中设置
  KVM_XEN_EVTCHN_UPDATE 来更改 vCPU 和优先级，但对于给定的发送端口，其他字段不能更改。通过在
  flags 字段中使用 KVM_XEN_EVTCHN_DEASSIGN 来移除端口映射。在 flags 字段中传入 KVM_XEN_EVTCHN_RESET
  会移除对所有出站事件通道的拦截。flags 字段的值是互斥的，不能组合成位掩码。

KVM_XEN_ATTR_TYPE_XEN_VERSION
  当 KVM_CAP_XEN_HVM ioctl 指示支持 KVM_XEN_HVM_CONFIG_EVTCHN_SEND 特性时，此属性可用。它配置
  客户机调用 XENVER_version 时返回的 32 位版本码；通常是（XEN_MAJOR << 16 | XEN_MINOR）。PV Xen
  客户机通常会使用它作为虚拟 hypercall 来触发事件通道投递，因此在内核中响应而不退出到用户空间是
  有益的。

KVM_XEN_ATTR_TYPE_RUNSTATE_UPDATE_FLAG
  当 KVM_CAP_XEN_HVM ioctl 指示支持 KVM_XEN_HVM_CONFIG_RUNSTATE_UPDATE_FLAG 时，此属性可用。它
  启用 XEN_RUNSTATE_UPDATE 标志，该标志允许客户机 vCPU 安全地读取其他 vCPU 的 vcpu_runstate_info。
  Xen 客户机通过 HYPERVISOR_vm_assist hypercall 的 VMASST_TYPE_runstate_update_flag 来启用此特性。

### 4.127 KVM_XEN_HVM_GET_ATTR


:Capability: KVM_CAP_XEN_HVM / KVM_XEN_HVM_CONFIG_SHARED_INFO
:Architectures: x86
:Type: vm ioctl
:Parameters: struct kvm_xen_hvm_attr
:Returns: 0 on success, < 0 on error

允许读取 Xen VM 属性。关于结构体和类型，请参见上面的 KVM_XEN_HVM_SET_ATTR。KVM_XEN_ATTR_TYPE_EVTCHN
属性不能被读取。
### 4.128 KVM_XEN_VCPU_SET_ATTR


:Capability: KVM_CAP_XEN_HVM / KVM_XEN_HVM_CONFIG_SHARED_INFO
:Architectures: x86
:Type: vcpu ioctl
:Parameters: struct kvm_xen_vcpu_attr
:Returns: 0 on success, < 0 on error

```

  struct kvm_xen_vcpu_attr {
	__u16 type;
	__u16 pad[3];
	union {
		__u64 gpa;
		__u64 pad[4];
		struct {
			__u64 state;
			__u64 state_entry_time;
			__u64 time_running;
			__u64 time_runnable;
			__u64 time_blocked;
			__u64 time_offline;
		} runstate;
		__u32 vcpu_id;
		struct {
			__u32 port;
			__u32 priority;
			__u64 expires_ns;
		} timer;
		__u8 vector;
	} u;
  };

```
type 值：

KVM_XEN_VCPU_ATTR_TYPE_VCPU_INFO
  设置给定 vCPU 的 vcpu_info 的客户机物理地址。与 VM 的 shared_info 页一样，如果启用了事件通道
  中断投递，相应页可能随时被弄脏，因此用户空间应始终假设该页是脏的，而不依赖于脏页记录。将 gpa
  设置为 KVM_XEN_INVALID_GPA 将禁用 vcpu_info。

KVM_XEN_VCPU_ATTR_TYPE_VCPU_INFO_HVA
  如果在 Xen 能力中也设置了 KVM_XEN_HVM_CONFIG_SHARED_INFO_HVA 标志，则可以使用此属性来设置
  给定 vCPU 的 vcpu_info 的用户空间地址。它只应在 vcpu_info 位于 shared_info 页中的"默认"位置
  时使用。在这种情况下，可以安全地假设用户空间地址不会改变，因为 shared_info 页是客户机内存上的
  一个覆盖层（overlay），无论它映射在客户机物理地址空间的何处，都保持在固定的宿主机地址，因此如果
  客户机内存布局被修改，可以避免对内部缓存进行不必要的失效。如果 vcpu_info 不位于"默认"位置，则
  不能保证它保持在相同的宿主机地址，因此需要上述的缓存失效。

KVM_XEN_VCPU_ATTR_TYPE_VCPU_TIME_INFO
  设置给定 vCPU 的额外 pvclock 结构的客户机物理地址。这通常用于客户机 vsyscall 支持。将 gpa 设置
  为 KVM_XEN_INVALID_GPA 将禁用该结构。

KVM_XEN_VCPU_ATTR_TYPE_RUNSTATE_ADDR
  设置给定 vCPU 的 vcpu_runstate_info 的客户机物理地址。Xen 客户机通过它来跟踪 steal time 等 CPU
  状态。将 gpa 设置为 KVM_XEN_INVALID_GPA 将禁用 runstate 区域。

KVM_XEN_VCPU_ATTR_TYPE_RUNSTATE_CURRENT
  从结构体的 .u.runstate.state 成员设置给定 vCPU 的 runstate（RUNSTATE_running/_runnable/_blocked/
  _offline）。KVM 自动计算 running 和 runnable 时间，但 blocked 和 offline 状态只能显式进入。

KVM_XEN_VCPU_ATTR_TYPE_RUNSTATE_DATA
  从结构体的 .u.runstate 成员设置 vCPU runstate 数据的所有字段，包括当前 runstate。state_entry_time
  必须等于其他四个时间的总和。

KVM_XEN_VCPU_ATTR_TYPE_RUNSTATE_ADJUST
  这将结构体的 .u.runstate 成员的内容**加**到给定 vCPU 的 runstate 数据的相应成员上，从而允许
  对 runstate 时间进行原子调整。对 state_entry_time 的调整必须等于对其他四个时间的调整之和。
  state 字段必须设置为 -1，或设置为有效的 runstate 值（RUNSTATE_running、RUNSTATE_runnable、
  RUNSTATE_blocked 或 RUNSTATE_offline），以将当前计入状态设置为调整后的 state_entry_time 时的状态。

KVM_XEN_VCPU_ATTR_TYPE_VCPU_ID
  当 KVM_CAP_XEN_HVM ioctl 指示支持 KVM_XEN_HVM_CONFIG_EVTCHN_SEND 特性时，此属性可用。它设置
  给定 vCPU 的 Xen vCPU ID，以允许与定时器相关的 VCPU 操作被 KVM 拦截。

KVM_XEN_VCPU_ATTR_TYPE_TIMER
  当 KVM_CAP_XEN_HVM ioctl 指示支持 KVM_XEN_HVM_CONFIG_EVTCHN_SEND 特性时，此属性可用。它设置
  该 vCPU 的 VIRQ_TIMER 的事件通道端口/优先级，并允许保存/恢复一个挂起的定时器。将定时器端口
  设置为零会禁用内核对该单次触发（singleshot）定时器的处理。

KVM_XEN_VCPU_ATTR_TYPE_UPCALL_VECTOR
  当 KVM_CAP_XEN_HVM ioctl 指示支持 KVM_XEN_HVM_CONFIG_EVTCHN_SEND 特性时，此属性可用。它设置
  每 vCPU 的本地 APIC upcall 向量，由 Xen 客户机通过 HVMOP_set_evtchn_upcall_vector hypercall 配置。
  这通常由 Windows 客户机使用，并且与通过 HVM_PARAM_CALLBACK_IRQ 配置的 VM 范围的 upcall 向量不同。
  通过将向量设置为零来禁用它。


### 4.129 KVM_XEN_VCPU_GET_ATTR


:Capability: KVM_CAP_XEN_HVM / KVM_XEN_HVM_CONFIG_SHARED_INFO
:Architectures: x86
:Type: vcpu ioctl
:Parameters: struct kvm_xen_vcpu_attr
:Returns: 0 on success, -1 on error

允许读取 Xen vCPU 属性。关于结构体和类型，请参见上面的 KVM_XEN_VCPU_SET_ATTR。

KVM_XEN_VCPU_ATTR_TYPE_RUNSTATE_ADJUST 类型不能与 KVM_XEN_VCPU_GET_ATTR ioctl 一起使用。

### 4.130 KVM_ARM_MTE_COPY_TAGS


:Capability: KVM_CAP_ARM_MTE
:Architectures: arm64
:Type: vm ioctl
:Parameters: struct kvm_arm_copy_mte_tags
:Returns: number of bytes copied, < 0 on error (-EINVAL for incorrect
          arguments, -EFAULT if memory cannot be accessed).

```

  struct kvm_arm_copy_mte_tags {
	__u64 guest_ipa;
	__u64 length;
	void __user *addr;
	__u64 flags;
	__u64 reserved[2];
  };

```
在客户机标签内存之间复制内存标记扩展（MTE）标签。`guest_ipa` 和 `length` 字段必须与 `PAGE_SIZE`
对齐。`length` 不得大于 2^31 - PAGE_SIZE 字节。`addr` 字段必须指向一个缓冲区，标签将被复制进出其中。

`flags` 指定复制的方向，可以是 `KVM_ARM_TAGS_TO_GUEST` 或 `KVM_ARM_TAGS_FROM_GUEST`。

用于存储标签的缓冲区大小为 `(length / 16)` 字节（MTE 中的粒度为 16 字节）。每个字节包含一个
标签值。这与 `PTRACE_PEEKMTETAGS` 和 `PTRACE_POKEMTETAGS` 的格式匹配。

如果在复制任何数据之前发生错误，则返回负的错误码。如果在发生错误之前已复制了一些标签，则返回
成功复制的字节数。如果调用成功完成，则返回 `length`。

### 4.131 KVM_GET_SREGS2


:Capability: KVM_CAP_SREGS2
:Architectures: x86
:Type: vcpu ioctl
:Parameters: struct kvm_sregs2 (out)
:Returns: 0 on success, -1 on error

从 vcpu 读取特殊寄存器。此 ioctl（在受支持时）取代 KVM_GET_SREGS。

```

        struct kvm_sregs2 {
                /* out (KVM_GET_SREGS2) / in (KVM_SET_SREGS2) */
                struct kvm_segment cs, ds, es, fs, gs, ss;
                struct kvm_segment tr, ldt;
                struct kvm_dtable gdt, idt;
                __u64 cr0, cr2, cr3, cr4, cr8;
                __u64 efer;
                __u64 apic_base;
                __u64 flags;
                __u64 pdptrs[4];
        };

```
`kvm_sregs2` 的 flags 值：

`KVM_SREGS2_FLAGS_PDPTRS_VALID`

  指示结构体包含有效的 PDPTR 值。


### 4.132 KVM_SET_SREGS2


:Capability: KVM_CAP_SREGS2
:Architectures: x86
:Type: vcpu ioctl
:Parameters: struct kvm_sregs2 (in)
:Returns: 0 on success, -1 on error

将特殊寄存器写入 vcpu。数据结构请参见 KVM_GET_SREGS2。此 ioctl（在受支持时）取代 KVM_SET_SREGS。

### 4.133 KVM_GET_STATS_FD


:Capability: KVM_CAP_STATS_BINARY_FD
:Architectures: all
:Type: vm ioctl, vcpu ioctl
:Parameters: none
:Returns: statistics file descriptor on success, < 0 on error

错误码：

  ======     ======================================================
  ENOMEM     如果由于内存不足而无法创建 fd
  EMFILE     如果打开的文件数超过了限制
  ======     ======================================================

返回的文件描述符可用于以二进制格式读取 VM/vCPU 统计数据。文件描述符中的数据由四个块组成，组织
如下：

+-------------+
|   Header    |
+-------------+
|  id string  |
+-------------+
| Descriptors |
+-------------+
| Stats Data  |
+-------------+

除了从偏移 0 开始的头部之外，请注意，不保证这四个块是相邻的或按上述顺序排列；id、descriptors 和
data 块的偏移量在头部中找到。但是，所有四个块都在文件中按 64 位偏移对齐，并且它们不重叠。

除 data 块之外的所有块都是不可变的。用户空间在获取文件描述符后只能读取它们一次，然后使用 `pread`
或 `lseek` 重复读取统计数据。

所有数据采用系统字节序。

```

	struct kvm_stats_header {
		__u32 flags;
		__u32 name_size;
		__u32 num_desc;
		__u32 id_offset;
		__u32 desc_offset;
		__u32 data_offset;
	};

```
`flags` 字段目前未被使用。它总是被读取为 0。

`name_size` 字段是统计数据名称字符串的大小（以字节为单位，包括结尾的 '\0'），该字符串包含在
"id string" 块中，并附加在每个描述符的末尾。

`num_desc` 字段是描述符块中包含的描述符数量。（data 块中的实际值数量可能更大，因为每个描述符
可能包含多个值）。

`id_offset` 字段是 id 字符串相对于文件描述符所指示的文件起始位置的偏移量。它是 8 的倍数。

`desc_offset` 字段是 Descriptors 块相对于文件描述符所指示的文件起始位置的偏移量。它是 8 的倍数。

`data_offset` 字段是 Stats Data 块相对于文件描述符所指示的文件起始位置的偏移量。它是 8 的倍数。

id 字符串块包含一个字符串，用于标识调用 KVM_GET_STATS_FD 的文件描述符。该块的大小（包括结尾的
`'\0'`）由头部中的 `name_size` 字段指示。

描述符块只需要在文件描述符的生命周期内读取一次，它包含一个 `struct kvm_stats_desc` 序列，每个
后面跟着一个大小为 `name_size` 的字符串。
```

	#define KVM_STATS_TYPE_SHIFT		0
	#define KVM_STATS_TYPE_MASK		(0xF << KVM_STATS_TYPE_SHIFT)
	#define KVM_STATS_TYPE_CUMULATIVE	(0x0 << KVM_STATS_TYPE_SHIFT)
	#define KVM_STATS_TYPE_INSTANT		(0x1 << KVM_STATS_TYPE_SHIFT)
	#define KVM_STATS_TYPE_PEAK		(0x2 << KVM_STATS_TYPE_SHIFT)
	#define KVM_STATS_TYPE_LINEAR_HIST	(0x3 << KVM_STATS_TYPE_SHIFT)
	#define KVM_STATS_TYPE_LOG_HIST		(0x4 << KVM_STATS_TYPE_SHIFT)
	#define KVM_STATS_TYPE_MAX		KVM_STATS_TYPE_LOG_HIST

	#define KVM_STATS_UNIT_SHIFT		4
	#define KVM_STATS_UNIT_MASK		(0xF << KVM_STATS_UNIT_SHIFT)
	#define KVM_STATS_UNIT_NONE		(0x0 << KVM_STATS_UNIT_SHIFT)
	#define KVM_STATS_UNIT_BYTES		(0x1 << KVM_STATS_UNIT_SHIFT)
	#define KVM_STATS_UNIT_SECONDS		(0x2 << KVM_STATS_UNIT_SHIFT)
	#define KVM_STATS_UNIT_CYCLES		(0x3 << KVM_STATS_UNIT_SHIFT)
	#define KVM_STATS_UNIT_BOOLEAN		(0x4 << KVM_STATS_UNIT_SHIFT)
	#define KVM_STATS_UNIT_MAX		KVM_STATS_UNIT_BOOLEAN

	#define KVM_STATS_BASE_SHIFT		8
	#define KVM_STATS_BASE_MASK		(0xF << KVM_STATS_BASE_SHIFT)
	#define KVM_STATS_BASE_POW10		(0x0 << KVM_STATS_BASE_SHIFT)
	#define KVM_STATS_BASE_POW2		(0x1 << KVM_STATS_BASE_SHIFT)
	#define KVM_STATS_BASE_MAX		KVM_STATS_BASE_POW2

	struct kvm_stats_desc {
		__u32 flags;
		__s16 exponent;
		__u16 size;
		__u32 offset;
		__u32 bucket_size;
		char name[];
	};

```
`flags` 字段包含此描述符所描述的统计数据数据的类型和单位。其字节序为 CPU 原生字节序。支持以下
标志：

`flags` 的位 0-3 编码类型：

  - `KVM_STATS_TYPE_CUMULATIVE`
    统计报告一个累积计数。数据的值只能增加。KVM 中使用的大多数计数器都是这种类型。该类型对应的
    `size` 字段始终为 1。所有累积统计数据都是读/写的。
  - `KVM_STATS_TYPE_INSTANT`
    统计报告一个瞬时值。其值可以增加或减少。这种类型通常用于测量某些资源，例如脏页数、大页数等。
    所有瞬时统计都是只读的。该类型对应的 `size` 字段始终为 1。
  - `KVM_STATS_TYPE_PEAK`
    统计数据报告一个峰值，例如哈希表桶中的最大项数、最长的等待时间等。数据的值只能增加。该类型
    对应的 `size` 字段始终为 1。
  - `KVM_STATS_TYPE_LINEAR_HIST`
    统计报告为线性直方图。桶的数量由 `size` 字段指定。桶的大小由 `hist_param` 字段指定。第 N 个
    桶（1 <= N < `size`）的范围是 [`hist_param`**(N-1), `hist_param`**N)，而最后一个桶的范围是
    [`hist_param`*(`size`-1), +INF)。（+INF 表示正无穷值。）
  - `KVM_STATS_TYPE_LOG_HIST`
    统计报告为对数直方图。桶的数量由 `size` 字段指定。第一个桶的范围是 [0, 1)，而最后一个桶的范围
    是 [pow(2, `size`-2), +INF)。否则，第 N 个桶（1 < N < `size`）覆盖 [pow(2, N-2), pow(2, N-1))。

`flags` 的位 4-7 编码单位：

  - `KVM_STATS_UNIT_NONE`
    统计数据值没有单位。这通常意味着该值是一个事件的简单计数器。
  - `KVM_STATS_UNIT_BYTES`
    它表示统计数据用于测量内存大小，单位为 Byte、KiByte、MiByte、GiByte 等。数据的单位由描述符中的
    `exponent` 字段决定。
  - `KVM_STATS_UNIT_SECONDS`
    它表示统计数据用于测量时间或延迟。
  - `KVM_STATS_UNIT_CYCLES`
    它表示统计数据用于测量 CPU 时钟周期。
  - `KVM_STATS_UNIT_BOOLEAN`
    它表示统计值将始终为 0 或 1。峰值类型的布尔统计永远不会从 1 回到 0。布尔统计可以是线性直方图
    （有两个桶），但不能是对数直方图。

注意，对于直方图，单位适用于桶的范围，而桶值指示落入该桶范围内的样本数量。

`flags` 的位 8-11 与 `exponent` 一起编码单位的量级：

  - `KVM_STATS_BASE_POW10`
    量级基于 10 的幂。它用于测量时间和 CPU 时钟周期。例如，指数 -9 可以与 `KVM_STATS_UNIT_SECONDS`
    一起使用，表示单位是纳秒。
  - `KVM_STATS_BASE_POW2`
    量级基于 2 的幂。它用于测量内存大小。例如，指数 20 可以与 `KVM_STATS_UNIT_BYTES` 一起使用，表示
    单位是 MiB。

`size` 字段是此统计数据值的数量。对于大多数简单统计，其值通常为 1。1 表示它包含一个无符号 64
位数据。

`offset` 字段是从 Data Block 起始位置到相应统计数据起始位置的偏移量。

`bucket_size` 字段用作直方图统计数据的参数。它仅由线性直方图统计数据使用，指定一个桶的大小，单位
由 `flags` 的位 4-11 与 `exponent` 一起表示。

`name` 字段是统计数据的名称字符串。名称字符串从 `struct kvm_stats_desc` 的末尾开始。包括结尾
`'\0'` 在内的最大长度由头部中的 `name_size` 指示。

Stats Data 块包含一个 64 位值数组，顺序与 Descriptors 块中的描述符相同。

### 4.134 KVM_GET_XSAVE2


:Capability: KVM_CAP_XSAVE2
:Architectures: x86
:Type: vcpu ioctl
:Parameters: struct kvm_xsave (out)
:Returns: 0 on success, -1 on error


```

  struct kvm_xsave {
	__u32 region[1024];
	__u32 extra[0];
  };

```
该 ioctl 会将当前 vcpu 的 xsave 结构体复制到用户空间。它复制的字节数等于 KVM_CHECK_EXTENSION(KVM_CAP_XSAVE2)
在 vm 文件描述符上调用时返回的值。KVM_CHECK_EXTENSION(KVM_CAP_XSAVE2) 返回的大小值总是至少为 4096。
目前，只有当某个动态特性已通过 `arch_prctl()` 启用时它才大于 4096，但这在未来可能会改变。

struct kvm_xsave 中各状态保存区域的偏移量遵循宿主机上 CPUID 叶子 0xD 的内容。

### 4.135 KVM_XEN_HVM_EVTCHN_SEND


:Capability: KVM_CAP_XEN_HVM / KVM_XEN_HVM_CONFIG_EVTCHN_SEND
:Architectures: x86
:Type: vm ioctl
:Parameters: struct kvm_irq_routing_xen_evtchn
:Returns: 0 on success, < 0 on error


```

   struct kvm_irq_routing_xen_evtchn {
	__u32 port;
	__u32 vcpu;
	__u32 priority;
   };

```
该 ioctl 将事件通道中断直接注入客户机 vCPU。

### 4.136 KVM_S390_PV_CPU_COMMAND


:Capability: KVM_CAP_S390_PROTECTED_DUMP
:Architectures: s390
:Type: vcpu ioctl
:Parameters: none
:Returns: 0 on success, < 0 on error

该 ioctl 与 `KVM_S390_PV_COMMAND` 非常相似，但处理针对 vcpu 的请求。它复用了 kvm_s390_pv_dmp
结构体，因此也共享命令 id。

**command：**

KVM_PV_DUMP
  提供一个 API，提供有助于转储受保护 VM 的 vcpu 的调用。

**subcommand：**

KVM_PV_DUMP_CPU
  提供加密的转储数据，如寄存器值。返回数据的长度由 uv_info.guest_cpu_stor_len 提供。

### 4.137 KVM_S390_ZPCI_OP


:Capability: KVM_CAP_S390_ZPCI_OP
:Architectures: s390
:Type: vm ioctl
:Parameters: struct kvm_s390_zpci_op (in)
:Returns: 0 on success, <0 on error

用于管理 zPCI 设备的硬件辅助虚拟化特性。

```

  struct kvm_s390_zpci_op {
	/* in */
	__u32 fh;		/* target device */
	__u8  op;		/* operation to perform */
	__u8  pad[3];
	union {
		/* for KVM_S390_ZPCIOP_REG_AEN */
		struct {
			__u64 ibv;	/* Guest addr of interrupt bit vector */
			__u64 sb;	/* Guest addr of summary bit */
			__u32 flags;
			__u32 noi;	/* Number of interrupts */
			__u8 isc;	/* Guest interrupt subclass */
			__u8 sbo;	/* Offset of guest summary bit vector */
			__u16 pad;
		} reg_aen;
		__u64 reserved[8];
	} u;
  };

```
操作类型在 "op" 字段中指定。KVM_S390_ZPCIOP_REG_AEN 用于为 VM 注册适配器事件通知解释（adapter
event notification interpretation），这将允许固件直接将适配器事件投递到 vm，由 KVM 提供备份投递
机制；KVM_S390_ZPCIOP_DEREG_AEN 用于随后禁用适配器事件通知的解释。

目标 zPCI 功能也必须通过 "fh" 字段指定。对于 KVM_S390_ZPCIOP_REG_AEN 操作，必须通过 "reg_aen"
结构体提供建立固件投递所需的额外信息。

"pad" 和 "reserved" 字段可用于未来的扩展，用户空间应将其设置为 0。

### 4.138 KVM_ARM_SET_COUNTER_OFFSET


:Capability: KVM_CAP_COUNTER_OFFSET
:Architectures: arm64
:Type: vm ioctl
:Parameters: struct kvm_arm_counter_offset (in)
:Returns: 0 on success, < 0 on error

该能力指示用户空间能够使用 KVM_ARM_SET_CNT_OFFSET ioctl 以及以下数据结构，将单一 VM 范围的偏移
应用到客户机所见的虚拟计数器和物理计数器：

```

	struct kvm_arm_counter_offset {
		__u64 counter_offset;
		__u64 reserved;
	};

```
该偏移描述了从虚拟和物理计数器视图中减去的计数器周期数（类似于 CNTVOFF_EL2 和 CNTPOFF_EL2 系统
寄存器的效果，但仅全局生效）。该偏移始终应用于此 VM 的所有 vcpu（已创建或在调用此 ioctl 之后
创建的）。

计算偏移是用户空间的责任，例如基于客户机计数器的先前值。

"reserved" 字段的任何非 0 值都可能导致返回错误（-EINVAL）。如果同时发出了任何 vcpu ioctl，此
ioctl 也可能返回 -EBUSY。

注意，使用此 ioctl 会导致 KVM 忽略随后用户空间使用 SET_ONE_REG 接口对 CNTVCT_EL0 和 CNTPCT_EL0
寄存器的写入。不会返回错误，但结果偏移不会被应用。


### 4.139 KVM_ARM_GET_REG_WRITABLE_MASKS


:Capability: KVM_CAP_ARM_SUPPORTED_REG_MASK_RANGES
:Architectures: arm64
:Type: vm ioctl
:Parameters: struct reg_mask_range (in/out)
:Returns: 0 on success, < 0 on error


```

        #define KVM_ARM_FEATURE_ID_RANGE	0
        #define KVM_ARM_FEATURE_ID_RANGE_SIZE	(3 * 8 * 8)

        struct reg_mask_range {
                __u64 addr;             /* Pointer to mask array */
                __u32 range;            /* Requested range */
                __u32 reserved[13];
        };

```
该 ioctl 将所选寄存器范围的 writable 掩码复制到用户空间。

`addr` 字段是指向目标数组的指针，KVM 将 writable 掩码复制到那里。

`range` 字段指示请求的寄存器范围。`KVM_CHECK_EXTENSION` 对 `KVM_CAP_ARM_SUPPORTED_REG_MASK_RANGES`
能力的查询返回受支持的范围，表示为一组标志。每个标志的位索引代表 `range` 字段的一个可能值。所有
其他值保留供将来使用，KVM 可能返回错误。

`reserved[^13^]` 数组保留供将来使用，应为 0，否则 KVM 可能返回错误。

##### KVM_ARM_FEATURE_ID_RANGE (0)


Feature ID 范围定义为 AArch64 系统寄存器空间，其中 op0==3、op1=={0, 1, 3}、CRn==0、CRn=={0-7}、
op2=={0-7}。

`addr` 指向的返回掩码数组由宏 `ARM64_FEATURE_ID_RANGE_IDX(op0, op1, crn, crm, op2)` 索引，使
用户空间能够知道 `op0, op1, crn, crm, op2` 所描述的系统寄存器可以更改哪些字段。KVM 会拒绝描述
系统所支持特性超集的 ID 寄存器值。

### 4.140 KVM_SET_USER_MEMORY_REGION2


:Capability: KVM_CAP_USER_MEMORY2
:Architectures: all
:Type: vm ioctl
:Parameters: struct kvm_userspace_memory_region2 (in)
:Returns: 0 on success, -1 on error

KVM_SET_USER_MEMORY_REGION2 是 KVM_SET_USER_MEMORY_REGION 的扩展，允许将 guest_memfd 内存映射到
客户机。所有与 KVM_SET_USER_MEMORY_REGION 共享的字段都完全相同。用户空间可以在 flags 中设置
KVM_MEM_GUEST_MEMFD，让 KVM 将内存区域绑定到给定的 guest_memfd 范围
[guest_memfd_offset, guest_memfd_offset + memory_size]。目标 guest_memfd 必须指向通过当前 VM 上的
KVM_CREATE_GUEST_MEMFD 创建的文件，且目标范围不得绑定到任何其他内存区域。所有标准的边界检查都
适用（请运用常识）。

```

  struct kvm_userspace_memory_region2 {
	__u32 slot;
	__u32 flags;
	__u64 guest_phys_addr;
	__u64 memory_size; /* bytes */
	__u64 userspace_addr; /* start of the userspace allocated memory */
	__u64 guest_memfd_offset;
	__u32 guest_memfd;
	__u32 pad1;
	__u64 pad2[14];
  };

```
KVM_MEM_GUEST_MEMFD 区域_必须_有一个有效的 guest_memfd（私有内存）和 userspace_addr（共享内存）。
但是，对于 userspace_addr 来说，"有效"仅仅意味着地址本身必须是一个合法的的用户空间地址。userspace_addr
的后备映射不需要在 KVM_SET_USER_MEMORY_REGION2 时有效/已填充，例如共享内存可以按需惰性映射/分配。

当将 gfn 映射到客户机时，KVM 根据 gfn 的 KVM_MEMORY_ATTRIBUTE_PRIVATE 状态选择共享还是私有，即
使用 userspace_addr 还是 guest_memfd。在创建 VM 时，所有内存都是共享的，即所有 gfn 的 PRIVATE
属性为 '0'。用户空间可以通过按需通过 KVM_SET_MEMORY_ATTRIBUTES 切换 KVM_MEMORY_ATTRIBUTE_PRIVATE
来控制内存是共享还是私有。

##### S390：


如果 VM 设置了 KVM_VM_S390_UCONTROL 标志，则返回 -EINVAL。
如果是在受保护的 VM 上调用，则返回 -EINVAL。

### 4.141 KVM_SET_MEMORY_ATTRIBUTES


:Capability: KVM_CAP_MEMORY_ATTRIBUTES
:Architectures: x86
:Type: vm ioctl
:Parameters: struct kvm_memory_attributes (in)
:Returns: 0 on success, <0 on error

KVM_SET_MEMORY_ATTRIBUTES 允许用户空间为一段客户机物理内存设置内存属性。

```

  struct kvm_memory_attributes {
	__u64 address;
	__u64 size;
	__u64 attributes;
	__u64 flags;
  };

  #define KVM_MEMORY_ATTRIBUTE_PRIVATE           (1ULL << 3)

```
address 和 size 必须与页对齐。受支持的属性可以通过在 KVM_CAP_MEMORY_ATTRIBUTES 上调用
ioctl(KVM_CHECK_EXTENSION) 获取。如果在 VM 上执行，KVM_CAP_MEMORY_ATTRIBUTES 精确返回该 VM 支持的
属性。如果在系统范围执行，KVM_CAP_MEMORY_ATTRIBUTES 返回 KVM 支持的所有属性。目前定义的唯一属性是
KVM_MEMORY_ATTRIBUTE_PRIVATE，它将相关的 gfn 标记为客机私有内存。

注意，没有"get" API。用户空间负责根据需要显式跟踪 gfn/页的状态。

"flags" 字段保留供将来扩展，必须为 '0'。

### 4.142 KVM_CREATE_GUEST_MEMFD


:Capability: KVM_CAP_GUEST_MEMFD
:Architectures: none
:Type: vm ioctl
:Parameters: struct kvm_create_guest_memfd(in)
:Returns: A file descriptor on success, <0 on error

KVM_CREATE_GUEST_MEMFD 创建一个匿名文件，并返回一个引用它的文件描述符。guest_memfd 文件大致类似于
通过 memfd_create() 创建的文件，例如，guest_memfd 文件驻留在 RAM 中，具有易失性存储，并在最后一个
引用被释放时自动释放。与"常规" memfd_create() 文件不同，guest_memfd 文件绑定到其拥有的虚拟机
（见下文），不能被用户空间映射、读取或写入，并且不能调整大小（不过 guest_memfd 文件支持
PUNCH_HOLE）。

```

  struct kvm_create_guest_memfd {
	__u64 size;
	__u64 flags;
	__u64 reserved[6];
  };

```
从概念上讲，支撑 guest_memfd 文件的 inode 代表物理内存，即与虚拟机作为一个事物耦合，而不是与
"struct kvm" 耦合。文件本身绑定到 "struct kvm"，是该实例对底层内存的视图，例如有效地提供客户机
地址到宿主机内存的转换。这允许这样的用例：多个 KVM 结构用于管理单个虚拟机，例如在执行虚拟机的
宿主机内（intrahost）迁移时。

KVM 目前仅支持通过 KVM_SET_USER_MEMORY_REGION2 映射 guest_memfd，更具体地说，通过
"struct kvm_userspace_memory_region2" 中的 guest_memfd 和 guest_memfd_offset 字段，其中
guest_memfd_offset 是进入 guest_memfd 实例的偏移量。对于给定的 guest_memfd 文件，每页最多有一个
映射，即不允许将多个内存区域绑定到单个 guest_memfd 范围（任何数量的内存区域都可以绑定到单个
guest_memfd 文件，但绑定的范围不得重叠）。

能力 KVM_CAP_GUEST_MEMFD_FLAGS 枚举了可通过 KVM_CREATE_GUEST_MEMFD 指定的 `flags`。当前定义的标志：

  ============================ ================================================
  GUEST_MEMFD_FLAG_MMAP        启用在 guest_memfd 文件描述符上使用 mmap()。
  GUEST_MEMFD_FLAG_INIT_SHARED 在 KVM_CREATE_GUEST_MEMFD 期间使文件中的所有内存为共享
                               （在没有 INIT_SHARED 的情况下创建的内存文件将被标记为私有）。
                               共享内存可以缺页映射到宿主机用户空间页表。私有内存则不能。
  ============================ ================================================

当 KVM MMU 执行 PFN 查找以服务客户机缺页，且后备 guest_memfd 设置了 GUEST_MEMFD_FLAG_MMAP 时，
无论该缺页是共享还是私有的，缺页都将始终从 guest_memfd 消费。

更多细节请参见 KVM_SET_USER_MEMORY_REGION2。

### 4.143 KVM_PRE_FAULT_MEMORY


:Capability: KVM_CAP_PRE_FAULT_MEMORY
:Architectures: none
:Type: vcpu ioctl
:Parameters: struct kvm_pre_fault_memory (in/out)
:Returns: 0 if at least one page is processed, < 0 on error

错误码：

  ========== ===============================================================
  EINVAL     指定的 `gpa` 和 `size` 无效（例如未页对齐、导致溢出，或 size
             为零）。
  ENOENT     指定的 `gpa` 在已定义的 memslot 之外。
  EINTR      存在未屏蔽的挂起信号，且未处理任何页。
  EFAULT     参数地址无效。
  EOPNOTSUPP 为 GPA 映射内存不受 hypervisor 支持，和/或针对当前 vCPU 状态/模式
             不支持。
  EIO        意外错误条件（也会导致 WARN）
  ========== ===============================================================

```

  struct kvm_pre_fault_memory {
	/* in/out */
	__u64 gpa;
	__u64 size;
	/* in */
	__u64 flags;
	__u64 padding[5];
  };

```
KVM_PRE_FAULT_MEMORY 填充 KVM 用于为当前 vCPU 状态映射内存的 stage-2 页表。KVM 像 vCPU 产生了
stage-2 读缺页一样映射内存，例如按需缺页映射内存，但不打破写时复制（CoW）。但是，KVM 不会将任何新
创建的 stage-2 PTE 标记为 Accessed。

在机密 VM 类型中，在客户机被"定稿"/度量之前需要对私有客机内存进行初始设置的情况下，此 ioctl 应
仅在完成所有必要的设置以将客户机置于"定稿"状态之后发出，以便上述语义能够被可靠地保证。

在某些情况下，多个 vCPU 可能共享页表。在这种情况下，该 ioctl 可以并行调用。

当 ioctl 返回时，输入值被更新以指向剩余范围。如果返回时 `size` > 0，调用者可以再次使用相同的
`struct kvm_map_memory` 参数发出该 ioctl。

影子页表无法支持此 ioctl，因为它们是通过虚拟地址或嵌套客户机物理地址索引的。当客户机使用影子页表
时（例如因为它正在运行带有嵌套页表的嵌套客户机）调用此 ioctl，即使 `KVM_CHECK_EXTENSION` 报告该
能力存在，也会以 `EOPNOTSUPP` 失败。

`flags` 目前必须为零。

### 4.144 KVM_S390_KEYOP


:Capability: KVM_CAP_S390_KEYOP
:Architectures: s390
:Type: vm ioctl
:Parameters: struct kvm_s390_keyop (in/out)
:Returns: 0 in case of success, < 0 on error

对给定的客户机地址执行指定的密钥操作。先前的存储键（或其相关部分）将在 `key` 中返回。

```

  struct kvm_s390_keyop {
	__u64 guest_addr;
	__u8  key;
	__u8  operation;
  };

```
目前 `operation` 支持的如下值：

KVM_S390_KEYOP_ISKE
  在 `key` 中返回客户机地址 `guest_addr` 的存储键。

KVM_S390_KEYOP_RRBE
  重置客户机地址 `guest_addr` 的引用位（reference bit），在 `key` 中返回旧存储键的 R 和 C 位；
  存储键的其余字段将被设置为 0。

KVM_S390_KEYOP_SSKE
  将客户机地址 `guest_addr` 的存储键设置为 `key` 中指定的键，在 `key` 中返回先前的值。


## 5. The kvm_run structure


应用程序代码通过 mmap() 一个 vcpu fd 来获取指向 kvm_run 结构体的指针。从那时起，应用程序代码可以通过
在调用 KVM_RUN ioctl 之前更改 kvm_run 中的字段来控制执行，并通过查找结构体成员来获取关于 KVM_RUN
返回原因的信息。

```

  struct kvm_run {
	/* in */
	__u8 request_interrupt_window;

```
请求 KVM_RUN 在可以将会外部中断注入客户机时返回。与 KVM_INTERRUPT 配合使用很有用。

```

	__u8 immediate_exit;

```
该字段在 KVM_RUN 启动时轮询一次；如果非零，KVM_RUN 立即退出，返回 -EINTR。在通常使用信号将 VCPU
"踢"出 KVM_RUN 的常见场景中，该字段可用于避免使用 KVM_SET_SIGNAL_MASK，后者的可扩展性较差。与其
在 KVM_RUN 之外阻塞信号，用户空间可以设置一个信号处理程序，将 run->immediate_exit 设置为非零值。

如果 KVM_CAP_IMMEDIATE_EXIT 不可用，则忽略此字段。

```

	__u8 padding1[6];

	/* out */
	__u32 exit_reason;

```
当 KVM_RUN 成功返回（返回值 0）时，这告知应用程序代码 KVM_RUN 为何返回。此字段的允许值在下面详述。

```

	__u8 ready_for_interrupt_injection;

```
如果已指定 request_interrupt_window，则此字段指示现在可以使用 KVM_INTERRUPT 注入中断。

```

	__u8 if_flag;

```
当前中断标志的值。仅在内核态本地 APIC 未使用时有效。

```

	__u16 flags;

```
更多架构相关的标志，详细说明 VCPU 的状态，可能
```

  /* x86, set if the VCPU is in system management mode */
  #define KVM_RUN_X86_SMM          (1 << 0)
  /* x86, set if bus lock detected in VM */
  #define KVM_RUN_X86_BUS_LOCK     (1 << 1)
  /* x86, set if the VCPU is executing a nested (L2) guest */
  #define KVM_RUN_X86_GUEST_MODE   (1 << 2)

  /* arm64, set for KVM_EXIT_DEBUG */
  #define KVM_DEBUG_ARCH_HSR_HIGH_VALID  (1 << 0)

```
```

	/* in (pre_kvm_run), out (post_kvm_run) */
	__u64 cr8;

```
cr8 寄存器的值。仅在内核态本地 APIC 未使用时有效。既输入又输出。

```

	__u64 apic_base;

```
APIC BASE msr 的值。仅在内核态本地 APIC 未使用时有效。既输入又输出。

```

	union {
		/* KVM_EXIT_UNKNOWN */
		struct {
			__u64 hardware_exit_reason;
		} hw;

```
如果 exit_reason 是 KVM_EXIT_UNKNOWN，则 vcpu 由于未知原因退出。进一步的架构相关信息可在
hardware_exit_reason 中获得。

```

		/* KVM_EXIT_FAIL_ENTRY */
		struct {
			__u64 hardware_entry_failure_reason;
			__u32 cpu; /* if KVM_LAST_CPU */
		} fail_entry;

```
如果 exit_reason 是 KVM_EXIT_FAIL_ENTRY，则由于未知原因 vcpu 无法运行。进一步的架构相关信息可在
hardware_entry_failure_reason 中获得。

```

		/* KVM_EXIT_EXCEPTION */
		struct {
			__u32 exception;
			__u32 error_code;
		} ex;

```
未使用。

```

		/* KVM_EXIT_IO */
		struct {
  #define KVM_EXIT_IO_IN  0
  #define KVM_EXIT_IO_OUT 1
			__u8 direction;
			__u8 size; /* bytes */
			__u16 port;
			__u32 count;
			__u64 data_offset; /* relative to kvm_run start */
		} io;

```
如果 exit_reason 是 KVM_EXIT_IO，则 vcpu 执行了一条无法被 kvm 满足的端口 I/O 指令。data_offset
描述了数据所在的位置（KVM_EXIT_IO_OUT）或 kvm 期望应用程序代码为下一次 KVM_RUN 调用放置数据的位置
（KVM_EXIT_IO_IN）。数据格式是打包数组。

```

		/* KVM_EXIT_DEBUG */
		struct {
			struct kvm_debug_exit_arch arch;
		} debug;

```
如果 exit_reason 是 KVM_EXIT_DEBUG，则 vcpu 正在处理一个调试事件，返回架构相关的信息。

```

		/* KVM_EXIT_MMIO */
		struct {
			__u64 phys_addr;
			__u8  data[8];
			__u32 len;
			__u8  is_write;
		} mmio;

```
如果 exit_reason 是 KVM_EXIT_MMIO，则 vcpu 执行了一条无法被 kvm 满足的内存映射 I/O 指令。'data'
成员包含写入的数据（如果 'is_write' 为 true），否则应由应用程序代码填充。

'data' 成员在其前 'len' 个字节中包含该值，就像 VCPU 直接对字节数组执行了适当宽度的加载或存储一样。


      For KVM_EXIT_IO, KVM_EXIT_MMIO, KVM_EXIT_OSI, KVM_EXIT_PAPR, KVM_EXIT_XEN,
      KVM_EXIT_EPR, KVM_EXIT_HYPERCALL, KVM_EXIT_TDX,
      KVM_EXIT_X86_RDMSR and KVM_EXIT_X86_WRMSR the corresponding
      operations are complete (and guest state is consistent) only after userspace
      has re-entered the kernel with KVM_RUN.  The kernel side will first finish
      incomplete operations and then check for pending signals.

      操作的非挂起状态不保存在用户空间可见的状态中，因此用户空间应确保在执行实时迁移之前操作已
      完成。用户空间可以通过带有未屏蔽挂起信号或设置了 immediate_exit 字段重新进入客户机来完成
      挂起的操作，而不允许执行任何进一步的指令。

```

		/* KVM_EXIT_HYPERCALL */
		struct {
			__u64 nr;
			__u64 args[6];
			__u64 ret;
			__u64 flags;
		} hypercall;


```
强烈建议用户空间使用 `KVM_EXIT_IO`（x86）或 `KVM_EXIT_MMIO`（除 s390 外的所有架构）来实现需要
客户机与宿主机用户空间交互的功能。
### 对于 arm64：


SMCCC 退出可根据 SMCCC 过滤器的配置启用。更多细节请参阅
Documentation/virt/kvm/devices/vm.rst 中的 `KVM_ARM_SMCCC_FILTER`。

`nr` 包含客户机 SMCCC 调用的功能 ID。用户空间应使用 `KVM_GET_ONE_REG`
ioctl 从 vCPU 的 GPR 中检索调用参数。

`flags` 的定义：
 - `KVM_HYPERCALL_EXIT_SMC`：表示客户机使用 SMC 通道发起 SMCCC 调用。
   若该位为 0，则客户机使用 HVC 通道发起 SMCCC 调用。

 - `KVM_HYPERCALL_EXIT_16BIT`：表示客户机使用 16 位指令发起 SMCCC 调用。
   若该位为 0，则客户机使用 32 位指令。AArch64 客户机该位始终为 0。

退出时，PC 指向陷阱指令之后的那条指令。

```

		/* KVM_EXIT_TPR_ACCESS */
		struct {
			__u64 rip;
			__u32 is_write;
			__u32 pad;
		} tpr_access;

```
待补充文档（KVM_TPR_ACCESS_REPORTING）。

```

		/* KVM_EXIT_S390_SIEIC */
		struct {
			__u8 icptcode;
			__u64 mask; /* psw 上半部分 */
			__u64 addr; /* psw 下半部分 */
			__u16 ipa;
			__u32 ipb;
		} s390_sieic;

```
s390 特有。

```

		/* KVM_EXIT_S390_RESET */
  #define KVM_S390_RESET_POR       1
  #define KVM_S390_RESET_CLEAR     2
  #define KVM_S390_RESET_SUBSYSTEM 4
  #define KVM_S390_RESET_CPU_INIT  8
  #define KVM_S390_RESET_IPL       16
		__u64 s390_reset_flags;

```
s390 特有。

```

		/* KVM_EXIT_S390_UCONTROL */
		struct {
			__u64 trans_exc_code;
			__u32 pgm_code;
		} s390_ucontrol;

```
s390 特有。用户控制的虚拟机（KVM_VM_S390_UNCONTROL）在其宿主页表上发生了
内核无法解析的缺页故障。
放置在 CPU lowcore 中的程序代码和转换异常代码在此处按 z 架构操作原理
（Principles of Operation）一书中动态地址转换（DAT）章节的定义呈现。

```

		/* KVM_EXIT_DCR */
		struct {
			__u32 dcrn;
			__u32 data;
			__u8  is_write;
		} dcr;

```
已废弃——曾用于 440 KVM。

```

		/* KVM_EXIT_OSI */
		struct {
			__u64 gprs[32];
		} osi;

```
MOL 使用了一种它称为“OSI”的特殊超级调用接口。为了启用它，我们捕获
超级调用并以该退出结构退出，其中包含了客户机的全部 GPR。

如果 exit_reason 为 KVM_EXIT_OSI，则表示 vCPU 触发了此类超级调用。
用户空间现在可以处理该超级调用，并在处理完成后按需修改 GPR。客户机
重新进入时，客户机所有 GPR 都将被此结构中的值替换。

```

		/* KVM_EXIT_PAPR_HCALL */
		struct {
			__u64 nr;
			__u64 ret;
			__u64 args[9];
		} papr_hcall;

```
在 64 位 PowerPC 上模拟 pSeries 分区（例如在 qemu 中使用“pseries”机型）
时使用。当客户机使用“sc 1”指令发起超级调用时发生。“nr”字段包含
超级调用号（取自客户机 R3），“args”包含参数（取自客户机 R4 - R12）。
用户空间应将返回码放入“ret”，并将任何额外的返回值放入 args[]。
可能的超级调用定义于 Power Architecture Platform Requirements（PAPR）
文档，可从 www.power.org 获取（访问需免费开发者注册）。

```

		/* KVM_EXIT_S390_TSCH */
		struct {
			__u16 subchannel_id;
			__u16 subchannel_nr;
			__u32 io_int_parm;
			__u32 io_int_word;
			__u32 ipb;
			__u8 dequeued;
		} s390_tsch;

```
s390 特有。当启用了 KVM_CAP_S390_CSS_SUPPORT 且拦截到 TEST SUBCHANNEL
时会发生此退出。如果 dequeued 被置位，则目标子通道上挂起的 I/O 中断
已被出队，并且 subchannel_id、subchannel_nr、io_int_parm 和 io_int_word
包含了该中断的参数。ipb 用于指令参数解码。

```

		/* KVM_EXIT_EPR */
		struct {
			__u32 epr;
		} epr;

```
在 FSL BookE PowerPC 芯片上，中断控制器有一条到核心的快速路径中断
应答通道。当核心成功递送一个中断时，它会自动用中断向量号填充 EPR
寄存器，并在中断控制器内部确认该中断。

当中断控制器位于用户空间时，我们需要通过它来完成中断确认周期，
以使用此退出获取下一个待递送的中断向量。

只要 KVM_CAP_PPC_EPR 被启用且有外部中断刚刚被递送到客户机，就会触发它。
用户空间应将已确认的中断向量放入“epr”字段。

```

		/* KVM_EXIT_SYSTEM_EVENT */
		struct {
  #define KVM_SYSTEM_EVENT_SHUTDOWN       1
  #define KVM_SYSTEM_EVENT_RESET          2
  #define KVM_SYSTEM_EVENT_CRASH          3
  #define KVM_SYSTEM_EVENT_WAKEUP         4
  #define KVM_SYSTEM_EVENT_SUSPEND        5
  #define KVM_SYSTEM_EVENT_SEV_TERM       6
  #define KVM_SYSTEM_EVENT_TDX_FATAL      7
			__u32 type;
                        __u32 ndata;
                        __u64 data[16];
		} system_event;

```
如果 exit_reason 为 KVM_EXIT_SYSTEM_EVENT，则表示 vCPU 通过某种架构
特定的机制（超级调用或某些特殊指令）触发了系统级事件。在 ARM64 上，
这是由 vCPU 基于 HVC 指令的 PSCI 调用触发的。

“type”字段描述了系统级事件的类型。
“type”的有效取值为：

 - KVM_SYSTEM_EVENT_SHUTDOWN——客户机请求关闭虚拟机。用户空间不必
   遵从该请求，如果遵从，也不必同步销毁虚拟机（即它可以在最终关闭
   发生之前再次调用 KVM_RUN）。
 - KVM_SYSTEM_EVENT_RESET——客户机请求重置虚拟机。与 SHUTDOWN 一样，
   用户空间可以选择忽略该请求，或者调度在未来的某个时刻进行重置，
   并可以再次调用 KVM_RUN。
 - KVM_SYSTEM_EVENT_CRASH——客户机发生了崩溃，并请求进行崩溃状态维护。
   用户空间可以选择忽略该请求，或者收集虚拟机内存核心转储和/或
   对虚拟机进行重置/关闭。
 - KVM_SYSTEM_EVENT_SEV_TERM——一个 AMD SEV 客户机请求终止。客户机
   GHCB 的客户机物理地址存储在 `data[^0^]` 中。
 - KVM_SYSTEM_EVENT_TDX_FATAL——TDX 客户机报告了致命错误状态。KVM 不做
   任何解析或转换，只是将 16 个通用寄存器按指令编码中 x86-64 通用
   寄存器 4 位索引的升序转储到用户空间，如 Intel SDM 中所定义。
 - KVM_SYSTEM_EVENT_WAKEUP——退出的 vCPU 处于挂起状态，KVM 识别到了
   唤醒事件。用户空间可以通过将该 vCPU 标记为可运行来接受该事件，
   或者拒绝它并再次调用 KVM_RUN。
 - KVM_SYSTEM_EVENT_SUSPEND——客户机请求挂起虚拟机。

如果 KVM_CAP_SYSTEM_EVENT_DATA 存在，则“data”字段可以包含该系统的
架构特定信息。data 数组中只有前 `ndata` 项（可能为零）是有效的。

 - 对于 arm64，如果客户机按照 PSCI 规范 v1.1 发出了 SYSTEM_RESET2 调用，
   则 data[^0^] 被设为 KVM_SYSTEM_EVENT_RESET_FLAG_PSCI_RESET2。

 - 对于 arm64，如果客户机按照 PSCI 规范 v1.3 发出了 SYSTEM_OFF2 调用，
   则 data[^0^] 被设为 KVM_SYSTEM_EVENT_SHUTDOWN_FLAG_PSCI_OFF2。

 - 对于 RISC-V，data[^0^] 被设为 `sbi_system_reset` 调用第二个参数的值。

早期版本的 Linux 在该结构中定义了一个 `flags` 成员。该字段现在已别名
为 `data[^0^]`。用户空间可以假定仅当 ndata 大于 0 时才会被写入。

### 对于 arm/arm64：


KVM_SYSTEM_EVENT_SUSPEND 退出通过 KVM_CAP_ARM_SYSTEM_SUSPEND 虚拟机能力
启用。如果客户机调用 PSCI SYSTEM_SUSPEND 函数，KVM 将以该事件类型退出
到用户空间。

用户空间全权负责按照 ARM DEN0022D.b 5.19“SYSTEM_SUSPEND”实现 PSCI
SYSTEM_SUSPEND 调用。KVM 在退出到用户空间之前不会改变 vCPU 的状态，因此
调用参数原地留在 vCPU 寄存器中。

用户空间_必须_对此类退出采取行动。它必须：

 - 接受客户机挂起虚拟机的请求。用户空间可以通过将被调用 vCPU 的状态
   设为 KVM_MP_STATE_SUSPENDED 来请求在内核中模拟挂起。被调用 vCPU 恢复时，
   用户空间必须按照传递给 PSCI 函数的参数配置 vCPU 状态。有关函数参数的
   详情请参见 ARM DEN0022D.b 5.19.1“预期用途”。

 - 拒绝客户机挂起虚拟机的请求。可能的返回值请参见 ARM DEN0022D.b 5.19.2
   “调用者职责”。

使用 PSCI SYSTEM_OFF2 调用的休眠在启用 PSCI v1.3 时启用。如果客户机调用
PSCI SYSTEM_OFF2 函数，KVM 将以 KVM_SYSTEM_EVENT_SHUTDOWN 事件类型退出到
用户空间，且 data[^0^] 被设为 KVM_SYSTEM_EVENT_SHUTDOWN_FLAG_PSCI_OFF2。
SYSTEM_OFF2 函数支持的休眠类型只有 HIBERNATE_OFF。

```

		/* KVM_EXIT_IOAPIC_EOI */
		struct {
			__u8 vector;
		} eoi;

```
表示 vCPU 的内核态本地 APIC 收到了一个电平触发型 IOAPIC 中断的 EOI。
此退出仅在 IOAPIC 实现于用户空间（即启用了 KVM_CAP_SPLIT_IRQCHIP）时
触发；用户空间 IOAPIC 应处理该 EOI，并在中断仍被断言时重新触发该中断。
vector 是收到 EOI 的 LAPIC 中断向量。

```

		struct kvm_hyperv_exit {
  #define KVM_EXIT_HYPERV_SYNIC          1
  #define KVM_EXIT_HYPERV_HCALL          2
  #define KVM_EXIT_HYPERV_SYNDBG         3
			__u32 type;
			__u32 pad1;
			union {
				struct {
					__u32 msr;
					__u32 pad2;
					__u64 control;
					__u64 evt_page;
					__u64 msg_page;
				} synic;
				struct {
					__u64 input;
					__u64 result;
					__u64 params[2];
				} hcall;
				struct {
					__u32 msr;
					__u32 pad2;
					__u64 control;
					__u64 status;
					__u64 send_page;
					__u64 recv_page;
					__u64 pending_page;
				} syndbg;
			} u;
		};
		/* KVM_EXIT_HYPERV */
                struct kvm_hyperv_exit hyperv;

```
表示 vCPU 退出到用户空间以处理与 Hyper-V 模拟相关的一些任务。

“type”的有效取值为：

 - KVM_EXIT_HYPERV_SYNIC——同步通知用户空间 Hyper-V SynIC 状态变更。
   该通知用于将 SynIC 事件/消息页重新映射，以及在用户空间中启用/禁用
   SynIC 消息/事件处理。

 - KVM_EXIT_HYPERV_SYNDBG——同步通知用户空间 Hyper-V 合成调试器状态变更。
   该通知用于更新 pending_page 位置，或发送控制命令（发送位于 send_page
   中的缓冲区，或接收缓冲区到 recv_page）。

```

		/* KVM_EXIT_ARM_NISV / KVM_EXIT_ARM_LDST64B */
		struct {
			__u64 esr_iss;
			__u64 fault_ipa;
		} arm_nisv;

```
- KVM_EXIT_ARM_NISV：

用于 arm64 系统。如果客户机访问了不在 memslot 中的内存，KVM 通常会返回
到用户空间并请求它代为进行 MMIO 模拟。但是，对于某些类别的指令，不提供
指令解码（方向、内存访问长度），而从虚拟机中取出并解码指令的过程在
内核中过于复杂。

历史上，发生这种情况时，KVM 会打印警告并杀死虚拟机。KVM 假设如果客户机
访问了非 memslot 内存，它就是在尝试进行 I/O，而该 I/O 无法被模拟，警告
消息也是据此措辞的。然而，更常见的情况是客户机 bug 导致访问了客户机
内存区域之外的地方，这应当导致更有意义的警告消息，并且如果访问没有落在
I/O 窗口内，则应在客户机中触发外部中止。

用户空间实现可以查询 KVM_CAP_ARM_NISV_TO_USER，并在创建虚拟机时启用该
能力。一旦完成，此类错误将改为以 KVM_EXIT_ARM_NISV 返回到用户空间，其中
ESR_EL2 中的有效位位于 esr_iss 字段，故障 IPA 位于 fault_ipa 字段。
用户空间可以通过从客户机内存中解码指令（如果它非常勇敢）来修复该访问
（如果是真正的 I/O 访问）并继续执行客户机，或者它可以选择挂起、转储或
重启客户机。

注意 KVM 不会像对 KVM_EXIT_MMIO 那样跳过故障指令，但如果用户空间决定
解码并模拟该指令，则必须模拟对处理状态的任何更改。

此特性对受保护的虚拟机不可用，因为用户空间无权访问执行模拟所需的
状态。相反，会直接向客户机注入一个数据中止异常。注意，尽管在受保护
虚拟机上下文之外查询时会报告 KVM_CAP_ARM_NISV_TO_USER，但在受保护虚拟机
文件描述符上查询时该特性不会暴露。

- KVM_EXIT_ARM_LDST64B：

用于 arm64 系统。当客户机在 memslot 之外使用 LD64B、ST64B、ST64BV、
ST64BV0 时，KVM 将以 KVM_EXIT_ARM_LDST64B 返回到用户空间，暴露相关的
ESR_EL2 信息和故障 IPA，与 KVM_EXIT_ARM_NISV 类似。

用户空间应完整模拟这些指令，包括：

 - 取出存储操作数，包括 ST64BV0 指令情况下的 ACCDATA_EL1
 - 处理客户机为大端序时的字节序问题
 - 模拟访问，包括访问未成功时递送异常
 - 在 ST64BV/ST64BV0 情况下提供返回值
 - 在加载情况下返回数据
 - 指令成功执行时递增 PC

注意对此模拟没有性能方面的预期，因为它涉及与客户机状态的大量交互。
然而，期望能够保留指令的语义，尤其是 64 字节访问的单副本原子性属性。

如果用户空间将 ID_AA64ISAR1_EL1.LS64 设为非零值（表示启用了 FEAT_LS64*），
则必须处理此退出原因。

```

		/* KVM_EXIT_X86_RDMSR / KVM_EXIT_X86_WRMSR */
		struct {
			__u8 error; /* user -> kernel */
			__u8 pad[7];
			__u32 reason; /* kernel -> user */
			__u32 index; /* kernel -> user */
			__u64 data; /* kernel <-> user */
		} msr;

```
用于 x86 系统。当虚拟机能力 KVM_CAP_X86_USER_SPACE_MSR 启用时，对会引发
KVM 内核代码 #GP 的寄存器的 MSR 访问，可能改为触发读方向的
KVM_EXIT_X86_RDMSR 退出和写方向的 KVM_EXIT_X86_WRMSR 退出。

“reason”字段指定了 MSR 拦截发生的原因。用户空间只会在通过 ENABLE_CAP
请求了特定原因时才会收到 MSR 退出。当前有效的退出原因有：

============================ ========================================
 KVM_MSR_EXIT_REASON_UNKNOWN 访问 KVM 未知的 MSR
 KVM_MSR_EXIT_REASON_INVAL   访问无效 MSR 或保留位
 KVM_MSR_EXIT_REASON_FILTER  被 KVM_X86_SET_MSR_FILTER 拦截的访问
============================ ========================================

对于 KVM_EXIT_X86_RDMSR，“index”字段告诉用户空间客户机想要读取哪个 MSR。
要以一次成功的读取响应此请求，用户空间将相应数据写入“data”字段，并且
必须继续执行客户机以确保读取的数据被传送进客户机寄存器状态。

如果 RDMSR 请求不成功，用户空间通过在“error”字段中写入“1”来指示。
这会在 VCPU 再次被执行时向客户机注入一个 #GP。

对于 KVM_EXIT_X86_WRMSR，“index”字段告诉用户空间客户机想要写入哪个 MSR。
处理完该事件后，用户空间必须继续执行 vCPU。如果 MSR 写入不成功，用户空间
也将“error”字段设为“1”。

有关与 MSR 过滤交互的细节，请参阅 KVM_X86_SET_MSR_FILTER。

```

		struct kvm_xen_exit {
  #define KVM_EXIT_XEN_HCALL          1
			__u32 type;
			union {
				struct {
					__u32 longmode;
					__u32 cpl;
					__u64 input;
					__u64 result;
					__u64 params[6];
				} hcall;
			} u;
		};
		/* KVM_EXIT_XEN */
                struct kvm_hyperv_exit xen;

```
表示 vCPU 退出到用户空间以处理与 Xen 模拟相关的一些任务。

“type”的有效取值为：

  - KVM_EXIT_XEN_HCALL——同步通知用户空间 Xen 超级调用。用户空间应当在
    再次调用 KVM_RUN 之前将超级调用结果放入相应字段。

```

		/* KVM_EXIT_RISCV_SBI */
		struct {
			unsigned long extension_id;
			unsigned long function_id;
			unsigned long args[6];
			unsigned long ret[2];
		} riscv_sbi;

```
如果退出原因为 KVM_EXIT_RISCV_SBI，则表示 VCPU 执行了不由 KVM RISC-V
内核模块处理的 SBI 调用。SBI 调用的细节可在 kvm_run 结构的“riscv_sbi”
成员中获得。“riscv_sbi”的“extension_id”字段表示 SBI 扩展 ID，而
“function_id”字段表示给定 SBI 扩展的函数 ID。“riscv_sbi”的“args”数组
字段表示 SBI 调用的参数，“ret”数组表示返回值。用户空间应在恢复 VCPU
之前更新 SBI 调用的返回值。有关 RISC-V SBI 规范的更多细节，请参阅
https://github.com/riscv/riscv-sbi-doc。

```

		/* KVM_EXIT_MEMORY_FAULT */
		struct {
  #define KVM_MEMORY_EXIT_FLAG_PRIVATE	(1ULL << 3)
			__u64 flags;
			__u64 gpa;
			__u64 size;
		} memory_fault;

```
KVM_EXIT_MEMORY_FAULT 表示 vCPU 遇到了 KVM 无法解析的内存故障。“gpa”和
“size”（以字节为单位）描述了故障的客户机物理地址范围 [gpa, gpa + size)。
“flags”字段描述了可能与故障相关的访问属性：

 - KVM_MEMORY_EXIT_FLAG_PRIVATE——置位时，表示内存故障发生在私有内存
   访问上；清零时，表示故障发生在共享访问上。

注意！KVM_EXIT_MEMORY_FAULT 在所有 KVM 退出原因中独一无二，它伴随的返回
码是“-1”而非“0”！当 KVM 以 KVM_EXIT_MEMORY_FAULT 退出时，errno 将始终
设为 EFAULT 或 EHWPOISON，对于所有其他错误码，用户空间应假定
kvm_run.exit_reason 是过期/未定义的。

```

    /* KVM_EXIT_NOTIFY */
    struct {
  #define KVM_NOTIFY_CONTEXT_INVALID	(1 << 0)
      __u32 flags;
    } notify;

```
用于 x86 系统。当虚拟机能力 KVM_CAP_X86_NOTIFY_VMEXIT 启用时，如果在 VM
非根模式下经过指定时长仍无事件窗口发生，则生成 VM 退出。一旦在启用该能力时
设置了 KVM_X86_NOTIFY_VMEXIT_USER，它将以退出原因 KVM_EXIT_NOTIFY 退出到
用户空间以进行进一步处理。“flags”字段包含更详细的信息。

“flags”的有效取值为：

  - KVM_NOTIFY_CONTEXT_INVALID——VM 上下文已损坏且在 VMCS 中无效。如果恢复
    目标虚拟机，将导致未知结果。

```

		/* KVM_EXIT_TDX */
		struct {
			__u64 flags;
			__u64 nr;
			union {
				struct {
					u64 ret;
					u64 data[5];
				} unknown;
				struct {
					u64 ret;
					u64 gpa;
					u64 size;
				} get_quote;
				struct {
					u64 ret;
					u64 leaf;
					u64 r11, r12, r13, r14;
				} get_tdvmcall_info;
				struct {
					u64 ret;
					u64 vector;
				} setup_event_notify;
			};
		} tdx;

```
处理来自客户机的 TDVMCALL。KVM 基于 Guest-Hypervisor 通信接口（GHCI）规范
转发选定的 TDVMCALL；KVM 以最小改动将这些请求桥接到用户空间 VMM，将输入
放入 union，并在重新进入时复制回客户机。

flags 当前始终为零，而 `nr` 包含来自 R11 寄存器的 TDVMCALL 号。union 的
其余字段提供了 TDVMCALL 的输入和输出。当前定义了以下 `nr` 值：

 - `TDVMCALL_GET_QUOTE`：客户机已请求生成由运行在宿主上的 TD-Quoting
   飞地（Enclave）签名的 TD-Quote。参数和返回值位于 union 的 `get_quote`
   字段。`gpa` 字段和 `size` 指定了客户机物理地址（未设置共享位）以及
   共享内存缓冲区的大小，TDX 客户机通过该缓冲区传递 TD Report。`ret`
   字段表示 GetQuote 请求的返回值。当请求成功入队后，TDX 客户机可以轮询
   共享内存区域中的状态字段，以检查 Quote 生成是否完成。完成后，生成的
   Quote 通过同一缓冲区返回。

 - `TDVMCALL_GET_TD_VM_CALL_INFO`：客户机已请求 TDVMCALL 的支持状态。给定
   leaf 的输出值应放入 union 的 `get_tdvmcall_info` 字段中从 `r11` 到
   `r14` 的字段。

 - `TDVMCALL_SETUP_EVENT_NOTIFY_INTERRUPT`：客户机已请求为向量 `vector`
   设置通知中断。

KVM 将来可能会增加对更多值的支持，这些值可能导致用户空间退出，即使没有
调用 `KVM_ENABLE_CAP` 或类似接口。在这种情况下，它将带着已有效的输出字段
进入；通常情况下，union 的 `unknown.ret` 字段为
`TDVMCALL_STATUS_SUBFUNC_UNSUPPORTED`。如果用户空间不希望支持某个 TDVMCALL，
则无需做任何处理。

```

		/* KVM_EXIT_ARM_SEA */
		struct {
  #define KVM_EXIT_ARM_SEA_FLAG_GPA_VALID   (1ULL << 0)
			__u64 flags;
			__u64 esr;
			__u64 gva;
			__u64 gpa;
		} arm_sea;

```
用于 arm64 系统。当虚拟机能力 `KVM_CAP_ARM_SEA_TO_USER` 启用时，如果客户机
访问导致了同步外部中止（SEA）且宿主 APEI 无法处理该 SEA，KVM 会退出到
用户空间。

`esr` 被设为从进入 KVM 的异常中取出的 ESR_EL2 的净化值，包含以下字段：

 - `ESR_EL2.EC`
 - `ESR_EL2.IL`
 - `ESR_EL2.FnV`
 - `ESR_EL2.EA`
 - `ESR_EL2.CM`
 - `ESR_EL2.WNR`
 - `ESR_EL2.FSC`
 - `ESR_EL2.SET`（当为 VM 实现了 FEAT_RAS 时）

当 `ESR_EL2.FnV == 0` 时，`gva` 被设为从进入 KVM 的异常中取出的 FAR_EL2
的值。否则，`gva` 的值未知。

当 `KVM_EXIT_ARM_SEA_FLAG_GPA_VALID` 标志置位时，`gpa` 被设为从进入 KVM 的
异常中取出的故障 IPA。否则，`gpa` 的值未知。

```

		/* 固定 union 的大小。 */
		char padding[256];
	};

	/*
	 * kvm 与用户空间之间共享的寄存器。
	 * kvm_valid_regs 指定由宿主设置的寄存器类别
	 * kvm_dirty_regs 指定由用户空间弄脏的寄存器类别
	 * struct kvm_sync_regs 是架构特定的，kvm_valid_regs 和
	 * kvm_dirty_regs 的位也是架构特定的
	 */
	__u64 kvm_valid_regs;
	__u64 kvm_dirty_regs;
	union {
		struct kvm_sync_regs regs;
		char padding[SYNC_REGS_SIZE_BYTES];
	} s;

```
如果定义了 KVM_CAP_SYNC_REGS，这些字段允许用户空间不必调用 SET/GET_*REGS
即可访问某些客户机寄存器。因此，如果用户空间需要处理退出，我们可以避免
一些系统调用开销。用户空间可以通过检查 kvm_valid_regs 的特定位来查询该
结构的有效性。这些位是架构特定的，通常定义一组寄存器的有效性（例如，一位
对应通用寄存器）。

请注意，内核被允许使用 kvm_run 结构作为某些寄存器类型的主存储。因此，即使
kvm_dirty_regs 中相应的位未置位，内核也可能使用 kvm_run 中的值。

```

		/* KVM_EXIT_SNP_REQ_CERTS */
		struct kvm_exit_snp_req_certs {
			__u64 gpa;
			__u64 npages;
			__u64 ret;
		};

```
KVM_EXIT_SNP_REQ_CERTS 表示一个启用了证书获取的 SEV-SNP 客户机（见
KVM_SEV_SNP_ENABLE_REQ_CERTS）生成了一个扩展型客户机请求 NAE #VMGEXIT
（SNP_GUEST_REQUEST），消息类型为 MSG_REPORT_REQ，即已从固件请求了证明
报告，并希望由虚拟机监控器随请求一并提供与证明报告签名相对应的证书数据。

为了允许用户空间提供证书，“gpa”和“npages”原样从客户机请求转发
（分别为 RAX 和 RBX GHCB 字段）。“ret”不是来自 KVM 的“输出”，退出时
始终为“0”。KVM 在退出到用户空间之前会验证“gpa”是 4KiB 对齐的，但
除此之外不会验证来自客户机的信息。

在下一次 KVM_RUN 时（例如用户空间已服务该请求或没有服务之后），KVM 将
完成 #VMGEXIT，使用“ret”字段确定是向客户机发信号成功还是失败，失败时
通过 SW_EXITINFO2 告知何种原因码。如果“ret”被设为不支持的值（见下表），
KVM_RUN 将以 -EINVAL 失败。对于“ret”为“ENOSPC”的情况，KVM 还消费“npages”
字段，即用户空间可以用该字段告知客户机保存全部证书数据所需的页数。

支持的“ret”值及其对应的 SW_EXITINFO2 编码：

  ======     =============================================================
  0          0x0，即成功。KVM 将向 SNP 固件发出 SNP_GUEST_REQUEST 命令
  ENOSPC     0x0000000100000000，即客户机页不足以容纳证书表和证书数据。
             KVM 还会在 GHBC 中将 RBX 字段设为“npages”。
  EAGAIN     0x0000000200000000，即宿主正忙，客户机应重试该请求。
  EIO        0xffffffff00000000，用于所有其他错误（此返回码是 KVM 定义的
             虚拟机监控器值，如 GHCB 所允许）
  ======     =============================================================


## 6. 可在 vCPU 上启用的能力


有某些能力在启用时会改变虚拟 CPU 或虚拟机的行为。要启用它们，请参阅
KVM_ENABLE_CAP。

下面你可以找到一份能力列表，以及启用它们时对 vCPU 或虚拟机的影响。

随描述一并提供以下信息：

  Architectures（架构）：
      哪些指令集架构提供此 ioctl。x86 同时包含 i386 和 x86_64。

  Target（目标）：
      这是每 vCPU 还是每 VM 的能力。

  Parameters（参数）：
      该能力接受哪些参数。

  Returns（返回值）：
      返回的值。通用错误码（EBADF、ENOMEM、EINVAL）不做详细说明，但具有
      特定含义的错误会予以说明。


### 6.1 KVM_CAP_PPC_OSI


:Architectures: ppc
:Target: vcpu
:Parameters: none
:Returns: 0 on success; -1 on error

此能力启用 OSI 超级调用的拦截，否则这些调用会被当作注入到客户机的普通
系统调用。OSI 超级调用由 Mac-on-Linux 发明，用于在客户机和宿主之间提供
标准化的通信机制。

启用此能力时，可能发生 KVM_EXIT_OSI。


### 6.2 KVM_CAP_PPC_PAPR


:Architectures: ppc
:Target: vcpu
:Parameters: none
:Returns: 0 on success; -1 on error

此能力启用 PAPR 超级调用的拦截。PAPR 超级调用使用超级调用指令“sc 1”发起。

它还将客户机特权级别设为“supervisor”模式。通常客户机运行在“hypervisor”
特权模式下，但缺少一些特性。

除以上之外，它还改变了 SDR1 的语义。在此模式下，SDR1 的 HTAB 地址部分
包含 HVA 而非 GPA，因为 PAPR 对客户机隐藏了 HTAB。

启用此能力时，可能发生 KVM_EXIT_PAPR_HCALL。


### 6.3 KVM_CAP_SW_TLB


:Architectures: ppc
:Target: vcpu
:Parameters: args[^0^] 是一个 struct kvm_config_tlb 的地址
:Returns: 0 on success; -1 on error

```

  struct kvm_config_tlb {
	__u64 params;
	__u64 array;
	__u32 mmu_type;
	__u32 array_len;
  };

```
配置虚拟 CPU 的 TLB 数组，在用户空间和 KVM 之间建立共享内存区域。“params”
和“array”字段是 mmu 类型特定数据结构的用户空间地址。“array_len”字段
是一个安全机制，应设为用户空间为数组保留的内存大小（以字节计）。它至少
必须是“mmu_type”和“params”所要求的大小。

当 KVM_RUN 处于活动状态时，共享区域由 KVM 控制。其内容未定义，用户空间
对其进行的任何修改都会导致有界的未定义行为。

从 KVM_RUN 返回时，共享区域将反映客户机 TLB 的当前状态。如果用户空间
进行任何更改，它必须在再次对此 vcpu 调用 KVM_RUN 之前调用 KVM_DIRTY_TLB
来告知 KVM 哪些条目已被更改。

对于 mmu 类型 KVM_MMU_FSL_BOOKE_NOHV 和 KVM_MMU_FSL_BOOKE_HV：

 - “params”字段的类型为“struct kvm_book3e_206_tlb_params”。
 - “array”字段指向一个“struct kvm_book3e_206_tlb_entry”类型的数组。
 - 该数组由第一个 TLB 中的全部条目组成，后跟第二个 TLB 中的全部条目。
 - 在一个 TLB 内部，条目先按集合号递增排序。在一个集合内部，条目按
   路（way，递增的 ESEL）排序。
 - 确定 TLB0 中集合号的哈希为：(MAS2 >> 12) & (num_sets - 1)，其中
   “num_sets”是 tlb_sizes[] 值除以 tlb_ways[] 值。
 - mas1 的 tsize 字段在 TLB0 上应设为 4K，尽管硬件对此值忽略不计。

### 6.4 KVM_CAP_S390_CSS_SUPPORT


:Architectures: s390
:Target: vcpu
:Parameters: none
:Returns: 0 on success; -1 on error

此能力启用对通道 I/O 指令处理支持。

TEST PENDING INTERRUPTION 以及 TEST SUBCHANNEL 的中断部分在内核中处理，
而其他 I/O 指令则传递给用户空间。

启用此能力时，会在 TEST SUBCHANNEL 拦截时发生 KVM_EXIT_S390_TSCH。

注意，即使此能力是按 vCPU 启用的，整个虚拟机都会受到影响。

### 6.5 KVM_CAP_PPC_EPR


:Architectures: ppc
:Target: vcpu
:Parameters: args[^0^] 定义代理设施是否处于活动状态
:Returns: 0 on success; -1 on error

此能力启用或禁用通过外部代理设施递送中断。

启用时（args[^0^] != 0），每次客户机收到一个外部中断递送时，它会自动
以 KVM_EXIT_EPR 退出进入用户空间，以接收最顶层的终端向量。

禁用时（args[^0^] == 0），行为如同此设施不受支持。

启用此能力时，可能发生 KVM_EXIT_EPR。

### 6.6 KVM_CAP_IRQ_MPIC


:Architectures: ppc
:Parameters: args[^0^] 是 MPIC 设备 fd；
             args[^1^] 是此 vcpu 的 MPIC CPU 号

此能力将 vcpu 连接到内核态 MPIC 设备。

### 6.7 KVM_CAP_IRQ_XICS


:Architectures: ppc
:Target: vcpu
:Parameters: args[^0^] 是 XICS 设备 fd；
             args[^1^] 是此 vcpu 的 XICS CPU 号（server ID）

此能力将 vcpu 连接到内核态 XICS 设备。

### 6.8 KVM_CAP_S390_IRQCHIP


:Architectures: s390
:Target: vm
:Parameters: none

此能力启用 s390 的内核态 irqchip。详情请参阅“4.24 KVM_CREATE_IRQCHIP”。

### 6.9 KVM_CAP_MIPS_FPU


:Architectures: mips
:Target: vcpu
:Parameters: args[^0^] 为将来保留（应为 0）。

此能力允许客户机使用宿主的浮点单元（FPU）。它允许设置 Config1.FP 位以在
客户机中启用 FPU。一旦完成，就可以访问 `KVM_REG_MIPS_FPR_**` 和
`KVM_REG_MIPS_FCR_**` 寄存器（取决于当前客户机 FPU 寄存器模式），并且
Status.FR、Config5.FRE 位可通过 KVM API 以及从客户机访问，前提是 FPU
支持它们。

### 6.10 KVM_CAP_MIPS_MSA


:Architectures: mips
:Target: vcpu
:Parameters: args[^0^] 为将来保留（应为 0）。

此能力允许客户机使用 MIPS SIMD 架构（MSA）。它允许设置 Config3.MSAP 位以
在客户机中启用 MSA 的使用。一旦完成，就可以访问 `KVM_REG_MIPS_VEC_**` 和
`KVM_REG_MIPS_MSA_**` 寄存器，并且 Config5.MSAEn 位可通过 KVM API 以及从
客户机访问。

### 6.74 KVM_CAP_SYNC_REGS


:Architectures: s390, x86
:Target: s390：始终启用，x86：vcpu
:Parameters: none
:Returns: x86：KVM_CHECK_EXTENSION 返回一个位数组，指示支持哪些寄存器集
          （位域定义于 arch/x86/include/uapi/asm/kvm.h）。

如上文 kvm_run 中 kvm_sync_regs 结构信息所述，KVM_CAP_SYNC_REGS
“允许[用户空间]不必调用 SET/GET_*REGS 即可访问某些客户机寄存器”。这通过
消除设置/获取寄存器值的重复 ioctl 调用减少了开销。当用户空间正在进行
同步的客户机状态修改（例如，在用户空间中模拟和/或拦截指令）时，这一点
尤为重要。

有关 s390 的细节，请参阅源代码。

对于 x86：

- 要复制到 kvm_run 的寄存器集可由用户空间选择（而不是每次退出都复制出
  所有寄存器集）。
- 除 regs 和 sregs 外，还可使用 vcpu_events。

对于 x86，struct kvm_run 的“kvm_valid_regs”字段被重载，充当由用户空间
设置的输入位数组字段，以指示在下一次退出时要复制出的特定寄存器集。

为了指示用户空间已修改了应复制进 vCPU 的值，必须设置所有架构通用的位数组
字段“kvm_dirty_regs”。这使用与“kvm_valid_regs”字段相同的位标志完成。
如果未设置 dirty 位，则即使寄存器集值已被修改，也不会被复制进 vCPU。

位数组中未使用的位字段必须设为零。

```

  struct kvm_sync_regs {
        struct kvm_regs regs;
        struct kvm_sregs sregs;
        struct kvm_vcpu_events events;
  };

```
### 6.75 KVM_CAP_PPC_IRQ_XIVE


:Architectures: ppc
:Target: vcpu
:Parameters: args[^0^] 是 XIVE 设备 fd；
             args[^1^] 是此 vcpu 的 XIVE CPU 号（server ID）

此能力将 vcpu 连接到内核态 XIVE 设备。

### 6.76 KVM_CAP_HYPERV_SYNIC


:Architectures: x86
:Target: vcpu

此能力，若 KVM_CHECK_EXTENSION 指示其可用，意味着内核实现了 Hyper-V 合成
中断控制器（SynIC）。Hyper-V SynIC 用于支持基于 Windows Hyper-V 的客户机
半虚拟化驱动（VMBus）。

为了使用 SynIC，必须通过 vcpu fd 上的 KVM_ENABLE_CAP ioctl 设置此能力来
激活它。注意这会禁用 APIC 硬件虚拟化的使用（即使 CPU 支持），因为它与
SynIC 的自动 EOI 行为不兼容。

### 6.77 KVM_CAP_HYPERV_SYNIC2


:Architectures: x86
:Target: vcpu

此能力启用更新版本的 Hyper-V 合成中断控制器（SynIC）。与 KVM_CAP_HYPERV_SYNIC
唯一的区别是，当通过写入相应的 MSR 启用时，KVM 不会清除 SynIC 消息和事件
标志页。

### 6.78 KVM_CAP_HYPERV_DIRECT_TLBFLUSH


:Architectures: x86
:Target: vcpu

此能力表示运行在 Hyper-V 虚拟机监控器之上的 KVM 为其客户机启用直接 TLB
刷新，意味着 TLB 刷新超级调用由 0 级虚拟机监控器（Hyper-V）处理，绕过
KVM。由于 Hyper-V 和 KVM 之间超级调用参数的 ABI 不同，启用此能力会有效
禁用 KVM 的所有超级调用处理（因为某些 KVM 超级调用可能被 Hyper-V 误当作
TLB 刷新超级调用），因此用户空间应在 CPUID 中禁用 KVM 标识，只暴露 Hyper-V
标识。在这种情况下，客户机以为自己运行在 Hyper-V 上，并且只使用 Hyper-V
超级调用。

### 6.79 KVM_CAP_HYPERV_ENFORCE_CPUID


:Architectures: x86
:Target: vcpu

启用时，KVM 将根据 Hyper-V CPUID 特性叶中的位，禁用提供给客户机的模拟
Hyper-V 特性。否则，只要在 HYPERV_CPUID_INTERFACE（0x40000001）叶中设置了
Hyper-V 标识，所有当前已实现的 Hyper-V 特性都会无条件提供。

### 6.80 KVM_CAP_ENFORCE_PV_FEATURE_CPUID


:Architectures: x86
:Target: vcpu

启用时，KVM 将根据 KVM_CPUID_FEATURES CPUID 叶（0x40000001）中的位，禁用
提供给客户机的半虚拟化特性。否则，客户机可能使用半虚拟化特性，而不论
实际通过 CPUID 叶暴露了什么。



## 7. 可在 VM 上启用的能力


有某些能力在启用时会改变虚拟机的行为。要启用它们，请参阅 KVM_ENABLE_CAP
一节。下面你可以找到一份能力列表，以及启用它们时对 VM 的影响。

随描述一并提供以下信息：

  Architectures（架构）：
      哪些指令集架构提供此 ioctl。x86 同时包含 i386 和 x86_64。

  Parameters（参数）：
      该能力接受哪些参数。

  Returns（返回值）：
      返回的值。通用错误码（EBADF、ENOMEM、EINVAL）不做详细说明，但具有
      特定含义的错误会予以说明。


### 7.1 KVM_CAP_PPC_ENABLE_HCALL


:Architectures: ppc
:Parameters: args[^0^] 是 sPAPR hcall 号；
	     args[^1^] 为 0 表示禁用，1 表示启用内核态处理

此能力控制各个 sPAPR 超级调用（hcall）是由内核处理还是不处理。启用或
禁用某个 hcall 的内核态处理在整个 VM 范围内生效。创建时，会启用一组初始
的 hcall 进行内核态处理，这些 hcall 由在本能力实现之前就已经实现了内核态
处理函数的那些超级调用组成。如果禁用，内核将不会尝试处理该 hcall，而是
总是退出到用户空间处理它。注意，启用一组相关 hcall 中的某些而禁用另一些
可能没有意义，但 KVM 不会阻止用户空间这样做。

如果指定的 hcall 号不是具有内核态实现的那个，则 KVM_ENABLE_CAP ioctl 将
以 EINVAL 错误失败。

### 7.2 KVM_CAP_S390_USER_SIGP


:Architectures: s390
:Parameters: none

此能力控制哪些 SIGP 顺序将完全在用户空间处理。启用此能力后，所有快速顺序
将完全在内核中处理：

- SENSE
- SENSE RUNNING
- EXTERNAL CALL
- EMERGENCY SIGNAL
- CONDITIONAL EMERGENCY SIGNAL

所有其他顺序将完全在用户空间处理。

只有特权操作异常会在内核中（或在拦截之前的硬件中）检查。如果未启用此能力，
则使用旧的 SIGP 顺序处理方式（部分在内核、部分在用户空间）。

### 7.3 KVM_CAP_S390_VECTOR_REGISTERS


:Architectures: s390
:Parameters: none
:Returns: 0 on success, negative value on error

允许使用随 z13 处理器引入的向量寄存器，并为主机和用户空间之间的同步提供支持。
如果机器不支持向量，将返回 -EINVAL。

### 7.4 KVM_CAP_S390_USER_STSI


:Architectures: s390
:Parameters: none

此能力允许 STSI 指令的后处理器。在内核中初步处理之后，KVM 以 KVM_EXIT_S390_STSI
退出到用户空间，以允许用户空间插入进一步的数据。

在退出到用户空间之前，kvm 处理器应填充 kvm_run 的 s390_stsi 字段：

```

  struct {
	__u64 addr;
	__u8 ar;
	__u8 reserved;
	__u8 fc;
	__u8 sel1;
	__u16 sel2;
  } s390_stsi;

  @addr - STSI SYSIB 的客户机地址
  @fc   - 功能码
  @sel1 - 选择器 1
  @sel2 - 选择器 2
  @ar   - 访问寄存器号

```
KVM 处理器应以 rc = -EREMOTE 退出到用户空间。

### 7.5 KVM_CAP_SPLIT_IRQCHIP


:Architectures: x86
:Parameters: args[^0^] - 为用户空间 IOAPIC 保留的路由数
:Returns: 0 on success, -1 on error

在内核中为每个处理器创建一个本地 apic。如果用户空间 VMM 希望模拟 IOAPIC 和
PIC（以及 PIT，尽管 PIT 必须单独启用），可以用它替代 KVM_CREATE_IRQCHIP。

此能力还启用了内核态的中断请求路由；当启用 KVM_CAP_SPLIT_IRQCHIP 时，IRQ
路由表中只使用 KVM_IRQ_ROUTING_MSI 类型的路由。前 args[^0^] 个 MSI 路由为
IOAPIC 引脚保留。每当 LAPIC 收到这些路由的 EOI 时，就会向用户空间报告一个
KVM_EXIT_IOAPIC_EOI vmexit。

如果已创建了 VCPU，或者 irqchip 已经在内核中（即已经调用过
KVM_CREATE_IRQCHIP），则失败。

### 7.6 KVM_CAP_S390_RI


:Architectures: s390
:Parameters: none

允许使用随 zEC12 处理器引入的运行时指令（runtime-instrumentation）。如果
机器不支持运行时指令，将返回 -EINVAL。如果已创建了 VCPU，将返回 -EBUSY。
### 7.7 KVM_CAP_X2APIC_API


:Architectures: x86
:Parameters: args[^0^] - 应启用的特性
:Returns: 0 on success, -EINVAL when args[^0^] contains invalid features

```

  #define KVM_X2APIC_API_USE_32BIT_IDS                          (1ULL << 0)
  #define KVM_X2APIC_API_DISABLE_BROADCAST_QUIRK                (1ULL << 1)
  #define KVM_X2APIC_ENABLE_SUPPRESS_EOI_BROADCAST              (1ULL << 2)
  #define KVM_X2APIC_DISABLE_SUPPRESS_EOI_BROADCAST             (1ULL << 3)

```
启用 KVM_X2APIC_API_USE_32BIT_IDS 改变了 KVM_SET_GSI_ROUTING、KVM_SIGNAL_MSI、
KVM_SET_LAPIC 和 KVM_GET_LAPIC 的行为，允许使用 32 位 APIC ID。请参阅各自
章节中的 KVM_CAP_X2APIC_API。

必须启用 KVM_X2APIC_API_DISABLE_BROADCAST_QUIRK，x2APIC 才能在逻辑模式或
超过 255 个 VCPU 的情况下工作。否则，即使在 x2APIC 模式下，KVM 也会把 0xff
当作广播，以支持没有中断重映射的物理 x2APIC。这在逻辑模式下是不可取的，因为
0xff 表示 cluster 0 中的 CPU 0-7。

设置 KVM_X2APIC_ENABLE_SUPPRESS_EOI_BROADCAST 指示 KVM 启用抑制 EOI 广播
（Suppress EOI Broadcasts）。当客户机在 SPIV 寄存器中设置了抑制 EOI 广播位时，
KVM 会向客户机通告对抑制 EOI 广播的支持，并在客户机设置该位时抑制 LAPIC
的 EOI 广播。此标志仅在使用 split IRQCHIP 时受支持。

设置 KVM_X2APIC_DISABLE_SUPPRESS_EOI_BROADCAST 则完全禁用对抑制 EOI 广播的
支持，即指示 KVM 不要向客户机通告支持。

现代 VMM 应当启用 KVM_X2APIC_ENABLE_SUPPRESS_EOI_BROADCAST 或
KVM_X2APIC_DISABLE_SUPPRESS_EOI_BROADCAST 之一。否则将使用 KVM 的遗留古怪
行为：在 split IRQCHIP 模式下，KVM 会向客户机通告对抑制 EOI 广播的支持，
但实际上并不抑制 EOI 广播；在内核态 IRQCHIP 模式下，KVM 不会通告对抑制 EOI
广播的支持。

同时设置 KVM_X2APIC_ENABLE_SUPPRESS_EOI_BROADCAST 和
KVM_X2APIC_DISABLE_SUPPRESS_EOI_BROADCAST 将以 EINVAL 错误失败，在未使用
split IRQCHIP 的情况下设置 KVM_X2APIC_ENABLE_SUPPRESS_EOI_BROADCAST 同样会
失败。

### 7.8 KVM_CAP_S390_USER_INSTR0


:Architectures: s390
:Parameters: none

启用此能力后，非法的指令 0x0000（2 字节）将被拦截并转发到用户空间。用户空间
可以利用此机制实现例如 2 字节软件断点。内核不会为这些指令注入操作异常，
用户空间必须自行处理。

即使在 VCPU 已被创建并正在运行的情况下，也可以动态启用此能力。

### 7.9 KVM_CAP_S390_GS


:Architectures: s390
:Parameters: none
:Returns: 0 on success; -EINVAL if the machine does not support
          guarded storage; -EBUSY if a VCPU has already been created.

允许 KVM 客户机使用守护存储（guarded storage）。

### 7.10 KVM_CAP_S390_AIS


:Architectures: s390
:Parameters: none

允许使用适配器中断抑制（adapter-interruption suppression）。
:Returns: 0 on success; -EBUSY if a VCPU has already been created.

### 7.11 KVM_CAP_PPC_SMT


:Architectures: ppc
:Parameters: vsmt_mode, flags

在 VM 上启用此能力为用户空间提供了一种设置期望的虚拟 SMT 模式（即每个虚拟
核心的虚拟 CPU 数）的方法。虚拟 SMT 模式 vsmt_mode 必须是 1 到 8 之间的 2 的
幂。在 POWER8 上，vsmt_mode 还不得大于宿主每个子核的线程数。当前 flags 必须
为 0。成功调用以启用此能力后，当随后为 VM 查询 KVM_CAP_PPC_SMT 能力时，将
返回 vsmt_mode。此能力仅由 HV KVM 支持，并且只能在创建任何 VCPU 之前设置。
KVM_CAP_PPC_SMT_POSSIBLE 能力指示哪些虚拟 SMT 模式可用。

### 7.12 KVM_CAP_PPC_FWNMI


:Architectures: ppc
:Parameters: none

借助此能力，客户机地址空间中的机器检查异常将导致 KVM 以 NMI 退出原因退出
客户机。这使得 QEMU 能够构建错误日志并跳转到客户机内核注册的机器检查处理
例程。若没有此能力，KVM 将跳转到客户机的 0x200 中断向量。

### 7.13 KVM_CAP_X86_DISABLE_EXITS


:Architectures: x86
:Parameters: args[^0^] 定义禁用哪些退出
:Returns: 0 on success, -EINVAL when args[^0^] contains invalid exits
          or if any vCPUs have already been created

```

  #define KVM_X86_DISABLE_EXITS_MWAIT            (1 << 0)
  #define KVM_X86_DISABLE_EXITS_HLT              (1 << 1)
  #define KVM_X86_DISABLE_EXITS_PAUSE            (1 << 2)
  #define KVM_X86_DISABLE_EXITS_CSTATE           (1 << 3)
  #define KVM_X86_DISABLE_EXITS_APERFMPERF       (1 << 4)

```
在 VM 上启用此能力为用户空间提供了一种不再拦截某些指令的方法，从而在某些
工作负载下改善延迟，建议在 vCPU 关联到专用物理 CPU 时使用。未来可以添加更多
位；用户空间只需将 KVM_CHECK_EXTENSION 的结果传给 KVM_ENABLE_CAP 即可禁用
所有此类 vmexit。

如果禁用了 HLT 退出，请勿启用 KVM_FEATURE_PV_UNHALT。

虚拟化 `IA32_APERF` 和 `IA32_MPERF` MSR 需要的不仅仅是禁用 APERF/MPERF 退出。
虽然 Intel 和 AMD 都记录了这些 MSR 的严格使用条件——强调只有它们在一段时间
区间（T0 到 T1）内增量的比值在架构上有定义——但简单地透传这些 MSR 仍可能
产生不正确的比值。

如果在 T0 和 T1 之间发生以下情况，就可能出现这个错误的比值：

1. vCPU 线程在逻辑处理器之间迁移。
2. 发生实时迁移或挂起/恢复操作。
3. 另一个任务共享 vCPU 的逻辑处理器。
4. 模拟了低于 C0 的 C-state（例如通过 HLT 拦截）。
5. 客户机 TSC 频率与宿主 TSC 频率不匹配。

由于这些复杂性，KVM 不会自动将此透传能力与客户机 CPUID 位
`CPUID.6:ECX.APERFMPERF[bit 0]` 相关联。认为此机制足以虚拟化 `IA32_APERF`
和 `IA32_MPERF` MSR 的用户空间 VMM 必须显式设置客户机 CPUID 位。


### 7.14 KVM_CAP_S390_HPAGE_1M


:Architectures: s390
:Parameters: none
:Returns: 0 on success, -EINVAL if hpage module parameter was not set
	  or cmma is enabled, or the VM has the KVM_VM_S390_UCONTROL
	  flag set

借助此能力，可以为 VM 启用 KVM 对通过 hugetlbfs 用 1M 页做内存后端的支持。
启用该能力后，cmma 不能再被启用，pfmfi 和存储键解释也被禁用。如果 cmma 已经
被启用或者 hpage 模块参数未设为 1，则返回 -EINVAL。

虽然通常可以在没有此能力的情况下创建使用大页后端的 VM，但 VM 将不能运行。

### 7.15 KVM_CAP_MSR_PLATFORM_INFO


:Architectures: x86
:Parameters: args[^0^] 特性是否应启用

借助此能力，客户机可以读取 MSR_PLATFORM_INFO MSR。否则，当客户机尝试访问时会
引发 #GP。当前，此能力不启用该 MSR 对客户机的写入权限。

### 7.16 KVM_CAP_PPC_NESTED_HV


:Architectures: ppc
:Parameters: none
:Returns: 0 on success, -EINVAL when the implementation doesn't support
	  nested-HV virtualization.

POWER9 及以后系统上的 HV-KVM 允许“嵌套 HV”虚拟化，它为客户的客户机（guest
VM）提供了一种能够使用 CPU 超级visor 模式（特权非虚拟机监控器状态）运行的
方式。在 VM 上启用此能力取决于 CPU 是否具有必要的功能，以及该设施是否通过
kvm-hv 模块参数启用。

### 7.17 KVM_CAP_EXCEPTION_PAYLOAD


:Architectures: x86
:Parameters: args[^0^] 特性是否应启用

启用此能力后，当 L1 拦截发生在 L2 中的 #PF 异常时，在模拟的 VM-exit 之前不会
修改 CR2。类似地，仅对 kvm-intel，当 L1 拦截发生在 L2 中的 #DB 异常时，在
模拟的 VM-exit 之前不会修改 DR6。因此，当 KVM_GET_VCPU_EVENTS 报告 L2 有一个
挂起的 #PF（或 #DB）异常时，exception.has_payload 将被置位，并且故障地址（或
新的 DR6 位\*）将报告在 exception_payload 字段中。类似地，当用户空间使用
KVM_SET_VCPU_EVENTS 向 L2 注入一个 #PF（或 #DB）时，应置位
exception.has_payload，并将故障地址——或新的 DR6 位\ [#]_——放入 exception_payload
字段。

此能力还启用了 struct kvm_vcpu_events 中的 exception.pending，这允许用户空间
区分挂起的异常和注入的异常。


       will clear DR6.RTM.

### 7.18 KVM_CAP_MANUAL_DIRTY_LOG_PROTECT2


:Architectures: x86, arm64, mips
:Parameters: args[^0^] 特性是否应启用

```

  #define KVM_DIRTY_LOG_MANUAL_PROTECT_ENABLE   (1 << 0)
  #define KVM_DIRTY_LOG_INITIALLY_SET           (1 << 1)

```
设置了 KVM_DIRTY_LOG_MANUAL_PROTECT_ENABLE 时，KVM_GET_DIRTY_LOG 不会自动清除
并写保护所有作为脏页返回的内存页。相反，用户空间必须使用 KVM_CLEAR_DIRTY_LOG
单独执行此操作。

以略微更复杂的操作为代价，这在两方面提供了更好的可扩展性和响应性。首先，
KVM_CLEAR_DIRTY_LOG ioctl 可以以 64 页的粒度操作，而不需要同步整个 memslot；
这确保了 KVM 不会长时间持有关自旋锁。其次，在某些情况下，在调用
KVM_GET_DIRTY_LOG 和用户空间实际使用页中数据之间会经过大量时间。在此期间页
可能被修改，这对客户机和用户空间都是低效的：客户机将因写保护故障而承受更高的
惩罚，而用户空间可能看到脏页的误报。手动重新保护有助于减少这段时间，改善客户机
性能并减少脏日志的假阳性数量。

设置了 KVM_DIRTY_LOG_INITIALLY_SET 时，脏位图的所有位在创建时都初始化为 1。
这也改善了性能，因为脏日志可以在首次调用 KVM_CLEAR_DIRTY_LOG 时以小块逐步
启用。KVM_DIRTY_LOG_INITIALLY_SET 依赖
KVM_DIRTY_LOG_MANUAL_PROTECT_ENABLE（目前它也只在 x86、arm64 和 riscv 上可用）。

KVM_CAP_MANUAL_DIRTY_LOG_PROTECT2 此前曾以 KVM_CAP_MANUAL_DIRTY_LOG_PROTECT
之名提供，但其实现存在缺陷，导致难以或无法正确使用。提供
KVM_CAP_MANUAL_DIRTY_LOG_PROTECT2 即表示这些缺陷已被修复。用户空间不应尝试
使用 KVM_CAP_MANUAL_DIRTY_LOG_PROTECT。

### 7.19 KVM_CAP_PPC_SECURE_GUEST


:Architectures: ppc

此能力表示 KVM 正运行在拥有 ultravisor 固件、因而能够支持安全客户机的主机上。
在这样的系统上，客户机可以请求 ultravisor 使其成为安全客户机，其内存在客户机
之外对宿主不可访问，除非是显式请求与客户机共享的页。当客户机请求成为安全客户机
时，ultravisor 会通知 KVM，KVM 有机会否决这一转换。

如果存在，此能力可以为 VM 启用，意味着 KVM 将允许转换到安全客户机模式。否则
KVM 将否决该转换。

### 7.20 KVM_CAP_HALT_POLL


:Architectures: all
:Target: VM
:Parameters: args[^0^] 是以纳秒为单位的最大轮询时间
:Returns: 0 on success; -1 on error

KVM_CAP_HALT_POLL 覆盖 kvm.halt_poll_ns 模块参数，以设置目标 VM 中所有 vCPU 的
最大暂停轮询（halt-polling）时间。此能力可以在任何时间、任意次数调用，以动态
更改最大暂停轮询时间。

有关暂停轮询的更多信息，请参阅 Documentation/virt/kvm/halt-polling.rst。

### 7.21 KVM_CAP_X86_USER_SPACE_MSR


:Architectures: x86
:Target: VM
:Parameters: args[^0^] 包含要报告的 KVM_MSR_EXIT_REASON_* 事件掩码
:Returns: 0 on success; -1 on error

此能力允许用户空间在 MSR 访问被拒绝时拦截 RDMSR 和 WRMSR 指令。默认情况下，
KVM 在被拒绝的访问上注入 #GP。

当客户机请求读取或写入某个 MSR 时，KVM 可能无法实现与相应系统相关的所有 MSR。
它也不会按 CPU 类型区分。

为了对 MSR 处理进行更细粒度的控制，用户空间可以启用此能力。启用后，匹配
args[^0^] 中指定掩码、并且会在客户机内触发 #GP 的 MSR 访问将改为触发
KVM_EXIT_X86_RDMSR 和 KVM_EXIT_X86_WRMSR 退出通知。然后用户空间可以实现特定
型号的 MSR 处理，和/或向用户发出通知，告知某个 MSR 未被 KVM 模拟/虚拟化。

有效的掩码标志为：

============================ ===============================================
 KVM_MSR_EXIT_REASON_UNKNOWN 拦截对（KVM 未知的）MSR 的访问
 KVM_MSR_EXIT_REASON_INVAL   拦截根据 vCPU 型号和/或模式在架构上非法的访问
 KVM_MSR_EXIT_REASON_FILTER  拦截被用户空间通过 KVM_X86_SET_MSR_FILTER 拒绝的访问
============================ ===============================================

### 7.22 KVM_CAP_X86_BUS_LOCK_EXIT


:Architectures: x86
:Target: VM
:Parameters: args[^0^] 定义客户机中检测到总线锁时使用的策略
:Returns: 0 on success, -EINVAL when args[^0^] contains invalid bits

```

  #define KVM_BUS_LOCK_DETECTION_OFF      (1 << 0)
  #define KVM_BUS_LOCK_DETECTION_EXIT     (1 << 1)

```
在 VM 上启用此能力为用户空间提供了一种选择策略来处理客户机中检测到的总线锁
的方法。用户空间可以从 KVM_CHECK_EXTENSION 的结果中获取受支持的模式，并通过
KVM_ENABLE_CAP 进行定义。受支持的模式是互斥的。

此能力允许用户空间强制在客户机中检测到的总线锁上发生 VM 退出，无论宿主是否
启用了 split-lock 检测（后者会触发 KVM 拦截的 #AC 异常）。此能力旨在缓解恶意/
有 bug 的客户机利用总线锁降低整个系统性能的攻击。

如果设置了 KVM_BUS_LOCK_DETECTION_OFF，KVM 不会强制客户机总线锁发生 VM 退出，
尽管宿主内核的 split-lock #AC 检测（如果启用）仍然适用。

如果设置了 KVM_BUS_LOCK_DETECTION_EXIT，KVM 会启用一个 CPU 特性，确保客户机中
的总线锁触发 VM 退出，并且 KVM 为所有此类 VM 退出退出到用户空间，例如允许用户
空间对违规的客户机进行限流和/或应用其他基于策略的缓解措施。退出到用户空间时，
KVM 在 vcpu-run->flags 中设置 KVM_RUN_X86_BUS_LOCK，并有条件地将 exit_reason 设为
KVM_EXIT_X86_BUS_LOCK。

由于底层硬件实现的差异，退出时 vCPU 的 RIP 在 Intel 和 AMD 之间有所不同。在
Intel 宿主上，RIP 指向下一条指令，即退出是陷阱式的（trap-like）。在 AMD 宿主上，
RIP 指向违规指令，即退出是故障式的（fault-like）。

注意！检测到的总线锁可能与其他退出到用户空间同时发生，即如果用户空间希望对
所有检测到的总线锁采取行动，则应检查 KVM_RUN_X86_BUS_LOCK，而不论主退出原因
为何。

### 7.23 KVM_CAP_PPC_DAWR1


:Architectures: ppc
:Parameters: none
:Returns: 0 on success, -EINVAL when CPU doesn't support 2nd DAWR

此能力可用于检查/启用由 POWER10 处理器提供的第 2 个 DAWR 特性。


### 7.24 KVM_CAP_VM_COPY_ENC_CONTEXT_FROM


:Architectures: x86 SEV enabled
:Type: vm
:Parameters: args[^0^] 是源 vm 的 fd
:Returns: 0 on success; ENOTTY on error

此能力允许用户空间将加密上下文从由该 fd 指示的 vm 复制到调用此能力的 vm 上。

这旨在支持由宿主调度的客户机内工作负载。这使得客户机内工作负载能够维护其自身的
NPT，并使两个 vm 不会因为中断等而意外互相破坏（独立的 APIC/MSR 等）。

### 7.25 KVM_CAP_SGX_ATTRIBUTE


:Architectures: x86
:Target: VM
:Parameters: args[^0^] 是 securityfs 中 SGX 属性文件的文件句柄
:Returns: 0 on success, -EINVAL if the file handle is invalid or if a requested
          attribute is not supported by KVM.

KVM_CAP_SGX_ATTRIBUTE 使用户空间 VMM 能够授予 VM 对一个或多个特权飞地（enclave）
属性的访问权限。args[^0^] 必须持有与 KVM 支持/限制的属性（当前只有 PROVISIONKEY）
相对应的有效 SGX 属性文件的文件句柄。

SGX 子系统限制对一部分飞地属性的访问，以便为未被攻破的内核提供额外安全性，例如
PROVISIONKEY 的使用受到限制，以阻止恶意软件利用 PROVISIONKEY 获得稳定的系统指纹。
为了防止用户空间通过在 VM 中运行飞地来规避此类限制，KVM 默认阻止对特权属性的
访问。

更多细节请参阅 Documentation/arch/x86/sgx.rst。

### 7.27 KVM_CAP_EXIT_ON_EMULATION_FAILURE


:Architectures: x86
:Parameters: args[^0^] 特性是否应启用

当启用此能力时，模拟失败将导致以 KVM_INTERNAL_ERROR 退出到用户空间（调用模拟器
处理 VMware 后门指令的情况除外）。此外，KVM 现在将为任何因模拟失败导致的退出到
用户空间提供最多 15 条指令字节。当发生这些退出到用户空间时，使用 emulation_failure
结构而非 internal 结构。它们具有相同的布局，但 emulation_failure 结构更贴合内容。
它还显式定义了“flags”字段，用于描述结构中有效的字段（即：如果在“flags”字段中
设置了 KVM_INTERNAL_ERROR_EMULATION_FLAG_INSTRUCTION_BYTES，则“insn_size”和
“insn_bytes”都包含有效数据）。

### 7.28 KVM_CAP_ARM_MTE


:Architectures: arm64
:Parameters: none

此能力表示 KVM（以及硬件）支持向客户机暴露内存标记扩展（MTE）。在创建任何 VCPU
之前，它也必须由 VMM 启用，以允许客户机访问。注意 MTE 仅对客户机在 AArch64 模式下
运行时可用，启用此能力将导致尝试创建 AArch32 VCPU 失败。

启用后，客户机能够访问与提供给客户机的任何内存相关联的标记。KVM 将确保在宿主的
交换或休眠期间维护这些标记；但是，如果 VM 被迁移，VMM 需要适当地手动保存/恢复
这些标记。

启用此能力时，memslot 中的所有内存必须映射为 `MAP_ANONYMOUS` 或使用基于 RAM 的
文件映射（`tmpfs`、`memfd`），尝试用无效的 mmap 创建 memslot 将导致返回 -EINVAL。

启用时，VMM 可以利用 `KVM_ARM_MTE_COPY_TAGS` ioctl 在客户机之间批量复制标记。

### 7.29 KVM_CAP_VM_MOVE_ENC_CONTEXT_FROM


:Architectures: x86 SEV enabled
:Type: vm
:Parameters: args[^0^] 是源 vm 的 fd
:Returns: 0 on success

此能力允许用户空间将加密上下文从由该 fd 指示的 VM 迁移到调用此能力的 VM 上。

这旨在支持用户空间 VMM 之间的 VM 宿内迁移，在不中断客户机的情况下升级 VMM 进程。

### 7.31 KVM_CAP_DISABLE_QUIRKS2


:Parameters: args[^0^] - 要禁用的 KVM 怪癖（quirk）集合
:Architectures: x86
:Type: vm

此能力如果启用，将导致 KVM 禁用一些行为怪癖（quirk）。

为此能力调用 KVM_CHECK_EXTENSION 将返回可在 KVM 中禁用的怪癖的位掩码。

为此能力调用 KVM_ENABLE_CAP 的参数是一个要禁用的怪癖的位掩码，且必须是
KVM_CHECK_EXTENSION 返回的位掩码的子集。

cap.args[^0^] 中的有效位为：

========================================   ================================================
KVM_X86_QUIRK_LINT0_REENABLED              默认情况下，LVT LINT0 寄存器的复位值是 0x700
                                           （APIC_MODE_EXTINT）。禁用此怪癖时，复位值为
                                           0x10000（APIC_LVT_MASKED）。

KVM_X86_QUIRK_CD_NW_CLEARED                默认情况下，KVM 清除 AMD CPU 上的 CR0.CD 和
                                           CR0.NW，以规避以 CR0.CD（即缓存处于“no fill”
                                           模式）永久运行的客户机固件 bug。

                                           禁用此怪癖时，KVM 不会改变 CR0.CD 和 CR0.NW
                                           的值。

KVM_X86_QUIRK_LAPIC_MMIO_HOLE              默认情况下，即使配置为 x2APIC 模式，MMIO
                                           LAPIC 接口也可用。禁用此怪癖时，如果 LAPIC 处于
                                           x2APIC 模式，KVM 会禁用 MMIO LAPIC 接口。

KVM_X86_QUIRK_OUT_7E_INC_RIP               默认情况下，KVM 在退出到用户空间处理向 0x7e
                                           端口的 OUT 指令之前预递增 %rip。禁用此怪癖时，
                                           KVM 在退出到用户空间之前不会预递增 %rip。

KVM_X86_QUIRK_MISC_ENABLE_NO_MWAIT         禁用此怪癖时，如果 IA32_MISC_ENABLE[bit 18]
                                           （MWAIT）被置位，KVM 设置
                                           CPUID.01H:ECX[bit 3]（MONITOR/MWAIT）。此外，
                                           禁用此怪癖时，如果 IA32_MISC_ENABLE[bit 18]被
                                           清零，KVM 清除 CPUID.01H:ECX[bit 3]。

KVM_X86_QUIRK_FIX_HYPERCALL_INSN           默认情况下，KVM 重写客户机 VMMCALL/VMCALL
                                           指令，以匹配系统供应商的超级调用指令。禁用此
                                           怪癖时，KVM 不再重写无效的客户机超级调用指令。
                                           执行错误的超级调用指令将在客户机内生成 #UD。

KVM_X86_QUIRK_MWAIT_NEVER_UD_FAULTS        默认情况下，KVM 将 MONITOR/MWAIT（如果被
                                           拦截）模拟为 NOP，不论根据客户机 CPUID 它们是否
                                           受支持。禁用此怪癖且未设置
                                           KVM_X86_DISABLE_EXITS_MWAIT（MONITOR/MWAIT 被
                                           拦截）时，如果根据客户机 CPUID 它们不受支持，
                                           KVM 将在 MONITOR/MWAIT 上注入 #UD。注意，如果
                                           KVM_X86_QUIRK_MISC_ENABLE_NO_MWAIT 被禁用，KVM
                                           将在写入 MISC_ENABLE 时修改客户机 CPUID 中的
                                           MONITOR/MWAIT 支持。

KVM_X86_QUIRK_SLOT_ZAP_ALL                 默认情况下，对于 KVM_X86_DEFAULT_VM 类型的
                                           VM，KVM 在删除或移动 memslot 时使所有 memslot 和
                                           地址空间中的所有 SPTE 失效。禁用此怪癖（或 VM 类型
                                           不是 KVM_X86_DEFAULT_VM）时，KVM 只确保被删除或
                                           移动的 memslot 的后备内存不可达，即 KVM _可能_ 只
                                           使与该 memslot 相关的 SPTE 失效。

KVM_X86_QUIRK_STUFF_FEATURE_MSRS           默认情况下，在创建 vCPU 时，KVM 将 vCPU 的
                                           MSR_IA32_PERF_CAPABILITIES（0x345）、
                                           MSR_IA32_ARCH_CAPABILITIES（0x10a）、
                                           MSR_PLATFORM_INFO（0xce）以及所有 VMX MSR
                                           （0x480..0x492）设为 KVM 支持的最大能力。KVM 还将
                                           MSR_IA32_UCODE_REV（0x8b）设为任意值（Intel 与 AMD
                                           不同）。最后，当设置客户机 CPUID 时（由用户空间），
                                           KVM 修改选定的 VMX MSR 字段，以强制客户机 CPUID 与
                                           L2 的有效 ISA 之间的一致性。禁用此怪癖时，KVM 将
                                           vCPU 的 MSR 值清零（有两个例外，见下文），即将特性
                                           MSR 视为 CPUID 叶，给予用户空间对 vCPU 型号定义的
                                           完全控制。此怪癖不影响 VMX MSR CR0/CR4_FIXED1
                                           （0x487 和 0x489），因为 KVM 现在不允许它们由用户空间
                                           设置（KVM 根据客户机 CPUID 设置它们，出于安全目的）。

KVM_X86_QUIRK_IGNORE_GUEST_PAT             默认情况下，在 Intel 平台上，KVM 忽略客户机
                                           PAT，并在 EPT 中强制有效内存类型为 WB。该怪癖在
                                           无法安全尊重客户机 PAT 的 Intel 平台（即没有 CPU
                                           自嗅探，KVM 总是忽略客户机 PAT 并强制有效内存类型
                                           为 WB）上不可用。在 AMD 平台或（在 Intel 上）当 VM
                                           分配了非一致 DMA 设备时，它也被忽略；KVM 在此类
                                           情况下总是尊重客户机 PAT。需要此怪癖以避免某些 Intel
                                           Xeon 平台（例如 ICX、SPR）上的性能下降，这些平台
                                           支持自嗅探特性，但 UC 足够慢，会导致一些使用 UC 而非
                                           WC 映射显存的较老客户机出现问题。如果用户空间知道没有
                                           此类客户机软件，例如它没有暴露 bochs 图形设备（已知
                                           其驱动有 bug），则可以禁用此怪癖以尊重客户机 PAT。

KVM_X86_QUIRK_VMCS12_ALLOW_FREEZE_IN_SMM   默认情况下，KVM 放宽对 vmcs12 中
                                           GUEST_IA32_DEBUGCTL 的一致性检查，以允许设置
                                           FREEZE_IN_SMM。禁用此怪癖时，KVM 要求该位被清零。
                                           注意，无论怪癖设置如何，vmcs02 的该位仍完全由宿主
                                           控制。
========================================   ================================================

### 7.32 KVM_CAP_MAX_VCPU_ID


:Architectures: x86
:Target: VM
:Parameters: args[^0^] - 为当前 VM 设置的最大 APIC ID 值
:Returns: 0 on success, -EINVAL if args[^0^] is beyond KVM_MAX_VCPU_IDS
          supported in KVM or if it has been set.

此能力允许用户在创建 vCPU 之前，为当前 VM 会话指定分配的最大可能 APIC ID，从而为
按 APIC ID 索引的数据结构节省内存。用户空间能够根据指定的 CPU 拓扑计算出 APIC ID
值的限制。

该值只能在 KVM_ENABLE_CAP 被设为非零值之前，或直到创建 vCPU 之前更改。在创建
第一个 vCPU 时，如果值被设为 0 或未调用 KVM_ENABLE_CAP，KVM 将使用
KVM_CHECK_EXTENSION(KVM_CAP_MAX_VCPU_ID) 的返回值作为最大 APIC ID。

### 7.33 KVM_CAP_X86_NOTIFY_VMEXIT


:Architectures: x86
:Target: VM
:Parameters: args[^0^] 是通知窗口的值以及一些标志
:Returns: 0 on success, -EINVAL if args[^0^] contains invalid flags or notify
          VM exit is unsupported.

args[^0^] 的 63:32 位用于通知窗口。
```

  #define KVM_X86_NOTIFY_VMEXIT_ENABLED    (1 << 0)
  #define KVM_X86_NOTIFY_VMEXIT_USER       (1 << 1)

```
此能力允许用户在 VM 创建期间在每 VM 范围内配置通知 VM 退出的开/关。默认情况下
禁用通知 VM 退出。当用户空间在 args[^0^] 中设置 KVM_X86_NOTIFY_VMEXIT_ENABLED
位时，VMM 将使用提供的通知窗口启用此特性，如果在 VM 非根模式下经过指定时间（通知
窗口）仍无事件窗口发生，将生成 VM 退出。

如果在 args[^0^] 中设置了 KVM_X86_NOTIFY_VMEXIT_USER，则在发生通知 VM 退出时，
KVM 将退出到用户空间进行处理。

此能力旨在缓解恶意 VM 导致 CPU 卡住（由于事件窗口未打开）并使 CPU 对宿主或其他
VM 不可用的威胁。

### 7.35 KVM_CAP_X86_APIC_BUS_CYCLES_NS


:Architectures: x86
:Target: VM
:Parameters: args[^0^] 是期望的 APIC 总线时钟频率，以纳秒为单位
:Returns: 0 on success, -EINVAL if args[^0^] contains an invalid value for the
          frequency or if any vCPUs have been created, -ENXIO if a virtual
          local APIC has not been created using KVM_CREATE_IRQCHIP.

此能力设置 VM 的 APIC 总线时钟频率，KVM 的内核态虚拟 APIC 在模拟 APIC 定时器时
使用它。KVM 的默认值可通过 KVM_CHECK_EXTENSION 获取。

注意：如果将非零的 CPUID 0x15 暴露给客户机，用户空间负责正确配置 CPUID 0x15，即
核心晶振时钟频率。

### 7.36 KVM_CAP_DIRTY_LOG_RING/KVM_CAP_DIRTY_LOG_RING_ACQ_REL


:Architectures: x86, arm64, riscv
:Type: vm
:Parameters: args[^0^] - 脏日志环的大小

KVM 能够使用 mmap 到用户空间的环形缓冲区来跟踪脏内存；每个 vcpu 有一个脏环。

脏环对用户空间可用，是一个
```

  struct kvm_dirty_gfn {
          __u32 flags;
          __u32 slot; /* as_id | slot_id */
          __u64 offset;
  };

```
为定义 flags 字段，定义了以下值
```

  #define KVM_DIRTY_GFN_F_DIRTY           BIT(0)
  #define KVM_DIRTY_GFN_F_RESET           BIT(1)
  #define KVM_DIRTY_GFN_F_MASK            0x3

```
用户空间应在 KVM_CREATE_VM ioctl 之后立即调用 KVM_ENABLE_CAP ioctl，为新客户机
启用此能力并设置环的大小。启用该能力只允许在创建任何 vCPU 之前进行，且环的大小
必须是 2 的幂。环缓冲区越大，环满且 VM 被迫退出到用户空间的可能性越小。最优大小
取决于工作负载，但建议至少为 64 KiB（4096 个条目）。

与脏页位图一样，缓冲区跟踪对设置了 KVM_MEM_LOG_DIRTY_PAGES 标志的 KVM_SET_USER_MEMORY_REGION
的所有用户内存区域的写入。一旦内存区域以该标志注册，用户空间就可以开始从环形缓冲区
收集脏页。

环形缓冲区中的一个条目可以是未使用的（标志位 `00`）、脏的（标志位 `01`）或已收集的
（标志位 `1X`）。
```

          dirtied         harvested        reset
     00 -----------> 01 -------------> 1X -------+
      ^                                          |
      |                                          |
      +------------------------------------------+

```
要收集脏页，用户空间访问 mmap 的环形缓冲区以读取脏的 GFN。如果 flags 设置了 DIRTY
位（在此阶段 RESET 位必须清零），则意味着此 GFN 是脏 GFN。用户空间应收集此 GFN 并将
标志从状态 `01b` 改为 `1Xb`（位 0 将被 KVM 忽略，但位 1 必须设置以表明此 GFN 已被
收集并等待重置），然后继续下一个 GFN。用户空间应持续此操作，直到某个 GFN 的 flags
的 DIRTY 位被清零，意味着它已收集了所有可用的脏 GFN。

注意，在弱内存序架构上，用户空间对环形缓冲区（更具体地说是“flags”字段）的访问必须
有序，在可用时使用 load-acquire/store-release 访问器，或使用任何其他能确保此有序性
的内存屏障。

用户空间没有必要一次性收集所有脏 GFN。但它必须按顺序收集脏 GFN，即用户空间程序不能
跳过某个脏 GFN 去收集它旁边的那个。

在处理环形缓冲区中的一个或多个条目之后，用户空间调用 VM ioctl KVM_RESET_DIRTY_RINGS
来通知内核，以便内核重新保护那些已收集的 GFN。因此，必须在读取脏页内容_之前_调用
此 ioctl。

脏环可能会变满。当这种情况发生时，vcpu 的 KVM_RUN 将以退出原因 KVM_EXIT_DIRTY_RING_FULL
返回。

脏环接口与 KVM_GET_DIRTY_LOG 接口相比有一个主要区别：从用户空间读取脏环时，内核仍
可能尚未将处理器的脏页缓冲区刷新到内核缓冲区（而对于脏位图，刷新是由
KVM_GET_DIRTY_LOG ioctl 完成的）。为此，需要使用信号将 vcpu 踢出 KVM_RUN。由此产生的
vmexit 确保所有脏 GFN 都被刷新到脏环中。

注意：KVM_CAP_DIRTY_LOG_RING_ACQ_REL 是弱内存序架构唯一应暴露的能力，以指示在读取
条目状态并将其从 DIRTY 变为 HARVESTED 时对用户空间施加的额外内存有序性要求。具有类 TSO
有序性（如 x86）的架构允许同时向用户空间暴露 KVM_CAP_DIRTY_LOG_RING 和
KVM_CAP_DIRTY_LOG_RING_ACQ_REL。

启用脏环后，用户空间需要检测 KVM_CAP_DIRTY_LOG_RING_WITH_BITMAP 能力，以查看环结构
是否可以由每插槽（per-slot）位图支持。通告此能力意味着该架构可以在没有 vcpu/环上下文
的情况下弄脏客户机页，因此部分脏信息仍将维护在位图结构中。如果尚未启用
KVM_CAP_DIRTY_LOG_RING_ACQ_REL 能力，或已存在任何 memslot，则不能启用
KVM_CAP_DIRTY_LOG_RING_WITH_BITMAP。

注意，这里的位图只是环结构的备份。仅当只有极少量内存在 vcpu/环上下文之外被弄脏时，
使用环和位图组合才有益。否则，需要考虑独立的每插槽位图机制。

要收集备份位图中的脏位，用户空间可以使用相同的 KVM_GET_DIRTY_LOG ioctl。只要所有脏位
的生成都在单次遍历中完成，就不需要 KVM_CLEAR_DIRTY_LOG。收集脏位图应该是 VMM 在认为
状态完整之前做的最后一件事。VMM 需要确保脏状态是最终的，并避免丢失在比特图收集之后
排序的另一个 ioctl 产生的脏页。

注意：使用备份位图的多个示例：（1）通过 KVM 设备“kvm-arm-vgic-its”上的命令
KVM_DEV_ARM_{VGIC_GRP_CTRL, ITS_SAVE_TABLES} 保存 vgic/its 表。（2）通过 KVM 设备
“kvm-arm-vgic-its”上的命令 KVM_DEV_ARM_{VGIC_GRP_CTRL, ITS_RESTORE_TABLES} 恢复
vgic/its 表。VGICv3 LPI 挂起状态被恢复。（3）通过 KVM 设备“kvm-arm-vgic-v3”上的
命令 KVM_DEV_ARM_VGIC_{GRP_CTRL, SAVE_PENDING_TABLES} 保存 vgic3 挂起表。

### 7.37 KVM_CAP_PMU_CAPABILITY


:Architectures: x86
:Type: vm
:Parameters: arg[^0^] 是 PMU 虚拟化能力的位掩码。
:Returns: 0 on success, -EINVAL when arg[^0^] contains invalid bits

此能力改变 KVM 中的 PMU 虚拟化。

为此能力调用 KVM_CHECK_EXTENSION 将返回可在 VM 上调整的 PMU 虚拟化能力的位掩码。

KVM_ENABLE_CAP 的参数也是一个位掩码，并选择要应用到 VM 的特定 PMU 虚拟化能力。这
只能在创建 VCPU 之前对 VM 调用。

目前，KVM_PMU_CAP_DISABLE 是唯一的能力。设置此能力将禁用该 VM 的 PMU 虚拟化。
用户态应调整 CPUID 叶 0xA 以反映 PMU 已禁用。

### 7.38 KVM_CAP_VM_DISABLE_NX_HUGE_PAGES


:Architectures: x86
:Type: vm
:Parameters: arg[^0^] 必须为 0。
:Returns: 0 on success, -EPERM if the userspace process does not
          have CAP_SYS_BOOT, -EINVAL if args[^0^] is not 0 or any vCPUs have been
          created.

此能力禁用针对 iTLB MULTIHIT 的 NX 大页缓解措施。

如果未设置 nx_huge_pages 模块参数，则该能力无效。

此能力只能在创建任何 vCPU 之前设置。

### 7.39 KVM_CAP_ARM_EAGER_SPLIT_CHUNK_SIZE


:Architectures: arm64
:Type: vm
:Parameters: arg[^0^] 是新的拆分块大小。
:Returns: 0 on success, -EINVAL if any memslot was already created.

此能力设置 Eager Page Splitting（积极页拆分）中使用的块大小。

当客户机内存由大页（huge-page）支持时，Eager Page Splitting 改善了脏日志（用于
实时迁移）的性能。它通过在启用脏日志（为内存区域设置 KVM_MEM_LOG_DIRTY_PAGES
标志）或使用 KVM_CLEAR_DIRTY_LOG 时积极地拆分，避免在缺页时拆分大页（为 PAGE_SIZE
页）。

块大小指定每次拆分多少页，为每个块使用单次分配。块大小越大，需要提前分配的页越多。

块大小必须是有效的块大小。可接受的块大小列表作为 64 位位图暴露在
KVM_CAP_ARM_SUPPORTED_BLOCK_SIZES 中（每个位描述一个块大小）。默认值为 0，即禁用
积极页拆分。

### 7.40 KVM_CAP_EXIT_HYPERCALL


:Architectures: x86
:Type: vm

此能力如果启用，将导致 KVM 以 KVM_EXIT_HYPERCALL 退出原因退出到用户空间以处理某些
超级调用。

为此能力调用 KVM_CHECK_EXTENSION 将返回可配置为退出到用户空间的超级调用的位掩码。
目前，唯一的此类超级调用是 KVM_HC_MAP_GPA_RANGE。

KVM_ENABLE_CAP 的参数也是一个位掩码，且必须是 KVM_CHECK_EXTENSION 结果的子集。KVM
将把对应位在参数中的超级调用转发到用户空间，并对其余的返回 ENOSYS。

### 7.41 KVM_CAP_ARM_SYSTEM_SUSPEND


:Architectures: arm64
:Type: vm

启用时，KVM 将以类型为 KVM_SYSTEM_EVENT_SUSPEND 的 KVM_EXIT_SYSTEM_EVENT 退出到
用户空间，以处理客户机挂起请求。

### 7.42 KVM_CAP_ARM_WRITABLE_IMP_ID_REGS


:Architectures: arm64
:Target: VM
:Parameters: None
:Returns: 0 on success, -EINVAL if vCPUs have been created before enabling this
          capability.

此能力改变了标识 Arm 架构 PE 实现的寄存器的行为：MIDR_EL1、REVIDR_EL1 和
AIDR_EL1。默认情况下，这些寄存器对用户空间可见，但被视为不变量。

启用此能力时，KVM 允许用户在第一次 KVM_RUN 之前更改上述寄存器。这些寄存器是 VM
作用域的，意味着同一组值会呈现给给定 VM 中的所有 vCPU。

### 7.43 KVM_CAP_RISCV_MP_STATE_RESET


:Architectures: riscv
:Type: VM
:Parameters: None
:Returns: 0 on success, -EINVAL if arg[^0^] is not zero

启用此能力时，KVM 在通过 IOCTL 设置 MP_STATE_INIT_RECEIVED 时重置 VCPU。原始的
MP_STATE 被保留。
### 7.44 KVM_CAP_ARM_CACHEABLE_PFNMAP_SUPPORTED


:Architectures: arm64
:Target: VM
:Parameters: None

此能力向用户空间指示一个 PFNMAP 内存区域是否可以安全地映射为可缓存（cacheable）。
这依赖于硬件上是否存在强制写回（force write back，FWB）特性支持。

### 7.45 KVM_CAP_ARM_SEA_TO_USER


:Architecture: arm64
:Target: VM
:Parameters: none
:Returns: 0 on success, -EINVAL if unsupported.

启用此能力时，KVM 可能会因客户机访问导致的、进入 EL2 的 SEA 而退出到用户空间。
更多信息请参阅 `KVM_EXIT_ARM_SEA`。

### 7.46 KVM_CAP_S390_USER_OPEREXEC


:Architectures: s390
:Parameters: none

启用此能力时，KVM 会将其自身不处理的操作异常全部转发到用户空间。这也包括由
KVM_CAP_S390_USER_INSTR0 管理的 0x0000 指令。如果用户空间希望模拟（尚）未在硬件
中实现的指令，这会很有帮助。

即使在 VCPU 已被创建并正在运行的情况下，也可以动态启用此能力。

## 8. 其他能力。


本节列出提供有关 KVM 实现其他特性信息的能力。

### 8.1 KVM_CAP_PPC_HWRNG


:Architectures: ppc

此能力，如果 KVM_CHECK_EXTENSION 指示其可用，意味着内核实现了由硬件随机数生成器
支撑的 H_RANDOM 超级调用。如果存在，内核的 H_RANDOM 处理程序可以通过
KVM_CAP_PPC_ENABLE_HCALL 能力为客户机使用而启用。

### 8.3 KVM_CAP_PPC_MMU_RADIX


:Architectures: ppc

此能力，如果 KVM_CHECK_EXTENSION 指示其可用，意味着内核可以支持使用 Power ISA
V3.00（如 POWER9 处理器中所实现）中定义的 radix MMU 的客户机。

### 8.4 KVM_CAP_PPC_MMU_HASH_V3


:Architectures: ppc

此能力，如果 KVM_CHECK_EXTENSION 指示其可用，意味着内核可以支持使用 Power ISA
V3.00（如 POWER9 处理器中所实现）中定义的哈希页表 MMU 的客户机，包括内存中的段表。

### 8.5 KVM_CAP_MIPS_VZ


:Architectures: mips

此能力，如果在主 kvm 句柄上执行 KVM_CHECK_EXTENSION 指示其可用，意味着可以通过
KVM 使用硬件的完全硬件辅助虚拟化能力。必须向 KVM_CREATE_VM 传递一个合适的
KVM_VM_MIPS_* 类型来创建一个利用它的 VM。

如果在 kvm VM 句柄上执行 KVM_CHECK_EXTENSION 指示此能力可用，则意味着该 VM 正在
使用硬件的完全硬件辅助虚拟化能力。这在用 KVM_VM_MIPS_DEFAULT 创建 VM 之后检查
很有用。

KVM_CHECK_EXTENSION 返回的值应与已知值（见下文）进行比较。所有其他值均保留。这是
为了允许其他可能与 MIPS VZ ASE 不兼容的硬件辅助虚拟化实现存在的可能性。

==  ==========================================================================
 0  使用 trap & emulate 实现在用户模式下运行客户机代码。客户机虚拟内存段被重排以
    使客户机适应于用户模式地址空间。

 1  使用 MIPS VZ ASE，提供完全硬件辅助虚拟化，包括标准的客户机虚拟内存段。
==  ==========================================================================

### 8.7 KVM_CAP_MIPS_64BIT


:Architectures: mips

此能力指示客户机支持的架构类型，即支持的寄存器和地址宽度。

当在 kvm VM 句柄上通过 KVM_CHECK_EXTENSION 检查此能力时，返回的值大致对应于
CP0_Config.AT 寄存器字段，并应针对已知值（见下文）专门检查。所有其他值均保留。

==  ========================================================================
 0  MIPS32 或 microMIPS32。寄存器和地址均为 32 位宽。只能运行 32 位客户机代码。

 1  MIPS64 或 microMIPS64，但只能访问 32 位兼容段。寄存器为 64 位宽，但地址为
    32 位宽。可以运行 64 位客户机代码，但无法访问 MIPS64 内存段。也可以运行 32 位
    客户机代码。

 2  MIPS64 或 microMIPS64，可访问所有地址段。寄存器和地址均为 64 位宽。可以运行
    64 位或 32 位客户机代码。
==  ========================================================================

### 8.9 KVM_CAP_ARM_USER_IRQ


:Architectures: arm64

此能力，如果 KVM_CHECK_EXTENSION 指示其可用，意味着如果用户空间创建了没有内核态
中断控制器的 VM，它将收到对内核态模拟设备输出电平变化的通知，这些设备可以生成
虚拟中断并呈现给 VM。对于此类 VM，每次返回到用户空间时，内核都会更新 vcpu 的
run->s.regs.device_irq_level 字段以表示设备的实际输出电平。

每当 kvm 检测到设备输出电平发生变化时，kvm 保证在运行 VM 之前至少返回一次用户空间。
此退出可以是 KVM_EXIT_INTR 或任何其他退出事件，如 KVM_EXIT_MMIO。这样，用户空间
总是可以采样设备输出电平并重新计算用户空间中断控制器的状态。用户空间应总是在每次
kvm 退出时检查 run->s.regs.device_irq_level 的状态。run->s.regs.device_irq_level
中的值可以表示电平触发和边沿触发的中断信号，取决于设备。边沿触发的中断信号将在每次
边沿信号时以 run->s.regs.device_irq_level 中的位恰好置位一次的方式退出到用户空间。

run->s.regs.device_irq_level 字段的可用性不依赖于 run->kvm_valid_regs 或
run->kvm_dirty_regs 位。

如果支持 KVM_CAP_ARM_USER_IRQ，KVM_CHECK_EXTENSION ioctl 返回一个大于 0 的数字，
指示所实现的此能力版本，从而指示 run->s.regs.device_irq_level 中的哪些位可以发出
信号值。

```

  KVM_CAP_ARM_USER_IRQ >= 1:

    KVM_ARM_DEV_EL1_VTIMER -  EL1 虚拟定时器
    KVM_ARM_DEV_EL1_PTIMER -  EL1 物理定时器
    KVM_ARM_DEV_PMU        -  ARM PMU 溢出中断信号

```
kvm 的未来版本可能实现额外的事件。这些将通过从 KVM_CHECK_EXTENSION 返回更高的数字
来指示，并将在上面列出。

### 8.10 KVM_CAP_PPC_SMT_POSSIBLE


:Architectures: ppc

查询此能力返回一个位图，指示可以使用 KVM_CAP_PPC_SMT 设置的虚拟 SMT 模式。如果
（从右起）第 N 位被置位，则 2^N 的虚拟 SMT 模式可用。

### 8.12 KVM_CAP_HYPERV_VP_INDEX


:Architectures: x86

此能力指示用户空间可以加载 HV_X64_MSR_VP_INDEX msr。其值用于表示 SynIC 中断的
目标 vcpu。为了兼容性，KVM 将此 msr 初始化为 KVM 的内部 vcpu 索引。当此能力不存在
时，用户空间仍可以查询此 msr 的值。

### 8.13 KVM_CAP_S390_AIS_MIGRATION


:Architectures: s390

此能力指示 flic 设备是否将能够通过 KVM_DEV_FLIC_AISM_ALL 属性获取/设置用于迁移的
AIS 状态，并允许在不必创建 flic 设备的情况下发现这一点。

### 8.14 KVM_CAP_S390_PSW


:Architectures: s390

此能力指示 PSW 通过 kvm_run 结构暴露。

### 8.15 KVM_CAP_S390_GMAP


:Architectures: s390

此能力指示用作客户机映射的用户空间内存可以位于用户内存地址空间中的任何位置，只要
内存槽按段（1MB）边界对齐并调整大小。

### 8.16 KVM_CAP_S390_COW


:Architectures: s390

此能力指示用作客户机映射的用户空间内存可以使用写时复制（copy-on-write）语义，以及
通过只读页表进行脏页跟踪。

### 8.17 KVM_CAP_S390_BPB


:Architectures: s390

此能力指示 kvm 将实现用于处理分支预测阻塞的重置、迁移和嵌套 KVM 的接口。如果没有
此能力，不应向客户机提供 stfle facility 82。

### 8.18 KVM_CAP_HYPERV_TLBFLUSH


:Architectures: x86

此能力指示 KVM 支持半虚拟化 Hyper-V TLB 刷新超级调用：
HvFlushVirtualAddressSpace、HvFlushVirtualAddressSpaceEx、
HvFlushVirtualAddressList、HvFlushVirtualAddressListEx。

### 8.19 KVM_CAP_ARM_INJECT_SERROR_ESR


:Architectures: arm64

此能力指示用户空间可以指定（通过 KVM_SET_VCPU_EVENTS ioctl）当客户机发生虚拟 SError
中断异常时报告给客户机的综合征（syndrome）值。如果 KVM 通告此能力，用户空间只能指定
ESR 综合征的 ISS 字段。ESR 的其他部分（例如 EC）在异常发生时由 CPU 生成。如果这个
虚拟 SError 使用 AArch64 进入 EL1，此值将报告在 ESR_ELx 的 ISS 字段中。

更多细节请参阅 KVM_CAP_VCPU_EVENTS。

### 8.20 KVM_CAP_HYPERV_SEND_IPI


:Architectures: x86

此能力指示 KVM 支持半虚拟化 Hyper-V IPI 发送超级调用：
HvCallSendSyntheticClusterIpi、HvCallSendSyntheticClusterIpiEx。

### 8.22 KVM_CAP_S390_VCPU_RESETS


:Architectures: s390

此能力指示 KVM_S390_NORMAL_RESET 和 KVM_S390_CLEAR_RESET ioctl 可用。

### 8.23 KVM_CAP_S390_PROTECTED


:Architectures: s390

此能力指示 Ultravisor 已初始化，因此 KVM 可以启动受保护的 VM。此能力管辖
KVM_S390_PV_COMMAND ioctl 和 KVM_MP_STATE_LOAD MP_STATE。对于受保护的客户机，当
状态变更无效时，KVM_SET_MP_STATE 可能失败。

### 8.24 KVM_CAP_STEAL_TIME


:Architectures: arm64, x86

此能力指示 KVM 支持窃取时间（steal time）记账。当支持窃取时间记账时，可以通过
架构特定的接口启用。此能力和架构特定的接口必须一致，即如果一个说支持该特性，另一个
也应该支持，反之亦然。对于 arm64，请参阅 Documentation/virt/kvm/devices/vcpu.rst 的
“KVM_ARM_VCPU_PVTIME_CTRL”。对于 x86，请参阅 Documentation/virt/kvm/x86/msr.rst 的
“MSR_KVM_STEAL_TIME”。

### 8.25 KVM_CAP_S390_DIAG318


:Architectures: s390

此能力使客户机能够设置有关其控制程序（即客户机内核类型和版本）的信息。这些信息在
系统/固件服务事件期间很有帮助，提供关于机器上运行的客户机环境的额外数据。

该信息与 DIAGNOSE 0x318 指令相关联，该指令设置一个 8 字节的值，由一个字节的控制
程序名代码（CPNC）和 7 字节的控制程序版本代码（CPVC）组成。CPNC 确定控制程序运行
于何种环境（例如 Linux、z/VM……），CPVC 用于 OS 特定的信息（例如 Linux 版本、
Linux 发行版……）。

如果此能力可用，则 CPNC 和 CPVC 可以通过同步寄存器机制（KVM_SYNC_DIAG318）在 KVM
和用户空间之间同步。

### 8.26 KVM_CAP_X86_USER_SPACE_MSR


:Architectures: x86

此能力指示 KVM 支持将 MSR 读取和写入转向用户空间。它可以在 VM 级别启用。如果启用，
通常会由 KVM 向客户机触发 #GP 的 MSR 访问，将改为通过 KVM_EXIT_X86_RDMSR 和
KVM_EXIT_X86_WRMSR 退出通知弹回用户空间。

### 8.27 KVM_CAP_X86_MSR_FILTER


:Architectures: x86

此能力指示 KVM 支持拒绝访问用户定义的 MSR。暴露此能力后，KVM 导出新的 VM ioctl
KVM_X86_SET_MSR_FILTER，用户空间可以调用它来指定 KVM 应拒绝访问的 MSR 范围的位图。

结合 KVM_CAP_X86_USER_SPACE_MSR，这允许用户空间捕获并模拟超出 KVM 范围的 MSR，以及
限制 KVM 的 MSR 模拟代码的攻击面。

### 8.30 KVM_CAP_XEN_HVM


:Architectures: x86

此能力指示 Xen 支持的用于托管 Xen 的特性
```

  #define KVM_XEN_HVM_CONFIG_HYPERCALL_MSR		(1 << 0)
  #define KVM_XEN_HVM_CONFIG_INTERCEPT_HCALL		(1 << 1)
  #define KVM_XEN_HVM_CONFIG_SHARED_INFO		(1 << 2)
  #define KVM_XEN_HVM_CONFIG_RUNSTATE			(1 << 3)
  #define KVM_XEN_HVM_CONFIG_EVTCHN_2LEVEL		(1 << 4)
  #define KVM_XEN_HVM_CONFIG_EVTCHN_SEND		(1 << 5)
  #define KVM_XEN_HVM_CONFIG_RUNSTATE_UPDATE_FLAG	(1 << 6)
  #define KVM_XEN_HVM_CONFIG_PVCLOCK_TSC_UNSTABLE	(1 << 7)

```
KVM_XEN_HVM_CONFIG_HYPERCALL_MSR 标志指示 KVM_XEN_HVM_CONFIG ioctl 可用，供客户机
设置其超级调用页。

如果也设置了 KVM_XEN_HVM_CONFIG_INTERCEPT_HCALL，则可以在提供给 KVM_XEN_HVM_CONFIG
的 flags 中提供相同的标志（不提供超级调用页内容），以请求 KVM 自动生成超级调用页
内容，并启用对客户机超级调用的拦截（KVM_EXIT_XEN）。

KVM_XEN_HVM_CONFIG_SHARED_INFO 标志指示 KVM_XEN_HVM_SET_ATTR、KVM_XEN_HVM_GET_ATTR、
KVM_XEN_VCPU_SET_ATTR 和 KVM_XEN_VCPU_GET_ATTR ioctl 的可用性，以及在 vcpu 的
vcpu_info 的 evtchn_upcall_pending 字段被置位时递送事件通道 upcall 的异常向量。

KVM_XEN_HVM_CONFIG_RUNSTATE 标志指示 runstate 相关特性
KVM_XEN_VCPU_ATTR_TYPE_RUNSTATE_ADDR/_CURRENT/_DATA/_ADJUST 受
KVM_XEN_VCPU_SET_ATTR/KVM_XEN_VCPU_GET_ATTR ioctl 支持。

KVM_XEN_HVM_CONFIG_EVTCHN_2LEVEL 标志指示支持类型为 KVM_IRQ_ROUTING_XEN_EVTCHN 的
IRQ 路由条目，其 priority 字段被设为表示 2 级事件通道递送。

KVM_XEN_HVM_CONFIG_EVTCHN_SEND 标志指示 KVM 支持使用 KVM_XEN_HVM_EVTCHN_SEND ioctl
将事件通道事件直接注入客户机。它还指示支持 KVM_XEN_ATTR_TYPE_EVTCHN/XEN_VERSION HVM
属性，以及 KVM_XEN_VCPU_ATTR_TYPE_VCPU_ID/TIMER/UPCALL_VECTOR vCPU 属性，这些与事件
通道递送、定时器以及 XENVER_version 拦截相关。

KVM_XEN_HVM_CONFIG_RUNSTATE_UPDATE_FLAG 标志指示 KVM 在 KVM_XEN_SET_ATTR 和
KVM_XEN_GET_ATTR ioctl 中支持 KVM_XEN_ATTR_TYPE_RUNSTATE_UPDATE_FLAG 属性。这控制 KVM
是否会在更新 runstate 信息时设置客户机内存映射的 vcpu_runstate_info 中的
XEN_RUNSTATE_UPDATE 标志。注意，支持上述 RUNSTATE 特性但不支持 RUNSTATE_UPDATE_FLAG
特性的 KVM 版本，在更新客户机结构时总是会设置 XEN_RUNSTATE_UPDATE 标志，这也许有悖
直觉。当通告此标志时，KVM 的行为将更正确，在（由客户机发起超级调用、导致 VMM 启用
KVM_XEN_ATTR_TYPE_RUNSTATE_UPDATE_FLAG 属性）之前不会使用 XEN_RUNSTATE_UPDATE 标志。

KVM_XEN_HVM_CONFIG_PVCLOCK_TSC_UNSTABLE 标志指示 KVM 支持在 Xen pvclock 源中清除
PVCLOCK_TSC_STABLE_BIT 标志。这将在 KVM_CAP_XEN_HVM ioctl 设置
KVM_XEN_HVM_CONFIG_PVCLOCK_TSC_UNSTABLE 标志时完成。

### 8.31 KVM_CAP_SPAPR_MULTITCE


:Architectures: ppc
:Type: vm

此能力意味着内核能够处理超调用 H_PUT_TCE_INDIRECT 和 H_STUFF_TCE，而无需将这些传递
到用户空间。这显著加速了 PPC KVM 客户机的 DMA 操作。如果用户空间之前已在 KVM 中
注册了 LIOBN（通过 KVM_CREATE_SPAPR_TCE 或类似调用），用户空间应预期这些超级调用的
处理程序不会被调用。

为了在客户机中启用 H_PUT_TCE_INDIRECT 和 H_STUFF_TCE 的使用，用户空间可能必须为客户机
通告它。例如，如果“ibm,hypertas-functions”设备树属性中存在“hcall-multi-tce”，IBM
pSeries（sPAPR）客户机就会开始使用它们。

上述超级调用可能在也可能不在基于内核的快速路径中成功处理。如果内核无法处理它们，它们
将被传递给用户空间。因此，尽管有内核态加速，用户空间仍然必须为这些超级调用保留实现。

此能力始终启用。

### 8.32 KVM_CAP_PTP_KVM


:Architectures: arm64

此能力指示宿主支持 KVM 虚拟 PTP 服务。VMM 可以在迁移时检查该服务对客户机是否可用。

### 8.37 KVM_CAP_S390_PROTECTED_DUMP


:Architectures: s390
:Type: vm

此能力指示 KVM 和 Ultravisor 支持转储 PV 客户机。`KVM_PV_DUMP` 命令可用于
`KVM_S390_PV_COMMAND` ioctl，`KVM_PV_INFO` 命令提供与转储相关的 UV 数据。此外，vcpu
ioctl `KVM_S390_PV_CPU_COMMAND` 也可用，并支持 `KVM_PV_DUMP_CPU` 子命令。

### 8.39 KVM_CAP_S390_CPU_TOPOLOGY


:Architectures: s390
:Type: vm

此能力指示 KVM 将提供 S390 CPU 拓扑设施，它包括对功能码 2 的 PTF 指令的解释，以及对
功能码 0 或 1 的 PTF 指令与 STSI(15,1,x) 指令的拦截和转发到用户态虚拟机监控器。

如果没有此能力，不应向客户机指示 stfle facility 11（CPU 拓扑设施）。

存在此能力时，KVM 在 vm fd 上提供一个新的属性组 KVM_S390_VM_CPU_TOPOLOGY。这个新的
属性允许通过 kvm_device_attr 结构获取、设置或清除 SCA 的 Modified Change Topology
Report（MTCR）位。

当获取 Modified Change Topology Report 值时，attr->addr 必须指向一个字节，值将存储到
其中或从中取出。

### 8.41 KVM_CAP_VM_TYPES


:Architectures: x86
:Type: system ioctl

此能力返回受支持 VM 类型的位图。位 @n 置 1 表示
```

  #define KVM_X86_DEFAULT_VM	0
  #define KVM_X86_SW_PROTECTED_VM	1
  #define KVM_X86_SEV_VM	2
  #define KVM_X86_SEV_ES_VM	3

```
注意，KVM_X86_SW_PROTECTED_VM 目前仅用于开发和测试。不要将 KVM_X86_SW_PROTECTED_VM
用于“真正的”VM，尤其是不要用于生产环境。软件保护的 VM 的行为和有效 ABI 是不稳定的。

### 8.42 KVM_CAP_PPC_RPT_INVALIDATE


:Architectures: ppc

此能力指示内核能够处理 H_RPT_INVALIDATE 超级调用。

为了在客户机中启用 H_RPT_INVALIDATE 的使用，用户空间可能必须为客户机通告它。例如，
如果“ibm,hypertas-functions”设备树属性中存在“hcall-rpt-invalidate”，IBM pSeries
（sPAPR）客户机就会开始使用它。

此能力在支持 radix MMU 的 POWER9 等平台上的虚拟机监控器中启用。

### 8.43 KVM_CAP_PPC_AIL_MODE_3


:Architectures: ppc

此能力指示内核支持通过 H_SET_MODE 超级调用控制的“中断时的地址转换模式”（Address
Translation Mode on Interrupt），又称“备用中断位置”（Alternate Interrupt Location）
资源的模式 3 设置。

此能力允许客户机内核使用更高性能的模式来处理中断和系统调用。

### 8.44 KVM_CAP_MEMORY_FAULT_INFO


:Architectures: x86

存在此能力指示，如果 KVM 无法解析客户机页故障 VM-Exit（例如存在有效的 memslot 但
相应的宿主虚拟地址没有后备 VMA），KVM_RUN 将填充 kvm_run.memory_fault。

kvm_run.memory_fault 中的信息当且仅当 KVM_RUN 以 errno=EFAULT 或 errno=EHWPOISON
错误返回 **并且** kvm_run.exit_reason 被设为 KVM_EXIT_MEMORY_FAULT 时才有效。

注意：尝试解决内存故障以重试 KVM_RUN 的用户空间应注意防止重复收到相同的错误/带注解
故障。

更多信息请参阅 KVM_EXIT_MEMORY_FAULT。

### 8.45 KVM_CAP_X86_GUEST_MODE


:Architectures: x86

存在此能力指示 KVM_RUN 将更新 kvm_run.flags 中的 KVM_RUN_X86_GUEST_MODE 位，以指示
vCPU 退出时是否正在执行嵌套客户机代码。

### 8.46 KVM_CAP_S390_KEYOP


:Architectures: s390

存在此能力指示 KVM_S390_KEYOP ioctl 可用。

KVM 退出时带有 L1 或 L2 客户机的寄存器状态，取决于退出时执行的是哪一个。用户空间必须
注意区分这些情况。

### 8.47 KVM_CAP_S390_VSIE_ESAMODE


:Architectures: s390

存在此能力指示嵌套 KVM 客户机可以以 ESA 模式启动。

## 9. 已知的 KVM API 问题


在某些情况下，KVM 的 API 存在一些不一致或用户空间需要注意的常见陷阱。本节详述其中
一些问题。

其中大部分是架构特定的，因此本节按架构划分。

### 9.1. x86


##### ``KVM_GET_SUPPORTED_CPUID`` 问题


通常，`KVM_GET_SUPPORTED_CPUID` 的设计使得可以将其结果直接传给 `KVM_SET_CPUID2`。
本节记录了一些需要特别小心的情况。

#### 本地 APIC 特性


CPU[EAX=1]:ECX[^21^]（X2APIC）由 `KVM_GET_SUPPORTED_CPUID` 报告，但只有在使用
`KVM_CREATE_IRQCHIP` 或 `KVM_ENABLE_CAP(KVM_CAP_IRQCHIP_SPLIT)` 来启用本地 APIC 的
内核态模拟时，才能启用它。

对于 `KVM_FEATURE_PV_UNHALT` 半虚拟化特性也是如此。

在较旧版本的 Linux 上，`KVM_GET_SUPPORTED_CPUID` 不报告 CPU[EAX=1]:ECX[^24^]
（TSC_DEADLINE），但如果存在 `KVM_CAP_TSC_DEADLINE_TIMER` 且内核已启用本地 APIC 的
内核态模拟，则可以启用它。在较新版本上，`KVM_GET_SUPPORTED_CPUID` 确实将该位报告为
可用。

#### CPU 拓扑


几个 CPUID 值包含宿主 CPU 的拓扑信息：Intel 系统的 0x0b 和 0x1f，AMD 系统的
0x8000001e。不同版本的 KVM 为此信息返回不同的值，用户空间不应依赖它。当前它们返回
全零。

如果用户空间希望设置客户机拓扑，应注意这三个叶（leaf）的值对于每个 CPU 都不同。特别
是，APIC ID 位于 0x0b 和 0x1f 所有子叶的 EDX 中，以及 0x8000001e 的 EAX 中；后者还将
核心 id 和节点 id 分别编码在 EBX 和 ECX 的 7:0 位中。

##### 已废弃的 ioctl 与能力


KVM_CAP_DISABLE_QUIRKS 不会让用户空间知道哪些怪癖实际可用。如果可用，请改用
`KVM_CHECK_EXTENSION(KVM_CAP_DISABLE_QUIRKS2)`。

##### KVM_GET_*/KVM_SET_* ioctl 的顺序


TBD
