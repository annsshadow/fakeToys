## PCI 测试端点功能


name: 应为 "pci_epf_test" 以绑定到 pci_epf_test 驱动

可配置字段：

================   ===========================================================
vendorid	   应为 0x104c
deviceid	   对于 DRA74x 应为 0xb500，对DRA72x 应为 0xb501
revid		   不关
progif_code	   不关
subclass_code	   不关
baseclass_code	   应为 0xff
cache_line_size	   不关
subsys_vendor_id   不关
subsys_id	   不关
interrupt_pin	   应为 1 - INTA, 2 - INTB, 3 - INTC, 4 -INTD
msi_interrupts	   应为 1 32，取决于要测试的 MSI 中断数量
msix_interrupts	   应为 1 2048，取决于要测试的 MSI-X 中断数量
================   ===========================================================
