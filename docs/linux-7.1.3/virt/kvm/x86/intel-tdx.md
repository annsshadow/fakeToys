
## Intel 信任域扩展（TDX）


## 概述


Intel 的信任域扩展（TDX）保护机密客户机 VM 免受主机和物理攻击。一个名为
“TDX module”的、经由 CPU 证明的软件模块运行在一个新的 CPU 隔离范围内，
提供管理和运行受保护 VM（即 TDX 客户机或 TD）的功能。

白皮书、规范及其他资源请参考 [^1^]。

本文档描述 TDX 特有的 KVM ABI。TDX module 需要先进行初始化，之后才能被 KVM
用于运行任何 TDX 客户机。宿主核心内核提供对 TDX module 初始化的支持，相关
说明见 Documentation/arch/x86/tdx.rst。

## API 描述


### KVM_MEMORY_ENCRYPT_OP

:Type: vm ioctl, vcpu ioctl

对于 TDX 操作，KVM_MEMORY_ENCRYPT_OP 被重新用作一个通用的 ioctl，携带
TDX 特定的子 ioctl() 命令。

```

  /* Trust Domain Extensions 子 ioctl() 命令。 */
  enum kvm_tdx_cmd_id {
          KVM_TDX_CAPABILITIES = 0,
          KVM_TDX_INIT_VM,
          KVM_TDX_INIT_VCPU,
          KVM_TDX_INIT_MEM_REGION,
          KVM_TDX_FINALIZE_VM,
          KVM_TDX_GET_CPUID,

          KVM_TDX_CMD_NR_MAX,
  };

  struct kvm_tdx_cmd {
        /* enum kvm_tdx_cmd_id */
        __u32 id;
        /* 子命令的标志位。若子命令不使用，置零。 */
        __u32 flags;
        /*
         * 每个子命令的数据。进程虚拟地址中实际数据的立即数或指针。
         * 若子命令不使用，置零。
         */
        __u64 data;
        /*
         * 辅助错误码。除了 -Exxx 之外，子命令还可能返回 TDX SEAMCALL
         * 的状态码。
         */
        __u64 hw_error;
  };

```
### KVM_TDX_CAPABILITIES

:Type: vm ioctl
:Returns: 成功返回 0，错误返回 <0

返回当前 KVM 在系统中加载特定 TDX module 后所支持的 TDX 能力。它报告哪些
特性/能力被允许配置给 TDX 客户机。

- id: KVM_TDX_CAPABILITIES
- flags: 必须为 0
- data: 指向 struct kvm_tdx_capabilities 的指针
- hw_error: 必须为 0

```

  struct kvm_tdx_capabilities {
        __u64 supported_attrs;
        __u64 supported_xfam;

        /* 分别在内核中执行并转发到用户空间的 TDG.VP.VMCALL 超级调用 */
        __u64 kernel_tdvmcallinfo_1_r11;
        __u64 user_tdvmcallinfo_1_r11;

        /* 分别在内核中执行并转发到用户空间的 TDG.VP.VMCALL 指令执行子功能 */
        __u64 kernel_tdvmcallinfo_1_r12;
        __u64 user_tdvmcallinfo_1_r12;

        __u64 reserved[250];

        /* 供用户空间配置的可配置 CPUID 位 */
        struct kvm_cpuid2 cpuid;
  };


```
### KVM_TDX_INIT_VM

:Type: vm ioctl
:Returns: 成功返回 0，错误返回 <0

执行 TDX 特定的 VM 初始化。这需要在 KVM_CREATE_VM 之后、创建任何 VCPU 之前调用。

- id: KVM_TDX_INIT_VM
- flags: 必须为 0
- data: 指向 struct kvm_tdx_init_vm 的指针
- hw_error: 必须为 0

```

  struct kvm_tdx_init_vm {
          __u64 attributes;
          __u64 xfam;
          __u64 mrconfigid[6];          /* sha384 摘要 */
          __u64 mrowner[6];             /* sha384 摘要 */
          __u64 mrownerconfig[6];       /* sha384 摘要 */

          /* TD_PARAMS 中 CPUID 之前的总空间为 256 字节 */
          __u64 reserved[12];

        /*
         * 在创建 vcpu 之前、即 KVM_SET_CPUID2 之前调用 KVM_TDX_INIT_VM。
         * 该配置会取代 VCPU 的 KVM_SET_CPUID2，因为 TDX module 直接
         * 虚拟化那些 CPUID，而不经由 VMM。用户空间 VMM（例如 qemu）应使
         * KVM_SET_CPUID2 与这些值保持一致。如果不一致，KVM 可能对客户机的
         * vCPUID 产生错误认识，并可能错误地模拟 TDX module 未虚拟化的
         * CPUID 或 MSR。
         */
          struct kvm_cpuid2 cpuid;
  };


```
### KVM_TDX_INIT_VCPU

:Type: vcpu ioctl
:Returns: 成功返回 0，错误返回 <0

执行 TDX 特定的 VCPU 初始化。

- id: KVM_TDX_INIT_VCPU
- flags: 必须为 0
- data: 客户机 TD VCPU RCX 的初始值
- hw_error: 必须为 0

### KVM_TDX_INIT_MEM_REGION

:Type: vcpu ioctl
:Returns: 成功返回 0，错误返回 <0

用来自 @source_addr 的用户空间提供数据，初始化从 @gpa 开始的 @nr_pages 个
TDX 客户机私有内存页。@source_addr 必须按 PAGE_SIZE 对齐。

注意，在调用此子命令之前，范围 [gpa, gpa + nr_pages] 的内存属性需要是私有的。
用户空间可以使用 KVM_SET_MEMORY_ATTRIBUTES 来设置该属性。

如果指定了 KVM_TDX_MEASURE_MEMORY_REGION 标志，它还会扩展度量（measurement）。

- id: KVM_TDX_INIT_MEM_REGION
- flags: 目前仅定义了 KVM_TDX_MEASURE_MEMORY_REGION
- data: 指向 struct kvm_tdx_init_mem_region 的指针
- hw_error: 必须为 0

```

  #define KVM_TDX_MEASURE_MEMORY_REGION   (1UL << 0)

  struct kvm_tdx_init_mem_region {
          __u64 source_addr;
          __u64 gpa;
          __u64 nr_pages;
  };


```
### KVM_TDX_FINALIZE_VM

:Type: vm ioctl
:Returns: 成功返回 0，错误返回 <0

完成初始 TD 内容的度量，并将其标记为可运行。

- id: KVM_TDX_FINALIZE_VM
- flags: 必须为 0
- data: 必须为 0
- hw_error: 必须为 0


### KVM_TDX_GET_CPUID

:Type: vcpu ioctl
:Returns: 成功返回 0，错误返回 <0

获取 TDX module 为 TD 客户机虚拟化的 CPUID 值。当它返回 -E2BIG 时，用户空间
应分配更大的缓冲并重试。最小缓冲大小会在 struct kvm_cpuid2 的 nent 字段中更新。

- id: KVM_TDX_GET_CPUID
- flags: 必须为 0
- data: 指向 struct kvm_cpuid2 的指针（in/out）
- hw_error: 必须为 0（out）

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
## KVM TDX 创建流程


除了标准的 KVM 流程外，还需要调用新的 TDX ioctl。控制流如下：

#. 检查系统级能力

   - KVM_CAP_VM_TYPES：检查 VM 类型是否受支持，以及 KVM_X86_TDX_VM 是否受支持。

#. 创建 VM

   - KVM_CREATE_VM
   - KVM_TDX_CAPABILITIES：查询用于创建 TDX 客户机的能力。
   - KVM_CHECK_EXTENSION(KVM_CAP_MAX_VCPUS)：查询 TD 在 VM 级别可支持的最大 VCPU
     数量（TDX 对此有自身限制）。
   - KVM_SET_TSC_KHZ：如果希望使用与宿主不同的 TSC 频率，则配置 TD 的 TSC 频率。
     这是可选的。
   - KVM_TDX_INIT_VM：传入 TDX 特定的 VM 参数。

#. 创建 VCPU

   - KVM_CREATE_VCPU
   - KVM_TDX_INIT_VCPU：传入 TDX 特定的 VCPU 参数。
   - KVM_SET_CPUID2：配置 TD 的 CPUID。
   - KVM_SET_MSRS：配置 TD 的 MSR。

#. 初始化初始客户机内存

   - 准备初始客户机内存的内容。
   - KVM_TDX_INIT_MEM_REGION：添加初始客户机内存。
   - KVM_TDX_FINALIZE_VM：完成 TDX 客户机的度量。

#. 运行 VCPU

## 参考


https://www.intel.com/content/www/us/en/developer/tools/trust-domain-extensions/documentation.html
