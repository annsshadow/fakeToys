## PCI NTB 端点功能


1) configfs 中，pci_epf_ntb 目录创建一个子目录
标准 EPF 可配置字段：

================   ===========================================================
vendorid	   应为 0x104c
deviceid	   对于 TI J721E SoC，应0xb00d
revid		   不关progif_code	   不关subclass_code	   应为 0x00
baseclass_code	   应为 0x5
cache_line_size	   不关subsys_vendor_id   不关subsys_id	   不关interrupt_pin	   不关msi_interrupts	   不关msix_interrupts	   不关================   ===========================================================

2) 1 中创建的目录创建一个子目录

NTB EPF 特定可配置字段：

================   ===========================================================
db_count	   门铃（doorbell）数量；默认 = 4
mw1     	   内存窗口 1 的大mw2     	   内存窗口 2 的大mw3     	   内存窗口 3 的大mw4     	   内存窗口 4 的大num_mws     	   内存窗口数量；最大= 4
spad_count     	   暂存寄存器（scratchpad）数量；默认 = 64
================   ===========================================================
