
## 受保护 KVM（pKVM）


**注意**：pKVM 目前是一项实验性的开发特性，随着新隔离特性的实现，可能会发生破坏性变更。
如有任何问题，请联系开发者 kvmarm@lists.linux.dev。

## 概述


以 '`kvm-arm.mode=protected`' 引导主机内核会启用“受保护 KVM”（pKVM）。在引导过程中，
pKVM 为主机安装一个 stage-2 恒等映射（identity map）页表，并用它将运行在 EL2 的
管理程序（hypervisor）与运行在 EL1/0 的主机其余部分隔离。

pKVM 通过将机器类型标识符 `KVM_VM_TYPE_ARM_PROTECTED` 传给 `KVM_CREATE_VM` ioctl()
来允许创建受保护虚拟机（pVM）。管理程序通过在一个 pVM 访问时从 stage-2 恒等映射中取消映射
页面，将 pVM 与主机隔离。提供了超级调用（hypercall）供 pVM 将其 IPA 空间的特定区域共享回
主机，以便与 VMM 通信。Linux 客户机必须配置 `CONFIG_ARM_PKVM_GUEST=y` 才能发出这些
超级调用。

更多细节请参见 hypercalls.rst。

## 隔离机制


pKVM 依赖多种机制将 pVM 与主机隔离：

### CPU 内存隔离


状态：匿名内存与元数据页的隔离。

元数据页（例如页表页与 '`struct kvm_vcpu`' 页）在创建 pVM 时从主机捐赠给管理程序，并
因此在 pVM 被销毁之前从 stage-2 恒等映射中取消映射。

与常规 KVM 类似，页面是惰性地映射到客户机中的，以响应由主机处理的 stage-2 页错误。但是，
在运行 pVM 时，这些页面首先被固定（pinned），然后作为捐赠过程的一部分从 stage-2 恒等
映射中取消映射。这导致与非受保护 VM 相比一些用户可见的差异，主要由于缺乏 MMU 通知器
（notifier）：

- 一旦 pVM 开始运行，内存槽（memslot）就不能被移动或删除。
- 不支持只读内存槽与脏页记录（dirty logging）。
- 除交换（swap）外，基于文件的页不能映射到 pVM。
- 捐赠的页计入 `RLIMIT_MLOCK`，因此 VMM 必须有足够的资源限制或被授予 `CAP_IPC_LOCK`。
  缺乏运行时回收机制意味着为 pVM 锁定的内存将保持锁定，直到 pVM 被销毁。
- 对 VMM 地址空间的更改（例如，在关联内存槽的映射上执行 `MAP_FIXED` mmap()）不会反映到
  客户机中，并可能导致一致性（coherency）丢失。
- 访问未共享回的 pVM 内存将导致发送 SIGSEGV。
- 如果系统调用访问了未共享回的 pVM 内存，则它会返回 `-EFAULT` 或强制回收内存页。被回收的
  内存由管理程序清零，随后在 pVM 中访问它的尝试将从 `VCPU_RUN` ioctl() 返回 `-EFAULT`。

### CPU 状态隔离


状态：**未实现。**

### 使用 IOMMU 的 DMA 隔离


状态：**未实现。**

### Trustzone 服务的代理


状态：来自主机的 FF-A 与 PSCI 调用由 pKVM 管理程序代理。

FF-A 代理确保主机不能将 pVM 或管理程序内存作为“ confused deputy（混淆代理）”攻击的一部分
共享给 Trustzone。

PSCI 代理确保 CPU 在执行于主机中时始终安装有 stage-2 恒等映射。

### 受保护 VM 固件（pvmfw）


状态：**未实现。**

## 资源


Quentin Perret 在 KVM Forum 2022 题为“Protected KVM on arm64: A technical deep dive”的
演讲仍然是了解 pKVM 的良好资源，尽管其间一些细节已发生变化：

https://www.youtube.com/watch?v=9npebeVFbFw
