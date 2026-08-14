## PCI NTB 端点功能


1) 在 configfs 中，为 pci_epf_ntb 目录创建一个子目录。

标准 EPF 可配置字段：

================   ===========================================================
vendorid	   应为 0x104c
deviceid	   对于 TI 的 J721E SoC，应为 0xb00d
revid		   不关心
progif_code	   不关心
subclass_code	   应为 0x00
baseclass_code	   应为 0x5
cache_line_size	   不关心
subsys_vendor_id   不关心
subsys_id	   不关心
interrupt_pin	   不关心
msi_interrupts	   不关心
msix_interrupts	   不关心
================   ===========================================================

2) 为 1 中创建的目录创建一个子目录

NTB EPF 特定可配置字段：

================   ===========================================================
db_count	   门铃（doorbell）数量；默认 = 4
mw1     	   内存窗口 1 的大小
mw2     	   内存窗口 2 的大小
mw3     	   内存窗口 3 的大小
mw4     	   内存窗口 4 的大小
num_mws     	   内存窗口数量；最大值 = 4
spad_count     	   暂存寄存器（scratchpad）数量；默认 = 64
================   ===========================================================
