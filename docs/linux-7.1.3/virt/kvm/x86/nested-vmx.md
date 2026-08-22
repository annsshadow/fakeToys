
## Nested VMX（嵌VMX


### 概述

Intel 处理器上，KVM 利用 Intel VMX（Virtual-Machine eXtensions，虚拟机扩展）来轻松、高效地运行客户机操作系统。通常情况下，客户*不能**自身作为 hypervisor 再去运行自己的客户机，因为客户机无法使用 VMX 指令

"Nested VMX"（嵌VMX）特性填补了这一缺失的能力——它允许客户hypervisor（使VMX）再去运行嵌套的客户机。通过允许客户机使VMX 指令，并以单VMX 可用的硬件正确地、高效地模拟，来实现这一点

关于嵌套 VMX 特性背后的理论、实现及性能特征的更详细描述，可参OSDI 2010 论文 "The Turtles Project: Design and Implementation of Nested Virtualization"，地址如下

https://www.usenix.org/events/osdi10/tech/full_papers/Ben-Yehuda.pdf


### Terminology（术语）

单级虚拟化包含两个层级——host（KVM）与客户机（guests）。在嵌套虚拟化中，则有三个层级：host（KVM，称L0）、guest hypervisor（称L1）、嵌套的客户机（称为 L2）


### 运行 nested VMX

Linux 内核 v4.20 起，nested VMX 特性默认启用。在更早Linux 内核上，可通过kvm-intel 模块传"nested=1" 选项来启用

这不需要对用户空间（qemu）做任何修改。不过，qemu 默认模拟CPU 类型（qemu64）并未列"VMX" CPU 特性，因此需要显式启用，可通过以下任一qemu 选项

- 处理host（使模拟CPU 特性等同于真实 CPU

- 处理qemu64,+vmx（在已命名的 CPU 类型上仅添加 vmx 特性）


### ABIs

Nested VMX 的目标是（最终）guest hypervisor 呈现一个标准、完全可用的 VMX 实现。因此，官方规范ABI Intel VMX 规范提供，即《Intel 64 IA-32 体系结构软件开发者手册》第 3B 卷。目VMX 的某些特性已被完全支持，目标是最终支持全部特性，优先支持那些在实践中流行hypervisor（KVM 及其它）所用的 VMX 特性

VMX 的实现中，nested VMX L1 呈现 VMCS 结构体。按规范要求，其中两个字`revision_id` `abort` 是用户可见的；该结构体对用户而言*不透明**（opaque）的，用户不应关心内部结构，而应通过 VMREAD VMWRITE 指令来访问

不过，出于调试目的，KVM 开发者可能有兴趣了解该结构体的内部。该结构`vmcs12` 定义`arch/x86/kvm/vmx.c` 中

名称 "vmcs12" 指由 L1 L2 构建VMCS；代码中"vmcs01" 指由 L0 L1 构建VMCSvmcs02" 指由 L0 实际为运行中L2 构建VMCS——这在前述论文中有详细解释

为方便起见，这里复述 `vmcs12` 结构体的内容。若其内部发生变化，将破坏跨 KVM 版本的实时迁移（live migration）。只有当 `vmcs12` 的内部结构体`shadow_vmcs` 发生变化时，才需要修`VMCS12_REVISION`（位vmx.c 中）

```
	typedef u64 natural_width;
	struct __packed vmcs12 {
		/* According to the Intel spec, a VMCS region must start with
		 * these two user-visible fields */
		u32 revision_id;
		u32 abort;

		u32 launch_state; /* set to 0 by VMCLEAR, to 1 by VMLAUNCH */
		u32 padding[7]; /* room for future expansion */

		u64 io_bitmap_a;
		u64 io_bitmap_b;
		u64 msr_bitmap;
		u64 vm_exit_msr_store_addr;
		u64 vm_exit_msr_load_addr;
		u64 vm_entry_msr_load_addr;
		u64 tsc_offset;
		u64 virtual_apic_page_addr;
		u64 apic_access_addr;
		u64 ept_pointer;
		u64 guest_physical_address;
		u64 vmcs_link_pointer;
		u64 guest_ia32_debugctl;
		u64 guest_ia32_pat;
		u64 guest_ia32_efer;
		u64 guest_pdptr0;
		u64 guest_pdptr1;
		u64 guest_pdptr2;
		u64 guest_pdptr3;
		u64 host_ia32_pat;
		u64 host_ia32_efer;
		u64 padding64[8]; /* room for future expansion */
		natural_width cr0_guest_host_mask;
		natural_width cr4_guest_host_mask;
		natural_width cr0_read_shadow;
		natural_width cr4_read_shadow;
		natural_width dead_space[4]; /* Last remnants of cr3_target_value[0-3]. */
		natural_width exit_qualification;
		natural_width guest_linear_address;
		natural_width guest_cr0;
		natural_width guest_cr3;
		natural_width guest_cr4;
		natural_width guest_es_base;
		natural_width guest_cs_base;
		natural_width guest_ss_base;
		natural_width guest_ds_base;
		natural_width guest_fs_base;
		natural_width guest_gs_base;
		natural_width guest_ldtr_base;
		natural_width guest_tr_base;
		natural_width guest_gdtr_base;
		natural_width guest_idtr_base;
		natural_width guest_dr7;
		natural_width guest_rsp;
		natural_width guest_rip;
		natural_width guest_rflags;
		natural_width guest_pending_dbg_exceptions;
		natural_width guest_sysenter_esp;
		natural_width guest_sysenter_eip;
		natural_width host_cr0;
		natural_width host_cr3;
		natural_width host_cr4;
		natural_width host_fs_base;
		natural_width host_gs_base;
		natural_width host_tr_base;
		natural_width host_gdtr_base;
		natural_width host_idtr_base;
		natural_width host_ia32_sysenter_esp;
		natural_width host_ia32_sysenter_eip;
		natural_width host_rsp;
		natural_width host_rip;
		natural_width paddingl[8]; /* room for future expansion */
		u32 pin_based_vm_exec_control;
		u32 cpu_based_vm_exec_control;
		u32 exception_bitmap;
		u32 page_fault_error_code_mask;
		u32 page_fault_error_code_match;
		u32 cr3_target_count;
		u32 vm_exit_controls;
		u32 vm_exit_msr_store_count;
		u32 vm_exit_msr_load_count;
		u32 vm_entry_controls;
		u32 vm_entry_msr_load_count;
		u32 vm_entry_intr_info_field;
		u32 vm_entry_exception_error_code;
		u32 vm_entry_instruction_len;
		u32 tpr_threshold;
		u32 secondary_vm_exec_control;
		u32 vm_instruction_error;
		u32 vm_exit_reason;
		u32 vm_exit_intr_info;
		u32 vm_exit_intr_error_code;
		u32 idt_vectoring_info_field;
		u32 idt_vectoring_error_code;
		u32 vm_exit_instruction_len;
		u32 vmx_instruction_info;
		u32 guest_es_limit;
		u32 guest_cs_limit;
		u32 guest_ss_limit;
		u32 guest_ds_limit;
		u32 guest_fs_limit;
		u32 guest_gs_limit;
		u32 guest_ldtr_limit;
		u32 guest_tr_limit;
		u32 guest_gdtr_limit;
		u32 guest_idtr_limit;
		u32 guest_es_ar_bytes;
		u32 guest_cs_ar_bytes;
		u32 guest_ss_ar_bytes;
		u32 guest_ds_ar_bytes;
		u32 guest_fs_ar_bytes;
		u32 guest_gs_ar_bytes;
		u32 guest_ldtr_ar_bytes;
		u32 guest_tr_ar_bytes;
		u32 guest_interruptibility_info;
		u32 guest_activity_state;
		u32 guest_sysenter_cs;
		u32 host_ia32_sysenter_cs;
		u32 padding32[8]; /* room for future expansion */
		u16 virtual_processor_id;
		u16 guest_es_selector;
		u16 guest_cs_selector;
		u16 guest_ss_selector;
		u16 guest_ds_selector;
		u16 guest_fs_selector;
		u16 guest_gs_selector;
		u16 guest_ldtr_selector;
		u16 guest_tr_selector;
		u16 host_es_selector;
		u16 host_cs_selector;
		u16 host_ss_selector;
		u16 host_ds_selector;
		u16 host_fs_selector;
		u16 host_gs_selector;
		u16 host_tr_selector;
	};
```


### Authors

补丁由以下人员编写：

- Abel Gordon, abelg < > il.ibm.com
- Nadav Har'El, nyh < > il.ibm.com
- Orit Wasserman, oritw < > il.ibm.com
- Ben-Ami Yassor, benami < > il.ibm.com
- Muli Ben-Yehuda, muli < > il.ibm.com

贡献者：

- Anthony Liguori, aliguori < > us.ibm.com
- Mike Day, mdday < > us.ibm.com
- Michael Factor, factor < > il.ibm.com
- Zvi Dubitzky, dubi < > il.ibm.com

有价值的审阅

- Avi Kivity, avi < > redhat.com
- Gleb Natapov, gleb < > redhat.com
- Marcelo Tosatti, mtosatti < > redhat.com
- Kevin Tian, kevin.tian < > intel.com
- 以及其他人
