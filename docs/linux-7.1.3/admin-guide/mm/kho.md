
## Kexec Handover 使用


Kexec HandOver（KHO，kexec 移交）是一种机制，允许 Linux 在 kexec 过程中保留可能包含序列化系统状态的
内存区域。

本文档假定你已熟悉基础 KHO 概念 <kho-concepts>。如果你还没有阅读它们，请现在阅读。

## 先决条件


当内核编译时将 `CONFIG_KEXEC_HANDOVER` 设置为 y 时，KHO 可用。每个 KHO 生产者可能有自己的配置选项，
如果你希望跨 kexec 保留它们各自的状态，则需要启用它。

要使用 KHO，请使用 `kho=on` 命令行参数引导内核。你可以使用 `kho_scratch` 参数定义 scratch 区域的大小。
例如 `kho_scratch=16M,512M,256M` 将在引导时保留一个 16 MiB 的低内存 scratch 区域、一个 512 MiB 的全局
scratch 区域，以及每个 NUMA 节点 256 MiB 的 scratch 区域。

## 执行一次 KHO kexec


要执行一次 KHO kexec，加载目标负载并 kexec 进入它。重要的是你要使用 `-s` 参数来使用内核内的 kexec 文件
加载器，因为用户空间 kexec 工具当前没有
```

  # kexec -l /path/to/bzImage --initrd /path/to/initrd -s
  # kexec -e

```
新内核将启动并包含前一个内核的部分状态。

例如，如果你使用了 `reserve_mem` 命令行参数创建一个早期内存保留，新内核将在与前一个内核相同的物理地址
拥有该内存。

## Kexec 元数据


KHO 自动跟踪关于 kexec 链的元数据，将前一个内核的信息传递给下一个内核。此特性有助于诊断仅在从特定内核
版本 kexec 时才复现的缺陷。

在每次 KHO kexec 时，内核会记录前一个内核的版本和
```

    [    0.000000] KHO: exec from: 6.19.0-rc4-next-20260107 (count 1)

```
元数据包括：

`previous_release`
    发起 kexec 的内核的版本字符串（来自 `uname -r`）。

`kexec_count`
    自上次冷启动以来的 kexec 引导次数。在冷启动时，此计数器从 0 开始，并在每次 kexec 时递增。这有助于
    识别仅在多次连续 kexec 重启后才显现的问题。

### 用例


此元数据对于调试 kexec 转换缺陷特别有用，其中一个有缺陷的内核 kexec 进入新内核，而该缺陷仅出现在第二个
内核中。此类缺陷的示例包括：

- 前一内核的内存损坏影响了新内核
- 前一内核遗留的不正确的硬件状态
- 仅在 kexec 场景出现的固件/ACPI 状态问题

在大规模下，将崩溃与前一内核版本相关联，能够在问题仅出现在特定内核转换场景时实现更快的根因分析。

## debugfs 接口


这些 debugfs 接口在内核编译时启用了 `CONFIG_KEXEC_HANDOVER_DEBUGFS` 时可用。

目前 KHO 创建以下 debugfs 接口。注意这些接口将来可能会改变。一旦 KHO 稳定，它们将被移到 sysfs。

`/sys/kernel/debug/kho/out/fdt`
    内核在此文件中暴露携带其当前 KHO 状态的扁平设备树 blob。Kexec 用户空间工具可以使用此文件作为
    KHO 负载映像的输入文件。

`/sys/kernel/debug/kho/out/scratch_len`
    KHO scratch 区域的长度，这些是物理上连续的、将始终可用于未来 kexec 分配的内存区域。Kexec 用户空间
    工具可以使用此文件来确定应将负载映像放在何处。

`/sys/kernel/debug/kho/out/scratch_phys`
    KHO scratch 区域的物理位置。Kexec 用户空间工具可以将此文件与 scratch_phys 结合使用以确定应将负载
    映像放在何处。

`/sys/kernel/debug/kho/out/sub_fdts/`
    KHO 生产者可以在此目录下注册它们自己的 FDT 或另一个二进制 blob。

`/sys/kernel/debug/kho/in/fdt`
    当内核以 Kexec HandOver（KHO）引导时，携带前一内核状态元数据的状态树以扁平设备树的格式位于此文件
    中。当它的所有消费者都完成对其元数据的解析后，此文件可能会消失。

`/sys/kernel/debug/kho/in/sub_fdts/`
    类似于 `kho/out/sub_fdts/`，但包含从前一内核传递过来的 KHO 生产者的子 blob。
