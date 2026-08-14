
## PAT（页属性表）

x86 的页属性表（Page Attribute Table，PAT）允许在页粒度上设置内存属性。
PAT 是对 MTRR 设置的补充，MTRR 用于在物理地址范围上设置内存类型。然而，
PAT 比 MTRR 更灵活，因为它能够在页级别设置属性，而且此类属性设置的数量
没有硬件限制。增加的灵活性伴随着一些准则：对于同一物理内存的多个虚拟
地址，不应出现内存类型别名。

PAT 允许使用不同类型的内存属性。当前将支持的最常用类型如下：

===  ==============
WB   写回
UC   不缓存
WC   写合并
WT   写直达
UC-  不缓存减型
===  ==============


## PAT API

内核中有许多不同的 API 允许在页级别设置内存属性。为了避免别名，应当
谨慎使用这些接口。下面是一张可用接口表，列出了它们的预期用途及其内存
属性关系。在内部，这些 API 在物理地址范围上使用
reserve_memtype()/free_memtype() 接口来避免任何别名。

+------------------------+----------+--------------+------------------+
| API                    |    RAM   |  ACPI,...    |  Reserved/Holes  |
+------------------------+----------+--------------+------------------+
| ioremap                |    --    |    UC-       |       UC-        |
+------------------------+----------+--------------+------------------+
| ioremap_cache          |    --    |    WB        |       WB         |
+------------------------+----------+--------------+------------------+
| ioremap_uc             |    --    |    UC        |       UC         |
+------------------------+----------+--------------+------------------+
| ioremap_wc             |    --    |    --        |       WC         |
+------------------------+----------+--------------+------------------+
| ioremap_wt             |    --    |    --        |       WT         |
+------------------------+----------+--------------+------------------+
| set_memory_uc,         |    UC-   |    --        |       --         |
| set_memory_wb          |          |              |                  |
+------------------------+----------+--------------+------------------+
| set_memory_wc,         |    WC    |    --        |       --         |
| set_memory_wb          |          |              |                  |
+------------------------+----------+--------------+------------------+
| set_memory_wt,         |    WT    |    --        |       --         |
| set_memory_wb          |          |              |                  |
+------------------------+----------+--------------+------------------+
| pci sysfs resource     |    --    |    --        |       UC-        |
+------------------------+----------+--------------+------------------+
| pci sysfs resource_wc  |    --    |    --        |       WC         |
| is IORESOURCE_PREFETCH |          |              |                  |
+------------------------+----------+--------------+------------------+
| pci proc               |    --    |    --        |       UC-        |
| !PCIIOC_WRITE_COMBINE  |          |              |                  |
+------------------------+----------+--------------+------------------+
| pci proc               |    --    |    --        |       WC         |
| PCIIOC_WRITE_COMBINE   |          |              |                  |
+------------------------+----------+--------------+------------------+
| /dev/mem               |    --    |   WB/WC/UC-  |    WB/WC/UC-     |
| read-write             |          |              |                  |
+------------------------+----------+--------------+------------------+
| /dev/mem               |    --    |    UC-       |       UC-        |
| mmap SYNC flag         |          |              |                  |
+------------------------+----------+--------------+------------------+
| /dev/mem               |    --    |   WB/WC/UC-  |  WB/WC/UC-       |
| mmap !SYNC flag        |          |              |                  |
| and                    |          |(from existing|  (from existing  |
| any alias to this area |          |alias)        |  alias)          |
+------------------------+----------+--------------+------------------+
| /dev/mem               |    --    |    WB        |       WB         |
| mmap !SYNC flag        |          |              |                  |
| no alias to this area  |          |              |                  |
| and                    |          |              |                  |
| MTRR says WB           |          |              |                  |
+------------------------+----------+--------------+------------------+
| /dev/mem               |    --    |    --        |       UC-        |
| mmap !SYNC flag        |          |              |                  |
| no alias to this area  |          |              |                  |
| and                    |          |              |                  |
| MTRR says !WB          |          |              |                  |
+------------------------+----------+--------------+------------------+


## 面向驱动的高级 API


A. 使用 remap_pfn_range、io_remap_pfn_range、vmf_insert_pfn 向用户导出页。

希望向用户空间导出某些页的驱动，通过 mmap 接口以及以下组合来实现：

  1) pgprot_noncached()
  2) io_remap_pfn_range() 或 remap_pfn_range() 或 vmf_insert_pfn()

借助 PAT 支持，正在新增一个 API pgprot_writecombine。因此，驱动可以继续
使用上述序列，在第 1 步使用 pgprot_noncached() 或 pgprot_writecombine()，
然后执行第 2 步。

此外，第 2 步会在内部将该区域作为 UC 或 WC 在 memtype 列表中进行追踪，
以确保不会出现冲突的映射。

注意，这组 API 仅适用于 IO（非 RAM）区域。如果驱动想要导出 RAM 区域，
则必须如上面的第 0 步那样执行 set_memory_uc() 或 set_memory_wc()，并且
还要追踪这些页的使用情况，并在该页释放回空闲池之前使用 set_memory_wb()。

## MTRR 对 PAT / 非 PAT 系统的影响


下表说明了在 x86 上同时使用 ioremap*() 调用时，使用写合并型 MTRR 对
非 PAT 和 PAT 系统的影响。理想情况下，mtrr_add() 的使用将被逐步淘汰，
转而使用 arch_phys_wc_add()，后者在启用 PAT 的系统上为空操作。执行
arch_phys_wc_add() 的区域应当已经通过 WC 属性或 PAT 表项进行了 ioremap，
这可以通过使用 ioremap_wc() / set_memory_wc() 来完成。对于希望将需要保持
不可缓存的 IO 内存区域与适合写合并的区域组合在一起的设备，应考虑使用
ioremap_uc() 后接 set_memory_wc() 来将有效的写合并区域加入白名单。不过，
这种用法仍然不被鼓励，因为有效的内存类型被视为实现定义的，但此策略可
作为最后手段用于那些空间受限、否则 MTRR 写合并将不起作用的设备。
```

  ====  =======  ===  =========================  =====================
  MTRR  Non-PAT  PAT  Linux ioremap value        Effective memory type
  ====  =======  ===  =========================  =====================
        PAT                                        Non-PAT |  PAT
        |PCD                                               |
        ||PWT                                              |
        |||                                                |
  WC    000      WB   _PAGE_CACHE_MODE_WB             WC   |   WC
  WC    001      WC   _PAGE_CACHE_MODE_WC             WC*  |   WC
  WC    010      UC-  _PAGE_CACHE_MODE_UC_MINUS       WC*  |   UC
  WC    011      UC   _PAGE_CACHE_MODE_UC             UC   |   UC
  ====  =======  ===  =========================  =====================

  (*) denotes implementation defined and is discouraged

```
  其中的 -- 由内核严格执行。其他一些目前并未真正强制执行，但将来
  可能会强制执行。

对于 ioremap 以及通过 /sys 或 /proc 进行的 PCI 访问——在某些情况下，如果
该地址存在任何已有的别名，返回的实际类型可能会更严格。例如：如果已存在
一个不可缓存的映射，那么新的 ioremap_wc 可能会返回不可缓存的映射，而非
所请求的写合并。

set_memory_[uc|wc|wt] 和 set_memory_wb 应当成对使用，驱动首先将某区域
设为 uc、wc 或 wt，使用后再将其切换回 wb。

随着时间的推移，对 /proc/mtrr 的写入将被废弃，转而使用基于 PAT 的接口。
建议写入 /proc/mtrr 的用户使用上述接口。

驱动应当使用 ioremap_[uc|wc] 来访问具有 [uc|wc] 访问类型的 PCI BAR。

驱动应当使用 set_memory_[uc|wc|wt] 来设置 RAM 范围的访问类型。


## PAT 调试


```

  # mount -t debugfs debugfs /sys/kernel/debug
  # cat /sys/kernel/debug/x86/pat_memtype_list
  PAT memtype list:
  uncached-minus @ 0x7fadf000-0x7fae0000
  uncached-minus @ 0x7fb19000-0x7fb1a000
  uncached-minus @ 0x7fb1a000-0x7fb1b000
  uncached-minus @ 0x7fb1b000-0x7fb1c000
  uncached-minus @ 0x7fb1c000-0x7fb1d000
  uncached-minus @ 0x7fb1d000-0x7fb1e000
  uncached-minus @ 0x7fb1e000-0x7fb25000
  uncached-minus @ 0x7fb25000-0x7fb26000
  uncached-minus @ 0x7fb26000-0x7fb27000
  uncached-minus @ 0x7fb27000-0x7fb28000
  uncached-minus @ 0x7fb28000-0x7fb2e000
  uncached-minus @ 0x7fb2e000-0x7fb2f000
  uncached-minus @ 0x7fb2f000-0x7fb30000
  uncached-minus @ 0x7fb31000-0x7fb32000
  uncached-minus @ 0x80000000-0x90000000

```
此列表显示了物理地址范围以及用于访问这些物理地址范围的各种 PAT 设置。

另一种更详细的获取 PAT 相关调试消息的方式是使用 "debugpat" 引导参数。
使用该参数后，各种调试消息会被打印到 dmesg 日志中。

## PAT 初始化


下表描述了在各种配置下 PAT 如何被初始化。PAT MSR 必须由 Linux 更新，
以支持 WC 和 WT 属性。否则，PAT MSR 中保存的是固件写入其中的值。注意，
Xen 在客户机的 PAT MSR 中启用了 WC 属性。

 ==== ===== ==========================  =========  =======
 MTRR PAT   Call Sequence               PAT State  PAT MSR
 ==== ===== ==========================  =========  =======
 E    E     MTRR -> PAT init            Enabled    OS
 E    D     MTRR -> PAT init            Disabled    -
 D    E     MTRR -> PAT disable         Disabled   BIOS
 D    D     MTRR -> PAT disable         Disabled    -
 - np/E  PAT  -> PAT disable         Disabled   BIOS
 - np/D  PAT  -> PAT disable         Disabled    -
 E    !P/E  MTRR -> PAT init            Disabled   BIOS
 D    !P/E  MTRR -> PAT disable         Disabled   BIOS
 !M   !P/E  MTRR stub -> PAT disable    Disabled   BIOS
 ==== ===== ==========================  =========  =======

  图例

 ========= =======================================
 E         CPU 中启用的特性
 D	       CPU 中禁用/不支持的特性
 np	       指定了 "nopat" 引导选项
 !P	       CONFIG_X86_PAT 选项未设置
 !M	       CONFIG_MTRR 选项未设置
 Enabled   PAT 状态设为已启用
 Disabled  PAT 状态设为已禁用
 OS        PAT 使用 OS 设置初始化 PAT MSR
 BIOS      PAT 保持 PAT MSR 的 BIOS 设置
 ========= =======================================

