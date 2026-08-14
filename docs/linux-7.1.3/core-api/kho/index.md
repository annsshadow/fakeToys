


## Kexec Handover 子系统


## 概述


Kexec HandOver（KHO）是一种机制，它允许 Linux 在 kexec 过程中保留内存区域，这些区域可能包含被序列化的系统状态。

KHO 使用扁平设备树（FDT）<kho_fdt> 将有关保留状态的信息从 exec 前内核传递到 kexec 后内核，并使用 :ref:`scratch 内存区域 <kho_scratch>` 来确保保留内存的完整性。


## KHO FDT


每次 KHO kexec 都携带一个特定于 KHO 的扁平设备树（FDT）blob，描述保留的状态。FDT 包含描述保留内存区域的属性以及持有子系统特定状态的节点。

保留的内存区域包含被序列化的子系统状态，或不应在 kexec 过程中被触碰的内存中数据。在 KHO 之后，子系统可以从 KHO FDT 检索并恢复保留的状态。

参与 KHO 的子系统可以定义它们自己的状态序列化与保留格式。

KHO FDT 与子系统定义的结构体构成了 exec 前内核与 exec 后内核之间的 ABI。此 ABI 由 `include/linux/kho/abi` 目录中的头文件定义。

- [abi.rst](abi.rst)


## Scratch 区域


要引导进入 kexec，我们需要一个不包含任何交接内存的物理连续内存范围。然后 kexec 将目标内核与 initrd 放入该区域。新内核在引导期间、直到页分配器初始化之前，独占使用该区域进行内存分配。

我们通过 scratch 区域保证始终拥有这样的区域：在首次引导时，KHO 分配若干物理连续的内存区域。由于 kexec 之后这些区域将被早期内存分配使用，因此每个 NUMA 节点都有一个 scratch 区域，外加一个用于满足不要求特定 NUMA 节点分配的内存请求的 scratch 区域。默认情况下，scratch 区域的大小基于引导期间分配的内存量计算。`kho_scratch` 内核命令行选项可用于显式定义 scratch 区域的大小。当页分配器初始化时，scratch 区域被声明为 CMA，以便其内存在系统运行期间可被使用。CMA 向我们保证没有交接页落入该区域，因为交接页必须位于静态的物理内存位置，而 CMA 强制只有可移动页才能位于其中。

在 KHO kexec 之后，我们忽略 `kho_scratch` 内核命令行选项，而是复用最初分配的完全相同区域。这允许我们递归地执行任意次数的 KHO kexec。由于我们在引导内存分配以及作为 kexec blob 的目标内存中使用了该区域，该内存区域的某些部分可能已被保留。这些保留对下一次 KHO 无关紧要，因为 kexec 甚至可以覆盖原始内核。

## Kexec Handover 基数树


  :doc: Kexec Handover Radix Tree

## 公共 API


  :export:

## 另见


- [/admin-guide/mm/kho](/admin-guide/mm/kho)
